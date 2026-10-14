// 由 tool/src/heap/cppgc-internal/object-start-bitmap.h.js 从 v8/src/heap/cppgc-internal/object-start-bitmap.h
// 生成; 修订就地改, 复跑覆盖
// ObjectStartBitmap 页内对象起始位图 + PlatformAware 子类 (单写多读)
// 模板面承接: 原版 `template <AccessMode mode>` → Rust 运行期 mode 参数 (同 v8-heap-base::AccessMode);
// 单元存储用 [AtomicU8; _]: kNonAtomic 走 Relaxed 载 + get_mut 纯存 (与原版纯访问同码),
// kAtomic 走 acquire 载 / acq_rel exchange (h:167-189 atomic_ref 双臂)。
// 推迟台账 (详账见 readme/zh.md): ShouldForceNonAtomic 的 ARMv7 臂 (h:252-257, 依赖
// WriteBarrier::IsEnabled) 属非目标架构裁剪面 (五: 架构范围), 目标 x64/arm64 恒 false。

use core::ptr::from_ref;
use std::{
  ptr::null_mut,
  sync::atomic::{AtomicU8, Ordering},
};

use crate::{
  globals::{
    AccessMode, Address, ConstAddress, K_ALLOCATION_GRANULARITY, K_ALLOCATION_MASK,
    K_PAGE_BASE_MASK, K_PAGE_OFFSET_MASK, K_PAGE_SIZE,
  },
  heap_object_header::HeapObjectHeader,
};

/// v8: src/heap/cppgc-internal/object-start-bitmap.h:81 kBitsPerCell = sizeof(uint8_t) * CHAR_BIT
pub const K_BITS_PER_CELL: usize = 8;
/// v8: src/heap/cppgc-internal/object-start-bitmap.h:82 kCellMask
pub const K_CELL_MASK: usize = K_BITS_PER_CELL - 1;
/// v8: src/heap/cppgc-internal/object-start-bitmap.h:83-85 kBitmapSize
/// = (kPageSize + 8*kAllocationGranularity - 1) / (8*kAllocationGranularity)
/// = (131072 + 63) / 64 = 2048 (每格 8 bit, 每位表 kAllocationGranularity 对齐地址)
pub const K_BITMAP_SIZE: usize = K_PAGE_SIZE.div_ceil(K_BITS_PER_CELL * K_ALLOCATION_GRANULARITY);
/// v8: src/heap/cppgc-internal/object-start-bitmap.h:86-87 kReservedForBitmap (位图区按分配粒度对齐)
pub const K_RESERVED_FOR_BITMAP: usize = (K_BITMAP_SIZE + K_ALLOCATION_MASK) & !K_ALLOCATION_MASK;

/// v8: src/heap/cppgc-internal/object-start-bitmap.h:26-27 依赖注释 (kPageSize/kAllocationGranularity)
const _: () = assert!(K_PAGE_SIZE == 1 << 17);
const _: () = assert!(K_RESERVED_FOR_BITMAP == K_BITMAP_SIZE);

/// v8: src/heap/cppgc-internal/object-start-bitmap.h:33-105 ObjectStartBitmap
/// 并发多读 / 单 mutator 写; 依赖自身分配在同一 NormalPage 内 (h:32)。
/// `fully_populated_` 非原子 (h:91-100) — 并发清扫期禁查, 数据竞争即误用。
#[repr(C)]
pub struct ObjectStartBitmap {
  /// 页全对象就绪标志 (非原子, 见上)
  fully_populated: bool,
  /// 每 kAllocationGranularity 对齐地址 1 位
  object_start_bit_map: [AtomicU8; K_RESERVED_FOR_BITMAP],
}

impl ObjectStartBitmap {
  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:107-110 构造 — Clear() + MarkAsFullyPopulated()
  pub fn new() -> Self {
    let mut bitmap = Self {
      fully_populated: false,
      object_start_bit_map: [const { AtomicU8::new(0) }; K_RESERVED_FOR_BITMAP],
    };
    bitmap.clear();
    bitmap.mark_as_fully_populated();
    bitmap
  }

  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:36 Granularity()
  pub const GRANULARITY: usize = K_ALLOCATION_GRANULARITY;
  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:39-41 MaxEntries()
  pub const MAX_ENTRIES: usize = K_RESERVED_FOR_BITMAP * K_BITS_PER_CELL;

  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:112-141 FindHeader<mode>
  /// 地址可指对象中部; 自该格起向低地址回退找对象起始。
  ///
  /// # Safety
  /// `self` 与 `address` 须落在同一页 (原版 DCHECK h:119), 且位图已 fully_populated。
  pub unsafe fn find_header(
    &self,
    mode: AccessMode,
    address_maybe_pointing_to_the_middle_of_object: ConstAddress,
  ) -> *mut HeapObjectHeader {
    debug_assert!(self.fully_populated);
    let addr = address_maybe_pointing_to_the_middle_of_object as usize;
    let page_base = addr & K_PAGE_BASE_MASK;
    debug_assert_eq!(page_base, from_ref(self) as usize & K_PAGE_BASE_MASK);
    let object_offset = addr & K_PAGE_OFFSET_MASK;
    let mut object_start_number = object_offset / K_ALLOCATION_GRANULARITY;
    let mut cell_index = object_start_number / K_BITS_PER_CELL;
    debug_assert!(self.object_start_bit_map.len() > cell_index);
    let bit = object_start_number & K_CELL_MASK;
    // 掩出 <=bit 的低位 (含本位), 全零则向低格回退 (h:127-131)
    let mut byte = self.load(mode, cell_index) & ((1 << (bit + 1)) - 1) as u8;
    while byte == 0 && cell_index > 0 {
      cell_index -= 1;
      byte = self.load(mode, cell_index);
    }
    if byte == 0 {
      return null_mut();
    }
    // 格内最高已置位 (原版 bits::CountLeadingZeros(uint8_t) 以 8 位为基, 等价 u8::leading_zeros)
    let leading_zeroes = byte.leading_zeros() as usize;
    object_start_number = cell_index * K_BITS_PER_CELL + (K_BITS_PER_CELL - 1) - leading_zeroes;
    // SAFETY: 根据位图已置位格点与分配粒度换算出的同页内对象头有效指针。
    ((page_base + object_start_number * K_ALLOCATION_GRANULARITY) as ConstAddress)
      .cast_mut()
      .cast()
  }

  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:143-150 SetBit<mode>
  /// 单写者免 CAS: load | (1<<bit) 后 store。
  pub fn set_bit(&mut self, mode: AccessMode, header_address: ConstAddress) {
    let (cell_index, object_bit) = self.object_start_index_and_bit(header_address);
    let value = self.load(AccessMode::NonAtomic, cell_index) | (1 << object_bit) as u8;
    self.store(mode, cell_index, value);
  }

  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:152-158 ClearBit<mode>
  pub fn clear_bit(&mut self, mode: AccessMode, header_address: ConstAddress) {
    let (cell_index, object_bit) = self.object_start_index_and_bit(header_address);
    let value = self.load(AccessMode::NonAtomic, cell_index) & !(1 << object_bit) as u8;
    self.store(mode, cell_index, value);
  }

  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:160-165 CheckBit<mode>
  pub fn check_bit(&self, mode: AccessMode, header_address: ConstAddress) -> bool {
    let (cell_index, object_bit) = self.object_start_index_and_bit(header_address);
    self.load(mode, cell_index) & (1 << object_bit) as u8 != 0
  }

  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:203-222 Iterate<Callback> — CTZ 逐位,
  /// 临时值清位推进, 全表扫; 回调收对象起始地址 (原版 void(Address))。
  pub fn iterate(&self, mut callback: impl FnMut(Address)) {
    let page_base = from_ref(self) as usize & K_PAGE_BASE_MASK;
    for (cell_index, cell) in self.object_start_bit_map.iter().enumerate() {
      let mut value = cell.load(Ordering::Relaxed);
      if value == 0 {
        continue;
      }
      while value != 0 {
        let trailing_zeroes = value.trailing_zeros() as usize;
        let object_start_number = cell_index * K_BITS_PER_CELL + trailing_zeroes;
        callback((page_base + K_ALLOCATION_GRANULARITY * object_start_number) as Address);
        // 清当前对象位以推进 (原版在临时值上清, 不回写位图)
        value &= !(1 << (object_start_number & K_CELL_MASK)) as u8;
      }
    }
  }

  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:224-227 MarkAsFullyPopulated
  pub fn mark_as_fully_populated(&mut self) {
    debug_assert!(!self.fully_populated);
    self.fully_populated = true;
  }

  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:229-232 Clear — 先置未满再清零
  pub fn clear(&mut self) {
    self.fully_populated = false;
    for cell in &self.object_start_bit_map {
      cell.store(0, Ordering::Relaxed);
    }
  }

  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:181-189 load<mode> — kAtomic 走 acquire
  fn load(&self, mode: AccessMode, cell_index: usize) -> u8 {
    let cell = &self.object_start_bit_map[cell_index];
    match mode {
      // kNonAtomic 原版为纯读; Relaxed 原子读在 x64/arm64 生成同一纯载入指令。
      AccessMode::NonAtomic => cell.load(Ordering::Relaxed),
      AccessMode::Atomic => cell.load(Ordering::Acquire),
    }
  }

  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:167-179 store<mode> — kAtomic 用 acq_rel
  /// exchange, 防指针写被重排到置位之前 (并发标记器漏对象)。
  fn store(&mut self, mode: AccessMode, cell_index: usize, value: u8) {
    let cell = &mut self.object_start_bit_map[cell_index];
    if mode == AccessMode::NonAtomic {
      *cell.get_mut() = value;
      return;
    }
    cell.swap(value, Ordering::AcqRel);
  }

  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:191-201 ObjectStartIndexAndBit
  /// 寻址式: offset/gran → cell/8 → bit&7; DCHECK 分配对齐。
  fn object_start_index_and_bit(&self, header_address: ConstAddress) -> (usize, usize) {
    let object_offset = header_address as usize & K_PAGE_OFFSET_MASK;
    debug_assert!(object_offset & K_ALLOCATION_MASK == 0);
    let object_start_number = object_offset / K_ALLOCATION_GRANULARITY;
    let cell_index = object_start_number / K_BITS_PER_CELL;
    debug_assert!(K_BITMAP_SIZE > cell_index);
    (cell_index, object_start_number & K_CELL_MASK)
  }
}

impl Default for ObjectStartBitmap {
  fn default() -> Self {
    Self::new()
  }
}

/// v8: src/heap/cppgc-internal/object-start-bitmap.h:236-247 PlatformAwareObjectStartBitmap
/// 原版继承 ObjectStartBitmap; Rust 无实现继承 → 组合承接 (内层位图直露, 同页内布局语义)。
#[repr(transparent)]
pub struct PlatformAwareObjectStartBitmap {
  /// 原版基类子对象
  pub bitmap: ObjectStartBitmap,
}

impl PlatformAwareObjectStartBitmap {
  /// 原版 h:249-259 ShouldForceNonAtomic<mode> — 仅 V8_HOST_ARCH_ARM 生效 (裁剪面, 恒 false)。
  pub const fn should_force_non_atomic(mode: AccessMode) -> bool {
    // v8: object-start-bitmap.h:252-257 kAtomic && !WriteBarrier::IsEnabled() → true (ARMv7)
    let _ = mode;
    false
  }

  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:261-268 SetBit<mode>
  pub fn set_bit(&mut self, mode: AccessMode, header_address: ConstAddress) {
    if Self::should_force_non_atomic(mode) {
      self.bitmap.set_bit(AccessMode::NonAtomic, header_address);
      return;
    }
    self.bitmap.set_bit(mode, header_address);
  }

  /// v8: src/heap/cppgc-internal/object-start-bitmap.h:270-277 ClearBit<mode>
  pub fn clear_bit(&mut self, mode: AccessMode, header_address: ConstAddress) {
    if Self::should_force_non_atomic(mode) {
      self.bitmap.clear_bit(AccessMode::NonAtomic, header_address);
      return;
    }
    self.bitmap.clear_bit(mode, header_address);
  }
}
