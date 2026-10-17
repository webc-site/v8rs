// 由 tool/src/tracing/traced-value.h.js 从 v8/src/tracing/traced-value.h;src/tracing/traced-value.cc 生成; 修订就地改, 复跑覆盖
// 转义表+签名骨架, 语义体就地修订

//! TracedValue: 追踪事件的结构化 JSON 参数构造器 (traced-value.h/.cc, 89+231 行)。
//!
//! 原版继承 `ConvertableToTraceFormat` (include/v8-platform.h:389) —
//! `AppendAsTraceFormat` 输出完整 JSON 对象 (首尾 `{}` 由本方法补)。
//! `EscapeAndAppendString` (traced-value.cc:31-69) 字节级转义:
//!   显式 6 转义 + c < 0x20 || c == 0x7F → `\\uXXXX` (大写十六进制, SNPrintF "%04X")。
//! 双精度走 `internal::DoubleToStringView` (src/numbers/conversions.cc:1147,
//! ECMA-262 9.8.1 最短表示) — 本模块以 v8-numbers `double_to_string_view` 等价落地。
//! DEBUG 构建 `nesting_stack_` 校验 {Begin,End}{Dictionary,Array} 配对
//! (traced-value.cc:19-29): Rust 以 debug_assert 等价承接。

use itoa::Buffer;
use v8_numbers::conversions::{DOUBLE_TO_STRING_MIN_BUFFER_SIZE, double_to_string_view};

use crate::platform::ConvertableToTraceFormat;

/// `kStackTypeDict` (traced-value.cc:22)。
const STACK_TYPE_DICT: bool = false;
/// `kStackTypeArray` (traced-value.cc:23)。
const STACK_TYPE_ARRAY: bool = true;

/// 大写十六进制位表 (SNPrintF "%04X", traced-value.cc:60)。
const HEX_UPPER: &[u8; 16] = b"0123456789ABCDEF";

/// `EscapeAndAppendString` (traced-value.cc:31): 带引号转义追加。
///
/// 字节级处理: 转义字节就地展开, 普通段 (含 UTF-8 多字节序列) 切片直拷 —
/// 与 C++ 逐字节 memcpy 语义一致。
pub(crate) fn escape_and_append_string(value: &str, result: &mut String) {
  result.push('"');
  // run: 尚未落盘的普通字节段起点 (切片拷贝零逐字节 push)
  let mut run = 0;
  for (i, &c) in value.as_bytes().iter().enumerate() {
    match c {
      b'\x08' | b'\x0c' | b'\n' | b'\r' | b'\t' | b'"' | b'\\' => {
        result.push_str(&value[run..i]);
        result.push_str(match c {
          b'\x08' => "\\b",
          b'\x0c' => "\\f",
          b'\n' => "\\n",
          b'\r' => "\\r",
          b'\t' => "\\t",
          b'"' => "\\\"",
          _ => "\\\\",
        });
        run = i + 1;
      }
      // c < 0x20 || c == 0x7F: 高两位恒为 00 → "\\u00" + 两个大写十六进制位
      _ if c < 0x20 || c == 0x7f => {
        result.push_str(&value[run..i]);
        result.push_str("\\u00");
        result.push(HEX_UPPER[(c >> 4) as usize] as char);
        result.push(HEX_UPPER[(c & 0x0f) as usize] as char);
        run = i + 1;
      }
      _ => {}
    }
  }
  result.push_str(&value[run..]);
  result.push('"');
}

/// `v8::tracing::TracedValue` (traced-value.h:22-85): JSON 构造器。
///
/// `data_` 只存根容器内部内容 (首尾 `{}` 由 `append_as_trace_format` 补);
/// `first_item_` 控制逗号; `nesting_stack_` debug 校验容器配对。
#[derive(Debug)]
pub struct TracedValue {
  data: String,
  first_item: bool,
  nesting_stack: Vec<bool>,
}

impl Default for TracedValue {
  /// 私有构造器 (traced-value.cc:72): first_item_ = true + 根字典压栈。
  fn default() -> Self {
    Self {
      data: String::new(),
      first_item: true,
      nesting_stack: vec![STACK_TYPE_DICT],
    }
  }
}

impl TracedValue {
  /// `Create()` (traced-value.cc:78-80): 根容器恒为字典。
  #[must_use]
  pub fn create() -> Box<Self> {
    Box::new(Self::default())
  }

  /// `WriteComma` (traced-value.cc:211-217)。
  fn write_comma(&mut self) {
    if self.first_item {
      self.first_item = false;
    } else {
      self.data.push(',');
    }
  }

  /// `WriteName` (traced-value.cc:219-223)。
  fn write_name(&mut self, name: &str) {
    self.write_comma();
    self.data.push('"');
    self.data.push_str(name);
    self.data.push_str("\":");
  }

  // ---- Set 族 (字典容器内, traced-value.cc:82-128) ----

  /// `SetInteger` (traced-value.cc:82)。
  pub fn set_integer(&mut self, name: &str, value: i32) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_DICT));
    let mut buf = Buffer::new();
    let s = buf.format(value);
    self.write_name(name);
    self.data.push_str(s);
  }

  /// `SetUnsignedInteger` (traced-value.cc:87)。
  pub fn set_unsigned_integer(&mut self, name: &str, value: u64) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_DICT));
    let mut buf = Buffer::new();
    let s = buf.format(value);
    self.write_name(name);
    self.data.push_str(s);
  }

  /// `SetDouble` (traced-value.cc:92): DoubleToStringView (conversions.cc:1147)。
  pub fn set_double(&mut self, name: &str, value: f64) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_DICT));
    let mut buf = [0u8; DOUBLE_TO_STRING_MIN_BUFFER_SIZE];
    let s = double_to_string_view(value, &mut buf);
    self.write_name(name);
    self.data.push_str(s);
  }

  /// `SetBoolean` (traced-value.cc:99)。
  pub fn set_boolean(&mut self, name: &str, value: bool) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_DICT));
    self.write_name(name);
    self.data.push_str(if value { "true" } else { "false" });
  }

  /// `SetString` (traced-value.cc:104)。
  pub fn set_string(&mut self, name: &str, value: &str) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_DICT));
    self.write_name(name);
    escape_and_append_string(value, &mut self.data);
  }

  /// `SetValue` (traced-value.cc:110): 嵌入子 TracedValue 的完整 JSON。
  pub fn set_value(&mut self, name: &str, value: &Self) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_DICT));
    self.write_name(name);
    value.append_as_trace_format(&mut self.data);
  }

  /// `BeginDictionary(name)` (traced-value.cc:117)。
  pub fn begin_dictionary(&mut self, name: &str) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_DICT));
    self.nesting_stack.push(STACK_TYPE_DICT);
    self.write_name(name);
    self.data.push('{');
    self.first_item = true;
  }

  /// `BeginArray(name)` (traced-value.cc:124)。
  pub fn begin_array(&mut self, name: &str) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_DICT));
    self.nesting_stack.push(STACK_TYPE_ARRAY);
    self.write_name(name);
    self.data.push('[');
    self.first_item = true;
  }

  // ---- Append 族 (数组容器内, traced-value.cc:131-162) ----

  /// `AppendInteger` (traced-value.cc:131)。
  pub fn append_integer(&mut self, value: i32) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_ARRAY));
    let mut buf = Buffer::new();
    let s = buf.format(value);
    self.write_comma();
    self.data.push_str(s);
  }

  /// `AppendDouble` (traced-value.cc:137)。
  pub fn append_double(&mut self, value: f64) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_ARRAY));
    let mut buf = [0u8; DOUBLE_TO_STRING_MIN_BUFFER_SIZE];
    let s = double_to_string_view(value, &mut buf);
    self.write_comma();
    self.data.push_str(s);
  }

  /// `AppendBoolean` (traced-value.cc:144)。
  pub fn append_boolean(&mut self, value: bool) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_ARRAY));
    self.write_comma();
    self.data.push_str(if value { "true" } else { "false" });
  }

  /// `AppendString` (traced-value.cc:150)。
  pub fn append_string(&mut self, value: &str) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_ARRAY));
    self.write_comma();
    escape_and_append_string(value, &mut self.data);
  }

  /// `BeginDictionary()` 无参重载 (traced-value.cc:157-163): 数组内嵌字典。
  pub fn begin_nested_dictionary(&mut self) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_ARRAY));
    self.nesting_stack.push(STACK_TYPE_DICT);
    self.write_comma();
    self.data.push('{');
    self.first_item = true;
  }

  /// `BeginArray()` 无参重载 (traced-value.cc:165-171): 数组内嵌数组。
  pub fn begin_nested_array(&mut self) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_ARRAY));
    self.nesting_stack.push(STACK_TYPE_ARRAY);
    self.write_comma();
    self.data.push('[');
    self.first_item = true;
  }

  // ---- End 族 (traced-value.cc:168-179) ----

  /// `EndDictionary` (traced-value.cc:168)。
  pub fn end_dictionary(&mut self) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_DICT));
    self.nesting_stack.pop();
    self.data.push('}');
    self.first_item = false;
  }

  /// `EndArray` (traced-value.cc:173)。
  pub fn end_array(&mut self) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_ARRAY));
    self.nesting_stack.pop();
    self.data.push(']');
    self.first_item = false;
  }

  /// `AppendAsTraceFormat` (traced-value.cc:225-229): 补首尾 `{}`。
  /// 固有方法直调 (trait 实现转此处)。
  pub fn append_as_trace_format(&self, out: &mut String) {
    out.push('{');
    out.push_str(&self.data);
    out.push('}');
  }
}

impl ConvertableToTraceFormat for TracedValue {
  fn append_as_trace_format(&self, out: &mut String) {
    self.append_as_trace_format(out)
  }
}

impl Drop for TracedValue {
  /// 析构配对校验 (traced-value.cc:75-77 的 DCHECK 族)。
  fn drop(&mut self) {
    debug_assert_eq!(self.nesting_stack.last(), Some(&STACK_TYPE_DICT));
    self.nesting_stack.pop();
    debug_assert!(self.nesting_stack.is_empty());
  }
}
