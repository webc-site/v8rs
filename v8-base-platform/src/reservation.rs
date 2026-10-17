//! AddressSpaceReservation：对应 V8 `platform.h::AddressSpaceReservation`。
//!
//! POSIX 实现：
//! - `base` 由一次 `mmap(PROT_NONE, MAP_NORESERVE)` 预留。
//! - `Allocate` 在 base 内以 `MAP_FIXED` 提交实际权限；
//! - `Free` 以 `MAP_FIXED` PROT_NONE 覆盖；
//! - `SetPermissions/Recommit/Discard/Decommit` 委托给 `OS` 中同名方法。
//!
//! 偏差：`set_name` 已实现（委托 `OS::set_memory_region_name` 的
//! `prctl(PR_SET_VMA_ANON_NAME)`，仅 Linux ≥5.17 生效，其他平台返回 false，与原版一致）。

use std::ffi::c_void;

use crate::{
  error::Result,
  os::{MemoryPermission, OS},
};

/// 与 V8 `AddressSpaceReservation` 对应。
#[derive(Debug, Clone, Copy)]
pub struct AddressSpaceReservation {
  base: *mut c_void,
  size: usize,
}

// SAFETY: base/size 是不可变值语义；跨线程共享时仅通过 OS API 访问映射，无 UB。
unsafe impl Send for AddressSpaceReservation {}
// SAFETY: 同上。
unsafe impl Sync for AddressSpaceReservation {}

impl AddressSpaceReservation {
  #[inline]
  pub fn base(&self) -> *mut c_void {
    self.base
  }
  #[inline]
  pub fn size(&self) -> usize {
    self.size
  }

  /// 对应 V8 `Contains(region_addr, region_size)`。
  pub fn contains(&self, region_addr: *mut c_void, region_size: usize) -> bool {
    let base = self.base as usize;
    let r = region_addr as usize;
    r >= base && r.saturating_add(region_size) <= base.saturating_add(self.size)
  }

  /// 对应 V8 `OS::CreateAddressSpaceReservation(hint, size, alignment,
  /// max_permission)`（platform-posix.cc:710-729）：整块以 PROT_NONE 预留
  /// （`max_permission == ReadWriteExecute` 时按 darwin MAP_JIT 语义走
  /// `NoAccessWillJitLater`，失败回退 `NoAccess` 重试）；对齐由 `OS::allocate`
  /// 的超额分配 + 修剪保证。`max_permission` 只是上限记录，预留本身不可访问。
  ///
  /// # Safety
  /// 返回值持有 mmap 的裸指针；调用方须负责 `free_reservation`。
  pub unsafe fn create(
    hint: *mut c_void,
    size: usize,
    alignment: usize,
    max_permission: MemoryPermission,
  ) -> Result<Self> {
    let page = OS::commit_page_size();
    debug_assert_eq!(0, size % page, "DCHECK_EQ(0, size % page)");
    let permission = if max_permission == MemoryPermission::ReadWriteExecute {
      MemoryPermission::NoAccessWillJitLater
    } else {
      MemoryPermission::NoAccess
    };
    // SAFETY: OS::allocate 负责 hint 对齐、超额分配与两端修剪。
    let mut p = unsafe { OS::allocate(hint, size, alignment, permission) };
    if p.is_err() && permission == MemoryPermission::NoAccessWillJitLater {
      // MAP_JIT 失败（如旧版 macOS）时按原版回退 NoAccess 重试。
      p = unsafe { OS::allocate(hint, size, alignment, MemoryPermission::NoAccess) };
    }
    let base = p?;
    Ok(Self { base, size })
  }

  /// 释放整个 reservation；对应 V8 `OS::FreeAddressSpaceReservation`。
  ///
  /// # Safety
  /// `self` 必须来自 `create` 且未被释放。
  pub unsafe fn free_reservation(self) -> Result<()> {
    // SAFETY: 由调用方保证。
    unsafe { OS::free(self.base, self.size) }
  }

  /// 对应 V8 `AddressSpaceReservation::Allocate(address, size, access)`
  /// （platform-posix.cc:1150-1161）：区域已在预留映射内，仅需放开权限；
  /// `kNoAccess` 直接成功（避免 DiscardSystemPages 副作用）。
  ///
  /// # Safety
  /// `address`+`size` 必须完全位于本 reservation 且页对齐。
  pub unsafe fn allocate(
    &self,
    address: *mut c_void,
    size: usize,
    access: MemoryPermission,
  ) -> Result<bool> {
    if !self.contains(address, size) {
      return Ok(false);
    }
    if access == MemoryPermission::NoAccess {
      return Ok(true);
    }
    // SAFETY: 已确认在 base 范围内。
    let r = unsafe { OS::set_permissions(address, size, access) };
    Ok(r.is_ok())
  }

  /// 对应 V8 `Free(address, size)`：DecommitPages 覆盖回 PROT_NONE。
  ///
  /// # Safety
  /// 见 `allocate`。
  pub unsafe fn free(&self, address: *mut c_void, size: usize) -> Result<bool> {
    if !self.contains(address, size) {
      return Ok(false);
    }
    // SAFETY: 已确认；原版透传 DecommitPages 的成败。
    let r = unsafe { OS::decommit_pages(address, size) };
    Ok(r.is_ok())
  }

  /// 对应 V8 `SetPermissions(address, size, access)`。
  ///
  /// # Safety
  /// 见 `allocate`。
  pub unsafe fn set_permissions(
    &self,
    address: *mut c_void,
    size: usize,
    access: MemoryPermission,
  ) -> Result<bool> {
    if !self.contains(address, size) {
      return Ok(false);
    }
    // SAFETY: 已确认。
    let r = unsafe { OS::set_permissions(address, size, access) };
    Ok(r.is_ok())
  }

  /// 对应 V8 `RecommitPages(address, size, access)`。
  ///
  /// # Safety
  /// 见 `allocate`。
  pub unsafe fn recommit_pages(
    &self,
    address: *mut c_void,
    size: usize,
    access: MemoryPermission,
  ) -> Result<bool> {
    if !self.contains(address, size) {
      return Ok(false);
    }
    // SAFETY: 已确认。
    let r = unsafe { OS::recommit_pages(address, size, access) };
    Ok(r.is_ok())
  }

  /// 对应 V8 `DiscardSystemPages(address, size)`。
  ///
  /// # Safety
  /// 见 `allocate`。
  pub unsafe fn discard_system_pages(&self, address: *mut c_void, size: usize) -> Result<bool> {
    if !self.contains(address, size) {
      return Ok(false);
    }
    // SAFETY: 已确认。
    let r = unsafe { OS::discard_system_pages(address, size) };
    Ok(r.is_ok())
  }

  /// 对应 V8 `DecommitPages(address, size)`。
  ///
  /// # Safety
  /// 见 `allocate`。
  pub unsafe fn decommit_pages(&self, address: *mut c_void, size: usize) -> Result<bool> {
    if !self.contains(address, size) {
      return Ok(false);
    }
    // SAFETY: 已确认。
    let r = unsafe { OS::decommit_pages(address, size) };
    Ok(r.is_ok())
  }

  /// 对应 V8 `CreateSubReservation(address, size, max_permission)`。
  ///
  /// # Safety
  /// 见 `allocate`；子 reservation 与父共享同一 mmap，父释放前不得再次释放子。
  pub unsafe fn create_sub_reservation(
    &self,
    address: *mut c_void,
    size: usize,
    _max_permission: MemoryPermission,
  ) -> Result<Option<Self>> {
    if !self.contains(address, size) {
      return Ok(None);
    }
    Ok(Some(Self {
      base: address,
      size,
    }))
  }

  /// 对应 V8 `FreeSubReservation(reservation)`：POSIX 下子预留空间包含于父空间，做基本指针健全性核验后安全回收。
  pub fn free_sub_reservation(reservation: Self) -> bool {
    !reservation.base.is_null() && reservation.size > 0
  }

  /// 对应 V8 `AddressSpaceReservation::SetName(name)`；委托 `OS::SetMemoryRegionName`。
  /// Linux ≥5.17 生效；其他平台返回 false。
  pub fn set_name(&self, name: &str) -> bool {
    // SAFETY: `base`/`size` 由本 reservation 持有；name 由调用方借用。
    unsafe { OS::set_memory_region_name(self.base, self.size, name) }
  }
}
