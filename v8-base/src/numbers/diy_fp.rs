//! 原版 v8/src/base/numbers/diy-fp.h/cc — "Do It Yourself Floating Point"

/// DiyFp: uint64 尾数 + int 指数; 规格化数最高位置 1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiyFp {
  f_: u64,
  e_: i32,
}

impl Default for DiyFp {
  fn default() -> Self {
    Self::new(0, 0)
  }
}

impl DiyFp {
  /// kSignificandSize
  pub const SIGNIFICAND_SIZE: i32 = 64;
  const UINT64_MSB: u64 = 1 << 63;

  #[must_use]
  pub const fn new(f: u64, e: i32) -> Self {
    Self { f_: f, e_: e }
  }

  /// this = this - other; 指数必须相同且 this.f >= other.f
  pub fn subtract(&mut self, other: Self) {
    debug_assert_eq!(self.e_, other.e_);
    debug_assert!(self.f_ >= other.f_);
    self.f_ -= other.f_;
  }

  /// a - b (不规格化)
  #[must_use]
  pub fn minus(a: Self, b: Self) -> Self {
    let mut result = a;
    result.subtract(b);
    result
  }

  /// this = this * other
  pub fn multiply(&mut self, other: Self) {
    *self = Self::times(*self, other);
  }

  /// a * b — 128 位乘积取高 64 (+ lo>>63), 指数相加 + 64
  #[must_use]
  pub fn times(a: Self, b: Self) -> Self {
    let mul = u128::from(a.f_) * u128::from(b.f_);
    let hi = (mul >> 64) as u64;
    let lo = mul as u64;
    Self::new(hi + (lo >> 63), a.e_ + b.e_ + 64)
  }

  /// 规格化: 最高位置 1 (先按 10 位步进优化边界情形)
  pub fn normalize(&mut self) {
    debug_assert_ne!(self.f_, 0);
    let mut f = self.f_;
    let mut e = self.e_;

    const K10_MS_BITS: u64 = 0x3FF << 54;
    while f & K10_MS_BITS == 0 {
      f <<= 10;
      e -= 10;
    }
    while f & Self::UINT64_MSB == 0 {
      f <<= 1;
      e -= 1;
    }
    self.f_ = f;
    self.e_ = e;
  }

  #[must_use]
  pub fn normalize_new(a: Self) -> Self {
    let mut result = a;
    result.normalize();
    result
  }

  #[must_use]
  pub const fn f(&self) -> u64 {
    self.f_
  }

  #[must_use]
  pub const fn e(&self) -> i32 {
    self.e_
  }

  pub const fn set_f(&mut self, new_value: u64) {
    self.f_ = new_value;
  }

  pub const fn set_e(&mut self, new_value: i32) {
    self.e_ = new_value;
  }
}
