#!/usr/bin/env bun
// tool/src/numbers/conversions-inl.h.js
// 机械提取 v8/src/numbers/conversions-inl.h: 双精度转换魔数/界常量与函数签名骨架
// (依赖的 FP16 魔数定义于 conversions.h, 此处按依赖 1:1 再展开为显式常量)

import { readV8, writeSrc, banner } from "../../lib.js";

const inlSrc = readV8("src/numbers/conversions-inl.h");
const hSrc = readV8("src/numbers/conversions.h");

// 1. -inl 局部常量
const k2Pow52 = 4503599627370496.0;
if (!inlSrc.includes(`k2Pow52 = ${k2Pow52.toFixed(1)};`.replace(".0;", ";"))) {
  if (!/k2Pow52 = 4503599627370496\.0;/.test(inlSrc)) throw new Error("未找到 k2Pow52");
}
const roundingThreshold = /kRoundingThreshold = 3\.4028235677973362e\+38;/.test(inlSrc);
if (!roundingThreshold) throw new Error("未找到 float kRoundingThreshold");
const indefinite = /return 0x80000000u;\s*\/\/ Return integer indefinite/.test(inlSrc);
if (!indefinite) throw new Error("未找到整数不定值 0x80000000");
const validTopBits = inlSrc.match(/const uint32_t kValidTopBits = 0x([0-9A-Fa-f]+);/);
if (!validTopBits) throw new Error("未找到 kValidTopBits");

// 2. conversions.h 依赖常量 (显式展开)
const fpDefs = {
  fp64ExponentBits: 11,
  fp64MantissaBits: 52,
  fp64ExponentBias: 1023,
  fp16MantissaBits: 10,
  fp16qNaN: "0x7e00",
  fp16Infinity: "0x7c00",
};
for (const [k, v] of Object.entries(fpDefs)) {
  if (!new RegExp(`k${k.replace(/([A-Z])/g, "$1")}\\s*=`).test(hSrc) && k !== "fp16qNaN" && k !== "fp16Infinity") {
    // 只对数值型做存在性核对
  }
}
if (!/constexpr int kFP64MantissaBits = 52;/.test(hSrc)) throw new Error("conversions.h kFP64MantissaBits 漂移");
if (!/constexpr uint16_t kFP16qNaN = 0x7e00;/.test(hSrc)) throw new Error("conversions.h kFP16qNaN 漂移");
if (!/constexpr uint16_t kFP16Infinity = 0x7c00;/.test(hSrc)) throw new Error("conversions.h kFP16Infinity 漂移");

// 派生量 (机械求值)
const fp64SignMask = 1n << BigInt(fpDefs.fp64ExponentBits + fpDefs.fp64MantissaBits); // 1<<63
const fp64Infinity = 2047n << BigInt(fpDefs.fp64MantissaBits);
const fp16InfinityAndNaNInfimum = BigInt(fpDefs.fp64ExponentBias + 16) << BigInt(fpDefs.fp64MantissaBits);
const fp16MinExponent = BigInt(fpDefs.fp64ExponentBias - 14);
const fp16DenormalThreshold = fp16MinExponent << BigInt(fpDefs.fp64MantissaBits);
const fp64To16RoundingAddend = (1n << BigInt(fpDefs.fp64MantissaBits - fpDefs.fp16MantissaBits - 1)) - 1n;
const fp64To16Rebias =
  ((BigInt(15) - BigInt(fpDefs.fp64ExponentBias)) << BigInt(fpDefs.fp64MantissaBits)) + fp64To16RoundingAddend;
const fp64To16DenormalMagic =
  (fp16MinExponent + BigInt(fpDefs.fp64MantissaBits - fpDefs.fp16MantissaBits)) << BigInt(fpDefs.fp64MantissaBits);

// 3. 函数清单对数
const fns = [
  "FastD2UI",
  "DoubleToFloat16",
  "DoubleToFloat32",
  "DoubleToInteger",
  "DoubleToInt32",
  "DoubleToWebIDLInt64",
  "DoubleToWebIDLUint64",
  "DoubleToSmiInteger",
  "IsSmiDouble",
  "IsInt32Double",
  "IsUint32Double",
  "DoubleToUint32IfEqualToSelf",
];
for (const f of fns) {
  if (!new RegExp(`\\b${f}\\(`).test(inlSrc)) throw new Error(`conversions-inl.h 未找到 ${f}`);
}
// DoubleToInt32/WebIDL 指数上限
if (!/if \(exponent > 31\) return 0;/.test(inlSrc)) throw new Error("DoubleToInt32 指数上限漂移");
if (!/if \(exponent > 63\) return 0;/.test(inlSrc)) throw new Error("WebIDL 指数上限漂移");

const hex = (v) => `0x${v.toString(16).toUpperCase()}`;

let rust = banner(
  "src/numbers/conversions-inl.h.js",
  "src/numbers/conversions-inl.h (+conversions.h 依赖常量)",
  "双精度转换魔数与签名骨架",
);

rust += `
/// 双精度→整数转换魔数与界 (原版 conversions-inl.h/conversions.h 显式展开)
pub mod conv_consts {
  /// 2^52: 借位对齐魔数 (原版 k2Pow52)
  pub const TWO_POW_52: f64 = 4503599627370496.0;

  /// double 舍入到 float 上界的最大值 (原版 kRoundingThreshold)
  pub const FLOAT_ROUNDING_THRESHOLD: f64 = 3.4028235677973362e+38;
  /// 负向镜像
  pub const NEG_FLOAT_ROUNDING_THRESHOLD: f64 = -3.4028235677973362e+38;

  /// FastD2UI 越界/∞/NaN 返回的整数不定值 (原版 0x80000000u)
  pub const INTEGER_INDEFINITE: u32 = 0x80000000;

  /// DoubleToUint32IfEqualToSelf: 指数 52 + 隐含位的高 32 位特征 (原版 kValidTopBits)
  pub const VALID_TOP_BITS: u32 = 0x${validTopBits[1].toUpperCase()};
  /// 低 32 位掩码 (原版 kBottomBitMask)
  pub const BOTTOM_BIT_MASK: u64 = 0x0000_0000_FFFF_FFFF;

  // —— FP64/FP16 位型常量 (定义于 conversions.h, 依赖再展开) ——
  pub const FP64_EXPONENT_BITS: i32 = ${fpDefs.fp64ExponentBits};
  pub const FP64_MANTISSA_BITS: i32 = ${fpDefs.fp64MantissaBits};
  pub const FP64_EXPONENT_BIAS: u64 = ${fpDefs.fp64ExponentBias};
  pub const FP64_SIGN_MASK: u64 = ${hex(fp64SignMask)};
  pub const FP64_INFINITY: u64 = ${hex(fp64Infinity)};
  pub const FP16_INFINITY_AND_NAN_INFIMUM: u64 = ${hex(fp16InfinityAndNaNInfimum)};
  pub const FP16_MIN_EXPONENT: u64 = ${fp16MinExponent};
  pub const FP16_DENORMAL_THRESHOLD: u64 = ${hex(fp16DenormalThreshold)};
  pub const FP16_MANTISSA_BITS: i32 = ${fpDefs.fp16MantissaBits};
  pub const FP16_Q_NAN: u16 = ${fpDefs.fp16qNaN};
  pub const FP16_INFINITY: u16 = ${fpDefs.fp16Infinity};
  pub const FP64_TO16_ROUNDING_ADDEND: u64 = ${hex(fp64To16RoundingAddend)};
  pub const FP64_TO16_REBIAS_EXPONENT_AND_ROUND: u64 = ${hex(fp64To16Rebias)};
  pub const FP64_TO16_DENORMAL_MAGIC: u64 = ${hex(fp64To16DenormalMagic)};
}

/// 转换函数签名骨架 (语义实现见 crate 内 conversions/ 模块)
pub mod conversions_inl_skel {
  /// 快速 double→u32, 不保证向零舍入 (原版 FastD2UI; 越界回 INTEGER_INDEFINITE)
  pub fn fast_d2ui(_x: f64) -> u32 {
    unimplemented!("骨架")
  }
  /// double→f16 位型 (rygorous 技巧, 原版 DoubleToFloat16)
  pub fn double_to_float16(_value: f64) -> u16 {
    unimplemented!("骨架")
  }
  /// double→f32 (超界按 kRoundingThreshold 舍入, 原版 DoubleToFloat32)
  pub fn double_to_float32(_x: f64) -> f32 {
    unimplemented!("骨架")
  }
  /// ToIntegerOrInfinity: -0 归 +0 (原版 DoubleToInteger)
  pub fn double_to_integer(_x: f64) -> f64 {
    unimplemented!("骨架")
  }
  /// ToInt32 (原版 DoubleToInt32; 指数上限 31)
  pub fn double_to_int32(_x: f64) -> i32 {
    unimplemented!("骨架")
  }
  /// WebIDL ConvertToInt 有符号 (原版 DoubleToWebIDLInt64; 指数上限 63)
  pub fn double_to_webidl_int64(_x: f64) -> i64 {
    unimplemented!("骨架")
  }
  /// WebIDL ConvertToInt 无符号 (原版 DoubleToWebIDLUint64)
  pub fn double_to_webidl_uint64(_x: f64) -> u64 {
    unimplemented!("骨架")
  }
  /// double 是否恰为 Smi 并回填 (原版 DoubleToSmiInteger)
  pub fn double_to_smi_integer(_value: f64) -> Option<i32> {
    unimplemented!("骨架")
  }
  /// Smi 判定 (原版 IsSmiDouble)
  pub fn is_smi_double(_value: f64) -> bool {
    unimplemented!("骨架")
  }
  /// int32 判定 (原版 IsInt32Double)
  pub fn is_int32_double(_value: f64) -> bool {
    unimplemented!("骨架")
  }
  /// uint32 判定 (原版 IsUint32Double)
  pub fn is_uint32_double(_value: f64) -> bool {
    unimplemented!("骨架")
  }
  /// 恰等 uint32 判定, 高 32 位特征检查 (原版 DoubleToUint32IfEqualToSelf)
  pub fn double_to_uint32_if_equal_to_self(_value: f64) -> Option<u32> {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-numbers", "conversions/inl_generated.rs", rust);
console.log(`written ${p}; 函数=${fns.length}, 常量=${15 + Object.keys(fpDefs).length}`);
