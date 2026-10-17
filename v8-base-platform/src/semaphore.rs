//! Semaphore：对应原 `src/base/platform/semaphore.{h,cc}`。
//!
//! 偏差：现代 V8 在 darwin 上使用 `dispatch_semaphore`；在 POSIX 上使用 `sem_*`；
//! 本实现统一以 `std::sync::Mutex<i64> + Condvar` 提供跨平台等价语义：
//! - `Signal` 递增计数并唤醒一个等待者。
//! - `Wait` 阻塞直到计数 > 0，随后递减。
//! - `WaitFor(rel)` 返回 `true` 表示在超时前获取；`false` 表示超时（计数保持不变）。
//! - `WaitFor(0)` 在计数 > 0 时立即返回 true，否则返回 false（与原 `sem_timedwait` 一致）。

use std::{
  sync::{Condvar, Mutex},
  time::{Duration, Instant},
};

use crate::time::TimeDelta;

/// 与 V8 `Semaphore` 对应。
#[derive(Debug, Default)]
pub struct Semaphore {
  count: Mutex<i64>,
  cv: Condvar,
}

impl Semaphore {
  /// 对应 V8 `Semaphore(int count)`。count 视为非负；负数按 0 处理。
  pub fn new(count: i32) -> Self {
    Self {
      count: Mutex::new(i64::from(count.max(0))),
      cv: Condvar::new(),
    }
  }

  /// 对应 V8 `Signal()`。
  pub fn signal(&self) {
    let mut guard = match self.count.lock() {
      Ok(g) => g,
      Err(p) => p.into_inner(),
    };
    *guard += 1;
    self.cv.notify_one();
  }

  /// 对应 V8 `Wait()`：阻塞直到计数 > 0，随后原子地递减。
  pub fn wait(&self) {
    let mut guard = match self.count.lock() {
      Ok(g) => g,
      Err(p) => p.into_inner(),
    };
    while *guard <= 0 {
      guard = match self.cv.wait(guard) {
        Ok(g) => g,
        Err(p) => p.into_inner(),
      };
    }
    *guard -= 1;
  }

  /// 对应 V8 `WaitFor(rel_time)`；返回是否成功获取。
  pub fn wait_for(&self, rel_time: TimeDelta) -> bool {
    let mut guard = match self.count.lock() {
      Ok(g) => g,
      Err(p) => p.into_inner(),
    };
    if *guard > 0 {
      *guard -= 1;
      return true;
    }
    if rel_time.in_microseconds() <= 0 {
      return false;
    }
    let dur = rel_time
      .to_duration()
      .unwrap_or(Duration::from_secs(u64::MAX));
    // 溢出（如 `TimeDelta::max()`）对应原版远未来 timespec → 无限期等待，
    // 不得 panic（`Instant + Duration` 溢出会 panic）。
    let Some(deadline) = Instant::now().checked_add(dur) else {
      loop {
        guard = match self.cv.wait(guard) {
          Ok(g) => g,
          Err(p) => p.into_inner(),
        };
        if *guard > 0 {
          *guard -= 1;
          return true;
        }
      }
    };
    loop {
      let remaining = deadline.saturating_duration_since(Instant::now());
      if remaining.is_zero() {
        return false;
      }
      let outcome = self.cv.wait_timeout_while(guard, remaining, |c| *c <= 0);
      let (g, wtr) = match outcome {
        Ok(pair) => pair,
        Err(poison) => poison.into_inner(),
      };
      guard = g;
      if *guard > 0 {
        *guard -= 1;
        return true;
      }
      if wtr.timed_out() {
        return false;
      }
    }
  }

  /// 当前计数；仅用于测试/调试。
  pub fn count(&self) -> i64 {
    let guard = match self.count.lock() {
      Ok(g) => g,
      Err(p) => p.into_inner(),
    };
    *guard
  }
}
