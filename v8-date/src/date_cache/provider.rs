// provider.rs — 时区支持 Trait 与基于 v8-base 的实现
pub use v8_base::timezone_cache::TimeZoneDetection;
use v8_base::timezone_cache::{LocalTimezoneCache, TimezoneCache};

/// 时区与夏令时信息提供者 Trait
pub trait TimezoneProvider: Send + Sync {
  /// 从系统/底层获取指定秒级时间戳的夏令时偏移（毫秒）
  fn get_daylight_savings_offset_from_os(&mut self, time_sec: i64) -> i32;

  /// 从系统/底层获取指定毫秒时间戳的本地偏移（毫秒）
  fn get_local_offset_from_os(&mut self, time_ms: i64, is_utc: bool) -> i32;

  /// 本地时区名称简写 (如 "UTC", "PST", "CST")
  fn local_timezone(&mut self, time_ms: f64) -> &str;

  /// 清理时区缓存
  fn clear(&mut self, detection: TimeZoneDetection);
}

/// 默认系统时区提供者（包装 v8-base 的 LocalTimezoneCache）
pub struct SystemTimezoneProvider<T: TimezoneCache = LocalTimezoneCache> {
  inner: T,
  cached_tz_name: String,
}

impl<T: TimezoneCache> SystemTimezoneProvider<T> {
  pub fn new(inner: T) -> Self {
    Self {
      inner,
      cached_tz_name: "UTC".to_string(),
    }
  }
}

impl Default for SystemTimezoneProvider<LocalTimezoneCache> {
  fn default() -> Self {
    Self::new(LocalTimezoneCache::new())
  }
}

impl<T: TimezoneCache + Send + Sync> TimezoneProvider for SystemTimezoneProvider<T> {
  fn get_daylight_savings_offset_from_os(&mut self, time_sec: i64) -> i32 {
    self
      .inner
      .daylight_savings_offset((time_sec as f64) * 1000.0) as i32
  }

  fn get_local_offset_from_os(&mut self, time_ms: i64, is_utc: bool) -> i32 {
    self.inner.local_time_offset(time_ms as f64, is_utc) as i32
  }

  fn local_timezone(&mut self, time_ms: f64) -> &str {
    if let Some(s) = self
      .inner
      .local_timezone(time_ms)
      .and_then(|cstr| cstr.to_str().ok())
    {
      if self.cached_tz_name != s {
        self.cached_tz_name.clear();
        self.cached_tz_name.push_str(s);
      }
      return &self.cached_tz_name;
    }
    "UTC"
  }

  fn clear(&mut self, detection: TimeZoneDetection) {
    self.inner.clear(detection);
  }
}
