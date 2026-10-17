#!/usr/bin/env bun
// tool/src/objects/ordered-hash-table.tq.js — 覆盖 v8/src/objects/ordered-hash-table.tq
// → v8-objects-collections/src/small_ordered_hash_table_generated.rs (现有
// small_ordered_hash_table.rs 为他人手写语义面, 冲突让位 _generated 后缀)
// 机械提取: SmallOrderedHashSet/Map/NameDictionary 类布局字段序与表维度 +
//   tq generates 常量 (取值 oracle = clang -E ordered-hash-table.h)。
// 配置判定: 目标配置指针压缩 (kTaggedSize=4) → TAGGED_SIZE_8_BYTES 为假,
//   padding 取 @ifnot 分支 (padding[1] / padding_0: void)。

import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { root, readV8, writeSrc, banner } from "../../lib.js";

const TQ = "src/objects/ordered-hash-table.tq";
const H = "src/objects/ordered-hash-table.h";
const tq = readV8(TQ);
const hSrc = readV8(H);

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
// generates 常量对数 (SmallOrderedHashTable C++ 侧取值)
must(hOracle.includes("static const int kMaxCapacity = 254;"), "oracle kMaxCapacity=254");
must(hOracle.includes("static const uint8_t kNotFound = 0xFF;"), "oracle kNotFound=0xFF");
must(hOracle.includes("static const int kLoadFactor = 2;"), "oracle kLoadFactor=2");
// 目标配置 tagged 4 字节 (决定 padding 分支)
const pp = spawnSync("clang", [...ORACLE_ARGS, join(root, "v8", H)], {
  encoding: "utf8",
  maxBuffer: 1 << 26,
}).stdout;
must(/constexpr int kTaggedSize = kInt32Size;/.test(pp), "目标配置 kTaggedSize 须为 4 (→ TAGGED_SIZE_8_BYTES 为假)");

// —— tq 解析: 3 个 @cppObjectLayoutDefinition 类的字段序 ——
function tqClass(name) {
  const re = new RegExp(`extern class ${name} extends [^\\{]+\\{([\\s\\S]*?)\\n\\}`, "m");
  const m = tq.match(re);
  if (!m) throw new Error(`tq 缺类 ${name}`);
  return m[1];
}
// 字段行: name: type; / name[N]: type; / @if/@ifnot 行 (维度跨行时先拼接)
function fieldsOf(body) {
  const out = [];
  const joined = [];
  for (let rawLine of body.split("\n")) {
    const l = rawLine.trim();
    if (l.endsWith(":") && !l.endsWith("};")) {
      joined.push(l);
      continue;
    }
    joined.push(joined.length > 0 && joined.at(-1).endsWith(":") ? `${joined.pop()} ${l}` : l);
  }
  for (const l of joined) {
    const fm = l.match(/^(@ifnot\(TAGGED_SIZE_8_BYTES\)|@if\(TAGGED_SIZE_8_BYTES\))\s*(padding\w*)(?:\[(\d+)\])?:\s*(\w+);?$/);
    if (fm) {
      out.push({ name: fm[2], arr: fm[3] ? +fm[3] : null, type: fm[4], branch: fm[1] });
      continue;
    }
    const m = l.match(/^((?:const\s+)?)([a-z_][\w]*)(?:\[([^\]]+)\])?:\s*([^;]+);$/);
    if (m && !l.startsWith("@") && !l.startsWith("macro") && !l.startsWith("dcheck")) {
      out.push({ name: m[2], dim: m[3] ?? null, type: m[4].trim(), const: m[1] !== "" });
    }
  }
  return out;
}
// 目标配置 4 字节 tagged: 剔除 @if(TAGGED_SIZE_8_BYTES) 分支字段, 保留 @ifnot
function selectTargetBranch(fields, expectDropped) {
  const kept = [];
  let dropped = 0;
  for (const f of fields) {
    if (f.branch === "@if(TAGGED_SIZE_8_BYTES)") {
      dropped++;
      continue;
    }
    kept.push(f);
  }
  if (dropped !== expectDropped) throw new Error(`@if(TAGGED_SIZE_8_BYTES) 分支数 ${dropped}, 期望 ${expectDropped}`);
  return kept;
}
const setFields = selectTargetBranch(fieldsOf(tqClass("SmallOrderedHashSet")), 1);
const mapFields = selectTargetBranch(fieldsOf(tqClass("SmallOrderedHashMap")), 1);
const dictFields = selectTargetBranch(fieldsOf(tqClass("SmallOrderedNameDictionary")), 2);
must(setFields.length === 7, `SmallOrderedHashSet 字段数 ${setFields.length}, 期望 7`);
must(mapFields.length === 7, `SmallOrderedHashMap 字段数 ${mapFields.length}, 期望 7`);
must(dictFields.length === 9, `SmallOrderedNameDictionary 字段数 ${dictFields.length}, 期望 9 (剔除 8 字节 tagged 分支后)`);
// 头部字段序核对 (机械: 前 4 字段名)
must(
  setFields.slice(0, 4).map((f) => f.name).join(",") ===
    "number_of_elements,number_of_deleted_elements,number_of_buckets,padding",
  "SmallOrderedHashSet 头部字段序",
);
must(
  dictFields.slice(0, 5).map((f) => f.name).join(",") ===
    "hash,padding_0,number_of_elements,number_of_deleted_elements,number_of_buckets",
  "SmallOrderedNameDictionary 头部字段序",
);
// padding 分支: 目标配置取 @ifnot
must(setFields[3].branch === "@ifnot(TAGGED_SIZE_8_BYTES)" && setFields[3].arr === 1, "Set padding 须取 @ifnot[1]");
must(dictFields[1].branch === "@ifnot(TAGGED_SIZE_8_BYTES)" && dictFields[1].type === "void", "Dict padding_0 须取 @ifnot void");
must(dictFields[5].branch === "@ifnot(TAGGED_SIZE_8_BYTES)" && dictFields[5].arr === 1, "Dict padding_1 须取 @ifnot[1]");
// 表维度: data_table = buckets * kLoadFactor; hash_table = buckets;
//   chain_table: Set/Map = buckets * kLoadFactor, NameDictionary = buckets
const dtSet = setFields.find((f) => f.name === "data_table");
must(dtSet.dim === "Convert<intptr>(number_of_buckets) * kSmallOrderedHashTableLoadFactor", "Set data_table 维度");
const ctSet = setFields.find((f) => f.name === "chain_table");
must(ctSet.dim === "Convert<intptr>(number_of_buckets) * kSmallOrderedHashTableLoadFactor", "Set chain_table 维度");
const ctDict = dictFields.find((f) => f.name === "chain_table");
must(ctDict.dim === "number_of_buckets", "NameDictionary chain_table 维度 (无装载因子乘子)");
must(tq.includes("struct HashMapEntry {"), "HashMapEntry 结构锚");
must(tq.includes("struct NameDictionaryEntry {"), "NameDictionaryEntry 结构锚");

// —— 产物 ——
const FIELD_LIST = (fs) =>
  fs
    .map((f) => {
      const dim = f.arr !== undefined && f.arr !== null ? `[${f.arr}]` : f.dim ? "[dyn]" : "";
      const branch = f.branch ? ` (${f.branch.startsWith("@ifnot") ? "非 8 字节 tagged" : "8 字节 tagged"}分支)` : "";
      return `  ("${f.name}", "${f.type}${dim}"),${branch ? ` // ${branch}` : ""}`;
    })
    .join("\n");

let rust = banner("src/objects/ordered-hash-table.tq.js", TQ, "SmallOrdered* tq 布局字段序与表维度 (目标配置: 指针压缩)");
rust += `//! SmallOrderedHashTable 家族 Torque 布局 (ordered-hash-table.tq)。
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
${FIELD_LIST(setFields)}
];

/// SmallOrderedHashMap 字段序 (4 字节 tagged 分支)。
pub const SMALL_ORDERED_HASH_MAP_FIELDS: &[(&str, &str)] = &[
${FIELD_LIST(mapFields)}
];

/// SmallOrderedNameDictionary 字段序 (4 字节 tagged 分支; hash 前缀 + padding_0)。
pub const SMALL_ORDERED_NAME_DICTIONARY_FIELDS: &[(&str, &str)] = &[
${FIELD_LIST(dictFields)}
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
`;

const p = writeSrc("v8-objects-collections", "small_ordered_hash_table_generated.rs", rust);
console.log(
  `[ordered-hash-table.tq.js] ${p}: 3 类布局 ${setFields.length}/${mapFields.length}/${dictFields.length} 字段, generates 3 常量 clang oracle 对数 (254/0xFF/2), padding 分支 @ifnot`,
);
