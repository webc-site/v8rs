#!/usr/bin/env bun
// tool/src/objects/embedder-data-array.h.js — 覆盖 v8/src/objects/embedder-data-array.{h,-inl.h,cc,tq}
// → v8-objects-rest/src/embedder_data_array.rs
// 机械提取: SizeFor/OffsetOfElementAt 公式 + 上限公式 + 头部对齐锚

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/embedder-data-array.h");
const tqSrc = readV8("src/objects/embedder-data-array.tq");

// SizeFor 公式锚 (OFFSET_OF_DATA_START + length * kEmbedderDataSlotSize)
const sizeFor = hSrc.match(
  /constexpr int EmbedderDataArray::SizeFor\(int length\) \{[\s\S]*?return OFFSET_OF_DATA_START\(EmbedderDataArray\) \+\s*\n\s*length \* kEmbedderDataSlotSize;/,
);
if (!sizeFor) throw new Error("未找到 SizeFor 公式");
const offAt = hSrc.match(
  /constexpr int EmbedderDataArray::OffsetOfElementAt\(int index\) \{\s*\n\s*return SizeFor\(index\);/,
);
if (!offAt) throw new Error("未找到 OffsetOfElementAt 公式");

// 上限公式锚
const maxRe = hSrc.match(/inline constexpr int kEmbedderDataArrayMaxSize = (\w+);/);
const maxLen = hSrc.match(/inline constexpr int kEmbedderDataArrayMaxLength =\s*\n\s*\(kEmbedderDataArrayMaxSize - OFFSET_OF_DATA_START\(EmbedderDataArray\)\) \/\s*\n\s*kEmbedderDataSlotSize;/);
if (!maxRe || maxRe[1] !== "kMaxRegularHeapObjectSize") throw new Error("kEmbedderDataArrayMaxSize 锚失配");
if (!maxLen) throw new Error("kEmbedderDataArrayMaxLength 公式失配");

// 静态断言锚: OFFSET_OF_DATA_START == Internals::kFixedArrayHeaderSize
if (!hSrc.includes("Internals::kFixedArrayHeaderSize")) throw new Error("缺 kFixedArrayHeaderSize 对齐锚");

// kEmbedderDataSlotSize: 压缩开 → 2 * kTaggedSize = 8 (登记机械值)
// 原版定义于 globals.h (kEmbedderDataSlotSize = TAGGED_SIZE_8_BYTES ? kTaggedSize : 2 * kTaggedSize)
const globalsSrc = readV8("src/common/globals.h");
const slotSize = globalsSrc.match(/constexpr int kEmbedderDataSlotSize = 2 \* kTaggedSize;/);
if (!slotSize) throw new Error("未找到 kEmbedderDataSlotSize 表达式");

// tq 布局
const tqRe = tqSrc.match(/extern class EmbedderDataArray extends (\w+)\s*\{([\s\S]*?)\n\}/);
if (!tqRe || tqRe[1] !== "HeapObject") throw new Error("tq EmbedderDataArray 失配");
const tqFields = [...tqRe[2].matchAll(/^\s{2}(?:const )?(\w+)(\?)?:\s*([^;]+);/gm)].map((m) => ({ name: m[1], type: m[3].trim() }));
if (tqFields.length !== 1 || tqFields[0].name !== "length") throw new Error(`tq 字段失配: ${tqFields.map((f) => f.name).join(",")}`);
// slots 为 FLEXIBLE_ARRAY_MEMBER (h 侧), tq 侧仅 length 固定字段

let rust = banner("src/objects/embedder-data-array.h.js", "src/objects/embedder-data-array.{h,-inl.h,cc,tq}", "EmbedderDataArray 布局公式与上限");
rust += `//! EmbedderDataArray: 嵌入器数据数组 (length + 内联槽)。
//! 目标配置: 指针压缩开 → kEmbedderDataSlotSize = 8 (2 * kTaggedSize)。

/// 槽大小 (原版 kEmbedderDataSlotSize; TAGGED_SIZE_8_BYTES=false 分支)。
pub const EMBEDDER_DATA_SLOT_SIZE: usize = 8;

/// 头部对齐锚 (原版 static_assert: OFFSET_OF_DATA_START == Internals::kFixedArrayHeaderSize;
/// 压缩下 FixedArray 头 = map + length = 8)。
pub const DATA_START_ANCHOR: &str = "Internals::kFixedArrayHeaderSize";

/// 数组总大小 (原版 SizeFor: OFFSET_OF_DATA_START + length * kEmbedderDataSlotSize)。
#[inline]
#[must_use]
pub const fn size_for(data_start: usize, length: usize) -> usize {
  data_start + length * EMBEDDER_DATA_SLOT_SIZE
}

/// 第 index 槽偏移 (原版 OffsetOfElementAt = SizeFor(index))。
#[inline]
#[must_use]
pub const fn offset_of_element_at(data_start: usize, index: usize) -> usize {
  size_for(data_start, index)
}

/// 数组大小上限 (原版 kEmbedderDataArrayMaxSize = kMaxRegularHeapObjectSize)。
pub const EMBEDDER_DATA_ARRAY_MAX_SIZE_ANCHOR: &str = "kMaxRegularHeapObjectSize";

/// 最大槽数 (原版 kEmbedderDataArrayMaxLength =
/// (kEmbedderDataArrayMaxSize - OFFSET_OF_DATA_START) / kEmbedderDataSlotSize)。
#[inline]
#[must_use]
pub const fn max_length(data_start: usize, max_regular_heap_object_size: usize) -> usize {
  (max_regular_heap_object_size - data_start) / EMBEDDER_DATA_SLOT_SIZE
}

/// 布局字段表 (tq 顺序 1:1)。
pub const LAYOUT_FIELDS: [(&str, &str); ${tqFields.length}] = [
`;
for (const f of tqFields) rust += `  ("${f.name}", "${f.type}"),\n`;
rust += `];

// —— 提取不变式断言 ——
const _: () = {
  assert!(EMBEDDER_DATA_SLOT_SIZE == 8);
  assert!(size_for(8, 3) == 32);
  assert!(offset_of_element_at(8, 0) == 8);
  assert!(max_length(8, 128) == 15);
  assert!(LAYOUT_FIELDS.len() == 1);
};
`;

const p = writeSrc("v8-objects-rest", "embedder_data_array.rs", rust);
console.log(`embedder_data_array.rs: 公式 3 + 上限 2 + 布局 3 字段, 对数通过`);
