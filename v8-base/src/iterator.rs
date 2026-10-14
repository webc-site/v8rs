//! v8/src/base/iterator.h 的 1:1 转写
//! C++ iterator 类别标签 → ForwardIteratorTag/BidirectionalIteratorTag 标记类型
//! base::iterator<Category,Type,...> → IteratorBase (关联类型)
//! iterator_range<ForwardIterator> → IteratorRange<I: Cursor> (游标为值语义)
//! TupleIterator/zip → 二目具名化 (C++ 变参模板), 另提供 std::iter::Zip 直通

use std::{
  iter::{Rev, Zip},
  mem::size_of,
  ptr::{null, null_mut},
};

/// 原版 std::forward_iterator_tag
pub struct ForwardIteratorTag;
/// 原版 std::bidirectional_iterator_tag
pub struct BidirectionalIteratorTag;

/// 原版 base::iterator<Category, Type, Diff, Pointer, Reference>
/// difference_type 由 std 侧的 isize 承担
pub trait IteratorBase {
  type Category;
  type ValueType;
  type Reference;
}

/// 前向游标 (C++ forward iterator 的值语义): operator* / operator++
pub trait Cursor: Clone + Copy + PartialEq {
  type Item: Copy;
  /// value_type operator*() const
  fn deref(&self) -> Self::Item;
  /// Iterator& operator++()
  fn advance(&mut self);
}

/// 双向游标: Iterator& operator--()
pub trait BidirectionalCursor: Cursor {
  fn retreat(&mut self);
}

/// 随机访问游标 (原版 Random Access iterators only 的 size/operator[])
pub trait RandomAccessCursor: BidirectionalCursor {
  /// cend() - cbegin() 的端点距离 (self 在前)
  fn distance(&self, other: &Self) -> usize;
}

/// iterator_range<ForwardIterator>
/// 以两个游标封装区间, 像容器一样使用 (原版注释)
#[derive(Debug, Clone, Copy)]
pub struct IteratorRange<I: Cursor> {
  begin_: I,
  end_: I,
}

impl<I: Cursor> IteratorRange<I> {
  /// iterator_range(ForwardIterator begin, ForwardIterator end)
  #[inline]
  pub fn new(begin: I, end: I) -> Self {
    Self {
      begin_: begin,
      end_: end,
    }
  }

  /// iterator_range() : begin_(), end_() (原版值初始化空区间)
  #[inline]
  pub fn default_empty() -> Self
  where
    I: Default,
  {
    Self {
      begin_: I::default(),
      end_: I::default(),
    }
  }

  #[inline]
  pub fn begin(&self) -> I {
    self.begin_
  }
  #[inline]
  pub fn end(&self) -> I {
    self.end_
  }
  #[inline]
  pub fn cbegin(&self) -> I {
    self.begin_
  }
  #[inline]
  pub fn cend(&self) -> I {
    self.end_
  }

  /// bool empty() const
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.cbegin() == self.cend()
  }

  /// range-based for: begin()..end()
  #[inline]
  pub fn iter(&self) -> RangeIter<I> {
    RangeIter {
      current: self.begin_,
      end: self.end_,
    }
  }
}

impl<I: BidirectionalCursor> IteratorRange<I> {
  /// rbegin()/rend(): 原版 std::make_reverse_iterator
  #[inline]
  pub fn rbegin_rend(&self) -> IteratorRange<ReversedCursor<I>> {
    IteratorRange::new(
      ReversedCursor { inner: self.end_ },
      ReversedCursor { inner: self.begin_ },
    )
  }

  /// IterateWithoutLast(t): 空区间原版 DCHECK → 显式 None
  #[inline]
  pub fn iterate_without_last(&self) -> Option<IteratorRange<I>> {
    if self.is_empty() {
      return None;
    }
    let mut new_end = self.end_;
    new_end.retreat();
    Some(IteratorRange::new(self.begin_, new_end))
  }

  /// IterateWithoutFirst(t): 空区间原版 DCHECK → 显式 None
  #[inline]
  pub fn iterate_without_first(&self) -> Option<IteratorRange<I>> {
    if self.is_empty() {
      return None;
    }
    let mut new_begin = self.begin_;
    new_begin.advance();
    Some(IteratorRange::new(new_begin, self.end_))
  }
}

impl<I: RandomAccessCursor> IteratorRange<I> {
  /// difference_type size() const = cend() - cbegin()
  #[inline]
  pub fn size(&self) -> usize {
    self.begin_.distance(&self.end_)
  }

  /// reference operator[](difference_type n) = begin()[n]
  #[inline]
  pub fn index(&self, n: usize) -> I::Item {
    let mut it = self.begin_;
    for _ in 0..n {
      it.advance();
    }
    it.deref()
  }
}

/// Reversed(t): 容器区间倒序 (原版 make_iterator_range(rbegin, rend))
#[inline]
pub fn reversed_range<I: BidirectionalCursor>(
  range: &IteratorRange<I>,
) -> IteratorRange<ReversedCursor<I>> {
  range.rbegin_rend()
}

/// RangeIter: IteratorRange → std::iterator 适配
pub struct RangeIter<I: Cursor> {
  current: I,
  end: I,
}

impl<I: Cursor> Iterator for RangeIter<I> {
  type Item = I::Item;
  #[inline]
  fn next(&mut self) -> Option<I::Item> {
    if self.current == self.end {
      return None;
    }
    let value = self.current.deref();
    self.current.advance();
    Some(value)
  }
}

impl<I: BidirectionalCursor> DoubleEndedIterator for RangeIter<I> {
  #[inline]
  fn next_back(&mut self) -> Option<I::Item> {
    if self.current == self.end {
      return None;
    }
    self.end.retreat();
    Some(self.end.deref())
  }
}

/// ReversedCursor: 反向游标 (原版 std::reverse_iterator)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReversedCursor<I: BidirectionalCursor> {
  inner: I,
}

impl<I: BidirectionalCursor> Cursor for ReversedCursor<I> {
  type Item = I::Item;
  /// reverse_iterator::operator* → (--base).deref()
  #[inline]
  fn deref(&self) -> I::Item {
    let mut tmp = self.inner;
    tmp.retreat();
    tmp.deref()
  }
  /// reverse_iterator::operator++ → 底层游标后退
  #[inline]
  fn advance(&mut self) {
    self.inner.retreat();
  }
}

impl<I: BidirectionalCursor> BidirectionalCursor for ReversedCursor<I> {
  /// reverse_iterator::operator-- → 底层游标前进
  #[inline]
  fn retreat(&mut self) {
    self.inner.advance();
  }
}

/// make_iterator_range(begin, end)
#[inline]
pub fn make_iterator_range<I: Cursor>(begin: I, end: I) -> IteratorRange<I> {
  IteratorRange::new(begin, end)
}

/// Reversed(t): 原版对容器取 std::rbegin/rend; Rust 以 DoubleEnded 迭代器倒序
#[inline]
pub fn reversed<I: DoubleEndedIterator>(it: I) -> Rev<I> {
  it.rev()
}

/// DerefPtrIterator<T>: 指向 "指针数组" 的解引用游标
/// 原版 `T* const* ptr` + `T& operator*()`
/// 手写 Clone/Copy/PartialEq/Eq: derive 会给泛型参数 T 强加 Copy 约束,
/// 而原版只比较指针本身, 与 T 是否 Copy 无关
#[derive(Debug)]
pub struct DerefPtrIterator<T> {
  ptr: *const *mut T,
}

impl<T> Clone for DerefPtrIterator<T> {
  #[inline]
  fn clone(&self) -> Self {
    *self
  }
}

impl<T> Copy for DerefPtrIterator<T> {}

impl<T> PartialEq for DerefPtrIterator<T> {
  /// bool operator==(const DerefPtrIterator& other) — 比指针
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.ptr == other.ptr
  }
}

impl<T> Eq for DerefPtrIterator<T> {}

impl<T> Default for DerefPtrIterator<T> {
  /// 原版值初始化 (T* const* ptr = nullptr)
  #[inline]
  fn default() -> Self {
    Self { ptr: null() }
  }
}

impl<T> DerefPtrIterator<T> {
  /// explicit DerefPtrIterator(T* const* ptr)
  #[inline]
  pub fn new(ptr: *const *mut T) -> Self {
    Self { ptr }
  }

  /// T* const* ptr() — 端点比较/算术面
  #[inline]
  pub fn raw(&self) -> *const *mut T {
    self.ptr
  }

  /// 当前指向的对象指针 (原版 **ptr 的地址侧); 空槽显式 None (禁 panic)
  #[inline]
  pub fn slot(&self) -> Option<*mut T> {
    if self.ptr.is_null() {
      return None;
    }
    // SAFETY: 调用方保证 ptr 落在合法的 T* 数组内 (原版 V8_GSL_POINTER 约束)
    let slot = unsafe { *self.ptr };
    (!slot.is_null()).then_some(slot)
  }
}

impl<T> IteratorBase for DerefPtrIterator<T> {
  type Category = BidirectionalIteratorTag;
  type ValueType = T;
  type Reference = *mut T;
}

/// 游标推进等价 operator++/-- (指针算术)
impl<T> Cursor for DerefPtrIterator<T> {
  type Item = *mut T;
  #[inline]
  fn deref(&self) -> *mut T {
    self.slot().unwrap_or(null_mut())
  }
  #[inline]
  fn advance(&mut self) {
    // SAFETY: 与原版一致, 调用方保证不越过数组末端
    self.ptr = unsafe { self.ptr.add(1) };
  }
}

impl<T> BidirectionalCursor for DerefPtrIterator<T> {
  #[inline]
  fn retreat(&mut self) {
    // SAFETY: 与原版一致, 调用方保证不越过数组首端
    self.ptr = unsafe { self.ptr.sub(1) };
  }
}

impl<T> RandomAccessCursor for DerefPtrIterator<T> {
  #[inline]
  fn distance(&self, other: &Self) -> usize {
    // 指针差值 (原版 cend()-cbegin()), 两指针须属同一数组, 由调用方保证
    (other.ptr as isize - self.ptr as isize) as usize / size_of::<*mut T>()
  }
}

/// TupleIterator<A, B>: 原版变参 TupleIterator 的二目形态
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TupleIterator<A: Cursor, B: Cursor> {
  a: A,
  b: B,
}

impl<A: Cursor, B: Cursor> TupleIterator<A, B> {
  /// explicit TupleIterator(Iterators... its)
  #[inline]
  pub fn new(a: A, b: B) -> Self {
    Self { a, b }
  }
}

impl<A: Cursor, B: Cursor> IteratorBase for TupleIterator<A, B> {
  type Category = BidirectionalIteratorTag;
  type ValueType = (A::Item, B::Item);
  type Reference = (A::Item, B::Item);
}

impl<A: Cursor, B: Cursor> Cursor for TupleIterator<A, B> {
  /// value_type = std::tuple<reference...> → Rust 元组
  type Item = (A::Item, B::Item);
  #[inline]
  fn deref(&self) -> Self::Item {
    (self.a.deref(), self.b.deref())
  }
  #[inline]
  fn advance(&mut self) {
    self.a.advance();
    self.b.advance();
  }
}

impl<A: BidirectionalCursor, B: BidirectionalCursor> BidirectionalCursor for TupleIterator<A, B> {
  #[inline]
  fn retreat(&mut self) {
    self.a.retreat();
    self.b.retreat();
  }
}

/// zip(containers...): 多容器并行遍历 (原版二目情形, 端点由调用方给出)
#[inline]
pub fn zip<A: Cursor, B: Cursor>(
  begin_a: A,
  end_a: A,
  begin_b: B,
  end_b: B,
) -> IteratorRange<TupleIterator<A, B>> {
  IteratorRange::new(
    TupleIterator::new(begin_a, begin_b),
    TupleIterator::new(end_a, end_b),
  )
}

/// 容器已实现 std::iterator 时的等价入口 (原版 zip 的惯用替代)
#[inline]
pub fn zip_iters<A: Iterator, B: Iterator>(a: A, b: B) -> Zip<A, B> {
  a.zip(b)
}
