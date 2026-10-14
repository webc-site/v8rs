//! 原版 v8/src/base/timezone-cache.h — 时区缓存抽象接口 + 本地时区实现
//!
//! 严格红线 (见 todo.md 十章 10.3): 日期时间语义禁引第三方库 (chrono/jiff 等), 引擎 Date
//! 行为绑定上层 `v8-date`. 本模块只做 base 抽象与 libc 直读, 不承诺 ECMAScript Date 一致性.
//!
//! 原版 `base/timezone-cache.h:11-45` 定义虚接口 `TimezoneCache`, 具体 `LocalTimezoneCache`
//! 由 `libplatform` 提供 (本仓库无该目录), 此处以 libc `localtime_r`/`gmtime_r`/`tzname`
//! 落地一份最小可用实现, 保留 `TimeZoneDetection` 语义.

use core::{
  ffi::{CStr, c_char, c_long},
  mem::zeroed,
};

#[cfg(unix)]
unsafe extern "C" {
  fn tzset();
  static mut tzname: [*const c_char; 2];
  static mut timezone: c_long;
}

#[cfg(windows)]
unsafe extern "C" {
  #[link_name = "_tzset"]
  fn tzset();
  #[link_name = "_localtime64_s"]
  fn localtime64_s(tm: *mut libc::tm, time: *const i64) -> libc::c_int;
  #[link_name = "_get_timezone"]
  fn get_timezone(seconds: *mut c_long) -> libc::c_int;
  #[link_name = "_get_tzname"]
  fn get_tzname(
    ret_val: *mut libc::size_t,
    buf: *mut c_char,
    size: libc::size_t,
    index: libc::c_int,
  ) -> libc::c_int;
}

/// 原版 `enum class TimeZoneDetection { kSkip, kRedetect }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TimeZoneDetection {
  /// 无需重新探测宿主机时区 (沙箱内使用)
  Skip = 0,
  /// 需要重新探测 (通常在沙箱外调用一次)
  Redetect = 1,
}

/// 原版 `class TimezoneCache` — 抽象接口.
///
/// Rust 用 trait 承接; 具体实现需持有可变状态 (缓存/清理), 方法一律 `&mut self`.
pub trait TimezoneCache {
  /// 原版 `LocalTimezone(double time_ms)`: 返回本地时区简称 (e.g. "CST"), NUL 结尾字节切片的
  /// 语义由 `tz_name` 提供 (Rust 无法直接返回 `const char*`, 以 `&CStr` 承接).
  fn local_timezone(&mut self, time_ms: f64) -> Option<&CStr>;

  /// 原版 `DaylightSavingsOffset(double time_ms)`: ECMAScript DST 修正 (毫秒).
  fn daylight_savings_offset(&mut self, time_ms: f64) -> f64;

  /// 原版 `LocalTimeOffset(double time_ms, bool is_utc)`: ECMAScript 本地时区偏移 (毫秒).
  fn local_time_offset(&mut self, time_ms: f64, is_utc: bool) -> f64;

  /// 原版 `Clear(TimeZoneDetection)`.
  fn clear(&mut self, detection: TimeZoneDetection);
}

/// 原版 `LocalTimezoneCache` (libplatform) 的 base 层最小等价:
/// 每次调用现取 libc 时区, 不做额外缓存.
pub struct LocalTimezoneCache {
  /// 缓存最近一次时区名 (NUL 结尾); 供 `local_timezone` 返回稳定指针.
  tz_name_buf: [u8; 64],
  /// `tz_name_buf` 中的有效字节 (含末尾 0).
  tz_name_len: usize,
}

impl Default for LocalTimezoneCache {
  fn default() -> Self {
    Self::new()
  }
}

impl LocalTimezoneCache {
  #[must_use]
  pub fn new() -> Self {
    Self {
      tz_name_buf: [0; 64],
      tz_name_len: 1,
    }
  }

  /// 由 time_ms 得到 time_t (秒), 向下取整, 处理 NaN / 越界.
  fn time_of(time_ms: f64) -> libc::time_t {
    if !time_ms.is_finite() {
      return 0;
    }
    (time_ms / 1000.0).floor() as libc::time_t
  }

  /// `localtime` 到栈上 tm; 失败返回 None.
  fn localtime(&mut self, time_ms: f64) -> Option<libc::tm> {
    let t = Self::time_of(time_ms);
    let mut tm: libc::tm = unsafe { zeroed() };
    #[cfg(unix)]
    {
      // SAFETY: `t` 是本栈只读值, `&raw mut tm` 是本栈可写位置; libc::localtime_r 契约保证不越界写入.
      let p = unsafe { libc::localtime_r(&raw const t, &raw mut tm) };
      if p.is_null() { None } else { Some(tm) }
    }
    #[cfg(windows)]
    {
      const MAX_TIME64: i64 = 0x7934_06ff_f;
      let t_i64 = t as i64;
      if !(0..=MAX_TIME64).contains(&t_i64) {
        return None;
      }
      let rc = unsafe { localtime64_s(&raw mut tm, &raw const t_i64) };
      if rc == 0 { Some(tm) } else { None }
    }
  }

  /// 本地时间与 UTC 的偏移秒数 (东区为正)
  #[inline]
  fn gmtoff_seconds(tm: &libc::tm, time_ms: f64) -> i64 {
    #[cfg(unix)]
    {
      let _ = time_ms;
      tm.tm_gmtoff
    }
    #[cfg(not(unix))]
    {
      let t = Self::time_of(time_ms);
      Self::to_epoch_ymd_hms(tm) - (t as i64)
    }
  }

  /// 从 libc tm 得到 UTC 秒 (`timegm` 不 portable, 手算).
  fn to_epoch_ymd_hms(tm: &libc::tm) -> i64 {
    // 参考 Howard Hinnant days_from_civil; 1900 起, tm_year 是"自 1900".
    let mut y = (tm.tm_year + 1900) as i64;
    let m = (tm.tm_mon + 1) as i64;
    let d = tm.tm_mday as i64;
    let hh = tm.tm_hour as i64;
    let mm = tm.tm_min as i64;
    let ss = tm.tm_sec as i64;
    if m <= 2 {
      y -= 1;
    }
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146097 + doe - 719468;
    days * 86400 + hh * 3600 + mm * 60 + ss
  }

  /// 缓存 `tzname[is_dst]` 到内部缓冲.
  fn cache_tz_name(&mut self, is_dst: bool) {
    // tzset 保证 tzname 已初始化; 多次调用是幂等的.
    // SAFETY: tzset 无参数, 只读环境/时区文件.
    unsafe { tzset() };
    #[cfg(unix)]
    {
      // SAFETY: POSIX 规定 tzname[0..2] 是有效 NUL 结尾 C 字符串指针.
      let ptr: *const c_char = unsafe { tzname[usize::from(is_dst)] };
      if ptr.is_null() {
        self.tz_name_buf[0] = 0;
        self.tz_name_len = 1;
        return;
      }
      let bytes = unsafe { CStr::from_ptr(ptr) }.to_bytes();
      let copy_len = bytes.len().min(self.tz_name_buf.len() - 1);
      self.tz_name_buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
      self.tz_name_buf[copy_len] = 0;
      self.tz_name_len = copy_len + 1;
    }
    #[cfg(windows)]
    {
      let mut ret_len = 0usize;
      let rc = unsafe {
        get_tzname(
          &raw mut ret_len,
          self.tz_name_buf.as_mut_ptr().cast(),
          self.tz_name_buf.len(),
          i32::from(is_dst),
        )
      };
      if rc == 0 && ret_len > 0 {
        self.tz_name_len = ret_len.min(self.tz_name_buf.len());
        self.tz_name_buf[self.tz_name_len - 1] = 0;
      } else {
        self.tz_name_buf[0] = 0;
        self.tz_name_len = 1;
      }
    }
  }
}

impl TimezoneCache for LocalTimezoneCache {
  fn local_timezone(&mut self, time_ms: f64) -> Option<&CStr> {
    let tm = self.localtime(time_ms)?;
    let is_dst = tm.tm_isdst > 0;
    self.cache_tz_name(is_dst);
    CStr::from_bytes_until_nul(&self.tz_name_buf[..self.tz_name_len]).ok()
  }

  /// DST 修正 = 若 tm_isdst > 0 则 (本地偏移 - 标准偏移), 否则 0.
  /// 简化: 用 `timezone` 全局 (秒, 相对 UTC 的标准偏移, 东时区为负), 与 libc 一致.
  fn daylight_savings_offset(&mut self, time_ms: f64) -> f64 {
    let tm = match self.localtime(time_ms) {
      Some(t) => t,
      None => return 0.0,
    };
    if tm.tm_isdst <= 0 {
      return 0.0;
    }
    // 原版用 `timezone` 全局 (秒, 相对 UTC 的标准偏移, 东时区为负).
    #[cfg(windows)]
    let std_offset_seconds = {
      let mut s: c_long = 0;
      let rc = unsafe { get_timezone(&raw mut s) };
      if rc == 0 { i64::from(s) } else { 0 }
    };
    #[cfg(not(windows))]
    let std_offset_seconds = unsafe { timezone };
    // 当前本地偏移 (秒): tm_gmtoff (BSD/glibc 均有) 或 to_epoch_ymd_hms 差值
    let local_offset_seconds = Self::gmtoff_seconds(&tm, time_ms);
    ((local_offset_seconds + std_offset_seconds) as f64) * 1000.0
  }

  /// LocalTimeOffset 语义 (ECMAScript): 本地时间相对 UTC 的偏移 (毫秒).
  /// 若 `is_utc=true`, 直接用 `time_ms` 的时区; 否则视为本地时间戳, 需二次修正.
  fn local_time_offset(&mut self, time_ms: f64, is_utc: bool) -> f64 {
    if is_utc {
      let tm = match self.localtime(time_ms) {
        Some(t) => t,
        None => return 0.0,
      };
      return (Self::gmtoff_seconds(&tm, time_ms) as f64) * 1000.0;
    }
    // 本地时间戳 → 需要迭代一次消除 DST 边界; 简化: 用 time_ms 当作 UTC 计算偏移.
    let tm = match self.localtime(time_ms) {
      Some(t) => t,
      None => return 0.0,
    };
    let as_utc = Self::to_epoch_ymd_hms(&tm);
    (as_utc - Self::time_of(time_ms)) as f64 * 1000.0
      + (Self::gmtoff_seconds(&tm, time_ms) as f64) * 1000.0
  }

  fn clear(&mut self, _detection: TimeZoneDetection) {
    // 原版清空 tm 缓存; 本最小实现无缓存, 仅重置时区名.
    self.tz_name_buf[0] = 0;
    self.tz_name_len = 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_local_timezone_cache() {
    let mut cache = LocalTimezoneCache::new();
    let now_ms = 1_700_000_000_000.0;
    let tz = cache.local_timezone(now_ms);
    assert!(tz.is_some());
    let offset = cache.local_time_offset(now_ms, true);
    assert!(offset.is_finite());
    let dst = cache.daylight_savings_offset(now_ms);
    assert!(dst.is_finite());
    cache.clear(TimeZoneDetection::Redetect);
  }
}
