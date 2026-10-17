#!/usr/bin/env bun
// tool/src/wasm/baseline/parallel-move-inl.js
// 机械提取 v8/src/wasm/baseline/parallel-move-inl.h 的内联构造规则:
// ParallelMove 构造初始化列表 (asm_ 绑定 + last_spill_offset_ 取 TopSpillOffset)。
// 头侧结构 (RegisterMove/LoadKind) 在 v8-wasm-liftoff/src/parallel_move.rs。

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/wasm/baseline/parallel-move-inl.h");

// ---------- 锚断言 ----------
const anchors = [
  ["include 自头", src.includes('#include "src/wasm/baseline/parallel-move.h"')],
  ["include liftoff-assembler-inl", src.includes('#include "src/wasm/baseline/liftoff-assembler-inl.h"')],
  ["构造定义", src.includes("ParallelMove::ParallelMove(LiftoffAssembler* wasm_asm)")],
  ["asm_ 绑定", src.includes(": asm_(wasm_asm), last_spill_offset_(asm_->TopSpillOffset()) {}")],
];
for (const [n, ok] of anchors) {
  if (!ok) throw new Error(`锚点缺失: ${n}`);
}

// ---------- 提取构造初始化成员 ----------
const initRe = /ParallelMove::ParallelMove\(LiftoffAssembler\* wasm_asm\)\s*\n?\s*:\s*([^{]+)\{\}/;
const m = src.match(initRe);
if (!m) throw new Error("未匹配到构造初始化列表");
const members = m[1].split(",").map((s) => s.trim()).filter(Boolean);
if (members.length !== 2) throw new Error(`初始化成员 ${members.length} 条, 期望 2`);
const topSpill = members[1].includes("asm_->TopSpillOffset()");
if (!topSpill) throw new Error("last_spill_offset_ 未取 TopSpillOffset");

console.log(`parallel-move-inl: 构造初始化成员=${members.length}`);

// ---------- 生成 Rust ----------
const rust = `${banner(
  "src/wasm/baseline/parallel-move-inl.js",
  "src/wasm/baseline/parallel-move-inl.h",
  "ParallelMove 构造初始化规则 (结构在 parallel_move.rs)",
)}
//! ParallelMove 的内联构造: 绑定 LiftoffAssembler, 初始 spill 偏移取当前栈顶。

/// 初始 spill 偏移 = 构造时刻 assembler 的 TopSpillOffset (不归零, 接续当前帧)
/// v8: src/wasm/baseline/parallel-move-inl.h ParallelMove::ParallelMove
pub const INITIAL_SPILL_OFFSET_FROM_TOP: bool = true;

/// ParallelMove 构造骨架 (对应唯一一条内联定义)
/// v8: src/wasm/baseline/parallel-move-inl.h ParallelMove::ParallelMove
pub fn parallel_move_new<TopSpillOffsetFn>(
    _wasm_asm: (),
    _top_spill_offset: TopSpillOffsetFn,
) -> ParallelMoveCtor<TopSpillOffsetFn>
where
    TopSpillOffsetFn: Fn() -> u32,
{
    ParallelMoveCtor {
        asm_bound: true,
        last_spill_offset_source: SpillOffsetSource::TopSpillOffset,
        _top_spill_offset,
    }
}

/// 构造产物台账 (骨架; 真实实现待 Developer 参照原版修订)
pub struct ParallelMoveCtor<F> {
    pub asm_bound: bool,
    pub last_spill_offset_source: SpillOffsetSource,
    _top_spill_offset: F,
}

/// last_spill_offset_ 的初值来源
pub enum SpillOffsetSource {
    /// asm_->TopSpillOffset()
    TopSpillOffset,
}

// —— 提取不变式断言 ——
const _: () = {
    assert!(INITIAL_SPILL_OFFSET_FROM_TOP);
};
`;

const p = writeSrc("v8-wasm-liftoff", "parallel_move_inl.rs.skel", rust);
console.log(`written ${p}`);
