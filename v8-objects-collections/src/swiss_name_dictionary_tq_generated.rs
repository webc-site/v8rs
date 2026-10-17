// 由 tool/src/objects/swiss-name-dictionary.tq.js 从 v8/src/objects/swiss-name-dictionary.tq 生成; 修订就地改, 复跑覆盖
// SwissNameDictionary tq 布局/容量与元表纯函数 (SIMD 16 宽)
//! SwissNameDictionary Torque 面 (swiss-name-dictionary.tq)。
//!
//! 布局字段序 + generates 常量 + tq @export 容量/尺寸纯函数;
//! 取值公式经 clang -E swiss-name-dictionary.h/-inl.h oracle 对数。
//! 控制字节与 H1/H2 见 crate::swiss_hash_table_helpers。

/// tq generates 'SwissNameDictionary::kNotFoundSentinel'。
pub const K_NOT_FOUND_SENTINEL: i32 = -1;
/// tq generates 'SwissNameDictionary::kDataTableEntryCount'。
pub const K_DATA_TABLE_ENTRY_COUNT: u32 = 2;
/// tq generates 'SwissNameDictionary::kMax1ByteMetaTableCapacity' (1 << 8)。
pub const K_MAX_1BYTE_META_TABLE_CAPACITY: u32 = 256;
/// tq generates 'SwissNameDictionary::kMax2ByteMetaTableCapacity' (1 << 16)。
pub const K_MAX_2BYTE_META_TABLE_CAPACITY: u32 = 65536;
/// 元表槽: 现存元素计数。
pub const K_META_TABLE_ELEMENT_COUNT_FIELD_INDEX: u32 = 0;
/// 元表槽: 已删除元素计数。
pub const K_META_TABLE_DELETED_ELEMENT_COUNT_FIELD_INDEX: u32 = 1;
/// 元表槽: 枚举数据起始。
pub const K_META_TABLE_ENUMERATION_DATA_START_INDEX: u32 = 2;

/// SwissNameDictionary 布局字段序 (tq @cppObjectLayoutDefinition)。
pub const SWISS_NAME_DICTIONARY_FIELDS: &[(&str, &str)] = &[
  ("hash", "uint32"),
  ("capacity", "uint32"),
  ("meta_table", "ByteArray"),
  ("data_table", "JSAny|TheHole[capacity * 2]"),
  ("ctrl_table", "uint8[capacity + kGroupWidth]"),
  ("property_details_table", "uint8[capacity]"),
];

/// 容量选择 (tq SwissNameDictionaryCapacityFor / C++ CapacityFor):
/// 0 → 0; ≤4 → 4 (组宽 16; 组宽 8 时 4 → 8); 其余 n + n/7 向上取 2 的幂。
#[inline]
#[must_use]
pub const fn capacity_for(at_least_space_for: u32) -> u32 {
  if at_least_space_for == 0 {
    0
  } else if at_least_space_for <= 4 {
    4
  } else {
    (at_least_space_for + at_least_space_for / 7).next_power_of_two()
  }
}

/// 最大可用容量 (tq SwissNameDictionaryMaxUsableCapacity): capacity - capacity/8
/// (组宽 8 且 capacity 4 的特例返回 3, 属 portable 配置, 目标 SIMD 16 宽不触发)。
#[inline]
#[must_use]
pub const fn max_usable_capacity(capacity: u32) -> u32 {
  capacity - capacity / 8
}

/// 数据表字节数 = capacity * kTaggedSize * kDataTableEntryCount (kTaggedSize=4 压缩配置)。
#[inline]
#[must_use]
pub const fn data_table_size(capacity: u32) -> u32 {
  capacity * 4 * K_DATA_TABLE_ENTRY_COUNT
}

/// 控制表字节数 = (capacity + kGroupWidth) * 1 (组宽 16, 末尾复制首组)。
#[inline]
#[must_use]
pub const fn ctrl_table_size(capacity: u32) -> u32 {
  capacity + 16
}

/// 相对对象头的字节偏移链 (C++ -inl.h): 头 + 8 (hash+capacity) + kTaggedSize (meta_table)。
#[inline]
#[must_use]
pub const fn data_table_start_offset_from_object_start(heap_object_header_size: u32) -> u32 {
  heap_object_header_size + 8 + 4
}

/// 控制表起始相对偏移 = 数据表起始 + 数据表大小。
#[inline]
#[must_use]
pub const fn ctrl_table_start_offset_from_object_start(
  capacity: u32,
  heap_object_header_size: u32,
) -> u32 {
  data_table_start_offset_from_object_start(heap_object_header_size) + data_table_size(capacity)
}

/// 元表每条目字节数: ≤256 → 1; ≤65536 → 2; 其余 4。
#[inline]
#[must_use]
pub const fn meta_table_size_per_entry_for(capacity: u32) -> u32 {
  if capacity <= K_MAX_1BYTE_META_TABLE_CAPACITY {
    1
  } else if capacity <= K_MAX_2BYTE_META_TABLE_CAPACITY {
    2
  } else {
    4
  }
}

/// 元表字节数 = (2 + 最大可用容量) * 每条目字节数。
#[inline]
#[must_use]
pub const fn meta_table_size_for(capacity: u32) -> u32 {
  (2 + max_usable_capacity(capacity)) * meta_table_size_per_entry_for(capacity)
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_NOT_FOUND_SENTINEL == -1);
  assert!(K_DATA_TABLE_ENTRY_COUNT == 2);
  assert!(K_MAX_1BYTE_META_TABLE_CAPACITY == 256);
  assert!(K_MAX_2BYTE_META_TABLE_CAPACITY == 65536);
  assert!(SWISS_NAME_DICTIONARY_FIELDS.len() == 6);
  assert!(capacity_for(0) == 0);
  assert!(capacity_for(3) == 4);
  assert!(capacity_for(9) == 16);
  assert!(capacity_for(1024) == 2048);
  assert!(max_usable_capacity(8) == 7);
  assert!(max_usable_capacity(256) == 224);
  assert!(data_table_size(8) == 64);
  assert!(ctrl_table_size(8) == 24);
  assert!(meta_table_size_per_entry_for(257) == 2);
  assert!(meta_table_size_for(16) == 16);
};
