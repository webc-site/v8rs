#!/usr/bin/env bun
// tool/src/objects/heap-number-inl.h.js — 覆盖 v8/src/objects/heap-number-inl.h
// 组→文件映射 (本脚本管辖): 席位表差集 (.h/.tq 布局归既有 tool/src/objects/heap-number.h.js)
// → v8-objects-js-async/src/heap_number.rs
// 机械提取: 构造器双形态 + value/value_as_bits 访问器 + is_the_hole (kHoleNanInt64) + Uninitialized 构造

import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/heap-number-inl.h");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[heap-number-inl.h.js] ${m}`); };

// ── 1. 构造器双形态 (double / Float64) ───────────────────────────────────
must(inl.includes("HeapNumber::HeapNumber(const AllocationWitness& witness, double value)"), "inl 缺 double 构造器");
must(inl.includes("HeapNumber::HeapNumber(const AllocationWitness& witness, Float64 value)"), "inl 缺 Float64 构造器");
must(inl.includes("witness.roots().heap_number_map()"), "inl 缺 heap_number_map 根");

// ── 2. 访问器 (value/value_as_bits 成对) ─────────────────────────────────
for (const s of [
  "double HeapNumber::value() const { return value_.value(); }",
  "void HeapNumber::set_value(double value) {",
  "uint64_t HeapNumber::value_as_bits() const { return value_.value_as_bits(); }",
  "void HeapNumber::set_value_as_bits(uint64_t bits) {",
  "value_.set_value_as_bits(bits);",
]) must(inl.includes(s), `inl 缺 ${s}`);

// ── 3. is_the_hole (位模式比较, 非 NaN 语义比较) ─────────────────────────
must(inl.includes("bool HeapNumber::is_the_hole() const {"), "inl 缺 is_the_hole 声明");
must(inl.includes("return value_as_bits() == kHoleNanInt64;"), "inl 缺 kHoleNanInt64 位比较");
// kHoleNanInt64 值 (globals.h; 非 MIPS #else 分支: upper/lower 皆为 0xFFF7FFFF)
const globals = readV8("src/common/globals.h");
const halves = [...globals.matchAll(/constexpr uint32_t kHoleNanUpper32 = (0x[0-9A-Fa-f]+);\s*\nconstexpr uint32_t kHoleNanLower32 = (0x[0-9A-Fa-f]+);/g)];
must(halves.length === 2, `kHoleNanUpper32/Lower32 分支数 ${halves.length} != 2 (MIPS + 默认)`);
const [, upper, lower] = halves[1]; // 第二组 = #else 非 MIPS 默认分支
must(upper === "0xFFF7FFFF" && lower === "0xFFF7FFFF", `hole NaN 半字失配: ${upper}/${lower}`);
const holeVal = (BigInt(upper) << 32n) | BigInt(lower);
must(holeVal === 0xfff7fffffff7ffffn, `kHoleNanInt64 拼接失配: 0x${holeVal.toString(16)}`);

// ── 4. UninitializedHeapNumber 构造 (0.0 占位) ──────────────────────────
must(inl.includes("UninitializedHeapNumber::UninitializedHeapNumber(\n    const AllocationWitness& witness)"), "inl 缺 Uninitialized 构造器");
must(inl.includes("witness.roots().uninitialized_heap_number_map()"), "inl 缺 uninitialized_heap_number_map 根");
must(inl.includes("value_(0.0)"), "inl 缺 0.0 占位初始化");

let rust = banner("src/objects/heap-number-inl.h.js", "src/objects/heap-number-inl.h",
  "席位表差集: 构造/访问/hole 判定 (.h/.tq 布局归既有 heap-number.h.js)");
rust += `//! HeapNumber -inl 差集: 双精度盒的构造与位级访问。
//!
//! 布局 (value: float64) 归既有 heap-number.h.js; 本文件登记构造器
//! 双形态、位访问与 hole 判定 (位模式比较, NaN 语义不可用)。

/// hole NaN 的位模式 (原版 kHoleNanInt64 = 0xFFF7FFFF_FFF7FFFF, 非 MIPS; 不可用 NaN 语义比较)。
pub const K_HOLE_NAN_INT64: u64 = 0x${holeVal.toString(16).toUpperCase()};

// ── 访问器骨架 (语义就地修订) ──
/// 原版 value() / set_value() (double 语义读写)。
pub fn value() {
  todo!("就地修订: heap-number-inl.h value()/set_value()")
}

/// 原版 value_as_bits() / set_value_as_bits() (位模式读写, 免 NaN 比较陷阱)。
pub fn value_as_bits() {
  todo!("就地修订: heap-number-inl.h value_as_bits()/set_value_as_bits()")
}

/// 原版 is_the_hole() — hole 判定 = 位模式等于 K_HOLE_NAN_INT64。
#[inline]
#[must_use]
pub const fn is_the_hole(bits: u64) -> bool {
  bits == K_HOLE_NAN_INT64
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(is_the_hole(K_HOLE_NAN_INT64));
  assert!(!is_the_hole(0));
  assert!(!is_the_hole(f64::INFINITY.to_bits()));
};
`;

const p = writeSrc("v8-objects-js-async", "heap_number.rs", rust);
console.log(`heap-number-inl.h.js: ${p} — 构造器 3 + 访问器 4 + is_the_hole (kHoleNanInt64=0x${holeVal.toString(16)}), 断言 ${checks} 项全过`);
