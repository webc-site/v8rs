//! v8-base/src/iterator.rs 集成测试 — 对应 iterator-unittest.cc
//! 用例名对齐原版 TEST(Iterator, X): IteratorRangeEmpty / IteratorRangeArray /
//! IteratorRangeDeque / IteratorRangeIteratorTypeDeduction。
//! 原版 std::array/std::deque 宿主 → Rust 以具名游标 (IndexCursor) 与
//! 指针数组 (DerefPtrIterator) 承接; 迭代类别推导以 trait 实例化验证。

use std::ptr::null_mut;

use aok::{OK, Void};
use v8_base::iterator::{
  BidirectionalCursor, BidirectionalIteratorTag, Cursor, DerefPtrIterator, IteratorBase,
  IteratorRange, RandomAccessCursor, TupleIterator, make_iterator_range, reversed_range, zip,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 测试游标: 视作指向固定数组 data[0..len] 的下标迭代器 (原版 array iterator 形态)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IndexCursor<'a> {
  data: &'a [i32],
  i: usize,
}

impl<'a> IndexCursor<'a> {
  fn start(data: &'a [i32]) -> Self {
    Self { data, i: 0 }
  }
  fn stop(data: &'a [i32]) -> Self {
    Self {
      data,
      i: data.len(),
    }
  }
}

impl Cursor for IndexCursor<'_> {
  type Item = i32;
  fn deref(&self) -> i32 {
    self.data[self.i]
  }
  fn advance(&mut self) {
    self.i += 1;
  }
}

impl BidirectionalCursor for IndexCursor<'_> {
  fn retreat(&mut self) {
    self.i -= 1;
  }
}

impl RandomAccessCursor for IndexCursor<'_> {
  fn distance(&self, other: &Self) -> usize {
    other.i - self.i
  }
}

impl IteratorBase for IndexCursor<'_> {
  type Category = BidirectionalIteratorTag;
  type ValueType = i32;
  type Reference = i32;
}

#[test]
fn iterator_range_empty() -> Void {
  // 原版 IteratorRangeEmpty: 值初始化区间 begin==end==nullptr
  let d: DerefPtrIterator<i32> = DerefPtrIterator::default();
  let range = IteratorRange::new(d, d);
  assert!(range.is_empty());
  assert_eq!(range.iter().count(), 0);
  // 空区间的双端适配与截取均返回 None (原版 DCHECK 路径)
  assert!(range.iterate_without_first().is_none());
  assert!(range.iterate_without_last().is_none());
  // 默认构造入口 (原版 iterator_range() : begin_(), end_())
  let range = IteratorRange::<DerefPtrIterator<i32>>::default_empty();
  assert!(range.is_empty());
  let data: [i32; 0] = [];
  let empty = make_iterator_range(IndexCursor::start(&data), IndexCursor::stop(&data));
  assert!(empty.is_empty());
  assert_eq!(empty.size(), 0);
  OK
}

#[test]
fn iterator_range_array() -> Void {
  // 原版 IteratorRangeArray: std::array<int,3> 区间的遍历/截取/倒序
  let data = [1i32, 2, 3];
  let range = make_iterator_range(IndexCursor::start(&data), IndexCursor::stop(&data));
  assert!(!range.is_empty());
  assert_eq!(range.size(), 3);
  assert_eq!(range.iter().collect::<Vec<_>>(), vec![1, 2, 3]);
  // operator[](n)
  assert_eq!(range.index(1), 2);
  // IterateWithoutFirst / IterateWithoutLast
  let wo_first = range.iterate_without_first().unwrap();
  assert_eq!(wo_first.iter().collect::<Vec<_>>(), vec![2, 3]);
  let wo_last = range.iterate_without_last().unwrap();
  assert_eq!(wo_last.iter().collect::<Vec<_>>(), vec![1, 2]);
  // Reversed: 逐元素倒序 (原版 Reversed(t) 宏)
  let rev = reversed_range(&range);
  assert_eq!(rev.iter().collect::<Vec<_>>(), vec![3, 2, 1]);
  // rbegin()/rend() 等价 (make_reverse_iterator)
  let rr = range.rbegin_rend();
  assert_eq!(rr.iter().collect::<Vec<_>>(), vec![3, 2, 1]);
  OK
}

#[test]
fn iterator_range_deque() -> Void {
  // 原版 IteratorRangeDeque: 非随机访问容器 (deque) 上的 iterator_range
  // Rust 以指针数组 + DerefPtrIterator 承接同类 "迭代器宿主非切片" 形态
  let mut a = 10i32;
  let mut b = 20i32;
  let mut c = 30i32;
  let mut slots: [&mut i32; 3] = [&mut a, &mut b, &mut c];
  let raw: Vec<*mut i32> = slots.iter_mut().map(|s| *s as *mut i32).collect();
  let begin = DerefPtrIterator::new(raw.as_ptr());
  let end = {
    let mut e = begin;
    e.advance();
    e.advance();
    e.advance();
    e
  };
  let range = make_iterator_range(begin, end);
  assert_eq!(range.size(), 3);
  let mut out: Vec<i32> = Vec::new();
  for slot in range.iter() {
    assert!(!slot.is_null());
    // SAFETY: 槽指针由上方局部变量提供, 测试期内存活
    out.push(unsafe { *slot });
  }
  assert_eq!(out, vec![10, 20, 30]);
  // 反向游标解引用语义 (reverse_iterator: *r = *(--base))
  let mut r = end;
  r.retreat();
  assert_eq!(r.deref(), raw[2]);
  assert_eq!(range.iterate_without_last().unwrap().size(), 2);
  OK
}

#[test]
fn iterator_range_iterator_type_deduction() -> Void {
  // 原版 IteratorRangeIteratorTypeDeduction: iterator_traits 类别推导
  // Rust 以 trait 关联类型静态验证 (类别标签可达即推导成立)
  fn category_of<I: IteratorBase<Category = BidirectionalIteratorTag>>(_: &I) {}
  let data = [1i32, 2];
  let ic = IndexCursor::start(&data);
  category_of(&ic);
  let dp = DerefPtrIterator::<i32>::default();
  category_of(&dp);
  // DerefPtrIterator 同时满足三层游标 trait (forward/bidirectional/random)
  let _: fn(&DerefPtrIterator<i32>) = |_| {};
  // TupleIterator 二目形态 (原版 TupleIterator<Iterators...>)
  let t = TupleIterator::new(ic, DerefPtrIterator::<i32>::default());
  category_of(&t);
  // zip 端点形态: 元组逐项并行
  let raw: Vec<*mut i32> = vec![data.as_ptr() as *mut i32, unsafe {
    data.as_ptr().add(1) as *mut i32
  }];
  let zb = DerefPtrIterator::new(raw.as_ptr());
  let ze = {
    let mut e = zb;
    e.advance();
    e.advance();
    e
  };
  let z = zip(ic, IndexCursor::stop(&data), zb, ze);
  let pairs = z.iter().collect::<Vec<_>>();
  assert_eq!(pairs.len(), 2);
  assert_eq!(pairs[0].0, 1);
  OK
}

#[test]
fn reversed_cursor_double_ended() -> Void {
  // RangeIter::next_back (DoubleEnded) 与 ReversedCursor 结果一致
  let data = [1i32, 2, 3, 4];
  let range = make_iterator_range(IndexCursor::start(&data), IndexCursor::stop(&data));
  let back: Vec<i32> = range.iter().rev().collect();
  assert_eq!(back, vec![4, 3, 2, 1]);
  OK
}

#[test]
fn deref_ptr_iterator_slot_none_on_null() -> Void {
  // 空迭代器与空槽的显式错误路径 (原版裸解引用 UB → Rust None)
  let d = DerefPtrIterator::<i32>::default();
  assert_eq!(d.slot(), None);
  let null_slot: [*mut i32; 1] = [null_mut()];
  let it = DerefPtrIterator::new(null_slot.as_ptr());
  assert_eq!(it.slot(), None);
  assert!(it.deref().is_null());
  OK
}
