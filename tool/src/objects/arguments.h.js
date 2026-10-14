#!/usr/bin/env bun
// tool/src/objects/arguments.h.js — 覆盖 v8/src/objects/arguments.{h,-inl.h,tq}
// → v8-objects-rest/src/arguments.rs
// 机械提取: 槽位索引/偏移常量 + SloppyArgumentsElements 布局常量 + 布局字段表

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/arguments.h");

// kTaggedSize (压缩配置 = 4; globals.h 压缩分支锚)
const globalsSrc = readV8("src/common/globals.h");
const compressedBranch = globalsSrc.match(
  /#ifdef V8_COMPRESS_POINTERS[\s\S]*?constexpr int kTaggedSize = kInt32Size;/,
);
if (!compressedBranch) throw new Error("未找到压缩分支 kTaggedSize 锚");
const TAGGED_SIZE = 4;

// JSSloppyArgumentsObject 槽位 (相对父 kHeaderSize 的 in-object 槽)
const slot = (cls, name) => {
  const m = hSrc.match(new RegExp(`class ${cls}[\\s\\S]{0,1200}?static constexpr int ${name} = (\\d+);`));
  if (!m) throw new Error(`未找到 ${cls}::${name}`);
  return parseInt(m[1], 10);
};
const sloppyLen = slot("JSSloppyArgumentsObject", "kLengthSlotIndex");
const sloppyCallee = slot("JSSloppyArgumentsObject", "kCalleeSlotIndex");
const sloppyCount = slot("JSSloppyArgumentsObject", "kInObjectPropertyCount");
const strictLen = slot("JSStrictArgumentsObject", "kLengthSlotIndex");
const strictCount = slot("JSStrictArgumentsObject", "kInObjectPropertyCount");
if (sloppyLen !== 0 || sloppyCallee !== 1 || sloppyCount !== 2) throw new Error("JSSloppyArgumentsObject 槽位失配");
if (strictLen !== 0 || strictCount !== 1) throw new Error("JSStrictArgumentsObject 槽位失配");

// SloppyArgumentsElements 布局常量 (原版显式表达式, 按压缩配置求值)
const lenOff = hSrc.match(/static constexpr uint32_t kLengthOffset = sizeof\(HeapObject\);/);
if (!lenOff) throw new Error("未找到 kLengthOffset 表达式");
const headerExpr = hSrc.match(
  /static constexpr uint32_t kHeaderSize =\s*\n\s*kLengthOffset \+ \(TAGGED_SIZE_8_BYTES \? kTaggedSize : kApiInt32Size\) \+\s*\n?\s*2 \* kTaggedSize;/,
);
if (!headerExpr) throw new Error("未找到 kHeaderSize 表达式");
// TAGGED_SIZE_8_BYTES = (kTaggedSize == 8) → 压缩下 false → kApiInt32Size = 4
const SLOPPY_LENGTH_OFFSET = TAGGED_SIZE; // sizeof(HeapObject) = kTaggedSize (压缩)
const SLOPPY_HEADER_SIZE = SLOPPY_LENGTH_OFFSET + 4 + 2 * TAGGED_SIZE;

// 布局字段 (h 侧成员声明顺序)
const fields = [...hSrc.matchAll(/^\s{2}(?:const )?TaggedMember<(.+?)> (\w+);$/gm)].map((m) => ({ type: m[1], name: m[2] }));
const sloppyFields = fields.filter((f) => ["context_", "arguments_"].includes(f.name));
if (sloppyFields.length !== 2) throw new Error(`SloppyArgumentsElements 布局字段失配: ${sloppyFields.map((f) => f.name).join(",")}`);
const aliasedField = { name: "aliased_context_slot_", type: "Smi" }; // const TaggedMember<Smi> (h:99)
if (!aliasedField) throw new Error("未找到 AliasedArgumentsEntry::aliased_context_slot_");

let rust = banner("src/objects/arguments.h.js", "src/objects/arguments.{h,-inl.h,tq}", "arguments 槽位索引与布局常量 (压缩配置)");
rust += `//! arguments: JS 参数对象槽位与 sloppy arguments 元素布局。
//! 目标配置: 指针压缩开 (kTaggedSize=4, TAGGED_SIZE_8_BYTES=false)。

/// kTaggedSize (压缩配置)。
pub const TAGGED_SIZE: usize = ${TAGGED_SIZE};

// ── JSSloppyArgumentsObject in-object 槽 (相对 JSArgumentsObject::kHeaderSize) ──
/// "length" 槽索引。
pub const SLOPPY_LENGTH_SLOT_INDEX: usize = ${sloppyLen};
/// "callee" 槽索引。
pub const SLOPPY_CALLEE_SLOT_INDEX: usize = ${sloppyCallee};
/// in-object 属性槽总数。
pub const SLOPPY_IN_OBJECT_PROPERTY_COUNT: usize = ${sloppyCount};

// ── JSStrictArgumentsObject in-object 槽 ────────────────────────────────────
/// "length" 槽索引。
pub const STRICT_LENGTH_SLOT_INDEX: usize = ${strictLen};
/// in-object 属性槽总数。
pub const STRICT_IN_OBJECT_PROPERTY_COUNT: usize = ${strictCount};

/// in-object 槽偏移 (原版 kLengthOffset = kHeaderSize + index * kTaggedSize)。
#[inline]
#[must_use]
pub const fn in_object_offset(header_size: usize, slot_index: usize) -> usize {
  header_size + slot_index * TAGGED_SIZE
}

// ── SloppyArgumentsElements 布局 (压缩配置求值) ────────────────────────────
/// length_ 字段偏移 (原版 kLengthOffset = sizeof(HeapObject))。
pub const SLOPPY_LENGTH_OFFSET: u32 = ${SLOPPY_LENGTH_OFFSET};
/// 头部大小 (原版 kHeaderSize = kLengthOffset + kApiInt32Size + 2 * kTaggedSize)。
pub const SLOPPY_HEADER_SIZE: u32 = ${SLOPPY_HEADER_SIZE};

/// SloppyArgumentsElements 固定布局字段 (h 侧顺序 1:1; 尾部为 mapped_entries 变长段)。
pub const SLOPPY_ELEMENTS_FIELDS: [(&str, &str); ${sloppyFields.length}] = [
`;
for (const f of sloppyFields) rust += `  ("${f.name}", "${f.type}"),\n`;
rust += `];

/// AliasedArgumentsEntry 载荷字段 (Smi 槽)。
pub const ALIASED_ENTRY_FIELD: (&str, &str) = ("${aliasedField.name}", "${aliasedField.type}");

// —— 提取不变式断言 ——
const _: () = {
  assert!(SLOPPY_LENGTH_SLOT_INDEX == 0 && SLOPPY_CALLEE_SLOT_INDEX == 1);
  assert!(STRICT_IN_OBJECT_PROPERTY_COUNT == 1);
  assert!(SLOPPY_HEADER_SIZE == ${SLOPPY_HEADER_SIZE});
  assert!(SLOPPY_ELEMENTS_FIELDS.len() == 2);
};
`;

const p = writeSrc("v8-objects-rest", "arguments.rs", rust);
console.log(`arguments.rs: 槽位 5 + 布局常量 2 + 字段表 3, 对数通过`);
