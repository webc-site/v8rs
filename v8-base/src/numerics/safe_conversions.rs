//! 原版 v8/src/base/numerics/safe_conversions.h + safe_conversions_impl.h +
//! safe_conversions_arm_impl.h — 数值安全转换族 (1:1 语义承接).
//!
//! C++ 模板机制的对应关系:
//! - `std::numeric_limits<T>` / `kMaxExponent` / `kIntegerBitsPlusSign` → [`Numeric`] trait 常量;
//! - `DstRangeRelationToSrcRange` 五路特化 → [`dst_range_relation_to_src_range`] (按 trait 常量分派,
//!   判定顺序与原版逐条一致);
//! - `static_cast` 跨类型转换 → [`CastTo`] 成对 trait (宏展开 12×12, 与 Rust `as` 舍入/截断一致);
//! - arm `ssat/usat` 饱和指令: Rust clamp 即等价语义, 无需内联汇编特化.
//!
//! 与原版的已知语义差异 (登记):
//! - float→int 越界转换: C++ `static_cast` 为 UB, Rust `as` 为饱和; 本实现取饱和 (UB 防护).
//! - `StrictNumeric`/`strict_cast` 的编译期区间 requires 降为 `debug_assert`.

use core::{cmp::Ordering, ops::Not};

use crate::immediate_crash::immediate_crash;

/// v8: src/base/numerics/safe_conversions_impl.h enum class IntegerRepresentation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntegerRepresentation {
  /// v8: kUnsigned
  Unsigned,
  /// v8: kSigned
  Signed,
}

/// v8: src/base/numerics/safe_conversions_impl.h enum class NumericRangeRepresentation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NumericRangeRepresentation {
  /// v8: kNotContained
  NotContained,
  /// v8: kContained
  Contained,
}

/// 数值类型信息表
/// v8: src/base/numerics/safe_conversions_impl.h kMaxExponent / kIntegerBitsPlusSign
pub trait Numeric: Copy + PartialOrd + Send + Sync + 'static {
  /// v8: std::is_signed_v<T> (浮点亦归为有符号)
  const IS_SIGNED: bool;
  /// v8: std::is_floating_point_v<T>
  const IS_FLOAT: bool;
  /// v8: std::numeric_limits<T>::digits (有效位/尾数位)
  const DIGITS: u32;
  /// v8: kMaxExponent<T> = is_floating_point ? max_exponent : digits + 1
  const MAX_EXPONENT: u32;
  /// v8: kIntegerBitsPlusSign<T> = digits + is_signed
  const BITS_PLUS_SIGN: u32;

  /// v8: std::numeric_limits<T>::lowest()
  fn min_val() -> Self;
  /// v8: std::numeric_limits<T>::max()
  fn max_val() -> Self;
  /// v8: SaturationDefaultLimits<T>::NaN() — 整型无 quiet_NaN 时取 T() = 0
  fn nan_or_zero() -> Self;

  /// v8: SaturationDefaultLimits<T>::Overflow() — 浮点取 +inf, 整型取 max()
  #[inline]
  fn sat_overflow() -> Self {
    Self::max_val()
  }
  /// v8: SaturationDefaultLimits<T>::Underflow() — 浮点取 -inf, 整型取 lowest()
  #[inline]
  fn sat_underflow() -> Self {
    Self::min_val()
  }

  /// 桥接: 整数 → i128 精确; 浮点 → None
  fn to_i128_exact(self) -> Option<i128>;
  /// 桥接: → f64 (`as` 舍入与 C++ static_cast<double> 一致)
  fn to_f64(self) -> f64;
}

/// v8: static_cast<Dst>(Src) — 成对桥接 (单一舍入, 与 Rust `as` 一致)
pub trait CastTo<Dst> {
  fn cast_to(self) -> Dst;
}

macro_rules! impl_numeric_signed_int {
  ($($t:ty),*) => {
    $(impl Numeric for $t {
      const IS_SIGNED: bool = true;
      const IS_FLOAT: bool = false;
      const DIGITS: u32 = <$t>::BITS - 1;
      const MAX_EXPONENT: u32 = <$t>::BITS;
      const BITS_PLUS_SIGN: u32 = <$t>::BITS;
      #[inline] fn min_val() -> Self { <$t>::MIN }
      #[inline] fn max_val() -> Self { <$t>::MAX }
      #[inline] fn nan_or_zero() -> Self { 0 }
      #[inline] fn to_i128_exact(self) -> Option<i128> { Some(self as i128) }
      #[inline] fn to_f64(self) -> f64 { self as f64 }
    })*
  };
}
macro_rules! impl_numeric_unsigned_int {
  ($($t:ty),*) => {
    $(impl Numeric for $t {
      const IS_SIGNED: bool = false;
      const IS_FLOAT: bool = false;
      const DIGITS: u32 = <$t>::BITS;
      const MAX_EXPONENT: u32 = <$t>::BITS + 1;
      const BITS_PLUS_SIGN: u32 = <$t>::BITS;
      #[inline] fn min_val() -> Self { <$t>::MIN }
      #[inline] fn max_val() -> Self { <$t>::MAX }
      #[inline] fn nan_or_zero() -> Self { 0 }
      #[inline] fn to_i128_exact(self) -> Option<i128> { Some(self as i128) }
      #[inline] fn to_f64(self) -> f64 { self as f64 }
    })*
  };
}
macro_rules! impl_numeric_float {
  ($($t:ty),* $(,)?) => {
    $(impl Numeric for $t {
      const IS_SIGNED: bool = true;
      const IS_FLOAT: bool = true;
      const DIGITS: u32 = <$t>::MANTISSA_DIGITS;
      // v8: numeric_limits<float>::max_exponent=128, double=1024
      const MAX_EXPONENT: u32 = if core::mem::size_of::<$t>() == 4 { 128 } else { 1024 };
      // v8: kIntegerBitsPlusSign = digits + is_signed (浮点 is_signed 为真)
      const BITS_PLUS_SIGN: u32 = <$t>::MANTISSA_DIGITS + 1;
      #[inline] fn min_val() -> Self { <$t>::MIN }
      #[inline] fn max_val() -> Self { <$t>::MAX }
      #[inline] fn nan_or_zero() -> Self { <$t>::NAN }
      #[inline] fn sat_overflow() -> Self { <$t>::INFINITY }
      #[inline] fn sat_underflow() -> Self { <$t>::NEG_INFINITY }
      #[inline] fn to_i128_exact(self) -> Option<i128> { None }
      #[inline] fn to_f64(self) -> f64 { self as f64 }
    })*
  };
}
impl_numeric_signed_int!(i8, i16, i32, i64, isize);
impl_numeric_unsigned_int!(u8, u16, u32, u64, usize);
impl_numeric_float!(f32, f64);

/// 12 类型全配对转换表 (与 v8 可实例化的算术类型集一致)
macro_rules! cast_row {
  ($src:ty; $($dst:ty),* $(,)?) => { $(
    impl CastTo<$dst> for $src {
      #[inline]
      fn cast_to(self) -> $dst {
        self as $dst
      }
    }
  )* };
}
cast_row!(i8; i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
cast_row!(u8; i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
cast_row!(i16; i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
cast_row!(u16; i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
cast_row!(i32; i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
cast_row!(u32; i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
cast_row!(i64; i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
cast_row!(u64; i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
cast_row!(isize; i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
cast_row!(usize; i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
cast_row!(f32; i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
cast_row!(f64; i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);

/// v8: src/base/numerics/safe_conversions_impl.h class RangeCheck
/// 双 bool 位标志 (成员顺序 = is_underflow_ 先于 is_overflow_, 原版注明勿动)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RangeCheck {
  /// v8: is_underflow_
  pub is_underflow: bool,
  /// v8: is_overflow_
  pub is_overflow: bool,
}

impl RangeCheck {
  /// v8: RangeCheck(is_in_lower_bound, is_in_upper_bound) — 入参为"在界内", 取反存标志
  #[inline]
  #[must_use]
  pub const fn new(is_in_lower_bound: bool, is_in_upper_bound: bool) -> Self {
    Self {
      is_underflow: !is_in_lower_bound,
      is_overflow: !is_in_upper_bound,
    }
  }

  /// v8: RangeCheck::IsValid()
  #[inline]
  #[must_use]
  pub const fn is_valid(&self) -> bool {
    !self.is_overflow && !self.is_underflow
  }

  /// v8: RangeCheck::IsInvalid()
  #[inline]
  #[must_use]
  pub const fn is_invalid(&self) -> bool {
    self.is_overflow && self.is_underflow
  }

  /// v8: RangeCheck::IsOverflow()
  #[inline]
  #[must_use]
  pub const fn overflow(&self) -> bool {
    self.is_overflow && !self.is_underflow
  }

  /// v8: RangeCheck::IsUnderflow()
  #[inline]
  #[must_use]
  pub const fn underflow(&self) -> bool {
    !self.is_overflow && self.is_underflow
  }

  /// v8: RangeCheck::IsOverflowFlagSet()
  #[inline]
  #[must_use]
  pub const fn is_overflow_flag_set(&self) -> bool {
    self.is_overflow
  }

  /// v8: RangeCheck::IsUnderflowFlagSet()
  #[inline]
  #[must_use]
  pub const fn is_underflow_flag_set(&self) -> bool {
    self.is_underflow
  }
}

/// v8: src/base/numerics/safe_conversions_impl.h kStaticDstRangeRelationToSrcRange 三路特化
#[inline]
#[must_use]
pub const fn static_dst_range_relation_to_src_range<D: Numeric, S: Numeric>()
-> NumericRangeRepresentation {
  // 有符号→无符号 (su): 无法静态判定包含 (原版恒 kNotContained)
  if !D::IS_SIGNED && S::IS_SIGNED {
    NumericRangeRepresentation::NotContained
  } else if D::IS_SIGNED && !S::IS_SIGNED {
    // 无符号→有符号 (us): 严格大于才包含
    if D::MAX_EXPONENT > S::MAX_EXPONENT {
      NumericRangeRepresentation::Contained
    } else {
      NumericRangeRepresentation::NotContained
    }
  } else if D::MAX_EXPONENT >= S::MAX_EXPONENT {
    NumericRangeRepresentation::Contained
  } else {
    NumericRangeRepresentation::NotContained
  }
}

/// v8: src/base/numerics/safe_conversions_impl.h kIsTypeInRangeForNumericType<Dst, Src>
#[inline]
#[must_use]
pub const fn is_type_in_range_for_numeric_type<D: Numeric, S: Numeric>() -> bool {
  matches!(
    static_dst_range_relation_to_src_range::<D, S>(),
    NumericRangeRepresentation::Contained
  )
}

/// f64 界值按 Src 精度舍入 (原版比较在 Src/Promotion 类型内进行)
fn round_to_src_precision<S: Numeric>(v: f64) -> f64 {
  if S::MAX_EXPONENT == 128 {
    (v as f32) as f64
  } else {
    v
  }
}

/// v8: NarrowingRange<Dst, Src, Bounds>::Adjust — float→整型时屏蔽比较精度不足的低位
/// kShift = (kMaxExponent<Src> > kMaxExponent<Dst> && Src.digits < Dst.digits)
///          ? Dst.digits - Src.digits : 0
fn narrowing_adjust<D: Numeric, S: Numeric>(bound: i128) -> f64 {
  let shift = if S::MAX_EXPONENT > D::MAX_EXPONENT && S::DIGITS < D::DIGITS {
    D::DIGITS - S::DIGITS
  } else {
    0
  };
  if shift == 0 {
    return bound as f64;
  }
  // ConditionalNegate(SafeUnsignedAbs(bound) & ~((1<<kShift)-1), IsValueNegative(bound))
  let mask = !((1i128 << shift) - 1);
  let neg = bound < 0;
  let abs = if neg {
    bound.unsigned_abs() as i128
  } else {
    bound
  } & mask;
  if neg { -(abs as f64) } else { abs as f64 }
}

/// v8: src/base/numerics/safe_conversions_impl.h DstRangeRelationToSrcRange 五路特化统一入口
/// (静态包含 → 常量短路恒有效; 四种符号组合逐条对应原版 Check())
#[inline]
#[must_use]
pub fn dst_range_relation_to_src_range<D: Numeric, S: Numeric + CastTo<D>>(value: S) -> RangeCheck {
  if is_type_in_range_for_numeric_type::<D, S>() {
    // 默认特化: 两个常量短路条件恒真
    return RangeCheck::new(true, true);
  }

  let d_min = D::min_val().to_i128_exact();
  let d_max = D::max_val().to_i128_exact();

  if !D::IS_SIGNED {
    if S::IS_SIGNED {
      // Signed to unsigned: 负值下溢; 上界 = 常量(Src max ≤ Dst max) 或值 ≤ 调整后 Dst max
      let ge_zero = if S::IS_FLOAT {
        value.to_f64() > round_to_src_precision::<S>(-1f64)
      } else {
        value.to_i128_exact().is_some_and(|v| v >= 0)
      };
      let lower = ge_zero && (d_min == Some(0) || CastTo::<D>::cast_to(value) >= D::min_val());
      let upper = if S::IS_FLOAT {
        let bound = round_to_src_precision::<S>(narrowing_adjust::<D, S>(d_max.unwrap()));
        value.to_f64() <= bound
      } else {
        let v = value.to_i128_exact().unwrap();
        S::max_val().to_i128_exact().unwrap() <= d_max.unwrap() || v <= d_max.unwrap()
      };
      RangeCheck::new(lower, upper)
    } else {
      // Unsigned to unsigned narrowing: 仅上界可越
      let v = value.to_i128_exact().unwrap();
      RangeCheck::new(d_min == Some(0) || v >= d_min.unwrap(), v <= d_max.unwrap())
    }
  } else if S::IS_SIGNED {
    // Signed to signed narrowing (含 float→float 收窄与 float→int):
    let (lower, upper) = if S::IS_FLOAT || D::IS_FLOAT {
      let f = value.to_f64();
      let low_f = round_to_src_precision::<S>(if D::IS_FLOAT {
        D::min_val().to_f64()
      } else {
        narrowing_adjust::<D, S>(d_min.unwrap())
      });
      let high_f = round_to_src_precision::<S>(if D::IS_FLOAT {
        D::max_val().to_f64()
      } else {
        narrowing_adjust::<D, S>(d_max.unwrap())
      });
      (f >= low_f, f <= high_f)
    } else {
      let v = value.to_i128_exact().unwrap();
      (v >= d_min.unwrap(), v <= d_max.unwrap())
    };
    RangeCheck::new(lower, upper)
  } else {
    // Unsigned to signed: 仅上界可越 (标准极限下界恒 ≤ 0)
    let upper = if S::IS_FLOAT {
      let bound = round_to_src_precision::<S>(narrowing_adjust::<D, S>(d_max.unwrap()));
      value.to_f64() <= bound
    } else {
      let v = value.to_i128_exact().unwrap();
      v <= d_max.unwrap()
    };
    RangeCheck::new(d_min.unwrap_or(0) <= 0, upper)
  }
}

/// v8: src/base/numerics/safe_conversions.h IsValueNegative
#[inline]
#[must_use]
pub fn is_value_negative<T: Numeric>(value: T) -> bool {
  if T::IS_FLOAT {
    value.to_f64() < 0f64
  } else if T::IS_SIGNED {
    value.to_i128_exact().is_some_and(|v| v < 0)
  } else {
    false
  }
}

/// v8: src/base/numerics/safe_conversions.h IsValueInRangeForNumericType
/// (IsValueInRangeFastOp 快路径与泛型路径逐值一致, 统一走关系判定)
#[inline]
#[must_use]
pub fn is_value_in_range_for<Dst: Numeric, Src: Numeric + CastTo<Dst>>(value: Src) -> bool {
  dst_range_relation_to_src_range::<Dst, Src>(value).is_valid()
}

/// v8: src/base/numerics/safe_conversions.h checked_cast — 越界即 CHECK 崩溃
#[inline]
#[must_use]
pub fn checked_cast<Dst: Numeric, Src: Numeric + CastTo<Dst>>(value: Src) -> Dst {
  if is_value_in_range_for::<Dst, Src>(value) {
    value.cast_to()
  } else {
    // v8: CheckOnFailure::HandleFailure → __builtin_trap
    unsafe { immediate_crash() }
  }
}

/// try_cast: checked_cast 的 Rust 化可失败版本 (供上层替代 CHECK 路径)
#[inline]
#[must_use]
pub fn try_cast<Dst: Numeric, Src: Numeric + CastTo<Dst>>(value: Src) -> Option<Dst> {
  is_value_in_range_for::<Dst, Src>(value).then(|| value.cast_to())
}

/// v8: src/base/numerics/safe_conversions.h strict_cast — 要求静态包含 (requires 降为 debug_assert)
#[inline]
#[must_use]
pub fn strict_cast<Dst: Numeric, Src: Numeric + CastTo<Dst>>(value: Src) -> Dst {
  debug_assert!(
    is_type_in_range_for_numeric_type::<Dst, Src>(),
    "strict_cast: destination range does not contain source range"
  );
  value.cast_to()
}

/// v8: src/base/numerics/safe_conversions.h saturated_cast
/// (SaturateFastOp 整数快路径与 saturated_cast_impl 逐值等价, 统一按后者实现;
///  边界取 SaturationDefaultLimits: 整型 max/lowest/NaN→0, 浮点 ±inf/NaN)
#[inline]
#[must_use]
pub fn saturated_cast<Dst: Numeric, Src: Numeric + CastTo<Dst>>(value: Src) -> Dst {
  let constraint = dst_range_relation_to_src_range::<Dst, Src>(value);
  if !constraint.is_overflow_flag_set() {
    if constraint.is_underflow_flag_set() {
      Dst::sat_underflow()
    } else {
      value.cast_to()
    }
  } else if !Src::IS_FLOAT || !constraint.is_underflow_flag_set() {
    Dst::sat_overflow()
  } else {
    Dst::nan_or_zero()
  }
}

/// saturated_cast 针对 f64 的单泛型特化版本
#[inline]
#[must_use]
pub fn saturated_cast_f64<Dst: Numeric>(value: f64) -> Dst
where
  f64: CastTo<Dst>,
{
  saturated_cast::<Dst, f64>(value)
}

/// v8: src/base/numerics/safe_conversions.h ClampFloor — 向负无穷舍入后饱和转换
/// (整型 Src 的 floor 为恒等, requires 限定浮点; 此处泛化到全部 Numeric)
#[inline]
#[must_use]
pub fn clamp_floor<Dst: Numeric, Src: Numeric + CastTo<Dst>>(value: Src) -> Dst
where
  f64: CastTo<Dst>,
{
  if Src::IS_FLOAT {
    saturated_cast::<Dst, f64>(value.to_f64().floor())
  } else {
    saturated_cast::<Dst, Src>(value)
  }
}

/// v8: ClampCeil
#[inline]
#[must_use]
pub fn clamp_ceil<Dst: Numeric, Src: Numeric + CastTo<Dst>>(value: Src) -> Dst
where
  f64: CastTo<Dst>,
{
  if Src::IS_FLOAT {
    saturated_cast::<Dst, f64>(value.to_f64().ceil())
  } else {
    saturated_cast::<Dst, Src>(value)
  }
}

/// v8: ClampRound — 就近舍入, .5 远离零 (std::round)
#[inline]
#[must_use]
pub fn clamp_round<Dst: Numeric, Src: Numeric + CastTo<Dst>>(value: Src) -> Dst
where
  f64: CastTo<Dst>,
{
  if Src::IS_FLOAT {
    saturated_cast::<Dst, f64>(value.to_f64().round())
  } else {
    saturated_cast::<Dst, Src>(value)
  }
}

/// v8: ClampFloor/ClampCeil/ClampRound 的 i32 常用入口 (默认 Dst=int)
#[inline]
#[must_use]
pub fn clamp_floor_i32(value: f64) -> i32 {
  clamp_floor(value)
}

/// ClampCeil (默认 i32)
#[inline]
#[must_use]
pub fn clamp_ceil_i32(value: f64) -> i32 {
  clamp_ceil(value)
}

/// ClampRound (默认 i32)
#[inline]
#[must_use]
pub fn clamp_round_i32(value: f64) -> i32 {
  clamp_round(value)
}

/// ClampFloor (i64)
#[inline]
#[must_use]
pub fn clamp_floor_i64(value: f64) -> i64 {
  clamp_floor(value)
}

/// ClampCeil (i64)
#[inline]
#[must_use]
pub fn clamp_ceil_i64(value: f64) -> i64 {
  clamp_ceil(value)
}

/// ClampRound (i64)
#[inline]
#[must_use]
pub fn clamp_round_i64(value: f64) -> i64 {
  clamp_round(value)
}

/// 有符号整数 → 同宽无符号 (SafeUnsignedAbs / as_unsigned 的静态转换面)
pub trait SignedToUnsigned: Copy + Numeric {
  type Unsigned: Copy + Numeric + PartialOrd + UnsignedWrapSub;
  fn to_unsigned(self) -> Self::Unsigned;
  fn signed_from_unsigned(value: Self::Unsigned) -> Self;
}

macro_rules! impl_signed_to_unsigned {
  ($($s:ty => $u:ty),* $(,)?) => {
    $(impl SignedToUnsigned for $s {
      type Unsigned = $u;
      #[inline] fn to_unsigned(self) -> $u { self as $u }
      #[inline] fn signed_from_unsigned(value: $u) -> Self { value as $s }
    })*
  };
}
impl_signed_to_unsigned!(i8 => u8, i16 => u16, i32 => u32, i64 => u64, isize => usize);

/// v8: src/base/numerics/safe_conversions.h SafeUnsignedAbs — 0u - x 无符号回绕绝对值
/// (原版无 DCHECK: INT_MIN 良定义地返回 2^(N-1))
#[inline]
#[must_use]
pub fn safe_unsigned_abs<T: SignedToUnsigned>(value: T) -> T::Unsigned {
  if is_value_negative(value) {
    T::Unsigned::ZERO_WRAPPING.sub_wrapping(value.to_unsigned())
  } else {
    value.to_unsigned()
  }
}

/// 无符号回绕减桥接 (SafeUnsignedAbs 内核)
pub trait UnsignedWrapSub: Copy {
  const ZERO_WRAPPING: Self;
  fn sub_wrapping(self, rhs: Self) -> Self;
}
macro_rules! impl_unsigned_wrap_sub {
  ($($t:ty),* $(,)?) => { $(impl UnsignedWrapSub for $t {
    const ZERO_WRAPPING: Self = 0;
    #[inline] fn sub_wrapping(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
  })* };
}
impl_unsigned_wrap_sub!(u8, u16, u32, u64, usize);

/// v8: src/base/numerics/safe_conversions_impl.h as_unsigned
#[inline]
#[must_use]
pub fn as_unsigned<T: SignedToUnsigned>(value: T) -> T::Unsigned {
  value.to_unsigned()
}

/// v8: src/base/numerics/safe_conversions_impl.h as_signed
#[inline]
#[must_use]
pub fn as_signed<T: SignedToUnsigned>(value: T::Unsigned) -> T {
  T::signed_from_unsigned(value)
}

/// v8: safe_conversions_impl.h kIsIntegerArithmeticSafe — T 是否可无回绕地容下 Lhs/Rhs 的运算
#[inline]
#[must_use]
pub const fn is_integer_arithmetic_safe<T: Numeric, Lhs: Numeric, Rhs: Numeric>() -> bool {
  !T::IS_FLOAT
    && !Lhs::IS_FLOAT
    && !Rhs::IS_FLOAT
    && T::IS_SIGNED >= Lhs::IS_SIGNED
    && T::BITS_PLUS_SIGN >= 2 * Lhs::BITS_PLUS_SIGN
    && T::IS_SIGNED >= Rhs::IS_SIGNED
    && T::BITS_PLUS_SIGN >= 2 * Rhs::BITS_PLUS_SIGN
}

/// v8: safe_conversions_impl.h MaxExponentPromotion — 取较大指数域的一侧
#[inline]
#[must_use]
pub const fn max_exponent_promotion_is_lhs<Lhs: Numeric, Rhs: Numeric>() -> bool {
  Lhs::MAX_EXPONENT > Rhs::MAX_EXPONENT
}

/// 提升公共类型比较 (原版 SumT = decltype(L() + R())): 双整数 i128 精确, 含浮点走 f64;
/// NaN 序外返回 None
fn cmp_common<L: Numeric, R: Numeric>(lhs: L, rhs: R) -> Option<Ordering> {
  if let (Some(a), Some(b)) = (lhs.to_i128_exact(), rhs.to_i128_exact()) {
    Some(a.cmp(&b))
  } else {
    lhs.to_f64().partial_cmp(&rhs.to_f64())
  }
}

/// v8: src/base/numerics/safe_conversions_impl.h IsLess
#[inline]
#[must_use]
pub fn is_less<L: Numeric + CastTo<R>, R: Numeric + CastTo<L>>(lhs: L, rhs: R) -> bool {
  let l_range = dst_range_relation_to_src_range::<R, L>(lhs);
  let r_range = dst_range_relation_to_src_range::<L, R>(rhs);
  l_range.underflow()
    || r_range.overflow()
    || (l_range == r_range && cmp_common(lhs, rhs) == Some(Ordering::Less))
}

/// v8: IsLessOrEqual
#[inline]
#[must_use]
pub fn is_less_or_equal<L: Numeric + CastTo<R>, R: Numeric + CastTo<L>>(lhs: L, rhs: R) -> bool {
  let l_range = dst_range_relation_to_src_range::<R, L>(lhs);
  let r_range = dst_range_relation_to_src_range::<L, R>(rhs);
  l_range.underflow()
    || r_range.overflow()
    || (l_range == r_range && cmp_common(lhs, rhs).is_some_and(|o| o != Ordering::Greater))
}

/// v8: IsGreater
#[inline]
#[must_use]
pub fn is_greater<L: Numeric + CastTo<R>, R: Numeric + CastTo<L>>(lhs: L, rhs: R) -> bool {
  let l_range = dst_range_relation_to_src_range::<R, L>(lhs);
  let r_range = dst_range_relation_to_src_range::<L, R>(rhs);
  l_range.overflow()
    || r_range.underflow()
    || (l_range == r_range && cmp_common(lhs, rhs) == Some(Ordering::Greater))
}

/// v8: IsGreaterOrEqual
#[inline]
#[must_use]
pub fn is_greater_or_equal<L: Numeric + CastTo<R>, R: Numeric + CastTo<L>>(lhs: L, rhs: R) -> bool {
  let l_range = dst_range_relation_to_src_range::<R, L>(lhs);
  let r_range = dst_range_relation_to_src_range::<L, R>(rhs);
  l_range.overflow()
    || r_range.underflow()
    || (l_range == r_range && cmp_common(lhs, rhs).is_some_and(|o| o != Ordering::Less))
}

/// v8: IsEqual
#[inline]
#[must_use]
pub fn is_equal<L: Numeric + CastTo<R>, R: Numeric + CastTo<L>>(lhs: L, rhs: R) -> bool {
  let l_range = dst_range_relation_to_src_range::<R, L>(lhs);
  let r_range = dst_range_relation_to_src_range::<L, R>(rhs);
  l_range == r_range && cmp_common(lhs, rhs) == Some(Ordering::Equal)
}

/// v8: IsNotEqual
#[inline]
#[must_use]
pub fn is_not_equal<L: Numeric + CastTo<R>, R: Numeric + CastTo<L>>(lhs: L, rhs: R) -> bool {
  let l_range = dst_range_relation_to_src_range::<R, L>(lhs);
  let r_range = dst_range_relation_to_src_range::<L, R>(rhs);
  l_range != r_range || cmp_common(lhs, rhs) != Some(Ordering::Equal)
}

/// v8: src/base/numerics/safe_conversions.h CommonMaxOrMin — 目标类型域端点选择
#[inline]
#[must_use]
pub fn common_max_or_min<T: Numeric>(is_min: bool) -> T {
  if is_min { T::min_val() } else { T::max_val() }
}

/// v8: src/base/numerics/safe_conversions_impl.h kIsMaxInRangeForNumericType
#[inline]
#[must_use]
pub fn is_max_in_range_for_numeric_type<
  D: Numeric + CastTo<S>,
  S: Numeric + CastTo<S> + CastTo<D>,
>() -> bool {
  is_greater_or_equal::<D, S>(D::max_val(), S::max_val())
}

/// v8: kIsMinInRangeForNumericType
#[inline]
#[must_use]
pub fn is_min_in_range_for_numeric_type<
  D: Numeric + CastTo<S>,
  S: Numeric + CastTo<S> + CastTo<D>,
>() -> bool {
  is_less_or_equal::<D, S>(D::min_val(), S::min_val())
}

/// v8: src/base/numerics/safe_conversions.h class StrictNumeric<T>
/// 编译期区间 requires 降为 debug_assert; transparent newtype 零开销.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct StrictNumeric<T: Numeric> {
  value: T,
}

impl<T: Numeric> StrictNumeric<T> {
  /// v8: StrictNumeric(Src value) : value_(strict_cast<T>(value))
  #[inline]
  #[must_use]
  pub const fn new(value: T) -> Self {
    Self { value }
  }

  /// v8: operator Dst() — 显式取值
  #[inline]
  #[must_use]
  pub const fn get(self) -> T {
    self.value
  }

  /// v8: operator Dst() 跨型隐式转换 (strict_cast 校验)
  #[inline]
  #[must_use]
  pub fn cast<Dst: Numeric>(self) -> Dst
  where
    T: CastTo<Dst>,
  {
    strict_cast::<Dst, T>(self.value)
  }
}

/// v8: operator! — 逻辑取反 (C++ 标量 !x: 仅 ±0.0 / 0 为 true, NaN 为 false)
impl<T: Numeric> Not for StrictNumeric<T> {
  type Output = bool;

  #[inline]
  fn not(self) -> bool {
    if T::IS_FLOAT {
      self.value.to_f64() == 0f64
    } else {
      self.value.to_i128_exact() == Some(0)
    }
  }
}

/// v8: src/base/numerics/safe_conversions.h MakeStrictNum
#[inline]
#[must_use]
pub fn make_strict_num<T: Numeric>(value: T) -> StrictNumeric<T> {
  StrictNumeric::new(value)
}
