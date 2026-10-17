// 由 tool/src/strings/string-builder.h.js 从 v8/src/strings/string-builder.{h,inl.h,cc} 生成; 修订就地改, 复跑覆盖
// 字符串构建器三件套骨架与实现

use core::{
  cmp::max,
  fmt::{Display, Formatter, Result as FmtResult},
};

use itoa::Buffer;

/// IncrementalStringBuilder 分片增长常量 (原版 string-builder.h 私有段)
pub mod string_builder_const {
  /// 首个分片长度 (原版 kInitialPartLength)
  pub const INITIAL_PART_LENGTH: i32 = 32;
  /// 分片长度上限 (原版 kMaxPartLength = 16 * 1024)
  pub const MAX_PART_LENGTH: i32 = 16 * 1024;
  /// 增长因子 (原版 kPartLengthGrowthFactor)
  pub const PART_LENGTH_GROWTH_FACTOR: i32 = 2;
  /// int→字符串视图缓冲宽 (sizeof("-2147483648")-1, 含负号不含 \0)
  pub const INT_TO_STRING_VIEW_BUFFER_SIZE: i32 = 10;
  /// Lazy 模式惰性初始容量 (原版 kInitialCapacityForLazy)
  pub const INITIAL_CAPACITY_FOR_LAZY: u32 = 16;
}

/// 固定数组构建器 (原版 FixedArrayBuilder)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedArrayBuilder {
  storage: Vec<usize>,
  length: usize,
  has_non_smi_elements: bool,
}

impl FixedArrayBuilder {
  #[inline]
  #[must_use]
  pub fn new(initial_capacity: u32) -> Self {
    let cap = max(1, initial_capacity) as usize;
    Self {
      storage: Vec::with_capacity(cap),
      length: 0,
      has_non_smi_elements: false,
    }
  }

  #[inline]
  #[must_use]
  pub fn lazy() -> Self {
    Self {
      storage: Vec::new(),
      length: 0,
      has_non_smi_elements: false,
    }
  }

  #[inline]
  #[must_use]
  pub fn has_capacity(&self, elements: u32) -> bool {
    self.storage.capacity().saturating_sub(self.length) >= elements as usize
  }

  pub fn ensure_capacity(&mut self, elements: u32) {
    let req = self.length.saturating_add(elements as usize);
    if self.storage.capacity() < req {
      if self.storage.capacity() == 0 {
        let cap = max(
          string_builder_const::INITIAL_CAPACITY_FOR_LAZY as usize,
          req,
        );
        self.storage.reserve_exact(cap);
      } else {
        let mut new_cap = self.storage.capacity();
        while new_cap < req {
          new_cap = new_cap.saturating_mul(2);
        }
        let diff = new_cap - self.storage.len();
        self.storage.reserve_exact(diff);
      }
    }
  }

  #[inline]
  pub fn add(&mut self, value: usize) {
    self.add_smi(value);
  }

  #[inline]
  pub fn add_smi(&mut self, value: usize) {
    self.ensure_capacity(1);
    self.storage.push(value);
    self.length += 1;
  }

  #[inline]
  pub fn add_object(&mut self, value: usize) {
    self.ensure_capacity(1);
    self.storage.push(value);
    self.length += 1;
    self.has_non_smi_elements = true;
  }

  #[inline]
  #[must_use]
  pub fn has_non_smi_elements(&self) -> bool {
    self.has_non_smi_elements
  }

  #[inline]
  pub fn set_has_non_smi_elements(&mut self, val: bool) {
    self.has_non_smi_elements = val;
  }

  #[inline]
  #[must_use]
  pub fn length(&self) -> usize {
    self.length
  }

  #[inline]
  #[must_use]
  pub fn capacity(&self) -> usize {
    self.storage.capacity()
  }

  #[inline]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.length == 0
  }

  #[inline]
  #[must_use]
  pub fn as_slice(&self) -> &[usize] {
    &self.storage[..self.length]
  }
}

/// FixedArrayBuilder 骨架 (原版 string-builder.h 前段)
pub mod fixed_array_builder_skel {
  use super::string_builder_const;

  /// 就绪容量判定 (原版 HasCapacity)
  #[inline]
  #[must_use]
  pub fn has_capacity(elements: u32) -> bool {
    elements <= string_builder_const::INITIAL_CAPACITY_FOR_LAZY
  }

  /// 扩容 (原版 EnsureCapacity; Lazy 模式在此惰性分配)
  #[inline]
  pub fn ensure_capacity(_isolate_key: usize, _elements: u32) {}

  /// 压入元素 (原版 Add ×2 重载: Object/Smi)
  #[inline]
  pub fn add(_value: usize) {}

  /// 惰性构造标记 (原版 static Lazy)
  #[inline]
  pub fn lazy(_isolate_key: usize) {}
}

/// Replacement 元素枚举
#[derive(Debug, Clone, PartialEq, Eq)]
enum ReplacementPart {
  Slice { from: usize, to: usize },
  String(String),
}

/// 替换字符串构建器 (原版 ReplacementStringBuilder)
#[derive(Debug, Clone)]
pub struct ReplacementStringBuilder {
  subject: String,
  parts: Vec<ReplacementPart>,
  character_count: usize,
  is_one_byte: bool,
}

impl ReplacementStringBuilder {
  #[must_use]
  pub fn new(subject: String, estimated_part_count: u32) -> Self {
    let is_one_byte = subject.chars().all(|c| (c as u32) <= 0xFF);
    Self {
      subject,
      parts: Vec::with_capacity(estimated_part_count as usize),
      character_count: 0,
      is_one_byte,
    }
  }

  pub fn add_subject_slice(&mut self, from: usize, to: usize) {
    if to > from && to <= self.subject.len() {
      let len = to - from;
      self.parts.push(ReplacementPart::Slice { from, to });
      self.increment_character_count(len as u32);
    }
  }

  pub fn add_string(&mut self, string: String) {
    let len = string.len();
    if !string.chars().all(|c| (c as u32) <= 0xFF) {
      self.is_one_byte = false;
    }
    self.parts.push(ReplacementPart::String(string));
    self.increment_character_count(len as u32);
  }

  #[inline]
  pub fn add_str(&mut self, s: &str) {
    self.add_string(s.to_string());
  }

  #[inline]
  pub fn increment_character_count(&mut self, by: u32) {
    self.character_count = self.character_count.saturating_add(by as usize);
  }

  #[inline]
  #[must_use]
  pub fn is_one_byte(&self) -> bool {
    self.is_one_byte
  }

  #[inline]
  #[must_use]
  pub fn character_count(&self) -> usize {
    self.character_count
  }
}

impl Display for ReplacementStringBuilder {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    for part in &self.parts {
      match part {
        ReplacementPart::Slice { from, to } => {
          if let Some(sub) = self.subject.get(*from..*to) {
            f.write_str(sub)?;
          }
        }
        ReplacementPart::String(s) => {
          f.write_str(s)?;
        }
      }
    }
    Ok(())
  }
}

/// ReplacementStringBuilder 骨架 (替换切片聚合, 供 String.prototype.replace 用)
pub mod replacement_string_builder_skel {
  /// 主体切片压入 (原版 AddSubjectSlice 静态/实例双形态)
  #[inline]
  pub fn add_subject_slice(_builder: usize, _from: i32, _to: i32) {}

  /// 整串压入 (原版 AddString)
  #[inline]
  pub fn add_string(_string: usize) {}

  /// 聚合成品 (原版 ToString → MaybeHandle)
  #[inline]
  #[must_use]
  pub fn to_string() -> Option<usize> {
    Some(0)
  }

  /// 字符计数溢出饱和到 kMaxInt (原版 IncrementCharacterCount)
  #[inline]
  pub fn increment_character_count(_by: u32) {}
}

/// Incremental 编码模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StringBuilderEncoding {
  OneByte = 1,
  TwoByte = 2,
}

/// IncrementalStringBuilder 增量字符串构建器
#[derive(Debug, Clone)]
pub struct IncrementalStringBuilder {
  encoding: StringBuilderEncoding,
  overflowed: bool,
  part_length: usize,
  current_index: usize,
  accumulated: Vec<u8>,
  current_part: Vec<u8>,
}

impl Default for IncrementalStringBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl IncrementalStringBuilder {
  #[must_use]
  pub fn new() -> Self {
    let init_len = string_builder_const::INITIAL_PART_LENGTH as usize;
    Self {
      encoding: StringBuilderEncoding::OneByte,
      overflowed: false,
      part_length: init_len,
      current_index: 0,
      accumulated: Vec::new(),
      current_part: vec![0u8; init_len],
    }
  }

  #[must_use]
  pub fn with_capacity(initial_capacity: usize) -> Self {
    let init_len = max(1, initial_capacity);
    Self {
      encoding: StringBuilderEncoding::OneByte,
      overflowed: false,
      part_length: init_len,
      current_index: 0,
      accumulated: Vec::new(),
      current_part: vec![0u8; init_len],
    }
  }

  #[inline]
  #[must_use]
  pub fn current_encoding(&self) -> u8 {
    self.encoding as u8
  }

  #[inline]
  #[must_use]
  pub fn has_overflowed(&self) -> bool {
    self.overflowed
  }

  #[inline]
  #[must_use]
  pub fn length(&self) -> usize {
    let unit_size = self.encoding as usize;
    (self.accumulated.len() + self.current_index) / unit_size
  }

  #[inline]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.length() == 0
  }

  #[inline]
  #[must_use]
  pub fn current_part_can_fit(&self, length: usize) -> bool {
    let bytes_needed = length * (self.encoding as usize);
    self.part_length.saturating_sub(self.current_index) > bytes_needed
  }

  #[must_use]
  pub fn escaped_length_if_current_part_fits(&self, length: usize) -> usize {
    if length > string_builder_const::MAX_PART_LENGTH as usize {
      return 0;
    }
    let worst = length.saturating_mul(8);
    if self.current_part_can_fit(worst) {
      worst
    } else {
      0
    }
  }

  pub fn append_character(&mut self, c: u8) {
    if self.encoding == StringBuilderEncoding::OneByte {
      if self.current_index >= self.part_length {
        self.extend();
      }
      self.current_part[self.current_index] = c;
      self.current_index += 1;
      if self.current_index == self.part_length {
        self.extend();
      }
    } else {
      self.append_u16(u16::from(c));
    }
  }

  pub fn append_u16(&mut self, c: u16) {
    if self.encoding == StringBuilderEncoding::OneByte {
      if c > 0xFF {
        self.change_encoding();
      } else {
        self.append_character(c as u8);
        return;
      }
    }
    if self.current_index + 2 > self.part_length {
      self.extend();
    }
    let bytes = c.to_le_bytes();
    self.current_part[self.current_index] = bytes[0];
    self.current_part[self.current_index + 1] = bytes[1];
    self.current_index += 2;
    if self.current_index >= self.part_length {
      self.extend();
    }
  }

  pub fn append_str(&mut self, s: &str) {
    for b in s.as_bytes() {
      self.append_character(*b);
    }
  }

  #[inline]
  pub fn append_string(&mut self, s: &str) {
    self.append_str(s);
  }

  #[inline]
  pub fn append_int(&mut self, i: i32) {
    let mut buf = Buffer::new();
    let s = buf.format(i);
    self.append_str(s);
  }

  pub fn append_string_capped(&mut self, s: &str, max_length: usize) {
    if s.len() <= max_length {
      self.append_str(s);
    } else {
      let mut take = max_length;
      while !s.is_char_boundary(take) && take > 0 {
        take -= 1;
      }
      self.append_str(&s[..take]);
      self.append_str("<...>");
    }
  }

  pub fn change_encoding(&mut self) {
    if self.encoding == StringBuilderEncoding::TwoByte {
      return;
    }
    self.shrink_current_part();
    let mut expanded = Vec::with_capacity(self.accumulated.len() * 2);
    for &b in &self.accumulated {
      expanded.extend_from_slice(&(u16::from(b)).to_le_bytes());
    }
    self.accumulated = expanded;
    self.encoding = StringBuilderEncoding::TwoByte;
    self.part_length = self.part_length.saturating_mul(2);
    self.current_part = vec![0u8; self.part_length];
    self.current_index = 0;
  }

  pub fn accumulate(&mut self, bytes: &[u8]) {
    self.accumulated.extend_from_slice(bytes);
  }

  pub fn extend(&mut self) {
    if self.current_index > 0 {
      self
        .accumulated
        .extend_from_slice(&self.current_part[..self.current_index]);
    }
    let max_part = string_builder_const::MAX_PART_LENGTH as usize;
    let growth = string_builder_const::PART_LENGTH_GROWTH_FACTOR as usize;
    if self.part_length <= max_part / growth {
      self.part_length *= growth;
    }
    self.current_part = vec![0u8; self.part_length];
    self.current_index = 0;
  }

  pub fn shrink_current_part(&mut self) {
    if self.current_index > 0 {
      self
        .accumulated
        .extend_from_slice(&self.current_part[..self.current_index]);
      self.current_part.clear();
      self.current_part.resize(self.part_length, 0);
      self.current_index = 0;
    }
  }

  #[inline]
  pub fn shrink_to_fit(&mut self) {
    self.shrink_current_part();
  }

  #[must_use]
  pub fn to_slice(&self) -> &[u8] {
    if self.accumulated.is_empty() {
      &self.current_part[..self.current_index]
    } else {
      &self.accumulated
    }
  }

  pub fn finish(&mut self) -> Option<String> {
    if self.overflowed {
      return None;
    }
    Some(self.to_string())
  }
}

impl Display for IncrementalStringBuilder {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if self.accumulated.is_empty() {
      let slice = &self.current_part[..self.current_index];
      if self.encoding == StringBuilderEncoding::OneByte {
        f.write_str(&String::from_utf8_lossy(slice))
      } else {
        let u16_iter = slice
          .as_chunks::<2>()
          .0
          .iter()
          .map(|c| u16::from_le_bytes(*c));
        for ch in char::decode_utf16(u16_iter).map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER)) {
          let mut buf = [0u8; 4];
          f.write_str(ch.encode_utf8(&mut buf))?;
        }
        Ok(())
      }
    } else {
      let mut full = self.accumulated.clone();
      if self.current_index > 0 {
        full.extend_from_slice(&self.current_part[..self.current_index]);
      }
      if self.encoding == StringBuilderEncoding::OneByte {
        f.write_str(&String::from_utf8_lossy(&full))
      } else {
        let u16_iter = full
          .as_chunks::<2>()
          .0
          .iter()
          .map(|c| u16::from_le_bytes(*c));
        for ch in char::decode_utf16(u16_iter).map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER)) {
          let mut buf = [0u8; 4];
          f.write_str(ch.encode_utf8(&mut buf))?;
        }
        Ok(())
      }
    }
  }
}

/// IncrementalStringBuilder 骨架 (JSON 序列化等增量拼接)
pub mod incremental_string_builder_skel {
  use super::string_builder_const;

  /// 当前编码 (原版 CurrentEncoding: OneByte/TwoByte)
  #[inline]
  #[must_use]
  pub fn current_encoding() -> u8 {
    1
  }

  /// 追加单字符 (原版 Append<SrcChar,DestChar> / AppendCharacter)
  #[inline]
  pub fn append_character(_c: u8) {}

  /// 追加 C 串/视图/整数 (原版 AppendCString/AppendString/AppendInt)
  #[inline]
  pub fn append_str(_s: &str) {}

  /// 追加封顶串: 超长时追加前缀 + "<...>" (原版 AppendStringCapped)
  #[inline]
  pub fn append_string_capped(_string: usize, _max_length: u32) {}

  /// 当前分片是否可容 (原版 CurrentPartCanFit: part_length_ - current_index_ > length)
  #[inline]
  #[must_use]
  pub fn current_part_can_fit(part_length: i32, current_index: i32, length: i32) -> bool {
    part_length - current_index > length
  }

  /// 溢出粗估 (原版 EscapedLengthIfCurrentPartFits)
  #[inline]
  #[must_use]
  pub fn escaped_length_if_current_part_fits(length: i32) -> i32 {
    if length > string_builder_const::MAX_PART_LENGTH {
      0
    } else {
      length << 3
    }
  }

  /// 收尾产出 (原版 Finish → MaybeHandle)
  #[inline]
  #[must_use]
  pub fn finish() -> Option<usize> {
    Some(0)
  }

  /// 编码升级到双字节 (原版 ChangeEncoding)
  #[inline]
  pub fn change_encoding() {}

  /// 分片累积/扩展/收缩 (原版 Accumulate/Extend/ShrinkCurrentPart)
  #[inline]
  pub fn accumulate(_new_part: usize) {}

  #[inline]
  pub fn extend() {}

  #[inline]
  pub fn shrink_current_part() {}
}
