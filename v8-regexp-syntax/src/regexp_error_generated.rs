// 由 tool/src/regexp/regexp-error.h.js 从 v8/src/regexp/regexp-error.h 生成; 修订就地改, 复跑覆盖
// REGEXP_ERROR_MESSAGES X-macro展开; None → 成功哨兵不进枚举

/// 正则解析/编译错误类别 (原版 enum class Error, kNone 以 Option/成功路径表达)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegexpError {
  StackOverflow,
  AnalysisStackOverflow,
  TooLarge,
  UnterminatedGroup,
  UnmatchedParen,
  EscapeAtEndOfPattern,
  InvalidPropertyName,
  InvalidEscape,
  InvalidDecimalEscape,
  InvalidUnicodeEscape,
  NothingToRepeat,
  LoneQuantifierBrackets,
  RangeOutOfOrder,
  IncompleteQuantifier,
  InvalidQuantifier,
  InvalidGroup,
  MultipleFlagDashes,
  NotLinear,
  RepeatedFlag,
  InvalidFlagGroup,
  TooManyCaptures,
  InvalidCaptureGroupName,
  DuplicateCaptureGroupName,
  InvalidNamedReference,
  InvalidNamedCaptureReference,
  InvalidClassPropertyName,
  InvalidCharacterClass,
  UnterminatedCharacterClass,
  OutOfOrderCharacterClass,
  InvalidClassSetOperation,
  InvalidCharacterInClass,
  NegatedCharacterClassWithStrings,
  UnsupportedBytecode,
}

pub const NUM_REGEXP_ERRORS: usize = 33;

// ── 对数断言: 枚举判别数 == 宏实数 ──
const _: () = assert!(RegexpError::VARIANTS.len() == NUM_REGEXP_ERRORS);

impl RegexpError {
  /// 全部变体 (None 哨兵除外, 序与原版一致)
  pub const VARIANTS: &'static [RegexpError] = &[
    RegexpError::StackOverflow,
    RegexpError::AnalysisStackOverflow,
    RegexpError::TooLarge,
    RegexpError::UnterminatedGroup,
    RegexpError::UnmatchedParen,
    RegexpError::EscapeAtEndOfPattern,
    RegexpError::InvalidPropertyName,
    RegexpError::InvalidEscape,
    RegexpError::InvalidDecimalEscape,
    RegexpError::InvalidUnicodeEscape,
    RegexpError::NothingToRepeat,
    RegexpError::LoneQuantifierBrackets,
    RegexpError::RangeOutOfOrder,
    RegexpError::IncompleteQuantifier,
    RegexpError::InvalidQuantifier,
    RegexpError::InvalidGroup,
    RegexpError::MultipleFlagDashes,
    RegexpError::NotLinear,
    RegexpError::RepeatedFlag,
    RegexpError::InvalidFlagGroup,
    RegexpError::TooManyCaptures,
    RegexpError::InvalidCaptureGroupName,
    RegexpError::DuplicateCaptureGroupName,
    RegexpError::InvalidNamedReference,
    RegexpError::InvalidNamedCaptureReference,
    RegexpError::InvalidClassPropertyName,
    RegexpError::InvalidCharacterClass,
    RegexpError::UnterminatedCharacterClass,
    RegexpError::OutOfOrderCharacterClass,
    RegexpError::InvalidClassSetOperation,
    RegexpError::InvalidCharacterInClass,
    RegexpError::NegatedCharacterClassWithStrings,
    RegexpError::UnsupportedBytecode,
  ];

  /// 错误消息 (原版 ErrorString)
  #[must_use]
  pub const fn message(self) -> &'static str {
    match self {
      RegexpError::StackOverflow => "Maximum call stack size exceeded",
      RegexpError::AnalysisStackOverflow => "Stack overflow",
      RegexpError::TooLarge => "Regular expression too large",
      RegexpError::UnterminatedGroup => "Unterminated group",
      RegexpError::UnmatchedParen => "Unmatched ')'",
      RegexpError::EscapeAtEndOfPattern => "\\ at end of pattern",
      RegexpError::InvalidPropertyName => "Invalid property name",
      RegexpError::InvalidEscape => "Invalid escape",
      RegexpError::InvalidDecimalEscape => "Invalid decimal escape",
      RegexpError::InvalidUnicodeEscape => "Invalid Unicode escape",
      RegexpError::NothingToRepeat => "Nothing to repeat",
      RegexpError::LoneQuantifierBrackets => "Lone quantifier brackets",
      RegexpError::RangeOutOfOrder => "numbers out of order in {} quantifier",
      RegexpError::IncompleteQuantifier => "Incomplete quantifier",
      RegexpError::InvalidQuantifier => "Invalid quantifier",
      RegexpError::InvalidGroup => "Invalid group",
      RegexpError::MultipleFlagDashes => "Multiple dashes in flag group",
      RegexpError::NotLinear => "Cannot be executed in linear time",
      RegexpError::RepeatedFlag => "Repeated flag in flag group",
      RegexpError::InvalidFlagGroup => "Invalid flag group",
      RegexpError::TooManyCaptures => "Too many captures",
      RegexpError::InvalidCaptureGroupName => "Invalid capture group name",
      RegexpError::DuplicateCaptureGroupName => "Duplicate capture group name",
      RegexpError::InvalidNamedReference => "Invalid named reference",
      RegexpError::InvalidNamedCaptureReference => "Invalid named capture referenced",
      RegexpError::InvalidClassPropertyName => "Invalid property name in character class",
      RegexpError::InvalidCharacterClass => "Invalid character class",
      RegexpError::UnterminatedCharacterClass => "Unterminated character class",
      RegexpError::OutOfOrderCharacterClass => "Range out of order in character class",
      RegexpError::InvalidClassSetOperation => "Invalid set operation in character class",
      RegexpError::InvalidCharacterInClass => "Invalid character in character class",
      RegexpError::NegatedCharacterClassWithStrings => {
        "Negated character class may contain strings"
      }
      RegexpError::UnsupportedBytecode => "Unsupported Bytecode",
    }
  }

  /// 是否栈溢出类 (原版 ErrorIsStackOverflow: kStackOverflow / kAnalysisStackOverflow / kTooLarge)
  #[must_use]
  pub const fn is_stack_overflow(self) -> bool {
    matches!(
      self,
      RegexpError::StackOverflow | RegexpError::AnalysisStackOverflow | RegexpError::TooLarge
    )
  }
}
