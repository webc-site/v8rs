#!/usr/bin/env bun
// tool/src/heap/ephemeron-remembered-set.h.js
// 机械提取 v8/src/heap/ephemeron-remembered-set.{h,cc}:
// EphemeronRememberedSet 常量/类型别名/方法台账 (Minor GC 专用 old-to-new 记忆集)。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/ephemeron-remembered-set.h");
const cc = readV8("src/heap/ephemeron-remembered-set.cc");

for (const a of [
  "class EphemeronRememberedSet final {",
  "static constexpr int kEphemeronTableListSegmentSize = 128;",
  "using TableList = ::heap::base::Worklist<Tagged<EphemeronHashTable>,",
  "using IndicesSet = absl::flat_hash_set<int>;",
  "using TableMap = std::unordered_map<Tagged<EphemeronHashTable>, IndicesSet,",
  "void RecordEphemeronKeyWrite(Tagged<EphemeronHashTable> table,",
  "void RecordEphemeronKeyWrites(Tagged<EphemeronHashTable> table,",
  "TableMap* tables() { return &tables_; }",
  "base::Mutex insertion_mutex_;",
  "TableMap tables_;",
]) {
  if (!h.includes(a)) throw new Error(`ephemeron-remembered-set.h 锚点缺失: ${a}`);
}
for (const fn of [
  "RecordEphemeronKeyWrite",
  "RecordEphemeronKeyWrites",
]) {
  if (!new RegExp(`void EphemeronRememberedSet::${fn}\\(`).test(cc)) {
    throw new Error(`ephemeron-remembered-set.cc: ${fn} 定义缺失`);
  }
}
console.log("EphemeronRememberedSet: 常量 1 + 别名 3 + 方法 2");

let rust = banner(
  "src/heap/ephemeron-remembered-set.h.js",
  "src/heap/ephemeron-remembered-set.{h,cc}",
  "ephemeron 键记忆集: 老生代表→新生代键 Minor GC 记忆集骨架",
);

rust += `
/// 表清单 worklist 分段大小 (对应 kEphemeronTableListSegmentSize = 128)。
pub const K_EPHEMERON_TABLE_LIST_SEGMENT_SIZE: usize = 128;

/// ephemeron 表 → 记录键槽位集合 (对应 TableMap = unordered_map<
/// Tagged<EphemeronHashTable>, IndicesSet, Object::Hasher>)。
/// IndicesSet 对应 absl::flat_hash_set<int>。
/// v8: src/heap/ephemeron-remembered-set.h EphemeronRememberedSet::TableMap
pub type EphemeronIndicesSet = std::collections::HashSet<i32>;
pub type EphemeronTableMap = std::collections::HashMap<u64, EphemeronIndicesSet>;

/// ephemeron 记忆集 (对应 class EphemeronRememberedSet final)。
/// 仅 Minor GC 使用; Full GC 增量标记启动即整表清空。
/// v8: src/heap/ephemeron-remembered-set.h EphemeronRememberedSet
pub struct EphemeronRememberedSetSkeleton {
  /// 对应 base::Mutex insertion_mutex_; (互斥由外部临界区表达)
  /// 对应 TableMap tables_
  pub tables: EphemeronTableMap,
}

impl EphemeronRememberedSetSkeleton {
  /// 对应 RecordEphemeronKeyWrite(table, key_slot):
  /// 在 table 条目追加槽位 (键不存在则建表项)
  /// v8: src/heap/ephemeron-remembered-set.cc RecordEphemeronKeyWrite
  pub fn record_ephemeron_key_write(&mut self, table: u64, key_slot: i32) {
    self.tables.entry(table).or_default().insert(key_slot);
  }

  /// 对应 RecordEphemeronKeyWrites(table, indices): 批量并入
  /// v8: src/heap/ephemeron-remembered-set.cc RecordEphemeronKeyWrites
  pub fn record_ephemeron_key_writes(&mut self, table: u64, indices: EphemeronIndicesSet) {
    self.tables.entry(table).or_default().extend(indices);
  }

  /// 对应 tables() { return &tables_; }
  pub fn tables(&mut self) -> &mut EphemeronTableMap {
    &mut self.tables
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(K_EPHEMERON_TABLE_LIST_SEGMENT_SIZE, 128);
  let mut s = EphemeronRememberedSetSkeleton { tables: EphemeronTableMap::new() };
  s.record_ephemeron_key_write(1, 7);
  s.record_ephemeron_key_write(1, 9);
  s.record_ephemeron_key_write(2, 5);
  assert_eq!(s.tables().len(), 2);
  assert_eq!(s.tables()[&1].len(), 2);
  let mut batch = EphemeronIndicesSet::new();
  batch.insert(7);
  batch.insert(11);
  s.record_ephemeron_key_writes(2, batch);
  assert_eq!(s.tables()[&2].len(), 2);
};
`;

const p = writeSrc("v8-heap-core", "ephemeron_remembered_set.rs.skel", rust);
console.log(`written ${p}`);
