//! 对应 `src/tasks/operations-barrier.{h,cc}`：多线程操作生命周期屏障。

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use v8_base_platform::{ConditionVariable, Mutex};

/// 对应原 `OperationsBarrier::Token`：RAII 句柄，析构即宣告操作完成。
/// 原版嵌套类 `Token`，crate 根导出名 `BarrierToken`。
pub struct BarrierToken<'a> {
  barrier: Option<&'a OperationsBarrier>,
}

impl<'a> BarrierToken<'a> {
  fn valid(barrier: &'a OperationsBarrier) -> Self {
    Self {
      barrier: Some(barrier),
    }
  }

  fn empty() -> Self {
    Self { barrier: None }
  }

  /// 对应原 `operator bool()`：为真则可安全执行操作且 happens-before CancelAndWait。
  pub fn ok(&self) -> bool {
    self.barrier.is_some()
  }
}

impl Drop for BarrierToken<'_> {
  fn drop(&mut self) {
    if let Some(barrier) = self.barrier.take() {
      barrier.release();
    }
  }
}

/// 对应原 `class OperationsBarrier`。线程安全；析构前必须调用 `cancel_and_wait`
/// （原版 DCHECK，降级为 debug_assert，偏差登记 readme）。
pub struct OperationsBarrier {
  mutex: Mutex,
  release_condition: ConditionVariable,
  cancelled: AtomicBool,
  /// 仅在持有 `mutex` 时访问；用原子量仅为满足 `Sync`。
  operations_count: AtomicUsize,
}

impl Default for OperationsBarrier {
  fn default() -> Self {
    Self::new()
  }
}

impl Drop for OperationsBarrier {
  fn drop(&mut self) {
    debug_assert!(self.cancelled());
  }
}

impl OperationsBarrier {
  pub fn new() -> Self {
    Self {
      mutex: Mutex::new(),
      release_condition: ConditionVariable::new(),
      cancelled: AtomicBool::new(false),
      operations_count: AtomicUsize::new(0),
    }
  }

  /// 对应原 `cancelled()`。
  pub fn cancelled(&self) -> bool {
    self.cancelled.load(Ordering::Relaxed)
  }

  /// 对应原 `TryLock()`：取消后返回空 Token。
  pub fn try_lock(&self) -> BarrierToken<'_> {
    self.mutex.lock();
    if self.cancelled.load(Ordering::Relaxed) {
      // SAFETY: 本函数持有 `mutex`。
      unsafe {
        self.mutex.unlock();
      }
      return BarrierToken::empty();
    }
    self.operations_count.fetch_add(1, Ordering::Relaxed);
    // SAFETY: 本函数持有 `mutex`。
    unsafe {
      self.mutex.unlock();
    }
    BarrierToken::valid(self)
  }

  /// 对应原 `CancelAndWait()`：阻止后续 TryLock 成功并等待所有进行中操作完成。
  /// 注意：只能调用一次。
  pub fn cancel_and_wait(&self) {
    self.mutex.lock();
    debug_assert!(!self.cancelled.load(Ordering::Relaxed));
    self.cancelled.store(true, Ordering::Release);
    while self.operations_count.load(Ordering::Relaxed) > 0 {
      // 容忍 spurious wakeup：唤醒条件由循环复查。
      self.release_condition.wait(&self.mutex);
    }
    // SAFETY: 本函数持有 `mutex`。
    unsafe {
      self.mutex.unlock();
    }
  }

  /// 对应原 `Release()`：Token 析构回调。
  fn release(&self) {
    self.mutex.lock();
    let prev = self.operations_count.fetch_sub(1, Ordering::Relaxed);
    if prev == 1 && self.cancelled.load(Ordering::Relaxed) {
      self.release_condition.notify_one();
    }
    // SAFETY: 本函数持有 `mutex`。
    unsafe {
      self.mutex.unlock();
    }
  }
}
