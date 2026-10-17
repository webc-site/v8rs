//! ConditionVariable：对应原 `src/base/platform/condition-variable.{h,cc}`。
//!
//! 偏差：现代 V8 走 `absl::CondVar`；本实现使用 `parking_lot::RawMutex` 保护的等待队列
//! + `std::thread::park` 自持。语义严格对齐：
//! - `Wait(mutex)` 原子地释放 mutex 并进入等待，唤醒后重新持有。
//! - `WaitFor(mutex, rel_time)` 返回 `true` 表示在超时前收到通知；`false` 表示超时。
//! - 允许 spurious wakeup。
//! - 不缓存通知（对应 V8：`WaitForAfterNofityOnSameThread`）。

use std::{
  cell::UnsafeCell,
  collections::VecDeque,
  fmt::{self, Debug, Formatter},
  sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
  },
  thread::{self, Thread},
  time::{Duration, Instant},
};

use parking_lot::{RawMutex, lock_api::RawMutex as _};
use v8_base::lazy_instance::LazyInstance;

use crate::{mutex::Mutex, time::TimeDelta};

/// 等待队列节点。
struct WaitNode {
  thread: Thread,
  notified: AtomicBool,
}

/// 与 V8 `ConditionVariable` 对应。
pub struct ConditionVariable {
  /// 保护 `queue`。使用 `parking_lot::RawMutex`；`queue` 只在持有该锁时访问。
  inner: RawMutex,
  queue: UnsafeCell<VecDeque<Arc<WaitNode>>>,
}

// SAFETY: `queue` 仅在持有 `inner` 时被读写；因此跨线程可共享。
unsafe impl Sync for ConditionVariable {}
// SAFETY: `RawMutex` 与内部结构均可 `Send`。
unsafe impl Send for ConditionVariable {}

impl Debug for ConditionVariable {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    self.inner.lock();
    // SAFETY: 已持有 `inner`。
    let len = unsafe { (*self.queue.get()).len() };
    // SAFETY: 对应上面的 `lock()`。
    unsafe {
      self.inner.unlock();
    }
    f.debug_struct("ConditionVariable")
      .field("waiters", &len)
      .finish()
  }
}

impl Default for ConditionVariable {
  fn default() -> Self {
    Self::new()
  }
}

/// 原 `LazyConditionVariable`（condition-variable.h:73-88）：
/// `LazyStaticInstance<ConditionVariable, DefaultConstructTrait, ThreadSafeInitOnceTrait>`。
/// 已回收 → `v8-base::lazy_instance::LazyInstance`（与 LazyMutex 同方向）。
pub struct LazyConditionVariable {
  inner: LazyInstance<ConditionVariable>,
}

/// 原 `LAZY_CONDITION_VARIABLE_INITIALIZER` 宏（condition-variable.h:88）：
/// [`LazyConditionVariable::new`] const fn 直用（interior mutability 单元格
/// 不落 const 具名值，同 LazyMutex 说明）。
impl LazyConditionVariable {
  /// const 构造入口；等价原版初始化器。
  #[inline]
  pub const fn new() -> Self {
    Self {
      inner: LazyInstance::new(),
    }
  }

  /// 对应原 `Pointer()`：首次调用构造并返回内部 `ConditionVariable`。
  #[inline]
  pub fn pointer(&self) -> &ConditionVariable {
    self.inner.get()
  }
}

impl Default for LazyConditionVariable {
  fn default() -> Self {
    Self::new()
  }
}

impl ConditionVariable {
  pub fn new() -> Self {
    Self {
      inner: RawMutex::INIT,
      queue: UnsafeCell::new(VecDeque::new()),
    }
  }

  #[inline]
  fn with_queue_locked<R>(&self, f: impl FnOnce(&mut VecDeque<Arc<WaitNode>>) -> R) -> R {
    self.inner.lock();
    // SAFETY: 持有 `inner` 期间独占 `queue`。
    let r = unsafe { f(&mut *self.queue.get()) };
    // SAFETY: 对应上面的 `lock()`；独占持有中。
    unsafe {
      self.inner.unlock();
    }
    r
  }

  #[inline]
  fn push_waiter(&self, node: Arc<WaitNode>) {
    self.with_queue_locked(|q| q.push_back(node));
  }

  /// 从队列中移除 `node`；返回是否成功移除。
  #[inline]
  fn remove_waiter(&self, node: &Arc<WaitNode>) -> bool {
    self.with_queue_locked(|q| {
      let idx = q.iter().position(|n| Arc::ptr_eq(n, node));
      match idx {
        Some(i) => {
          q.remove(i);
          true
        }
        None => false,
      }
    })
  }

  /// 唤醒一个等待者；对应 V8 `NotifyOne()`。
  pub fn notify_one(&self) {
    let popped = self.with_queue_locked(|q| {
      let n = q.pop_front();
      if let Some(n) = &n {
        // 在持锁时置位，避免与 timeout 摘除竞争。
        n.notified.store(true, Ordering::Release);
      }
      n
    });
    if let Some(n) = popped {
      n.thread.unpark();
    }
  }

  /// 唤醒所有等待者；对应 V8 `NotifyAll()`。
  pub fn notify_all(&self) {
    let drained: Vec<Arc<WaitNode>> = self.with_queue_locked(|q| {
      let v: Vec<Arc<WaitNode>> = q.drain(..).collect();
      for n in &v {
        n.notified.store(true, Ordering::Release);
      }
      v
    });
    for n in drained {
      n.thread.unpark();
    }
  }

  /// 对应 V8 `Wait(Mutex*)`。原子地释放 mutex 并阻塞；唤醒后重新持有。
  pub fn wait(&self, mutex: &Mutex) {
    let node = Arc::new(WaitNode {
      thread: thread::current(),
      notified: AtomicBool::new(false),
    });
    self.push_waiter(node.clone());
    // SAFETY: 调用方在持有 mutex 时才允许 wait（对应 V8 契约）。
    unsafe {
      mutex.unlock();
    }
    // 允许 spurious wakeup；以 `notified` 为最终判定。
    while !node.notified.load(Ordering::Acquire) {
      thread::park();
    }
    mutex.lock();
  }

  /// 对应 V8 `WaitFor(Mutex*, rel_time)`。返回 `true` 表示通知先于超时。
  pub fn wait_for(&self, mutex: &Mutex, rel_time: TimeDelta) -> bool {
    let node = Arc::new(WaitNode {
      thread: thread::current(),
      notified: AtomicBool::new(false),
    });
    self.push_waiter(node.clone());
    // SAFETY: 调用方持有 mutex。
    unsafe {
      mutex.unlock();
    }
    let timed_out = if rel_time.in_microseconds() <= 0 {
      // 立即超时；仍给通知一个竞争机会。
      if node.notified.load(Ordering::Acquire) {
        false
      } else {
        let _ = self.remove_waiter(&node);
        !node.notified.load(Ordering::Acquire)
      }
    } else {
      let dur = rel_time
        .to_duration()
        .unwrap_or(Duration::from_secs(u64::MAX));
      // 溢出（如 `TimeDelta::max()`）对应原版远未来 timespec → 无限期等待，
      // 不得 panic（`Instant + Duration` 溢出会 panic）。
      let deadline = Instant::now().checked_add(dur);
      let mut timed_out = false;
      match deadline {
        None => {
          while !node.notified.load(Ordering::Acquire) {
            thread::park();
          }
        }
        Some(deadline) => loop {
          if node.notified.load(Ordering::Acquire) {
            break;
          }
          let now = Instant::now();
          if now >= deadline {
            if self.remove_waiter(&node) {
              // 二次确认。
              if node.notified.load(Ordering::Acquire) {
                break;
              }
              timed_out = true;
              break;
            }
            // 已被 Notify 摘走。
            break;
          }
          thread::park_timeout(deadline.saturating_duration_since(now));
        },
      }
      timed_out
    };
    mutex.lock();
    !timed_out
  }
}
