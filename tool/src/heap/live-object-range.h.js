#!/usr/bin/env bun
// tool/src/heap/live-object-range.h.js
// 机械提取 v8/src/heap/live-object-range.{h,-inl.h}:
// 标记位图活对象迭代器状态机骨架 (cell 扫描 + trailing_zeros 定位 + filler 跳过)。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/live-object-range.h");
const inl = readV8("src/heap/live-object-range-inl.h");

for (const a of [
  "class LiveObjectRange final {",
  "class iterator final {",
  "using value_type = std::pair<Tagged<HeapObject>, SafeHeapObjectSize>;",
  "inline bool AdvanceToNextMarkedObject();",
  "inline void AdvanceToNextValidObject();",
  "MarkingBitmap::CellIndex current_cell_index_ = 0;",
  "MarkingBitmap::CellType current_cell_ = 0;",
  "Tagged<HeapObject> current_object_;",
  "Tagged<Map> current_map_;",
  "uint32_t current_size_ = 0;",
  "inline iterator begin();",
  "inline iterator end();",
]) {
  if (!h.includes(a)) throw new Error(`live-object-range.h 锚点缺失: ${a}`);
}
for (const a of [
  "LiveObjectRange::iterator::iterator() : cage_base_(kNullAddress) {}",
  "LiveObjectRange::iterator::iterator(const NormalPage* page)",
  "while (AdvanceToNextMarkedObject() &&",
  "InstanceTypeChecker::IsFreeSpaceOrFiller(current_map_)) {",
  "bool LiveObjectRange::iterator::AdvanceToNextMarkedObject() {",
  "Address next_object = current_object_.address() + current_size_;",
  "if (MemoryChunk::IsAligned(next_object)) {",
  "const MarkBit::CellType mask =",
  "MarkingBitmap::IndexInCellMask(next_markbit_index);",
  "current_cell_ = cells_[current_cell_index_] & ~(mask - 1);",
  "const auto trailing_zeros = base::bits::CountTrailingZeros(current_cell_);",
  "Address object_address = current_cell_base + trailing_zeros * kTaggedSize;",
  "current_size_ = ALIGN_TO_ALLOCATION_ALIGNMENT(",
  "if (++current_cell_index_ >= MarkingBitmap::kCellsCount) break;",
  "LiveObjectRange::iterator LiveObjectRange::begin() { return iterator(page_); }",
]) {
  if (!inl.includes(a)) throw new Error(`live-object-range-inl.h 锚点缺失: ${a}`);
}
console.log("LiveObjectRange: 迭代器状态机锚 16");

let rust = banner(
  "src/heap/live-object-range.h.js",
  "src/heap/live-object-range.{h,-inl.h}",
  "活对象区间: 标记位图 cell 扫描迭代器骨架",
);

rust += `
/// 标记位图活对象迭代器 (对应 LiveObjectRange::iterator)。
/// 语义: cell 内 trailing_zeros 定位标记对象, filler/free-space 跳过;
/// 越过对象尾后用 IndexInCellMask 掩掉低位继续扫下一 cell。
/// v8: src/heap/live-object-range-inl.h AdvanceToNextMarkedObject
pub struct LiveObjectIteratorSkeleton {
  /// 对应 current_cell_index_ (MarkingBitmap::CellIndex)
  pub current_cell_index: u32,
  /// 对应 current_cell_ (MarkingBitmap::CellType, 剩余未扫位)
  pub current_cell: u32,
  /// 对应 current_object_
  pub current_object: Option<u64>,
  /// 对应 current_size_ (ALIGN_TO_ALLOCATION_ALIGNMENT 后)
  pub current_size: u32,
}

impl LiveObjectIteratorSkeleton {
  /// 对应 AdvanceToNextMarkedObject 的掩码步进公式:
  /// current_cell_ = cells_[i] & ~(IndexInCellMask(end_index) - 1)
  /// v8: src/heap/live-object-range-inl.h AdvanceToNextMarkedObject
  #[must_use]
  pub fn mask_from_end_index(cell_value: u32, end_index_in_cell: u32) -> u32 {
    // IndexInCellMask: 只含 end_index_in_cell 位的掩码
    let mask = 1u32 << end_index_in_cell;
    cell_value & !(mask.wrapping_sub(1))
  }

  /// 对应 cell 内定位: object_address = cell_base + trailing_zeros * kTaggedSize
  /// v8: src/heap/live-object-range-inl.h AdvanceToNextMarkedObject
  #[must_use]
  pub fn object_offset_in_cell(cell: u32) -> Option<u32> {
    if cell == 0 {
      None
    } else {
      Some(cell.trailing_zeros())
    }
  }

  /// 对应 AdvanceToNextValidObject: 标记命中但 IsFreeSpaceOrFiller → 续扫
  /// v8: src/heap/live-object-range-inl.h AdvanceToNextValidObject
  pub fn advance_past_filler(&mut self, is_free_space_or_filler: impl Fn(u64) -> bool) -> bool {
    while let Some(obj) = self.current_object {
      if !is_free_space_or_filler(obj) {
        return true;
      }
      if !self.advance_to_next_marked_object() {
        return false;
      }
      let _ = obj;
    }
    false
  }

  /// 对应 AdvanceToNextMarkedObject 主体 (位图依赖页/位图族, 此处骨架)
  pub fn advance_to_next_marked_object(&mut self) -> bool {
    unimplemented!("骨架: MarkingBitmap cells/kCellsCount 待 marking 族就位");
  }
}

// —— 提取不变式断言 ——
const _: () = {
  // 掩掉 end 位以下 (含 end 位自身保留与否: ~(mask-1) 保留 end 位)
  assert_eq!(LiveObjectIteratorSkeleton::mask_from_end_index(0b1111, 2), 0b1100);
  assert_eq!(LiveObjectIteratorSkeleton::mask_from_end_index(0b0001, 0), 0b0001);
  assert_eq!(LiveObjectIteratorSkeleton::object_offset_in_cell(0), None);
  assert_eq!(LiveObjectIteratorSkeleton::object_offset_in_cell(0b0100), Some(2));
};
`;

const p = writeSrc("v8-heap-core", "live_object_range.rs.skel", rust);
console.log(`written ${p}`);
