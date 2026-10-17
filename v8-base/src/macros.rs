//! v8/src/base/macros.h 的 1:1 转写 — 运行期语义辅助函数
//!
//! 纯预处理机制 (CONCAT/EXPAND/COUNT_MACRO_ARGS/V8PRIxPTR 等) 在 Rust 无对应,
//! 由语言自身承载, 不迁移 (登记)。本模块承接 macros.h 中有运行期语义的部分:
//! make_uint64 / RoundDown / RoundUp / IsAligned / AlignedAddress / is_inbounds。
//! bit_cast/implicit_cast/USE/arraysize 在 Rust 由 as/形参/切片长度天然表达。

use core::{
  fmt,
  ops::{Add, Sub},
};

/// make_uint64(high, low): 两个 u32 拼一个 u64 (原版 macros.h:408)
#[inline]
#[must_use]
pub const fn make_uint64(high: u32, low: u32) -> u64 {
  ((high as u64) << 32) + low as u64
}

/// 位掩码 RoundDown 载体: `x & static_cast<T>(-m)` 的截断语义
pub trait RoundMask: Copy {
  /// static_cast<T>(v): i64 位模式截断到本类型
  fn from_i64(v: i64) -> Self;
  fn and(self, mask: Self) -> Self;
  fn wrapping_add(self, rhs: Self) -> Self;
  fn wrapping_sub(self, rhs: Self) -> Self;
  fn as_i64(self) -> i64;
  /// 溢出预检上限 (u64/usize 无 → None)
  fn max_i64() -> Option<i64>;
}

macro_rules! impl_round_mask {
  ($($t:ty),*) => {$(
    impl RoundMask for $t {
      #[inline] fn from_i64(v: i64) -> Self { v as Self }
      #[inline] fn and(self, mask: Self) -> Self { self & mask }
      #[inline] fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
      #[inline] fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
      #[inline] fn as_i64(self) -> i64 { self as i64 }
      #[inline] fn max_i64() -> Option<i64> { i64::try_from(<$t>::MAX).ok() }
    }
  )*};
}

impl_round_mask!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

/// RoundDown(x, m): 不大于 x 的 m 的最大倍数, m 须为 2 的幂
/// (原版 macros.h:414-419; `x & static_cast<T>(-m)`)
#[inline]
#[must_use]
pub fn round_down<T: RoundMask>(x: T, m: i64) -> T {
  debug_assert!(m != 0 && (m & (m - 1)) == 0, "m 必须是 2 的幂");
  x.and(T::from_i64(m.wrapping_neg()))
}

/// RoundUp(x, m): 不小于 x 的 m 的最小倍数, m 须为 2 的幂
/// (原版 macros.h:430-436; DCHECK(x >= 0) 与溢出预检)
#[inline]
#[must_use]
pub fn round_up<T: RoundMask + PartialOrd + Default + fmt::Debug>(x: T, m: i64) -> T {
  debug_assert!(x >= T::default(), "RoundUp 要求 x >= 0");
  // 原版 DCHECK_GE(max - x, m - 1): u64/usize 超出 i64 全量程时跳过
  if let Some(max) = T::max_i64() {
    debug_assert!(max - x.as_i64() >= m - 1, "RoundUp 溢出");
  }
  round_down(x.wrapping_add(T::from_i64(m - 1)), m)
}

/// RoundDown<m>(x): m 编译期常量形态 (原版 macros.h:421-425, static_assert 对应 const 块)
#[inline]
#[must_use]
pub fn round_down_c<const M: i64, T: RoundMask>(x: T) -> T {
  const { assert!(M != 0 && (M & (M - 1)) == 0, "m 必须是 2 的幂") }
  x.and(T::from_i64(M.wrapping_neg()))
}

/// RoundUp<m>(x): m 编译期常量形态 (原版 macros.h:438-443)
#[inline]
#[must_use]
pub fn round_up_c<const M: i64, T: RoundMask + PartialOrd + Default + fmt::Debug>(x: T) -> T {
  round_up(x, M)
}

/// IsAligned(value, alignment): (value & (alignment - 1)) == 0
/// (原版 macros.h:446-449; 原版无 alignment 校验, 保持一致)
#[inline]
#[must_use]
pub fn is_aligned<T: RoundMask>(value: T, alignment: T) -> bool {
  value.and(alignment.wrapping_sub(T::from_i64(1))).as_i64() == 0
}

/// IsAlignedAddress (原版 macros.h:451-454, reinterpret_cast 承接)
#[inline]
#[must_use]
pub const fn is_aligned_address(address: usize, alignment: usize) -> bool {
  address & alignment.wrapping_sub(1) == 0
}

/// AlignedAddress: 向下对齐地址 (原版 macros.h:456-461)
#[inline]
#[must_use]
pub const fn aligned_address(address: usize, alignment: usize) -> usize {
  address & alignment.wrapping_neg()
}

/// RoundUpAddress: 向上对齐地址 (原版 macros.h:462-467)
#[inline]
#[must_use]
pub const fn round_up_address(address: usize, alignment: usize) -> usize {
  // 原版走 RoundUp (含溢出 DCHECK); const fn 内以回绕承接, 越界由调用方契约保证
  address.wrapping_add(alignment.wrapping_sub(1)) & alignment.wrapping_neg()
}

// ---- is_inbounds: 浮点 → 整数截断转换的界检查 (原版 macros.h:470-486) ----

/// is_inbounds 的整数目标类型规格 (sizeof(int_t) < sizeof(i64) 才可界检)
pub trait InboundsInt {
  const MIN_I: i64;
  const MAX_I: i64;
}

macro_rules! impl_inbounds_int {
  ($($t:ty),*) => {$(
    impl InboundsInt for $t {
      const MIN_I: i64 = <$t>::MIN as i64;
      const MAX_I: i64 = <$t>::MAX as i64;
    }
  )*};
}

impl_inbounds_int!(i8, i16, i32, u8, u16, u32);

/// is_inbounds 的浮点源类型规格 (float_t: f32/f64)
pub trait InboundsFloat: Copy + PartialOrd + Add<Output = Self> + Sub<Output = Self> {
  const ONE: Self;
  fn from_i64(v: i64) -> Self;
  /// float_t → i64 (界内常量不越 i64 界, 与 C++ cast 等价)
  fn to_i64(self) -> i64;
}

impl InboundsFloat for f64 {
  const ONE: Self = 1.0;
  #[inline]
  fn from_i64(v: i64) -> Self {
    v as Self
  }
  #[inline]
  fn to_i64(self) -> i64 {
    self as i64
  }
}

impl InboundsFloat for f32 {
  const ONE: Self = 1.0;
  #[inline]
  fn from_i64(v: i64) -> Self {
    v as Self
  }
  #[inline]
  fn to_i64(self) -> i64 {
    self as i64
  }
}

/// is_inbounds<int_t, float_t>(v): v 经截断可无损落入 int_t
/// 合法域为开区间 (min-1, max+1); 边界常数先按 float_t 精度回绕再与整数界比对
/// (kLowerBoundIsMin / kUpperBoundIsMax 决定开闭)
/// (原版 macros.h:470-486)
#[inline]
#[must_use]
pub fn is_inbounds<I: InboundsInt, F: InboundsFloat>(v: F) -> bool {
  let lower = F::from_i64(I::MIN_I) - F::ONE;
  let upper = F::from_i64(I::MAX_I) + F::ONE;
  let lower_is_min = lower.to_i64() == I::MIN_I;
  let upper_is_max = upper.to_i64() == I::MAX_I;
  (if lower_is_min { lower <= v } else { lower < v })
    && (if upper_is_max { v <= upper } else { v < upper })
}
