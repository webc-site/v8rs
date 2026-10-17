//! 原版 v8/src/base/vlq-base64.cc/h — base64 变长编码解码 (source map 风格)

/// charToDigit 查表 (原版 kCharToDigit, -1 = 非法字符)
#[rustfmt::skip]
const CHAR_TO_DIGIT: [i8; 128] = [
  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0x3e, -1, -1, -1, 0x3f,
  0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, -1, -1,
  -1, -1, -1, -1, -1, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
  0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12,
  0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, -1, -1, -1, -1, -1,
  -1, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24,
  0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30,
  0x31, 0x32, 0x33, -1, -1, -1, -1, -1,
];

const CONTINUE_SHIFT: u32 = 5;
const CONTINUE_MASK: i32 = 1 << CONTINUE_SHIFT;
const DATA_MASK: i32 = CONTINUE_MASK - 1;

/// 原版匿名 namespace 的 charToDigitDecode
#[must_use]
pub fn char_to_digit_decode(c: u8) -> i8 {
  if c < 128 {
    CHAR_TO_DIGIT[c as usize]
  } else {
    -1
  }
}

/// charToDigitDecodeForTesting
#[must_use]
pub fn char_to_digit_decode_for_testing(c: u8) -> i8 {
  char_to_digit_decode(c)
}

/// VLQBase64Decode — 坏输入返回 i32::MIN; 合法值域 [-2^31+1, 2^31-1]
///
/// `start`/`sz` 对应原版 (const char*, size_t), `pos` 为读写游标。
pub fn vlq_base64_decode(start: &[u8], pos: &mut usize) -> i32 {
  let sz = start.len();
  let mut res: u32 = 0;
  let mut shift: u32 = 0;
  loop {
    if *pos >= sz {
      return i32::MIN;
    }
    let digit = i32::from(char_to_digit_decode(start[*pos]));
    let is_last_byte = shift + CONTINUE_SHIFT >= 32;
    if digit == -1 || (is_last_byte && (digit >> 2) != 0) {
      return i32::MIN;
    }
    res = res.wrapping_add(((digit & DATA_MASK) as u32).wrapping_shl(shift));
    shift += CONTINUE_SHIFT;
    *pos += 1;
    if digit & CONTINUE_MASK == 0 {
      break;
    }
  }
  if (res & 1) == 1 {
    -((res >> 1) as i32)
  } else {
    (res >> 1) as i32
  }
}
