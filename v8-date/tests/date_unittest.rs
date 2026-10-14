// 1:1 迁移自 v8/test/unittests/date/date-unittest.cc
use aok::{OK, Void};
use v8_date::{
  algorithms::*,
  date_cache::{
    cache::DateCache,
    constants::MS_PER_DAY,
    provider::{TimeZoneDetection, TimezoneProvider},
  },
  dateparser::parser::DateParser,
};

#[derive(Debug, Clone)]
struct Rule {
  year: i32,
  start_month: i32,
  start_day: i32,
  end_month: i32,
  end_day: i32,
  offset_sec: i32,
}

#[derive(Debug, Clone)]
struct DateCacheMockProvider {
  local_offset: i32,
  rules: Vec<Rule>,
}

impl DateCacheMockProvider {
  fn new(local_offset: i32, rules: Vec<Rule>) -> Self {
    Self {
      local_offset,
      rules,
    }
  }

  fn find_rule_for(&self, year: i32, month: i32, day: i32, time_in_day_sec: i32) -> Option<&Rule> {
    let mut result = None;
    for rule in &self.rules {
      if self.match_rule(rule, year, month, day, time_in_day_sec) {
        result = Some(rule);
      }
    }
    result
  }

  fn match_rule(&self, rule: &Rule, year: i32, month: i32, day: i32, time_in_day_sec: i32) -> bool {
    if rule.year != 0 && rule.year != year {
      return false;
    }
    if rule.start_month > month || rule.end_month < month {
      return false;
    }
    let start_day = self.compute_rule_day(year, rule.start_month, rule.start_day);
    if rule.start_month == month && start_day > day {
      return false;
    }
    if rule.start_month == month && start_day == day && 2 * 3600 > time_in_day_sec {
      return false;
    }
    let end_day = self.compute_rule_day(year, rule.end_month, rule.end_day);
    if rule.end_month == month && end_day < day {
      return false;
    }
    if rule.end_month == month && end_day == day && 2 * 3600 <= time_in_day_sec {
      return false;
    }
    true
  }

  fn compute_rule_day(&self, year: i32, month: i32, mut day: i32) -> i32 {
    if day != 0 {
      return day;
    }
    let days = days_from_year_month(year, month);
    while weekday(days + day) != 6 {
      day += 1;
    }
    day + 1
  }
}

impl TimezoneProvider for DateCacheMockProvider {
  fn get_daylight_savings_offset_from_os(&mut self, time_sec: i64) -> i32 {
    let days = days_from_time(time_sec * 1000);
    let time_in_day_sec = time_in_day(time_sec * 1000, days) / 1000;
    let (year, month, day) = year_month_day_from_days_uncached(days);
    let rule = self.find_rule_for(year, month, day, time_in_day_sec);
    rule.map_or(0, |r| r.offset_sec * 1000)
  }

  fn get_local_offset_from_os(&mut self, time_ms: i64, _is_utc: bool) -> i32 {
    let dst = self.get_daylight_savings_offset_from_os(time_ms / 1000);
    self.local_offset + dst
  }

  fn local_timezone(&mut self, _time_ms: f64) -> &str {
    "Mock"
  }

  fn clear(&mut self, _detection: TimeZoneDetection) {}
}

fn check_dst(date_cache: &mut DateCache<DateCacheMockProvider>, time: i64) {
  let actual = date_cache.to_local(time);
  let expected = time + date_cache.get_local_offset_from_os(time, true) as i64;
  assert_eq!(actual, expected);
}

fn time_from_year_month_day(year: i32, month: i32, day: i32) -> i64 {
  let result = days_from_year_month(year, month) as i64;
  (result + day as i64 - 1) * MS_PER_DAY
}

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_daylight_savings_time() -> Void {
  let rules = vec![
    Rule {
      year: 0,
      start_month: 2,
      start_day: 0,
      end_month: 10,
      end_day: 0,
      offset_sec: 3600,
    }, // DST from March to November in any year.
    Rule {
      year: 2010,
      start_month: 2,
      start_day: 0,
      end_month: 7,
      end_day: 20,
      offset_sec: 3600,
    }, // DST from March to August 20 in 2010.
    Rule {
      year: 2010,
      start_month: 7,
      start_day: 20,
      end_month: 8,
      end_day: 10,
      offset_sec: 0,
    }, // No DST from August 20 to September 10 in 2010.
    Rule {
      year: 2010,
      start_month: 8,
      start_day: 10,
      end_month: 10,
      end_day: 0,
      offset_sec: 3600,
    }, // DST from September 10 to November in 2010.
  ];

  let local_offset_ms = -36_000_000; // -10 hours.
  let mock_provider = DateCacheMockProvider::new(local_offset_ms, rules);
  let mut date_cache = DateCache::new(mock_provider);

  let start_of_2010 = time_from_year_month_day(2010, 0, 1);
  let start_of_2011 = time_from_year_month_day(2011, 0, 1);
  let august_20 = time_from_year_month_day(2010, 7, 20);
  let september_10 = time_from_year_month_day(2010, 8, 10);

  check_dst(&mut date_cache, (august_20 + september_10) / 2);
  check_dst(&mut date_cache, september_10);
  check_dst(&mut date_cache, september_10 + 2 * 3600 * 1000);
  check_dst(&mut date_cache, september_10 + 2 * 3600 * 1000 - 1000);
  check_dst(&mut date_cache, august_20 + 2 * 3600 * 1000);
  check_dst(&mut date_cache, august_20 + 2 * 3600 * 1000 - 1000);
  check_dst(&mut date_cache, august_20);

  // Check each day of 2010.
  let mut time = start_of_2011 + 2 * 3600 * 1000;
  while time >= start_of_2010 {
    check_dst(&mut date_cache, time);
    check_dst(&mut date_cache, time - 1000);
    check_dst(&mut date_cache, time + 1000);
    time -= MS_PER_DAY;
  }

  // Check one day from 2010 to 2100.
  for year in (2010..=2100).rev() {
    check_dst(&mut date_cache, time_from_year_month_day(year, 5, 5));
  }

  check_dst(&mut date_cache, (august_20 + september_10) / 2);
  check_dst(&mut date_cache, september_10);
  check_dst(&mut date_cache, september_10 + 2 * 3600 * 1000);
  check_dst(&mut date_cache, september_10 + 2 * 3600 * 1000 - 1000);
  check_dst(&mut date_cache, august_20 + 2 * 3600 * 1000);
  check_dst(&mut date_cache, august_20 + 2 * 3600 * 1000 - 1000);
  check_dst(&mut date_cache, august_20);

  OK
}

#[test]
fn test_date_parse_legacy_use_counter() -> Void {
  fn record_call(count: &mut i32, s: &str) {
    let mut out = [0.0f64; 8];
    let (success, legacy) = DateParser::parse_with_legacy_flag(s, &mut out);
    if success && legacy {
      *count += 1;
    }
  }

  let mut legacy_count = 0;
  assert_eq!(0, legacy_count);
  record_call(&mut legacy_count, "2015-02-31");
  assert_eq!(0, legacy_count);
  record_call(&mut legacy_count, "2015-02-31T11:22:33.444Z01:23");
  assert_eq!(0, legacy_count);
  record_call(&mut legacy_count, "2015-02-31T11:22:33.444");
  assert_eq!(0, legacy_count);
  record_call(&mut legacy_count, "2000 01 01");
  assert_eq!(1, legacy_count);
  record_call(&mut legacy_count, "2015-02-31T11:22:33.444     ");
  assert_eq!(1, legacy_count);

  OK
}
