// composer.rs — 日期/时间/时区合成器
use crate::dateparser::tables::*;

pub const SMI_MAX_VALUE: i32 = (1 << 30) - 1;
pub const SMI_MIN_VALUE: i32 = -(1 << 30);

#[inline]
pub const fn is_valid_smi(value: i32) -> bool {
  value >= SMI_MIN_VALUE && value <= SMI_MAX_VALUE
}

#[inline]
pub const fn between(x: i32, lo: i32, hi: i32) -> bool {
  (x as u32).wrapping_sub(lo as u32) <= (hi as u32).wrapping_sub(lo as u32)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayComposer {
  comp: [i32; 3],
  index: usize,
  named_month: i32,
  is_iso_date: bool,
}

impl Default for DayComposer {
  fn default() -> Self {
    Self::new()
  }
}

impl DayComposer {
  pub const fn new() -> Self {
    Self {
      comp: [0; 3],
      index: 0,
      named_month: K_NONE,
      is_iso_date: false,
    }
  }

  #[inline]
  pub const fn is_empty(&self) -> bool {
    self.index == 0
  }

  pub fn add(&mut self, n: i32) -> bool {
    if self.index < 3 {
      self.comp[self.index] = n;
      self.index += 1;
      true
    } else {
      false
    }
  }

  #[inline]
  pub fn set_named_month(&mut self, n: i32) {
    self.named_month = n;
  }

  #[inline]
  pub fn set_iso_date(&mut self) {
    self.is_iso_date = true;
  }

  #[inline]
  pub const fn is_month(x: i32) -> bool {
    between(x, 1, 12)
  }

  #[inline]
  pub const fn is_day(x: i32) -> bool {
    between(x, 1, 31)
  }

  pub fn write(&mut self, output: &mut [f64; OUTPUT_SIZE]) -> bool {
    if self.index < 1 {
      return false;
    }
    while self.index < 3 {
      self.comp[self.index] = 1;
      self.index += 1;
    }

    let mut year = 0;
    let month;
    let day;

    if self.named_month == K_NONE {
      if self.is_iso_date || (self.index == 3 && !Self::is_day(self.comp[0])) {
        // YMD
        year = self.comp[0];
        month = self.comp[1];
        day = self.comp[2];
      } else {
        // MD(Y)
        month = self.comp[0];
        day = self.comp[1];
        if self.index == 3 {
          year = self.comp[2];
        }
      }
    } else {
      month = self.named_month;
      if self.index == 1 {
        // MD or DM
        day = self.comp[0];
      } else if !Self::is_day(self.comp[0]) {
        // YMD, MYD, or YDM
        year = self.comp[0];
        day = self.comp[1];
      } else {
        // DMY, MDY, or DYM
        day = self.comp[0];
        year = self.comp[1];
      }
    }

    if !self.is_iso_date {
      if between(year, 0, 49) {
        year += 2000;
      } else if between(year, 50, 99) {
        year += 1900;
      }
    }

    if !is_valid_smi(year) || !Self::is_month(month) || !Self::is_day(day) {
      return false;
    }

    output[YEAR] = year as f64;
    output[MONTH] = (month - 1) as f64;
    output[DAY] = day as f64;
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeComposer {
  comp: [i32; 4],
  index: usize,
  hour_offset: i32,
}

impl Default for TimeComposer {
  fn default() -> Self {
    Self::new()
  }
}

impl TimeComposer {
  pub const fn new() -> Self {
    Self {
      comp: [0; 4],
      index: 0,
      hour_offset: K_NONE,
    }
  }

  #[inline]
  pub const fn is_empty(&self) -> bool {
    self.index == 0
  }

  #[inline]
  pub const fn is_expecting(&self, n: i32) -> bool {
    (self.index == 1 && Self::is_minute(n))
      || (self.index == 2 && Self::is_second(n))
      || (self.index == 3 && Self::is_millisecond(n))
  }

  pub fn add(&mut self, n: i32) -> bool {
    if self.index < 4 {
      self.comp[self.index] = n;
      self.index += 1;
      true
    } else {
      false
    }
  }

  pub fn add_final(&mut self, n: i32) -> bool {
    if !self.add(n) {
      return false;
    }
    while self.index < 4 {
      self.comp[self.index] = 0;
      self.index += 1;
    }
    true
  }

  #[inline]
  pub fn set_hour_offset(&mut self, n: i32) {
    self.hour_offset = n;
  }

  #[inline]
  pub const fn is_minute(x: i32) -> bool {
    between(x, 0, 59)
  }

  #[inline]
  pub const fn is_hour(x: i32) -> bool {
    between(x, 0, 23)
  }

  #[inline]
  pub const fn is_second(x: i32) -> bool {
    between(x, 0, 59)
  }

  #[inline]
  pub const fn is_hour_12(x: i32) -> bool {
    between(x, 0, 12)
  }

  #[inline]
  pub const fn is_millisecond(x: i32) -> bool {
    between(x, 0, 999)
  }

  pub fn write(&mut self, output: &mut [f64; OUTPUT_SIZE]) -> bool {
    while self.index < 4 {
      self.comp[self.index] = 0;
      self.index += 1;
    }

    let mut hour = self.comp[0];
    let minute = self.comp[1];
    let second = self.comp[2];
    let millisecond = self.comp[3];

    if self.hour_offset != K_NONE {
      if !Self::is_hour_12(hour) {
        return false;
      }
      hour %= 12;
      hour += self.hour_offset;
    }

    let valid_regular_time = Self::is_hour(hour)
      && Self::is_minute(minute)
      && Self::is_second(second)
      && Self::is_millisecond(millisecond);
    let valid_midnight_24 = hour == 24 && minute == 0 && second == 0 && millisecond == 0;
    if !valid_regular_time && !valid_midnight_24 {
      return false;
    }

    output[HOUR] = hour as f64;
    output[MINUTE] = minute as f64;
    output[SECOND] = second as f64;
    output[MILLISECOND] = millisecond as f64;
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeZoneComposer {
  sign: i32,
  hour: i32,
  minute: i32,
}

impl Default for TimeZoneComposer {
  fn default() -> Self {
    Self::new()
  }
}

impl TimeZoneComposer {
  pub const fn new() -> Self {
    Self {
      sign: K_NONE,
      hour: K_NONE,
      minute: K_NONE,
    }
  }

  pub fn set(&mut self, offset_in_hours: i32) {
    self.sign = if offset_in_hours < 0 { -1 } else { 1 };
    self.hour = offset_in_hours * self.sign;
    self.minute = 0;
  }

  #[inline]
  pub fn set_sign(&mut self, sign: i32) {
    self.sign = if sign < 0 { -1 } else { 1 };
  }

  #[inline]
  pub fn set_absolute_hour(&mut self, hour: i32) {
    self.hour = hour;
  }

  #[inline]
  pub fn set_absolute_minute(&mut self, minute: i32) {
    self.minute = minute;
  }

  #[inline]
  pub const fn is_expecting(&self, n: i32) -> bool {
    self.hour != K_NONE && self.minute == K_NONE && TimeComposer::is_minute(n)
  }

  #[inline]
  pub const fn is_utc(&self) -> bool {
    self.hour == 0 && self.minute == 0
  }

  #[inline]
  pub const fn is_empty(&self) -> bool {
    self.hour == K_NONE
  }

  pub fn write(&self, output: &mut [f64; OUTPUT_SIZE]) -> bool {
    if self.sign != K_NONE {
      let h = if self.hour == K_NONE { 0 } else { self.hour } as u32;
      let m = if self.minute == K_NONE {
        0
      } else {
        self.minute
      } as u32;
      // 原版 (dateparser.cc:106-108): unsigned 32 位算术, 乘加按 2^32 回绕
      // (C++ 有意用 unsigned 规避有符号溢出 UB, 回绕是定义行为, 须 1:1 复现)
      let total_seconds_unsigned = ((h as u64) * 3600 + (m as u64) * 60) & 0xFFFF_FFFF;
      if total_seconds_unsigned > SMI_MAX_VALUE as u64 {
        return false;
      }
      let mut total_seconds = total_seconds_unsigned as i32;
      if self.sign < 0 {
        total_seconds = -total_seconds;
      }
      debug_assert!(is_valid_smi(total_seconds));
      output[UTC_OFFSET] = total_seconds as f64;
    } else {
      output[UTC_OFFSET] = f64::NAN;
    }
    true
  }
}
