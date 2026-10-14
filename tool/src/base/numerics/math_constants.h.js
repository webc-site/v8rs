#!/usr/bin/env bun
// tool/src/base/numerics/math_constants.h.js — 覆盖 v8/src/base/numerics/math_constants.h
// 解析 constexpr 数学常量表 (kMeanGravityDouble/Float)
// → v8-base/src/numerics/math_constants.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/numerics/math_constants.h");

for (const anchor of [
  "V8_BASE_NUMERICS_MATH_CONSTANTS_H_",
  "kMeanGravityDouble",
  "kMeanGravityFloat",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: constexpr <type> <name> = <value>;
const constRe = /constexpr (double|float) (\w+) = ([^;]+);/g;
const consts = [];
for (const m of h.matchAll(constRe)) consts.push({ type: m[1], name: m[2], value: m[3].trim() });
if (consts.length !== 2) throw new Error(`expect 2 consts, got ${consts.length}`);

// Rust 后缀: 9.80665f → 9.80665_f32
const rustVal = (v) => v.replace(/f$/, "_f32").replace(/(?<![\w.])(\d)/, "$1");

const body = `${banner("src/base/numerics/math_constants.h.js", "src/base/numerics/math_constants.h", "数学常量表")}
//! v8 侧常量计数: ${consts.length}

${consts
  .map(
    (c) => `/// 地球平均重力加速度 m/s^2
/// v8: src/base/numerics/math_constants.h ${c.name} = ${c.value}
pub const ${c.name.replace(/^k(\w)/, (_, ch) => ch.toUpperCase()).toUpperCase()}_MEAN_GRAVITY: ${c.type === "double" ? "f64" : "f32"} = ${rustVal(c.value)};`,
  )
  .join("\n\n")}
`;

writeSrc("v8-base", "numerics/math_constants.rs.skel", body);
console.log(`math_constants.h.js: consts=${consts.length} (${consts.map((c) => c.name).join(",")})`);
