// 由 tool/src/builtins/builtins-intl.js 从 v8/src/builtins/builtins-intl.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 92 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "StringPrototypeToUpperCaseIntl",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "StringPrototypeNormalizeIntl",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "StringPrototypeLocaleCompareIntl",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "V8BreakIteratorSupportedLocalesOf",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "NumberFormatSupportedLocalesOf",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "NumberFormatPrototypeFormatToParts",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DateTimeFormatPrototypeResolvedOptions",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DateTimeFormatSupportedLocalesOf",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DateTimeFormatPrototypeFormatToParts",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DateTimeFormatPrototypeFormatRange",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DateTimeFormatPrototypeFormatRangeToParts",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DisplayNamesConstructor",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DisplayNamesPrototypeResolvedOptions",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DisplayNamesSupportedLocalesOf",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DisplayNamesPrototypeOf",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DurationFormatConstructor",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DurationFormatPrototypeResolvedOptions",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DurationFormatSupportedLocalesOf",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DurationFormatPrototypeFormat",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DurationFormatPrototypeFormatToParts",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "NumberFormatConstructor",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "NumberFormatPrototypeResolvedOptions",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "NumberFormatPrototypeFormatNumber",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "NumberFormatInternalFormatNumber",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "NumberFormatPrototypeFormatRange",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "NumberFormatPrototypeFormatRangeToParts",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DateTimeFormatConstructor",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DateTimeFormatPrototypeFormat",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DateTimeFormatInternalFormat",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "IntlGetCanonicalLocales",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "IntlSupportedValuesOf",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ListFormatConstructor",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ListFormatPrototypeResolvedOptions",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ListFormatSupportedLocalesOf",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocaleConstructor",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeMaximize",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeMinimize",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeGetCalendars",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeGetCollations",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeGetHourCycles",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeGetNumberingSystems",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeGetTextInfo",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeGetTimeZones",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeGetWeekInfo",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "RelativeTimeFormatSupportedLocalesOf",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "RelativeTimeFormatPrototypeFormat",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "RelativeTimeFormatPrototypeFormatToParts",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeLanguage",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeScript",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeRegion",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeVariants",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeBaseName",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeCalendar",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeCaseFirst",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeCollation",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeFirstDayOfWeek",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeHourCycle",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeNumeric",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeNumberingSystem",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "LocalePrototypeToString",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "RelativeTimeFormatConstructor",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "RelativeTimeFormatPrototypeResolvedOptions",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "StringPrototypeToLocaleUpperCase",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "PluralRulesConstructor",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "PluralRulesPrototypeResolvedOptions",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "PluralRulesPrototypeSelect",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "PluralRulesPrototypeSelectRange",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "PluralRulesSupportedLocalesOf",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CollatorConstructor",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CollatorPrototypeResolvedOptions",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CollatorSupportedLocalesOf",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CollatorPrototypeCompare",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CollatorInternalCompare",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "SegmentIteratorPrototypeNext",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "SegmenterConstructor",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "SegmenterSupportedLocalesOf",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "SegmenterPrototypeResolvedOptions",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "SegmenterPrototypeSegment",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "SegmentsPrototypeContaining",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "SegmentsPrototypeIterator",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "V8BreakIteratorConstructor",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "V8BreakIteratorPrototypeResolvedOptions",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "V8BreakIteratorPrototypeAdoptText",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "V8BreakIteratorInternalAdoptText",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "V8BreakIteratorPrototypeFirst",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "V8BreakIteratorInternalFirst",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "V8BreakIteratorPrototypeNext",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "V8BreakIteratorInternalNext",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "V8BreakIteratorPrototypeCurrent",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "V8BreakIteratorInternalCurrent",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "V8BreakIteratorPrototypeBreakType",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "V8BreakIteratorInternalBreakType",
    assembler: "",
  },
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 92);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
