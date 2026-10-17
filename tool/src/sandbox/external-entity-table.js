#!/usr/bin/env bun
// tool/src/sandbox/external-entity-table.js — 外部实体表骨架生成
// 组→文件映射 (本脚本管辖):
//   v8/src/sandbox/external-entity-table.h + external-entity-table-inl.h
//   → v8-sandbox/src/external_entity_table.rs
// 规则:
//   - RO 段数 / null 表项 / 禁止分配标记常量核对
//   - freelist DCLP (acquire load + lock + CAS) 语义锚点核对
//   - 方法签名骨架直出, 语义修订阶段实现
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/sandbox/external-entity-table.h");
const inl = readV8("src/sandbox/external-entity-table-inl.h");

// ---- 锚点断言 ----
const checks = [];
function check(desc, ok) {
  if (!ok) throw new Error(`external-entity-table: 对数核对失败: ${desc}`);
  checks.push(desc);
}
check("kNumReadOnlySegments = 64KB / kSegmentSize", h.includes("kNumReadOnlySegments = 64 * KB / kSegmentSize"));
check("kInternalNullEntryIndex = 0", h.includes("kInternalNullEntryIndex = 0"));
check("kEntryAllocationIsForbiddenMarker = FreelistHead(-1,-1)", h.includes("FreelistHead(-1, -1)"));
check("freelist 原子字 acquire 载入", inl.includes("freelist_head_.load(std::memory_order_acquire)"));
check("CAS acq_rel 抢占表项", inl.includes("compare_exchange_strong(\n      freelist, new_freelist, std::memory_order_acq_rel)"));
check("Extend release 存储", inl.includes("freelist_head_.store(freelist, std::memory_order_release)"));
check("空 freelist 加锁扩段 (DCLP)", inl.includes("base::MutexGuard guard(&space->mutex_)"));
check("sweep 自顶向下 (Reversed)", inl.includes("base::Reversed(space->segments_)"));
check("全空段回收 (>= kNumReadOnlySegments)", inl.includes("segment.number() >= kNumReadOnlySegments"));
check("AllocateEntry 两次尝试后 OOM", inl.includes("kAllocationTries = 2"));
check("AttachSpaceToReadOnlySegments 先解封再取 null 表项", inl.includes("UnsealReadOnlySegmentScope writable(this)"));
check("RO space 强制 allocate_black", inl.includes("space->set_allocate_black(true)"));
check("AllocateEntryBelow 阈值判断", inl.includes("freelist.next() >= threshold_index"));

const body = `${banner("src/sandbox/external-entity-table.js", "src/sandbox/external-entity-table.h + src/sandbox/external-entity-table-inl.h", "外部实体表: Space(共享 freelist 的段集合) + 原子 CAS 分配 + 通用 sweep; 对数核对 " + checks.length + " 条通过")}
//! ExternalEntityTable; 对应原版 external-entity-table{.h,-inl.h}。
//! 语义: 表分段 (64KB), 段归 Space; Space 共享一条 freelist (原子头字 = next|length);
//! 分配 = CAS 弹出头表项; freelist 空 = 加锁扩段 (DCLP);
//! sweep 自顶向下重建 freelist, 全空段回收。

use std::collections::BTreeSet;

use v8_base_platform::Mutex;

use crate::segmented_table::{
  AtomicFreelistHead, FreelistHead, K_SEGMENT_SIZE, KB, SegmentBase, SegmentedTable, TableEntry,
};

/// RO 段数 (external-entity-table.h:63)
pub const K_NUM_READ_ONLY_SEGMENTS: usize = 64 * KB / K_SEGMENT_SIZE; // = 1
/// null 表项固定索引 0 (external-entity-table.h:264)
pub const K_INTERNAL_NULL_ENTRY_INDEX: u32 = 0;
/// RO 区末尾后的首索引 (external-entity-table.h:265-266)
pub const K_END_OF_READ_ONLY_INDEX: u32 = u32::MAX; // 修订: kEntriesPerSegment * kNumReadOnlySegments
/// 分配禁止标记 (sweep 期间) (external-entity-table.h:227-228)
pub const K_ENTRY_ALLOCATION_IS_FORBIDDEN_MARKER: FreelistHead = FreelistHead::new(u32::MAX, u32::MAX);

// ---- Space (external-entity-table.h:79-162) ----
/// 段集合 + 共享 freelist; 生命周期由所属表管理
pub struct Space {
  /// freelist 头 (原子单字)
  pub freelist_head: AtomicFreelistHead,
  /// 段号集合 (std::set&lt;Segment&gt; 有序 → BTreeSet)
  pub segments: BTreeSet<u32>,
  /// 是否挂接表内部 RO 段
  pub is_internal_read_only_space: bool,
  /// 段集合互斥锁 (扩段/枚举持锁)
  pub mutex: Mutex,
  /// 黑分配 (RO space 强制 true)
  pub allocate_black: bool,
}

impl Space {
  /// 原版 freelist_length (近似值)
  pub fn freelist_length(&self) -> u32 {
    todo!("修订阶段实现")
  }
  /// 原版 num_segments (调用方持锁)
  pub fn num_segments(&self) -> u32 {
    todo!("修订阶段实现")
  }
  /// 原版 NumSegmentsForTesting (自行加锁)
  pub fn num_segments_for_testing(&self) -> u32 {
    todo!("修订阶段实现")
  }
  /// 原版 capacity (调用方持锁)
  pub fn capacity(&self) -> u32 {
    todo!("修订阶段实现")
  }
  /// 原版 Contains(index)
  pub fn contains(&self, index: u32) -> bool {
    todo!("修订阶段实现")
  }
}

// ---- ExternalEntityTable&lt;Entry, size&gt; (external-entity-table.h:53-274) ----
/// 外部实体表: 段式表 + Space 条目分配/sweep
pub struct ExternalEntityTable<E: TableEntry, const SIZE: usize> {
  pub table: SegmentedTable<E, SIZE>,
}

impl<E: TableEntry, const SIZE: usize> ExternalEntityTable<E, SIZE> {
  /// 原版 kSupportsCompaction = false (Compactible 子类覆盖)
  pub const K_SUPPORTS_COMPACTION: bool = false;

  /// 原版 Initialize (external-entity-table-inl.h:48-52)
  pub fn initialize(&mut self) -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 TearDown
  pub fn tear_down(&mut self) -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 InitializeSpace / TearDownSpace (inl.h:60-76)
  pub fn initialize_space(&self, space: &mut Space) {
    todo!("修订阶段实现")
  }
  pub fn tear_down_space(&self, space: &mut Space) -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 AttachSpaceToReadOnlySegments / DetachSpaceFromReadOnlySegments (inl.h:86-120)
  pub fn attach_space_to_read_only_segments(&mut self, space: &mut Space) -> Result<()> {
    todo!("修订阶段实现")
  }
  pub fn detach_space_from_read_only_segments(&mut self, space: &mut Space) {
    todo!("修订阶段实现")
  }

  /// 原版 ZeroInternalNullEntry (inl.h:78-84)
  pub fn zero_internal_null_entry(&mut self) {
    todo!("修订阶段实现")
  }

  /// 原版 AllocateEntry (inl.h:122-132): 两次尝试, 失败原版 OOM → 返回 Err
  pub fn allocate_entry(&mut self, space: &mut Space) -> Result<u32> {
    todo!("修订阶段实现")
  }

  /// 原版 TryAllocateEntry (inl.h:134-184): DCLP + CAS
  pub fn try_allocate_entry(&mut self, space: &mut Space) -> Result<Option<u32>> {
    todo!("修订阶段实现")
  }

  /// 原版 AllocateEntryBelow (inl.h:186-206)
  pub fn allocate_entry_below(&mut self, space: &mut Space, threshold_index: u32) -> u32 {
    todo!("修订阶段实现")
  }

  /// 原版 TryAllocateEntryFromFreelist (inl.h:208-238): CAS 抢占头表项
  pub fn try_allocate_entry_from_freelist(&mut self, space: &mut Space, freelist: FreelistHead) -> bool {
    todo!("修订阶段实现")
  }

  /// 原版 TryExtend (inl.h:240-266): freelist 空时扩段
  pub fn try_extend(&mut self, space: &mut Space) -> Result<Option<FreelistHead>> {
    todo!("修订阶段实现")
  }

  /// 原版 Extend (inl.h:268-286): 段入集 + release 发布 freelist
  pub fn extend(&mut self, space: &mut Space, segment: SegmentBase, freelist: FreelistHead) {
    todo!("修订阶段实现")
  }

  /// 原版 GenericSweep (inl.h:288-361): 自顶向下重建 freelist, 全空段回收
  pub fn generic_sweep(&mut self, space: &mut Space) -> Result<u32> {
    todo!("修订阶段实现")
  }
  /// 原版 GenericSweep 带回调版
  pub fn generic_sweep_with(&mut self, space: &mut Space, mut marked: impl FnMut(&mut E)) -> Result<u32> {
    todo!("修订阶段实现")
  }

  /// 原版 IterateEntriesIn (inl.h:363-375)
  pub fn iterate_entries_in(&mut self, space: &mut Space, mut callback: impl FnMut(u32)) {
    todo!("修订阶段实现")
  }
}
`;

const p = writeSrc("v8-sandbox", "external_entity_table.rs", body);
console.log(`external-entity-table.js: ${p} (对数核对 ${checks.length} 条通过)`);
