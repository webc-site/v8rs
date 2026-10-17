//! AST 字符串与值表示 (v8/src/ast/ast-value-factory.h/.cc)

use core::convert::Infallible;
use std::{cmp::Ordering, fmt, str::FromStr};

use gxhash::HashSet;
use v8_numbers::conversions::is_special_index;
use v8_strings::string_hasher::RunningStringHasher;

use crate::value_generated::AstStringConstant;

/// 单字节/双字节 AST 裸字符串 (原版 AstRawString)
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AstRawString {
  literal_bytes: Vec<u8>,
  raw_hash_field: u32,
  is_one_byte: bool,
}

impl AstRawString {
  /// 从单字节切片或 ASCII/Latin-1 字符串构造
  pub fn from_one_byte(bytes: &[u8]) -> Self {
    let raw_hash = compute_one_byte_hash(bytes);
    Self {
      literal_bytes: bytes.to_vec(),
      raw_hash_field: raw_hash,
      is_one_byte: true,
    }
  }

  /// 从 UTF-8 字符串构造 (自动判断是否为单字节 Latin-1)
  pub fn from_utf8(s: &str) -> Self {
    if s.bytes().all(|b| b <= 0x7F) {
      Self::from_one_byte(s.as_bytes())
    } else {
      let utf16: Vec<u16> = s.encode_utf16().collect();
      Self::from_two_byte(&utf16)
    }
  }

  /// 从 UTF-16 切片构造
  pub fn from_two_byte(chars: &[u16]) -> Self {
    // 检查是否全为 Latin-1 (<= 0xFF)
    if chars.iter().all(|&c| c <= 0xFF) {
      let one_byte: Vec<u8> = chars.iter().map(|&c| c as u8).collect();
      return Self::from_one_byte(&one_byte);
    }
    let mut bytes = Vec::with_capacity(chars.len() * 2);
    for &c in chars {
      bytes.extend_from_slice(&c.to_le_bytes());
    }
    let raw_hash = compute_two_byte_hash(chars);
    Self {
      literal_bytes: bytes,
      raw_hash_field: raw_hash,
      is_one_byte: false,
    }
  }

  /// 字符长度 (单字节长度等于字节数, 双字节长度等于字符数)
  #[inline]
  pub fn length(&self) -> usize {
    if self.is_one_byte {
      self.literal_bytes.len()
    } else {
      self.literal_bytes.len() / 2
    }
  }

  /// 字节大小
  #[inline]
  pub fn byte_length(&self) -> usize {
    self.literal_bytes.len()
  }

  /// 是否为空串
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.literal_bytes.is_empty()
  }

  /// 是否为单字节 (Latin-1/ASCII)
  #[inline]
  pub fn is_one_byte(&self) -> bool {
    self.is_one_byte
  }

  /// 底层原始字节
  #[inline]
  pub fn raw_data(&self) -> &[u8] {
    &self.literal_bytes
  }

  /// 首字符 (代码点)
  pub fn first_character(&self) -> u16 {
    if self.literal_bytes.is_empty() {
      0
    } else if self.is_one_byte {
      self.literal_bytes[0] as u16
    } else {
      u16::from_le_bytes([self.literal_bytes[0], self.literal_bytes[1]])
    }
  }

  /// 是否为私有字段名 (形如 `#foo`)
  #[inline]
  pub fn is_private_name(&self) -> bool {
    self.length() > 0 && self.first_character() == b'#' as u16
  }

  /// 原始哈希位域
  #[inline]
  pub fn raw_hash_field(&self) -> u32 {
    self.raw_hash_field
  }

  /// 转换/解析为数组索引 (原版 AsArrayIndex)
  ///
  /// 仅单字节、十进制非负整数, 取值范围 `0..=4294967294`, 且无前导 0 (除非是 `"0"` 本身)。
  pub fn as_array_index(&self, index: &mut u32) -> bool {
    if !self.is_one_byte || self.literal_bytes.is_empty() {
      return false;
    }
    let bytes = &self.literal_bytes;
    if bytes[0] == b'0' && bytes.len() > 1 {
      return false;
    }
    let mut val: u64 = 0;
    for &b in bytes {
      if !b.is_ascii_digit() {
        return false;
      }
      val = val.wrapping_mul(10).wrapping_add((b - b'0') as u64);
      if val > 4294967294 {
        return false;
      }
    }
    *index = val as u32;
    true
  }

  /// 是否为特殊数组索引 (原版 IsSpecialIndex)
  pub fn is_special_index(&self) -> bool {
    if self.is_one_byte {
      is_special_index(&self.literal_bytes)
    } else {
      let len = self.length();
      if len == 0 || len > 24 {
        return false;
      }
      let mut buf = [0u16; 24];
      for (i, chunk) in self.literal_bytes.as_chunks::<2>().0.iter().enumerate() {
        buf[i] = u16::from_le_bytes(*chunk);
      }
      is_special_index(&buf[..len])
    }
  }

  /// 尝试转换为字符串
  pub fn to_string_lossy(&self) -> String {
    if self.is_one_byte {
      self.literal_bytes.iter().map(|&b| b as char).collect()
    } else {
      let u16s: Vec<u16> = self
        .literal_bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|c| u16::from_le_bytes(*c))
        .collect();
      String::from_utf16_lossy(&u16s)
    }
  }
}

impl From<&str> for AstRawString {
  fn from(s: &str) -> Self {
    Self::from_utf8(s)
  }
}

impl FromStr for AstRawString {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self::from_utf8(s))
  }
}

impl fmt::Debug for AstRawString {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "AstRawString({:?})", self.to_string_lossy())
  }
}

impl fmt::Display for AstRawString {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_string_lossy())
  }
}

impl PartialOrd for AstRawString {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for AstRawString {
  fn cmp(&self, other: &Self) -> Ordering {
    // 按码点序比较 (对应原版 AstRawString::Compare)
    let len1 = self.length();
    let len2 = other.length();
    let min_len = len1.min(len2);
    for i in 0..min_len {
      let c1 = self.char_at(i);
      let c2 = other.char_at(i);
      if c1 != c2 {
        return c1.cmp(&c2);
      }
    }
    len1.cmp(&len2)
  }
}

impl AstRawString {
  #[inline]
  fn char_at(&self, idx: usize) -> u16 {
    if self.is_one_byte {
      self.literal_bytes[idx] as u16
    } else {
      let pos = idx * 2;
      u16::from_le_bytes([self.literal_bytes[pos], self.literal_bytes[pos + 1]])
    }
  }
}

/// 计算单字节哈希值 (使用 v8-strings 标准哈希器)
fn compute_one_byte_hash(bytes: &[u8]) -> u32 {
  let mut hasher = RunningStringHasher::new(0);
  for &b in bytes {
    hasher.add_character(b as u16);
  }
  hasher.finalize()
}

/// 计算双字节哈希值 (使用 v8-strings 标准哈希器)
fn compute_two_byte_hash(chars: &[u16]) -> u32 {
  let mut hasher = RunningStringHasher::new(0);
  for &c in chars {
    hasher.add_character(c);
  }
  hasher.finalize()
}

/// 复合字符串 (原版 AstConsString)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AstConsString {
  segments: Vec<AstRawString>,
}

impl AstConsString {
  pub fn new() -> Self {
    Self {
      segments: Vec::new(),
    }
  }

  pub fn add_string(&mut self, s: AstRawString) -> &mut Self {
    if !s.is_empty() {
      self.segments.push(s);
    }
    self
  }

  pub fn is_empty(&self) -> bool {
    self.segments.is_empty() || self.segments.iter().all(|s| s.is_empty())
  }
}

impl fmt::Display for AstConsString {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for s in &self.segments {
      write!(f, "{}", s.to_string_lossy())?;
    }
    Ok(())
  }
}

/// BigInt 字面量串 (原版 AstBigInt)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstBigInt {
  digits: String,
}

impl AstBigInt {
  pub fn new(digits: impl Into<String>) -> Self {
    Self {
      digits: digits.into(),
    }
  }

  pub fn as_str(&self) -> &str {
    &self.digits
  }

  /// BigInt 在布尔上下文中是否为真值 (原版 ToBooleanIsTrue)
  ///
  /// `"0"`, `"0b0"`, `"0o0"`, `"0x0"`, `"0b000"` 等为 false, 其余非零为 true。
  pub fn to_boolean_is_true(&self) -> bool {
    let s = self.digits.as_bytes();
    if s.is_empty() {
      return false;
    }
    if s.len() == 1 && s[0] == b'0' {
      return false;
    }
    let start =
      if s[0] == b'0' && s.len() > 1 && matches!(s[1], b'b' | b'B' | b'o' | b'O' | b'x' | b'X') {
        2
      } else if s[0] == b'0' {
        // 兼容可能无基数前缀的 "000"
        1
      } else {
        0
      };
    for &b in &s[start..] {
      if b != b'0' {
        return true;
      }
    }
    false
  }
}

/// AST 字面量值
#[derive(Debug, Clone, PartialEq)]
pub enum AstValue {
  Null,
  Undefined,
  Boolean(bool),
  Smi(i32),
  Number(f64),
  String(AstRawString),
  ConsString(AstConsString),
  BigInt(AstBigInt),
  TheHole,
}

impl AstValue {
  /// 是否在布尔上下文中为真值 (原版 Literal::ToBooleanIsTrue)
  pub fn to_boolean_is_true(&self) -> bool {
    match self {
      Self::Null | Self::Undefined | Self::TheHole => false,
      Self::Boolean(b) => *b,
      Self::Smi(i) => *i != 0,
      Self::Number(n) => !n.is_nan() && *n != 0.0,
      Self::String(s) => !s.is_empty(),
      Self::ConsString(c) => !c.is_empty(),
      Self::BigInt(b) => b.to_boolean_is_true(),
    }
  }
}

/// AST 值与字符串工厂 (原版 AstValueFactory)
#[derive(Default)]
pub struct AstValueFactory<'z> {
  zone: Option<&'z v8_zone::Zone<'z>>,
  strings: HashSet<AstRawString>,
}
impl<'z> AstValueFactory<'z> {
  pub fn new() -> Self {
    Self {
      zone: None,
      strings: HashSet::default(),
    }
  }

  /// 指定 Zone 构造
  pub fn new_in_zone(zone: &'z v8_zone::Zone<'z>) -> Self {
    Self {
      zone: Some(zone),
      strings: HashSet::default(),
    }
  }

  /// 获取关联的 Zone (若有)
  pub fn zone(&self) -> Option<&'z v8_zone::Zone<'z>> {
    self.zone
  }

  /// 缓存的字符串总数
  pub fn string_count(&self) -> usize {
    self.strings.len()
  }

  /// 从单字节切片获取或创建 AstRawString (字典去重)
  pub fn get_one_byte_string_from_bytes(&mut self, bytes: &[u8]) -> AstRawString {
    let raw = AstRawString::from_one_byte(bytes);
    if let Some(existing) = self.strings.get(&raw) {
      existing.clone()
    } else {
      self.strings.insert(raw.clone());
      raw
    }
  }

  /// 从双字节字符切片获取或创建 AstRawString (全 Latin-1 自动规范化为单字节去重)
  pub fn get_two_byte_string(&mut self, chars: &[u16]) -> AstRawString {
    let raw = AstRawString::from_two_byte(chars);
    if let Some(existing) = self.strings.get(&raw) {
      existing.clone()
    } else {
      self.strings.insert(raw.clone());
      raw
    }
  }

  /// 获取或创建单字节字符串 (从 &str)
  pub fn get_one_byte_string(&mut self, s: &str) -> AstRawString {
    self.get_one_byte_string_from_bytes(s.as_bytes())
  }

  /// 获取或创建通用字符串 (自动探测编码并字典去重)
  pub fn get_string(&mut self, s: &str) -> AstRawString {
    let raw = AstRawString::from_utf8(s);
    if let Some(existing) = self.strings.get(&raw) {
      existing.clone()
    } else {
      self.strings.insert(raw.clone());
      raw
    }
  }

  /// 创建空 ConsString
  pub fn new_cons_string(&self) -> AstConsString {
    AstConsString::new()
  }

  /// 创建 BigInt
  pub fn new_bigint(&self, s: &str) -> AstBigInt {
    AstBigInt::new(s)
  }

  // ── 单例常量表 (原版 AstStringConstants) ──

  /// 按常量项内化字符串 (原版 AstStringConstants::name##_string 单例)
  pub fn get_constant(&mut self, constant: AstStringConstant) -> AstRawString {
    self.get_string(constant.contents())
  }

  /// 按字节内化单字符字符串 (原版 one_character_string, 仅 0..128 有效)
  pub fn one_character_string(&mut self, c: u8) -> Option<AstRawString> {
    AstStringConstant::one_char(c).map(|k| self.get_constant(k))
  }
}
