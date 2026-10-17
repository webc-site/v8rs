// 由 tool/src/sandbox/segmented-table.js 从 v8/src/sandbox/segmented-table.h + segmented-table-inl.h + segmented-table.cc 生成; 修订就地改, 复跑覆盖
// 段式表基类: 大预留 + 64KB 段按需 commit; 生成脚本对数核对 14 条通过

//! SegmentedTableBase / SegmentedTable; 对应原版 segmented-table.{h,cc,-inl.h}。
//! 64 位: 连续大预留 (kUseContiguousMemory), 段 = 64KB, 段分配 = 预留内 mprotect RW,
//! 段释放 = 回收为 PROT_NONE (沙箱安全边界: 释放段不可访问)。

use std::{
  collections::BTreeSet,
  marker::PhantomData,
  mem::{replace, size_of},
  ptr::null_mut,
  result,
  slice::from_raw_parts,
  sync::atomic::{AtomicU64, Ordering},
};

use v8_base_platform::{AddressSpaceReservation, MemoryPermission};

use crate::{Error, Result, internal::KB};

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
    Self {
      number: offset / K_SEGMENT_SIZE as u32,
    }
  }
  pub const fn number(&self) -> u32 {
    self.number
  }
  pub const fn offset(&self) -> u32 {
    self.number * K_SEGMENT_SIZE as u32
  }
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
  pub const fn new(next: u32, length: u32) -> Self {
    Self { next, length }
  }
  pub const fn is_empty(&self) -> bool {
    self.length == 0
  }
}

/// 打包单字: next 占低 32 位, length 占高 32 位
const fn pack_freelist(head: FreelistHead) -> u64 {
  (head.next as u64) | ((head.length as u64) << 32)
}
/// 解包单字
const fn unpack_freelist(raw: u64) -> FreelistHead {
  FreelistHead {
    next: raw as u32,
    length: (raw >> 32) as u32,
  }
}

/// std::atomic&lt;FreelistHead&gt; 等价: u64 单字打包原子
#[derive(Debug, Default)]
pub struct AtomicFreelistHead {
  raw: AtomicU64,
}

impl AtomicFreelistHead {
  /// 以指定头构造
  pub const fn new(head: FreelistHead) -> Self {
    Self {
      raw: AtomicU64::new(pack_freelist(head)),
    }
  }
  /// 读取当前头
  pub fn load(&self, order: Ordering) -> FreelistHead {
    unpack_freelist(self.raw.load(order))
  }
  /// 整字发布
  pub fn store(&self, head: FreelistHead, order: Ordering) {
    self.raw.store(pack_freelist(head), order);
  }
  /// CAS 抢占; 失败返回现场头
  pub fn compare_exchange_weak(
    &self,
    current: FreelistHead,
    new: FreelistHead,
    success: Ordering,
    failure: Ordering,
  ) -> result::Result<FreelistHead, FreelistHead> {
    self
      .raw
      .compare_exchange_weak(pack_freelist(current), pack_freelist(new), success, failure)
      .map(unpack_freelist)
      .map_err(unpack_freelist)
  }
}

// ---- 表条目契约 (segmented-table.h:193-196) ----
/// Entry 类型契约: freelist 表示 + mark 位 (GenericSweep 用)
pub trait TableEntry: Copy {
  /// 写保护标记 (原版 Entry::IsWriteProtected; CFI/pkey 集成登记后续梯队)
  const IS_WRITE_PROTECTED: bool = false;
  /// 原版 kNullEntry: null 表项编码 (索引 0 清零用)
  const NULL: Self;
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
  pub(crate) used_segments: BTreeSet<usize>,
  /// 已用 RO 段数
  pub(crate) read_only_segments_used: u32,
  /// RO 预留大小
  pub(crate) read_only_reservation_size: usize,
}

impl Default for SegmentedTableBase {
  fn default() -> Self {
    Self {
      base: null_mut(),
      reservation: None,
      used_segments: BTreeSet::new(),
      read_only_segments_used: 0,
      read_only_reservation_size: 0,
    }
  }
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
    if self.reservation.is_some() {
      return Err(Error::AlreadyInitialized);
    }
    // 原版 DCHECK: 预留按段对齐; RO 区按段对齐且不超预留
    if !reservation_size.is_multiple_of(K_SEGMENT_SIZE)
      || !read_only_reservation_size.is_multiple_of(K_SEGMENT_SIZE)
      || read_only_reservation_size > reservation_size
    {
      return Err(Error::InvalidArgument);
    }
    // SAFETY: 预留生命周期由 self.reservation 持有, tear_down 统一释放。
    let reservation = unsafe {
      AddressSpaceReservation::create(
        null_mut(),
        reservation_size,
        K_ALIGNMENT,
        MemoryPermission::NoAccess,
      )
    }
    .map_err(|_| Error::SandboxReservationFailed)?;
    let base = reservation.base().cast::<u8>();
    if read_only_reservation_size > 0 {
      // SAFETY: [base, ro_size) 完全位于刚创建的预留内且按段 (含页) 对齐。
      let committed = unsafe {
        reservation.allocate(
          base.cast(),
          read_only_reservation_size,
          MemoryPermission::Read,
        )
      }
      .map_err(|_| Error::SandboxReservationFailed)?;
      if !committed {
        // SAFETY: 预留未登记, 就地释放防泄漏。
        unsafe { reservation.free_reservation() }.ok();
        return Err(Error::SandboxReservationFailed);
      }
    }
    self.base = base;
    self.read_only_reservation_size = read_only_reservation_size;
    self.reservation = Some(reservation);
    Ok(())
  }

  /// 原版 TearDown (segmented-table.cc:99-116)
  pub fn tear_down(&mut self) -> Result<()> {
    if let Some(reservation) = self.reservation.take() {
      // SAFETY: 预留来自 initialize 且未被释放。
      unsafe { reservation.free_reservation() }.map_err(|_| Error::SandboxReservationFailed)?;
    }
    self.base = null_mut();
    self.used_segments.clear();
    self.read_only_segments_used = 0;
    self.read_only_reservation_size = 0;
    Ok(())
  }

  pub const fn is_initialized(&self) -> bool {
    self.reservation.is_some()
  }

  /// 原版 base()
  pub const fn base(&self) -> *mut u8 {
    self.base
  }

  /// 原版 TryAllocateSegment (segmented-table.cc:123-137): first-fit 段位分配 + RW 提交
  pub fn try_allocate_segment(&mut self) -> Result<Option<SegmentBase>> {
    let number = {
      let Some(reservation) = &self.reservation else {
        return Err(Error::NotInitialized);
      };
      let total_segments = reservation.size() / K_SEGMENT_SIZE;
      let first_free = self.read_only_reservation_size / K_SEGMENT_SIZE;
      // first-fit: RO 区之后第一个未占用段号
      let mut number = first_free;
      while self.used_segments.contains(&number) {
        number += 1;
      }
      if number >= total_segments {
        return Ok(None);
      }
      let segment_base = self.base.wrapping_add(number * K_SEGMENT_SIZE);
      // SAFETY: 目标段完全位于预留内且按段 (含页) 对齐。
      let committed = unsafe {
        reservation.allocate(
          segment_base.cast(),
          K_SEGMENT_SIZE,
          MemoryPermission::ReadWrite,
        )
      }
      .map_err(|_| Error::SandboxReservationFailed)?;
      if !committed {
        return Err(Error::SandboxReservationFailed);
      }
      number
    };
    self.used_segments.insert(number);
    Ok(Some(SegmentBase {
      number: number as u32,
    }))
  }

  /// 原版 FreeTableSegment (segmented-table.cc:118-121): 回收为不可访问
  pub fn free_table_segment(&mut self, segment: SegmentBase) -> Result<()> {
    let freed = {
      let Some(reservation) = &self.reservation else {
        return Err(Error::NotInitialized);
      };
      let segment_base = self.base.wrapping_add(segment.offset() as usize);
      // SAFETY: 段号来自本表分配, 段区间在预留内。
      unsafe { reservation.free(segment_base.cast(), K_SEGMENT_SIZE) }
        .map_err(|_| Error::SandboxReservationFailed)?
    };
    if !freed {
      return Err(Error::SandboxViolation);
    }
    self.used_segments.remove(&(segment.number as usize));
    Ok(())
  }

  /// 原版 AllocateReadOnlySegment (segmented-table.cc:139-146)
  pub fn allocate_read_only_segment(&mut self) -> Result<SegmentBase> {
    if self.reservation.is_none() {
      return Err(Error::NotInitialized);
    }
    if self.read_only_segments_used as usize >= self.read_only_reservation_size / K_SEGMENT_SIZE {
      return Err(Error::ReadOnlySegmentExhausted);
    }
    let segment = SegmentBase {
      number: self.read_only_segments_used,
    };
    self.read_only_segments_used += 1;
    Ok(segment)
  }

  /// 原版 ResetReadOnlySegments (segmented-table.cc:148-150)
  pub fn reset_read_only_segments(&mut self) -> u32 {
    replace(&mut self.read_only_segments_used, 0)
  }

  pub const fn read_only_segments_used(&self) -> u32 {
    self.read_only_segments_used
  }

  /// 原版 UnsealReadOnlySegments (segmented-table.cc:152-167): RO 区临时放开为 RW
  pub fn unseal_read_only_segments(&self) -> Result<()> {
    let Some(reservation) = &self.reservation else {
      return Err(Error::NotInitialized);
    };
    if self.read_only_reservation_size == 0 {
      return Ok(());
    }
    // SAFETY: RO 区在预留内且已按页提交。
    let ok = unsafe {
      reservation.set_permissions(
        self.base.cast(),
        self.read_only_reservation_size,
        MemoryPermission::ReadWrite,
      )
    }
    .map_err(|_| Error::SandboxReservationFailed)?;
    if !ok {
      return Err(Error::SandboxReservationFailed);
    }
    Ok(())
  }

  /// 原版 SealReadOnlySegments (segmented-table.cc:152-167): RO 区收回为只读
  pub fn seal_read_only_segments(&self) -> Result<()> {
    let Some(reservation) = &self.reservation else {
      return Err(Error::NotInitialized);
    };
    if self.read_only_reservation_size == 0 {
      return Ok(());
    }
    // SAFETY: 同 unseal。
    let ok = unsafe {
      reservation.set_permissions(
        self.base.cast(),
        self.read_only_reservation_size,
        MemoryPermission::Read,
      )
    }
    .map_err(|_| Error::SandboxReservationFailed)?;
    if !ok {
      return Err(Error::SandboxReservationFailed);
    }
    Ok(())
  }
}

// ---- UnsealReadOnlySegmentScope (segmented-table.h:166-177) ----
/// 临时解封 RO 段 (RW); Drop 时重新密封 (Read)
pub struct UnsealReadOnlySegmentScope<'a> {
  table: &'a SegmentedTableBase,
}

impl<'a> UnsealReadOnlySegmentScope<'a> {
  /// 原版构造: 解封 RO 区
  pub fn new(table: &'a SegmentedTableBase) -> Result<Self> {
    table.unseal_read_only_segments()?;
    Ok(Self { table })
  }

  /// 显式重新密封 (对应析构语义的提前触发)
  pub fn seal(&self) -> Result<()> {
    self.table.seal_read_only_segments()
  }
}

impl Drop for UnsealReadOnlySegmentScope<'_> {
  fn drop(&mut self) {
    // Drop 不能传播错误 (原版析构为 void); 密封失败记日志
    if let Err(e) = self.table.seal_read_only_segments() {
      log::debug!("seal_read_only_segments 失败: {e}");
    }
  }
}

// ---- SegmentedTable&lt;Entry, size&gt; (segmented-table.h:197-255, segmented-table-inl.h) ----
/// 泛型段式表: 条目访问 + 段 freelist 初始化
pub struct SegmentedTable<E: TableEntry, const SIZE: usize> {
  pub base: SegmentedTableBase,
  pub(crate) _marker: PhantomData<E>,
}

impl<E: TableEntry, const SIZE: usize> Default for SegmentedTable<E, SIZE> {
  fn default() -> Self {
    Self {
      base: SegmentedTableBase::default(),
      _marker: PhantomData,
    }
  }
}

impl<E: TableEntry, const SIZE: usize> SegmentedTable<E, SIZE> {
  /// 原版 kEntrySize / kEntriesPerSegment / kReservationSize / kMaxCapacity
  pub const K_ENTRY_SIZE: usize = size_of::<E>();
  pub const K_ENTRIES_PER_SEGMENT: usize = K_SEGMENT_SIZE / Self::K_ENTRY_SIZE;
  pub const K_RESERVATION_SIZE: usize = SIZE;
  pub const K_MAX_CAPACITY: usize = SIZE / Self::K_ENTRY_SIZE;

  /// 条目 index → 表内字节偏移
  #[inline]
  const fn entry_offset(index: u32) -> usize {
    index as usize * Self::K_ENTRY_SIZE
  }

  /// 原版 at(index) (segmented-table-inl.h:32-40)
  ///
  /// # Safety
  ///
  /// index 须在已分配段内; 语义与原版一致不做越界检查。
  pub unsafe fn at(&mut self, index: u32) -> &mut E {
    // SAFETY: 调用方保证槽位已提交可写。
    unsafe {
      &mut *self
        .base
        .base
        .wrapping_add(Self::entry_offset(index))
        .cast::<E>()
    }
  }

  /// 原版 at 的 const 重载: 共享只读视角
  ///
  /// # Safety
  ///
  /// index 须在已分配段内。
  pub unsafe fn at_read(&self, index: u32) -> &E {
    // SAFETY: 调用方保证槽位已提交可读。
    unsafe {
      &*self
        .base
        .base
        .wrapping_add(Self::entry_offset(index))
        .cast::<E>()
    }
  }

  /// 段切片只读视角 (sweep / 枚举批量读)
  ///
  /// # Safety
  ///
  /// 段须已分配。
  pub unsafe fn segment_slice_read(&self, segment: SegmentBase) -> &[E] {
    // SAFETY: 调用方保证段已提交可读; 长度 = 每段条目数。
    unsafe {
      from_raw_parts(
        self
          .base
          .base
          .wrapping_add(segment.offset() as usize)
          .cast::<E>(),
        Self::K_ENTRIES_PER_SEGMENT,
      )
    }
  }

  /// 原版 InitializeFreeList (segmented-table-inl.h:48-62): 段内条目串链, 尾指向 0
  pub fn initialize_free_list(&mut self, segment: SegmentBase) -> Result<FreelistHead> {
    if !self.base.is_initialized() {
      return Err(Error::NotInitialized);
    }
    let start = segment.number as usize * Self::K_ENTRIES_PER_SEGMENT;
    let end = start + Self::K_ENTRIES_PER_SEGMENT;
    for i in start..end - 1 {
      // SAFETY: 段刚提交为 RW, 槽位在段内。
      unsafe { self.at(i as u32) }.make_freelist_entry((i + 1) as u32);
    }
    // SAFETY: 同上。
    unsafe { self.at((end - 1) as u32) }.make_freelist_entry(0);
    Ok(FreelistHead::new(
      start as u32,
      Self::K_ENTRIES_PER_SEGMENT as u32,
    ))
  }

  /// 原版 TryAllocateAndInitializeSegment (segmented-table-inl.h:75-84)
  pub fn try_allocate_and_initialize_segment(
    &mut self,
  ) -> Result<Option<(SegmentBase, FreelistHead)>> {
    let Some(segment) = self.base.try_allocate_segment()? else {
      return Ok(None);
    };
    let freelist = self.initialize_free_list(segment)?;
    Ok(Some((segment, freelist)))
  }
}
