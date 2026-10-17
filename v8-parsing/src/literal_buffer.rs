//! 字面量收集器 (对应原版 v8/src/parsing/literal-buffer.*)

use v8_ast::{AstRawString, AstValueFactory};
use v8_strings::unicode::utf16;

use crate::literal_buffer_generated::K_INITIAL_CAPACITY;

/// 字面量字符收集器 (紧凑双字节/单字节自适应，支持 Latin-1 折叠与向 AST 工厂移交)
#[derive(Debug, Clone)]
pub struct LiteralBuffer {
  one_byte: Vec<u8>,
  two_byte: Vec<u16>,
  is_one_byte: bool,
}

impl Default for LiteralBuffer {
  fn default() -> Self {
    Self::new()
  }
}

impl LiteralBuffer {
  /// 创建新的字面量收集器，预分配初始容量
  #[inline]
  #[must_use]
  pub fn new() -> Self {
    Self {
      one_byte: Vec::with_capacity(K_INITIAL_CAPACITY),
      two_byte: Vec::new(),
      is_one_byte: true,
    }
  }

  /// 重置字面量收集器状态，开始收集新字面量 (保留底层分配内存)
  #[inline]
  pub fn start(&mut self) {
    self.one_byte.clear();
    self.two_byte.clear();
    self.is_one_byte = true;
  }

  /// 是否为单字节 (Latin-1) 模式
  #[inline]
  #[must_use]
  pub const fn is_one_byte(&self) -> bool {
    self.is_one_byte
  }

  /// 是否为空
  #[inline]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// 当前字面量的代码单元长度
  #[inline]
  #[must_use]
  pub fn len(&self) -> usize {
    if self.is_one_byte {
      self.one_byte.len()
    } else {
      self.two_byte.len()
    }
  }

  /// 添加单个字符 (码点或代码单元)
  #[inline]
  pub fn add_char(&mut self, code_point: u32) {
    if self.is_one_byte {
      if code_point <= 0xFF {
        self.one_byte.push(code_point as u8);
        return;
      }
      self.convert_to_two_byte();
    }
    self.add_two_byte_char(code_point);
  }

  /// 添加单个 ASCII 字符
  #[inline]
  pub fn add_ascii_char(&mut self, c: u8) {
    if self.is_one_byte {
      self.one_byte.push(c);
    } else {
      self.two_byte.push(c as u16);
    }
  }

  /// 在双字节模式下添加字符 (自动分解代理对)
  #[inline]
  pub fn add_two_byte_char(&mut self, code_point: u32) {
    debug_assert!(!self.is_one_byte);
    if code_point <= 0xFFFF {
      self.two_byte.push(code_point as u16);
    } else {
      self.two_byte.push(utf16::lead_surrogate(code_point));
      self.two_byte.push(utf16::trail_surrogate(code_point));
    }
  }

  /// 追加 UTF-16 代码单元切片
  pub fn add_range_from_utf16(&mut self, slice: &[u16]) {
    if self.is_one_byte {
      if slice.iter().all(|&c| c <= 0xFF) {
        self.one_byte.extend(slice.iter().map(|&c| c as u8));
        return;
      }
      self.convert_to_two_byte();
    }
    self.two_byte.extend_from_slice(slice);
  }

  /// 追加单字节切片
  pub fn add_range_from_u8(&mut self, slice: &[u8]) {
    if self.is_one_byte {
      self.one_byte.extend_from_slice(slice);
    } else {
      self.two_byte.extend(slice.iter().map(|&b| b as u16));
    }
  }

  /// 转换为双字节模式
  fn convert_to_two_byte(&mut self) {
    debug_assert!(self.is_one_byte);
    self.two_byte.clear();
    self
      .two_byte
      .extend(self.one_byte.iter().map(|&b| b as u16));
    self.is_one_byte = false;
  }

  /// 尝试将双字节内容折叠回单字节 Latin-1 (若全字符 <= 0xFF)
  pub fn fold_to_latin1(&mut self) -> bool {
    if self.is_one_byte {
      return true;
    }
    if self.two_byte.iter().all(|&c| c <= 0xFF) {
      self.one_byte.clear();
      self.one_byte.extend(self.two_byte.iter().map(|&c| c as u8));
      self.two_byte.clear();
      self.is_one_byte = true;
      true
    } else {
      false
    }
  }

  /// 单字节字面量切片
  #[inline]
  #[must_use]
  pub fn one_byte_literal(&self) -> &[u8] {
    &self.one_byte
  }

  /// 双字节字面量切片
  #[inline]
  #[must_use]
  pub fn two_byte_literal(&self) -> &[u16] {
    &self.two_byte
  }

  /// 匹配单字节关键字
  #[inline]
  #[must_use]
  pub fn equals(&self, keyword: &[u8]) -> bool {
    self.is_one_byte && self.one_byte.as_slice() == keyword
  }

  /// 转换为标准 Rust String
  #[must_use]
  pub fn to_string_lossy(&self) -> String {
    if self.is_one_byte {
      if self.one_byte.is_ascii() {
        // SAFETY: 纯 ASCII 序列与 UTF-8 编码完全相同
        unsafe { String::from_utf8_unchecked(self.one_byte.clone()) }
      } else {
        self.one_byte.iter().map(|&b| b as char).collect()
      }
    } else {
      String::from_utf16_lossy(&self.two_byte)
    }
  }

  /// 将字面量内化并移交给 AST 工厂 (原版 LiteralBuffer::Internalize)
  pub fn internalize(&self, factory: &mut AstValueFactory) -> AstRawString {
    if self.is_one_byte {
      factory.get_one_byte_string_from_bytes(self.one_byte_literal())
    } else {
      factory.get_two_byte_string(self.two_byte_literal())
    }
  }
}
