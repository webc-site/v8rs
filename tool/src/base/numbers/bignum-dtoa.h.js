#!/usr/bin/env bun
// tool/src/base/numbers/bignum-dtoa.h.js — 族: src/base/numbers/bignum-dtoa.h + .cc
// 覆盖: BignumDtoaMode 枚举 (3) + BignumDtoa 入口 + 10 个静态内部函数
// → v8-base/src/numbers/bignum_dtoa.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/numbers/bignum-dtoa.h");
const ccSrc = readV8("src/base/numbers/bignum-dtoa.cc");

// —— 枚举真解析 ——
const enumBody = hSrc.match(/enum BignumDtoaMode \{([\s\S]*?)\};/)[1];
const modes = [...enumBody.matchAll(/^\s{2}(BIGNUM_DTOA_\w+),?$/gm)].map((m) => m[1]);
if (modes.join(",") !== "BIGNUM_DTOA_SHORTEST,BIGNUM_DTOA_FIXED,BIGNUM_DTOA_PRECISION") {
  throw new Error(`模式枚举 ${modes}`);
}

// —— 锚点断言 ——
if (!hSrc.includes("V8_BASE_EXPORT void BignumDtoa(double v, BignumDtoaMode mode,")) throw new Error("缺入口声明");
if (!hSrc.includes("// The result should be interpreted as buffer * 10^(point-length).")) throw new Error("缺结果语义注释");
if (!ccSrc.includes("void BignumDtoa(double v, BignumDtoaMode mode, int requested_digits,")) throw new Error("缺入口定义");

// —— 真解析: 静态内部函数对数 ——
const statics = [...ccSrc.matchAll(/^static (?:void|int) (\w+)\(/gm)].map((m) => m[1]);
const expected = ["NormalizedExponent", "EstimatePower", "InitialScaledStartValues", "FixupMultiply10",
  "GenerateShortestDigits", "BignumToFixed", "GenerateCountedDigits",
  "InitialScaledStartValuesPositiveExponent",
  "InitialScaledStartValuesNegativeExponentPositivePower",
  "InitialScaledStartValuesNegativeExponentNegativePower"];
for (const e of expected) if (!statics.includes(e)) throw new Error(`cc 缺静态函数 ${e}`);
const modeCases = (ccSrc.match(/^    case BIGNUM_DTOA_\w+:/gm) || []).length;
if (modeCases !== 3) throw new Error(`模式分派 ${modeCases} != 3`);
console.log(`bignum-dtoa: 模式 3, 入口 1, 静态内核 ${statics.length}`);

let rust = banner(
  "src/base/numbers/bignum-dtoa.h.js",
  "src/base/numbers/bignum-dtoa.h + src/base/numbers/bignum-dtoa.cc",
  "Bignum 精确 double→ASCII",
);

rust += `
//! v8: src/base/numbers/bignum-dtoa.h/.cc

/// v8: bignum-dtoa.h BignumDtoaMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BignumDtoaMode {
    /// 最短正确表示 (读回恒等 v; 等距时进位); requested_digits 忽略
    Shortest,
    /// 定点小数位: requested_digits 位小数; 可能偏短, 调用方补 '0'; 半数进位
    Fixed,
    /// 定精度: requested_digits 位有效数字 (首位非 0); 可能偏短; 半数进位
    Precision,
}

/// v8: bignum-dtoa.cc BignumDtoa(v, mode, requested_digits, buffer) → (length, point)
/// 结果语义: value = buffer * 10^(point - length); 输入须 > 0 且非 NaN/Inf
/// 流程: Normalize → EstimatePower (估 10 的幂) → InitialScaledStartValues
///       (正/负指数×正/负幂 四路) → 按 mode 分派 → FixupMultiply10 (奇偶补 10)
pub fn bignum_dtoa(v: f64, mode: BignumDtoaMode, requested_digits: i32) -> (Vec<u8>, i32);

// —— v8: 内核函数 (静态, Developer 补语义) ——
// NormalizedExponent(significand, exponent) — 规格化二进制指数
// EstimatePower(exponent) — 估十进制幂 (kEstimatedPowerFactor 折半)
// InitialScaledStartValues{PositiveExponent,NegativeExponentPositivePower,NegativeExponentNegativePower}
//   — 分子分母初始标定 (Bignum 组装)
// GenerateShortestDigits — SHORTEST: 逐位除法 + Weeding 区间判定
// GenerateCountedDigits — PRECISION: 计数位生成
// BignumToFixed — FIXED: 定点小数位生成
// FixupMultiply10 — estimated_power 奇偶校正 (×10 与指数回写)
`;

writeSrc("v8-base", "numbers/bignum_dtoa.rs.skel", rust);
console.log("written v8-base/src/numbers/bignum_dtoa.rs.skel");
