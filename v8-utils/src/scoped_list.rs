// 由 tool/src/utils/consts.js 从 v8/src/utils/scoped-list.h 生成; 修订就地改, 复跑覆盖

//! 作用域列表, 移植自 v8/src/utils/scoped-list。
//!
//! 共享外层 `Vec` backing 的区间视图: 析构 (Rewind) 把 backing 截回 start;
//! 内层作用域经 [`ScopedList::scope`] 开启 — keep=true 即上游 MergeInto(parent)
//! (区间并入外层), keep=false 即内层析构回卷。
//!
//! 上游 "外层在有内层存活时禁止 Add" 的 DCHECK 契约, 由闭包借用检查编译期承担。

use std::slice::Iter;

/// 作用域列表 (ScopedList<T>)
///
/// 上游 TBacking 允许 void* 位存指针族; Rust 版直接以 T 槽位承载
/// (指针族用 `*mut T` 实例化即可, 语义等价)。
pub struct ScopedList<'a, T> {
  buffer: &'a mut Vec<T>,
  start: usize,
  end: usize,
  /// 已并入外层 (MergeInto), 析构不再回卷
  merged: bool,
}

impl<'a, T> ScopedList<'a, T> {
  /// ScopedList(buffer): 接管 backing 尾部空区
  pub fn new(buffer: &'a mut Vec<T>) -> Self {
    let start = buffer.len();
    ScopedList {
      buffer,
      start,
      end: start,
      merged: false,
    }
  }

  /// 开启内层作用域并以 `f` 填充。
  /// keep=false: 内层析构回卷 (上游默认析构 Rewind);
  /// keep=true: 区间并入本列表 (上游 `inner.MergeInto(this)`), end 推进到内层尾。
  /// 内层存活期间本列表不可访问 (上游 "外层禁 Add" 契约的编译期强化)。
  pub fn scope<R>(&mut self, keep: bool, f: impl FnOnce(&mut ScopedList<'_, T>) -> R) -> R {
    let mut inner = ScopedList::new(self.buffer);
    let ret = f(&mut inner);
    if keep {
      // 上游 MergeInto: parent->end_ = end_; start_ = end_ (内层归零, 不回卷)
      self.end = inner.end;
      inner.merged = true;
    }
    ret
    // keep=false 时 inner 析构回卷 (Drop → Rewind)
  }

  /// Rewind: 截回 start (析构函数语义)
  pub fn rewind(&mut self) {
    debug_assert_eq!(self.buffer.len(), self.end);
    self.buffer.truncate(self.start);
    self.end = self.start;
  }

  /// length
  pub fn length(&self) -> usize {
    self.end - self.start
  }

  /// at(i) 常量引用
  pub fn at(&self, i: usize) -> &T {
    let index = self.start + i;
    debug_assert!(index < self.buffer.len());
    &self.buffer[index]
  }

  /// at(i) 可变引用
  pub fn at_mut(&mut self, i: usize) -> &mut T {
    let index = self.start + i;
    debug_assert!(index < self.buffer.len());
    &mut self.buffer[index]
  }

  /// ToConstVector: 区间切片
  pub fn as_slice(&self) -> &[T] {
    &self.buffer[self.start..self.end]
  }

  /// Add: 尾加 (无内层作用域占位, 即 buffer 尾 == end)
  pub fn add(&mut self, value: T) {
    debug_assert_eq!(self.buffer.len(), self.end);
    self.buffer.push(value);
    self.end += 1;
  }

  /// AddAll: 尾加整段
  pub fn add_all(&mut self, list: &[T])
  where
    T: Clone,
  {
    debug_assert_eq!(self.buffer.len(), self.end);
    self.buffer.extend_from_slice(list);
    self.end += list.len();
  }

  /// begin/end 迭代
  pub fn iter(&self) -> Iter<'_, T> {
    self.as_slice().iter()
  }
}

impl<T> Drop for ScopedList<'_, T> {
  fn drop(&mut self) {
    if !self.merged {
      self.rewind();
    }
  }
}
