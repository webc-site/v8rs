// 由 tool/src/torque/ls.js 从 v8/src/torque/ls/json-parser.h 生成; 修订就地改, 复跑覆盖
// LS JSON 解析器

use core::str::from_utf8;
use std::result;

use crate::{
  error::Result,
  ls::json::{JsonObject, JsonValue},
};

/// 原版 JsonParserResult 1:1 (error 字段解析失败时携带消息)
#[derive(Debug, Clone, Default)]
pub struct JsonParserResult {
  pub value: Option<JsonValue>,
  pub error: Option<String>,
}

impl JsonParserResult {
  /// 原版成功路径
  #[must_use]
  pub const fn success(value: JsonValue) -> Self {
    Self {
      value: Some(value),
      error: None,
    }
  }

  /// 原版失败路径 (原版静态标志 → 显式字段, 登记偏差)
  #[must_use]
  pub fn failure(message: String) -> Self {
    Self {
      value: None,
      error: Some(message),
    }
  }
}

/// 原版 ParseJson — 手写单遍递归下降 (LS 组自含, 不引 JSON 库)。
/// 词法错 (未知 token) 报 "Lexer Error: unknown token";
/// 语法错 (token 序列非法) 报 "Parser Error: unexpected token" (对齐 json-parser.cc 消息面)。
pub fn parse_json(input: &str) -> Result<JsonParserResult> {
  let mut p = Parser {
    b: input.as_bytes(),
    i: 0,
  };
  p.skip_ws();
  let value = match p.value() {
    Ok(v) => v,
    Err(e) => return Ok(JsonParserResult::failure(e)),
  };
  p.skip_ws();
  if p.i != p.b.len() {
    return Ok(JsonParserResult::failure(
      "Parser Error: unexpected token".to_owned(),
    ));
  }
  Ok(JsonParserResult::success(value))
}

/// 单遍游标: 词法与语法合一 (错误消息按层区分)。
struct Parser<'a> {
  b: &'a [u8],
  i: usize,
}

impl Parser<'_> {
  fn skip_ws(&mut self) {
    while matches!(self.b.get(self.i), Some(b' ' | b'\t' | b'\n' | b'\r')) {
      self.i += 1;
    }
  }

  fn value(&mut self) -> result::Result<JsonValue, String> {
    match self.b.get(self.i) {
      Some(b'{') => self.object(),
      Some(b'[') => self.array(),
      Some(b'"') => Ok(JsonValue::String(self.string()?)),
      Some(b't') | Some(b'f') | Some(b'n') => self.keyword(),
      Some(c) if c.is_ascii_digit() || *c == b'-' => self.number(),
      _ => Err(unknown_token(self.i == self.b.len())),
    }
  }

  fn object(&mut self) -> result::Result<JsonValue, String> {
    self.i += 1; // '{'
    let mut map = JsonObject::default();
    self.skip_ws();
    if self.b.get(self.i) == Some(&b'}') {
      self.i += 1;
      return Ok(JsonValue::Object(map));
    }
    loop {
      self.skip_ws();
      if self.b.get(self.i) != Some(&b'"') {
        // 语法标点在 key 位 → 语法错; 其余 (裸标识符等) → 词法错 (对齐原版两层消息)
        return Err(mismatch_token(self.b.get(self.i).copied()));
      }
      let name = self.string()?;
      self.skip_ws();
      if self.b.get(self.i) != Some(&b':') {
        return Err(unexpected_token());
      }
      self.i += 1;
      self.skip_ws();
      let v = self.value()?;
      map.insert(name, v);
      self.skip_ws();
      match self.b.get(self.i) {
        Some(b',') => self.i += 1,
        Some(b'}') => {
          self.i += 1;
          return Ok(JsonValue::Object(map));
        }
        _ => return Err(unexpected_token()),
      }
    }
  }

  fn array(&mut self) -> result::Result<JsonValue, String> {
    self.i += 1; // '['
    let mut arr = Vec::new();
    self.skip_ws();
    if self.b.get(self.i) == Some(&b']') {
      self.i += 1;
      return Ok(JsonValue::Array(arr));
    }
    loop {
      self.skip_ws();
      arr.push(self.value()?);
      self.skip_ws();
      match self.b.get(self.i) {
        Some(b',') => self.i += 1,
        Some(b']') => {
          self.i += 1;
          return Ok(JsonValue::Array(arr));
        }
        _ => return Err(unexpected_token()),
      }
    }
  }

  fn string(&mut self) -> result::Result<String, String> {
    self.i += 1; // '"'
    let mut out = String::new();
    loop {
      match self.b.get(self.i) {
        None | Some(b'\n') => return Err(unexpected_token()),
        Some(b'"') => {
          self.i += 1;
          return Ok(out);
        }
        Some(b'\\') => {
          self.i += 1;
          match self.b.get(self.i) {
            Some(b'"') => out.push('"'),
            Some(b'\\') => out.push('\\'),
            Some(b'/') => out.push('/'),
            Some(b'b') => out.push('\u{0008}'),
            Some(b'f') => out.push('\u{000C}'),
            Some(b'n') => out.push('\n'),
            Some(b'r') => out.push('\r'),
            Some(b't') => out.push('\t'),
            Some(b'u') => {
              let hex = self
                .b
                .get(self.i + 1..self.i + 5)
                .ok_or_else(unexpected_token)?;
              let s = from_utf8(hex).map_err(|_| unexpected_token())?;
              let cp = u32::from_str_radix(s, 16).map_err(|_| unexpected_token())?;
              self.i += 4;
              // 代理对
              let ch = if (0xD800..0xDC00).contains(&cp) {
                if self.b.get(self.i + 1) == Some(&b'\\') && self.b.get(self.i + 2) == Some(&b'u') {
                  let hex2 = self
                    .b
                    .get(self.i + 3..self.i + 7)
                    .ok_or_else(unexpected_token)?;
                  let s2 = from_utf8(hex2).map_err(|_| unexpected_token())?;
                  let lo = u32::from_str_radix(s2, 16).map_err(|_| unexpected_token())?;
                  self.i += 6;
                  let c = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                  char::from_u32(c).ok_or_else(unexpected_token)?
                } else {
                  return Err(unexpected_token());
                }
              } else {
                char::from_u32(cp).ok_or_else(unexpected_token)?
              };
              out.push(ch);
            }
            _ => return Err(unexpected_token()),
          }
          self.i += 1;
        }
        Some(&c) if c < 0x80 => {
          out.push(c as char);
          self.i += 1;
        }
        Some(_) => {
          // 多字节 UTF-8: 就地解码
          let rest = &self.b[self.i..];
          let s = from_utf8(rest).map_err(|_| unexpected_token())?;
          let ch = s.chars().next().ok_or_else(unexpected_token)?;
          out.push(ch);
          self.i += ch.len_utf8();
        }
      }
    }
  }

  fn keyword(&mut self) -> result::Result<JsonValue, String> {
    for (kw, v) in [
      ("true", JsonValue::Bool(true)),
      ("false", JsonValue::Bool(false)),
      ("null", JsonValue::Null),
    ] {
      if self.b[self.i..].starts_with(kw.as_bytes()) {
        self.i += kw.len();
        return Ok(v);
      }
    }
    Err(unknown_token(false))
  }

  fn number(&mut self) -> result::Result<JsonValue, String> {
    let start = self.i;
    if self.b.get(self.i) == Some(&b'-') {
      self.i += 1;
    }
    while matches!(self.b.get(self.i), Some(c) if c.is_ascii_digit()) {
      self.i += 1;
    }
    if self.b.get(self.i) == Some(&b'.') {
      self.i += 1;
      while matches!(self.b.get(self.i), Some(c) if c.is_ascii_digit()) {
        self.i += 1;
      }
    }
    if matches!(self.b.get(self.i), Some(b'e' | b'E')) {
      self.i += 1;
      if matches!(self.b.get(self.i), Some(b'+' | b'-')) {
        self.i += 1;
      }
      while matches!(self.b.get(self.i), Some(c) if c.is_ascii_digit()) {
        self.i += 1;
      }
    }
    let s = from_utf8(&self.b[start..self.i]).map_err(|_| unexpected_token())?;
    s.parse::<f64>()
      .map(JsonValue::Number)
      .map_err(|_| unexpected_token())
  }
}

fn unexpected_token() -> String {
  "Parser Error: unexpected token".to_owned()
}

fn unknown_token(eof: bool) -> String {
  if eof {
    unexpected_token()
  } else {
    "Lexer Error: unknown token".to_owned()
  }
}

/// key 位非 '"': 语法标点 → Parser Error; 其余字符 → Lexer Error。
fn mismatch_token(c: Option<u8>) -> String {
  match c {
    Some(b']' | b'}' | b'[' | b'{' | b':' | b',') => unexpected_token(),
    _ => unknown_token(false),
  }
}
// ── 原版 src/torque/ls/json-parser.cc ──
// L42: std::optional<ParseResult> MakeNullLiteral(ParseResultIterator* child_results)
// L48: std::optional<ParseResult> MakeNumberLiteral( ParseResultIterator* child_results)
// L55: std::optional<ParseResult> MakeStringLiteral( ParseResultIterator* child_results)
// L61: std::optional<ParseResult> MakeArray(ParseResultIterator* child_results)
// L66: std::optional<ParseResult> MakeMember(ParseResultIterator* child_results)
// L74: std::optional<ParseResult> MakeObject(ParseResultIterator* child_results)
// L181: JsonParserResult ParseJson(const std::string& input)
