//! 原版 v8/src/base/address-region.h — 地址区间值类型

use std::cmp::Ordering;

/// AddressRegion: 起始地址 + 大小 的区间
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AddressRegion {
  address: usize,
  size: usize,
}

impl AddressRegion {
  #[must_use]
  pub const fn new(address: usize, size: usize) -> Self {
    Self { address, size }
  }

  #[must_use]
  pub const fn begin(&self) -> usize {
    self.address
  }

  #[must_use]
  pub const fn end(&self) -> usize {
    // 原版 address_ + size_ 为无符号回绕; wrapping 对齐避免 debug 溢出 panic
    self.address.wrapping_add(self.size)
  }

  #[must_use]
  pub const fn size(&self) -> usize {
    self.size
  }

  pub fn set_size(&mut self, size: usize) {
    self.size = size;
  }

  #[must_use]
  pub const fn is_empty(&self) -> bool {
    self.size == 0
  }

  #[must_use]
  pub const fn contains_address(&self, address: usize) -> bool {
    address.wrapping_sub(self.begin()) < self.size()
  }

  #[must_use]
  pub const fn contains_region(&self, address: usize, size: usize) -> bool {
    let offset = address.wrapping_sub(self.begin());
    // offset + size 可能回绕 (原版无符号语义), 以 wrapping 对齐
    offset < self.size && offset.wrapping_add(size) <= self.size
  }

  #[must_use]
  pub const fn contains(&self, region: Self) -> bool {
    self.contains_region(region.address, region.size)
  }

  /// GetOverlap — 与原版相同: overlap_end 取 max(start, min(end, other.end))
  #[must_use]
  pub const fn get_overlap(&self, region: Self) -> Self {
    let overlap_start = if self.begin() > region.begin() {
      self.begin()
    } else {
      region.begin()
    };
    let min_end = if self.end() < region.end() {
      self.end()
    } else {
      region.end()
    };
    let overlap_end = if overlap_start > min_end {
      overlap_start
    } else {
      min_end
    };
    Self::new(overlap_start, overlap_end - overlap_start)
  }

  /// 从切片构造 (对应 AddressRegionOf(ptr, size))
  #[must_use]
  pub fn of_slice<T>(s: &[T]) -> Self {
    Self::new(s.as_ptr() as usize, size_of_val(s))
  }
}

/// StartAddressLess — 按起始地址比较的 comparator (可用于排序)
#[derive(Debug, Clone, Copy)]
pub struct StartAddressLess;

impl StartAddressLess {
  #[must_use]
  pub fn is_less(a: AddressRegion, b: AddressRegion) -> bool {
    a.begin() < b.begin()
  }

  #[must_use]
  pub fn compare(a: &AddressRegion, b: &AddressRegion) -> Ordering {
    a.begin().cmp(&b.begin())
  }
}
