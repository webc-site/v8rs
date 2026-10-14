// 由 tool/src/objects/ordered-hash-table.cc.js 从 v8/src/objects/ordered-hash-table.{cc,tq} 生成; 修订就地改, 复跑覆盖
// 有序哈希表 .cc 容量规则 + .tq Small 布局 (基座常量归 ordered_hash_table.rs)

//! 有序哈希表 .cc/.tq 段规则面 (大表 Ordered* 与小表 SmallOrdered*)。
//!
//! 布局常量 (kLoadFactor=2 / kNotFound=-1 / kInitialCapacity=4 /
//! Small kMaxCapacity=254 / 0xFF) 归 crate::ordered_hash_table
//! (ordered-hash-table.h.js + 开发模块)。
//!
//! 本文件登记: 容量演算 + 扩缩阈值 + .tq Small 布局字段序。

// ── 容量演算 (ordered-hash-table.cc:22-53 Allocate) ────────────────────
/// 分配容量归一: 2 的幂, 下限 kInitialCapacity (bit_ceil(max(4, capacity)))。
#[inline]
#[must_use]
pub const fn normalized_capacity(capacity: u32) -> u32 {
  let c = if capacity < 4 { 4 } else { capacity };
  c.next_power_of_two()
}

/// 桶数 = 容量 / 负载因子 2 (容量恒 2 的幂 → 整除)。
#[inline]
#[must_use]
pub const fn number_of_buckets(capacity: u32) -> u32 {
  capacity / 2
}

/// 底层数组长度 = HashTableStartIndex + 桶数 + 容量 × 每项槽数。
#[inline]
#[must_use]
pub const fn backing_length(
  hash_table_start_index: usize,
  capacity: u32,
  entry_size: usize,
) -> usize {
  hash_table_start_index + (capacity / 2) as usize + capacity as usize * entry_size
}

// ── 扩容/收缩阈值 (ordered-hash-table.cc:79-119) ───────────────────────
/// 添加前容量决策 (EnsureCapacityForAdding):
/// - nof + nod < capacity → 不动
/// - capacity == 0 → kInitialCapacity (4)
/// - nod ≥ capacity/2 → 原容量重哈希 (清删除项即可)
/// - 否则倍增
#[inline]
#[must_use]
pub const fn ensure_capacity_target(
  number_of_elements: u32,
  number_of_deleted_elements: u32,
  capacity: u32,
) -> Option<u32> {
  if number_of_elements + number_of_deleted_elements < capacity {
    return None;
  }
  if capacity == 0 {
    return Some(4); // kInitialCapacity
  }
  if number_of_deleted_elements >= (capacity >> 1) {
    return Some(capacity);
  }
  Some(capacity << 1)
}

/// 收缩决策 (Shrink): nof ≥ capacity/4 保留; 否则减半。
#[inline]
#[must_use]
pub const fn shrink_target(number_of_elements: u32, capacity: u32) -> Option<u32> {
  if number_of_elements >= (capacity >> 2) {
    return None;
  }
  Some(capacity / 2)
}

// ── .tq Small 布局规则 (ordered-hash-table.tq) ─────────────────────────
/// Small 表公共头字段序 (uint8 三计数 + padding)。
pub const SMALL_TABLE_HEAD_FIELDS: [&str; 4] = [
  "number_of_elements: uint8",
  "number_of_deleted_elements: uint8",
  "number_of_buckets: uint8 (const)",
  "padding[5|1]: uint8 (TAGGED_SIZE_8_BYTES ? 5 : 1)",
];
/// SmallOrderedHashSet 字段序 (tq 1:1, 条件 padding 两行并列)。
pub const SMALL_ORDERED_HASH_SET_FIELDS: [&str; 8] = [
  "number_of_elements: uint8",
  "number_of_deleted_elements: uint8",
  "number_of_buckets: uint8",
  "padding[5]: uint8",
  "padding[1]: uint8",
  "data_table[number_of_buckets * kSmallOrderedHashTableLoadFactor]: JSAny|TheHole",
  "hash_table[number_of_buckets]: uint8",
  "chain_table[number_of_buckets * kSmallOrderedHashTableLoadFactor]: uint8",
];
/// SmallOrderedHashMap 字段序 (tq 1:1)。
pub const SMALL_ORDERED_HASH_MAP_FIELDS: [&str; 8] = [
  "number_of_elements: uint8",
  "number_of_deleted_elements: uint8",
  "number_of_buckets: uint8",
  "padding[5]: uint8",
  "padding[1]: uint8",
  "data_table[number_of_buckets * kSmallOrderedHashTableLoadFactor]: HashMapEntry",
  "hash_table[number_of_buckets]: uint8",
  "chain_table[number_of_buckets * kSmallOrderedHashTableLoadFactor]: uint8",
];
/// Small 表数据表长度 = number_of_buckets × kLoadFactor(2)。
pub const SMALL_DATA_TABLE_ENTRIES_PER_BUCKET: u32 = 2;
/// Small 表哈希桶/链表初始值 = kNotFound (0xFF)。
pub const SMALL_NOT_FOUND_U8: u8 = 0xFF;
/// Small 表容量上限 (kMaxCapacity = 254, uint8 桶计数约束)。
pub const SMALL_MAX_CAPACITY: u32 = 254;
/// SmallOrderedNameDictionary 前置 hash 字段 (int32; 其余同 Small 表头)。
pub const SMALL_NAME_DICT_HASH_FIELD: &str = "hash: int32 (前置)";

// ── 方法面登记 (对数基准) ──────────────────────────────────────────────
/// OrderedHashTable .cc 定义 (9 项)。
pub const ORDEREDHASHTABLE_CC_METHODS: [&str; 9] = [
  "Allocate",
  "AllocateEmpty",
  "Clear",
  "Delete",
  "EnsureCapacityForAdding",
  "FindEntry",
  "HasKey",
  "Rehash",
  "Shrink",
];

/// OrderedHashSet .cc 定义 (7 项)。
pub const ORDEREDHASHSET_CC_METHODS: [&str; 7] = [
  "Add",
  "Allocate",
  "AllocateEmpty",
  "ConvertToKeysArray",
  "EnsureCapacityForAdding",
  "GetEmpty",
  "Rehash",
];

/// OrderedHashMap .cc 定义 (8 项)。
pub const ORDEREDHASHMAP_CC_METHODS: [&str; 8] = [
  "Add",
  "Allocate",
  "AllocateEmpty",
  "EnsureCapacityForAdding",
  "GetEmpty",
  "GetHash",
  "Rehash",
  "SetEntry",
];

/// OrderedNameDictionary .cc 定义 (10 项)。
pub const ORDEREDNAMEDICTIONARY_CC_METHODS: [&str; 10] = [
  "Add",
  "Allocate",
  "AllocateEmpty",
  "DeleteEntry",
  "EnsureCapacityForAdding",
  "FindEntry",
  "Grow",
  "Rehash",
  "SetEntry",
  "Shrink",
];

/// SmallOrderedHashTableImpl .cc 定义 (8 项)。
pub const SMALLORDEREDHASHTABLEIMPL_CC_METHODS: [&str; 8] = [
  "Allocate",
  "Delete",
  "FindEntry",
  "Grow",
  "HasKey",
  "Initialize",
  "Rehash",
  "Shrink",
];

/// SmallOrderedHashSet .cc 定义 (5 项)。
pub const SMALLORDEREDHASHSET_CC_METHODS: [&str; 5] = ["Add", "Delete", "Grow", "HasKey", "Rehash"];

/// SmallOrderedHashMap .cc 定义 (5 项)。
pub const SMALLORDEREDHASHMAP_CC_METHODS: [&str; 5] = ["Add", "Delete", "Grow", "HasKey", "Rehash"];

/// OrderedHashTableIterator .cc 定义 (2 项)。
pub const ORDEREDHASHTABLEITERATOR_CC_METHODS: [&str; 2] = ["HasMore", "Transition"];

// ── 堆访问签名骨架 (就地修订; 参数为 Address/索引占位) ─────────────────
/// 分配表 (Allocate: 归一容量 + 桶清 kNotFound + 计数清零)。
///
/// v8: src/objects/ordered-hash-table.cc OrderedHashTable::Allocate
pub fn allocate(_isolate: u64, capacity: u32, _allocation: u8) -> u64 {
  // 容量归一 normalized_capacity 已落上方 const fn; 实体分配需 Isolate Factory
  // (NewOrderedHashSet/Map/NameDictionary) + 按 map instance_type 的变体分派 → L4 推迟
  let _ = normalized_capacity(capacity);
  todo!("L4: 需 Isolate Factory 分配 + Set/Map/NameDictionary/Small 变体分派 + MemoryAllocator")
}
/// 空表分配 (AllocateEmpty: 仅 HashTableStartIndex 头, 桶数 0)。
///
/// v8: src/objects/ordered-hash-table.cc OrderedHashTable::AllocateEmpty
pub fn allocate_empty(_isolate: u64, _root_index: u32) -> u64 {
  todo!("L4: 需 Isolate Factory::NewOrderedHashTable(empty) + root index → 空表布局")
}
/// 重哈希 (Rehash: 旧表链遍历 → 新表插入 + 链表重建)。
///
/// v8: src/objects/ordered-hash-table.cc OrderedHashTable::Rehash
pub fn rehash(_isolate: u64, _table: u64, new_capacity: u32) -> u64 {
  // 目标容量由 EnsureCapacityForAdding 决策 (ensure_capacity_target const fn 已落);
  // 实体重哈希需 Factory 分配新表 + 按变体调 trait rehash → L4 推迟
  let _ = new_capacity;
  todo!("L4: 需 Isolate Factory 分配新表 + 变体分派调 OrderedHashSetExt/MapExt::rehash")
}
/// 删除项 (Delete: 链摘除 + 计数调整 + 必要时 Shrink)。
///
/// v8: src/objects/ordered-hash-table.cc OrderedHashTable::Delete
pub fn delete(_isolate: u64, _table: u64, _key: u64) -> (u64, bool) {
  todo!("L4: 需按 map instance_type 选 Set/Map/NameDictionary 变体后 trait delete + Factory Shrink")
}
/// Clear (清空为 canonical empty)。
///
/// v8: src/objects/ordered-hash-table.cc OrderedHashTable::Clear
pub fn clear(_isolate: u64, _table: u64) -> u64 {
  todo!("L4: 需 Factory 分配 canonical empty + 原表写 obsolete 指针 + 变体分派 trait clear")
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(
    normalized_capacity(0) == 4 && normalized_capacity(5) == 8 && normalized_capacity(16) == 16
  );
  assert!(number_of_buckets(8) == 4);
  assert!(backing_length(4, 8, 2) == 4 + 4 + 16);
  assert!(ensure_capacity_target(3, 0, 4).is_none());
  assert!(matches!(ensure_capacity_target(4, 0, 4), Some(8)));
  assert!(matches!(ensure_capacity_target(0, 0, 0), Some(4)));
  assert!(matches!(ensure_capacity_target(2, 2, 4), Some(4))); // 满且删除过半: 原容量重哈希
  assert!(shrink_target(4, 16).is_none() && matches!(shrink_target(3, 16), Some(8)));
  assert!(SMALL_NOT_FOUND_U8 == 0xFF && SMALL_MAX_CAPACITY == 254);
  assert!(SMALL_DATA_TABLE_ENTRIES_PER_BUCKET == 2);
  assert!(
    ORDEREDHASHTABLE_CC_METHODS.len() == 9
      && ORDEREDHASHSET_CC_METHODS.len() == 7
      && ORDEREDHASHMAP_CC_METHODS.len() == 8
      && ORDEREDNAMEDICTIONARY_CC_METHODS.len() == 10
      && SMALLORDEREDHASHTABLEIMPL_CC_METHODS.len() == 8
      && SMALLORDEREDHASHSET_CC_METHODS.len() == 5
      && SMALLORDEREDHASHMAP_CC_METHODS.len() == 5
      && ORDEREDHASHTABLEITERATOR_CC_METHODS.len() == 2
  );
};
