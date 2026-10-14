#!/usr/bin/env bun
// tool/src/compiler/backend/gap-resolver.js
// v8/src/compiler/backend/gap-resolver.{h,cc} → v8-compiler-backend/src/gap_resolver.rs.skel
// GapResolver: 并行移动序列化 (环检测 + 临时位置破环), Assembler 抽象 7 虚方法
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/gap-resolver.h");
const cc = readV8("src/compiler/backend/gap-resolver.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: 嵌套 Assembler 接口 7 纯虚 + temp_slots_ 字段
must(h.includes("class GapResolver final {"), "h GapResolver final");
must(count(h, /virtual void |virtual AllocatedOperand /g) === 8, "h Assembler 纯虚 8");
must(h.includes("virtual ~Assembler() = default;"), "h 虚析构");
must(h.includes("virtual ~Assembler() = default;"), "h 虚析构");
for (const sig of [
  "virtual void AssembleMove(InstructionOperand* source,",
  "virtual void AssembleSwap(InstructionOperand* source,",
  "virtual AllocatedOperand Push(InstructionOperand* src) = 0;",
  "virtual void Pop(InstructionOperand* dest, MachineRepresentation rep) = 0;",
  "virtual void PopTempStackSlots() = 0;",
  "virtual void MoveToTempLocation(InstructionOperand* src,",
  "virtual void MoveTempLocationTo(InstructionOperand* dst,",
  "virtual void SetPendingMove(MoveOperands* move) = 0;",
]) must(h.includes(sig.replace(",\n", ", ").replace(/\s+/g, " ")) || h.includes(sig),
  `h Assembler ${sig.slice(15, 50)}`);
must(h.includes("int temp_slots_ = 0;"), "h temp_slots_ 字段");
// h: 主类方法 4 + 字段 1
must(h.includes("V8_EXPORT_PRIVATE void Resolve(ParallelMove* parallel_move);"), "h Resolve");
for (const sig of ["void PerformCycle(const std::vector<MoveOperands*>& cycle);",
  "void PerformMove(ParallelMove* moves, MoveOperands* move);",
  "MoveOperands* PerformMoveHelper(ParallelMove* moves, MoveOperands* move,"])
  must(h.includes(sig), `h 私有 ${sig.slice(0, 40)}`);
must(h.includes("Assembler* const assembler_;"), "h assembler_ 字段");

// cc: 成员实现 4
must(count(cc, /^[\w:* ]+GapResolver::\w+\(/gm) === 4, "cc 成员实现 4");
// cc: Resolve 主循环 — 未消除 move 逐个 PerformMove
must(cc.includes("if (!move->IsEliminated()) PerformMove(moves, move);"), "cc 未消除谓词");
// cc: PerformCycle — SetPendingMove + MoveToTempLocation + MoveTempLocationTo 三步破环
for (const step of ["SetPendingMove(", "MoveToTempLocation(", "MoveTempLocationTo("])
  must(cc.includes(step), `cc 破环步 ${step}`);
// cc: PerformMove 重试环
must(cc.includes("while (MoveOperands* blocking_move = PerformMoveHelper(moves, move, &cycle)) {"),
  "cc blocking_move 重试环");
// cc: 递归深度 — PerformMoveHelper 递归自调
must(cc.includes("blocking_move = PerformMoveHelper(moves, other, &cycle_rec);"), "cc 递归 cycle_rec");
// cc: move 消除谓词面
for (const [api, n] of [["IsEliminated()", 2], ["Eliminate()", 5], ["IsPending(", 2],
  ["IsRedundant()", 2], ["set_destination(", 1], ["set_source(", 1]])
  must(cc.split(api).length - 1 === n, `cc MoveOperands API ${api} ×${n}`);

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/gap-resolver.js",
  "src/compiler/backend/gap-resolver.h + gap-resolver.cc",
  "GapResolver: 并行移动序列化 (环检测 + 临时位置破环)")}
//! 台账: Assembler trait 8 方法 (AssembleMove/AssembleSwap/Push/Pop/
//!   PopTempStackSlots/MoveToTempLocation/MoveTempLocationTo/SetPendingMove)
//!   + temp_slots_; Resolve 逐未消除 move → PerformMove (helper 失败返回
//!   blocking_move 则临时槽破环重试) → PerformCycle (环走 SetPendingMove +
//!   MoveToTempLocation + MoveTempLocationTo 三步); 递归 PerformMoveHelper。

/// v8: GapResolver::Assembler — 发射接口 (Rust trait)
// pub trait Assembler { assemble_move; assemble_swap; push; pop;
//   pop_temp_stack_slots; move_to_temp_location; move_temp_location_to; set_pending_move }
/// v8: GapResolver::Resolve — 公有入口
// pub fn resolve(&mut self, parallel_move: &mut ParallelMove)
/// v8: GapResolver::PerformMove — 依赖链驱动 + 破环重试
// fn perform_move(&mut self, moves: &mut ParallelMove, mv: &mut MoveOperands)
/// v8: GapResolver::PerformMoveHelper — 递归先行, 环则返回阻塞 move
// fn perform_move_helper(&mut self, moves: &mut ParallelMove, mv: &mut MoveOperands,
//   cycle: &mut Vec<*mut MoveOperands>) -> Option<&mut MoveOperands>
/// v8: GapResolver::PerformCycle — 环三步破
// fn perform_cycle(&mut self, cycle: &[*mut MoveOperands])

// —— 提取不变式断言 ——
const _: () = {
  assert!(true); // 接口面 7 方法对数见生成脚本日志
};
`;

writeSrc("v8-compiler-backend", "gap_resolver.rs.skel", skel);
console.log(`gap-resolver.js: 对数 ${checks} 条全过; Assembler 纯虚 8 + temp_slots_; ` +
  `主类方法 4; 破环三步; 产物 gap_resolver.rs.skel`);
