//! v8/src/base/unique-array.h 的 1:1 转写
//! `UniqueArray<T>`: 记录长度的动态分配数组 (std::unique_ptr<T[]> + size)。
//! C++ 移动构造/赋值 → Rust 原生 move; "other 置空" 语义由 Default (空数组)
//! 加 `std::mem::take` 等价承担 (登记差异: 编译器自动生成的 move 不会清空 moved-from,
//! Rust 侧由借用检查禁止再读; 需要可观察的置空时用 take)。
//!
//! `UniqueArray<T>` → `UniqueArray<const T>` 的协变转换: Rust 以只读视图方法
//! (as_slice / as_vector) 覆盖, 不做类型形参转写。

use std::{
  iter::Rev,
  mem::{MaybeUninit, take, transmute},
  slice,
};

use crate::vector::{ConstVector, Vector};

/// 原版 `UniqueArray<T>` (不可复制; move-only)
pub struct UniqueArray<T> {
  /// unique_ptr<T[]> data_ (null 表示空数组)
  data: Option<Box<[T]>>,
  /// size_t length_
  length: usize,
}

impl<T> UniqueArray<T> {
  /// static UniqueArray New(size_t size) — 元素值初始化 (Default)
  pub fn new_with_default(size: usize) -> Self
  where
    T: Default,
  {
    if size == 0 {
      return Self::default();
    }
    let boxed: Box<[T]> = (0..size)
      .map(|_| T::default())
      .collect::<Vec<T>>()
      .into_boxed_slice();
    Self {
      length: boxed.len(),
      data: Some(boxed),
    }
  }

  /// static UniqueArray New(size_t size, U init) — 全量填充
  pub fn new_filled(size: usize, init: T) -> Self
  where
    T: Clone,
  {
    if size == 0 {
      return Self::default();
    }
    let boxed = vec![init; size].into_boxed_slice();
    Self {
      length: boxed.len(),
      data: Some(boxed),
    }
  }

  /// static UniqueArray NewForOverwrite(size_t size) — 默认初始化 (原版不写入有效值)。
  /// 对齐原版: 分配后不构造元素, 读取前必须逐槽覆盖写入。
  ///
  /// # Safety
  /// 调用方在读取或移交任何元素前必须先写入该元素; 元素类型若不可平凡析构,
  /// 未在 Drop 前全部初始化会泄漏/未定义 (原版仅限 trivially_copyable 用法)。
  pub unsafe fn new_for_overwrite(size: usize) -> Self {
    if size == 0 {
      return Self::default();
    }
    let buf: Box<[MaybeUninit<T>]> = (0..size)
      .map(|_| MaybeUninit::uninit())
      .collect::<Vec<_>>()
      .into_boxed_slice();
    // SAFETY: Box<[MaybeUninit<T>]> 与 Box<[T]> 大小/对齐/布局一致
    // (MaybeUninit 与 T 同尺寸同对齐), 初始化契约转嫁给调用方 (见 fn Safety)
    let data: Box<[T]> = unsafe { transmute::<Box<[MaybeUninit<T>]>, Box<[T]>>(buf) };
    Self {
      length: data.len(),
      data: Some(data),
    }
  }

  /// static UniqueArray CopiedFrom(std::span<const T>)
  pub fn copied_from(src: &[T]) -> Self
  where
    T: Clone,
  {
    // 逐元素 clone 到定长 boxed slice (base::Copy 的带所有权等价)
    let boxed: Box<[T]> = src.to_vec().into_boxed_slice();
    Self {
      length: boxed.len(),
      data: Some(boxed),
    }
  }

  /// size_t size()
  #[inline]
  pub fn size(&self) -> usize {
    self.length
  }

  /// bool empty()
  #[inline]
  pub fn empty(&self) -> bool {
    self.length == 0
  }

  /// 原版 operator== (std::nullptr_t): data_ == nullptr
  #[inline]
  pub fn is_null(&self) -> bool {
    self.data.is_none()
  }

  /// T* begin()/end() → 迭代器
  #[inline]
  pub fn iter(&self) -> slice::Iter<'_, T> {
    self.as_slice().iter()
  }

  /// rbegin()/rend()
  #[inline]
  pub fn riter(&self) -> Rev<slice::Iter<'_, T>> {
    self.as_slice().iter().rev()
  }

  /// T* data()
  #[inline]
  pub fn data(&self) -> *const T {
    self.as_slice().as_ptr()
  }

  /// std::span<T> as_span() → 只读切片视图 (可变视图见 as_mut_slice)
  #[inline]
  pub fn as_slice(&self) -> &[T] {
    match &self.data {
      Some(b) => b,
      None => &[],
    }
  }

  #[inline]
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    match &mut self.data {
      Some(b) => b,
      None => &mut [],
    }
  }

  /// T& operator[](size_t index) const — DCHECK_LT → 越界返回 None (禁 panic)
  #[inline]
  pub fn at(&self, index: usize) -> Option<&T> {
    debug_assert!(index < self.length);
    self.as_slice().get(index)
  }

  /// 原版 `Vector<T> as_vector() const` (const 返回可写视图是 C++ 特例);
  /// Rust 以 &mut 借用呈现读写视图
  #[inline]
  pub fn as_mut_vector(&mut self) -> Vector<'_, T> {
    Vector::new(self.as_mut_slice())
  }

  /// 原版 const 侧 `Vector<const T> as_vector() const`
  #[inline]
  pub fn as_vector(&self) -> ConstVector<'_, T> {
    ConstVector::new(self.as_slice())
  }

  /// std::unique_ptr<T[]> ReleaseData() — 移交堆数据, 自身变空
  pub fn release_data(&mut self) -> Option<Box<[T]>> {
    self.length = 0;
    take(&mut self.data)
  }
}

impl<T> Default for UniqueArray<T> {
  /// UniqueArray() = default
  fn default() -> Self {
    Self {
      data: None,
      length: 0,
    }
  }
}

/// 原版 `bool operator==(std::nullptr_t)` 的测试友好等价: 切片判等
impl<T: PartialEq> PartialEq<[T]> for UniqueArray<T> {
  fn eq(&self, other: &[T]) -> bool {
    self.as_slice() == other
  }
}

impl<T: PartialEq> PartialEq for UniqueArray<T> {
  fn eq(&self, other: &Self) -> bool {
    self.as_slice() == other.as_slice()
  }
}

/// inline UniqueArray<T> UniqueCopyOf(const T* data, size_t size)
pub fn unique_copy_of<T: Clone>(src: &[T]) -> UniqueArray<T> {
  UniqueArray::copied_from(src)
}
