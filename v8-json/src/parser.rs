use crate::{
  error::{Error, Result},
  token::{
    EscapeKind, JsonToken, get_character_json_scan_flags, get_escape_kind, get_one_char_json_token,
  },
  value::JsonValue,
};

/// 默认最大嵌套栈深度
pub const DEFAULT_MAX_DEPTH: usize = 1000;

/// JSON 流式解析器
pub struct JsonParser<'a> {
  source: &'a str,
  bytes: &'a [u8],
  cursor: usize,
  max_depth: usize,
  current_depth: usize,
}

impl<'a> JsonParser<'a> {
  /// 构造解析器
  #[inline]
  pub fn new(source: &'a str) -> Self {
    Self::with_max_depth(source, DEFAULT_MAX_DEPTH)
  }

  /// 指定最大递归深度的构造器
  #[inline]
  pub fn with_max_depth(source: &'a str, max_depth: usize) -> Self {
    Self {
      source,
      bytes: source.as_bytes(),
      cursor: 0,
      max_depth,
      current_depth: 0,
    }
  }

  /// 解析整个输入流为 JsonValue，确保消费完所有有效字符且没有末尾残留
  pub fn parse(&mut self) -> Result<JsonValue> {
    self.skip_whitespace();
    if self.is_eof() {
      return Err(Error::UnexpectedEndOfInput);
    }

    let val = self.parse_value()?;
    self.skip_whitespace();

    if !self.is_eof() {
      let pos = self.cursor;
      let ch = self.current_char().unwrap_or('\0');
      return Err(Error::UnexpectedToken {
        token: ch.to_string(),
        position: pos,
      });
    }

    Ok(val)
  }

  #[inline]
  fn is_eof(&self) -> bool {
    self.cursor >= self.bytes.len()
  }

  #[inline]
  fn peek_byte(&self) -> Option<u8> {
    self.bytes.get(self.cursor).copied()
  }

  #[inline]
  fn current_char(&self) -> Option<char> {
    self.source.get(self.cursor..)?.chars().next()
  }

  #[inline]
  fn skip_whitespace(&mut self) {
    while let Some(b) = self.peek_byte() {
      if get_one_char_json_token(b) == JsonToken::Whitespace {
        self.cursor += 1;
      } else {
        break;
      }
    }
  }

  fn parse_value(&mut self) -> Result<JsonValue> {
    self.skip_whitespace();
    let b = match self.peek_byte() {
      Some(b) => b,
      None => return Err(Error::UnexpectedEndOfInput),
    };

    match get_one_char_json_token(b) {
      JsonToken::NullLiteral => self.parse_null(),
      JsonToken::TrueLiteral => self.parse_true(),
      JsonToken::FalseLiteral => self.parse_false(),
      JsonToken::String => self.parse_string().map(JsonValue::String),
      JsonToken::Lbrack => self.parse_array(),
      JsonToken::Lbrace => self.parse_object(),
      JsonToken::Number => self.parse_number(),
      _ => {
        let pos = self.cursor;
        let token = self
          .current_char()
          .map(|c| c.to_string())
          .unwrap_or_default();
        Err(Error::UnexpectedToken {
          token,
          position: pos,
        })
      }
    }
  }

  fn scan_literal(&mut self, expected: &[u8]) -> Result<()> {
    for (i, &exp_b) in expected.iter().enumerate() {
      if self.cursor + i >= self.bytes.len() {
        return Err(Error::UnexpectedEndOfInput);
      }
      let actual = self.bytes[self.cursor + i];
      if actual != exp_b {
        self.cursor += i;
        let token = self
          .current_char()
          .map(|c| c.to_string())
          .unwrap_or_default();
        return Err(Error::UnexpectedToken {
          token,
          position: self.cursor,
        });
      }
    }
    self.cursor += expected.len();
    Ok(())
  }

  #[inline]
  fn parse_null(&mut self) -> Result<JsonValue> {
    self.scan_literal(b"null")?;
    Ok(JsonValue::Null)
  }

  #[inline]
  fn parse_true(&mut self) -> Result<JsonValue> {
    self.scan_literal(b"true")?;
    Ok(JsonValue::Bool(true))
  }

  #[inline]
  fn parse_false(&mut self) -> Result<JsonValue> {
    self.scan_literal(b"false")?;
    Ok(JsonValue::Bool(false))
  }

  fn parse_string(&mut self) -> Result<String> {
    // 消费起始引号 '"'
    let _start_pos = self.cursor;
    self.cursor += 1;

    let mut result = String::new();

    while !self.is_eof() {
      let b = self.bytes[self.cursor];

      if b == b'"' {
        self.cursor += 1;
        return Ok(result);
      }

      if b == b'\\' {
        self.cursor += 1;
        if self.is_eof() {
          return Err(Error::UnexpectedEndOfInput);
        }
        let esc = self.bytes[self.cursor];
        self.cursor += 1;
        let flags = get_character_json_scan_flags(esc);
        match get_escape_kind(flags) {
          EscapeKind::SelfEscape => result.push(esc as char),
          EscapeKind::Backspace => result.push('\u{0008}'),
          EscapeKind::FormFeed => result.push('\u{000c}'),
          EscapeKind::NewLine => result.push('\n'),
          EscapeKind::CarriageReturn => result.push('\r'),
          EscapeKind::Tab => result.push('\t'),
          EscapeKind::Unicode => {
            let u1 = self.parse_hex4()?;
            // 检查是否为 UTF-16 高代理 (0xD800..=0xDBFF)
            if (0xd800..=0xdbff).contains(&u1) {
              // 检查紧随其后是否是低代理 \uDC00..\uDFFF
              if self.cursor + 6 <= self.bytes.len()
                && self.bytes[self.cursor] == b'\\'
                && self.bytes[self.cursor + 1] == b'u'
              {
                let saved = self.cursor;
                self.cursor += 2;
                if let Ok(u2) = self.parse_hex4()
                  && (0xdc00..=0xdfff).contains(&u2)
                {
                  let scalar = 0x10000 + (((u1 - 0xd800) as u32) << 10) + ((u2 - 0xdc00) as u32);
                  if let Some(ch) = char::from_u32(scalar) {
                    result.push(ch);
                    continue;
                  }
                }
                // 未匹配到有效低代理，恢复游标并作为孤立字符或者 replacement 字符
                self.cursor = saved;
              }
            }
            if let Some(ch) = char::from_u32(u1 as u32) {
              result.push(ch);
            } else {
              result.push('\u{fffd}');
            }
          }
          EscapeKind::Illegal => {
            return Err(Error::BadEscapeCharacter {
              position: self.cursor - 1,
            });
          }
        }
      } else if b < 0x20 {
        return Err(Error::BadControlCharacter {
          position: self.cursor,
        });
      } else {
        // UTF-8 字符消费 (基于 get 保护字符边界，彻底杜绝切片 panic)
        let s = self
          .source
          .get(self.cursor..)
          .ok_or(Error::UnexpectedEndOfInput)?;
        let ch = s.chars().next().ok_or(Error::UnexpectedEndOfInput)?;
        result.push(ch);
        self.cursor += ch.len_utf8();
      }
    }

    Err(Error::UnexpectedEndOfInput)
  }

  /// 解析 4 位十六进制 Unicode 码元 (纯字节流位运算解析，零分配、杜绝 UTF-8 字符边界截断 panic)
  fn parse_hex4(&mut self) -> Result<u16> {
    if self.cursor + 4 > self.bytes.len() {
      return Err(Error::UnexpectedEndOfInput);
    }
    let mut val: u16 = 0;
    let pos = self.cursor;
    for &b in &self.bytes[self.cursor..self.cursor + 4] {
      let digit = match b {
        b'0'..=b'9' => (b - b'0') as u16,
        b'a'..=b'f' => (b - b'a' + 10) as u16,
        b'A'..=b'F' => (b - b'A' + 10) as u16,
        _ => {
          self.cursor += 4;
          return Err(Error::BadUnicodeEscape { position: pos });
        }
      };
      val = (val << 4) | digit;
    }
    self.cursor += 4;
    Ok(val)
  }

  fn parse_number(&mut self) -> Result<JsonValue> {
    let start = self.cursor;

    // 1. 可选负号
    if let Some(b'-') = self.peek_byte() {
      self.cursor += 1;
    }

    // 2. 整数部分
    match self.peek_byte() {
      Some(b'0') => {
        self.cursor += 1;
        // 规范禁止除单个 0 外的前导 0 (例如 01, 00 非法)
        if let Some(b) = self.peek_byte()
          && b.is_ascii_digit()
        {
          return Err(Error::UnexpectedNumber { position: start });
        }
      }
      Some(b'1'..=b'9') => {
        self.cursor += 1;
        while let Some(b) = self.peek_byte() {
          if b.is_ascii_digit() {
            self.cursor += 1;
          } else {
            break;
          }
        }
      }
      _ => {
        return Err(Error::UnexpectedNumber { position: start });
      }
    }

    // 3. 可选小数部分
    if let Some(b'.') = self.peek_byte() {
      self.cursor += 1;
      let mut frac_digits = 0;
      while let Some(b) = self.peek_byte() {
        if b.is_ascii_digit() {
          frac_digits += 1;
          self.cursor += 1;
        } else {
          break;
        }
      }
      if frac_digits == 0 {
        return Err(Error::UnexpectedNumber { position: start });
      }
    }

    // 4. 可选指数部分
    if let Some(b'e' | b'E') = self.peek_byte() {
      self.cursor += 1;
      if let Some(b'+' | b'-') = self.peek_byte() {
        self.cursor += 1;
      }
      let mut exp_digits = 0;
      while let Some(b) = self.peek_byte() {
        if b.is_ascii_digit() {
          exp_digits += 1;
          self.cursor += 1;
        } else {
          break;
        }
      }
      if exp_digits == 0 {
        return Err(Error::UnexpectedNumber { position: start });
      }
    }

    let num_str = self
      .source
      .get(start..self.cursor)
      .ok_or(Error::UnexpectedNumber { position: start })?;
    let n: f64 = num_str
      .parse()
      .map_err(|_| Error::UnexpectedNumber { position: start })?;
    Ok(JsonValue::Number(n))
  }

  fn parse_array(&mut self) -> Result<JsonValue> {
    // 消费 '['
    self.cursor += 1;
    self.enter_depth()?;

    self.skip_whitespace();
    if let Some(b']') = self.peek_byte() {
      self.cursor += 1;
      self.leave_depth();
      return Ok(JsonValue::Array(Vec::new()));
    }

    let mut elements = Vec::new();

    loop {
      let val = self.parse_value()?;
      elements.push(val);

      self.skip_whitespace();
      match self.peek_byte() {
        Some(b',') => {
          self.cursor += 1;
          self.skip_whitespace();
          // 严禁末尾尾逗号如 [1, 2,]
          if let Some(b']') = self.peek_byte() {
            let pos = self.cursor;
            return Err(Error::UnexpectedToken {
              token: "]".to_string(),
              position: pos,
            });
          }
        }
        Some(b']') => {
          self.cursor += 1;
          self.leave_depth();
          return Ok(JsonValue::Array(elements));
        }
        Some(_) => {
          let pos = self.cursor;
          let ch = self.current_char().unwrap_or('\0');
          return Err(Error::UnexpectedToken {
            token: ch.to_string(),
            position: pos,
          });
        }
        None => return Err(Error::UnexpectedEndOfInput),
      }
    }
  }

  fn parse_object(&mut self) -> Result<JsonValue> {
    // 消费 '{'
    self.cursor += 1;
    self.enter_depth()?;

    self.skip_whitespace();
    if let Some(b'}') = self.peek_byte() {
      self.cursor += 1;
      self.leave_depth();
      return Ok(JsonValue::Object(Vec::new()));
    }

    let mut entries = Vec::new();

    loop {
      self.skip_whitespace();
      if let Some(b) = self.peek_byte() {
        if b != b'"' {
          let pos = self.cursor;
          let ch = self.current_char().unwrap_or('\0');
          return Err(Error::UnexpectedToken {
            token: ch.to_string(),
            position: pos,
          });
        }
      } else {
        return Err(Error::UnexpectedEndOfInput);
      }

      let key = self.parse_string()?;
      self.skip_whitespace();

      match self.peek_byte() {
        Some(b':') => self.cursor += 1,
        Some(_) => {
          let pos = self.cursor;
          let ch = self.current_char().unwrap_or('\0');
          return Err(Error::UnexpectedToken {
            token: ch.to_string(),
            position: pos,
          });
        }
        None => return Err(Error::UnexpectedEndOfInput),
      }

      let val = self.parse_value()?;
      entries.push((key, val));

      self.skip_whitespace();
      match self.peek_byte() {
        Some(b',') => {
          self.cursor += 1;
          self.skip_whitespace();
          // 严禁末尾尾逗号如 {"a": 1,}
          if let Some(b'}') = self.peek_byte() {
            let pos = self.cursor;
            return Err(Error::UnexpectedToken {
              token: "}".to_string(),
              position: pos,
            });
          }
        }
        Some(b'}') => {
          self.cursor += 1;
          self.leave_depth();
          return Ok(JsonValue::Object(entries));
        }
        Some(_) => {
          let pos = self.cursor;
          let ch = self.current_char().unwrap_or('\0');
          return Err(Error::UnexpectedToken {
            token: ch.to_string(),
            position: pos,
          });
        }
        None => return Err(Error::UnexpectedEndOfInput),
      }
    }
  }

  #[inline]
  fn enter_depth(&mut self) -> Result<()> {
    self.current_depth += 1;
    if self.current_depth > self.max_depth {
      Err(Error::StackOverflow {
        limit: self.max_depth,
      })
    } else {
      Ok(())
    }
  }

  #[inline]
  fn leave_depth(&mut self) {
    if self.current_depth > 0 {
      self.current_depth -= 1;
    }
  }
}
