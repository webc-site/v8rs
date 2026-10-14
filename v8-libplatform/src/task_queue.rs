//! 对应 `v8/src/libplatform/task-queue.{h,cc}`：工作线程消费的 FIFO 任务队列。
//!
//! 语义 1:1：`append` 入队并发信号；`get_next` 阻塞取任务，队列终止后返回 None
//! （并接力发信号唤醒其余读者）；`terminate` 置终止位并发信号。
//! 并发纪律沿用原版：`state` 只在持有 `lock`（base::Mutex）时访问，
//! UnsafeCell 仅为满足 Rust 内部可变性，同 v8-base-platform 内部先例。

use std::{cell::UnsafeCell, collections::VecDeque};

use v8_base_platform::{Mutex, OS, Semaphore, TimeDelta};

use crate::api::BoxTask;

/// 锁保护的状态体，对应原 `task_queue_` + `terminated_`。
struct TaskQueueState {
  queue: VecDeque<BoxTask>,
  terminated: bool,
}

/// 对应原 `class v8::platform::TaskQueue`。
pub struct TaskQueue {
  process_queue_semaphore: Semaphore,
  lock: Mutex,
  state: UnsafeCell<TaskQueueState>,
}

// SAFETY: `state` 只在持有 `lock` 时读写；`Semaphore`/`Mutex` 自身跨线程安全。
unsafe impl Send for TaskQueue {}
// SAFETY: 同上。
unsafe impl Sync for TaskQueue {}

impl Default for TaskQueue {
  fn default() -> Self {
    Self::new()
  }
}

impl TaskQueue {
  /// 对应原构造：`process_queue_semaphore_(0), terminated_(false)`。
  pub fn new() -> Self {
    Self {
      process_queue_semaphore: Semaphore::new(0),
      lock: Mutex::new(),
      state: UnsafeCell::new(TaskQueueState {
        queue: VecDeque::new(),
        terminated: false,
      }),
    }
  }

  /// 对应原 `Append`：入队并发信号；队列取回任务所有权。
  pub fn append(&self, task: BoxTask) {
    let _guard = self.lock.lock_guard();
    // SAFETY: 已持有 `lock`。
    let state = unsafe { &mut *self.state.get() };
    // 原版 DCHECK(!terminated_)，release 降级登记。
    debug_assert!(!state.terminated);
    state.queue.push_back(task);
    self.process_queue_semaphore.signal();
  }

  /// 对应原 `GetNext`：无任务时阻塞；终止后返回 None 并接力发信号。
  pub fn get_next(&self) -> Option<BoxTask> {
    loop {
      {
        let _guard = self.lock.lock_guard();
        // SAFETY: 已持有 `lock`。
        let state = unsafe { &mut *self.state.get() };
        if let Some(task) = state.queue.pop_front() {
          return Some(task);
        }
        if state.terminated {
          // 广播式接力唤醒其余读者（同原版 Signal 再传播语义）。
          self.process_queue_semaphore.signal();
          return None;
        }
      }
      self.process_queue_semaphore.wait();
    }
  }

  /// 对应原 `Terminate`。
  pub fn terminate(&self) {
    let _guard = self.lock.lock_guard();
    // SAFETY: 已持有 `lock`。
    let state = unsafe { &mut *self.state.get() };
    debug_assert!(!state.terminated);
    state.terminated = true;
    self.process_queue_semaphore.signal();
  }

  /// 对应原 `BlockUntilQueueEmptyForTesting`：5ms 轮询直至队列清空。
  pub fn block_until_queue_empty_for_testing(&self) {
    loop {
      {
        let _guard = self.lock.lock_guard();
        // SAFETY: 已持有 `lock`。
        if unsafe { &*self.state.get() }.queue.is_empty() {
          return;
        }
      }
      OS::sleep(TimeDelta::from_milliseconds(5));
    }
  }
}

impl Drop for TaskQueue {
  fn drop(&mut self) {
    let _guard = self.lock.lock_guard();
    // SAFETY: 已持有 `lock`。
    let state = unsafe { &*self.state.get() };
    // 原版 ~TaskQueue DCHECK(terminated_ && task_queue_.empty())，release 降级登记。
    debug_assert!(state.terminated, "TaskQueue 未终止即析构");
    debug_assert!(state.queue.is_empty(), "TaskQueue 析构时仍有滞留任务");
  }
}
