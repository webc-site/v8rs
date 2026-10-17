// 由 tool/src/utils/consts.js 从 v8/src/utils/boxed-float.h;src/common/globals.h 生成; 修订就地改, 复跑覆盖

//! 位模式安全浮点包装 (Float32/Float64), 移植自 v8/src/utils/boxed-float。
//!
//! 去优化传参时保持 NaN 位模式不被静默改写。

use std::fmt;

use crate::utils::hash64;

/// Float32 指数域右移位数
const EXPONENT_SHIFT: u32 = 23;
/// Float32 quiet-NaN 标志位
const QUIET_NAN_BIT: u32 = 1 << 22;
/// Float32 符号位
const SIGN_BIT: u32 = 1 << 31;
/// hole NaN 位模式 (kHoleNanInt64, globals.h)
pub const HOLE_NAN_INT64: u64 = ((0xFFF7FFFFu64) << 32) | 0xFFF7FFFFu64;
/// undefined NaN 位模式 (kUndefinedNanInt64, V8_ENABLE_UNDEFINED_DOUBLE)
pub const UNDEFINED_NAN_INT64: u64 = ((0xFFF6FFFFu64) << 32) | 0xFFF6FFFFu64;
/// Float64 quiet-NaN 位 (bit 51)
const QUIET_NAN_BIT_64: u64 = 1 << 51;

/// `Float32` (boxed-float.h:17): f32 位模式持有体。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Float32 {
  bit_pattern: u32,
}

impl Float32 {
  /// `Float32(float)` (boxed-float.h:24): NaN 输入不保证位保持 (调用方契约)。
  #[must_use]
  pub fn new(value: f32) -> Self {
    debug_assert!(!value.is_nan());
    Self {
      bit_pattern: value.to_bits(),
    }
  }

  /// `FromBits` (boxed-float.h:96)。
  #[must_use]
  pub const fn from_bits(bits: u32) -> Self {
    Self { bit_pattern: bits }
  }

  /// `get_bits` (boxed-float.h:31)。
  #[must_use]
  pub const fn get_bits(self) -> u32 {
    self.bit_pattern
  }

  /// `get_scalar` (boxed-float.h:33)。
  #[must_use]
  pub const fn get_scalar(self) -> f32 {
    f32::from_bits(self.bit_pattern)
  }

  /// `is_nan` (boxed-float.h:35): 指数全 1 且尾数非 0。
  #[must_use]
  pub const fn is_nan(self) -> bool {
    self.exponent() == 0xFF && self.mantissa() != 0
  }

  /// `is_quiet_nan` (boxed-float.h:40)。
  #[must_use]
  pub const fn is_quiet_nan(self) -> bool {
    self.is_nan() && (self.bit_pattern & QUIET_NAN_BIT) != 0
  }

  /// `is_inf` (boxed-float.h:44): 指数全 1 且尾数为 0。
  #[must_use]
  pub const fn is_inf(self) -> bool {
    self.exponent() == 0xFF && self.mantissa() == 0
  }

  /// `is_negative` (boxed-float.h:48)。
  #[must_use]
  pub const fn is_negative(self) -> bool {
    self.bit_pattern & SIGN_BIT != 0
  }

  /// `to_quiet_nan` (boxed-float.h:51): 置 quiet 位。
  #[must_use]
  pub const fn to_quiet_nan(self) -> Self {
    Self::from_bits(self.bit_pattern | QUIET_NAN_BIT)
  }

  /// `to_negative` (boxed-float.h:57): 置符号位。
  #[must_use]
  pub const fn to_negative(self) -> Self {
    Self::from_bits(self.bit_pattern | SIGN_BIT)
  }

  /// `operator-` (boxed-float.h:61): 翻转符号位。
  #[must_use]
  pub const fn neg(self) -> Self {
    Self::from_bits(self.bit_pattern ^ SIGN_BIT)
  }

  /// `quiet_nan()` (boxed-float.h:71)。
  #[must_use]
  pub const fn quiet_nan() -> Self {
    Self::from_bits((0xFF << EXPONENT_SHIFT) | QUIET_NAN_BIT)
  }

  /// `infinity()` (boxed-float.h:75)。
  #[must_use]
  pub const fn infinity() -> Self {
    Self::from_bits(0xFF << EXPONENT_SHIFT)
  }

  const fn exponent(self) -> u32 {
    (self.bit_pattern >> EXPONENT_SHIFT) & 0xFF
  }

  const fn mantissa(self) -> u32 {
    self.bit_pattern & ((1 << EXPONENT_SHIFT) - 1)
  }
}

impl fmt::Display for Float32 {
  /// operator<<: "scalar (0x%08x)" (boxed-float.h:118)。
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} (0x{:08x})", self.get_scalar(), self.bit_pattern)
  }
}

/// `Float64` (boxed-float.h:124): f64 位模式持有体。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Float64 {
  bit_pattern: u64,
}

impl Float64 {
  /// `Float64(double)` (boxed-float.h:129): NaN 输入不保证位保持 (调用方契约)。
  #[must_use]
  pub fn new(value: f64) -> Self {
    debug_assert!(!value.is_nan());
    Self {
      bit_pattern: value.to_bits(),
    }
  }

  /// `FromBits` (boxed-float.h:187)。
  #[must_use]
  pub const fn from_bits(bits: u64) -> Self {
    Self { bit_pattern: bits }
  }

  /// `FromMaybeNaN` (boxed-float.h:192): 允许 NaN (位模式语义放宽)。
  #[must_use]
  pub const fn from_maybe_nan(value: f64) -> Self {
    Self {
      bit_pattern: value.to_bits(),
    }
  }

  /// `quiet_nan()` (boxed-float.h:139)。
  #[must_use]
  pub const fn quiet_nan() -> Self {
    Self::from_bits(f64::NAN.to_bits())
  }

  /// `hole_nan()` (boxed-float.h:142)。
  #[must_use]
  pub const fn hole_nan() -> Self {
    Self::from_bits(HOLE_NAN_INT64)
  }

  /// `undefined_nan()` (V8_ENABLE_UNDEFINED_DOUBLE 分支)。
  #[must_use]
  pub const fn undefined_nan() -> Self {
    Self::from_bits(UNDEFINED_NAN_INT64)
  }

  /// `get_bits` (boxed-float.h:147)。
  #[must_use]
  pub const fn get_bits(self) -> u64 {
    self.bit_pattern
  }

  /// `get_scalar` (boxed-float.h:148)。
  #[must_use]
  pub const fn get_scalar(self) -> f64 {
    f64::from_bits(self.bit_pattern)
  }

  /// `is_hole_nan` (boxed-float.h:149)。
  #[must_use]
  pub const fn is_hole_nan(self) -> bool {
    self.bit_pattern == HOLE_NAN_INT64
  }

  /// `is_undefined_nan` (boxed-float.h:151)。
  #[must_use]
  pub const fn is_undefined_nan(self) -> bool {
    self.bit_pattern == UNDEFINED_NAN_INT64
  }

  /// `is_undefined_or_hole_nan` (boxed-float.h:153)。
  #[must_use]
  pub const fn is_undefined_or_hole_nan(self) -> bool {
    self.is_undefined_nan() || self.is_hole_nan()
  }

  /// `is_nan` (boxed-float.h:158): 指数全 1 且尾数非 0。
  #[must_use]
  pub const fn is_nan(self) -> bool {
    self.exponent() == 0x7FF && self.mantissa() != 0
  }

  /// `is_quiet_nan` (boxed-float.h:165)。
  #[must_use]
  pub const fn is_quiet_nan(self) -> bool {
    self.is_nan() && (self.bit_pattern & QUIET_NAN_BIT_64) != 0
  }

  /// `is_signalling_nan` (boxed-float.h:168)。
  #[must_use]
  pub const fn is_signalling_nan(self) -> bool {
    self.is_nan() && !self.is_quiet_nan()
  }

  /// `to_quiet_nan` (boxed-float.h:170): 置 quiet 位 (bit 51)。
  #[must_use]
  pub const fn to_quiet_nan(self) -> Self {
    Self::from_bits(self.bit_pattern | QUIET_NAN_BIT_64)
  }

  /// `hash_value` (boxed-float.h:208): base::hash_value(bits) = hash64 混淆。
  #[must_use]
  pub const fn hash_value(self) -> u64 {
    hash64(self.bit_pattern)
  }

  const fn exponent(self) -> u64 {
    (self.bit_pattern >> 52) & ((1 << 11) - 1)
  }

  const fn mantissa(self) -> u64 {
    self.bit_pattern & ((1 << 52) - 1)
  }
}

impl fmt::Display for Float64 {
  /// operator<<: "scalar (0x%016x)" (boxed-float.h:213)。
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} (0x{:016x})", self.get_scalar(), self.bit_pattern)
  }
}
