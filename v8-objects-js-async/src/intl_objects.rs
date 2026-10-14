// 由 tool/src/objects/intl-objects.h.js 从 v8/src/objects/intl-objects.{h,cc,tq} 生成; 修订就地改, 复跑覆盖
// Intl 基础设施: 选项枚举/数字格式化参数结构/ASCII collation 权重表锚
//! Intl 公共面 (ECMA402): 选项枚举 + 数字位数参数 + 区域工具。
//!
//! ICU 交互实现面登记跳过 (3344 行 .cc), 由 Developer 就地实现;
//! ASCII collation 权重表 (L1/L3 各 256 项) 数值面归 .cc 数据表就地迁移。

/// 原版 Intl::Notation。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Notation {
  /// STANDARD
  STANDARD = 0,
  /// SCIENTIFIC
  SCIENTIFIC = 1,
  /// ENGINEERING
  ENGINEERING = 2,
  /// COMPACT
  COMPACT = 3,
}

/// 原版 Intl::CompactDisplay。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CompactDisplay {
  /// SHORT
  SHORT = 0,
  /// LONG
  LONG = 1,
}

/// 原版 Intl::FormatRangeSource。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FormatRangeSource {
  /// kShared
  Shared = 0,
  /// kStartRange
  StartRange = 1,
  /// kEndRange
  EndRange = 2,
}

/// 原版 Intl::CompareStringsOptions。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CompareStringsOptions {
  /// kNone
  None = 0,
  /// kTryFastPath
  TryFastPath = 1,
}

/// 原版 Intl::RoundingPriority。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RoundingPriority {
  /// kAuto
  Auto = 0,
  /// kMorePrecision
  MorePrecision = 1,
  /// kLessPrecision
  LessPrecision = 2,
}

/// 原版 Intl::RoundingType。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RoundingType {
  /// kFractionDigits
  FractionDigits = 0,
  /// kSignificantDigits
  SignificantDigits = 1,
  /// kMorePrecision
  MorePrecision = 2,
  /// kLessPrecision
  LessPrecision = 3,
}

/// 原版 Intl::RoundingMode。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RoundingMode {
  /// kCeil
  Ceil = 0,
  /// kFloor
  Floor = 1,
  /// kExpand
  Expand = 2,
  /// kTrunc
  Trunc = 3,
  /// kHalfCeil
  HalfCeil = 4,
  /// kHalfFloor
  HalfFloor = 5,
  /// kHalfExpand
  HalfExpand = 6,
  /// kHalfTrunc
  HalfTrunc = 7,
  /// kHalfEven
  HalfEven = 8,
}

/// 原版 Intl::TrailingZeroDisplay。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TrailingZeroDisplay {
  /// kAuto
  Auto = 0,
  /// kStripIfInteger
  StripIfInteger = 1,
}

/// 原版 Intl::MatcherOption。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MatcherOption {
  /// kBestFit
  BestFit = 0,
  /// kLookup
  Lookup = 1,
}

// ── BoundFunctionContextSlot (上下文槽, 自 MIN_CONTEXT_SLOTS=2 起) ──
pub const K_BOUND_FUNCTION_CONTEXT_SLOT: u32 = 2;
pub const K_BOUND_FUNCTION_CONTEXT_LENGTH: u32 = 4;

// ── NumberFormatDigitOptions (原版 struct, 9 字段) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberFormatDigitOptions {
  /// minimum_integer_digits
  pub minimum_integer_digits: i32,
  /// minimum_fraction_digits
  pub minimum_fraction_digits: i32,
  /// maximum_fraction_digits
  pub maximum_fraction_digits: i32,
  /// minimum_significant_digits
  pub minimum_significant_digits: i32,
  /// maximum_significant_digits
  pub maximum_significant_digits: i32,
  /// rounding_priority
  pub rounding_priority: RoundingPriority,
  /// rounding_type
  pub rounding_type: RoundingType,
  /// rounding_increment
  pub rounding_increment: i32,
  /// rounding_mode
  pub rounding_mode: RoundingMode,
  /// trailing_zero_display
  pub trailing_zero_display: TrailingZeroDisplay,
}

// ── kCaseMappingSpecialLocales (az/el/lt/tr, 大小写特殊映射) ──
pub const K_CASE_MAPPING_SPECIAL_LOCALES: [&str; 4] = ["az", "el", "lt", "tr"];

// ── ASCII collation 权重表锚 (h 声明 / cc 落值 / tq 断言, 三方对数) ──
/// ASCII collation 权重表长度 (L1/L3 各 256 槽: 显式 128 + 隐式零 128; tq static_assert ×2)。
pub const K_ASCII_COLLATION_WEIGHTS_LENGTH: usize = 256;
/// L1/L3 表显式初始化项数 (仅 ASCII 前半 0..127; 后半聚合初始化隐式补零)。
pub const K_ASCII_COLLATION_WEIGHTS_EXPLICIT_ENTRIES: usize = 128;

// ── .cc 时间常数 (Temporal/DateTime 用) ──
/// 毫秒时间下界 (原版 kMsMinInstant = -8.64e15)。
pub const K_MS_MIN_INSTANT: i64 = -8_640_000_000_000_000;
/// 毫秒时间上界 (原版 kMsMaxInstant = 8.64e15)。
pub const K_MS_MAX_INSTANT: i64 = 8_640_000_000_000_000;

// ── .cc Intl:: 方法签名骨架 (深水推迟台账, 依 readme/zh.md; ICU + Isolate 面) ──
/// v8: src/objects/intl-objects.cc Intl::CanonicalizeLocaleList。
pub fn canonicalize_locale_list() {
  todo!(
    "Intl::CanonicalizeLocaleList: 依赖 Isolate + JSArray + Intl 枚举 + Lookup 匹配; 解锁 v8-execution-isolate + v8-objects-js-core + ICU"
  )
}

/// v8: src/objects/intl-objects.cc Intl::GetCanonicalLocales。
pub fn get_canonical_locales() {
  todo!(
    "Intl::GetCanonicalLocales: 依赖 Isolate + JSArray + icu::Locale; 解锁 ICU + v8-execution-isolate"
  )
}

/// v8: src/objects/intl-objects.cc Intl::SupportedValuesOf。
pub fn supported_values_of() {
  todo!(
    "Intl::SupportedValuesOf: 依赖 icu::Locale + 支持键枚举 + Isolate + JSArray; 解锁 ICU + v8-execution-isolate"
  )
}

/// v8: src/objects/intl-objects.cc Intl::CompareStrings。
pub fn compare_strings() {
  todo!("Intl::CompareStrings: 依赖 icu::Collator + Handle<String> + 快路径; 解锁 ICU + v8-handles")
}

/// v8: src/objects/intl-objects.cc Intl::SetNumberFormatDigitOptions。
pub fn set_number_format_digit_options() {
  todo!(
    "Intl::SetNumberFormatDigitOptions: 依赖 option-utils Get* + Isolate + Handle<Object>; 解锁 v8-execution-isolate + v8-objects-js-core"
  )
}

/// v8: src/objects/intl-objects.cc Intl::ResolveLocale。
pub fn resolve_locale() {
  todo!(
    "Intl::ResolveLocale: 依赖 icu::Locale + locale matching + Isolate; 解锁 ICU + v8-execution-isolate"
  )
}

/// v8: src/objects/intl-objects.cc Intl::ToLanguageTag。
pub fn to_language_tag() {
  todo!("Intl::ToLanguageTag: 依赖 icu::Locale → BCP 47 序列化; 解锁 ICU")
}

// ── 提取不变式断言 ── (const 上下文无 PartialEq<&str>, 用字节循环 const fn)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}
const _: () = {
  assert!(Notation::STANDARD as u8 == 0);
  assert!(Notation::COMPACT as u8 == 3);
  assert!(RoundingMode::HalfEven as u8 == 8);
  assert!(K_BOUND_FUNCTION_CONTEXT_SLOT == 2);
  assert!(K_BOUND_FUNCTION_CONTEXT_LENGTH == 4);
  assert!(K_CASE_MAPPING_SPECIAL_LOCALES.len() == 4);
  assert!(str_eq(K_CASE_MAPPING_SPECIAL_LOCALES[0], "az"));
  assert!(K_ASCII_COLLATION_WEIGHTS_LENGTH == 256);
  assert!(K_MS_MIN_INSTANT == -8_640_000_000_000_000);
  assert!(K_MS_MAX_INSTANT == 8_640_000_000_000_000);
  assert!(-K_MS_MIN_INSTANT == K_MS_MAX_INSTANT);
};
