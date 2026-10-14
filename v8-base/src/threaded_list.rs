//! v8/src/base/threaded-list.h 的 1:1 转写
//! ThreadedListBase<T, BaseClass, TLTraits, kSupportsUnsafeInsertion>
//! 原版是基于裸指针的侵入式链表: 链指针内嵌于节点 (C++ `T** next()`), 尾指针 tail_
//! 指向"最后一个节点的 next 槽"或 &head_ (空表)。
//! Rust 保持同一链接不变量:
//! - head_: *mut T   非拥有裸指针 (节点存活由使用方保证, 与 C++ 契约一致)
//! - tail_: Option<*mut *mut T> — None ⇔ 原版 tail_ == &head_ (空/清表态)。
//!   以编码消除自引用裸地址 (原版靠移动构造重绑 &head_; Rust 无法拦截按位 move,
//!   故空表态不存地址), 跨 move 语义等价。
//!
//! unsafe 集中于 L::next 解引用面; 逐处 SAFETY 注释。

use std::{marker::PhantomData, ptr};

/// 原版 ThreadedListTraits<T>: next() 返回节点内"下一指针槽"的地址 (C++ T**)
/// 实现方契约: t 非空且指向存活节点时返回值有效 (与 C++ 裸指针契约一致)
pub trait ThreadedListTraits<T> {
  /// static T** next(T* t)
  ///
  /// # Safety
  /// t 必须为指向存活 T 节点的合法指针
  unsafe fn next(t: *mut T) -> *mut *mut T;

  /// static T** start(T** t) — 恒等 (原版双重载仅 const 之分)
  fn start(t: *mut *mut T) -> *mut *mut T {
    t
  }
}

/// 默认 traits 的节点侧接口: 节点自带 next 槽 (对应 C++ 节点成员 `T* next_; T** next()`)
pub trait ThreadedNextSlot<T> {
  /// 节点内 next 字段的地址 (等价 C++ `T** next() { return &next_; }`)
  fn next_slot(&mut self) -> *mut *mut T;
}

/// 原版缺省 ThreadedListTraits<T> (调用 t->next())
pub struct DefaultThreadedListTraits;

impl<T: ThreadedNextSlot<T>> ThreadedListTraits<T> for DefaultThreadedListTraits {
  #[inline]
  unsafe fn next(t: *mut T) -> *mut *mut T {
    // SAFETY: trait 契约要求调用方保证 t 为存活节点指针
    unsafe { (*t).next_slot() }
  }
}

/// 原版 class Iterator (entry_ 为 C++ T**)
/// 表被 move 后既有迭代器失效 (与 C++ 悬垂语义一致); begin()/end() 每次现取槽地址。
/// 节点/链特征协变标记 (隔离 T/L 的拥有语义)
type ListMarker<T, L> = PhantomData<(fn() -> T, fn() -> L)>;

#[derive(Debug, Clone, Copy)]
pub struct ThreadedListIterator<T, L: ThreadedListTraits<T>> {
  entry_: *mut *mut T,
  _marker: ListMarker<T, L>,
}

impl<T, L: ThreadedListTraits<T>> PartialEq for ThreadedListIterator<T, L> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    // bool operator==(const Iterator& other) { return entry_ == other.entry_; }
    self.entry_ == other.entry_
  }
}

impl<T, L: ThreadedListTraits<T>> ThreadedListIterator<T, L> {
  /// explicit Iterator(T** entry) — 原版 private + friend, Rust 侧 crate 内可见
  #[inline]
  pub(crate) fn from_slot(entry: *mut *mut T) -> Self {
    Self {
      entry_: entry,
      _marker: PhantomData,
    }
  }

  /// 内部: 槽地址 (RemoveAt/Rewind/MoveTail 的 entry_ 传递面)
  #[inline]
  pub(crate) fn slot(&self) -> *mut *mut T {
    self.entry_
  }

  /// T*& operator*
  #[inline]
  pub fn get(&self) -> *mut T {
    // SAFETY: entry_ 由表不变量保证指向有效槽 (head 槽或存活节点的 next 槽)
    unsafe { *self.entry_ }
  }

  /// Iterator& operator++
  #[inline]
  pub fn advance(&mut self) -> &mut Self {
    // SAFETY: 同上; end() 槽不参与 ++ (原版循环条件一致)
    unsafe {
      self.entry_ = L::next(*self.entry_);
    }
    self
  }

  /// bool operator==(const Iterator&) 面
  #[inline]
  pub fn same(&self, other: &ThreadedListIterator<T, L>) -> bool {
    self.entry_ == other.entry_
  }

  /// bool is_null() — 原版 entry_ 恒非 null, 保留接口
  #[inline]
  pub fn is_null(&self) -> bool {
    self.entry_.is_null()
  }

  /// Iterator& operator=(T* entry) — 就地以 entry 替换当前槽所指节点, 保留其后继链
  ///
  /// # Safety
  /// entry_ 指向有效槽; entry 须为存活节点 (原版裸指针契约)
  #[inline]
  pub unsafe fn assign(&mut self, entry: *mut T) -> &mut Self {
    // SAFETY: entry_ 指向有效槽; entry 为存活节点 (原版裸指针契约)
    unsafe {
      let cur = *self.entry_;
      let next = *L::next(cur);
      *L::next(entry) = next;
      *self.entry_ = entry;
    }
    self
  }

  /// void InsertBefore(T* value) — 在当前迭代器位置插入 value 并停在 value 的 next 槽
  ///
  /// # Safety
  /// entry_ 指向有效槽; value 须为存活且 next 槽未接线的节点
  #[inline]
  pub unsafe fn insert_before(&mut self, value: *mut T) {
    // SAFETY: entry_ 指向有效槽; value 为存活且 next 槽未接线 (原版 DCHECK 前置)
    unsafe {
      let old_entry_value = *self.entry_;
      *self.entry_ = value;
      self.entry_ = L::next(value);
      *self.entry_ = old_entry_value;
    }
  }
}

/// ThreadedListBase<T, BaseClass, TLTraits, kSupportsUnsafeInsertion>
/// (原版 BaseClass 仅 EmptyBase 空继承, Rust 由组合替代 — 见 5 章决策)
pub struct ThreadedListBase<T, L, const SUPPORTS_UNSAFE_INSERTION: bool> {
  head_: *mut T,
  /// None ⇔ 原版 tail_ == &head_
  tail_: Option<*mut *mut T>,
  _marker: ListMarker<T, L>,
}

impl<T, L: ThreadedListTraits<T>, const SUPPORTS_UNSAFE_INSERTION: bool> Default
  for ThreadedListBase<T, L, SUPPORTS_UNSAFE_INSERTION>
{
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl<T, L: ThreadedListTraits<T>, const SUPPORTS_UNSAFE_INSERTION: bool>
  ThreadedListBase<T, L, SUPPORTS_UNSAFE_INSERTION>
{
  /// ThreadedListBase() : head_(nullptr), tail_(&head_)
  #[inline]
  pub fn new() -> Self {
    Self {
      head_: ptr::null_mut(),
      tail_: None,
      _marker: PhantomData,
    }
  }

  /// head 槽地址 (begin()/start() 与空表尾态的对照点)
  #[inline]
  fn head_slot(&mut self) -> *mut *mut T {
    ptr::from_mut(&mut self.head_)
  }

  /// 只读 head 槽地址 (比较用)
  #[inline]
  fn head_slot_ref(&self) -> *mut *mut T {
    ptr::from_ref(&self.head_).cast_mut()
  }

  /// 实际尾槽: tail_ 或 &head_ (原版裸 tail_)
  #[inline]
  fn tail_slot(&mut self) -> *mut *mut T {
    // SAFETY: head_slot 写权限仅在 &mut self 生命周期内经裸指针使用 (原版 tail_==&head_ 语义)
    match self.tail_ {
      Some(t) => t,
      None => ptr::from_mut(&mut self.head_),
    }
  }

  /// void Add(T* v)
  ///
  /// # Safety
  /// v 须为存活节点且 next 槽未接线 (原版 DCHECK_NULL 前置)
  pub unsafe fn add(&mut self, v: *mut T) {
    self.ensure_valid_tail();
    // SAFETY: tail 槽与 v 节点均存活 (原版裸指针契约, 见类型文档);
    // 经尾槽写 head_ 仅在 None 态发生, 此时 self 为独占借用
    unsafe {
      let tail = self.tail_slot();
      debug_assert!((*tail).is_null()); // DCHECK_NULL(*tail_)
      let vnext = L::next(v);
      debug_assert!((*vnext).is_null()); // DCHECK_NULL(*TLTraits::next(v))
      *tail = v;
      self.tail_ = Some(vnext);
      // Check that only one element was added (and that hasn't created a cycle).
      debug_assert!((*vnext).is_null()); // DCHECK_NULL(*tail_)
    }
  }

  /// void AddFront(T* v)
  ///
  /// # Safety
  /// v 须为存活节点且 next 槽未接线 (原版 DCHECK_NULL 前置)
  pub unsafe fn add_front(&mut self, v: *mut T) {
    debug_assert!(!v.is_null()); // DCHECK_NOT_NULL(v)
    // SAFETY: v 为存活节点; 其 next 槽未接线 (原版 DCHECK_NULL 前置)
    unsafe {
      debug_assert!((*L::next(v)).is_null());
      let next = L::next(v);
      *next = self.head_;
      if self.head_.is_null() {
        self.tail_ = Some(next);
      }
      self.head_ = v;
    }
  }

  /// static void AddAfter(T* after_this, T* v)
  /// 仅当 SUPPORTS_UNSAFE_INSERTION; 会临时破坏 tail_ 不变量 (原版注释)
  ///
  /// # Safety
  /// 两指针均须为存活节点且 v 的 next 槽未接线
  pub unsafe fn add_after(after_this: *mut T, v: *mut T) {
    debug_assert!(SUPPORTS_UNSAFE_INSERTION); // DCHECK(kSupportsUnsafeInsertion)
    // SAFETY: 两指针均为存活节点 (原版静态方法契约)
    unsafe {
      debug_assert!((*L::next(v)).is_null());
      *L::next(v) = *L::next(after_this);
      *L::next(after_this) = v;
    }
  }

  /// void DropHead()
  pub fn drop_head(&mut self) {
    debug_assert!(!self.head_.is_null()); // DCHECK_NOT_NULL(head_)
    // SAFETY: head_ 为存活节点
    unsafe {
      let old_head = self.head_;
      self.head_ = *L::next(old_head);
      if self.head_.is_null() {
        self.tail_ = None; // tail_ = &head_
      }
      *L::next(old_head) = ptr::null_mut();
    }
  }

  /// bool Contains(T* v)
  pub fn contains(&mut self, v: *mut T) -> bool {
    let mut it = self.begin();
    loop {
      let end = self.end();
      if it.same(&end) {
        return false;
      }
      if it.get() == v {
        return true;
      }
      it.advance();
    }
  }

  /// void Append(ThreadedListBase&& list) — Rust 以 &mut other 承接移动源
  pub fn append(&mut self, list: &mut Self) {
    if list.is_empty() {
      return;
    }
    self.ensure_valid_tail();
    // SAFETY: 两表节点均存活; 尾槽写指针 (原版语义)
    unsafe {
      let tail = self.tail_slot();
      *tail = list.head_;
      // 非空表不变量: tail_ 指向节点槽 (None 仅空表)
      debug_assert!(list.tail_.is_some());
      self.tail_ = list.tail_;
    }
    list.clear();
  }

  /// void Prepend(ThreadedListBase&& list)
  pub fn prepend(&mut self, list: &mut Self) {
    if list.head_.is_null() {
      return;
    }
    self.ensure_valid_tail();
    // SAFETY: 同 append
    unsafe {
      let new_head = list.head_;
      let tail = list.tail_slot();
      *tail = self.head_;
      if self.head_.is_null() {
        // 本表为空: 他表尾即新尾
        self.tail_ = list.tail_;
      }
      self.head_ = new_head;
    }
    list.clear();
  }

  /// void TruncateAt(ThreadedListBase* rem, T* v)
  /// 原版 CHECK 前置 (rem 空 / v 在本表内) 失败时 Rust 侧显式拒绝: 返回 false 且不改状态
  #[must_use = "false 表示前置条件不满足, 状态未变更 (原版 CHECK 失败)"]
  pub fn truncate_at(&mut self, rem: &mut Self, v: *mut T) -> bool {
    if v.is_null() || !rem.is_empty() {
      return false; // CHECK_NOT_NULL(v) / CHECK(rem->is_empty())
    }
    let mut it = self.begin();
    let mut last: *mut T = ptr::null_mut();
    let found = loop {
      let end = self.end();
      if it.same(&end) {
        break false; // CHECK_EQ(v, *it) 失败
      }
      if it.get() == v {
        break true;
      }
      last = it.get();
      it.advance();
    };
    if !found {
      return false;
    }

    // Remaining list.
    rem.head_ = v;
    let tail_ptr = self.tail_slot();
    // 非空表尾不可能是 head 槽; None 仅空表防御性编码
    rem.tail_ = if tail_ptr == self.head_slot_ref() {
      None
    } else {
      Some(tail_ptr)
    };

    if last.is_null() {
      // CHECK_EQ(head_, v): 头即 v → 本表清空
      if self.head_ != v {
        return false;
      }
      self.clear();
    } else {
      // SAFETY: last 为 v 的直接前驱 (迭代到达), 存活
      unsafe {
        let next_slot = L::next(last);
        *next_slot = ptr::null_mut();
        self.tail_ = Some(next_slot);
      }
    }
    true
  }

  /// void Clear() — head_ = nullptr, tail_ = &head_
  #[inline]
  pub fn clear(&mut self) {
    self.head_ = ptr::null_mut();
    self.tail_ = None;
  }

  /// operator=(ThreadedListBase&&) — 调试版语义 (other.Clear()),
  /// 显式命名以规避 Rust 按位 move 无法执行重绑的问题
  pub fn move_from(&mut self, other: &mut Self) {
    self.head_ = other.head_;
    self.tail_ = if other.head_.is_null() {
      None
    } else {
      other.tail_
    };
    other.clear();
  }

  /// ThreadedListBase(ThreadedListBase&&) — 构造面 (同上注释)
  pub fn steal(other: &mut Self) -> Self {
    let head = other.head_;
    let tail = if head.is_null() { None } else { other.tail_ };
    other.clear();
    Self {
      head_: head,
      tail_: tail,
      _marker: PhantomData,
    }
  }

  /// bool Remove(T* v)
  pub fn remove(&mut self, v: *mut T) -> bool {
    let mut current = self.first();
    if current == v {
      self.drop_head();
      return true;
    }

    self.ensure_valid_tail();
    // SAFETY: 沿链解引用均为存活节点 (原版遍历契约)
    unsafe {
      while !current.is_null() {
        let next = *L::next(current);
        if next == v {
          let nn = *L::next(next);
          let cur_next = L::next(current);
          *cur_next = nn;
          *L::next(next) = ptr::null_mut();

          if L::next(next) == self.tail_slot_raw() {
            self.tail_ = Some(cur_next);
          }
          return true;
        }
        current = next;
      }
    }
    false
  }

  /// &self 态尾槽读取 (remove 循环内比较用; 节点槽地址不受 self 可变性影响)
  #[inline]
  fn tail_slot_raw(&self) -> *mut *mut T {
    // SAFETY: None 态返回 head 槽只读地址, 仅做指针相等比较, 不写
    match self.tail_ {
      Some(t) => t,
      None => ptr::from_ref(&self.head_).cast_mut(),
    }
  }

  /// Iterator begin() — Iterator(TLTraits::start(&head_))
  #[inline]
  pub fn begin(&mut self) -> ThreadedListIterator<T, L> {
    ThreadedListIterator::from_slot(L::start(self.head_slot()))
  }

  /// Iterator end() — EnsureValidTail(); Iterator(tail_)
  #[inline]
  pub fn end(&mut self) -> ThreadedListIterator<T, L> {
    self.ensure_valid_tail();
    ThreadedListIterator::from_slot(self.tail_slot())
  }

  /// void Rewind(Iterator reset_point) — 尾回卷到 reset_point, 截断其后 (含该点)
  pub fn rewind(&mut self, reset_point: &ThreadedListIterator<T, L>) {
    let slot = reset_point.slot();
    if slot == self.head_slot() {
      // 原版 tail_ = &head_ 且 *tail_ = nullptr ⇒ head_ = nullptr
      self.head_ = ptr::null_mut();
      self.tail_ = None;
    } else {
      // SAFETY: slot 为存活节点的 next 槽 (迭代器有效性契约)
      unsafe {
        *slot = ptr::null_mut();
      }
      self.tail_ = Some(slot);
    }
  }

  /// void MoveTail(ThreadedListBase* from_list, Iterator from_location)
  pub fn move_tail(&mut self, from_list: &mut Self, from_location: &ThreadedListIterator<T, L>) {
    let from_end = from_list.end();
    if !from_end.same(from_location) {
      self.ensure_valid_tail();
      // SAFETY: 两表节点存活; 尾槽写指针
      unsafe {
        let tail = self.tail_slot();
        debug_assert!((*tail).is_null()); // DCHECK_NULL(*tail_)
        *tail = from_location.get();
        self.tail_ = from_list.tail_;
      }
      from_list.rewind(from_location);
    }
  }

  /// Iterator RemoveAt(Iterator it) — 删除 it 指向元素, 返回后继迭代器
  /// (it 不应为 end(); 原版前置条件保留)
  pub fn remove_at(&mut self, it: &ThreadedListIterator<T, L>) -> ThreadedListIterator<T, L> {
    // SAFETY: it.slot() 为表内有效槽 (head 槽或存活节点 next 槽), 指向当前元素槽
    unsafe {
      if *it.slot() == self.head_ {
        self.drop_head();
        self.begin()
      } else if self.tail_slot() == L::next(*it.slot()) {
        // it 指向尾元素: tail_ = it.entry_; *it.entry_ = nullptr; return end();
        // (it.slot() 此处必为节点槽: head 槽情况已被上分支捕获)
        *it.slot() = ptr::null_mut();
        self.tail_ = Some(it.slot());
        self.end()
      } else {
        let old_entry = *it.slot();
        *it.slot() = *L::next(old_entry);
        *L::next(old_entry) = ptr::null_mut();
        ThreadedListIterator::from_slot(it.slot())
      }
    }
  }

  /// bool is_empty() const
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.head_.is_null()
  }

  /// T* first() const
  #[inline]
  pub fn first(&self) -> *mut T {
    self.head_
  }

  /// int LengthForTest()
  pub fn length_for_test(&mut self) -> usize {
    let mut result = 0;
    let mut it = self.begin();
    loop {
      let end = self.end();
      if it.same(&end) {
        break result;
      }
      result += 1;
      it.advance();
    }
  }

  /// T* AtForTest(int i)
  pub fn at_for_test(&mut self, i: usize) -> *mut T {
    let mut it = self.begin();
    for _ in 0..i {
      it.advance();
    }
    it.get()
  }

  /// bool Verify() const — 原版 CHECK_EQ(&head_, tail_) / CHECK_EQ(next(last), tail_)
  pub fn verify(&mut self) -> bool {
    self.ensure_valid_tail();
    let last = self.first();
    if last.is_null() {
      return self.tail_.is_none(); // CHECK_EQ(&head_, tail_)
    }
    // SAFETY: 链上节点存活 (原版遍历契约)
    unsafe {
      let mut last = last;
      while !(*L::next(last)).is_null() {
        last = *L::next(last);
      }
      self.tail_ == Some(L::next(last)) // CHECK_EQ(TLTraits::next(last), tail_)
    }
  }

  /// inline void EnsureValidTail() const (原版 mutable tail_)
  pub fn ensure_valid_tail(&mut self) {
    if !SUPPORTS_UNSAFE_INSERTION {
      // DCHECK_EQ(*tail_, nullptr)
      debug_assert!(match self.tail_ {
        None => self.head_.is_null(),
        // SAFETY: 表不变量保证 Some(槽) 指向存活节点 next 槽
        Some(t) => unsafe { (*t).is_null() },
      });
      return;
    }
    // unsafe-insertion 表: AddAfter 可只改节点指针不更新表, 尾槽值可能非空。
    // if (*tail_ == nullptr) return; — None 态即 head 槽, 读 head_:
    let tail_val = match self.tail_ {
      None => self.head_,
      // SAFETY: 同上不变量
      Some(t) => unsafe { *t },
    };
    if tail_val.is_null() {
      return;
    }
    // T* last = *tail_; while (*next(last)) last = *next(last); tail_ = next(last);
    let mut last = tail_val;
    // SAFETY: 链上节点存活 (原版走链修复契约)
    unsafe {
      while !(*L::next(last)).is_null() {
        last = *L::next(last);
      }
      self.tail_ = Some(L::next(last));
    }
  }
}

/// struct EmptyBase {} — Rust 无继承; 保留标记类型以对照原版
#[derive(Debug, Clone, Copy, Default)]
pub struct EmptyBase;

/// Check ThreadedListBase::EnsureValidTail.
pub const K_UNSAFE_INSERTION: bool = true;

/// template <typename T, typename TLTraits = ThreadedListTraits<T>>
/// using ThreadedList = ThreadedListBase<T, EmptyBase, TLTraits>;
pub type ThreadedList<T, L = DefaultThreadedListTraits> = ThreadedListBase<T, L, false>;

/// using ThreadedListWithUnsafeInsertions = ThreadedListBase<T, EmptyBase, TLTraits, kUnsafeInsertion>;
pub type ThreadedListWithUnsafeInsertions<T, L = DefaultThreadedListTraits> =
  ThreadedListBase<T, L, K_UNSAFE_INSERTION>;
