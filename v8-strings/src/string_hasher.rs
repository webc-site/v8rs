// 由 tool/src/strings/string-hasher.h.js 从 v8/src/strings/string-hasher.h + string-hasher.cc + string-hasher-inl.h 生成; 修订就地改, 复跑覆盖
// hash 算法语义绑定 1:1; SIMD 位点标量化 (IsOnly8BitSIMD/HashConvertingTo8Bit)

//! 字符串哈希: array index 哈希 / rapidhash 全串哈希 / 增量 RunningStringHasher。
//! V8_ENABLE_SEEDED_ARRAY_INDEX_HASH 默认关闭, 未承接 (见 README 遗留清单)。

use core::slice::from_raw_parts;

use crate::{
  hash_field::{
    ARRAY_INDEX_LENGTH_SHIFT, ARRAY_INDEX_VALUE_MASK, ARRAY_INDEX_VALUE_SHIFT, HASH_BITS_MAX,
    HashFieldType, MAX_ARRAY_INDEX, MAX_ARRAY_INDEX_SIZE, MAX_CACHED_ARRAY_INDEX_LENGTH,
    MAX_HASH_CALC_LENGTH, MAX_INTEGER_INDEX_SIZE, contains_cached_array_index,
    create_hash_field_value,
  },
  hash_seed::HashSeedData,
  rapidhash::{ConvertTo8BitReader, PlainReader, rapidhash},
};

/// 哈希为 0 时以 27 代替 (kZeroHash; 0 保留给内部属性)。
pub const ZERO_HASH: u32 = 27;

/// 限制到支持位宽并兜底 kZeroHash (ConvertRawHashToUsableHash)。
#[inline]
fn convert_raw_to_usable(raw_hash: u64) -> u32 {
  let hash = (raw_hash & u64::from(HASH_BITS_MAX)) as u32;
  if hash == 0 { ZERO_HASH } else { hash }
}

/// 简单增量哈希器 (RunningStringHasher) — 允许逐字符特殊处理。
#[derive(Debug, Clone)]
pub struct RunningStringHasher {
  running_hash: u32,
}

impl RunningStringHasher {
  #[must_use]
  pub fn new(seed: u32) -> Self {
    Self { running_hash: seed }
  }

  /// 追加一个字符。
  #[inline]
  pub fn add_character(&mut self, c: u16) {
    self.running_hash = self.running_hash.wrapping_add(u32::from(c));
    self.running_hash = self.running_hash.wrapping_add(self.running_hash << 10);
    self.running_hash ^= self.running_hash >> 6;
  }

  /// 终结并产出可用哈希。
  #[inline]
  pub fn finalize(&mut self) -> u32 {
    self.running_hash = self.running_hash.wrapping_add(self.running_hash << 3);
    self.running_hash ^= self.running_hash >> 11;
    self.running_hash = self.running_hash.wrapping_add(self.running_hash << 15);
    convert_raw_to_usable(u64::from(self.running_hash))
  }
}

/// u16 串是否全在 Latin-1 (≤0xFF) — IsOnly8BitSIMD 标量化。
#[must_use]
pub fn is_only_8_bit(chars: &[u16]) -> bool {
  chars.iter().all(|&c| c <= 0xFF)
}

/// 索引解析结果 (IndexParseResult)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IndexParseResult {
  Success,
  NonIndex,
  Overflow,
}

/// 数字符 → 数字值; 非数字返回 >9 (与 c - '0' 的回绕语义一致)。
#[inline]
fn digit_val(c: u32) -> u32 {
  c.wrapping_sub(u32::from(b'0'))
}

/// TryParseArrayIndex: 无前导零 (除 "0") 的数字串 → index 值。
/// 返回 (结果, 已消费位置 i, 解析值)。
fn try_parse_array_index<C: Copy + Into<u32>>(
  chars: &[C],
  length: usize,
) -> (IndexParseResult, usize, u64) {
  let mut index = u64::from(digit_val(chars[0].into()));
  let mut i = 1usize;
  if index > 9 {
    return (IndexParseResult::NonIndex, i, index);
  }
  if index == 0 {
    if length > 1 {
      return (IndexParseResult::NonIndex, i, index);
    }
    return (IndexParseResult::Success, i, index);
  }
  if length > MAX_ARRAY_INDEX_SIZE as usize {
    return (IndexParseResult::Overflow, i, index);
  }
  while i < length {
    let val = digit_val(chars[i].into());
    if val > 9 {
      return (IndexParseResult::NonIndex, i, index);
    }
    index = 10 * index + u64::from(val);
    i += 1;
  }
  // 64 位: index 用 u64 承载, 仅需上限比较
  if index > u64::from(MAX_ARRAY_INDEX) {
    return (IndexParseResult::Overflow, i, index);
  }
  (IndexParseResult::Success, i, index)
}

/// TryParseIntegerIndex: array index 溢出后的整数索引解析 (≤ 2^53-1)。
fn try_parse_integer_index<C: Copy + Into<u32>>(
  chars: &[C],
  length: usize,
  mut i: usize,
  mut index: u64,
) -> IndexParseResult {
  const MAX_SAFE_INTEGER_UINT64: u64 = (1u64 << 53) - 1;
  while i < length {
    let val = digit_val(chars[i].into());
    if val > 9 {
      return IndexParseResult::NonIndex;
    }
    index = 10 * index + u64::from(val);
    i += 1;
  }
  if index > MAX_SAFE_INTEGER_UINT64 {
    return IndexParseResult::Overflow;
  }
  IndexParseResult::Success
}

/// MakeArrayIndexHash (raw 编码): value << VALUE_SHIFT | length << LENGTH_SHIFT。
#[inline]
#[must_use]
pub fn make_array_index_hash_raw(value: u32, length: u32) -> u32 {
  value << ARRAY_INDEX_VALUE_SHIFT | length << ARRAY_INDEX_LENGTH_SHIFT
}

/// DecodeArrayIndexFromHashField (raw 解码)。
#[inline]
#[must_use]
pub fn decode_array_index_from_hash_field(raw_hash_field: u32) -> u32 {
  (raw_hash_field >> ARRAY_INDEX_VALUE_SHIFT) & ARRAY_INDEX_VALUE_MASK
}

/// GetTrivialHash: 长串 (>kMaxHashCalcLength) 仅按长度取哈希。
#[inline]
#[must_use]
pub fn get_trivial_hash(length: u32) -> u32 {
  debug_assert!(length > MAX_HASH_CALC_LENGTH);
  create_hash_field_value(length, HashFieldType::Hash)
}

/// GetUsableRapidHash — 单字节内容。
fn usable_rapid_one_byte(chars: &[u8], seed: &HashSeedData) -> u32 {
  convert_raw_to_usable(rapidhash::<u8, PlainReader>(
    chars,
    chars.len(),
    seed.seed,
    &seed.secrets,
  ))
}

/// GetUsableRapidHash — 双字节内容; 返回 (hash, 是否全 Latin-1)。
fn usable_rapid_two_byte(chars: &[u16], seed: &HashSeedData) -> (u32, bool) {
  let one_byte = is_only_8_bit(chars);
  let h = if one_byte {
    // Latin-1 内容压缩到 8 bit: 与单字节串哈希一致
    rapidhash::<u16, ConvertTo8BitReader>(chars, chars.len(), seed.seed, &seed.secrets)
  } else {
    // SAFETY: chars 是已初始化的有效 u16 切片, u8 对齐要求 (1) 严格弱于 u16 对齐 (2),
    // 且长度 chars.len() * 2 恰覆盖全部底层字节; 小端目标内存序与 UTF-16 字节序天然一致。
    #[cfg(target_endian = "little")]
    let bytes: &[u8] = unsafe { from_raw_parts(chars.as_ptr().cast::<u8>(), chars.len() * 2) };
    #[cfg(not(target_endian = "little"))]
    let bytes_vec = chars
      .iter()
      .flat_map(|&c| c.to_le_bytes())
      .collect::<Vec<u8>>();
    #[cfg(not(target_endian = "little"))]
    let bytes = &bytes_vec[..];
    rapidhash::<u8, PlainReader>(bytes, bytes.len(), seed.seed, &seed.secrets)
  };
  (convert_raw_to_usable(h), one_byte)
}

/// HashSequentialString — 单字节内容。
/// 返回 raw_hash_field; out_one_byte_content 在真正扫描内容的路径上被置位。
pub fn hash_sequential_one_byte(
  chars: &[u8],
  seed: &HashSeedData,
  mut out_one_byte_content: Option<&mut bool>,
) -> u32 {
  let length = chars.len();
  if length >= 1 && length <= MAX_INTEGER_INDEX_SIZE as usize {
    match try_parse_array_index(chars, length) {
      (IndexParseResult::Success, _, index) => {
        if let Some(out) = out_one_byte_content.as_deref_mut() {
          *out = true;
        }
        return make_array_index_hash_raw(index as u32, length as u32);
      }
      (IndexParseResult::Overflow, i, index)
        if try_parse_integer_index(chars, length, i, index) == IndexParseResult::Success =>
      {
        let hash = create_hash_field_value(
          usable_rapid_one_byte(chars, seed),
          HashFieldType::IntegerIndex,
        );
        if let Some(out) = out_one_byte_content {
          *out = true;
        }
        return fix_cached_index_lookalike(hash);
      }
      _ => {}
    }
  } else if length > MAX_HASH_CALC_LENGTH as usize {
    return get_trivial_hash(length as u32);
  }
  // 非索引哈希
  if let Some(out) = out_one_byte_content {
    *out = true;
  }
  create_hash_field_value(usable_rapid_one_byte(chars, seed), HashFieldType::Hash)
}

/// HashSequentialString — 双字节内容。
pub fn hash_sequential_two_byte(
  chars: &[u16],
  seed: &HashSeedData,
  mut out_one_byte_content: Option<&mut bool>,
) -> u32 {
  let length = chars.len();
  if length >= 1 && length <= MAX_INTEGER_INDEX_SIZE as usize {
    match try_parse_array_index(chars, length) {
      (IndexParseResult::Success, _, index) => {
        // 数字串必然全 Latin-1
        if let Some(out) = out_one_byte_content.as_deref_mut() {
          *out = true;
        }
        return make_array_index_hash_raw(index as u32, length as u32);
      }
      (IndexParseResult::Overflow, i, index)
        if try_parse_integer_index(chars, length, i, index) == IndexParseResult::Success =>
      {
        let (h, one_byte) = usable_rapid_two_byte(chars, seed);
        if let Some(out) = out_one_byte_content {
          *out = one_byte;
        }
        return fix_cached_index_lookalike(create_hash_field_value(h, HashFieldType::IntegerIndex));
      }
      _ => {}
    }
  } else if length > MAX_HASH_CALC_LENGTH as usize {
    return get_trivial_hash(length as u32);
  }
  let (h, one_byte) = usable_rapid_two_byte(chars, seed);
  if let Some(out) = out_one_byte_content {
    *out = one_byte;
  }
  create_hash_field_value(h, HashFieldType::Hash)
}

/// 整数索引哈希碰巧形如缓存 array index 时, 置长度位规避。
#[inline]
fn fix_cached_index_lookalike(mut hash: u32) -> u32 {
  if contains_cached_array_index(hash) {
    hash |= (MAX_CACHED_ARRAY_INDEX_LENGTH + 1) << ARRAY_INDEX_LENGTH_SHIFT;
  }
  debug_assert!(!contains_cached_array_index(hash));
  hash
}
