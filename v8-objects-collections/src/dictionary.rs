// 由 tool/src/objects/dictionary.h.js 从 v8/src/objects/dictionary.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// Dictionary 族 Shape 布局与慢元素位掩码 (规则面)

//! Dictionary 家族: HashTable 之上的键值字典 (可带 PropertyDetails)。
//!
//! - entry 布局: [key, value(, details)] (kEntryValueIndex=1, kEntryDetailsIndex=2)
//! - NameDictionary 前缀: [0]=下一枚举索引 [1]=对象哈希 [2]=flags
//! - NumberDictionary 前缀: [0]=max_number_key (含 requires_slow_elements 位)

// ── 前缀槽位 (BaseNameDictionary / dictionary.h:189-192) ───────────────
use crate::v8_map::Map;
/// 下一枚举索引槽 (NameDictionary/GlobalDictionary 前缀 [0])。
pub const K_NEXT_ENUMERATION_INDEX_INDEX: usize = 3; // = HashTableBase::kPrefixStartIndex
/// 对象哈希槽 (前缀 [1])。
pub const K_OBJECT_HASH_INDEX: usize = K_NEXT_ENUMERATION_INDEX_INDEX + 1;
/// entry 内 value 下标。
pub const K_ENTRY_VALUE_INDEX: usize = 1;
/// entry 内 PropertyDetails 下标。
pub const K_ENTRY_DETAILS_INDEX: usize = 2;

// ── NameDictionary (dictionary.h:284-316) ──────────────────────────────
/// flags 槽 (NameDictionary 前缀 [3] = kObjectHashIndex + 1)。
pub const K_FLAGS_INDEX: usize = K_OBJECT_HASH_INDEX + 1;
/// NameDictionary 初始容量 (kInitialCapacity)。
pub const K_INITIAL_CAPACITY: usize = 2;
/// flags 默认值。
pub const K_FLAGS_DEFAULT: u32 = 0;
/// flags 位段: may_have_interesting_properties (bit0, 1 位)。
pub const MAY_HAVE_INTERESTING_PROPERTIES_SHIFT: u32 = 0;
/// 位宽。
pub const MAY_HAVE_INTERESTING_PROPERTIES_SIZE: u32 = 1;
/// 掩码。
pub const MAY_HAVE_INTERESTING_PROPERTIES_MASK: u32 = 1;

// ── NumberDictionary 慢元素位掩码 (dictionary.h:475-482) ───────────────
/// max_number_key 的 requires_slow_elements 位 (bit0)。
pub const K_REQUIRES_SLOW_ELEMENTS_MASK: u32 = 1;
/// 位移右移量 (真实键值 = max_index_object >> 1)。
pub const K_REQUIRES_SLOW_ELEMENTS_TAG_SIZE: u32 = 1;
/// 触发慢元素的索引上限: (1 << 29) - 1。
pub const K_REQUIRES_SLOW_ELEMENTS_LIMIT: u32 = (1 << 29) - 1;
/// JSObject 偏好字典元素的经验因子 (省内存阈值)。
pub const K_PREFER_FAST_ELEMENTS_SIZE_FACTOR: u32 = 3;
/// max_number_key 前缀槽 (NumberDictionary 前缀 [0] = kPrefixStartIndex)。
pub const K_MAX_NUMBER_KEY_INDEX: usize = 3;

/// 是否要求慢元素 (dictionary-inl.h:125 requires_slow_elements)。
#[inline]
#[must_use]
pub const fn requires_slow_elements(max_index_object: u32) -> bool {
  0 != (max_index_object & K_REQUIRES_SLOW_ELEMENTS_MASK)
}

/// 解码真实 max_number_key (dictionary-inl.h:133 max_number_key)。
///
/// 契约: 仅在 requires_slow_elements() 为假时结果有意义 (原版注释)。
#[inline]
#[must_use]
pub const fn max_number_key(max_index_object: u32) -> u32 {
  max_index_object >> K_REQUIRES_SLOW_ELEMENTS_TAG_SIZE
}

/// 构造 requires_slow_elements 标记值 (set_requires_slow_elements: kMaxNumberKeyIndex ← 掩码)。
#[inline]
#[must_use]
pub const fn requires_slow_elements_marker() -> u32 {
  K_REQUIRES_SLOW_ELEMENTS_MASK
}

// ── Shape 规则面 (dictionary.h 展开) ───────────────────────────────────
/// Dictionary Shape 静态参数。
pub struct DictionaryShapeParams {
  /// 前缀槽数。
  pub prefix_size: usize,
  /// 每 entry 槽数。
  pub entry_size: usize,
  /// entry 是否含 PropertyDetails 槽。
  pub has_details: bool,
}
/// NameDictionaryShape (key/value/details, 前缀 3 槽)。
pub const NAME_DICTIONARY_SHAPE: DictionaryShapeParams = DictionaryShapeParams {
  prefix_size: 3,
  entry_size: 3,
  has_details: true,
};
/// SimpleNameDictionaryShape (key/value, 无前缀无 details)。
pub const SIMPLE_NAME_DICTIONARY_SHAPE: DictionaryShapeParams = DictionaryShapeParams {
  prefix_size: 0,
  entry_size: 2,
  has_details: false,
};
/// GlobalDictionaryShape (key=PropertyCell, 前缀 2 槽)。
pub const GLOBAL_DICTIONARY_SHAPE: DictionaryShapeParams = DictionaryShapeParams {
  prefix_size: 2,
  entry_size: 1,
  has_details: true,
};
/// NumberDictionaryShape (key/value/details, 前缀 1 槽)。
pub const NUMBER_DICTIONARY_SHAPE: DictionaryShapeParams = DictionaryShapeParams {
  prefix_size: 1,
  entry_size: 3,
  has_details: true,
};
/// SimpleNumberDictionaryShape (key/value, 无前缀无 details)。
pub const SIMPLE_NUMBER_DICTIONARY_SHAPE: DictionaryShapeParams = DictionaryShapeParams {
  prefix_size: 0,
  entry_size: 2,
  has_details: false,
};

// ── 堆访问面: 基于 hash_table 类型化基座 (就地修订完成) ────────────────
use v8_objects_core::{
  property_details::PropertyDetails,
  smi::Smi,
  tagged::{Object, Tagged},
};

use crate::{
  allocator::MemoryAllocator,
  error::Result,
  hash_table::{self, HashTable, HashTableExt, ShapeParams as HashShapeParams, UNDEFINED},
  internal_index::InternalIndex,
  ordered_hash_table::{THE_HOLE, compute_object_hash, keys_equal},
};

/// Dictionary Shape → 泛型 HashTable Shape 换算 (不启用哈希分散)。
///
/// kMatchNeedsHoleCheck 按原版逐 Shape 登记:
/// NameDictionary/SimpleName=false; Global/Number/SimpleNumber=true (dictionary.h:180-388)。
#[inline]
#[must_use]
pub const fn to_hash_shape(shape: DictionaryShapeParams) -> HashShapeParams {
  HashShapeParams {
    prefix_size: shape.prefix_size,
    entry_size: shape.entry_size,
    match_needs_hole_check: shape.match_needs_hole_check,
    do_hash_spreading: false,
    hash_bits: 0,
  }
}

/// entry 槽基址 (DerivedHashTable::EntryToIndex)。
#[inline]
#[must_use]
fn entry_index(shape: DictionaryShapeParams, entry: InternalIndex) -> usize {
  hash_table::entry_to_index(
    entry.as_uint32(),
    shape.entry_size,
    hash_table::elements_start_index(shape.prefix_size),
  )
}

/// 读 entry 的 PropertyDetails (BaseDictionaryShape::DetailsAt, dictionary-inl.h:181)。
///
/// v8: src/objects/dictionary-inl.h BaseDictionaryShape::DetailsAt
#[inline]
#[must_use]
pub fn details_at(
  dict: Tagged<HashTable>,
  entry: InternalIndex,
  shape: DictionaryShapeParams,
) -> PropertyDetails {
  debug_assert!(shape.has_details, "无 details 槽的 Shape 不允许读取");
  debug_assert!(entry.is_found());
  let raw = dict.read_slot(entry_index(shape, entry) + K_ENTRY_DETAILS_INDEX);
  // 原版 Cast<Smi> 硬转换; 仿真堆容忍未初始化槽 → Empty (对数语义不变)
  if raw.is_smi() {
    PropertyDetails::from_smi(raw.cast::<Smi>())
  } else {
    PropertyDetails::empty()
  }
}

/// 写 entry 的 PropertyDetails (BaseDictionaryShape::DetailsAtPut, dictionary-inl.h:191)。
///
/// v8: src/objects/dictionary-inl.h BaseDictionaryShape::DetailsAtPut
#[inline]
pub fn details_at_put(
  dict: Tagged<HashTable>,
  entry: InternalIndex,
  value: PropertyDetails,
  shape: DictionaryShapeParams,
) {
  debug_assert!(shape.has_details, "无 details 槽的 Shape 不允许写入");
  let smi = value.as_smi();
  dict.write_slot(
    entry_index(shape, entry) + K_ENTRY_DETAILS_INDEX,
    smi.cast(),
  );
}

/// 读 entry 的 value 槽 (Dictionary::ValueAt, dictionary-inl.h)。
///
/// v8: src/objects/dictionary-inl.h Dictionary::ValueAt
#[inline]
#[must_use]
pub fn value_at(
  dict: Tagged<HashTable>,
  entry: InternalIndex,
  shape: DictionaryShapeParams,
) -> Tagged<Object> {
  dict.read_slot(entry_index(shape, entry) + K_ENTRY_VALUE_INDEX)
}

/// 写 entry 的 value 槽 (Dictionary::ValueAtPut, dictionary-inl.h:61)。
///
/// v8: src/objects/dictionary-inl.h Dictionary::ValueAtPut
#[inline]
pub fn value_at_put(
  dict: Tagged<HashTable>,
  entry: InternalIndex,
  value: Tagged<Object>,
  shape: DictionaryShapeParams,
) {
  dict.write_slot(entry_index(shape, entry) + K_ENTRY_VALUE_INDEX, value);
}

/// 写整 entry (key/value/details; dictionary-inl.h:161 三参展开)。
///
/// v8: src/objects/dictionary-inl.h Dictionary::SetEntry
pub fn set_entry(
  dict: Tagged<HashTable>,
  entry: InternalIndex,
  key: Tagged<Object>,
  value: Tagged<Object>,
  details: PropertyDetails,
  shape: DictionaryShapeParams,
) {
  debug_assert!(shape.entry_size == 2 || shape.entry_size == 3);
  let index = entry_index(shape, entry);
  dict.write_slot(index + hash_table::K_ENTRY_KEY_INDEX, key);
  dict.write_slot(index + K_ENTRY_VALUE_INDEX, value);
  if shape.has_details {
    details_at_put(dict, entry, details, shape);
  }
}

/// 删除 entry (键/值置 the_hole, details 置 Empty; dictionary-inl.h:141)。
///
/// v8: src/objects/dictionary-inl.h Dictionary::ClearEntry
pub fn clear_entry(dict: Tagged<HashTable>, entry: InternalIndex, shape: DictionaryShapeParams) {
  set_entry(
    dict,
    entry,
    THE_HOLE,
    THE_HOLE,
    PropertyDetails::empty(),
    shape,
  );
}

/// 键已存在则覆写值 (与 details), 否则走 Add 派生流。
///
/// v8: src/objects/objects.cc Dictionary::AtPut
pub fn at_put<A: MemoryAllocator>(
  allocator: &mut A,
  map: Tagged<Map>,
  dict: Tagged<HashTable>,
  key: Tagged<Object>,
  value: Tagged<Object>,
  details: PropertyDetails,
  shape: DictionaryShapeParams,
) -> Result<Tagged<HashTable>> {
  let hash_shape = to_hash_shape(shape);
  let hash = compute_object_hash(key);
  let entry = dict.find_entry(key, hash, hash_shape, keys_equal);
  if entry.is_found() {
    // 命中: 只覆写值 (与 details), 枚举索引不动
    value_at_put(dict, entry, value, shape);
    if shape.entry_size == 3 {
      details_at_put(dict, entry, details, shape);
    }
    return Ok(dict);
  }
  // 慢路径: Derived::Add (此处转写通用 Dictionary::Add; 枚举索引分配见 base_name_add 面)
  add(allocator, map, dict, key, value, details, shape)
}

/// 插入新键 (容量保证 → FindInsertionEntry → SetEntry → 计数; objects.cc:5667)。
///
/// v8: src/objects/objects.cc Dictionary::Add
pub fn add<A: MemoryAllocator>(
  allocator: &mut A,
  map: Tagged<Map>,
  dict: Tagged<HashTable>,
  key: Tagged<Object>,
  value: Tagged<Object>,
  details: PropertyDetails,
  shape: DictionaryShapeParams,
) -> Result<Tagged<HashTable>> {
  let hash_shape = to_hash_shape(shape);
  let hash = compute_object_hash(key);
  debug_assert!(
    dict
      .find_entry(key, hash, hash_shape, keys_equal)
      .is_not_found(),
    "Add 前键必须不存在 (原版 SLOW_DCHECK)"
  );
  let dict = hash_table::ensure_capacity(allocator, map, dict, 1, hash_shape)?;
  let entry = dict.find_insertion_entry(hash, hash_shape);
  set_entry(dict, entry, key, value, details, shape);
  dict.element_added();
  Ok(dict)
}

/// 慢速反查 value → key; 未找到返回 undefined (objects.cc:5857)。
///
/// 原版 ToKey 含 Shape::Unwrap (GlobalDictionary 解 PropertyCell);
/// 泛型面直接返回键槽原值 (Name/Number 系 Unwrap 即恒等)。
///
/// v8: src/objects/objects.cc Dictionary::SlowReverseLookup
pub fn slow_reverse_lookup(
  dict: Tagged<HashTable>,
  value: Tagged<Object>,
  shape: DictionaryShapeParams,
) -> Tagged<Object> {
  let hash_shape = to_hash_shape(shape);
  (0..dict.capacity())
    .map(|i| InternalIndex::new(i as usize))
    .find(|&entry| {
      let k = dict.key_at(entry, hash_shape);
      hash_table::is_key(k) && value_at(dict, entry, shape) == value
    })
    .map_or(UNDEFINED, |entry| dict.key_at(entry, hash_shape))
}

// dictionary-inl.h 定义清单 (对数基准, 30 项):
// NumberDictionary::requires_slow_elements
// NumberDictionary::max_number_key
// NumberDictionary::set_requires_slow_elements
// GlobalDictionaryShape::Unwrap
// GlobalDictionary::GetMap
// NameDictionary::NameAt
// NameDictionary::GetMap
// NameDictionary::flags
// NameDictionary::set_flags
// GlobalDictionary::CellAt
// GlobalDictionary::NameAt
// GlobalDictionary::ValueAt
// GlobalDictionary::SetEntry
// GlobalDictionary::ClearEntry
// GlobalDictionary::ValueAtPut
// NumberDictionaryBaseShape::IsMatch
// NumberDictionaryBaseShape::Hash
// NumberDictionaryBaseShape::HashForObject
// NumberDictionaryBaseShape::AsHandle
// NumberDictionary::GetMap
// SimpleNameDictionary::GetMap
// SimpleNumberDictionary::GetMap
// BaseNameDictionaryShape::IsMatch
// BaseNameDictionaryShape::Hash
// BaseNameDictionaryShape::HashForObject
// GlobalDictionaryShape::IsMatch
// GlobalDictionaryShape::HashForObject
// BaseNameDictionaryShape::AsHandle
// GlobalDictionaryShape::DetailsAt
// GlobalDictionaryShape::DetailsAtPut

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_NEXT_ENUMERATION_INDEX_INDEX == 3 && K_OBJECT_HASH_INDEX == 4 && K_FLAGS_INDEX == 5);
  assert!(K_ENTRY_VALUE_INDEX == 1 && K_ENTRY_DETAILS_INDEX == 2);
  assert!(K_INITIAL_CAPACITY == 2 && K_FLAGS_DEFAULT == 0);
  assert!(MAY_HAVE_INTERESTING_PROPERTIES_MASK == 1);
  assert!(K_REQUIRES_SLOW_ELEMENTS_MASK == 1 && K_REQUIRES_SLOW_ELEMENTS_TAG_SIZE == 1);
  assert!(K_REQUIRES_SLOW_ELEMENTS_LIMIT == (1 << 29) - 1);
  assert!(K_PREFER_FAST_ELEMENTS_SIZE_FACTOR == 3);
  assert!(!requires_slow_elements(0) && requires_slow_elements(1) && requires_slow_elements(6));
  assert!(max_number_key(6) == 3);
  assert!(NAME_DICTIONARY_SHAPE.prefix_size == 3 && NAME_DICTIONARY_SHAPE.entry_size == 3);
  assert!(GLOBAL_DICTIONARY_SHAPE.entry_size == 1 && GLOBAL_DICTIONARY_SHAPE.prefix_size == 2);
  assert!(
    !SIMPLE_NAME_DICTIONARY_SHAPE.has_details && SIMPLE_NAME_DICTIONARY_SHAPE.entry_size == 2
  );
  assert!(
    NUMBER_DICTIONARY_SHAPE.prefix_size == 1 && SIMPLE_NUMBER_DICTIONARY_SHAPE.prefix_size == 0
  );
};
