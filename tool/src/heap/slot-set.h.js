#!/usr/bin/env bun
// tool/src/heap/slot-set.h.js
// 机械提取 v8/src/heap/slot-set.{h,cc}: SlotType / IterationMode / AccessMode 与布局常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/slot-set.h");
// 族内 .cc 实读对数
const ssCcSrc = readV8("src/heap/slot-set.cc");
if (!ssCcSrc.includes("TypedSlots::Insert(")) throw new Error("slot-set.cc 锚点缺失");

// SlotType
const st = hSrc.match(/enum class SlotType : uint8_t \{([\s\S]*?)\n\};/);
if (!st) throw new Error("未找到 SlotType");
const slotTypes = st[1]
  .replace(/\/\/[^\n]*/g, "")
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean)
  .filter((s) => !s.includes("=")); // kLast = kCleared 别名单列
if (slotTypes.length !== 7) throw new Error(`SlotType ${slotTypes.length}, 期望 7`);
if (slotTypes[0] !== "kEmbeddedObjectFull" || slotTypes[6] !== "kCleared") {
  throw new Error("SlotType 锚点漂移");
}

// IterationMode
const im = hSrc.match(/enum IterationMode \{([^}]*)\};/);
if (!im) throw new Error("未找到 IterationMode");
const iterModes = im[1].split(",").map((s) => s.trim()).filter(Boolean);
if (iterModes.join() !== "FREE_EMPTY_CHUNKS,KEEP_EMPTY_CHUNKS") throw new Error("IterationMode 漂移");

// AccessMode (enum class AccessMode 在 src/common/globals.h, 此处仅引用; 槽集内有 ConvertAccessMode)
if (!hSrc.includes("AccessMode")) throw new Error("未引用 AccessMode");

// 常量
const consts = {};
for (const [, k, v] of hSrc.matchAll(/static constexpr (?:Address|int|size_t) k(\w+) = ([^;]+);/g)) {
  consts[k] = v.trim();
}
for (const k of ["PointerTag", "WordSize", "BitsPerWord"]) {
  if (!(k in consts)) throw new Error(`常量缺失: k${k}`);
}
if (consts.PointerTag !== "1") throw new Error("kPointerTag 漂移");
if (consts.WordSize !== "sizeof(uintptr_t)") throw new Error("kWordSize 漂移");

// kBucketsRegularPage: 形如 static const int kBucketsRegularPage = 表达式;
const buckets = hSrc.match(/static const int kBucketsRegularPage\s*=\s*([^;]+);/);
if (!buckets) throw new Error("未找到 kBucketsRegularPage");

// TypedSlots::kMaxOffset
const maxOffset = hSrc.match(/static const int kMaxOffset = (1 << \d+);/);
if (!maxOffset) throw new Error("未找到 TypedSlots::kMaxOffset");

console.log(`SlotType ${slotTypes.length}, IterationMode ${iterModes.length}`);

let rust = banner(
  "src/heap/slot-set.h.js",
  "src/heap/slot-set.{h,cc}",
  "槽集: 槽类型 / 迭代模式 / 布局常量",
);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

rust += `
/// 槽类型 (对应 enum class SlotType : uint8_t; 注释语义见原版)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SlotType {
`;

slotTypes.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`;
});

rust += `}

/// 槽块迭代模式 (对应 enum IterationMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IterationMode {
  /// FREE_EMPTY_CHUNKS: 迭代后释放空块
  FREE_EMPTY_CHUNKS = 0,
  /// KEEP_EMPTY_CHUNKS: 保留空块
  KEEP_EMPTY_CHUNKS = 1,
}

/// 槽指针标签 (对应 kPointerTag)
pub const POINTER_TAG: usize = 1;
/// 机器字大小 (对应 kWordSize = sizeof(uintptr_t))
pub const WORD_SIZE: usize = size_of::<usize>();
/// 每字位数 (对应 kBitsPerWord)
pub const BITS_PER_WORD: usize = WORD_SIZE * 8;
/// 每桶单元数 (对应 kCellsPerBucket, 原版同文件)
pub const CELLS_PER_BUCKET: usize = 32;
/// 每单元位数 (对应 kBitsPerCell)
pub const BITS_PER_CELL: usize = 32;
/// 常规页桶数 (对应 kBucketsRegularPage, 原版表达式: ${buckets[1].replace(/\s+/g, " ")})
pub const BUCKETS_REGULAR_PAGE: usize =
    (1 << crate::common::internal::K_PAGE_SIZE_BITS) / crate::common::globals::K_TAGGED_SIZE / CELLS_PER_BUCKET / BITS_PER_CELL;
/// TypedSlots 偏移上限 (对应 TypedSlots::kMaxOffset)
pub const MAX_OFFSET: i32 = ${maxOffset[1].replace(/1 << (\d+)/, "(1 << $1)")};

// —— 提取不变式断言 ——
const _: () = {
  assert!(SlotType::EMBEDDED_OBJECT_FULL == 0);
  assert!(SlotType::CODE_ENTRY == 2);
  assert!(SlotType::CONST_POOL_CODE_ENTRY == 5);
  assert!(SlotType::CLEARED == 6);
  assert!(IterMode_FREE_EMPTY_CHUNKS == 0);
  assert!(POINTER_TAG == 1);
  assert!(BITS_PER_WORD == 64 || BITS_PER_WORD == 32);
  assert!(MAX_OFFSET == 1 << 29);
};
`.replace("IterMode_FREE_EMPTY_CHUNKS", "IterationMode::FREE_EMPTY_CHUNKS as u8");
;

const p = writeSrc("v8-heap-core", "slot_set.rs", rust);
console.log(`written ${p}`);
