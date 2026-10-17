#!/usr/bin/env bun
// tool/src/base/division-by-constant.h.js — 族: src/base/division-by-constant.h + .cc
// 覆盖: MagicNumbersForDivision 结构 + Signed/UnsignedDivisionByConstant (32/64 位特化)
// → v8-base/src/division_by_constant.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/base/division-by-constant.h");
const ccSrc = readV8("src/base/division-by-constant.cc");

// —— 锚点断言 ——
if (!hSrc.includes("struct EXPORT_TEMPLATE_DECLARE(V8_BASE_EXPORT) MagicNumbersForDivision {")) {
  throw new Error("缺 MagicNumbersForDivision");
}
for (const f of ["T multiplier;", "unsigned shift;", "bool add;"]) {
  if (!hSrc.includes(f)) throw new Error(`缺字段 ${f}`);
}
if (!hSrc.includes("requires std::unsigned_integral<T>")) throw new Error("缺 unsigned 版 requires");
if (!hSrc.includes("requires std::signed_integral<T>")) throw new Error("缺 signed 包装版 requires");
if (!ccSrc.includes("DCHECK(d != static_cast<T>(-1) && d != 0 && d != 1);")) throw new Error("缺 signed 除数前置");
if (!ccSrc.includes("DCHECK_NE(d, 0);")) throw new Error("缺 unsigned 除数前置");
if (!ccSrc.includes("return MagicNumbersForDivision<T>(q2 + 1, p - bits, a);")) throw new Error("缺 unsigned 返回式");

// —— 真解析: 显式实例化对数 (cc 用 EXPORT_TEMPLATE_DEFINE) ——
const inst = [...ccSrc.matchAll(/template struct EXPORT_TEMPLATE_DEFINE\(V8_BASE_EXPORT\)\s*\n\s*MagicNumbersForDivision<(u?int\d+_t)>;/g)].map((m) => m[1]);
const fnInst = [...ccSrc.matchAll(/template EXPORT_TEMPLATE_DEFINE\(V8_BASE_EXPORT\)\s*\n\s*MagicNumbersForDivision<(u?int\d+_t)> (Signed|Unsigned)DivisionByConstant\(/g)].map((m) => `${m[2]}:${m[1]}`);
if (inst.join(",") !== "uint32_t,uint64_t") throw new Error(`结构实例化 ${inst}`);
if (fnInst.join(",") !== "Signed:uint32_t,Signed:uint64_t,Unsigned:uint32_t,Unsigned:uint64_t") {
  throw new Error(`函数实例化 ${fnInst}`);
}
console.log(`division-by-constant: 结构 1, 函数 2 (signed u32/u64 + unsigned u32/u64 共 4 实例化)`);

let rust = banner(
  "src/base/division-by-constant.h.js",
  "src/base/division-by-constant.h + src/base/division-by-constant.cc",
  "乘法代除法魔数 (Hacker's Delight ch.10)",
);

rust += `
//! v8: src/base/division-by-constant.h/.cc

/// v8: src/base/division-by-constant.h MagicNumbersForDivision<T>
/// 三元组 (multiplier, shift, add): n / d ≈ 高位乘 mul 后 >> shift (+1 if add)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MagicNumbersForDivision<T> {
    pub multiplier: T,
    pub shift: u32,
    pub add: bool,
}

/// v8: src/base/division-by-constant.cc SignedDivisionByConstant (unsigned T 核心)
/// 前置: d ∉ {-1, 0, 1} (按有符号解释); 返回 (neg ? -mul : mul, p - bits, false)
pub fn signed_division_by_constant_u32(d: u32) -> MagicNumbersForDivision<u32>;
/// v8: 同上 uint64_t 实例化
pub fn signed_division_by_constant_u64(d: u64) -> MagicNumbersForDivision<u64>;

/// v8: src/base/division-by-constant.h SignedDivisionByConstant (signed T 包装)
/// 转 unsigned 求魔数再 cast 回 T
pub fn signed_division_by_constant_i32(d: i32) -> MagicNumbersForDivision<i32>;
pub fn signed_division_by_constant_i64(d: i64) -> MagicNumbersForDivision<i64>;

/// v8: src/base/division-by-constant.cc UnsignedDivisionByConstant
/// 前置: d != 0; leading_zeros 已知 dividend 高位零个数可加速;
/// 返回 (q2 + 1, p - bits, a)
pub fn unsigned_division_by_constant_u32(d: u32, leading_zeros: u32) -> MagicNumbersForDivision<u32>;
pub fn unsigned_division_by_constant_u64(d: u64, leading_zeros: u32) -> MagicNumbersForDivision<u64>;
`;

writeSrc("v8-base", "division_by_constant.rs.skel", rust);
console.log("written v8-base/src/division_by_constant.rs.skel");
