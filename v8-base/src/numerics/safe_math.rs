//! 原版 v8/src/base/numerics/safe_math.h — 聚合 checked/clamped/safe_conversions.
//!
//! C++ 该头仅做 include 聚合, 另提供 variadic `CheckedAdd(...)` 等自由函数与
//! 运算符重载. Rust 无运算符重载, 以自由函数表达同样的多参数语义.

pub use super::{
  checked_math::{Checked, CheckedOps},
  clamped_math::{Clamped, ClampedOps},
  safe_conversions::*,
};

/// 原版 variadic `checked_add(...)`: 链式相加, 任一溢出即失效.
#[must_use]
pub fn checked_add<T: CheckedOps>(lhs: T, rhs: T) -> Checked<T> {
  Checked::new(lhs).add(&Checked::new(rhs))
}

/// 原版 variadic `checked_sub(...)`.
#[must_use]
pub fn checked_sub<T: CheckedOps>(lhs: T, rhs: T) -> Checked<T> {
  Checked::new(lhs).subtract(&Checked::new(rhs))
}

/// 原版 variadic `checked_mul(...)`.
#[must_use]
pub fn checked_mul<T: CheckedOps>(lhs: T, rhs: T) -> Checked<T> {
  Checked::new(lhs).multiply(&Checked::new(rhs))
}

/// 原版 variadic `checked_div(...)`.
#[must_use]
pub fn checked_div<T: CheckedOps>(lhs: T, rhs: T) -> Checked<T> {
  Checked::new(lhs).divide(&Checked::new(rhs))
}

/// 原版 variadic `clamped_add(...)`: 饱和相加.
#[must_use]
pub fn clamped_add<T: ClampedOps>(lhs: T, rhs: T) -> Clamped<T> {
  Clamped::new(lhs).add(&Clamped::new(rhs))
}

/// 原版 variadic `clamped_sub(...)`.
#[must_use]
pub fn clamped_sub<T: ClampedOps>(lhs: T, rhs: T) -> Clamped<T> {
  Clamped::new(lhs).subtract(&Clamped::new(rhs))
}

/// 原版 variadic `clamped_mul(...)`.
#[must_use]
pub fn clamped_mul<T: ClampedOps>(lhs: T, rhs: T) -> Clamped<T> {
  Clamped::new(lhs).multiply(&Clamped::new(rhs))
}

/// 原版 variadic `clamped_div(...)`.
#[must_use]
pub fn clamped_div<T: ClampedOps>(lhs: T, rhs: T) -> Clamped<T> {
  Clamped::new(lhs).divide(&Clamped::new(rhs))
}
