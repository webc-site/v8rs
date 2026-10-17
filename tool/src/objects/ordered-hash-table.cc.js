#!/usr/bin/env bun
// tool/src/objects/ordered-hash-table.cc.js — 覆盖 ordered-hash-table 同名族 .cc/.tq 段:
//   v8/src/objects/ordered-hash-table.cc + ordered-hash-table.tq
//   → v8-objects-collections/src/ordered_hash_table_cc_generated.rs
// 布局常量 (.h) 归 ordered-hash-table.h.js 与已开发模块 ordered_hash_table.rs;
// 本脚本只提取 .cc 容量演算规则 (Allocate/EnsureCapacityForAdding/Shrink)
// 与 .tq SmallOrdered* 布局规则, 函数体留签名骨架 + todo!("就地修订")。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/objects/ordered-hash-table.cc");
const tq = readV8("src/objects/ordered-hash-table.tq");
const h = readV8("src/objects/ordered-hash-table.h"); // 常量取值侧锚

// ── .cc 容量规则锚 (must 断言) ────────────────────────────────────────────
const anchors = [
  [cc, "capacity = std::bit_ceil<uint32_t>(std::max({kInitialCapacity, capacity}));", "ordered-hash-table.cc (Allocate)"],
  [cc, "int num_buckets = capacity / kLoadFactor;", "ordered-hash-table.cc (Allocate)"],
  [cc, "int length = HashTableStartIndex() + num_buckets + (capacity * kEntrySize);", "ordered-hash-table.cc (Allocate)"],
  [cc, "new_capacity = kInitialCapacity;", "ordered-hash-table.cc (空表起步)"],
  [cc, "} else if (nod >= (capacity >> 1)) {", "ordered-hash-table.cc (删除过半不扩容)"],
  [cc, "new_capacity = capacity << 1;", "ordered-hash-table.cc (倍增)"],
  [cc, "if (nof >= (capacity >> 2)) return table;", "ordered-hash-table.cc (Shrink 阈值)"],
  [cc, "return Derived::Rehash(isolate, table, capacity / 2).ToHandleChecked();", "ordered-hash-table.cc (Shrink 减半)"],
  [cc, "memset(reinterpret_cast<uint8_t*>(hashtable_start), kNotFound,", "ordered-hash-table.cc (哈希桶清 kNotFound)"],
  [tq, "const kSmallOrderedHashTableMaxCapacity: constexpr int31", "ordered-hash-table.tq"],
  [tq, "hashTableSize = capacity / kSmallOrderedHashTableLoadFactor;", "ordered-hash-table.tq (Allocate 宏)"],
];
for (const [s, sub, f] of anchors) {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 52)}..."`);
}

// ── .h 常量取值侧锚 (只锚不复写, 归 ordered_hash_table.rs) ───────────────
const hAnchors = [
  "static const int kLoadFactor = 2;",
  "static const int kNotFound = -1;",
  "static const int kInitialCapacity = 4;",
  "static const int kMaxCapacity = 254;", // SmallOrderedHashTable
];
for (const a of hAnchors) {
  if (!h.includes(a)) throw new Error(`ordered-hash-table.h 常量锚失配: ${a}`);
}

// ── .cc 方法面 (数量 must 断言) ───────────────────────────────────────────
// 注意负向后顾: OrderedHashSet 排除 SmallOrderedHashSet 前缀误捕
const groups = {
  OrderedHashTable: [...new Set([...cc.matchAll(/OrderedHashTable<Derived, entrysize>::(\w+)\(/g)].map((m) => m[1]))].sort(),
  OrderedHashSet: [...new Set([...cc.matchAll(/(?<!Small)OrderedHashSet::(\w+)\(/g)].map((m) => m[1]))].sort(),
  OrderedHashMap: [...new Set([...cc.matchAll(/(?<!Small)OrderedHashMap::(\w+)\(/g)].map((m) => m[1]))].sort(),
  OrderedNameDictionary: [...new Set([...cc.matchAll(/OrderedNameDictionary::(\w+)\(/g)].map((m) => m[1]))].sort(),
  SmallOrderedHashTableImpl: [...new Set([...cc.matchAll(/SmallOrderedHashTableImpl<Derived>::(\w+)\(/g)].map((m) => m[1]))].sort(),
  SmallOrderedHashSet: [...new Set([...cc.matchAll(/SmallOrderedHashSet::(\w+)\(/g)].map((m) => m[1]))].sort(),
  SmallOrderedHashMap: [...new Set([...cc.matchAll(/SmallOrderedHashMap::(\w+)\(/g)].map((m) => m[1]))].sort(),
  OrderedHashTableIterator: [...new Set([...cc.matchAll(/OrderedHashTableIterator<Derived, TableType>::(\w+)\(/g)].map((m) => m[1]))].sort(),
};
const total = Object.values(groups).reduce((a, g) => a + g.length, 0);
// 手数基准 (原版逐类核对): 9/7/8/10/8/5/5/2 = 54
const EXPECT = { OrderedHashTable: 9, OrderedHashSet: 7, OrderedHashMap: 8, OrderedNameDictionary: 10, SmallOrderedHashTableImpl: 8, SmallOrderedHashSet: 5, SmallOrderedHashMap: 5, OrderedHashTableIterator: 2 };
for (const [g, n] of Object.entries(EXPECT)) {
  if (groups[g].length !== n) throw new Error(`${g} 方法应 ${n}, 实得 ${groups[g].length}: ${groups[g].join(",")}`);
}
if (total !== 54) throw new Error(`方法总数应 54, 实得 ${total}`);
if (groups.OrderedHashTable.join(",") !== "Allocate,AllocateEmpty,Clear,Delete,EnsureCapacityForAdding,FindEntry,HasKey,Rehash,Shrink") {
  throw new Error("OrderedHashTable 方法面漂移");
}

// ── .tq 布局规则 (SmallOrdered* 类字段, 数量断言) ─────────────────────────
function tqClassFields(name) {
  const m = tq.match(new RegExp(`extern class ${name} extends SmallOrderedHashTable \\{([\\s\\S]*?)\\n\\}`));
  if (!m) throw new Error(`tq 缺 extern class ${name}`);
  // 行级解析: 剥离 @if/@ifnot 前缀 (合并为条件字段) 与 const 修饰
  const raw = m[1].split("\n").map((l) => l.trim());
  const lines = raw.filter((l) => l && !l.startsWith("//") && /:$|:\s*\S/.test(l));
  const fields = [];
  for (let i = 0; i < lines.length; i++) {
    const l = lines[i];
    const fm = l.match(/^(?:@\w+\([^)]*\)\s*)?(?:const\s+)?(\w+)(?:\[(.*?)\])?\s*:\s*(.*?);?$/);
    if (!fm) throw new Error(`tq 字段不可解析 (${name}): ${l}`);
    let ty = fm[3].replace(/;$/, "");
    if (!ty && i + 1 < raw.length && !raw[raw.indexOf(l) + 1].includes(":")) {
      ty = raw[raw.indexOf(l) + 1].replace(/;$/, ""); // 类型续行
    }
    fields.push({ field: fm[1], extent: fm[2] ?? null, ty });
  }
  return fields;
}
const sosFields = tqClassFields("SmallOrderedHashSet");
const somFields = tqClassFields("SmallOrderedHashMap");
// 7 字段: 3 计数 + 条件 padding (两行合并视为 1) + 数据表 + 哈希表 + 链表
if (sosFields.length !== 8) throw new Error(`SmallOrderedHashSet 字段行应 8 (含两行条件 padding), 实得 ${sosFields.length}`);
if (somFields.length !== 8) throw new Error(`SmallOrderedHashMap 字段行应 8, 实得 ${somFields.length}`);
// 头三字段 + 尾三字段 must 断言 (禁吞账)
for (const fs of [sosFields, somFields]) {
  if (fs[0].field !== "number_of_elements" || fs[1].field !== "number_of_deleted_elements" || fs[2].field !== "number_of_buckets") {
    throw new Error("Small 表头字段序失配");
  }
  const names = fs.map((f) => f.field);
  if (names[5] !== "data_table" || names[6] !== "hash_table" || names[7] !== "chain_table") {
    throw new Error(`Small 表尾字段序失配: ${names.join(",")}`);
  }
  if (!fs[5].extent?.includes("* kSmallOrderedHashTableLoadFactor")) throw new Error("数据表长度公式失配");
}
// 链表/哈希表初始化值: kNotFound (0xFF)
if (!tq.includes("ConstantIterator<uint8>(kSmallOrderedHashTableNotFound)")) throw new Error("kNotFound 初始化锚失配");
// SmallOrderedNameDictionary 前置 hash 字段
if (!tq.includes("extern class SmallOrderedNameDictionary extends SmallOrderedHashTable {\n  hash: int32;")) throw new Error("SmallOrderedNameDictionary hash 前置字段失配");

const body = `${banner("src/objects/ordered-hash-table.cc.js", "src/objects/ordered-hash-table.{cc,tq}", "有序哈希表 .cc 容量规则 + .tq Small 布局 (基座常量归 ordered_hash_table.rs)")}
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
pub const fn backing_length(hash_table_start_index: usize, capacity: u32, entry_size: usize) -> usize {
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
pub const SMALL_ORDERED_HASH_SET_FIELDS: [&str; ${sosFields.length}] = [
${sosFields.map((f) => `  "${f.field}${f.extent ? `[${f.extent.trim().replace(/Convert<intptr>\(|\)/g, "").trim()}]` : ""}: ${f.ty}",`).join("\n")}
];
/// SmallOrderedHashMap 字段序 (tq 1:1)。
pub const SMALL_ORDERED_HASH_MAP_FIELDS: [&str; ${somFields.length}] = [
${somFields.map((f) => `  "${f.field}${f.extent ? `[${f.extent.trim().replace(/Convert<intptr>\(|\)/g, "").trim()}]` : ""}: ${f.ty}",`).join("\n")}
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
${Object.entries(groups).map(([g, list]) => `/// ${g} .cc 定义 (${list.length} 项)。
pub const ${g.toUpperCase()}_CC_METHODS: [&str; ${list.length}] = [
${list.map((s) => `  "${s}",`).join("\n")}
];`).join("\n\n")}

// ── 堆访问签名骨架 (就地修订; 参数为 Address/索引占位) ─────────────────
/// 分配表 (Allocate: 归一容量 + 桶清 kNotFound + 计数清零)。
pub fn allocate(_isolate: u64, capacity: u32, _allocation: u8) -> u64 { todo!("就地修订") }
/// 空表分配 (AllocateEmpty: 仅 HashTableStartIndex 头, 桶数 0)。
pub fn allocate_empty(_isolate: u64, _root_index: u32) -> u64 { todo!("就地修订") }
/// 重哈希 (Rehash: 旧表链遍历 → 新表插入 + 链表重建)。
pub fn rehash(_isolate: u64, _table: u64, new_capacity: u32) -> u64 { todo!("就地修订") }
/// 删除项 (Delete: 链摘除 + 计数调整 + 必要时 Shrink)。
pub fn delete(_isolate: u64, _table: u64, _key: u64) -> (u64, bool) { todo!("就地修订") }
/// Clear (清空为 canonical empty)。
pub fn clear(_isolate: u64, _table: u64) -> u64 { todo!("就地修订") }

// —— 提取不变式断言 ——
const _: () = {
  assert!(normalized_capacity(0) == 4 && normalized_capacity(5) == 8 && normalized_capacity(16) == 16);
  assert!(number_of_buckets(8) == 4);
  assert!(backing_length(4, 8, 2) == 4 + 4 + 16);
  assert!(ensure_capacity_target(3, 0, 4).is_none());
  assert!(ensure_capacity_target(4, 0, 4) == Some(8));
  assert!(ensure_capacity_target(0, 0, 0) == Some(4));
  assert!(ensure_capacity_target(1, 2, 4) == Some(4)); // 删除过半: 原容量重哈希
  assert!(shrink_target(4, 16).is_none() && shrink_target(3, 16) == Some(8));
  assert!(SMALL_NOT_FOUND_U8 == 0xFF && SMALL_MAX_CAPACITY == 254);
  assert!(SMALL_DATA_TABLE_ENTRIES_PER_BUCKET == 2);
  assert!(${Object.entries(groups).map(([g, l]) => `${g.toUpperCase()}_CC_METHODS.len() == ${l.length}`).join(" && ")});
};
`;

writeSrc("v8-objects-collections", "ordered_hash_table_cc_generated.rs", body);
console.log(
  `ordered_hash_table_cc_generated.rs: const fn 5 + Small 布局 ${sosFields.length}/${somFields.length} 字段 + 方法面 ${total} (${Object.entries(groups).map(([g, l]) => `${g}:${l.length}`).join(" ")}) + 骨架 5, 对数通过`,
);
