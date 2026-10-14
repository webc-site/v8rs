#!/usr/bin/env bun
// tool/src/heap/marking.js
// 机械提取 v8/src/heap/marking{.h,.cc,-inl.h} 族:
// MarkBit / MarkingBitmap 常量与纯函数 / MarkingHelper 枚举

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/marking.h");
const ccSrc = readV8("src/heap/marking.cc");
const inlSrc = readV8("src/heap/marking-inl.h");

// —— 锚点断言: 源结构漂移即抛 ——

const anchors = [
  [hSrc, "using CellType = uintptr_t;"],
  [hSrc, "kBitsPerCell = sizeof(CellType) * kBitsPerByte"],
  [hSrc, "kBitIndexMask = kBitsPerCell - 1"],
  [hSrc, "kBytesPerCell = kBitsPerCell / kBitsPerByte"],
  [hSrc, "kLength = ((1 << kPageSizeBits) >> kTaggedSizeLog2)"],
  [hSrc, "kCellsCount ="],
  [hSrc, "kSize = kCellsCount * kBytesPerCell"],
  [hSrc, "return index >> kBitsPerCellLog2;"],
  [hSrc, "return index << kTaggedSizeLog2;"],
  [hSrc, "return index & kBitIndexMask;"],
  [hSrc, "return index & ~kBitIndexMask;"],
  [hSrc, "struct MarkingHelper final : public AllStatic"],
  [inlSrc, "return MemoryChunk::AddressToOffset(address) >> kTaggedSizeLog2;"],
  [inlSrc, "if (MemoryChunk::IsAligned(address)) return kLength;"],
  [ccSrc, "bool MarkingBitmap::AllBitsSetInRange("],
  [ccSrc, "bool MarkingBitmap::IsClean() const {"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// —— 枚举提取 (MarkingHelper) ——

function enumBody(src, name) {
  const m = src.match(new RegExp(`enum class ${name}[^\\{]*\\{([^}]*)\\};`, "s"));
  if (!m) throw new Error(`未找到枚举 ${name}`);
  return m[1].replace(/\/\/[^\n]*/g, "").split(",").map((s) => s.trim()).filter(Boolean);
}
const worklistTarget = enumBody(hSrc, "WorklistTarget");
const livenessMode = enumBody(hSrc, "LivenessMode");
if (worklistTarget.join() !== "kRegular") throw new Error("WorklistTarget 漂移");
if (livenessMode.join() !== "kMarkbit,kAlwaysLive") throw new Error("LivenessMode 漂移");

// MarkingBitmap.cc 实现方法计数 (对数用)
const ccMethods = ["AllBitsSetInRange", "AllBitsClearInRange", "Print", "IsClean"];
for (const m of ccMethods) if (!ccSrc.includes(`MarkingBitmap::${m}(`)) throw new Error(`cc 缺 ${m}`);
// inl 模板实例计数
const inlTpl = ["SetBitsInCell", "ClearBitsInCell", "ClearCellRangeRelaxed", "SetCellRangeRelaxed", "SetRange", "ClearRange"];
for (const m of inlTpl) if (!inlSrc.includes(`MarkingBitmap::${m}`)) throw new Error(`inl 缺 ${m}`);

console.log(`marking: 枚举 2 (${worklistTarget.length}/${livenessMode.length}), cc 方法 ${ccMethods.length}, inl 模板 ${inlTpl.length}`);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
function emitEnum(rustName, doc, entries) {
  let out = `/// ${doc}\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub enum ${rustName} {\n`;
  entries.forEach((k, i) => (out += `  /// ${k}\n  ${snake(k)} = ${i},\n`));
  return out + "}\n\n";
}

let rust = banner(
  "src/heap/marking.js",
  "src/heap/marking.h, src/heap/marking.cc, src/heap/marking-inl.h",
  "MarkBit/MarkingBitmap 位图布局与 MarkingHelper 枚举",
);

rust += `//! 标记位图: 每页一张, CellType = uintptr_t (64 位平台 8 字节)。
//! kPageSizeBits 默认 18 (见 v8/src/base/build_config.h:80), 指针压缩 kTaggedSizeLog2 = 2。

/// v8: src/heap/marking.h MarkBit::CellType
pub type MarkBitCell = usize;

/// v8: src/heap/marking.h MarkBit
/// 单个标记位 = (cell 指针, 掩码); Set 返回是否 0→1。
pub struct MarkBit {
  cell: *mut MarkBitCell,
  mask: MarkBitCell,
}

/// v8: src/heap/marking.h MarkingBitmap
#[allow(clippy::upper_case_acronyms)]
pub mod marking_bitmap {
  /// v8: MarkingBitmap::kBitsPerCell = sizeof(CellType) * 8
  pub const K_BITS_PER_CELL: u32 = 64;
  /// v8: MarkingBitmap::kBitsPerCellLog2 = CountTrailingZeros(64)
  pub const K_BITS_PER_CELL_LOG2: u32 = 6;
  /// v8: MarkingBitmap::kBitIndexMask = kBitsPerCell - 1
  pub const K_BIT_INDEX_MASK: u32 = 63;
  /// v8: MarkingBitmap::kBytesPerCell = kBitsPerCell / 8
  pub const K_BYTES_PER_CELL: u32 = 8;
  /// v8: MarkingBitmap::kBytesPerCellLog2 = kBitsPerCellLog2 - 3
  pub const K_BYTES_PER_CELL_LOG2: u32 = 3;
  /// v8: MarkingBitmap::kLength = (1 << kPageSizeBits) >> kTaggedSizeLog2  (页内标记位总数)
  /// kPageSizeBits=18 且指针压缩 → 131072; 非压缩 → 65536。
  pub const K_LENGTH: usize = (1 << K_PAGE_SIZE_BITS) >> K_TAGGED_SIZE_LOG2;
  /// v8: MarkingBitmap::kCellsCount = (kLength + kBitsPerCell - 1) >> kBitsPerCellLog2
  pub const K_CELLS_COUNT: usize = (K_LENGTH + K_BITS_PER_CELL as usize - 1) >> K_BITS_PER_CELL_LOG2;
  /// v8: MarkingBitmap::kSize = kCellsCount * kBytesPerCell
  pub const K_SIZE: usize = K_CELLS_COUNT * K_BYTES_PER_CELL as usize;

  /// v8: src/heap/marking.h MarkingBitmap::IndexToCell
  pub const fn index_to_cell(index: u32) -> u32 { index >> K_BITS_PER_CELL_LOG2 }
  /// v8: src/heap/marking.h MarkingBitmap::IndexToAddressOffset
  pub const fn index_to_address_offset(index: u32) -> usize { (index as usize) << K_TAGGED_SIZE_LOG2 }
  /// v8: src/heap/marking.h MarkingBitmap::CellToBase
  pub const fn cell_to_base(cell_index: u32) -> usize { index_to_address_offset(cell_index << K_BITS_PER_CELL_LOG2) }
  /// v8: src/heap/marking.h MarkingBitmap::IndexInCell
  pub const fn index_in_cell(index: u32) -> u32 { index & K_BIT_INDEX_MASK }
  /// v8: src/heap/marking.h MarkingBitmap::IndexInCellMask
  pub const fn index_in_cell_mask(index: u32) -> MarkBitCell { 1usize << index_in_cell(index) }
  /// v8: src/heap/marking.h MarkingBitmap::CellAlignIndex
  pub const fn cell_align_index(index: u32) -> u32 { index & !K_BIT_INDEX_MASK }
  /// v8: src/heap/marking-inl.h MarkingBitmap::AddressToIndex
  /// = MemoryChunk::AddressToOffset(address) >> kTaggedSizeLog2
  pub const fn address_to_index(address: usize) -> u32 {
    ((address & K_PAGE_ALIGNMENT_MASK) >> K_TAGGED_SIZE_LOG2) as u32
  }
  /// v8: src/heap/marking-inl.h MarkingBitmap::LimitAddressToIndex
  /// 原文: 页对齐 → kLength, 否则 AddressToIndex(address)
  pub const fn limit_address_to_index(address: usize) -> u32 {
    if address & K_PAGE_ALIGNMENT_MASK == 0 { K_LENGTH as u32 } else { address_to_index(address) }
  }
}

// —— cc / inl 语义面骨架 (语义体留锚) ——

/// v8: src/heap/marking.cc MarkingBitmap::AllBitsSetInRange
/// v8: src/heap/marking.cc MarkingBitmap::AllBitsClearInRange
/// v8: src/heap/marking.cc MarkingBitmap::IsClean
/// v8: src/heap/marking-inl.h MarkingBitmap::SetRange / ClearRange / SetBitsInCell / ClearBitsInCell
/// v8: src/heap/marking-inl.h MarkingBitmap::FindPreviousValidObject

`;

rust += emitEnum("WorklistTarget", "标记入队目标 (MarkingHelper::WorklistTarget, 原版仅 kRegular)", worklistTarget);
rust += emitEnum("LivenessMode", "存活判定方式 (MarkingHelper::LivenessMode)", livenessMode);

rust += `
/// v8: src/heap/marking.h MarkingHelper::ShouldMarkObject
/// v8: src/heap/marking.h MarkingHelper::IsUnmarked
/// v8: src/heap/marking-inl.h MarkingHelper (实现面)

// —— 提取不变式断言 ——
const _: () = {
  assert!(marking_bitmap::K_BITS_PER_CELL == 64);
  assert!(marking_bitmap::K_BIT_INDEX_MASK == 63);
  assert!(marking_bitmap::K_BYTES_PER_CELL_LOG2 == 3);
  assert!(LivenessMode::ALWAYS_LIVE == 1);
  assert!(marking_bitmap::index_to_cell(64) == 1);
  assert!(marking_bitmap::index_in_cell(70) == 6);
};
`;

const p = writeSrc("v8-heap-gc", "marking.rs.skel", rust);
console.log(`written ${p}`);
