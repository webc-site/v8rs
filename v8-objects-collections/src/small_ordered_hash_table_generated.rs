// 由 tool/src/objects/ordered-hash-table.tq.js 从 v8/src/objects/ordered-hash-table.tq 生成; 修订就地改, 复跑覆盖
// SmallOrdered* tq 布局字段序与表维度 (目标配置: 指针压缩)
//! SmallOrderedHashTable 家族 Torque 布局 (ordered-hash-table.tq)。
//!
//! 目标配置指针压缩 (kTaggedSize=4, TAGGED_SIZE_8_BYTES 为假):
//! padding 取 @ifnot 分支 (padding[1] / padding_0 为 void)。
//! generates 常量取值经 clang -E ordered-hash-table.h oracle 对数。

/// tq generates 'SmallOrderedHashTable::kMaxCapacity' (oracle: ordered-hash-table.h)。
pub const K_MAX_CAPACITY: u32 = 254;
/// tq generates 'SmallOrderedHashTable::kNotFound' (uint8 域)。
pub const K_NOT_FOUND: u8 = 0xFF;
/// tq generates 'SmallOrderedHashTable::kLoadFactor'。
pub const K_LOAD_FACTOR: u32 = 2;

/// SmallOrderedHashSet 字段序 (tq @cppObjectLayoutDefinition; 4 字节 tagged 分支)。
pub const SMALL_ORDERED_HASH_SET_FIELDS: &[(&str, &str)] = &[
  ("number_of_elements", "uint8"),
  ("number_of_deleted_elements", "uint8"),
  ("number_of_buckets", "uint8"),
  ("padding", "uint8[1]"), //  (非 8 字节 tagged分支)
  ("data_table", "JSAny|TheHole[dyn]"),
  ("hash_table", "uint8[dyn]"),
  ("chain_table", "uint8[dyn]"),
];

/// SmallOrderedHashMap 字段序 (4 字节 tagged 分支)。
pub const SMALL_ORDERED_HASH_MAP_FIELDS: &[(&str, &str)] = &[
  ("number_of_elements", "uint8"),
  ("number_of_deleted_elements", "uint8"),
  ("number_of_buckets", "uint8"),
  ("padding", "uint8[1]"), //  (非 8 字节 tagged分支)
  ("data_table", "HashMapEntry[dyn]"),
  ("hash_table", "uint8[dyn]"),
  ("chain_table", "uint8[dyn]"),
];

/// SmallOrderedNameDictionary 字段序 (4 字节 tagged 分支; hash 前缀 + padding_0)。
pub const SMALL_ORDERED_NAME_DICTIONARY_FIELDS: &[(&str, &str)] = &[
  ("hash", "int32"),
  ("padding_0", "void"), //  (非 8 字节 tagged分支)
  ("number_of_elements", "uint8"),
  ("number_of_deleted_elements", "uint8"),
  ("number_of_buckets", "uint8"),
  ("padding_1", "uint8[1]"), //  (非 8 字节 tagged分支)
  ("data_table", "NameDictionaryEntry[dyn]"),
  ("hash_table", "uint8[dyn]"),
  ("chain_table", "uint8[dyn]"),
];

/// data_table 条目数 = 桶数 * 装载因子 (Set/Map/NameDictionary 同式)。
#[inline]
#[must_use]
pub const fn data_table_size(number_of_buckets: u32) -> u32 {
  number_of_buckets * K_LOAD_FACTOR
}

/// hash_table 字节数 = 桶数 (uint8 桶链头)。
#[inline]
#[must_use]
pub const fn hash_table_size(number_of_buckets: u32) -> u32 {
  number_of_buckets
}

/// chain_table 字节数: Set/Map = 桶数 * 装载因子; NameDictionary = 桶数 (无乘子)。
#[inline]
#[must_use]
pub const fn chain_table_size(number_of_buckets: u32, with_load_factor: bool) -> u32 {
  if with_load_factor {
    number_of_buckets * K_LOAD_FACTOR
  } else {
    number_of_buckets
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_MAX_CAPACITY == 254);
  assert!(K_NOT_FOUND == 0xFF);
  assert!(K_LOAD_FACTOR == 2);
  assert!(SMALL_ORDERED_HASH_SET_FIELDS.len() == 7);
  assert!(SMALL_ORDERED_HASH_MAP_FIELDS.len() == 7);
  assert!(SMALL_ORDERED_NAME_DICTIONARY_FIELDS.len() == 9);
  assert!(data_table_size(4) == 8);
  assert!(chain_table_size(4, true) == 8);
  assert!(chain_table_size(4, false) == 4);
};
