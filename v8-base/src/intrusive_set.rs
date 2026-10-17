//! v8/src/base/intrusive-set.h 的 1:1 转写
//! IntrusiveSet<T, GetIntrusiveSetIndex, Container>: 元素自带 set 内位置索引,
//! Add/Contains/Remove O(1), 无 hash 表。
//! C++: T 为指针样值 + 元素内存持有 IntrusiveSetIndex::value (size_t)。
//! Rust: 索引槽以 Cell<usize> 表达 (元素侧自持, 与原版"所有副本指向同一索引内存"约束一致);
//! 元素向量用 UnsafeCell 承接原版 "迭代中 Add/Remove" 的非 const 语义
//! (unsafe 集中在容器访问两处; 句柄 T 的索引定位由用户在 GetIndex 实现)。

use std::{
  cell::{Cell, UnsafeCell},
  marker::PhantomData,
  ptr::null,
};

/// 原版 IntrusiveSetIndex::kNotInSet = std::numeric_limits<size_t>::max()
pub const K_NOT_IN_SET: usize = usize::MAX;

/// 原版 class IntrusiveSetIndex { size_t value = kNotInSet; }
/// Rust 以 Cell 承接 "容器经共享引用改写索引" 的原版语义。
#[derive(Debug)]
pub struct IntrusiveSetIndex {
  value: Cell<usize>,
}

impl Default for IntrusiveSetIndex {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl IntrusiveSetIndex {
  /// 等价构造: value = kNotInSet
  #[inline]
  pub fn new() -> Self {
    Self {
      value: Cell::new(K_NOT_IN_SET),
    }
  }

  #[inline]
  pub fn get(&self) -> usize {
    self.value.get()
  }

  #[inline]
  pub fn set(&self, v: usize) {
    self.value.set(v);
  }

  /// 索引内存地址 (迭代器识别 swap-out 用, 对应原版 &Index(x))
  #[inline]
  pub fn addr(&self) -> *const usize {
    self.value.as_ptr().cast_const()
  }
}

/// 原版 GetIntrusiveSetIndex 函子: 由 T (指针样值) 定位其 IntrusiveSetIndex 槽。
/// 实现方约束与原版相同: 同一元素的所有 T 副本必须返回同一槽, 且元素在被移除
/// 且迭代结束前该内存必须存活。
pub trait GetIndex<T> {
  fn index_of<'a>(&self, x: &T) -> &'a IntrusiveSetIndex;
}

/// 原版 Container 模板参数的最小接口 (std::vector 面: size/push_back/pop_back/back/[])
pub trait IntrusiveContainer<T> {
  fn ilen(&self) -> usize;
  fn ipush(&mut self, x: T);
  fn ipop(&mut self) -> Option<T>;
  fn iget(&self, i: usize) -> Option<&T>;
  fn iset(&mut self, i: usize, x: T) -> bool;
}

impl<T: Copy> IntrusiveContainer<T> for Vec<T> {
  #[inline]
  fn ilen(&self) -> usize {
    self.len()
  }
  #[inline]
  fn ipush(&mut self, x: T) {
    self.push(x);
  }
  #[inline]
  fn ipop(&mut self) -> Option<T> {
    self.pop()
  }
  #[inline]
  fn iget(&self, i: usize) -> Option<&T> {
    self.get(i)
  }
  #[inline]
  fn iset(&mut self, i: usize, x: T) -> bool {
    match self.get_mut(i) {
      Some(slot) => {
        *slot = x;
        true
      }
      None => false,
    }
  }
}

/// IntrusiveSet<T, GetIntrusiveSetIndex, Container>
/// 原版 static_assert(is_trivially_copyable_v<T>) → Rust 要求 T: Copy + PartialEq
/// (DCHECK_EQ(x, elements_[index]) 使用相等比较)。
/// 单线程使用 (UnsafeCell), 迭代期间允许 Add (新元素会被访问) 与 Remove 当前元素。
pub struct IntrusiveSet<T, G: GetIndex<T>, C: IntrusiveContainer<T>> {
  /// 原版允许 const 方法改 elements_ (迭代中增删), Rust 以 UnsafeCell 承接;
  /// 借用互斥由使用方按原版契约维持 (禁重入 &mut)。
  elements_: UnsafeCell<C>,
  index_functor_: G,
  _marker: PhantomData<T>,
}

impl<T: Copy + PartialEq, G: GetIndex<T>, C: IntrusiveContainer<T>> IntrusiveSet<T, G, C> {
  /// explicit IntrusiveSet(Container, GetIntrusiveSetIndex = {})
  #[inline]
  pub fn new(container: C, index_functor: G) -> Self {
    Self {
      elements_: UnsafeCell::new(container),
      index_functor_: index_functor,
      _marker: PhantomData,
    }
  }

  /// size_t& Index(T x) const — 原版经引用读写
  /// (命名 index_slot 避免与 std::ops::Index::index 混淆)
  #[inline]
  pub fn index_slot(&self, x: &T) -> &IntrusiveSetIndex {
    self.index_functor_.index_of(x)
  }

  // SAFETY 组: 以下两处的可变访问不与他处 &借用 并发交叠 —— 原版 C++ 即以
  // const 成员函数改 elements_, 全部增删只经本结构 &self 方法; 方法间无嵌套
  // 活动引用 (每次访问在语句结束前完成读/写), 单线程契约由类型使用方维持。
  #[inline]
  fn elements(&self) -> &C {
    // SAFETY: 见上; 瞬时读, 调用点不与 elements_mut() 的活动借用重叠
    unsafe { &*self.elements_.get() }
  }

  /// # Safety
  /// 返回的指针仅在瞬时语句内解引用为 &mut, 不得与其他活动借用重叠 (单线程契约)
  #[inline]
  fn elements_mut_ptr(&self) -> *mut C {
    self.elements_.get()
  }

  /// bool Contains(T x) const
  #[inline]
  pub fn contains(&self, x: T) -> bool {
    self.index_slot(&x).get() != K_NOT_IN_SET
  }

  /// elements_.size() — 调试/断言面
  #[inline]
  pub fn size(&self) -> usize {
    self.elements().ilen()
  }

  /// 只读快照 (原版直接暴露迭代; 此处辅助惯用 Rust 侧)
  pub fn snapshot(&self) -> Vec<T> {
    self.iter().collect()
  }

  /// void Add(T x) — 迭代中插入允许, 新元素会被本次迭代访问
  pub fn add(&self, x: T) {
    debug_assert!(!self.contains(x)); // DCHECK(!Contains(x))
    let pos = self.elements().ilen();
    self.index_slot(&x).set(pos);
    // SAFETY: 瞬时 &mut, 无重叠借用
    unsafe { &mut *self.elements_mut_ptr() }.ipush(x);
  }

  /// void Remove(T x) — swap-pop; 迭代中移除当前元素合法 (见 iterator 注释)
  pub fn remove(&self, x: T) {
    let slot = self.index_slot(&x);
    let index = slot.get();
    debug_assert!(index != K_NOT_IN_SET); // DCHECK(Contains(x))
    debug_assert!(self.elements().iget(index).is_some_and(|e| *e == x)); // DCHECK_EQ(x, elements_[index])
    // Index(elements_.back()) = index; elements_[index] = elements_.back();
    let back = match self.elements().iget(self.elements().ilen() - 1).copied() {
      Some(back) => back,
      None => return, // 空容器不可移除 (debug_assert 已覆盖不变量)
    };
    self.index_slot(&back).set(index);
    // SAFETY: 瞬时 &mut, 无重叠借用
    unsafe { &mut *self.elements_mut_ptr() }.iset(index, back);
    slot.set(K_NOT_IN_SET);
    // SAFETY: 瞬时 &mut, 无重叠借用
    unsafe { &mut *self.elements_mut_ptr() }.ipop();
  }

  /// iterator begin()
  #[inline]
  pub fn iter(&self) -> IntrusiveSetIter<'_, T, G, C> {
    IntrusiveSetIter {
      set_: self,
      index_: 0,
      last_index_location_: null(),
    }
  }
}

/// 原版 class iterator
/// 支持: 迭代中插入 (新元素会被访问); 移除当前元素 (swap-in 的新元素会被访问)。
/// 移除"已访问过"的元素为原版 UB — 此处行为同样不保证。
/// ATTENTION: 被移除元素的索引内存须存活到迭代结束 (原版注释)。
pub struct IntrusiveSetIter<'s, T: Copy + PartialEq, G: GetIndex<T>, C: IntrusiveContainer<T>> {
  set_: &'s IntrusiveSet<T, G, C>,
  index_: usize,
  /// const size_t* last_index_location_ — 上次 operator* 记录的索引内存地址
  last_index_location_: *const usize,
}

impl<T: Copy + PartialEq, G: GetIndex<T>, C: IntrusiveContainer<T>> Iterator
  for IntrusiveSetIter<'_, T, G, C>
{
  type Item = T;

  /// 组合原版 `T x = *it; ...; ++it`:
  /// 上一次记录的元素若仍在本槽 (索引地址未变) → 前进;
  /// 若被 Remove swap-pop 换入新元素 → 不前进, 下次访问换入的新元素。
  fn next(&mut self) -> Option<T> {
    // iterator::operator++ 的判定 (首调用无记录, 不前进)
    if !self.last_index_location_.is_null()
      && let Some(x) = self.set_.elements().iget(self.index_)
      && self.set_.index_slot(x).addr() == self.last_index_location_
    {
      self.index_ += 1;
    }
    // iterator::operator!=(end_iterator)
    if self.index_ >= self.set_.size() {
      return None;
    }
    // iterator::operator* — 记录索引内存地址
    let x = self.set_.elements().iget(self.index_).copied()?;
    self.last_index_location_ = self.set_.index_slot(&x).addr();
    Some(x)
  }
}
