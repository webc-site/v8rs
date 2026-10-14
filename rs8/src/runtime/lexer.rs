//! JavaScript 词法分析器 (Lexer)
//!
//! 零拷贝字符流扫描，直接基于源码切片与字节索引流式解析，
//! 消除任何中间 Vec<char> 分配，对关键字、数字字面量、运算符零堆分配。

use crate::error::{Error, Result};

/// 词法记号类型
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  // 字面量
  Number(f64),
  BigInt(i128),
  String(String),
  RegExp { pattern: String, flags: String },
  Identifier(String),
  True,
  False,
  Null,
  Undefined,

  // 关键字
  Var,
  Let,
  Const,
  Function,
  Return,
  If,
  Else,
  While,
  For,
  Break,
  Continue,
  New,
  This,
  Typeof,
  Try,
  Catch,
  Finally,
  Throw,
  Instanceof,
  In,
  Switch,
  Case,
  Default,
  Do,
  Void,
  Delete,

  // 运算符
  Plus,
  Minus,
  Star,
  Slash,
  Percent,
  Ampersand,
  Pipe,
  Caret,
  Tilde,
  Exclamation,
  Equal,
  PlusEqual,
  MinusEqual,
  StarEqual,
  SlashEqual,
  PercentEqual,
  AmpersandEqual,
  PipeEqual,
  CaretEqual,
  DoublePlus,
  DoubleMinus,
  DoubleEqual,
  TripleEqual,
  ExclamationEqual,
  ExclamationDoubleEqual,
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
  DoubleAmpersand,
  DoublePipe,
  DoubleLess,
  DoubleGreater,
  TripleGreater,
  DoubleLessEqual,
  DoubleGreaterEqual,
  TripleGreaterEqual,

  // 分隔符
  LParen,
  RParen,
  LBrace,
  RBrace,
  LBracket,
  RBracket,
  Semicolon,
  Comma,
  Dot,
  Colon,
  Question,

  Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub line: usize,
  pub col: usize,
}

pub struct Lexer<'a> {
  source: &'a str,
  bytes: &'a [u8],
  pos: usize,
  line: usize,
  col: usize,
  last_token_kind: Option<TokenKind>,
}

impl<'a> Lexer<'a> {
  /// 零拷贝构造词法扫描器 (零堆分配)
  pub fn new(source: &'a str) -> Self {
    Self {
      source,
      bytes: source.as_bytes(),
      pos: 0,
      line: 1,
      col: 1,
      last_token_kind: None,
    }
  }

  #[inline]
  fn peek(&self) -> Option<char> {
    if self.pos < self.bytes.len() {
      let b = self.bytes[self.pos];
      if b < 0x80 {
        Some(b as char)
      } else {
        self.source[self.pos..].chars().next()
      }
    } else {
      None
    }
  }

  #[inline]
  fn peek_next(&self) -> Option<char> {
    if self.pos < self.bytes.len() {
      let cur_len = if self.bytes[self.pos] < 0x80 {
        1
      } else {
        self.source[self.pos..]
          .chars()
          .next()
          .map_or(1, |c| c.len_utf8())
      };
      let next_pos = self.pos + cur_len;
      if next_pos < self.bytes.len() {
        let b = self.bytes[next_pos];
        if b < 0x80 {
          Some(b as char)
        } else {
          self.source[next_pos..].chars().next()
        }
      } else {
        None
      }
    } else {
      None
    }
  }

  #[inline]
  fn advance(&mut self) -> Option<char> {
    if self.pos < self.bytes.len() {
      let b = self.bytes[self.pos];
      let (ch, len) = if b < 0x80 {
        (b as char, 1)
      } else {
        let c = self.source[self.pos..].chars().next().unwrap();
        (c, c.len_utf8())
      };
      self.pos += len;
      if ch == '\n' {
        self.line += 1;
        self.col = 1;
      } else {
        self.col += 1;
      }
      Some(ch)
    } else {
      None
    }
  }

  fn skip_whitespace_and_comments(&mut self) {
    while let Some(ch) = self.peek() {
      match ch {
        ' ' | '\t' | '\r' | '\n' | '\x0B' | '\x0C' | '\u{00A0}' | '\u{FEFF}' | '\u{2028}'
        | '\u{2029}' => {
          self.advance();
        }
        '/' if self.peek_next() == Some('/') => {
          // 单行注释
          while let Some(c) = self.advance() {
            if c == '\n' {
              break;
            }
          }
        }
        '/' if self.peek_next() == Some('*') => {
          // 多行注释
          self.advance();
          self.advance();
          while let Some(c) = self.advance() {
            if c == '*' && self.peek() == Some('/') {
              self.advance();
              break;
            }
          }
        }
        _ => break,
      }
    }
  }

  pub fn next_token(&mut self) -> Result<Token> {
    self.skip_whitespace_and_comments();

    let start_line = self.line;
    let start_col = self.col;

    let ch = match self.advance() {
      Some(c) => c,
      None => {
        return Ok(Token {
          kind: TokenKind::Eof,
          line: start_line,
          col: start_col,
        });
      }
    };

    let kind = match ch {
      '(' => TokenKind::LParen,
      ')' => TokenKind::RParen,
      '{' => TokenKind::LBrace,
      '}' => TokenKind::RBrace,
      '[' => TokenKind::LBracket,
      ']' => TokenKind::RBracket,
      ';' => TokenKind::Semicolon,
      ',' => TokenKind::Comma,
      ':' => TokenKind::Colon,
      '?' => TokenKind::Question,
      '.' => TokenKind::Dot,
      '~' => TokenKind::Tilde,

      '+' => {
        if self.peek() == Some('+') {
          self.advance();
          TokenKind::DoublePlus
        } else if self.peek() == Some('=') {
          self.advance();
          TokenKind::PlusEqual
        } else {
          TokenKind::Plus
        }
      }

      '-' => {
        if self.peek() == Some('-') {
          self.advance();
          TokenKind::DoubleMinus
        } else if self.peek() == Some('=') {
          self.advance();
          TokenKind::MinusEqual
        } else {
          TokenKind::Minus
        }
      }

      '*' => {
        if self.peek() == Some('=') {
          self.advance();
          TokenKind::StarEqual
        } else {
          TokenKind::Star
        }
      }

      '/' => {
        let is_division = matches!(
          &self.last_token_kind,
          Some(
            TokenKind::Number(_)
              | TokenKind::String(_)
              | TokenKind::Identifier(_)
              | TokenKind::True
              | TokenKind::False
              | TokenKind::Null
              | TokenKind::Undefined
              | TokenKind::This
              | TokenKind::RParen
              | TokenKind::RBracket
              | TokenKind::RBrace
              | TokenKind::DoublePlus
              | TokenKind::DoubleMinus,
          )
        );

        if is_division {
          if self.peek() == Some('=') {
            self.advance();
            TokenKind::SlashEqual
          } else {
            TokenKind::Slash
          }
        } else {
          let mut pattern = String::new();
          let mut in_class = false;
          loop {
            match self.advance() {
              Some('\\') => {
                pattern.push('\\');
                if let Some(next_c) = self.advance() {
                  pattern.push(next_c);
                }
              }
              Some('[') => {
                in_class = true;
                pattern.push('[');
              }
              Some(']') => {
                in_class = false;
                pattern.push(']');
              }
              Some('/') if !in_class => {
                break;
              }
              Some('\n') | None => {
                return Err(Error::ParseError(format!(
                  "Unterminated regular expression literal at {start_line}:{start_col}"
                )));
              }
              Some(c) => {
                pattern.push(c);
              }
            }
          }
          let mut flags = String::new();
          while let Some(c) = self.peek() {
            if c.is_alphabetic() {
              flags.push(c);
              self.advance();
            } else {
              break;
            }
          }
          TokenKind::RegExp { pattern, flags }
        }
      }

      '%' => {
        if self.peek() == Some('=') {
          self.advance();
          TokenKind::PercentEqual
        } else {
          TokenKind::Percent
        }
      }

      '=' => {
        if self.peek() == Some('=') {
          self.advance();
          if self.peek() == Some('=') {
            self.advance();
            TokenKind::TripleEqual
          } else {
            TokenKind::DoubleEqual
          }
        } else {
          TokenKind::Equal
        }
      }

      '!' => {
        if self.peek() == Some('=') {
          self.advance();
          if self.peek() == Some('=') {
            self.advance();
            TokenKind::ExclamationDoubleEqual
          } else {
            TokenKind::ExclamationEqual
          }
        } else {
          TokenKind::Exclamation
        }
      }

      '<' => {
        if self.peek() == Some('<') {
          self.advance();
          if self.peek() == Some('=') {
            self.advance();
            TokenKind::DoubleLessEqual
          } else {
            TokenKind::DoubleLess
          }
        } else if self.peek() == Some('=') {
          self.advance();
          TokenKind::LessEqual
        } else {
          TokenKind::Less
        }
      }

      '>' => {
        if self.peek() == Some('>') {
          self.advance();
          if self.peek() == Some('>') {
            self.advance();
            if self.peek() == Some('=') {
              self.advance();
              TokenKind::TripleGreaterEqual
            } else {
              TokenKind::TripleGreater
            }
          } else if self.peek() == Some('=') {
            self.advance();
            TokenKind::DoubleGreaterEqual
          } else {
            TokenKind::DoubleGreater
          }
        } else if self.peek() == Some('=') {
          self.advance();
          TokenKind::GreaterEqual
        } else {
          TokenKind::Greater
        }
      }

      '&' => {
        if self.peek() == Some('&') {
          self.advance();
          TokenKind::DoubleAmpersand
        } else if self.peek() == Some('=') {
          self.advance();
          TokenKind::AmpersandEqual
        } else {
          TokenKind::Ampersand
        }
      }

      '|' => {
        if self.peek() == Some('|') {
          self.advance();
          TokenKind::DoublePipe
        } else if self.peek() == Some('=') {
          self.advance();
          TokenKind::PipeEqual
        } else {
          TokenKind::Pipe
        }
      }

      '^' => {
        if self.peek() == Some('=') {
          self.advance();
          TokenKind::CaretEqual
        } else {
          TokenKind::Caret
        }
      }

      '"' | '\'' => {
        let quote = ch;
        let mut s = String::new();
        while let Some(c) = self.advance() {
          if c == quote {
            break;
          }
          if c == '\\' {
            if let Some(escaped) = self.advance() {
              match escaped {
                'n' => s.push('\n'),
                't' => s.push('\t'),
                'r' => s.push('\r'),
                'b' => s.push('\x08'),
                'f' => s.push('\x0C'),
                'v' => s.push('\x0B'),
                '0' => s.push('\0'),
                '\\' => s.push('\\'),
                '"' => s.push('"'),
                '\'' => s.push('\''),
                'u' => {
                  let mut code: u32 = 0;
                  let mut valid = true;
                  for _ in 0..4 {
                    if let Some(hc) = self.advance() {
                      if let Some(d) = hc.to_digit(16) {
                        code = (code << 4) | d;
                      } else {
                        valid = false;
                        break;
                      }
                    } else {
                      valid = false;
                      break;
                    }
                  }
                  if valid && let Some(ch) = char::from_u32(code) {
                    s.push(ch);
                  }
                }
                'x' => {
                  let mut code: u32 = 0;
                  let mut valid = true;
                  for _ in 0..2 {
                    if let Some(hc) = self.advance() {
                      if let Some(d) = hc.to_digit(16) {
                        code = (code << 4) | d;
                      } else {
                        valid = false;
                        break;
                      }
                    } else {
                      valid = false;
                      break;
                    }
                  }
                  if valid && let Some(ch) = char::from_u32(code) {
                    s.push(ch);
                  }
                }
                _ => {
                  s.push(escaped);
                }
              }
            }
          } else {
            s.push(c);
          }
        }
        TokenKind::String(s)
      }

      '0'..='9' => {
        let start = self.pos - 1;
        // 检查十六进制 0x、二进制 0b、八进制 0o
        if ch == '0' && matches!(self.peek(), Some('x' | 'X')) {
          self.advance();
          let mut hex_val: u128 = 0;
          while let Some(c) = self.peek() {
            if let Some(digit) = c.to_digit(16) {
              hex_val = (hex_val << 4) | (digit as u128);
              self.advance();
            } else if c == '_' {
              self.advance();
            } else {
              break;
            }
          }
          if self.peek() == Some('n') {
            self.advance();
            TokenKind::BigInt(hex_val as i128)
          } else {
            TokenKind::Number(hex_val as f64)
          }
        } else if ch == '0' && matches!(self.peek(), Some('b' | 'B')) {
          self.advance();
          let mut bin_val: u128 = 0;
          while let Some(c) = self.peek() {
            if c == '0' || c == '1' {
              bin_val = (bin_val << 1) | (if c == '1' { 1 } else { 0 });
              self.advance();
            } else if c == '_' {
              self.advance();
            } else {
              break;
            }
          }
          if self.peek() == Some('n') {
            self.advance();
            TokenKind::BigInt(bin_val as i128)
          } else {
            TokenKind::Number(bin_val as f64)
          }
        } else if ch == '0' && matches!(self.peek(), Some('o' | 'O')) {
          self.advance();
          let mut oct_val: u128 = 0;
          while let Some(c) = self.peek() {
            if ('0'..='7').contains(&c) {
              oct_val = (oct_val << 3) | ((c as u128) - ('0' as u128));
              self.advance();
            } else if c == '_' {
              self.advance();
            } else {
              break;
            }
          }
          if self.peek() == Some('n') {
            self.advance();
            TokenKind::BigInt(oct_val as i128)
          } else {
            TokenKind::Number(oct_val as f64)
          }
        } else {
          let mut has_dot = false;
          let mut has_underscore = false;
          while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
              self.advance();
            } else if c == '_' {
              has_underscore = true;
              self.advance();
            } else if c == '.' && !has_dot {
              has_dot = true;
              self.advance();
            } else {
              break;
            }
          }
          let mut has_exp = false;
          if matches!(self.peek(), Some('e' | 'E')) {
            has_exp = true;
            self.advance();
            if matches!(self.peek(), Some('+' | '-')) {
              self.advance();
            }
            while let Some(c) = self.peek() {
              if c.is_ascii_digit() || c == '_' {
                self.advance();
              } else {
                break;
              }
            }
          }
          if !has_dot && !has_exp && self.peek() == Some('n') {
            let slice = &self.source[start..self.pos];
            self.advance();
            let filtered: String = if has_underscore {
              slice.chars().filter(|&c| c != '_').collect()
            } else {
              slice.to_string()
            };
            let val = filtered
              .parse::<i128>()
              .map_err(|e| Error::ParseError(format!("Invalid BigInt literal '{slice}n': {e}")))?;
            TokenKind::BigInt(val)
          } else {
            let slice = &self.source[start..self.pos];
            let val = if has_underscore {
              let filtered: String = slice.chars().filter(|&c| c != '_').collect();
              filtered
                .parse::<f64>()
                .map_err(|e| Error::ParseError(format!("Invalid number literal '{slice}': {e}")))?
            } else {
              slice
                .parse::<f64>()
                .map_err(|e| Error::ParseError(format!("Invalid number literal '{slice}': {e}")))?
            };
            TokenKind::Number(val)
          }
        }
      }

      c if c.is_alphabetic() || c == '_' || c == '$' => {
        let start = self.pos - c.len_utf8();
        while let Some(next_c) = self.peek() {
          if next_c.is_alphanumeric() || next_c == '_' || next_c == '$' {
            self.advance();
          } else {
            break;
          }
        }
        let ident = &self.source[start..self.pos];
        match ident {
          "var" => TokenKind::Var,
          "let" => TokenKind::Let,
          "const" => TokenKind::Const,
          "function" => TokenKind::Function,
          "return" => TokenKind::Return,
          "if" => TokenKind::If,
          "else" => TokenKind::Else,
          "while" => TokenKind::While,
          "for" => TokenKind::For,
          "break" => TokenKind::Break,
          "continue" => TokenKind::Continue,
          "new" => TokenKind::New,
          "this" => TokenKind::This,
          "typeof" => TokenKind::Typeof,
          "try" => TokenKind::Try,
          "catch" => TokenKind::Catch,
          "finally" => TokenKind::Finally,
          "throw" => TokenKind::Throw,
          "instanceof" => TokenKind::Instanceof,
          "in" => TokenKind::In,
          "switch" => TokenKind::Switch,
          "case" => TokenKind::Case,
          "default" => TokenKind::Default,
          "do" => TokenKind::Do,
          "void" => TokenKind::Void,
          "delete" => TokenKind::Delete,
          "true" => TokenKind::True,
          "false" => TokenKind::False,
          "null" => TokenKind::Null,
          "undefined" => TokenKind::Undefined,
          _ => TokenKind::Identifier(ident.to_string()),
        }
      }

      other => {
        return Err(Error::ParseError(format!(
          "Unexpected character '{other}' at {start_line}:{start_col}"
        )));
      }
    };

    let tok = Token {
      kind,
      line: start_line,
      col: start_col,
    };
    self.last_token_kind = Some(tok.kind.clone());
    Ok(tok)
  }

  pub fn tokenize_all(&mut self) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    loop {
      let tok = self.next_token()?;
      let is_eof = tok.kind == TokenKind::Eof;
      tokens.push(tok);
      if is_eof {
        break;
      }
    }
    Ok(tokens)
  }
}
