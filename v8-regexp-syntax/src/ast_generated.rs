// 由 tool/src/regexp/regexp-ast.h.js 从 v8/src/regexp/regexp-ast.h 生成; 修订就地改, 复跑覆盖
// X-macro/枚举展开; 节点结构体与语义在 ast.rs

/// RegExpTree 节点种类全集 (FOR_EACH_REG_EXP_TREE_TYPE 展开)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TreeKind {
  Disjunction = 0,
  Alternative = 1,
  Assertion = 2,
  ClassRanges = 3,
  ClassSetOperand = 4,
  ClassSetExpression = 5,
  Atom = 6,
  Quantifier = 7,
  Capture = 8,
  Group = 9,
  Lookaround = 10,
  BackReference = 11,
  Empty = 12,
  Text = 13,
}

pub const NUM_TREE_KINDS: usize = 14;

// ── 对数断言: 枚举实数 == 宏实数 ──
const _: () = assert!(TreeKind::Text as usize + 1 == NUM_TREE_KINDS);

/// 命名标准字符集 (原版 enum class StandardCharacterSet : char)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StandardCharacterSet {
  Whitespace = 0,
  NotWhitespace = 1,
  Word = 2,
  NotWord = 3,
  Digit = 4,
  NotDigit = 5,
  LineTerminator = 6,
  NotLineTerminator = 7,
  Everything = 8,
}

pub const NUM_STANDARD_CHARACTER_SETS: usize = 9;
const _: () = assert!(StandardCharacterSet::Everything as usize + 1 == NUM_STANDARD_CHARACTER_SETS);

impl StandardCharacterSet {
  /// 原 char 值 (转义字母或简写符号)
  pub const CHAR: [char; NUM_STANDARD_CHARACTER_SETS] =
    ['s', 'S', 'w', 'W', 'd', 'D', 'n', '.', '*'];

  /// 按转义字符检索 ('\u{5c}' 转义的目标字符)
  #[must_use]
  pub const fn from_escape(c: char) -> Option<Self> {
    match c {
      's' => Some(Self::Whitespace),
      'S' => Some(Self::NotWhitespace),
      'w' => Some(Self::Word),
      'W' => Some(Self::NotWord),
      'd' => Some(Self::Digit),
      'D' => Some(Self::NotDigit),
      'n' => Some(Self::LineTerminator),
      '.' => Some(Self::NotLineTerminator),
      '*' => Some(Self::Everything),
      _ => None,
    }
  }
}

/// ClassRanges 附加标志 (原版 ClassRanges::Flag)
pub type ClassRangesFlags = u8;
pub const NEGATED: ClassRangesFlags = 1 << 0;
pub const CONTAINS_SPLIT_SURROGATE: ClassRangesFlags = 1 << 1;
pub const NO_CASE_FOLDING_NEEDED: ClassRangesFlags = 1 << 2;
pub const IS_CERTAINLY_ONE_CODE_POINT: ClassRangesFlags = 1 << 3;
pub const IS_CERTAINLY_TWO_CODE_POINTS: ClassRangesFlags = 1 << 4;

pub const NUM_CLASS_RANGES_FLAGS: usize = 5;

/// 断言类型 (原版 Assertion::Type, 序数 1:1; SCREAMING → PascalCase 承接)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AssertionType {
  StartOfLine = 0,
  StartOfInput = 1,
  EndOfLine = 2,
  EndOfInput = 3,
  EndOfBuffer = 4,
  Boundary = 5,
  NonBoundary = 6,
}

pub const NUM_ASSERTION_TYPES: usize = 7;
const _: () = assert!(AssertionType::NonBoundary as usize + 1 == NUM_ASSERTION_TYPES);
