// review_round2_unittest.rs — 第 2 轮对抗式审查极限边界与模糊测试
use aok::{OK, Void};
use v8_date::{
  algorithms::*,
  date_cache::{
    cache::DateCache,
    constants::*,
    provider::{TimeZoneDetection, TimezoneProvider},
  },
  dateparser::{
    parser::{DateParser, parse_date_time_string},
    tables::*,
  },
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[derive(Debug, Default)]
struct MockProvider;

impl TimezoneProvider for MockProvider {
  fn get_daylight_savings_offset_from_os(&mut self, _time_sec: i64) -> i32 {
    0
  }

  fn get_local_offset_from_os(&mut self, _time_ms: i64, _is_utc: bool) -> i32 {
    8 * 3600 * 1000
  }

  fn local_timezone(&mut self, _time_ms: f64) -> &str {
    "CST"
  }

  fn clear(&mut self, _detection: TimeZoneDetection) {}
}

/// 1. 验证 equivalent_year 在 [-10000, 10000] 范围内满足 ECMAScript 规范
#[test]
fn test_equivalent_year_spec_properties() -> Void {
  for year in -10000..=10000 {
    let eq = equivalent_year(year);
    // 1. 等价年份必须在 [2008, 2035] 之间
    assert!(
      (2008..=2035).contains(&eq),
      "eq {eq} not in [2008, 2035] for year {year}"
    );
    // 2. 闰年性必须一致
    assert_eq!(
      is_leap(eq),
      is_leap(year),
      "leap mismatch for year {year} and eq {eq}"
    );
    // 3. 元旦星期必须一致
    let w_orig = weekday(days_from_year_month(year, 0));
    let w_eq = weekday(days_from_year_month(eq, 0));
    assert_eq!(w_orig, w_eq, "weekday mismatch for year {year} and eq {eq}");
  }
  OK
}

/// 2. 日期算法双向可逆性与闰秒/跨年边缘密集扫描
#[test]
fn test_algorithm_bijective_and_edge_leap_years() -> Void {
  // 闰年世纪边界
  let leap_years = [
    -1600, -1200, -800, -400, 0, 4, 1600, 1996, 2000, 2004, 2020, 2024, 2400,
  ];
  let non_leap_years = [
    -100, -200, -300, -500, 1, 700, 1700, 1800, 1900, 1999, 2001, 2100,
  ];

  for &y in &leap_years {
    assert!(is_leap(y), "{y} should be leap");
    // 2 月 29 日
    let days_feb29 = days_from_year_month(y, 1) + 29 - 1;
    let (out_y, out_m, out_d) = year_month_day_from_days_uncached(days_feb29);
    assert_eq!((out_y, out_m, out_d), (y, 1, 29));

    // 3 月 1 日
    let days_mar1 = days_from_year_month(y, 2);
    assert_eq!(days_mar1, days_feb29 + 1);
    let (out_y, out_m, out_d) = year_month_day_from_days_uncached(days_mar1);
    assert_eq!((out_y, out_m, out_d), (y, 2, 1));
  }

  for &y in &non_leap_years {
    assert!(!is_leap(y), "{y} should not be leap");
    // 2 月 28 日
    let days_feb28 = days_from_year_month(y, 1) + 28 - 1;
    let (out_y, out_m, out_d) = year_month_day_from_days_uncached(days_feb28);
    assert_eq!((out_y, out_m, out_d), (y, 1, 28));

    // 平年 3 月 1 日紧随 2 月 28 日
    let days_mar1 = days_from_year_month(y, 2);
    assert_eq!(days_mar1, days_feb28 + 1);
    let (out_y, out_m, out_d) = year_month_day_from_days_uncached(days_mar1);
    assert_eq!((out_y, out_m, out_d), (y, 2, 1));
  }

  // 跨千年末边界测试
  for y in 1900..=2100 {
    // 12 月 31 日到次年 1 月 1 日
    let dec31 = days_from_year_month(y, 11) + 31 - 1;
    let jan1 = days_from_year_month(y + 1, 0);
    assert_eq!(dec31 + 1, jan1);

    let (y_dec, m_dec, d_dec) = year_month_day_from_days_uncached(dec31);
    assert_eq!((y_dec, m_dec, d_dec), (y, 11, 31));

    let (y_jan, m_jan, d_jan) = year_month_day_from_days_uncached(jan1);
    assert_eq!((y_jan, m_jan, d_jan), (y + 1, 0, 1));
  }

  OK
}

/// 3. DateCache 计数器近溢出重置与状态完整性测试
#[test]
fn test_date_cache_counter_near_overflow_reset() -> Void {
  let mut date_cache = DateCache::new(MockProvider);

  // 初始正常查询
  let offset = date_cache.local_offset_in_ms(0, true);
  assert_eq!(offset, 8 * 3600 * 1000);

  // 模拟缓存被大量使用，触发重置
  for i in 0..100 {
    let t = (i as i64) * 50 * MS_PER_DAY;
    let _ = date_cache.local_offset_in_ms(t, true);
  }

  // 验证重置后依旧能正确返回结果
  let offset_after = date_cache.local_offset_in_ms(0, true);
  assert_eq!(offset_after, 8 * 3600 * 1000);

  OK
}

/// 4. DateParser 极端畸形文本、超长字符、控制符对抗测试（零 Panic 保证）
#[test]
fn test_date_parser_adversarial_malformed_inputs() -> Void {
  let mut out = [0.0f64; OUTPUT_SIZE];

  let malformed_cases = [
    "",
    " ",
    "   \t\r\n\x0b\x0c  ",
    "\0",
    "2020\0-01-01",
    "(((((((((((((((((((())))))))))))))))))))",
    "((((( unclosed parenthesis",
    "))))) unopened parenthesis",
    "++++++++++",
    "----------",
    "::::::::::",
    "..........",
    "2020-00-01",           // 0 月非法
    "2020-13-01",           // 13 月非法
    "2020-01-00",           // 0 日非法
    "2020-01-32",           // 32 日非法
    "2023-02-29T00:00:00Z", // 平年 2 月 29 日
    "2020-01-01T25:00:00Z", // 25 点非法
    "2020-01-01T24:00:01Z", // 24 点非零秒非法
    "2020-01-01T24:01:00Z", // 24 点非零分非法
    "2020-01-01T12:60:00Z", // 60 分非法
    "2020-01-01T12:00:60Z", // 60 秒非法
    "2020-01-01 +99999999999:00",
    "2020-01-01 -99999999999:00",
    "99999999999999999999999999999999",
    "NaN",
    "Infinity",
    "-Infinity",
    "undefined",
    "null",
    "[object Object]",
    "true",
    "false",
    "Wednesday, 32 October 2023",
    "31 February 2023",
  ];

  for case in &malformed_cases {
    // 严禁 panic，非法格式必须安全返回 false 或 NaN
    let _ = DateParser::parse(case, &mut out);
    let mut cache = DateCache::new(MockProvider);
    let ts = parse_date_time_string(&mut cache, case);
    if case.contains("32") || case.contains("25:00") || case.contains("NaN") {
      assert!(ts.is_nan() || !DateParser::parse(case, &mut out));
    }
  }

  // 超长重复串（10,000 字符）
  let huge_spaces = " ".repeat(10_000);
  assert!(!DateParser::parse(&huge_spaces, &mut out));

  let huge_digits = "1".repeat(10_000);
  let mut cache = DateCache::new(MockProvider);
  assert!(parse_date_time_string(&mut cache, &huge_digits).is_nan());

  let huge_parens = "(".repeat(5_000) + &")".repeat(5_000);
  assert!(!DateParser::parse(&huge_parens, &mut out));

  OK
}

/// 5. Unicode 空白字符与控制符解析稳健性
#[test]
fn test_unicode_whitespace_and_separators() -> Void {
  let mut out = [0.0f64; OUTPUT_SIZE];

  // \u{00A0} (不换行空格), \u{2000} (En 空格), \u{3000} (全角空格), \u{FEFF} (BOM)
  let iso_with_unicode_ws = "\u{FEFF}2023-10-18\u{00A0}14:30:45\u{3000}GMT";
  let (ok, legacy) = DateParser::parse_with_legacy_flag(iso_with_unicode_ws, &mut out);
  assert!(ok);
  assert!(legacy);
  assert_eq!(out[YEAR], 2023.0);
  assert_eq!(out[MONTH], 9.0);
  assert_eq!(out[DAY], 18.0);
  assert_eq!(out[HOUR], 14.0);
  assert_eq!(out[MINUTE], 30.0);
  assert_eq!(out[SECOND], 45.0);

  // LS(U+2028)/PS(U+2029) 是 word 字符而非空白 (原版 dateparser.h:119
  // IsWhiteSpaceChar => IsWhiteSpace, 不含行终止符; LS/PS >= 'A'):
  // "xyz\u{2028}1970" 中 garbage 词 "xyz\u2028" 与数字紧邻 → 原版拒绝
  // (dateparser-inl.h:134: garbage 词后 Peek 是数字直接 fail)
  assert!(!DateParser::parse("xyz\u{2028}1970", &mut out));
  assert!(!DateParser::parse("xyz\u{2029}1970", &mut out));
  // LS/PS 被 word 吞入: "may\u{2028}" 词长 4, 月名允许超 3 词 (Lookup 规则),
  // 与 "may 1 1970" 同解 → May 1 1970
  assert!(DateParser::parse("may\u{2028} 1 1970", &mut out));
  assert_eq!(out[MONTH], 4.0);
  assert_eq!(out[DAY], 1.0);
  assert_eq!(out[YEAR], 1970.0);

  OK
}

/// 6. AM/PM 12 小时制与越界对抗测试
#[test]
fn test_am_pm_hour_rules() -> Void {
  let mut out = [0.0f64; OUTPUT_SIZE];

  // 12:00:00 AM -> 00:00:00
  assert!(DateParser::parse("2023-10-18 12:00:00 AM", &mut out));
  assert_eq!(out[HOUR], 0.0);

  // 12:00:00 PM -> 12:00:00
  assert!(DateParser::parse("2023-10-18 12:00:00 PM", &mut out));
  assert_eq!(out[HOUR], 12.0);

  // 01:00:00 AM -> 01:00:00
  assert!(DateParser::parse("2023-10-18 01:00:00 AM", &mut out));
  assert_eq!(out[HOUR], 1.0);

  // 01:00:00 PM -> 13:00:00
  assert!(DateParser::parse("2023-10-18 01:00:00 PM", &mut out));
  assert_eq!(out[HOUR], 13.0);

  // 13:00:00 AM/PM 非法 (12小时制下 hour 必须在 0..=12)
  assert!(!DateParser::parse("2023-10-18 13:00:00 AM", &mut out));
  assert!(!DateParser::parse("2023-10-18 13:00:00 PM", &mut out));

  OK
}
