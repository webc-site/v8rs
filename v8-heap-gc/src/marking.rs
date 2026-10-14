// 由 tool/src/heap/marking.js 从 v8/src/heap/marking.{h,cc,inl.h} 收编; 布局常量入 heap_layout, 本模块承载位图数据结构与非原子操作 1:1 面。
//! 标记位图: 每页一张, CellType = usize (64 位平台 8 字节)。
//! 本面为非原子 (AccessMode::NON_ATOMIC) 语义 1:1; 原子/并发路径 (Relaxed_SetBits /
//! SeqCst fence) 属多线程 GC 大盘, 见 readme 推迟台账。
//! 夹逼偏离注: 原版 SetRange/ClearRange/AllBits*InRange 假定 index ≤ kLength (调用方 DCHECK
//! 保证, 越界即 OOB); 本面无 unsafe, 以 `min(kLength)` 夹逼输入, 域内逐位行为与原版一致,
//! 域外由 UB 收敛为截断 (tests/heap_gc.rs 超界保护例锚定)。

use std::mem::size_of;

use crate::heap_layout::{
  K_ALL_BITS_SET_IN_CELL_VALUE, K_BITS_PER_BYTE, K_BITS_PER_BYTE_LOG2, K_PAGE_ALIGNMENT_MASK,
  K_REGULAR_PAGE_SIZE, K_TAGGED_SIZE_LOG2,
};

/// v8: src/heap/marking.h MarkBit::CellType
pub type MarkBitCell = usize;

// 位图布局常量面 (对应 MarkingBitmap 的 static constexpr 成员)。
// v8: src/heap/marking.h MarkingBitmap
/// kBitsPerCell = sizeof(CellType) * kBitsPerByte
/// v8: src/heap/marking.h:99 kBitsPerCell
pub const K_BITS_PER_CELL: u32 = (size_of::<MarkBitCell>() as u32) * K_BITS_PER_BYTE;
/// kBitsPerCellLog2 = CountTrailingZeros(kBitsPerCell)
/// v8: src/heap/marking.h:100 kBitsPerCellLog2
pub const K_BITS_PER_CELL_LOG2: u32 = K_BITS_PER_CELL.trailing_zeros();
/// kBitIndexMask = kBitsPerCell - 1
/// v8: src/heap/marking.h:102 kBitIndexMask
pub const K_BIT_INDEX_MASK: u32 = K_BITS_PER_CELL - 1;
/// kBytesPerCell = kBitsPerCell / kBitsPerByte
/// v8: src/heap/marking.h:103 kBytesPerCell
pub const K_BYTES_PER_CELL: u32 = K_BITS_PER_CELL / K_BITS_PER_BYTE;
/// kBytesPerCellLog2 = kBitsPerCellLog2 - kBitsPerByteLog2
/// v8: src/heap/marking.h:104 kBytesPerCellLog2
pub const K_BYTES_PER_CELL_LOG2: u32 = K_BITS_PER_CELL_LOG2 - K_BITS_PER_BYTE_LOG2;
/// kLength = (1 << kPageSizeBits) >> kTaggedSizeLog2 (页内标记位总数)
/// v8: src/heap/marking.h:108 kLength
pub const K_LENGTH: usize = (K_REGULAR_PAGE_SIZE) >> (K_TAGGED_SIZE_LOG2 as usize);
/// kCellsCount = (kLength + kBitsPerCell - 1) >> kBitsPerCellLog2
/// v8: src/heap/marking.h:110 kCellsCount
pub const K_CELLS_COUNT: usize =
  (K_LENGTH + K_BITS_PER_CELL as usize - 1) >> (K_BITS_PER_CELL_LOG2 as usize);
/// kSize = kCellsCount * kBytesPerCell (位图字节数)
/// v8: src/heap/marking.h:114 kSize
pub const K_SIZE: usize = K_CELLS_COUNT * K_BYTES_PER_CELL as usize;

/// index → 所在 cell 下标。
/// v8: src/heap/marking.h:120 MarkingBitmap::IndexToCell
#[must_use]
pub const fn index_to_cell(index: u32) -> u32 {
  index >> K_BITS_PER_CELL_LOG2
}
/// index → 页内地址偏移 (index << kTaggedSizeLog2)。
/// v8: src/heap/marking.h:124 MarkingBitmap::IndexToAddressOffset
#[must_use]
pub const fn index_to_address_offset(index: u32) -> usize {
  (index as usize) << K_TAGGED_SIZE_LOG2
}
/// cell 下标 → 该 cell 首位的页内地址偏移。
/// v8: src/heap/marking.h:128 MarkingBitmap::CellToBase
#[must_use]
pub const fn cell_to_base(cell_index: u32) -> usize {
  index_to_address_offset(cell_index << K_BITS_PER_CELL_LOG2)
}
/// index 在 cell 内的位序。
/// v8: src/heap/marking.h:132 MarkingBitmap::IndexInCell
#[must_use]
pub const fn index_in_cell(index: u32) -> u32 {
  index & K_BIT_INDEX_MASK
}
/// index 在 cell 内的位掩码。
/// v8: src/heap/marking.h:136 MarkingBitmap::IndexInCellMask
#[must_use]
pub const fn index_in_cell_mask(index: u32) -> MarkBitCell {
  1usize << index_in_cell(index)
}
/// 将 index 对齐到 cell 首 (清除 cell 内低位)。
/// v8: src/heap/marking.h:141 MarkingBitmap::CellAlignIndex
#[must_use]
pub const fn cell_align_index(index: u32) -> u32 {
  index & !K_BIT_INDEX_MASK
}
/// 页内地址 → 标记位 index = AddressToOffset(address) >> kTaggedSizeLog2。
/// v8: src/heap/marking-inl.h:184 MarkingBitmap::AddressToIndex
#[must_use]
pub const fn address_to_index(address: usize) -> u32 {
  ((address & K_PAGE_ALIGNMENT_MASK) >> K_TAGGED_SIZE_LOG2) as u32
}
/// limit 地址 → 标记位 index; 页对齐则回落 kLength。
/// v8: src/heap/marking-inl.h:190 MarkingBitmap::LimitAddressToIndex
#[must_use]
pub const fn limit_address_to_index(address: usize) -> u32 {
  if address & K_PAGE_ALIGNMENT_MASK == 0 {
    K_LENGTH as u32
  } else {
    address_to_index(address)
  }
}

/// 单个标记位 = (cell 下标, 位掩码); 非原子读写回落到所属位图 cells。
/// v8: src/heap/marking.h MarkBit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkBit {
  /// v8: MarkBit::cell_ (此处以 cell 下标承接, 免裸指针)
  pub cell_index: u32,
  /// v8: MarkBit::mask_
  pub mask: MarkBitCell,
}

impl MarkBit {
  /// Get<NON_ATOMIC>: (*cell_ & mask_) != 0
  /// v8: src/heap/marking.h:73 MarkBit::Get
  #[must_use]
  pub fn get(&self, cells: &[MarkBitCell]) -> bool {
    (cells[self.cell_index as usize] & self.mask) != 0
  }

  /// Set<NON_ATOMIC>: 返回是否 0→1。
  /// v8: src/heap/marking.h:62 MarkBit::Set
  pub fn set(&self, cells: &mut [MarkBitCell]) -> bool {
    let old = cells[self.cell_index as usize];
    if (old & self.mask) == self.mask {
      return false;
    }
    cells[self.cell_index as usize] = old | self.mask;
    true
  }

  /// Clear: 返回原值是否已置位 (mask_ 全命中)。
  /// v8: src/heap/marking.h:80 MarkBit::Clear
  pub fn clear(&self, cells: &mut [MarkBitCell]) -> bool {
    let old = cells[self.cell_index as usize];
    cells[self.cell_index as usize] = old & !self.mask;
    (old & self.mask) == self.mask
  }
}

/// 标记位图数据结构 (每页一张); cells 零初始化。
/// v8: src/heap/marking.h:94 MarkingBitmap
pub struct MarkingBitmap {
  /// v8: MarkingBitmap::cells_[kCellsCount] = {0}
  pub cells: [MarkBitCell; K_CELLS_COUNT],
}

impl Default for MarkingBitmap {
  fn default() -> Self {
    Self::new()
  }
}

impl MarkingBitmap {
  /// 零初始化位图。
  /// v8: src/heap/marking.h:196 MarkingBitmap::MarkingBitmap
  #[must_use]
  pub const fn new() -> Self {
    Self {
      cells: [0; K_CELLS_COUNT],
    }
  }

  /// MarkBitFromIndexForTesting: 取 index 对应标记位。
  /// v8: src/heap/marking.h:235 MarkingBitmap::MarkBitFromIndexForTesting
  #[must_use]
  pub fn mark_bit_from_index(&self, index: u32) -> MarkBit {
    MarkBit {
      cell_index: index_to_cell(index),
      mask: index_in_cell_mask(index),
    }
  }

  /// SetBitsInCell<NON_ATOMIC>: cells()[i] |= mask
  /// v8: src/heap/marking-inl.h:18 MarkingBitmap::SetBitsInCell
  fn set_bits_in_cell(&mut self, cell_index: u32, mask: MarkBitCell) {
    self.cells[cell_index as usize] |= mask;
  }

  /// ClearBitsInCell<NON_ATOMIC>: cells()[i] &= ~mask
  /// v8: src/heap/marking-inl.h:30 MarkingBitmap::ClearBitsInCell
  fn clear_bits_in_cell(&mut self, cell_index: u32, mask: MarkBitCell) {
    self.cells[cell_index as usize] &= !mask;
  }

  /// SetCellRangeRelaxed<NON_ATOMIC>: [start,end) 全置 1。
  /// v8: src/heap/marking-inl.h:70 MarkingBitmap::SetCellRangeRelaxed
  fn set_cell_range_relaxed(&mut self, start_cell: u32, end_cell: u32) {
    for cell in &mut self.cells[start_cell as usize..end_cell as usize] {
      *cell = K_ALL_BITS_SET_IN_CELL_VALUE;
    }
  }

  /// ClearCellRangeRelaxed<NON_ATOMIC>: [start,end) 清零。
  /// v8: src/heap/marking-inl.h:52 MarkingBitmap::ClearCellRangeRelaxed
  fn clear_cell_range_relaxed(&mut self, start_cell: u32, end_cell: u32) {
    for cell in &mut self.cells[start_cell as usize..end_cell as usize] {
      *cell = 0;
    }
  }

  /// Clear<NON_ATOMIC>: 整幅位图清零。
  /// v8: src/heap/marking-inl.h:78 MarkingBitmap::Clear
  pub fn clear(&mut self) {
    self.clear_cell_range_relaxed(0, K_CELLS_COUNT as u32);
  }

  /// SetRange<NON_ATOMIC>: 置 [start_index, end_index) 全 1。
  /// v8: src/heap/marking-inl.h:88 MarkingBitmap::SetRange
  pub fn set_range(&mut self, start_index: u32, end_index: u32) {
    let start_index = start_index.min(K_LENGTH as u32);
    let end_index = end_index.min(K_LENGTH as u32);
    if start_index >= end_index {
      return;
    }
    let end_index = end_index - 1;
    let start_cell = index_to_cell(start_index);
    let start_mask = index_in_cell_mask(start_index);
    let end_cell = index_to_cell(end_index);
    let end_mask = index_in_cell_mask(end_index);
    if start_cell != end_cell {
      self.set_bits_in_cell(start_cell, !(start_mask - 1));
      self.set_cell_range_relaxed(start_cell + 1, end_cell);
      self.set_bits_in_cell(end_cell, end_mask | (end_mask - 1));
    } else {
      self.set_bits_in_cell(start_cell, end_mask | (end_mask - start_mask));
    }
  }

  /// ClearRange<NON_ATOMIC>: 清 [start_index, end_index)。
  /// v8: src/heap/marking-inl.h:118 MarkingBitmap::ClearRange
  pub fn clear_range(&mut self, start_index: u32, end_index: u32) {
    let start_index = start_index.min(K_LENGTH as u32);
    let end_index = end_index.min(K_LENGTH as u32);
    if start_index >= end_index {
      return;
    }
    let end_index = end_index - 1;
    let start_cell = index_to_cell(start_index);
    let start_mask = index_in_cell_mask(start_index);
    let end_cell = index_to_cell(end_index);
    let end_mask = index_in_cell_mask(end_index);
    if start_cell != end_cell {
      self.clear_bits_in_cell(start_cell, !(start_mask - 1));
      self.clear_cell_range_relaxed(start_cell + 1, end_cell);
      self.clear_bits_in_cell(end_cell, end_mask | (end_mask - 1));
    } else {
      self.clear_bits_in_cell(start_cell, end_mask | (end_mask - start_mask));
    }
  }

  /// AllBitsSetInRange: [start,end) 是否全 1。
  /// v8: src/heap/marking.cc:17 MarkingBitmap::AllBitsSetInRange
  #[must_use]
  pub fn all_bits_set_in_range(&self, start_index: u32, end_index: u32) -> bool {
    let start_index = start_index.min(K_LENGTH as u32);
    let end_index = end_index.min(K_LENGTH as u32);
    if start_index >= end_index {
      return false;
    }
    let end_index = end_index - 1;
    let start_cell = index_to_cell(start_index);
    let start_mask = index_in_cell_mask(start_index);
    let end_cell = index_to_cell(end_index);
    let end_mask = index_in_cell_mask(end_index);
    if start_cell != end_cell {
      let matching_mask = !(start_mask - 1);
      if (self.cells[start_cell as usize] & matching_mask) != matching_mask {
        return false;
      }
      for i in start_cell + 1..end_cell {
        if self.cells[i as usize] != K_ALL_BITS_SET_IN_CELL_VALUE {
          return false;
        }
      }
      let matching_mask = end_mask | (end_mask - 1);
      (self.cells[end_cell as usize] & matching_mask) == matching_mask
    } else {
      let matching_mask = end_mask | (end_mask - start_mask);
      (self.cells[end_cell as usize] & matching_mask) == matching_mask
    }
  }

  /// AllBitsClearInRange: [start,end) 是否全 0。
  /// v8: src/heap/marking.cc:44 MarkingBitmap::AllBitsClearInRange
  #[must_use]
  pub fn all_bits_clear_in_range(&self, start_index: u32, end_index: u32) -> bool {
    let start_index = start_index.min(K_LENGTH as u32);
    let end_index = end_index.min(K_LENGTH as u32);
    if start_index >= end_index {
      return true;
    }
    let end_index = end_index - 1;
    let start_cell = index_to_cell(start_index);
    let start_mask = index_in_cell_mask(start_index);
    let end_cell = index_to_cell(end_index);
    let end_mask = index_in_cell_mask(end_index);
    if start_cell != end_cell {
      let matching_mask = !(start_mask - 1);
      if self.cells[start_cell as usize] & matching_mask != 0 {
        return false;
      }
      for i in start_cell + 1..end_cell {
        if self.cells[i as usize] != 0 {
          return false;
        }
      }
      let matching_mask = end_mask | (end_mask - 1);
      self.cells[end_cell as usize] & matching_mask == 0
    } else {
      let matching_mask = end_mask | (end_mask - start_mask);
      self.cells[end_cell as usize] & matching_mask == 0
    }
  }

  /// IsClean: 全位是否清零。
  /// v8: src/heap/marking.cc:132 MarkingBitmap::IsClean
  #[must_use]
  pub fn is_clean(&self) -> bool {
    self.cells.iter().all(|&cell| cell == 0)
  }
}

/// 标记入队目标 (MarkingHelper::WorklistTarget, 原版仅 kRegular)。
/// v8: src/heap/marking.h MarkingHelper::WorklistTarget
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WorklistTarget {
  /// kRegular
  Regular = 0,
}

/// 存活判定方式 (MarkingHelper::LivenessMode)。
/// v8: src/heap/marking.h MarkingHelper::LivenessMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LivenessMode {
  /// kMarkbit
  Markbit = 0,
  /// kAlwaysLive
  AlwaysLive = 1,
}

// MarkingHelper::ShouldMarkObject / IsUnmarked 依赖 Heap + Tagged<HeapObject>,
// 属并发标记大盘, 见 readme 推迟台账。

// —— 布局不变式 (编译期钉死) ——
const _: () = {
  assert!(K_BITS_PER_CELL == 64);
  assert!(K_BITS_PER_CELL_LOG2 == 6);
  assert!(K_BIT_INDEX_MASK == 63);
  assert!(K_BYTES_PER_CELL == 8);
  assert!(K_BYTES_PER_CELL_LOG2 == 3);
  assert!(K_LENGTH == 65536);
  assert!(K_CELLS_COUNT == 1024);
  assert!(K_SIZE == 8192);
  assert!(matches!(LivenessMode::AlwaysLive, LivenessMode::AlwaysLive));
  assert!(index_to_cell(64) == 1);
  assert!(index_in_cell(70) == 6);
  assert!(cell_align_index(70) == 64);
  assert!(cell_to_base(1) == 256);
};
