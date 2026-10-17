//! criterion 评测基础层: 核心评测操作 trait 与参与评测的各实现
//!
//! v8-parsing 公开面 (见 src/lib.rs): CharacterStream / Scanner / LiteralBuffer / Token,
//! 无独立 Parser 入口, 故评测聚焦词法层.
//!
//! 评测操作设计:
//! - scan_tokens: Scanner 全量词法扫描 (标识符/关键字/数字字面量/字符串/模板/注释/正则起始歧义)
//! - scan_regexp: 扫描中遇 '/' 歧义时走 scan_regexp_pattern + scan_regexp_flags 公开 API 重扫
//! - lookahead: 4 槽环形 lookahead 的 peek_ahead / peek_ahead_ahead 命中路径
//! - literal_fill: LiteralBuffer 写入路径 (start + add_char 单字节模式)
//!
//! 对比实现 bench-naive: 单遍手写简易 tokenizer, 只扫空白/注释/标点的最小等价路径,
//! 用于体现 Scanner 完整 ECMAScript 词法语义的成本.

use v8_parsing::{LiteralBuffer, OneByteCharacterStream, Scanner, Token};

/// 本 crate 核心评测操作 trait, 评测 id 基于 NAME 生成
pub trait ParseBench {
  /// 实现名, 作为 criterion 评测 id 前缀
  const NAME: &'static str;

  /// 全量词法扫描, 返回 token 数 (含 Eos)
  fn scan_tokens(src: &str) -> usize;

  /// 词法扫描 + '/' 歧义处按正则字面量重扫, 返回 token 数
  fn scan_regexp(src: &str) -> usize;

  /// 环形 lookahead 命中路径: 预填满缓存后连续向前看, 返回命中次数
  fn lookahead(src: &str, rounds: usize) -> usize;

  /// 字面量缓冲写入: reps 次 start + 每次 units 个单字节字符, 返回累计长度
  fn literal_fill(units: usize, reps: usize) -> usize;
}

/// 固定确定性小型 JS 源码样本: 标识符/关键字/数字字面量族/字符串/模板/注释/正则起始歧义
const SMALL_SRC: &str = r#"// 行注释 line-comment
/* 块注释
   block-comment */
const greeting = "hello \"world\"";
let count = 0x1F + 1_000 + 0b1010 + 0o17 + 3.14e-2 + 077n;
function fib(n) {
  if (n < 2) return n;
  return fib(n - 1) + fib(n - 2);
}
class Point {
  #x = 1;
  get x() { return this.#x; }
}
const ok = /foo\/bar/gi.test("a/b");
let ratio = count / 2 / 3;
const tpl = `a ${count} b \(esc\) \n tail`;
let arr = [1, 2, 3, ...extra], obj = { "k": 'v', m: (a, b) => a ?? b };
outer: for (let i = 0; i < 10; i++) { if (arr[i] > 2) continue outer; }
let mix = a<b ? c>>>d : e?.f ?? g, arrow = () => void 0;
"#;

/// 正则歧义专项样本: 正则字面量密集, 供 scan_regexp 操作使用
const REGEXP_SRC: &str = r#"const pats = [/\d+/g, /^[a-z_][a-z0-9_]*$/i, /foo\/bar/gim, /[\]\\]/g];
function splitAll(s) { return s.split(/\s*,\s*/); }
let ratio = 1024 / 512 / 2;
let masked = /"[^"]*"/.exec(line) ? 1 : 0;
"#;

/// 评测样本集: (类别名, 源码), 全部固定确定性生成
#[must_use]
pub fn samples() -> Vec<(&'static str, String)> {
  let small = SMALL_SRC.to_string();
  let medium = SMALL_SRC.repeat(12);
  let large = SMALL_SRC.repeat(120);
  vec![
    ("small", small),
    ("medium", medium),
    ("large", large),
    ("regexp", REGEXP_SRC.to_string()),
  ]
}

/// lookahead 命中轮数 (固定确定性)
pub const LOOKAHEAD_ROUNDS: usize = 512;

/// literal_fill 参数: 每个字面量 256 个代码单元 × 512 次写入
pub const LITERAL_UNITS: usize = 256;
pub const LITERAL_REPS: usize = 512;

/// 被测对象: v8rs 完整 ECMAScript 语义 Scanner
pub struct V8Scanner;

/// 构建单字节流 Scanner 并完成初始化预取
fn make_scanner<'a>(src: &'a str) -> Scanner<OneByteCharacterStream<'a>> {
  let mut scanner = Scanner::new(OneByteCharacterStream::new(src.as_bytes()));
  scanner.initialize();
  scanner
}

impl ParseBench for V8Scanner {
  const NAME: &'static str = "v8-scanner";

  fn scan_tokens(src: &str) -> usize {
    let mut scanner = make_scanner(src);
    let mut count = 0usize;
    loop {
      let token = scanner.next_token();
      count += 1;
      if token == Token::Eos {
        break;
      }
    }
    count
  }

  fn scan_regexp(src: &str) -> usize {
    let mut scanner = make_scanner(src);
    let mut count = 0usize;
    // 上限护栏: 每个 token 至多消耗一次循环, 防止状态机异常时死循环
    let mut guard = src.len() + 16;
    while guard > 0 {
      guard -= 1;
      let next = scanner.peek();
      if next == Token::Eos {
        count += 1;
        break;
      }
      if next == Token::Div {
        // '/' 歧义路径: 尝试按正则字面量重扫下一 token
        if scanner.scan_regexp_pattern() {
          let _ = scanner.scan_regexp_flags();
        }
      }
      let _ = scanner.next_token();
      count += 1;
    }
    count
  }

  fn lookahead(src: &str, rounds: usize) -> usize {
    let mut scanner = make_scanner(src);
    // 预取填满 4 槽环形缓冲 (当前 / peek / peek_ahead / peek_ahead_ahead)
    let _ = scanner.peek_ahead();
    let _ = scanner.peek_ahead_ahead();
    let mut hits = 0usize;
    for _ in 0..rounds {
      let a = scanner.peek_ahead();
      let b = scanner.peek_ahead_ahead();
      // 不消费 token, 全部走缓存命中路径
      hits += (a != Token::Uninitialized) as usize + (b != Token::Uninitialized) as usize;
    }
    hits
  }

  fn literal_fill(units: usize, reps: usize) -> usize {
    let mut buffer = LiteralBuffer::new();
    let mut total = 0usize;
    for _ in 0..reps {
      buffer.start();
      for i in 0..units {
        buffer.add_char(u32::from(b'a') + (i % 26) as u32);
      }
      let _ = buffer.equals(b"abcdefghijklmnopqrstuvwxyz");
      total += buffer.len();
    }
    total
  }
}

/// 对比实现: 单遍手写简易 tokenizer (最小等价路径, feature bench-naive)
#[cfg(feature = "bench-naive")]
pub struct SimpleTokenizer;

/// 简易 tokenizer 谓词: 标识符起始 (ASCII 字母 / '_' / '$' / 非 ASCII 字节一律视作起始)
#[cfg(feature = "bench-naive")]
fn is_ident_start(c: u8) -> bool {
  c.is_ascii_alphabetic() || c == b'_' || c == b'$' || c >= 0x80
}

/// 简易 tokenizer 谓词: 标识符续接
#[cfg(feature = "bench-naive")]
fn is_ident_part(c: u8) -> bool {
  is_ident_start(c) || c.is_ascii_digit()
}

/// 跳过空白与 '//' 行注释 / '/*' 块注释, 返回首个非 trivia 位置
#[cfg(feature = "bench-naive")]
fn skip_trivia(b: &[u8], mut i: usize) -> usize {
  loop {
    match b.get(i) {
      None => return i,
      Some(&c) => {
        if c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' {
          i += 1;
        } else if c == b'/' && b.get(i + 1) == Some(&b'/') {
          while let Some(&x) = b.get(i) {
            if x == b'\n' {
              break;
            }
            i += 1;
          }
        } else if c == b'/' && b.get(i + 1) == Some(&b'*') {
          i += 2;
          while b.get(i).is_some() {
            if b[i] == b'*' && b.get(i + 1) == Some(&b'/') {
              i += 2;
              break;
            }
            i += 1;
          }
        } else {
          return i;
        }
      }
    }
  }
}

/// 从已定位到 token 起点的位置消费一个 token, 返回结束位置
#[cfg(feature = "bench-naive")]
fn token_end(b: &[u8], i: usize) -> usize {
  match b.get(i) {
    None => i,
    Some(&c) => {
      let mut j = i + 1;
      match c {
        b'"' | b'\'' => {
          while let Some(&x) = b.get(j) {
            if x == b'\\' {
              j += 2;
              continue;
            }
            j += 1;
            if x == c {
              break;
            }
          }
          j
        }
        b'0'..=b'9' => {
          // 数字: 贪婪吞掉字母/数字/'.'/'_' 即可, 不校验 ECMAScript 文法
          while matches!(b.get(j), Some(&x) if x.is_ascii_alphanumeric() || x == b'.' || x == b'_')
          {
            j += 1;
          }
          j
        }
        _ => {
          if is_ident_start(c) {
            while matches!(b.get(j), Some(&x) if is_ident_part(x)) {
              j += 1;
            }
          }
          j
        }
      }
    }
  }
}

/// 定位并消费一个 token (含跳过前置 trivia), 返回结束位置; 已处 EOF 时返回原位
#[cfg(feature = "bench-naive")]
fn next_token(b: &[u8], from: usize) -> usize {
  let start = skip_trivia(b, from);
  if start >= b.len() {
    start
  } else {
    token_end(b, start)
  }
}

/// '/' 处正则起始歧义的最小等价路径: 吞到下一个未转义 '/' 或行尾, 再吞 flags
#[cfg(feature = "bench-naive")]
fn regexp_end(b: &[u8], start: usize) -> usize {
  let mut j = start + 1;
  let mut closed = false;
  while let Some(&x) = b.get(j) {
    if x == b'\\' {
      j += 2;
      continue;
    }
    if x == b'\n' {
      break;
    }
    j += 1;
    if x == b'/' {
      closed = true;
      break;
    }
  }
  if closed {
    while matches!(b.get(j), Some(&x) if is_ident_part(x)) {
      j += 1;
    }
  }
  j.max(start + 1)
}

#[cfg(feature = "bench-naive")]
impl ParseBench for SimpleTokenizer {
  const NAME: &'static str = "naive-tokenizer";

  fn scan_tokens(src: &str) -> usize {
    let b = src.as_bytes();
    let mut i = 0usize;
    let mut count = 0usize;
    loop {
      let j = next_token(b, i);
      if j == i {
        break;
      }
      count += 1;
      i = j;
    }
    count
  }

  fn scan_regexp(src: &str) -> usize {
    let b = src.as_bytes();
    let mut i = 0usize;
    let mut count = 0usize;
    loop {
      let start = skip_trivia(b, i);
      if start >= b.len() {
        break;
      }
      let is_div = b[start] == b'/' && !matches!(b.get(start + 1), Some(&b'/') | Some(&b'*'));
      let j = if is_div {
        regexp_end(b, start)
      } else {
        token_end(b, start)
      };
      count += 1;
      i = j.max(start + 1);
    }
    count
  }

  fn lookahead(src: &str, rounds: usize) -> usize {
    let b = src.as_bytes();
    // 无缓存等价路径: 每轮都从当前位置重新扫后两个 token (恒走 miss 成本)
    let i = next_token(b, 0);
    let mut hits = 0usize;
    for _ in 0..rounds {
      let a = next_token(b, i);
      let c = next_token(b, a);
      hits += (a != i) as usize + (c != a) as usize;
    }
    hits
  }

  fn literal_fill(units: usize, reps: usize) -> usize {
    let mut buffer: Vec<u8> = Vec::with_capacity(units);
    let mut total = 0usize;
    for _ in 0..reps {
      buffer.clear();
      for i in 0..units {
        buffer.push(b'a' + (i % 26) as u8);
      }
      total += buffer.len();
    }
    total
  }
}
