//! 原版 v8/src/base/numerics/clamped_math.h — 溢出即饱和的算术包装

/// 可参与 Clamped 运算的数值 (整数走 saturating_*, 浮点走原生舍入)
pub trait ClampedOps: Copy {
  fn saturated_add_impl(self, rhs: Self) -> Self;
  fn saturated_sub_impl(self, rhs: Self) -> Self;
  fn saturated_mul_impl(self, rhs: Self) -> Self;
  /// 除零/无效情形原版返回 0
  fn saturated_div_impl(self, rhs: Self) -> Self;
}

macro_rules! impl_clamped_int {
  ($($t:ty),*) => {
    $(impl ClampedOps for $t {
      fn saturated_add_impl(self, rhs: Self) -> Self { <$t>::saturating_add(self, rhs) }
      fn saturated_sub_impl(self, rhs: Self) -> Self { <$t>::saturating_sub(self, rhs) }
      fn saturated_mul_impl(self, rhs: Self) -> Self { <$t>::saturating_mul(self, rhs) }
      fn saturated_div_impl(self, rhs: Self) -> Self {
        if rhs == 0 { 0 } else { self.saturating_div(rhs) }
      }
    })*
  };
}
impl_clamped_int!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

macro_rules! impl_clamped_float {
  ($($t:ty),*) => {
    $(impl ClampedOps for $t {
      fn saturated_add_impl(self, rhs: Self) -> Self { self + rhs }
      fn saturated_sub_impl(self, rhs: Self) -> Self { self - rhs }
      fn saturated_mul_impl(self, rhs: Self) -> Self { self * rhs }
      fn saturated_div_impl(self, rhs: Self) -> Self {
        // 原版: 除零产生 inf, 但浮点保持 inf (DCHECK 关闭时)
        self / rhs
      }
    })*
  };
}
impl_clamped_float!(f32, f64);

/// Clamped<T> — 结果总饱和到类型边界 (对应 ClampedValue/Clamped)
#[derive(Clone, Copy, Debug)]
pub struct Clamped<T: ClampedOps> {
  value_: T,
}

impl<T: ClampedOps> Clamped<T> {
  /// ClampedValue
  #[must_use]
  pub const fn new(value: T) -> Self {
    Self { value_: value }
  }

  /// 取回值 (原版无失效态)
  #[must_use]
  pub const fn value(&self) -> T {
    self.value_
  }

  #[must_use]
  pub fn add(&self, rhs: &Self) -> Self {
    Self::new(self.value_.saturated_add_impl(rhs.value_))
  }

  #[must_use]
  pub fn subtract(&self, rhs: &Self) -> Self {
    Self::new(self.value_.saturated_sub_impl(rhs.value_))
  }

  #[must_use]
  pub fn multiply(&self, rhs: &Self) -> Self {
    Self::new(self.value_.saturated_mul_impl(rhs.value_))
  }

  #[must_use]
  pub fn divide(&self, rhs: &Self) -> Self {
    Self::new(self.value_.saturated_div_impl(rhs.value_))
  }
}

/// ClampedInt32
pub type ClampedInt32 = Clamped<i32>;
/// ClampedUint32
pub type ClampedUint32 = Clamped<u32>;
/// ClampedInt64
pub type ClampedInt64 = Clamped<i64>;
/// ClampedSize
pub type ClampedSize = Clamped<usize>;
