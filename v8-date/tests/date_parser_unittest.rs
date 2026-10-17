// date_parser_unittest.rs — 日期算法、解析器与格式化测试
use aok::{OK, Void};
use v8_date::{
  algorithms::*,
  date_cache::{cache::DateCache, constants::*},
  dateparser::{
    parser::{DateParser, parse_date_time_string},
    tables::*,
  },
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_algorithms_basic() -> Void {
  // 1970-01-01 00:00:00 UTC 是 0 天
  assert_eq!(days_from_time(0), 0);
  assert_eq!(time_in_day(0, 0), 0);
  assert_eq!(weekday(0), 4); // 1970-01-01 是周四 (4)

  // 1970-01-02 01:00:00 UTC
  let t = 86_400_000 + 3_600_000;
  assert_eq!(days_from_time(t), 1);
  assert_eq!(time_in_day(t, 1), 3_600_000);
  assert_eq!(weekday(1), 5); // 周五 (5)

  // 负时间戳: 1969-12-31 23:59:59 UTC
  let neg_t = -1000;
  assert_eq!(days_from_time(neg_t), -1);
  assert_eq!(time_in_day(neg_t, -1), 86_400_000 - 1000);
  assert_eq!(weekday(-1), 3); // 周三 (3)

  // 闰年判断
  assert!(is_leap(2000));
  assert!(!is_leap(1900));
  assert!(is_leap(2024));
  assert!(!is_leap(2023));

  // days_from_year_month 与 year_month_day_from_days_uncached 互逆验证
  for y in [1970, 2000, 2024, 2030, -100] {
    for m in 0..12 {
      let days = days_from_year_month(y, m);
      let (out_y, out_m, out_d) = year_month_day_from_days_uncached(days);
      assert_eq!(out_y, y);
      assert_eq!(out_m, m);
      assert_eq!(out_d, 1);
    }
  }

  // MakeDay / MakeTime / MakeDate
  let day = make_day(1970.0, 0.0, 1.0);
  assert_eq!(day, 0.0);
  let time = make_time(12.0, 30.0, 45.0, 500.0);
  assert_eq!(
    time,
    12.0 * 3_600_000.0 + 30.0 * 60_000.0 + 45.0 * 1000.0 + 500.0
  );
  let dt = make_date(day, time);
  assert_eq!(dt, time);

  // TimeClip
  let mut valid_clip = 12345.67;
  assert!(try_time_clip(&mut valid_clip));
  assert_eq!(valid_clip, 12345.0);

  let mut overflow_clip = 1e20;
  assert!(!try_time_clip(&mut overflow_clip));

  OK
}

#[test]
fn test_iso_date_parsing() -> Void {
  let mut out = [0.0f64; 8];

  // 完整 ISO 8601 UTC
  assert!(DateParser::parse("2023-10-18T14:30:45.123Z", &mut out));
  assert_eq!(out[YEAR], 2023.0);
  assert_eq!(out[MONTH], 9.0); // 0-based Oct
  assert_eq!(out[DAY], 18.0);
  assert_eq!(out[HOUR], 14.0);
  assert_eq!(out[MINUTE], 30.0);
  assert_eq!(out[SECOND], 45.0);
  assert_eq!(out[MILLISECOND], 123.0);
  assert_eq!(out[UTC_OFFSET], 0.0);

  // 带正时区偏移 +08:00 (28800 秒)
  assert!(DateParser::parse("2023-10-18T14:30:45+08:00", &mut out));
  assert_eq!(out[UTC_OFFSET], 8.0 * 3600.0);

  // 带 hhmm 格式 -0500 (-18000 秒)
  assert!(DateParser::parse("2023-10-18T14:30:45-0500", &mut out));
  assert_eq!(out[UTC_OFFSET], -5.0 * 3600.0);

  // 仅日期格式 (默认 UTC 偏移 0)
  assert!(DateParser::parse("2023-10-18", &mut out));
  assert_eq!(out[YEAR], 2023.0);
  assert_eq!(out[MONTH], 9.0);
  assert_eq!(out[DAY], 18.0);
  assert_eq!(out[HOUR], 0.0);
  assert_eq!(out[MINUTE], 0.0);
  assert_eq!(out[SECOND], 0.0);
  assert_eq!(out[MILLISECOND], 0.0);
  assert_eq!(out[UTC_OFFSET], 0.0);

  // 年月格式
  assert!(DateParser::parse("2023-10", &mut out));
  assert_eq!(out[YEAR], 2023.0);
  assert_eq!(out[MONTH], 9.0);
  assert_eq!(out[DAY], 1.0);

  // 扩展 6 位年份
  assert!(DateParser::parse("+002023-10-18T00:00:00Z", &mut out));
  assert_eq!(out[YEAR], 2023.0);

  // 24:00:00 特殊边界 (允许作为一天的结束)
  assert!(DateParser::parse("2023-10-18T24:00:00Z", &mut out));
  assert_eq!(out[HOUR], 24.0);

  // 非法 24:01:00
  assert!(!DateParser::parse("2023-10-18T24:01:00Z", &mut out));

  OK
}

#[test]
fn test_legacy_date_parsing() -> Void {
  let mut out = [0.0f64; 8];

  // 标准 RFC 2822 / 英语月份
  let (success, legacy) =
    DateParser::parse_with_legacy_flag("Wed, 18 Oct 2023 14:30:45 GMT", &mut out);
  assert!(success);
  assert!(legacy);
  assert_eq!(out[YEAR], 2023.0);
  assert_eq!(out[MONTH], 9.0);
  assert_eq!(out[DAY], 18.0);
  assert_eq!(out[HOUR], 14.0);
  assert_eq!(out[MINUTE], 30.0);
  assert_eq!(out[SECOND], 45.0);
  assert_eq!(out[UTC_OFFSET], 0.0);

  // 斜杠与空格分隔
  let (success, legacy) = DateParser::parse_with_legacy_flag("10/18/2023 02:30:45 PM", &mut out);
  assert!(success);
  assert!(legacy);
  assert_eq!(out[YEAR], 2023.0);
  assert_eq!(out[MONTH], 9.0);
  assert_eq!(out[DAY], 18.0);
  assert_eq!(out[HOUR], 14.0); // PM -> 2 + 12 = 14
  assert_eq!(out[MINUTE], 30.0);

  // 两位年份自动补全: 0..=49 -> 2000s; 50..=99 -> 1900s
  assert!(DateParser::parse("Oct 18 23 12:00:00", &mut out));
  assert_eq!(out[YEAR], 2023.0);

  assert!(DateParser::parse("Oct 18 85 12:00:00", &mut out));
  assert_eq!(out[YEAR], 1985.0);

  // :: 语法（带日期）
  assert!(DateParser::parse("2020 12::", &mut out));
  assert_eq!(out[HOUR], 12.0);
  assert_eq!(out[MINUTE], 0.0);
  // 纯时间缺少日期部分按 V8 规范返回 false
  assert!(!DateParser::parse("12::", &mut out));

  OK
}

#[test]
fn test_to_date_string_formatting() -> Void {
  let mut date_cache = DateCache::default();

  // 1970-01-01T00:00:00.000Z -> 0.0
  let s_iso = date_cache.to_date_string(0.0, ToDateStringMode::ISODateAndTime);
  assert_eq!(s_iso, "1970-01-01T00:00:00.000Z");

  let s_utc = date_cache.to_date_string(0.0, ToDateStringMode::UTCDateAndTime);
  assert_eq!(s_utc, "Thu, 01 Jan 1970 00:00:00 GMT");

  // Invalid date
  let s_nan = date_cache.to_date_string(f64::NAN, ToDateStringMode::ISODateAndTime);
  assert_eq!(s_nan, "Invalid Date");

  OK
}

#[test]
fn test_parse_date_time_string_end_to_end() -> Void {
  let mut date_cache = DateCache::default();

  // 1970-01-01T00:00:00Z
  let ts = parse_date_time_string(&mut date_cache, "1970-01-01T00:00:00Z");
  assert_eq!(ts, 0.0);

  // 1970-01-01 (默认按 UTC 解析)
  let ts2 = parse_date_time_string(&mut date_cache, "1970-01-01");
  assert_eq!(ts2, 0.0);

  // 非法日期返回 NaN
  let ts_invalid = parse_date_time_string(&mut date_cache, "invalid date string");
  assert!(ts_invalid.is_nan());

  OK
}

#[test]
fn test_const_algorithms_and_zero_alloc_iter() -> Void {
  const {
    assert!(days_from_time(0) == 0);
    assert!(time_in_day(12345, 0) == 12345);
    assert!(weekday(0) == 4);
    assert!(is_leap(2024));
    assert!(days_from_year_month(2024, 1) == 19754);
    let (h, m, s, ms) = split_time_in_day(3661005);
    assert!(h == 1 && m == 1 && s == 1 && ms == 5);
    let tb = break_down_time_uncached(0);
    assert!(tb.year == 1970 && tb.month == 0 && tb.day == 1);
  }

  // 测试流式解析 iterator (零堆分配)
  let chars = [
    b'2' as u32,
    b'0' as u32,
    b'2' as u32,
    b'3' as u32,
    b'-' as u32,
    b'1' as u32,
    b'0' as u32,
  ];
  let mut out = [0.0f64; 8];
  let (ok, _) = DateParser::parse_chars_with_legacy_flag(&chars, &mut out);
  assert!(ok);
  assert_eq!(out[YEAR], 2023.0);
  assert_eq!(out[MONTH], 9.0);

  OK
}

#[test]
fn test_inl_generated_helpers() -> Void {
  use v8_date::dateparser::{
    DateToken,
    inl_generated::{input_reader_skel, normalize_timestamp, parse_es5_skel},
    tokenizer::InputReader,
  };

  // 1. read_milliseconds: 数字 token 正确缩放, 非数字返回 -1
  let num_1digit = DateToken::number(5, 1);
  assert_eq!(parse_es5_skel::read_milliseconds(&num_1digit), 500);

  let num_3digit = DateToken::number(123, 3);
  assert_eq!(parse_es5_skel::read_milliseconds(&num_3digit), 123);

  let num_4digit = DateToken::number(1234, 4);
  assert_eq!(parse_es5_skel::read_milliseconds(&num_4digit), 123);

  let sym_token = DateToken::symbol(':');
  assert_eq!(parse_es5_skel::read_milliseconds(&sym_token), -1);

  // 2. input_reader_skel: 无上下文默认 false, 绑定 reader 后真实跳步
  assert!(!input_reader_skel::skip_white_space());
  assert!(!input_reader_skel::skip_parentheses());

  let ws_chars = [b' ' as u32, b'x' as u32];
  let mut ws_reader = InputReader::new(ws_chars.iter().copied());
  assert!(input_reader_skel::skip_white_space_in(&mut ws_reader));
  assert_eq!(ws_reader.ch(), b'x' as u32);

  let paren_chars = [b'(' as u32, b'a' as u32, b')' as u32, b'z' as u32];
  let mut paren_reader = InputReader::new(paren_chars.iter().copied());
  assert!(input_reader_skel::skip_parentheses_in(&mut paren_reader));
  assert_eq!(paren_reader.ch(), b'z' as u32);

  // 3. normalize_timestamp: 有效范围规范化, 超界返回 NaN
  let clipped = normalize_timestamp(12345.67);
  assert_eq!(clipped, 12345.0);
  let nan = normalize_timestamp(1e20);
  assert!(nan.is_nan());

  OK
}
