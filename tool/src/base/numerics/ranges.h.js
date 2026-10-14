#!/usr/bin/env bun
// tool/src/base/numerics/ranges.h.js — 覆盖 v8/src/base/numerics/ranges.h
// 解析 IsApproximatelyEqual 模板签名 (abs(rhs-lhs) <= tolerance)
// → v8-base/src/numerics/ranges.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/numerics/ranges.h");

for (const anchor of [
  "V8_BASE_NUMERICS_RANGES_H_",
  "IsApproximatelyEqual",
  "std::abs(rhs - lhs) <= tolerance",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 唯一模板函数签名 + 判定式
const fnRe = /constexpr bool (\w+)\(T (\w+), T (\w+), T (\w+)\)/;
const m = h.match(fnRe);
if (!m) throw new Error("IsApproximatelyEqual signature not found");

const body = `${banner("src/base/numerics/ranges.h.js", "src/base/numerics/ranges.h", "近似相等判定")}
//! v8 侧模板函数 1 个: ${m[1]}

/// |rhs - lhs| <= tolerance
/// v8: src/base/numerics/ranges.h IsApproximatelyEqual(lhs, rhs, tolerance)
/// static_assert: T 必须算术类型 (Rust 用 PartialOrd + Sub 数值 trait 约束)
pub fn is_approximately_equal<T>(lhs: T, rhs: T, tolerance: T) -> bool
where
    T: Copy + PartialOrd + core::ops::Sub<Output = T>,
{
    // 语义: (rhs - lhs).abs() <= tolerance — 浮点/整数各自实现, 就地修订
    todo!()
}
`;

writeSrc("v8-base", "numerics/ranges.rs.skel", body);
console.log(`ranges.h.js: fns=1 (${m[1]})`);
