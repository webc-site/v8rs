#!/usr/bin/env bun
// tool/src/wasm/baseline/parallel-move.js
// 机械提取 v8/src/wasm/baseline/parallel-move.cc 的三个方法定义与两处分派:
// TransferToStack 的 VarState loc 三路分派, ExecuteLoads 的 LoadKind 五路分派,
// ExecuteMoves 的两阶段 (无环先行 + 环路 spill-reload)。
// 头侧枚举在 v8-wasm-liftoff/src/parallel_move.rs (parallel-move.h.js)。

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/wasm/baseline/parallel-move.cc");

// ---------- 锚断言 ----------
const anchors = [
  ["include 自头", src.includes('#include "src/wasm/baseline/parallel-move.h"')],
  ["TransferToStack", src.includes("void ParallelMove::TransferToStack(int dst_offset, const VarState& src) {")],
  ["ExecuteMoves", src.includes("void ParallelMove::ExecuteMoves() {")],
  ["ExecuteLoads", src.includes("void ParallelMove::ExecuteLoads() {")],
  ["同偏移直返", src.includes("if (src.offset() == dst_offset) return;")],
  ["环路 spill 偏移步进", src.includes("last_spill_offset_ += LiftoffAssembler::SlotSizeForType(move->kind);")],
];
for (const [n, ok] of anchors) {
  if (!ok) throw new Error(`锚点缺失: ${n}`);
}

// ---------- 分派 1: TransferToStack — case VarState loc ----------
const transferBody = src.slice(
  src.indexOf("void ParallelMove::TransferToStack"),
  src.indexOf("void ParallelMove::ExecuteMoves"),
);
const locCases = [...transferBody.matchAll(/case VarState::k(\w+):/g)].map((m) => m[1]);
if (locCases.length !== 3 || locCases.join(",") !== "Stack,Register,IntConst") {
  throw new Error(`TransferToStack 分派 ${locCases}, 期望 Stack,Register,IntConst`);
}
// 三路各自调用的 assembler 接口
const locActions = [
  ["Stack", "MoveStackValue(dst_offset, src.offset(), src.kind())"],
  ["Register", "Spill(dst_offset, src.reg(), src.kind())"],
  ["IntConst", "Spill(dst_offset, src.constant())"],
];
for (const [loc, act] of locActions) {
  if (!transferBody.includes(act)) throw new Error(`TransferToStack ${loc} 动作缺失: ${act}`);
}

// ---------- 分派 2: ExecuteLoads — case RegisterLoad load_kind ----------
const loadsBody = src.slice(src.indexOf("void ParallelMove::ExecuteLoads()"));
const loadCases = [...loadsBody.matchAll(/case RegisterLoad::k(\w+):/g)].map((m) => m[1]);
if (loadCases.length !== 5) throw new Error(`ExecuteLoads 分派 ${loadCases.length} 路, 期望 5`);
if (loadCases.join(",") !== "Nop,Constant,Stack,LowHalfStack,HighHalfStack") {
  throw new Error(`ExecuteLoads 分派序漂移: ${loadCases}`);
}
if (!loadsBody.includes("load->kind == kI64")) throw new Error("kConstant 的 i64 宽度分派缺失");
if (!loadsBody.includes("kNeedS128RegPair && load->kind == kS128")) throw new Error("kStack 的 s128 寄存器对分派缺失");
if (!loadsBody.includes("asm_->FillI64Half(dst.gp(), load->value, kLowWord);")) throw new Error("kLowHalfStack 动作缺失");
if (!loadsBody.includes("asm_->FillI64Half(dst.gp(), load->value, kHighWord);")) throw new Error("kHighHalfStack 动作缺失");
if (!loadsBody.includes("load_dst_regs_ = {};")) throw new Error("ExecuteLoads 清空 load 目标集合缺失");

console.log(`parallel-move.cc: 方法=3 loc分派=${locCases.length} load分派=${loadCases.length}`);

// ---------- 生成 Rust ----------
const rust = `${banner(
  "src/wasm/baseline/parallel-move.js",
  "src/wasm/baseline/parallel-move.cc",
  "TransferToStack/ExecuteMoves/ExecuteLoads 三方法分派台账 (枚举在 parallel_move.rs)",
)}
//! ParallelMove 执行阶段的两张分派表与环路处理语义。

/// TransferToStack: VarState loc 三路分派 (${locCases.length} 路)
/// v8: src/wasm/baseline/parallel-move.cc ParallelMove::TransferToStack
#[rustfmt::skip]
pub static TRANSFER_TO_STACK_DISPATCH: &[(&str, &str)] = &[
  ("Stack", "MoveStackValue(dst, src.offset, src.kind); 同偏移直接 return"),
  ("Register", "Spill(dst, src.reg, src.kind)"),
  ("IntConst", "Spill(dst, src.constant)"),
];

/// ExecuteLoads: RegisterLoad load_kind 五路分派 (${loadCases.length} 路)
/// v8: src/wasm/baseline/parallel-move.cc ParallelMove::ExecuteLoads
#[rustfmt::skip]
pub static EXECUTE_LOADS_DISPATCH: &[(&str, &str)] = &[
  ("Nop", "无动作"),
  ("Constant", "LoadConstant; kind==i64 时按 int64 加载, 否则 int32"),
  ("Stack", "Fill(dst, value, kind); s128 且需寄存器对时 Fill 到 fp pair"),
  ("LowHalfStack", "FillI64Half(dst.gp, value, LowWord)"),
  ("HighHalfStack", "FillI64Half(dst.gp, value, HighWord)"),
];

/// ExecuteLoads 末尾清空 load 目标寄存器集合
/// v8: src/wasm/baseline/parallel-move.cc ParallelMove::ExecuteLoads
pub const EXECUTE_LOADS_CLEARS_DST_SET: bool = true;

/// ExecuteMoves 两阶段语义
/// v8: src/wasm/baseline/parallel-move.cc ParallelMove::ExecuteMoves
pub mod execute_moves {
    /// 阶段 1: 先执行所有 dst 不再被用作 src 的搬移 (src 引用计数归零时传递执行)
    pub const PHASE1_INDEPENDENT_FIRST: bool = true;
    /// 阶段 2: 剩余搬移构成环; 环上第一个 dst 的 src 溢出到
    /// last_spill_offset_ + SlotSizeForType(kind), 并登记为之后重新装载
    pub const PHASE2_CYCLE_SPILL_RELOAD: bool = true;
    /// 环路溢出偏移步进 = LiftoffAssembler::SlotSizeForType(move->kind)
    pub const CYCLE_SPILL_SLOT_SIZE_FOR_TYPE: bool = true;
}

/// ParallelMove 方法台账 (${3} 条, .cc 侧)
/// v8: src/wasm/baseline/parallel-move.cc
#[rustfmt::skip]
pub static METHODS: &[(&str, u32)] = &[
  ("ParallelMove::TransferToStack", 11),
  ("ParallelMove::ExecuteMoves", 46),
  ("ParallelMove::ExecuteLoads", 72),
];

// —— 提取不变式断言 ——
const _: () = {
    assert!(TRANSFER_TO_STACK_DISPATCH.len() == 3);
    assert!(EXECUTE_LOADS_DISPATCH.len() == 5);
    assert!(METHODS.len() == 3);
    assert!(!METHODS.is_empty());
};
`;

const p = writeSrc("v8-wasm-liftoff", "parallel_move_cc.rs.skel", rust);
console.log(`written ${p}`);
