#!/usr/bin/env bun
// tool/src/base/numbers/diy-fp.h.js — 族: src/base/numbers/diy-fp.h + src/base/numbers/diy-fp.cc
// 覆盖: DiyFp (uint64 尾数 + int 指数) — Subtract/Minus/Multiply/Times/Normalize + 访问器
// → v8-base/src/numbers/diy_fp.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/numbers/diy-fp.h");
const ccSrc = readV8("src/base/numbers/diy-fp.cc");

// —— 锚点断言 ——
if (!hSrc.includes("class DiyFp {")) throw new Error("缺 DiyFp 类");
if (!hSrc.includes("static const int kSignificandSize = 64;")) throw new Error("缺尾数位宽常量");
if (!hSrc.includes("absl::uint128 mul = a.f_ * absl::uint128(b.f_);")) throw new Error("缺 128 位乘法");
if (!hSrc.includes("return {hi + (lo >> 63), a.e_ + b.e_ + 64};")) throw new Error("缺乘法规格语义");
if (!hSrc.includes("const uint64_t k10MSBits = static_cast<uint64_t>(0x3FF) << 54;")) throw new Error("缺 10 位规格化快路径");
if (!hSrc.includes("while ((f & kUint64MSB) == 0) {")) throw new Error("缺逐位规格化");
if (!hSrc.includes("static const uint64_t kUint64MSB = static_cast<uint64_t>(1) << 63;")) throw new Error("缺 MSB 常量");
// cc 仅 include + 空命名空间 (全部实现内联在 .h): 无任何函数定义
if (/[a-z_]+\s+\w+\([^)]*\)\s*\{/.test(ccSrc.replace(/#[^\n]*/g, ""))) throw new Error("diy-fp.cc 应无函数定义");

// —— 真解析: 方法面 ——
const methods = ["Subtract", "Minus", "Multiply", "Times", "Normalize", "f()", "e()", "set_f", "set_e"];
for (const m of methods) if (!hSrc.includes(m)) throw new Error(`缺方法 ${m}`);
console.log("diy-fp: 方法 9 (减/乘/规格化 + 访问器), 常量 3, cc 空 (全内联)");

let rust = banner(
  "src/base/numbers/diy-fp.h.js",
  "src/base/numbers/diy-fp.h + src/base/numbers/diy-fp.cc",
  "DIY 浮点 (uint64 尾数 × 2^e, 不含 NaN/Inf)",
);

rust += `
//! v8: src/base/numbers/diy-fp.h/.cc

/// v8: diy-fp.h DiyFp
/// 归一化约定: 最高有效位置 1; 乘法/减法结果不归一化
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DiyFp {
    f: u64,
    e: i32,
}

impl DiyFp {
    /// v8: kSignificandSize = 64
    pub const SIGNIFICAND_SIZE: i32 = 64;

    /// v8: diy-fp.h DiyFp(f, e)
    pub const fn new(f: u64, e: i32) -> Self;

    /// v8: diy-fp.h Subtract — 要求 e 相等且 f_ >= other.f_ (双 DCHECK); 结果不归一化
    pub fn subtract(&mut self, other: &DiyFp);

    /// v8: diy-fp.h Minus — a - b
    pub fn minus(a: DiyFp, b: DiyFp) -> DiyFp;

    /// v8: diy-fp.h Times — 128 位乘: 取高 64 位 + 低半最高位进位 (lo>>63),
    ///     指数 a.e + b.e + 64; 结果不归一化
    pub fn times(a: DiyFp, b: DiyFp) -> DiyFp;

    /// v8: diy-fp.h Normalize — 先按 10 位快移 (0x3FF<<54 探测), 再逐位补到 MSB=1
    pub fn normalize(&mut self);

    /// v8: diy-fp.h Normalize(a) — 值版本
    pub fn normalized(a: DiyFp) -> DiyFp;

    /// v8: diy-fp.h f()/e()/set_f/set_e
    pub const fn f(&self) -> u64;
    pub const fn e(&self) -> i32;
    pub const fn set_f(&mut self, v: u64);
    pub const fn set_e(&mut self, v: i32);
}
`;

writeSrc("v8-base", "numbers/diy_fp.rs.skel", rust);
console.log("written v8-base/src/numbers/diy_fp.rs.skel");
