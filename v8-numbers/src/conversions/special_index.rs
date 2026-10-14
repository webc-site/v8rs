//! 原版 conversions.cc 1590-1656 — IsSpecialIndex
//!
//! 判定字符串是否为 canonical 数组索引或特殊索引 ("NaN"/"Infinity"/
//! "-Infinity" 及 double 往返字符串): 快路径 ≤15 位纯数字, 慢路径做
//! DoubleToStringView(StringToDouble(s)) == s 往返比对。
//! 原版 Tagged<String>/access_guard 入口的 raw 等价 (对象绑定推迟)。

use super::{
  parse_double::string_to_double,
  parse_int::{ConversionFlag, NumChar},
  to_string::{DOUBLE_TO_STRING_MIN_BUFFER_SIZE, double_to_string_view},
};

/// IsDecimalDigit — '0'..='9'
#[inline]
fn is_decimal_digit(c: u32) -> bool {
  (u32::from(b'0')..=u32::from(b'9')).contains(&c)
}

/// IsSpecialIndex — 双向往返判定 (输入为 16 位字符序列)
#[must_use]
pub fn is_special_index<C: NumChar>(s: &[C]) -> bool {
  /// canonical double 最长串: -X.XXXXXXXXXXXXXXXXX-eXXX
  const BUFFER_SIZE: usize = 24;
  let length = s.len();
  if length == 0 || length > BUFFER_SIZE {
    return false;
  }
  let mut buffer = [0u16; BUFFER_SIZE];
  for (dst, src) in buffer[..length].iter_mut().zip(s.iter()) {
    *dst = src.code() as u16;
  }
  let buffer = &buffer[..length];

  // 首字符非数字且不匹配 '(-)Infinity'/'NaN' 前缀则直接失败
  let mut offset = 0usize;
  if !is_decimal_digit(buffer[0].into()) {
    if buffer[0] == u16::from(b'-') {
      if length == 1 {
        return false; // "-" 非法
      }
      if !is_decimal_digit(buffer[1].into()) {
        if buffer[1] == u16::from(b'I') && length == 9 {
          // 放行到下方 "-Infinity" 匹配
        } else {
          return false;
        }
      }
      offset += 1;
    } else if buffer[0] == u16::from(b'I') && length == 8 {
      // 放行到下方 "Infinity" 匹配
    } else if buffer[0] == u16::from(b'N') && length == 3 {
      // NaN
      return buffer[1] == u16::from(b'a') && buffer[2] == u16::from(b'N');
    } else {
      return false;
    }
  }

  // 快路径: 整数索引
  const REPRESENTABLE_INTEGER_LENGTH: usize = 15; // (-)XXXXXXXXXXXXXXX
  if length - offset <= REPRESENTABLE_INTEGER_LENGTH {
    let initial_offset = offset;
    let mut matches = true;
    while offset < length {
      matches &= is_decimal_digit(buffer[offset].into());
      offset += 1;
    }
    if matches {
      // 0 与 -0 仅当单字符 "0"/"-0" 形态
      if buffer[initial_offset] == u16::from(b'0') {
        return initial_offset == length - 1;
      }
      return true;
    }
  }

  // 慢路径: DoubleToStringView(StringToDouble(s)) == s
  let d = string_to_double(buffer, ConversionFlag::NoConversionFlag, 0.0);
  if d.is_nan() {
    return false;
  }
  let mut reverse_buffer = [0u8; DOUBLE_TO_STRING_MIN_BUFFER_SIZE];
  let reverse_string = double_to_string_view(d, &mut reverse_buffer);
  if reverse_string.len() != length {
    return false;
  }
  reverse_string
    .bytes()
    .zip(buffer.iter())
    .all(|(a, &b)| u16::from(a) == b)
}
