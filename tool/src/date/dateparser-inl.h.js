#!/usr/bin/env bun
// tool/src/date/dateparser-inl.h.js
// 机械提取 v8/src/date/dateparser-inl.h: ES5 日期文法界常量、Scan 决策表、
// ParseES5DateTime / InputReader 跳读签名骨架
// (dateparser.h 的 DateToken tag/KeywordType/区域判定已由 tool/src/date/dateparser.h.js 产出 tables.rs, 不重复)

import { readV8, writeSrc, banner } from "../../lib.js";

const inlSrc = readV8("src/date/dateparser-inl.h");
const hSrc = readV8("src/date/dateparser.h");

// 1. 对数守卫: -inl.h 内的关键函数
const mustHave = [
  "DateParser::Parse(",
  "DateStringTokenizer<CharType>::Scan",
  "InputReader<Char>::SkipWhiteSpace",
  "InputReader<Char>::SkipParentheses",
  "DateParser::ParseES5DateTime",
];
for (const k of mustHave) {
  if (!inlSrc.includes(k)) throw new Error(`dateparser-inl.h 未找到 ${k}`);
}

// 2. dateparser.h 侧关键常量 (跨文件引用核对)
const kNoneM = hSrc.match(/static const int kNone = (kMaxInt);/);
if (!kNoneM) throw new Error("未找到 kNone = kMaxInt");
const kMaxSigM = hSrc.match(/static const int kMaxSignificantDigits = (\d+);/);
if (!kMaxSigM) throw new Error("未找到 kMaxSignificantDigits");
const kMaxSignificantDigits = parseInt(kMaxSigM[1], 10);

const prefixM = hSrc.match(/KeywordTable::kPrefixLength,\s*3\)/) || hSrc.match(/kPrefixLength = (\d+)/);
if (!prefixM && !/kPrefixLength/.test(hSrc)) throw new Error("未找到 KeywordTable::kPrefixLength");
// Scan() 内有 DCHECK_EQ(KeywordTable::kPrefixLength, 3)
if (!/DCHECK_EQ\(KeywordTable::kPrefixLength, 3\)/.test(inlSrc)) throw new Error("Scan 未断言 kPrefixLength==3");

// 3. Scan() 的符号决策表 (机械提取 Skip(char) 次序)
const scanSymbols = [...inlSrc.matchAll(/in_->Skip\('(.)'\)\) return DateToken::Symbol\('\1'\);/g)].map(
  (m) => m[1],
);
if (scanSymbols.join("") !== ":-+.)") throw new Error(`Scan 符号序漂移: ${scanSymbols.join("")}`);

// 4. GetAsciiSignValue 魔数 (44 - ch: '+'(43)=1, '-'(45)=-1)
if (!/44 - static_cast<int>\(ch_\)/.test(hSrc)) throw new Error("未找到 GetAsciiSignValue 魔数 44");

// 5. ES5 文法界常量 (Parse 注释 + ParseES5DateTime 判定机械登记)
const es5Bounds = {
  yearDigits: 4, // yyyy ∈ 0000..9999
  signYearDigits: 6, // ±yyyyyy ∈ -999999..+999999, -000000 非法
  monthDigits: 2, // MM ∈ 01..12
  dayDigits: 2, // DD ∈ 01..31
  hourDigits: 2, // HH ∈ 00..23 (24 仅全零午夜)
  minuteDigits: 2, // mm ∈ 00..59
  secondDigits: 2, // ss ∈ 00..59
  msDigits: 3, // sss ∈ 000..999, 允许非 3 位但 ≥1
  tzHourDigits2: 2, // hh:mm 标准式
  tzHourDigits4: 4, // hhmm 扩展式
  maxHour24: 24, // ES5 允许 24:00[:00[.000]]
  maxSecond: 59,
  maxMinute: 59,
  maxMillisecond: 999,
  monthMin: 1,
  monthMax: 12,
  dayMin: 1,
  dayMax: 31,
};
// 判定函数对数: dateparser.h 谓词界与上表一致
if (!/IsMinute\(int x\) \{ return Between\(x, 0, 59\);/.test(hSrc)) throw new Error("IsMinute 界漂移");
if (!/IsHour\(int x\) \{ return Between\(x, 0, 23\);/.test(hSrc)) throw new Error("IsHour 界漂移");
if (!/IsMonth\(int x\) \{ return Between\(x, 1, 12\);/.test(hSrc)) throw new Error("IsMonth 界漂移");
if (!/IsDay\(int x\) \{ return Between\(x, 1, 31\);/.test(hSrc)) throw new Error("IsDay 界漂移");

let rust = banner("src/date/dateparser-inl.h.js", "src/date/dateparser-inl.h", "日期解析器 -inl 实现: 文法界常量与骨架");

rust += `
use super::tables::{DateToken, KeywordType};

/// 丢失分量哨兵 (原版 kNone = kMaxInt)
pub const NONE: i32 = i32::MAX;

/// 数字有效位上限, 超出部分跳过 (原版 kMaxSignificantDigits = ${kMaxSignificantDigits})
pub const MAX_SIGNIFICANT_DIGITS: i32 = ${kMaxSignificantDigits};

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
  pub const YEAR_DIGITS: usize = ${es5Bounds.yearDigits};
  /// ±yyyyyy 位数, 值域 -999999..+999999; -000000 非法 (年零须为正)
  pub const SIGNED_YEAR_DIGITS: usize = ${es5Bounds.signYearDigits};
  /// MM 位数, 值域 01..12
  pub const MONTH_DIGITS: usize = ${es5Bounds.monthDigits};
  pub const MONTH_MIN: i32 = ${es5Bounds.monthMin};
  pub const MONTH_MAX: i32 = ${es5Bounds.monthMax};
  /// DD 位数, 值域 01..31, 缺省 01
  pub const DAY_DIGITS: usize = ${es5Bounds.dayDigits};
  pub const DAY_MIN: i32 = ${es5Bounds.dayMin};
  pub const DAY_MAX: i32 = ${es5Bounds.dayMax};
  /// HH 位数, 一般 00..23; 24 仅当 mm/ss/sss 全零 (当日末尾午夜)
  pub const HOUR_DIGITS: usize = ${es5Bounds.hourDigits};
  pub const MAX_HOUR: i32 = ${es5Bounds.maxHour24};
  /// mm 位数, 00..59, 缺省 00
  pub const MINUTE_DIGITS: usize = ${es5Bounds.minuteDigits};
  pub const MAX_MINUTE: i32 = ${es5Bounds.maxMinute};
  /// ss 位数, 00..59, 缺省 00
  pub const SECOND_DIGITS: usize = ${es5Bounds.secondDigits};
  pub const MAX_SECOND: i32 = ${es5Bounds.maxSecond};
  /// sss 位数为规约值 3, 实际允许 ≥1 位
  pub const MILLISECOND_DIGITS: usize = ${es5Bounds.msDigits};
  pub const MAX_MILLISECOND: i32 = ${es5Bounds.maxMillisecond};
  /// 时区: Z 缺省; hh:mm 标准式 hh ∈ 00..23
  pub const TZ_HOUR_DIGITS: usize = ${es5Bounds.tzHourDigits2};
  /// 时区扩展式 hhmm
  pub const TZ_HHMM_DIGITS: usize = ${es5Bounds.tzHourDigits4};
}

/// DateStringTokenizer::Scan 决策序 (原版实现机械登记):
/// 1) 输入尽 → EndOfInput
/// 2) ASCII 数字 → 读无符号数 (截 ${kMaxSignificantDigits} 位) → Number(n, len)
/// 3) 符号按序 ${scanSymbols.map((s) => `'${s}'`).join(" ")} → Symbol(ch)
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

  /// 毫秒读取骨架 (原版 ReadMilliseconds): token 为数字 → 按 10^补齐 3 位, 否则 -1
  pub fn read_milliseconds(token: &DateToken) -> i32 {
    let _ = token;
    unimplemented!("骨架")
  }
}

/// InputReader 跳读骨架
pub mod input_reader_skel {
  /// SkipWhiteSpace: 当前为空白则前进并返回 true
  pub fn skip_white_space() -> bool {
    unimplemented!("骨架")
  }
  /// SkipParentheses: ch=='(' 时按配平跳过整个括号段, 恒返回 true (自 '(' 起)
  pub fn skip_parentheses() -> bool {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-date", "dateparser/inl_generated.rs", rust);
console.log(
  `written ${p}; 符号表=${scanSymbols.length}, ES5界=${Object.keys(es5Bounds).length}, 遗留分支=13`,
);
