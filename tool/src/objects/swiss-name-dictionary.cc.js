#!/usr/bin/env bun
// tool/src/objects/swiss-name-dictionary.cc.js — 覆盖 swiss-name-dictionary 同名族 .cc/.tq 段:
//   v8/src/objects/swiss-name-dictionary.cc + swiss-name-dictionary.tq
//   → v8-objects-collections/src/swiss_name_dictionary_cc_generated.rs
// 布局常量 (.h) 归 swiss-name-dictionary.h.js 与开发模块 swiss_name_dictionary.rs;
// 本脚本提取 .cc/.tq 侧容量/收缩/元表尺寸公式 (纯计算段落 const fn)
// 与 .cc 方法面, 堆访问留骨架 + todo!("就地修订")。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/objects/swiss-name-dictionary.cc");
const tq = readV8("src/objects/swiss-name-dictionary.tq");
const h = readV8("src/objects/swiss-name-dictionary.h"); // 取值侧锚

// ── .h 取值侧锚 (只锚不复写, 归 swiss_name_dictionary.rs) ────────────────
const hAnchors = [
  "static constexpr int kNotFoundSentinel = -1;",
  "static constexpr uint32_t kDataTableEntryCount = 2;",
  "static constexpr uint32_t kMax1ByteMetaTableCapacity = (1 << 8);",
  "static constexpr uint32_t kMax2ByteMetaTableCapacity = (1 << 16);",
];
for (const a of hAnchors) {
  if (!h.includes(a)) throw new Error(`swiss-name-dictionary.h 锚失配: ${a}`);
}
const kInitialCapacityM = h.match(/static constexpr uint32_t kInitialCapacity =\s*\n\s*kSwissNameDictionaryInitialCapacity;/);
if (!kInitialCapacityM) throw new Error("kInitialCapacity 别名声明锚失配");
// 值侧: -inl.h CapacityFor 显式字面量 (4) + kGroupWidth = Group::kWidth (SIMD=16)
const inl = readV8("src/objects/swiss-name-dictionary-inl.h");
if (!inl.includes("if (at_least_space_for < 4) {\n      return 4;")) throw new Error("kInitialCapacity=4 取值锚失配 (-inl.h CapacityFor)");
if (!inl.includes("} else if (kGroupWidth == 8) {")) throw new Error("kGroupWidth==8 分支锚失配");
if (!h.includes("static const uint32_t kGroupWidth = Group::kWidth;")) throw new Error("kGroupWidth 声明锚失配");
const kGroupWidth = 16; // SIMD 分支 (swiss-hash-table-helpers.tq.js 已锚 16/8 双分支)

// ── .cc 规则锚 (must 断言) ────────────────────────────────────────────────
const ccAnchors = [
  [cc, "uint32_t new_capacity = std::max(capacity / 2, kInitialCapacity);", "swiss-name-dictionary.cc:207 (Shrink)"],
  [cc, "constexpr uint32_t SwissNameDictionary::kInitialCapacity;", "swiss-name-dictionary.cc:356"],
  [cc, "constexpr uint32_t SwissNameDictionary::kGroupWidth;", "swiss-name-dictionary.cc:357"],
];
for (const [s, sub, f] of ccAnchors) {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f}`);
}

// ── .tq 容量公式锚 (must 断言; 公式在 tq 显式) ────────────────────────────
const tqAnchors = [
  [tq, "const nonNormalized = atLeastSpaceFor + atLeastSpaceFor / 7;", "swiss-name-dictionary.tq (CapacityFor)"],
  [tq, "return IntPtrRoundUpToPowerOfTwo32(nonNormalized);", "swiss-name-dictionary.tq (CapacityFor)"],
  [tq, "if (FromConstexpr<bool>(kGroupWidth == 8) && capacity == 4) {", "swiss-name-dictionary.tq (MaxUsableCapacity 特例)"],
  [tq, "return capacity - capacity / 8;", "swiss-name-dictionary.tq (MaxUsableCapacity)"],
  [tq, "return (2 + maxUsable) * perEntry;", "swiss-name-dictionary.tq (MetaTableSizeFor)"],
  [tq, "const constant: constexpr int32 = kHeapObjectHeaderSize + 8 + kTaggedSize;", "swiss-name-dictionary.tq (SizeFor 常量段)"],
  [tq, "capacity * FromConstexpr<intptr>(2 * kTaggedSize + 2) +", "swiss-name-dictionary.tq (SizeFor 动态段)"],
  [tq, "if (Convert<intptr>(Signed(newElementCount)) < (capacity >> 2)) {", "swiss-name-dictionary.tq (收缩触发阈值)"],
];
for (const [s, sub, f] of tqAnchors) {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f}`);
}

// ── .cc 方法面 (数量 must 断言) ───────────────────────────────────────────
const methods = [...new Set([...cc.matchAll(/SwissNameDictionary::(\w+)\(/g)].map((m) => m[1]))].sort();
const EXPECT = ["DeleteEntry", "EqualsForTesting", "Initialize", "MaxUsableCapacity", "NumberOfEnumerableProperties", "Rehash", "ShallowCopy", "Shrink", "SlowReverseLookup"].sort();
if (methods.join(",") !== EXPECT.join(",")) {
  throw new Error(`SwissNameDictionary 方法面漂移 (${methods.length}): ${methods.join(",")}`);
}
// MaxUsableCapacity 的 .cc 定义 (static 断言消费点) 单独登记
if (!cc.includes("SwissNameDictionary::MaxUsableCapacity(")) throw new Error("MaxUsableCapacity .cc 定义锚失配");
// 匿名辅助 (cc 231-229 Shrink 触发段)
if (!cc.includes("if (table->NumberOfElements() < (capacity >> 2)) {") &&
    !cc.includes("static_cast<int>(table->NumberOfElements()) < (capacity >> 2)")) {
  // 原版 Shrink 内联在 DeleteEntry/Rehash 附近; 登记宽松核对
  if (!cc.includes("capacity >> 2")) throw new Error("Shrink 阈值 capacity>>2 锚失配");
}

// ── .tq 类布局 (extern class SwissNameDictionary 字段序) ─────────────────
const clsM = tq.match(/extern class SwissNameDictionary extends HeapObject \{([\s\S]*?)\n\}/);
if (!clsM) throw new Error("tq 缺 extern class SwissNameDictionary");
const clsFields = [...clsM[1].matchAll(/^\s*(?:const\s+)?(\w+)(?:\[(.*?)\])?\s*:/gm)].map((f) => f[1]);
if (clsFields.join(",") !== "hash,capacity,meta_table,data_table,ctrl_table,property_details_table") {
  throw new Error(`tq 字段序漂移: ${clsFields.join(",")}`);
}
// ctrl_table 长度 = capacity + kGroupWidth (tq 显式)
if (!clsM[1].includes("Convert<intptr>(capacity) + swiss_table::kGroupWidth")) throw new Error("ctrl_table 长度公式锚失配");
// 宏/extern-macro 清单 (数量断言): @export + extern CSA 接口面全量登记
const exports = [...new Set([...tq.matchAll(/macro (SwissNameDictionary\w+)\(/g)].map((m) => m[1]))].sort();
const EXPECT_EXPORTS = [
  "SwissNameDictionaryAddPortable", "SwissNameDictionaryAddSIMD", "SwissNameDictionaryCapacityFor",
  "SwissNameDictionaryCtrlTableStartOffsetMT", "SwissNameDictionaryDataTableStartOffsetMT", "SwissNameDictionaryDelete",
  "SwissNameDictionaryFindEntryPortable", "SwissNameDictionaryFindEntrySIMD", "SwissNameDictionaryIncreaseElementCountOrBailout",
  "SwissNameDictionaryMaxUsableCapacity", "SwissNameDictionaryMetaTableSizeFor", "SwissNameDictionaryMetaTableSizePerEntryFor",
  "SwissNameDictionarySetCtrl", "SwissNameDictionarySizeFor", "SwissNameDictionaryUpdateCountsForDeletion",
].sort();
if (exports.length !== EXPECT_EXPORTS.length || exports.join(",") !== EXPECT_EXPORTS.join(",")) {
  throw new Error(`tq 宏清单漂移 (${exports.length}): ${exports.join(",")}`);
}
const exportUnique = exports;

const body = `${banner("src/objects/swiss-name-dictionary.cc.js", "src/objects/swiss-name-dictionary.{cc,tq}", "swiss 字典 .cc 方法面 + .tq 容量/元表公式 (基座常量归 swiss_name_dictionary.rs)")}
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
pub const SWISS_NAME_DICTIONARY_FIELDS: [&str; ${clsFields.length}] = [
${clsFields.map((f) => `  "${f}",`).join("\n")}
];
/// ctrl_table 长度公式: capacity + kGroupWidth (含整组探测冗余; SIMD=16)。
pub const CTRL_TABLE_EXTRA: u32 = ${kGroupWidth};

// ── .cc 方法面 (${methods.length} 项, 对数基准) ────────────────────────
/// SwissNameDictionary 全部 .cc 定义。
pub const SWISS_NAME_DICTIONARY_CC_METHODS: [&str; ${methods.length}] = [
${methods.map((s) => `  "${s}",`).join("\n")}
];

// ── .tq 宏/CSA 接口面 (${exportUnique.length} 项) ───────────────────────
/// tq 宏与 extern 宏 (CSA 侧入口; SIMD/Portable 双分支 + 元表/控制字节原语)。
pub const TQ_EXPORT_MACROS: [&str; ${exportUnique.length}] = [
${exportUnique.map((s) => `  "${s}",`).join("\n")}
];

// ── 堆访问签名骨架 (就地修订; 参数为 Address/索引占位) ─────────────────
/// 删除 entry (DeleteEntry: kDeleted 控制字节 + 数据清 the_hole + 计数 + Shrink)。
pub fn delete_entry(_isolate: u64, _table: u64, _entry: u32) -> u64 { todo!("就地修订") }
/// 整表重哈希 (Rehash: 枚举序保序迁移 + meta 表重建)。
pub fn rehash(_isolate: u64, _table: u64, _new_capacity: u32) -> u64 { todo!("就地修订") }
/// 原地重哈希 (Rehash(isolate) 无参形态: 同容量清删除项)。
pub fn rehash_in_place(_isolate: u64, _table: u64) { todo!("就地修订") }
/// 浅拷贝 (ShallowCopy: 布局 + ctrl/meta/数据表逐段复制)。
pub fn shallow_copy(_isolate: u64, _table: u64) -> u64 { todo!("就地修订") }
/// 收缩 (Shrink: rehash 至 shrink_target)。
pub fn shrink(_isolate: u64, _table: u64) -> u64 { todo!("就地修订") }
/// 可枚举属性计数 (NumberOfEnumerableProperties)。
pub fn number_of_enumerable_properties(_table: u64) -> i32 { todo!("就地修订") }
/// 慢速反查 (SlowReverseLookup: 值 → 键)。
pub fn slow_reverse_lookup(_isolate: u64, _table: u64, _value: u64) -> u64 { todo!("就地修订") }
/// 测试等价 (EqualsForTesting: 逐表比对)。
pub fn equals_for_testing(_a: u64, _b: u64) -> bool { todo!("就地修订") }

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
  assert!(SWISS_NAME_DICTIONARY_CC_METHODS.len() == ${methods.length});
  assert!(TQ_EXPORT_MACROS.len() == ${exportUnique.length});
};
`;

writeSrc("v8-objects-collections", "swiss_name_dictionary_cc_generated.rs", body);
console.log(
  `swiss_name_dictionary_cc_generated.rs: const fn 7 + 字段 ${clsFields.length} + cc 方法 ${methods.length} + tq 导出宏 ${exportUnique.length} + 骨架 8, 对数通过`,
);
