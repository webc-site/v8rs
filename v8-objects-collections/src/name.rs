// 由 tool/src/objects/name.tq.js 从 v8/src/objects/name.tq + 取值侧 src/objects/name.h 生成; 修订就地改, 复跑覆盖
// Name 哈希字段位段与数组索引哈希规则面

//! Name/Symbol 的 Torque 侧规则面 (hash 字段位段 + 数组索引哈希编码)。
//!
//! NameHash (uint32) 位段:
//! - hash_field_type (bit0-1): HashFieldType
//! - array_index_value (bit2-25): 24 位
//! - array_index_length (bit26-31): 6 位

// ── HashFieldType (name.h:168, tq hash_field_type 取值域) ─────────────
/// 哈希字段类型判别值 (bit0-1)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum HashFieldType {
  /// 整数索引 (数组下标)。
  IntegerIndex = 0b00,
  /// 转发表索引。
  ForwardingIndex = 0b01,
  /// 已计算哈希。
  Hash = 0b10,
  /// 空 (未计算)。
  Empty = 0b11,
}

// ── NameHash 位段 (name.tq:11-15) ─────────────────────────────────────
/// hash_field_type 位宽 (= kNofHashBitFields)。
pub const K_NOF_HASH_BIT_FIELDS: u32 = 2;
/// array_index_value 位宽 (= kArrayIndexValueBits)。
pub const K_ARRAY_INDEX_VALUE_BITS: u32 = 24;
/// array_index_length 位宽 (= kArrayIndexLengthBits = 32 - 24 - 2)。
pub const K_ARRAY_INDEX_LENGTH_BITS: u32 = 32 - K_ARRAY_INDEX_VALUE_BITS - K_NOF_HASH_BIT_FIELDS;
/// array_index_value 移位 (name.tq:70)。
pub const K_ARRAY_INDEX_VALUE_BITS_SHIFT: u32 = K_NOF_HASH_BIT_FIELDS;
/// array_index_length 移位 (name.tq:71-72)。
pub const K_ARRAY_INDEX_LENGTH_BITS_SHIFT: u32 = K_NOF_HASH_BIT_FIELDS + K_ARRAY_INDEX_VALUE_BITS;

// ── generates 常量 (值取 name.h 原版侧) ────────────────────────────────
/// 可缓存数组索引的最大串长 (kMaxCachedArrayIndexLength)。
pub const K_MAX_CACHED_ARRAY_INDEX_LENGTH: u32 = 7;
/// 数组索引判定的最大字符数 (kMaxArrayIndexSize)。
pub const K_MAX_ARRAY_INDEX_SIZE: u32 = 10;
/// 位掩码: 任一位置 1 则哈希字段不含缓存数组索引
/// (name.h:250 = (~kMaxCachedArrayIndexLength << 26) | 0b11)。
pub const K_DOES_NOT_CONTAIN_CACHED_ARRAY_INDEX_MASK: u32 = 0xE0000003;

// ── PrivateSymbolKind (name.tq:21-26, 顺序值) ─────────────────────────
/// 私有 Symbol 种类 (SymbolFlags 低 2 位)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PrivateSymbolKind {
  /// 公开 (默认)。
  Public = 0,
  /// 内部。
  Internal = 1,
  /// 字段名。
  FieldName = 2,
  /// brand (private field 品牌)。
  Brand = 3,
}

// ── SymbolFlags 位段 (name.tq:29-34) ──────────────────────────────────
/// private_symbol_kind 移位。
pub const SYMBOL_FLAGS_PRIVATE_SYMBOL_KIND_SHIFT: u32 = 0;
/// is_well_known_symbol 移位。
pub const SYMBOL_FLAGS_IS_WELL_KNOWN_SYMBOL_SHIFT: u32 = 2;
/// is_in_public_symbol_table 移位。
pub const SYMBOL_FLAGS_IS_IN_PUBLIC_SYMBOL_TABLE_SHIFT: u32 = 3;
/// is_interesting_symbol 移位。
pub const SYMBOL_FLAGS_IS_INTERESTING_SYMBOL_SHIFT: u32 = 4;

// ── 宏规则面 (name.tq 61-112 纯计算段) ────────────────────────────────
/// 哈希字段是否含缓存数组索引 (name.tq:61 ContainsCachedArrayIndex)。
#[inline]
#[must_use]
pub const fn contains_cached_array_index(hash: u32) -> bool {
  (hash & K_DOES_NOT_CONTAIN_CACHED_ARRAY_INDEX_MASK) == 0
}

/// 哈希字段是否为整数索引 (name.tq:83 IsIntegerIndex)。
#[inline]
#[must_use]
pub const fn is_integer_index(raw_hash_field: u32) -> bool {
  (raw_hash_field & 0b11) == HashFieldType::IntegerIndex as u32
}

/// 10 的 exponent 次幂 (name.tq:74 TenToThe; dcheck: exponent ≤ 9)。
#[inline]
#[must_use]
pub const fn ten_to_the(exponent: u32) -> u32 {
  let mut answer: u32 = 1;
  let mut i = 0;
  while i < exponent {
    answer *= 10;
    i += 1;
  }
  answer
}

/// 数组索引 → 哈希字段编码 (name.tq:90 MakeArrayIndexHashRaw, 无种子形态)。
///
/// (value << 2) | (length << 26); dcheck 契约: length ≤ kMaxArrayIndexSize 且
/// ten_to_the(kMaxCachedArrayIndexLength) < 2^24 (8 项断言见尾)。
#[inline]
#[must_use]
pub const fn make_array_index_hash_raw(value: u32, length: u32) -> u32 {
  (value << K_ARRAY_INDEX_VALUE_BITS_SHIFT) | (length << K_ARRAY_INDEX_LENGTH_BITS_SHIFT)
}

/// 哈希字段 → 数组索引解码 (name.tq:108 DecodeArrayIndexFromHashFieldRaw)。
///
/// dcheck 契约: contains_cached_array_index || is_integer_index。
#[inline]
#[must_use]
pub const fn decode_array_index_from_hash_field_raw(raw_hash_field: u32) -> u32 {
  (raw_hash_field >> K_ARRAY_INDEX_VALUE_BITS_SHIFT) & ((1 << K_ARRAY_INDEX_VALUE_BITS) - 1)
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(
    K_NOF_HASH_BIT_FIELDS == 2 && K_ARRAY_INDEX_VALUE_BITS == 24 && K_ARRAY_INDEX_LENGTH_BITS == 6
  );
  assert!(K_ARRAY_INDEX_VALUE_BITS_SHIFT == 2 && K_ARRAY_INDEX_LENGTH_BITS_SHIFT == 26);
  assert!(K_MAX_CACHED_ARRAY_INDEX_LENGTH == 7 && K_MAX_ARRAY_INDEX_SIZE == 10);
  // (~7 << 26) | 0b11 = 0xE0000003 (掩码高位检 length ≤ 7)
  assert!(K_DOES_NOT_CONTAIN_CACHED_ARRAY_INDEX_MASK == 0xE000_0003);
  assert!(HashFieldType::IntegerIndex as u32 == 0 && HashFieldType::Empty as u32 == 3);
  assert!(contains_cached_array_index(make_array_index_hash_raw(5, 3)));
  assert!(!contains_cached_array_index(0x8000_0000));
  assert!(is_integer_index(make_array_index_hash_raw(9, 2)));
  assert!(decode_array_index_from_hash_field_raw(make_array_index_hash_raw(1234, 7)) == 1234);
  // name.tq:93 dcheck 的编译期形态: 10^7 < 2^24? 否 (10_000_000 < 16_777_216 成立)
  assert!(ten_to_the(7) == 10_000_000 && 10_000_000 < (1 << K_ARRAY_INDEX_VALUE_BITS));
};
