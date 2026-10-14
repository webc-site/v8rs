//! v8-base `TimeTicks`/`TimeDelta`/`ElapsedTimer` 的最小私有实现。
//!
//! 【待回收】v8-base 转写完成后,本模块整体删除,改用 `v8_base::time`。
//! 语义对照 `v8/src/base/platform/time.h`:
//! - `TimeDelta` 内部以微秒(i64)存储;
//! - `TimeTicks::Now()` 返回自某个固定但任意原点起的单调时钟微秒数;
//! - `TimeDelta::Max()` 为 `i64::MAX` 微秒。

use std::ops::{Add, AddAssign, Sub};

use coarsetime::Instant;

/// 时间差,微秒精度(1:1 对应 `base::TimeDelta`)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TimeDelta {
  us: i64,
}

impl TimeDelta {
  #[inline]
  pub fn from_microseconds(us: i64) -> Self {
    Self { us }
  }

  #[inline]
  pub fn from_milliseconds(ms: i64) -> Self {
    Self { us: ms * 1000 }
  }

  #[inline]
  pub fn in_microseconds(self) -> i64 {
    self.us
  }

  /// 整除截断,同 C++ `InMilliseconds()`;`Max()` 保持 `i64::MAX`
  /// 防溢出(time.cc:234-240 `IsMax() → max`)—— `RecordAbandon` 依赖
  /// `static_cast<int>(Max().InMilliseconds()) == -1` 的截断结果。
  #[inline]
  pub fn in_milliseconds(self) -> i64 {
    if self.us == i64::MAX {
      return i64::MAX;
    }
    self.us / 1000
  }

  /// 同 `base::TimeDelta::Max()`。
  #[inline]
  pub fn max() -> Self {
    Self { us: i64::MAX }
  }

  #[inline]
  pub fn is_zero(self) -> bool {
    self.us == 0
  }
}

impl Add for TimeDelta {
  type Output = TimeDelta;
  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    // 饱和加,避免 panic(C++ 中此处为 UB;此处保守处理)。
    TimeDelta {
      us: self.us.saturating_add(rhs.us),
    }
  }
}

impl AddAssign for TimeDelta {
  #[inline]
  fn add_assign(&mut self, rhs: Self) {
    self.us = self.us.saturating_add(rhs.us);
  }
}

impl Sub for TimeDelta {
  type Output = TimeDelta;
  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    TimeDelta {
      us: self.us.saturating_sub(rhs.us),
    }
  }
}

/// 单调时刻(1:1 对应 `base::TimeTicks`)。默认值(内部值 0)表示"空"。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TimeTicks {
  internal: i64,
}

impl TimeTicks {
  /// `base::TimeTicks::Now()`:单调时钟微秒(coarsetime 低开销时钟)。
  /// `as_ticks()`(coarsetime)在本平台(darwin)为自单调时钟原点起的纳秒计数。
  #[inline]
  pub fn now() -> Self {
    let ns = Instant::now().as_ticks();
    Self {
      internal: ns as i64 / 1000,
    }
  }

  #[inline]
  pub fn from_internal_value(v: i64) -> Self {
    Self { internal: v }
  }

  #[inline]
  pub fn to_internal_value(self) -> i64 {
    self.internal
  }

  #[inline]
  pub fn is_null(self) -> bool {
    self.internal == 0
  }
}

/// 对应 `base::TimeTicks operator+(TimeTicks, TimeDelta)`。
impl Add<TimeDelta> for TimeTicks {
  type Output = TimeTicks;
  #[inline]
  fn add(self, delta: TimeDelta) -> Self::Output {
    TimeTicks {
      internal: self.internal.saturating_add(delta.in_microseconds()),
    }
  }
}

impl Sub for TimeTicks {
  type Output = TimeDelta;
  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    TimeDelta::from_microseconds(self.internal.saturating_sub(rhs.internal))
  }
}

/// 计时器(1:1 对应 `base::ElapsedTimer`)。
#[derive(Debug, Clone, Copy, Default)]
pub struct ElapsedTimer {
  start_ticks: TimeTicks,
  elapsed: TimeDelta,
}

impl ElapsedTimer {
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }

  #[inline]
  pub fn start(&mut self) {
    self.start_at(TimeTicks::now());
  }

  #[inline]
  pub fn start_at(&mut self, now: TimeTicks) {
    self.start_ticks = now;
    self.elapsed = TimeDelta::default();
  }

  #[inline]
  pub fn is_started(&self) -> bool {
    !self.start_ticks.is_null()
  }

  /// 同 C++:未启动时 `Elapsed()` 返回 0。
  #[inline]
  pub fn elapsed(&self) -> TimeDelta {
    self.elapsed_at(TimeTicks::now())
  }

  #[inline]
  pub fn elapsed_at(&self, now: TimeTicks) -> TimeDelta {
    if self.is_started() {
      self.elapsed + (now - self.start_ticks)
    } else {
      self.elapsed
    }
  }

  #[inline]
  pub fn stop(&mut self) {
    // C++ Stop() 仅 DCHECK 已启动;此处清零以便可复用。
    self.elapsed = TimeDelta::default();
    self.start_ticks = TimeTicks::default();
  }

  #[inline]
  pub fn pause(&mut self, now: TimeTicks) {
    debug_assert!(self.is_started());
    self.elapsed += now - self.start_ticks;
    self.start_ticks = TimeTicks::default();
  }

  #[inline]
  pub fn resume(&mut self, now: TimeTicks) {
    debug_assert!(!self.is_started());
    self.start_ticks = now;
  }
}

/// 单调递增毫秒数(近似 `base::Time::Now().ToMonotonicTime().ToJsTimeMS()`
/// 的"自任意原点毫秒"语义;V8FileLogger::Time 的可预测分支使用)。
#[inline]
pub fn monotonically_increasing_time_in_ms() -> f64 {
  let ns = Instant::now().as_ticks();
  ns as f64 / 1_000_000.0
}

/// 线程 CPU 时间是否可用。【待回收】v8-base `ThreadTicks::IsSupported`。
#[inline]
pub fn thread_ticks_supported() -> bool {
  false
}
