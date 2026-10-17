// 由 tool/src/heap/base/worklist.js 从 v8/src/heap/base/worklist.h + worklist.cc 生成; 修订就地改, 复跑覆盖
// 并发工作队列: 段链 + 线程本地视图; 原版 v8::base::Mutex/哨兵段 → Rust Mutex/Option<Box<Segment>>

//! Worklist: 段(segment)式全局工作队列, 线程本地生产/消费 + 全局窃取。
//! 原版内存序: size_ 计数全程 memory_order_relaxed (对数核对通过: relaxed);
//! 段链表拓扑由 lock_ 全量互斥保护, 故 relaxed 计数只作近似读。
//! 哨兵段 (sentinel) → Option::None; Local 视图经 &Worklist 共享,
//! 故 top 链放 UnsafeCell, 仅在持锁期间访问 (锁纪律见 top_locked SAFETY)。

use std::{
  cell::UnsafeCell,
  mem,
  sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use v8_base_platform::Mutex;

/// 原 WorklistBase (worklist.cc:10-13): 全局可预测序开关 (单线程模式 push/pop 顺序可预期)。
pub struct WorklistBase;

static PREDICTABLE_ORDER: AtomicBool = AtomicBool::new(false);

impl WorklistBase {
  /// 原 EnforcePredictableOrder (worklist.cc:13)。
  pub fn enforce_predictable_order() {
    PREDICTABLE_ORDER.store(true, Ordering::Relaxed);
  }
  /// 原 PredictableOrder。
  pub fn predictable_order() -> bool {
    PREDICTABLE_ORDER.load(Ordering::Relaxed)
  }
}

/// 段: 有界条目栈 + 侵入式 next 链。
pub struct Segment<T> {
  next: Option<Box<Segment<T>>>,
  entries: Vec<T>,
}

impl<T> Segment<T> {
  /// 原 Segment::Create (worklist.h:218-233)。
  /// 偏差: 原版 AllocateAtLeast 超配容量优化省略, 恒按 min_segment_size 预留;
  /// with_capacity 允许实配 ≥ 请求 (与原版超配语义同向), is_full 按实配判定。
  pub fn create(min_segment_size: usize) -> Box<Self> {
    Box::new(Self {
      next: None,
      entries: Vec::with_capacity(min_segment_size),
    })
  }
  pub fn size(&self) -> usize {
    self.entries.len()
  }
  pub fn capacity(&self) -> usize {
    self.entries.capacity()
  }
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }
  pub fn is_full(&self) -> bool {
    self.entries.len() == self.entries.capacity()
  }
  /// 原 Clear: index_ = 0。
  pub fn clear(&mut self) {
    self.entries.clear()
  }
  pub fn push(&mut self, entry: T) {
    debug_assert!(!self.is_full());
    self.entries.push(entry);
  }
  pub fn pop(&mut self) -> Option<T> {
    debug_assert!(!self.is_empty());
    self.entries.pop()
  }
  /// 原 Iterate (worklist.h:294-300)。
  pub fn iterate(&self, callback: impl FnMut(&T)) {
    self.entries.iter().for_each(callback);
  }
}

impl<T: Copy> Segment<T> {
  /// 原 Update (worklist.h:283-291): 回调 (源条目, 目的槽) -> 是否保留; 槽内压缩。
  /// 偏差 (README 已登记): 目的槽预落位 `*dst = *src`, 保留项无须回调自写;
  /// 回调仍可覆写目的槽 (原版契约) — 单条目 i == new_index 时源目的同槽,
  /// 借临时副本承载, 故限 `T: Copy` (原版条目即指针语义)。
  pub fn update(&mut self, mut callback: impl FnMut(&mut T, &mut T) -> bool) {
    let entries = &mut self.entries;
    let mut new_index = 0usize;
    for i in 0..entries.len() {
      let mut src = entries[i];
      let mut dst = src;
      if callback(&mut src, &mut dst) {
        entries[new_index] = dst;
        new_index += 1;
      }
    }
    entries.truncate(new_index);
  }
}

/// 全局工作队列: 只统计已发布段。
pub struct Worklist<T, const MIN_SEGMENT_SIZE: usize> {
  /// 段链互斥; top (UnsafeCell) 仅在持本锁期间访问。
  lock: Mutex,
  top: UnsafeCell<Option<Box<Segment<T>>>>,
  size: AtomicUsize,
}

// SAFETY: top 仅在 lock 持有期间读写 (见 top_locked), 并发访问被锁全序化;
// T: Send 时段链跨线程移动安全。
unsafe impl<T: Send, const N: usize> Sync for Worklist<T, N> {}

impl<T, const MIN_SEGMENT_SIZE: usize> Worklist<T, MIN_SEGMENT_SIZE> {
  pub const K_MIN_SEGMENT_SIZE: usize = MIN_SEGMENT_SIZE;

  pub fn new() -> Self {
    Self {
      lock: Mutex::new(),
      top: UnsafeCell::new(None),
      size: AtomicUsize::new(0),
    }
  }

  /// 原 Push (worklist.h:106-112): 头插 + relaxed 计数 (锁内)。
  fn push_segment(&self, mut segment: Box<Segment<T>>) {
    debug_assert!(!segment.is_empty());
    let _g = self.lock.lock_guard();
    // SAFETY: g 持锁期间访问 UnsafeCell — top 仅在持锁期间读写 (结构不变式,
    // 见 Sync impl 的 SAFETY)。
    let top = unsafe { &mut *self.top.get() };
    segment.next = top.take();
    *top = Some(segment);
    self.size.fetch_add(1, Ordering::Relaxed);
  }

  /// 原 Pop (worklist.h:114-123)。
  fn pop_segment(&self) -> Option<Box<Segment<T>>> {
    let _g = self.lock.lock_guard();
    // SAFETY: g 持锁期间访问 UnsafeCell。
    let top = unsafe { &mut *self.top.get() };
    let mut head = top.take()?;
    *top = head.next.take();
    self.size.fetch_sub(1, Ordering::Relaxed);
    Some(head)
  }

  /// 原 IsEmpty (worklist.h:126-128)。
  pub fn is_empty(&self) -> bool {
    self.size() == 0
  }

  /// 原 Size (worklist.h:130-136): 无锁近似读。
  pub fn size(&self) -> usize {
    self.size.load(Ordering::Relaxed)
  }

  /// 原 Clear (worklist.h:139-149): 计数清零与摘链同锁
  /// (store(0) 不得覆盖并发 push 晚到的 fetch_add — 并发回归测试锚点)。
  pub fn clear(&self) {
    let _g = self.lock.lock_guard();
    self.size.store(0, Ordering::Relaxed);
    // SAFETY: g 持锁期间访问 UnsafeCell。
    let top = unsafe { &mut *self.top.get() };
    while let Some(mut seg) = top.take() {
      *top = seg.next.take();
    }
  }

  /// 原 Update (worklist.h:152-177): 全段压缩, 空段释放。
  pub fn update(&self, mut callback: impl FnMut(&mut T, &mut T) -> bool)
  where
    T: Copy,
  {
    let _g = self.lock.lock_guard();
    let mut num_deleted = 0usize;
    // SAFETY: g 持锁期间访问 UnsafeCell。
    let top = unsafe { &mut *self.top.get() };
    // 摘链 → 逐段压缩 → 逆序头插复原, 单次遍历无额外分配。
    let mut reversed = None;
    let mut current = top.take();
    while let Some(mut seg) = current {
      let next = seg.next.take();
      seg.update(&mut callback);
      if seg.is_empty() {
        num_deleted += 1;
      } else {
        seg.next = reversed;
        reversed = Some(seg);
      }
      current = next;
    }
    while let Some(mut seg) = reversed {
      let next = seg.next.take();
      seg.next = top.take();
      *top = Some(seg);
      reversed = next;
    }
    self.size.fetch_sub(num_deleted, Ordering::Relaxed);
  }

  /// 原 Iterate (worklist.h:179-186)。
  pub fn iterate(&self, mut callback: impl FnMut(&T)) {
    let _g = self.lock.lock_guard();
    // SAFETY: g 持锁期间访问 UnsafeCell; 只读遍历不改链。
    let mut current = unsafe { &*self.top.get() }.as_deref();
    while let Some(seg) = current {
      seg.iterate(|e| callback(e));
      current = seg.next.as_deref();
    }
  }

  /// 原 Merge (worklist.h:188-212): other 全部段搬入 this, other 置空。
  pub fn merge(&self, other: &Self) {
    let (mut other_top, other_size) = {
      let _g = other.lock.lock_guard();
      // SAFETY: g 持锁期间访问 UnsafeCell。
      let top = unsafe { &mut *other.top.get() };
      match top.take() {
        None => return,
        Some(head) => (Some(head), other.size.swap(0, Ordering::Relaxed)),
      }
    };
    // 链已从 other 独占摘出。逆序后逐段头插回 this, 借纯值语义绕开
    // &mut 链尾行走的借用环 (原 while (end->next()) end = end->next())。
    let mut reversed = None;
    while let Some(mut seg) = other_top {
      let next = seg.next.take();
      seg.next = reversed;
      reversed = Some(seg);
      other_top = next;
    }
    let _g = self.lock.lock_guard();
    self.size.fetch_add(other_size, Ordering::Relaxed);
    // SAFETY: g 持锁期间访问 UnsafeCell。
    let top = unsafe { &mut *self.top.get() };
    // 头插恢复原序: other 链整体位于 this 原链之前 (与原版拓扑一致)。
    while let Some(mut seg) = reversed {
      let next = seg.next.take();
      seg.next = top.take();
      *top = Some(seg);
      reversed = next;
    }
  }
}

impl<T, const MIN_SEGMENT_SIZE: usize> Default for Worklist<T, MIN_SEGMENT_SIZE> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T, const MIN_SEGMENT_SIZE: usize> Drop for Worklist<T, MIN_SEGMENT_SIZE> {
  /// 原 ~Worklist 的 CHECK(IsEmpty()) → debug_assert (禁 panic); 迭代式摘链防深递归。
  fn drop(&mut self) {
    debug_assert!(self.is_empty());
    let top = self.top.get_mut();
    while let Some(mut seg) = top.take() {
      *top = seg.next.take();
    }
  }
}

/// 线程本地视图: 未发布的 push/pop 段对外不可见。
/// 原 ItemType 别名 (worklist.h:306) 由泛型参数 `T` 直接承载。
pub struct Local<'a, T, const MIN_SEGMENT_SIZE: usize> {
  worklist: &'a Worklist<T, MIN_SEGMENT_SIZE>,
  push_segment: Option<Box<Segment<T>>>,
  pop_segment: Option<Box<Segment<T>>>,
}

impl<'a, T, const MIN_SEGMENT_SIZE: usize> Local<'a, T, MIN_SEGMENT_SIZE> {
  /// 原 Local (worklist.h:377-382): 哨兵态 = None。
  pub fn new(worklist: &'a Worklist<T, MIN_SEGMENT_SIZE>) -> Self {
    Self {
      worklist,
      push_segment: None,
      pop_segment: None,
    }
  }

  /// 原 Push (worklist.h:392-399): push 段满则先发布再换新段。
  pub fn push(&mut self, entry: T) {
    match &mut self.push_segment {
      Some(seg) if !seg.is_full() => seg.push(entry),
      _ => {
        self.publish_push_segment();
        let mut seg = Segment::create(MIN_SEGMENT_SIZE);
        seg.push(entry);
        self.push_segment = Some(seg);
      }
    }
  }

  /// 原 Pop (worklist.h:401-413): 本地段优先, 空则窃取全局段。
  pub fn pop(&mut self) -> Option<T> {
    let pop_empty = self.pop_segment.as_ref().is_none_or(|s| s.is_empty());
    if pop_empty {
      if self.push_segment.as_ref().is_some_and(|s| !s.is_empty()) {
        mem::swap(&mut self.push_segment, &mut self.pop_segment);
      } else if !self.steal_pop_segment() {
        return None;
      }
    }
    self.pop_segment.as_mut()?.pop()
  }

  /// 原 IsLocalAndGlobalEmpty (worklist.h:414-417)。
  pub fn is_local_and_global_empty(&self) -> bool {
    self.is_local_empty() && self.is_global_empty()
  }

  /// 原 IsLocalEmpty (worklist.h:419-422): 哨兵 (None) 视为空。
  pub fn is_local_empty(&self) -> bool {
    self.push_segment.as_ref().is_none_or(|s| s.is_empty())
      && self.pop_segment.as_ref().is_none_or(|s| s.is_empty())
  }

  /// 原 IsGlobalEmpty (worklist.h:424-426)。
  pub fn is_global_empty(&self) -> bool {
    self.worklist.is_empty()
  }

  /// 原 PushSegmentSize (worklist.h:330)。
  pub fn push_segment_size(&self) -> usize {
    self.push_segment.as_ref().map_or(0, |s| s.size())
  }

  /// 原 Publish (worklist.h:429-439): 非空段全部发布, 视图回哨兵态。
  pub fn publish(&mut self) {
    if self.push_segment.as_ref().is_some_and(|s| !s.is_empty()) {
      self.publish_push_segment();
      self.push_segment = None;
    }
    if self.pop_segment.as_ref().is_some_and(|s| !s.is_empty()) {
      self.publish_pop_segment();
      self.pop_segment = None;
    }
  }

  /// 原 Merge (worklist.h:441-445): 合并的是两个全局池。
  pub fn merge(&mut self, other: &mut Self) {
    self.worklist.merge(other.worklist);
  }

  /// 原 Clear (worklist.h:473-477): 清空本地段条目。
  pub fn clear(&mut self) {
    if let Some(seg) = &mut self.push_segment {
      seg.clear();
    }
    if let Some(seg) = &mut self.pop_segment {
      seg.clear();
    }
  }

  /// 原 PublishPushSegment (worklist.h:447-452)。
  fn publish_push_segment(&mut self) {
    if let Some(seg) = self.push_segment.take() {
      self.worklist.push_segment(seg);
    }
  }

  /// 原 PublishPopSegment (worklist.h:454-459)。
  fn publish_pop_segment(&mut self) {
    if let Some(seg) = self.pop_segment.take() {
      self.worklist.push_segment(seg);
    }
  }

  /// 原 StealPopSegment (worklist.h:461-471)。
  fn steal_pop_segment(&mut self) -> bool {
    if self.worklist.is_empty() {
      return false;
    }
    match self.worklist.pop_segment() {
      Some(seg) => {
        self.pop_segment = Some(seg);
        true
      }
      None => false,
    }
  }
}

impl<T, const MIN_SEGMENT_SIZE: usize> Drop for Local<'_, T, MIN_SEGMENT_SIZE> {
  /// 原 ~Local 的 CHECK_IMPLIES(段非空) → debug_assert (禁 panic);
  /// 段经 Box 语义自动释放, 无泄漏。
  fn drop(&mut self) {
    debug_assert!(self.push_segment.as_ref().is_none_or(|s| s.is_empty()));
    debug_assert!(self.pop_segment.as_ref().is_none_or(|s| s.is_empty()));
  }
}
