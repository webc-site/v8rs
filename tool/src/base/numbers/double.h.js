#!/usr/bin/env bun
// tool/src/base/numbers/double.h.js — 族: src/base/numbers/double.h
// 覆盖: double_to_uint64/uint64_to_double + Double 类 (位级操作) — 常量 10 + 方法 17
// → v8-base/src/numbers/double.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/base/numbers/double.h");

// —— 锚点断言 ——
if (!src.includes("inline constexpr uint64_t double_to_uint64(double d) {")) throw new Error("缺位转换");
if (!src.includes("inline constexpr double uint64_to_double(uint64_t d64) {")) throw new Error("缺逆转换");
if (!src.includes("class Double {")) throw new Error("缺 Double 类");
if (!src.includes("while ((f & kHiddenBit) == 0) {")) throw new Error("缺 denormal 规格化循环");

// —— 真解析: 常量表 (公有 6 + 私有 4), 对数 ——
const pubConsts = {
  kSignMask: "0x8000000000000000",
  kExponentMask: "0x7FF0000000000000",
  kSignificandMask: "0x000FFFFFFFFFFFFF",
  kHiddenBit: "0x0010000000000000",
};
for (const [name, val] of Object.entries(pubConsts)) {
  const m = src.match(new RegExp(`static constexpr uint64_t ${name} = (0x[\\dA-F']+);`));
  if (!m || m[1].replace(/'/g, "") !== val) throw new Error(`常量 ${name} 异常`);
}
if (!src.includes("kPhysicalSignificandSize =")) throw new Error("缺物理尾数位宽");
if (!src.includes("kSignificandSize = 53;")) throw new Error("缺尾数位宽");
if (!src.includes("static constexpr int kExponentBias = 0x3FF + kPhysicalSignificandSize;")) throw new Error("缺指数偏置");
if (!src.includes("static constexpr int kDenormalExponent = -kExponentBias + 1;")) throw new Error("缺 denormal 指数");
if (!src.includes("static constexpr uint64_t kInfinity = 0x7FF0'0000'0000'0000;")) throw new Error("缺 Inf 位型");

// 方法面对数
const methods = ["AsDiyFp", "AsNormalizedDiyFp", "AsUint64", "NextDouble", "Exponent", "Significand",
  "IsDenormal", "IsSpecial", "IsInfinite", "Sign", "UpperBoundary", "NormalizedBoundaries", "value",
  "SignificandSizeForOrderOfMagnitude", "DiyFpToUint64"];
for (const m of methods) if (!src.includes(m)) throw new Error(`缺方法 ${m}`);
console.log(`double: 转换 2, 类 1, 方法 ${methods.length}, 常量 10`);

let rust = banner(
  "src/base/numbers/double.h.js",
  "src/base/numbers/double.h",
  "double 位级工具 (dtoa 族基础设施)",
);

rust += `
//! v8: src/base/numbers/double.h
//! 假设: double 与 u64 同端序 (Rust: f64::to_bits/from_bits 保证)

/// v8: double.h double_to_uint64 / uint64_to_double — bit_cast 双向
pub const fn double_to_uint64(d: f64) -> u64;
pub const fn uint64_to_double(d64: u64) -> f64;

// —— v8: 公有常量 ——
pub const SIGN_MASK: u64 = 0x8000_0000_0000_0000;          // v8: kSignMask
pub const EXPONENT_MASK: u64 = 0x7FF0_0000_0000_0000;      // v8: kExponentMask
pub const SIGNIFICAND_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;   // v8: kSignificandMask
pub const HIDDEN_BIT: u64 = 0x0010_0000_0000_0000;         // v8: kHiddenBit
/// v8: kPhysicalSignificandSize — 不含隐藏位
pub const PHYSICAL_SIGNIFICAND_SIZE: i32 = 52;
/// v8: kSignificandSize — 含隐藏位
pub const SIGNIFICAND_SIZE: i32 = 53;
// —— v8: 私有常量 ——
pub const EXPONENT_BIAS: i32 = 0x3FF + PHYSICAL_SIGNIFICAND_SIZE; // v8: kExponentBias
pub const DENORMAL_EXPONENT: i32 = -EXPONENT_BIAS + 1;           // v8: kDenormalExponent
pub const MAX_EXPONENT: i32 = 0x7FF - EXPONENT_BIAS;             // v8: kMaxExponent
pub const INFINITY_BITS: u64 = 0x7FF0_0000_0000_0000;            // v8: kInfinity

/// v8: double.h Double — d64_ 位型容器
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Double { d64: u64 }

impl Double {
    /// v8: Double(double) / Double(uint64_t) / Double(DiyFp)
    pub const fn from_f64(d: f64) -> Self;
    pub const fn from_bits(d64: u64) -> Self;
    pub const fn from_diy_fp(diy_fp: DiyFp) -> Self;

    /// v8: AsDiyFp — 要求 ≥ +0.0 且非特殊值
    pub fn as_diy_fp(&self) -> DiyFp;
    /// v8: AsNormalizedDiyFp — denormal 循环左移到隐藏位, 再对齐 64 位尾数
    pub fn as_normalized_diy_fp(&self) -> DiyFp;
    /// v8: AsUint64
    pub const fn as_uint64(&self) -> u64;
    /// v8: NextDouble — ULP 步进; +Inf 进 +Inf; -0.0 → 0.0; 负数按位减
    pub const fn next_double(&self) -> f64;
    /// v8: Exponent — denormal 恒 kDenormalExponent, 否则 biased - bias
    pub const fn exponent(&self) -> i32;
    /// v8: Significand — 非 denormal 补隐藏位
    pub const fn significand(&self) -> u64;
    /// v8: IsDenormal — 指数域全 0
    pub const fn is_denormal(&self) -> bool;
    /// v8: IsSpecial — 指数域全 1 (仅 Inf/NaN; denormal 不算特殊)
    pub const fn is_special(&self) -> bool;
    /// v8: IsInfinite — 指数全 1 且尾数 0
    pub const fn is_infinite(&self) -> bool;
    /// v8: Sign — 符号位 0→1, 1→-1
    pub const fn sign(&self) -> i32;
    /// v8: UpperBoundary — DiyFp(Significand()*2+1, Exponent()-1)
    pub fn upper_boundary(&self) -> DiyFp;
    /// v8: NormalizedBoundaries — m_plus 归一; 尾数==0 且非 denormal 最小值时
    ///     m_minus 取 (f<<2)-1 (边界减半距离), 否则 (f<<1)-1; m_minus 对齐到 m_plus.e
    pub fn normalized_boundaries(&self) -> (DiyFp, DiyFp);
    /// v8: value
    pub const fn value(&self) -> f64;
    /// v8: SignificandSizeForOrderOfMagnitude — denormal 有效位收缩
    pub fn significand_size_for_order_of_magnitude(order: i32) -> i32;

    /// v8: DiyFpToUint64 — 溢上界→kInfinity, 溢下界→0; denormal 移位装填;
    ///     恰 kDenormalExponent 且无隐藏位时指数域置 0
    fn diy_fp_to_uint64(diy_fp: DiyFp) -> u64;
}
`;

writeSrc("v8-base", "numbers/double.rs.skel", rust);
console.log("written v8-base/src/numbers/double.rs.skel");
