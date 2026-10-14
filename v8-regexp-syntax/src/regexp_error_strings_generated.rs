// 由 tool/src/regexp/regexp-error.cc.js 从 v8/src/regexp/regexp-error.cc + regexp-error.h 生成; 修订就地改, 复跑覆盖
// 正则错误消息表 (REGEXP_ERROR_MESSAGES 展开)

/// 错误消息字符串表 (对应原版 kErrorStrings[], 索引即 Error 枚举序)
pub const ERROR_STRINGS: &[&str] = &[
  "", // None
  "Maximum call stack size exceeded", // StackOverflow
  "Stack overflow", // AnalysisStackOverflow
  "Regular expression too large", // TooLarge
  "Unterminated group", // UnterminatedGroup
  "Unmatched ')'", // UnmatchedParen
  "\\ at end of pattern", // EscapeAtEndOfPattern
  "Invalid property name", // InvalidPropertyName
  "Invalid escape", // InvalidEscape
  "Invalid decimal escape", // InvalidDecimalEscape
  "Invalid Unicode escape", // InvalidUnicodeEscape
  "Nothing to repeat", // NothingToRepeat
  "Lone quantifier brackets", // LoneQuantifierBrackets
  "numbers out of order in {} quantifier", // RangeOutOfOrder
  "Incomplete quantifier", // IncompleteQuantifier
  "Invalid quantifier", // InvalidQuantifier
  "Invalid group", // InvalidGroup
  "Multiple dashes in flag group", // MultipleFlagDashes
  "Cannot be executed in linear time", // NotLinear
  "Repeated flag in flag group", // RepeatedFlag
  "Invalid flag group", // InvalidFlagGroup
  "Too many captures", // TooManyCaptures
  "Invalid capture group name", // InvalidCaptureGroupName
  "Duplicate capture group name", // DuplicateCaptureGroupName
  "Invalid named reference", // InvalidNamedReference
  "Invalid named capture referenced", // InvalidNamedCaptureReference
  "Invalid property name in character class", // InvalidClassPropertyName
  "Invalid character class", // InvalidCharacterClass
  "Unterminated character class", // UnterminatedCharacterClass
  "Range out of order in character class", // OutOfOrderCharacterClass
  "Invalid set operation in character class", // InvalidClassSetOperation
  "Invalid character in character class", // InvalidCharacterInClass
  "Negated character class may contain strings", // NegatedCharacterClassWithStrings
  "Unsupported Bytecode", // UnsupportedBytecode
];

/// 错误名登记 (与 ERROR_STRINGS 同序)
pub const ERROR_NAMES: &[&str] = &[
  "None",
  "StackOverflow",
  "AnalysisStackOverflow",
  "TooLarge",
  "UnterminatedGroup",
  "UnmatchedParen",
  "EscapeAtEndOfPattern",
  "InvalidPropertyName",
  "InvalidEscape",
  "InvalidDecimalEscape",
  "InvalidUnicodeEscape",
  "NothingToRepeat",
  "LoneQuantifierBrackets",
  "RangeOutOfOrder",
  "IncompleteQuantifier",
  "InvalidQuantifier",
  "InvalidGroup",
  "MultipleFlagDashes",
  "NotLinear",
  "RepeatedFlag",
  "InvalidFlagGroup",
  "TooManyCaptures",
  "InvalidCaptureGroupName",
  "DuplicateCaptureGroupName",
  "InvalidNamedReference",
  "InvalidNamedCaptureReference",
  "InvalidClassPropertyName",
  "InvalidCharacterClass",
  "UnterminatedCharacterClass",
  "OutOfOrderCharacterClass",
  "InvalidClassSetOperation",
  "InvalidCharacterInClass",
  "NegatedCharacterClassWithStrings",
  "UnsupportedBytecode",
];

/// 原版 ErrorString(Error) — 越界由调用方保证 (DCHECK_LT(error, Error::NumErrors))
#[inline]
#[must_use]
pub fn error_string(error: usize) -> &'static str {
  ERROR_STRINGS[error]
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(ERROR_STRINGS.len() == 34);
  assert!(ERROR_NAMES.len() == ERROR_STRINGS.len());
};
