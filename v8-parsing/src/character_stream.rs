//! 字符流抽象与实现 (对应原版 v8/src/parsing/scanner-character-streams.*)

use crate::{
  literal_buffer::LiteralBuffer,
  scanner_generated::{K_END_OF_INPUT, Location},
};

/// UTF-16 字符流核心 Trait
pub trait CharacterStream {
  /// 当前代码单元位置 (0-based)
  fn pos(&self) -> usize;

  /// 向前看下一个 UTF-16 代码单元 (不消费), 若到达末尾返回 K_END_OF_INPUT
  fn peek(&mut self) -> u32;

  /// 消费并返回下一个 UTF-16 代码单元, 若到达末尾返回 K_END_OF_INPUT
  fn advance(&mut self) -> u32;

  /// 回退一个字符位置 (撤销最近一次 advance)
  fn back(&mut self);

  /// 跳转至指定字符位置
  fn seek(&mut self, pos: usize);

  /// 是否存在解析器错误状态
  fn has_parser_error(&self) -> bool;

  /// 设置解析器错误状态
  fn set_parser_error(&mut self);

  /// 重置解析器错误状态
  fn reset_parser_error_flag(&mut self);

  /// 字符流总长度 (UTF-16 代码单元数)
  fn len(&self) -> usize;

  /// 字符流是否为空
  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// 消费字符直到满足 check 或到达末尾, 返回终止字符
  fn advance_until<F>(&mut self, mut check: F) -> u32
  where
    F: FnMut(u32) -> bool,
  {
    loop {
      let c = self.peek();
      if c == K_END_OF_INPUT || check(c) {
        return c;
      }
      self.advance();
    }
  }

  /// 消费字符并向回调报告跳过区间 [start, end), 返回终止字符
  fn advance_until_range<F, R>(&mut self, mut check: F, mut on_range: R) -> u32
  where
    F: FnMut(u32) -> bool,
    R: FnMut(usize, usize),
  {
    let start = self.pos();
    let mut end = start;
    loop {
      let c = self.peek();
      if c == K_END_OF_INPUT || check(c) {
        if end > start {
          on_range(start, end);
        }
        return c;
      }
      self.advance();
      end = self.pos();
    }
  }

  /// 将 [start, end) 范围的字符切片写入字面量缓冲区
  fn copy_range_to_literal(&self, start: usize, end: usize, buffer: &mut LiteralBuffer);

  /// 获取 [start, end) 范围的 UTF-16 切片副本
  fn slice_utf16(&self, start: usize, end: usize) -> Vec<u16>;

  /// 获取指定区间的字符切片为 Location
  fn location_slice(&self, loc: Location) -> Vec<u16> {
    if loc.is_valid() {
      self.slice_utf16(loc.beg_pos, loc.end_pos)
    } else {
      Vec::new()
    }
  }
}

/// 单字节字符流 (Latin-1 / ASCII)
#[derive(Debug, Clone)]
pub struct OneByteCharacterStream<'a> {
  data: &'a [u8],
  pos: usize,
  has_error: bool,
}

impl<'a> OneByteCharacterStream<'a> {
  pub const fn new(data: &'a [u8]) -> Self {
    Self {
      data,
      pos: 0,
      has_error: false,
    }
  }
}

impl CharacterStream for OneByteCharacterStream<'_> {
  #[inline]
  fn pos(&self) -> usize {
    self.pos
  }

  #[inline]
  fn peek(&mut self) -> u32 {
    if self.has_error {
      return K_END_OF_INPUT;
    }
    match self.data.get(self.pos) {
      Some(&b) => b as u32,
      None => K_END_OF_INPUT,
    }
  }

  #[inline]
  fn advance(&mut self) -> u32 {
    if self.has_error {
      return K_END_OF_INPUT;
    }
    match self.data.get(self.pos) {
      Some(&b) => {
        self.pos += 1;
        b as u32
      }
      None => K_END_OF_INPUT,
    }
  }

  #[inline]
  fn back(&mut self) {
    if self.pos > 0 {
      self.pos -= 1;
    }
  }

  #[inline]
  fn seek(&mut self, pos: usize) {
    self.pos = pos.min(self.data.len());
  }

  #[inline]
  fn has_parser_error(&self) -> bool {
    self.has_error
  }

  #[inline]
  fn set_parser_error(&mut self) {
    self.has_error = true;
  }

  #[inline]
  fn reset_parser_error_flag(&mut self) {
    self.has_error = false;
  }

  #[inline]
  fn len(&self) -> usize {
    self.data.len()
  }

  #[inline]
  fn copy_range_to_literal(&self, start: usize, end: usize, buffer: &mut LiteralBuffer) {
    let s = start.min(self.data.len());
    let e = end.min(self.data.len());
    if e > s {
      buffer.add_range_from_u8(&self.data[s..e]);
    }
  }

  #[inline]
  fn slice_utf16(&self, start: usize, end: usize) -> Vec<u16> {
    let s = start.min(self.data.len());
    let e = end.min(self.data.len());
    if e > s {
      self.data[s..e].iter().map(|&b| b as u16).collect()
    } else {
      Vec::new()
    }
  }
}

/// 双字节字符流 (UTF-16 借用切片)
#[derive(Debug, Clone)]
pub struct TwoByteCharacterStream<'a> {
  data: &'a [u16],
  pos: usize,
  has_error: bool,
}

impl<'a> TwoByteCharacterStream<'a> {
  pub const fn new(data: &'a [u16]) -> Self {
    Self {
      data,
      pos: 0,
      has_error: false,
    }
  }
}

impl CharacterStream for TwoByteCharacterStream<'_> {
  #[inline]
  fn pos(&self) -> usize {
    self.pos
  }

  #[inline]
  fn peek(&mut self) -> u32 {
    if self.has_error {
      return K_END_OF_INPUT;
    }
    match self.data.get(self.pos) {
      Some(&c) => c as u32,
      None => K_END_OF_INPUT,
    }
  }

  #[inline]
  fn advance(&mut self) -> u32 {
    if self.has_error {
      return K_END_OF_INPUT;
    }
    match self.data.get(self.pos) {
      Some(&c) => {
        self.pos += 1;
        c as u32
      }
      None => K_END_OF_INPUT,
    }
  }

  #[inline]
  fn back(&mut self) {
    if self.pos > 0 {
      self.pos -= 1;
    }
  }

  #[inline]
  fn seek(&mut self, pos: usize) {
    self.pos = pos.min(self.data.len());
  }

  #[inline]
  fn has_parser_error(&self) -> bool {
    self.has_error
  }

  #[inline]
  fn set_parser_error(&mut self) {
    self.has_error = true;
  }

  #[inline]
  fn reset_parser_error_flag(&mut self) {
    self.has_error = false;
  }

  #[inline]
  fn len(&self) -> usize {
    self.data.len()
  }

  #[inline]
  fn copy_range_to_literal(&self, start: usize, end: usize, buffer: &mut LiteralBuffer) {
    let s = start.min(self.data.len());
    let e = end.min(self.data.len());
    if e > s {
      buffer.add_range_from_utf16(&self.data[s..e]);
    }
  }

  #[inline]
  fn slice_utf16(&self, start: usize, end: usize) -> Vec<u16> {
    let s = start.min(self.data.len());
    let e = end.min(self.data.len());
    if e > s {
      self.data[s..e].to_vec()
    } else {
      Vec::new()
    }
  }
}

/// 拥有所有权的双字节 UTF-16 字符流
#[derive(Debug, Clone)]
pub struct OwnedTwoByteCharacterStream {
  data: Vec<u16>,
  pos: usize,
  has_error: bool,
}

impl OwnedTwoByteCharacterStream {
  pub const fn new(data: Vec<u16>) -> Self {
    Self {
      data,
      pos: 0,
      has_error: false,
    }
  }
}

impl CharacterStream for OwnedTwoByteCharacterStream {
  #[inline]
  fn pos(&self) -> usize {
    self.pos
  }

  #[inline]
  fn peek(&mut self) -> u32 {
    if self.has_error {
      return K_END_OF_INPUT;
    }
    match self.data.get(self.pos) {
      Some(&c) => c as u32,
      None => K_END_OF_INPUT,
    }
  }

  #[inline]
  fn advance(&mut self) -> u32 {
    if self.has_error {
      return K_END_OF_INPUT;
    }
    match self.data.get(self.pos) {
      Some(&c) => {
        self.pos += 1;
        c as u32
      }
      None => K_END_OF_INPUT,
    }
  }

  #[inline]
  fn back(&mut self) {
    if self.pos > 0 {
      self.pos -= 1;
    }
  }

  #[inline]
  fn seek(&mut self, pos: usize) {
    self.pos = pos.min(self.data.len());
  }

  #[inline]
  fn has_parser_error(&self) -> bool {
    self.has_error
  }

  #[inline]
  fn set_parser_error(&mut self) {
    self.has_error = true;
  }

  #[inline]
  fn reset_parser_error_flag(&mut self) {
    self.has_error = false;
  }

  #[inline]
  fn len(&self) -> usize {
    self.data.len()
  }

  #[inline]
  fn copy_range_to_literal(&self, start: usize, end: usize, buffer: &mut LiteralBuffer) {
    let s = start.min(self.data.len());
    let e = end.min(self.data.len());
    if e > s {
      buffer.add_range_from_utf16(&self.data[s..e]);
    }
  }

  #[inline]
  fn slice_utf16(&self, start: usize, end: usize) -> Vec<u16> {
    let s = start.min(self.data.len());
    let e = end.min(self.data.len());
    if e > s {
      self.data[s..e].to_vec()
    } else {
      Vec::new()
    }
  }
}

/// 通用字符流统一枚举
#[derive(Debug, Clone)]
pub enum SourceStream<'a> {
  OneByte(OneByteCharacterStream<'a>),
  TwoByte(TwoByteCharacterStream<'a>),
  OwnedTwoByte(OwnedTwoByteCharacterStream),
}

impl<'a> SourceStream<'a> {
  /// 从 Rust 字符串切片自适应创建字符流
  /// ASCII 优先使用零分配单字节流, 包含高码点则转为 UTF-16 流
  pub fn from_source(s: &'a str) -> Self {
    if s.is_ascii() {
      Self::OneByte(OneByteCharacterStream::new(s.as_bytes()))
    } else {
      let utf16: Vec<u16> = s.encode_utf16().collect();
      Self::OwnedTwoByte(OwnedTwoByteCharacterStream::new(utf16))
    }
  }
}

impl CharacterStream for SourceStream<'_> {
  #[inline]
  fn pos(&self) -> usize {
    match self {
      Self::OneByte(s) => s.pos(),
      Self::TwoByte(s) => s.pos(),
      Self::OwnedTwoByte(s) => s.pos(),
    }
  }

  #[inline]
  fn peek(&mut self) -> u32 {
    match self {
      Self::OneByte(s) => s.peek(),
      Self::TwoByte(s) => s.peek(),
      Self::OwnedTwoByte(s) => s.peek(),
    }
  }

  #[inline]
  fn advance(&mut self) -> u32 {
    match self {
      Self::OneByte(s) => s.advance(),
      Self::TwoByte(s) => s.advance(),
      Self::OwnedTwoByte(s) => s.advance(),
    }
  }

  #[inline]
  fn back(&mut self) {
    match self {
      Self::OneByte(s) => s.back(),
      Self::TwoByte(s) => s.back(),
      Self::OwnedTwoByte(s) => s.back(),
    }
  }

  #[inline]
  fn seek(&mut self, pos: usize) {
    match self {
      Self::OneByte(s) => s.seek(pos),
      Self::TwoByte(s) => s.seek(pos),
      Self::OwnedTwoByte(s) => s.seek(pos),
    }
  }

  #[inline]
  fn has_parser_error(&self) -> bool {
    match self {
      Self::OneByte(s) => s.has_parser_error(),
      Self::TwoByte(s) => s.has_parser_error(),
      Self::OwnedTwoByte(s) => s.has_parser_error(),
    }
  }

  #[inline]
  fn set_parser_error(&mut self) {
    match self {
      Self::OneByte(s) => s.set_parser_error(),
      Self::TwoByte(s) => s.set_parser_error(),
      Self::OwnedTwoByte(s) => s.set_parser_error(),
    }
  }

  #[inline]
  fn reset_parser_error_flag(&mut self) {
    match self {
      Self::OneByte(s) => s.reset_parser_error_flag(),
      Self::TwoByte(s) => s.reset_parser_error_flag(),
      Self::OwnedTwoByte(s) => s.reset_parser_error_flag(),
    }
  }

  #[inline]
  fn len(&self) -> usize {
    match self {
      Self::OneByte(s) => s.len(),
      Self::TwoByte(s) => s.len(),
      Self::OwnedTwoByte(s) => s.len(),
    }
  }

  #[inline]
  fn copy_range_to_literal(&self, start: usize, end: usize, buffer: &mut LiteralBuffer) {
    match self {
      Self::OneByte(s) => s.copy_range_to_literal(start, end, buffer),
      Self::TwoByte(s) => s.copy_range_to_literal(start, end, buffer),
      Self::OwnedTwoByte(s) => s.copy_range_to_literal(start, end, buffer),
    }
  }

  #[inline]
  fn slice_utf16(&self, start: usize, end: usize) -> Vec<u16> {
    match self {
      Self::OneByte(s) => s.slice_utf16(start, end),
      Self::TwoByte(s) => s.slice_utf16(start, end),
      Self::OwnedTwoByte(s) => s.slice_utf16(start, end),
    }
  }
}

/// 测试用流工厂辅助 (对应原版 ScannerStream)
pub struct ScannerStream;

impl ScannerStream {
  #[inline]
  #[must_use]
  pub fn for_testing(src: &str) -> SourceStream<'_> {
    SourceStream::from_source(src)
  }

  #[inline]
  #[must_use]
  pub const fn for_one_byte(data: &[u8]) -> SourceStream<'_> {
    SourceStream::OneByte(OneByteCharacterStream::new(data))
  }

  #[inline]
  #[must_use]
  pub const fn for_two_byte(data: &[u16]) -> SourceStream<'_> {
    SourceStream::TwoByte(TwoByteCharacterStream::new(data))
  }
}
