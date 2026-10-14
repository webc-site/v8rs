#!/usr/bin/env bun
// tool/src/base/overflowing-math.h.js — 覆盖 v8/src/base/overflowing-math.h
// 解析 OP_WITH_WRAPAROUND X-macro (Add/Sub/Mul + int16 特化)、Negate/Shl/Divide/Recip 族
// → v8-base/src/overflowing_math.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/overflowing-math.h");

for (const anchor of [
  "V8_BASE_OVERFLOWING_MATH_H_",
  "OP_WITH_WRAPAROUND",
  "NegateWithWraparound",
  "ShlWithWraparound",
  "inline T Divide(T x, T y)",
  "RecipSqrt",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: X-macro 实例化行
const wrapOps = [...h.matchAll(/^OP_WITH_WRAPAROUND\((\w+), (\S)\)$/gm)].map((m) => ({ name: m[1], sym: m[2] }));
// 真解析: 其余模板/内联函数
const others = [...h.matchAll(/^inline (?:signed_type|T|float|int16_t) (\w+)\(/gm)].map((m) => m[1]);
// int16 特化
const hasI16Spec = h.includes("inline int16_t MulWithWraparound(int16_t a, int16_t b)");

const body = `${banner("src/base/overflowing-math.h.js", "src/base/overflowing-math.h", "有符号回绕算术与安全除法")}
//! v8 侧: X-macro 算子 ${wrapOps.length} (含 int16 特化 ${hasI16Spec ? 1 : 0}) + 其余函数 ${new Set(others).size}

// ---- OP_WITH_WRAPAROUND X-macro 展开 (unsigned 中转绕 UB) ----
${wrapOps.map((o) => `/// v8: OP_WITH_WRAPAROUND(${o.name}, ${o.sym})
pub fn ${o.name.toLowerCase()}_with_wraparound<T: Copy>(a: T, b: T) -> T { /* 语义: wrapping_${o.name.toLowerCase()} */ todo!() }`).join("\n")}
/// v8: src/base/overflowing-math.h MulWithWraparound<int16_t> 特化
/// C++ 整型提升陷阱 (llvm bug 25580): 提升到 uint32 乘后截断; Rust i16::wrapping_mul 等价
${hasI16Spec ? `pub fn mul_with_wraparound_i16(a: i16, b: i16) -> i16 { a.wrapping_mul(b) }` : ""}

// ---- 其余函数 ----
${[...new Set(others)].filter((n) => !wrapOps.some((o) => o.name === n)).map((n) => `/// v8: src/base/overflowing-math.h ${n}
/// ${n === "NegateWithWraparound" ? "MIN 不变 (避免 -MIN UB), 其余取负" : n === "ShlWithWraparound" ? "移位量 & (位宽-1) 掩码后无符号左移" : n === "Divide" ? "y==0 时 IEEE 语义: x=0/NaN→NaN, 同号→+inf, 异号→-inf" : n === "Recip" ? "1.0f / a (经 Divide)" : "a=0→±inf, 否则 1/sqrt(a)"}
pub fn ${n.replace(/([a-z])([A-Z])/g, "$1_$2").toLowerCase()}<T: Copy>(a: T) -> T { todo!() }`).join("\n")}
`;

writeSrc("v8-base", "overflowing_math.rs.skel", body);
console.log(`overflowing-math.h.js: wrapOps=${wrapOps.length} i16spec=${hasI16Spec} others=${new Set(others).size}`);
