//! 分配观察者。
//! 对应原版 v8/src/heap/allocation-observer.h (124 行) + allocation-observer.cc (174 行)。
//!
//! 落地范围 (与 tool/src/heap/allocation-observer.h.js + allocation-observer.cc.js 锚逐项对齐):
//! - [`AllocationObserver`] 基类面 7 项 (原版 h:18-51) → Rust trait + 手动 fn 指针虚表 (本仓禁 dyn)。
//! - [`AllocationCounter`] 9 方法 + 内嵌 [`AllocationObserverCounter`] + 7 字段 (原版 h:54-107),
//!   逻辑体逐行承接 cc:15-159。
//! - [`PauseAllocationObserversScope`] 3 项 (原版 h:109-119) + cc:161-171 → 窄 [`PauseEnv`] trait 承接。
//!
//! 耦合推迟登记 (与主控任务差集 ③ 一致, 双行号):
//! PauseScope 构造/析构依赖 `Heap::gc_state()`/`Heap::allocator()`/`pause_allocation_observers_depth_`
//! — 属 Heap 票面。本 crate 仅定义 [`PauseEnv`] 窄 trait 承接调用面 (原版 cc:161-171 ↔ 本文件
//! `PauseAllocationObserversScope` 区), 具体 Heap 实现推迟至 Heap 票, 禁假绿。

use std::{cmp::min, collections::HashSet};

use v8_base::logging::check_with_msg;
use v8_common::Address;

/// 观察者不再使用固定步长的哨兵值。原版 h:20 `kNotUsingFixedStepSize = -1` (intptr_t)。
pub const K_NOT_USING_FIXED_STEP_SIZE: isize = -1;

/// 观察者虚表: 等价 C++ 虚表槽, 以 fn 指针承接异构观察者的多态调用 (本仓禁 dyn)。
type StepFn = unsafe fn(this: *mut (), bytes_allocated: i32, soon_object: Address, size: usize);
type NextStepSizeFn = unsafe fn(this: *mut ()) -> isize;

/// 分配观察者基类。原版 h:18-51 `class AllocationObserver`。
///
/// - `step` = 原版纯虚 `Step` (h:39): 至少 `step_size` 字节被分配后回调。
/// - `next_step_size` = 原版虚 `GetNextStepSize` (h:42-45): 默认返回 `step_size`, 子类可覆写。
/// - `step_size` = 原版字段 `step_size_` (h:48, const)。
pub trait AllocationObserver {
  /// 原版 h:39 纯虚 `virtual void Step(int bytes_allocated, Address soon_object, size_t size) = 0`。
  ///
  /// 1. `soon_object` 在页边界填充对象等情形下可能为 nullptr (原版 h:32-33)。
  /// 2. 折叠分配组时 size 为组大小 (原版 h:34-35)。
  /// 3. size 是分配时请求大小, 右修剪可动态改变对象大小 (原版 h:36-38)。
  fn step(&mut self, bytes_allocated: i32, soon_object: Address, size: usize);

  /// 原版 h:21 ctor 字段 `step_size_` (h:48 const intptr_t) 的读取面。
  fn step_size(&self) -> isize;

  /// 原版 h:42-45 虚 `GetNextStepSize` 默认实现: DCHECK_NE(kNotUsingFixedStepSize, step_size_)。
  /// Rust 以带默认实现的方法承接; 子类覆写以动态化步长 (原版 h:41)。
  fn next_step_size(&self) -> isize {
    debug_assert_ne!(
      self.step_size(),
      K_NOT_USING_FIXED_STEP_SIZE,
      "默认 GetNextStepSize 不得作用于 kNotUsingFixedStepSize (原版 h:43 DCHECK_NE)"
    );
    self.step_size()
  }
}

/// 类型擦除观察者引用: 存对象裸指针 + 按具体类型生成的虚表 fn 指针 (等价 C++ 基类指针 vtable)。
///
/// 身份以指针值判定 (原版以 `AllocationObserver*` 指针相等比较, 见 cc:19/48)。
#[derive(Clone, Copy)]
pub struct ObserverRef {
  /// 观察者实例裸指针 (原版 AllocationObserver*)。
  ptr: *mut (),
  step: StepFn,
  next_step_size: NextStepSizeFn,
}

impl ObserverRef {
  /// 由具体观察者 `&mut T` 构造引用: 按 T 单态化生成虚表 fn 指针 (零 dyn, 零堆分配)。
  pub fn new<T: AllocationObserver>(observer: &mut T) -> Self {
    // 原版虚表槽 Step: 转发到 T::step。
    // SAFETY: `this` 由本构造函数取自存活的 &mut T, 单态化后与 ptr 同类型; 生命周期由持有者
    // (AllocationCounter, 上层保证) 维持, 调用点均在对象存活期内。
    unsafe fn step_disp<T: AllocationObserver>(
      this: *mut (),
      bytes_allocated: i32,
      soon_object: Address,
      size: usize,
    ) {
      // SAFETY: 见 `ObserverRef::new` 的 SAFETY 注释, this 指向存活的 T。
      unsafe { (*this.cast::<T>()).step(bytes_allocated, soon_object, size) }
    }
    // 原版虚表槽 GetNextStepSize: 转发到 T::next_step_size。
    // SAFETY: 同上契约。
    unsafe fn nss_disp<T: AllocationObserver>(this: *mut ()) -> isize {
      // SAFETY: 见 `ObserverRef::new` 的 SAFETY 注释, this 指向存活的 T。
      unsafe { (*this.cast::<T>()).next_step_size() }
    }
    Self {
      ptr: observer as *mut T as *mut (),
      step: step_disp::<T>,
      next_step_size: nss_disp::<T>,
    }
  }

  /// 原版 cc:100 `aoc.observer_->Step(...)`。
  ///
  /// # Safety
  /// 调用方须保证 `self.ptr` 指向的观察者仍存活, 且当前无非别名的可变借用活跃
  /// (AllocationCounter 内单例持有, 迭代期间不外借该观察者)。
  unsafe fn invoke_step(&mut self, bytes_allocated: i32, soon_object: Address, size: usize) {
    // SAFETY: 由本方法 Safety 契约保证 ptr 有效且可变借用独占。
    unsafe { (self.step)(self.ptr, bytes_allocated, soon_object, size) }
  }

  /// 原版 cc:104/121 `aoc.observer_->GetNextStepSize()`。
  ///
  /// # Safety
  /// 调用方须保证 `self.ptr` 指向的观察者仍存活 (只读访问)。
  unsafe fn invoke_next_step_size(&self) -> isize {
    // SAFETY: 由本方法 Safety 契约保证 ptr 有效。
    unsafe { (self.next_step_size)(self.ptr) }
  }

  /// 身份键 (指针地址), 用于 pending_removed_ 集合与重复注册预检 (原版以指针相等比较)。
  #[inline]
  fn key(&self) -> usize {
    self.ptr as usize
  }
}

/// 原版 h:88-98 内嵌 `struct AllocationObserverCounter final`。
struct AllocationObserverCounter {
  observer_: ObserverRef,
  prev_counter_: usize,
  next_counter_: usize,
}

impl AllocationObserverCounter {
  /// 原版 h:89-93 ctor(observer, prev_counter, next_counter)。
  #[inline]
  fn new(observer: ObserverRef, prev_counter: usize, next_counter: usize) -> Self {
    Self {
      observer_: observer,
      prev_counter_: prev_counter,
      next_counter_: next_counter,
    }
  }
}

/// 可挂载观察者的全局分配计数器。原版 h:54-107 `class AllocationCounter final`。
///
/// 逻辑体 1:1 承接 cc:15-159 (纯计数逻辑, 无 Isolate)。
pub struct AllocationCounter {
  /// 原版 h:100 `std::vector<AllocationObserverCounter> observers_`。
  observers_: Vec<AllocationObserverCounter>,
  /// 原版 h:101 `std::vector<AllocationObserverCounter> pending_added_` (Step 进行中新增缓冲)。
  pending_added_: Vec<AllocationObserverCounter>,
  /// 原版 h:102 `std::unordered_set<AllocationObserver*> pending_removed_`, 键为观察者指针地址。
  pending_removed_: HashSet<usize>,
  /// 原版 h:104 `size_t current_counter_ = 0`。
  current_counter_: usize,
  /// 原版 h:105 `size_t next_counter_ = 0`。
  next_counter_: usize,
  /// 原版 h:106 `bool step_in_progress_ = false`。
  step_in_progress_: bool,
}

impl Default for AllocationCounter {
  /// 原版 h:56 `AllocationCounter() = default` — 字段默认初值 (h:104-106)。
  fn default() -> Self {
    Self {
      observers_: Vec::new(),
      pending_added_: Vec::new(),
      pending_removed_: HashSet::new(),
      current_counter_: 0,
      next_counter_: 0,
      step_in_progress_: false,
    }
  }
}

impl AllocationCounter {
  /// 原版 h:59 声明 / cc:15-43 实现 `AddAllocationObserver`。May be called from Step (pending 队列承接)。
  pub fn add_allocation_observer(&mut self, observer: ObserverRef) {
    #[cfg(debug_assertions)]
    {
      // 原版 cc:16-22 #if DEBUG find_if 重复注册预检: DCHECK_EQ(observers_.end(), it)
      debug_assert!(
        !self
          .observers_
          .iter()
          .any(|aoc| aoc.observer_.key() == observer.key()),
        "观察者重复注册 (原版 cc:21 DCHECK_EQ)"
      );
    }

    // 原版 cc:24-27 Step 进行中 → 入 pending 队列 (哨兵 prev=next=0), 直接返回。
    if self.step_in_progress_ {
      self
        .pending_added_
        .push(AllocationObserverCounter::new(observer, 0, 0));
      return;
    }

    // 原版 cc:29-30 step_size = observer->GetNextStepSize(); observer_next_counter = cur + step_size
    // SAFETY: 非 Step 期, 观察者存活, 只读调用。
    let step_size = unsafe { observer.invoke_next_step_size() };
    let observer_next_counter = (self.current_counter_ as isize + step_size) as usize;

    // 原版 cc:32-33 observers_.push_back(AOC(observer, current, observer_next))
    self.observers_.push(AllocationObserverCounter::new(
      observer,
      self.current_counter_,
      observer_next_counter,
    ));

    if self.observers_.len() == 1 {
      // 原版 cc:35-37 DCHECK_EQ(current,next); next = observer_next_counter
      debug_assert_eq!(
        self.current_counter_, self.next_counter_,
        "唯一观察者时 cur==next (原版 cc:36 DCHECK_EQ)"
      );
      self.next_counter_ = observer_next_counter;
    } else {
      // 原版 cc:39-42 missing = next - current; next = current + min((intptr)missing, step_size)
      let missing_bytes = self.next_counter_ - self.current_counter_;
      let folded = min(missing_bytes as isize, step_size);
      self.next_counter_ = (self.current_counter_ as isize + folded) as usize;
    }
  }

  /// 原版 h:62 声明 / cc:45-73 实现 `RemoveAllocationObserver`。May be called from Step。
  pub fn remove_allocation_observer(&mut self, observer: ObserverRef) {
    // 原版 cc:46-50 find_if + DCHECK_NE(end, it) — 须存在。
    let idx = self
      .observers_
      .iter()
      .position(|aoc| aoc.observer_.key() == observer.key());
    debug_assert!(idx.is_some(), "移除未注册的观察者 (原版 cc:50 DCHECK_NE)");

    // 原版 cc:52-56 Step 进行中 → 入 pending_removed_ (DCHECK 未重复), 返回。
    if self.step_in_progress_ {
      // 原版 cc:53 DCHECK_EQ(pending_removed_.count(observer), 0)
      debug_assert!(
        !self.pending_removed_.contains(&observer.key()),
        "Step 期内重复登记移除 (原版 cc:53 DCHECK_EQ)"
      );
      self.pending_removed_.insert(observer.key());
      return;
    }

    if let Some(i) = idx {
      self.observers_.remove(i);
    }

    if self.observers_.is_empty() {
      // 原版 cc:60-61 current = next = 0
      self.current_counter_ = 0;
      self.next_counter_ = 0;
    } else {
      // 原版 cc:63-71 重算 step_size: 各观察者剩余步长的最小值
      let mut step_size = 0usize;
      for oc in &self.observers_ {
        let left_in_step = oc.next_counter_ - self.current_counter_;
        // 原版 cc:67 DCHECK_GT(left_in_step, 0)
        debug_assert!(left_in_step > 0, "剩余步长须正 (原版 cc:67 DCHECK_GT)");
        step_size = if step_size != 0 {
          min(step_size, left_in_step)
        } else {
          left_in_step
        };
      }
      self.next_counter_ = self.current_counter_ + step_size;
    }
  }

  /// 原版 h:65 声明 / cc:75-80 实现 `AdvanceAllocationObservers(allocated)`。不触发任何观察者。
  pub fn advance_allocation_observers(&mut self, allocated: usize) {
    // 原版 cc:76 if (observers_.empty()) return
    if self.observers_.is_empty() {
      return;
    }
    debug_assert!(
      !self.step_in_progress_,
      "Advance 不得在 Step 期 (原版 cc:77 DCHECK)"
    );
    // 原版 cc:78 DCHECK_LT(allocated, next - current)
    debug_assert!(
      allocated < self.next_counter_ - self.current_counter_,
      "Advance 量须小于剩余 (原版 cc:78 DCHECK_LT)"
    );
    self.current_counter_ += allocated;
  }

  /// 原版 h:67-69 声明 / cc:82-159 实现 `InvokeAllocationObservers(soon_object, object_size, aligned_object_size)`。
  /// 触发到期的 Step 回调并计算新步长; 不推进 current_counter_。
  pub fn invoke_allocation_observers(
    &mut self,
    soon_object: Address,
    object_size: usize,
    aligned_object_size: usize,
  ) {
    // 原版 cc:85 if (observers_.empty()) return
    if self.observers_.is_empty() {
      return;
    }
    debug_assert!(
      !self.step_in_progress_,
      "Invoke 不得重入 (原版 cc:86 DCHECK)"
    );
    // 原版 cc:87 DCHECK_GE(aligned_object_size, next - current)
    debug_assert!(
      aligned_object_size >= self.next_counter_ - self.current_counter_,
      "对齐大小须覆盖剩余步长 (原版 cc:87 DCHECK_GE)"
    );
    // 原版 cc:88 DCHECK(soon_object) — 非零地址
    debug_assert_ne!(soon_object, 0, "soon_object 须非零 (原版 cc:88 DCHECK)");

    let mut step_run = false;
    self.step_in_progress_ = true;
    let mut step_size = 0usize;

    // 原版 cc:93-94 pending 队列 Invoke 前须空
    debug_assert!(self.pending_added_.is_empty(), "原版 cc:93 DCHECK");
    debug_assert!(self.pending_removed_.is_empty(), "原版 cc:94 DCHECK");

    // 原版 cc:96-114 逐观察者: 到期则回调 Step 并推进其 next; 折叠最小剩余步长。
    // 注: cc:99 `DisallowGarbageCollection no_gc` 属 Heap 票面 → 推迟登记 (见模块头),
    // Rust 侧回调期禁 GC 的语义由上层在具体 Step 实现内保证; 此处仅承接计数/回调编排。
    let current = self.current_counter_;
    for aoc in &mut self.observers_ {
      if aoc.next_counter_ - current <= aligned_object_size {
        // 原版 cc:100-102 Step((int)(current - aoc.prev), soon_object, object_size)
        let bytes_allocated = (current - aoc.prev_counter_) as i32;
        // SAFETY: Step 期观察者由上层保证存活, 且本 Rust 模型下 Step 不可重入 counter
        // (借用检查阻止), aoc 的可变借用仅在指针转发处, 转发后即归还给 T 自身。
        unsafe {
          aoc
            .observer_
            .invoke_step(bytes_allocated, soon_object, object_size)
        };

        // 原版 cc:104 observer_step_size = observer->GetNextStepSize()
        // SAFETY: 同上, 只读调用, 观察者存活。
        let observer_step_size = unsafe { aoc.observer_.invoke_next_step_size() };

        // 原版 cc:106-108 prev = current; next = current + aligned + observer_step_size
        aoc.prev_counter_ = current;
        aoc.next_counter_ =
          (current as isize + aligned_object_size as isize + observer_step_size) as usize;
        step_run = true;
      }

      // 原版 cc:112-113 left = next - current; step_size = step_size ? min(step_size, left) : left
      let left_in_step = aoc.next_counter_ - current;
      step_size = if step_size != 0 {
        min(step_size, left_in_step)
      } else {
        left_in_step
      };
    }

    // 原版 cc:116 CHECK(step_run) — 硬断言 (abort, 非 panic), 承接为 check_with_msg。
    check_with_msg(step_run, "step_run");

    // 原版 cc:118-130 处理 Step 期新增观察者 (pending_added_)。
    for aoc in &mut self.pending_added_ {
      // 原版 cc:120 DCHECK_EQ(0, aoc.next_counter_) — 哨兵
      debug_assert_eq!(
        aoc.next_counter_, 0,
        "pending 新增须哨兵 next=0 (原版 cc:120 DCHECK_EQ)"
      );
      // SAFETY: 新增观察者存活, 只读调用。
      let observer_step_size = unsafe { aoc.observer_.invoke_next_step_size() };
      aoc.prev_counter_ = current;
      aoc.next_counter_ =
        (current as isize + aligned_object_size as isize + observer_step_size) as usize;

      // 原版 cc:126 DCHECK_NE(step_size, 0)
      debug_assert_ne!(step_size, 0, "原版 cc:126 DCHECK_NE");
      // 原版 cc:127 step_size = min(step_size, aligned + observer_step_size)
      let candidate = (aligned_object_size as isize + observer_step_size) as usize;
      step_size = min(step_size, candidate);
    }
    // 原版 cc:129 observers_.push_back(aoc); cc:132 pending_added_.clear()
    self.observers_.append(&mut self.pending_added_);

    // 原版 cc:134-155 pending_removed_ 批量擦除 + 重算步长。
    if !self.pending_removed_.is_empty() {
      self
        .observers_
        .retain(|aoc| !self.pending_removed_.contains(&aoc.observer_.key()));
      self.pending_removed_.clear();

      // 原版 cc:144-148 重算 step_size
      step_size = 0;
      for aoc in &self.observers_ {
        let left_in_step = aoc.next_counter_ - current;
        step_size = if step_size != 0 {
          min(step_size, left_in_step)
        } else {
          left_in_step
        };
      }

      // 原版 cc:150-154 观察者清空则归零并结束 Step (提前 return)。
      if self.observers_.is_empty() {
        self.next_counter_ = 0;
        self.current_counter_ = 0;
        self.step_in_progress_ = false;
        return;
      }
    }

    // 原版 cc:157-158 next = current + step_size; step_in_progress = false
    self.next_counter_ = current + step_size;
    self.step_in_progress_ = false;
  }

  /// 原版 h:73 `bool IsStepInProgress() const`。
  #[inline]
  #[must_use]
  pub fn is_step_in_progress(&self) -> bool {
    self.step_in_progress_
  }

  /// 原版 h:75-78 `size_t NextBytes() const` — 空集返回 SIZE_MAX。
  #[inline]
  #[must_use]
  pub fn next_bytes(&self) -> usize {
    if self.observers_.is_empty() {
      return usize::MAX;
    }
    self.next_counter_ - self.current_counter_
  }

  /// 原版 h:80-85 `HasAllocationObservers()` — DEBUG-only (#if DEBUG)。
  #[cfg(debug_assertions)]
  #[must_use]
  pub fn has_allocation_observers(&self) -> bool {
    !self.observers_.is_empty()
      || !self.pending_added_.is_empty()
      || !self.pending_removed_.is_empty()
  }
}

/// 暂停分配观察者的窄环境接口。原版 PauseScope 依赖的 `Heap` 调用面承接点 (cc:163-170)。
///
/// 具体实现 (`gc_state`/`allocator()->Pause|Resume`/`pause_allocation_observers_depth_`) 属 Heap 票面,
/// 本 crate 只定义调用面 (推迟登记见模块头, 原版 cc:161-171 ↔ 本 trait 与本文件 PauseScope 区)。
pub trait PauseEnv {
  /// 原版 cc:163 `DCHECK_EQ(heap->gc_state(), Heap::NOT_IN_GC)` 的判定面: 当前须不在 GC 中。
  fn gc_state_is_not_in_gc(&self) -> bool;
  /// 原版 cc:164 `heap->allocator()->PauseAllocationObservers()`。
  fn pause_allocation_observers(&mut self);
  /// 原版 cc:165 `heap_->pause_allocation_observers_depth_++`。
  fn increment_pause_depth(&mut self);
  /// 原版 cc:169 `heap_->pause_allocation_observers_depth_--`。
  fn decrement_pause_depth(&mut self);
  /// 原版 cc:170 `heap_->allocator()->ResumeAllocationObservers()`。
  fn resume_allocation_observers(&mut self);
}

/// 暂停分配观察者作用域 (RAII)。原版 h:109-119 `class V8_EXPORT_PRIVATE V8_NODISCARD PauseAllocationObserversScope`。
///
/// 进入即暂停, Drop 即恢复 (cc:161-171 实现 → Rust Drop trait, RAII 语义等价)。
/// `#[must_use]` 承接原版 V8_NODISCARD (h:109): 丢弃即立即 Drop, 无暂停效果。
#[must_use = "原版 V8_NODISCARD h:109: 临时量会被立即 Drop, 暂停无任何效果"]
pub struct PauseAllocationObserversScope<'a, P: PauseEnv> {
  /// 原版 h:118 `Heap* heap_` → 借用窄环境。
  env: &'a mut P,
}

impl<'a, P: PauseEnv> PauseAllocationObserversScope<'a, P> {
  /// 原版 h:111 声明 / cc:161-166 实现 ctor(Heap*): DCHECK gc_state + allocator pause + depth++。
  pub fn new(env: &'a mut P) -> Self {
    // 原版 cc:163 DCHECK_EQ(heap->gc_state(), NOT_IN_GC)
    debug_assert!(
      env.gc_state_is_not_in_gc(),
      "PauseScope 须在非 GC 态构造 (原版 cc:163 DCHECK_EQ)"
    );
    env.pause_allocation_observers();
    env.increment_pause_depth();
    Self { env }
  }
}

impl<P: PauseEnv> Drop for PauseAllocationObserversScope<'_, P> {
  /// 原版 h:112 声明 / cc:168-171 实现 dtor: depth-- + allocator resume。
  fn drop(&mut self) {
    self.env.decrement_pause_depth();
    self.env.resume_allocation_observers();
  }
}
