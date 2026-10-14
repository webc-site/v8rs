// base.rs — v8-date 核心评测操作 trait 定义与各库实现
//
// 本文件职责:
// 1. 定义本 crate 的核心评测操作 trait `DateBench` (评测 id 基于 const NAME 生成)
// 2. v8rs 实现: v8-date 自身的解析 (零堆分配流式) / DateCache 年月查询 / 日内拆分
// 3. 对比库实现 (引用库):
//    - chrono: date 解析与时间戳分解路径 (feature bench-chrono 门控)
//    - std SystemTime: 时间戳处理对照组 (feature bench-std 门控)
// 样本为固定确定性数据集, 保证跨机器可比。

use v8_date::{
  algorithms::{days_from_time, split_time_in_day, time_in_day},
  date_cache::{DateCache, TimeZoneDetection, TimezoneProvider},
  dateparser::parse_date_time_string,
};

/// 解析评测样本集: ISO 格式 / RFC 2822 风格 / legacy / 超界年份 / 边界钳制 / 非法串
pub const PARSE_SAMPLES: &[&str] = &[
  "2023-10-18T14:30:45.123Z",      // 完整 ISO
  "2023-10-18T14:30:45+08:00",     // ISO + 冒号偏移
  "2023-10-18T14:30:45-0500",      // ISO 紧凑偏移
  "2023-10-18",                    // 纯日期 (本地时区路径)
  "2023-10",                       // 年-月
  "+002023-10-18T00:00:00Z",       // 扩展 6 位年份
  "2023-10-18T24:00:00Z",          // 24 点边界 (钳制到次日 0 点)
  "2023-10-18T24:01:00Z",          // 超 24 点非法
  "Wed, 18 Oct 2023 14:30:45 GMT", // RFC 2822 风格
  "10/18/2023 02:30:45 PM",        // 美式 legacy
  "Oct 18 23 12:00:00",            // 两位年 legacy
  "Oct 18 85 12:00:00",            // 两位年 legacy (19 世纪滑移)
  "99999999999999-01-01",          // 超界年份
  "2023-13-45",                    // 非法月份/日
  "invalid date string",           // 完全非法
  "T2023",                         // 非法前缀
];

/// 时间戳查询样本集: 自 2020-01-01 起逐日连续 60 个 (DateCache 命中路径),
/// 末尾补 epoch / 闰日 / 负时间戳边界
pub const TIME_SAMPLES: &[i64] = &{
  let mut out = [0i64; 64];
  let mut i = 0;
  let mut t = 1_577_836_800_000i64; // 2020-01-01T00:00:00Z
  while i < 60 {
    out[i] = t;
    t += 86_400_000;
    i += 1;
  }
  out[60] = 0; // Unix epoch
  out[61] = 951_782_400_000; // 2000-02-29T00:00:00Z 闰日
  out[62] = -2_208_988_800_000; // 1900-01-01T00:00:00Z 负时间戳
  out[63] = 4_102_444_800_000; // 2100-01-01T00:00:00Z 远期
  out
};

/// 本 crate 核心评测操作 trait, 所有参与评测的库都实现它, 评测框架 (main.rs) 基于此泛型驱动
pub trait DateBench: Default {
  /// 实现库名, 评测 id 基于该名字生成
  const NAME: &'static str;
  /// 是否支持字符串解析评测 (std 对照组无日期字符串解析能力)
  const HAS_PARSE: bool = true;

  /// 操作 1: 日期时间字符串 -> Epoch 毫秒, 失败返回 NaN
  fn parse_date_time(&mut self, input: &str) -> f64;

  /// 操作 2: Epoch 毫秒 -> (year, month 从 0 起, day 从 1 起), 对应 DateCache 年份/月份查询命中路径
  fn epoch_ms_to_ymd(&mut self, time_ms: i64) -> (i32, i32, i32);

  /// 操作 3: Epoch 毫秒 -> 日内 (hour, min, sec, ms), 对应 LocalTimePhase 相关纯函数拆分
  fn epoch_ms_to_time_of_day(&mut self, time_ms: i64) -> (i32, i32, i32, i32);
}

/// 固定零偏移时区提供者: 屏蔽宿主机时区, 保证评测确定性
pub struct FixedUtcProvider;

impl TimezoneProvider for FixedUtcProvider {
  fn get_daylight_savings_offset_from_os(&mut self, _time_sec: i64) -> i32 {
    0
  }

  fn get_local_offset_from_os(&mut self, _time_ms: i64, _is_utc: bool) -> i32 {
    0
  }

  fn local_timezone(&mut self, _time_ms: f64) -> &str {
    "UTC"
  }

  fn clear(&mut self, _detection: TimeZoneDetection) {}
}

/// v8rs 实现: v8-date 自身 (零堆分配流式解析 + DateCache 缓存查询)
pub struct V8DateBench {
  date_cache: DateCache<FixedUtcProvider>,
}

impl Default for V8DateBench {
  fn default() -> Self {
    Self {
      date_cache: DateCache::new(FixedUtcProvider),
    }
  }
}

impl DateBench for V8DateBench {
  const NAME: &'static str = "v8-date";

  fn parse_date_time(&mut self, input: &str) -> f64 {
    parse_date_time_string(&mut self.date_cache, input)
  }

  fn epoch_ms_to_ymd(&mut self, time_ms: i64) -> (i32, i32, i32) {
    let days = days_from_time(time_ms);
    self.date_cache.year_month_day_from_days(days)
  }

  fn epoch_ms_to_time_of_day(&mut self, time_ms: i64) -> (i32, i32, i32, i32) {
    let days = days_from_time(time_ms);
    let time_in_day_ms = time_in_day(time_ms, days);
    split_time_in_day(time_in_day_ms)
  }
}

/// chrono 实现: date 解析 / 时间戳分解路径 (引用库: chrono)
#[cfg(feature = "bench-chrono")]
#[derive(Default)]
pub struct ChronoDateBench;

#[cfg(feature = "bench-chrono")]
impl DateBench for ChronoDateBench {
  const NAME: &'static str = "chrono";

  fn parse_date_time(&mut self, input: &str) -> f64 {
    use chrono::NaiveDateTime;
    // 依次尝试 ISO/RFC3339、RFC 2822、常用 naive 格式, 全部失败返回 NaN
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(input) {
      return dt.timestamp_millis() as f64;
    }
    if let Ok(dt) = chrono::DateTime::parse_from_rfc2822(input) {
      return dt.timestamp_millis() as f64;
    }
    for fmt in [
      "%Y-%m-%dT%H:%M:%S%.f",
      "%Y-%m-%dT%H:%M:%S",
      "%Y-%m-%d %H:%M:%S",
      "%Y-%m-%d",
    ] {
      if let Ok(ndt) = NaiveDateTime::parse_from_str(input, fmt) {
        return ndt.and_utc().timestamp_millis() as f64;
      }
    }
    f64::NAN
  }

  fn epoch_ms_to_ymd(&mut self, time_ms: i64) -> (i32, i32, i32) {
    use chrono::{Datelike, LocalResult, TimeZone, Utc};
    match Utc.timestamp_millis_opt(time_ms) {
      LocalResult::Single(dt) => (dt.year(), dt.month0() as i32, dt.day() as i32),
      _ => (0, 0, 0),
    }
  }

  fn epoch_ms_to_time_of_day(&mut self, time_ms: i64) -> (i32, i32, i32, i32) {
    use chrono::{LocalResult, TimeZone, Timelike, Utc};
    match Utc.timestamp_millis_opt(time_ms) {
      LocalResult::Single(dt) => {
        let t = dt.time();
        (
          t.hour() as i32,
          t.minute() as i32,
          t.second() as i32,
          (t.nanosecond() / 1_000_000) as i32,
        )
      }
      _ => (0, 0, 0, 0),
    }
  }
}

/// std SystemTime 对照组: 无字符串解析能力, 只做时间戳载体转换 (引用库: std::time)
#[cfg(feature = "bench-std")]
#[derive(Default)]
pub struct StdSystemTimeBench;

#[cfg(feature = "bench-std")]
impl DateBench for StdSystemTimeBench {
  const NAME: &'static str = "std-systemtime";
  const HAS_PARSE: bool = false;

  fn parse_date_time(&mut self, _input: &str) -> f64 {
    f64::NAN
  }

  fn epoch_ms_to_ymd(&mut self, time_ms: i64) -> (i32, i32, i32) {
    let days = system_time_days(time_ms);
    civil_from_days(days)
  }

  fn epoch_ms_to_time_of_day(&mut self, time_ms: i64) -> (i32, i32, i32, i32) {
    let secs = system_time_secs(time_ms);
    let day_secs = secs.rem_euclid(86_400);
    (
      (day_secs / 3600) as i32,
      (day_secs % 3600 / 60) as i32,
      (day_secs % 60) as i32,
      (time_ms.rem_euclid(1000)) as i32,
    )
  }
}

/// Epoch 毫秒 -> SystemTime -> 自 Unix 纪元的秒
#[cfg(feature = "bench-std")]
fn system_time_secs(time_ms: i64) -> i64 {
  use std::time::{Duration, UNIX_EPOCH};
  let st = if time_ms >= 0 {
    UNIX_EPOCH + Duration::from_millis(time_ms as u64)
  } else {
    UNIX_EPOCH - Duration::from_millis(time_ms.unsigned_abs())
  };
  match st.duration_since(UNIX_EPOCH) {
    Ok(d) => d.as_secs() as i64,
    Err(e) => -(e.duration().as_secs() as i64),
  }
}

/// Epoch 毫秒 -> SystemTime -> 自 Unix 纪元的天数 (向下取整)
#[cfg(feature = "bench-std")]
fn system_time_days(time_ms: i64) -> i64 {
  system_time_secs(time_ms).div_euclid(86_400)
}

/// days -> (year, month 从 0 起, day), Howard Hinnant civil_from_days 算法 (对照组实现)
#[cfg(feature = "bench-std")]
fn civil_from_days(days: i64) -> (i32, i32, i32) {
  let z = days + 719_468;
  let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
  let doe = z - era * 146_097;
  let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
  let y = yoe + era * 400;
  let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
  let mp = (5 * doy + 2) / 153;
  let d = doy - (153 * mp + 2) / 5 + 1;
  let m = if mp < 10 { mp + 3 } else { mp - 9 };
  let year = if m <= 2 { y + 1 } else { y };
  (year as i32, (m - 1) as i32, d as i32)
}
