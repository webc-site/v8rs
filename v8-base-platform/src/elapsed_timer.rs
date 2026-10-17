//! ElapsedTimer / ScopedTimer：对应 `src/base/platform/elapsed-timer.h`。
//!
//! 偏差：原实现使用匿名 `union { TimeTicks; TimeDelta }` 保存 `start_ticks_` /
//! `paused_elapsed_`（同一 i64 存储）；Rust 用 `enum` 表达相同语义，读写严格二选一。
//! `debug_assert` 保留 started/paused 检查（`debug_assertions` 下生效）。

use std::cell::Cell;

use crate::time::{TimeDelta, TimeTicks};

#[derive(Debug, Clone, Copy)]
enum TimerState {
  Stopped,
  Started(TimeTicks),
  PausedElapsed(TimeDelta),
}

/// 与 V8 `ElapsedTimer` 对应。
#[derive(Debug)]
pub struct ElapsedTimer {
  state: TimerState,
}

impl Default for ElapsedTimer {
  fn default() -> Self {
    Self::new()
  }
}

impl ElapsedTimer {
  /// 初始为 stopped（等价 `start_ticks_()`）。
  pub const fn new() -> Self {
    Self {
      state: TimerState::Stopped,
    }
  }

  pub fn start(&mut self) {
    self.start_at(TimeTicks::now());
  }

  pub fn start_at(&mut self, now: TimeTicks) {
    debug_assert!(!now.is_null());
    debug_assert!(matches!(self.state, TimerState::Stopped), "already started");
    self.state = TimerState::Started(now);
  }

  pub fn stop(&mut self) {
    debug_assert!(matches!(
      self.state,
      TimerState::Started(_) | TimerState::PausedElapsed(_)
    ));
    self.state = TimerState::Stopped;
  }

  pub fn is_started(&self) -> bool {
    !matches!(self.state, TimerState::Stopped)
  }

  #[cfg(debug_assertions)]
  pub fn is_paused(&self) -> bool {
    matches!(self.state, TimerState::PausedElapsed(_))
  }

  pub fn restart(&mut self) -> TimeDelta {
    self.restart_at(TimeTicks::now())
  }

  pub fn restart_at(&mut self, now: TimeTicks) -> TimeDelta {
    debug_assert!(!now.is_null());
    match self.state {
      TimerState::Started(start) => {
        let elapsed = now.minus(start);
        debug_assert!(elapsed.in_microseconds() >= 0);
        self.state = TimerState::Started(now);
        elapsed
      }
      _ => {
        debug_assert!(false, "restart on unstarted timer");
        TimeDelta::new()
      }
    }
  }

  pub fn pause(&mut self) {
    self.pause_at(TimeTicks::now());
  }

  pub fn pause_at(&mut self, now: TimeTicks) {
    let elapsed = self.elapsed_at(now);
    debug_assert!(matches!(self.state, TimerState::Started(_)));
    self.state = TimerState::PausedElapsed(elapsed);
  }

  pub fn resume(&mut self) {
    self.resume_at(TimeTicks::now());
  }

  pub fn resume_at(&mut self, now: TimeTicks) {
    debug_assert!(!now.is_null());
    if let TimerState::PausedElapsed(paused) = self.state {
      self.state = TimerState::Started(now.minus_delta(paused));
    } else {
      debug_assert!(false, "resume on non-paused timer");
    }
  }

  pub fn elapsed(&self) -> TimeDelta {
    self.elapsed_at(TimeTicks::now())
  }

  pub fn elapsed_at(&self, now: TimeTicks) -> TimeDelta {
    debug_assert!(!now.is_null());
    match self.state {
      TimerState::Started(start) => {
        let d = now.minus(start);
        debug_assert!(d.in_microseconds() >= 0);
        d
      }
      // V8 中 paused 状态下访问 start_ticks_ 是 UB；此处返回 paused_elapsed_。
      TimerState::PausedElapsed(elapsed) => elapsed,
      TimerState::Stopped => TimeDelta::new(),
    }
  }

  pub fn has_expired(&self, delta: TimeDelta) -> bool {
    debug_assert!(self.is_started());
    self.elapsed() >= delta
  }
}

/// 与 V8 `ScopedTimer` 对应：作用域结束把 elapsed 累加到目标 `TimeDelta`。
#[derive(Debug)]
pub struct ScopedTimer<'a> {
  timer: ElapsedTimer,
  location: Option<&'a Cell<TimeDelta>>,
}

impl<'a> ScopedTimer<'a> {
  pub fn new(location: &'a Cell<TimeDelta>) -> Self {
    let mut timer = ElapsedTimer::new();
    timer.start();
    Self {
      timer,
      location: Some(location),
    }
  }

  pub fn take(&mut self) -> TimeDelta {
    self.timer.elapsed()
  }
}

impl Drop for ScopedTimer<'_> {
  fn drop(&mut self) {
    if let Some(loc) = self.location.take() {
      let cur = loc.get();
      loc.set(cur + self.timer.elapsed());
    }
  }
}
