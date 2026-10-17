// 由 tool/src/heap/base/basic-slot-set.h.js 从 v8/src/heap/base/basic-slot-set.h 生成; 修订就地改, 复跑覆盖
// 两级位图槽集合: bucket(cell(bitmap32)) — GC 记录集核心

//! BasicSlotSet: 槽集合, 假设槽按 SlotGranularity 对齐, 偏移域切分为 bucket,
//! 每 bucket 32 cell × 32 bit 位图。
//! 原版分配在指针前藏 num_buckets 头 (写屏障快路径); Rust 侧 buckets.len() 承载。
//! 原版 AsAtomic32 load+store 竞态写 → Rust fetch_or/fetch_and RMW (语义增强, 消丢更新)。

use std::{
  array::from_fn,
  ptr,
  sync::atomic::{AtomicPtr, AtomicU32, Ordering},
};

use crate::error::{Error, Result};

/// 原 SlotCallbackResult (basic-slot-set.h:22)。
pub enum SlotCallbackResult {
  KeepSlot,
  RemoveSlot,
}

/// 原 AccessMode (basic-slot-set.h:37-40): bucket 级是否允许并发访问。
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
  Atomic,
  NonAtomic,
}

/// 原 EmptyBucketMode (basic-slot-set.h:42-45)。
pub enum EmptyBucketMode {
  /// 空桶立即释放。
  FreeEmptyBuckets,
  /// 空桶保留。
  KeepEmptyBuckets,
}

/// 位标志常量表 (机械提取 + 交叉验算通过):
/// kCellsPerBucket=32 kCellsPerBucketLog2=5
/// kCellSizeBytesLog2=2 kCellSizeBytes=1 << kCellSizeBytesLog2
/// kBitsPerCell=32 kBitsPerCellLog2=5
/// kBitsPerBucket=kCellsPerBucket * kBitsPerCell=1024 kBitsPerBucketLog2=kCellsPerBucketLog2 + kBitsPerCellLog2=10
/// v8: src/heap/base/basic-slot-set.h:277-285。
pub const K_CELLS_PER_BUCKET: usize = 32;
/// v8: src/heap/base/basic-slot-set.h:278 kCellsPerBucketLog2。
pub const K_CELLS_PER_BUCKET_LOG2: usize = 5;
/// v8: src/heap/base/basic-slot-set.h:279 kCellSizeBytesLog2。
pub const K_CELL_SIZE_BYTES_LOG2: usize = 2;
/// v8: src/heap/base/basic-slot-set.h:280 kCellSizeBytes = 1 << kCellSizeBytesLog2。
pub const K_CELL_SIZE_BYTES: usize = 1 << K_CELL_SIZE_BYTES_LOG2;
/// v8: src/heap/base/basic-slot-set.h:281 kBitsPerCell。
pub const K_BITS_PER_CELL: usize = 32;
/// v8: src/heap/base/basic-slot-set.h:282 kBitsPerCellLog2。
pub const K_BITS_PER_CELL_LOG2: usize = 5;
/// v8: src/heap/base/basic-slot-set.h:283 kBitsPerBucket。
pub const K_BITS_PER_BUCKET: usize = 1024;
/// v8: src/heap/base/basic-slot-set.h:284-285 kBitsPerBucketLog2。
pub const K_BITS_PER_BUCKET_LOG2: usize = 10;

/// 原 Bucket (basic-slot-set.h:287-338): 32 × u32 原子 cell。
struct Bucket {
  cells: [AtomicU32; K_CELLS_PER_BUCKET],
}

impl Bucket {
  /// 原 Bucket() = default: 全零初始化。
  fn new() -> Self {
    Self {
      cells: from_fn(|_| AtomicU32::new(0)),
    }
  }
}

impl Default for Bucket {
  fn default() -> Self {
    Self::new()
  }
}

impl Bucket {
  /// 原 LoadCell (basic-slot-set.h:296-303): ATOMIC → Acquire, 否则普通读。
  fn load_cell(&self, cell_index: usize, mode: AccessMode) -> u32 {
    debug_assert!(cell_index < K_CELLS_PER_BUCKET);
    let ord = if mode == AccessMode::Atomic {
      Ordering::Acquire
    } else {
      Ordering::Relaxed
    };
    self.cells[cell_index].load(ord)
  }
  /// 原 SetCellBits (basic-slot-set.h:305-312): ATOMIC → fetch_or (RMW 消丢更新), 否则普通置位。
  fn set_cell_bits(&self, cell_index: usize, mask: u32, mode: AccessMode) {
    let ord = if mode == AccessMode::Atomic {
      Ordering::Release
    } else {
      Ordering::Relaxed
    };
    self.cells[cell_index].fetch_or(mask, ord);
  }
  /// 原 ClearCellBits (basic-slot-set.h:314-321): ATOMIC → fetch_and !mask。
  fn clear_cell_bits(&self, cell_index: usize, mask: u32, mode: AccessMode) {
    let ord = if mode == AccessMode::Atomic {
      Ordering::Release
    } else {
      Ordering::Relaxed
    };
    self.cells[cell_index].fetch_and(!mask, ord);
  }
  /// 原 StoreCell (basic-slot-set.h:323-325): Release 整字写。
  fn store_cell(&self, cell_index: usize, value: u32) {
    self.cells[cell_index].store(value, Ordering::Release);
  }
  /// 原 IsEmpty (basic-slot-set.h:327-334): 32 cell 全零。
  fn is_empty(&self) -> bool {
    self.cells.iter().all(|c| c.load(Ordering::Relaxed) == 0)
  }
}

/// 原 BasicSlotSet<SlotGranularity> (basic-slot-set.h:30): 槽粒度对齐的两级位图。
pub struct BasicSlotSet<const SLOT_GRANULARITY: usize> {
  /// 桶指针数组 (Box::into_raw 挂载; null = 桶缺省)。
  buckets: Vec<AtomicPtr<Bucket>>,
}

impl<const SLOT_GRANULARITY: usize> BasicSlotSet<SLOT_GRANULARITY> {
  /// 原 Allocate (basic-slot-set.h:49-75): buckets 个桶全 null。
  pub fn allocate(buckets: usize) -> Box<Self> {
    let set = Self {
      buckets: (0..buckets)
        .map(|_| AtomicPtr::new(ptr::null_mut()))
        .collect(),
    };
    Box::new(set)
  }
  /// 原 num_buckets (basic-slot-set.h:340-342): 原版藏头字段 → Vec::len。
  pub fn num_buckets(&self) -> usize {
    self.buckets.len()
  }
  /// 原 BucketsForSize (basic-slot-set.h:89-92): 上取整。
  pub const fn buckets_for_size(size: usize) -> usize {
    size.div_ceil(SLOT_GRANULARITY * K_BITS_PER_BUCKET)
  }
  /// 原 BucketForSlot (basic-slot-set.h:95-98)。
  pub const fn bucket_for_slot(slot_offset: usize) -> usize {
    slot_offset / (SLOT_GRANULARITY * K_BITS_PER_BUCKET)
  }
  /// 原 OffsetForBucket (basic-slot-set.h:101-103)。
  pub const fn offset_for_bucket(bucket_index: usize) -> usize {
    bucket_index * SLOT_GRANULARITY * K_BITS_PER_BUCKET
  }
  /// 原 Insert (basic-slot-set.h:108-134): 缺桶 CAS 补挂, 置位。
  pub fn insert(&self, mode: AccessMode, slot_offset: usize) -> Result<()> {
    let (bucket_index, cell_index, bit_index) = self.slot_to_indices(slot_offset)?;
    let mask = 1u32 << bit_index;
    let existing = self.load_bucket_ptr(bucket_index, mode);
    let bucket = if existing.is_null() {
      let new_ptr = Box::into_raw(Box::new(Bucket::new()));
      if !self.swap_in_new_bucket(bucket_index, new_ptr) {
        // SAFETY: CAS 失败 → 并发赢家已接管该槽位, 本地所有权回收。
        drop(unsafe { Box::from_raw(new_ptr) });
      }
      // SAFETY: 挂载成功或并发方已挂载; 桶一旦挂载不做并发释放
      // (原版 monotonicity 注释), 指针在结构生存期内有效。
      unsafe { &*self.load_bucket_ptr(bucket_index, mode) }
    } else {
      // SAFETY: 同上, 非空指针即已挂载桶。
      unsafe { &*existing }
    };
    // ATOMIC 路径先探测再置位 (原版免 RMW 快路径); NON_ATOMIC 恒置位。
    let already_set = mode == AccessMode::Atomic && bucket.load_cell(cell_index, mode) & mask != 0;
    if !already_set {
      bucket.set_cell_bits(cell_index, mask, mode);
    }
    Ok(())
  }
  /// 原 Contains (basic-slot-set.h:138-145)。
  pub fn contains(&self, slot_offset: usize) -> Result<bool> {
    let (bucket_index, cell_index, bit_index) = self.slot_to_indices(slot_offset)?;
    Ok(
      self
        .load_bucket(bucket_index, AccessMode::Atomic)
        .is_some_and(|b| b.load_cell(cell_index, AccessMode::Atomic) & (1u32 << bit_index) != 0),
    )
  }
  /// 原 Remove (basic-slot-set.h:148-160)。
  pub fn remove(&self, slot_offset: usize) -> Result<()> {
    let (bucket_index, cell_index, bit_index) = self.slot_to_indices(slot_offset)?;
    let bit_mask = 1u32 << bit_index;
    if let Some(bucket) = self.load_bucket(bucket_index, AccessMode::Atomic)
      && bucket.load_cell(cell_index, AccessMode::Atomic) & bit_mask != 0
    {
      bucket.clear_cell_bits(cell_index, bit_mask, AccessMode::Atomic);
    }
    Ok(())
  }
  /// 原 RemoveRange (basic-slot-set.h:164-238): 区间清位, 越界 CHECK → Err。
  pub fn remove_range(
    &self,
    start_offset: usize,
    end_offset: usize,
    buckets: usize,
    mode: EmptyBucketMode,
  ) -> Result<()> {
    if end_offset > buckets * K_BITS_PER_BUCKET * SLOT_GRANULARITY {
      return Err(Error::RemoveRangeOutOfBounds {
        end: end_offset,
        buckets,
      });
    }
    debug_assert!(start_offset <= end_offset);
    if end_offset == 0 {
      return Ok(());
    }
    let (start_bucket, start_cell, start_bit) = self.slot_to_indices(start_offset)?;
    // 独占端点 → 转含端点再进位 (对齐原版溢位链)。
    let (mut end_bucket, mut end_cell, mut end_bit) =
      self.slot_to_indices(end_offset - SLOT_GRANULARITY)?;
    end_bit += 1;
    if end_bit >= K_BITS_PER_CELL {
      end_bit = 0;
      end_cell += 1;
      if end_cell >= K_CELLS_PER_BUCKET {
        end_cell = 0;
        end_bucket += 1;
      }
    }
    let start_mask = (1u32 << start_bit) - 1;
    let end_mask = !((1u32 << end_bit) - 1);
    if start_bucket == end_bucket && start_cell == end_cell {
      if let Some(bucket) = self.load_bucket(start_bucket, AccessMode::Atomic) {
        bucket.clear_cell_bits(start_cell, !(start_mask | end_mask), AccessMode::Atomic);
      }
      return Ok(());
    }
    let mut current_bucket = start_bucket;
    let mut current_cell = start_cell;
    if let Some(bucket) = self.load_bucket(current_bucket, AccessMode::Atomic) {
      bucket.clear_cell_bits(current_cell, !start_mask, AccessMode::Atomic);
    }
    current_cell += 1;
    if current_bucket < end_bucket {
      if let Some(bucket) = self.load_bucket(current_bucket, AccessMode::Atomic) {
        Self::clear_bucket(bucket, current_cell);
      }
      // 当前桶余部已清, 进入下一桶。
      current_bucket += 1;
      current_cell = 0;
    }
    debug_assert!(
      current_bucket == end_bucket || (current_bucket < end_bucket && current_cell == 0)
    );
    while current_bucket < end_bucket {
      match mode {
        EmptyBucketMode::FreeEmptyBuckets => self.release_bucket(current_bucket),
        EmptyBucketMode::KeepEmptyBuckets => {
          if let Some(bucket) = self.load_bucket(current_bucket, AccessMode::Atomic) {
            Self::clear_bucket(bucket, 0);
          }
        }
      }
      current_bucket += 1;
    }
    debug_assert!(current_bucket == end_bucket);
    if current_bucket == buckets {
      return Ok(());
    }
    if let Some(bucket) = self.load_bucket(current_bucket, AccessMode::Atomic) {
      debug_assert!(current_cell <= end_cell);
      while current_cell < end_cell {
        bucket.store_cell(current_cell, 0);
        current_cell += 1;
      }
      bucket.clear_cell_bits(end_cell, !end_mask, AccessMode::Atomic);
    }
    Ok(())
  }
  /// 原 Lookup (basic-slot-set.h:241-248): 原版与 Contains 同体。
  pub fn lookup(&self, slot_offset: usize) -> Result<bool> {
    self.contains(slot_offset)
  }
  /// 原 Iterate (basic-slot-set.h:265-275 + 345-385): 回调 KEEP_SLOT 保留 / REMOVE_SLOT 清位;
  /// 返回保留槽总数; FREE_EMPTY_BUCKETS 释放空桶。
  pub fn iterate(
    &self,
    chunk_start: usize,
    start_bucket: usize,
    end_bucket: usize,
    mut callback: impl FnMut(usize) -> SlotCallbackResult,
    mode: EmptyBucketMode,
  ) -> usize {
    let mut new_count = 0;
    for bucket_index in start_bucket..end_bucket {
      if let Some(bucket) = self.load_bucket(bucket_index, AccessMode::Atomic) {
        let mut in_bucket_count = 0usize;
        let mut cell_offset = bucket_index << K_BITS_PER_BUCKET_LOG2;
        for (i, cell_ref) in bucket.cells.iter().enumerate() {
          // Iterate 恒原子访问 (原版同): Acquire 读 cell
          let mut cell = cell_ref.load(Ordering::Acquire);
          if cell != 0 {
            let mut mask = 0u32;
            while cell != 0 {
              let bit_offset = cell.trailing_zeros() as usize;
              let bit_mask = 1u32 << bit_offset;
              let slot = (cell_offset + bit_offset) * SLOT_GRANULARITY;
              if let SlotCallbackResult::KeepSlot = callback(chunk_start + slot) {
                in_bucket_count += 1;
              } else {
                mask |= bit_mask;
              }
              cell ^= bit_mask;
            }
            // mask 只累积被清除的位, 非零即有清除需求
            if mask != 0 {
              bucket.clear_cell_bits(i, mask, AccessMode::Atomic);
            }
          }
          cell_offset += K_BITS_PER_CELL;
        }
        if in_bucket_count == 0 && matches!(mode, EmptyBucketMode::FreeEmptyBuckets) {
          self.release_bucket(bucket_index);
        }
        new_count += in_bucket_count;
      }
    }
    new_count
  }
  /// 原 FreeBucketIfEmpty (basic-slot-set.h:387-398, 非原子访问):
  /// 桶缺省或空 → 释放并返回 true; 非空 → false。
  pub fn free_bucket_if_empty(&self, bucket_index: usize) -> bool {
    match self.load_bucket(bucket_index, AccessMode::NonAtomic) {
      Some(bucket) if !bucket.is_empty() => false,
      Some(_) => {
        self.release_bucket(bucket_index);
        true
      }
      None => true,
    }
  }
  /// 原 SlotToIndices (basic-slot-set.h:458-468): 桶越界 CHECK → Err。
  fn slot_to_indices(&self, slot_offset: usize) -> Result<(usize, usize, usize)> {
    debug_assert!(slot_offset.is_multiple_of(SLOT_GRANULARITY));
    let slot = slot_offset / SLOT_GRANULARITY;
    let bucket_index = slot >> K_BITS_PER_BUCKET_LOG2;
    if bucket_index >= self.buckets.len() {
      return Err(Error::SlotBucketOutOfRange {
        bucket: bucket_index,
        buckets: self.buckets.len(),
      });
    }
    let cell_index = (slot >> K_BITS_PER_CELL_LOG2) & (K_CELLS_PER_BUCKET - 1);
    let bit_index = slot & (K_BITS_PER_CELL - 1);
    Ok((bucket_index, cell_index, bit_index))
  }
  /// 原 ReleaseBucket (basic-slot-set.h:410-415): 换出并释放。
  fn release_bucket(&self, bucket_index: usize) {
    let p = self.buckets[bucket_index].swap(ptr::null_mut(), Ordering::Release);
    if !p.is_null() {
      // SAFETY: swap 取回唯一所有权, 无并发持有者。
      drop(unsafe { Box::from_raw(p) });
    }
  }
  fn load_bucket_ptr(&self, bucket_index: usize, mode: AccessMode) -> *mut Bucket {
    let ord = if mode == AccessMode::Atomic {
      Ordering::Acquire
    } else {
      Ordering::Relaxed
    };
    self.buckets[bucket_index].load(ord)
  }
  /// 原 LoadBucket (basic-slot-set.h:417-428): ATOMIC → Acquire。
  fn load_bucket(&self, bucket_index: usize, mode: AccessMode) -> Option<&Bucket> {
    let p = self.load_bucket_ptr(bucket_index, mode);
    // SAFETY: 桶指针由本结构独占管理, 非空即指向已挂载 Bucket
    // (挂载后仅在 stop-the-world 阶段释放, 不与读者并发)。
    (!p.is_null()).then(|| unsafe { &*p })
  }
  /// 原 SwapInNewBucket (basic-slot-set.h:444-455): CAS(null→value)。
  fn swap_in_new_bucket(&self, bucket_index: usize, value: *mut Bucket) -> bool {
    self.buckets[bucket_index]
      .compare_exchange(ptr::null_mut(), value, Ordering::Release, Ordering::Relaxed)
      .is_ok()
  }
  /// 原 ClearBucket (basic-slot-set.h:400-408): 自 start_cell 清到桶尾
  /// (原版 end_cell 参数被循环忽略, 调用点恒传 kCellsPerBucket, 予以省并)。
  fn clear_bucket(bucket: &Bucket, from_cell: usize) {
    debug_assert!(from_cell <= K_CELLS_PER_BUCKET);
    for cell in from_cell..K_CELLS_PER_BUCKET {
      bucket.store_cell(cell, 0);
    }
  }
}

impl<const SLOT_GRANULARITY: usize> Drop for BasicSlotSet<SLOT_GRANULARITY> {
  /// 原 Delete (basic-slot-set.h:77-87): 逐桶释放。
  fn drop(&mut self) {
    for slot in &mut self.buckets {
      let p = *slot.get_mut();
      if !p.is_null() {
        // SAFETY: get_mut 独占访问 (drop 期无并发), 取回所有权。
        drop(unsafe { Box::from_raw(p) });
      }
    }
  }
}
