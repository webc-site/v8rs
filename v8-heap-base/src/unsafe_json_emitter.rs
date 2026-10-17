// 由 tool/src/heap/base/unsafe-json-emitter.js 从 v8/src/heap/base/unsafe-json-emitter.h + unsafe-json-emitter.cc 生成; 修订就地改, 复跑覆盖
// 信任输入的 JSON 拼接器

//! UnsafeJsonEmitter: 只用于可信输入的轻量 JSON 对象拼接。
//! 重载集 (机械提取): bool → true/false; 字符串 → 引号包裹; 算术 → 原样输出。
//! 偏差: C++ ostream 浮点默认 6 位有效数字; Rust {} 为最短往返表示。

use std::{fmt, fmt::Write as _};

/// 原 ToString → Display (cc:20); 整数输出用 itoa。
impl fmt::Display for UnsafeJsonEmitter {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&self.out)
  }
}

/// 原 UnsafeJsonEmitter (unsafe-json-emitter.h:15)。
pub struct UnsafeJsonEmitter {
  out: String,
  first: bool,
}

/// 对应 emit_value 重载集 (unsafe-json-emitter.h:33-37): 各类型的输出协议。
pub trait JsonValue {
  fn emit(&self, out: &mut String);
}

impl JsonValue for bool {
  /// 原 emit_value(bool) (h:33): true/false。
  fn emit(&self, out: &mut String) {
    out.push_str(if *self { "true" } else { "false" });
  }
}

impl JsonValue for &str {
  /// 原 emit_value(const char*) (h:34): "\"...\""。
  fn emit(&self, out: &mut String) {
    out.push('"');
    out.push_str(self);
    out.push('"');
  }
}

impl JsonValue for String {
  /// 拥有型字符串 (原 const char* 的 Rust 泛化)。
  fn emit(&self, out: &mut String) {
    self.as_str().emit(out)
  }
}

macro_rules! impl_json_float {
  ($($t:ty),* $(,)?) => {
    $(
      impl JsonValue for $t {
        /// 原 emit_value 浮点重载 (h:36-37); Rust 最短往返表示 (偏差已登记)。
        fn emit(&self, out: &mut String) {
          let _ = write!(out, "{}", self);
        }
      }
    )*
  };
}
impl_json_float!(f64, f32);

macro_rules! impl_json_int {
  ($($t:ty),* $(,)?) => {
    $(
      impl JsonValue for $t {
        /// 原 emit_value 算术重载 (h:36-37): itoa 十进制。
        fn emit(&self, out: &mut String) {
          out.push_str(itoa::Buffer::new().format(*self));
        }
      }
    )*
  };
}
impl_json_int!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl UnsafeJsonEmitter {
  pub fn new() -> Self {
    Self {
      out: String::new(),
      first: false,
    }
  }

  /// 原 object_start (cc:9-13): '{' + first_ = true。
  pub fn object_start(&mut self) -> &mut Self {
    self.out.push('{');
    self.first = true;
    self
  }

  /// 原 object_end (cc:15-18): first_ 保持 (重开对象需重新 object_start)。
  pub fn object_end(&mut self) -> &mut Self {
    self.out.push('}');
    self
  }

  /// 原 p (h:23-29): 非首属性前置逗号; "\""name\":value"。
  pub fn p(&mut self, name: &str, value: impl JsonValue) -> &mut Self {
    if !self.first {
      self.out.push(',');
    }
    self.emit_property_name(name);
    value.emit(&mut self.out);
    self.first = false;
    self
  }

  /// 原 emit_property_name (cc:22-24)。
  fn emit_property_name(&mut self, name: &str) {
    let _ = write!(self.out, "\"{name}\":");
  }
}

impl Default for UnsafeJsonEmitter {
  fn default() -> Self {
    Self::new()
  }
}
