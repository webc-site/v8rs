//! v8/src/base/emulated-virtual-address-subspace.h/cc 的转写 — 模拟子空间
//! 可选地由一段父空间页分配背书, 模拟一个可能更大的虚拟地址空间:
//! 优先从背书映射满足分配, 必要时在未映射区以分配 hint 取得新映射
//! 注意 (原版同款警告): 模拟子空间破坏"父空间页分配不会落入子空间"的不变量,
//! 不提供同等安全保证
//! 差异登记: SharedMemory 族未迁移 (依赖 platform 层 handle, 同
//! virtual_address_space 登记项); 父空间以 ParentSpace 裸引用持有

use std::sync::Mutex;

use fastrand::Rng;

use crate::{
  region_allocator::{RegionAllocator, RegionState},
  virtual_address_space::{NO_HINT, PagePermissions, ParentSpace, VirtualAddressSpace},
};

/// 未映射区分配的最大尝试次数 (原版 kMaxAttempts)
const MAX_ATTEMPTS: u32 = 10;

struct EmulatedInner {
  region_allocator: RegionAllocator,
  rng: fastrand::Rng,
}

/// 模拟虚拟地址子空间 (原版 EmulatedVirtualAddressSubspace)
pub struct EmulatedVirtualAddressSubspace {
  page_size: usize,
  allocation_granularity: usize,
  base: usize,
  total_size: usize,
  max_page_permissions: PagePermissions,
  /// 背书映射大小 (位于空间头部)
  mapped_size: usize,
  parent: ParentSpace,
  /// 互斥保护非线程安全的 RegionAllocator 与 rng
  inner: Mutex<EmulatedInner>,
}

impl EmulatedVirtualAddressSubspace {
  /// 构造: parent 分配的 [base, base+mapped_size) 背书一个 total_size 的模拟空间,
  /// 本实例接管背书分配的释放
  ///
  /// # Safety
  /// parent 须存活于本实例整个生命周期 (原版同款契约)
  pub unsafe fn new(
    parent: &dyn VirtualAddressSpace,
    base: usize,
    mapped_size: usize,
    total_size: usize,
  ) -> crate::Result<Self> {
    // 幂要求: 随机地址可简单位掩码生成, 且落入未映射区概率 >= 50% 或恒不落
    // (原版注释同款)
    debug_assert!(mapped_size.is_power_of_two());
    debug_assert!(total_size.is_power_of_two());
    let page_size = parent.page_size();
    let allocation_granularity = parent.allocation_granularity();
    let region_allocator = RegionAllocator::new(base, mapped_size, page_size)?;
    Ok(Self {
      page_size,
      allocation_granularity,
      base,
      total_size,
      max_page_permissions: parent.max_page_permissions(),
      mapped_size,
      // SAFETY: 构造契约由本 unsafe fn 的文档承担
      parent: unsafe { ParentSpace::new(parent) },
      inner: Mutex::new(EmulatedInner {
        region_allocator,
        rng: Rng::new(),
      }),
    })
  }

  /// 未映射区基址
  #[must_use]
  pub const fn unmapped_base(&self) -> usize {
    self.base + self.mapped_size
  }

  /// 未映射区大小
  #[must_use]
  pub const fn unmapped_size(&self) -> usize {
    self.total_size - self.mapped_size
  }

  /// 未映射区是否包含区间
  #[must_use]
  pub const fn unmapped_region_contains(&self, addr: usize, length: usize) -> bool {
    addr >= self.unmapped_base() && addr + length <= self.unmapped_base() + self.unmapped_size()
  }

  /// 背书映射区是否包含区间
  #[must_use]
  pub const fn mapped_region_contains(&self, addr: usize, length: usize) -> bool {
    addr >= self.base && addr + length <= self.base + self.mapped_size
  }

  /// 区间是否包含于整个模拟空间
  #[must_use]
  pub const fn contains_range(&self, addr: usize, length: usize) -> bool {
    addr >= self.base && addr + length <= self.base + self.total_size
  }

  /// 未映射区可用分配大小上限 (限制循环期望运行时间, 原版同款)
  #[must_use]
  pub const fn is_usable_size_for_unmapped_region(&self, size: usize) -> bool {
    size <= self.unmapped_size() / 2
  }

  /// 生成空间内随机地址并按对齐粒度向下取整
  fn random_address_aligned(&self, rng: &mut fastrand::Rng) -> usize {
    let addr = self.base + (rng.u64(0..=u64::MAX) as usize % self.total_size);
    addr & !(self.allocation_granularity - 1)
  }
}

impl Drop for EmulatedVirtualAddressSubspace {
  fn drop(&mut self) {
    // 释放背书映射 (原版析构)
    // SAFETY: 契约保证父空间存活
    unsafe {
      self.parent.get().free_pages(self.base, self.mapped_size);
    }
  }
}

impl VirtualAddressSpace for EmulatedVirtualAddressSubspace {
  fn page_size(&self) -> usize {
    self.page_size
  }

  fn allocation_granularity(&self) -> usize {
    self.allocation_granularity
  }

  fn base(&self) -> usize {
    self.base
  }

  fn size(&self) -> usize {
    self.total_size
  }

  fn max_page_permissions(&self) -> PagePermissions {
    self.max_page_permissions
  }

  fn set_random_seed(&self, seed: i64) {
    let mut inner = self.inner.lock().expect("emulated mutex 不应中毒");
    inner.rng = Rng::with_seed(seed as u64);
  }

  fn random_page_address(&self) -> usize {
    let mut inner = self.inner.lock().expect("emulated mutex 不应中毒");
    self.random_address_aligned(&mut inner.rng)
  }

  fn allocate_pages(
    &self,
    hint: usize,
    size: usize,
    alignment: usize,
    permissions: PagePermissions,
  ) -> Option<usize> {
    // SAFETY: 父空间存活契约
    let parent = unsafe { self.parent.get() };
    let mut hint = hint;
    if hint == NO_HINT || self.mapped_region_contains(hint, size) {
      let mut inner = self.inner.lock().expect("emulated mutex 不应中毒");
      // 先尝试背书映射内分配
      if let Some(address) = inner
        .region_allocator
        .allocate_region_with_hint(hint, size, alignment)
      {
        if parent.set_page_permissions(address, size, permissions) {
          return Some(address);
        }
        // 权限失败 (多半 OOM), 仍尝试未映射区
        // (原版 CHECK_EQ: 回滚 FreeRegion 须在 release 亦执行)
        let freed = inner.region_allocator.free_region(address);
        assert_eq!(freed, Some(size), "AllocatePages 回滚 FreeRegion");
      }
    }

    // 未映射区以分配 hint 取得新映射
    if !self.is_usable_size_for_unmapped_region(size) {
      return None;
    }

    let mut inner = self.inner.lock().expect("emulated mutex 不应中毒");
    for _ in 0..MAX_ATTEMPTS {
      // 未映射区至少覆盖一半空间, 而分配被限制到其 50%, 随机地址命中可用
      // 基址概率 >= 25%, 循环通常很快终止 (原版注释同款)
      debug_assert!(self.unmapped_size() >= self.mapped_size);
      while !self.unmapped_region_contains(hint, size) {
        hint = self.random_address_aligned(&mut inner.rng);
      }
      hint &= !(alignment - 1);

      let result = parent.allocate_pages(hint, size, alignment, permissions);
      match result {
        Some(address) if self.unmapped_region_contains(address, size) => return Some(address),
        Some(address) => parent.free_pages(address, size),
        None => {}
      }
      // 换个地址重试
      hint = self.random_address_aligned(&mut inner.rng);
    }
    None
  }

  fn free_pages(&self, address: usize, size: usize) {
    // SAFETY: 父空间存活契约
    let parent = unsafe { self.parent.get() };
    if self.mapped_region_contains(address, size) {
      let mut inner = self.inner.lock().expect("emulated mutex 不应中毒");
      // 原版 CHECK_EQ + CHECK: 回收与去提交均须在 release 亦执行
      let freed = inner.region_allocator.free_region(address);
      assert_eq!(freed, Some(size), "FreePages FreeRegion");
      assert!(
        parent.decommit_pages(address, size),
        "FreePages DecommitPages"
      );
    } else {
      debug_assert!(self.unmapped_region_contains(address, size));
      parent.free_pages(address, size);
    }
  }

  fn set_page_permissions(
    &self,
    address: usize,
    size: usize,
    permissions: PagePermissions,
  ) -> bool {
    debug_assert!(self.contains_range(address, size));
    // SAFETY: 父空间存活契约
    unsafe { self.parent.get() }.set_page_permissions(address, size, permissions)
  }

  fn allocate_guard_region(&self, address: usize, size: usize) -> bool {
    // SAFETY: 父空间存活契约
    let parent = unsafe { self.parent.get() };
    if self.mapped_region_contains(address, size) {
      let mut inner = self.inner.lock().expect("emulated mutex 不应中毒");
      return inner
        .region_allocator
        .allocate_region_at(address, size, RegionState::Allocated);
    }
    if !self.unmapped_region_contains(address, size) {
      return false;
    }
    parent.allocate_guard_region(address, size)
  }

  fn free_guard_region(&self, address: usize, size: usize) {
    // SAFETY: 父空间存活契约
    let parent = unsafe { self.parent.get() };
    if self.mapped_region_contains(address, size) {
      let mut inner = self.inner.lock().expect("emulated mutex 不应中毒");
      // 原版 CHECK_EQ: 区间回收须在 release 亦执行
      let freed = inner.region_allocator.free_region(address);
      assert_eq!(freed, Some(size), "FreeGuardRegion FreeRegion");
    } else {
      debug_assert!(self.unmapped_region_contains(address, size));
      parent.free_guard_region(address, size);
    }
  }

  fn can_allocate_subspaces(&self) -> bool {
    // 实践上暂无需要, 不支持 (原版同款)
    false
  }

  fn allocate_subspace(
    &self,
    _hint: usize,
    _size: usize,
    _alignment: usize,
    _max_page_permissions: PagePermissions,
  ) -> Option<Box<dyn VirtualAddressSpace>> {
    // 原版 UNIMPLEMENTED → Rust 以 None 承接 (禁 panic)
    None
  }

  fn recommit_pages(&self, address: usize, size: usize, permissions: PagePermissions) -> bool {
    debug_assert!(self.contains_range(address, size));
    // SAFETY: 父空间存活契约
    unsafe { self.parent.get() }.recommit_pages(address, size, permissions)
  }

  fn discard_system_pages(&self, address: usize, size: usize) -> bool {
    debug_assert!(self.contains_range(address, size));
    // SAFETY: 父空间存活契约
    unsafe { self.parent.get() }.discard_system_pages(address, size)
  }

  fn decommit_pages(&self, address: usize, size: usize) -> bool {
    debug_assert!(self.contains_range(address, size));
    // SAFETY: 父空间存活契约
    unsafe { self.parent.get() }.decommit_pages(address, size)
  }

  fn free_subspace(&self, _base: usize, _size: usize) {
    // can_allocate_subspaces 恒 false, 不会有子空间走此路径
  }
}
