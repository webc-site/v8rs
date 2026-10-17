// 由 tool/src/torque/ls.js 从 v8/src/torque/ls/json.h 生成; 修订就地改, 复跑覆盖
// LS JSON 模型 (tag union → enum)

use std::collections::BTreeMap;

use crate::error::{Error, Result};

// 原版 JsonValue tag{OBJECT,ARRAY,STRING,NUMBER,BOOL,IS_NULL} + 5 字段 union →
// Rust enum; unique_ptr 成员由 enum 载荷直接承担。JsonObject/std::map → BTreeMap
// (确定性序, 登记偏差: 原版 map 本就有序)。
/// 原版 using JsonObject = map<string, JsonValue>
pub type JsonObject = BTreeMap<String, JsonValue>;

/// 原版 using JsonArray = vector<JsonValue>
pub type JsonArray = Vec<JsonValue>;

/// 原版 JsonValue 1:1 (tag + union → enum)
#[derive(Debug, Clone, PartialEq, Default)]
pub enum JsonValue {
  Object(JsonObject),
  Array(JsonArray),
  String(String),
  Number(f64),
  Bool(bool),
  #[default]
  Null,
}

/// 原版 JsonMember (object 键值对)
#[derive(Debug, Clone, PartialEq)]
pub struct JsonMember {
  pub name: String,
  pub value: JsonValue,
}

impl JsonValue {
  /// 原版 From(double)
  #[must_use]
  pub fn number(n: f64) -> Self {
    Self::Number(n)
  }

  /// 原版 From(JsonObject)
  #[must_use]
  pub fn object(o: JsonObject) -> Self {
    Self::Object(o)
  }

  /// 原版 From(bool)
  #[must_use]
  pub fn boolean(b: bool) -> Self {
    Self::Bool(b)
  }

  /// 原版 From(string)
  #[must_use]
  pub fn string(s: String) -> Self {
    Self::String(s)
  }

  /// 原版 From(JsonArray)
  #[must_use]
  pub fn array(a: JsonArray) -> Self {
    Self::Array(a)
  }

  /// 原版 JsonNull
  #[must_use]
  pub fn json_null() -> Self {
    Self::Null
  }

  /// 原版 IsNumber/ToNumber → 健壮返回
  pub fn to_number(&self) -> Result<f64> {
    match self {
      Self::Number(n) => Ok(*n),
      _ => Err(Error::Unimplemented),
    }
  }

  /// 原版 ToBool
  pub fn to_bool(&self) -> Result<bool> {
    match self {
      Self::Bool(b) => Ok(*b),
      _ => Err(Error::Unimplemented),
    }
  }

  /// 原版 ToString
  pub fn to_string_ref(&self) -> Result<&str> {
    match self {
      Self::String(s) => Ok(s),
      _ => Err(Error::Unimplemented),
    }
  }

  /// 原版 ToObject
  pub fn to_object(&self) -> Result<&JsonObject> {
    match self {
      Self::Object(o) => Ok(o),
      _ => Err(Error::Unimplemented),
    }
  }

  /// 原版 ToArray
  pub fn to_array(&self) -> Result<&JsonArray> {
    match self {
      Self::Array(a) => Ok(a),
      _ => Err(Error::Unimplemented),
    }
  }
}

/// 原版 SerializeToString
#[must_use]
pub fn serialize_to_string(value: &JsonValue) -> String {
  // 原版 json.cc: 紧凑无空白形态
  let mut out = String::new();
  serialize_into(&mut out, value);
  out
}

fn serialize_into(out: &mut String, value: &JsonValue) {
  match value {
    JsonValue::Object(m) => {
      out.push('{');
      for (i, (k, v)) in m.iter().enumerate() {
        if i > 0 {
          out.push(',');
        }
        serialize_string(out, k);
        out.push(':');
        serialize_into(out, v);
      }
      out.push('}');
    }
    JsonValue::Array(a) => {
      out.push('[');
      for (i, v) in a.iter().enumerate() {
        if i > 0 {
          out.push(',');
        }
        serialize_into(out, v);
      }
      out.push(']');
    }
    JsonValue::String(s) => serialize_string(out, s),
    // f64 Display: 整数值给 "5" 形 (对齐原版 double 流输出)
    JsonValue::Number(n) => out.push_str(&format!("{n}")),
    JsonValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
    JsonValue::Null => out.push_str("null"),
  }
}

fn serialize_string(out: &mut String, s: &str) {
  out.push('"');
  for c in s.chars() {
    match c {
      '"' => out.push_str("\\\""),
      '\\' => out.push_str("\\\\"),
      '\n' => out.push_str("\\n"),
      '\r' => out.push_str("\\r"),
      '\t' => out.push_str("\\t"),
      '\u{0008}' => out.push_str("\\b"),
      '\u{000C}' => out.push_str("\\f"),
      c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
      c => out.push(c),
    }
  }
  out.push('"');
}
// ── 原版 src/torque/ls/json.cc ──
// L18: void SerializeToString(std::stringstream& str, const JsonValue& value)
// L60: std::string SerializeToString(const JsonValue& value)
