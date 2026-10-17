// 由 tool/src/sandbox/external-entity-table.js 从 v8/src/sandbox/external-entity-table.h + src/sandbox/external-entity-table-inl.h 生成; 修订就地改, 复跑覆盖
// 外部实体表: Space(共享 freelist 的段集合) + 原子 CAS 分配 + 通用 sweep; 对数核对 13 条通过

//! ExternalEntityTable; 对应原版 external-entity-table{.h,-inl.h}。
//! 语义: 表分段 (64KB), 段归 Space; Space 共享一条 freelist (原子头字 = next|length);
//! 分配 = CAS 弹出头表项; freelist 空 = 加锁扩段 (DCLP);
//! sweep 自顶向下重建 freelist, 全空段回收。

use std::{collections::BTreeSet, mem::size_of, sync::atomic::Ordering};

use v8_base_platform::Mutex;

use crate::{
  Error, Result,
  internal::KB,
  segmented_table::{
    AtomicFreelistHead, FreelistHead, K_SEGMENT_SIZE, SegmentBase, SegmentedTable, TableEntry,
  },
};

/// RO 段数 (external-entity-table.h:63)
pub const K_NUM_READ_ONLY_SEGMENTS: usize = 64 * KB / K_SEGMENT_SIZE; // = 1
/// null 表项固定索引 0 (external-entity-table.h:264)
pub const K_INTERNAL_NULL_ENTRY_INDEX: u32 = 0;
/// 分配禁止标记 (sweep 期间) (external-entity-table.h:227-228)
pub const K_ENTRY_ALLOCATION_IS_FORBIDDEN_MARKER: FreelistHead =
  FreelistHead::new(u32::MAX, u32::MAX);

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

impl Default for Space {
  fn default() -> Self {
    Self::new()
  }
}

impl Space {
  /// 原版 Space 构造: 空 freelist + 空段集
  pub fn new() -> Self {
    Self {
      freelist_head: AtomicFreelistHead::new(FreelistHead::EMPTY),
      segments: BTreeSet::new(),
      is_internal_read_only_space: false,
      mutex: Mutex::new(),
      allocate_black: false,
    }
  }

  /// freelist 为空且无任何段
  pub fn is_empty(&self) -> bool {
    self.freelist_head.load(Ordering::Relaxed).is_empty() && self.segments.is_empty()
  }

  /// 原版 freelist_length (近似值: 原子头内计数)
  pub fn freelist_length(&self) -> u32 {
    self.freelist_head.load(Ordering::Relaxed).length
  }

  /// 原版 num_segments (调用方持锁)
  pub fn num_segments(&self) -> u32 {
    self.segments.len() as u32
  }

  /// 原版 NumSegmentsForTesting (自行加锁)
  pub fn num_segments_for_testing(&self) -> u32 {
    let _guard = self.mutex.lock_guard();
    self.segments.len() as u32
  }

  /// 原版 capacity: 段数 × 每段条目数 (eps 由表条目大小决定, 调用方传入)
  pub fn capacity(&self, entries_per_segment: u32) -> u32 {
    self.num_segments().saturating_mul(entries_per_segment)
  }

  /// 原版 Contains(index): index 所在段是否归属本 space
  pub fn contains(&self, index: u32, entries_per_segment: u32) -> bool {
    self.segments.contains(&(index / entries_per_segment))
  }
}

// ---- ExternalEntityTable&lt;Entry, size&gt; (external-entity-table.h:53-274) ----
/// 外部实体表: 段式表 + Space 条目分配/sweep
pub struct ExternalEntityTable<E: TableEntry, const SIZE: usize> {
  pub table: SegmentedTable<E, SIZE>,
}

impl<E: TableEntry, const SIZE: usize> Default for ExternalEntityTable<E, SIZE> {
  fn default() -> Self {
    Self {
      table: SegmentedTable::default(),
    }
  }
}

impl<E: TableEntry, const SIZE: usize> ExternalEntityTable<E, SIZE> {
  /// 原版 kSupportsCompaction = false (Compactible 子类覆盖)
  pub const K_SUPPORTS_COMPACTION: bool = false;

  /// 每段条目数 (kEntriesPerSegment)
  pub const K_ENTRIES_PER_SEGMENT: u32 = (K_SEGMENT_SIZE / size_of::<E>()) as u32;
  /// RO 区末尾后的首索引 = kEntriesPerSegment × kNumReadOnlySegments (external-entity-table.h:265-266)
  pub const K_END_OF_READ_ONLY_INDEX: u32 =
    Self::K_ENTRIES_PER_SEGMENT * K_NUM_READ_ONLY_SEGMENTS as u32;

  /// 原版 Initialize (external-entity-table-inl.h:48-52):
  /// 预留 SIZE, RO 区 = kNumReadOnlySegments 段
  pub fn initialize(&mut self) -> Result<()> {
    self
      .table
      .base
      .initialize(SIZE, K_NUM_READ_ONLY_SEGMENTS * K_SEGMENT_SIZE, false)
  }

  pub fn is_initialized(&self) -> bool {
    self.table.base.is_initialized()
  }

  /// 原版 TearDown
  pub fn tear_down(&mut self) -> Result<()> {
    self.table.base.tear_down()
  }

  /// 原版 InitializeSpace / TearDownSpace (inl.h:60-76): 复位 space 全部状态
  pub fn initialize_space(&self, space: &mut Space) {
    space
      .freelist_head
      .store(FreelistHead::EMPTY, Ordering::Release);
    space.segments.clear();
    space.is_internal_read_only_space = false;
    space.allocate_black = false;
  }

  /// 原版 TearDownSpace: 回收 space 全部常规段 (RO 段生命周期归表)
  pub fn tear_down_space(&mut self, space: &mut Space) -> Result<()> {
    let ro_segments = K_NUM_READ_ONLY_SEGMENTS as u32;
    let mut result = Ok(());
    for &number in space.segments.iter() {
      if number < ro_segments {
        continue;
      }
      if let Err(e) = self.table.base.free_table_segment(SegmentBase { number }) {
        result = Err(e);
      }
    }
    self.initialize_space(space);
    result
  }

  /// 原版 AttachSpaceToReadOnlySegments (inl.h:86-120):
  /// RO 段挂接 + null 表项清零 + freelist 覆盖段内 [1, end)
  pub fn attach_space_to_read_only_segments(&mut self, space: &mut Space) -> Result<()> {
    if !self.table.base.is_initialized() {
      return Err(Error::NotInitialized);
    }
    space.is_internal_read_only_space = true;
    space.allocate_black = true;
    {
      let _guard = space.mutex.lock_guard();
      for number in 0..K_NUM_READ_ONLY_SEGMENTS as u32 {
        space.segments.insert(number);
      }
    }
    // 写 RO 段须临时解封 (原版 UnsealReadOnlySegments 语义)
    self.table.base.unseal_read_only_segments()?;
    // null 表项 (索引 0) 清零
    // SAFETY: 索引 0 位于 RO 首段, 已解封为 RW。
    *unsafe { self.table.at(K_INTERNAL_NULL_ENTRY_INDEX) } = E::NULL;
    let start = K_INTERNAL_NULL_ENTRY_INDEX + 1;
    let end = Self::K_END_OF_READ_ONLY_INDEX;
    for i in start..end - 1 {
      // SAFETY: 同上。
      unsafe { self.table.at(i) }.make_freelist_entry(i + 1);
    }
    // SAFETY: 同上。
    unsafe { self.table.at(end - 1) }.make_freelist_entry(0);
    self.table.base.seal_read_only_segments()?;
    space
      .freelist_head
      .store(FreelistHead::new(start, end - start), Ordering::Release);
    Ok(())
  }

  /// 原版 DetachSpaceFromReadOnlySegments (inl.h:86-120 后半):
  /// 摘除 RO 段 + 清空 freelist + RO 段发放计数复位
  pub fn detach_space_from_read_only_segments(&mut self, space: &mut Space) {
    space.is_internal_read_only_space = false;
    space.allocate_black = false;
    {
      let _guard = space.mutex.lock_guard();
      for number in 0..K_NUM_READ_ONLY_SEGMENTS as u32 {
        space.segments.remove(&number);
      }
    }
    space
      .freelist_head
      .store(FreelistHead::EMPTY, Ordering::Release);
    // RO 段生命周期归表: 发放计数复位
    let _ = self.table.base.reset_read_only_segments();
  }

  /// 原版 ZeroInternalNullEntry (inl.h:78-84)
  pub fn zero_internal_null_entry(&mut self) {
    // SAFETY: 索引 0 位于 RO 首段; 调用方保证表已初始化 (原版 DCHECK 同前置)。
    *unsafe { self.table.at(K_INTERNAL_NULL_ENTRY_INDEX) } = E::NULL;
  }

  /// 原版 AllocateEntry (inl.h:122-132): 两次尝试, 失败原版 OOM → 返回 Err
  pub fn allocate_entry(&mut self, space: &mut Space) -> Result<u32> {
    self
      .try_allocate_entry(space)?
      .ok_or(Error::OutOfTableSpace)
  }

  /// 原版 TryAllocateEntry (inl.h:134-184): DCLP + CAS
  pub fn try_allocate_entry(&mut self, space: &mut Space) -> Result<Option<u32>> {
    let mut freelist = space.freelist_head.load(Ordering::Acquire);
    if freelist.is_empty() {
      // DCLP: 锁内复查后扩段 (锁只借 mutex 字段, 扩段拆字段传递避开整结构借用)
      let _guard = space.mutex.lock_guard();
      freelist = space.freelist_head.load(Ordering::Acquire);
      if freelist.is_empty() {
        match self.try_extend_locked(&mut space.segments, &space.freelist_head)? {
          Some(extended) => freelist = extended,
          None => return Ok(None),
        }
      }
    }
    // CAS 弹出头表项 (原版 TryAllocateEntryFromFreelist)
    loop {
      if freelist.is_empty() {
        return Ok(None);
      }
      let index = freelist.next;
      // SAFETY: freelist 链上条目均在已分配段内。
      let next = unsafe { self.table.at(index) }
        .next_freelist_entry()
        .unwrap_or(0);
      let new_head = FreelistHead::new(next, freelist.length - 1);
      match space.freelist_head.compare_exchange_weak(
        freelist,
        new_head,
        Ordering::AcqRel,
        Ordering::Acquire,
      ) {
        Ok(_) => return Ok(Some(index)),
        Err(actual) => freelist = actual,
      }
    }
  }

  /// 原版 AllocateEntryBelow (inl.h:186-206): 只取低于阈值的表项, 不扩段;
  /// 无可用返回 0 (kNullEntry 语义)
  pub fn allocate_entry_below(&mut self, space: &mut Space, threshold_index: u32) -> u32 {
    let _guard = space.mutex.lock_guard();
    let freelist = space.freelist_head.load(Ordering::Acquire);
    if freelist.is_empty() {
      return 0;
    }
    let mut previous = 0;
    let mut current = freelist.next;
    while current != 0 {
      // SAFETY: freelist 链上条目均在已分配段内。
      let next = unsafe { self.table.at(current) }
        .next_freelist_entry()
        .unwrap_or(0);
      if current < threshold_index {
        if previous == 0 {
          // 摘的是链头: 长度减一, 头指向 next
          space.freelist_head.store(
            FreelistHead::new(next, freelist.length - 1),
            Ordering::Release,
          );
        } else {
          // 摘的是中间项: 前驱改指 next
          // SAFETY: 同上。
          unsafe { self.table.at(previous) }.make_freelist_entry(next);
          space.freelist_head.store(
            FreelistHead::new(freelist.next, freelist.length - 1),
            Ordering::Release,
          );
        }
        return current;
      }
      previous = current;
      current = next;
    }
    0
  }

  /// 原版 TryAllocateEntryFromFreelist (inl.h:208-238): CAS 抢占头表项
  pub fn try_allocate_entry_from_freelist(
    &mut self,
    space: &mut Space,
    freelist: FreelistHead,
  ) -> bool {
    if freelist.is_empty() {
      return false;
    }
    // SAFETY: freelist 链上条目均在已分配段内。
    let next = unsafe { self.table.at(freelist.next) }
      .next_freelist_entry()
      .unwrap_or(0);
    let new_head = FreelistHead::new(next, freelist.length - 1);
    space
      .freelist_head
      .compare_exchange_weak(freelist, new_head, Ordering::AcqRel, Ordering::Acquire)
      .is_ok()
  }

  /// 原版 TryExtend (inl.h:240-266): freelist 空时分配新段重建 freelist (自行加锁)
  pub fn try_extend(&mut self, space: &mut Space) -> Result<Option<FreelistHead>> {
    let _guard = space.mutex.lock_guard();
    self.try_extend_locked(&mut space.segments, &space.freelist_head)
  }

  /// 扩段内核 (调用方持 space 锁; 拆字段传递, 与锁借用共存)
  fn try_extend_locked(
    &mut self,
    segments: &mut BTreeSet<u32>,
    freelist_head: &AtomicFreelistHead,
  ) -> Result<Option<FreelistHead>> {
    match self.table.try_allocate_and_initialize_segment()? {
      Some((segment, freelist)) => {
        segments.insert(segment.number);
        freelist_head.store(freelist, Ordering::Release);
        Ok(Some(freelist))
      }
      None => Ok(None),
    }
  }

  /// 原版 Extend (inl.h:268-286): 段入集 + release 发布 freelist (调用方持锁)
  pub fn extend(&mut self, space: &mut Space, segment: SegmentBase, freelist: FreelistHead) {
    space.segments.insert(segment.number);
    space.freelist_head.store(freelist, Ordering::Release);
  }

  /// 原版 GenericSweep (inl.h:288-361): 自顶向下重建 freelist, 全空段回收;
  /// 返回存活条目数
  pub fn generic_sweep(&mut self, space: &mut Space) -> Result<u32> {
    self.generic_sweep_with(space, |_| {})
  }

  /// 原版 GenericSweep 带回调版
  pub fn generic_sweep_with(
    &mut self,
    space: &mut Space,
    mut marked: impl FnMut(&mut E),
  ) -> Result<u32> {
    let _guard = space.mutex.lock_guard();
    let eps = Self::K_ENTRIES_PER_SEGMENT;
    let mut freelist = FreelistHead::EMPTY;
    let mut live = 0u32;
    let mut to_free: Vec<SegmentBase> = Vec::new();
    for &number in space.segments.iter().rev() {
      let saved = freelist;
      let mut segment_live = 0u32;
      for i in (number * eps..(number + 1) * eps).rev() {
        // SAFETY: 段内条目均在已分配段。
        let entry = unsafe { self.table.at(i) };
        if entry.is_marked() {
          entry.unmark();
          segment_live += 1;
          marked(entry);
        } else {
          entry.make_freelist_entry(freelist.next);
          freelist = FreelistHead::new(i, freelist.length + 1);
        }
      }
      if segment_live == 0 && number >= K_NUM_READ_ONLY_SEGMENTS as u32 {
        // 全空常规段: 回收 + freelist 回退 (本段条目不得留在链上)
        freelist = saved;
        to_free.push(SegmentBase { number });
      } else {
        live += segment_live;
      }
    }
    for segment in to_free {
      space.segments.remove(&segment.number);
      // SAFETY: 段号来自本表分配。
      self.table.base.free_table_segment(segment)?;
    }
    space.freelist_head.store(freelist, Ordering::Release);
    Ok(live)
  }

  /// 原版 IterateEntriesIn (inl.h:363-375): space 全部条目 (按段升序)
  pub fn iterate_entries_in(&mut self, space: &mut Space, mut callback: impl FnMut(u32)) {
    let _guard = space.mutex.lock_guard();
    let eps = Self::K_ENTRIES_PER_SEGMENT;
    for &number in space.segments.iter() {
      for i in number * eps..(number + 1) * eps {
        callback(i);
      }
    }
  }
}
