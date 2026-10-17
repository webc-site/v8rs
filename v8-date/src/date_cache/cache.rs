// cache.rs — DateCache 完整实现与时区段缓存状态机
use std::mem::swap;

use crate::{
  algorithms::*,
  date_cache::{
    constants::*,
    provider::{SystemTimezoneProvider, TimeZoneDetection, TimezoneProvider},
  },
};

#[derive(Debug, Clone, Copy, Default)]
pub struct CacheItem {
  pub start_ms: i64,
  pub end_ms: i64,
  pub offset_ms: i32,
  pub last_used: i32,
}

#[derive(Debug)]
pub struct DateCache<P: TimezoneProvider = SystemTimezoneProvider> {
  pub provider: P,
  cache: [CacheItem; CACHE_SIZE],
  cache_usage_counter: i32,
  before: usize,
  after: usize,
  local_offset_ms: i32,

  // ymd 缓存
  ymd_valid: bool,
  ymd_days: i32,
  ymd_year: i32,
  ymd_month: i32,
  ymd_day: i32,

  tz_name: Option<String>,
  dst_tz_name: Option<String>,
}

impl Default for DateCache<SystemTimezoneProvider> {
  fn default() -> Self {
    Self::new(SystemTimezoneProvider::default())
  }
}

impl<P: TimezoneProvider> DateCache<P> {
  pub fn new(provider: P) -> Self {
    let mut s = Self {
      provider,
      cache: [CacheItem::default(); CACHE_SIZE],
      cache_usage_counter: 0,
      before: 0,
      after: 1,
      local_offset_ms: INVALID_LOCAL_OFFSET_IN_MS,
      ymd_valid: false,
      ymd_days: 0,
      ymd_year: 0,
      ymd_month: 0,
      ymd_day: 0,
      tz_name: None,
      dst_tz_name: None,
    };
    s.reset_date_cache(TimeZoneDetection::Skip);
    s
  }

  pub fn reset_date_cache(&mut self, detection: TimeZoneDetection) {
    for seg in &mut self.cache {
      Self::clear_segment(seg);
    }
    self.cache_usage_counter = 0;
    self.before = 0;
    self.after = 1;
    self.ymd_valid = false;
    self.local_offset_ms = INVALID_LOCAL_OFFSET_IN_MS;
    self.provider.clear(detection);
    self.tz_name = None;
    self.dst_tz_name = None;
  }

  #[inline]
  pub fn clear_segment(segment: &mut CacheItem) {
    segment.start_ms = 0;
    segment.end_ms = -1;
    segment.offset_ms = 0;
    segment.last_used = 0;
  }

  #[inline]
  pub fn invalid_segment(segment: &CacheItem) -> bool {
    segment.start_ms > segment.end_ms
  }

  pub fn year_month_day_from_days(&mut self, days: i32) -> (i32, i32, i32) {
    if self.ymd_valid {
      let new_day = self.ymd_day + (days - self.ymd_days);
      if (1..=28).contains(&new_day) {
        self.ymd_day = new_day;
        self.ymd_days = days;
        return (self.ymd_year, self.ymd_month, new_day);
      }
    }
    let (year, month, day) = year_month_day_from_days_uncached(days);
    self.ymd_valid = true;
    self.ymd_year = year;
    self.ymd_month = month;
    self.ymd_day = day;
    self.ymd_days = days;
    (year, month, day)
  }

  pub fn break_down_time(&mut self, time_ms: i64) -> TimeBreakdown {
    let days = days_from_time(time_ms);
    let time_in_day_ms = time_in_day(time_ms, days);
    let (year, month, day) = self.year_month_day_from_days(days);
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

  #[inline]
  pub fn get_daylight_savings_offset_from_os(&mut self, time_sec: i64) -> i32 {
    self.provider.get_daylight_savings_offset_from_os(time_sec)
  }

  pub fn daylight_savings_offset_in_ms(&mut self, time_ms: i64) -> i32 {
    let time_sec = if (0..=MAX_EPOCH_TIME_IN_MS).contains(&time_ms) {
      time_ms / 1000
    } else {
      equivalent_time(time_ms) / 1000
    };
    self.get_daylight_savings_offset_from_os(time_sec)
  }

  pub fn get_local_offset_from_os(&mut self, time_ms: i64, is_utc: bool) -> i32 {
    if self.local_offset_ms == INVALID_LOCAL_OFFSET_IN_MS {
      self.local_offset_ms = self.provider.get_local_offset_from_os(time_ms, is_utc);
    }
    let mut offset = self.local_offset_ms;
    let mut time_ms_for_dst = time_ms;
    if !is_utc {
      let ms_per_hour = 3600 * 1000;
      time_ms_for_dst -= (offset + ms_per_hour) as i64;
    }
    offset += self.daylight_savings_offset_in_ms(time_ms_for_dst);
    debug_assert!(offset < INVALID_LOCAL_OFFSET_IN_MS);
    offset
  }

  fn least_recently_used_cache_item(&mut self, skip: usize) -> usize {
    let idx = self
      .cache
      .iter()
      .enumerate()
      .filter(|&(i, _)| i != skip)
      .min_by_key(|&(_, seg)| seg.last_used)
      .map_or(0, |(i, _)| i);

    Self::clear_segment(&mut self.cache[idx]);
    idx
  }

  fn extend_the_after_segment(&mut self, time_ms: i64, offset_ms: i32) {
    if !Self::invalid_segment(&self.cache[self.after])
      && self.cache[self.after].offset_ms == offset_ms
      && self.cache[self.after].start_ms - (DEFAULT_TIME_ZONE_OFFSET_DELTA_IN_MS as i64) <= time_ms
      && time_ms <= self.cache[self.after].end_ms
    {
      self.cache[self.after].start_ms = time_ms;
    } else {
      if !Self::invalid_segment(&self.cache[self.after]) {
        self.after = self.least_recently_used_cache_item(self.before);
      }
      self.cache[self.after].start_ms = time_ms;
      self.cache[self.after].end_ms = time_ms;
      self.cache[self.after].offset_ms = offset_ms;
      self.cache_usage_counter += 1;
      self.cache[self.after].last_used = self.cache_usage_counter;
    }
  }

  fn probe_cache(&mut self, time_ms: i64) {
    let mut before_opt: Option<usize> = None;
    let mut after_opt: Option<usize> = None;

    for (i, seg) in self.cache.iter().enumerate() {
      if Self::invalid_segment(seg) {
        continue;
      }
      if seg.start_ms <= time_ms {
        if before_opt.is_none_or(|b| self.cache[b].start_ms < seg.start_ms) {
          before_opt = Some(i);
        }
      } else if time_ms < seg.end_ms && after_opt.is_none_or(|a| self.cache[a].end_ms > seg.end_ms)
      {
        after_opt = Some(i);
      }
    }

    let before = match before_opt {
      Some(b) => b,
      None => {
        if Self::invalid_segment(&self.cache[self.before]) {
          self.before
        } else {
          let after_idx = after_opt.unwrap_or(self.after);
          self.least_recently_used_cache_item(after_idx)
        }
      }
    };

    let after = match after_opt {
      Some(a) => a,
      None => {
        if Self::invalid_segment(&self.cache[self.after]) && before != self.after {
          self.after
        } else {
          self.least_recently_used_cache_item(before)
        }
      }
    };

    debug_assert_ne!(before, after);
    self.before = before;
    self.after = after;
  }

  pub fn local_offset_in_ms(&mut self, time_ms: i64, is_utc: bool) -> i32 {
    if !is_utc {
      return self.get_local_offset_from_os(time_ms, is_utc);
    }

    if self.cache_usage_counter >= i32::MAX - 10 {
      self.cache_usage_counter = 0;
      for seg in &mut self.cache {
        Self::clear_segment(seg);
      }
    }

    // 乐观快速命中
    if self.cache[self.before].start_ms <= time_ms && time_ms <= self.cache[self.before].end_ms {
      self.cache_usage_counter += 1;
      self.cache[self.before].last_used = self.cache_usage_counter;
      return self.cache[self.before].offset_ms;
    }

    self.probe_cache(time_ms);

    if Self::invalid_segment(&self.cache[self.before]) {
      let offset = self.get_local_offset_from_os(time_ms, is_utc);
      self.cache[self.before].start_ms = time_ms;
      self.cache[self.before].end_ms = time_ms;
      self.cache[self.before].offset_ms = offset;
      self.cache_usage_counter += 1;
      self.cache[self.before].last_used = self.cache_usage_counter;
      return offset;
    }

    if time_ms <= self.cache[self.before].end_ms {
      self.cache_usage_counter += 1;
      self.cache[self.before].last_used = self.cache_usage_counter;
      return self.cache[self.before].offset_ms;
    }

    let default_delta = DEFAULT_TIME_ZONE_OFFSET_DELTA_IN_MS as i64;
    if time_ms - default_delta > self.cache[self.before].end_ms {
      let offset_ms = self.get_local_offset_from_os(time_ms, is_utc);
      self.extend_the_after_segment(time_ms, offset_ms);
      swap(&mut self.before, &mut self.after);
      return offset_ms;
    }

    self.cache_usage_counter += 1;
    self.cache[self.before].last_used = self.cache_usage_counter;

    let new_after_start_ms = self.cache[self.before].end_ms + default_delta;
    if Self::invalid_segment(&self.cache[self.after])
      || new_after_start_ms <= self.cache[self.after].start_ms
    {
      let new_offset_ms = self.get_local_offset_from_os(new_after_start_ms, is_utc);
      self.extend_the_after_segment(new_after_start_ms, new_offset_ms);
    } else {
      self.cache_usage_counter += 1;
      self.cache[self.after].last_used = self.cache_usage_counter;
    }

    if self.cache[self.before].offset_ms == self.cache[self.after].offset_ms {
      let after_end = self.cache[self.after].end_ms;
      self.cache[self.before].end_ms = after_end;
      Self::clear_segment(&mut self.cache[self.after]);
      return self.cache[self.before].offset_ms;
    }

    // 二分搜索夏令时分界点（最多 5 轮）
    for i in (0..5).rev() {
      let delta = self.cache[self.after].start_ms - self.cache[self.before].end_ms;
      let middle_sec = if i == 0 {
        time_ms
      } else {
        self.cache[self.before].end_ms + delta / 2
      };
      let offset_ms = self.get_local_offset_from_os(middle_sec, is_utc);
      if self.cache[self.before].offset_ms == offset_ms {
        self.cache[self.before].end_ms = middle_sec;
        if time_ms <= self.cache[self.before].end_ms {
          return offset_ms;
        }
      } else {
        debug_assert_eq!(self.cache[self.after].offset_ms, offset_ms);
        self.cache[self.after].start_ms = middle_sec;
        if time_ms >= self.cache[self.after].start_ms {
          swap(&mut self.before, &mut self.after);
          return offset_ms;
        }
      }
    }
    self.cache[self.before].offset_ms
  }

  #[inline]
  pub fn to_local(&mut self, time_ms: i64) -> i64 {
    time_ms + self.local_offset_in_ms(time_ms, true) as i64
  }

  #[inline]
  pub fn to_utc(&mut self, time_ms: i64) -> i64 {
    time_ms - self.local_offset_in_ms(time_ms, false) as i64
  }

  #[inline]
  pub fn timezone_offset_ms(&mut self, time_ms: i64) -> i32 {
    (time_ms - self.to_local(time_ms)) as i32
  }

  #[inline]
  pub fn timezone_offset(&mut self, time_ms: i64) -> i32 {
    self.timezone_offset_ms(time_ms) / MS_PER_MIN
  }

  pub fn local_timezone(&mut self, time_ms: i64) -> &str {
    let norm_time_ms = if !(0..=MAX_EPOCH_TIME_IN_MS).contains(&time_ms) {
      equivalent_time(time_ms)
    } else {
      time_ms
    };
    let is_dst = self.daylight_savings_offset_in_ms(norm_time_ms) != 0;
    if is_dst {
      if self.dst_tz_name.is_none() {
        self.dst_tz_name = Some(
          self
            .provider
            .local_timezone(norm_time_ms as f64)
            .to_string(),
        );
      }
      self.dst_tz_name.as_deref().unwrap_or("UTC")
    } else {
      if self.tz_name.is_none() {
        self.tz_name = Some(
          self
            .provider
            .local_timezone(norm_time_ms as f64)
            .to_string(),
        );
      }
      self.tz_name.as_deref().unwrap_or("UTC")
    }
  }

  pub fn to_date_string(&mut self, time_val: f64, mode: ToDateStringMode) -> String {
    if !time_val.is_finite()
      || !(-MAX_TIME_IN_MS as f64..=MAX_TIME_IN_MS as f64).contains(&time_val)
    {
      return "Invalid Date".to_string();
    }
    let time_ms = time_val as i64;
    let local_time_ms =
      if mode == ToDateStringMode::UTCDateAndTime || mode == ToDateStringMode::ISODateAndTime {
        time_ms
      } else {
        self.to_local(time_ms)
      };
    let tb = self.break_down_time(local_time_ms);

    let wk = SHORT_WEEK_DAYS[tb.weekday as usize];
    let mo = SHORT_MONTHS[tb.month as usize];

    match mode {
      ToDateStringMode::LocalDate => {
        if tb.year < 0 {
          format!("{wk} {mo} {:02} {:05}", tb.day, tb.year)
        } else {
          format!("{wk} {mo} {:02} {:04}", tb.day, tb.year)
        }
      }
      ToDateStringMode::LocalTime => {
        let timezone_offset = -self.timezone_offset(time_ms);
        let timezone_hour = timezone_offset.abs() / 60;
        let timezone_min = timezone_offset.abs() % 60;
        let sign = if timezone_offset < 0 { '-' } else { '+' };
        let local_tz = self.local_timezone(time_ms);
        format!(
          "{:02}:{:02}:{:02} GMT{}{:02}{:02} ({local_tz})",
          tb.hour, tb.min, tb.sec, sign, timezone_hour, timezone_min
        )
      }
      ToDateStringMode::LocalDateAndTime => {
        let timezone_offset = -self.timezone_offset(time_ms);
        let timezone_hour = timezone_offset.abs() / 60;
        let timezone_min = timezone_offset.abs() % 60;
        let sign = if timezone_offset < 0 { '-' } else { '+' };
        let local_tz = self.local_timezone(time_ms);
        if tb.year < 0 {
          format!(
            "{wk} {mo} {:02} {:05} {:02}:{:02}:{:02} GMT{}{:02}{:02} ({local_tz})",
            tb.day, tb.year, tb.hour, tb.min, tb.sec, sign, timezone_hour, timezone_min
          )
        } else {
          format!(
            "{wk} {mo} {:02} {:04} {:02}:{:02}:{:02} GMT{}{:02}{:02} ({local_tz})",
            tb.day, tb.year, tb.hour, tb.min, tb.sec, sign, timezone_hour, timezone_min
          )
        }
      }
      ToDateStringMode::UTCDateAndTime => {
        if tb.year < 0 {
          format!(
            "{wk}, {:02} {mo} {:05} {:02}:{:02}:{:02} GMT",
            tb.day, tb.year, tb.hour, tb.min, tb.sec
          )
        } else {
          format!(
            "{wk}, {:02} {mo} {:04} {:02}:{:02}:{:02} GMT",
            tb.day, tb.year, tb.hour, tb.min, tb.sec
          )
        }
      }
      ToDateStringMode::ISODateAndTime => {
        if (0..=9999).contains(&tb.year) {
          format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
            tb.year,
            tb.month + 1,
            tb.day,
            tb.hour,
            tb.min,
            tb.sec,
            tb.ms
          )
        } else if tb.year < 0 {
          format!(
            "-{:06}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
            -tb.year,
            tb.month + 1,
            tb.day,
            tb.hour,
            tb.min,
            tb.sec,
            tb.ms
          )
        } else {
          format!(
            "+{:06}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
            tb.year,
            tb.month + 1,
            tb.day,
            tb.hour,
            tb.min,
            tb.sec,
            tb.ms
          )
        }
      }
    }
  }
}
