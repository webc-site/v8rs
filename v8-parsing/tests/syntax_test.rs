//! C++ 官方 V8 parsing-unittest.cc 核心与极端 ECMAScript 语法转写测试
//! 迁移自 v8/test/unittests/parser/parsing-unittest.cc 与 test-parsing.cc

use aok::{OK, Void};
use v8_parsing::{Scanner, ScannerFlags, ScannerStream, Token};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn scan_tokens(src: &str) -> Vec<Token> {
  let mut scanner = Scanner::with_flags(ScannerStream::for_testing(src), ScannerFlags::default());
  scanner.initialize();
  let mut tokens = Vec::new();
  loop {
    let t = scanner.next_token();
    tokens.push(t);
    if t == Token::Eos {
      break;
    }
  }
  tokens
}

fn scan_tokens_module(src: &str) -> Vec<Token> {
  let flags = ScannerFlags { is_module: true };
  let mut scanner = Scanner::with_flags(ScannerStream::for_testing(src), flags);
  scanner.initialize();
  let mut tokens = Vec::new();
  loop {
    let t = scanner.next_token();
    tokens.push(t);
    if t == Token::Eos {
      break;
    }
  }
  tokens
}

/// 1. 自动分号补全 (ASI) 与除法/正则词法模式
#[test]
fn test_asi_and_regex_division_disambiguation() -> Void {
  // 换行后词法分析器按 Div 处理，结合 scan_regexp_pattern 切换
  let mut scanner = Scanner::with_flags(
    ScannerStream::for_testing("a = b\n/pattern/g"),
    ScannerFlags::default(),
  );
  scanner.initialize();
  assert_eq!(scanner.next_token(), Token::Identifier);
  assert_eq!(scanner.next_token(), Token::Assign);
  assert_eq!(scanner.next_token(), Token::Identifier);
  assert_eq!(scanner.peek(), Token::Div);
  assert!(scanner.scan_regexp_pattern());
  let flags = scanner.scan_regexp_flags();
  assert_eq!(flags, Some(1 << 0));
  assert_eq!(scanner.next_token(), Token::RegExpLiteral);
  assert_eq!(scanner.next_token(), Token::Eos);

  // 无换行连续除法表达式
  let tokens2 = scan_tokens("a = b / c / g");
  assert_eq!(tokens2[0], Token::Identifier);
  assert_eq!(tokens2[1], Token::Assign);
  assert_eq!(tokens2[2], Token::Identifier);
  assert_eq!(tokens2[3], Token::Div);
  assert_eq!(tokens2[4], Token::Identifier);
  assert_eq!(tokens2[5], Token::Div);
  assert_eq!(tokens2[6], Token::Identifier);
  assert_eq!(tokens2[7], Token::Eos);

  OK
}

/// 2. 数值分隔符 (Numeric Separators) 与进制字面量
#[test]
fn test_numeric_separators_and_radix_literals() -> Void {
  // 二进制/八进制/十六进制数值分隔符
  let tokens = scan_tokens("0b1010_0101 0o755_123 0xCAFE_BABE 1_000_000_000n");
  assert_eq!(tokens[0], Token::Number);
  assert_eq!(tokens[1], Token::Number);
  assert_eq!(tokens[2], Token::Number);
  assert_eq!(tokens[3], Token::BigInt);
  assert_eq!(tokens[4], Token::Eos);

  OK
}

/// 3. 可选链 (Optional Chaining) 与空值合并 (Nullish Coalescing)
#[test]
fn test_optional_chaining_and_nullish_coalescing() -> Void {
  let tokens = scan_tokens("a?.b?.[c]?.() ?? defaultVal");
  assert_eq!(tokens[0], Token::Identifier);
  assert_eq!(tokens[1], Token::QuestionPeriod);
  assert_eq!(tokens[2], Token::Identifier);
  assert_eq!(tokens[3], Token::QuestionPeriod);
  assert_eq!(tokens[4], Token::LeftBracket);
  assert_eq!(tokens[5], Token::Identifier);
  assert_eq!(tokens[6], Token::RightBracket);
  assert_eq!(tokens[7], Token::QuestionPeriod);
  assert_eq!(tokens[8], Token::LeftParen);
  assert_eq!(tokens[9], Token::RightParen);
  assert_eq!(tokens[10], Token::Nullish);
  assert_eq!(tokens[11], Token::Identifier);
  assert_eq!(tokens[12], Token::Eos);

  OK
}

/// 4. 逻辑赋值运算符 (Logical Assignment Operators)
#[test]
fn test_logical_assignment_operators() -> Void {
  let tokens = scan_tokens("x &&= y; a ||= b; c ??= d;");
  assert_eq!(tokens[0], Token::Identifier);
  assert_eq!(tokens[1], Token::AssignAnd);
  assert_eq!(tokens[2], Token::Identifier);
  assert_eq!(tokens[3], Token::Semicolon);

  assert_eq!(tokens[4], Token::Identifier);
  assert_eq!(tokens[5], Token::AssignOr);
  assert_eq!(tokens[6], Token::Identifier);
  assert_eq!(tokens[7], Token::Semicolon);

  assert_eq!(tokens[8], Token::Identifier);
  assert_eq!(tokens[9], Token::AssignNullish);
  assert_eq!(tokens[10], Token::Identifier);
  assert_eq!(tokens[11], Token::Semicolon);

  OK
}

/// 5. 箭头函数与解构参数语法
#[test]
fn test_arrow_function_and_destructuring_tokens() -> Void {
  let tokens = scan_tokens("({ a: [b = 1, { c }] } = {}) => a + b + c;");
  assert_eq!(tokens[0], Token::LeftParen);
  assert_eq!(tokens[1], Token::LeftBrace);
  assert_eq!(tokens[2], Token::Identifier);
  assert_eq!(tokens[3], Token::Colon);
  assert_eq!(tokens[4], Token::LeftBracket);
  assert_eq!(tokens[5], Token::Identifier);
  assert_eq!(tokens[6], Token::Assign);
  assert_eq!(tokens[7], Token::Smi);
  assert_eq!(tokens[8], Token::Comma);
  assert_eq!(tokens[9], Token::LeftBrace);
  assert_eq!(tokens[10], Token::Identifier);
  assert_eq!(tokens[11], Token::RightBrace);
  assert_eq!(tokens[12], Token::RightBracket);
  assert_eq!(tokens[13], Token::RightBrace);
  assert_eq!(tokens[14], Token::Assign);
  assert_eq!(tokens[15], Token::LeftBrace);
  assert_eq!(tokens[16], Token::RightBrace);
  assert_eq!(tokens[17], Token::RightParen);
  assert_eq!(tokens[18], Token::Arrow);

  OK
}

/// 6. 私有标识符 (Private Identifier #field)
#[test]
fn test_private_identifiers() -> Void {
  let tokens = scan_tokens("class Foo { #bar = 1; get #baz() { return this.#bar; } }");
  assert_eq!(tokens[0], Token::Class);
  assert_eq!(tokens[1], Token::Identifier);
  assert_eq!(tokens[2], Token::LeftBrace);
  assert_eq!(tokens[3], Token::PrivateName);
  assert_eq!(tokens[4], Token::Assign);
  assert_eq!(tokens[5], Token::Smi);
  assert_eq!(tokens[6], Token::Semicolon);
  assert_eq!(tokens[7], Token::Get);
  assert_eq!(tokens[8], Token::PrivateName);

  OK
}

/// 7. 模块顶层 Await 与动态导入
#[test]
fn test_top_level_await_and_dynamic_import() -> Void {
  let tokens = scan_tokens_module("await import('./module.js');");
  assert_eq!(tokens[0], Token::Await);
  assert_eq!(tokens[1], Token::Import);
  assert_eq!(tokens[2], Token::LeftParen);
  assert_eq!(tokens[3], Token::String);
  assert_eq!(tokens[4], Token::RightParen);
  assert_eq!(tokens[5], Token::Semicolon);

  OK
}
