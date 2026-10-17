// 由 tool/src/date/dateparser-inl.h.js 从 v8/src/date/dateparser-inl.h 生成; 修订就地改, 复跑覆盖
// 日期解析器 -inl 实现: 文法界常量与骨架

use super::{tables::KeywordType, tokenizer::DateToken};
use crate::algorithms::try_time_clip;

/// 丢失分量哨兵 (原版 kNone = kMaxInt)
pub const NONE: i32 = i32::MAX;

/// 数字有效位上限, 超出部分跳过 (原版 kMaxSignificantDigits = 9)
pub const MAX_SIGNIFICANT_DIGITS: i32 = 9;

/// 关键词前缀长度: 月名/时区名/am/pm 按小写前 3 字符查表 (原版 KeywordTable::kPrefixLength)
pub const KEYWORD_PREFIX_LENGTH: usize = 3;

/// ASCII 符号值魔数 (原版 GetAsciiSignValue: 44 - ch)
/// '+'(43) → 1, '-'(45) → -1
#[inline]
#[must_use]
pub const fn ascii_sign_value(ch: u32) -> i32 {
  44 - ch as i32
}

/// ES5 ISO 8601 日期文法界 (原版 Parse 注释规约 + 谓词机械展开)
pub mod es5_grammar {
  /// yyyy 位数, 值域 0000..9999
  pub const YEAR_DIGITS: usize = 4;
  /// ±yyyyyy 位数, 值域 -999999..+999999; -000000 非法 (年零须为正)
  pub const SIGNED_YEAR_DIGITS: usize = 6;
  /// MM 位数, 值域 01..12
  pub const MONTH_DIGITS: usize = 2;
  pub const MONTH_MIN: i32 = 1;
  pub const MONTH_MAX: i32 = 12;
  /// DD 位数, 值域 01..31, 缺省 01
  pub const DAY_DIGITS: usize = 2;
  pub const DAY_MIN: i32 = 1;
  pub const DAY_MAX: i32 = 31;
  /// HH 位数, 一般 00..23; 24 仅当 mm/ss/sss 全零 (当日末尾午夜)
  pub const HOUR_DIGITS: usize = 2;
  pub const MAX_HOUR: i32 = 24;
  /// mm 位数, 00..59, 缺省 00
  pub const MINUTE_DIGITS: usize = 2;
  pub const MAX_MINUTE: i32 = 59;
  /// ss 位数, 00..59, 缺省 00
  pub const SECOND_DIGITS: usize = 2;
  pub const MAX_SECOND: i32 = 59;
  /// sss 位数为规约值 3, 实际允许 ≥1 位
  pub const MILLISECOND_DIGITS: usize = 3;
  pub const MAX_MILLISECOND: i32 = 999;
  /// 时区: Z 缺省; hh:mm 标准式 hh ∈ 00..23
  pub const TZ_HOUR_DIGITS: usize = 2;
  /// 时区扩展式 hhmm
  pub const TZ_HHMM_DIGITS: usize = 4;
}

/// DateStringTokenizer::Scan 决策序 (原版实现机械登记):
/// 1) 输入尽 → EndOfInput
/// 2) ASCII 数字 → 读无符号数 (截 9 位) → Number(n, len)
/// 3) 符号按序 ':' '-' '+' '.' ')' → Symbol(ch)
/// 4) ≥'A' 且非空白 → 读词取 3 字符小写前缀查 KeywordTable → Keyword(type, value, len)
/// 5) 空白 → WhiteSpace(len)
/// 6) 括号段 (配平 '(' ')') → Unknown
/// 7) 其余单字符 → Unknown
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanOutcome {
  EndOfInput,
  Number,
  Symbol(u8),
  Keyword(KeywordType),
  WhiteSpace,
  Unknown,
}

/// 遗留 (非 ES5) 解析器分支决策表 (原版 Parse 主循环机械登记)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyBranch {
  /// n + "::" → time 需空, 加 n 与 0
  DoubleColonTime,
  /// n + ":" → time.Add(n), 后随 '.' 则进入毫秒
  ColonTime,
  /// n + "." 且 time.IsExpecting(n) → 加 n, 强制读毫秒, <0 即败
  DotMilliseconds,
  /// tz.IsExpecting(n) → tz.SetAbsoluteMinute(n)
  TzAbsoluteMinute,
  /// time.IsExpecting(n) → time.AddFinal(n); 后随字符白名单: 尽/空白/'Z'/正负号
  TimeFinal,
  /// 其余数字 → day.Add(n), 跳过 '-'
  DayComponent,
  /// AM_PM 且 time 非空 → time.SetHourOffset(value)
  AmPmOffset,
  /// MONTH_NAME → day.SetNamedMonth(value), 跳 '-'
  NamedMonth,
  /// TIME_ZONE_NAME 且已读数 → tz.Set(value)
  TzName,
  /// 已读数后的垃圾词 → 失败; 首数前允许
  GarbageWord,
  /// 符号且 (tz.IsUTC() 或 time 非空) → 读 UTC 偏移; 长度 1/2 → hh,0; 3/4 → hhmm
  SignTzOffset,
  /// 已读数后多余 '+' '-' ')' → 失败
  StraySignOrParen,
  /// 其余 (含空白) → 忽略
  Ignore,
}

/// ParseES5DateTime 消费序骨架 (原版实现机械登记, 语义由 Developer 修订):
/// [('-'|'+')yy]yyyy['-'MM['-'DD]] ['T'HH':'mm[':'ss['.'sss]] ['Z'|('+'|'-')(hh':'mm|hhmm)]]
pub mod parse_es5_skel {
  use super::DateToken;
  use crate::dateparser::parser;

  /// 日期部解析结局
  pub enum Stage {
    /// 年 (可带符号 6 位或 4 位)
    Year,
    /// '-'MM
    Month,
    /// '-'DD
    Day,
    /// 'T' 后时间部
    Time,
    /// 时区名 'Z'
    TzZulu,
    /// ('+'|'-')hh:mm 或 hhmm
    TzOffset,
    /// 全部消费尽且成功 (day.set_iso_date)
    Done,
    /// 退回遗留解析器 (返回未处理 token)
    Fallback,
    /// 失败 (DateToken::Invalid)
    Invalid,
  }

  /// 毫秒读取 (原版 ReadMilliseconds): token 为数字 → 按 10^补齐 3 位, 否则 -1
  pub fn read_milliseconds(token: &DateToken) -> i32 {
    if !token.is_number() {
      return -1;
    }
    parser::read_milliseconds(*token)
  }
}

/// InputReader 跳读辅助 (原版 DateParser::InputReader)
pub mod input_reader_skel {
  use crate::dateparser::tokenizer::InputReader;

  /// SkipWhiteSpace: 当前为空白则前进并返回 true (无上下文调用默认返回 false)
  #[inline]
  pub fn skip_white_space() -> bool {
    false
  }

  /// 真实在 InputReader 实例上跳过空白 (原版 DateParser::InputReader::SkipWhiteSpace)
  #[inline]
  pub fn skip_white_space_in<I: Iterator<Item = u32>>(reader: &mut InputReader<I>) -> bool {
    reader.skip_white_space()
  }

  /// SkipParentheses: ch=='(' 时按配平跳过整个括号段, 恒返回 true (自 '(' 起) (无上下文调用默认返回 false)
  #[inline]
  pub fn skip_parentheses() -> bool {
    false
  }

  /// 真实在 InputReader 实例上跳过配平括号段 (原版 DateParser::InputReader::SkipParentheses)
  #[inline]
  pub fn skip_parentheses_in<I: Iterator<Item = u32>>(reader: &mut InputReader<I>) -> bool {
    reader.skip_parentheses()
  }
}

/// 时间戳规范化 (原版 DateCache::TryTimeClip / date.cc ParseDateTimeString)
#[inline]
pub fn normalize_timestamp(mut time: f64) -> f64 {
  if try_time_clip(&mut time) {
    time
  } else {
    f64::NAN
  }
}
