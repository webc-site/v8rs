#!/usr/bin/env bun
// tool/src/base/numerics/angle_conversions.h.js — 覆盖 v8/src/base/numerics/angle_conversions.h
// 解析 DegToRad/RadToDeg 模板签名与 requires floating_point 约束、pi_v/180 换算式
// → v8-base/src/numerics/angle_conversions.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/numerics/angle_conversions.h");

// 锚点断言: 缺了说明 v8 侧结构变了, 规则失效
for (const anchor of [
  "V8_BASE_NUMERICS_ANGLE_CONVERSIONS_H_",
  "DegToRad",
  "RadToDeg",
  "std::numbers::pi_v<T>",
  "std::floating_point<T>",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 抓 template 函数签名 (name + body 里的换算表达式)
const fnRe =
  /constexpr T (\w+)\(T (\w+)\) \{\s*return \w+ ([^;]+);\s*\}/g;
const fns = [];
for (const m of h.matchAll(fnRe)) fns.push({ name: m[1], param: m[2], expr: m[3].trim() });
if (fns.length !== 2) throw new Error(`expect 2 fns, got ${fns.length}`);

// Rust 骨架: 泛型浮点换算, 语义就地修订
const body = `${banner("src/base/numerics/angle_conversions.h.js", "src/base/numerics/angle_conversions.h", "角度/弧度换算规则面")}
//! v8 侧 2 个模板函数, 解析计数: ${fns.length}

use core::ops::{Div, Mul};

/// 度 → 弧度: deg * pi / 180
/// v8: src/base/numerics/angle_conversions.h DegToRad
pub fn deg_to_rad<T>(deg: T) -> T
where
    T: Copy + Mul<Output = T> + Div<Output = T> + PartialEq,
{
    // 语义: deg * std::f64::consts::PI / 180.0 (按 T 特化)
    /// v8: expr = ${fns[0].expr}
    todo!()
}

/// 弧度 → 度: rad * 180 / pi
/// v8: src/base/numerics/angle_conversions.h RadToDeg
pub fn rad_to_deg<T>(rad: T) -> T
where
    T: Copy + Mul<Output = T> + Div<Output = T> + PartialEq,
{
    // 语义: rad * 180.0 / std::f64::consts::PI (按 T 特化)
    /// v8: expr = ${fns[1].expr}
    todo!()
}
`;

writeSrc("v8-base", "numerics/angle_conversions.rs.skel", body);
console.log(`angle_conversions.h.js: fns=${fns.length} (${fns.map((f) => f.name).join(",")})`);
