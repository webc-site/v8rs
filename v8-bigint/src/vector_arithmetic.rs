// 由手工从 v8/src/bigint/vector-arithmetic-inl.h 转写; 对应原版逐行
//! 作用于 digit 向量的辅助算术

use crate::bigint::{
  Digit, K_DIGIT_BITS, count_leading_zeros, digit_add2, digit_add3, digit_sub, digit_sub2, msd,
  norm,
};

/// Z += X, 返回溢出进位
pub fn add_and_return_overflow(z: &mut [Digit], x: &[Digit]) -> Digit {
  let x = norm(x);
  if x.is_empty() {
    return 0;
  }
  debug_assert!(z.len() >= x.len());
  let mut carry = 0;
  let mut i = 0;
  while i < x.len() {
    let (v, c) = digit_add3(z[i], x[i], carry);
    z[i] = v;
    carry = c;
    i += 1;
  }
  while i < z.len() && carry != 0 {
    let (v, c) = digit_add2(z[i], carry);
    z[i] = v;
    carry = c;
    i += 1;
  }
  carry
}

/// Z -= X, 返回溢出借位
pub fn sub_and_return_borrow(z: &mut [Digit], x: &[Digit]) -> Digit {
  let x = norm(x);
  if x.is_empty() {
    return 0;
  }
  debug_assert!(z.len() >= x.len());
  let mut borrow = 0;
  let mut i = 0;
  while i < x.len() {
    let (v, b) = digit_sub2(z[i], x[i], borrow);
    z[i] = v;
    borrow = b;
    i += 1;
  }
  while i < z.len() && borrow != 0 {
    let (v, b) = digit_sub(z[i], borrow);
    z[i] = v;
    borrow = b;
    i += 1;
  }
  borrow
}

/// 最高位是否已置 1 (bit 规范化)
#[inline]
pub fn is_bit_normalized(x: &[Digit]) -> bool {
  (msd(x) >> (K_DIGIT_BITS - 1)) == 1
}

/// 位长 (按规范化输入)
#[inline]
pub fn bit_length_digits(x: &[Digit]) -> u32 {
  let x = norm(x);
  if x.is_empty() {
    return 0;
  }
  (x.len() as u32) * K_DIGIT_BITS - count_leading_zeros(msd(x))
}
