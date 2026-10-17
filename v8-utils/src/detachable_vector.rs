// 由 tool/src/utils/consts.js 从 v8/src/utils/detachable-vector.h;src/utils/detachable-vector.cc 生成; 修订就地改, 复跑覆盖

//! 可分离向量, 移植自 v8/src/utils/detachable-vector。
//!
//! 上游手工管理裸指针 backing store, detach() 仅切断引用不释放内存
//! (所有权转移给位拷贝出的另一实例); Rust 版以 `Vec` + 显式 capacity 字段
//! 承载同一增长/收缩契约 (2 倍扩容 / 半量以下收缩 / detach 移交所有权)。

use std::mem::take;

/// 初始容量 (kMinimumCapacity)
pub const MINIMUM_CAPACITY: usize = 8;

/// `DetachableVector<T>` (detachable-vector.h:37)。
///
/// `capacity` 与 Vec 内部分配解耦显式管理, 对齐原版 push/shrink 的精确容量语义
/// (push 满 2 倍; shrink 仅当 new_capacity < capacity/2)。
#[derive(Debug, Clone)]
pub struct DetachableVector<T> {
  data: Vec<T>,
  capacity: usize,
}

impl<T> Default for DetachableVector<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> DetachableVector<T> {
  /// 默认构造: 零容量零元素 (detachable-vector.h:41)。
  pub const fn new() -> Self {
    Self {
      data: Vec::new(),
      capacity: 0,
    }
  }

  /// `push_back` (detachable-vector.h:44): 满则 max(kMinimumCapacity, 2*capacity)。
  pub fn push_back(&mut self, value: T) {
    if self.data.len() == self.capacity {
      let new_capacity = (2 * self.capacity).max(MINIMUM_CAPACITY);
      self.data.reserve_exact(new_capacity - self.data.len());
      self.capacity = new_capacity;
    }
    self.data.push(value);
  }

  /// `free` (detachable-vector.h:52): 释放 backing store 并清引用。
  pub fn free(&mut self) {
    self.data = Vec::new();
    self.capacity = 0;
  }

  /// `detach` 的 Rust 等价物: 显式移交 backing store 所有权 (位拷贝语义)。
  pub fn detach_into(&mut self) -> Vec<T> {
    self.capacity = 0;
    take(&mut self.data)
  }

  /// `pop_back` (detachable-vector.h:26): 空向量调用为无操作边界。
  pub fn pop_back(&mut self) {
    self.data.pop();
  }

  /// `at(i)` (detachable-vector.h:59): i < size 调用方契约 (原版 DCHECK_LT)。
  #[must_use]
  pub fn at(&self, i: usize) -> &T {
    debug_assert!(i < self.data.len());
    &self.data[i]
  }

  /// `back()` (detachable-vector.h:62): 非空调用方契约。
  #[must_use]
  pub fn back(&self) -> &T {
    self.at(self.data.len() - 1)
  }

  /// `front()` (detachable-vector.h:63): 非空调用方契约。
  #[must_use]
  pub fn front(&self) -> &T {
    self.at(0)
  }

  /// `shrink_to_fit` (detachable-vector.h:65): new = max(size, kMinimumCapacity),
  /// 仅当 new < capacity/2 才真正收缩。
  pub fn shrink_to_fit(&mut self) {
    let new_capacity = self.data.len().max(MINIMUM_CAPACITY);
    if new_capacity < self.capacity / 2 {
      self.data.shrink_to_fit();
      // reserve_exact 后 shrink_to_fit 收缩到 len; 显式容量记 new_capacity
      self.data.reserve_exact(new_capacity - self.data.len());
      self.capacity = new_capacity;
    }
  }

  /// `capacity()` (detachable-vector.h:27)。
  #[must_use]
  pub const fn capacity(&self) -> usize {
    self.capacity
  }

  /// `size()` (detachable-vector.h:28)。
  #[must_use]
  pub fn size(&self) -> usize {
    self.data.len()
  }

  /// `empty()` (detachable-vector.h:29)。
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }
}
