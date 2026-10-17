//! 原版 v8/src/base/numerics/checked_math.h — 溢出即失效的算术包装

use crate::immediate_crash::debug_unreachable;

/// 可参与 Checked 运算的数值 (对应 std::is_arithmetic + CheckedNumericImpl)
pub trait CheckedOps: Copy + PartialOrd {
  fn checked_add_impl(self, rhs: Self) -> Option<Self>;
  fn checked_sub_impl(self, rhs: Self) -> Option<Self>;
  fn checked_mul_impl(self, rhs: Self) -> Option<Self>;
  fn checked_div_impl(self, rhs: Self) -> Option<Self>;
}

macro_rules! impl_checked_ops_int {
  ($($t:ty),*) => {
    $(impl CheckedOps for $t {
      fn checked_add_impl(self, rhs: Self) -> Option<Self> { <$t>::checked_add(self, rhs) }
      fn checked_sub_impl(self, rhs: Self) -> Option<Self> { <$t>::checked_sub(self, rhs) }
      fn checked_mul_impl(self, rhs: Self) -> Option<Self> { <$t>::checked_mul(self, rhs) }
      // 除零与 INT_MIN/-1 均失效 (原版 IsDivisionValid 语义)
      fn checked_div_impl(self, rhs: Self) -> Option<Self> { <$t>::checked_div(self, rhs) }
    })*
  };
}
impl_checked_ops_int!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl CheckedOps for f32 {
  fn checked_add_impl(self, rhs: Self) -> Option<Self> {
    let r = self + rhs;
    (r.is_finite()).then_some(r)
  }
  fn checked_sub_impl(self, rhs: Self) -> Option<Self> {
    let r = self - rhs;
    (r.is_finite()).then_some(r)
  }
  fn checked_mul_impl(self, rhs: Self) -> Option<Self> {
    let r = self * rhs;
    (r.is_finite()).then_some(r)
  }
  fn checked_div_impl(self, rhs: Self) -> Option<Self> {
    let r = self / rhs;
    (r.is_finite()).then_some(r)
  }
}

impl CheckedOps for f64 {
  fn checked_add_impl(self, rhs: Self) -> Option<Self> {
    let r = self + rhs;
    (r.is_finite()).then_some(r)
  }
  fn checked_sub_impl(self, rhs: Self) -> Option<Self> {
    let r = self - rhs;
    (r.is_finite()).then_some(r)
  }
  fn checked_mul_impl(self, rhs: Self) -> Option<Self> {
    let r = self * rhs;
    (r.is_finite()).then_some(r)
  }
  fn checked_div_impl(self, rhs: Self) -> Option<Self> {
    let r = self / rhs;
    (r.is_finite()).then_some(r)
  }
}

/// Checked<T> — 运算溢出后进入失效态, 后续运算保持失效 (原版行为)
#[derive(Clone, Copy, Debug)]
pub struct Checked<T: CheckedOps> {
  value_: T,
  is_valid_: bool,
}

impl<T: CheckedOps> Checked<T> {
  /// 由数值构造 (总有效)
  #[must_use]
  pub const fn new(value: T) -> Self {
    Self {
      value_: value,
      is_valid_: true,
    }
  }

  /// 失效态构造 (ValueOrDie 会 crash, 对应原版 InvalidCheckedValue)
  #[must_use]
  pub const fn invalid(value: T) -> Self {
    Self {
      value_: value,
      is_valid_: false,
    }
  }

  #[must_use]
  pub const fn is_valid(&self) -> bool {
    self.is_valid_
  }

  #[must_use]
  pub const fn has_overflowed(&self) -> bool {
    !self.is_valid_
  }

  /// value_dcheck — 原版 DCHECK 语义: 失效时不可达崩溃
  #[must_use]
  pub fn value_dcheck(&self) -> T {
    debug_assert!(self.is_valid_, "Checked value has overflowed");
    if self.is_valid_ {
      self.value_
    } else {
      unsafe { debug_unreachable() }
    }
  }

  /// 取回结果; 失效返回 None (Rust 化替代 ValueOrFail)
  #[must_use]
  pub const fn value(&self) -> Option<T> {
    if self.is_valid_ {
      Some(self.value_)
    } else {
      None
    }
  }

  #[must_use]
  pub fn add(&self, rhs: &Self) -> Self {
    // 失效传播: 任一失效即失效
    if !self.is_valid_ || !rhs.is_valid_ {
      return Self::invalid(self.value_);
    }
    match self.value_.checked_add_impl(rhs.value_) {
      Some(v) => Self::new(v),
      None => Self::invalid(self.value_),
    }
  }

  #[must_use]
  pub fn subtract(&self, rhs: &Self) -> Self {
    if !self.is_valid_ || !rhs.is_valid_ {
      return Self::invalid(self.value_);
    }
    match self.value_.checked_sub_impl(rhs.value_) {
      Some(v) => Self::new(v),
      None => Self::invalid(self.value_),
    }
  }

  #[must_use]
  pub fn multiply(&self, rhs: &Self) -> Self {
    if !self.is_valid_ || !rhs.is_valid_ {
      return Self::invalid(self.value_);
    }
    match self.value_.checked_mul_impl(rhs.value_) {
      Some(v) => Self::new(v),
      None => Self::invalid(self.value_),
    }
  }

  #[must_use]
  pub fn divide(&self, rhs: &Self) -> Self {
    if !self.is_valid_ || !rhs.is_valid_ {
      return Self::invalid(self.value_);
    }
    match self.value_.checked_div_impl(rhs.value_) {
      Some(v) => Self::new(v),
      None => Self::invalid(self.value_),
    }
  }
}

/// CheckedInt/CheckedUint 类型别名 (常用)
pub type CheckedInt32 = Checked<i32>;
/// 无符号 32 位
pub type CheckedUint32 = Checked<u32>;
/// 有符号 64 位
pub type CheckedInt64 = Checked<i64>;
/// 无符号 64 位
pub type CheckedUint64 = Checked<u64>;
/// size_t
pub type CheckedSize = Checked<usize>;
/// 32 位平台指针差 (此处即 isize)
pub type CheckedSsize = Checked<isize>;
