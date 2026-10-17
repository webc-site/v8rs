use aok::Void;
use v8_regexp_syntax::{
  CharacterRange, Error, Interval, K_INFINITY, Parser, RegExpFlags, RegexpError, StackLimiter,
  StandardCharacterSet, flags_to_string, format_ast,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn check_parse_eq(input: &str, expected: &str, unicode: bool) {
  let u16_input: Vec<u16> = input.encode_utf16().collect();
  let mut flags = RegExpFlags::default();
  if unicode {
    flags |= RegExpFlags::UNICODE;
  }
  let mut parser = Parser::new(&u16_input, flags);
  let compile_data = match parser.parse() {
    Ok(data) => data,
    Err(e) => panic!("Parse failed for {:?}: {:?}", input, e),
  };
  let actual = format_ast(&compile_data.tree);
  assert_eq!(actual, expected, "Mismatch for input {:?}", input);
}

fn check_parse_eq_with_flags(input: &str, expected: &str, flags: RegExpFlags) {
  let u16_input: Vec<u16> = input.encode_utf16().collect();
  let mut parser = Parser::new(&u16_input, flags);
  let compile_data = match parser.parse() {
    Ok(data) => data,
    Err(e) => panic!("Parse failed for {:?}: {:?}", input, e),
  };
  let actual = format_ast(&compile_data.tree);
  assert_eq!(actual, expected, "Mismatch for input {:?}", input);
}

fn check_simple(input: &str) -> bool {
  let u16_input: Vec<u16> = input.encode_utf16().collect();
  let mut parser = Parser::new(&u16_input, RegExpFlags::default());
  let compile_data = parser.parse().expect("Failed to parse");
  compile_data.simple
}

fn check_min_max(input: &str, min: i32, max: i32) {
  let u16_input: Vec<u16> = input.encode_utf16().collect();
  let mut parser = Parser::new(&u16_input, RegExpFlags::default());
  let compile_data = parser.parse().expect("Failed to parse");
  assert_eq!(
    compile_data.tree.min_match(),
    min,
    "min mismatch for {:?}",
    input
  );
  assert_eq!(
    compile_data.tree.max_match(),
    max,
    "max mismatch for {:?}",
    input
  );
}

fn expect_error(input: &str, expected_kind: RegexpError, unicode: bool) {
  let u16_input: Vec<u16> = input.encode_utf16().collect();
  let mut flags = RegExpFlags::default();
  if unicode {
    flags |= RegExpFlags::UNICODE;
  }
  let mut parser = Parser::new(&u16_input, flags);
  match parser.parse() {
    Ok(_) => panic!(
      "Expected error {:?} for {:?}, but parse succeeded",
      expected_kind, input
    ),
    Err(Error::Parse { kind, .. }) => {
      assert_eq!(kind, expected_kind, "Error kind mismatch for {:?}", input);
    }
  }
}

#[test]
fn test_parser_basic() -> Void {
  check_parse_eq("abc", "'abc'", false);
  check_parse_eq("", "%", false);
  check_parse_eq("abc|def", "(| 'abc' 'def')", false);
  check_parse_eq("abc|def|ghi", "(| 'abc' 'def' 'ghi')", false);
  check_parse_eq("^xxx$", "(: @^i 'xxx' @$i)", false);
  check_parse_eq("ab\\b\\d\\bcd", "(: 'ab' @b [0-9] @b 'cd')", false);
  check_parse_eq("\\w|\\d", "(| [0-9 A-Z _ a-z] [0-9])", false);
  check_parse_eq("a*", "(# 0 - g 'a')", false);
  check_parse_eq("a*?", "(# 0 - n 'a')", false);
  check_parse_eq("abc+", "(: 'ab' (# 1 - g 'c'))", false);
  check_parse_eq("abc+?", "(: 'ab' (# 1 - n 'c'))", false);
  check_parse_eq("xyz?", "(: 'xy' (# 0 1 g 'z'))", false);
  check_parse_eq("xyz??", "(: 'xy' (# 0 1 n 'z'))", false);
  check_parse_eq("xyz{0,1}", "(: 'xy' (# 0 1 g 'z'))", false);
  check_parse_eq("xyz{0,1}?", "(: 'xy' (# 0 1 n 'z'))", false);
  check_parse_eq("xyz{93}", "(: 'xy' (# 93 93 g 'z'))", false);
  check_parse_eq("xyz{93}?", "(: 'xy' (# 93 93 n 'z'))", false);
  check_parse_eq("xyz{1,32}", "(: 'xy' (# 1 32 g 'z'))", false);
  check_parse_eq("xyz{1,32}?", "(: 'xy' (# 1 32 n 'z'))", false);
  check_parse_eq("xyz{1,}", "(: 'xy' (# 1 - g 'z'))", false);
  check_parse_eq("xyz{1,}?", "(: 'xy' (# 1 - n 'z'))", false);
  check_parse_eq(
    "a\\fb\\nc\\rd\\te\\vf",
    "'a\\x0cb\\x0ac\\x0dd\\x09e\\x0bf'",
    false,
  );
  check_parse_eq("a\\nb\\bc", "(: 'a\\x0ab' @b 'c')", false);
  check_parse_eq("(?:foo)", "(?: 'foo')", false);
  check_parse_eq("(?: foo )", "(?: ' foo ')", false);
  check_parse_eq("(foo|bar|baz)", "(^ (| 'foo' 'bar' 'baz'))", false);
  check_parse_eq(
    "foo|(bar|baz)|quux",
    "(| 'foo' (^ (| 'bar' 'baz')) 'quux')",
    false,
  );
  check_parse_eq("foo(?=bar)baz", "(: 'foo' (-> + 'bar') 'baz')", false);
  check_parse_eq("foo(?!bar)baz", "(: 'foo' (-> - 'bar') 'baz')", false);
  check_parse_eq("foo(?<=bar)baz", "(: 'foo' (<- + 'bar') 'baz')", false);
  check_parse_eq("foo(?<!bar)baz", "(: 'foo' (<- - 'bar') 'baz')", false);
  check_parse_eq("()", "(^ %)", false);
  check_parse_eq("(?=)", "(-> + %)", false);
  check_parse_eq("[]", "^[\\x00-\\u{10ffff}]", false);
  check_parse_eq("[^]", "[\\x00-\\u{10ffff}]", false);
  check_parse_eq("[x]", "[x]", false);
  check_parse_eq("[xyz]", "[x y z]", false);
  check_parse_eq("[a-zA-Z0-9]", "[a-z A-Z 0-9]", false);
  check_parse_eq("[-123]", "[- 1 2 3]", false);
  check_parse_eq("[^123]", "^[1 2 3]", false);
  check_parse_eq("]", "']'", false);
  check_parse_eq("}", "'}'", false);
  check_parse_eq("[a-b-c]", "[a-b - c]", false);
  check_parse_eq("[\\d]", "[0-9]", false);
  check_parse_eq("[x\\dz]", "[x 0-9 z]", false);
  check_parse_eq("[\\d-z]", "[0-9 - z]", false);
  check_parse_eq("[\\d-\\d]", "[0-9 0-9 -]", false);
  check_parse_eq("[z-\\d]", "[0-9 z -]", false);

  aok::OK
}

#[test]
fn test_parser_controls_and_escapes() -> Void {
  check_parse_eq(
    "\\cj\\cJ\\ci\\cI\\ck\\cK",
    "'\\x0a\\x0a\\x09\\x09\\x0b\\x0b'",
    false,
  );
  check_parse_eq("\\c!", "'\\c!'", false);
  check_parse_eq("\\c_", "'\\c_'", false);
  check_parse_eq("\\c~", "'\\c~'", false);
  check_parse_eq("\\c1", "'\\c1'", false);

  check_parse_eq("[\\c!]", "[\\ c !]", false);
  check_parse_eq("[\\c_]", "[\\x1f]", false);
  check_parse_eq("[\\c~]", "[\\ c ~]", false);
  check_parse_eq("[\\ca]", "[\\x01]", false);
  check_parse_eq("[\\cz]", "[\\x1a]", false);
  check_parse_eq("[\\cA]", "[\\x01]", false);
  check_parse_eq("[\\cZ]", "[\\x1a]", false);
  check_parse_eq("[\\c1]", "[\\x11]", false);

  check_parse_eq("[a\\]c]", "[a ] c]", false);
  check_parse_eq("\\[\\]\\{\\}\\(\\)\\%\\^\\#\\ ", "'[]{}()%^# '", false);
  check_parse_eq(
    "[\\[\\]\\{\\}\\(\\)\\%\\^\\#\\ ]",
    "[[ ] { } ( ) % ^ #  ]",
    false,
  );
  check_parse_eq("\\0", "'\\x00'", false);
  check_parse_eq("\\8", "'8'", false);
  check_parse_eq("\\9", "'9'", false);
  check_parse_eq("\\11", "'\\x09'", false);
  check_parse_eq("\\11a", "'\\x09a'", false);
  check_parse_eq("\\011", "'\\x09'", false);
  check_parse_eq("\\00011", "'\\x0011'", false);
  check_parse_eq("\\118", "'\\x098'", false);
  check_parse_eq("\\111", "'I'", false);
  check_parse_eq("\\1111", "'I1'", false);

  check_parse_eq("(x)(x)(x)\\1", "(: (^ 'x') (^ 'x') (^ 'x') (<- 1))", false);
  check_parse_eq("(x)(x)(x)\\2", "(: (^ 'x') (^ 'x') (^ 'x') (<- 2))", false);
  check_parse_eq("(x)(x)(x)\\3", "(: (^ 'x') (^ 'x') (^ 'x') (<- 3))", false);
  check_parse_eq("(x)(x)(x)\\4", "(: (^ 'x') (^ 'x') (^ 'x') '\\x04')", false);
  check_parse_eq(
    "(x)(x)(x)\\1*",
    "(: (^ 'x') (^ 'x') (^ 'x') (# 0 - g (<- 1)))",
    false,
  );
  check_parse_eq(
    "(x)(x)(x)\\2*",
    "(: (^ 'x') (^ 'x') (^ 'x') (# 0 - g (<- 2)))",
    false,
  );
  check_parse_eq(
    "(x)(x)(x)\\3*",
    "(: (^ 'x') (^ 'x') (^ 'x') (# 0 - g (<- 3)))",
    false,
  );
  check_parse_eq(
    "(x)(x)(x)\\4*",
    "(: (^ 'x') (^ 'x') (^ 'x') (# 0 - g '\\x04'))",
    false,
  );

  check_parse_eq(
    "(x)(x)(x)(x)(x)(x)(x)(x)(x)(x)\\10",
    "(: (^ 'x') (^ 'x') (^ 'x') (^ 'x') (^ 'x') (^ 'x') (^ 'x') (^ 'x') (^ 'x') (^ 'x') (<- 10))",
    false,
  );
  check_parse_eq(
    "(x)(x)(x)(x)(x)(x)(x)(x)(x)(x)\\11",
    "(: (^ 'x') (^ 'x') (^ 'x') (^ 'x') (^ 'x') (^ 'x') (^ 'x') (^ 'x') (^ 'x') (^ 'x') '\\x09')",
    false,
  );

  check_parse_eq("(a)\\1", "(: (^ 'a') (<- 1))", false);
  check_parse_eq("(a\\1)", "(^ 'a')", false);
  check_parse_eq("(\\1a)", "(^ 'a')", false);
  check_parse_eq("(\\2)(\\1)", "(: (^ (<- 2)) (^ (<- 1)))", false);
  check_parse_eq("(?=a)?a", "'a'", false);
  check_parse_eq("(?=a){0,10}a", "'a'", false);
  check_parse_eq("(?=a){1,10}a", "(: (-> + 'a') 'a')", false);
  check_parse_eq("(?=a){9,10}a", "(: (-> + 'a') 'a')", false);
  check_parse_eq("(?!a)?a", "'a'", false);
  check_parse_eq("\\1(a)", "(: (<- 1) (^ 'a'))", false);
  check_parse_eq("(?!(a))\\1", "(: (-> - (^ 'a')) (<- 1))", false);
  check_parse_eq(
    "(?!\\1(a\\1)\\1)\\1",
    "(: (-> - (: (<- 1) (^ 'a') (<- 1))) (<- 1))",
    false,
  );

  check_parse_eq("[\\0]", "[\\x00]", false);
  check_parse_eq("[\\11]", "[\\x09]", false);
  check_parse_eq("[\\11a]", "[\\x09 a]", false);
  check_parse_eq("[\\011]", "[\\x09]", false);
  check_parse_eq("[\\00011]", "[\\x00 1 1]", false);
  check_parse_eq("[\\118]", "[\\x09 8]", false);
  check_parse_eq("[\\111]", "[I]", false);
  check_parse_eq("[\\1111]", "[I 1]", false);

  check_parse_eq("\\x34", "'4'", false);
  check_parse_eq("\\x60", "'`'", false);
  check_parse_eq("\\x3z", "'x3z'", false);
  check_parse_eq("\\c", "'\\c'", false);
  check_parse_eq("\\u0034", "'4'", false);
  check_parse_eq("\\u003z", "'u003z'", false);
  check_parse_eq("foo[z]*", "(: 'foo' (# 0 - g [z]))", false);
  check_parse_eq(
    "^^^$$$\\b\\b\\b\\b",
    "(: @^i @^i @^i @$i @$i @$i @b @b @b @b)",
    false,
  );
  check_parse_eq(
    "\\b\\b\\b\\b\\B\\B\\B\\B\\b\\b\\b\\b",
    "(: @b @b @b @b @B @B @B @B @b @b @b @b)",
    false,
  );
  check_parse_eq("\\b\\B\\b", "(: @b @B @b)", false);

  // Lookarounds are not simple Assertions and must not be merged
  check_parse_eq("(?=a)(?=a)", "(: (-> + 'a') (-> + 'a'))", false);

  aok::OK
}

#[test]
fn test_parser_unicode() -> Void {
  check_parse_eq("\\u{12345}", "'\\ud808\\udf45'", true);
  check_parse_eq(
    "\\u{12345}\\u{23456}",
    "(! '\\ud808\\udf45' '\\ud84d\\udc56')",
    true,
  );
  check_parse_eq(
    "\\u{12345}|\\u{23456}",
    "(| '\\ud808\\udf45' '\\ud84d\\udc56')",
    true,
  );
  check_parse_eq("\\u{12345}{3}", "(# 3 3 g '\\ud808\\udf45')", true);
  check_parse_eq("\\u{12345}*", "(# 0 - g '\\ud808\\udf45')", true);
  check_parse_eq("\\ud808\\udf45*", "(# 0 - g '\\ud808\\udf45')", true);
  check_parse_eq(
    "[\\ud808\\udf45-\\ud809\\udccc]",
    "[\\u{012345}-\\u{0124cc}]",
    true,
  );

  aok::OK
}

#[test]
fn test_parser_simple() -> Void {
  assert!(!check_simple(""));
  assert!(check_simple("a"));
  assert!(!check_simple("a|b"));
  assert!(!check_simple("a\\n"));
  assert!(!check_simple("^a"));
  assert!(!check_simple("a$"));
  assert!(!check_simple("a\\b!"));
  assert!(!check_simple("a\\Bb"));
  assert!(!check_simple("a*"));
  assert!(!check_simple("a*?"));
  assert!(!check_simple("a?"));
  assert!(!check_simple("a??"));
  assert!(!check_simple("a{0,1}?"));
  assert!(!check_simple("a{1,1}?"));
  assert!(!check_simple("a{1,2}?"));
  assert!(!check_simple("a+?"));
  assert!(!check_simple("(a)"));
  assert!(!check_simple("(a)\\1"));
  assert!(!check_simple("(\\1a)"));
  assert!(!check_simple("\\1(a)"));
  assert!(!check_simple("a\\s"));
  assert!(!check_simple("a\\S"));
  assert!(!check_simple("a\\d"));
  assert!(!check_simple("a\\D"));
  assert!(!check_simple("a\\w"));
  assert!(!check_simple("a\\W"));
  assert!(!check_simple("a."));
  assert!(!check_simple("a\\q"));
  assert!(!check_simple("a[a]"));
  assert!(!check_simple("a[^a]"));
  assert!(!check_simple("a[a-z]"));
  assert!(!check_simple("a[\\q]"));
  assert!(!check_simple("a(?:b)"));
  assert!(!check_simple("a(?=b)"));
  assert!(!check_simple("a(?!b)"));
  assert!(!check_simple("\\x60"));
  assert!(!check_simple("\\u0060"));
  assert!(!check_simple("\\cA"));
  assert!(!check_simple("\\q"));
  assert!(!check_simple("\\1112"));
  assert!(!check_simple("\\0"));

  aok::OK
}

#[test]
fn test_parser_quantifiers_and_min_max() -> Void {
  check_parse_eq("a{}", "'a{}'", false);
  check_parse_eq("a{,}", "'a{,}'", false);
  check_parse_eq("a{", "'a{'", false);
  check_parse_eq("a{z}", "'a{z}'", false);
  check_parse_eq("a{1z}", "'a{1z}'", false);
  check_parse_eq("a{12z}", "'a{12z}'", false);
  check_parse_eq("a{12,", "'a{12,'", false);
  check_parse_eq("a{12,3b", "'a{12,3b'", false);
  check_parse_eq("{}", "'{}'", false);
  check_parse_eq("{,}", "'{,}'", false);
  check_parse_eq("{", "'{'", false);
  check_parse_eq("{z}", "'{z}'", false);
  check_parse_eq("{1z}", "'{1z}'", false);
  check_parse_eq("{12z}", "'{12z}'", false);
  check_parse_eq("{12,", "'{12,'", false);
  check_parse_eq("{12,3b", "'{12,3b'", false);

  check_min_max("a", 1, 1);
  check_min_max("abc", 3, 3);
  check_min_max("a[bc]d", 3, 3);
  check_min_max("a|bc", 1, 2);
  check_min_max("ab|c", 1, 2);
  check_min_max("a||bc", 0, 2);
  check_min_max("|", 0, 0);
  check_min_max("(?:ab)", 2, 2);
  check_min_max("(?:ab|cde)", 2, 3);
  check_min_max("(?:ab)|cde", 2, 3);
  check_min_max("(ab)", 2, 2);
  check_min_max("(ab|cde)", 2, 3);
  check_min_max("(ab)\\1", 2, K_INFINITY);
  check_min_max("(ab|cde)\\1", 2, K_INFINITY);
  check_min_max("(?:ab)?", 0, 2);
  check_min_max("(?:ab)*", 0, K_INFINITY);
  check_min_max("(?:ab)+", 2, K_INFINITY);
  check_min_max("a?", 0, 1);
  check_min_max("a*", 0, K_INFINITY);
  check_min_max("a+", 1, K_INFINITY);
  check_min_max("a??", 0, 1);
  check_min_max("a*?", 0, K_INFINITY);
  check_min_max("a+?", 1, K_INFINITY);
  check_min_max("(?:a?)?", 0, 1);
  check_min_max("(?:a*)?", 0, K_INFINITY);
  check_min_max("(?:a+)?", 0, K_INFINITY);
  check_min_max("(?:a?)+", 0, K_INFINITY);
  check_min_max("(?:a*)+", 0, K_INFINITY);
  check_min_max("(?:a+)+", 1, K_INFINITY);
  check_min_max("(?:a?)*", 0, K_INFINITY);
  check_min_max("(?:a*)*", 0, K_INFINITY);
  check_min_max("(?:a+)*", 0, K_INFINITY);
  check_min_max("a{0}", 0, 0);
  check_min_max("(?:a+){0}", 0, 0);
  check_min_max("(?:a+){0,0}", 0, 0);
  check_min_max("a*b", 1, K_INFINITY);
  check_min_max("a+b", 2, K_INFINITY);
  check_min_max("a*b|c", 1, K_INFINITY);
  check_min_max("a+b|c", 1, K_INFINITY);
  check_min_max("(?:a{5,1000000}){3,1000000}", 15, K_INFINITY);
  check_min_max("(?:ab){4,7}", 8, 14);
  check_min_max("a\\bc", 2, 2);
  check_min_max("a\\Bc", 2, 2);
  check_min_max("a\\sc", 3, 3);
  check_min_max("a\\Sc", 3, 3);
  check_min_max("a(?=b)c", 2, 2);
  check_min_max("a(?=bbb|bb)c", 2, 2);
  check_min_max("a(?!bbb|bb)c", 2, 2);

  aok::OK
}

#[test]
fn test_parser_named_captures() -> Void {
  check_parse_eq(
    "(?<a>x)(?<b>x)(?<c>x)\\k<a>",
    "(: (^ 'x') (^ 'x') (^ 'x') (<- 1))",
    true,
  );
  check_parse_eq(
    "(?<a>x)(?<b>x)(?<c>x)\\k<b>",
    "(: (^ 'x') (^ 'x') (^ 'x') (<- 2))",
    true,
  );
  check_parse_eq(
    "(?<a>x)(?<b>x)(?<c>x)\\k<c>",
    "(: (^ 'x') (^ 'x') (^ 'x') (<- 3))",
    true,
  );
  check_parse_eq("(?<a>a)\\k<a>", "(: (^ 'a') (<- 1))", true);
  check_parse_eq("(?<a>a\\k<a>)", "(^ 'a')", true);
  check_parse_eq("(?<a>\\k<a>a)", "(^ 'a')", true);
  check_parse_eq(
    "(?<a>\\k<b>)(?<b>\\k<a>)",
    "(: (^ (<- 2)) (^ (<- 1)))",
    true,
  );
  check_parse_eq("\\k<a>(?<a>a)", "(: (<- 1) (^ 'a'))", true);

  check_parse_eq("(?<\\u{03C0}>a)", "(^ 'a')", true);
  check_parse_eq("(?<\\u03C0>a)", "(^ 'a')", true);
  check_parse_eq("(?<a>x)\\1", "(: (^ 'x') (<- 1))", true);

  aok::OK
}

#[test]
fn test_parser_regression() -> Void {
  check_parse_eq("[A-Z$-][x]", "(! [A-Z $ -] [x])", false);
  check_parse_eq("a{3,4*}", "(: 'a{3,' (# 0 - g '4') '}')", false);
  check_parse_eq("{", "'{'", false);
  check_parse_eq("a|", "(| 'a' %)", false);

  aok::OK
}

#[test]
fn test_parser_errors() -> Void {
  expect_error("\\", RegexpError::EscapeAtEndOfPattern, false);
  expect_error("(foo", RegexpError::UnterminatedGroup, false);
  expect_error("(?", RegexpError::InvalidGroup, false);
  expect_error("[", RegexpError::UnterminatedCharacterClass, false);
  expect_error("[a-", RegexpError::UnterminatedCharacterClass, false);
  expect_error("*", RegexpError::NothingToRepeat, false);
  expect_error("?", RegexpError::NothingToRepeat, false);
  expect_error("+", RegexpError::NothingToRepeat, false);
  expect_error("{1}", RegexpError::NothingToRepeat, false);
  expect_error("{1,2}", RegexpError::NothingToRepeat, false);
  expect_error("{1,}", RegexpError::NothingToRepeat, false);

  expect_error("(?<>.)", RegexpError::InvalidCaptureGroupName, true);
  expect_error("(?<1>.)", RegexpError::InvalidCaptureGroupName, true);
  expect_error("(?<_%>.)", RegexpError::InvalidCaptureGroupName, true);
  expect_error("\\k<a", RegexpError::InvalidCaptureGroupName, true);
  expect_error(
    "(?<a>.)(?<a>.)",
    RegexpError::DuplicateCaptureGroupName,
    true,
  );
  expect_error("(?<\\u{FISK}", RegexpError::InvalidUnicodeEscape, true);
  expect_error("\\k<a>", RegexpError::InvalidNamedCaptureReference, true);
  expect_error(
    "(?<b>)\\k<a>",
    RegexpError::InvalidNamedCaptureReference,
    true,
  );
  expect_error("\\ka", RegexpError::InvalidNamedReference, true);
  expect_error("(?<=a)+", RegexpError::InvalidQuantifier, false);
  expect_error("(?=a)+", RegexpError::InvalidQuantifier, true);
  expect_error(
    "(x)\\k<foo>",
    RegexpError::InvalidNamedCaptureReference,
    true,
  );

  aok::OK
}

#[test]
fn test_flags_to_string() -> Void {
  let flags = RegExpFlags::IGNORE_CASE | RegExpFlags::GLOBAL;
  assert_eq!(flags_to_string(flags), "gi");

  let flags_empty = RegExpFlags::default();
  assert_eq!(flags_to_string(flags_empty), "");

  let all_flags = RegExpFlags::HAS_INDICES
    | RegExpFlags::GLOBAL
    | RegExpFlags::IGNORE_CASE
    | RegExpFlags::MULTILINE
    | RegExpFlags::DOT_ALL
    | RegExpFlags::UNICODE
    | RegExpFlags::STICKY;
  assert_eq!(flags_to_string(all_flags), "dgimsuy");

  aok::OK
}

#[test]
fn test_character_class_escapes() -> Void {
  let mut digit_ranges = Vec::new();
  CharacterRange::add_class_escape(StandardCharacterSet::Digit, &mut digit_ranges, false);
  assert_eq!(digit_ranges.len(), 1);
  assert_eq!(digit_ranges[0].from, b'0' as u32);
  assert_eq!(digit_ranges[0].to, b'9' as u32);

  let mut word_ranges = Vec::new();
  CharacterRange::add_class_escape(StandardCharacterSet::Word, &mut word_ranges, false);
  assert_eq!(word_ranges.len(), 4);
  assert_eq!(
    word_ranges[0],
    CharacterRange::range(b'0' as u32, b'9' as u32)
  );
  assert_eq!(
    word_ranges[1],
    CharacterRange::range(b'A' as u32, b'Z' as u32)
  );
  assert_eq!(word_ranges[2], CharacterRange::singleton(b'_' as u32));
  assert_eq!(
    word_ranges[3],
    CharacterRange::range(b'a' as u32, b'z' as u32)
  );

  let mut ws_ranges = Vec::new();
  CharacterRange::add_class_escape(StandardCharacterSet::Whitespace, &mut ws_ranges, false);
  assert!(!ws_ranges.is_empty());

  aok::OK
}

#[test]
fn test_character_set_canonicalize() -> Void {
  let mut list = vec![
    CharacterRange::range(10, 20),
    CharacterRange::range(30, 40),
    CharacterRange::range(50, 60),
  ];
  CharacterRange::canonicalize(&mut list);
  assert_eq!(list.len(), 3);
  assert_eq!(list[0], CharacterRange::range(10, 20));
  assert_eq!(list[1], CharacterRange::range(30, 40));
  assert_eq!(list[2], CharacterRange::range(50, 60));

  let mut list2 = vec![
    CharacterRange::range(10, 20),
    CharacterRange::range(50, 60),
    CharacterRange::range(30, 40),
  ];
  CharacterRange::canonicalize(&mut list2);
  assert_eq!(list2.len(), 3);
  assert_eq!(list2[0], CharacterRange::range(10, 20));
  assert_eq!(list2[1], CharacterRange::range(30, 40));
  assert_eq!(list2[2], CharacterRange::range(50, 60));

  let mut list3 = vec![
    CharacterRange::range(10, 19),
    CharacterRange::range(21, 30),
    CharacterRange::range(20, 20),
  ];
  CharacterRange::canonicalize(&mut list3);
  assert_eq!(list3.len(), 1);
  assert_eq!(list3[0], CharacterRange::range(10, 30));

  aok::OK
}

#[test]
fn test_unicode_sets_v_mode() -> Void {
  // Test /v flag intersection and subtraction
  let flags_v = RegExpFlags::UNICODE_SETS;

  // Union of ranges: [[a-c][e-g]]
  check_parse_eq_with_flags("[[a-c][e-g]]", "++[++[![a-c]] ++[![e-g]]]", flags_v);

  // Intersection of ranges: [[a-z]&&[d-g]]
  check_parse_eq_with_flags("[[a-z]&&[d-g]]", "&&[++[![a-z]] ++[![d-g]]]", flags_v);

  // Subtraction of ranges: [[a-z]--[d-g]]
  check_parse_eq_with_flags("[[a-z]--[d-g]]", "--[++[![a-z]] ++[![d-g]]]", flags_v);

  // Class string disjunction: [\q{abc|def}]
  check_parse_eq_with_flags("[\\q{abc|def}]", "++[![ 'abc' 'def']]", flags_v);

  aok::OK
}

#[test]
fn test_non_ascii_escapes() -> Void {
  // Non-ASCII code point whose low byte is '1' (0x0131: Turkish dotless i)
  // Must not be confused with backreference \1 in non-unicode mode
  check_parse_eq("\\\u{0131}", "'\\u0131'", false);
  // Non-ASCII code point whose low byte is '^' (0x015E)
  check_parse_eq("\\\u{015E}", "'\\u015e'", false);
  aok::OK
}

#[test]
fn test_char_range_algebra() -> Void {
  // try_range 边界
  assert!(CharacterRange::try_range(0, 0x10FFFF).is_some());
  assert!(CharacterRange::try_range(100, 100).is_some());
  assert!(CharacterRange::try_range(20, 10).is_none());
  assert!(CharacterRange::try_range(0, 0x110000).is_none());

  // BMP 取补
  let a_range = vec![CharacterRange::range(b'a' as u32, b'a' as u32)];
  let mut negated_bmp = Vec::new();
  CharacterRange::negate_bmp(&a_range, &mut negated_bmp);
  assert_eq!(negated_bmp.len(), 2);
  assert_eq!(negated_bmp[0], CharacterRange::range(0, b'a' as u32 - 1));
  assert_eq!(
    negated_bmp[1],
    CharacterRange::range(b'a' as u32 + 1, 0xFFFF)
  );

  // Unicode 全集取补
  let mut negated_unicode = Vec::new();
  CharacterRange::negate(&a_range, &mut negated_unicode);
  assert_eq!(negated_unicode.len(), 2);
  assert_eq!(
    negated_unicode[0],
    CharacterRange::range(0, b'a' as u32 - 1)
  );
  assert_eq!(
    negated_unicode[1],
    CharacterRange::range(b'a' as u32 + 1, 0x10FFFF)
  );

  // 全集取补为空集
  let full_unicode = vec![CharacterRange::everything()];
  let mut empty_neg = Vec::new();
  CharacterRange::negate(&full_unicode, &mut empty_neg);
  assert!(empty_neg.is_empty());

  // Intersect 与 Subtract
  let lhs = vec![CharacterRange::range(10, 30), CharacterRange::range(50, 70)];
  let rhs = vec![CharacterRange::range(20, 60)];
  let mut intersection = Vec::new();
  CharacterRange::intersect(&lhs, &rhs, &mut intersection);
  assert_eq!(intersection.len(), 2);
  assert_eq!(intersection[0], CharacterRange::range(20, 30));
  assert_eq!(intersection[1], CharacterRange::range(50, 60));

  let mut subtraction = Vec::new();
  CharacterRange::subtract(&lhs, &rhs, &mut subtraction);
  assert_eq!(subtraction.len(), 2);
  assert_eq!(subtraction[0], CharacterRange::range(10, 19));
  assert_eq!(subtraction[1], CharacterRange::range(61, 70));

  aok::OK
}

#[test]
fn test_parser_extreme_captures() -> Void {
  // 达到或超过 65536 个捕获组触发 TooManyCaptures
  let mut excessive = String::with_capacity(65537 * 2);
  for _ in 0..=65536 {
    excessive.push_str("()");
  }
  expect_error(&excessive, RegexpError::TooManyCaptures, false);

  // 多分支复用同名捕获组允许
  check_parse_eq("(?<name>a)|(?<name>b)", "(| (^ 'a') (^ 'b'))", true);

  // 同分支重复捕获组名报错
  expect_error(
    "(?<name>a)(?<name>b)",
    RegexpError::DuplicateCaptureGroupName,
    true,
  );

  // 嵌套重复捕获组名报错
  expect_error(
    "(?<name>.(?<name>.))",
    RegexpError::DuplicateCaptureGroupName,
    true,
  );

  // 多分支同名捕获组的统一回填
  check_parse_eq(
    "(?:(?<foo>a)|(?<foo>b))\\k<foo>",
    "(: (?u: (| (^ 'a') (^ 'b'))) (<- 1,2))",
    true,
  );

  aok::OK
}

#[test]
fn test_parser_quantifier_overflow() -> Void {
  // 量词数字巨大溢出到 K_INFINITY (max 为 K_INFINITY 时 AstNodePrinter 输出 '-')
  check_parse_eq("a{99999999999999999999}", "(# 2147483647 - g 'a')", false);
  check_parse_eq("a{1,99999999999999999999}", "(# 1 - g 'a')", false);

  // 逆序范围报错
  expect_error("a{5,3}", RegexpError::RangeOutOfOrder, false);

  aok::OK
}

#[test]
fn test_unicode_sets_edge_cases() -> Void {
  let flags_v = RegExpFlags::UNICODE_SETS;

  // 差集：从 a-z 中排除元音
  check_parse_eq_with_flags(
    "[[a-z]--[aeiou]]",
    "--[++[![a-z]] ++[![a e i o u]]]",
    flags_v,
  );

  // 字符串类取反报错
  let u16_input: Vec<u16> = "[^\\q{abc}]".encode_utf16().collect();
  let mut parser = Parser::new(&u16_input, flags_v);
  match parser.parse() {
    Err(Error::Parse { kind, .. }) => {
      assert_eq!(kind, RegexpError::NegatedCharacterClassWithStrings);
    }
    Ok(_) => panic!("Expected NegatedCharacterClassWithStrings error"),
  }

  // 连续差集: [[a-z]--[d-z]--[a-b]]
  check_parse_eq_with_flags(
    "[[a-z]--[d-z]--[a-b]]",
    "--[++[![a-z]] ++[![d-z]] ++[![a-b]]]",
    flags_v,
  );

  // 连续交集: [[a-z]&&[d-w]&&[f-m]]
  check_parse_eq_with_flags(
    "[[a-z]&&[d-w]&&[f-m]]",
    "&&[++[![a-z]] ++[![d-w]] ++[![f-m]]]",
    flags_v,
  );

  // 嵌套类包含字符串类并取反拦截
  let u16_neg_str: Vec<u16> = "[^[\\q{abc}]--a]".encode_utf16().collect();
  let mut parser_neg_str = Parser::new(&u16_neg_str, flags_v);
  match parser_neg_str.parse() {
    Err(Error::Parse { kind, .. }) => {
      assert_eq!(kind, RegexpError::NegatedCharacterClassWithStrings);
    }
    Ok(_) => panic!("Expected NegatedCharacterClassWithStrings error"),
  }

  // 非法混合集合运算拦截
  let u16_mix1: Vec<u16> = "[a&&b--c]".encode_utf16().collect();
  assert!(Parser::new(&u16_mix1, flags_v).parse().is_err());
  let u16_mix2: Vec<u16> = "[a--b&&c]".encode_utf16().collect();
  assert!(Parser::new(&u16_mix2, flags_v).parse().is_err());

  aok::OK
}

#[test]
fn test_interval_and_capture_registers() -> Void {
  // Interval::empty & valid
  let empty = Interval::empty();
  assert!(empty.is_empty());
  assert!(empty.is_valid());

  let iv1 = Interval::new(2, 5);
  let iv2 = Interval::new(4, 9);
  assert_eq!(iv1.union(iv2), Interval::new(2, 9));
  assert_eq!(empty.union(iv1), iv1);
  assert_eq!(iv1.union(empty), iv1);

  // Interval::invalid 传染性 (双向 union 必须保持 invalid, 决不可被吞或污染边界)
  let invalid = Interval::invalid();
  assert!(!invalid.is_valid());
  assert_eq!(invalid.union(iv1), invalid);
  assert_eq!(iv1.union(invalid), invalid);
  assert_eq!(invalid.union(empty), invalid);
  assert_eq!(empty.union(invalid), invalid);

  // capture_registers 正常递归与栈预算限制
  let u16_input: Vec<u16> = "((a)|(b(?<c>c)))".encode_utf16().collect();
  let mut parser = Parser::new(&u16_input, RegExpFlags::UNICODE);
  let data = parser.parse().expect("Failed to parse");

  // 正常预算: 覆盖捕获寄存器 2..=7 (组 1: 2..3, 组 2: 4..5, 组 3: 6..7, 组 4: 8..9)
  let reg = data.tree.capture_registers(StackLimiter::new(100));
  assert!(reg.is_valid());
  assert_eq!(reg.from(), 2);
  assert_eq!(reg.to(), 9);

  // 预算为 0 或负数: 模拟递归溢出, 返回 invalid
  let overflow = data.tree.capture_registers(StackLimiter::new(0));
  assert!(!overflow.is_valid());

  aok::OK
}

#[test]
fn test_certainly_anchored() -> Void {
  const BUDGET: i32 = 100;

  let parse_pattern = |s: &str, flags: RegExpFlags| {
    let u16_input: Vec<u16> = s.encode_utf16().collect();
    let mut parser = Parser::new(&u16_input, flags);
    parser.parse().expect("Failed to parse pattern")
  };

  // 1. ^abc: 肯定锚定于首, 否定锚定于尾
  let d1 = parse_pattern("^abc", RegExpFlags::default());
  assert!(d1.tree.is_certainly_anchored_at_start(BUDGET));
  assert!(!d1.tree.is_certainly_anchored_at_end(BUDGET));

  // 2. abc$: 否定首, 肯定尾
  let d2 = parse_pattern("abc$", RegExpFlags::default());
  assert!(!d2.tree.is_certainly_anchored_at_start(BUDGET));
  assert!(d2.tree.is_certainly_anchored_at_end(BUDGET));

  // 3. ^abc$: 双向肯定
  let d3 = parse_pattern("^abc$", RegExpFlags::default());
  assert!(d3.tree.is_certainly_anchored_at_start(BUDGET));
  assert!(d3.tree.is_certainly_anchored_at_end(BUDGET));

  // 4. 析取分支: 全部肯定才为肯定
  let d4 = parse_pattern("(^a|^b)", RegExpFlags::default());
  assert!(d4.tree.is_certainly_anchored_at_start(BUDGET));

  let d5 = parse_pattern("(^a|b)", RegExpFlags::default());
  assert!(!d5.tree.is_certainly_anchored_at_start(BUDGET));

  let d6 = parse_pattern("(a$|b$)", RegExpFlags::default());
  assert!(d6.tree.is_certainly_anchored_at_end(BUDGET));

  let d7 = parse_pattern("(a$|b)", RegExpFlags::default());
  assert!(!d7.tree.is_certainly_anchored_at_end(BUDGET));

  // 5. 环视与分组
  let d8 = parse_pattern("(?=^a)b", RegExpFlags::default());
  assert!(d8.tree.is_certainly_anchored_at_start(BUDGET));

  let d9 = parse_pattern("(?<=^a)b", RegExpFlags::default());
  assert!(!d9.tree.is_certainly_anchored_at_start(BUDGET));

  let d10 = parse_pattern("(?:^a)", RegExpFlags::default());
  assert!(d10.tree.is_certainly_anchored_at_start(BUDGET));

  let d11 = parse_pattern("(?<name>^a)", RegExpFlags::UNICODE);
  assert!(d11.tree.is_certainly_anchored_at_start(BUDGET));

  // 6. 前序有消费字符: 不算锚定于首 (Alternative 中 max_match > 0 阻断后续 ^)
  let d12 = parse_pattern("a^", RegExpFlags::default());
  assert!(!d12.tree.is_certainly_anchored_at_start(BUDGET));

  // 7. 递归预算耗尽 (budget <= 0) 返回 false
  assert!(!d1.tree.is_certainly_anchored_at_start(0));
  assert!(!d1.tree.is_certainly_anchored_at_start(-1));

  aok::OK
}

#[test]
fn test_ast_impl_equivalence() -> Void {
  use v8_regexp_syntax::{
    Alternative, AssertionType, Atom, Capture, ClassRanges, Disjunction, Group, Lookaround,
    LookaroundType, Quantifier, QuantifierType, Tree,
    ast_generated::{IS_CERTAINLY_TWO_CODE_POINTS, NEGATED, NO_CASE_FOLDING_NEEDED},
    ast_impl::{
      XMACRO_TREE_KINDS, anchored_alternative, anchored_assertion, anchored_capture,
      anchored_disjunction, anchored_group, anchored_lookaround_at_start,
      capture_registers_alternative, capture_registers_capture, capture_registers_disjunction,
      capture_registers_group, capture_registers_lookaround, capture_registers_quantifier,
      ctor_alternative, ctor_class_ranges, ctor_class_set_expression, ctor_class_set_operand,
      ctor_disjunction, increase_by, rules,
    },
  };

  // 1. XMACRO 种类与规则锚
  assert_eq!(XMACRO_TREE_KINDS.len(), 14);
  const _: () = assert!(rules::K_INFINITY_CAP);
  assert_eq!(rules::K_BMP_THRESHOLD, 0x10000);
  assert_eq!(rules::K_CLASS_SET_EMPTY_MIN_MAX, (0, 0));
  assert_eq!(rules::K_CLASS_SET_NONEMPTY_MIN_MAX, (1, 2));
  assert_eq!(rules::K_NEGATED_MAX_MATCH, 2);

  // 2. increase_by 溢出钳制
  assert_eq!(increase_by(10, 20), 30);
  assert_eq!(increase_by(K_INFINITY - 5, 10), K_INFINITY);
  assert_eq!(increase_by(K_INFINITY, 1), K_INFINITY);

  // 3. ctor_disjunction vs Disjunction::new
  let alt_nodes1 = vec![
    Tree::Atom(Box::new(Atom::new(vec![b'a' as u16]))),
    Tree::Atom(Box::new(Atom::new(vec![b'b' as u16, b'c' as u16]))),
  ];
  let (min1, max1) = ctor_disjunction(&alt_nodes1);
  let disj1 = Disjunction::new(alt_nodes1);
  assert_eq!(disj1.min_match, min1);
  assert_eq!(disj1.max_match, max1);

  // 4. ctor_alternative vs Alternative::new
  let alt_nodes2 = vec![
    Tree::Atom(Box::new(Atom::new(vec![b'a' as u16]))),
    Tree::Atom(Box::new(Atom::new(vec![b'b' as u16, b'c' as u16]))),
  ];
  let (min2, max2) = ctor_alternative(&alt_nodes2);
  let alt2 = Alternative::new(alt_nodes2);
  assert_eq!(alt2.min_match, min2);
  assert_eq!(alt2.max_match, max2);

  // 5. ctor_class_ranges vs ClassRanges::new
  let (r_empty, f_empty) = ctor_class_ranges(vec![], 0);
  let cr_empty = ClassRanges::new(vec![], 0);
  assert_eq!(cr_empty.set.ranges, r_empty);
  assert_eq!(cr_empty.class_ranges_flags, f_empty);
  assert_eq!(r_empty, vec![CharacterRange::everything()]);
  assert_eq!(f_empty, NEGATED);

  let non_bmp = vec![CharacterRange::range(0x10000, 0x10FFFF)];
  let (r_non_bmp, f_non_bmp) = ctor_class_ranges(non_bmp.clone(), NO_CASE_FOLDING_NEEDED);
  let cr_non_bmp = ClassRanges::new(non_bmp, NO_CASE_FOLDING_NEEDED);
  assert_eq!(cr_non_bmp.set.ranges, r_non_bmp);
  assert_eq!(cr_non_bmp.class_ranges_flags, f_non_bmp);
  assert!(ClassRanges::has_flag(
    f_non_bmp,
    IS_CERTAINLY_TWO_CODE_POINTS
  ));

  // 6. ctor_class_set_operand & ctor_class_set_expression
  let (cso_min_empty, cso_max_empty) = ctor_class_set_operand(&[], None);
  assert_eq!((cso_min_empty, cso_max_empty), (0, 0));
  let (cso_min, cso_max) =
    ctor_class_set_operand(&[CharacterRange::range(b'a' as u32, b'z' as u32)], None);
  assert_eq!((cso_min, cso_max), (1, 2));

  assert_eq!(ctor_class_set_expression(true, &[]), 2);
  let op_trees = vec![Tree::Atom(Box::new(Atom::new(vec![
    b'a' as u16,
    b'b' as u16,
  ])))];
  assert_eq!(ctor_class_set_expression(false, &op_trees), 2);

  // 7. capture_registers standalone vs Tree
  let limiter = StackLimiter::new(100);
  let make_body = || {
    Tree::Capture(Box::new(Capture::new(
      1,
      None,
      Tree::Atom(Box::new(Atom::new(vec![b'x' as u16]))),
    )))
  };

  let body_tree = make_body();
  assert_eq!(
    capture_registers_group(&body_tree, limiter),
    Tree::Group(Box::new(Group::new(make_body(), RegExpFlags::default())))
      .capture_registers(limiter)
  );

  assert_eq!(
    capture_registers_capture(1, &body_tree, limiter),
    body_tree.capture_registers(limiter)
  );

  assert_eq!(
    capture_registers_lookaround(&body_tree, limiter),
    Tree::Lookaround(Box::new(Lookaround::new(
      make_body(),
      true,
      1,
      1,
      LookaroundType::Lookahead,
      0,
    )))
    .capture_registers(limiter)
  );

  assert_eq!(
    capture_registers_quantifier(&body_tree, limiter),
    Tree::Quantifier(Box::new(Quantifier::new(
      0,
      1,
      QuantifierType::Greedy,
      0,
      make_body(),
    )))
    .capture_registers(limiter)
  );

  let alts = vec![make_body(), make_body()];
  assert_eq!(
    capture_registers_disjunction(&alts, limiter),
    Tree::Disjunction(Box::new(Disjunction::new(vec![make_body(), make_body()])))
      .capture_registers(limiter)
  );

  assert_eq!(
    capture_registers_alternative(&alts, limiter),
    Tree::Alternative(Box::new(Alternative::new(vec![make_body(), make_body()])))
      .capture_registers(limiter)
  );

  // 8. anchored_* standalone
  assert!(anchored_assertion(AssertionType::StartOfInput, true));
  assert!(!anchored_assertion(AssertionType::StartOfInput, false));
  assert!(anchored_assertion(AssertionType::EndOfInput, false));
  assert!(!anchored_assertion(AssertionType::EndOfInput, true));

  let make_start = || {
    Tree::Assertion(Box::new(v8_regexp_syntax::Assertion {
      assertion_type: AssertionType::StartOfInput,
    }))
  };
  let make_end = || {
    Tree::Assertion(Box::new(v8_regexp_syntax::Assertion {
      assertion_type: AssertionType::EndOfInput,
    }))
  };

  let start_anchor = make_start();
  let end_anchor = make_end();

  assert!(anchored_alternative(&[make_start()], true, 10));
  assert!(!anchored_alternative(&[make_start()], false, 10));
  assert!(anchored_alternative(&[make_end()], false, 10));

  assert!(anchored_disjunction(&[make_start()], true, 10));
  assert!(!anchored_disjunction(
    &[make_start(), Tree::Empty(Box::new(v8_regexp_syntax::Empty))],
    true,
    10
  ));

  assert!(anchored_lookaround_at_start(
    true,
    LookaroundType::Lookahead,
    &start_anchor,
    10
  ));
  assert!(!anchored_lookaround_at_start(
    false,
    LookaroundType::Lookahead,
    &start_anchor,
    10
  ));
  assert!(!anchored_lookaround_at_start(
    true,
    LookaroundType::Lookbehind,
    &start_anchor,
    10
  ));

  assert!(anchored_capture(&start_anchor, true, 10));
  assert!(!anchored_capture(&start_anchor, false, 10));
  assert!(anchored_capture(&end_anchor, false, 10));

  assert!(anchored_group(&start_anchor, true, 10));
  assert!(!anchored_group(&start_anchor, false, 10));
  assert!(anchored_group(&end_anchor, false, 10));

  aok::OK
}
