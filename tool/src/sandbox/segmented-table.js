#!/usr/bin/env bun
// tool/src/sandbox/segmented-table.js — 段式表基类骨架生成
// 组→文件映射 (本脚本管辖):
//   v8/src/sandbox/segmented-table.h + segmented-table-inl.h + segmented-table.cc
//   → v8-sandbox/src/segmented_table.rs
// 规则:
//   - 段大小/对齐/RO 段偏移常量核对 (64 位分支)
//   - FreelistHead 单字 (next|length) 布局核对
//   - 方法签名骨架直出, 语义修订阶段实现
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/sandbox/segmented-table.h");
const inl = readV8("src/sandbox/segmented-table-inl.h");
const cc = readV8("src/sandbox/segmented-table.cc");

// ---- 锚点断言 (64 位分支) ----
const checks = [];
function check(desc, ok) {
  if (!ok) throw new Error(`segmented-table: 对数核对失败: ${desc}`);
  checks.push(desc);
}
check("64 位 kUseContiguousMemory == true", /#ifdef V8_TARGET_ARCH_64_BIT[\s\S]{0,300}?kUseContiguousMemory = true/.test(h));
check("64 位 kSegmentSize == 64 * KB", /kSegmentSize = 64 \* KB;/.test(h));
check("kInternalReadOnlySegmentsOffset == 0", h.includes("kInternalReadOnlySegmentsOffset = 0"));
check("kAlignment == kSegmentSize", h.includes("kAlignment = kSegmentSize"));
check("FreelistHead 单原子字断言", h.includes("std::atomic<FreelistHead>::is_always_lock_free"));
check("FreelistHead 布局 next_/length_", /uint32_t next_ = 0;\s*\n\s*uint32_t length_ = 0;/.test(h));
check("freelist 链: 尾节点指向 0", inl.includes('it.index() == last ? 0 : it.index() + 1'));
check("Entry 契约: MakeFreelistEntry / GetNextFreelistEntryIndex", h.includes("MakeFreelistEntry") && h.includes("GetNextFreelistEntry"));
check("TryAllocateSegment: AllocatePages(kSegmentSize, kSegmentSize, kReadWrite)", /AllocatePages\(VirtualAddressSpace::kNoHint, kSegmentSize,\s*\n\s*kSegmentSize, PagePermissions::kReadWrite\)/.test(cc));
check("FreeTableSegment: FreePages 后不可访问", cc.includes("vas_->FreePages(segment_start, kSegmentSize)"));
check("RO 段分配权限 kRead", cc.includes("PagePermissions::kRead"));
check("RO 段 seal/unseal = SetPagePermissions", cc.includes("PagePermissions::kReadWrite") && cc.includes("PagePermissions::kRead"));
check("Initialize: 子空间预留对齐 kAlignment", cc.includes("kAlignment"));
check("InitializeFreeList 返回 FreelistHead(first, num_entries)", inl.includes("FreelistHead(first, num_entries)"));

const body = `${banner("src/sandbox/segmented-table.js", "src/sandbox/segmented-table.h + segmented-table-inl.h + segmented-table.cc", "段式表基类: 大预留 + 64KB 段按需 commit; 生成脚本对数核对 " + checks.length + " 条通过")}
//! SegmentedTableBase / SegmentedTable; 对应原版 segmented-table.{h,cc,-inl.h}。
//! 64 位: 连续大预留 (kUseContiguousMemory), 段 = 64KB, 段分配 = 预留内 mprotect RW,
//! 段释放 = 回收为 PROT_NONE (沙箱安全边界: 释放段不可访问)。

use std::collections::BTreeSet;

use v8_base_platform::{AddressSpaceReservation, MemoryPermission};

use crate::internal::KB;
use crate::{Error, Result};

/// 段大小 (64 位分支, segmented-table.h:40)
pub const K_SEGMENT_SIZE: usize = 64 * KB;
/// 64 位连续大预留 (segmented-table.h:39)
pub const K_USE_CONTIGUOUS_MEMORY: bool = true;
/// RO 段区偏移 (segmented-table.h:52)
pub const K_INTERNAL_READ_ONLY_SEGMENTS_OFFSET: usize = 0;
/// 表预留对齐 = 段大小 (segmented-table.h:53)
pub const K_ALIGNMENT: usize = K_SEGMENT_SIZE;

// ---- 段标识 (segmented-table.h:127-164 SegmentBase) ----
/// 段号 = 段在表内偏移 / 段大小
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SegmentBase {
  pub number: u32,
}

impl SegmentBase {
  /// 原版 SegmentBase::At(offset)
  pub fn at(offset: u32) -> Self {
    debug_assert_eq!(offset as usize % K_SEGMENT_SIZE, 0);
    Self { number: offset / K_SEGMENT_SIZE as u32 }
  }
  pub const fn number(&self) -> u32 { self.number }
  pub const fn offset(&self) -> u32 { self.number * K_SEGMENT_SIZE as u32 }
}

// ---- FreelistHead (segmented-table.h:288-307) ----
/// 单原子字 (next | length<<32) 的 freelist 头; next=0 且 length=0 为空
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreelistHead {
  pub next: u32,
  pub length: u32,
}

impl FreelistHead {
  pub const EMPTY: Self = Self { next: 0, length: 0 };
  pub const fn new(next: u32, length: u32) -> Self { Self { next, length } }
  pub const fn is_empty(&self) -> bool { self.length == 0 }
}

/// std::atomic&lt;FreelistHead&gt; 等价: u64 单字打包原子
#[derive(Debug, Default)]
pub struct AtomicFreelistHead {
  raw: std::sync::atomic::AtomicU64,
}

// ---- 表条目契约 (segmented-table.h:193-196) ----
/// Entry 类型契约: freelist 表示 + mark 位 (GenericSweep 用)
pub trait TableEntry: Copy {
  /// 写保护标记 (原版 Entry::IsWriteProtected; CFI/pkey 集成登记后续梯队)
  const IS_WRITE_PROTECTED: bool = false;
  /// 原版 MakeFreelistEntry(next_entry_index)
  fn make_freelist_entry(&mut self, next_entry_index: u32);
  /// 原版 GetNextFreelistEntryIndex: 非 freelist 表项返回 None
  fn next_freelist_entry(&self) -> Option<u32>;
  /// 原版 IsMarked
  fn is_marked(&self) -> bool;
  /// 原版 Unmark
  fn unmark(&mut self);
}

// ---- SegmentedTableBase (segmented-table.h:25-120, segmented-table.cc) ----
/// 段式表基类: 预留/段分配/释放/RO 段管理; 对条目类型不感知
pub struct SegmentedTableBase {
  /// 表基址 (预留起点)
  pub(crate) base: *mut u8,
  /// 地址空间预留 (OS::allocate PROT_NONE 建立)
  pub(crate) reservation: Option<AddressSpaceReservation>,
  /// 已占用段号集合 (first-fit 分配 + 防重)
  pub(crate) used_segments: BTreeSet<u32>,
  /// 已用 RO 段数
  pub(crate) read_only_segments_used: u32,
  /// RO 预留大小
  pub(crate) read_only_reservation_size: usize,
}

impl SegmentedTableBase {
  /// 原版 Initialize(reservation_size, read_only_reservation_size, wants_write_protection)
  /// (segmented-table.cc:16-97)。预留整块 PROT_NONE; RO 区以 Read 权限前置提交于表基址。
  pub fn initialize(
    &mut self,
    reservation_size: usize,
    read_only_reservation_size: usize,
    _wants_write_protection: bool,
  ) -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 TearDown (segmented-table.cc:99-116)
  pub fn tear_down(&mut self) -> Result<()> {
    todo!("修订阶段实现")
  }

  pub const fn is_initialized(&self) -> bool {
    todo!("修订阶段实现")
  }

  /// 原版 base()
  pub const fn base(&self) -> *mut u8 {
    todo!("修订阶段实现")
  }

  /// 原版 TryAllocateSegment (segmented-table.cc:123-137): first-fit 段位分配 + RW 提交
  pub fn try_allocate_segment(&mut self) -> Result<Option<SegmentBase>> {
    todo!("修订阶段实现")
  }

  /// 原版 FreeTableSegment (segmented-table.cc:118-121): 回收为不可访问
  pub fn free_table_segment(&mut self, segment: SegmentBase) -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 AllocateReadOnlySegment (segmented-table.cc:139-146)
  pub fn allocate_read_only_segment(&mut self) -> Result<SegmentBase> {
    todo!("修订阶段实现")
  }

  /// 原版 ResetReadOnlySegments (segmented-table.cc:148-150)
  pub fn reset_read_only_segments(&mut self) -> u32 {
    todo!("修订阶段实现")
  }

  pub const fn read_only_segments_used(&self) -> u32 {
    todo!("修订阶段实现")
  }

  /// 原版 UnsealReadOnlySegments / SealReadOnlySegments (segmented-table.cc:152-167)
  pub fn unseal_read_only_segments(&self) -> Result<()> {
    todo!("修订阶段实现")
  }
  pub fn seal_read_only_segments(&self) -> Result<()> {
    todo!("修订阶段实现")
  }
}

// ---- UnsealReadOnlySegmentScope (segmented-table.h:166-177) ----
/// 临时解封 RO 段 (RW); Drop 时重新密封 (Read)
pub struct UnsealReadOnlySegmentScope<'a> {
  table: &'a SegmentedTableBase,
}

// ---- SegmentedTable&lt;Entry, size&gt; (segmented-table.h:197-255, segmented-table-inl.h) ----
/// 泛型段式表: 条目访问 + 段 freelist 初始化
pub struct SegmentedTable<E: TableEntry, const SIZE: usize> {
  pub base: SegmentedTableBase,
  pub(crate) _marker: std::marker::PhantomData<E>,
}

impl<E: TableEntry, const SIZE: usize> SegmentedTable<E, SIZE> {
  /// 原版 kEntrySize / kEntriesPerSegment / kReservationSize / kMaxCapacity
  pub const K_ENTRY_SIZE: usize = std::mem::size_of::<E>();
  pub const K_ENTRIES_PER_SEGMENT: usize = K_SEGMENT_SIZE / Self::K_ENTRY_SIZE;
  pub const K_RESERVATION_SIZE: usize = SIZE;
  pub const K_MAX_CAPACITY: usize = SIZE / Self::K_ENTRY_SIZE;

  /// 原版 at(index) (segmented-table-inl.h:32-40)
  ///
  /// SAFETY: index 须在已分配段内。
  pub unsafe fn at(&self, index: u32) -> &mut E {
    todo!("修订阶段实现")
  }

  /// 原版 InitializeFreeList (segmented-table-inl.h:48-62)
  pub fn initialize_free_list(&self, segment: SegmentBase) -> Result<FreelistHead> {
    todo!("修订阶段实现")
  }

  /// 原版 TryAllocateAndInitializeSegment (segmented-table-inl.h:75-84)
  pub fn try_allocate_and_initialize_segment(&mut self) -> Result<Option<(SegmentBase, FreelistHead)>> {
    todo!("修订阶段实现")
  }
}
`;

const p = writeSrc("v8-sandbox", "segmented_table.rs", body);
console.log(`segmented-table.js: ${p} (对数核对 ${checks.length} 条通过)`);
