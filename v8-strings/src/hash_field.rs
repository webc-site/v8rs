// 由 tool/src/strings/string-hasher.h.js 从 v8/src/objects/name.h + src/objects/string.h 哈希字段位域 生成; 修订就地改, 复跑覆盖
// objects 层未落地, 位域语义绑定原地承接; v8-objects-strings 落地后归并

//! Name::raw_hash_field 位域布局 (kBitsPerInt=32, HashFieldType 占低 2 位)。
//!
//! 布局 (低位 → 高位):
//! - HashFieldType (2 bit)
//! - ArrayIndexValue (`K_ARRAY_INDEX_VALUE_BITS` bit, 仅 integer-index 编码)
//! - ArrayIndexLength (`K_ARRAY_INDEX_LENGTH_BITS` bit, 仅 integer-index 编码)
//! - Hash (30 bit, 仅 hash 编码)

/// 哈希字段类型 (HashFieldType)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum HashFieldType {
  /// 普通哈希。
  Hash = 0b10,
  /// 整数索引 (array index 或更长的整数串)。
  IntegerIndex = 0b00,
  /// 转发索引 (内部化)。
  ForwardingIndex = 0b01,
  /// 空 (未计算)。
  Empty = 0b11,
}

pub const HASH_FIELD_TYPE_BITS: u32 = 2;
/// 类型位掩码 (HashFieldTypeBits::kMask)。
pub const HASH_FIELD_TYPE_MASK: u32 = (1 << HASH_FIELD_TYPE_BITS) - 1;
/// 哈希值位宽 (kBitsPerInt - 2)。
pub const HASH_BITS_SIZE: u32 = 30;
/// 哈希值移位 (HashBits::kShift)。
pub const HASH_BITS_SHIFT: u32 = HASH_FIELD_TYPE_BITS;
/// 哈希值掩码 (HashBits::kMax)。
pub const HASH_BITS_MAX: u32 = 0x3fffffff;

/// array index 字符串短于此长度时, 索引可缓存在哈希字段内。
pub const MAX_CACHED_ARRAY_INDEX_LENGTH: u32 = 7;
/// 最大 array index 值 (kMaxUInt32 - 1)。
pub const MAX_ARRAY_INDEX: u32 = 4294967294;
/// array index 十进制串最大字符数。
pub const MAX_ARRAY_INDEX_SIZE: u32 = 10;
/// 整数索引串最大字符数 (64 位: 10 * (usize/4) 与 16 取小)。
pub const MAX_INTEGER_INDEX_SIZE: u32 = 16;
/// array index 值位宽。
pub const ARRAY_INDEX_VALUE_BITS: u32 = 24;
/// array index 值掩码。
pub const ARRAY_INDEX_VALUE_MASK: u32 = 0xffffff;
/// array index 长度位宽。
pub const ARRAY_INDEX_LENGTH_BITS: u32 = 6;
/// 值域在 raw_hash_field 中的移位。
pub const ARRAY_INDEX_VALUE_SHIFT: u32 = 2;
/// 长度域在 raw_hash_field 中的移位。
pub const ARRAY_INDEX_LENGTH_SHIFT: u32 = 26;
/// 任一位置位 → 哈希字段不含缓存 array index。
pub const DOES_NOT_CONTAIN_CACHED_ARRAY_INDEX_MASK: u32 =
  (!MAX_CACHED_ARRAY_INDEX_LENGTH).wrapping_shl(ARRAY_INDEX_LENGTH_SHIFT) | HASH_FIELD_TYPE_MASK;
/// 任一位置位 → 哈希字段不含整数/转发索引。
pub const DOES_NOT_CONTAIN_INTEGER_OR_FORWARDING_INDEX_MASK: u32 = 0b10;

/// 哈希计算的最大串长; 超出仅按长度取哈希。
pub const MAX_HASH_CALC_LENGTH: u32 = 16383;

/// Name::CreateHashFieldValue。
#[inline]
#[must_use]
pub fn create_hash_field_value(hash: u32, ty: HashFieldType) -> u32 {
  debug_assert_ne!(ty, HashFieldType::ForwardingIndex);
  (hash & HASH_BITS_MAX) << HASH_BITS_SHIFT | ty as u32
}

/// Name::ContainsCachedArrayIndex。
#[inline]
#[must_use]
pub fn contains_cached_array_index(raw_hash_field: u32) -> bool {
  raw_hash_field & DOES_NOT_CONTAIN_CACHED_ARRAY_INDEX_MASK == 0
}

/// Name::IsIntegerIndex。
#[inline]
#[must_use]
pub fn is_integer_index(raw_hash_field: u32) -> bool {
  raw_hash_field & DOES_NOT_CONTAIN_INTEGER_OR_FORWARDING_INDEX_MASK == 0
}
