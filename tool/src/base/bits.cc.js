#!/usr/bin/env bun
// tool/src/base/bits.cc.js — 族: src/base/bits.cc (bits.h 归别席/既有, 本族只管 .cc 实现面)
// 覆盖: 12 个乘除法实现 (Signed/Unsigned MulHigh / Div / Mod / SaturatedAdd/Sub)
// → v8-base/src/bits_cc.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/bits.cc");

// —— 真解析: 抽全部函数签名 (带返回类型), 对数 ——
const sigs = [...src.matchAll(/^(int32_t|int64_t|uint32_t|uint64_t) (\w+)\(([^)]*)\) \{/gm)].map(
  (m) => `${m[1]} ${m[2]}(${m[3].replace(/\s+/g, " ").trim()})`,
);
const expected = [
  "int32_t SignedMulHigh32(int32_t lhs, int32_t rhs)",
  "int64_t SignedMulHigh64(int64_t u, int64_t v)",
  "uint64_t UnsignedMulHigh64(uint64_t u, uint64_t v)",
  "uint32_t UnsignedMulHigh32(uint32_t lhs, uint32_t rhs)",
  "int32_t SignedMulHighAndAdd32(int32_t lhs, int32_t rhs, int32_t acc)",
  "int32_t SignedDiv32(int32_t lhs, int32_t rhs)",
  "int64_t SignedDiv64(int64_t lhs, int64_t rhs)",
  "int32_t SignedMod32(int32_t lhs, int32_t rhs)",
  "int64_t SignedMod64(int64_t lhs, int64_t rhs)",
  "int64_t SignedSaturatedAdd64(int64_t lhs, int64_t rhs)",
  "int64_t SignedSaturatedSub64(int64_t lhs, int64_t rhs)",
];
for (const e of expected) if (!sigs.includes(e)) throw new Error(`bits.cc 缺函数: ${e}`);
if (sigs.length !== 11) throw new Error(`bits.cc 函数总数 ${sigs.length} != 11`);

// 边界语义锚点
if (!src.includes("if (rhs == 0) return 0;")) throw new Error("缺除零语义");
if (!src.includes("if (rhs == -1) return lhs == std::numeric_limits<int32_t>::min() ? lhs : -lhs;")) {
  throw new Error("缺 INT_MIN/-1 语义");
}
console.log(`bits.cc: 函数 ${sigs.length} (MulHigh 4 + Add 1 + Div 2 + Mod 2 + Saturated 2)`);

let rust = banner(
  "src/base/bits.cc.js",
  "src/base/bits.cc",
  "乘除法边角实现 (高半乘积/安全除法/饱和加减)",
);

rust += `
//! v8: src/base/bits.cc — 对应 bits.h 声明 (bits.h 归 base-core 顶层模块)

/// v8: src/base/bits.cc SignedMulHigh32
/// i64 全乘后取高 32 位 (bit_cast 防截断 UB); Rust: ((lhs as i64 * rhs as i64) >> 32) as i32
pub fn signed_mul_high32(lhs: i32, rhs: i32) -> i32;

/// v8: src/base/bits.cc SignedMulHigh64 — Hacker's Delight 8.2 32 位分段乘法
pub fn signed_mul_high64(u: i64, v: i64) -> i64;

/// v8: src/base/bits.cc UnsignedMulHigh64 — Hacker's Delight 8.2 无符号版
pub fn unsigned_mul_high64(u: u64, v: u64) -> u64;

/// v8: src/base/bits.cc UnsignedMulHigh32
pub fn unsigned_mul_high32(lhs: u32, rhs: u32) -> u32;

/// v8: src/base/bits.cc SignedMulHighAndAdd32 — 高 32 位乘积 + acc (回绕加)
pub fn signed_mul_high_and_add32(lhs: i32, rhs: i32, acc: i32) -> i32;

/// v8: src/base/bits.cc SignedDiv32 — rhs==0 返回 0; rhs==-1 且 lhs==INT_MIN 返回 lhs (防溢出)
pub fn signed_div32(lhs: i32, rhs: i32) -> i32;

/// v8: src/base/bits.cc SignedDiv64 — 同 SignedDiv32 语义的 64 位版
pub fn signed_div64(lhs: i64, rhs: i64) -> i64;

/// v8: src/base/bits.cc SignedMod32 — rhs==0 或 rhs==-1 返回 0
pub fn signed_mod32(lhs: i32, rhs: i32) -> i32;

/// v8: src/base/bits.cc SignedMod64
pub fn signed_mod64(lhs: i64, rhs: i64) -> i64;

/// v8: src/base/bits.cc SignedSaturatedAdd64 — 下溢取 min / 上溢取 max
pub fn signed_saturated_add64(lhs: i64, rhs: i64) -> i64;

/// v8: src/base/bits.cc SignedSaturatedSub64
pub fn signed_saturated_sub64(lhs: i64, rhs: i64) -> i64;
`;

// 产物名 bits_cc: bits.h (别席) 产出 bits.rs.skel, 本 .cc 族错开命名防互覆
writeSrc("v8-base", "bits_cc.rs.skel", rust);
console.log("written v8-base/src/bits_cc.rs.skel");
