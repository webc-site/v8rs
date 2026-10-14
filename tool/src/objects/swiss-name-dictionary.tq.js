#!/usr/bin/env bun
// tool/src/objects/swiss-name-dictionary.tq.js — 覆盖 v8/src/objects/swiss-name-dictionary.tq
// → v8-objects-collections/src/swiss_name_dictionary_tq_generated.rs
//   (现有 swiss_name_dictionary.rs / swiss_hash_table_helpers.rs 为他人模块, 冲突让位)
// 机械提取: SwissNameDictionary tq 类布局字段序 + namespace swiss_table generates 常量
//   + tq @export 容量/尺寸纯函数 (capacity_for / max_usable_capacity / size_for /
//   meta_table_size_per_entry_for / meta_table_size_for / ctrl_table_start_offset)。
// 对数 oracle: clang -E swiss-name-dictionary.h/-inl.h (C++ 侧同名静态函数取值公式),
//   禁止用脚本自身解析结果对数。

import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { root, readV8, writeSrc, banner } from "../../lib.js";

const TQ = "src/objects/swiss-name-dictionary.tq";
const H = "src/objects/swiss-name-dictionary.h";
const INL = "src/objects/swiss-name-dictionary-inl.h";
const tq = readV8(TQ);
const hSrc = readV8(H);
const inlSrc = readV8(INL);

function must(cond, msg) {
  if (!cond) throw new Error(`对数失败: ${msg}`);
}

// —— clang oracle (目标配置: 指针压缩 + 沙箱) ——
const ORACLE_ARGS = [
  "-E",
  "-std=c++20",
  "-I" + join(root, "v8"),
  "-I" + join(root, "v8", "include"),
  "-DV8_COMPRESS_POINTERS",
  "-DV8_ENABLE_SANDBOX",
  "-x",
  "c++",
];
function clangMeat(rel) {
  const abs = join(root, "v8", rel);
  const r = spawnSync("clang", [...ORACLE_ARGS, abs], { encoding: "utf8", maxBuffer: 1 << 26 });
  if (!r.stdout || r.stdout.length < 1000) throw new Error(`clang -E 无正文: ${rel}`);
  const esc = abs.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const marks = [...r.stdout.matchAll(new RegExp(`^# (\\d+) "${esc}"`, "gm"))].map((m) => +m[1]);
  const total = readV8(rel).split("\n").length;
  if (Math.max(...marks, 0) < total - 2) throw new Error(`clang -E 正文不完整: ${rel}`);
  const chunks = [];
  let cur = null;
  for (const l of r.stdout.split("\n")) {
    if (l.startsWith("# ")) {
      cur = new RegExp(`^# \\d+ "${esc}"`).test(l) ? [] : null;
      if (cur) chunks.push(cur);
      continue;
    }
    if (cur) cur.push(l);
  }
  return chunks.map((c) => c.join("\n")).join("\n");
}

const hOracle = clangMeat(H);
const inlOracle = clangMeat(INL);

// —— 1. raw 锚点 ——
must(tq.includes("extern class SwissNameDictionary extends HeapObject {"), "tq 类声明锚");
must(tq.includes("namespace swiss_table {"), "tq namespace 锚");
must(tq.includes("macro SwissNameDictionaryCapacityFor(atLeastSpaceFor: intptr): intptr {"), "tq CapacityFor 锚");
must(tq.includes("macro SwissNameDictionaryMaxUsableCapacity(capacity: intptr): intptr {"), "tq MaxUsableCapacity 锚");

// —— 2. clang oracle 对数: .h 布局常量 + C++ 侧同名公式 ——
const ORACLE_H = [
  ["static constexpr int kNotFoundSentinel = -1;", 1],
  ["static constexpr uint32_t kDataTableEntryCount = 2;", 1],
  ["static constexpr uint32_t kMetaTableElementCountFieldIndex = 0;", 1],
  ["static constexpr uint32_t kMetaTableDeletedElementCountFieldIndex = 1;", 1],
  ["static constexpr uint32_t kMetaTableEnumerationDataStartIndex = 2;", 1],
  ["static constexpr uint32_t kMax1ByteMetaTableCapacity = (1 << 8);", 1],
  ["static constexpr uint32_t kMax2ByteMetaTableCapacity = (1 << 16);", 1],
];
for (const [pat, n] of ORACLE_H) {
  const c = hOracle.split(pat).length - 1;
  must(c === n, `swiss .h oracle "${pat}" 出现 ${c}, 期望 ${n}`);
}
// -inl.h C++ 侧公式 (tq @export macro 的 C++ 对应)
must(inlOracle.includes("uint32_t non_normalized = at_least_space_for + at_least_space_for / 7;"), "inl oracle CapacityFor +1/7 公式");
must(inlOracle.includes("return std::bit_ceil<uint32_t>(non_normalized);"), "inl oracle bit_ceil 取 2 的幂");
must(inlOracle.includes("return capacity - capacity / 8;"), "inl oracle MaxUsableCapacity cap - cap/8");
must(inlOracle.includes("return capacity * kTaggedSize * kDataTableEntryCount;"), "inl oracle DataTableSize");
must(inlOracle.includes("return (capacity + kGroupWidth) * kOneByteSize;"), "inl oracle CtrlTableSize");
must(inlOracle.includes("return PrefixOffset() + sizeof(uint32_t);"), "inl oracle CapacityOffset");
must(inlOracle.includes("return MetaTablePointerOffset() + kTaggedSize;"), "inl oracle DataTableStartOffset");
must(inlOracle.includes("return DataTableStartOffset() + DataTableSize(capacity);"), "inl oracle CtrlTableStartOffset");
// meta per entry 1/2/4 分支 (tq 与 C++ 同构; 定义签名跨行, 锚限定定义处调用形)
must(inlOracle.includes("SwissNameDictionary::MetaTableSizePerEntryFor("), "inl oracle MetaTableSizePerEntryFor 定义");

// —— 3. tq 解析: 类布局字段序 + generates 常量名 ——
const classM = tq.match(/extern class SwissNameDictionary extends HeapObject \{([\s\S]*?)\n\}/);
must(classM, "tq SwissNameDictionary 布局体");
const fieldLines = classM[1]
  .split("\n")
  .map((l) => l.trim())
  .filter((l) => /^[a-z_]\w*\[?/.test(l) && l.includes(":"));
const fields = fieldLines.map((l) => {
  const m = l.match(/^(?:const\s+)?([a-z_]\w*)(?:\[([^\]]+)\])?:\s*([^;]+);$/);
  if (!m) throw new Error(`tq 字段行无法解析: ${l}`);
  return { name: m[1], dim: m[2] ?? null, type: m[3].trim() };
});
must(fields.length === 6, `SwissNameDictionary 布局字段数 ${fields.length}, 期望 6`);
must(
  fields.map((f) => f.name).join(",") === "hash,capacity,meta_table,data_table,ctrl_table,property_details_table",
  "SwissNameDictionary 字段序",
);
must(fields[0].type === "uint32" && fields[1].type === "uint32", "hash/capacity 须 uint32");
must(fields[2].type === "ByteArray", "meta_table 须 ByteArray");
must(fields[3].dim === "Convert<intptr>(capacity) * 2", "data_table 维度 = capacity * 2");
must(fields[4].dim === "Convert<intptr>(capacity) + swiss_table::kGroupWidth", "ctrl_table 维度 = capacity + kGroupWidth");
must(fields[5].dim === "Convert<intptr>(capacity)", "property_details_table 维度 = capacity");

// generates 常量 (namespace swiss_table 内)
const gen = [...tq.matchAll(/const (\w+):[\s\S]{0,80}?generates '([^']+)';/g)].map((m) => [m[1], m[2]]);
must(gen.length === 5, `tq generates 常量数 ${gen.length}, 期望 5`);
const genNames = gen.map((g) => g[0]);
for (const n of [
  "kDataTableEntryCount",
  "kMax1ByteMetaTableCapacity",
  "kMax2ByteMetaTableCapacity",
  "kNotFoundSentinel",
  "kDataTableStartOffsetMT",
]) {
  must(genNames.includes(n), `tq generates 缺 ${n}`);
}
// offset 常量取值公式 (DataTableStartOffset - tag, minus tag 语义)
must(
  gen.find((g) => g[0] === "kDataTableStartOffsetMT")[1] ===
    "SwissNameDictionary::DataTableStartOffset() - kHeapObjectTag",
  "kDataTableStartOffsetMT generates 公式",
);

// —— 4. 纯函数语义核对 (tq 公式 JS 重算) ——
const roundUpPow2 = (x) => {
  let v = 1;
  while (v < x) v <<= 1;
  return v;
};
const capacityFor = (n, groupWidth = 16) => {
  if (n <= 4) {
    if (n === 0) return 0;
    if (n < 4) return 4;
    if (groupWidth === 16) return 4;
    return 8;
  }
  return roundUpPow2(n + Math.floor(n / 7));
};
for (const [n, exp] of [[0, 0], [1, 4], [3, 4], [4, 4], [5, 8], [7, 8], [8, 16], [9, 16], [100, 128]]) {
  must(capacityFor(n) === exp, `capacity_for(${n})=${capacityFor(n)}, 期望 ${exp}`);
}
must(capacityFor(1024) === 2048, `capacity_for(1024)=${capacityFor(1024)} (1024+146=1170 → 2048)`);
const maxUsable = (cap, groupWidth = 16) => (groupWidth === 8 && cap === 4 ? 3 : cap - Math.floor(cap / 8));
for (const [c, exp] of [[0, 0], [4, 4], [8, 7], [16, 14], [32, 28], [256, 224]]) {
  must(maxUsable(c) === exp, `max_usable_capacity(${c})=${maxUsable(c)}, 期望 ${exp}`);
}
const metaPerEntry = (cap) => (cap <= 256 ? 1 : cap <= 65536 ? 2 : 4);
must(metaPerEntry(256) === 1 && metaPerEntry(257) === 2 && metaPerEntry(65536) === 2 && metaPerEntry(65537) === 4, "meta per entry 分支");
const metaSize = (cap) => (2 + maxUsable(cap)) * metaPerEntry(cap);
must(metaSize(4) === 6 && metaSize(16) === 16 && metaSize(256) === 226, "meta_table_size_for 抽样");

// —— 5. 产物 ——
let rust = banner("src/objects/swiss-name-dictionary.tq.js", TQ, "SwissNameDictionary tq 布局/容量与元表纯函数 (SIMD 16 宽)");
rust += `//! SwissNameDictionary Torque 面 (swiss-name-dictionary.tq)。
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
${fields.map((f) => {
  const dim = f.dim
    ? "[capacity" + (f.name === "data_table" ? " * 2" : f.name === "ctrl_table" ? " + kGroupWidth" : "") + "]"
    : "";
  return `  ("${f.name}", "${f.type}${dim}"),`;
}).join("\n")}
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
`;

const p = writeSrc("v8-objects-collections", "swiss_name_dictionary_tq_generated.rs", rust);
console.log(
  `[swiss-name-dictionary.tq.js] ${p}: 布局 ${fields.length} 字段, generates ${gen.length} 常量, oracle .h ${ORACLE_H.length} 组 + inl 8 公式, 纯函数抽样 ${[0, 3, 9, 1024].map((n) => capacityFor(n)).join("/")}`,
);
