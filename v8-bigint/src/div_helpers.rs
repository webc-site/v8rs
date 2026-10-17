// 由手工从 v8/src/bigint/div-helpers-inl.h 转写; 对应原版逐行
//! 除法算法共享的辅助: 复制 / digit 内移位 / ShiftedDigits

use crate::bigint::{Digit, K_DIGIT_BITS, K_NEWTON_INVERSION_THRESHOLD, count_leading_zeros, norm};

/// Barrett 除法的 scratch 需求
#[inline]
pub const fn divide_barrett_scratch_space(n: u32) -> u32 {
  n + 2
}

/// Newton 求逆的局部 S/W/U 额外空间
pub const K_INVERT_NEWTON_EXTRA_SPACE: u32 = 5;

#[inline]
pub const fn invert_newton_scratch_space(n: u32) -> u32 {
  3 * n + 2 * K_INVERT_NEWTON_EXTRA_SPACE
}

#[inline]
pub const fn invert_scratch_space(n: u32) -> u32 {
  if n < K_NEWTON_INVERSION_THRESHOLD {
    2 * n
  } else {
    invert_newton_scratch_space(n)
  }
}

/// Z := X (X 不足 Z 长则补零)
pub fn copy(z: &mut [Digit], x: &[Digit]) {
  let mut i = 0;
  while i < x.len() {
    z[i] = x[i];
    i += 1;
  }
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
}

/// Z := X << shift, shift < kDigitBits; Z 与 X 允许同一缓冲 (就地)
pub fn left_shift(z: &mut [Digit], x: &[Digit], shift: u32) {
  debug_assert!(shift < K_DIGIT_BITS);
  debug_assert!(z.len() >= x.len());
  if shift == 0 {
    return copy(z, x);
  }
  let mut carry = 0;
  let mut i = 0;
  while i < x.len() {
    let d = x[i];
    z[i] = (d << shift) | carry;
    carry = d >> (K_DIGIT_BITS - shift);
    i += 1;
  }
  if i < z.len() {
    z[i] = carry;
    i += 1;
  } else {
    debug_assert!(carry == 0);
  }
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
}

/// Z := X >> shift, shift < kDigitBits; Z 与 X 允许同一缓冲 (就地)
pub fn right_shift(z: &mut [Digit], x: &[Digit], shift: u32) {
  debug_assert!(shift < K_DIGIT_BITS);
  let x = norm(x);
  debug_assert!(z.len() >= x.len());
  if shift == 0 {
    return copy(z, x);
  }
  let mut i = 0;
  if !x.is_empty() {
    let mut carry = x[0] >> shift;
    let last = x.len() - 1;
    while i < last {
      let d = x[i + 1];
      z[i] = (d << (K_DIGIT_BITS - shift)) | carry;
      carry = d >> shift;
      i += 1;
    }
    z[i] = carry;
    i += 1;
  }
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
}

/// 把 A 复制到 Z 的低 count 位, 不足补零
pub fn put_at(z: &mut [Digit], a: &[Digit], count: usize) {
  let len = a.len().min(count);
  z[..len].copy_from_slice(&a[..len]);
  for d in z[len..count].iter_mut() {
    *d = 0;
  }
}

/// 左移到 bit 规范化的输入视图 (原版 ShiftedDigits)
/// shift = None 自动取 msd 前导零; 请求的 shift 超过前导零时结果多一个 digit
pub struct ShiftedDigits {
  /// 实际左移量
  pub shift: u32,
  data: Vec<Digit>,
}

impl ShiftedDigits {
  pub fn new(original: &[Digit], shift: Option<u32>) -> Self {
    if original.is_empty() {
      return Self {
        shift: 0,
        data: Vec::new(),
      };
    }
    let leading_zeros = count_leading_zeros(original[original.len() - 1]);
    let mut len = original.len();
    let shift = match shift {
      None => leading_zeros,
      Some(s) => {
        if s > leading_zeros {
          // 放不下, 需要一个额外 digit
          len += 1;
        }
        s
      }
    };
    let mut buf = vec![0; len];
    left_shift(&mut buf, original, shift);
    Self { shift, data: buf }
  }

  /// 移位后的 digits
  #[inline]
  pub fn digits(&self) -> &[Digit] {
    &self.data
  }
}
