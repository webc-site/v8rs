//! v8/src/base/doubly-threaded-list.h 的 1:1 转写
//! DoublyThreadedList<T, DTLTraits>: 侵入式双向链表, T 为指针样句柄 (原版 Elem*)。
//! 节点持有普通 next 指针; prev 槽存放"前驱元素 next 字段的地址"(threaded),
//! 故删除无需特判表头、甚至无需持有表句柄 (原版 static Remove)。
//! 类型层数对照 (以 T = *mut N 实例化):
//! - prev_ 字段: 存放 *mut T (head_ 槽或前驱 next_ 字段的地址), 其自身地址 = *mut *mut T
//! - next_ 字段: 存放 T (后继句柄), 其自身地址 = *mut T
//!
//! unsafe 集中于 DTLTraits 槽访问; 表按位 move 后需 rebind_head_slot (等价原版移动构造)。

use std::{marker::PhantomData, mem::take, ptr};

/// 原版 DoublyThreadedListTraits<T>
pub trait DoublyThreadedListTraits<T> {
  /// static T** prev(T t) — 节点内 prev_ 字段的地址 (字段值类型为 *mut T)
  ///
  /// # Safety
  /// t 必须为存活节点句柄 (原版裸指针契约)
  unsafe fn prev(t: T) -> *mut *mut T;

  /// static T* next(T t) — 节点内 next_ 字段的地址 (字段值类型为 T)
  ///
  /// # Safety
  /// 同上
  unsafe fn next(t: T) -> *mut T;

  /// static bool non_empty(T t) — t != nullptr
  fn non_empty(t: T) -> bool;

  /// 原版 Contains 依赖的可选成员 (默认 traits 未提供 in_use → Contains 不可编译;
  /// Rust 以带默认实现的方法承接: prev_ 槽非空 ⇔ 挂在某条链上)
  ///
  /// # Safety
  /// 同上
  unsafe fn in_use(t: T) -> bool {
    unsafe { !(*Self::prev(t)).is_null() }
  }
}

/// 默认 traits 的节点侧接口 (对应原版节点成员函数 prev()/next())
pub trait DoublyThreadedSlots<T> {
  /// 节点内 prev_ 字段地址 (C++ `Elem*** prev() { return &prev_; }`)
  fn prev_slot(&mut self) -> *mut *mut T;
  /// 节点内 next_ 字段地址 (C++ `Elem** next() { return &next_; }`)
  fn next_slot(&mut self) -> *mut T;
}

/// 原版缺省 DoublyThreadedListTraits<T> (T = 节点裸指针 *mut N)
pub struct DefaultDoublyThreadedListTraits;

impl<N: DoublyThreadedSlots<*mut N>> DoublyThreadedListTraits<*mut N>
  for DefaultDoublyThreadedListTraits
{
  #[inline]
  unsafe fn prev(t: *mut N) -> *mut *mut *mut N {
    // SAFETY: trait 契约要求 t 为存活节点句柄
    unsafe { (*t).prev_slot() }
  }

  #[inline]
  unsafe fn next(t: *mut N) -> *mut *mut N {
    // SAFETY: 同上
    unsafe { (*t).next_slot() }
  }

  #[inline]
  fn non_empty(t: *mut N) -> bool {
    !t.is_null()
  }
}

/// 原版 class iterator : base::iterator<forward_iterator_tag, T>
#[derive(Debug)]
pub struct DoublyThreadedListIterator<T, D: DoublyThreadedListTraits<T>> {
  curr_: T,
  _marker: PhantomData<fn() -> D>,
}

impl<T: Clone, D: DoublyThreadedListTraits<T>> Clone for DoublyThreadedListIterator<T, D> {
  #[inline]
  fn clone(&self) -> Self {
    Self {
      curr_: self.curr_.clone(),
      _marker: PhantomData,
    }
  }
}

impl<T: Copy, D: DoublyThreadedListTraits<T>> Copy for DoublyThreadedListIterator<T, D> {}

impl<T: Clone, D: DoublyThreadedListTraits<T>> DoublyThreadedListIterator<T, D> {
  /// explicit iterator(T head) — 原版 private friend
  #[inline]
  pub(crate) fn new(curr: T) -> Self {
    Self {
      curr_: curr,
      _marker: PhantomData,
    }
  }

  /// T operator*
  #[inline]
  pub fn get(&self) -> T {
    self.curr_.clone()
  }

  /// bool operator==(end_iterator) 的反面: it != end ⇔ non_empty(curr_)
  #[inline]
  pub fn has_next(&self) -> bool {
    D::non_empty(self.curr_.clone())
  }

  /// iterator& operator++ / iterator operator++(int) 的推进面
  pub fn advance(&mut self) -> &mut Self {
    debug_assert!(D::non_empty(self.curr_.clone())); // DCHECK(non_empty(curr_))
    // SAFETY: 迭代器契约: curr_ 为存活节点句柄
    unsafe {
      self.curr_ = (*D::next(self.curr_.clone())).clone();
    }
    self
  }

  /// iterator operator++(int) — 后置: 返回旧值, 前进自身
  pub fn advance_post(&mut self) -> DoublyThreadedListIterator<T, D> {
    let tmp = self.clone();
    self.advance();
    tmp
  }
}

impl<T: Clone, D: DoublyThreadedListTraits<T>> Iterator for DoublyThreadedListIterator<T, D> {
  type Item = T;
  /// range-for 面 (原版 `for (Elem* e : list)`)
  fn next(&mut self) -> Option<T> {
    if !self.has_next() {
      return None;
    }
    let curr = self.curr_.clone();
    self.advance();
    Some(curr)
  }
}

/// 原版 class end_iterator {}
#[derive(Debug, Clone, Copy)]
pub struct DoublyThreadedListEnd;

/// 原版 class DoublyThreadedList<T, DTLTraits>
pub struct DoublyThreadedList<T, D: DoublyThreadedListTraits<T>> {
  /// T head_{}
  head_: T,
  _marker: PhantomData<fn() -> D>,
}

impl<T: Clone + Default + PartialEq, D: DoublyThreadedListTraits<T>> DoublyThreadedList<T, D> {
  /// DoublyThreadedList() = default
  #[inline]
  pub fn new() -> Self {
    Self {
      head_: T::default(),
      _marker: PhantomData,
    }
  }

  /// head_ 槽地址 (原版 &head_); prev_ 字段值类型即 *mut T, 与此吻合
  #[inline]
  fn head_slot(&mut self) -> *mut T {
    ptr::from_mut(&mut self.head_)
  }

  /// static void Remove(T x)
  /// 经 x 的 prev_ 槽间接改写前驱 (或表头) — 不要求持有表句柄 (原版设计)。
  /// x 已不在链上时为 no-op (原版判定)。
  /// 注: x 为某活表表头且同时持有该表 &mut 借用时, 请改用 remove()/pop_front()
  /// (等价路径, 避免 Rust 侧裸指针别名写)。
  pub fn remove_x(x: T) {
    // SAFETY: 原版静态方法契约 — x 为存活节点句柄; 前驱节点/槽同样存活
    unsafe {
      if (*D::prev(x.clone())).is_null() {
        // DCHECK(empty(*DTLTraits::next(x))) — x 已不在链上
        debug_assert!(Self::empty_t(&*D::next(x.clone())));
        return;
      }
      let prev = D::prev(x.clone());
      let next = D::next(x.clone());
      **prev = (*next).clone();
      let nx = (*next).clone();
      if D::non_empty(nx.clone()) {
        // 原版: *DTLTraits::prev(*next) = *prev — 写后继节点的 prev_ 字段 (值类型 *mut T)
        *D::prev(nx) = *prev;
      }
      *prev = ptr::null_mut();
      *next = T::default();
    }
  }

  /// void PushFront(T x) — x 对既有迭代器不可见, 不使既有迭代器失效 (原版注释)
  pub fn push_front(&mut self, x: T) {
    // SAFETY: 原版 DCHECK 前置: x 存活且未挂链 (next 空、prev 槽空);
    // head_ 槽与节点槽写均为本侵入式结构的常规别名写
    unsafe {
      debug_assert!(Self::empty_t(&*D::next(x.clone()))); // DCHECK(empty(*next(x)))
      debug_assert!((*D::prev(x.clone())).is_null()); // DCHECK_EQ(*prev(x), nullptr)
      *D::next(x.clone()) = self.head_.clone();
      *D::prev(x.clone()) = self.head_slot();
      if D::non_empty(self.head_.clone()) {
        // 原表头的 prev_ 字段改指 x 的 next_ 字段槽
        *D::prev(self.head_.clone()) = D::next(x.clone());
      }
      self.head_ = x;
    }
  }

  /// T Front() const
  #[inline]
  pub fn front(&self) -> T {
    debug_assert!(!self.is_empty()); // DCHECK(!empty())
    self.head_.clone()
  }

  /// void PopFront() — 等价 Remove(Front()), 走安全路径直接改写 head_
  pub fn pop_front(&mut self) {
    debug_assert!(!self.is_empty());
    let f = self.head_.clone();
    let next;
    // SAFETY: f 为存活头节点; 后继 (若有) 同样存活
    unsafe {
      next = (*D::next(f.clone())).clone();
      if D::non_empty(next.clone()) {
        // 新表头的 prev_ 字段指回本表 head_ 槽
        *D::prev(next.clone()) = self.head_slot();
      }
      *D::next(f.clone()) = T::default();
      *D::prev(f.clone()) = ptr::null_mut();
    }
    self.head_ = next;
  }

  /// 成员面 Remove — 表头特化走 pop_front (避免别名写), 其余走 static 路径
  pub fn remove(&mut self, x: T) {
    if self.head_ == x {
      self.pop_front();
      return;
    }
    Self::remove_x(x);
  }

  /// bool empty() const
  #[inline]
  pub fn is_empty(&self) -> bool {
    !D::non_empty(self.head_.clone())
  }

  /// iterator begin() const
  #[inline]
  pub fn begin(&self) -> DoublyThreadedListIterator<T, D> {
    DoublyThreadedListIterator::new(self.head_.clone())
  }

  /// end_iterator end() const
  #[inline]
  pub fn end(&self) -> DoublyThreadedListEnd {
    DoublyThreadedListEnd
  }

  /// iterator RemoveAt(iterator& it) — it 指向的元素被移除, 返回指向后继的新迭代器
  /// (it 不应指向 end; 原版 DCHECK 前置)
  pub fn remove_at(
    &mut self,
    it: &DoublyThreadedListIterator<T, D>,
  ) -> DoublyThreadedListIterator<T, D> {
    let curr = it.get();
    debug_assert!(D::non_empty(curr.clone()));
    // SAFETY: 迭代器契约: curr 为存活节点
    let next = unsafe { (*D::next(curr.clone())).clone() };
    // 原版经 prev 槽联动 head_; 表头情况走安全特化
    if self.head_ == curr {
      self.pop_front();
    } else {
      Self::remove_x(curr);
    }
    DoublyThreadedListIterator::new(next)
  }

  /// bool Contains(T needle) const — 原版 DCHECK_EQ(in_use, ContainsSlow(needle))
  pub fn contains(&self, needle: T) -> bool {
    // SAFETY: needle 为存活节点句柄 (原版契约)
    let in_use = unsafe { D::in_use(needle.clone()) };
    debug_assert_eq!(in_use, self.contains_slow(needle));
    in_use
  }

  /// bool ContainsSlow(T needle) const
  pub fn contains_slow(&self, needle: T) -> bool {
    let mut it = DoublyThreadedListIterator::<T, D>::new(self.head_.clone());
    while it.has_next() {
      if it.get() == needle {
        return true;
      }
      it.advance();
    }
    false
  }

  /// static bool empty(T x) — 原版 private 辅助
  #[inline]
  fn empty_t(x: &T) -> bool {
    !D::non_empty(x.clone())
  }

  /// 等价原版移动构造: 按位 move 完成后在目的地重绑 head 的 prev_ 槽。
  /// Rust 无法拦截按位 move, 故显式提供; 源实例此后不得再以本表身份使用。
  ///
  /// # Safety
  /// self 为刚按位移动得到的实例 (head_ 节点的 prev_ 可能仍指源地址)
  pub unsafe fn rebind_head_slot(&mut self) {
    if D::non_empty(self.head_.clone()) {
      // SAFETY: 契约见文档注释; head_ 节点存活
      unsafe {
        *D::prev(self.head_.clone()) = self.head_slot();
      }
    }
  }

  /// 原版移动构造的完整 Rust 等价: 窃取 other 表头并重绑, other 置空
  pub fn steal(other: &mut Self) -> Self {
    let mut this = Self::new();
    this.head_ = take(&mut other.head_);
    // SAFETY: head_ 现指原表首节点, 其 prev_ 仍指 other.head_ 槽 — 立即重绑
    unsafe {
      this.rebind_head_slot();
    }
    this
  }
}

impl<T: Clone + Default + PartialEq, D: DoublyThreadedListTraits<T>> Default
  for DoublyThreadedList<T, D>
{
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}
