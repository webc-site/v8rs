//! 原版 v8/src/base/vlq.h — 变长编码 (VLQ)

/// kContinueShift
pub const CONTINUE_SHIFT: u32 = 7;
/// kContinueBit
pub const CONTINUE_BIT: u32 = 1 << CONTINUE_SHIFT;
/// kDataMask
pub const DATA_MASK: u32 = CONTINUE_BIT - 1;

/// VLQEncodeUnsigned — 每字节 7 位数据 + 1 位续接位
pub fn vlq_encode_unsigned<F: FnMut(u8)>(process_byte: &mut F, value: u32) {
  const DATA_BITS_PER_BYTE: u32 = CONTINUE_SHIFT;
  let mut value = value;
  macro_rules! write_byte {
    () => {{
      process_byte(value as u8 | CONTINUE_BIT as u8);
      value >>= CONTINUE_SHIFT;
    }};
  }
  if value >= 1 << (DATA_BITS_PER_BYTE) {
    if value >= 1 << (2 * DATA_BITS_PER_BYTE) {
      if value >= 1 << (3 * DATA_BITS_PER_BYTE) {
        if value >= 1 << (4 * DATA_BITS_PER_BYTE) {
          write_byte!();
        }
        write_byte!();
      }
      write_byte!();
    }
    write_byte!();
  }
  // 最后一字节不带续接位
  process_byte(value as u8);
}

/// VLQConvertToUnsigned — 符号编码进最低位
#[must_use]
pub fn vlq_convert_to_unsigned(value: i32) -> u32 {
  debug_assert_ne!(value, i32::MIN);
  let is_negative = value < 0;
  let mag = value.unsigned_abs();
  // 原版 (is_negative ? -value : value) << 1 | is_negative
  (mag << 1) | u32::from(is_negative)
}

/// VLQEncode — 有符号编码到回调
pub fn vlq_encode<F: FnMut(u8)>(process_byte: &mut F, value: i32) {
  vlq_encode_unsigned(process_byte, vlq_convert_to_unsigned(value));
}

/// VLQEncode 的 Vec 容器版本 (对应 std::vector 包装)
pub fn vlq_encode_to_vec(data: &mut Vec<u8>, value: i32) {
  vlq_encode(&mut |b| data.push(b), value);
}

/// VLQEncodeUnsigned 的 Vec 容器版本
pub fn vlq_encode_unsigned_to_vec(data: &mut Vec<u8>, value: u32) {
  vlq_encode_unsigned(&mut |b| data.push(b), value);
}

/// VLQDecodeUnsigned — 从回调逐字节解码
pub fn vlq_decode_unsigned<F: FnMut() -> u8>(get_next: &mut F) -> u32 {
  let cur_byte = get_next();
  // 单字节快路径
  if u32::from(cur_byte) <= DATA_MASK {
    return u32::from(cur_byte);
  }
  let mut bits = u32::from(cur_byte) & DATA_MASK;
  let mut shift = CONTINUE_SHIFT;
  while shift <= 32 {
    let cur_byte = get_next();
    // 原版为 uint32_t 移位截断, 逐位对齐使用 wrapping
    bits |= (u32::from(cur_byte) & DATA_MASK).wrapping_shl(shift);
    if u32::from(cur_byte) <= DATA_MASK {
      break;
    }
    shift += CONTINUE_SHIFT;
  }
  bits
}

/// VLQDecodeUnsigned — 连续内存 + 游标版本
pub fn vlq_decode_unsigned_at(data: &[u8], index: &mut usize) -> u32 {
  vlq_decode_unsigned(&mut || {
    let b = data[*index];
    *index += 1;
    b
  })
}

/// VLQDecode — 有符号解码
pub fn vlq_decode(data: &[u8], index: &mut usize) -> i32 {
  let bits = vlq_decode_unsigned_at(data, index);
  let is_negative = (bits & 1) == 1;
  let result = (bits >> 1) as i32;
  if is_negative { -result } else { result }
}
