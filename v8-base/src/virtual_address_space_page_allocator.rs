//! v8/src/base/virtual-address-space-page-allocator.h/cc 的转写
//! 把 VirtualAddressSpace (未来内存管理 API) 桥接为 PageAllocator (现行 API)
//! 差异登记: 追踪原分配大小的 resized_allocations 以 BTreeMap 承接 (原 unordered_map)

use std::{collections::BTreeMap, sync::Mutex};

use crate::{
  page_allocator::{PageAllocator, Permission},
  virtual_address_space::{PagePermissions, VirtualAddressSpace},
};

/// Permission → PagePermissions (原版 ConvertPageAllocatorPermission;
/// NoAccessWillJitLater 原版 UNREACHABLE → 同款全构建致命, 不静默降权)
fn convert_permission(permission: Permission) -> PagePermissions {
  match permission {
    Permission::NoAccess => PagePermissions::NO_ACCESS,
    Permission::Read => PagePermissions::READ,
    Permission::ReadWrite => PagePermissions::READ_WRITE,
    Permission::ReadWriteExecute => PagePermissions::READ_WRITE_EXECUTE,
    Permission::ReadExecute => PagePermissions::READ_EXECUTE,
    Permission::NoAccessWillJitLater => {
      unreachable!("VAS 不接受 NoAccessWillJitLater")
    }
  }
}

/// VirtualAddressSpace → PageAllocator 桥接适配器
/// 调用方须保证被包装的空间存活于本实例整个生命周期 (原版同款契约)
pub struct VirtualAddressSpacePageAllocator<'a> {
  vas: &'a dyn VirtualAddressSpace,
  /// resize 过的分配 → 原始大小 (FreePages 时还原, 原版同款机制)
  resized_allocations: Mutex<BTreeMap<usize, usize>>,
}

impl<'a> VirtualAddressSpacePageAllocator<'a> {
  /// 原版构造
  #[must_use]
  pub fn new(vas: &'a dyn VirtualAddressSpace) -> Self {
    Self {
      vas,
      resized_allocations: Mutex::new(BTreeMap::new()),
    }
  }
}

impl PageAllocator for VirtualAddressSpacePageAllocator<'_> {
  fn allocate_page_size(&self) -> usize {
    self.vas.allocation_granularity()
  }

  fn commit_page_size(&self) -> usize {
    self.vas.page_size()
  }

  fn set_random_mmap_seed(&self, seed: i64) {
    self.vas.set_random_seed(seed);
  }

  fn get_random_mmap_addr(&self) -> usize {
    self.vas.random_page_address()
  }

  fn allocate_pages(
    &self,
    hint: usize,
    length: usize,
    alignment: usize,
    permissions: Permission,
  ) -> Option<usize> {
    self
      .vas
      .allocate_pages(hint, length, alignment, convert_permission(permissions))
  }

  fn free_pages(&self, address: usize, length: usize) -> bool {
    let mut resized = self
      .resized_allocations
      .lock()
      .expect("vas 适配器 mutex 不应中毒");
    // 曾被 resize 过则用原始大小释放
    let length = resized.remove(&address).unwrap_or(length);
    self.vas.free_pages(address, length);
    true
  }

  fn release_pages(&self, address: usize, length: usize, new_length: usize) -> bool {
    // VAS 不支持真正的部分释放 (Windows 占位映射限制), 以归档尾部页模拟;
    // 原始大小记录在案, 供后续 FreePages 使用 (原版注释同款)
    debug_assert!(new_length <= length);
    let mut resized = self
      .resized_allocations
      .lock()
      .expect("vas 适配器 mutex 不应中毒");
    // 原版 unordered_map::insert: 已存在时不覆盖 (保留首次记录, 二次 resize
    // 属调用方契约破坏, 仅 debug 提示)
    use std::collections::btree_map::Entry;
    match resized.entry(address) {
      Entry::Vacant(v) => {
        v.insert(length);
      }
      Entry::Occupied(_) => debug_assert!(false, "同一分配不允许二次 resize"),
    }
    // 原版 CHECK(vas_->DecommitPages(...)): release 亦须致命
    assert!(
      self
        .vas
        .decommit_pages(address + new_length, length - new_length),
      "ReleasePages DecommitPages"
    );
    true
  }

  fn set_permissions(&self, address: usize, length: usize, permissions: Permission) -> bool {
    self
      .vas
      .set_page_permissions(address, length, convert_permission(permissions))
  }

  fn recommit_pages(&self, address: usize, length: usize, permissions: Permission) -> bool {
    self
      .vas
      .recommit_pages(address, length, convert_permission(permissions))
  }

  fn discard_system_pages(&self, address: usize, size: usize) -> bool {
    self.vas.discard_system_pages(address, size)
  }

  fn decommit_pages(&self, address: usize, size: usize) -> bool {
    self.vas.decommit_pages(address, size)
  }

  fn seal_pages(&self, _address: usize, _size: usize) -> bool {
    // 原版同款: 桥接层不支持封页
    false
  }
}
