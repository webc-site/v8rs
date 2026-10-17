#!/usr/bin/env bun
// tool/src/objects/hash-table.h.js — 覆盖 hash-table 同名族:
//   v8/src/objects/hash-table.h + hash-table-inl.h
//   → v8-objects-collections/src/hash_table.rs
// 机械提取: HashTableBase / HashTable 槽位索引与容量常量 + 探测公式
//   + 各 Shape (kPrefixSize/kEntrySize/kMatchNeedsHoleCheck/kDoHashSpreading/kHashBits)
// 函数体: 纯计算 (ComputeCapacity/FirstProbe/NextProbe/EntryToIndex/IndexToEntry) 落 const fn;
//   堆访问方法 (FindEntry/Rehash/EnsureCapacity 等) 留签名骨架 + todo!("就地修订")。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/hash-table.h");
const inl = readV8("src/objects/hash-table-inl.h");

// ── 锚点与常量提取 (带 must 断言) ─────────────────────────────────────────
const grab = (s, re, f) => {
  const m = s.match(re);
  if (!m) throw new Error(`常量提取失配: ${f}`);
  return m[1];
};
const anchors = [
  [h, "class V8_EXPORT_PRIVATE HashTableBase : public NON_EXPORTED_BASE(FixedArray)", "hash-table.h"],
  [h, "static const int kNumberOfElementsIndex = 0;", "hash-table.h"],
  [h, "static const int kNumberOfDeletedElementsIndex = 1;", "hash-table.h"],
  [h, "static const int kCapacityIndex = 2;", "hash-table.h"],
  [h, "static const int kPrefixStartIndex = 3;", "hash-table.h"],
  [h, "static const int kEntryKeyIndex = 0;", "hash-table.h"],
  [h, "static const int kMinShrinkCapacity = 16;", "hash-table.h"],
  [h, "static const int kMinCapacityForPretenure = 256;", "hash-table.h"],
  [h, "static const int kMaxRegularCapacity = kMaxRegularHeapObjectSize / 32;", "hash-table.h"],
  [h, "static const uint32_t kMinCapacity = 4;", "hash-table.h"],
  [inl, "uint32_t raw_cap = at_least_space_for + (at_least_space_for >> 1);", "hash-table-inl.h"],
  [inl, "uint32_t capacity = std::bit_ceil(raw_cap);", "hash-table-inl.h"],
  [inl, "return std::max({capacity, kMinCapacity});", "hash-table-inl.h"],
  [h, "return InternalIndex((last.as_uint32() + number) & (size - 1));", "hash-table.h (NextProbe)"],
];
for (const [s, sub, f] of anchors) {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 52)}..."`);
}

// kMaxRegularHeapObjectSize = 1 << (kPageSizeBits - 1), 默认配置 kPageSizeBits=18 → 131072
// (src/base/build_config.h:80 默认分支; PPC64/巨页分支登记跳过)
const K_MAX_REGULAR_CAPACITY = 131072 / 32;
if (K_MAX_REGULAR_CAPACITY !== 4096) throw new Error("kMaxRegularCapacity 推导失配");

// kMaxFixedArrayCapacity (fixed-array-base.h:22-23, 非 LOWER_LIMITS 配置)
const fab = readV8("src/objects/fixed-array-base.h");
if (!/kMaxFixedArrayCapacity =\s*V8_LOWER_LIMITS_MODE_BOOL \? \(1 \* 1024 \* 1024\) : \(128 \* 1024 \* 1024\);/s.test(fab)) {
  throw new Error("kMaxFixedArrayCapacity 公式失配 (非 LOWER_LIMITS 分支应 128M)");
}
const kMaxFixedArrayCapacity = 128 * 1024 * 1024;

// EphemeronHashTableShape::kHashBits = 10 (定义于 objects.cc:5112, 跨文件引用核对)
const objectsCc = readV8("src/objects/objects.cc");
const kEphemeronHashBits = grab(objectsCc, /const uint32_t EphemeronHashTableShape::kHashBits = (\d+);/, "EphemeronHashTableShape::kHashBits");
if (kEphemeronHashBits !== "10") throw new Error("EphemeronHashTableShape::kHashBits != 10");

// ── Shape 规则面 (逐条解析 + 数量 must 断言) ──────────────────────────────
function shapeConsts(classSrc, name) {
  const out = {};
  const boolRe = /static const bool (k\w+) = (true|false);/g;
  let m;
  while ((m = boolRe.exec(classSrc)) !== null) out[m[1]] = m[2] === "true";
  const intRe = /static const int (k\w+) = (-?\d+);/g;
  while ((m = intRe.exec(classSrc)) !== null) out[m[1]] = Number(m[2]);
  // 表达式形态常量 (如 ObjectMultiHashTableShape 的 kEntrySize = 1 + N)
  const exprRe = /static const int (k\w+) = (1 \+ N);/g;
  while ((m = exprRe.exec(classSrc)) !== null) out[m[1]] = m[2];
  const u32Re = /static const uint32_t (kHashBits)(?: = (\d+))?;/g;
  while ((m = u32Re.exec(classSrc)) !== null) out[m[1]] = m[2] !== undefined ? Number(m[2]) : null;
  if (!Object.keys(out).length) throw new Error(`Shape ${name} 无常量`);
  return out;
}
function classBody(src, className) {
  // 词边界匹配, 避免命中前缀同名类 (如 ObjectHashTableShapeBase)
  const re = new RegExp(`class ${className}(?![\\w])`);
  const m = re.exec(src);
  if (!m) throw new Error(`未找到类 ${className}`);
  const rest = src.slice(m.index);
  // 取至下一个 class 声明或 EXTERN 宏
  const j = rest.slice(1).search(/\nclass |\nEXTERN_DECLARE|template </);
  return j < 0 ? rest : rest.slice(0, j + 1);
}
const shapes = [
  ["ObjectHashTableShapeBase", classBody(h, "ObjectHashTableShapeBase")],
  ["ObjectHashTableShape", classBody(h, "ObjectHashTableShape")],
  ["EphemeronHashTableShape", classBody(h, "EphemeronHashTableShape")],
  ["ObjectMultiHashTableShape", classBody(h, "ObjectMultiHashTableShape")],
  ["ObjectHashSetShape", classBody(h, "ObjectHashSetShape")],
  ["NameToIndexShape", classBody(h, "NameToIndexShape")],
  ["RegisteredSymbolTableShape", classBody(h, "RegisteredSymbolTableShape")],
];
if (shapes.length !== 7) throw new Error(`hash-table.h Shape 应 7 个, 实得 ${shapes.length}`);
const parsed = shapes.map(([n, s]) => [n, shapeConsts(s, n)]);
// 关键数值 must 断言 (禁吞账)
const must = (name, key, v) => {
  const got = parsed.find(([n]) => n === name)?.[1][key];
  if (got !== v) throw new Error(`${name}.${key} 应 ${v}, 实得 ${got}`);
};
must("ObjectHashTableShapeBase", "kPrefixSize", 0);
must("ObjectHashTableShapeBase", "kEntrySize", 2);
must("ObjectHashTableShapeBase", "kMatchNeedsHoleCheck", false);
must("ObjectHashTableShape", "kDoHashSpreading", false);
must("ObjectHashTableShape", "kHashBits", 0);
must("EphemeronHashTableShape", "kDoHashSpreading", true);
must("EphemeronHashTableShape", "kHashBits", null); // 定义在 objects.cc (补 10)
must("ObjectHashSetShape", "kEntrySize", 1);
must("NameToIndexShape", "kEntrySize", 2);
must("RegisteredSymbolTableShape", "kEntrySize", 2);
if (parsed.find(([n]) => n === "ObjectMultiHashTableShape")?.[1].kEntrySize !== "1 + N") {
  throw new Error("ObjectMultiHashTableShape kEntrySize = 1 + N 失配");
}

// ── cc/-inl 方法骨架清单 (FindEntry/Rehash 等堆访问段) ────────────────────
// 参数类型占位: Address=堆地址(u64), InternalIndex=u32
const skeletonFns = [
  ["find_entry", "(_roots: u64, _key: u64, hash: i32) -> u32", "按二次探测查键; undefined 空槽即停, the_hole 视 kMatchNeedsHoleCheck 跳过"],
  ["find_insertion_entry", "(_roots: u64, hash: i32) -> u32", "查可插入槽: 跳过 undefined/the_hole 直至非键槽"],
  ["rehash", "(_new_table: u64)", "整表重哈希 (EntryForProbe 逐 probe 归位)"],
  ["ensure_capacity", "(_isolate: u64, table: u64, n: i32) -> u64", "容量不足时扩容 (EnsureCapacity)"],
  ["has_sufficient_capacity_to_add", "(_n: i32) -> bool", "实例形态容量判定 (四参静态形态与 ComputeCapacity 同步, hash-table.cc 归属)"],
];
if (skeletonFns.length !== 5) throw new Error("骨架清单漂移");

// ── 产物 ────────────────────────────────────────────────────────────────
const body = `${banner("src/objects/hash-table.h.js", "src/objects/hash-table.{h,-inl.h} + 跨文件锚 objects.cc/fixed-array-base.h", "开放寻址二次探测哈希表基座 (Shape 参数化规则面)")}
//! HashTable 家族规则面: HashTableBase / HashTable<Derived, Shape>。
//!
//! - 空槽 = undefined, 删除槽 = the_hole; 探测遇 undefined 停, 遇 the_hole 继续
//! - 容量恒 2 的幂 (bit_ceil + 50% 松弛, 下限 kMinCapacity)
//! - 大表哈希分散 (kDoHashSpreading): hash * (size >> kHashBits) & (size-1)
//!
//! 槽位语义 (Smi 计数字段): [0]=元素数 [1]=删除数 [2]=容量 [3..3+prefix)=前缀。

// ── HashTableBase 槽位索引 (hash-table.h:103-109) ─────────────────────
/// 元素计数槽。
pub const K_NUMBER_OF_ELEMENTS_INDEX: usize = 0;
/// 删除元素计数槽。
pub const K_NUMBER_OF_DELETED_ELEMENTS_INDEX: usize = 1;
/// 容量槽。
pub const K_CAPACITY_INDEX: usize = 2;
/// 前缀起始槽 (前缀归各派生 Shape 使用)。
pub const K_PREFIX_START_INDEX: usize = 3;
/// 新表最小容量 (kMinCapacity)。
pub const K_MIN_CAPACITY: u32 = 4;

// ── HashTable 容量常量 (hash-table.h:201-220) ────────────────────────
/// 键槽在 entry 内的下标。
pub const K_ENTRY_KEY_INDEX: usize = 0;
/// 不收缩到该容量以下 (kMinShrinkCapacity)。
pub const K_MIN_SHRINK_CAPACITY: u32 = 16;
/// 超过该容量转为老年代分配 (kMinCapacityForPretenure)。
pub const K_MIN_CAPACITY_FOR_PRETENURE: u32 = 256;
/// 普通对象区最大容量 = kMaxRegularHeapObjectSize / 32 (默认页配置 kPageSizeBits=18)。
pub const K_MAX_REGULAR_CAPACITY: u32 = ${K_MAX_REGULAR_CAPACITY};
/// 底层 FixedArray 最大长度 (kMaxFixedArrayCapacity, 非 LOWER_LIMITS 配置)。
pub const K_FIXED_ARRAY_MAX_LENGTH: u32 = ${kMaxFixedArrayCapacity};

/// 计算至少容纳 at_least_space_for 的容量 (hash-table-inl.h:88 ComputeCapacity)。
///
/// 加 50% 松弛 + 2 的幂上取整, 下限 kMinCapacity;
/// 与 CodeStubAssembler::HashTableComputeCapacity 保持同步 (原版注释)。
#[inline]
#[must_use]
pub const fn compute_capacity(at_least_space_for: u32) -> u32 {
  let raw_cap = at_least_space_for + (at_least_space_for >> 1);
  let capacity = raw_cap.next_power_of_two();
  if capacity < K_MIN_CAPACITY { K_MIN_CAPACITY } else { capacity }
}

/// 下一探测位置: 二次探测 (hash-table.h:122 NextProbe)。
#[inline]
#[must_use]
pub const fn next_probe(last: u32, number: u32, size: u32) -> u32 {
  (last.wrapping_add(number)) & (size - 1)
}

/// 首探测位置 (hash-table.h:163 FirstProbe)。
///
/// 未启用哈希分散或 size ≤ 2^kHashBits 时直接取模;
/// 大表 + 哈希位不足时按 coefficient = size >> kHashBits 放大分散
/// (与 WeakCollectionsBuiltinsAssembler 生成的代码同步, 原版注释)。
#[inline]
#[must_use]
pub const fn first_probe(hash: u32, size: u32, do_hash_spreading: bool, hash_bits: u32) -> u32 {
  if !do_hash_spreading || size <= (1u32 << hash_bits) {
    return hash & (size - 1);
  }
  let coefficient = size >> hash_bits;
  (hash.wrapping_mul(coefficient)) & (size - 1)
}

/// entry → 槽下标 (hash-table.h:223 EntryToIndex)。
#[inline]
#[must_use]
pub const fn entry_to_index(entry: u32, entry_size: usize, elements_start_index: usize) -> usize {
  (entry as usize) * entry_size + elements_start_index
}

/// 槽下标 → entry (hash-table.h:228 IndexToEntry)。
#[inline]
#[must_use]
pub const fn index_to_entry(index: usize, entry_size: usize, elements_start_index: usize) -> u32 {
  ((index - elements_start_index) / entry_size) as u32
}

// ── Shape 规则面 (hash-table.h 各 Shape 展开) ────────────────────────
/// Shape 静态参数 (C++ Shape 类的关联常量集)。
pub struct ShapeParams {
  /// 前缀槽数 (kPrefixSize)。
  pub prefix_size: usize,
  /// 每 entry 槽数 (kEntrySize)。
  pub entry_size: usize,
  /// IsMatch 是否需处理 the_hole 删除槽 (kMatchNeedsHoleCheck)。
  pub match_needs_hole_check: bool,
  /// 大表哈希分散开关 (kDoHashSpreading)。
  pub do_hash_spreading: bool,
  /// 哈希可用位数 (kHashBits; 0 = 不分散)。
  pub hash_bits: u32,
}

/// ObjectHashTableShapeBase (键 = 任意对象, 身份哈希)。
pub const OBJECT_HASH_TABLE_SHAPE_BASE: ShapeParams = ShapeParams {
  prefix_size: 0,
  entry_size: 2,
  match_needs_hole_check: false,
  do_hash_spreading: false,
  hash_bits: 0,
};
/// ObjectHashTableShape。
pub const OBJECT_HASH_TABLE_SHAPE: ShapeParams = OBJECT_HASH_TABLE_SHAPE_BASE;
/// EphemeronHashTableShape (WeakMap 大表分散探测; kHashBits=10 定义于 objects.cc)。
pub const EPHEMERON_HASH_TABLE_SHAPE: ShapeParams = ShapeParams {
  do_hash_spreading: true,
  hash_bits: ${kEphemeronHashBits},
  ..OBJECT_HASH_TABLE_SHAPE_BASE
};
/// ObjectHashSetShape (集合: 每 entry 1 槽)。
pub const OBJECT_HASH_SET_SHAPE: ShapeParams = ShapeParams {
  entry_size: 1,
  ..OBJECT_HASH_TABLE_SHAPE_BASE
};
/// NameToIndexShape (Name → 索引)。
pub const NAME_TO_INDEX_SHAPE: ShapeParams = OBJECT_HASH_TABLE_SHAPE_BASE;
/// RegisteredSymbolTableShape (描述符字符串 → Symbol)。
pub const REGISTERED_SYMBOL_TABLE_SHAPE: ShapeParams = OBJECT_HASH_TABLE_SHAPE_BASE;
/// ObjectMultiHashTableShape<N>: kEntrySize = 1 + N (每键 N 值, N>1)。
#[inline]
#[must_use]
pub const fn object_multi_hash_table_entry_size(n: usize) -> usize {
  1 + n
}
/// ObjectTwoHashTable (N = 2)。
pub const OBJECT_TWO_HASH_TABLE_ENTRY_SIZE: usize = object_multi_hash_table_entry_size(2);

/// kElementsStartIndex = kPrefixStartIndex + Shape::kPrefixSize (hash-table.h:201)。
#[inline]
#[must_use]
pub const fn elements_start_index(prefix_size: usize) -> usize {
  K_PREFIX_START_INDEX + prefix_size
}

/// 最大容量 = bit_floor((FixedArray::kMaxLength - kElementsStartIndex) / kEntrySize)
/// (hash-table.h:211; 保证 EntryToIndex 不溢出)。
#[inline]
#[must_use]
pub const fn max_capacity(prefix_size: usize, entry_size: usize) -> u32 {
  ((K_FIXED_ARRAY_MAX_LENGTH - elements_start_index(prefix_size) as u32) / entry_size as u32)
    .next_power_of_two()
    >> 1
}

// ── 堆访问方法签名骨架 (就地修订; 参数类型为 Address/索引占位) ──────────
${skeletonFns.map(([fn, sig, doc]) => `/// ${doc}。
pub fn ${fn}${sig} { todo!("就地修订") }`).join("\n\n")}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_NUMBER_OF_ELEMENTS_INDEX == 0 && K_CAPACITY_INDEX == 2 && K_PREFIX_START_INDEX == 3);
  assert!(K_MIN_CAPACITY == 4 && K_MIN_SHRINK_CAPACITY == 16 && K_MIN_CAPACITY_FOR_PRETENURE == 256);
  assert!(K_MAX_REGULAR_CAPACITY == 4096);
  assert!(compute_capacity(1) == 4 && compute_capacity(3) == 8 && compute_capacity(100) == 256);
  assert!(next_probe(0, 1, 8) == 1 && next_probe(7, 3, 8) == 2);
  assert!(first_probe(7, 8, false, 0) == 7);
  // 分散分支: size > 2^10 且 kHashBits=10 → coefficient = size >> 10
  assert!(first_probe(7, 2048, true, 10) == (7 * 2) & 2047);
  assert!(OBJECT_HASH_SET_SHAPE.entry_size == 1 && EPHEMERON_HASH_TABLE_SHAPE.hash_bits == 10);
  assert!(OBJECT_TWO_HASH_TABLE_ENTRY_SIZE == 3);
  assert!(elements_start_index(0) == 3 && elements_start_index(3) == 6);
};
`;

writeSrc("v8-objects-collections", "hash_table.rs", body);
console.log(
  `hash_table.rs: 槽位常量 5 + 容量常量 4 + const fn 7 + Shape ${parsed.length} + 骨架 ${skeletonFns.length}, 对数通过`,
);
