#!/usr/bin/env bun
// tool/src/base/numerics/byte_conversions.h.js — 覆盖 v8/src/base/numerics/byte_conversions.h + byte_conversions_unittest.cc
// 解析 {U,I}{8,16,32,64}+Float+Double × {From,To}{Native,Little,Big}Endian 全函数矩阵
// → v8-base/src/numerics/byte_conversions.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/numerics/byte_conversions.h");
const ut = readV8("src/base/numerics/byte_conversions_unittest.cc");

for (const anchor of [
  "V8_BASE_NUMERICS_BYTE_CONVERSIONS_H_",
  "ByteSwap",
  "FromNativeEndian",
  "FromLittleEndian",
  "FromBigEndian",
  "ToNativeEndian",
  "ToLittleEndian",
  "ToBigEndian",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: inline constexpr 函数签名 → (方向, 端序, 类型) 矩阵
const fnRe = /inline constexpr ([\w:<>, ]+?) (\w+?)From(Native|Little|Big)Endian\(/g;
const fromFns = [...h.matchAll(fnRe)].map((m) => ({ type: m[2], order: m[3] }));
const toFnRe = /inline constexpr std::array<uint8_t, (\d+)u> (\w+?)To(Native|Little|Big)Endian\(/g;
const toFns = [...h.matchAll(toFnRe)].map((m) => ({ type: m[2], order: m[3], nbytes: m[1] }));
if (fromFns.length === 0 || toFns.length === 0) throw new Error("matrix empty");

const types = [...new Set([...fromFns.map((f) => f.type), ...toFns.map((f) => f.type)])];
const orders = ["Native", "Little", "Big"];
// unittest 对数
const testCount = (ut.match(/^TEST\(/gm) || []).length;

// 类型 → Rust 类型/字节数
const rustType = { U8: "u8", U16: "u16", U32: "u32", U64: "u64", I8: "i8", I16: "i16", I32: "i32", I64: "i64", Float: "f32", Double: "f64" };
const typeBytes = { U8: 1, U16: 2, U32: 4, U64: 8, I8: 1, I16: 2, I32: 4, I64: 8, Float: 4, Double: 8 };

const body = `${banner("src/base/numerics/byte_conversions.h.js", "src/base/numerics/byte_conversions.h + byte_conversions_unittest.cc", "字节序换算 API 矩阵 (10 类型 × 3 端序 × 双向)")}
//! v8 侧 From*Endian ${fromFns.length} 个 + To*Endian ${toFns.length} 个 = ${fromFns.length + toFns.length} 函数
//! 类型 ${types.length} 种: ${types.join(", ")}; 端序 ${orders.length} 种: ${orders.join("/")}
//! unittest: ${testCount} 个 TEST 用例 (迁移对数基准)
//! Rust 策略: Native → to_ne_bytes; Little → to_le_bytes; Big → to_be_bytes; Float/Double 经 bitcast

${types
  .map((t) => {
    const rt = rustType[t];
    const n = typeBytes[t];
    const snake = t.toLowerCase();
    return `// ---- ${t} (${rt}, ${n} 字节) ----
/// v8: src/base/numerics/byte_conversions.h ${t}FromNativeEndian
pub fn ${snake}_from_native_endian(bytes: [u8; ${n}]) -> ${rt} { ${rt}::from_ne_bytes(bytes) }
/// v8: src/base/numerics/byte_conversions.h ${t}FromLittleEndian
pub fn ${snake}_from_little_endian(bytes: [u8; ${n}]) -> ${rt} { ${rt}::from_le_bytes(bytes) }
/// v8: src/base/numerics/byte_conversions.h ${t}FromBigEndian
pub fn ${snake}_from_big_endian(bytes: [u8; ${n}]) -> ${rt} { ${rt}::from_be_bytes(bytes) }
/// v8: src/base/numerics/byte_conversions.h ${t}ToNativeEndian
pub fn ${snake}_to_native_endian(val: ${rt}) -> [u8; ${n}] { val.to_ne_bytes() }
/// v8: src/base/numerics/byte_conversions.h ${t}ToLittleEndian
pub fn ${snake}_to_little_endian(val: ${rt}) -> [u8; ${n}] { val.to_le_bytes() }
/// v8: src/base/numerics/byte_conversions.h ${t}ToBigEndian
pub fn ${snake}_to_big_endian(val: ${rt}) -> [u8; ${n}] { val.to_be_bytes() }
/// Float/Double 走 bitcast (f32 ↔ u32, f64 ↔ u64) 再换端序 — Developer 补
`;
  })
  .join("\n\n")}
`;

writeSrc("v8-base", "numerics/byte_conversions.rs.skel", body);
console.log(
  `byte_conversions.h.js: from=${fromFns.length} to=${toFns.length} types=${types.length} orders=3 tests=${testCount}`,
);
