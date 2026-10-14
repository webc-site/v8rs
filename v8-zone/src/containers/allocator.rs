//! 原版 src/zone/zone-allocator.h 转写
//!
//! C++ allocator 概念在 Rust 无语言对应, 此处保留显式 allocate/deallocate
//! 形态供容器与单测使用 (行为 1:1, 含 RecyclingZoneAllocator 的头部空闲链)。

use std::{cell::Cell, marker::PhantomData, mem, ptr};

use crate::{error::Result, internal::bits::ALIGNMENT, zone::core::Zone};

/// 原版 ZoneAllocator<T>
#[derive(Clone, Copy)]
pub struct ZoneAllocator<'z, 'a, T> {
  zone: &'z Zone<'a>,
  _p: PhantomData<T>,
}

impl<'z, 'a, T> ZoneAllocator<'z, 'a, T> {
  /// 原版 explicit ZoneAllocator(Zone*)
  pub fn new(zone: &'z Zone<'a>) -> ZoneAllocator<'z, 'a, T> {
    ZoneAllocator {
      zone,
      _p: PhantomData,
    }
  }

  /// 原版 zone()
  pub fn zone(&self) -> &'z Zone<'a> {
    self.zone
  }

  /// 原版 allocate(length): 返回 length 个 T 的未初始化存储。
  /// D5 修复: 原版经 zone->AllocateArray<T> (static_assert alignof(T) <=
  /// kAlignmentInBytes), Rust 版转调 allocate_array 获得同等对齐检查
  pub fn allocate(&self, length: usize) -> Result<*mut T> {
    let storage = self.zone.allocate_array::<T>(length)?;
    Ok(storage.as_mut_ptr().cast::<T>())
  }

  /// 原版 deallocate(p, length)
  ///
  /// # Safety
  /// p 必须由本分配器 allocate(length) 返回且内容已不再被引用
  pub unsafe fn deallocate(&self, p: *mut T, length: usize) {
    if mem::size_of::<T>() == 0 || length == 0 {
      return;
    }
    self.zone.delete(p, length);
  }
}

impl<T> PartialEq for ZoneAllocator<'_, '_, T> {
  /// 原版 operator==: 同一 zone 即相等
  fn eq(&self, other: &Self) -> bool {
    ptr::eq(self.zone, other.zone)
  }
}

impl<T> Eq for ZoneAllocator<'_, '_, T> {}

/// 原版 RecyclingZoneAllocator 的头部空闲块
#[repr(C)]
struct FreeBlock {
  next: *mut FreeBlock,
  size: usize,
}

/// 原版 RecyclingZoneAllocator<T>: 头部 O(1) 复用的回收分配器
pub struct RecyclingZoneAllocator<'z, 'a, T> {
  zone: &'z Zone<'a>,
  free_list: Cell<*mut FreeBlock>,
  _p: PhantomData<T>,
}

impl<'z, 'a, T> RecyclingZoneAllocator<'z, 'a, T> {
  /// 原版 explicit RecyclingZoneAllocator(Zone*)
  pub fn new(zone: &'z Zone<'a>) -> RecyclingZoneAllocator<'z, 'a, T> {
    RecyclingZoneAllocator {
      zone,
      free_list: Cell::new(ptr::null_mut()),
      _p: PhantomData,
    }
  }

  /// 原版 zone() (继承自 ZoneAllocator)
  pub fn zone(&self) -> &'z Zone<'a> {
    self.zone
  }

  /// 原版 allocate(n): 仅查看链头 (链头 >= 其余块), O(1)
  pub fn allocate(&self, n: usize) -> Result<*mut T> {
    let head = self.free_list.get();
    // SAFETY: 链头块由本分配器持有, 内容仅为 FreeBlock 簿记
    if !head.is_null() && unsafe { (*head).size } >= n {
      self.free_list.set(unsafe { (*head).next });
      return Ok(head.cast::<T>());
    }
    self.base_allocate(n)
  }

  /// 原版 deallocate(p, n)
  ///
  /// # Safety
  /// p 必须由本分配器 allocate(n) 返回且内容已不再被引用
  pub unsafe fn deallocate(&self, p: *mut T, n: usize) {
    let Some(total_bytes) = mem::size_of::<T>().checked_mul(n) else {
      return;
    };
    if total_bytes < mem::size_of::<FreeBlock>() {
      return;
    }
    let head = self.free_list.get();
    // 仅当不链头更小时入链, 保证 allocate 只看链头 O(1)
    if head.is_null() || unsafe { (*head).size } <= n {
      // SAFETY: 块大小 >= FreeBlock, 簿记写入块内 (原版将链表存入块本身)
      unsafe {
        ptr::write(
          p.cast::<FreeBlock>(),
          FreeBlock {
            next: head,
            size: n,
          },
        )
      };
      self.free_list.set(p.cast::<FreeBlock>());
    }
  }

  /// 原版 ZoneAllocator<T>::allocate (D5: 同样经 allocate_array 对齐检查)
  fn base_allocate(&self, n: usize) -> Result<*mut T> {
    let storage = self.zone.allocate_array::<T>(n)?;
    Ok(storage.as_mut_ptr().cast::<T>())
  }
}

/// 对齐断言: FreeBlock 布局不超过 zone 对齐能力
const _: () = {
  assert!(mem::align_of::<FreeBlock>() <= ALIGNMENT);
};

/// 原版 using ZoneBoolAllocator = ZoneAllocator<bool> (zone-allocator.h:105)
pub type ZoneBoolAllocator<'z, 'a> = ZoneAllocator<'z, 'a, bool>;

/// 原版 using ZoneIntAllocator = ZoneAllocator<int> (zone-allocator.h:106)
pub type ZoneIntAllocator<'z, 'a> = ZoneAllocator<'z, 'a, i32>;
