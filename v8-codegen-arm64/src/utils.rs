// 由 tool/src/codegen/arm64/utils-arm64.h.js 从 v8/src/codegen/arm64/utils-arm64.{h,cc} 生成; 修订就地改, 复跑覆盖
// arm64 位工具: REV 置换表/位计数/NaN 分类签名骨架

use crate::constants::{
  K_DOUBLE_EXPONENT_BITS, K_DOUBLE_MANTISSA_BITS, K_FLOAT_EXPONENT_BITS, K_FLOAT_MANTISSA_BITS,
  K_FLOAT16_EXPONENT_BITS, K_FLOAT16_MANTISSA_BITS,
};

/// ReverseBytes 置换表 (原版 permute_table[3][8]):
/// [0] REV16, [1] REV32/REV_w, [2] REV 恒等
#[rustfmt::skip]
pub const PERMUTE_TABLE: [[u8; 8]; 3] = [
  [6, 7, 4, 5, 2, 3, 0, 1],
  [4, 5, 6, 7, 0, 1, 2, 3],
  [0, 1, 2, 3, 4, 5, 6, 7],
];

/// float16 静默 NaN 掩码 (原版 kFP16QuietNaNMask)
pub const K_FP16_QUIET_NAN_MASK: u16 = 0x0200;
// 双精度静默 NaN 掩码引用 (kDQuietNanMask, 定义于 constants 模块)
// 单精度 kSQuietNanMask 同上 — 本模块不重复定义

/// float16classify 返回类别 (fpclassify 语义)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FpClass {
  FpNan = 0,
  FpInfinite = 1,
  FpZero = 2,
  FpSubnormal = 3,
  FpNormal = 4,
}

/// 原版 CountLeadingZeros(value, width): width 内前导零 (机械直译)
#[inline]
#[must_use]
pub const fn count_leading_zeros(value: u64, width: u32) -> i32 {
  if value == 0 {
    return width as i32;
  }
  (value << (64 - width)).leading_zeros() as i32
}

/// 原版 CountTrailingZeros(value, width)
#[inline]
#[must_use]
pub const fn count_trailing_zeros(value: u64, width: u32) -> i32 {
  if value == 0 {
    return width as i32;
  }
  value.trailing_zeros() as i32
}

/// 原版 LargestPowerOf2Divisor: value & (~value + 1) (二补码防溢出形态)
#[inline]
#[must_use]
pub const fn largest_power_of_2_divisor(value: u64) -> u64 {
  value.isolate_lowest_one()
}

/// 原版 ReverseBytes(value, block_bytes_log2): REV16/REV32/REV 语义
/// (机械直译; T 以 u64 承载, 调用方按 4/8 字节宽度截取)
#[must_use]
pub const fn reverse_bytes(value: u64, block_bytes_log2: u32) -> u64 {
  assert!(block_bytes_log2 >= 1 && block_bytes_log2 <= 3);
  let bytes = value.to_le_bytes();
  let row = PERMUTE_TABLE[(block_bytes_log2 - 1) as usize];
  let mut result: u64 = 0;
  let mut i = 0;
  while i < 8 {
    result <<= 8;
    result |= bytes[row[i] as usize] as u64;
    i += 1;
  }
  result
}

// —— 浮点分解/打包/位计数 (语义填体) ——

/// 原版 unsigned_bitextract_32(hi, lo, x): (x >> lo) & ((1 << (hi-lo+1)) - 1)
#[inline]
#[must_use]
pub const fn unsigned_bitextract_32(hi: u32, lo: u32, x: u32) -> u32 {
  (x >> lo) & (u32::MAX >> (31 - (hi - lo)))
}

/// 原版 unsigned_bitextract_64(hi, lo, x): (x >> lo) & ((1 << (hi-lo+1)) - 1)
#[inline]
#[must_use]
pub const fn unsigned_bitextract_64(hi: u32, lo: u32, x: u64) -> u64 {
  (x >> lo) & (u64::MAX >> (63 - (hi - lo)))
}

/// v8: src/codegen/arm64/utils-arm64.cc float_sign
#[inline]
#[must_use]
pub fn float_sign(val: f32) -> u32 {
  let bits = val.to_bits();
  unsigned_bitextract_32(31, 31, bits)
}

/// v8: src/codegen/arm64/utils-arm64.cc float_exp
#[inline]
#[must_use]
pub fn float_exp(val: f32) -> u32 {
  let bits = val.to_bits();
  unsigned_bitextract_32(30, 23, bits)
}

/// v8: src/codegen/arm64/utils-arm64.cc float_mantissa
#[inline]
#[must_use]
pub fn float_mantissa(val: f32) -> u32 {
  let bits = val.to_bits();
  unsigned_bitextract_32(22, 0, bits)
}

/// v8: src/codegen/arm64/utils-arm64.cc double_sign
#[inline]
#[must_use]
pub fn double_sign(val: f64) -> u32 {
  let bits = val.to_bits();
  unsigned_bitextract_64(63, 63, bits) as u32
}

/// v8: src/codegen/arm64/utils-arm64.cc double_exp
#[inline]
#[must_use]
pub fn double_exp(val: f64) -> u32 {
  let bits = val.to_bits();
  unsigned_bitextract_64(62, 52, bits) as u32
}

/// v8: src/codegen/arm64/utils-arm64.cc double_mantissa
#[inline]
#[must_use]
pub fn double_mantissa(val: f64) -> u64 {
  let bits = val.to_bits();
  unsigned_bitextract_64(51, 0, bits)
}

/// v8: src/codegen/arm64/utils-arm64.cc float_pack
#[inline]
#[must_use]
pub fn float_pack(sign: u32, exp: u32, mantissa: u32) -> f32 {
  let bits = sign.wrapping_shl(K_FLOAT_EXPONENT_BITS) | exp;
  f32::from_bits((bits.wrapping_shl(K_FLOAT_MANTISSA_BITS)) | mantissa)
}

/// v8: src/codegen/arm64/utils-arm64.cc double_pack
#[inline]
#[must_use]
pub fn double_pack(sign: u64, exp: u64, mantissa: u64) -> f64 {
  let bits = sign.wrapping_shl(K_DOUBLE_EXPONENT_BITS) | exp;
  f64::from_bits((bits.wrapping_shl(K_DOUBLE_MANTISSA_BITS)) | mantissa)
}

/// v8: src/codegen/arm64/utils-arm64.cc float16classify
#[inline]
#[must_use]
pub const fn float16classify(value: u16) -> i32 {
  let exponent_max: u16 = (1u16 << K_FLOAT16_EXPONENT_BITS) - 1;
  let exponent_mask: u16 = exponent_max << K_FLOAT16_MANTISSA_BITS;
  let mantissa_mask: u16 = (1u16 << K_FLOAT16_MANTISSA_BITS) - 1;

  let exponent = (value & exponent_mask) >> K_FLOAT16_MANTISSA_BITS;
  let mantissa = value & mantissa_mask;
  if exponent == 0 {
    if mantissa == 0 {
      FpClass::FpZero as i32
    } else {
      FpClass::FpSubnormal as i32
    }
  } else if exponent == exponent_max {
    if mantissa == 0 {
      FpClass::FpInfinite as i32
    } else {
      FpClass::FpNan as i32
    }
  } else {
    FpClass::FpNormal as i32
  }
}

/// v8: src/codegen/arm64/utils-arm64.cc CountLeadingSignBits
#[inline]
#[must_use]
pub const fn count_leading_sign_bits(value: i64, width: i32) -> i32 {
  let width_u = width as u32;
  debug_assert!(width_u.is_power_of_two() && width_u <= 64);
  if value >= 0 {
    count_leading_zeros(value as u64, width_u) - 1
  } else {
    count_leading_zeros(!(value as u64), width_u) - 1
  }
}

/// v8: src/codegen/arm64/utils-arm64.cc CountSetBits
#[inline]
#[must_use]
pub const fn count_set_bits(value: u64, width: i32) -> i32 {
  debug_assert!(width == 32 || width == 64);
  if width == 64 {
    value.count_ones() as i32
  } else {
    ((value & 0xFFFF_FFFF) as u32).count_ones() as i32
  }
}

/// v8: src/codegen/arm64/utils-arm64.cc LowestSetBitPosition
#[inline]
#[must_use]
pub const fn lowest_set_bit_position(value: u64) -> i32 {
  debug_assert!(value != 0);
  value.trailing_zeros() as i32 + 1
}

/// v8: src/codegen/arm64/utils-arm64.cc HighestSetBitPosition
#[inline]
#[must_use]
pub const fn highest_set_bit_position(value: u64) -> i32 {
  debug_assert!(value != 0);
  63 - count_leading_zeros(value, 64)
}

/// v8: src/codegen/arm64/utils-arm64.cc MaskToBit
#[inline]
#[must_use]
pub const fn mask_to_bit(mask: u64) -> i32 {
  debug_assert!(mask.count_ones() == 1);
  mask.trailing_zeros() as i32
}

/// 原版 IsSignallingNaN(f64)
#[inline]
#[must_use]
pub fn is_signalling_nan_f64(num: f64, d_quiet_nan_mask: u64) -> bool {
  let raw = num.to_bits();
  num.is_nan() && (raw & d_quiet_nan_mask) == 0
}

/// 原版 IsSignallingNaN(f32)
#[inline]
#[must_use]
pub fn is_signalling_nan_f32(num: f32, s_quiet_nan_mask: u32) -> bool {
  let raw = num.to_bits();
  num.is_nan() && (raw & s_quiet_nan_mask) == 0
}

/// 原版 IsSignallingNaN(f16)
#[inline]
#[must_use]
pub fn is_signalling_nan_f16(num: u16) -> bool {
  float16classify(num) == (FpClass::FpNan as i32) && (num & K_FP16_QUIET_NAN_MASK) == 0
}

/// 原版 IsQuietNaN(f64)
#[inline]
#[must_use]
pub fn is_quiet_nan_f64(num: f64, d_quiet_nan_mask: u64) -> bool {
  num.is_nan() && !is_signalling_nan_f64(num, d_quiet_nan_mask)
}

/// 原版 IsQuietNaN(f32)
#[inline]
#[must_use]
pub fn is_quiet_nan_f32(num: f32, s_quiet_nan_mask: u32) -> bool {
  num.is_nan() && !is_signalling_nan_f32(num, s_quiet_nan_mask)
}

/// 原版 ToQuietNaN(f64)
#[inline]
#[must_use]
pub fn to_quiet_nan_f64(num: f64, d_quiet_nan_mask: u64) -> f64 {
  debug_assert!(num.is_nan());
  f64::from_bits(num.to_bits() | d_quiet_nan_mask)
}

/// 原版 ToQuietNaN(f32)
#[inline]
#[must_use]
pub fn to_quiet_nan_f32(num: f32, s_quiet_nan_mask: u32) -> f32 {
  debug_assert!(num.is_nan());
  f32::from_bits(num.to_bits() | s_quiet_nan_mask)
}

/// 原版 FusedMultiplyAdd (f64)
#[inline]
#[must_use]
pub fn fused_multiply_add(op1: f64, op2: f64, a: f64) -> f64 {
  op1.mul_add(op2, a)
}

/// 原版 FusedMultiplyAdd (f32)
#[inline]
#[must_use]
pub fn fused_multiply_add_f32(op1: f32, op2: f32, a: f32) -> f32 {
  op1.mul_add(op2, a)
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_FP16_QUIET_NAN_MASK == 0x0200);
  assert!(count_leading_zeros(0, 32) == 32);
  assert!(count_leading_zeros(1, 32) == 31);
  assert!(count_trailing_zeros(0b1000, 32) == 3);
  assert!(largest_power_of_2_divisor(48) == 16);
  assert!(reverse_bytes(0x0123_4567_89ab_cdef, 3) == 0xefcd_ab89_6745_2301);
};
