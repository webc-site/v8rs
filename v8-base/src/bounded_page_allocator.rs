//! v8/src/base/bounded-page-allocator.h/cc 的转写 — 预留区内的页分配器
//! 在预先预留的虚拟地址区间内分配页 (RegionAllocator 管理区间切分,
//! 实际映射操作委托底层 PageAllocator — 通常是 mmap 实现)
//! 主要用途: 堆指针压缩 (整个堆需连续地址区间) / 可执行页分配 (32 位
//! PC 相对位移); 线程安全 (原版同款)
//! 差异登记: ReserveForSharedMemoryMapping 保留接口 (Linux 外 No-op 语义同原版);
//! AllocationHint 的 WithAddress/WithMayGrow 以结构体字面构造承接

use std::sync::Mutex;

use crate::{
  page_allocator::{PageAllocator, Permission},
  region_allocator::{AllocationStrategy, RegionAllocator, RegionState},
};

/// 页初始化模式 (原版 PageInitializationMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageInitializationMode {
  /// 分配页必须零初始化 (释放/缩减时归档)
  AllocatedPagesMustBeZeroInitialized,
  /// 分配页可为脏数据 (释放仅置不可访问, 略快)
  AllocatedPagesCanBeUninitialized,
  /// 假定页已处于丢弃态且权限正确 (须配 PageFreeingMode::Discard)
  RecommitOnly,
}

/// 页释放模式 (原版 PageFreeingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFreeingMode {
  /// 置 NoAccess 释放 (平台允许权限重配时首选)
  MakeInaccessible,
  /// DiscardSystemPages 释放 (macOS arm64 不允许把 RWX 改回其他权限时使用;
  /// 与 AllocatedPagesMustBeZeroInitialized 不兼容)
  Discard,
}

/// 分配结果状态 (原版 AllocationStatus)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AllocationStatus {
  /// 成功
  #[default]
  Success,
  /// 权限/提交失败 (多半 OOM)
  FailedToCommit,
  /// 预留区耗尽
  RanOutOfReservation,
  /// hint 地址被占用或不可用
  HintedAddressTakenOrNotFound,
}

/// AllocationStatus 文案 (原版 AllocationStatusToString)
#[must_use]
pub const fn allocation_status_to_string(status: AllocationStatus) -> &'static str {
  match status {
    AllocationStatus::Success => "Success",
    AllocationStatus::FailedToCommit => "Failed to commit",
    AllocationStatus::RanOutOfReservation => "Ran out of reservation",
    AllocationStatus::HintedAddressTakenOrNotFound => "Hinted address was taken or not found",
  }
}

/// 空间统计 (原版 BoundedPageAllocator::Stats)
#[derive(Debug, Clone, Copy)]
pub struct Stats {
  /// 空闲总量
  pub free_size: usize,
  /// 最大空闲连续区
  pub largest_free_region: usize,
  /// 最近一次分配状态
  pub allocation_status: AllocationStatus,
}

/// 分配提示 (原版 v8::PageAllocator::AllocationHint)
#[derive(Debug, Clone, Copy, Default)]
pub struct AllocationHint {
  /// 期望地址 (0 = 无)
  pub address: usize,
  /// 允许增长 → 采用最大空闲区策略
  pub may_grow: bool,
}

impl AllocationHint {
  /// 原版 WithAddress
  #[must_use]
  pub const fn with_address(self, address: usize) -> Self {
    Self {
      address,
      may_grow: self.may_grow,
    }
  }

  /// 原版 WithMayGrow
  #[must_use]
  pub const fn with_may_grow(self) -> Self {
    Self {
      address: self.address,
      may_grow: true,
    }
  }
}

/// 受限页分配器: 在预留区内分配页
/// 底层分配器以借用持有 (原版裸指针; 调用方保证存活期)
pub struct BoundedPageAllocator<'a> {
  allocate_page_size: usize,
  commit_page_size: usize,
  page_allocator: &'a dyn PageAllocator,
  page_initialization_mode: PageInitializationMode,
  page_freeing_mode: PageFreeingMode,
  /// 互斥保护 RegionAllocator 与分配状态 (原版 mutex_)
  inner: Mutex<BoundedInner>,
}

struct BoundedInner {
  region_allocator: RegionAllocator,
  allocation_status: AllocationStatus,
}

impl<'a> BoundedPageAllocator<'a> {
  /// 构造 (原版构造 DCHECK 以 Result 承接对齐校验)
  ///
  /// # Errors
  /// RegionAllocator 参数非法时返回 [`crate::Error::InvalidArg`]
  pub fn new(
    page_allocator: &'a dyn PageAllocator,
    start: usize,
    size: usize,
    allocate_page_size: usize,
    page_initialization_mode: PageInitializationMode,
    page_freeing_mode: PageFreeingMode,
  ) -> crate::Result<Self> {
    debug_assert_eq!(allocate_page_size % page_allocator.allocate_page_size(), 0);
    let commit_page_size = page_allocator.commit_page_size();
    debug_assert_eq!(allocate_page_size % commit_page_size, 0);
    let region_allocator = RegionAllocator::new(start, size, allocate_page_size)?;
    Ok(Self {
      allocate_page_size,
      commit_page_size,
      page_allocator,
      page_initialization_mode,
      page_freeing_mode,
      inner: Mutex::new(BoundedInner {
        region_allocator,
        allocation_status: AllocationStatus::Success,
      }),
    })
  }

  /// 预留区起始地址
  #[must_use]
  pub fn begin(&self) -> usize {
    self
      .inner
      .lock()
      .expect("bounded mutex 不应中毒")
      .region_allocator
      .begin()
  }

  /// 预留区大小
  #[must_use]
  pub fn size(&self) -> usize {
    self
      .inner
      .lock()
      .expect("bounded mutex 不应中毒")
      .region_allocator
      .size()
  }

  /// 地址是否在本分配器控制的区间内
  #[must_use]
  pub fn contains(&self, address: usize) -> bool {
    self
      .inner
      .lock()
      .expect("bounded mutex 不应中毒")
      .region_allocator
      .contains(address)
  }

  /// [address, address+size) 是否完整落在预留区内 (原版 two-arg Contains;
  /// 供 set_permissions/recommit_pages 的 DCHECK 区间检查使用)
  #[must_use]
  pub fn region_contains(&self, address: usize, size: usize) -> bool {
    self
      .inner
      .lock()
      .expect("bounded mutex 不应中毒")
      .region_allocator
      .contains_range(address, size)
  }

  /// 在指定地址分配页 (原版 AllocatePagesAt)
  pub fn allocate_pages_at(&self, address: usize, size: usize, access: Permission) -> bool {
    let mut inner = self.inner.lock().expect("bounded mutex 不应中毒");
    debug_assert_eq!(address % self.allocate_page_size, 0);
    debug_assert_eq!(size % self.allocate_page_size, 0);
    debug_assert!(inner.region_allocator.contains_range(address, size));

    if !inner
      .region_allocator
      .allocate_region_at(address, size, RegionState::Allocated)
    {
      inner.allocation_status = AllocationStatus::HintedAddressTakenOrNotFound;
      return false;
    }
    if !self.page_allocator.set_permissions(address, size, access) {
      // 多半 OOM: 回滚区间 (原版 CHECK_EQ: FreeRegion 须在 release 亦执行)
      let freed = inner.region_allocator.free_region(address);
      assert_eq!(freed, Some(size), "AllocatePagesAt 回滚 FreeRegion");
      inner.allocation_status = AllocationStatus::FailedToCommit;
      return false;
    }
    inner.allocation_status = AllocationStatus::Success;
    true
  }

  /// 原地扩容分配 (原版 ResizeAllocationAt; 缩容不支持)
  pub fn resize_allocation_at(
    &self,
    address: usize,
    old_size: usize,
    new_size: usize,
    access: Permission,
  ) -> bool {
    let mut inner = self.inner.lock().expect("bounded mutex 不应中毒");
    debug_assert_eq!(old_size % self.commit_page_size, 0);
    debug_assert_eq!(new_size % self.commit_page_size, 0);

    if new_size < old_size {
      // 缩容暂不支持 (原版同款)
      return false;
    } else if new_size == old_size {
      return true;
    }

    // 原版 bounded-page-allocator.cc:145 DCHECK_LT(old_size, new_size)
    debug_assert!(old_size < new_size);

    let allocated_old_size = old_size.div_ceil(self.allocate_page_size) * self.allocate_page_size;
    let allocated_new_size = new_size.div_ceil(self.allocate_page_size) * self.allocate_page_size;

    if allocated_old_size < allocated_new_size
      && !inner
        .region_allocator
        .try_grow_region(address, allocated_new_size)
    {
      inner.allocation_status = AllocationStatus::HintedAddressTakenOrNotFound;
      return false;
    }

    if !self
      .page_allocator
      .set_permissions(address + old_size, new_size - old_size, access)
    {
      if allocated_old_size < allocated_new_size {
        // 多半 OOM: 回滚增长 (原版 CHECK_EQ: TrimRegion 须在 release 亦执行)
        let trimmed = inner
          .region_allocator
          .trim_region(address, allocated_old_size);
        assert_eq!(
          trimmed,
          Some(allocated_new_size - allocated_old_size),
          "ResizeAllocationAt 回滚 TrimRegion"
        );
      }
      inner.allocation_status = AllocationStatus::FailedToCommit;
      return false;
    }
    inner.allocation_status = AllocationStatus::Success;
    true
  }

  /// 预留区间供共享内存映射使用 (标记 Excluded + NoAccess)
  pub fn reserve_for_shared_memory_mapping(&self, address: usize, size: usize) -> bool {
    let mut inner = self.inner.lock().expect("bounded mutex 不应中毒");
    debug_assert_eq!(address % self.allocate_page_size, 0);
    debug_assert_eq!(size % self.commit_page_size, 0);
    debug_assert!(inner.region_allocator.contains_range(address, size));

    // 区间表按分配页大小管理, 尾部零头直接并入 (原版同款)
    let region_size = size.div_ceil(self.allocate_page_size) * self.allocate_page_size;
    if !inner
      .region_allocator
      .allocate_region_at(address, region_size, RegionState::Excluded)
    {
      inner.allocation_status = AllocationStatus::HintedAddressTakenOrNotFound;
      return false;
    }
    let success = self
      .page_allocator
      .set_permissions(address, size, Permission::NoAccess);
    inner.allocation_status = if success {
      AllocationStatus::Success
    } else {
      AllocationStatus::FailedToCommit
    };
    success
  }

  /// 最近一次分配状态 (原版 get_last_allocation_status)
  #[must_use]
  pub fn get_last_allocation_status(&self) -> AllocationStatus {
    self
      .inner
      .lock()
      .expect("bounded mutex 不应中毒")
      .allocation_status
  }

  /// 统计快照 (原版 RecordStats)
  #[must_use]
  pub fn record_stats(&self) -> Stats {
    let inner = self.inner.lock().expect("bounded mutex 不应中毒");
    Stats {
      free_size: inner.region_allocator.free_size(),
      largest_free_region: inner.region_allocator.get_largest_free_region_size(),
      allocation_status: inner.allocation_status,
    }
  }

  /// 带 hint 的分配主体 (原版 AllocatePages(size, alignment, access, hint) 重载)
  fn allocate_pages_hinted(
    &self,
    size: usize,
    alignment: usize,
    access: Permission,
    hint: AllocationHint,
  ) -> Option<usize> {
    let mut inner = self.inner.lock().expect("bounded mutex 不应中毒");
    debug_assert_eq!(alignment % inner.region_allocator.page_size(), 0);
    debug_assert_eq!(alignment % self.allocate_page_size, 0);

    let mut address = None;
    let hint_address = hint.address;
    if hint_address != 0
      && hint_address.is_multiple_of(alignment)
      && inner.region_allocator.contains_range(hint_address, size)
      && inner
        .region_allocator
        .allocate_region_at(hint_address, size, RegionState::Allocated)
    {
      address = Some(hint_address);
    }

    let strategy = if hint.may_grow {
      AllocationStrategy::LargestFit
    } else {
      AllocationStrategy::FirstFit
    };

    let address = match address {
      Some(a) => a,
      None => {
        let allocated = if alignment <= self.allocate_page_size {
          // 原版 TODO: 此处可考虑随机化版本
          inner
            .region_allocator
            .allocate_region_with_strategy(size, strategy)
        } else {
          inner
            .region_allocator
            .allocate_aligned_region(size, alignment)
        };
        match allocated {
          Some(a) => a,
          None => {
            // 原版 cc:71: 区间耗尽时显式设置状态再返回失败
            inner.allocation_status = AllocationStatus::RanOutOfReservation;
            return None;
          }
        }
      }
    };

    // 空闲区假定为 NoAccess/WillJitLater 态, 无需权限操作
    if matches!(
      access,
      Permission::NoAccess | Permission::NoAccessWillJitLater
    ) {
      inner.allocation_status = AllocationStatus::Success;
      return Some(address);
    }

    let committed = if self.page_initialization_mode == PageInitializationMode::RecommitOnly {
      self.page_allocator.recommit_pages(address, size, access)
    } else {
      self.page_allocator.set_permissions(address, size, access)
    };
    if committed {
      inner.allocation_status = AllocationStatus::Success;
      return Some(address);
    }

    // 多半 OOM: 回滚 (原版 CHECK_EQ: FreeRegion 须在 release 亦执行)
    let freed = inner.region_allocator.free_region(address);
    assert_eq!(freed, Some(size), "AllocatePages 回滚 FreeRegion");
    inner.allocation_status = AllocationStatus::FailedToCommit;
    None
  }
}

impl PageAllocator for BoundedPageAllocator<'_> {
  fn allocate_page_size(&self) -> usize {
    self.allocate_page_size
  }

  fn commit_page_size(&self) -> usize {
    self.commit_page_size
  }

  fn set_random_mmap_seed(&self, seed: i64) {
    self.page_allocator.set_random_mmap_seed(seed);
  }

  fn get_random_mmap_addr(&self) -> usize {
    self.page_allocator.get_random_mmap_addr()
  }

  fn allocate_pages(
    &self,
    hint: usize,
    length: usize,
    alignment: usize,
    permissions: Permission,
  ) -> Option<usize> {
    self.allocate_pages_hinted(
      length,
      alignment,
      permissions,
      AllocationHint {
        address: hint,
        may_grow: false,
      },
    )
  }

  fn free_pages(&self, address: usize, length: usize) -> bool {
    // 注意: 权限操作开销大, 不持锁执行 (原版注释同款)
    let success = if self.page_initialization_mode
      == PageInitializationMode::AllocatedPagesMustBeZeroInitialized
    {
      debug_assert_ne!(self.page_freeing_mode, PageFreeingMode::Discard);
      // 归档保证下次分配拿到零页
      let ok = self.page_allocator.decommit_pages(address, length);
      // 原版 CHECK(success): 零页契约下 decommit 失败必须立即致命, release 亦执行
      assert!(ok, "FreePages: 必须零初始化时 DecommitPages 失败");
      ok
    } else {
      match self.page_freeing_mode {
        PageFreeingMode::MakeInaccessible => {
          debug_assert_eq!(
            self.page_initialization_mode,
            PageInitializationMode::AllocatedPagesCanBeUninitialized
          );
          self
            .page_allocator
            .set_permissions(address, length, Permission::NoAccess)
        }
        PageFreeingMode::Discard => self.page_allocator.discard_system_pages(address, length),
      }
    };

    let mut inner = self.inner.lock().expect("bounded mutex 不应中毒");
    // 原版 CHECK_EQ: 区间回收须在 release 亦执行 (非仅断言)
    let freed = inner.region_allocator.free_region(address);
    assert_eq!(freed, Some(length), "FreePages FreeRegion");
    success
  }

  fn release_pages(&self, address: usize, length: usize, new_length: usize) -> bool {
    debug_assert_eq!(address % self.allocate_page_size, 0);
    debug_assert!(new_length < length);
    debug_assert_eq!((length - new_length) % self.commit_page_size, 0);

    let mut inner = self.inner.lock().expect("bounded mutex 不应中毒");
    let allocated_size = length.div_ceil(self.allocate_page_size) * self.allocate_page_size;
    let new_allocated_size = new_length.div_ceil(self.allocate_page_size) * self.allocate_page_size;

    debug_assert_eq!(inner.region_allocator.check_region(address), allocated_size);

    if new_allocated_size < allocated_size {
      inner
        .region_allocator
        .trim_region(address, new_allocated_size);
    }

    // 区间保持占用, 仅去提交尾部页
    let free_address = address + new_length;
    let free_size = length - new_length;
    if self.page_initialization_mode == PageInitializationMode::AllocatedPagesMustBeZeroInitialized
    {
      debug_assert_ne!(self.page_freeing_mode, PageFreeingMode::Discard);
      return self.page_allocator.decommit_pages(free_address, free_size);
    }
    if self.page_freeing_mode == PageFreeingMode::MakeInaccessible {
      debug_assert_eq!(
        self.page_initialization_mode,
        PageInitializationMode::AllocatedPagesCanBeUninitialized
      );
      return self
        .page_allocator
        .set_permissions(free_address, free_size, Permission::NoAccess);
    }
    // 原版 CHECK_EQ(page_freeing_mode_, kDiscard): release 亦执行
    assert_eq!(self.page_freeing_mode, PageFreeingMode::Discard);
    self
      .page_allocator
      .discard_system_pages(free_address, free_size)
  }

  fn set_permissions(&self, address: usize, length: usize, permissions: Permission) -> bool {
    debug_assert_eq!(address % self.commit_page_size, 0);
    debug_assert_eq!(length % self.commit_page_size, 0);
    // 原版 DCHECK(region_allocator_.contains(address, size)): 按区间检查
    debug_assert!(self.region_contains(address, length));
    let success = self
      .page_allocator
      .set_permissions(address, length, permissions);
    if !success {
      let mut inner = self.inner.lock().expect("bounded mutex 不应中毒");
      inner.allocation_status = AllocationStatus::FailedToCommit;
    }
    success
  }

  fn recommit_pages(&self, address: usize, length: usize, permissions: Permission) -> bool {
    debug_assert_eq!(address % self.commit_page_size, 0);
    debug_assert_eq!(length % self.commit_page_size, 0);
    // 原版 DCHECK(region_allocator_.contains(address, size)): 按区间检查
    debug_assert!(self.region_contains(address, length));
    let success = self
      .page_allocator
      .recommit_pages(address, length, permissions);
    if !success {
      let mut inner = self.inner.lock().expect("bounded mutex 不应中毒");
      inner.allocation_status = AllocationStatus::FailedToCommit;
    }
    success
  }

  fn discard_system_pages(&self, address: usize, size: usize) -> bool {
    self.page_allocator.discard_system_pages(address, size)
  }

  fn decommit_pages(&self, address: usize, size: usize) -> bool {
    self.page_allocator.decommit_pages(address, size)
  }

  fn seal_pages(&self, address: usize, size: usize) -> bool {
    self.page_allocator.seal_pages(address, size)
  }
}
