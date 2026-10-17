// 由 tool/src/builtins/builtins-misc.js 生成; 来源 v8/src/builtins
// 内建名表 + 核心调度元数据 (165 内建)

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// 内建名表 (共 165 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  CollatorConstructor,
  CollatorInternalCompare,
  CollatorPrototypeCompare,
  CollatorPrototypeResolvedOptions,
  CollatorSupportedLocalesOf,
  ConsoleContext,
  ConsoleTime,
  ConsoleTimeEnd,
  ConsoleTimeLog,
  ConsoleTimeStamp,
  DateConstructor,
  DateNow,
  DateParse,
  DatePrototypeGetDate,
  DatePrototypeGetDay,
  DatePrototypeGetFullYear,
  DatePrototypeGetHours,
  DatePrototypeGetMilliseconds,
  DatePrototypeGetMinutes,
  DatePrototypeGetMonth,
  DatePrototypeGetSeconds,
  DatePrototypeGetTime,
  DatePrototypeGetTimezoneOffset,
  DatePrototypeGetUTCDate,
  DatePrototypeGetUTCDay,
  DatePrototypeGetUTCFullYear,
  DatePrototypeGetUTCHours,
  DatePrototypeGetUTCMilliseconds,
  DatePrototypeGetUTCMinutes,
  DatePrototypeGetUTCMonth,
  DatePrototypeGetUTCSeconds,
  DatePrototypeGetYear,
  DatePrototypeSetDate,
  DatePrototypeSetFullYear,
  DatePrototypeSetHours,
  DatePrototypeSetMilliseconds,
  DatePrototypeSetMinutes,
  DatePrototypeSetMonth,
  DatePrototypeSetSeconds,
  DatePrototypeSetTime,
  DatePrototypeSetUTCDate,
  DatePrototypeSetUTCFullYear,
  DatePrototypeSetUTCHours,
  DatePrototypeSetUTCMilliseconds,
  DatePrototypeSetUTCMinutes,
  DatePrototypeSetUTCMonth,
  DatePrototypeSetUTCSeconds,
  DatePrototypeSetYear,
  DatePrototypeToDateString,
  DatePrototypeToISOString,
  DatePrototypeToJson,
  DatePrototypeToLocaleDateString,
  DatePrototypeToLocaleString,
  DatePrototypeToLocaleTimeString,
  DatePrototypeToPrimitive,
  DatePrototypeToString,
  DatePrototypeToTemporalInstant,
  DatePrototypeToTimeString,
  DatePrototypeToUTCString,
  DatePrototypeValueOf,
  DateTimeFormatConstructor,
  DateTimeFormatInternalFormat,
  DateTimeFormatPrototypeFormat,
  DateTimeFormatPrototypeFormatRange,
  DateTimeFormatPrototypeFormatRangeToParts,
  DateTimeFormatPrototypeFormatToParts,
  DateTimeFormatPrototypeResolvedOptions,
  DateTimeFormatSupportedLocalesOf,
  DateUTC,
  DisplayNamesConstructor,
  DisplayNamesPrototypeOf,
  DisplayNamesPrototypeResolvedOptions,
  DisplayNamesSupportedLocalesOf,
  DurationFormatConstructor,
  DurationFormatPrototypeFormat,
  DurationFormatPrototypeFormatToParts,
  DurationFormatPrototypeResolvedOptions,
  DurationFormatSupportedLocalesOf,
  IntlGetCanonicalLocales,
  IntlSupportedValuesOf,
  JsonIsRawJson,
  JsonParse,
  JsonRawJson,
  JsonStringify,
  ListFormatConstructor,
  ListFormatPrototypeFormat,
  ListFormatPrototypeFormatToParts,
  ListFormatPrototypeResolvedOptions,
  ListFormatSupportedLocalesOf,
  LocaleConstructor,
  LocalePrototypeBaseName,
  LocalePrototypeCalendar,
  LocalePrototypeCaseFirst,
  LocalePrototypeCollation,
  LocalePrototypeFirstDayOfWeek,
  LocalePrototypeGetCalendars,
  LocalePrototypeGetCollations,
  LocalePrototypeGetHourCycles,
  LocalePrototypeGetNumberingSystems,
  LocalePrototypeGetTextInfo,
  LocalePrototypeGetTimeZones,
  LocalePrototypeGetWeekInfo,
  LocalePrototypeHourCycle,
  LocalePrototypeLanguage,
  LocalePrototypeMaximize,
  LocalePrototypeMinimize,
  LocalePrototypeNumberingSystem,
  LocalePrototypeNumeric,
  LocalePrototypeRegion,
  LocalePrototypeScript,
  LocalePrototypeToString,
  LocalePrototypeVariants,
  NumberFormatConstructor,
  NumberFormatInternalFormatNumber,
  NumberFormatPrototypeFormatNumber,
  NumberFormatPrototypeFormatRange,
  NumberFormatPrototypeFormatRangeToParts,
  NumberFormatPrototypeFormatToParts,
  NumberFormatPrototypeResolvedOptions,
  NumberFormatSupportedLocalesOf,
  PluralRulesConstructor,
  PluralRulesPrototypeResolvedOptions,
  PluralRulesPrototypeSelect,
  PluralRulesPrototypeSelectRange,
  PluralRulesSupportedLocalesOf,
  RelativeTimeFormatConstructor,
  RelativeTimeFormatPrototypeFormat,
  RelativeTimeFormatPrototypeFormatToParts,
  RelativeTimeFormatPrototypeResolvedOptions,
  RelativeTimeFormatSupportedLocalesOf,
  SegmentIteratorPrototypeNext,
  SegmenterConstructor,
  SegmenterPrototypeResolvedOptions,
  SegmenterPrototypeSegment,
  SegmenterSupportedLocalesOf,
  SegmentsPrototypeContaining,
  SegmentsPrototypeIterator,
  StringPrototypeLocaleCompareIntl,
  StringPrototypeNormalizeIntl,
  StringPrototypeToLocaleLowerCase,
  StringPrototypeToLocaleUpperCase,
  StringPrototypeToLowerCaseIntl,
  StringPrototypeToUpperCaseIntl,
  StringToLowerCaseIntl,
  TemporalDurationConstructor,
  TemporalPlainDateConstructor,
  TemporalPlainDateTimeConstructor,
  TemporalPlainMonthDayConstructor,
  TemporalPlainTimeConstructor,
  TemporalPlainYearMonthConstructor,
  TemporalZonedDateTimeConstructor,
  V8BreakIteratorConstructor,
  V8BreakIteratorInternalAdoptText,
  V8BreakIteratorInternalBreakType,
  V8BreakIteratorInternalCurrent,
  V8BreakIteratorInternalFirst,
  V8BreakIteratorInternalNext,
  V8BreakIteratorPrototypeAdoptText,
  V8BreakIteratorPrototypeBreakType,
  V8BreakIteratorPrototypeCurrent,
  V8BreakIteratorPrototypeFirst,
  V8BreakIteratorPrototypeNext,
  V8BreakIteratorPrototypeResolvedOptions,
  V8BreakIteratorSupportedLocalesOf,
  WasmStringToLowerCaseIntl,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::CollatorConstructor => "CollatorConstructor",
      Self::CollatorInternalCompare => "CollatorInternalCompare",
      Self::CollatorPrototypeCompare => "CollatorPrototypeCompare",
      Self::CollatorPrototypeResolvedOptions => "CollatorPrototypeResolvedOptions",
      Self::CollatorSupportedLocalesOf => "CollatorSupportedLocalesOf",
      Self::ConsoleContext => "ConsoleContext",
      Self::ConsoleTime => "ConsoleTime",
      Self::ConsoleTimeEnd => "ConsoleTimeEnd",
      Self::ConsoleTimeLog => "ConsoleTimeLog",
      Self::ConsoleTimeStamp => "ConsoleTimeStamp",
      Self::DateConstructor => "DateConstructor",
      Self::DateNow => "DateNow",
      Self::DateParse => "DateParse",
      Self::DatePrototypeGetDate => "DatePrototypeGetDate",
      Self::DatePrototypeGetDay => "DatePrototypeGetDay",
      Self::DatePrototypeGetFullYear => "DatePrototypeGetFullYear",
      Self::DatePrototypeGetHours => "DatePrototypeGetHours",
      Self::DatePrototypeGetMilliseconds => "DatePrototypeGetMilliseconds",
      Self::DatePrototypeGetMinutes => "DatePrototypeGetMinutes",
      Self::DatePrototypeGetMonth => "DatePrototypeGetMonth",
      Self::DatePrototypeGetSeconds => "DatePrototypeGetSeconds",
      Self::DatePrototypeGetTime => "DatePrototypeGetTime",
      Self::DatePrototypeGetTimezoneOffset => "DatePrototypeGetTimezoneOffset",
      Self::DatePrototypeGetUTCDate => "DatePrototypeGetUTCDate",
      Self::DatePrototypeGetUTCDay => "DatePrototypeGetUTCDay",
      Self::DatePrototypeGetUTCFullYear => "DatePrototypeGetUTCFullYear",
      Self::DatePrototypeGetUTCHours => "DatePrototypeGetUTCHours",
      Self::DatePrototypeGetUTCMilliseconds => "DatePrototypeGetUTCMilliseconds",
      Self::DatePrototypeGetUTCMinutes => "DatePrototypeGetUTCMinutes",
      Self::DatePrototypeGetUTCMonth => "DatePrototypeGetUTCMonth",
      Self::DatePrototypeGetUTCSeconds => "DatePrototypeGetUTCSeconds",
      Self::DatePrototypeGetYear => "DatePrototypeGetYear",
      Self::DatePrototypeSetDate => "DatePrototypeSetDate",
      Self::DatePrototypeSetFullYear => "DatePrototypeSetFullYear",
      Self::DatePrototypeSetHours => "DatePrototypeSetHours",
      Self::DatePrototypeSetMilliseconds => "DatePrototypeSetMilliseconds",
      Self::DatePrototypeSetMinutes => "DatePrototypeSetMinutes",
      Self::DatePrototypeSetMonth => "DatePrototypeSetMonth",
      Self::DatePrototypeSetSeconds => "DatePrototypeSetSeconds",
      Self::DatePrototypeSetTime => "DatePrototypeSetTime",
      Self::DatePrototypeSetUTCDate => "DatePrototypeSetUTCDate",
      Self::DatePrototypeSetUTCFullYear => "DatePrototypeSetUTCFullYear",
      Self::DatePrototypeSetUTCHours => "DatePrototypeSetUTCHours",
      Self::DatePrototypeSetUTCMilliseconds => "DatePrototypeSetUTCMilliseconds",
      Self::DatePrototypeSetUTCMinutes => "DatePrototypeSetUTCMinutes",
      Self::DatePrototypeSetUTCMonth => "DatePrototypeSetUTCMonth",
      Self::DatePrototypeSetUTCSeconds => "DatePrototypeSetUTCSeconds",
      Self::DatePrototypeSetYear => "DatePrototypeSetYear",
      Self::DatePrototypeToDateString => "DatePrototypeToDateString",
      Self::DatePrototypeToISOString => "DatePrototypeToISOString",
      Self::DatePrototypeToJson => "DatePrototypeToJson",
      Self::DatePrototypeToLocaleDateString => "DatePrototypeToLocaleDateString",
      Self::DatePrototypeToLocaleString => "DatePrototypeToLocaleString",
      Self::DatePrototypeToLocaleTimeString => "DatePrototypeToLocaleTimeString",
      Self::DatePrototypeToPrimitive => "DatePrototypeToPrimitive",
      Self::DatePrototypeToString => "DatePrototypeToString",
      Self::DatePrototypeToTemporalInstant => "DatePrototypeToTemporalInstant",
      Self::DatePrototypeToTimeString => "DatePrototypeToTimeString",
      Self::DatePrototypeToUTCString => "DatePrototypeToUTCString",
      Self::DatePrototypeValueOf => "DatePrototypeValueOf",
      Self::DateTimeFormatConstructor => "DateTimeFormatConstructor",
      Self::DateTimeFormatInternalFormat => "DateTimeFormatInternalFormat",
      Self::DateTimeFormatPrototypeFormat => "DateTimeFormatPrototypeFormat",
      Self::DateTimeFormatPrototypeFormatRange => "DateTimeFormatPrototypeFormatRange",
      Self::DateTimeFormatPrototypeFormatRangeToParts => {
        "DateTimeFormatPrototypeFormatRangeToParts"
      }
      Self::DateTimeFormatPrototypeFormatToParts => "DateTimeFormatPrototypeFormatToParts",
      Self::DateTimeFormatPrototypeResolvedOptions => "DateTimeFormatPrototypeResolvedOptions",
      Self::DateTimeFormatSupportedLocalesOf => "DateTimeFormatSupportedLocalesOf",
      Self::DateUTC => "DateUTC",
      Self::DisplayNamesConstructor => "DisplayNamesConstructor",
      Self::DisplayNamesPrototypeOf => "DisplayNamesPrototypeOf",
      Self::DisplayNamesPrototypeResolvedOptions => "DisplayNamesPrototypeResolvedOptions",
      Self::DisplayNamesSupportedLocalesOf => "DisplayNamesSupportedLocalesOf",
      Self::DurationFormatConstructor => "DurationFormatConstructor",
      Self::DurationFormatPrototypeFormat => "DurationFormatPrototypeFormat",
      Self::DurationFormatPrototypeFormatToParts => "DurationFormatPrototypeFormatToParts",
      Self::DurationFormatPrototypeResolvedOptions => "DurationFormatPrototypeResolvedOptions",
      Self::DurationFormatSupportedLocalesOf => "DurationFormatSupportedLocalesOf",
      Self::IntlGetCanonicalLocales => "IntlGetCanonicalLocales",
      Self::IntlSupportedValuesOf => "IntlSupportedValuesOf",
      Self::JsonIsRawJson => "JsonIsRawJson",
      Self::JsonParse => "JsonParse",
      Self::JsonRawJson => "JsonRawJson",
      Self::JsonStringify => "JsonStringify",
      Self::ListFormatConstructor => "ListFormatConstructor",
      Self::ListFormatPrototypeFormat => "ListFormatPrototypeFormat",
      Self::ListFormatPrototypeFormatToParts => "ListFormatPrototypeFormatToParts",
      Self::ListFormatPrototypeResolvedOptions => "ListFormatPrototypeResolvedOptions",
      Self::ListFormatSupportedLocalesOf => "ListFormatSupportedLocalesOf",
      Self::LocaleConstructor => "LocaleConstructor",
      Self::LocalePrototypeBaseName => "LocalePrototypeBaseName",
      Self::LocalePrototypeCalendar => "LocalePrototypeCalendar",
      Self::LocalePrototypeCaseFirst => "LocalePrototypeCaseFirst",
      Self::LocalePrototypeCollation => "LocalePrototypeCollation",
      Self::LocalePrototypeFirstDayOfWeek => "LocalePrototypeFirstDayOfWeek",
      Self::LocalePrototypeGetCalendars => "LocalePrototypeGetCalendars",
      Self::LocalePrototypeGetCollations => "LocalePrototypeGetCollations",
      Self::LocalePrototypeGetHourCycles => "LocalePrototypeGetHourCycles",
      Self::LocalePrototypeGetNumberingSystems => "LocalePrototypeGetNumberingSystems",
      Self::LocalePrototypeGetTextInfo => "LocalePrototypeGetTextInfo",
      Self::LocalePrototypeGetTimeZones => "LocalePrototypeGetTimeZones",
      Self::LocalePrototypeGetWeekInfo => "LocalePrototypeGetWeekInfo",
      Self::LocalePrototypeHourCycle => "LocalePrototypeHourCycle",
      Self::LocalePrototypeLanguage => "LocalePrototypeLanguage",
      Self::LocalePrototypeMaximize => "LocalePrototypeMaximize",
      Self::LocalePrototypeMinimize => "LocalePrototypeMinimize",
      Self::LocalePrototypeNumberingSystem => "LocalePrototypeNumberingSystem",
      Self::LocalePrototypeNumeric => "LocalePrototypeNumeric",
      Self::LocalePrototypeRegion => "LocalePrototypeRegion",
      Self::LocalePrototypeScript => "LocalePrototypeScript",
      Self::LocalePrototypeToString => "LocalePrototypeToString",
      Self::LocalePrototypeVariants => "LocalePrototypeVariants",
      Self::NumberFormatConstructor => "NumberFormatConstructor",
      Self::NumberFormatInternalFormatNumber => "NumberFormatInternalFormatNumber",
      Self::NumberFormatPrototypeFormatNumber => "NumberFormatPrototypeFormatNumber",
      Self::NumberFormatPrototypeFormatRange => "NumberFormatPrototypeFormatRange",
      Self::NumberFormatPrototypeFormatRangeToParts => "NumberFormatPrototypeFormatRangeToParts",
      Self::NumberFormatPrototypeFormatToParts => "NumberFormatPrototypeFormatToParts",
      Self::NumberFormatPrototypeResolvedOptions => "NumberFormatPrototypeResolvedOptions",
      Self::NumberFormatSupportedLocalesOf => "NumberFormatSupportedLocalesOf",
      Self::PluralRulesConstructor => "PluralRulesConstructor",
      Self::PluralRulesPrototypeResolvedOptions => "PluralRulesPrototypeResolvedOptions",
      Self::PluralRulesPrototypeSelect => "PluralRulesPrototypeSelect",
      Self::PluralRulesPrototypeSelectRange => "PluralRulesPrototypeSelectRange",
      Self::PluralRulesSupportedLocalesOf => "PluralRulesSupportedLocalesOf",
      Self::RelativeTimeFormatConstructor => "RelativeTimeFormatConstructor",
      Self::RelativeTimeFormatPrototypeFormat => "RelativeTimeFormatPrototypeFormat",
      Self::RelativeTimeFormatPrototypeFormatToParts => "RelativeTimeFormatPrototypeFormatToParts",
      Self::RelativeTimeFormatPrototypeResolvedOptions => {
        "RelativeTimeFormatPrototypeResolvedOptions"
      }
      Self::RelativeTimeFormatSupportedLocalesOf => "RelativeTimeFormatSupportedLocalesOf",
      Self::SegmentIteratorPrototypeNext => "SegmentIteratorPrototypeNext",
      Self::SegmenterConstructor => "SegmenterConstructor",
      Self::SegmenterPrototypeResolvedOptions => "SegmenterPrototypeResolvedOptions",
      Self::SegmenterPrototypeSegment => "SegmenterPrototypeSegment",
      Self::SegmenterSupportedLocalesOf => "SegmenterSupportedLocalesOf",
      Self::SegmentsPrototypeContaining => "SegmentsPrototypeContaining",
      Self::SegmentsPrototypeIterator => "SegmentsPrototypeIterator",
      Self::StringPrototypeLocaleCompareIntl => "StringPrototypeLocaleCompareIntl",
      Self::StringPrototypeNormalizeIntl => "StringPrototypeNormalizeIntl",
      Self::StringPrototypeToLocaleLowerCase => "StringPrototypeToLocaleLowerCase",
      Self::StringPrototypeToLocaleUpperCase => "StringPrototypeToLocaleUpperCase",
      Self::StringPrototypeToLowerCaseIntl => "StringPrototypeToLowerCaseIntl",
      Self::StringPrototypeToUpperCaseIntl => "StringPrototypeToUpperCaseIntl",
      Self::StringToLowerCaseIntl => "StringToLowerCaseIntl",
      Self::TemporalDurationConstructor => "TemporalDurationConstructor",
      Self::TemporalPlainDateConstructor => "TemporalPlainDateConstructor",
      Self::TemporalPlainDateTimeConstructor => "TemporalPlainDateTimeConstructor",
      Self::TemporalPlainMonthDayConstructor => "TemporalPlainMonthDayConstructor",
      Self::TemporalPlainTimeConstructor => "TemporalPlainTimeConstructor",
      Self::TemporalPlainYearMonthConstructor => "TemporalPlainYearMonthConstructor",
      Self::TemporalZonedDateTimeConstructor => "TemporalZonedDateTimeConstructor",
      Self::V8BreakIteratorConstructor => "V8BreakIteratorConstructor",
      Self::V8BreakIteratorInternalAdoptText => "V8BreakIteratorInternalAdoptText",
      Self::V8BreakIteratorInternalBreakType => "V8BreakIteratorInternalBreakType",
      Self::V8BreakIteratorInternalCurrent => "V8BreakIteratorInternalCurrent",
      Self::V8BreakIteratorInternalFirst => "V8BreakIteratorInternalFirst",
      Self::V8BreakIteratorInternalNext => "V8BreakIteratorInternalNext",
      Self::V8BreakIteratorPrototypeAdoptText => "V8BreakIteratorPrototypeAdoptText",
      Self::V8BreakIteratorPrototypeBreakType => "V8BreakIteratorPrototypeBreakType",
      Self::V8BreakIteratorPrototypeCurrent => "V8BreakIteratorPrototypeCurrent",
      Self::V8BreakIteratorPrototypeFirst => "V8BreakIteratorPrototypeFirst",
      Self::V8BreakIteratorPrototypeNext => "V8BreakIteratorPrototypeNext",
      Self::V8BreakIteratorPrototypeResolvedOptions => "V8BreakIteratorPrototypeResolvedOptions",
      Self::V8BreakIteratorSupportedLocalesOf => "V8BreakIteratorSupportedLocalesOf",
      Self::WasmStringToLowerCaseIntl => "WasmStringToLowerCaseIntl",
    }
  }

  /// 关联至全局注册表 `BUILTIN_TABLE` 获取全局内建 ID
  #[inline]
  pub fn global_id(&self) -> Option<usize> {
    find_builtin(self.name())
  }

  /// 获取对应的接口描述符符号
  #[inline]
  pub fn descriptor_name(&self) -> String {
    descriptor_name(self.name())
  }

  /// 是否为 Date 日期内建
  #[inline]
  pub const fn is_date(&self) -> bool {
    matches!(
      self,
      Self::DateConstructor
        | Self::DateNow
        | Self::DateParse
        | Self::DatePrototypeGetDate
        | Self::DatePrototypeGetDay
        | Self::DatePrototypeGetFullYear
        | Self::DatePrototypeGetHours
        | Self::DatePrototypeGetMilliseconds
        | Self::DatePrototypeGetMinutes
        | Self::DatePrototypeGetMonth
        | Self::DatePrototypeGetSeconds
        | Self::DatePrototypeGetTime
        | Self::DatePrototypeGetTimezoneOffset
        | Self::DatePrototypeGetUTCDate
        | Self::DatePrototypeGetUTCDay
        | Self::DatePrototypeGetUTCFullYear
        | Self::DatePrototypeGetUTCHours
        | Self::DatePrototypeGetUTCMilliseconds
        | Self::DatePrototypeGetUTCMinutes
        | Self::DatePrototypeGetUTCMonth
        | Self::DatePrototypeGetUTCSeconds
        | Self::DatePrototypeGetYear
        | Self::DatePrototypeSetDate
        | Self::DatePrototypeSetFullYear
        | Self::DatePrototypeSetHours
        | Self::DatePrototypeSetMilliseconds
        | Self::DatePrototypeSetMinutes
        | Self::DatePrototypeSetMonth
        | Self::DatePrototypeSetSeconds
        | Self::DatePrototypeSetTime
        | Self::DatePrototypeSetUTCDate
        | Self::DatePrototypeSetUTCFullYear
        | Self::DatePrototypeSetUTCHours
        | Self::DatePrototypeSetUTCMilliseconds
        | Self::DatePrototypeSetUTCMinutes
        | Self::DatePrototypeSetUTCMonth
        | Self::DatePrototypeSetUTCSeconds
        | Self::DatePrototypeSetYear
        | Self::DatePrototypeToDateString
        | Self::DatePrototypeToISOString
        | Self::DatePrototypeToJson
        | Self::DatePrototypeToLocaleDateString
        | Self::DatePrototypeToLocaleString
        | Self::DatePrototypeToLocaleTimeString
        | Self::DatePrototypeToPrimitive
        | Self::DatePrototypeToString
        | Self::DatePrototypeToTemporalInstant
        | Self::DatePrototypeToTimeString
        | Self::DatePrototypeToUTCString
        | Self::DatePrototypeValueOf
        | Self::DateUTC
    )
  }

  /// 是否为 Intl 国际化内建
  #[inline]
  pub const fn is_intl(&self) -> bool {
    matches!(
      self,
      Self::CollatorConstructor
        | Self::CollatorInternalCompare
        | Self::CollatorPrototypeCompare
        | Self::CollatorPrototypeResolvedOptions
        | Self::CollatorSupportedLocalesOf
        | Self::DateTimeFormatConstructor
        | Self::DateTimeFormatInternalFormat
        | Self::DateTimeFormatPrototypeFormat
        | Self::DateTimeFormatPrototypeFormatRange
        | Self::DateTimeFormatPrototypeFormatRangeToParts
        | Self::DateTimeFormatPrototypeFormatToParts
        | Self::DateTimeFormatPrototypeResolvedOptions
        | Self::DateTimeFormatSupportedLocalesOf
        | Self::DisplayNamesConstructor
        | Self::DisplayNamesPrototypeOf
        | Self::DisplayNamesPrototypeResolvedOptions
        | Self::DisplayNamesSupportedLocalesOf
        | Self::DurationFormatConstructor
        | Self::DurationFormatPrototypeFormat
        | Self::DurationFormatPrototypeFormatToParts
        | Self::DurationFormatPrototypeResolvedOptions
        | Self::DurationFormatSupportedLocalesOf
        | Self::IntlGetCanonicalLocales
        | Self::IntlSupportedValuesOf
        | Self::ListFormatConstructor
        | Self::ListFormatPrototypeFormat
        | Self::ListFormatPrototypeFormatToParts
        | Self::ListFormatPrototypeResolvedOptions
        | Self::ListFormatSupportedLocalesOf
        | Self::LocaleConstructor
        | Self::LocalePrototypeBaseName
        | Self::LocalePrototypeCalendar
        | Self::LocalePrototypeCaseFirst
        | Self::LocalePrototypeCollation
        | Self::LocalePrototypeFirstDayOfWeek
        | Self::LocalePrototypeGetCalendars
        | Self::LocalePrototypeGetCollations
        | Self::LocalePrototypeGetHourCycles
        | Self::LocalePrototypeGetNumberingSystems
        | Self::LocalePrototypeGetTextInfo
        | Self::LocalePrototypeGetTimeZones
        | Self::LocalePrototypeGetWeekInfo
        | Self::LocalePrototypeHourCycle
        | Self::LocalePrototypeLanguage
        | Self::LocalePrototypeMaximize
        | Self::LocalePrototypeMinimize
        | Self::LocalePrototypeNumberingSystem
        | Self::LocalePrototypeNumeric
        | Self::LocalePrototypeRegion
        | Self::LocalePrototypeScript
        | Self::LocalePrototypeToString
        | Self::LocalePrototypeVariants
        | Self::NumberFormatConstructor
        | Self::NumberFormatInternalFormatNumber
        | Self::NumberFormatPrototypeFormatNumber
        | Self::NumberFormatPrototypeFormatRange
        | Self::NumberFormatPrototypeFormatRangeToParts
        | Self::NumberFormatPrototypeFormatToParts
        | Self::NumberFormatPrototypeResolvedOptions
        | Self::NumberFormatSupportedLocalesOf
        | Self::PluralRulesConstructor
        | Self::PluralRulesPrototypeResolvedOptions
        | Self::PluralRulesPrototypeSelect
        | Self::PluralRulesPrototypeSelectRange
        | Self::PluralRulesSupportedLocalesOf
        | Self::RelativeTimeFormatConstructor
        | Self::RelativeTimeFormatPrototypeFormat
        | Self::RelativeTimeFormatPrototypeFormatToParts
        | Self::RelativeTimeFormatPrototypeResolvedOptions
        | Self::RelativeTimeFormatSupportedLocalesOf
        | Self::SegmentIteratorPrototypeNext
        | Self::SegmenterConstructor
        | Self::SegmenterPrototypeResolvedOptions
        | Self::SegmenterPrototypeSegment
        | Self::SegmenterSupportedLocalesOf
        | Self::SegmentsPrototypeContaining
        | Self::SegmentsPrototypeIterator
        | Self::StringPrototypeLocaleCompareIntl
        | Self::StringPrototypeNormalizeIntl
        | Self::StringPrototypeToLocaleLowerCase
        | Self::StringPrototypeToLocaleUpperCase
        | Self::StringPrototypeToLowerCaseIntl
        | Self::StringPrototypeToUpperCaseIntl
        | Self::StringToLowerCaseIntl
        | Self::V8BreakIteratorConstructor
        | Self::V8BreakIteratorInternalAdoptText
        | Self::V8BreakIteratorInternalBreakType
        | Self::V8BreakIteratorInternalCurrent
        | Self::V8BreakIteratorInternalFirst
        | Self::V8BreakIteratorInternalNext
        | Self::V8BreakIteratorPrototypeAdoptText
        | Self::V8BreakIteratorPrototypeBreakType
        | Self::V8BreakIteratorPrototypeCurrent
        | Self::V8BreakIteratorPrototypeFirst
        | Self::V8BreakIteratorPrototypeNext
        | Self::V8BreakIteratorPrototypeResolvedOptions
        | Self::V8BreakIteratorSupportedLocalesOf
        | Self::WasmStringToLowerCaseIntl
    )
  }

  /// 是否为 Temporal 新一代时间内建
  #[inline]
  pub const fn is_temporal(&self) -> bool {
    matches!(
      self,
      Self::DatePrototypeToTemporalInstant
        | Self::TemporalDurationConstructor
        | Self::TemporalPlainDateConstructor
        | Self::TemporalPlainDateTimeConstructor
        | Self::TemporalPlainMonthDayConstructor
        | Self::TemporalPlainTimeConstructor
        | Self::TemporalPlainYearMonthConstructor
        | Self::TemporalZonedDateTimeConstructor
    )
  }

  /// 是否为 JSON 内建
  #[inline]
  pub const fn is_json(&self) -> bool {
    matches!(
      self,
      Self::DatePrototypeToJson
        | Self::JsonIsRawJson
        | Self::JsonParse
        | Self::JsonRawJson
        | Self::JsonStringify
    )
  }

  /// 是否为 Console 控制台内建
  #[inline]
  pub const fn is_console(&self) -> bool {
    matches!(
      self,
      Self::ConsoleContext
        | Self::ConsoleTime
        | Self::ConsoleTimeEnd
        | Self::ConsoleTimeLog
        | Self::ConsoleTimeStamp
    )
  }
}

// 统一提供 165 项零成本内建工厂函数
pub const fn collator_constructor() -> Builtin {
  Builtin::CollatorConstructor
}

pub const fn collator_internal_compare() -> Builtin {
  Builtin::CollatorInternalCompare
}

pub const fn collator_prototype_compare() -> Builtin {
  Builtin::CollatorPrototypeCompare
}

pub const fn collator_prototype_resolved_options() -> Builtin {
  Builtin::CollatorPrototypeResolvedOptions
}

pub const fn collator_supported_locales_of() -> Builtin {
  Builtin::CollatorSupportedLocalesOf
}

pub const fn console_context() -> Builtin {
  Builtin::ConsoleContext
}

pub const fn console_time() -> Builtin {
  Builtin::ConsoleTime
}

pub const fn console_time_end() -> Builtin {
  Builtin::ConsoleTimeEnd
}

pub const fn console_time_log() -> Builtin {
  Builtin::ConsoleTimeLog
}

pub const fn console_time_stamp() -> Builtin {
  Builtin::ConsoleTimeStamp
}

pub const fn date_constructor() -> Builtin {
  Builtin::DateConstructor
}

pub const fn date_now() -> Builtin {
  Builtin::DateNow
}

pub const fn date_parse() -> Builtin {
  Builtin::DateParse
}

pub const fn date_prototype_get_date() -> Builtin {
  Builtin::DatePrototypeGetDate
}

pub const fn date_prototype_get_day() -> Builtin {
  Builtin::DatePrototypeGetDay
}

pub const fn date_prototype_get_full_year() -> Builtin {
  Builtin::DatePrototypeGetFullYear
}

pub const fn date_prototype_get_hours() -> Builtin {
  Builtin::DatePrototypeGetHours
}

pub const fn date_prototype_get_milliseconds() -> Builtin {
  Builtin::DatePrototypeGetMilliseconds
}

pub const fn date_prototype_get_minutes() -> Builtin {
  Builtin::DatePrototypeGetMinutes
}

pub const fn date_prototype_get_month() -> Builtin {
  Builtin::DatePrototypeGetMonth
}

pub const fn date_prototype_get_seconds() -> Builtin {
  Builtin::DatePrototypeGetSeconds
}

pub const fn date_prototype_get_time() -> Builtin {
  Builtin::DatePrototypeGetTime
}

pub const fn date_prototype_get_timezone_offset() -> Builtin {
  Builtin::DatePrototypeGetTimezoneOffset
}

pub const fn date_prototype_get_utc_date() -> Builtin {
  Builtin::DatePrototypeGetUTCDate
}

pub const fn date_prototype_get_utc_day() -> Builtin {
  Builtin::DatePrototypeGetUTCDay
}

pub const fn date_prototype_get_utc_full_year() -> Builtin {
  Builtin::DatePrototypeGetUTCFullYear
}

pub const fn date_prototype_get_utc_hours() -> Builtin {
  Builtin::DatePrototypeGetUTCHours
}

pub const fn date_prototype_get_utc_milliseconds() -> Builtin {
  Builtin::DatePrototypeGetUTCMilliseconds
}

pub const fn date_prototype_get_utc_minutes() -> Builtin {
  Builtin::DatePrototypeGetUTCMinutes
}

pub const fn date_prototype_get_utc_month() -> Builtin {
  Builtin::DatePrototypeGetUTCMonth
}

pub const fn date_prototype_get_utc_seconds() -> Builtin {
  Builtin::DatePrototypeGetUTCSeconds
}

pub const fn date_prototype_get_year() -> Builtin {
  Builtin::DatePrototypeGetYear
}

pub const fn date_prototype_set_date() -> Builtin {
  Builtin::DatePrototypeSetDate
}

pub const fn date_prototype_set_full_year() -> Builtin {
  Builtin::DatePrototypeSetFullYear
}

pub const fn date_prototype_set_hours() -> Builtin {
  Builtin::DatePrototypeSetHours
}

pub const fn date_prototype_set_milliseconds() -> Builtin {
  Builtin::DatePrototypeSetMilliseconds
}

pub const fn date_prototype_set_minutes() -> Builtin {
  Builtin::DatePrototypeSetMinutes
}

pub const fn date_prototype_set_month() -> Builtin {
  Builtin::DatePrototypeSetMonth
}

pub const fn date_prototype_set_seconds() -> Builtin {
  Builtin::DatePrototypeSetSeconds
}

pub const fn date_prototype_set_time() -> Builtin {
  Builtin::DatePrototypeSetTime
}

pub const fn date_prototype_set_utc_date() -> Builtin {
  Builtin::DatePrototypeSetUTCDate
}

pub const fn date_prototype_set_utc_full_year() -> Builtin {
  Builtin::DatePrototypeSetUTCFullYear
}

pub const fn date_prototype_set_utc_hours() -> Builtin {
  Builtin::DatePrototypeSetUTCHours
}

pub const fn date_prototype_set_utc_milliseconds() -> Builtin {
  Builtin::DatePrototypeSetUTCMilliseconds
}

pub const fn date_prototype_set_utc_minutes() -> Builtin {
  Builtin::DatePrototypeSetUTCMinutes
}

pub const fn date_prototype_set_utc_month() -> Builtin {
  Builtin::DatePrototypeSetUTCMonth
}

pub const fn date_prototype_set_utc_seconds() -> Builtin {
  Builtin::DatePrototypeSetUTCSeconds
}

pub const fn date_prototype_set_year() -> Builtin {
  Builtin::DatePrototypeSetYear
}

pub const fn date_prototype_to_date_string() -> Builtin {
  Builtin::DatePrototypeToDateString
}

pub const fn date_prototype_to_iso_string() -> Builtin {
  Builtin::DatePrototypeToISOString
}

pub const fn date_prototype_to_json() -> Builtin {
  Builtin::DatePrototypeToJson
}

pub const fn date_prototype_to_locale_date_string() -> Builtin {
  Builtin::DatePrototypeToLocaleDateString
}

pub const fn date_prototype_to_locale_string() -> Builtin {
  Builtin::DatePrototypeToLocaleString
}

pub const fn date_prototype_to_locale_time_string() -> Builtin {
  Builtin::DatePrototypeToLocaleTimeString
}

pub const fn date_prototype_to_primitive() -> Builtin {
  Builtin::DatePrototypeToPrimitive
}

pub const fn date_prototype_to_string() -> Builtin {
  Builtin::DatePrototypeToString
}

pub const fn date_prototype_to_temporal_instant() -> Builtin {
  Builtin::DatePrototypeToTemporalInstant
}

pub const fn date_prototype_to_time_string() -> Builtin {
  Builtin::DatePrototypeToTimeString
}

pub const fn date_prototype_to_utc_string() -> Builtin {
  Builtin::DatePrototypeToUTCString
}

pub const fn date_prototype_value_of() -> Builtin {
  Builtin::DatePrototypeValueOf
}

pub const fn date_time_format_constructor() -> Builtin {
  Builtin::DateTimeFormatConstructor
}

pub const fn date_time_format_internal_format() -> Builtin {
  Builtin::DateTimeFormatInternalFormat
}

pub const fn date_time_format_prototype_format() -> Builtin {
  Builtin::DateTimeFormatPrototypeFormat
}

pub const fn date_time_format_prototype_format_range() -> Builtin {
  Builtin::DateTimeFormatPrototypeFormatRange
}

pub const fn date_time_format_prototype_format_range_to_parts() -> Builtin {
  Builtin::DateTimeFormatPrototypeFormatRangeToParts
}

pub const fn date_time_format_prototype_format_to_parts() -> Builtin {
  Builtin::DateTimeFormatPrototypeFormatToParts
}

pub const fn date_time_format_prototype_resolved_options() -> Builtin {
  Builtin::DateTimeFormatPrototypeResolvedOptions
}

pub const fn date_time_format_supported_locales_of() -> Builtin {
  Builtin::DateTimeFormatSupportedLocalesOf
}

pub const fn date_utc() -> Builtin {
  Builtin::DateUTC
}

pub const fn display_names_constructor() -> Builtin {
  Builtin::DisplayNamesConstructor
}

pub const fn display_names_prototype_of() -> Builtin {
  Builtin::DisplayNamesPrototypeOf
}

pub const fn display_names_prototype_resolved_options() -> Builtin {
  Builtin::DisplayNamesPrototypeResolvedOptions
}

pub const fn display_names_supported_locales_of() -> Builtin {
  Builtin::DisplayNamesSupportedLocalesOf
}

pub const fn duration_format_constructor() -> Builtin {
  Builtin::DurationFormatConstructor
}

pub const fn duration_format_prototype_format() -> Builtin {
  Builtin::DurationFormatPrototypeFormat
}

pub const fn duration_format_prototype_format_to_parts() -> Builtin {
  Builtin::DurationFormatPrototypeFormatToParts
}

pub const fn duration_format_prototype_resolved_options() -> Builtin {
  Builtin::DurationFormatPrototypeResolvedOptions
}

pub const fn duration_format_supported_locales_of() -> Builtin {
  Builtin::DurationFormatSupportedLocalesOf
}

pub const fn intl_get_canonical_locales() -> Builtin {
  Builtin::IntlGetCanonicalLocales
}

pub const fn intl_supported_values_of() -> Builtin {
  Builtin::IntlSupportedValuesOf
}

pub const fn json_is_raw_json() -> Builtin {
  Builtin::JsonIsRawJson
}

pub const fn json_parse() -> Builtin {
  Builtin::JsonParse
}

pub const fn json_raw_json() -> Builtin {
  Builtin::JsonRawJson
}

pub const fn json_stringify() -> Builtin {
  Builtin::JsonStringify
}

pub const fn list_format_constructor() -> Builtin {
  Builtin::ListFormatConstructor
}

pub const fn list_format_prototype_format() -> Builtin {
  Builtin::ListFormatPrototypeFormat
}

pub const fn list_format_prototype_format_to_parts() -> Builtin {
  Builtin::ListFormatPrototypeFormatToParts
}

pub const fn list_format_prototype_resolved_options() -> Builtin {
  Builtin::ListFormatPrototypeResolvedOptions
}

pub const fn list_format_supported_locales_of() -> Builtin {
  Builtin::ListFormatSupportedLocalesOf
}

pub const fn locale_constructor() -> Builtin {
  Builtin::LocaleConstructor
}

pub const fn locale_prototype_base_name() -> Builtin {
  Builtin::LocalePrototypeBaseName
}

pub const fn locale_prototype_calendar() -> Builtin {
  Builtin::LocalePrototypeCalendar
}

pub const fn locale_prototype_case_first() -> Builtin {
  Builtin::LocalePrototypeCaseFirst
}

pub const fn locale_prototype_collation() -> Builtin {
  Builtin::LocalePrototypeCollation
}

pub const fn locale_prototype_first_day_of_week() -> Builtin {
  Builtin::LocalePrototypeFirstDayOfWeek
}

pub const fn locale_prototype_get_calendars() -> Builtin {
  Builtin::LocalePrototypeGetCalendars
}

pub const fn locale_prototype_get_collations() -> Builtin {
  Builtin::LocalePrototypeGetCollations
}

pub const fn locale_prototype_get_hour_cycles() -> Builtin {
  Builtin::LocalePrototypeGetHourCycles
}

pub const fn locale_prototype_get_numbering_systems() -> Builtin {
  Builtin::LocalePrototypeGetNumberingSystems
}

pub const fn locale_prototype_get_text_info() -> Builtin {
  Builtin::LocalePrototypeGetTextInfo
}

pub const fn locale_prototype_get_time_zones() -> Builtin {
  Builtin::LocalePrototypeGetTimeZones
}

pub const fn locale_prototype_get_week_info() -> Builtin {
  Builtin::LocalePrototypeGetWeekInfo
}

pub const fn locale_prototype_hour_cycle() -> Builtin {
  Builtin::LocalePrototypeHourCycle
}

pub const fn locale_prototype_language() -> Builtin {
  Builtin::LocalePrototypeLanguage
}

pub const fn locale_prototype_maximize() -> Builtin {
  Builtin::LocalePrototypeMaximize
}

pub const fn locale_prototype_minimize() -> Builtin {
  Builtin::LocalePrototypeMinimize
}

pub const fn locale_prototype_numbering_system() -> Builtin {
  Builtin::LocalePrototypeNumberingSystem
}

pub const fn locale_prototype_numeric() -> Builtin {
  Builtin::LocalePrototypeNumeric
}

pub const fn locale_prototype_region() -> Builtin {
  Builtin::LocalePrototypeRegion
}

pub const fn locale_prototype_script() -> Builtin {
  Builtin::LocalePrototypeScript
}

pub const fn locale_prototype_to_string() -> Builtin {
  Builtin::LocalePrototypeToString
}

pub const fn locale_prototype_variants() -> Builtin {
  Builtin::LocalePrototypeVariants
}

pub const fn number_format_constructor() -> Builtin {
  Builtin::NumberFormatConstructor
}

pub const fn number_format_internal_format_number() -> Builtin {
  Builtin::NumberFormatInternalFormatNumber
}

pub const fn number_format_prototype_format_number() -> Builtin {
  Builtin::NumberFormatPrototypeFormatNumber
}

pub const fn number_format_prototype_format_range() -> Builtin {
  Builtin::NumberFormatPrototypeFormatRange
}

pub const fn number_format_prototype_format_range_to_parts() -> Builtin {
  Builtin::NumberFormatPrototypeFormatRangeToParts
}

pub const fn number_format_prototype_format_to_parts() -> Builtin {
  Builtin::NumberFormatPrototypeFormatToParts
}

pub const fn number_format_prototype_resolved_options() -> Builtin {
  Builtin::NumberFormatPrototypeResolvedOptions
}

pub const fn number_format_supported_locales_of() -> Builtin {
  Builtin::NumberFormatSupportedLocalesOf
}

pub const fn plural_rules_constructor() -> Builtin {
  Builtin::PluralRulesConstructor
}

pub const fn plural_rules_prototype_resolved_options() -> Builtin {
  Builtin::PluralRulesPrototypeResolvedOptions
}

pub const fn plural_rules_prototype_select() -> Builtin {
  Builtin::PluralRulesPrototypeSelect
}

pub const fn plural_rules_prototype_select_range() -> Builtin {
  Builtin::PluralRulesPrototypeSelectRange
}

pub const fn plural_rules_supported_locales_of() -> Builtin {
  Builtin::PluralRulesSupportedLocalesOf
}

pub const fn relative_time_format_constructor() -> Builtin {
  Builtin::RelativeTimeFormatConstructor
}

pub const fn relative_time_format_prototype_format() -> Builtin {
  Builtin::RelativeTimeFormatPrototypeFormat
}

pub const fn relative_time_format_prototype_format_to_parts() -> Builtin {
  Builtin::RelativeTimeFormatPrototypeFormatToParts
}

pub const fn relative_time_format_prototype_resolved_options() -> Builtin {
  Builtin::RelativeTimeFormatPrototypeResolvedOptions
}

pub const fn relative_time_format_supported_locales_of() -> Builtin {
  Builtin::RelativeTimeFormatSupportedLocalesOf
}

pub const fn segment_iterator_prototype_next() -> Builtin {
  Builtin::SegmentIteratorPrototypeNext
}

pub const fn segmenter_constructor() -> Builtin {
  Builtin::SegmenterConstructor
}

pub const fn segmenter_prototype_resolved_options() -> Builtin {
  Builtin::SegmenterPrototypeResolvedOptions
}

pub const fn segmenter_prototype_segment() -> Builtin {
  Builtin::SegmenterPrototypeSegment
}

pub const fn segmenter_supported_locales_of() -> Builtin {
  Builtin::SegmenterSupportedLocalesOf
}

pub const fn segments_prototype_containing() -> Builtin {
  Builtin::SegmentsPrototypeContaining
}

pub const fn segments_prototype_iterator() -> Builtin {
  Builtin::SegmentsPrototypeIterator
}

pub const fn string_prototype_locale_compare_intl() -> Builtin {
  Builtin::StringPrototypeLocaleCompareIntl
}

pub const fn string_prototype_normalize_intl() -> Builtin {
  Builtin::StringPrototypeNormalizeIntl
}

pub const fn string_prototype_to_locale_lower_case() -> Builtin {
  Builtin::StringPrototypeToLocaleLowerCase
}

pub const fn string_prototype_to_locale_upper_case() -> Builtin {
  Builtin::StringPrototypeToLocaleUpperCase
}

pub const fn string_prototype_to_lower_case_intl() -> Builtin {
  Builtin::StringPrototypeToLowerCaseIntl
}

pub const fn string_prototype_to_upper_case_intl() -> Builtin {
  Builtin::StringPrototypeToUpperCaseIntl
}

pub const fn string_to_lower_case_intl() -> Builtin {
  Builtin::StringToLowerCaseIntl
}

pub const fn temporal_duration_constructor() -> Builtin {
  Builtin::TemporalDurationConstructor
}

pub const fn temporal_plain_date_constructor() -> Builtin {
  Builtin::TemporalPlainDateConstructor
}

pub const fn temporal_plain_date_time_constructor() -> Builtin {
  Builtin::TemporalPlainDateTimeConstructor
}

pub const fn temporal_plain_month_day_constructor() -> Builtin {
  Builtin::TemporalPlainMonthDayConstructor
}

pub const fn temporal_plain_time_constructor() -> Builtin {
  Builtin::TemporalPlainTimeConstructor
}

pub const fn temporal_plain_year_month_constructor() -> Builtin {
  Builtin::TemporalPlainYearMonthConstructor
}

pub const fn temporal_zoned_date_time_constructor() -> Builtin {
  Builtin::TemporalZonedDateTimeConstructor
}

pub const fn v8_break_iterator_constructor() -> Builtin {
  Builtin::V8BreakIteratorConstructor
}

pub const fn v8_break_iterator_internal_adopt_text() -> Builtin {
  Builtin::V8BreakIteratorInternalAdoptText
}

pub const fn v8_break_iterator_internal_break_type() -> Builtin {
  Builtin::V8BreakIteratorInternalBreakType
}

pub const fn v8_break_iterator_internal_current() -> Builtin {
  Builtin::V8BreakIteratorInternalCurrent
}

pub const fn v8_break_iterator_internal_first() -> Builtin {
  Builtin::V8BreakIteratorInternalFirst
}

pub const fn v8_break_iterator_internal_next() -> Builtin {
  Builtin::V8BreakIteratorInternalNext
}

pub const fn v8_break_iterator_prototype_adopt_text() -> Builtin {
  Builtin::V8BreakIteratorPrototypeAdoptText
}

pub const fn v8_break_iterator_prototype_break_type() -> Builtin {
  Builtin::V8BreakIteratorPrototypeBreakType
}

pub const fn v8_break_iterator_prototype_current() -> Builtin {
  Builtin::V8BreakIteratorPrototypeCurrent
}

pub const fn v8_break_iterator_prototype_first() -> Builtin {
  Builtin::V8BreakIteratorPrototypeFirst
}

pub const fn v8_break_iterator_prototype_next() -> Builtin {
  Builtin::V8BreakIteratorPrototypeNext
}

pub const fn v8_break_iterator_prototype_resolved_options() -> Builtin {
  Builtin::V8BreakIteratorPrototypeResolvedOptions
}

pub const fn v8_break_iterator_supported_locales_of() -> Builtin {
  Builtin::V8BreakIteratorSupportedLocalesOf
}

pub const fn wasm_string_to_lower_case_intl() -> Builtin {
  Builtin::WasmStringToLowerCaseIntl
}

/// 来自 data_time_json 相关 .tq 的声明元数据 (共 1 条)
pub const TQ_DECLS: &[(&str, &str)] = &[("javascript builtin", "FastConsoleAssert")];
