#!/usr/bin/env bun
// tool/src/compiler/backend/block-position.js
// v8/src/compiler/backend/block-position.{h,cc} → v8-compiler-backend/src/block_position.rs.skel
// BlockPositionNumberer: Pettis-Hansen top-down PGO 块重排 (最大执行计数贪心选后继)
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/block-position.h");
const cc = readV8("src/compiler/backend/block-position.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: 类签名 + 公有 1 + 私有方法 5 + 字段 2
must(h.includes("class V8_EXPORT_PRIVATE BlockPositionNumberer {"), "h 类签名");
must(h.includes("BlockPositionNumberer(InstructionSequence* sequence, Zone* zone)"),
  "h ctor (内联)");
must(h.includes("ZoneVector<int32_t> ComputeProfileGuidedBlockPosition();"),
  "h 公有 ComputeProfileGuidedBlockPosition");
for (const sig of [
  "const InstructionBlocks& instruction_blocks() const;",
  "InstructionBlock* InstructionBlockAt(RpoNumber rpo_number) const;",
  "std::pair<const InstructionBlock*, uint64_t> GetBestSuccessor(",
  "const InstructionBlock* GetBestSuccessorFromPlacedBlocks(",
  "ZoneVector<int32_t> ComputeTopdownPosition(const InstructionBlock* entry);",
  "Zone* zone() { return zone_; }",
]) must(h.includes(sig), `h 私有 ${sig.slice(0, 40)}`);
for (const f of ["InstructionSequence* const code_;", "Zone* zone_;"])
  must(h.includes(f), `h 字段 ${f}`);
must(h.includes("Pettis-Hansen"), "h 算法注释锚");

// cc: 成员实现 6 (头文件内联 ctor/zone() 外)
must(count(cc, /BlockPositionNumberer::\w+/g) === 6, "cc 成员实现 6");
// cc: GetBestSuccessor 单后继快路径 + 多后继贪心
must(cc.includes("if (block->SuccessorCount() == 1) {"), "cc 单后继快路径");
must(cc.includes("succ_count > best_execution_count || best_succ == nullptr"), "cc 零计数兜底 (两处同式)");
must(count(cc, /succ_count > best_execution_count \|\| best_succ == nullptr/g) === 2,
  "cc 零计数兜底 ×2 (GetBestSuccessor/FromPlacedBlocks)");
// cc: ComputeTopdownPosition 双层 while + CHECK ×2
must(count(cc, /CHECK\(/g) === 2, "cc CHECK ×2 (界检查/全覆盖检查)");
must(cc.includes("while (start) {") && cc.includes("while (next) {"), "cc 双层 while");
// cc: pgo_execution_count 消费 ×3 (单后继/多后继/—)
must(count(cc, /pgo_execution_count\(\)/g) === 2, "cc pgo_execution_count ×2");
must(cc.includes("std::set<RpoNumber> visited"), "cc visited 集合");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/block-position.js",
  "src/compiler/backend/block-position.h + block-position.cc",
  "BlockPositionNumberer: Pettis-Hansen top-down PGO 块重排 (贪心最大执行计数)")}
//! 台账: 公有 ComputeProfileGuidedBlockPosition → ComputeTopdownPosition(entry);
//!   GetBestSuccessor (单后继快路径继承计数 / 多后继取最大 pgo_execution_count,
//!   零计数后继不漏选) + GetBestSuccessorFromPlacedBlocks (已放块中找新起点);
//!   双层 while: 内层链式放后继, 外层从已放块重启; CHECK ×2 (下标界/visited 全覆盖);
//!   产物为块排列 ZoneVector<int32_t> (rpo 序)。

/// v8: BlockPositionNumberer::ComputeProfileGuidedBlockPosition — 公有入口
// pub fn compute_profile_guided_block_position(&self) -> Vec<i32>
/// v8: BlockPositionNumberer::ComputeTopdownPosition — 双层贪心主体
// fn compute_topdown_position(&self, entry: &InstructionBlock) -> Vec<i32>
/// v8: BlockPositionNumberer::GetBestSuccessor — (best 后继, 计数)
// fn get_best_successor(&self, block: &InstructionBlock, visited: &Set) -> (Option<&InstructionBlock>, u64)
/// v8: BlockPositionNumberer::GetBestSuccessorFromPlacedBlocks — 已放块重启起点
// fn get_best_successor_from_placed_blocks(&self, visited: &Set) -> Option<&InstructionBlock>
/// v8: BlockPositionNumberer::InstructionBlockAt — 序列索引
// fn instruction_block_at(&self, rpo: RpoNumber) -> &InstructionBlock

// —— 提取不变式断言 ——
const _: () = {
  assert!(true); // 纯过程 pass, 无常量表; 锚点对数见生成脚本日志
};
`;

writeSrc("v8-compiler-backend", "block_position.rs.skel", skel);
console.log(`block-position.js: 对数 ${checks} 条全过; 方法 6 (公有 1) + 字段 2; ` +
  `CHECK ×2; pgo_execution_count ×2; 产物 block_position.rs.skel`);
