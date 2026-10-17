//! v8/src/base/virtual-address-space.h/cc 的转写 — 虚拟地址空间抽象
//! 原版继承 ::v8::VirtualAddressSpace (include/v8-platform.h) → 本模块 trait;
//! 根空间 (base::VirtualAddressSpace) 直接走 OS mmap 族 (复用 page_allocator 原语);
//! 子空间 (VirtualAddressSubspace) 以 AddressSpaceReservation + RegionAllocator 落地
//! (posix 的 reservation 为 {base,size} 薄壳: Allocate=SetPermissions, Free=Decommit)
//! 差异登记 (未迁移, 上层需要时补建):
//! - SharedMemory 族 (AllocateSharedPages/FreeSharedPages/SharedMemoryHandle,
//!   依赖 platform 层 memfd, 属 v8-base-platform 职责)
//! - MemoryProtectionKey (PKU, pkey 字段与 SetPermissionsAndKey 调用链)
//! - 父空间以裸指针持有 (原版同款契约: 父须存活于子空间整个生命周期)

use std::{
  mem::transmute,
  ops::{BitAnd, BitOr},
  sync::{Mutex, OnceLock},
};

use fastrand::Rng;

use crate::{
  page_allocator::{
    DefaultPageAllocator, PageAllocator, Permission, os_allocate_page_size, os_commit_page_size,
    os_decommit_pages, os_discard_system_pages, os_get_random_mmap_addr, os_munmap,
    os_recommit_pages, os_set_memory_region_name, os_set_permissions, os_set_random_mmap_seed,
  },
  region_allocator::{RegionAllocator, RegionState},
};

/// 页权限位标志 (原版 v8-platform.h enum class PagePermissions)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PagePermissions(u8);

impl PagePermissions {
  /// 无任何权限
  pub const NO_ACCESS: Self = Self(0);
  /// 可读
  pub const READ: Self = Self(1);
  /// 可写
  pub const WRITE: Self = Self(2);
  /// 可执行
  pub const EXECUTE: Self = Self(4);
  /// 读写
  pub const READ_WRITE: Self = Self(Self::READ.0 | Self::WRITE.0);
  /// 读执行
  pub const READ_EXECUTE: Self = Self(Self::READ.0 | Self::EXECUTE.0);
  /// 写执行
  pub const WRITE_EXECUTE: Self = Self(Self::WRITE.0 | Self::EXECUTE.0);
  /// 读写执行
  pub const READ_WRITE_EXECUTE: Self = Self(Self::READ.0 | Self::WRITE.0 | Self::EXECUTE.0);

  /// IsSubset(lhs, rhs): self 是否为 other 的子集
  #[must_use]
  pub const fn is_subset_of(self, other: Self) -> bool {
    self.0 & other.0 == self.0
  }
}

impl BitOr for PagePermissions {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self {
    Self(self.0 | rhs.0)
  }
}

impl BitAnd for PagePermissions {
  type Output = Self;
  fn bitand(self, rhs: Self) -> Self {
    Self(self.0 & rhs.0)
  }
}

/// PagePermissions → OS 权限枚举 (原版 ToMemoryPermission)
fn to_permission(permissions: PagePermissions) -> Permission {
  match permissions {
    PagePermissions::NO_ACCESS => Permission::NoAccess,
    PagePermissions::READ => Permission::Read,
    PagePermissions::READ_WRITE => Permission::ReadWrite,
    PagePermissions::READ_WRITE_EXECUTE => Permission::ReadWriteExecute,
    PagePermissions::READ_EXECUTE => Permission::ReadExecute,
    // 原版 UNREACHABLE(): FATAL release 亦执行, 非法位组合不得静默降权
    _ => unreachable!("不支持的权限组合: {permissions:?}"),
  }
}

/// 原版 kNoHint: hint 为 0 表示让实现自选 (随机) 地址
pub const NO_HINT: usize = 0;

/// 虚拟地址空间接口 (原版 ::v8::VirtualAddressSpace + VirtualAddressSpaceBase)
/// 实现必须线程安全 (原版注释: V8 会从多线程并发调用)
pub trait VirtualAddressSpace: Send + Sync {
  /// 空间内页大小 (2 的幂), 页级操作的粒度
  fn page_size(&self) -> usize;

  /// 分配粒度 (2 的幂且为 page_size 倍数; posix 上与页大小一致)
  fn allocation_granularity(&self) -> usize;

  /// 空间基址
  fn base(&self) -> usize;

  /// 空间大小
  fn size(&self) -> usize;

  /// 空间内页面可获得的最高权限
  fn max_page_permissions(&self) -> PagePermissions;

  /// 地址是否在本空间内 (原版非虚辅助)
  #[must_use]
  fn contains(&self, address: usize) -> bool {
    address >= self.base() && address < self.base().wrapping_add(self.size())
  }

  /// 设置随机种子 (random_page_address 产生可复现序列)
  fn set_random_seed(&self, seed: i64);

  /// 空间内随机地址 (按 allocation_granularity 对齐, 作分配 hint)
  fn random_page_address(&self) -> usize;

  /// 分配私有内存页 (hint 对齐 alignment, size 为 granularity 倍数)
  fn allocate_pages(
    &self,
    hint: usize,
    size: usize,
    alignment: usize,
    permissions: PagePermissions,
  ) -> Option<usize>;

  /// 释放 allocate_pages 分配的页 (原版失败 FatalOOM → 此处 log::error)
  fn free_pages(&self, address: usize, size: usize);

  /// 修改已分配页权限 (失败仅 OOM 场景返回 false)
  fn set_page_permissions(&self, address: usize, size: usize, permissions: PagePermissions)
  -> bool;

  /// 在指定地址创建 guard 区 (访问必 fault)
  fn allocate_guard_region(&self, address: usize, size: usize) -> bool;

  /// 释放 guard 区
  fn free_guard_region(&self, address: usize, size: usize);

  /// 是否支持分配子空间 (posix 恒 true)
  fn can_allocate_subspaces(&self) -> bool;

  /// 分配子空间 (预留一段连续虚拟地址区间)
  fn allocate_subspace(
    &self,
    hint: usize,
    size: usize,
    alignment: usize,
    max_page_permissions: PagePermissions,
  ) -> Option<Box<dyn VirtualAddressSpace>>;

  /// 重新提交已丢弃的页 (以原权限)
  fn recommit_pages(&self, address: usize, size: usize, permissions: PagePermissions) -> bool;

  /// 向 OS 提示页不再需要
  fn discard_system_pages(&self, address: usize, size: usize) -> bool;

  /// 归档页 (OS 回收物理页并置 NoAccess, 再次访问保证为零)
  fn decommit_pages(&self, address: usize, size: usize) -> bool;

  /// 为匿名内存区命名 (根空间不支持, 原版同款返回 false)
  fn set_name(&self, _name: &str) -> bool {
    false
  }

  /// 原版 VirtualAddressSpaceBase::FreeSubspace (私有): 子空间 Drop 时回调,
  /// 释放父空间中对应的地址预留
  fn free_subspace(&self, base: usize, size: usize);
}

// ---------------------------------------------------------------------------
// AddressSpaceReservation (原版 platform 层, posix 实现为 {base,size} 薄壳)
// ---------------------------------------------------------------------------

/// 地址空间预留: 一段已 mmap (NoAccess) 的连续区间, 页分配只是改权限
#[derive(Debug, Clone, Copy)]
pub struct AddressSpaceReservation {
  base: usize,
  size: usize,
}

impl AddressSpaceReservation {
  /// 基址
  #[must_use]
  pub const fn base(&self) -> usize {
    self.base
  }

  /// 大小
  #[must_use]
  pub const fn size(&self) -> usize {
    self.size
  }

  /// 区间包含检查 (原版 Contains)
  #[must_use]
  pub const fn contains(&self, region_addr: usize, region_size: usize) -> bool {
    region_addr >= self.base && region_addr + region_size <= self.base + self.size
  }

  /// Allocate: 区间已 mmap, 仅需恢复访问权限 (NoAccess 时原版直接成功)
  fn allocate(&self, address: usize, size: usize, access: Permission) -> bool {
    debug_assert!(self.contains(address, size));
    if access == Permission::NoAccess {
      // 不走 SetPermissions: 避免把页标记为可丢弃 (原版注释同款)
      return true;
    }
    os_set_permissions(address, size, access)
  }

  /// Free: 归档 (重新匿名映射 PROT_NONE, 保证下次访问为零)
  fn free(&self, address: usize, size: usize) -> bool {
    debug_assert!(self.contains(address, size));
    os_decommit_pages(address, size)
  }

  /// SetPermissions
  fn set_permissions(&self, address: usize, size: usize, access: Permission) -> bool {
    debug_assert!(self.contains(address, size));
    os_set_permissions(address, size, access)
  }

  /// CreateSubReservation: posix 上子预留仅是区间切分
  #[must_use]
  pub const fn create_sub_reservation(&self, address: usize, size: usize) -> Option<Self> {
    if address >= self.base && address + size <= self.base + self.size {
      Some(Self {
        base: address,
        size,
      })
    } else {
      None
    }
  }

  /// SetName: 匿名区命名 (仅 Linux prctl 生效)
  fn set_name(&self, name: &str) -> bool {
    os_set_memory_region_name(self.base, self.size, name)
  }
}

/// OS::CreateAddressSpaceReservation — mmap 预留 (max=rwx 时用 WillJitLater,
/// 失败回退 NoAccess, 原版同款)
fn os_create_address_space_reservation(
  hint: usize,
  size: usize,
  alignment: usize,
  max_permission: Permission,
) -> Option<AddressSpaceReservation> {
  let allocator = DefaultPageAllocator;
  let mut permission = Permission::NoAccess;
  if max_permission == Permission::ReadWriteExecute {
    permission = Permission::NoAccessWillJitLater;
  }
  let mut base = allocator.allocate_pages(hint, size, alignment, permission);
  if base.is_none() && permission == Permission::NoAccessWillJitLater {
    // 老版 macOS 等场景回退不带 MAP_JIT
    permission = Permission::NoAccess;
    base = allocator.allocate_pages(hint, size, alignment, permission);
  }
  base.map(|base| AddressSpaceReservation { base, size })
}

/// OS::FreeAddressSpaceReservation — munmap 整段预留
fn os_free_address_space_reservation(reservation: AddressSpaceReservation) {
  os_munmap(reservation.base, reservation.size);
}

// ---------------------------------------------------------------------------
// 根空间 (原版 base::VirtualAddressSpace)
// ---------------------------------------------------------------------------

/// 当前进程的虚拟地址空间 (概念上应只有一个实例; 实际资源由内核管理,
/// 多实例无害 — 原版注释同款)
#[derive(Debug)]
pub struct RootVirtualAddressSpace {
  page_size: usize,
  allocation_granularity: usize,
  max_page_permissions: PagePermissions,
}

impl Default for RootVirtualAddressSpace {
  fn default() -> Self {
    Self::new()
  }
}

impl RootVirtualAddressSpace {
  /// 原版构造: page=CommitPageSize, granularity=AllocatePageSize,
  /// base=0, size=usize::MAX, max=RWX
  #[must_use]
  pub fn new() -> Self {
    let page_size = os_commit_page_size();
    let allocation_granularity = os_allocate_page_size();
    debug_assert!(page_size.is_power_of_two());
    debug_assert!(allocation_granularity.is_power_of_two());
    debug_assert!(allocation_granularity >= page_size);
    debug_assert_eq!(allocation_granularity % page_size, 0);
    Self {
      page_size,
      allocation_granularity,
      max_page_permissions: PagePermissions::READ_WRITE_EXECUTE,
    }
  }
}

impl VirtualAddressSpace for RootVirtualAddressSpace {
  fn page_size(&self) -> usize {
    self.page_size
  }

  fn allocation_granularity(&self) -> usize {
    self.allocation_granularity
  }

  fn base(&self) -> usize {
    0
  }

  fn size(&self) -> usize {
    usize::MAX
  }

  fn max_page_permissions(&self) -> PagePermissions {
    self.max_page_permissions
  }

  fn set_random_seed(&self, seed: i64) {
    os_set_random_mmap_seed(seed);
  }

  fn random_page_address(&self) -> usize {
    os_get_random_mmap_addr()
  }

  fn allocate_pages(
    &self,
    hint: usize,
    size: usize,
    alignment: usize,
    permissions: PagePermissions,
  ) -> Option<usize> {
    debug_assert_eq!(alignment % self.allocation_granularity, 0);
    debug_assert_eq!(hint % alignment, 0);
    debug_assert_eq!(size % self.allocation_granularity, 0);
    DefaultPageAllocator.allocate_pages(hint, size, alignment, to_permission(permissions))
  }

  fn free_pages(&self, address: usize, size: usize) {
    debug_assert_eq!(address % self.allocation_granularity, 0);
    debug_assert_eq!(size % self.allocation_granularity, 0);
    if !os_munmap(address, size) {
      log::error!("RootVirtualAddressSpace::free_pages({address:#x}, {size:#x}) 失败");
    }
  }

  fn set_page_permissions(
    &self,
    address: usize,
    size: usize,
    permissions: PagePermissions,
  ) -> bool {
    debug_assert_eq!(address % self.page_size, 0);
    debug_assert_eq!(size % self.page_size, 0);
    os_set_permissions(address, size, to_permission(permissions))
  }

  fn allocate_guard_region(&self, address: usize, size: usize) -> bool {
    debug_assert_eq!(address % self.allocation_granularity, 0);
    debug_assert_eq!(size % self.allocation_granularity, 0);
    // 无 MAP_FIXED 语义: 结果不在 hint 处则立即释放并报失败 (原版同款)
    let result = DefaultPageAllocator.allocate_pages(
      address,
      size,
      self.allocation_granularity,
      Permission::NoAccess,
    );
    match result {
      Some(base) if base == address => true,
      Some(base) => {
        os_munmap(base, size);
        false
      }
      None => false,
    }
  }

  fn free_guard_region(&self, address: usize, size: usize) {
    debug_assert_eq!(address % self.allocation_granularity, 0);
    debug_assert_eq!(size % self.allocation_granularity, 0);
    if !os_munmap(address, size) {
      log::error!("RootVirtualAddressSpace::free_guard_region({address:#x}, {size:#x}) 失败");
    }
  }

  fn can_allocate_subspaces(&self) -> bool {
    // OS::CanReserveAddressSpace, posix 恒 true
    true
  }

  fn allocate_subspace(
    &self,
    hint: usize,
    size: usize,
    alignment: usize,
    max_page_permissions: PagePermissions,
  ) -> Option<Box<dyn VirtualAddressSpace>> {
    debug_assert_eq!(alignment % self.allocation_granularity, 0);
    debug_assert_eq!(hint % alignment, 0);
    debug_assert_eq!(size % self.allocation_granularity, 0);
    let reservation = os_create_address_space_reservation(
      hint,
      size,
      alignment,
      to_permission(max_page_permissions),
    )?;
    Some(Box::new(VirtualAddressSubspace::new(
      reservation,
      // SAFETY: 父空间即 self, 存活期覆盖子空间 (子空间由 self 的 allocate_subspace 返回)
      unsafe { ParentSpace::new(self) },
      self.page_size,
      self.allocation_granularity,
      max_page_permissions,
    )))
  }

  fn recommit_pages(&self, address: usize, size: usize, permissions: PagePermissions) -> bool {
    debug_assert_eq!(address % self.page_size, 0);
    debug_assert_eq!(size % self.page_size, 0);
    os_recommit_pages(address, size, to_permission(permissions))
  }

  fn discard_system_pages(&self, address: usize, size: usize) -> bool {
    debug_assert_eq!(address % self.page_size, 0);
    debug_assert_eq!(size % self.page_size, 0);
    os_discard_system_pages(address, size)
  }

  fn decommit_pages(&self, address: usize, size: usize) -> bool {
    debug_assert_eq!(address % self.page_size, 0);
    debug_assert_eq!(size % self.page_size, 0);
    os_decommit_pages(address, size)
  }

  fn free_subspace(&self, base: usize, size: usize) {
    os_free_address_space_reservation(AddressSpaceReservation { base, size });
  }
}

// ---------------------------------------------------------------------------
// 子空间 (原版 base::VirtualAddressSubspace)
// ---------------------------------------------------------------------------

/// 父空间引用: 原版裸指针 + "父须存活于子整个生命周期" 契约的等价物
#[derive(Clone, Copy)]
pub struct ParentSpace(*const dyn VirtualAddressSpace);

// SAFETY: trait 已约束实现为 Send + Sync, 此处指针仅借用不加成生命周期;
// 存活期契约由构造方保证 (原版同款约束)
unsafe impl Send for ParentSpace {}
unsafe impl Sync for ParentSpace {}

impl ParentSpace {
  /// 从父空间引用构造 (生命周期擦除, 存活期契约由调用方承担)
  ///
  /// # Safety
  /// parent 须存活于本引用所有副本的整个使用期 (原版裸指针同款约束)
  pub unsafe fn new(parent: &dyn VirtualAddressSpace) -> Self {
    // SAFETY: 仅擦除 trait object 生命周期标注, 指针位模式不变
    Self(unsafe {
      transmute::<*const (dyn VirtualAddressSpace + '_), *const (dyn VirtualAddressSpace + 'static)>(
        parent,
      )
    })
  }

  /// 解引用调用父空间方法
  ///
  /// # Safety
  /// 父空间须仍存活 (构造契约: 父活得比子久)
  pub unsafe fn get(&self) -> &dyn VirtualAddressSpace {
    // SAFETY: 调用方保证存活期
    unsafe { &*self.0 }
  }
}

/// 父空间的虚拟地址子空间: 一段预留的连续虚拟地址区间
/// RegionAllocator 管理区间切分, 页分配通过 reservation 改权限实现
pub struct VirtualAddressSubspace {
  reservation: AddressSpaceReservation,
  page_size: usize,
  allocation_granularity: usize,
  max_page_permissions: PagePermissions,
  /// 互斥保护非线程安全的 RegionAllocator 与 rng (原版 mutex_)
  inner: Mutex<SubspaceInner>,
  parent: ParentSpace,
  /// 成功命名后非空 (仅 Linux prctl 生效)
  name: Mutex<String>,
}

struct SubspaceInner {
  region_allocator: RegionAllocator,
  rng: fastrand::Rng,
}

impl VirtualAddressSubspace {
  /// 原版私有构造 (由父空间 allocate_subspace 调用):
  /// SAFETY 约定见 `parent` 字段 — 父空间须存活于本实例整个生命周期
  fn new(
    reservation: AddressSpaceReservation,
    parent: ParentSpace,
    page_size: usize,
    allocation_granularity: usize,
    max_page_permissions: PagePermissions,
  ) -> Self {
    let region_allocator = RegionAllocator::new(
      reservation.base(),
      reservation.size(),
      allocation_granularity,
    )
    .expect("reservation 参数由父空间对齐保证");
    Self {
      reservation,
      page_size,
      allocation_granularity,
      max_page_permissions,
      inner: Mutex::new(SubspaceInner {
        region_allocator,
        rng: Rng::new(),
      }),
      parent,
      name: Mutex::new(String::new()),
    }
  }
}

impl Drop for VirtualAddressSubspace {
  fn drop(&mut self) {
    // 原版析构: parent_space_->FreeSubspace(this)
    // SAFETY: 契约保证父空间此刻仍存活 (父须活得比子久)
    unsafe {
      self
        .parent
        .get()
        .free_subspace(self.reservation.base(), self.reservation.size());
    }
  }
}

impl VirtualAddressSpace for VirtualAddressSubspace {
  fn page_size(&self) -> usize {
    self.page_size
  }

  fn allocation_granularity(&self) -> usize {
    self.allocation_granularity
  }

  fn base(&self) -> usize {
    self.reservation.base()
  }

  fn size(&self) -> usize {
    self.reservation.size()
  }

  fn max_page_permissions(&self) -> PagePermissions {
    self.max_page_permissions
  }

  fn set_random_seed(&self, seed: i64) {
    let mut inner = self.inner.lock().expect("subspace mutex 不应中毒");
    inner.rng = Rng::with_seed(seed as u64);
  }

  fn random_page_address(&self) -> usize {
    let mut inner = self.inner.lock().expect("subspace mutex 不应中毒");
    // size 非 2 的幂时分布不均匀 (原版注释同款)
    let addr = self.base() + (inner.rng.u64(0..=u64::MAX) as usize % self.size());
    addr & !(self.allocation_granularity - 1)
  }

  fn allocate_pages(
    &self,
    hint: usize,
    size: usize,
    alignment: usize,
    permissions: PagePermissions,
  ) -> Option<usize> {
    debug_assert_eq!(alignment % self.allocation_granularity, 0);
    debug_assert_eq!(hint % alignment, 0);
    debug_assert_eq!(size % self.allocation_granularity, 0);
    debug_assert!(permissions.is_subset_of(self.max_page_permissions));

    let mut inner = self.inner.lock().expect("subspace mutex 不应中毒");
    let address = inner
      .region_allocator
      .allocate_region_with_hint(hint, size, alignment)?;
    if !self
      .reservation
      .allocate(address, size, to_permission(permissions))
    {
      // 多半是 OOM: 回滚区间 (原版 CHECK_EQ: 须在 release 亦执行)
      let freed = inner.region_allocator.free_region(address);
      assert_eq!(freed, Some(size), "AllocatePages 回滚 FreeRegion");
      return None;
    }
    Some(address)
  }

  fn free_pages(&self, address: usize, size: usize) {
    debug_assert_eq!(address % self.allocation_granularity, 0);
    debug_assert_eq!(size % self.allocation_granularity, 0);
    let mut inner = self.inner.lock().expect("subspace mutex 不应中毒");
    if !self.reservation.free(address, size) {
      // OOM (如 VMA 耗尽) — 原版 FatalOOM, 此处记录后继续回收区间
      log::error!("VirtualAddressSubspace::free_pages({address:#x}, {size:#x}) 归档失败");
    }
    // 原版 CHECK_EQ: 区间回收须在 release 亦执行 (非仅断言)
    let freed = inner.region_allocator.free_region(address);
    assert_eq!(
      freed,
      Some(size),
      "VirtualAddressSubspace::FreePages FreeRegion"
    );

    let name = self.name.lock().expect("subspace mutex 不应中毒").clone();
    if !name.is_empty() {
      // MAP_FIXED 换页会重置区名, 需重新设置 (原版 CHECK 同款)。
      // 原版只对被释放子区间 [address, address+size) 命名
      // (OS::SetMemoryRegionName(address, size, name)), 不得对整段
      // reservation 命名 — 否则会波及兄弟子映射的既有区名
      assert!(
        os_set_memory_region_name(address, size, &name),
        "VirtualAddressSubspace::FreePages SetMemoryRegionName"
      );
    }
  }

  fn set_page_permissions(
    &self,
    address: usize,
    size: usize,
    permissions: PagePermissions,
  ) -> bool {
    debug_assert_eq!(address % self.page_size, 0);
    debug_assert_eq!(size % self.page_size, 0);
    debug_assert!(permissions.is_subset_of(self.max_page_permissions));
    self
      .reservation
      .set_permissions(address, size, to_permission(permissions))
  }

  fn allocate_guard_region(&self, address: usize, size: usize) -> bool {
    debug_assert_eq!(address % self.allocation_granularity, 0);
    debug_assert_eq!(size % self.allocation_granularity, 0);
    let mut inner = self.inner.lock().expect("subspace mutex 不应中毒");
    // 预留区本身不可访问, 只需在区间表中登记占用
    inner
      .region_allocator
      .allocate_region_at(address, size, RegionState::Allocated)
  }

  fn free_guard_region(&self, address: usize, size: usize) {
    debug_assert_eq!(address % self.allocation_granularity, 0);
    debug_assert_eq!(size % self.allocation_granularity, 0);
    let mut inner = self.inner.lock().expect("subspace mutex 不应中毒");
    // 原版 CHECK_EQ: 区间回收须在 release 亦执行
    let freed = inner.region_allocator.free_region(address);
    assert_eq!(freed, Some(size), "FreeGuardRegion FreeRegion");
  }

  fn can_allocate_subspaces(&self) -> bool {
    true
  }

  fn allocate_subspace(
    &self,
    hint: usize,
    size: usize,
    alignment: usize,
    max_page_permissions: PagePermissions,
  ) -> Option<Box<dyn VirtualAddressSpace>> {
    debug_assert_eq!(alignment % self.allocation_granularity, 0);
    debug_assert_eq!(hint % alignment, 0);
    debug_assert_eq!(size % self.allocation_granularity, 0);
    debug_assert!(max_page_permissions.is_subset_of(self.max_page_permissions));

    let mut inner = self.inner.lock().expect("subspace mutex 不应中毒");
    let address = inner
      .region_allocator
      .allocate_region_with_hint(hint, size, alignment)?;
    // posix: 子预留只是区间切分; 失败须回滚区间 (原版 CHECK_EQ)
    let Some(reservation) = self.reservation.create_sub_reservation(address, size) else {
      let freed = inner.region_allocator.free_region(address);
      assert_eq!(freed, Some(size), "AllocateSubspace 回滚 FreeRegion");
      return None;
    };
    Some(Box::new(VirtualAddressSubspace::new(
      reservation,
      // SAFETY: 父空间即 self, 存活期覆盖子空间 (子空间由 self 的 allocate_subspace 返回)
      unsafe { ParentSpace::new(self) },
      self.page_size,
      self.allocation_granularity,
      max_page_permissions,
    )))
  }

  fn recommit_pages(&self, address: usize, size: usize, permissions: PagePermissions) -> bool {
    debug_assert_eq!(address % self.page_size, 0);
    debug_assert_eq!(size % self.page_size, 0);
    debug_assert!(permissions.is_subset_of(self.max_page_permissions));
    os_recommit_pages(address, size, to_permission(permissions))
  }

  fn discard_system_pages(&self, address: usize, size: usize) -> bool {
    debug_assert_eq!(address % self.page_size, 0);
    debug_assert_eq!(size % self.page_size, 0);
    os_discard_system_pages(address, size)
  }

  fn decommit_pages(&self, address: usize, size: usize) -> bool {
    debug_assert_eq!(address % self.page_size, 0);
    debug_assert_eq!(size % self.page_size, 0);
    let success = os_decommit_pages(address, size);
    let name = self.name.lock().expect("subspace mutex 不应中毒").clone();
    if success && !name.is_empty() {
      // MAP_FIXED 归档会重置区名, 需重新设置 (原版 CHECK 同款);
      // 与 FreePages 同理只对 [address, address+size) 子区间命名
      assert!(
        os_set_memory_region_name(address, size, &name),
        "DecommitPages SetMemoryRegionName"
      );
    }
    success
  }

  fn set_name(&self, name: &str) -> bool {
    // 原版 CHECK(IsValidMappingName(name)): release 亦执行 (仅字母数字与 '-')
    assert!(
      name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'),
      "非法映射区名: {name}"
    );
    if self.reservation.set_name(name) {
      *self.name.lock().expect("subspace mutex 不应中毒") = name.to_owned();
      true
    } else {
      false
    }
  }

  fn free_subspace(&self, base: usize, size: usize) {
    let mut inner = self.inner.lock().expect("subspace mutex 不应中毒");
    // 原版 CHECK_EQ: 区间回收须在 release 亦执行
    let freed = inner.region_allocator.free_region(base);
    assert_eq!(freed, Some(size), "FreeSubspace FreeRegion");
    // posix: FreeSubReservation 无需操作
  }
}

/// 进程根空间单例 (原版全局使用模式的承接)
pub fn global_virtual_address_space() -> &'static RootVirtualAddressSpace {
  static GLOBAL: OnceLock<RootVirtualAddressSpace> = OnceLock::new();
  GLOBAL.get_or_init(RootVirtualAddressSpace::new)
}
