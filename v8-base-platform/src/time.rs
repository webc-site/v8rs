//! 时间原语：TimeConstants / TimeDelta / Time / TimeTicks / ThreadTicks。
//! 内部一律以微秒表示；语义层禁止泄漏绝对墙钟。

use std::{
  cmp::Ordering,
  hash::{Hash, Hasher},
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
  time::{Duration, SystemTime, UNIX_EPOCH},
};

use coarsetime::Clock;

use crate::monotonic::{
  is_high_resolution, monotonic_micros, thread_ticks_micros, thread_ticks_supported,
};

/// 时间常数；与原 V8 `TimeConstants` 一一对应。
pub struct TimeConstants;

impl TimeConstants {
  pub const HOURS_PER_DAY: i64 = 24;
  pub const MILLISECONDS_PER_SECOND: i64 = 1_000;
  pub const MILLISECONDS_PER_DAY: i64 =
    Self::MILLISECONDS_PER_SECOND * 60 * 60 * Self::HOURS_PER_DAY;
  pub const MICROSECONDS_PER_MILLISECOND: i64 = 1_000;
  pub const MICROSECONDS_PER_SECOND: i64 =
    Self::MICROSECONDS_PER_MILLISECOND * Self::MILLISECONDS_PER_SECOND;
  pub const MICROSECONDS_PER_MINUTE: i64 = Self::MICROSECONDS_PER_SECOND * 60;
  pub const MICROSECONDS_PER_HOUR: i64 = Self::MICROSECONDS_PER_MINUTE * 60;
  pub const MICROSECONDS_PER_DAY: i64 = Self::MICROSECONDS_PER_HOUR * Self::HOURS_PER_DAY;
  pub const MICROSECONDS_PER_WEEK: i64 = Self::MICROSECONDS_PER_DAY * 7;
  pub const NANOSECONDS_PER_MICROSECOND: i64 = 1_000;
  pub const NANOSECONDS_PER_SECOND: i64 =
    Self::NANOSECONDS_PER_MICROSECOND * Self::MICROSECONDS_PER_SECOND;
}

/// 对 int64 做有符号饱和加法；与原 V8 `bits::SignedSaturatedAdd64` 等价。
#[inline]
fn signed_saturated_add64(lhs: i64, rhs: i64) -> i64 {
  lhs.saturating_add(rhs)
}

/// 对 int64 做有符号饱和减法。
#[inline]
fn signed_saturated_sub64(lhs: i64, rhs: i64) -> i64 {
  lhs.saturating_sub(rhs)
}

/// 时长：微秒内部表示；与原 `TimeDelta` 1:1。
#[derive(Clone, Copy, Debug)]
pub struct TimeDelta {
  delta_: i64,
}

impl TimeDelta {
  #[inline]
  pub const fn new() -> Self {
    Self { delta_: 0 }
  }

  #[inline]
  pub const fn from_days(days: i32) -> Self {
    Self {
      delta_: (days as i64) * TimeConstants::MICROSECONDS_PER_DAY,
    }
  }
  #[inline]
  pub const fn from_hours(hours: i32) -> Self {
    Self {
      delta_: (hours as i64) * TimeConstants::MICROSECONDS_PER_HOUR,
    }
  }
  #[inline]
  pub const fn from_minutes(minutes: i32) -> Self {
    Self {
      delta_: (minutes as i64) * TimeConstants::MICROSECONDS_PER_MINUTE,
    }
  }
  #[inline]
  pub const fn from_seconds(seconds: i64) -> Self {
    Self {
      delta_: seconds * TimeConstants::MICROSECONDS_PER_SECOND,
    }
  }
  #[inline]
  pub const fn from_milliseconds(ms: i64) -> Self {
    Self {
      delta_: ms * TimeConstants::MICROSECONDS_PER_MILLISECOND,
    }
  }
  #[inline]
  pub const fn from_microseconds(us: i64) -> Self {
    Self { delta_: us }
  }
  #[inline]
  pub const fn from_nanoseconds(ns: i64) -> Self {
    Self {
      delta_: ns / TimeConstants::NANOSECONDS_PER_MICROSECOND,
    }
  }

  /// 对应原 `FromSecondsD`（time.h:97-99）：秒（f64）经饱和转换。
  #[inline]
  pub fn from_seconds_d(seconds: f64) -> Self {
    Self::from_double(seconds * TimeConstants::MICROSECONDS_PER_SECOND as f64)
  }

  /// 对应原 `FromMillisecondsD`（time.h:100-103）：毫秒（f64）经饱和转换。
  #[inline]
  pub fn from_milliseconds_d(milliseconds: f64) -> Self {
    Self::from_double(milliseconds * TimeConstants::MICROSECONDS_PER_MILLISECOND as f64)
  }

  /// 对应原私有 `FromDouble`（time.h:213-215）：`saturated_cast<int64_t>` 语义
  /// （NaN→0、超界饱和），Rust `as` 转换原生满足。
  #[inline]
  fn from_double(value: f64) -> Self {
    Self {
      delta_: value as i64,
    }
  }

  #[inline]
  pub const fn max() -> Self {
    Self { delta_: i64::MAX }
  }
  #[inline]
  pub const fn min() -> Self {
    Self { delta_: i64::MIN }
  }

  #[inline]
  pub const fn is_zero(&self) -> bool {
    self.delta_ == 0
  }
  #[inline]
  pub const fn is_max(&self) -> bool {
    self.delta_ == i64::MAX
  }
  #[inline]
  pub const fn is_min(&self) -> bool {
    self.delta_ == i64::MIN
  }

  pub fn in_days(&self) -> i32 {
    if self.is_max() {
      return i32::MAX;
    }
    (self.delta_ / TimeConstants::MICROSECONDS_PER_DAY) as i32
  }
  pub fn in_hours(&self) -> i32 {
    if self.is_max() {
      return i32::MAX;
    }
    (self.delta_ / TimeConstants::MICROSECONDS_PER_HOUR) as i32
  }
  pub fn in_minutes(&self) -> i32 {
    if self.is_max() {
      return i32::MAX;
    }
    (self.delta_ / TimeConstants::MICROSECONDS_PER_MINUTE) as i32
  }
  pub fn in_seconds_f(&self) -> f64 {
    if self.is_max() {
      return f64::INFINITY;
    }
    (self.delta_ as f64) / (TimeConstants::MICROSECONDS_PER_SECOND as f64)
  }
  pub fn in_seconds(&self) -> i64 {
    if self.is_max() {
      return i64::MAX;
    }
    self.delta_ / TimeConstants::MICROSECONDS_PER_SECOND
  }
  pub fn in_milliseconds_f(&self) -> f64 {
    if self.is_max() {
      return f64::INFINITY;
    }
    (self.delta_ as f64) / (TimeConstants::MICROSECONDS_PER_MILLISECOND as f64)
  }
  pub fn in_milliseconds(&self) -> i64 {
    if self.is_max() {
      return i64::MAX;
    }
    self.delta_ / TimeConstants::MICROSECONDS_PER_MILLISECOND
  }
  pub fn in_milliseconds_rounded_up(&self) -> i64 {
    if self.is_max() {
      return i64::MAX;
    }
    (self.delta_ + TimeConstants::MICROSECONDS_PER_MILLISECOND - 1)
      / TimeConstants::MICROSECONDS_PER_MILLISECOND
  }
  pub fn in_microseconds(&self) -> i64 {
    if self.is_max() {
      return i64::MAX;
    }
    self.delta_
  }
  pub fn in_nanoseconds(&self) -> i64 {
    if self.is_max() {
      return i64::MAX;
    }
    self.delta_ * TimeConstants::NANOSECONDS_PER_MICROSECOND
  }

  /// 转为 `std::time::Duration`；用于 `ConditionVariable::wait_for` 和 `sleep`。
  /// `is_max` 返回 `None` 视作无限等待，`is_min` 或零返回 `Some(Duration::ZERO)`。
  pub fn to_duration(&self) -> Option<Duration> {
    if self.is_max() {
      return None;
    }
    if self.delta_ <= 0 {
      return Some(Duration::ZERO);
    }
    let secs = (self.delta_ / TimeConstants::MICROSECONDS_PER_SECOND) as u64;
    let subsec_us = self.delta_ % TimeConstants::MICROSECONDS_PER_SECOND;
    Some(Duration::new(secs, (subsec_us as u64 * 1_000) as u32))
  }

  /// 由 `std::time::Duration` 转换。
  pub fn from_duration(d: Duration) -> Self {
    let us = d
      .as_secs()
      .saturating_mul(TimeConstants::MICROSECONDS_PER_SECOND as u64) as i64;
    let sub_us = (d.subsec_micros()) as i64;
    Self {
      delta_: us.saturating_add(sub_us),
    }
  }

  #[inline]
  pub fn times_of(&self, other: &Self) -> f64 {
    (self.delta_ as f64) / (other.delta_ as f64)
  }
  #[inline]
  pub fn percent_of(&self, other: &Self) -> f64 {
    self.times_of(other) * 100.0
  }
}

impl Default for TimeDelta {
  fn default() -> Self {
    Self::new()
  }
}

impl Add for TimeDelta {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    Self {
      delta_: self.delta_ + rhs.delta_,
    }
  }
}
impl Sub for TimeDelta {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    Self {
      delta_: self.delta_ - rhs.delta_,
    }
  }
}
impl AddAssign for TimeDelta {
  fn add_assign(&mut self, rhs: Self) {
    self.delta_ += rhs.delta_;
  }
}
impl SubAssign for TimeDelta {
  fn sub_assign(&mut self, rhs: Self) {
    self.delta_ -= rhs.delta_;
  }
}
impl Neg for TimeDelta {
  type Output = Self;
  fn neg(self) -> Self {
    Self {
      delta_: -self.delta_,
    }
  }
}
impl Mul<i64> for TimeDelta {
  type Output = Self;
  fn mul(self, rhs: i64) -> Self {
    Self {
      delta_: self.delta_ * rhs,
    }
  }
}
impl Div<i64> for TimeDelta {
  type Output = Self;
  fn div(self, rhs: i64) -> Self {
    Self {
      delta_: self.delta_ / rhs,
    }
  }
}
// 原 `operator*=(int64_t)` / `operator/=(int64_t)`（time.h:183-190）：与上面的
// Mul/Div 保持同一溢出语义（直接对 delta_ 运算，release 位回绕同 C++）。
impl MulAssign<i64> for TimeDelta {
  fn mul_assign(&mut self, rhs: i64) {
    self.delta_ *= rhs;
  }
}
impl DivAssign<i64> for TimeDelta {
  fn div_assign(&mut self, rhs: i64) {
    self.delta_ /= rhs;
  }
}

/// darwin `struct mach_timespec`（mach/clock_types.h:61-64）：
/// `tv_sec: unsigned int`；`tv_nsec: clock_res_t`（= `integer_t` = c_int）。逐字段 1:1。
#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MachTimeSpec {
  pub tv_sec: u32,
  pub tv_nsec: i32,
}

/// 原 `TimeDelta::FromMachTimespec / ToMachTimespec`（time.cc:267-287，V8_OS_DARWIN）。
#[cfg(target_os = "macos")]
impl TimeDelta {
  /// 对应 `FromMachTimespec`；原版 DCHECK 校验 nsec ∈ [0, 1e9)。
  pub fn from_mach_timespec(ts: MachTimeSpec) -> Self {
    debug_assert!(ts.tv_nsec >= 0, "DCHECK_GE(ts.tv_nsec, 0)");
    debug_assert!(
      i64::from(ts.tv_nsec) < TimeConstants::NANOSECONDS_PER_SECOND,
      "DCHECK_LT(ts.tv_nsec, kNanosecondsPerSecond)"
    );
    Self {
      delta_: i64::from(ts.tv_sec) * TimeConstants::MICROSECONDS_PER_SECOND
        + i64::from(ts.tv_nsec) / TimeConstants::NANOSECONDS_PER_MICROSECOND,
    }
  }

  /// 对应 `ToMachTimespec`；原版 DCHECK 要求 delta 非负。
  pub fn to_mach_timespec(&self) -> MachTimeSpec {
    debug_assert!(self.delta_ >= 0, "DCHECK_GE(delta_, 0)");
    // 窄化：`tv_sec` 与原 `static_cast<unsigned>` 一致（超限按模截断，同 C++ UB 的实现行为）；
    // `tv_nsec` 余数域 [0, 1e6) µs → [0, 1e9) ns，无损。
    MachTimeSpec {
      tv_sec: (self.delta_ / TimeConstants::MICROSECONDS_PER_SECOND) as u32,
      tv_nsec: ((self.delta_ % TimeConstants::MICROSECONDS_PER_SECOND)
        * TimeConstants::NANOSECONDS_PER_MICROSECOND) as i32,
    }
  }
}

impl PartialEq for TimeDelta {
  fn eq(&self, other: &Self) -> bool {
    self.delta_ == other.delta_
  }
}
impl Eq for TimeDelta {}
impl PartialOrd for TimeDelta {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
impl Ord for TimeDelta {
  fn cmp(&self, other: &Self) -> Ordering {
    self.delta_.cmp(&other.delta_)
  }
}

/// 内部公共 trait：时间值持有者；对应原 `time_internal::TimeBase<T>`。
/// 使用一个宏为 `Time / TimeTicks / ThreadTicks` 提供一致的实现。
macro_rules! impl_time_base {
  ($name:ident) => {
    impl $name {
      #[inline]
      pub const fn is_null(&self) -> bool {
        self.us_ == 0
      }
      #[inline]
      pub const fn max() -> Self {
        Self { us_: i64::MAX }
      }
      #[inline]
      pub const fn min() -> Self {
        Self { us_: i64::MIN }
      }
      #[inline]
      pub const fn is_max(&self) -> bool {
        self.us_ == i64::MAX
      }
      #[inline]
      pub const fn is_min(&self) -> bool {
        self.us_ == i64::MIN
      }
      #[inline]
      pub const fn to_internal_value(&self) -> i64 {
        self.us_
      }
      #[inline]
      pub const fn from_internal_value(us: i64) -> Self {
        Self { us_: us }
      }
      #[inline]
      pub const fn since_origin(&self) -> TimeDelta {
        TimeDelta::from_microseconds(self.us_)
      }
      #[inline]
      pub fn minus(&self, other: Self) -> TimeDelta {
        TimeDelta::from_microseconds(self.us_ - other.us_)
      }
      #[inline]
      pub fn plus_delta(&self, delta: TimeDelta) -> Self {
        Self {
          us_: signed_saturated_add64(delta.delta_, self.us_),
        }
      }
      #[inline]
      pub fn minus_delta(&self, delta: TimeDelta) -> Self {
        Self {
          us_: -signed_saturated_sub64(delta.delta_, self.us_),
        }
      }
    }

    impl Add<TimeDelta> for $name {
      type Output = Self;
      fn add(self, rhs: TimeDelta) -> Self {
        self.plus_delta(rhs)
      }
    }
    impl Sub<TimeDelta> for $name {
      type Output = Self;
      fn sub(self, rhs: TimeDelta) -> Self {
        self.minus_delta(rhs)
      }
    }
    impl Sub for $name {
      type Output = TimeDelta;
      fn sub(self, rhs: Self) -> TimeDelta {
        self.minus(rhs)
      }
    }
    impl AddAssign<TimeDelta> for $name {
      fn add_assign(&mut self, rhs: TimeDelta) {
        *self = *self + rhs;
      }
    }
    impl SubAssign<TimeDelta> for $name {
      fn sub_assign(&mut self, rhs: TimeDelta) {
        *self = *self - rhs;
      }
    }
    impl PartialEq for $name {
      fn eq(&self, other: &Self) -> bool {
        self.us_ == other.us_
      }
    }
    impl Eq for $name {}
    impl PartialOrd for $name {
      fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
      }
    }
    impl Ord for $name {
      fn cmp(&self, other: &Self) -> Ordering {
        self.us_.cmp(&other.us_)
      }
    }
    impl Hash for $name {
      fn hash<H: Hasher>(&self, state: &mut H) {
        self.us_.hash(state);
      }
    }
  };
}

/// 绝对时间点（epoch 起 µs）。
#[derive(Clone, Copy, Debug, Default)]
pub struct Time {
  us_: i64,
}

impl_time_base!(Time);

impl Time {
  #[inline]
  pub const fn new() -> Self {
    Self { us_: 0 }
  }

  /// POSIX `Time::Now`：使用 `gettimeofday`；与原语义一致。
  pub fn now() -> Self {
    Self::now_from_system_time()
  }

  pub fn now_from_system_time() -> Self {
    // 与原 POSIX 分支语义一致；这里用 std::SystemTime 以避免额外 unsafe FFI，精度到微秒。
    match SystemTime::now().duration_since(UNIX_EPOCH) {
      Ok(d) => {
        let us = d
          .as_secs()
          .saturating_mul(TimeConstants::MICROSECONDS_PER_SECOND as u64);
        let us = us.saturating_add(u64::from(d.subsec_micros()));
        Self { us_: us as i64 }
      }
      Err(e) => {
        let d = e.duration();
        let us = d
          .as_secs()
          .saturating_mul(TimeConstants::MICROSECONDS_PER_SECOND as u64);
        let us = us.saturating_add(u64::from(d.subsec_micros()));
        Self { us_: -(us as i64) }
      }
    }
  }

  /// 对应 V8 Windows `Clock::Now()` 的粗粒度缓存语义；POSIX 无原生 Clock 类，
  /// 这里以 `coarsetime::Clock::recent_since_epoch()` 提供近似值。仅当调用方明确接受
  /// 毫秒级抖动时使用；语义层禁止依赖此值。
  pub fn approx_now() -> Self {
    let secs = Clock::recent_since_epoch().as_f64();
    Self {
      us_: (secs * TimeConstants::MICROSECONDS_PER_SECOND as f64) as i64,
    }
  }

  #[inline]
  pub const fn unix_epoch() -> Self {
    Self { us_: 0 }
  }

  /// 与 V8 保持一致（time.cc:425-437）：`(sec=0, nsec=0) -> null`；
  /// `(time_t::MAX=i64::MAX, NS-1) -> max`。无 32 位哨兵。
  pub fn from_timespec(sec: i64, nsec: i64) -> Self {
    debug_assert!((0..TimeConstants::NANOSECONDS_PER_SECOND).contains(&nsec));
    if sec == 0 && nsec == 0 {
      return Self::new();
    }
    if nsec == TimeConstants::NANOSECONDS_PER_SECOND - 1 && sec == i64::MAX {
      return Self::max();
    }
    Self {
      us_: sec * TimeConstants::MICROSECONDS_PER_SECOND
        + nsec / TimeConstants::NANOSECONDS_PER_MICROSECOND,
    }
  }

  pub fn to_timespec(&self) -> (i64, i64) {
    if self.is_null() {
      return (0, 0);
    }
    if self.is_max() {
      return (i64::MAX, TimeConstants::NANOSECONDS_PER_SECOND - 1);
    }
    (
      self.us_ / TimeConstants::MICROSECONDS_PER_SECOND,
      (self.us_ % TimeConstants::MICROSECONDS_PER_SECOND)
        * TimeConstants::NANOSECONDS_PER_MICROSECOND,
    )
  }

  /// 与原 `FromTimeval` 一致：`(sec=0, usec=0) -> null`；`(time_t::MAX, USEC-1) -> max`。
  pub fn from_timeval(sec: i64, usec: i64) -> Self {
    if sec == 0 && usec == 0 {
      return Self::new();
    }
    if usec == TimeConstants::MICROSECONDS_PER_SECOND - 1 && sec == i64::MAX {
      return Self::max();
    }
    Self {
      us_: sec * TimeConstants::MICROSECONDS_PER_SECOND + usec,
    }
  }

  pub fn to_timeval(&self) -> (i64, i64) {
    if self.is_null() {
      return (0, 0);
    }
    if self.is_max() {
      return (i64::MAX, TimeConstants::MICROSECONDS_PER_SECOND - 1);
    }
    (
      self.us_ / TimeConstants::MICROSECONDS_PER_SECOND,
      self.us_ % TimeConstants::MICROSECONDS_PER_SECOND,
    )
  }

  pub fn from_js_time(ms_since_epoch: f64) -> Self {
    if ms_since_epoch == f64::MAX {
      return Self::max();
    }
    Self {
      us_: (ms_since_epoch * TimeConstants::MICROSECONDS_PER_MILLISECOND as f64) as i64,
    }
  }

  pub fn to_js_time(&self) -> f64 {
    if self.is_null() {
      return 0.0;
    }
    if self.is_max() {
      return f64::MAX;
    }
    (self.us_ as f64) / TimeConstants::MICROSECONDS_PER_MILLISECOND as f64
  }
}

/// 单调时钟；`Now()` 永不返回 null。
#[derive(Clone, Copy, Debug, Default)]
pub struct TimeTicks {
  us_: i64,
}

impl_time_base!(TimeTicks);

impl TimeTicks {
  #[inline]
  pub const fn new() -> Self {
    Self { us_: 0 }
  }

  /// 单调递增；永不返回 null；与原 V8 POSIX 分支 `TimeTicks(ticks+1)` 一致。
  pub fn now() -> Self {
    let d = monotonic_micros();
    Self { us_: d + 1 }
  }

  pub fn is_high_resolution() -> bool {
    is_high_resolution()
  }

  #[inline]
  pub const fn from_ms_ticks_for_testing(ticks: i64) -> Self {
    Self {
      us_: ticks * TimeConstants::MICROSECONDS_PER_MILLISECOND,
    }
  }
}

/// 每线程 CPU 时间；仅在支持平台上提供。
#[derive(Clone, Copy, Debug, Default)]
pub struct ThreadTicks {
  us_: i64,
}

impl_time_base!(ThreadTicks);

impl ThreadTicks {
  #[inline]
  pub const fn new() -> Self {
    Self { us_: 0 }
  }

  pub fn is_supported() -> bool {
    thread_ticks_supported()
  }

  /// 对应原 `ThreadTicks::WaitUntilInitialized`（time.h:470-474）：仅 Windows 需等待
  /// 异步初始化，darwin/linux 的读取路径同步完成，故为非 Windows 的空实现。
  #[inline]
  pub fn wait_until_initialized() {}

  pub fn now() -> Self {
    Self {
      us_: thread_ticks_micros(),
    }
  }
}
