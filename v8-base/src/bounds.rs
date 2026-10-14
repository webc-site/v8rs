//! v8/src/base/bounds.h 的 1:1 转写 — 单分支区间判定族
//!
//! 原版模板以 `make_unsigned_t` 回绕减法实现单分支判定
//! (value - lower <= higher - lower); Rust 以同宽无符号 wrapping 减法逐条对齐。
//! 原版 `sizeof(U) <= sizeof(T)` 的混合宽度约束: C++ 隐式提升后同样落到 T 宽度
//! 无符号运算, Rust 调用方先统一到 T 即等价。指针重载以 usize 承接
//! (reinterpret_cast<uintptr_t> 等价)。

use core::ops::Sub;

/// 同宽无符号回绕载体
pub trait InRangeCarrier: Copy + PartialOrd + Default {
  type U: Copy + PartialOrd + Sub<Output = Self::U>;
  fn to_u(self) -> Self::U;
  fn from_u(u: Self::U) -> Self;
  fn wrapping_sub_val(self, rhs: Self) -> Self;
  fn wrapping_sub_u(u: Self::U, rhs: Self::U) -> Self::U;
}

macro_rules! impl_carrier {
  ($($s:ty => $u:ty),* $(,)?) => {$(
    impl InRangeCarrier for $s {
      type U = $u;
      #[inline] fn to_u(self) -> $u { self as $u }
      #[inline] fn from_u(u: $u) -> Self { u as Self }
      #[inline] fn wrapping_sub_val(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
      #[inline] fn wrapping_sub_u(u: $u, rhs: $u) -> $u { u.wrapping_sub(rhs) }
    }
  )*};
}

impl_carrier!(i8 => u8, i16 => u16, i32 => u32, i64 => u64, isize => usize, u8 => u8, u16 => u16, u32 => u32, u64 => u64, usize => usize);

/// IsInRange: value ∈ [lower, higher], 单分支无符号回绕判定
/// (原版 bounds.h:16-28)
#[inline]
#[must_use]
pub fn is_in_range<T: InRangeCarrier>(value: T, lower: T, higher: T) -> bool {
  debug_assert!(lower <= higher);
  let v = T::wrapping_sub_u(T::to_u(value), T::to_u(lower));
  let span = T::wrapping_sub_u(T::to_u(higher), T::to_u(lower));
  v <= span
}

/// IsInRange 指针重载 (原版 bounds.h:32-37, reinterpret_cast 承接;
/// bounds.h:34 的 DCHECK_LE 由委托的 is_in_range 内断言承接)
#[inline]
#[must_use]
pub fn is_in_range_ptr(value: *const (), lower: *const (), higher: *const ()) -> bool {
  is_in_range(value as usize, lower as usize, higher as usize)
}

/// IsInHalfOpenRange: value ∈ [lower, higher) (原版 bounds.h:44-56)
#[inline]
#[must_use]
pub fn is_in_half_open_range<T: InRangeCarrier>(value: T, lower: T, higher: T) -> bool {
  debug_assert!(lower <= higher);
  let v = T::wrapping_sub_u(T::to_u(value), T::to_u(lower));
  let span = T::wrapping_sub_u(T::to_u(higher), T::to_u(lower));
  v < span
}

/// IsInBounds: [index, index+length) ⊆ [0, max), index+length 回绕亦正确
/// (原版 bounds.h:61-65, requires 无符号; U = T 约束保证同宽无符号)
#[inline]
#[must_use]
pub fn is_in_bounds<T: InRangeCarrier<U = T>>(index: T, length: T, max: T) -> bool {
  length <= max && index <= max.wrapping_sub_val(length)
}

/// ClampToBounds: 越界时把 length 钳到可用余量, 返回是否本就合法
/// (原版 bounds.h:70-82, requires 无符号)
#[inline]
#[must_use]
pub fn clamp_to_bounds<T: InRangeCarrier<U = T> + Default>(
  index: T,
  length: &mut T,
  max: T,
) -> bool {
  if index > max {
    *length = T::default();
    return false;
  }
  let avail = max.wrapping_sub_val(index);
  let oob = *length > avail;
  if oob {
    *length = avail;
  }
  !oob
}
