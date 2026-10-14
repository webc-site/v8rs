//! 原版 v8/src/base/numerics/ranges.h — 近似相等判定.

use core::ops::Sub;

/// 原版 `IsApproximatelyEqual`: `abs(rhs - lhs) <= tolerance`.
pub fn is_approximately_equal<T>(lhs: T, rhs: T, tolerance: T) -> bool
where
  T: Copy + Sub<Output = T> + AbsLike + PartialOrd,
{
  (rhs - lhs).abs_like() <= tolerance
}

/// abs 抽象 (f32/f64/整数), 避免引入外部泛型约束.
pub trait AbsLike {
  fn abs_like(self) -> Self;
}

macro_rules! impl_abs_float {
  ($t:ty) => {
    impl AbsLike for $t {
      fn abs_like(self) -> Self {
        <$t>::abs(self)
      }
    }
  };
}
macro_rules! impl_abs_int {
  ($t:ty) => {
    impl AbsLike for $t {
      fn abs_like(self) -> Self {
        <$t>::abs(self)
      }
    }
  };
}

impl_abs_float!(f32);
impl_abs_float!(f64);
impl_abs_int!(i8);
impl_abs_int!(i16);
impl_abs_int!(i32);
impl_abs_int!(i64);
impl_abs_int!(isize);
