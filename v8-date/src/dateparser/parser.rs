// parser.rs — DateParser 解析器核心实现
use crate::{
  algorithms::*,
  date_cache::{
    cache::DateCache, constants::MAX_TIME_BEFORE_UTC_IN_MS, provider::TimezoneProvider,
  },
  dateparser::{composer::*, tables::*, tokenizer::*},
};

const POW10: [i32; 7] = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];

/// 读取毫秒值（1-9位数字提取有效前三位）
#[inline]
pub const fn read_milliseconds(token: DateToken) -> i32 {
  let mut number = token.number_value();
  let length = token.length;
  if length < 3 {
    if length == 1 {
      number *= 100;
    } else if length == 2 {
      number *= 10;
    }
  } else if length > 3 {
    let idx = if length > MAX_SIGNIFICANT_DIGITS {
      MAX_SIGNIFICANT_DIGITS - 3
    } else {
      length - 3
    };
    number /= POW10[idx];
  }
  number
}

/// 解析 ES5 ISO 8601 日期时间字符串
pub fn parse_es5_date_time<I: Iterator<Item = u32>>(
  scanner: &mut DateStringTokenizer<I>,
  day: &mut DayComposer,
  time: &mut TimeComposer,
  tz: &mut TimeZoneComposer,
) -> DateToken {
  debug_assert!(day.is_empty());
  debug_assert!(time.is_empty());
  debug_assert!(tz.is_empty());

  // 解析日期部分: [('-'|'+')yy]yyyy[':'MM[':'DD]]
  if scanner.peek().is_ascii_sign() {
    let sign_token = scanner.next_token();
    if !scanner.peek().is_fixed_length_number(6) {
      return sign_token;
    }
    let sign = sign_token.ascii_sign();
    let year = scanner.next_token().number_value();
    if sign < 0 && year == 0 {
      return sign_token;
    }
    day.add(sign * year);
  } else if scanner.peek().is_fixed_length_number(4) {
    day.add(scanner.next_token().number_value());
  } else {
    return scanner.next_token();
  }

  if scanner.skip_symbol('-') {
    if !scanner.peek().is_fixed_length_number(2)
      || !DayComposer::is_month(scanner.peek().number_value())
    {
      return scanner.next_token();
    }
    day.add(scanner.next_token().number_value());
    if scanner.skip_symbol('-') {
      if !scanner.peek().is_fixed_length_number(2)
        || !DayComposer::is_day(scanner.peek().number_value())
      {
        return scanner.next_token();
      }
      day.add(scanner.next_token().number_value());
    }
  }

  // 检查可选时间部分: 'T'HH':'mm[':'ss['.'sss]]Z
  if !scanner.peek().is_keyword_type(KeywordType::TimeSeparator) {
    if !scanner.peek().is_end_of_input() {
      return scanner.next_token();
    }
  } else {
    scanner.next_token(); // 消费 'T'
    if !scanner.peek().is_fixed_length_number(2) || !between(scanner.peek().number_value(), 0, 24) {
      return DateToken::invalid();
    }
    let hour_is_24 = scanner.peek().number_value() == 24;
    time.add(scanner.next_token().number_value());
    if !scanner.skip_symbol(':') {
      return DateToken::invalid();
    }
    if !scanner.peek().is_fixed_length_number(2)
      || !TimeComposer::is_minute(scanner.peek().number_value())
      || (hour_is_24 && scanner.peek().number_value() > 0)
    {
      return DateToken::invalid();
    }
    time.add(scanner.next_token().number_value());
    if scanner.skip_symbol(':') {
      if !scanner.peek().is_fixed_length_number(2)
        || !TimeComposer::is_second(scanner.peek().number_value())
        || (hour_is_24 && scanner.peek().number_value() > 0)
      {
        return DateToken::invalid();
      }
      time.add(scanner.next_token().number_value());
      if scanner.skip_symbol('.') {
        if !scanner.peek().is_number() || (hour_is_24 && scanner.peek().number_value() > 0) {
          return DateToken::invalid();
        }
        time.add(read_milliseconds(scanner.next_token()));
      }
    }

    // 检查可选时区标识: 'Z' | ('+'|'-')hh':'mm
    if scanner.peek().is_keyword_z() {
      scanner.next_token();
      tz.set(0);
    } else if scanner.peek().is_symbol_char('+') || scanner.peek().is_symbol_char('-') {
      tz.set_sign(if scanner.next_token().symbol_char() == '+' {
        1
      } else {
        -1
      });
      if scanner.peek().is_fixed_length_number(4) {
        let hourmin = scanner.next_token().number_value();
        let hour = hourmin / 100;
        let min = hourmin % 100;
        if !TimeComposer::is_hour(hour) || !TimeComposer::is_minute(min) {
          return DateToken::invalid();
        }
        tz.set_absolute_hour(hour);
        tz.set_absolute_minute(min);
      } else {
        if !scanner.peek().is_fixed_length_number(2)
          || !TimeComposer::is_hour(scanner.peek().number_value())
        {
          return DateToken::invalid();
        }
        tz.set_absolute_hour(scanner.next_token().number_value());
        if !scanner.skip_symbol(':') {
          return DateToken::invalid();
        }
        if !scanner.peek().is_fixed_length_number(2)
          || !TimeComposer::is_minute(scanner.peek().number_value())
        {
          return DateToken::invalid();
        }
        tz.set_absolute_minute(scanner.next_token().number_value());
      }
    }

    if !scanner.peek().is_end_of_input() {
      return DateToken::invalid();
    }
  }

  if tz.is_empty() && time.is_empty() {
    tz.set(0);
  }
  day.set_iso_date();
  DateToken::end_of_input()
}

pub struct DateParser;

impl DateParser {
  pub fn parse_iter_with_legacy_flag<I: Iterator<Item = u32>>(
    iter: I,
    out: &mut [f64; OUTPUT_SIZE],
  ) -> (bool, bool) {
    let reader = InputReader::new(iter);
    let mut scanner = DateStringTokenizer::new(reader);
    let mut tz = TimeZoneComposer::new();
    let mut time = TimeComposer::new();
    let mut day = DayComposer::new();

    let next_unhandled_token = parse_es5_date_time(&mut scanner, &mut day, &mut time, &mut tz);
    if next_unhandled_token.is_invalid() {
      return (false, false);
    }
    let mut has_read_number = !day.is_empty();
    let mut legacy_parser = false;

    let mut token = next_unhandled_token;
    while !token.is_end_of_input() {
      if token.is_number() {
        legacy_parser = true;
        has_read_number = true;
        let n = token.number_value();
        if scanner.skip_symbol(':') {
          if scanner.skip_symbol(':') {
            if !time.is_empty() {
              return (false, legacy_parser);
            }
            time.add(n);
            time.add(0);
          } else {
            if !time.add(n) {
              return (false, legacy_parser);
            }
            if scanner.peek().is_symbol_char('.') {
              scanner.next_token();
            }
          }
        } else if scanner.skip_symbol('.') && time.is_expecting(n) {
          time.add(n);
          if !scanner.peek().is_number() {
            return (false, legacy_parser);
          }
          let ms = read_milliseconds(scanner.next_token());
          if ms < 0 {
            return (false, legacy_parser);
          }
          time.add_final(ms);
        } else if tz.is_expecting(n) {
          tz.set_absolute_minute(n);
        } else if time.is_expecting(n) {
          time.add_final(n);
          let peek = scanner.peek();
          if !peek.is_end_of_input()
            && !peek.is_white_space()
            && !peek.is_keyword_z()
            && !peek.is_ascii_sign()
          {
            return (false, legacy_parser);
          }
        } else {
          if !day.add(n) {
            return (false, legacy_parser);
          }
          scanner.skip_symbol('-');
        }
      } else if token.is_keyword() {
        legacy_parser = true;
        let kw_type = token.keyword_type();
        let value = token.keyword_value();
        if kw_type == KeywordType::AmPm && !time.is_empty() {
          time.set_hour_offset(value);
        } else if kw_type == KeywordType::MonthName {
          day.set_named_month(value);
          scanner.skip_symbol('-');
        } else if kw_type == KeywordType::TimeZoneName && has_read_number {
          tz.set(value);
        } else {
          if has_read_number {
            return (false, legacy_parser);
          }
          if scanner.peek().is_number() {
            return (false, legacy_parser);
          }
        }
      } else if token.is_ascii_sign() && (tz.is_utc() || !time.is_empty()) {
        legacy_parser = true;
        tz.set_sign(token.ascii_sign());
        let mut n = 0;
        let mut length = 0;
        if scanner.peek().is_number() {
          let next_token = scanner.next_token();
          length = next_token.length;
          n = next_token.number_value();
        }
        has_read_number = true;

        if scanner.peek().is_symbol_char(':') {
          tz.set_absolute_hour(n);
          tz.set_absolute_minute(K_NONE);
        } else if length == 2 || length == 1 {
          tz.set_absolute_hour(n);
          tz.set_absolute_minute(0);
        } else if length == 4 || length == 3 {
          tz.set_absolute_hour(n / 100);
          tz.set_absolute_minute(n % 100);
        } else {
          return (false, legacy_parser);
        }
      } else if (token.is_ascii_sign() || token.is_symbol_char(')')) && has_read_number {
        return (false, legacy_parser);
      }

      token = scanner.next_token();
    }

    let success = day.write(out) && time.write(out) && tz.write(out);
    (success, legacy_parser)
  }

  /// 将字符串解析为日期组件数组
  /// 返回 (是否解析成功, 是否触发了 legacy_parser 回退)
  #[inline]
  pub fn parse_with_legacy_flag(s: &str, output: &mut [f64; OUTPUT_SIZE]) -> (bool, bool) {
    Self::parse_iter_with_legacy_flag(s.chars().map(|c| c as u32), output)
  }

  /// 基础解析接口
  #[inline]
  pub fn parse(s: &str, output: &mut [f64; OUTPUT_SIZE]) -> bool {
    Self::parse_with_legacy_flag(s, output).0
  }

  #[inline]
  pub fn parse_chars_with_legacy_flag(chars: &[u32], out: &mut [f64; OUTPUT_SIZE]) -> (bool, bool) {
    Self::parse_iter_with_legacy_flag(chars.iter().copied(), out)
  }
}

/// ES6 section 20.3.1.16 Date Time String Format
/// 解析日期时间字符串为 Epoch 毫秒时间戳
pub fn parse_date_time_string<P: TimezoneProvider>(date_cache: &mut DateCache<P>, s: &str) -> f64 {
  let mut out = [0.0f64; OUTPUT_SIZE];
  if !DateParser::parse(s, &mut out) {
    return f64::NAN;
  }
  let day = make_day(out[YEAR], out[MONTH], out[DAY]);
  let time = make_time(out[HOUR], out[MINUTE], out[SECOND], out[MILLISECOND]);
  let mut date = make_date(day, time);
  if out[UTC_OFFSET].is_nan() {
    if (-MAX_TIME_BEFORE_UTC_IN_MS as f64..=MAX_TIME_BEFORE_UTC_IN_MS as f64).contains(&date) {
      date = date_cache.to_utc(date as i64) as f64;
    } else {
      return f64::NAN;
    }
  } else {
    date -= out[UTC_OFFSET] * 1000.0;
  }
  if !try_time_clip(&mut date) {
    return f64::NAN;
  }
  date
}
