// 由 tool/src/parsing/scanner.h.js 从 v8/src/parsing/scanner.h + scanner-inl.h 生成; 修订就地改, 复跑覆盖
// 词法扫描器扫描标志位、单字符Token表与数字类型

use crate::token_generated::Token;

/// 字符向前看缓冲大小 (1)
pub const K_CHARACTER_LOOKAHEAD_BUFFER_SIZE: usize = 1;

/// 最大 ASCII 字符值 (127)
pub const K_MAX_ASCII: u32 = 127;

/// 输入流结束标记
pub const K_END_OF_INPUT: u32 = u32::MAX;

/// 数值字面量分类 (原版 Scanner::NumberKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberKind {
  ImplicitOctal,
  Binary,
  Octal,
  Hex,
  Decimal,
  DecimalWithLeadingZero,
}

impl NumberKind {
  #[inline]
  #[must_use]
  pub const fn is_valid_bigint_kind(&self) -> bool {
    matches!(self, Self::Binary | Self::Octal | Self::Hex | Self::Decimal)
  }

  #[inline]
  #[must_use]
  pub const fn is_decimal_number_kind(&self) -> bool {
    matches!(self, Self::Decimal | Self::DecimalWithLeadingZero)
  }
}

/// 扫描标志位 (原版 Scanner::ScanFlags)
pub mod scan_flags {
  pub const K_TERMINATES_LITERAL: u8 = 1 << 0;
  pub const K_CANNOT_BE_KEYWORD: u8 = 1 << 1;
  pub const K_CANNOT_BE_KEYWORD_START: u8 = 1 << 2;
  pub const K_STRING_TERMINATOR: u8 = 1 << 3;
  pub const K_IDENTIFIER_NEEDS_SLOW_PATH: u8 = 1 << 4;
  pub const K_MULTILINE_COMMENT_CHARACTER_NEEDS_SLOW_PATH: u8 = 1 << 5;

  #[inline]
  #[must_use]
  pub const fn terminates_literal(flags: u8) -> bool {
    (flags & K_TERMINATES_LITERAL) != 0
  }

  #[inline]
  #[must_use]
  pub const fn can_be_keyword(flags: u8) -> bool {
    (flags & K_CANNOT_BE_KEYWORD) == 0
  }

  #[inline]
  #[must_use]
  pub const fn identifier_needs_slow_path(flags: u8) -> bool {
    (flags & K_IDENTIFIER_NEEDS_SLOW_PATH) != 0
  }

  #[inline]
  #[must_use]
  pub const fn multiline_comment_character_needs_slow_path(flags: u8) -> bool {
    (flags & K_MULTILINE_COMMENT_CHARACTER_NEEDS_SLOW_PATH) != 0
  }

  #[inline]
  #[must_use]
  pub const fn may_terminate_string(flags: u8) -> bool {
    (flags & K_STRING_TERMINATOR) != 0
  }
}

/// ASCII 0..127 扫描标志查找表 (character_scan_flags)
pub static CHARACTER_SCAN_FLAGS: [u8; 128] = [
  5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 45, 5, 5, 45, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
  5, 5, 13, 5, 6, 5, 5, 13, 5, 5, 37, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 5, 5, 5, 5, 5,
  5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 5, 29, 5, 5,
  6, 5, 0, 0, 0, 0, 0, 0, 0, 4, 0, 6, 4, 0, 4, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 4, 0, 6, 5, 5, 5, 5,
  5,
];

/// ASCII 0..127 单字符 Token 映射表 (one_char_tokens)
pub static ONE_CHAR_TOKENS: [Token; 128] = [
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Whitespace,
  Token::Whitespace,
  Token::Whitespace,
  Token::Whitespace,
  Token::Whitespace,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Illegal,
  Token::Whitespace,
  Token::Not,
  Token::String,
  Token::PrivateName,
  Token::Identifier,
  Token::Mod,
  Token::BitAnd,
  Token::String,
  Token::LeftParen,
  Token::RightParen,
  Token::Mul,
  Token::Add,
  Token::Comma,
  Token::Sub,
  Token::Period,
  Token::Div,
  Token::Number,
  Token::Number,
  Token::Number,
  Token::Number,
  Token::Number,
  Token::Number,
  Token::Number,
  Token::Number,
  Token::Number,
  Token::Number,
  Token::Colon,
  Token::Semicolon,
  Token::LessThan,
  Token::Assign,
  Token::GreaterThan,
  Token::Conditional,
  Token::Illegal,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::LeftBracket,
  Token::Identifier,
  Token::RightBracket,
  Token::BitXor,
  Token::Identifier,
  Token::TemplateSpan,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::Identifier,
  Token::LeftBrace,
  Token::BitOr,
  Token::RightBrace,
  Token::BitNot,
  Token::Illegal,
];

/// 源码位置区间 (原版 Scanner::Location)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Location {
  pub beg_pos: usize,
  pub end_pos: usize,
}

impl Location {
  #[inline]
  #[must_use]
  pub const fn new(beg_pos: usize, end_pos: usize) -> Self {
    Self { beg_pos, end_pos }
  }

  #[inline]
  #[must_use]
  pub const fn invalid() -> Self {
    Self {
      beg_pos: usize::MAX,
      end_pos: usize::MAX,
    }
  }

  #[inline]
  #[must_use]
  pub const fn is_valid(&self) -> bool {
    self.beg_pos != usize::MAX && self.beg_pos <= self.end_pos
  }

  #[inline]
  #[must_use]
  pub const fn length(&self) -> usize {
    self.end_pos.saturating_sub(self.beg_pos)
  }
}
