// tokenizer.rs — 日期字符串词法分析器与 Token 定义
// 注意: C++ 侧有两套空白谓词 (char-predicates.h:72/84) —
// IsWhiteSpace (不含行终止符, 用于 word 边界, dateparser.h:119 IsWhiteSpaceChar)
// 与 IsWhiteSpaceOrLineTerminator (含 LF/CR/LS/PS, 仅用于跳过空白, dateparser-inl.h:218)。
// LS(U+2028)/PS(U+2029) >= 'A', 若 word 边界误用含行终止符版本会提前断词, 语义漂移。
use v8_strings::char_predicates::{is_white_space, is_white_space_or_line_terminator};

use crate::dateparser::tables::*;

#[inline]
pub const fn is_decimal_digit(c: u32) -> bool {
  c.wrapping_sub(b'0' as u32) <= 9
}

/// 原版 `ch_ >= 'A'` (dateparser.h:118): 有意涵盖 >= 0x80 的非 ASCII 与 'Z' 后符号,
/// 由关键字表查找兜底拒绝
#[inline]
pub const fn is_ascii_alpha_or_above(c: u32) -> bool {
  c >= b'A' as u32
}

#[inline]
pub const fn ascii_alpha_to_lower(c: u32) -> u32 {
  c | 0x20
}

pub struct InputReader<I> {
  iter: I,
  index: usize,
  ch: u32,
}

impl<I: Iterator<Item = u32>> InputReader<I> {
  pub fn new(mut iter: I) -> Self {
    let ch = iter.next().unwrap_or(0);
    Self { iter, index: 1, ch }
  }

  #[inline]
  pub fn position(&self) -> usize {
    self.index
  }

  #[inline]
  pub fn ch(&self) -> u32 {
    self.ch
  }

  #[inline]
  pub fn next(&mut self) {
    self.ch = self.iter.next().unwrap_or(0);
    self.index += 1;
  }

  pub fn read_unsigned_numeral(&mut self) -> i32 {
    let mut n = 0;
    let mut i = 0;
    while self.ch == b'0' as u32 {
      self.next();
    }
    while is_decimal_digit(self.ch) {
      if i < MAX_SIGNIFICANT_DIGITS {
        n = n * 10 + (self.ch - b'0' as u32) as i32;
      }
      i += 1;
      self.next();
    }
    n
  }

  pub fn read_word(&mut self, prefix: &mut [u32; 3]) -> usize {
    let mut len = 0;
    // 原版 (dateparser.h:94): IsAsciiAlphaOrAbove() && !IsWhiteSpaceChar()
    // IsWhiteSpaceChar => IsWhiteSpace (不含行终止符)
    while is_ascii_alpha_or_above(self.ch) && !is_white_space(self.ch) {
      if len < prefix.len() {
        prefix[len] = ascii_alpha_to_lower(self.ch);
      }
      self.next();
      len += 1;
    }
    for item in prefix.iter_mut().skip(len) {
      *item = 0;
    }
    len
  }

  pub fn skip(&mut self, c: u32) -> bool {
    if self.ch == c {
      self.next();
      true
    } else {
      false
    }
  }

  pub fn skip_white_space(&mut self) -> bool {
    if is_white_space_or_line_terminator(self.ch) {
      self.next();
      true
    } else {
      false
    }
  }

  pub fn skip_parentheses(&mut self) -> bool {
    if self.ch != b'(' as u32 {
      return false;
    }
    let mut balance = 0;
    loop {
      if self.ch == b')' as u32 {
        balance -= 1;
      } else if self.ch == b'(' as u32 {
        balance += 1;
      }
      self.next();
      if balance <= 0 || self.ch == 0 {
        break;
      }
    }
    true
  }

  #[inline]
  pub fn is_end(&self) -> bool {
    self.ch == 0
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
  Invalid,
  Unknown,
  WhiteSpace,
  Number,
  Symbol,
  EndOfInput,
  Keyword(KeywordType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateToken {
  pub kind: TokenKind,
  pub length: usize,
  pub value: i32,
}

impl DateToken {
  pub const fn keyword(kw: KeywordType, value: i32, length: usize) -> Self {
    Self {
      kind: TokenKind::Keyword(kw),
      length,
      value,
    }
  }

  pub const fn number(value: i32, length: usize) -> Self {
    Self {
      kind: TokenKind::Number,
      length,
      value,
    }
  }

  pub const fn symbol(s: char) -> Self {
    Self {
      kind: TokenKind::Symbol,
      length: 1,
      value: s as i32,
    }
  }

  pub const fn end_of_input() -> Self {
    Self {
      kind: TokenKind::EndOfInput,
      length: 0,
      value: -1,
    }
  }

  pub const fn white_space(length: usize) -> Self {
    Self {
      kind: TokenKind::WhiteSpace,
      length,
      value: -1,
    }
  }

  pub const fn unknown() -> Self {
    Self {
      kind: TokenKind::Unknown,
      length: 1,
      value: -1,
    }
  }

  pub const fn invalid() -> Self {
    Self {
      kind: TokenKind::Invalid,
      length: 0,
      value: -1,
    }
  }

  #[inline]
  pub const fn is_invalid(&self) -> bool {
    matches!(self.kind, TokenKind::Invalid)
  }

  #[inline]
  pub const fn is_number(&self) -> bool {
    matches!(self.kind, TokenKind::Number)
  }

  #[inline]
  pub const fn is_symbol(&self) -> bool {
    matches!(self.kind, TokenKind::Symbol)
  }

  #[inline]
  pub const fn is_symbol_char(&self, s: char) -> bool {
    self.is_symbol() && self.value == s as i32
  }

  #[inline]
  pub const fn is_white_space(&self) -> bool {
    matches!(self.kind, TokenKind::WhiteSpace)
  }

  #[inline]
  pub const fn is_end_of_input(&self) -> bool {
    matches!(self.kind, TokenKind::EndOfInput)
  }

  #[inline]
  pub const fn is_keyword(&self) -> bool {
    matches!(self.kind, TokenKind::Keyword(_))
  }

  #[inline]
  pub const fn keyword_type(&self) -> KeywordType {
    match self.kind {
      TokenKind::Keyword(kw) => kw,
      _ => KeywordType::Invalid,
    }
  }

  #[inline]
  pub const fn keyword_value(&self) -> i32 {
    self.value
  }

  #[inline]
  pub const fn number_value(&self) -> i32 {
    self.value
  }

  #[inline]
  pub const fn symbol_char(&self) -> char {
    self.value as u8 as char
  }

  #[inline]
  pub const fn is_keyword_type(&self, kw: KeywordType) -> bool {
    matches!(self.kind, TokenKind::Keyword(k) if k as u8 == kw as u8)
  }

  #[inline]
  pub const fn is_fixed_length_number(&self, len: usize) -> bool {
    self.is_number() && self.length == len
  }

  #[inline]
  pub const fn is_ascii_sign(&self) -> bool {
    self.is_symbol() && (self.value == b'-' as i32 || self.value == b'+' as i32)
  }

  #[inline]
  pub const fn ascii_sign(&self) -> i32 {
    44 - self.value
  }

  #[inline]
  pub const fn is_keyword_z(&self) -> bool {
    self.is_keyword_type(KeywordType::TimeZoneName) && self.length == 1 && self.value == 0
  }
}

pub struct DateStringTokenizer<I> {
  reader: InputReader<I>,
  next_token: DateToken,
}

impl<I: Iterator<Item = u32>> DateStringTokenizer<I> {
  pub fn new(reader: InputReader<I>) -> Self {
    let mut tok = Self {
      reader,
      next_token: DateToken::invalid(),
    };
    tok.next_token = tok.scan();
    tok
  }

  pub fn next_token(&mut self) -> DateToken {
    let result = self.next_token;
    self.next_token = self.scan();
    result
  }

  #[inline]
  pub fn peek(&self) -> DateToken {
    self.next_token
  }

  pub fn skip_symbol(&mut self, symbol: char) -> bool {
    if self.next_token.is_symbol_char(symbol) {
      self.next_token = self.scan();
      true
    } else {
      false
    }
  }

  fn scan(&mut self) -> DateToken {
    let pre_pos = self.reader.position();
    if self.reader.is_end() {
      return DateToken::end_of_input();
    }
    if is_decimal_digit(self.reader.ch()) {
      let n = self.reader.read_unsigned_numeral();
      let length = self.reader.position() - pre_pos;
      return DateToken::number(n, length);
    }
    if self.reader.skip(b':' as u32) {
      return DateToken::symbol(':');
    }
    if self.reader.skip(b'-' as u32) {
      return DateToken::symbol('-');
    }
    if self.reader.skip(b'+' as u32) {
      return DateToken::symbol('+');
    }
    if self.reader.skip(b'.' as u32) {
      return DateToken::symbol('.');
    }
    if self.reader.skip(b')' as u32) {
      return DateToken::symbol(')');
    }
    // 原版 (dateparser-inl.h:198): IsAsciiAlphaOrAbove() && !IsWhiteSpaceChar()
    // IsWhiteSpaceChar => IsWhiteSpace (不含行终止符, LS/PS 属 word 字符)
    if is_ascii_alpha_or_above(self.reader.ch()) && !is_white_space(self.reader.ch()) {
      let mut buffer = [0u32; 3];
      let length = self.reader.read_word(&mut buffer);
      let idx = lookup_keyword(&buffer, length);
      return match idx {
        Some(i) => DateToken::keyword(KEYWORD_TABLE[i].kw_type, KEYWORD_TABLE[i].value, length),
        None => DateToken::keyword(KeywordType::Invalid, 0, length),
      };
    }
    if self.reader.skip_white_space() {
      return DateToken::white_space(self.reader.position() - pre_pos);
    }
    if self.reader.skip_parentheses() {
      return DateToken::unknown();
    }
    self.reader.next();
    DateToken::unknown()
  }
}
