//! 原版 conversions.cc 43-339 — SignificandToChars 快速写出族 + SimpleStringBuilder
//!
//! SignificandToChars 及其辅助受 dragonbox::to_chars 启发, 用两张 200 项
//! 查表 (见 [tables]) 以 2 位/步的粒度把 u64 尾数写成十进制字符。

use v8_base::numbers::double::Double;

use super::{
  QUIET_NAN_MASK,
  tables::{RADIX_100_HEAD_TABLE, RADIX_100_TABLE},
};

/// kUint32Mask — u32 低 32 位截取
const K_UINT32_MASK: u64 = u32::MAX as u64;

/// Convert2Digits — 两位数字 n 写入 buffer[0..2]
#[inline]
fn convert_2_digits(n: u8, buffer: &mut [u8]) {
  debug_assert!((n as usize) < RADIX_100_TABLE.len() / 2);
  let base = n as usize * 2;
  buffer[0] = RADIX_100_TABLE[base];
  buffer[1] = RADIX_100_TABLE[base + 1];
}

/// ConvertHeadDigits — 头部 1~2 位数字, 返回写出位数
#[inline]
pub fn convert_head_digits(n: u8, buffer: &mut [u8]) -> u8 {
  debug_assert!((n as usize) < RADIX_100_HEAD_TABLE.len() / 2);
  let digit_count = 1 + u8::from(n >= 10);
  debug_assert!(digit_count <= 2);
  let base = n as usize * 2;
  buffer[0] = RADIX_100_HEAD_TABLE[base];
  if digit_count == 2 {
    buffer[1] = RADIX_100_HEAD_TABLE[base + 1];
  }
  digit_count
}

/// Convert8Digits — 恰 8 位数字 (n < 1e8), 乘法移位快速分位
fn convert_8_digits(n: u32, buffer: &mut [u8]) {
  // 281474978 = ceil(2^48 / 1e6) + 1
  let mut prod = n as u64 * 281_474_978;
  prod >>= 16;
  prod += 1;
  convert_2_digits((prod >> 32) as u8, &mut buffer[..2]);
  prod = (prod & K_UINT32_MASK) * 100;
  convert_2_digits((prod >> 32) as u8, &mut buffer[2..4]);
  prod = (prod & K_UINT32_MASK) * 100;
  convert_2_digits((prod >> 32) as u8, &mut buffer[4..6]);
  prod = (prod & K_UINT32_MASK) * 100;
  convert_2_digits((prod >> 32) as u8, &mut buffer[6..8]);
}

/// ConvertUpTo9Digits — 1..=9 位数字, 返回写出位数 (原版非 static 符号)
#[must_use]
pub fn convert_up_to_9_digits(n: u32, buffer: &mut [u8]) -> u8 {
  if n >= 100_000_000 {
    // 9 位: 1441151882 = ceil(2^57 / 1e8) + 1
    let mut prod = n as u64 * 1_441_151_882;
    prod >>= 25;

    let head_digit = (prod >> 32) as u8;
    debug_assert!(head_digit < 10);
    buffer[0] = b'0' + head_digit;

    // 余 8 位
    prod = (prod & K_UINT32_MASK) * 100;
    convert_2_digits((prod >> 32) as u8, &mut buffer[1..3]);
    prod = (prod & K_UINT32_MASK) * 100;
    convert_2_digits((prod >> 32) as u8, &mut buffer[3..5]);
    prod = (prod & K_UINT32_MASK) * 100;
    convert_2_digits((prod >> 32) as u8, &mut buffer[5..7]);
    prod = (prod & K_UINT32_MASK) * 100;
    convert_2_digits((prod >> 32) as u8, &mut buffer[7..9]);
    return 9;
  }
  if n >= 1_000_000 {
    // 7 或 8 位: 281474978 = ceil(2^48 / 1e6) + 1
    let mut prod = n as u64 * 281_474_978;
    prod >>= 16;

    let head_digits = (prod >> 32) as u8;
    let head_digit_count = convert_head_digits(head_digits, buffer);
    let buffer = &mut buffer[head_digit_count as usize..];

    // 余 6 位
    prod = (prod & K_UINT32_MASK) * 100;
    convert_2_digits((prod >> 32) as u8, &mut buffer[..2]);
    prod = (prod & K_UINT32_MASK) * 100;
    convert_2_digits((prod >> 32) as u8, &mut buffer[2..4]);
    prod = (prod & K_UINT32_MASK) * 100;
    convert_2_digits((prod >> 32) as u8, &mut buffer[4..6]);
    return 6 + head_digit_count;
  }
  if n >= 10_000 {
    // 5 或 6 位: 429497 = ceil(2^32 / 1e4)
    let mut prod = n as u64 * 429_497;

    let head_digits = (prod >> 32) as u8;
    let head_digit_count = convert_head_digits(head_digits, buffer);
    let buffer = &mut buffer[head_digit_count as usize..];

    // 余 4 位
    prod = (prod & K_UINT32_MASK) * 100;
    convert_2_digits((prod >> 32) as u8, &mut buffer[..2]);
    prod = (prod & K_UINT32_MASK) * 100;
    convert_2_digits((prod >> 32) as u8, &mut buffer[2..4]);
    return 4 + head_digit_count;
  }
  if n >= 100 {
    // 3 或 4 位: 42949673 = ceil(2^32 / 1e4)
    let mut prod = n as u64 * 42_949_673;

    let head_digits = (prod >> 32) as u8;
    let head_digit_count = convert_head_digits(head_digits, buffer);
    let buffer = &mut buffer[head_digit_count as usize..];

    // 余 2 位
    prod = (prod & K_UINT32_MASK) * 100;
    convert_2_digits((prod >> 32) as u8, &mut buffer[..2]);
    return 2 + head_digit_count;
  }
  // 1 或 2 位
  convert_head_digits(n as u8, buffer)
}

/// SignificandToChars — u64 尾数 → 十进制字符 (n < 1e17), 返回写出位数
#[must_use]
pub fn significand_to_chars(n: u64, buffer: &mut [u8]) -> u8 {
  debug_assert!(n < 99_999_999_999_999_997); // 至多 17 位

  if n >= 100_000_000 {
    // >= 9 位: 拆两块, 第二块恰 8 位
    let first_block = (n / 100_000_000) as u32;
    let second_block = (n % 100_000_000) as u32;

    let first_block_digits = convert_up_to_9_digits(first_block, buffer);
    convert_8_digits(second_block, &mut buffer[first_block_digits as usize..]);
    first_block_digits + 8
  } else {
    convert_up_to_9_digits(n as u32, buffer)
  }
}

// ---------------------------------------------------------------------------
// SimpleStringBuilder — 定长缓冲的安全字符构建器
// ---------------------------------------------------------------------------

/// 结果字符串构建器: 原版 debug 模式越界检查, Rust 版借用检查天然闭环;
/// `finish` 返回写入长度 (调用方以 `&buf[..len]` 取视图)
pub struct SimpleStringBuilder<'a> {
  buffer: &'a mut [u8],
  cursor: usize,
}

impl<'a> SimpleStringBuilder<'a> {
  /// 构建器接管缓冲
  pub fn new(buffer: &'a mut [u8]) -> Self {
    Self { buffer, cursor: 0 }
  }

  /// position — 当前写入位置
  #[must_use]
  pub fn position(&self) -> usize {
    self.cursor
  }

  /// AddCharacter — 追加单字符 (0 字符禁入, 以 finish 终止)
  #[inline]
  pub fn add_character(&mut self, c: u8) {
    debug_assert_ne!(c, 0);
    debug_assert!(self.cursor < self.buffer.len());
    self.buffer[self.cursor] = c;
    self.cursor += 1;
  }

  /// AddSubstring — 追加子串 (原版 0 终止串的前 n 字节)
  #[inline]
  pub fn add_substring(&mut self, s: &[u8]) {
    debug_assert!(self.cursor + s.len() <= self.buffer.len());
    self.buffer[self.cursor..self.cursor + s.len()].copy_from_slice(s);
    self.cursor += s.len();
  }

  /// AddPadding — 追加 count 个填充字符 (count <= 0 不写)
  #[inline]
  pub fn add_padding(&mut self, c: u8, count: isize) {
    if count <= 0 {
      return;
    }
    let count = count as usize;
    debug_assert!(self.cursor + count <= self.buffer.len());
    self.buffer[self.cursor..self.cursor + count].fill(c);
    self.cursor += count;
  }

  /// AddExponent — 追加至多 3 位十进制指数 (0..=999)
  #[inline]
  pub fn add_exponent(&mut self, value: u32) {
    debug_assert!(value <= 999);
    if value >= 100 {
      // 6554 = ceil(2^16 / 10)
      let d1 = (value * 6554) >> 16;
      let d2 = value - 10 * d1;
      convert_2_digits(d1 as u8, &mut self.buffer[self.cursor..self.cursor + 2]);
      self.cursor += 2;
      self.add_character(b'0' + d2 as u8);
    } else if value >= 10 {
      convert_2_digits(value as u8, &mut self.buffer[self.cursor..self.cursor + 2]);
      self.cursor += 2;
    } else {
      self.add_character(b'0' + value as u8);
    }
  }

  /// Finalize — 校验无非 0 字符后返回写入长度
  #[must_use]
  pub fn finish(self) -> usize {
    debug_assert!(self.cursor <= self.buffer.len());
    self.cursor
  }
}

// ---------------------------------------------------------------------------
// 杂值 (conversions.cc 333-339)
// ---------------------------------------------------------------------------

/// JunkStringValue — 解析失败哨兵: quiet NaN (kQuietNaNMask 位型)
#[must_use]
#[inline]
pub fn junk_string_value() -> f64 {
  f64::from_bits(QUIET_NAN_MASK)
}

/// SignedZero — 按符号取 ±0
#[must_use]
#[inline]
pub fn signed_zero(negative: bool) -> f64 {
  if negative {
    f64::from_bits(Double::SIGN_MASK)
  } else {
    0.0
  }
}
