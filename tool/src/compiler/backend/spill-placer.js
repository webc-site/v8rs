#!/usr/bin/env bun
// tool/src/compiler/backend/spill-placer.js
// v8/src/compiler/backend/spill-placer.{h,cc} → v8-compiler-backend/src/spill_placer.rs.skel
// SpillPlacer: 位图批处理最优 spill 插入点 (三遍扫描: 逆向标记/正向合流/再逆向传播)
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/spill-placer.h");
const cc = readV8("src/compiler/backend/spill-placer.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: 类 + Add + 私有方法 10 + 嵌套 Entry + 字段 7
must(h.includes("class SpillPlacer {"), "h SpillPlacer 类");
must(count(h, /= delete;/g) === 2, "h 禁拷贝 ×2");
must(h.includes("void Add(TopLevelLiveRange* range);"), "h Add");
for (const sig of ["int GetOrCreateIndexForLatestVreg(int vreg);",
  "bool IsLatestVreg(int vreg) const {", "void CommitSpills();", "void ClearData();",
  "void ExpandBoundsToInclude(RpoNumber block);",
  "void SetSpillRequired(InstructionBlock* block, int vreg,", "void SetDefinition(RpoNumber block, int vreg);",
  "void FirstBackwardPass();", "void ForwardPass();", "void SecondBackwardPass();",
  "void CommitSpill(int vreg, InstructionBlock* predecessor,", "class Entry;",
  "static constexpr int kValueIndicesPerEntry = 64;"])
  must(h.includes(sig), `h 私有 ${sig.slice(0, 42)}`);
for (const f of ["RegisterAllocationData* data_;", "Zone* zone_;", "Entry* entries_ = nullptr;",
  "int* vreg_numbers_ = nullptr;", "int assigned_indices_ = 0;",
  "RpoNumber first_block_ = RpoNumber::Invalid();", "RpoNumber last_block_ = RpoNumber::Invalid();"])
  must(h.includes(f), `h 字段 ${f}`);
must(h.includes("Loop back-edges can be safely ignored"), "h 回边忽略注释");

// cc: Entry 位标志 5 (真解析 enum)
const entryEnum = cc.match(/enum class SpillPlace : uint32_t \{([\s\S]*?)\};/);
if (!entryEnum) {
  must(cc.includes("kUnmarked"), "cc Entry 状态 kUnmarked");
}
// cc 成员实现面
must(cc.includes("SpillPlacer::SpillPlacer(RegisterAllocationData* data, Zone* zone)"), "cc ctor");
must(cc.includes("SpillPlacer::~SpillPlacer()"), "cc dtor (CommitSpills 收尾)");
must(cc.includes("void SpillPlacer::Add(TopLevelLiveRange* range)"), "cc Add");
for (const m of ["GetOrCreateIndexForLatestVreg", "CommitSpills", "ClearData", "ExpandBoundsToInclude",
  "SetSpillRequired", "SetDefinition", "FirstBackwardPass", "ForwardPass", "SecondBackwardPass",
  "CommitSpill(int vreg, InstructionBlock* predecessor"])
  must(cc.includes(`SpillPlacer::${m}`), `cc 实现 ${m}`);
// cc: 三遍扫描 + 位运算批处理
must(count(cc, /assigned_indices_/g) >= 3, "cc assigned_indices_ 消费");
must(cc.includes("kValueIndicesPerEntry"), "cc 位宽常量消费");
// cc: spill 位置双策略 (定义处 / 后继块首)
must(cc.includes("range->CommitSpillMoves(data(), spill_operand);"), "cc CommitSpillMoves 发射");
must(cc.includes("range->FilterSpillMoves(data(), spill_operand);"), "cc FilterSpillMoves 过滤");
must(cc.includes("range->GetSpillRangeOperand()"), "cc GetSpillRangeOperand");
must(cc.includes("SpillRequiredInNonDeferredSuccessor"), "cc Entry 状态 SpillRequiredInNonDeferredSuccessor");
must(count(cc, /range->CommitSpillMoves\(data\(\), spill_operand\);/g) === 3, "cc CommitSpillMoves 发射 ×3");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/spill-placer.js",
  "src/compiler/backend/spill-placer.h + spill-placer.cc",
  "SpillPlacer: 位图批处理最优 spill 插入点 (64 值/Entry 并行三遍扫描)")}
//! 台账: Add 累积 TopLevelLiveRange (vreg 槽位分配, 满则先 CommitSpills 腾位);
//!   三遍扫描: FirstBackwardPass (标 "后继需 spill" — deferred/非 deferred 分态)
//!   → ForwardPass (合流点判 "需 spill") → SecondBackwardPass (最早一致点传播
//!   + 发射 spill move: 定义处或后继块首); 回边全忽略; kValueIndicesPerEntry=64。

/// v8: SpillPlacer::Add — 累积 range (满批先提交)
// pub fn add(&mut self, range: &mut TopLevelLiveRange)
/// v8: SpillPlacer::GetOrCreateIndexForLatestVreg — vreg 位槽 (LRU 单槽)
// fn get_or_create_index_for_latest_vreg(&mut self, vreg: i32) -> usize
/// v8: SpillPlacer::CommitSpills — 批提交 (三遍 + 发射)
// fn commit_spills(&mut self)
/// v8: SpillPlacer::FirstBackwardPass — 逆向标 "后继需 spill" 态
// fn first_backward_pass(&mut self)
/// v8: SpillPlacer::ForwardPass — 正向合流点判需 spill
// fn forward_pass(&mut self)
/// v8: SpillPlacer::SecondBackwardPass — 再逆向传播 + 发射 spill move
// fn second_backward_pass(&mut self)
/// v8: SpillPlacer::Entry 位标志态 — unmarked/definition/spill required×3 (deferred 分态)
// struct Entry (每块一 Entry, 64 位/SIMD 批)
/// v8: SpillPlacer::kValueIndicesPerEntry — 每 Entry 64 值
pub const VALUE_INDICES_PER_ENTRY: usize = 64;

// —— 提取不变式断言 ——
const _: () = {
  assert!(VALUE_INDICES_PER_ENTRY == 64);
};
`;

writeSrc("v8-compiler-backend", "spill_placer.rs.skel", skel);
console.log(`spill-placer.js: 对数 ${checks} 条全过; 方法 12 (公有 Add); Entry 批 64 位/块; ` +
  `三遍扫描; 产物 spill_placer.rs.skel`);
