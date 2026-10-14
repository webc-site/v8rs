#!/usr/bin/env bun
// tool/src/base/numerics/wrapping_math.h.js — 覆盖 v8/src/base/numerics/wrapping_math.h
// 解析 WrappingAdd/WrappingSub 模板签名与 unsigned 中转实现策略
// → v8-base/src/numerics/wrapping_math.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/numerics/wrapping_math.h");

for (const anchor of [
  "V8_BASE_NUMERICS_WRAPPING_MATH_H_",
  "WrappingAdd",
  "WrappingSub",
  "std::make_unsigned_t<T>",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 两个 inline constexpr 模板函数
const fnRe = /inline constexpr T (\w+)\(T (\w+), T (\w+)\)/g;
const fns = [...h.matchAll(fnRe)].map((m) => ({ name: m[1], a: m[2], b: m[3] }));
if (fns.length !== 2) throw new Error(`expect 2 fns, got ${fns.length}`);

// Rust: 原生 wrapping_add/wrapping_sub; 保留 v8 的 unsigned 中转语义说明
const body = `${banner("src/base/numerics/wrapping_math.h.js", "src/base/numerics/wrapping_math.h", "模 2^N 回绕加减")}
//! v8 侧模板函数 ${fns.length} 个: ${fns.map((f) => f.name).join(", ")}

/// a + b 按模 2^N 回绕 (N = T 位宽)
/// v8: src/base/numerics/wrapping_math.h WrappingAdd(a, b)
/// v8 实现策略: 转无符号相加再转回 (绕开有符号 UB); Rust 用 wrapping_add 无 UB
/// 注意: T 小于 int 时 C++ 整型提升, WrappingMul 需复杂实现 (v8 原注)
pub fn wrapping_add<T>(a: T, b: T) -> T
where
    T: Copy + core::ops::WrappingAdd<Output = T>,
{
    a.wrapping_add(&b)
}

/// a - b 按模 2^N 回绕 (N = T 位宽)
/// v8: src/base/numerics/wrapping_math.h WrappingSub(a, b)
/// v8 实现策略: 转无符号相减再转回; Rust 用 wrapping_sub 无 UB
pub fn wrapping_sub<T>(a: T, b: T) -> T
where
    T: Copy + core::ops::WrappingSub<Output = T>,
{
    a.wrapping_sub(&b)
}
`;

writeSrc("v8-base", "numerics/wrapping_math.rs.skel", body);
console.log(`wrapping_math.h.js: fns=${fns.length} (${fns.map((f) => f.name).join(",")})`);
