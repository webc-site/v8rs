//! 原版 v8/test/unittests/torque/earley-parser-unittest.cc 1:1
//!
//! 断言规约清单 (先规约后断言, 强度守恒):
//! - earley_parser_simple_arithmetic: 左递归四则文法 (earley-parser-unittest.cc:72-83),
//!   grammar.Parse("-5 - 5 + (3 + 5) * 2") → "6";
//!   grammar.Parse("((-1 + (1) * 2 + 3 - 4 * 5 + -6 * 7))") → "-58"
//!   覆盖: 一元负号 (-5 / -6)、隐式优先级 (mul 高于 sum)、括号嵌套、
//!   左递归产生式 (sum → sum + mul)
//! - earley_parser_ambiguous_grammar_reported: 二义文法回归 (CheckAmbiguity,
//!   earley-parser.cc:100-117), 消息拼写 "grammer" 随原版

use std::result::Result;

use aok::{OK, Void};
use log::info;
use v8_torque::{
  Error, Grammar, ParseResultIterator, ParseResultValue, PatternFunction, SourceId, SourcePosition,
  default_action, yield_matched_input,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 SimpleArithmeticGrammar::MatchWhitespace (earley-parser-unittest.cc:35-39)
fn match_whitespace(input: &str, pos: &mut usize) -> bool {
  let b = input.as_bytes();
  while *pos < b.len() && b[*pos].is_ascii_whitespace() {
    *pos += 1;
  }
  true
}

/// 原版 SimpleArithmeticGrammar::MatchInteger (earley-parser-unittest.cc:41-51):
/// '-'? digit+
fn match_integer(input: &str, pos: &mut usize) -> bool {
  let b = input.as_bytes();
  let mut cur = *pos;
  if cur < b.len() && b[cur] == b'-' {
    cur += 1;
  }
  if cur < b.len() && b[cur].is_ascii_digit() {
    while cur < b.len() && b[cur].is_ascii_digit() {
      cur += 1;
    }
    *pos = cur;
    return true;
  }
  false
}

/// 原版 MakeBinop<op> (earley-parser-unittest.cc:17-26):
/// 两个 string 子结果 → 整数运算 → string
fn binop(
  it: &mut ParseResultIterator,
  op: fn(i32, i32) -> i32,
) -> Result<Option<ParseResultValue>, Error> {
  let (Some(ParseResultValue::StdString(a)), Some(ParseResultValue::StdString(b))) =
    (it.next_result(), it.next_result())
  else {
    return Ok(None);
  };
  let (Some(a), Some(b)) = (a.parse().ok(), b.parse().ok()) else {
    return Ok(None);
  };
  Ok(Some(ParseResultValue::StdString(op(a, b).to_string())))
}

fn plus(it: &mut ParseResultIterator) -> Result<Option<ParseResultValue>, Error> {
  binop(it, |a, b| a + b)
}

fn minus(it: &mut ParseResultIterator) -> Result<Option<ParseResultValue>, Error> {
  binop(it, |a, b| a - b)
}

fn mul(it: &mut ParseResultIterator) -> Result<Option<ParseResultValue>, Error> {
  binop(it, |a, b| a * b)
}

/// 原版 SimpleArithmeticGrammar (earley-parser-unittest.cc:34-70):
/// integer / atomic (integer | '(' sum ')') / mul (左递归 *) / sum (左递归 + -)
fn simple_arithmetic_grammar() -> Grammar {
  let mut g = Grammar::default();
  g.set_whitespace(match_whitespace);
  let integer = g.tables.new_symbol();
  let atomic = g.new_symbol();
  let mul_sym = g.new_symbol();
  let sum = g.new_symbol();
  let int_pat: PatternFunction = match_integer;
  let int_tok = g.pattern(int_pat);
  g.tables
    .add_rule(integer, vec![int_tok], yield_matched_input);
  let lparen = g.token("(");
  let rparen = g.token(")");
  g.tables.add_rule(atomic, vec![integer], default_action);
  g.tables
    .add_rule(atomic, vec![lparen, sum, rparen], default_action);
  let star = g.token("*");
  g.tables.add_rule(mul_sym, vec![atomic], default_action);
  g.tables.add_rule(mul_sym, vec![mul_sym, star, atomic], mul);
  let plus_t = g.token("+");
  let minus_t = g.token("-");
  g.tables.add_rule(sum, vec![mul_sym], default_action);
  g.tables.add_rule(sum, vec![sum, plus_t, mul_sym], plus);
  g.tables.add_rule(sum, vec![sum, minus_t, mul_sym], minus);
  g.start = Some(sum);
  g
}

/// dummy_filename 源位置 (原版 SourceFileMap::AddSource("dummy_filename"))
fn dummy_origin() -> SourcePosition {
  SourcePosition {
    source: SourceId(0),
    ..Default::default()
  }
}

/// parse 成功路径取 string 结果; 失败即测试失败 (禁 panic 的断言收口)
fn expect_parsed(grammar: &mut Grammar, input: &str) -> Result<String, String> {
  match grammar.parse(input, dummy_origin()) {
    Ok(Some(ParseResultValue::StdString(s))) => Ok(s),
    Ok(Some(other)) => Err(format!("Parse 结果必为 string, got {other:?}")),
    Ok(None) => Err("Parse 结果为空 (动作未产出)".to_owned()),
    Err(e) => Err(e.to_string()),
  }
}

/// 原版 TEST(EarleyParser, SimpleArithmetic) 1:1
#[test]
fn earley_parser_simple_arithmetic() -> Void {
  info!("> earley_parser_simple_arithmetic");
  let mut grammar = simple_arithmetic_grammar();
  let s1 = expect_parsed(&mut grammar, "-5 - 5 + (3 + 5) * 2");
  assert_eq!(s1, Ok("6".to_owned()));
  // 同一文法二次 Parse (top_level 包装符号复用 tables 的场景)
  let s2 = expect_parsed(&mut grammar, "((-1 + (1) * 2 + 3 - 4 * 5 + -6 * 7))");
  assert_eq!(s2, Ok("-58".to_owned()));
  OK
}

/// 二义文法回归 (round1 恢复 CheckAmbiguity 语义, 原版 earley-parser.cc:100-117):
/// s → a | b, a → x, b → x ⇒ "x" 有两棵剖析树 → "ambiguous grammer rules"
/// (原版消息拼写 "grammer" 保留); 无二义文法 (上例) 不得误报
#[test]
fn earley_parser_ambiguous_grammar_reported() -> Void {
  info!("> earley_parser_ambiguous_grammar_reported");
  let mut g = Grammar::default();
  let x = g.token("x");
  let s = g.new_symbol();
  let a = g.new_symbol();
  let b = g.new_symbol();
  g.tables.add_rule(a, vec![x], yield_matched_input);
  g.tables.add_rule(b, vec![x], yield_matched_input);
  g.tables.add_rule(s, vec![a], default_action);
  g.tables.add_rule(s, vec![b], default_action);
  g.start = Some(s);
  let msg = match g.parse("x", dummy_origin()) {
    Err(e) => e.to_string(),
    Ok(other) => {
      panic!("两条规则同跨度推导同一符号, 必判二义, got {other:?}");
    }
  };
  assert!(
    msg.contains("ambiguous grammer rules"),
    "错误消息应含二义诊断: {msg}"
  );
  OK
}
