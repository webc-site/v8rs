// 由 tool/src/objects/swiss-name-dictionary.cc.js 从 v8/src/objects/swiss-name-dictionary.{cc,tq} 生成; 修订就地改, 复跑覆盖
// swiss 字典 .cc 方法面 + .tq 容量/元表公式 (基座常量归 swiss_name_dictionary.rs)

//! SwissNameDictionary 的 .cc/.tq 段规则面。
//!
//! 布局常量 (kNotFoundSentinel=-1 / kInitialCapacity=4 / kGroupWidth=16 /
//! kMax1Byte=256 / kMax2Byte=65536 / kDataTableEntryCount=2) 归
//! crate::swiss_name_dictionary (swiss-name-dictionary.h.js + 开发模块)。
//!
//! 本文件登记: 容量公式 + 最大可用容量 + 元表尺寸 + 收缩阈值 + 方法面。

// ── 容量公式 (swiss-name-dictionary.tq:63-80 CapacityFor) ──────────────
/// 至少容纳 at_least_space_for 的容量 (SIMD kGroupWidth=16 分支)。
///
/// ≤4 段: 0 → 0; <4 → 4; ==4 → 4 (组宽 16);
/// 其余: (n + n/7) 上取 2 的幂。
#[inline]
#[must_use]
pub const fn capacity_for(at_least_space_for: u32, group_width: u32) -> u32 {
  if at_least_space_for <= 4 {
    if at_least_space_for == 0 {
      return 0;
    }
    if at_least_space_for < 4 {
      return 4;
    }
    if group_width == 8 {
      return 8; // 便携组宽下容量 4 不可全用 (见 max_usable_capacity 特例)
    }
    return 4;
  }
  let non_normalized = at_least_space_for + at_least_space_for / 7;
  non_normalized.next_power_of_two()
}

// ── 最大可用容量 (tq:84-92 MaxUsableCapacity) ──────────────────────────
/// 容量对应的最大可用元素数 (删除计数上限)。
///
/// 组宽 8 且容量 4 → 3 (控制字节不足);
/// 其余 → capacity - capacity/8 (留 1/8 空槽保证探测终止)。
#[inline]
#[must_use]
pub const fn max_usable_capacity(capacity: u32, group_width: u32) -> u32 {
  if group_width == 8 && capacity == 4 {
    return 3;
  }
  capacity - capacity / 8
}

// ── 元表尺寸 (tq:106-125 MetaTableSizePerEntryFor / MetaTableSizeFor) ──
/// 每枚举条目的元表字节数 (容量分档: ≤256 → 1B, ≤65536 → 2B, 其余 4B)。
#[inline]
#[must_use]
pub const fn meta_table_size_per_entry_for(capacity: u32) -> u32 {
  if capacity <= (1 << 8) {
    1
  } else if capacity <= (1 << 16) {
    2
  } else {
    4
  }
}

/// 元表总字节数 = (2 + max_usable_capacity) × 每条目字节。
///
/// 头 2 项 = 元素计数 + 删除计数 (kMetaTableElementCountFieldIndex /
/// kMetaTableDeletedElementCountFieldIndex, 归 swiss_name_dictionary.rs)。
#[inline]
#[must_use]
pub const fn meta_table_size_for(capacity: u32, group_width: u32) -> u32 {
  (2 + max_usable_capacity(capacity, group_width)) * meta_table_size_per_entry_for(capacity)
}

// ── 对象尺寸 (tq:96-102 SizeFor; Address 布局公式) ─────────────────────
/// SizeFor 常量段: kHeapObjectHeaderSize + 8 (hash u32 + capacity u32) + kTaggedSize (meta_table)。
pub const SIZE_FOR_CONSTANT_TAGGED8: usize = 8 + 4 + 4; // kHeapObjectHeaderSize=8, kTaggedSize=4 (压缩)
/// SizeFor 动态段: capacity × (2×kTaggedSize + 2) + kGroupWidth。
#[inline]
#[must_use]
pub const fn size_for_dynamic(capacity: u32, group_width: u32) -> usize {
  capacity as usize * (2 * 4 + 2) + group_width as usize
}

// ── 收缩规则 (cc:195-213 Shrink + tq:265 删除触发) ─────────────────────
/// 收缩触发: 元素数 < capacity/4 (删除路径内联判定, tq:265)。
#[inline]
#[must_use]
pub const fn should_shrink(number_of_elements: u32, capacity: u32) -> bool {
  number_of_elements < (capacity >> 2)
}

/// 收缩目标容量: max(capacity/2, kInitialCapacity=4) (cc:207)。
#[inline]
#[must_use]
pub const fn shrink_target(capacity: u32) -> u32 {
  if capacity / 2 > 4 { capacity / 2 } else { 4 }
}

// ── .tq 布局字段序 (swiss-name-dictionary.tq:8-15) ─────────────────────
/// SwissNameDictionary 字段序 (tq 1:1)。
pub const SWISS_NAME_DICTIONARY_FIELDS: [&str; 6] = [
  "hash",
  "capacity",
  "meta_table",
  "data_table",
  "ctrl_table",
  "property_details_table",
];
/// ctrl_table 长度公式: capacity + kGroupWidth (含整组探测冗余; SIMD=16)。
pub const CTRL_TABLE_EXTRA: u32 = 16;

// ── .cc 方法面 (9 项, 对数基准) ────────────────────────
/// SwissNameDictionary 全部 .cc 定义。
pub const SWISS_NAME_DICTIONARY_CC_METHODS: [&str; 9] = [
  "DeleteEntry",
  "EqualsForTesting",
  "Initialize",
  "MaxUsableCapacity",
  "NumberOfEnumerableProperties",
  "Rehash",
  "ShallowCopy",
  "Shrink",
  "SlowReverseLookup",
];

// ── .tq 宏/CSA 接口面 (15 项) ───────────────────────
/// tq 宏与 extern 宏 (CSA 侧入口; SIMD/Portable 双分支 + 元表/控制字节原语)。
pub const TQ_EXPORT_MACROS: [&str; 15] = [
  "SwissNameDictionaryAddPortable",
  "SwissNameDictionaryAddSIMD",
  "SwissNameDictionaryCapacityFor",
  "SwissNameDictionaryCtrlTableStartOffsetMT",
  "SwissNameDictionaryDataTableStartOffsetMT",
  "SwissNameDictionaryDelete",
  "SwissNameDictionaryFindEntryPortable",
  "SwissNameDictionaryFindEntrySIMD",
  "SwissNameDictionaryIncreaseElementCountOrBailout",
  "SwissNameDictionaryMaxUsableCapacity",
  "SwissNameDictionaryMetaTableSizeFor",
  "SwissNameDictionaryMetaTableSizePerEntryFor",
  "SwissNameDictionarySetCtrl",
  "SwissNameDictionarySizeFor",
  "SwissNameDictionaryUpdateCountsForDeletion",
];

// ── 堆访问签名骨架 (就地修订; 参数为 Address/索引占位) ─────────────────

use core::slice::from_raw_parts;

use v8_objects_core::{property_details::PropertyAttributes, tagged::Tagged};

use crate::{
  internal_index::InternalIndex,
  swiss_name_dictionary::{K_GROUP_WIDTH, SwissNameDictionary, SwissNameDictionaryExt, is_full},
};

/// 删除 entry (DeleteEntry: kDeleted 控制字节 + 数据清 the_hole + 计数 + Shrink)。
///
/// v8: src/objects/swiss-name-dictionary.cc SwissNameDictionary::DeleteEntry
/// 核心逻辑: SetCtrl(kDeleted) + ClearDataTableEntry + 计数调整; Shrink 需 Factory → 返回原表。
pub fn delete_entry(_isolate: u64, table: u64, entry: u32) -> u64 {
  // SAFETY: table 为调用方 (DeleteEntry) 传入的合法 SwissNameDictionary Tagged 堆地址
  let dict: Tagged<SwissNameDictionary> = unsafe { Tagged::from_raw_address(table as usize) };
  dict.delete_entry(InternalIndex::new(entry as usize));
  // v8: cc:41 return Shrink(isolate, table); Shrink 需 Isolate Factory 分配 → 推迟, 原表地址回传
  table
}
/// 整表重哈希 (Rehash: 枚举序保序迁移 + meta 表重建)。
///
/// v8: src/objects/swiss-name-dictionary.cc SwissNameDictionary::Rehash
pub fn rehash(_isolate: u64, _table: u64, _new_capacity: u32) -> u64 {
  todo!("L4: 需 Isolate Factory::NewSwissNameDictionaryWithCapacity 分配新表 + 枚举序迁移")
}
/// 原地重哈希 (Rehash(isolate) 无参形态: 同容量清删除项)。
///
/// v8: src/objects/swiss-name-dictionary.cc SwissNameDictionary::Rehash(isolate)
pub fn rehash_in_place(_isolate: u64, table: u64) {
  // SAFETY: table 为调用方 (Rehash) 传入的合法 SwissNameDictionary Tagged 堆地址
  let dict: Tagged<SwissNameDictionary> = unsafe { Tagged::from_raw_address(table as usize) };
  dict.rehash_in_place();
}
/// 浅拷贝 (ShallowCopy: 布局 + ctrl/meta/数据表逐段复制)。
///
/// v8: src/objects/swiss-name-dictionary.cc SwissNameDictionary::ShallowCopy
pub fn shallow_copy(_isolate: u64, _table: u64) -> u64 {
  todo!("L4: 需 Factory 分配新表 + MemCopy 数据/控制/元表逐段复制")
}
/// 收缩 (Shrink: rehash 至 shrink_target)。
///
/// v8: src/objects/swiss-name-dictionary.cc SwissNameDictionary::Shrink
pub fn shrink(_isolate: u64, _table: u64) -> u64 {
  todo!("L4: 需 Rehash → Factory 分配; 收缩判定公式 shrink_target 已落上方 const fn")
}
/// 可枚举属性计数 (NumberOfEnumerableProperties)。
///
/// v8: src/objects/swiss-name-dictionary.cc SwissNameDictionary::NumberOfEnumerableProperties
pub fn number_of_enumerable_properties(table: u64) -> i32 {
  // SAFETY: table 为调用方 (NumberOfEnumerableProperties) 传入的合法 SwissNameDictionary 堆地址
  let dict: Tagged<SwissNameDictionary> = unsafe { Tagged::from_raw_address(table as usize) };
  dict
    .iter_entries_ordered()
    .filter(|&idx| {
      !dict
        .details_at(idx)
        .attributes()
        .contains(PropertyAttributes::DONT_ENUM)
    })
    .count() as i32
}
/// 慢速反查 (SlowReverseLookup: 值 → 键)。
///
/// v8: src/objects/swiss-name-dictionary.cc SwissNameDictionary::SlowReverseLookup
pub fn slow_reverse_lookup(_isolate: u64, table: u64, value: u64) -> u64 {
  // SAFETY: table 为调用方传入的合法 SwissNameDictionary Tagged 堆地址
  let dict: Tagged<SwissNameDictionary> = unsafe { Tagged::from_raw_address(table as usize) };
  dict
    .iter_entries_ordered()
    .find(|&idx| dict.value_at(idx).ptr() as u64 == value)
    .map_or(0, |idx| dict.key_at(idx).ptr() as u64)
}
/// 测试等价 (EqualsForTesting: 逐表比对)。
///
/// v8: src/objects/swiss-name-dictionary.cc SwissNameDictionary::EqualsForTesting
pub fn equals_for_testing(a: u64, b: u64) -> bool {
  // SAFETY: a/b 为测试调用方传入的两个合法 SwissNameDictionary 堆地址
  let dict_a: Tagged<SwissNameDictionary> = unsafe { Tagged::from_raw_address(a as usize) };
  let dict_b: Tagged<SwissNameDictionary> = unsafe { Tagged::from_raw_address(b as usize) };
  if dict_a.capacity() != dict_b.capacity()
    || dict_a.number_of_elements() != dict_b.number_of_elements()
    || dict_a.number_of_deleted_elements() != dict_b.number_of_deleted_elements()
    || dict_a.identity_hash() != dict_b.identity_hash()
  {
    return false;
  }
  let cap = dict_a.capacity();
  let ctrl_len = cap + K_GROUP_WIDTH;
  // 比对控制字节表 (含 kGroupWidth 尾部镜像区; 原版 cc:98, 零拷贝单次切片比较)
  // SAFETY: 两表容量相等，ctrl_table_ptr() 保证具有至少 ctrl_len 字节合法分配内存
  let ctrl_eq = unsafe {
    from_raw_parts(dict_a.ctrl_table_ptr(), ctrl_len)
      == from_raw_parts(dict_b.ctrl_table_ptr(), ctrl_len)
  };
  if !ctrl_eq {
    return false;
  }
  // 比对数据表 (key/value; full 槽另比 PropertyDetails; 原版 cc:103-110)
  for i in 0..cap {
    let idx = InternalIndex::new(i);
    if dict_a.key_at(idx).ptr() != dict_b.key_at(idx).ptr()
      || dict_a.value_at(idx).ptr() != dict_b.value_at(idx).ptr()
    {
      return false;
    }
    if is_full(dict_a.get_ctrl(i)) && dict_a.details_at(idx) != dict_b.details_at(idx) {
      return false;
    }
  }
  // 比对枚举序表 (原版 cc:111-114)
  for i in 0..dict_a.used_capacity() {
    if dict_a.entry_for_enumeration_index(i) != dict_b.entry_for_enumeration_index(i) {
      return false;
    }
  }
  true
}

// —— 提取不变式断言 ——
const _: () = {
  // CapacityFor (SIMD 分支)
  assert!(capacity_for(0, 16) == 0 && capacity_for(1, 16) == 4 && capacity_for(4, 16) == 4);
  assert!(capacity_for(4, 8) == 8); // 便携组宽特例
  assert!(capacity_for(5, 16) == 8); // 5 + 0 = 5 → 8
  assert!(capacity_for(8, 16) == 16); // 8 + 1 = 9 → 16
  assert!(capacity_for(56, 16) == 64); // 56 + 8 = 64
  // MaxUsableCapacity
  assert!(max_usable_capacity(4, 8) == 3);
  assert!(max_usable_capacity(4, 16) == 4);
  assert!(max_usable_capacity(16, 16) == 14);
  assert!(max_usable_capacity(256, 16) == 224);
  // 元表分档
  assert!(meta_table_size_per_entry_for(4) == 1 && meta_table_size_per_entry_for(256) == 1);
  assert!(meta_table_size_per_entry_for(257) == 2 && meta_table_size_per_entry_for(65536) == 2);
  assert!(meta_table_size_per_entry_for(65537) == 4);
  assert!(meta_table_size_for(16, 16) == (2 + 14) * meta_table_size_per_entry_for(16));
  assert!(size_for_dynamic(16, 16) == 16 * 10 + 16);
  // 收缩
  assert!(should_shrink(3, 16) && !should_shrink(4, 16));
  assert!(shrink_target(64) == 32 && shrink_target(8) == 4);
  assert!(CTRL_TABLE_EXTRA == 16);
  assert!(SWISS_NAME_DICTIONARY_CC_METHODS.len() == 9);
  assert!(TQ_EXPORT_MACROS.len() == 15);
};
