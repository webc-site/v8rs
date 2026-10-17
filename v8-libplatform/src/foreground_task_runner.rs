//! 对应 `v8/src/libplatform/default-foreground-task-runner.{h,cc}`：
//! 前台消息循环任务运行器（best-effort 顺序 + NonNestable 语义）。
//!
//! 并发纪律沿用原版：`state` 只在持有 `mutex`（base::Mutex）时访问；
//! `nesting_depth_` 原版即由 RunTaskScope 无锁增减（前台单线程假设），
//! Rust 以 AtomicI32 承接 Sync 约束。UnsafeCell 同 v8-base-platform 内部先例。

use std::{
  cell::UnsafeCell,
  cmp::{Ordering, Reverse},
  collections::{BinaryHeap, VecDeque},
  mem::take,
  sync::{
    Arc,
    atomic::{AtomicI32, Ordering as AtomicOrdering},
  },
};

use v8_base_platform::{ConditionVariable, Mutex, TimeConstants, TimeDelta};

use crate::api::{
  BoxIdleTask, BoxTask, IdleTaskSupport, MessageLoopBehavior, SourceLocation, TaskRunner,
  TimeFunction,
};

/// 对应原私有枚举 `Nestability { kNestable, kNonNestable }`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Nestability {
  Nestable,
  NonNestable,
}

/// 延迟队列条目；对应原私有 `struct DelayedEntry`。
struct DelayedEntry {
  timeout_time: f64,
  nestability: Nestability,
  /// 同刻延迟任务的投递序（原版 priority_queue 并列序未定义，此处显式化保确定性）。
  seq: u64,
  task: BoxTask,
}

impl PartialEq for DelayedEntry {
  fn eq(&self, other: &Self) -> bool {
    self.timeout_time.total_cmp(&other.timeout_time) == Ordering::Equal && self.seq == other.seq
  }
}

impl Eq for DelayedEntry {}

impl PartialOrd for DelayedEntry {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for DelayedEntry {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .timeout_time
      .total_cmp(&other.timeout_time)
      .then_with(|| self.seq.cmp(&other.seq))
  }
}

/// 锁保护的状态体；对应原 `task_queue_` / `idle_task_queue_` / `delayed_task_queue_` /
/// `terminated_`。
struct RunnerState {
  terminated: bool,
  task_queue: VecDeque<(Nestability, BoxTask)>,
  idle_task_queue: VecDeque<BoxIdleTask>,
  /// timeout 升序最小堆（对应原 priority_queue + 反向比较子）。
  delayed_task_queue: BinaryHeap<Reverse<DelayedEntry>>,
  next_seq: u64,
}

/// 对应原 `class DefaultForegroundTaskRunner`。
pub struct DefaultForegroundTaskRunner {
  mutex: Mutex,
  event_loop_control: ConditionVariable,
  nesting_depth: AtomicI32,
  state: UnsafeCell<RunnerState>,
  idle_task_support: IdleTaskSupport,
  time_function: TimeFunction,
}

// SAFETY: `state` 只在持有 `mutex` 时读写；任务本体受 `BoxTask: Send` 约束。
unsafe impl Send for DefaultForegroundTaskRunner {}
// SAFETY: 同上。
unsafe impl Sync for DefaultForegroundTaskRunner {}

/// 对应原嵌套类 `RunTaskScope`：执行任务期间持有，递增嵌套深度。
/// 原版 h:23 `class V8_NODISCARD RunTaskScope` — 丢弃式构造即时恢复深度、静默失去嵌套保护 (复审 WLR3 W3-6a)
#[must_use = "丢弃 RunTaskScope 会立刻递减嵌套深度, 失去执行期保护 (原版 V8_NODISCARD)"]
pub struct RunTaskScope {
  runner: Arc<DefaultForegroundTaskRunner>,
}

impl RunTaskScope {
  /// 对应原构造：`nesting_depth_++`。
  pub fn new(runner: Arc<DefaultForegroundTaskRunner>) -> Self {
    let prev = runner.nesting_depth.fetch_add(1, AtomicOrdering::Relaxed);
    debug_assert!(prev >= 0);
    Self { runner }
  }
}

impl Drop for RunTaskScope {
  fn drop(&mut self) {
    let prev = self
      .runner
      .nesting_depth
      .fetch_sub(1, AtomicOrdering::Relaxed);
    debug_assert!(prev > 0);
  }
}

impl DefaultForegroundTaskRunner {
  /// 对应原构造。
  pub fn new(idle_task_support: IdleTaskSupport, time_function: TimeFunction) -> Self {
    Self {
      mutex: Mutex::new(),
      event_loop_control: ConditionVariable::new(),
      nesting_depth: AtomicI32::new(0),
      state: UnsafeCell::new(RunnerState {
        terminated: false,
        task_queue: VecDeque::new(),
        idle_task_queue: VecDeque::new(),
        delayed_task_queue: BinaryHeap::new(),
        next_seq: 0,
      }),
      idle_task_support,
      time_function,
    }
  }

  /// 对应原 `MonotonicallyIncreasingTime`。
  pub fn monotonically_increasing_time(&self) -> f64 {
    (self.time_function)()
  }

  /// 对应原 `Terminate`：队列整体换出后在锁外销毁任务，避免死锁（同原版注释）。
  pub fn terminate(&self) {
    let obsolete_tasks;
    let obsolete_delayed_tasks;
    let obsolete_idle_tasks;
    {
      let _guard = self.lock();
      // SAFETY: 已持有 `mutex`。
      let state = unsafe { &mut *self.state.get() };
      state.terminated = true;
      obsolete_tasks = take(&mut state.task_queue);
      obsolete_delayed_tasks = take(&mut state.delayed_task_queue);
      obsolete_idle_tasks = take(&mut state.idle_task_queue);
    }
    drop((obsolete_tasks, obsolete_delayed_tasks, obsolete_idle_tasks));
  }

  /// 对应原 `PopTaskFromQueue`：先到期先搬运，嵌套态下只弹 nestable 任务。
  pub fn pop_task_from_queue(&self, wait_for_work: MessageLoopBehavior) -> Option<BoxTask> {
    // 销毁次序对齐原版: h:69-74 明文交回任务须"after releasing the lock"销毁,
    // .cc:142-143 tasks_to_delete 先于 guard 声明; Rust Drop 为声明逆序,
    // 故本声明序使任务在锁释放后才销毁 (滞留任务 Drop 重入同 runner 不死锁)。
    let mut tasks_to_delete = Vec::new();
    let _guard = self.lock();
    tasks_to_delete.extend(self.move_expired_delayed_tasks_locked());

    while !self.has_poppable_task_in_queue() {
      if wait_for_work == MessageLoopBehavior::DoNotWait {
        return None;
      }
      self.wait_for_task_locked();
      tasks_to_delete.extend(self.move_expired_delayed_tasks_locked());
    }

    // SAFETY: 已持有 `mutex`。
    let state = unsafe { &mut *self.state.get() };
    let depth = self.nesting_depth.load(AtomicOrdering::Relaxed);
    let idx = state
      .task_queue
      .iter()
      .position(|(nestability, _)| depth == 0 || *nestability == Nestability::Nestable);
    // 对应原 DCHECK(it != task_queue_.end())：上方谓词已保证存在，缺失时按 None 返回。
    let task = idx
      .and_then(|i| state.task_queue.remove(i))
      .map(|(_nestability, task)| task);
    // `tasks_to_delete` 随函数出口在锁释放后销毁（声明先于 _guard，Drop 逆序），同原版。
    task
  }

  /// 对应原 `PopTaskFromIdleQueue`。
  pub fn pop_task_from_idle_queue(&self) -> Option<BoxIdleTask> {
    let _guard = self.lock();
    // SAFETY: 已持有 `mutex`。
    unsafe { &mut *self.state.get() }
      .idle_task_queue
      .pop_front()
  }

  /// 持锁便捷方法；对应原 `base::MutexGuard guard(&mutex_)`。
  fn lock(&self) -> v8_base_platform::MutexGuard<'_> {
    self.mutex.lock_guard()
  }

  /// 对应原 `PostTaskLocked`：终止态下把任务原样交回（锁外销毁）。
  fn post_task_locked(&self, task: BoxTask, nestability: Nestability) -> Option<BoxTask> {
    self.mutex.assert_held();
    // SAFETY: 已持有 `mutex`。
    let state = unsafe { &mut *self.state.get() };
    if state.terminated {
      return Some(task);
    }
    state.task_queue.push_back((nestability, task));
    self.event_loop_control.notify_one();
    None
  }

  /// 对应原 `PostDelayedTaskLocked`。
  fn post_delayed_task_locked(
    &self,
    task: BoxTask,
    delay_in_seconds: f64,
    nestability: Nestability,
  ) {
    self.mutex.assert_held();
    debug_assert!(delay_in_seconds >= 0.0);
    // SAFETY: 已持有 `mutex`。
    let state = unsafe { &mut *self.state.get() };
    if state.terminated {
      return;
    }
    let deadline = self.monotonically_increasing_time() + delay_in_seconds;
    let seq = state.next_seq;
    state.next_seq += 1;
    state.delayed_task_queue.push(Reverse(DelayedEntry {
      timeout_time: deadline,
      nestability,
      seq,
      task,
    }));
    self.event_loop_control.notify_one();
  }

  /// 对应原 `PopTaskFromDelayedQueueLocked`：仅弹已到期最早项。
  fn pop_task_from_delayed_queue_locked(&self) -> Option<(BoxTask, Nestability)> {
    self.mutex.assert_held();
    // SAFETY: 已持有 `mutex`。
    let state = unsafe { &mut *self.state.get() };
    // 原版 .cc:171-176 顺序: 空堆先短路返回, 再取时基——注入时钟不自推空拨 (复审 WLR2 F2)
    if state.delayed_task_queue.peek()?.0.timeout_time > self.monotonically_increasing_time() {
      return None;
    }
    let Reverse(entry) = state.delayed_task_queue.pop()?;
    Some((entry.task, entry.nestability))
  }

  /// 对应原 `HasPoppableTaskInQueue`：非嵌套态任何任务可弹；嵌套态仅 nestable。
  fn has_poppable_task_in_queue(&self) -> bool {
    // SAFETY: 调用方持有 `mutex`（本方法只读）。
    let state = unsafe { &*self.state.get() };
    if self.nesting_depth.load(AtomicOrdering::Relaxed) == 0 {
      return !state.task_queue.is_empty();
    }
    state
      .task_queue
      .iter()
      .any(|(nestability, _)| *nestability == Nestability::Nestable)
  }

  /// 对应原 `MoveExpiredDelayedTasksLocked`：到期任务转即时队列；
  /// 已终止时交回的任务收集于此（调用方锁外销毁，对齐 .cc:142 收集序语义）。
  fn move_expired_delayed_tasks_locked(&self) -> Vec<BoxTask> {
    let mut expired_tasks_to_delete = Vec::new();
    while let Some((task, nestability)) = self.pop_task_from_delayed_queue_locked() {
      if let Some(to_delete) = self.post_task_locked(task, nestability) {
        expired_tasks_to_delete.push(to_delete);
      }
    }
    expired_tasks_to_delete
  }

  /// 对应原 `WaitForTaskLocked`：有延迟任务则限时等待到最早到期，否则无限期。
  fn wait_for_task_locked(&self) {
    self.mutex.assert_held();
    // SAFETY: 已持有 `mutex`（只读快照即可，fn 指针为 Copy）。
    let earliest = unsafe { &*self.state.get() }
      .delayed_task_queue
      .peek()
      .map(|r| r.0.timeout_time);
    if let Some(timeout_time) = earliest {
      let time_until_task = timeout_time - self.monotonically_increasing_time();
      if time_until_task > 0.0 {
        let us = (TimeConstants::MICROSECONDS_PER_SECOND as f64 * time_until_task) as i64;
        let _woken_up = self
          .event_loop_control
          .wait_for(&self.mutex, TimeDelta::from_microseconds(us));
      }
    } else {
      self.event_loop_control.wait(&self.mutex);
    }
  }
}

impl TaskRunner for DefaultForegroundTaskRunner {
  /// 对应原 `PostTaskImpl`。
  fn post_task(&self, task: BoxTask, _location: SourceLocation) {
    // 交回任务锁外销毁: 原版形参在局部 guard 之后析构 (.cc:53 注释), Rust 以内层块承接
    let to_delete;
    {
      let _guard = self.lock();
      to_delete = self.post_task_locked(task, Nestability::Nestable);
    }
    drop(to_delete);
  }

  /// 对应原 `PostNonNestableTaskImpl`。
  fn post_non_nestable_task(&self, task: BoxTask, _location: SourceLocation) {
    let to_delete;
    {
      let _guard = self.lock();
      to_delete = self.post_task_locked(task, Nestability::NonNestable);
    }
    drop(to_delete);
  }

  /// 对应原 `PostDelayedTaskImpl`。
  fn post_delayed_task(&self, task: BoxTask, delay_in_seconds: f64, _location: SourceLocation) {
    let _guard = self.lock();
    self.post_delayed_task_locked(task, delay_in_seconds, Nestability::Nestable);
  }

  /// 对应原 `PostNonNestableDelayedTaskImpl`。
  fn post_non_nestable_delayed_task(
    &self,
    task: BoxTask,
    delay_in_seconds: f64,
    _location: SourceLocation,
  ) {
    let _guard = self.lock();
    self.post_delayed_task_locked(task, delay_in_seconds, Nestability::NonNestable);
  }

  /// 对应原 `PostIdleTaskImpl`；原版 CHECK_EQ(支持空闲任务)——CHECK 即中止进程，
  /// Rust 侧降级为 error 日志并丢弃任务（偏差登记 README.md 双段）。
  fn post_idle_task(&self, task: BoxIdleTask, _location: SourceLocation) {
    if self.idle_task_support != IdleTaskSupport::Enabled {
      log::error!("post_idle_task: idle task support disabled（对应原版 CHECK_EQ 降级）");
      return;
    }
    let _guard = self.lock();
    // SAFETY: 已持有 `mutex`。
    let state = unsafe { &mut *self.state.get() };
    if state.terminated {
      return;
    }
    state.idle_task_queue.push_back(task);
  }

  /// 对应原 `IdleTasksEnabled`。
  fn idle_tasks_enabled(&self) -> bool {
    self.idle_task_support == IdleTaskSupport::Enabled
  }

  /// 对应原 `NonNestableTasksEnabled`。
  fn non_nestable_tasks_enabled(&self) -> bool {
    true
  }

  /// 对应原 `NonNestableDelayedTasksEnabled`。
  fn non_nestable_delayed_tasks_enabled(&self) -> bool {
    true
  }
}
