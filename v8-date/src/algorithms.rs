// algorithms.rs — ECMAScript 日期算法规范实现 (ECMA-262)
use crate::date_cache::constants::*;

/// 计算天数: floor(time_ms / kMsPerDay)
#[inline]
pub const fn days_from_time(time_ms: i64) -> i32 {
  time_ms.div_euclid(MS_PER_DAY) as i32
}

/// 计算一天内的毫秒数: modulo(time_ms, kMsPerDay)
#[inline]
pub const fn time_in_day(time_ms: i64, days: i32) -> i32 {
  (time_ms - (days as i64) * MS_PER_DAY) as i32
}

/// 分解天内毫秒数为 (hour, min, sec, ms)
#[inline]
pub const fn split_time_in_day(time_in_day_ms: i32) -> (i32, i32, i32, i32) {
  let hour = time_in_day_ms / MS_PER_HOUR;
  let rem_h = time_in_day_ms % MS_PER_HOUR;
  let min = rem_h / MS_PER_MIN;
  let rem_m = rem_h % MS_PER_MIN;
  let sec = rem_m / 1000;
  let ms = rem_m % 1000;
  (hour, min, sec, ms)
}

/// ECMA-262 TimeClip 操作
#[inline]
pub fn try_time_clip(time: &mut f64) -> bool {
  let max = MAX_TIME_IN_MS as f64;
  if (-max..=max).contains(time) {
    *time = time.trunc() + 0.0;
    true
  } else {
    false
  }
}

/// 根据自 Unix 纪元以来的天数计算星期 (0 = 周日, 1 = 周一, ..., 6 = 周六)
/// ECMA 262 - 15.9.1.6
#[inline]
pub const fn weekday(days: i32) -> i32 {
  (days + 4).rem_euclid(7)
}

/// 判断是否闰年
#[inline]
pub const fn is_leap(year: i32) -> bool {
  year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// 计算自 Unix 纪元到指定年份和月份第 1 天的天数
#[inline]
pub const fn days_from_year_month(year: i32, month: i32) -> i32 {
  let y = year + month.div_euclid(12);
  let m = month.rem_euclid(12) as usize;

  let year1 = y + YEAR_DELTA;
  let day_from_year = 365 * year1 + year1 / 4 - year1 / 100 + year1 / 400 - BASE_DAY;

  let day_from_month = if !is_leap(y) {
    DAY_FROM_MONTH[m]
  } else {
    DAY_FROM_MONTH_LEAP[m]
  };
  day_from_year + day_from_month
}

/// 纯算法计算天数对应的 (year, month, day)
/// month 从 0 开始 (0 = Jan, 11 = Dec), day 从 1 开始 (1..=31)
pub const fn year_month_day_from_days_uncached(mut days: i32) -> (i32, i32, i32) {
  let save_days = days;
  days += DAYS_OFFSET;
  let mut year = 400 * (days / DAYS_IN400_YEARS) - YEARS_OFFSET;
  days %= DAYS_IN400_YEARS;

  days -= 1;
  let yd1 = days / DAYS_IN100_YEARS;
  days %= DAYS_IN100_YEARS;
  year += 100 * yd1;

  days += 1;
  let yd2 = days / DAYS_IN4_YEARS;
  days %= DAYS_IN4_YEARS;
  year += 4 * yd2;

  days -= 1;
  let yd3 = days / 365;
  days %= 365;
  year += yd3;

  let leap = (yd1 == 0 || yd2 != 0) && yd3 == 0;
  if leap {
    days += 1;
  }

  let leap_offset = if leap { 1 } else { 0 };
  let (month, day) = if days >= 31 + 28 + leap_offset {
    days -= 31 + 28 + leap_offset;
    let mut m = 2;
    while m < 12 {
      let m_days = DAYS_IN_MONTHS[m] as i32;
      if days < m_days {
        break;
      }
      days -= m_days;
      m += 1;
    }
    let m = if m > 11 { 11 } else { m };
    (m as i32, days + 1)
  } else if days < 31 {
    (0, days + 1)
  } else {
    (1, days - 31 + 1)
  };

  debug_assert!(days_from_year_month(year, month) + day - 1 == save_days);
  (year, month, day)
}

/// 返回 [2008-2035] 范围内等价的年份（闰年性相同且元旦星期相同）
/// ECMA 262 - 15.9.1.9
#[inline]
pub const fn equivalent_year(year: i32) -> i32 {
  let week_day = weekday(days_from_year_month(year, 0));
  let recent_year = (if is_leap(year) { 1956 } else { 1967 }) + (week_day * 12) % 28;
  2008 + (recent_year + 3 * 28 - 2008) % 28
}

/// 计算等价时间戳
#[inline]
pub const fn equivalent_time(time_ms: i64) -> i64 {
  let days = days_from_time(time_ms);
  let time_within_day_ms = time_in_day(time_ms, days);
  let (year, month, day) = year_month_day_from_days_uncached(days);
  let new_days = days_from_year_month(equivalent_year(year), month) + day - 1;
  (new_days as i64) * MS_PER_DAY + (time_within_day_ms as i64)
}

/// ES6 section 20.3.1.14 MakeDate (day, time)
#[inline]
pub fn make_date(day: f64, time: f64) -> f64 {
  if day.is_finite() && time.is_finite() {
    time + day * MS_PER_DAY_F64
  } else {
    f64::NAN
  }
}

/// ES6 section 20.3.1.13 MakeDay (year, month, date)
pub fn make_day(year: f64, month: f64, date: f64) -> f64 {
  if (MIN_YEAR..=MAX_YEAR).contains(&year)
    && (MIN_MONTH..=MAX_MONTH).contains(&month)
    && date.is_finite()
  {
    let day_from_year = days_from_year_month(year as i32, month as i32);
    let d = date.trunc() + 0.0;
    (day_from_year - 1) as f64 + d
  } else {
    f64::NAN
  }
}

/// ES6 section 20.3.1.12 MakeTime (hour, min, sec, ms)
pub fn make_time(hour: f64, min: f64, sec: f64, ms: f64) -> f64 {
  if hour.is_finite() && min.is_finite() && sec.is_finite() && ms.is_finite() {
    let h = hour.trunc() + 0.0;
    let m = min.trunc() + 0.0;
    let s = sec.trunc() + 0.0;
    let milli = ms.trunc() + 0.0;
    h * MS_PER_HOUR_F64 + m * MS_PER_MINUTE + s * MS_PER_SECOND + milli
  } else {
    f64::NAN
  }
}

/// 日期分解结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TimeBreakdown {
  pub year: i32,
  pub month: i32,
  pub day: i32,
  pub weekday: i32,
  pub hour: i32,
  pub min: i32,
  pub sec: i32,
  pub ms: i32,
}

/// 分解时间戳
pub const fn break_down_time_uncached(time_ms: i64) -> TimeBreakdown {
  let days = days_from_time(time_ms);
  let time_in_day_ms = time_in_day(time_ms, days);
  let (year, month, day) = year_month_day_from_days_uncached(days);
  let (hour, min, sec, ms) = split_time_in_day(time_in_day_ms);
  TimeBreakdown {
    year,
    month,
    day,
    weekday: weekday(days),
    hour,
    min,
    sec,
    ms,
  }
}
