//! 词法扫描器 (对应原版 v8/src/parsing/scanner.*)

use std::{mem::swap, str::from_utf8_unchecked};

use v8_ast::{AstRawString, AstValueFactory};
use v8_strings::{
  char_predicates,
  unicode::{is_line_terminator, is_string_literal_line_terminator, utf16},
};

pub use crate::scanner_generated::Location;
use crate::{
  character_stream::CharacterStream,
  keywords_generated::lookup_keyword,
  literal_buffer::LiteralBuffer,
  scanner_generated::{
    CHARACTER_SCAN_FLAGS, K_END_OF_INPUT, K_MAX_ASCII, NumberKind, ONE_CHAR_TOKENS, scan_flags,
  },
  token_generated::Token,
};

/// 扫描器错误信息模板 (原版 MessageTemplate, 仅 scanner 相关子集)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageTemplate {
  #[default]
  None,
  BigIntTooBig,
  ContinuousNumericSeparator,
  HtmlCommentInModule,
  InvalidHexEscapeSequence,
  InvalidOrUnexpectedToken,
  InvalidUnicodeEscapeSequence,
  Strict8Or9Escape,
  StrictDecimalWithLeadingZero,
  StrictOctalEscape,
  StrictOctalLiteral,
  Template8Or9Escape,
  TemplateOctalLiteral,
  TrailingNumericSeparator,
  UndefinedUnicodeCodePoint,
  ZeroDigitNumericSeparator,
}

/// 扫描器配置标记
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ScannerFlags {
  pub is_module: bool,
}

/// 源码行列坐标 (line 和 column 均从 1 开始计, offset 从 0 开始计)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourcePosition {
  pub offset: usize,
  pub line: usize,
  pub column: usize,
}

impl SourcePosition {
  #[inline]
  #[must_use]
  pub const fn new(offset: usize, line: usize, column: usize) -> Self {
    Self {
      offset,
      line,
      column,
    }
  }
}

/// Token 描述体 (原版 Scanner::TokenDesc)
#[derive(Debug, Clone)]
pub struct TokenDesc {
  pub location: Location,
  pub literal_chars: LiteralBuffer,
  pub raw_literal_chars: LiteralBuffer,
  pub token: Token,
  pub invalid_template_escape_message: MessageTemplate,
  pub invalid_template_escape_location: Location,
  pub number_kind: NumberKind,
  pub smi_value: u32,
  pub after_line_terminator: bool,
}

impl Default for TokenDesc {
  fn default() -> Self {
    Self {
      location: Location::default(),
      literal_chars: LiteralBuffer::default(),
      raw_literal_chars: LiteralBuffer::default(),
      token: Token::Uninitialized,
      invalid_template_escape_message: MessageTemplate::None,
      invalid_template_escape_location: Location::default(),
      number_kind: NumberKind::Decimal,
      smi_value: 0,
      after_line_terminator: false,
    }
  }
}

/// JavaScript 词法扫描器
pub struct Scanner<S> {
  source: S,
  flags: ScannerFlags,
  c0: u32,
  token_storage: [TokenDesc; 4],
  current_idx: usize,
  next_idx: usize,
  next_next_idx: usize,
  next_next_next_idx: usize,
  found_html_comment: bool,
  scanner_error: MessageTemplate,
  scanner_error_location: Location,
  octal_pos: Location,
  octal_message: MessageTemplate,
  line_starts: Vec<usize>,
}

#[inline]
fn strip_radix_prefix(s: &str, prefix: u8) -> &str {
  if s.len() >= 2 && s.as_bytes()[0] == b'0' && (s.as_bytes()[1] | 0x20) == prefix {
    &s[2..]
  } else {
    s
  }
}

#[inline]
fn parse_radix_f64(s: &str, radix: u32) -> f64 {
  if s.is_empty() {
    return f64::NAN;
  }
  let mut val: f64 = 0.0;
  let radix_f64 = radix as f64;
  for &b in s.as_bytes() {
    let digit = match b {
      b'0'..=b'9' => (b - b'0') as f64,
      b'a'..=b'z' => (b - b'a' + 10) as f64,
      b'A'..=b'Z' => (b - b'A' + 10) as f64,
      _ => return f64::NAN,
    };
    if digit >= radix_f64 {
      return f64::NAN;
    }
    val = val * radix_f64 + digit;
  }
  val
}

impl<S: CharacterStream> Scanner<S> {
  /// 创建扫描器实例
  pub fn new(source: S) -> Self {
    Self::with_flags(source, ScannerFlags::default())
  }

  /// 指定编译标记创建扫描器实例
  pub fn with_flags(source: S, flags: ScannerFlags) -> Self {
    Self {
      source,
      flags,
      c0: K_END_OF_INPUT,
      token_storage: [
        TokenDesc::default(),
        TokenDesc::default(),
        TokenDesc::default(),
        TokenDesc::default(),
      ],
      current_idx: 0,
      next_idx: 1,
      next_next_idx: 2,
      next_next_next_idx: 3,
      found_html_comment: false,
      scanner_error: MessageTemplate::None,
      scanner_error_location: Location::invalid(),
      octal_pos: Location::invalid(),
      octal_message: MessageTemplate::None,
      line_starts: vec![0],
    }
  }

  /// 初始化扫描器 (预取首个字符与首个 Token)
  pub fn initialize(&mut self) {
    self.init();
    self.token_storage[self.next_idx].after_line_terminator = true;
    self.scan();
  }

  fn init(&mut self) {
    self.c0 = self.source.advance();
    self.current_idx = 0;
    self.next_idx = 1;
    self.next_next_idx = 2;
    self.next_next_next_idx = 3;
    self.found_html_comment = false;
    self.scanner_error = MessageTemplate::None;
  }

  /// 获取当前已消费的 Token (原版 Next())
  pub fn next_token(&mut self) -> Token {
    let prev = self.current_idx;
    self.current_idx = self.next_idx;

    if self.token_storage[self.next_next_idx].token == Token::Uninitialized {
      debug_assert_eq!(
        self.token_storage[self.next_next_next_idx].token,
        Token::Uninitialized
      );
      self.next_idx = prev;
      self.token_storage[prev].after_line_terminator = false;
      self.token_storage[prev].invalid_template_escape_message = MessageTemplate::None;
      self.token_storage[prev].invalid_template_escape_location = Location::invalid();
      self.scan();
    } else {
      self.next_idx = self.next_next_idx;
      if self.token_storage[self.next_next_next_idx].token == Token::Uninitialized {
        self.next_next_idx = prev;
      } else {
        self.next_next_idx = self.next_next_next_idx;
        self.next_next_next_idx = prev;
      }
      self.token_storage[prev].token = Token::Uninitialized;
      self.token_storage[prev].invalid_template_escape_message = MessageTemplate::None;
      self.token_storage[prev].invalid_template_escape_location = Location::invalid();
      debug_assert_ne!(Token::Uninitialized, self.current().token);
    }

    self.current().token
  }

  /// 查看即将到来的下一个 Token (原版 peek())
  #[inline]
  #[must_use]
  pub fn peek(&self) -> Token {
    self.next_desc().token
  }

  /// 向前看第二个 Token (原版 PeekAhead())
  pub fn peek_ahead(&mut self) -> Token {
    debug_assert!(self.next_desc().token != Token::Div);
    debug_assert!(self.next_desc().token != Token::AssignDiv);

    if self.token_storage[self.next_next_idx].token != Token::Uninitialized {
      return self.token_storage[self.next_next_idx].token;
    }
    swap(&mut self.next_idx, &mut self.next_next_idx);
    self.token_storage[self.next_idx].after_line_terminator = false;
    self.scan();
    swap(&mut self.next_idx, &mut self.next_next_idx);
    self.token_storage[self.next_next_idx].token
  }

  /// 向前看第三个 Token (原版 PeekAheadAhead())
  pub fn peek_ahead_ahead(&mut self) -> Token {
    if self.token_storage[self.next_next_next_idx].token != Token::Uninitialized {
      return self.token_storage[self.next_next_next_idx].token;
    }
    // PeekAhead() 必须先执行以保证 lookahead 环形缓冲有序连续
    if self.token_storage[self.next_next_idx].token == Token::Uninitialized {
      self.peek_ahead();
    }
    swap(&mut self.next_idx, &mut self.next_next_next_idx);
    self.token_storage[self.next_idx].after_line_terminator = false;
    self.scan();
    swap(&mut self.next_idx, &mut self.next_next_next_idx);
    self.token_storage[self.next_next_next_idx].token
  }

  /// 获取当前 Token
  #[inline]
  #[must_use]
  pub fn current_token(&self) -> Token {
    self.current().token
  }

  /// 当前 Token 的源码区间
  #[inline]
  #[must_use]
  pub fn location(&self) -> Location {
    self.current().location
  }

  /// 下一个 Token 的源码区间
  #[inline]
  #[must_use]
  pub fn peek_location(&self) -> Location {
    self.next_desc().location
  }

  /// 向前看第二个 Token 的源码区间
  #[inline]
  #[must_use]
  pub fn peek_ahead_location(&self) -> Location {
    self.token_storage[self.next_next_idx].location
  }

  /// 向前看第三个 Token 的源码区间
  #[inline]
  #[must_use]
  pub fn peek_ahead_ahead_location(&self) -> Location {
    self.token_storage[self.next_next_next_idx].location
  }

  /// 向前看第二个 Token 描述体引用
  #[inline]
  #[must_use]
  pub fn next_next_desc(&self) -> &TokenDesc {
    &self.token_storage[self.next_next_idx]
  }

  /// 向前看第三个 Token 描述体引用
  #[inline]
  #[must_use]
  pub fn next_next_next_desc(&self) -> &TokenDesc {
    &self.token_storage[self.next_next_next_idx]
  }

  /// 当前源码偏移位置
  #[inline]
  #[must_use]
  pub fn source_pos(&self) -> usize {
    let p = self.source.pos();
    if self.c0 == K_END_OF_INPUT {
      p
    } else if p > 0 {
      p - 1
    } else {
      0
    }
  }

  /// 计算指定 offset 的源码坐标 (line, column)
  #[must_use]
  pub fn source_position(&self, offset: usize) -> SourcePosition {
    let line_idx = self.line_starts.partition_point(|&start| start <= offset);
    let line = line_idx.max(1);
    let line_start = self.line_starts[line - 1];
    let column = if offset >= line_start {
      offset - line_start + 1
    } else {
      1
    };
    SourcePosition::new(offset, line, column)
  }

  /// 当前 Token 起始处的源码坐标
  #[inline]
  #[must_use]
  pub fn current_source_position(&self) -> SourcePosition {
    self.source_position(self.location().beg_pos)
  }

  /// 检查紧随其后是否为 ',' 或 ']' (原版 HasImmediateCommaOrRightBracket)
  #[inline]
  #[must_use]
  pub fn has_immediate_comma_or_right_bracket(&self) -> bool {
    if self.has_parser_error() {
      return false;
    }
    self.token_storage[self.next_next_idx].token == Token::Uninitialized
      && (self.c0 == b',' as u32 || self.c0 == b']' as u32)
  }

  /// 是否存在词法错误
  #[inline]
  #[must_use]
  pub fn has_error(&self) -> bool {
    self.scanner_error != MessageTemplate::None
  }

  /// 获取词法错误信息
  #[inline]
  #[must_use]
  pub fn error(&self) -> MessageTemplate {
    self.scanner_error
  }

  /// 获取词法错误位置
  #[inline]
  #[must_use]
  pub fn error_location(&self) -> Location {
    self.scanner_error_location
  }

  /// 是否存在非法的模板字符串转义 (原版 HasInvalidTemplateEscape)
  #[inline]
  #[must_use]
  pub fn has_invalid_template_escape(&self) -> bool {
    self.current().token.is_template()
      && self.current().invalid_template_escape_message != MessageTemplate::None
  }

  /// 获取非法模板字符串转义错误信息 (原版 InvalidTemplateEscapeMessage)
  #[inline]
  #[must_use]
  pub fn invalid_template_escape_message(&self) -> MessageTemplate {
    if self.current().token.is_template() {
      self.current().invalid_template_escape_message
    } else {
      MessageTemplate::None
    }
  }

  /// 获取非法模板字符串转义错误位置 (原版 InvalidTemplateEscapeLocation)
  #[inline]
  #[must_use]
  pub fn invalid_template_escape_location(&self) -> Location {
    if self.current().token.is_template() {
      self.current().invalid_template_escape_location
    } else {
      Location::invalid()
    }
  }

  /// 清除非法模板字符串转义错误信息
  #[inline]
  pub fn clear_invalid_template_escape_message(&mut self) {
    self.token_storage[self.current_idx].invalid_template_escape_message = MessageTemplate::None;
  }

  /// 八进制字面量位置
  #[inline]
  #[must_use]
  pub fn octal_position(&self) -> Location {
    self.octal_pos
  }

  /// 清除八进制标记位置
  #[inline]
  pub fn clear_octal_position(&mut self) {
    self.octal_pos = Location::invalid();
    self.octal_message = MessageTemplate::None;
  }

  /// 获取八进制错误信息
  #[inline]
  #[must_use]
  pub fn octal_message(&self) -> MessageTemplate {
    self.octal_message
  }

  /// 最近扫描的 Smi 数值
  #[inline]
  #[must_use]
  pub fn smi_value(&self) -> u32 {
    self.current().smi_value
  }

  /// peek Token 前是否存在换行
  #[inline]
  #[must_use]
  pub fn has_line_terminator_before_next(&self) -> bool {
    self.next_desc().after_line_terminator
  }

  /// peek Token 后是否存在换行
  pub fn has_line_terminator_after_next(&mut self) -> bool {
    let _ = self.peek_ahead();
    self.token_storage[self.next_next_idx].after_line_terminator
  }

  /// peek_ahead Token 后是否存在换行
  pub fn has_line_terminator_after_next_next(&mut self) -> bool {
    let _ = self.peek_ahead_ahead();
    self.token_storage[self.next_next_next_idx].after_line_terminator
  }

  /// 是否扫描到 HTML 注释
  #[inline]
  #[must_use]
  pub const fn found_html_comment(&self) -> bool {
    self.found_html_comment
  }

  /// 扫描正则字面量主体 (下一字符必须为 '/')
  pub fn scan_regexp_pattern(&mut self) -> bool {
    debug_assert_eq!(
      Token::Uninitialized,
      self.token_storage[self.next_next_idx].token
    );
    debug_assert!(
      self.next_desc().token == Token::Div || self.next_desc().token == Token::AssignDiv
    );

    let mut in_character_class = false;
    let next = &mut self.token_storage[self.next_idx];
    next.literal_chars.start();
    if next.token == Token::AssignDiv {
      next.literal_chars.add_char(b'=' as u32);
    }

    while self.c0 != b'/' as u32 || in_character_class {
      if self.c0 == K_END_OF_INPUT || is_line_terminator(self.c0) {
        return false;
      }
      if self.c0 == b'\\' as u32 {
        self.add_literal_char_advance();
        if self.c0 == K_END_OF_INPUT || is_line_terminator(self.c0) {
          return false;
        }
        self.add_literal_char_advance();
      } else {
        if self.c0 == b'[' as u32 {
          in_character_class = true;
        }
        if self.c0 == b']' as u32 {
          in_character_class = false;
        }
        self.add_literal_char_advance();
      }
    }
    self.advance(); // 消费 '/'
    self.token_storage[self.next_idx].token = Token::RegExpLiteral;
    true
  }

  /// 扫描正则 Flags (对应原版 ScanRegExpFlags: d:7, g:0, i:1, m:2, s:5, u:4, v:8, y:3)
  pub fn scan_regexp_flags(&mut self) -> Option<u32> {
    debug_assert_eq!(Token::RegExpLiteral, self.next_desc().token);
    let mut flags: u32 = 0;
    self.token_storage[self.next_idx].literal_chars.start();
    while char_predicates::is_identifier_part(self.c0) {
      if self.c0 > 0x7F {
        return None;
      }
      let flag_bit: u32 = match self.c0 as u8 {
        b'd' => 1 << 7,
        b'g' => 1 << 0,
        b'i' => 1 << 1,
        b'm' => 1 << 2,
        b's' => 1 << 5,
        b'u' => 1 << 4,
        b'v' => 1 << 8,
        b'y' => 1 << 3,
        _ => return None,
      };
      if (flags & flag_bit) != 0 {
        return None;
      }
      flags |= flag_bit;
      self.add_literal_char_advance();
    }
    self.token_storage[self.next_idx].location.end_pos = self.source_pos();
    Some(flags)
  }

  /// 扫描模板字面量续接段 (原版 ScanTemplateContinuation)
  pub fn scan_template_continuation(&mut self) -> Token {
    debug_assert_eq!(self.next_desc().token, Token::RightBrace);
    self.scan_template_span()
  }

  /// 设置解析器错误状态
  pub fn set_parser_error(&mut self) {
    if !self.has_parser_error() {
      self.c0 = K_END_OF_INPUT;
      self.source.set_parser_error();
      for desc in &mut self.token_storage {
        if desc.token != Token::Uninitialized {
          desc.token = Token::Illegal;
        }
      }
    }
  }

  /// 重置解析器错误标志
  pub fn reset_parser_error_flag(&mut self) {
    self.source.reset_parser_error_flag();
  }

  /// 是否存在解析器错误
  #[inline]
  #[must_use]
  pub fn has_parser_error(&self) -> bool {
    self.source.has_parser_error()
  }

  /// 跳转至指定位置 (原版 SeekForward)
  pub fn seek_forward(&mut self, pos: usize) {
    if pos == self.next_desc().location.beg_pos {
      return;
    }
    let current_pos = self.source_pos();
    debug_assert!(pos >= current_pos);
    if pos != current_pos {
      self.source.seek(pos);
      self.advance();
      self.token_storage[self.next_idx].after_line_terminator = false;
    }
    self.token_storage[self.next_next_idx].token = Token::Uninitialized;
    self.token_storage[self.next_next_next_idx].token = Token::Uninitialized;
    self.scan();
  }

  /// 重定位并重新扫描指定位置 (Bookmark 应用时调用)
  pub fn seek_next(&mut self, position: usize) {
    for token in &mut self.token_storage {
      token.token = Token::Uninitialized;
      token.invalid_template_escape_message = MessageTemplate::None;
      token.invalid_template_escape_location = Location::invalid();
    }
    self.source.seek(position);
    self.c0 = self.source.advance();
    self.token_storage[self.next_idx].after_line_terminator = false;
    self.scan();
    debug_assert_eq!(self.next_desc().location.beg_pos, position);
  }

  /// 创建书签 Scope
  #[inline]
  #[must_use]
  pub fn new_bookmark(&self) -> BookmarkScope {
    BookmarkScope::new(self)
  }

  /// 应用书签 Scope
  #[inline]
  pub fn apply_bookmark(&mut self, bookmark: &mut BookmarkScope) {
    bookmark.apply(self);
  }

  /// 获取当前数值字面量为 f64 (原版 DoubleValue)
  #[must_use]
  pub fn double_value(&self) -> f64 {
    let chars = self.current().literal_chars.one_byte_literal();
    // SAFETY: 数字字面量均为 ASCII 字节
    let s = unsafe { from_utf8_unchecked(chars) };
    match self.current().number_kind {
      NumberKind::Binary => {
        let trimmed = strip_radix_prefix(s, b'b');
        parse_radix_f64(trimmed, 2)
      }
      NumberKind::Octal => {
        let trimmed = strip_radix_prefix(s, b'o');
        parse_radix_f64(trimmed, 8)
      }
      NumberKind::ImplicitOctal => parse_radix_f64(s, 8),
      NumberKind::Hex => {
        let trimmed = strip_radix_prefix(s, b'x');
        parse_radix_f64(trimmed, 16)
      }
      NumberKind::Decimal | NumberKind::DecimalWithLeadingZero => {
        s.parse::<f64>().unwrap_or(f64::NAN)
      }
    }
  }

  /// 获取 BigInt 字面量字节
  #[inline]
  #[must_use]
  pub fn bigint_literal(&self) -> &[u8] {
    self.current().literal_chars.one_byte_literal()
  }

  /// 获取当前 Token 的 AST 符号 (原版 CurrentSymbol)
  pub fn current_symbol(&self, factory: &mut AstValueFactory) -> AstRawString {
    self.current().literal_chars.internalize(factory)
  }

  /// 获取下一个 Token 的 AST 符号 (原版 NextSymbol)
  pub fn next_symbol(&self, factory: &mut AstValueFactory) -> AstRawString {
    self.next_desc().literal_chars.internalize(factory)
  }

  /// 获取当前 Token 的 Raw AST 符号 (用于模板字面量)
  pub fn current_raw_symbol(&self, factory: &mut AstValueFactory) -> AstRawString {
    self.current().raw_literal_chars.internalize(factory)
  }

  // ── 私有辅助方法 ──

  #[inline]
  fn current(&self) -> &TokenDesc {
    // SAFETY: current_idx 始终在 0..4 闭环内
    unsafe { self.token_storage.get_unchecked(self.current_idx) }
  }

  #[inline]
  fn next_desc(&self) -> &TokenDesc {
    // SAFETY: next_idx 始终在 0..4 闭环内
    unsafe { self.token_storage.get_unchecked(self.next_idx) }
  }

  #[inline]
  fn advance(&mut self) {
    self.c0 = self.source.advance();
  }

  #[inline]
  fn advance_capture_raw(&mut self) {
    self.token_storage[self.next_idx]
      .raw_literal_chars
      .add_char(self.c0);
    self.c0 = self.source.advance();
  }

  #[inline]
  fn push_back(&mut self, ch: u32) {
    self.source.back();
    self.c0 = ch;
  }

  #[inline]
  fn add_literal_char(&mut self, c: u32) {
    self.token_storage[self.next_idx].literal_chars.add_char(c);
  }

  #[inline]
  fn add_literal_ascii(&mut self, c: u8) {
    self.token_storage[self.next_idx]
      .literal_chars
      .add_ascii_char(c);
  }

  #[inline]
  fn add_literal_char_advance(&mut self) {
    let c = self.c0;
    self.add_literal_char(c);
    self.advance();
  }

  #[inline]
  fn select(&mut self, tok: Token) -> Token {
    self.advance();
    tok
  }

  #[inline]
  fn select_branch(&mut self, next_char: u32, then_tok: Token, else_tok: Token) -> Token {
    self.advance();
    if self.c0 == next_char {
      self.advance();
      then_tok
    } else {
      else_tok
    }
  }

  fn record_line_start(&mut self, pos: usize) {
    if let Some(&last) = self.line_starts.last()
      && pos <= last
    {
      // 如果因回退 (Seek/Bookmark) 重复扫描，确保单调有序且不重复
      if let Err(idx) = self.line_starts.binary_search(&pos) {
        self.line_starts.insert(idx, pos);
      }
      return;
    }
    self.line_starts.push(pos);
  }

  fn report_scanner_error(&mut self, pos: usize, error: MessageTemplate) {
    if !self.has_error() {
      self.scanner_error = error;
      self.scanner_error_location = Location::new(pos, pos + 1);
    }
  }

  fn report_scanner_error_loc(&mut self, loc: Location, error: MessageTemplate) {
    if !self.has_error() {
      self.scanner_error = error;
      self.scanner_error_location = loc;
    }
  }

  fn scan(&mut self) {
    let token = self.scan_single_token();
    let has_error = self.has_parser_error();
    let end_pos = self.source_pos();
    let desc = &mut self.token_storage[self.next_idx];
    desc.token = if has_error { Token::Illegal } else { token };
    desc.location.end_pos = end_pos;
    if !desc.token.is_template() {
      desc.invalid_template_escape_message = MessageTemplate::None;
      desc.invalid_template_escape_location = Location::invalid();
    }
  }

  fn scan_single_token(&mut self) -> Token {
    let mut token;
    loop {
      self.token_storage[self.next_idx].location.beg_pos = self.source_pos();

      if self.c0 <= K_MAX_ASCII {
        token = ONE_CHAR_TOKENS[self.c0 as usize];

        match token {
          Token::LeftParen
          | Token::RightParen
          | Token::LeftBrace
          | Token::RightBrace
          | Token::LeftBracket
          | Token::RightBracket
          | Token::Colon
          | Token::Semicolon
          | Token::Comma
          | Token::BitNot
          | Token::Illegal => {
            return self.select(token);
          }

          Token::Conditional => {
            self.advance();
            if self.c0 == b'.' as u32 {
              self.advance();
              if !char_predicates::is_decimal_digit(self.c0) {
                return Token::QuestionPeriod;
              }
              self.push_back(b'.' as u32);
            } else if self.c0 == b'?' as u32 {
              return self.select_branch(b'=' as u32, Token::AssignNullish, Token::Nullish);
            }
            return Token::Conditional;
          }

          Token::String => return self.scan_string(),

          Token::LessThan => {
            self.advance();
            if self.c0 == b'=' as u32 {
              return self.select(Token::LessThanEq);
            }
            if self.c0 == b'<' as u32 {
              return self.select_branch(b'=' as u32, Token::AssignShl, Token::Shl);
            }
            if self.c0 == b'!' as u32 {
              token = self.scan_html_comment();
              if token == Token::Whitespace {
                continue;
              }
              return token;
            }
            return Token::LessThan;
          }

          Token::GreaterThan => {
            self.advance();
            if self.c0 == b'=' as u32 {
              return self.select(Token::GreaterThanEq);
            }
            if self.c0 == b'>' as u32 {
              self.advance();
              if self.c0 == b'=' as u32 {
                return self.select(Token::AssignSar);
              }
              if self.c0 == b'>' as u32 {
                return self.select_branch(b'=' as u32, Token::AssignShr, Token::Shr);
              }
              return Token::Sar;
            }
            return Token::GreaterThan;
          }

          Token::Assign => {
            self.advance();
            if self.c0 == b'=' as u32 {
              return self.select_branch(b'=' as u32, Token::EqStrict, Token::Eq);
            }
            if self.c0 == b'>' as u32 {
              return self.select(Token::Arrow);
            }
            return Token::Assign;
          }

          Token::Not => {
            self.advance();
            if self.c0 == b'=' as u32 {
              return self.select_branch(b'=' as u32, Token::NotEqStrict, Token::NotEq);
            }
            return Token::Not;
          }

          Token::Add => {
            self.advance();
            if self.c0 == b'+' as u32 {
              return self.select(Token::Inc);
            }
            if self.c0 == b'=' as u32 {
              return self.select(Token::AssignAdd);
            }
            return Token::Add;
          }

          Token::Sub => {
            self.advance();
            if self.c0 == b'-' as u32 {
              self.advance();
              if self.c0 == b'>' as u32 && self.token_storage[self.next_idx].after_line_terminator {
                token = self.skip_single_html_comment();
                if token == Token::Whitespace {
                  continue;
                }
                return token;
              }
              return Token::Dec;
            }
            if self.c0 == b'=' as u32 {
              return self.select(Token::AssignSub);
            }
            return Token::Sub;
          }

          Token::Mul => {
            self.advance();
            if self.c0 == b'*' as u32 {
              return self.select_branch(b'=' as u32, Token::AssignExp, Token::Exp);
            }
            if self.c0 == b'=' as u32 {
              return self.select(Token::AssignMul);
            }
            return Token::Mul;
          }

          Token::Mod => {
            return self.select_branch(b'=' as u32, Token::AssignMod, Token::Mod);
          }

          Token::Div => {
            self.advance();
            if self.c0 == b'/' as u32 {
              let _ = self.skip_single_line_comment();
              continue;
            }
            if self.c0 == b'*' as u32 {
              token = self.skip_multi_line_comment();
              if token == Token::Whitespace {
                continue;
              }
              return token;
            }
            if self.c0 == b'=' as u32 {
              return self.select(Token::AssignDiv);
            }
            return Token::Div;
          }

          Token::BitAnd => {
            self.advance();
            if self.c0 == b'&' as u32 {
              return self.select_branch(b'=' as u32, Token::AssignAnd, Token::And);
            }
            if self.c0 == b'=' as u32 {
              return self.select(Token::AssignBitAnd);
            }
            return Token::BitAnd;
          }

          Token::BitOr => {
            self.advance();
            if self.c0 == b'|' as u32 {
              return self.select_branch(b'=' as u32, Token::AssignOr, Token::Or);
            }
            if self.c0 == b'=' as u32 {
              return self.select(Token::AssignBitOr);
            }
            return Token::BitOr;
          }

          Token::BitXor => {
            return self.select_branch(b'=' as u32, Token::AssignBitXor, Token::BitXor);
          }

          Token::Period => {
            self.advance();
            if char_predicates::is_decimal_digit(self.c0) {
              return self.scan_number(true);
            }
            if self.c0 == b'.' as u32 && self.source.peek() == b'.' as u32 {
              self.advance();
              self.advance();
              return Token::Ellipsis;
            }
            return Token::Period;
          }

          Token::TemplateSpan => {
            self.advance();
            return self.scan_template_span();
          }

          Token::PrivateName => {
            if self.source_pos() == 0 && self.source.peek() == b'!' as u32 {
              let _ = self.skip_single_line_comment();
              continue;
            }
            return self.scan_private_name();
          }

          Token::Whitespace => {
            let _ = self.skip_white_space();
            continue;
          }

          Token::Number => return self.scan_number(false),

          Token::Identifier => return self.scan_identifier_or_keyword(),

          _ => return Token::Illegal,
        }
      }

      if char_predicates::is_identifier_start(self.c0)
        || (self.combine_surrogate_pair() && char_predicates::is_identifier_start(self.c0))
      {
        return self.scan_identifier_or_keyword();
      }

      if self.c0 == K_END_OF_INPUT {
        return if self.source.has_parser_error() {
          Token::Illegal
        } else {
          Token::Eos
        };
      }

      token = self.skip_white_space();
      if token == Token::Whitespace {
        continue;
      }
      return token;
    }
  }

  fn combine_surrogate_pair(&mut self) -> bool {
    if utf16::is_lead_surrogate(self.c0 as i32) {
      let c1 = self.source.advance();
      if utf16::is_trail_surrogate(c1 as i32) {
        self.c0 = utf16::combine_surrogate_pair(self.c0, c1);
        return true;
      }
      self.source.back();
    }
    false
  }

  #[inline]
  fn check_line_terminator(&mut self, c: u32) {
    if is_line_terminator(c) {
      self.token_storage[self.next_idx].after_line_terminator = true;
      if c == b'\r' as u32 && self.source.peek() == b'\n' as u32 {
        // \r\n will be handled when \n is advanced
      } else {
        self.record_line_start(self.source_pos() + 1);
      }
    }
  }

  fn skip_white_space(&mut self) -> Token {
    if !char_predicates::is_white_space_or_line_terminator(self.c0) {
      return Token::Illegal;
    }

    self.check_line_terminator(self.c0);

    let mut hint = b' ' as u32;
    loop {
      self.advance();
      let c = self.c0;
      if c == hint {
        continue;
      }
      if char_predicates::is_white_space_or_line_terminator(c) {
        self.check_line_terminator(c);
        hint = c;
        continue;
      }
      break;
    }

    Token::Whitespace
  }

  fn skip_single_html_comment(&mut self) -> Token {
    if self.flags.is_module {
      self.report_scanner_error(self.source_pos(), MessageTemplate::HtmlCommentInModule);
      return Token::Illegal;
    }
    self.skip_single_line_comment()
  }

  fn skip_single_line_comment(&mut self) -> Token {
    while self.c0 != K_END_OF_INPUT && !is_line_terminator(self.c0) {
      self.advance();
    }
    Token::Whitespace
  }

  fn skip_multi_line_comment(&mut self) -> Token {
    debug_assert_eq!(self.c0, b'*' as u32);
    self.advance();

    while self.c0 != K_END_OF_INPUT {
      self.check_line_terminator(self.c0);
      if self.c0 == b'*' as u32 {
        self.advance();
        if self.c0 == b'/' as u32 {
          self.advance();
          return Token::Whitespace;
        }
      } else {
        self.advance();
      }
    }

    Token::Illegal
  }

  fn scan_html_comment(&mut self) -> Token {
    debug_assert_eq!(self.c0, b'!' as u32);
    self.advance();
    if self.c0 != b'-' as u32 || self.source.peek() != b'-' as u32 {
      self.push_back(b'!' as u32);
      return Token::LessThan;
    }
    self.advance(); // 消费第二个 '-'
    self.found_html_comment = true;
    self.skip_single_html_comment()
  }

  fn scan_string(&mut self) -> Token {
    let quote = self.c0;
    self.token_storage[self.next_idx].literal_chars.start();
    self.advance(); // 消费开始引号

    while self.c0 != quote {
      // 快速路径: 普通可打印 ASCII 字符直接快速追加 (排除转义 '\' 与结束引号)
      if self.c0 >= 0x20 && self.c0 <= 0x7E && self.c0 != b'\\' as u32 {
        let b = self.c0 as u8;
        self.advance();
        self.add_literal_ascii(b);
        continue;
      }
      if self.c0 == K_END_OF_INPUT || is_string_literal_line_terminator(self.c0) {
        return Token::Illegal;
      }
      if self.c0 == b'\\' as u32 {
        self.advance();
        if self.c0 == K_END_OF_INPUT || !self.scan_escape::<false>() {
          return Token::Illegal;
        }
      } else {
        self.add_literal_char_advance();
      }
    }
    self.advance(); // 消费闭合引号
    Token::String
  }

  fn scan_private_name(&mut self) -> Token {
    debug_assert_eq!(self.c0, b'#' as u32);
    self.token_storage[self.next_idx].literal_chars.start();
    let pos = self.source_pos();
    self.advance();

    if char_predicates::is_identifier_start(self.c0)
      || (self.combine_surrogate_pair() && char_predicates::is_identifier_start(self.c0))
    {
      self.add_literal_char(b'#' as u32);
      let tok = self.scan_identifier_or_keyword_inner();
      if tok == Token::Illegal {
        Token::Illegal
      } else {
        Token::PrivateName
      }
    } else {
      self.report_scanner_error(pos, MessageTemplate::InvalidOrUnexpectedToken);
      Token::Illegal
    }
  }

  fn scan_template_span(&mut self) -> Token {
    let old_scanner_error = self.scanner_error;
    let old_scanner_error_location = self.scanner_error_location;
    let old_octal_message = self.octal_message;
    let old_octal_pos = self.octal_pos;
    self.scanner_error = MessageTemplate::None;
    self.scanner_error_location = Location::invalid();
    self.octal_message = MessageTemplate::None;
    self.octal_pos = Location::invalid();

    let mut result = Token::TemplateSpan;
    self.token_storage[self.next_idx].invalid_template_escape_message = MessageTemplate::None;
    self.token_storage[self.next_idx].invalid_template_escape_location = Location::invalid();
    self.token_storage[self.next_idx].literal_chars.start();
    self.token_storage[self.next_idx].raw_literal_chars.start();

    loop {
      let c = self.c0;
      if c == b'`' as u32 {
        self.advance();
        result = Token::TemplateTail;
        break;
      }
      if c == b'$' as u32 && self.source.peek() == b'{' as u32 {
        self.advance();
        self.advance();
        break;
      }
      if c == b'\\' as u32 {
        self.advance();
        self.token_storage[self.next_idx]
          .raw_literal_chars
          .add_char(b'\\' as u32);
        if is_line_terminator(self.c0) {
          let mut last_char = self.c0;
          self.advance();
          if last_char == b'\r' as u32 {
            if self.c0 == b'\n' as u32 {
              self.advance();
            }
            last_char = b'\n' as u32;
          }
          self.record_line_start(self.source_pos());
          self.token_storage[self.next_idx]
            .raw_literal_chars
            .add_char(last_char);
        } else {
          let _ = self.scan_escape::<true>();
          if self.scanner_error != MessageTemplate::None {
            if self.token_storage[self.next_idx].invalid_template_escape_message
              == MessageTemplate::None
            {
              self.token_storage[self.next_idx].invalid_template_escape_message =
                self.scanner_error;
              self.token_storage[self.next_idx].invalid_template_escape_location =
                self.scanner_error_location;
            }
            self.scanner_error = MessageTemplate::None;
            self.scanner_error_location = Location::invalid();
          }
          if self.octal_message != MessageTemplate::None {
            if self.token_storage[self.next_idx].invalid_template_escape_message
              == MessageTemplate::None
            {
              self.token_storage[self.next_idx].invalid_template_escape_message =
                self.octal_message;
              self.token_storage[self.next_idx].invalid_template_escape_location = self.octal_pos;
            }
            self.octal_message = MessageTemplate::None;
            self.octal_pos = Location::invalid();
          }
        }
      } else if c == K_END_OF_INPUT {
        break;
      } else {
        self.advance();
        let mut cooked = c;
        if c == b'\r' as u32 {
          if self.c0 == b'\n' as u32 {
            self.advance();
          }
          self.record_line_start(self.source_pos());
          cooked = b'\n' as u32;
        } else if is_line_terminator(c) {
          self.record_line_start(self.source_pos());
        }
        self.token_storage[self.next_idx]
          .raw_literal_chars
          .add_char(cooked);
        self.add_literal_char(cooked);
      }
    }

    self.scanner_error = old_scanner_error;
    self.scanner_error_location = old_scanner_error_location;
    self.octal_message = old_octal_message;
    self.octal_pos = old_octal_pos;
    self.token_storage[self.next_idx].location.end_pos = self.source_pos();
    self.token_storage[self.next_idx].token = result;
    result
  }

  fn scan_escape<const CAPTURE_RAW: bool>(&mut self) -> bool {
    let mut c = self.c0;
    if CAPTURE_RAW {
      self.advance_capture_raw();
    } else {
      self.advance();
    }

    if !CAPTURE_RAW && is_line_terminator(c) {
      if c == b'\r' as u32 && self.c0 == b'\n' as u32 {
        self.advance();
      }
      self.record_line_start(self.source_pos());
      return true;
    }

    if c <= 0x7F {
      match c as u8 {
        b'b' => c = 0x08,
        b'f' => c = 0x0C,
        b'n' => c = 0x0A,
        b'r' => c = 0x0D,
        b't' => c = 0x09,
        b'v' => c = 0x0B,
        b'u' => {
          c = self.scan_unicode_escape::<CAPTURE_RAW>();
          if c == K_END_OF_INPUT {
            return false;
          }
        }
        b'x' => {
          c = self.scan_hex_number::<CAPTURE_RAW>(2, false);
          if c == K_END_OF_INPUT {
            return false;
          }
        }
        b'0'..=b'7' => {
          c = self.scan_octal_escape::<CAPTURE_RAW>(c, 2);
        }
        b'8' | b'9' => {
          self.octal_pos = Location::new(self.source_pos() - 2, self.source_pos() - 1);
          self.octal_message = if CAPTURE_RAW {
            MessageTemplate::Template8Or9Escape
          } else {
            MessageTemplate::Strict8Or9Escape
          };
        }
        _ => {}
      }
    }

    self.add_literal_char(c);
    true
  }

  fn scan_octal_escape<const CAPTURE_RAW: bool>(&mut self, c: u32, length: usize) -> u32 {
    let mut x = c - b'0' as u32;
    let mut i = 0;
    while i < length {
      if !char_predicates::is_octal_digit(self.c0) {
        break;
      }
      let d = self.c0 - b'0' as u32;
      let nx = x * 8 + d;
      if nx >= 256 {
        break;
      }
      x = nx;
      if CAPTURE_RAW {
        self.advance_capture_raw();
      } else {
        self.advance();
      }
      i += 1;
    }

    if c != b'0' as u32 || i > 0 || char_predicates::is_non_octal_decimal_digit(self.c0) {
      self.octal_pos = Location::new(self.source_pos() - i - 1, self.source_pos() - 1);
      self.octal_message = if CAPTURE_RAW {
        MessageTemplate::TemplateOctalLiteral
      } else {
        MessageTemplate::StrictOctalEscape
      };
    }
    x
  }

  fn scan_hex_number<const CAPTURE_RAW: bool>(
    &mut self,
    expected_length: usize,
    unicode: bool,
  ) -> u32 {
    let begin = if self.source_pos() >= 2 {
      self.source_pos() - 2
    } else {
      0
    };
    let mut x = 0;
    for _ in 0..expected_length {
      let d = match char_predicates::hex_value(self.c0) {
        Some(v) => v,
        None => {
          self.report_scanner_error_loc(
            Location::new(begin, begin + expected_length + 2),
            if unicode {
              MessageTemplate::InvalidUnicodeEscapeSequence
            } else {
              MessageTemplate::InvalidHexEscapeSequence
            },
          );
          return K_END_OF_INPUT;
        }
      };
      x = x * 16 + d;
      if CAPTURE_RAW {
        self.advance_capture_raw();
      } else {
        self.advance();
      }
    }
    x
  }

  fn scan_unlimited_length_hex_number<const CAPTURE_RAW: bool>(
    &mut self,
    max_value: u32,
    beg_pos: usize,
  ) -> u32 {
    let mut x: u32 = 0;
    let mut d = char_predicates::hex_value(self.c0);
    if d.is_none() {
      return K_END_OF_INPUT;
    }
    while let Some(val) = d {
      x = match x.checked_mul(16).and_then(|v| v.checked_add(val)) {
        Some(nx) if nx <= max_value => nx,
        _ => {
          self.report_scanner_error_loc(
            Location::new(beg_pos, self.source_pos() + 1),
            MessageTemplate::UndefinedUnicodeCodePoint,
          );
          return K_END_OF_INPUT;
        }
      };
      if CAPTURE_RAW {
        self.advance_capture_raw();
      } else {
        self.advance();
      }
      d = char_predicates::hex_value(self.c0);
    }
    x
  }

  fn scan_unicode_escape<const CAPTURE_RAW: bool>(&mut self) -> u32 {
    if self.c0 == b'{' as u32 {
      let begin = if self.source_pos() >= 2 {
        self.source_pos() - 2
      } else {
        0
      };
      if CAPTURE_RAW {
        self.advance_capture_raw();
      } else {
        self.advance();
      }
      let cp = self.scan_unlimited_length_hex_number::<CAPTURE_RAW>(0x10FFFF, begin);
      if cp == K_END_OF_INPUT || self.c0 != b'}' as u32 {
        self.report_scanner_error(
          self.source_pos(),
          MessageTemplate::InvalidUnicodeEscapeSequence,
        );
        return K_END_OF_INPUT;
      }
      if CAPTURE_RAW {
        self.advance_capture_raw();
      } else {
        self.advance();
      }
      cp
    } else {
      self.scan_hex_number::<CAPTURE_RAW>(4, true)
    }
  }

  fn scan_identifier_unicode_escape(&mut self) -> u32 {
    self.advance();
    if self.c0 != b'u' as u32 {
      return K_END_OF_INPUT;
    }
    self.advance();
    self.scan_unicode_escape::<false>()
  }

  fn scan_identifier_or_keyword(&mut self) -> Token {
    self.token_storage[self.next_idx].literal_chars.start();
    self.scan_identifier_or_keyword_inner()
  }

  fn scan_identifier_or_keyword_inner(&mut self) -> Token {
    debug_assert!(char_predicates::is_identifier_start(self.c0));
    let mut escaped = false;
    let mut can_be_keyword = true;

    if self.c0 <= K_MAX_ASCII {
      if self.c0 != b'\\' as u32 {
        let scan_flags_val = CHARACTER_SCAN_FLAGS[self.c0 as usize];
        let mut scan_flags_shifted = scan_flags_val >> 1;
        self.add_literal_ascii(self.c0 as u8);

        loop {
          self.advance();
          let c = self.c0;
          if c > K_MAX_ASCII {
            scan_flags_shifted |= scan_flags::K_IDENTIFIER_NEEDS_SLOW_PATH;
            break;
          }
          let char_flags = CHARACTER_SCAN_FLAGS[c as usize];
          scan_flags_shifted |= char_flags;
          if scan_flags::terminates_literal(char_flags) {
            break;
          }
          self.add_literal_ascii(c as u8);
        }

        if !scan_flags::identifier_needs_slow_path(scan_flags_shifted) {
          if !scan_flags::can_be_keyword(scan_flags_shifted) {
            return Token::Identifier;
          }
          let chars = self.token_storage[self.next_idx]
            .literal_chars
            .one_byte_literal();
          // SAFETY: 快速路径已保证字符均 <= K_MAX_ASCII，为合法 ASCII
          let s = unsafe { from_utf8_unchecked(chars) };
          if let Some(token) = lookup_keyword(s) {
            return token;
          }
          return Token::Identifier;
        }

        can_be_keyword = scan_flags::can_be_keyword(scan_flags_shifted);
      } else {
        escaped = true;
        let c = self.scan_identifier_unicode_escape();
        if c == K_END_OF_INPUT || c == b'\\' as u32 || !char_predicates::is_identifier_start(c) {
          return Token::Illegal;
        }
        self.add_literal_char(c);
        can_be_keyword =
          c <= K_MAX_ASCII && scan_flags::can_be_keyword(CHARACTER_SCAN_FLAGS[c as usize]);
      }
    }

    self.scan_identifier_or_keyword_inner_slow(escaped, can_be_keyword)
  }

  fn scan_identifier_or_keyword_inner_slow(
    &mut self,
    mut escaped: bool,
    mut can_be_keyword: bool,
  ) -> Token {
    loop {
      if self.c0 == b'\\' as u32 {
        escaped = true;
        let c = self.scan_identifier_unicode_escape();
        if c == K_END_OF_INPUT || c == b'\\' as u32 || !char_predicates::is_identifier_part(c) {
          return Token::Illegal;
        }
        can_be_keyword = can_be_keyword
          && c <= K_MAX_ASCII
          && scan_flags::can_be_keyword(CHARACTER_SCAN_FLAGS[c as usize]);
        self.add_literal_char(c);
      } else if char_predicates::is_identifier_part(self.c0)
        || (self.combine_surrogate_pair() && char_predicates::is_identifier_part(self.c0))
      {
        can_be_keyword = can_be_keyword
          && self.c0 <= K_MAX_ASCII
          && scan_flags::can_be_keyword(CHARACTER_SCAN_FLAGS[self.c0 as usize]);
        self.add_literal_char_advance();
      } else {
        break;
      }
    }

    if can_be_keyword
      && self.token_storage[self.next_idx]
        .literal_chars
        .is_one_byte()
    {
      let chars = self.token_storage[self.next_idx]
        .literal_chars
        .one_byte_literal();
      // SAFETY: can_be_keyword 为 true 保证全为 ASCII 字符
      let s = unsafe { from_utf8_unchecked(chars) };
      if let Some(token) = lookup_keyword(s) {
        if (token as u8) >= (Token::Identifier as u8) && (token as u8) <= (Token::Yield as u8) {
          return token;
        }
        if token == Token::FutureStrictReservedWord {
          return if escaped {
            Token::EscapedStrictReservedWord
          } else {
            token
          };
        }
        if !escaped {
          return token;
        }
        if matches!(token, Token::Let | Token::Static) {
          return Token::EscapedStrictReservedWord;
        }
        return Token::EscapedKeyword;
      }
    }

    Token::Identifier
  }

  fn scan_number(&mut self, mut seen_period: bool) -> Token {
    let mut kind = NumberKind::Decimal;
    self.token_storage[self.next_idx].literal_chars.start();
    let mut at_start = !seen_period;
    let start_pos = self.source_pos();

    if seen_period {
      self.add_literal_char(b'.' as u32);
      if self.c0 == b'_' as u32 {
        return Token::Illegal;
      }
      if !self.scan_decimal_digits(true) {
        return Token::Illegal;
      }
    } else if self.c0 == b'0' as u32 {
      self.add_literal_char_advance();

      if (self.c0 | 0x20) == b'x' as u32 {
        self.add_literal_char_advance();
        kind = NumberKind::Hex;
        if !self.scan_hex_digits() {
          return Token::Illegal;
        }
      } else if (self.c0 | 0x20) == b'o' as u32 {
        self.add_literal_char_advance();
        kind = NumberKind::Octal;
        if !self.scan_octal_digits() {
          return Token::Illegal;
        }
      } else if (self.c0 | 0x20) == b'b' as u32 {
        self.add_literal_char_advance();
        kind = NumberKind::Binary;
        if !self.scan_binary_digits() {
          return Token::Illegal;
        }
      } else if char_predicates::is_octal_digit(self.c0) {
        kind = NumberKind::ImplicitOctal;
        if !self.scan_implicit_octal_digits(start_pos, &mut kind) {
          return Token::Illegal;
        }
        if kind == NumberKind::DecimalWithLeadingZero {
          at_start = false;
        }
      } else if char_predicates::is_non_octal_decimal_digit(self.c0) {
        kind = NumberKind::DecimalWithLeadingZero;
      } else if self.c0 == b'_' as u32 {
        self.report_scanner_error(
          self.source_pos(),
          MessageTemplate::ZeroDigitNumericSeparator,
        );
        return Token::Illegal;
      }
    }

    if kind.is_decimal_number_kind() {
      let allow_numeric_separator = kind != NumberKind::DecimalWithLeadingZero;
      if at_start {
        let mut value: u64 = 0;
        if !self.scan_decimal_as_smi(&mut value, allow_numeric_separator) {
          return Token::Illegal;
        }

        const SMI_MAX: u64 = 0x3fff_ffff; // 1 << 30 - 1
        if self.token_storage[self.next_idx]
          .literal_chars
          .one_byte_literal()
          .len()
          <= 10
          && value <= SMI_MAX
          && self.c0 != b'.' as u32
          && !char_predicates::is_identifier_start(self.c0)
        {
          self.token_storage[self.next_idx].smi_value = value as u32;
          if kind == NumberKind::DecimalWithLeadingZero {
            self.octal_pos = Location::new(start_pos, self.source_pos());
            self.octal_message = MessageTemplate::StrictDecimalWithLeadingZero;
          }
          self.token_storage[self.next_idx].number_kind = kind;
          return Token::Smi;
        }
      }

      if !self.scan_decimal_digits(allow_numeric_separator) {
        return Token::Illegal;
      }
      if self.c0 == b'.' as u32 {
        seen_period = true;
        self.add_literal_char_advance();
        if self.c0 == b'_' as u32 || !self.scan_decimal_digits(true) {
          return Token::Illegal;
        }
      }
    }

    let mut is_bigint = false;
    if self.c0 == b'n' as u32 && !seen_period && kind.is_valid_bigint_kind() {
      // 检查 BigInt 字面量长度是否超过上限 (BigInt::kMaxBits / 4 = 262144)
      const K_MAX_BIG_INT_CHARACTERS: usize = (1 << 20) / 4;
      let prefix_len = if kind != NumberKind::Decimal { 2 } else { 0 };
      let length = self
        .source_pos()
        .saturating_sub(start_pos)
        .saturating_sub(prefix_len);
      if length > K_MAX_BIG_INT_CHARACTERS {
        self.report_scanner_error_loc(
          Location::new(start_pos, self.source_pos()),
          MessageTemplate::BigIntTooBig,
        );
        return Token::Illegal;
      }
      is_bigint = true;
      self.advance();
    } else if (self.c0 | 0x20) == b'e' as u32 {
      if !kind.is_decimal_number_kind() {
        return Token::Illegal;
      }
      self.add_literal_char_advance();
      if !self.scan_signed_integer() {
        return Token::Illegal;
      }
    }

    if char_predicates::is_decimal_digit(self.c0) || char_predicates::is_identifier_start(self.c0) {
      return Token::Illegal;
    }

    if kind == NumberKind::DecimalWithLeadingZero {
      self.octal_pos = Location::new(start_pos, self.source_pos());
      self.octal_message = MessageTemplate::StrictDecimalWithLeadingZero;
    }

    self.token_storage[self.next_idx].number_kind = kind;
    if is_bigint {
      Token::BigInt
    } else {
      Token::Number
    }
  }

  fn scan_digits_with_numeric_separators<P>(&mut self, predicate: P, check_first: bool) -> bool
  where
    P: Fn(u32) -> bool,
  {
    if check_first && !predicate(self.c0) {
      return false;
    }

    let mut separator_seen = false;
    while predicate(self.c0) || self.c0 == b'_' as u32 {
      if self.c0 == b'_' as u32 {
        self.advance();
        if self.c0 == b'_' as u32 {
          self.report_scanner_error(
            self.source_pos(),
            MessageTemplate::ContinuousNumericSeparator,
          );
          return false;
        }
        separator_seen = true;
        continue;
      }
      separator_seen = false;
      self.add_literal_char_advance();
    }

    if separator_seen {
      self.report_scanner_error(self.source_pos(), MessageTemplate::TrailingNumericSeparator);
      return false;
    }

    true
  }

  fn scan_decimal_digits(&mut self, allow_separator: bool) -> bool {
    if allow_separator {
      self.scan_digits_with_numeric_separators(char_predicates::is_decimal_digit, false)
    } else {
      while char_predicates::is_decimal_digit(self.c0) {
        self.add_literal_char_advance();
      }
      if self.c0 == b'_' as u32 {
        self.report_scanner_error(self.source_pos(), MessageTemplate::InvalidOrUnexpectedToken);
        return false;
      }
      true
    }
  }

  fn scan_decimal_as_smi(&mut self, value: &mut u64, allow_separator: bool) -> bool {
    if allow_separator {
      let mut separator_seen = false;
      while char_predicates::is_decimal_digit(self.c0) || self.c0 == b'_' as u32 {
        if self.c0 == b'_' as u32 {
          self.advance();
          if self.c0 == b'_' as u32 {
            self.report_scanner_error(
              self.source_pos(),
              MessageTemplate::ContinuousNumericSeparator,
            );
            return false;
          }
          separator_seen = true;
          continue;
        }
        separator_seen = false;
        *value = (*value)
          .wrapping_mul(10)
          .wrapping_add((self.c0 - b'0' as u32) as u64);
        let first_char = self.c0 as u8;
        self.advance();
        self.add_literal_ascii(first_char);
      }
      if separator_seen {
        self.report_scanner_error(self.source_pos(), MessageTemplate::TrailingNumericSeparator);
        return false;
      }
    } else {
      while char_predicates::is_decimal_digit(self.c0) {
        *value = (*value)
          .wrapping_mul(10)
          .wrapping_add((self.c0 - b'0' as u32) as u64);
        let first_char = self.c0 as u8;
        self.advance();
        self.add_literal_ascii(first_char);
      }
    }
    true
  }

  fn scan_hex_digits(&mut self) -> bool {
    self.scan_digits_with_numeric_separators(char_predicates::is_hex_digit, true)
  }

  fn scan_binary_digits(&mut self) -> bool {
    self.scan_digits_with_numeric_separators(char_predicates::is_binary_digit, true)
  }

  fn scan_octal_digits(&mut self) -> bool {
    self.scan_digits_with_numeric_separators(char_predicates::is_octal_digit, true)
  }

  fn scan_implicit_octal_digits(&mut self, start_pos: usize, kind: &mut NumberKind) -> bool {
    loop {
      if char_predicates::is_non_octal_decimal_digit(self.c0) {
        *kind = NumberKind::DecimalWithLeadingZero;
        return true;
      }
      if !char_predicates::is_octal_digit(self.c0) {
        self.octal_pos = Location::new(start_pos, self.source_pos());
        self.octal_message = MessageTemplate::StrictOctalLiteral;
        return true;
      }
      self.add_literal_char_advance();
    }
  }

  fn scan_signed_integer(&mut self) -> bool {
    if self.c0 == b'+' as u32 || self.c0 == b'-' as u32 {
      self.add_literal_char_advance();
    }
    if !char_predicates::is_decimal_digit(self.c0) {
      return false;
    }
    self.scan_decimal_digits(true)
  }
}

impl<S: CharacterStream> Iterator for Scanner<S> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    let tok = self.next_token();
    if tok == Token::Eos { None } else { Some(tok) }
  }
}

/// 扫描器书签 Scope (原版 Scanner::BookmarkScope)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BookmarkScope {
  bookmark: usize,
  had_parser_error: bool,
}

impl BookmarkScope {
  pub const NO_BOOKMARK: usize = usize::MAX - 1;
  pub const BOOKMARK_WAS_APPLIED: usize = usize::MAX;

  #[inline]
  #[must_use]
  pub fn new<S: CharacterStream>(scanner: &Scanner<S>) -> Self {
    Self {
      bookmark: Self::NO_BOOKMARK,
      had_parser_error: scanner.has_parser_error(),
    }
  }

  #[inline]
  pub fn set(&mut self, position: usize) {
    debug_assert_eq!(self.bookmark, Self::NO_BOOKMARK);
    self.bookmark = position;
  }

  #[inline]
  #[must_use]
  pub fn has_been_set(&self) -> bool {
    self.bookmark != Self::NO_BOOKMARK && self.bookmark != Self::BOOKMARK_WAS_APPLIED
  }

  #[inline]
  #[must_use]
  pub fn has_been_applied(&self) -> bool {
    self.bookmark == Self::BOOKMARK_WAS_APPLIED
  }

  #[inline]
  pub fn apply<S: CharacterStream>(&mut self, scanner: &mut Scanner<S>) {
    debug_assert!(self.has_been_set());
    if self.had_parser_error {
      scanner.set_parser_error();
    } else {
      scanner.reset_parser_error_flag();
      scanner.seek_next(self.bookmark);
    }
    self.bookmark = Self::BOOKMARK_WAS_APPLIED;
  }
}
