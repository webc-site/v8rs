// review_round1_unittest.rs — 第 1 轮对抗式审查边缘测试
use aok::{OK, Void};
use v8_date::{
  algorithms::*,
  date_cache::{
    cache::DateCache,
    constants::*,
    provider::{TimeZoneDetection, TimezoneProvider},
  },
  dateparser::{
    parser::{DateParser, parse_date_time_string, read_milliseconds},
    tables::*,
    tokenizer::DateToken,
  },
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[derive(Debug, Default)]
struct MockOscillatingTzProvider {
  counter: i32,
}

impl TimezoneProvider for MockOscillatingTzProvider {
  fn get_daylight_savings_offset_from_os(&mut self, time_sec: i64) -> i32 {
    // 模拟每 20 天震荡一次的 DST
    if (time_sec / (20 * 86400)) % 2 == 0 {
      3600 * 1000
    } else {
      0
    }
  }

  fn get_local_offset_from_os(&mut self, time_ms: i64, _is_utc: bool) -> i32 {
    self.counter += 1;
    let dst = self.get_daylight_savings_offset_from_os(time_ms / 1000);
    8 * 3600 * 1000 + dst
  }

  fn local_timezone(&mut self, _time_ms: f64) -> &str {
    "CST"
  }

  fn clear(&mut self, _detection: TimeZoneDetection) {}
}

/// 1. ECMAScript 规范极端边界测试
#[test]
fn test_ecmascript_time_clip_and_year_bounds() -> Void {
  // TimeClip 精确边界: MAX_TIME_IN_MS = 8640000000000000.0 (100,000,000 天)
  let max_time = MAX_TIME_IN_MS as f64;
  let mut t = max_time;
  assert!(try_time_clip(&mut t));
  assert_eq!(t, max_time);

  let mut t_neg = -max_time;
  assert!(try_time_clip(&mut t_neg));
  assert_eq!(t_neg, -max_time);

  let mut t_over = max_time + 1.0;
  assert!(!try_time_clip(&mut t_over));

  let mut t_under = -max_time - 1.0;
  assert!(!try_time_clip(&mut t_under));

  // -0.0 规范化为 +0.0
  let mut neg_zero = -0.0f64;
  assert!(try_time_clip(&mut neg_zero));
  assert_eq!(neg_zero.to_bits(), 0.0f64.to_bits());

  // 特殊浮点值
  let mut nan_val = f64::NAN;
  assert!(!try_time_clip(&mut nan_val));
  let mut inf_val = f64::INFINITY;
  assert!(!try_time_clip(&mut inf_val));
  let mut neg_inf_val = f64::NEG_INFINITY;
  assert!(!try_time_clip(&mut neg_inf_val));

  // 100,000,000 天 (约 275760 年) 与 -100,000,000 天 (约 -271821 年) 互逆
  let max_days = 100_000_000;
  let (y_max, m_max, d_max) = year_month_day_from_days_uncached(max_days);
  assert_eq!(days_from_year_month(y_max, m_max) + d_max - 1, max_days);

  let min_days = -100_000_000;
  let (y_min, m_min, d_min) = year_month_day_from_days_uncached(min_days);
  assert_eq!(days_from_year_month(y_min, m_min) + d_min - 1, min_days);

  // 闰年 400 年 / 100 年 / 4 年周期边界
  assert!(is_leap(1600));
  assert!(!is_leap(1700));
  assert!(!is_leap(1800));
  assert!(!is_leap(1900));
  assert!(is_leap(2000));
  assert!(is_leap(2400));
  assert!(is_leap(0));
  assert!(!is_leap(-100));
  assert!(is_leap(-400));
  assert!(is_leap(-4));
  assert!(!is_leap(-1));

  // 负数月份跨多年计算: make_day 负数 month 倒推
  let d1 = make_day(2000.0, -1.0, 1.0); // 1999 年 12 月 1 日
  let d2 = make_day(1999.0, 11.0, 1.0);
  assert_eq!(d1, d2);

  let d3 = make_day(2000.0, -12.0, 1.0); // 1999 年 1 月 1 日
  let d4 = make_day(1999.0, 0.0, 1.0);
  assert_eq!(d3, d4);

  let d5 = make_day(2000.0, -25.0, 1.0); // 1997 年 12 月 1 日
  let d6 = make_day(1997.0, 11.0, 1.0);
  assert_eq!(d5, d6);

  OK
}

/// 2. DateCache 缓存命中、失效、LRU 满载与防 panic 审查
#[test]
fn test_date_cache_lru_and_extreme_inputs() -> Void {
  let mut date_cache = DateCache::new(MockOscillatingTzProvider::default());

  // 连续查询超过 CACHE_SIZE (32) 个不同时间片，触发 LRU 淘汰与置换
  for i in 0..50 {
    let t = (i as i64) * 30 * MS_PER_DAY;
    let offset = date_cache.local_offset_in_ms(t, true);
    assert!(offset > 0);
  }

  // 缓存再次查询命中
  for i in 0..10 {
    let t = (i as i64) * 30 * MS_PER_DAY;
    let _ = date_cache.local_offset_in_ms(t, true);
  }

  // ymd_valid 缓存快速分支测试 (1..=28 天内命中)
  let (y1, m1, d1) = date_cache.year_month_day_from_days(100);
  let (y2, m2, d2) = date_cache.year_month_day_from_days(105);
  assert_eq!(y1, y2);
  assert_eq!(m1, m2);
  assert_eq!(d2, d1 + 5);

  // 跨越 28 天，缓存失效重算
  let (y3, m3, d3) = date_cache.year_month_day_from_days(130);
  assert!(m3 != m1 || y3 != y1 || d3 != d1);

  // to_date_string 极端值防崩溃审查：
  // 必须返回 "Invalid Date"，绝不可 panic
  assert_eq!(
    date_cache.to_date_string(f64::NAN, ToDateStringMode::LocalDate),
    "Invalid Date"
  );
  assert_eq!(
    date_cache.to_date_string(f64::INFINITY, ToDateStringMode::LocalDate),
    "Invalid Date"
  );
  assert_eq!(
    date_cache.to_date_string(f64::NEG_INFINITY, ToDateStringMode::LocalTime),
    "Invalid Date"
  );
  assert_eq!(
    date_cache.to_date_string(1e30, ToDateStringMode::ISODateAndTime),
    "Invalid Date"
  );
  assert_eq!(
    date_cache.to_date_string(-1e30, ToDateStringMode::UTCDateAndTime),
    "Invalid Date"
  );

  // 公元前负数年份格式化验证
  // -1 年 (即 0 BC)
  let days_neg = days_from_year_month(-1, 0); // -1 年 1 月 1 日
  let time_neg = (days_neg as i64) * MS_PER_DAY;
  let iso_neg = date_cache.to_date_string(time_neg as f64, ToDateStringMode::ISODateAndTime);
  assert!(iso_neg.starts_with("-000001-01-01"), "got: {iso_neg}");

  let utc_neg = date_cache.to_date_string(time_neg as f64, ToDateStringMode::UTCDateAndTime);
  assert!(utc_neg.contains("-0001"), "got: {utc_neg}");

  OK
}

/// 3. DateParser 极端时区偏移与防溢出对抗单测
#[test]
fn test_date_parser_overflow_and_adversarial_timezone() -> Void {
  let mut out = [0.0f64; OUTPUT_SIZE];

  // 巨型时区偏移: 原版 (dateparser.cc:106-108) 用 unsigned 32 位回绕算术判定,
  // 回绕后 <= Smi::kMaxValue 则接受 — 须与 C++ 逐值一致, 绝不可 panic
  // 999999999*3600 mod 2^32 = 817402352 <= 1073741823 → C++ 接受, 偏移取负
  assert!(DateParser::parse(
    "2020-01-01 12:00:00 -999999999:00",
    &mut out
  ));
  assert_eq!(out[UTC_OFFSET], -817402352.0);
  // 2000000*3600 mod 2^32 = 2905032704 > SmiMax → 拒绝
  assert!(!DateParser::parse(
    "2020-01-01 12:00:00 +2000000:00",
    &mut out
  ));
  // 3000000*3600 mod 2^32 = 2210065408 > SmiMax → 拒绝
  assert!(!DateParser::parse(
    "2020-01-01 12:00:00 +3000000:00",
    &mut out
  ));
  // 1234567*3600 mod 2^32 = 149473904 <= SmiMax → 接受
  // (分钟须过 IsMinute <= 59, 否则解析器走 day.add 拒绝, 到不了 tz.Write)
  assert!(DateParser::parse(
    "2020-01-01 12:00:00 +1234567:00",
    &mut out
  ));
  assert_eq!(out[UTC_OFFSET], 149473904.0);

  // 合法最大时区范围
  assert!(DateParser::parse("2020-01-01 12:00:00 +23:59", &mut out));
  assert_eq!(out[UTC_OFFSET], (23 * 3600 + 59 * 60) as f64);

  // 畸形时区符号
  assert!(!DateParser::parse("2020-01-01 12:00:00 +", &mut out));
  assert!(!DateParser::parse("2020-01-01 12:00:00 +-12:00", &mut out));
  assert!(!DateParser::parse("2020-01-01 12:00:00 +123456", &mut out));

  // 各种命名的有效时区简写
  for (tz_str, expected_hours) in [
    ("GMT", 0),
    ("UTC", 0),
    ("UT", 0),
    ("Z", 0),
    ("EST", -5),
    ("EDT", -4),
    ("CST", -6),
    ("CDT", -5),
    ("MST", -7),
    ("MDT", -6),
    ("PST", -8),
    ("PDT", -7),
  ] {
    let s = format!("2020-01-01 12:00:00 {tz_str}");
    assert!(DateParser::parse(&s, &mut out), "failed on {tz_str}");
    assert_eq!(
      out[UTC_OFFSET],
      (expected_hours * 3600) as f64,
      "failed on offset for {tz_str}"
    );
  }

  // 非月份关键字超长不得被误识别为时区
  assert!(!DateParser::parse("2020-01-01 12:00:00 GMTX", &mut out));
  assert!(!DateParser::parse("2020-01-01 12:00:00 UTCEXTRA", &mut out));

  // 月份大小写混合与超长（按 V8 规则前 3 字母匹配即可）
  assert!(DateParser::parse("18 SePtEmBeR 2023", &mut out));
  assert_eq!(out[MONTH], 8.0); // 0-based Sep
  assert!(DateParser::parse("18 jAnUaRy 2023", &mut out));
  assert_eq!(out[MONTH], 0.0);

  // 括号语法与未闭合括号
  assert!(DateParser::parse(
    "2023-10-18 (China Standard Time) 12:00:00",
    &mut out
  ));
  assert!(DateParser::parse(
    "2023-10-18 ((nested (comment))) 12:00",
    &mut out
  ));
  // 读取数字后未闭合或多余的 ')' 应当失败
  assert!(!DateParser::parse("2023-10-18 ) 12:00:00", &mut out));

  OK
}

/// 4. DateParser 毫秒解析与 ISO 格式边界对抗单测
#[test]
fn test_milliseconds_and_iso_edge_cases() -> Void {
  // read_milliseconds 边界
  assert_eq!(read_milliseconds(DateToken::number(5, 1)), 500);
  assert_eq!(read_milliseconds(DateToken::number(50, 2)), 500);
  assert_eq!(read_milliseconds(DateToken::number(5, 2)), 50); // .05 => 50ms
  assert_eq!(read_milliseconds(DateToken::number(5, 3)), 5); // .005 => 5ms
  assert_eq!(read_milliseconds(DateToken::number(123, 3)), 123);
  assert_eq!(read_milliseconds(DateToken::number(1234, 4)), 123);
  assert_eq!(read_milliseconds(DateToken::number(123456789, 9)), 123);
  assert_eq!(read_milliseconds(DateToken::number(123456789, 12)), 123);

  let mut out = [0.0f64; OUTPUT_SIZE];

  // ISO 24 点测试: 24:00:00 允许，其他一律非法
  assert!(DateParser::parse("2023-01-01T24:00:00Z", &mut out));
  assert_eq!(out[HOUR], 24.0);
  assert_eq!(out[MINUTE], 0.0);

  assert!(!DateParser::parse("2023-01-01T24:00:01Z", &mut out));
  assert!(!DateParser::parse("2023-01-01T24:00:00.001Z", &mut out));
  assert!(!DateParser::parse("2023-01-01T24:01:00Z", &mut out));
  assert!(!DateParser::parse("2023-01-01T25:00:00Z", &mut out));

  // ISO 扩展年份: -000000 必须失败 (ECMA-262: year zero must be positive)
  assert!(!DateParser::parse("-000000-01-01T00:00:00Z", &mut out));
  assert!(DateParser::parse("+000000-01-01T00:00:00Z", &mut out));
  assert_eq!(out[YEAR], 0.0);

  // parse_date_time_string 端到端边界
  let mut cache = DateCache::default();
  let ts_epoch = parse_date_time_string(&mut cache, "1970-01-01T00:00:00Z");
  assert_eq!(ts_epoch, 0.0);

  let ts_neg = parse_date_time_string(&mut cache, "1969-12-31T23:59:59Z");
  assert_eq!(ts_neg, -1000.0);

  // 超过 MAX_TIME_IN_MS 的日期字符串解析后 clip 返回 NaN
  let ts_overflow = parse_date_time_string(&mut cache, "+275761-01-01T00:00:00Z");
  assert!(ts_overflow.is_nan());

  OK
}
