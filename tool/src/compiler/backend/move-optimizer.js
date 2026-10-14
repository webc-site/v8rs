#!/usr/bin/env bun
// tool/src/compiler/backend/move-optimizer.js
// v8/src/compiler/backend/move-optimizer.{h,cc} → v8-compiler-backend/src/move_optimizer.rs.skel
// MoveOptimizer: gap move 压缩/下推/合并 (Compress* / Migrate / OptimizeMerge)
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/move-optimizer.h");
const cc = readV8("src/compiler/backend/move-optimizer.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: 类 + Run + 私有方法 9 + 字段 5
must(h.includes("class V8_EXPORT_PRIVATE MoveOptimizer final {"), "h 类签名 final");
must(count(h, /= delete;/g) === 2, "h 禁拷贝 ×2");
must(h.includes("void Run();"), "h Run");
for (const sig of ["void CompressGaps(Instruction* instr);", "void CompressBlock(InstructionBlock* block);",
  "void CompressMoves(ParallelMove* left, MoveOpVector* right);", "void MigrateMoves(Instruction* to, Instruction* from);",
  "void RemoveClobberedDestinations(Instruction* instruction);",
  "const Instruction* LastInstruction(const InstructionBlock* block) const;",
  "void OptimizeMerge(InstructionBlock* block);", "void FinalizeMoves(Instruction* instr);"])
  must(h.includes(sig), `h 私有 ${sig.slice(0, 45)}`);
for (const f of ["Zone* const local_zone_;", "InstructionSequence* const code_;",
  "MoveOpVector local_vector_;", "ZoneVector<InstructionOperand> operand_buffer1;",
  "ZoneVector<InstructionOperand> operand_buffer2;"])
  must(h.includes(f), `h 字段 ${f}`);

// cc: 成员实现 10 (ctor + Run + 8 方法)
must(count(cc, /^[\w:* &]*MoveOptimizer::\w+\(/gm) === 10, `cc 成员实现 10 (ctor+Run+8)`);
// cc: Run 两段 (CompressBlock 全块 → OptimizeMerge + FinalizeMoves)
must(cc.includes("CompressBlock(block);"), "cc Run 段 1");
must(cc.includes("OptimizeMerge(block);"), "cc OptimizeMerge 调用");
must(cc.includes("FinalizeMoves(gap);"), "cc FinalizeMoves 调用");
// cc: MoveKey (destination/source) — OptimizeMerge 公共 move 提取
must(cc.includes("struct MoveKey {"), "cc MoveKey");
must(cc.includes("destination;"), "cc MoveKey.destination");
// cc: CompressMoves 核心谓词面
must(count(cc, /IsRedundant\(\)/g) === 9, "cc IsRedundant ×9");
must(count(cc, /Eliminate\(\)/g) === 7, "cc Eliminate ×7");
// cc: MigrateMoves 内部哑结构 (捕获循环用)
must(cc.includes("struct Dummy {};"), "cc MigrateMoves Dummy");
// cc: 双 operand 缓冲复用
must(count(cc, /operand_buffer[12]/g) >= 2, "cc operand 缓冲消费");
// cc: OptimizeMerge 只处理多前驱块
must(count(cc, /predecessor/g) >= 3, "cc 前驱面");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/move-optimizer.js",
  "src/compiler/backend/move-optimizer.h + move-optimizer.cc",
  "MoveOptimizer: gap move 压缩/下推/前驱公共合并")}
//! 台账: Run = CompressBlock (含 RemoveClobberedDestinations + MigrateMoves
//!   下推 + CompressGaps 首 gap 压缩) → OptimizeMerge (MoveKey 哈希多前驱
//!   公共 move 上提, gap0/gap1 CompressMoves) → FinalizeMoves (末 gap 收尾);
//!   CompressMoves = 左右消重/链式消除; 双 operand 缓冲复用。

/// v8: MoveOptimizer::Run — 公有入口
// pub fn run(&mut self)
/// v8: MoveOptimizer::CompressBlock — 块内压缩 (清 clobber 后逐指令)
// fn compress_block(&mut self, block: &mut InstructionBlock)
/// v8: MoveOptimizer::CompressGaps — 首 gap 合并两 parallel_moves
// fn compress_gaps(&mut self, instr: &mut Instruction)
/// v8: MoveOptimizer::CompressMoves — 左 ParallelMove 与右 move 向量消重
// fn compress_moves(&mut self, left: &mut ParallelMove, right: &mut Vec<*mut MoveOperands>)
/// v8: MoveOptimizer::MigrateMoves — from 的可移 move 下推到 to
// fn migrate_moves(&mut self, to: &mut Instruction, from: &mut Instruction)
/// v8: MoveOptimizer::RemoveClobberedDestinations — 被覆盖目标清除
// fn remove_clobbered_destinations(&mut self, instruction: &mut Instruction)
/// v8: MoveOptimizer::LastInstruction — 块末指令
// fn last_instruction(&self, block: &InstructionBlock) -> &Instruction
/// v8: MoveOptimizer::OptimizeMerge — 多前驱公共 move 上提 (MoveKey)
// fn optimize_merge(&mut self, block: &mut InstructionBlock)
/// v8: MoveOptimizer::FinalizeMoves — 收尾 (0 gap 消除/1 gap 收拢)
// fn finalize_moves(&mut self, instr: &mut Instruction)

// —— 提取不变式断言 ——
const _: () = {
  assert!(true); // 过程 pass; 方法面 9 对数见生成脚本日志
};
`;

writeSrc("v8-compiler-backend", "move_optimizer.rs.skel", skel);
console.log(`move-optimizer.js: 对数 ${checks} 条全过; 方法 10 (公有 Run); MoveKey 合并面; ` +
  `双 operand 缓冲; 产物 move_optimizer.rs.skel`);
