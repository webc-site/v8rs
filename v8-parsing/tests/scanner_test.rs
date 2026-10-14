//! Scanner 单元测试 (规约先行迁移)
//!
//! 迁移自 v8/test/unittests/parser/scanner-unittest.cc (5 用例)
//! 与 parsing-unittest.cc 中纯 scanner 用例 (4 用例), 测试名保留原名。

use aok::OK;
use v8_parsing::{
  BookmarkScope, CharacterStream, LiteralBuffer, OneByteCharacterStream,
  OwnedTwoByteCharacterStream, Scanner, ScannerFlags, ScannerStream, SourceStream, Token,
  TwoByteCharacterStream,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 构建测试扫描器 (对应原版 make_scanner: ScannerStream::ForTesting)
fn make_scanner<'a>(src: &'a str) -> Scanner<SourceStream<'a>> {
  let mut scanner = Scanner::with_flags(ScannerStream::for_testing(src), ScannerFlags::default());
  scanner.initialize();
  scanner
}

/// 构建指定 is_module 的测试扫描器
fn make_scanner_module<'a>(src: &'a str, is_module: bool) -> Scanner<SourceStream<'a>> {
  let flags = ScannerFlags { is_module };
  let mut scanner = Scanner::with_flags(ScannerStream::for_testing(src), flags);
  scanner.initialize();
  scanner
}

/// 扫描全部 token (含 Eos, 对应原版 do-while 收集循环)
fn scan_all(src: &str) -> Vec<Token> {
  let mut scanner = make_scanner(src);
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

const SRC_SIMPLE: &str = "function foo() { var x = 2 * a() + b; }";

// ---- 以下迁移自 scanner-unittest.cc ----

/// 原版 TEST_F(ScannerTest, HasImmediateCommaOrRightBracket)
#[test]
fn has_immediate_comma_or_right_bracket() -> aok::Void {
  struct Case {
    source: &'static str,
    literal: Token,
    following: Token,
    is_immediate_delimiter: bool,
  }
  let cases = [
    Case {
      source: "1,2",
      literal: Token::Smi,
      following: Token::Comma,
      is_immediate_delimiter: true,
    },
    Case {
      source: "1]",
      literal: Token::Smi,
      following: Token::RightBracket,
      is_immediate_delimiter: true,
    },
    Case {
      source: "1 ,2",
      literal: Token::Smi,
      following: Token::Comma,
      is_immediate_delimiter: false,
    },
    Case {
      source: "1\n]",
      literal: Token::Smi,
      following: Token::RightBracket,
      is_immediate_delimiter: false,
    },
    Case {
      source: "1/*comment*/,2",
      literal: Token::Smi,
      following: Token::Comma,
      is_immediate_delimiter: false,
    },
    Case {
      source: "1//comment\n]",
      literal: Token::Smi,
      following: Token::RightBracket,
      is_immediate_delimiter: false,
    },
    Case {
      source: "1n]",
      literal: Token::BigInt,
      following: Token::RightBracket,
      is_immediate_delimiter: true,
    },
    Case {
      source: "'s',2",
      literal: Token::String,
      following: Token::Comma,
      is_immediate_delimiter: true,
    },
    Case {
      source: "null]",
      literal: Token::NullLiteral,
      following: Token::RightBracket,
      is_immediate_delimiter: true,
    },
    Case {
      source: "1",
      literal: Token::Smi,
      following: Token::Eos,
      is_immediate_delimiter: false,
    },
    Case {
      source: "1/*",
      literal: Token::Smi,
      following: Token::Illegal,
      is_immediate_delimiter: false,
    },
    Case {
      source: "1+2]",
      literal: Token::Smi,
      following: Token::Add,
      is_immediate_delimiter: false,
    },
    Case {
      source: "1/*]*/+2",
      literal: Token::Smi,
      following: Token::Add,
      is_immediate_delimiter: false,
    },
    Case {
      source: "'s'.length]",
      literal: Token::String,
      following: Token::Period,
      is_immediate_delimiter: false,
    },
  ];

  for case in &cases {
    for lookahead in 0..3 {
      let mut scanner = make_scanner(case.source);
      if lookahead > 0 {
        scanner.peek_ahead();
      }
      if lookahead > 1 {
        scanner.peek_ahead_ahead();
      }
      assert_eq!(
        case.is_immediate_delimiter && lookahead == 0,
        scanner.has_immediate_comma_or_right_bracket(),
        "source={}, lookahead={}",
        case.source,
        lookahead
      );
      assert_eq!(
        case.literal,
        scanner.next_token(),
        "literal source={}",
        case.source
      );
      assert_eq!(
        case.following,
        scanner.next_token(),
        "following source={}",
        case.source
      );

      // 错误状态扫描器: 立即分隔符必为 false, Next 只出 Illegal
      let mut error_scanner = make_scanner(case.source);
      if lookahead > 0 {
        error_scanner.peek_ahead();
      }
      if lookahead > 1 {
        error_scanner.peek_ahead_ahead();
      }
      error_scanner.set_parser_error();
      assert!(
        !error_scanner.has_immediate_comma_or_right_bracket(),
        "error scanner source={}",
        case.source
      );
      assert_eq!(
        Token::Illegal,
        error_scanner.next_token(),
        "error source={}",
        case.source
      );
    }
  }
  OK
}

/// 原版 TEST_F(ScannerTest, Bookmarks)
#[test]
fn bookmarks() -> aok::Void {
  // 先扫一遍记录参考 token 序列
  let tokens = scan_all(SRC_SIMPLE);

  // 对每个位置: 扫描到位 → 设书签 → 再扫几个 → 回退到书签 → 扫到尾, 全程对照参考序列
  for bookmark_pos in 0..tokens.len() {
    let mut scanner = make_scanner(SRC_SIMPLE);
    let mut bookmark_pos_val = None;

    for (i, &expected_token) in tokens
      .iter()
      .enumerate()
      .take((bookmark_pos + 10).min(tokens.len()))
    {
      if i == bookmark_pos {
        bookmark_pos_val = Some(scanner.peek_location().beg_pos);
      }
      assert_eq!(
        expected_token,
        scanner.next_token(),
        "i={} bookmark_pos={}",
        i,
        bookmark_pos
      );
    }

    let mut bookmark = BookmarkScope::new(&scanner);
    bookmark.set(bookmark_pos_val.unwrap());
    scanner.apply_bookmark(&mut bookmark);
    for (i, &expected_token) in tokens.iter().enumerate().skip(bookmark_pos) {
      assert_eq!(
        expected_token,
        scanner.next_token(),
        "replay i={} bookmark_pos={}",
        i,
        bookmark_pos
      );
    }
  }
  OK
}

/// 原版 TEST_F(ScannerTest, AllThePushbacks)
#[test]
fn all_the_pushbacks() -> aok::Void {
  let test_cases: &[(&str, Vec<Token>)] = &[
    (
      "<-x",
      vec![Token::LessThan, Token::Sub, Token::Identifier, Token::Eos],
    ),
    (
      "<!x",
      vec![Token::LessThan, Token::Not, Token::Identifier, Token::Eos],
    ),
    (
      "<!-x",
      vec![
        Token::LessThan,
        Token::Not,
        Token::Sub,
        Token::Identifier,
        Token::Eos,
      ],
    ),
    ("<!-- xx -->\nx", vec![Token::Identifier, Token::Eos]),
  ];

  for (src, expected) in test_cases {
    let mut scanner = make_scanner(src);
    for tok in expected {
      assert_eq!(*tok, scanner.next_token(), "src={}", src);
    }
    assert_eq!(Token::Eos, scanner.next_token(), "final src={}", src);
  }
  OK
}

/// 原版 TEST_F(ScannerTest, PeekAheadAheadAwaitUsingDeclaration)
#[test]
fn peek_ahead_ahead_await_using_declaration() -> aok::Void {
  let src: &'static str = "await using a = 2;";
  let tokens = scan_all(src);

  let mut scanner = make_scanner(src);
  let beg = scanner.peek_location().beg_pos;
  let mut bookmark = BookmarkScope::new(&scanner);
  bookmark.set(beg);
  scanner.apply_bookmark(&mut bookmark);

  assert_eq!(tokens[0], scanner.next_token());
  assert_eq!(tokens[1], scanner.peek());
  assert_eq!(tokens[2], scanner.peek_ahead());
  assert_eq!(tokens[3], scanner.peek_ahead_ahead());
  OK
}

/// 原版 TEST_F(ScannerTest, PeekAheadAheadAwaitExpression)
#[test]
fn peek_ahead_ahead_await_expression() -> aok::Void {
  let src: &'static str = "await using + 5;";
  let tokens = scan_all(src);

  let mut scanner = make_scanner(src);
  let beg = scanner.peek_location().beg_pos;
  let mut bookmark = BookmarkScope::new(&scanner);
  bookmark.set(beg);
  scanner.apply_bookmark(&mut bookmark);

  assert_eq!(tokens[0], scanner.next_token());
  assert_eq!(tokens[1], scanner.peek());
  assert_eq!(tokens[2], scanner.peek_ahead());
  assert_eq!(tokens[3], scanner.peek_ahead_ahead());
  OK
}

// ---- 以下迁移自 parsing-unittest.cc (纯 scanner 用例) ----

/// 原版 TEST_F(ParsingTest, ScanKeywords)
#[test]
fn scan_keywords() -> aok::Void {
  // 遍历全部 Token: 关键字整串扫描命中自身, 截短/追加/替换后必为 Identifier
  for id in 0..(Token::COUNT as u8) {
    let Some(tok) = Token::from_repr(id) else {
      continue;
    };
    let Some(keyword) = tok.as_str() else {
      continue;
    };
    if !tok.is_keyword() {
      continue;
    }
    // 整串: 命中自身 + Eos
    {
      let mut scanner = make_scanner(keyword);
      assert_eq!(tok, scanner.next_token(), "keyword={}", keyword);
      assert_eq!(Token::Eos, scanner.next_token(), "keyword={}", keyword);
    }
    // 截短: Identifier (长度 1 的关键字跳过)
    if keyword.len() > 1 {
      let truncated = &keyword[..keyword.len() - 1];
      let mut scanner = make_scanner(truncated);
      assert_eq!(
        Token::Identifier,
        scanner.next_token(),
        "truncated={}",
        truncated
      );
      assert_eq!(Token::Eos, scanner.next_token(), "truncated={}", truncated);
    }
    // 追加: Identifier
    for extra in ['z', '0', '_'] {
      let mutated = format!("{keyword}{extra}");
      let mut scanner = make_scanner(&mutated);
      assert_eq!(
        Token::Identifier,
        scanner.next_token(),
        "appended={}",
        mutated
      );
      assert_eq!(Token::Eos, scanner.next_token(), "appended={}", mutated);
    }
    // 替换末字符: Identifier
    {
      let mut s = String::from(keyword);
      s.replace_range(s.len() - 1.., "_");
      let mut scanner = make_scanner(&s);
      assert_eq!(Token::Identifier, scanner.next_token(), "replaced={}", s);
      assert_eq!(Token::Eos, scanner.next_token(), "replaced={}", s);
    }
  }
  OK
}

/// 原版 TEST_F(ParsingTest, ScanHtmlComments)
#[test]
fn scan_html_comments() -> aok::Void {
  let src: &'static str = "a <!-- b --> c";
  // module 模式: HTML 注释非法
  {
    let mut scanner = make_scanner_module(src, true);
    assert_eq!(Token::Identifier, scanner.next_token());
    assert_eq!(Token::Illegal, scanner.next_token());
  }
  // sloppy 模式: 跳过 HTML 注释
  {
    let mut scanner = make_scanner_module(src, false);
    assert_eq!(Token::Identifier, scanner.next_token());
    assert_eq!(Token::Eos, scanner.next_token());
  }
  OK
}

/// 原版 TEST_F(ParsingTest, StreamScanner) — TestStreamScanner 辅助迁移
#[test]
fn stream_scanner() -> aok::Void {
  fn test_stream_scanner(src: &str, expected: &[Token], skip_pos: usize, skip_to: usize) {
    let mut scanner = make_scanner(src);
    let mut i = 0;
    loop {
      let want = expected[i];
      let got = scanner.next_token();
      assert_eq!(
        want,
        got,
        "src={} i={} want={} got={}",
        src,
        i,
        want.name(),
        got.name()
      );
      if scanner.location().end_pos == skip_pos {
        scanner.seek_forward(skip_to);
      }
      i += 1;
      if expected[i] == Token::Illegal {
        break;
      }
    }
  }

  // 1. 常规序列
  let str1: &'static str = "{ foo get for : */ <- \n\n /*foo*/ bib";
  test_stream_scanner(
    str1,
    &[
      Token::LeftBrace,
      Token::Identifier,
      Token::Get,
      Token::For,
      Token::Colon,
      Token::Mul,
      Token::Div,
      Token::LessThan,
      Token::Sub,
      Token::Identifier,
      Token::Eos,
      Token::Illegal,
    ],
    0,
    0,
  );

  // 2. SeekForward 跳过中段
  let str2: &'static str = "case default const {THIS\nPART\nSKIPPED} do";
  assert_eq!(b'{', str2.as_bytes()[19]);
  assert_eq!(b'}', str2.as_bytes()[37]);
  test_stream_scanner(
    str2,
    &[
      Token::Case,
      Token::Default,
      Token::Const,
      Token::LeftBrace,
      // 此处跳过被扫描的部分
      Token::RightBrace,
      Token::Do,
      Token::Eos,
      Token::Illegal,
    ],
    20,
    37,
  );

  // 3. Skip zero-four RightBrace
  let str3: &'static str = "{}}}}";
  let mut expectations = [
    Token::LeftBrace,
    Token::RightBrace,
    Token::RightBrace,
    Token::RightBrace,
    Token::RightBrace,
    Token::Eos,
    Token::Illegal,
  ];
  for i in 0..=4usize {
    expectations[6 - i] = Token::Illegal;
    expectations[5 - i] = Token::Eos;
    test_stream_scanner(str3, &expectations, 1, 1 + i);
  }
  OK
}

/// 原版 TEST_F(ParsingTest, RegExpScanning) — TestScanRegExp 辅助迁移
#[test]
fn regexp_scanning() -> aok::Void {
  fn test_scan_regexp(re_source: &'static str, expected: &str) {
    let mut scanner = make_scanner(re_source);
    let start = scanner.peek();
    assert!(
      start == Token::Div || start == Token::AssignDiv,
      "re_source={}",
      re_source
    );
    assert!(
      scanner.scan_regexp_pattern(),
      "pattern re_source={}",
      re_source
    );
    scanner.next_token(); // 当前 token 即正则字面量
    // 原版经 AstValueFactory 取 cooked 字面量; 此处直接取单字节字面量 (同一数据源)
    let actual = scanner.bigint_literal();
    assert_eq!(expected.as_bytes(), actual, "re_source={}", re_source);
  }

  // 附加垃圾: 正则只扫到 flipperwald 前的斜杠为止
  test_scan_regexp("/b/flipperwald", "b");
  // 不完整逃逸不会吞掉终结斜杠
  test_scan_regexp("/\\x/flipperwald", "\\x");
  test_scan_regexp("/\\u/flipperwald", "\\u");
  test_scan_regexp("/\\u1/flipperwald", "\\u1");
  test_scan_regexp("/\\u12/flipperwald", "\\u12");
  test_scan_regexp("/\\u123/flipperwald", "\\u123");
  test_scan_regexp("/\\c/flipperwald", "\\c");
  test_scan_regexp("/\\c//flipperwald", "\\c");
  // 字符类内的斜杠不终结
  test_scan_regexp("/[/]/flipperwald", "[/]");
  test_scan_regexp("/[\\s-/]/flipperwald", "[\\s-/]");
  // 字符类内不完整逃逸不会吞掉类结束符
  test_scan_regexp("/[\\c/]/flipperwald", "[\\c/]");
  test_scan_regexp("/[\\c]/flipperwald", "[\\c]");
  test_scan_regexp("/[\\x]/flipperwald", "[\\x]");
  test_scan_regexp("/[\\x1]/flipperwald", "[\\x1]");
  test_scan_regexp("/[\\u]/flipperwald", "[\\u]");
  test_scan_regexp("/[\\u1]/flipperwald", "[\\u1]");
  test_scan_regexp("/[\\u12]/flipperwald", "[\\u12]");
  test_scan_regexp("/[\\u123]/flipperwald", "[\\u123]");
  // 逃逸的 ']' 不会结束字符类
  test_scan_regexp("/[\\]/]/flipperwald", "[\\]/]");
  // 逃逸斜杠不终结
  test_scan_regexp("/\\//flipperwald", "\\/");
  // 以 '=' 开头同样适用
  test_scan_regexp("/=/", "=");
  test_scan_regexp("/=?/", "=?");
  OK
}

/// 数值与 BigInt 字面量扫描测试
#[test]
fn numbers_and_bigint() -> aok::Void {
  // 十六进制
  {
    let mut scanner = make_scanner("0x1f");
    assert_eq!(Token::Number, scanner.next_token());
    assert_eq!(31.0, scanner.double_value());
    assert_eq!(Token::Eos, scanner.next_token());
  }

  // 八进制
  {
    let mut scanner = make_scanner("0o77");
    assert_eq!(Token::Number, scanner.next_token());
    assert_eq!(63.0, scanner.double_value());
    assert_eq!(Token::Eos, scanner.next_token());
  }

  // 二进制
  {
    let mut scanner = make_scanner("0b1010");
    assert_eq!(Token::Number, scanner.next_token());
    assert_eq!(10.0, scanner.double_value());
    assert_eq!(Token::Eos, scanner.next_token());
  }

  // 浮点数与科学计数法
  {
    let mut scanner = make_scanner("3.25 1e3");
    assert_eq!(Token::Number, scanner.next_token());
    assert!((3.25 - scanner.double_value()).abs() < 1e-6);
    assert_eq!(Token::Number, scanner.next_token());
    assert_eq!(1000.0, scanner.double_value());
    assert_eq!(Token::Eos, scanner.next_token());
  }

  // Smi
  {
    let mut scanner = make_scanner("42");
    assert_eq!(Token::Smi, scanner.next_token());
    assert_eq!(42, scanner.smi_value());
    assert_eq!(42.0, scanner.double_value());
    assert_eq!(Token::Eos, scanner.next_token());
  }

  // BigInt
  {
    let mut scanner = make_scanner("12345678901234567890n");
    assert_eq!(Token::BigInt, scanner.next_token());
    assert_eq!(b"12345678901234567890", scanner.bigint_literal());
    assert_eq!(Token::Eos, scanner.next_token());
  }

  // 带下划线数值分隔符
  {
    let mut scanner = make_scanner("1_000_000");
    assert_eq!(Token::Smi, scanner.next_token());
    assert_eq!(1_000_000, scanner.smi_value());
    assert_eq!(Token::Eos, scanner.next_token());
  }

  OK
}

/// 字符串字面量转义测试
#[test]
fn string_escapes() -> aok::Void {
  // 单引号、双引号、转义字符
  let mut scanner = make_scanner(r#"'hello\nworld' "quote\"inside" '\x41\u0042'"#);
  assert_eq!(Token::String, scanner.next_token());
  assert_eq!(Token::String, scanner.next_token());
  assert_eq!(Token::String, scanner.next_token());
  assert_eq!(Token::Eos, scanner.next_token());
  OK
}

/// 模板字符串测试
#[test]
fn template_literals() -> aok::Void {
  // 完整无替换模板
  {
    let mut scanner = make_scanner("`simple template`");
    assert_eq!(Token::TemplateTail, scanner.next_token());
    assert_eq!(Token::Eos, scanner.next_token());
  }

  // 带替换模板: `head ${ x } tail`
  {
    let mut scanner = make_scanner("`head ${ x } tail`");
    assert_eq!(Token::TemplateSpan, scanner.next_token());
    assert_eq!(Token::Identifier, scanner.next_token());
    // 遇到 } 后切入模板尾扫描
    assert_eq!(Token::RightBrace, scanner.peek());
    assert_eq!(Token::TemplateTail, scanner.scan_template_continuation());
    assert_eq!(Token::TemplateTail, scanner.next_token());
    assert_eq!(Token::Eos, scanner.next_token());
  }

  OK
}

/// 源码行列计算测试
#[test]
fn source_positions() -> aok::Void {
  let src = "line1\nline2\r\nline3";
  let mut scanner = make_scanner(src);

  let t1 = scanner.next_token();
  assert_eq!(Token::Identifier, t1);
  let pos1 = scanner.source_position(scanner.location().beg_pos);
  assert_eq!(1, pos1.line);
  assert_eq!(1, pos1.column);

  let t2 = scanner.next_token();
  assert_eq!(Token::Identifier, t2);
  let pos2 = scanner.source_position(scanner.location().beg_pos);
  assert_eq!(2, pos2.line);
  assert_eq!(1, pos2.column);

  let t3 = scanner.next_token();
  assert_eq!(Token::Identifier, t3);
  let pos3 = scanner.source_position(scanner.location().beg_pos);
  assert_eq!(3, pos3.line);
  assert_eq!(1, pos3.column);

  OK
}

/// 字符流 Trait 与双字节 UTF-16 流测试
#[test]
fn character_streams() -> aok::Void {
  // 1. OneByteCharacterStream
  let bytes = b"abc";
  let mut s1 = OneByteCharacterStream::new(bytes);
  assert_eq!(3, s1.len());
  assert_eq!(b'a' as u32, s1.advance());
  assert_eq!(b'b' as u32, s1.peek());
  assert_eq!(b'b' as u32, s1.advance());
  s1.back();
  assert_eq!(b'b' as u32, s1.advance());
  assert_eq!(b'c' as u32, s1.advance());
  assert_eq!(v8_parsing::K_END_OF_INPUT, s1.advance());

  // 2. TwoByteCharacterStream
  let u16_chars = [0x4e2du16, 0x6587u16]; // "中文"
  let mut s2 = TwoByteCharacterStream::new(&u16_chars);
  assert_eq!(2, s2.len());
  assert_eq!(0x4e2d, s2.advance());
  assert_eq!(0x6587, s2.advance());
  assert_eq!(v8_parsing::K_END_OF_INPUT, s2.advance());

  // 3. OwnedTwoByteCharacterStream
  let mut s3 = OwnedTwoByteCharacterStream::new("hello".encode_utf16().collect());
  assert_eq!(5, s3.len());
  assert_eq!(b'h' as u32, s3.advance());
  s3.seek(4);
  assert_eq!(b'o' as u32, s3.advance());
  assert_eq!(v8_parsing::K_END_OF_INPUT, s3.advance());

  OK
}

/// LiteralBuffer 动态扩容与折叠测试
#[test]
fn literal_buffer() -> aok::Void {
  let mut buf = LiteralBuffer::new();
  buf.start();
  assert!(buf.is_one_byte());

  // 单字节字符追加
  for i in 0..300 {
    buf.add_char(b'a' as u32 + (i % 26));
  }
  assert_eq!(300, buf.len());
  assert!(buf.is_one_byte());
  assert_eq!(300, buf.one_byte_literal().len());

  // 升级到双字节
  buf.add_char(0x1234);
  assert!(!buf.is_one_byte());
  assert_eq!(301, buf.len());
  assert_eq!(301, buf.two_byte_literal().len());

  // 尝试 fold_to_latin1 (由于含 0x1234, 应该折叠失败保持双字节)
  assert!(!buf.fold_to_latin1());
  assert!(!buf.is_one_byte());

  // 纯 ASCII 双字节折叠
  let mut buf2 = LiteralBuffer::new();
  buf2.start();
  buf2.add_char(0x1234);
  buf2.start();
  buf2.add_char(b'x' as u32);
  assert!(buf2.is_one_byte());

  OK
}

/// 4 槽 Lookahead 环形缓冲状态机对抗测试
#[test]
fn lookahead_ring_buffer_integrity() -> aok::Void {
  let src = "first second third fourth";
  let mut scanner = make_scanner(src);

  // 1. peek() 对应 "first", beg_pos=0, end_pos=5
  assert_eq!(Token::Identifier, scanner.peek());
  let peek1_loc = scanner.peek_location();
  assert_eq!(0, peek1_loc.beg_pos);
  assert_eq!(5, peek1_loc.end_pos);

  // 2. peek_ahead() 对应 "second", 不得破坏 peek_location()
  assert_eq!(Token::Identifier, scanner.peek_ahead());
  assert_eq!(0, scanner.peek_location().beg_pos);
  assert_eq!(5, scanner.peek_location().end_pos);
  assert_eq!(6, scanner.peek_ahead_location().beg_pos);
  assert_eq!(12, scanner.peek_ahead_location().end_pos);

  // 3. peek_ahead_ahead() 对应 "third", 同样不得破坏前两者的 location
  assert_eq!(Token::Identifier, scanner.peek_ahead_ahead());
  assert_eq!(0, scanner.peek_location().beg_pos);
  assert_eq!(5, scanner.peek_location().end_pos);
  assert_eq!(6, scanner.peek_ahead_location().beg_pos);
  assert_eq!(12, scanner.peek_ahead_location().end_pos);
  assert_eq!(13, scanner.peek_ahead_ahead_location().beg_pos);
  assert_eq!(18, scanner.peek_ahead_ahead_location().end_pos);

  // 4. 实际消费 next_token(): 依次出 "first", "second", "third", "fourth", 验证位置绝对完整
  assert_eq!(Token::Identifier, scanner.next_token());
  assert_eq!(0, scanner.location().beg_pos);
  assert_eq!(5, scanner.location().end_pos);

  assert_eq!(Token::Identifier, scanner.next_token());
  assert_eq!(6, scanner.location().beg_pos);
  assert_eq!(12, scanner.location().end_pos);

  assert_eq!(Token::Identifier, scanner.next_token());
  assert_eq!(13, scanner.location().beg_pos);
  assert_eq!(18, scanner.location().end_pos);

  assert_eq!(Token::Identifier, scanner.next_token());
  assert_eq!(19, scanner.location().beg_pos);
  assert_eq!(25, scanner.location().end_pos);

  assert_eq!(Token::Eos, scanner.next_token());

  // 5. 乱序调用安全: 直接调用 peek_ahead_ahead() 必须自动安全初始化 peek_ahead()
  let mut scanner2 = make_scanner(src);
  assert_eq!(Token::Identifier, scanner2.peek_ahead_ahead());
  assert_eq!(Token::Identifier, scanner2.next_token()); // first
  assert_eq!(Token::Identifier, scanner2.next_token()); // second
  assert_eq!(Token::Identifier, scanner2.next_token()); // third
  assert_eq!(Token::Identifier, scanner2.next_token()); // fourth
  assert_eq!(Token::Eos, scanner2.next_token());

  OK
}

/// BookmarkScope 与 Lookahead 环形缓冲回溯复位测试
#[test]
fn bookmark_with_lookahead_reset() -> aok::Void {
  let src = "alpha beta gamma delta epsilon";
  let mut scanner = make_scanner(src);

  // 消费第一个 token: alpha
  assert_eq!(Token::Identifier, scanner.next_token());

  // 在 beta 位置设置书签
  let bookmark_pos = scanner.peek_location().beg_pos;
  assert_eq!(6, bookmark_pos);
  let mut bookmark = BookmarkScope::new(&scanner);
  bookmark.set(bookmark_pos);

  // 推进 lookahead 缓冲 (填充 next_next 与 next_next_next)
  assert_eq!(Token::Identifier, scanner.peek()); // beta
  assert_eq!(Token::Identifier, scanner.peek_ahead()); // gamma
  assert_eq!(Token::Identifier, scanner.peek_ahead_ahead()); // delta

  // 再消费两个 token
  assert_eq!(Token::Identifier, scanner.next_token()); // beta
  assert_eq!(Token::Identifier, scanner.next_token()); // gamma

  // 回滚书签到 beta
  scanner.apply_bookmark(&mut bookmark);

  // 验证回溯后环形缓冲彻底重置，无旧 Token 脏读:
  assert_eq!(Token::Identifier, scanner.peek());
  assert_eq!(6, scanner.peek_location().beg_pos);
  assert_eq!(Token::Identifier, scanner.next_token()); // beta
  assert_eq!(6, scanner.location().beg_pos);
  assert_eq!(10, scanner.location().end_pos);

  assert_eq!(Token::Identifier, scanner.next_token()); // gamma
  assert_eq!(Token::Identifier, scanner.next_token()); // delta
  assert_eq!(Token::Identifier, scanner.next_token()); // epsilon
  assert_eq!(Token::Eos, scanner.next_token());

  OK
}

/// 数值与 BigInt 扫描器异常报错对抗测试
#[test]
fn number_scanner_adversarial() -> aok::Void {
  use v8_parsing::MessageTemplate;

  // 1. 连续分隔符 1__2 报错
  {
    let mut scanner = make_scanner("1__2");
    assert_eq!(Token::Illegal, scanner.next_token());
    assert_eq!(MessageTemplate::ContinuousNumericSeparator, scanner.error());
  }

  // 2. 尾部分隔符 1_ 报错
  {
    let mut scanner = make_scanner("1_");
    assert_eq!(Token::Illegal, scanner.next_token());
    assert_eq!(MessageTemplate::TrailingNumericSeparator, scanner.error());
  }

  // 3. 前导零加分隔符 0_1 报错
  {
    let mut scanner = make_scanner("0_1");
    assert_eq!(Token::Illegal, scanner.next_token());
    assert_eq!(MessageTemplate::ZeroDigitNumericSeparator, scanner.error());
  }

  // 4. 浮点 BigInt 报错: 1.2n
  {
    let mut scanner = make_scanner("1.2n");
    assert_eq!(Token::Illegal, scanner.next_token());
  }

  // 5. 科学计数法 BigInt 报错: 1e2n
  {
    let mut scanner = make_scanner("1e2n");
    assert_eq!(Token::Illegal, scanner.next_token());
  }

  // 6. 前导零 BigInt 报错: 077n, 088n
  {
    let mut scanner = make_scanner("077n");
    assert_eq!(Token::Illegal, scanner.next_token());
  }
  {
    let mut scanner = make_scanner("088n");
    assert_eq!(Token::Illegal, scanner.next_token());
  }

  // 7. 隐式八进制 077 (严格模式 StrictOctalLiteral 标记, 非严格可转 double 63)
  {
    let mut scanner = make_scanner("077");
    assert_eq!(Token::Number, scanner.next_token());
    assert_eq!(63.0, scanner.double_value());
    assert!(scanner.octal_position().is_valid());
    assert_eq!(MessageTemplate::StrictOctalLiteral, scanner.octal_message());
  }

  // 8. 非法八进制 088 / 078 (StrictDecimalWithLeadingZero 标记)
  // V8 特性: 088 的 at_start 为 true, 优化直接产出 Token::Smi; 而 078 从八进制中途降级, at_start 为 false, 产出 Token::Number
  {
    let mut scanner = make_scanner("088");
    assert_eq!(Token::Smi, scanner.next_token());
    assert_eq!(88, scanner.smi_value());
    assert_eq!(88.0, scanner.double_value());
    assert!(scanner.octal_position().is_valid());
    assert_eq!(
      MessageTemplate::StrictDecimalWithLeadingZero,
      scanner.octal_message()
    );
  }
  {
    let mut scanner = make_scanner("078");
    assert_eq!(Token::Number, scanner.next_token());
    assert_eq!(78.0, scanner.double_value());
    assert!(scanner.octal_position().is_valid());
    assert_eq!(
      MessageTemplate::StrictDecimalWithLeadingZero,
      scanner.octal_message()
    );
  }

  // 9. BigInt 长度超限保护 (> 262144 字符)
  {
    let long_bigint = format!("1{}n", "0".repeat(262145));
    let mut scanner = make_scanner(&long_bigint);
    assert_eq!(Token::Illegal, scanner.next_token());
    assert_eq!(MessageTemplate::BigIntTooBig, scanner.error());
  }

  OK
}

/// 字符串与模板字符串转义隔离对抗测试
#[test]
fn string_and_template_escape_adversarial() -> aok::Void {
  use v8_parsing::MessageTemplate;

  // 1. Unicode 标量越界 \u{110000} 报错
  {
    let mut scanner = make_scanner(r#""\u{110000}""#);
    assert_eq!(Token::Illegal, scanner.next_token());
    assert_eq!(MessageTemplate::UndefinedUnicodeCodePoint, scanner.error());
  }

  // 2. 不完整十六进制 \x 报错
  {
    let mut scanner = make_scanner(r#""\x""#);
    assert_eq!(Token::Illegal, scanner.next_token());
    assert_eq!(MessageTemplate::InvalidHexEscapeSequence, scanner.error());
  }

  // 3. 严格模式八进制转义 \01 标记
  {
    let mut scanner = make_scanner(r#""\01""#);
    assert_eq!(Token::String, scanner.next_token());
    assert!(scanner.octal_position().is_valid());
    assert_eq!(MessageTemplate::StrictOctalEscape, scanner.octal_message());
  }

  // 4. 8/9 非八进制转义 \8
  {
    let mut scanner = make_scanner(r#""\8""#);
    assert_eq!(Token::String, scanner.next_token());
    assert!(scanner.octal_position().is_valid());
    assert_eq!(MessageTemplate::Strict8Or9Escape, scanner.octal_message());
  }

  // 5. 模板字符串中非法转义隔离 (Tagged template escape isolation)
  // `\x` 在普通模式下 scanner.has_error() 必须为 false (被 ErrorState 隔离),
  // 但 invalid_template_escape_message 必须记录 InvalidHexEscapeSequence
  {
    let mut scanner = make_scanner("`hello\\x`");
    assert_eq!(Token::TemplateTail, scanner.next_token());
    assert!(!scanner.has_error(), "scanner_error must be isolated");
    assert!(scanner.has_invalid_template_escape());
    assert_eq!(
      MessageTemplate::InvalidHexEscapeSequence,
      scanner.invalid_template_escape_message()
    );
  }

  // 6. 多层嵌套模板字符串状态转移: `${`${x}`}`
  {
    let src = "`head${`mid${x}end`}tail`";
    let mut scanner = make_scanner(src);

    // 外层模板头: `head${
    assert_eq!(Token::TemplateSpan, scanner.next_token());

    // 内层模板头: `mid${
    assert_eq!(Token::TemplateSpan, scanner.next_token());

    // 表达式 x
    assert_eq!(Token::Identifier, scanner.next_token());

    // 遇到内层 } 后调用 scan_template_continuation
    assert_eq!(Token::RightBrace, scanner.peek());
    assert_eq!(Token::TemplateTail, scanner.scan_template_continuation());
    assert_eq!(Token::TemplateTail, scanner.next_token()); // }end`

    // 遇到外层 } 后调用 scan_template_continuation
    assert_eq!(Token::RightBrace, scanner.peek());
    assert_eq!(Token::TemplateTail, scanner.scan_template_continuation());
    assert_eq!(Token::TemplateTail, scanner.next_token()); // }tail`

    assert_eq!(Token::Eos, scanner.next_token());
  }

  OK
}

/// 严格模式上下文关键字与标识符属性对抗测试
#[test]
fn context_keywords_and_identifier_predicates() -> aok::Void {
  // 1. is_strict_reserved_word 覆盖: yield, let, static, FutureStrictReservedWord, EscapedStrictReservedWord
  assert!(Token::Yield.is_strict_reserved_word());
  assert!(Token::Let.is_strict_reserved_word());
  assert!(Token::Static.is_strict_reserved_word());
  assert!(Token::FutureStrictReservedWord.is_strict_reserved_word());
  assert!(Token::EscapedStrictReservedWord.is_strict_reserved_word());

  assert!(!Token::Var.is_strict_reserved_word());
  assert!(!Token::Identifier.is_strict_reserved_word());
  assert!(!Token::Await.is_strict_reserved_word());

  // 2. is_valid_identifier 在不同模式下的判定
  // 普通标识符: 任意模式均有效
  assert!(Token::Identifier.is_valid_identifier(false, false, false));
  assert!(Token::Identifier.is_valid_identifier(true, true, true));

  // await: disallow_await 为 true 时无效
  assert!(Token::Await.is_valid_identifier(false, false, false));
  assert!(!Token::Await.is_valid_identifier(false, false, true));

  // yield: 严格模式下无效，generator 内无效，仅在 sloppy 且非 generator 下可作为标识符
  assert!(Token::Yield.is_valid_identifier(false, false, false));
  assert!(!Token::Yield.is_valid_identifier(true, false, false));
  assert!(!Token::Yield.is_valid_identifier(false, true, false));

  // 严格保留字 (如 implements/interface/package/private/protected/public 对应的 FutureStrictReservedWord, 以及 let/static):
  // 在 sloppy 模式下有效，在 strict 模式下无效
  assert!(Token::FutureStrictReservedWord.is_valid_identifier(false, false, false));
  assert!(!Token::FutureStrictReservedWord.is_valid_identifier(true, false, false));
  assert!(Token::Let.is_valid_identifier(false, false, false));
  assert!(!Token::Let.is_valid_identifier(true, false, false));

  // 3. 转义保留字扫描
  // \u0070ublic -> EscapedStrictReservedWord
  {
    let mut scanner = make_scanner(r#"\u0070ublic"#);
    assert_eq!(Token::EscapedStrictReservedWord, scanner.next_token());
  }
  // \u006cet -> EscapedStrictReservedWord
  {
    let mut scanner = make_scanner(r#"\u006cet"#);
    assert_eq!(Token::EscapedStrictReservedWord, scanner.next_token());
  }

  OK
}

/// 源码坐标 (SourcePosition) 各种换行符与多行模板字符串单调性测试
#[test]
fn source_position_various_line_terminators() -> aok::Void {
  // 测试: CRLF (\r\n), 单独 CR (\r), 单独 LF (\n), LS (\u2028), PS (\u2029)
  let src = "line1\r\nline2\rline3\nline4\u{2028}line5\u{2029}line6";
  let mut scanner = make_scanner(src);

  let tokens = ["line1", "line2", "line3", "line4", "line5", "line6"];
  for (idx, _name) in tokens.iter().enumerate() {
    assert_eq!(Token::Identifier, scanner.next_token());
    let pos = scanner.source_position(scanner.location().beg_pos);
    assert_eq!(idx + 1, pos.line, "token={} line", idx + 1);
    assert_eq!(1, pos.column, "token={} col", idx + 1);
  }
  assert_eq!(Token::Eos, scanner.next_token());

  // 多行模板字符串跨行坐标计算
  let tmpl_src = "`line1\nline2\r\nline3`\nnext";
  let mut scanner2 = make_scanner(tmpl_src);
  assert_eq!(Token::TemplateTail, scanner2.next_token());
  assert_eq!(Token::Identifier, scanner2.next_token()); // next
  let next_pos = scanner2.source_position(scanner2.location().beg_pos);
  assert_eq!(4, next_pos.line);
  assert_eq!(1, next_pos.column);

  // 字符串行延续跨行坐标计算
  let str_src = "\"line1\\\nline2\\\r\nline3\"\nnext";
  let mut scanner3 = make_scanner(str_src);
  assert_eq!(Token::String, scanner3.next_token());
  assert_eq!(Token::Identifier, scanner3.next_token());
  let str_next_pos = scanner3.source_position(scanner3.location().beg_pos);
  assert_eq!(4, str_next_pos.line);
  assert_eq!(1, str_next_pos.column);

  OK
}

/// 迁移自 v8/test/unittests/parser/parsing-unittest.cc: ScanHTMLEndComments
#[test]
fn scan_html_end_comments() -> aok::Void {
  let ok_tests = [
    "-->",
    "--> is eol-comment",
    "--> is eol-comment\nvar y = 37;\n",
    "\n --> is eol-comment\nvar y = 37;\n",
    "\n-->is eol-comment\nvar y = 37;\n",
    "\n-->\nvar y = 37;\n",
    "/* precomment */ --> is eol-comment\nvar y = 37;\n",
    "/* precomment */-->eol-comment\nvar y = 37;\n",
    "\n/* precomment */ --> is eol-comment\nvar y = 37;\n",
    "\n/*precomment*/-->eol-comment\nvar y = 37;\n",
    "var x = 42;\n--> is eol-comment\nvar y = 37;\n",
    "var x = 42;\n/* precomment */ --> is eol-comment\nvar y = 37;\n",
    "x/* precomment\n */ --> is eol-comment\nvar y = 37;\n",
    "var x = 42; /* precomment\n */ --> is eol-comment\nvar y = 37;\n",
    "var x = 42;/*\n*/-->is eol-comment\nvar y = 37;\n",
    "/* MLC \n */ /* SLDC */ --> is eol-comment\nvar y = 37;\n",
    "/* MLC \n */ /* SLDC1 */ /* SLDC2 */ --> is eol-comment\nvar y = 37;\n",
    "/* MLC1 \n */ /* MLC2 \n */ --> is eol-comment\nvar y = 37;\n",
    "/* SLDC */ /* MLC \n */ --> is eol-comment\nvar y = 37;\n",
    "/* MLC1 \n */ /* SLDC1 */ /* MLC2 \n */ /* SLDC2 */ --> is eol-comment\nvar y = 37;\n",
  ];

  for src in ok_tests {
    let mut scanner = make_scanner_module(src, false);
    while scanner.next_token() != Token::Eos {}
  }

  // 非换行开头的行内 "x -->": --> 会被解析为 -- (Dec) 和 > (GreaterThan)
  let fail_src = "x --> is eol-comment\nvar y = 37;\n";
  let mut scanner = make_scanner_module(fail_src, false);
  assert_eq!(Token::Identifier, scanner.next_token()); // x
  assert_eq!(Token::Dec, scanner.next_token()); // --
  assert_eq!(Token::GreaterThan, scanner.next_token()); // >

  OK
}

/// 转义关键字与上下文关键字 (原版 V8 保持 kAwait, kYield, kAsync, kGet 等原 Token, 其余转 EscapedKeyword)
#[test]
fn escaped_keywords_and_contextual_keywords() -> aok::Void {
  // 1. 上下文关键字与特定标识符 (kIdentifier..kYield) 转义后依然保持原 Token
  let contextual_cases = [
    (r#"\u0061wait"#, Token::Await),
    (r#"\u0061sync"#, Token::Async),
    (r#"\u0079ield"#, Token::Yield),
    (r#"\u0067et"#, Token::Get),
    (r#"\u0073et"#, Token::Set),
    (r#"\u006ff"#, Token::Of),
    (r#"\u0075sing"#, Token::Using),
    (r#"\u0061ccessor"#, Token::Accessor),
  ];

  for (src, expected) in contextual_cases {
    let mut scanner = make_scanner(src);
    assert_eq!(expected, scanner.next_token(), "src={}", src);
    assert_eq!(Token::Eos, scanner.next_token(), "eos src={}", src);
  }

  // 2. 严格保留字转义转 EscapedStrictReservedWord
  let strict_reserved = [
    (r#"\u006cet"#, Token::EscapedStrictReservedWord),
    (r#"\u0073tatic"#, Token::EscapedStrictReservedWord),
    (r#"\u0070ublic"#, Token::EscapedStrictReservedWord),
    (r#"\u0070rivate"#, Token::EscapedStrictReservedWord),
    (r#"\u0070ackage"#, Token::EscapedStrictReservedWord),
    (r#"\u0069mplements"#, Token::EscapedStrictReservedWord),
    (r#"\u0069nterface"#, Token::EscapedStrictReservedWord),
  ];
  for (src, expected) in strict_reserved {
    let mut scanner = make_scanner(src);
    assert_eq!(expected, scanner.next_token(), "src={}", src);
    assert_eq!(Token::Eos, scanner.next_token(), "eos src={}", src);
  }

  // 3. 普通关键字转义转 EscapedKeyword
  let escaped_keywords = [
    (r#"\u0066unction"#, Token::EscapedKeyword),
    (r#"\u0069f"#, Token::EscapedKeyword),
    (r#"\u0077hile"#, Token::EscapedKeyword),
    (r#"\u0063lass"#, Token::EscapedKeyword),
    (r#"\u0063onst"#, Token::EscapedKeyword),
    (r#"\u0072eturn"#, Token::EscapedKeyword),
  ];
  for (src, expected) in escaped_keywords {
    let mut scanner = make_scanner(src);
    assert_eq!(expected, scanner.next_token(), "src={}", src);
    assert_eq!(Token::Eos, scanner.next_token(), "eos src={}", src);
  }

  OK
}

/// Token 谓词全量与属性名/相等性判定严谨性测试
#[test]
fn token_predicates_and_property_name_rigorous() -> aok::Void {
  // 1. is_property_name: Keywords, AnyIdentifiers, EscapedKeyword 均为 true; 字面量 (String, Number, BigInt, Smi) 必为 false
  assert!(Token::Identifier.is_property_name());
  assert!(Token::If.is_property_name());
  assert!(Token::Function.is_property_name());
  assert!(Token::Return.is_property_name());
  assert!(Token::Default.is_property_name());
  assert!(Token::This.is_property_name());
  assert!(Token::EscapedKeyword.is_property_name());
  assert!(Token::Get.is_property_name());
  assert!(Token::Set.is_property_name());
  assert!(Token::Async.is_property_name());
  assert!(Token::Await.is_property_name());

  // 严正验证: String, Number, BigInt, Smi 不属于 Token::IsPropertyName
  assert!(!Token::String.is_property_name());
  assert!(!Token::Number.is_property_name());
  assert!(!Token::BigInt.is_property_name());
  assert!(!Token::Smi.is_property_name());
  assert!(!Token::Period.is_property_name());
  assert!(!Token::Illegal.is_property_name());

  // 2. is_equality_op: 仅 == (Eq) 与 === (EqStrict), 不含 != (NotEq) 与 !== (NotEqStrict)
  assert!(Token::Eq.is_equality_op());
  assert!(Token::EqStrict.is_equality_op());
  assert!(!Token::NotEq.is_equality_op());
  assert!(!Token::NotEqStrict.is_equality_op());
  assert!(!Token::LessThan.is_equality_op());
  assert!(!Token::In.is_equality_op());

  // 3. is_bit_op
  assert!(Token::BitOr.is_bit_op());
  assert!(Token::BitXor.is_bit_op());
  assert!(Token::BitAnd.is_bit_op());
  assert!(Token::Shl.is_bit_op());
  assert!(Token::Sar.is_bit_op());
  assert!(Token::Shr.is_bit_op());
  assert!(Token::BitNot.is_bit_op());
  assert!(!Token::Add.is_bit_op());
  assert!(!Token::Eq.is_bit_op());

  // 4. is_shift_op
  assert!(Token::Shl.is_shift_op());
  assert!(Token::Sar.is_shift_op());
  assert!(Token::Shr.is_shift_op());
  assert!(!Token::BitOr.is_shift_op());

  // 5. is_unary_or_count_op
  assert!(Token::Add.is_unary_or_count_op());
  assert!(Token::Sub.is_unary_or_count_op());
  assert!(Token::Not.is_unary_or_count_op());
  assert!(Token::BitNot.is_unary_or_count_op());
  assert!(Token::Delete.is_unary_or_count_op());
  assert!(Token::TypeOf.is_unary_or_count_op());
  assert!(Token::Void.is_unary_or_count_op());
  assert!(Token::Inc.is_unary_or_count_op());
  assert!(Token::Dec.is_unary_or_count_op());
  assert!(!Token::Eq.is_unary_or_count_op());

  OK
}

/// 正则表达式 Flags 精准位图与合法性验证 (对照 V8 REGEXP_FLAG_LIST)
#[test]
fn regexp_flags_exact_bits() -> aok::Void {
  // 单个标志位精准匹配: d:7, g:0, i:1, m:2, s:5, u:4, v:8, y:3
  let flag_checks = [
    ("/a/d", 1u32 << 7),
    ("/a/g", 1u32 << 0),
    ("/a/i", 1u32 << 1),
    ("/a/m", 1u32 << 2),
    ("/a/s", 1u32 << 5),
    ("/a/u", 1u32 << 4),
    ("/a/v", 1u32 << 8),
    ("/a/y", 1u32 << 3),
  ];

  for (src, expected_bit) in flag_checks {
    let mut scanner = make_scanner(src);
    assert_eq!(Token::Div, scanner.peek());
    assert!(scanner.scan_regexp_pattern());
    let flags = scanner.scan_regexp_flags();
    assert_eq!(Some(expected_bit), flags, "src={}", src);
  }

  // 复合标志
  {
    let mut scanner = make_scanner("/pattern/gimsuyvd");
    assert_eq!(Token::Div, scanner.peek());
    assert!(scanner.scan_regexp_pattern());
    let expected =
      (1 << 0) | (1 << 1) | (1 << 2) | (1 << 5) | (1 << 4) | (1 << 3) | (1 << 8) | (1 << 7);
    assert_eq!(Some(expected), scanner.scan_regexp_flags());
  }

  // 重复标志必须失败返回 None
  {
    let mut scanner = make_scanner("/pattern/gg");
    assert_eq!(Token::Div, scanner.peek());
    assert!(scanner.scan_regexp_pattern());
    assert_eq!(None, scanner.scan_regexp_flags());
  }

  // 非法字符标志必须失败返回 None
  {
    let mut scanner = make_scanner("/pattern/gx");
    assert_eq!(Token::Div, scanner.peek());
    assert!(scanner.scan_regexp_pattern());
    assert_eq!(None, scanner.scan_regexp_flags());
  }

  OK
}

/// 多层嵌套 BookmarkScope 回退一致性对抗测试
#[test]
fn nested_bookmark_scopes() -> aok::Void {
  let src = "a b c d e f g h";
  let tokens = scan_all(src);
  let mut scanner = make_scanner(src);

  // 消费 a
  assert_eq!(tokens[0], scanner.next_token());

  // 书签 1: 在 b 处
  let pos_b = scanner.peek_location().beg_pos;
  let mut bm1 = BookmarkScope::new(&scanner);
  bm1.set(pos_b);

  // 消费 b, c
  assert_eq!(tokens[1], scanner.next_token());
  assert_eq!(tokens[2], scanner.next_token());

  // 书签 2: 在 d 处
  let pos_d = scanner.peek_location().beg_pos;
  let mut bm2 = BookmarkScope::new(&scanner);
  bm2.set(pos_d);

  // 消费 d, e
  assert_eq!(tokens[3], scanner.next_token());
  assert_eq!(tokens[4], scanner.next_token());

  // 书签 3: 在 f 处
  let pos_f = scanner.peek_location().beg_pos;
  let mut bm3 = BookmarkScope::new(&scanner);
  bm3.set(pos_f);

  // 消费 f, g
  assert_eq!(tokens[5], scanner.next_token());
  assert_eq!(tokens[6], scanner.next_token());

  // 先回滚书签 3 (回退到 f)
  scanner.apply_bookmark(&mut bm3);
  assert_eq!(tokens[5], scanner.next_token()); // f
  assert_eq!(tokens[6], scanner.next_token()); // g

  // 再回滚书签 2 (回退到 d)
  scanner.apply_bookmark(&mut bm2);
  assert_eq!(tokens[3], scanner.next_token()); // d
  assert_eq!(tokens[4], scanner.next_token()); // e

  // 最后回滚书签 1 (回退到 b)
  scanner.apply_bookmark(&mut bm1);
  assert_eq!(tokens[1], scanner.next_token()); // b
  assert_eq!(tokens[2], scanner.next_token()); // c
  assert_eq!(tokens[3], scanner.next_token()); // d
  assert_eq!(tokens[4], scanner.next_token()); // e
  assert_eq!(tokens[5], scanner.next_token()); // f
  assert_eq!(tokens[6], scanner.next_token()); // g
  assert_eq!(tokens[7], scanner.next_token()); // h
  assert_eq!(Token::Eos, scanner.next_token());

  OK
}

/// 4 槽环形缓冲旋转下模板非法转义状态隔离测试 (杜绝槽位脏写与串扰)
#[test]
fn ring_buffer_template_escape_isolation() -> aok::Void {
  use v8_parsing::MessageTemplate;

  // 1. 扫描带非法转义 \8 的模板
  let src = "`bad\\8` a b c d `good`";
  let mut scanner = make_scanner(src);

  // 首个模板含有非法转义
  assert_eq!(Token::TemplateTail, scanner.next_token());
  assert!(scanner.has_invalid_template_escape());
  assert_eq!(
    MessageTemplate::Template8Or9Escape,
    scanner.invalid_template_escape_message()
  );

  // 连续推进 4 个 token: a, b, c, d (触发环形缓冲槽位完整循环复用)
  assert_eq!(Token::Identifier, scanner.next_token()); // a
  assert!(!scanner.has_invalid_template_escape());

  assert_eq!(Token::Identifier, scanner.next_token()); // b
  assert!(!scanner.has_invalid_template_escape());

  assert_eq!(Token::Identifier, scanner.next_token()); // c
  assert!(!scanner.has_invalid_template_escape());

  assert_eq!(Token::Identifier, scanner.next_token()); // d
  assert!(!scanner.has_invalid_template_escape());

  // 扫描第二个完全合法的模板: `good` (复用了首个模板所在槽位)
  assert_eq!(Token::TemplateTail, scanner.next_token());
  // 必须彻底隔离旧错误，绝对不得残留 invalid_template_escape
  assert!(
    !scanner.has_invalid_template_escape(),
    "recycled slot must not retain stale invalid_template_escape_message"
  );
  assert_eq!(
    MessageTemplate::None,
    scanner.invalid_template_escape_message()
  );

  assert_eq!(Token::Eos, scanner.next_token());
  OK
}
