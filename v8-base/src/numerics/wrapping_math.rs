//! 原版 v8/src/base/numerics/wrapping_math.h — 回绕加减 (modulo 2^N).
//!
//! C++ 借助无符号回绕实现; Rust 整数 `wrapping_add`/`wrapping_sub` 语义等价.

/// 原版 `WrappingAdd`/`WrappingSub` 抽象.
pub trait WrappingArith {
  fn wrapping_add_v(self, rhs: Self) -> Self;
  fn wrapping_sub_v(self, rhs: Self) -> Self;
}

macro_rules! impl_wrapping {
    ($($t:ty),*) => {
        $(impl WrappingArith for $t {
            fn wrapping_add_v(self, rhs: Self) -> Self {
                <$t>::wrapping_add(self, rhs)
            }
            fn wrapping_sub_v(self, rhs: Self) -> Self {
                <$t>::wrapping_sub(self, rhs)
            }
        })*
    };
}

impl_wrapping!(
  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

/// 原版 `v8::base::WrappingAdd(a, b)`.
pub fn wrapping_add<T: WrappingArith>(a: T, b: T) -> T {
  a.wrapping_add_v(b)
}

/// 原版 `v8::base::WrappingSub(a, b)`.
pub fn wrapping_sub<T: WrappingArith>(a: T, b: T) -> T {
  a.wrapping_sub_v(b)
}
