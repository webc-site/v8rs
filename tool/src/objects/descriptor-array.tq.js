#!/usr/bin/env bun
// tool/src/objects/descriptor-array.tq.js — 覆盖 v8/src/objects/descriptor-array.tq (单文件族)
// → v8-objects-maps/src/descriptor_array_tq.rs
// 机械提取: EnumCache / DescriptorEntry / DescriptorArrayFlags / DescriptorArray tq 布局
// 对数 oracle: clang -E 预处理 v8/src/objects/descriptor-array.h
//   (C++ 成员声明序 + OFFSET_OF_DATA_START 布局锚 1:1; 压缩指针 K_TAGGED_SIZE=4)

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { readV8, writeSrc, banner, root } from "../../lib.js";

const tq = readV8("src/objects/descriptor-array.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── clang -E 对数基准 (原版侧 = 编译器预处理产物; 缺失头自动空 stub) ──────
const ppShim = join(tmpdir(), "v8rs-r3o4-pp-shim");
function clangE(rel) {
  const args = [
    "-E", "-P", "-x", "c++", "-std=c++20", "-I", "v8", "-I", "v8/include",
    "-I", ppShim, "-DV8_INTL_SUPPORT=1", "-DV8_ENABLE_WEBASSEMBLY=1", "-",
  ];
  for (let round = 0; round < 48; round++) {
    try {
      return execFileSync("clang", args, {
        input: `#include "${rel}"\n`,
        encoding: "utf8",
        cwd: root,
        maxBuffer: 512 * 1024 * 1024,
      });
    } catch (e) {
      const err = String(e.stderr ?? "");
      const m = err.match(/fatal error: '([^']+)' file not found/);
      if (!m) throw new Error(`clang -E ${rel} 失败: ${err.slice(0, 300)}`);
      const stub = join(ppShim, m[1]);
      mkdirSync(dirname(stub), { recursive: true });
      writeFileSync(stub, "#pragma once\n");
    }
  }
  throw new Error(`clang -E ${rel}: stub 轮数超限`);
}

// ── 产物 fmt 归一 (与 cargo +nightly fmt 同配置; 幂等) ────────────────────
function fmtNormalize(p) {
  const rustfmt = execFileSync("rustup", ["which", "--toolchain", "nightly", "rustfmt"],
    { encoding: "utf8", cwd: root }).trim();
  execFileSync(rustfmt, ["--edition", "2024", "--config-path", join(root, "rustfmt.toml"), p],
    { cwd: root });
}


const hpp = clangE("src/objects/descriptor-array.h");

// ── 1. EnumCache (tq 2 字段 ↔ pp TaggedMember 序 1:1) ─────────────────────
const enumCacheTq = tq.match(/extern class EnumCache extends Struct \{([\s\S]*?)\n\}/);
if (!enumCacheTq) throw new Error("未找到 tq EnumCache");
const enumCacheFields = [...enumCacheTq[1].matchAll(/^ {2}(\w+): ([^;]+);$/gm)]
  .map((m) => [m[1], m[2].trim()]);
must(JSON.stringify(enumCacheFields) === JSON.stringify([["keys", "FixedArray"], ["indices", "FixedArray"]]),
  `EnumCache 字段失配: ${JSON.stringify(enumCacheFields)}`);
const ppEnumCache = hpp.slice(hpp.indexOf("class EnumCache : public Struct"));
const ppEnumCacheMembers = [...ppEnumCache.slice(0, ppEnumCache.indexOf("V8_OBJECT_END"))
  .matchAll(/TaggedMember<FixedArray> (\w+);/g)].map((m) => m[1]);
must(ppEnumCacheMembers.join(",") === "keys_,indices_", `pp EnumCache 成员失配: ${ppEnumCacheMembers}`);

// ── 2. DescriptorEntry (tq @export struct, 3 字段) ────────────────────────
const entryTq = tq.match(/@export\nstruct DescriptorEntry \{([\s\S]*?)\n\}/);
if (!entryTq) throw new Error("未找到 tq DescriptorEntry");
const entryFields = [...entryTq[1].matchAll(/^ {2}(\w+): ([^;]+);$/gm)]
  .map((m) => [m[1], m[2].replace(/\s+/g, " ").trim()]);
must(entryFields.length === 3, `DescriptorEntry 字段 ${entryFields.length} != 3`);
must(entryFields[0][0] === "key" && entryFields[1][0] === "details" && entryFields[2][0] === "value",
  "DescriptorEntry 字段序失配");
must(entryFields[0][1] === "Name|Undefined", `key 类型失配: ${entryFields[0][1]}`);
// pp 侧 Entry 联合类型锚
must(hpp.includes("TaggedMember<UnionOf<Name, Undefined>> key;"), "pp Entry key 锚缺失");
must(hpp.includes("TaggedMember<UnionOf<Smi, Undefined>> details;"), "pp Entry details 锚缺失");
must(hpp.includes("TaggedMember<UnionOf<JSAny, Weak<Map>, AccessorInfo, AccessorPair,"), "pp Entry value 锚缺失");

// ── 3. DescriptorArrayFlags (fast_iterable: 2 bit, uint32) ────────────────
const flagsTq = tq.match(/bitfield struct DescriptorArrayFlags extends uint32 \{([\s\S]*?)\n\}/);
if (!flagsTq) throw new Error("未找到 tq DescriptorArrayFlags");
const flagBits = [...flagsTq[1].matchAll(/^ {2}(\w+): ([\w<>']+): (\d+) bit;$/gm)]
  .map((m) => ({ name: m[1], type: m[2], len: parseInt(m[3], 10) }));
must(flagBits.length === 1 && flagBits[0].name === "fast_iterable" && flagBits[0].len === 2,
  `DescriptorArrayFlags 失配: ${JSON.stringify(flagBits)}`);
// FastIterableState 2 位双态锚 (kJsonFast/kJsonSlow/kUnknown 见 descriptor-array.h)
must(tq.includes("type FastIterableState extends uint8"), "缺 FastIterableState 类型别名");

// ── 4. DescriptorArray 主体 (头字段序 ↔ pp 成员序 1:1) ────────────────────
const daTq = tq.match(/extern class DescriptorArray extends HeapObject \{([\s\S]*?)\n\}/);
if (!daTq) throw new Error("未找到 tq DescriptorArray");
const daFields = [...daTq[1].matchAll(/^ {2}(?:const )?(\w+)(?:\[([^\]]+)\])?: ([^;]+);$/gm)]
  .map((m) => ({ name: m[1], extent: m[2] ?? "", type: m[3].replace(/\s+/g, " ").trim() }));
must(daFields.length === 5, `DescriptorArray 字段 ${daFields.length} != 5`);
must(daFields[0].name === "number_of_all_descriptors" && daFields[0].type === "uint16", "首字段失配");
must(daFields[1].name === "number_of_descriptors" && daFields[1].type === "uint16", "次字段失配");
must(daFields[2].name === "flags" && daFields[2].type === "DescriptorArrayFlags", "flags 位次失配");
must(daFields[3].name === "enum_cache" && daFields[3].type === "EnumCache", "enum_cache 位次失配");
must(daFields[4].name === "descriptors" && daFields[4].extent === "number_of_all_descriptors", "尾数组失配");
// pp 成员序锚 (std::atomic 包装 + FLEXIBLE_ARRAY_MEMBER 展开的尾数组)
for (const a of [
  "std::atomic<uint16_t> number_of_all_descriptors_;",
  "std::atomic<uint16_t> number_of_descriptors_;",
  "std::atomic<uint32_t> flags_",
  "TaggedMember<EnumCache> enum_cache_;",
  "Entry flexible_array_member_data_[0];",
]) must(hpp.includes(a), `pp 缺成员锚: ${a}`);

// ── 5. 压缩配置布局推导 (K_TAGGED_SIZE=4) ─────────────────────────────────
const T = 4;
// HeapObject 头 = 1 map word; 头字段: u16@4, u16@6, u32@8, tagged@12 → descriptors@16
const K_NUMBER_OF_ALL_OFFSET = T;
const K_NUMBER_OF_OFFSET = T + 2;
const K_FLAGS_OFFSET = T + 4;
const K_ENUM_CACHE_OFFSET = T + 8;
const K_DESCRIPTORS_OFFSET = T + 12;
must(K_DESCRIPTORS_OFFSET === 16, "布局推导失配");
must(hpp.includes("static_assert(IsAligned(DescriptorArray::kHeaderSize, kTaggedSize));"),
  "pp 缺 kHeaderSize 对齐断言");
must(hpp.includes(
  "OffsetOfDataStart<DescriptorArray>() +\n         number_of_all_descriptors * kEntrySize * kTaggedSize;"),
  "pp 缺 SizeFor 公式锚");

let rust = banner("src/objects/descriptor-array.tq.js", "src/objects/descriptor-array.tq",
  "DescriptorArray tq 布局 (clang -E descriptor-array.h 对数, 压缩配置)");
rust += `//! DescriptorArray 的 Torque 侧布局镜像: EnumCache / DescriptorEntry /
//! DescriptorArrayFlags / 头部字段偏移 (压缩指针 K_TAGGED_SIZE=4)。

/// tq extern class EnumCache 字段表 (顺序 1:1)。
pub const ENUM_CACHE_FIELDS: [(&str, &str); ${enumCacheFields.length}] = [
${enumCacheFields.map(([n, t]) => `  ("${n}", "${t}"),`).join("\n")}
];

/// tq @export struct DescriptorEntry 字段表 (顺序 1:1; 尾随变长数组元素类型)。
pub const DESCRIPTOR_ENTRY_FIELDS: [(&str, &str); ${entryFields.length}] = [
${entryFields.map(([n, t]) => `  ("${n}", "${t.replace(/\s*\|\s*/g, " | ")}"),`).join("\n")}
];

/// DescriptorArrayFlags: fast_iterable 位段 (2 bit @0, 容器 uint32)。
pub const FAST_ITERABLE_SHIFT: u32 = 0;
/// fast_iterable 掩码。
pub const FAST_ITERABLE_MASK: u32 = 0x3;

// ── DescriptorArray 头部布局 (压缩 K_TAGGED_SIZE=4) ──────────────────────
/// number_of_all_descriptors 偏移 (u16)。
pub const K_NUMBER_OF_ALL_DESCRIPTORS_OFFSET: usize = ${K_NUMBER_OF_ALL_OFFSET};
/// number_of_descriptors 偏移 (u16)。
pub const K_NUMBER_OF_DESCRIPTORS_OFFSET: usize = ${K_NUMBER_OF_OFFSET};
/// flags 偏移 (u32)。
pub const K_FLAGS_OFFSET: usize = ${K_FLAGS_OFFSET};
/// enum_cache 偏移 (tagged)。
pub const K_ENUM_CACHE_OFFSET: usize = ${K_ENUM_CACHE_OFFSET};
/// descriptors 数组起点 = 原版 kHeaderSize / kDescriptorsOffset。
pub const K_DESCRIPTORS_OFFSET: usize = ${K_DESCRIPTORS_OFFSET};

/// tq extern class DescriptorArray 头字段表 (顺序 1:1, 含尾随变长数组声明)。
pub const DESCRIPTOR_ARRAY_FIELDS: [(&str, &str); ${daFields.length}] = [
${daFields.map((f) => `  ("${f.name}${f.extent ? `[${f.extent}]` : ""}", "${f.type}"),`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(ENUM_CACHE_FIELDS.len() == 2 && DESCRIPTOR_ENTRY_FIELDS.len() == 3);
  assert!(DESCRIPTOR_ARRAY_FIELDS.len() == 5);
  assert!((FAST_ITERABLE_MASK == 0x3) && (FAST_ITERABLE_SHIFT == 0));
  assert!(K_DESCRIPTORS_OFFSET == 16);
  assert!((K_NUMBER_OF_DESCRIPTORS_OFFSET == 6) && (K_FLAGS_OFFSET == 8) && (K_ENUM_CACHE_OFFSET == 12));
};
`;

const p = writeSrc("v8-objects-maps", "descriptor_array_tq.rs", rust);
fmtNormalize(p);
console.log(`descriptor_array_tq.rs: EnumCache ${enumCacheFields.length} + Entry ${entryFields.length} + 主体 ${daFields.length} 字段 (pp 序 1:1), 头部偏移 ${K_DESCRIPTORS_OFFSET}, 断言 ${checks} 项通过 → ${p}`);
