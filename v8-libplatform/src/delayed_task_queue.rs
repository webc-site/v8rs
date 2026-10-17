//! 对应 `v8/src/libplatform/delayed-task-queue.{h,cc}`：即时 + 延迟任务的混合队列。
//!
//! 语义 1:1：即时任务按投递序执行；延迟任务按到期时刻（秒，来自注入的
//! `TimeFunction`）出队，到期即搬入即时队列。原版 `std::multimap<double, Task>`
//! 以最小堆（deadline 升序 + 投递序号 tie-break）等价承接。
//! 与原版一致：本类型**非线程安全**，应由调用方加锁保护（LP-B DWTR 用 base::Mutex）。

use std::{
  cmp::{Ordering, Reverse},
  collections::{BinaryHeap, VecDeque},
};

use v8_base_platform::{TimeConstants, TimeDelta};

use crate::api::{BoxTask, TimeFunction};

/// `TryGetNext` 的返回状态，对应原 `struct MaybeNextTask`。
pub enum MaybeNextTask {
  /// kTask：拿到可直接执行的任务。
  Task(BoxTask),
  /// kWaitIndefinite：无任务也无延迟任务，无限期等待。
  WaitIndefinite,
  /// kWaitDelayed：最近的延迟任务还需等待的时长。
  WaitDelayed(TimeDelta),
  /// kTerminated：队列已终止。
  Terminated,
}

/// 延迟队列条目（multimap 键值 → 堆元素）。
struct DelayedEntry {
  deadline: f64,
  /// 同 deadline 时按投递序（原版 multimap 等键并列序）。
  seq: u64,
  task: BoxTask,
}

impl PartialEq for DelayedEntry {
  fn eq(&self, other: &Self) -> bool {
    self.deadline.total_cmp(&other.deadline) == Ordering::Equal && self.seq == other.seq
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
      .deadline
      .total_cmp(&other.deadline)
      .then_with(|| self.seq.cmp(&other.seq))
  }
}

/// 对应原 `class v8::platform::DelayedTaskQueue`。
pub struct DelayedTaskQueue {
  task_queue: VecDeque<BoxTask>,
  /// deadline 升序最小堆（`Reverse` 翻转 `BinaryHeap` 的最大堆语义）。
  delayed_task_queue: BinaryHeap<Reverse<DelayedEntry>>,
  terminated: bool,
  time_function: TimeFunction,
  next_seq: u64,
}

impl DelayedTaskQueue {
  /// 对应原构造：注入单调时间函数。
  pub fn new(time_function: TimeFunction) -> Self {
    Self {
      task_queue: VecDeque::new(),
      delayed_task_queue: BinaryHeap::new(),
      terminated: false,
      time_function,
      next_seq: 0,
    }
  }

  /// 对应原 `MonotonicallyIncreasingTime`。
  pub fn monotonically_increasing_time(&self) -> f64 {
    (self.time_function)()
  }

  /// 对应原 `Append`：即时任务，按投递序执行。
  pub fn append(&mut self, task: BoxTask) {
    // 原版 DCHECK(!terminated_)，release 降级登记。
    debug_assert!(!self.terminated);
    self.task_queue.push_back(task);
  }

  /// 对应原 `AppendDelayed`：按 `now + delay_in_seconds` 记入延迟队列。
  pub fn append_delayed(&mut self, task: BoxTask, delay_in_seconds: f64) {
    debug_assert!(delay_in_seconds >= 0.0);
    debug_assert!(!self.terminated);
    let deadline = self.monotonically_increasing_time() + delay_in_seconds;
    let seq = self.next_seq;
    self.next_seq += 1;
    self.delayed_task_queue.push(Reverse(DelayedEntry {
      deadline,
      seq,
      task,
    }));
  }

  /// 对应原 `TryGetNext`：先把到期延迟任务搬入即时队列，再取即时任务；
  /// 否则回报等待策略。原版 for(;;) 各分支均 return，等价单趟。
  pub fn try_get_next(&mut self) -> MaybeNextTask {
    let now = self.monotonically_increasing_time();
    while let Some(task) = self.pop_task_from_delayed_queue(now) {
      self.task_queue.push_back(task);
    }
    if let Some(task) = self.task_queue.pop_front() {
      return MaybeNextTask::Task(task);
    }
    if self.terminated {
      return MaybeNextTask::Terminated;
    }
    // 即时队列此处必空；有延迟任务则回报等待时长，否则无限期。
    if let Some(Reverse(entry)) = self.delayed_task_queue.peek() {
      let wait_in_seconds = entry.deadline - now;
      return MaybeNextTask::WaitDelayed(TimeDelta::from_microseconds(
        (TimeConstants::MICROSECONDS_PER_SECOND as f64 * wait_in_seconds) as i64,
      ));
    }
    MaybeNextTask::WaitIndefinite
  }

  /// 对应原 `Terminate`。
  pub fn terminate(&mut self) {
    debug_assert!(!self.terminated);
    self.terminated = true;
  }

  /// 对应原 `PopTaskFromDelayedQueue`：弹出 deadline 已过的最早延迟任务。
  fn pop_task_from_delayed_queue(&mut self, now: f64) -> Option<BoxTask> {
    if self.delayed_task_queue.peek()?.0.deadline > now {
      return None;
    }
    let Reverse(entry) = self.delayed_task_queue.pop()?;
    Some(entry.task)
  }
}

impl Drop for DelayedTaskQueue {
  fn drop(&mut self) {
    // 对应原析构 DCHECK(terminated_) + DCHECK(task_queue_.empty())。
    debug_assert!(self.terminated);
    debug_assert!(self.task_queue.is_empty());
  }
}
