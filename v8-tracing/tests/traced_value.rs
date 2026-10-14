//! `v8/test/unittests/tracing/traced-value-unittest.cc` 1:1 规约迁移 (7 用例)。
//! 测试名保留原 gtest 名 (TEST_F(TracedValueTest, X) → traced_value_test_x)。

use aok::{OK, Void};
use v8_tracing::TracedValue;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// FlatDictionary (traced-value-unittest.cc:13-25)。
#[test]
fn traced_value_test_flat_dictionary() -> Void {
  let mut value = TracedValue::create();
  value.set_integer("int", 2014);
  value.set_double("double", 0.0);
  value.set_boolean("bool", true);
  value.set_string("string", "string");
  let mut json = String::from("PREFIX");
  value.append_as_trace_format(&mut json);
  assert_eq!(
    json,
    "PREFIX{\"int\":2014,\"double\":0,\"bool\":true,\"string\":\"string\"}"
  );
  OK
}

/// NoDotPathExpansion (traced-value-unittest.cc:27-39)。
#[test]
fn traced_value_test_no_dot_path_expansion() -> Void {
  let mut value = TracedValue::create();
  value.set_integer("in.t", 2014);
  value.set_double("doub.le", -20.25);
  value.set_boolean("bo.ol", true);
  value.set_string("str.ing", "str.ing");
  let mut json = String::new();
  value.append_as_trace_format(&mut json);
  assert_eq!(
    json,
    "{\"in.t\":2014,\"doub.le\":-20.25,\"bo.ol\":true,\"str.ing\":\"str.ing\"}"
  );
  OK
}

/// Hierarchy (traced-value-unittest.cc:41-79)。
#[test]
fn traced_value_test_hierarchy() -> Void {
  let mut value = TracedValue::create();
  value.set_integer("i0", 2014);
  value.begin_dictionary("dict1");
  value.set_integer("i1", 2014);
  value.begin_dictionary("dict2");
  value.set_boolean("b2", false);
  value.end_dictionary();
  value.set_string("s1", "foo");
  value.end_dictionary();
  value.set_double("d0", 0.0);
  value.set_double("d1", 10.5);
  value.set_boolean("b0", true);
  value.begin_array("a1");
  value.append_integer(1);
  value.append_boolean(true);
  value.begin_nested_dictionary();
  value.set_integer("i2", 3);
  value.end_dictionary();
  value.end_array();
  value.set_string("s0", "foo");

  value.begin_array("arr1");
  value.begin_nested_dictionary();
  value.end_dictionary();
  value.begin_nested_array();
  value.end_array();
  value.begin_nested_dictionary();
  value.end_dictionary();
  value.end_array();

  let mut json = String::new();
  value.append_as_trace_format(&mut json);
  assert_eq!(
    json,
    "{\"i0\":2014,\"dict1\":{\"i1\":2014,\"dict2\":{\"b2\":false},\"s1\":\"foo\"},\"d0\":0,\"d1\":10.5,\"b0\":true,\"a1\":[1,true,{\"i2\":3}],\"s0\":\"foo\",\"arr1\":[{},[],{}]}"
  );
  OK
}

/// Nesting (traced-value-unittest.cc:81-92)。
#[test]
fn traced_value_test_nesting() -> Void {
  let mut value = TracedValue::create();
  let mut v0 = TracedValue::create();
  let mut v2 = TracedValue::create();
  v0.set_string("s1", "Hello World!");
  v2.set_value("v0", &v0);
  value.set_value("v2", &v2);

  let mut json = String::new();
  value.append_as_trace_format(&mut json);
  assert_eq!(json, "{\"v2\":{\"v0\":{\"s1\":\"Hello World!\"}}}");
  OK
}

/// LongStrings (traced-value-unittest.cc:94-119)。
#[test]
fn traced_value_test_long_strings() -> Void {
  let long_string = "supercalifragilisticexpialidocious";
  let long_string2 = "0123456789012345678901234567890123456789";
  // C++ char[4096] 全填后末位 NUL → C 串 4095 字符
  let long_string3: String = (0..4095)
    .map(|i| char::from(b'a' + (i % 26) as u8))
    .collect();

  let mut value = TracedValue::create();
  value.set_string("a", "short");
  value.set_string("b", long_string);
  value.begin_array("c");
  value.append_string(long_string2);
  value.append_string("");
  value.begin_nested_dictionary();
  value.set_string("a", &long_string3);
  value.end_dictionary();
  value.end_array();

  let mut json = String::new();
  value.append_as_trace_format(&mut json);
  assert_eq!(
    json,
    format!(
      "{{\"a\":\"short\",\"b\":\"{long_string}\",\"c\":[\"{long_string2}\",\"\",{{\"a\":\"{long_string3}\"}}]}}"
    )
  );
  OK
}

/// Escaping (traced-value-unittest.cc:121-142)。
/// 期望串为 C++ 原文 R"(...)" 的 JSON 文本 (raw 字面量, 逐字符照抄)。
#[test]
fn traced_value_test_escaping() -> Void {
  // C++ 原文 "\\\\\"'\\\\\\\\x..." → 实串含 2 个反斜杠 (traced-value-unittest.cc:122)
  let string1 = "abc\"'\\\\x\"y'z\n\x09\x17";
  let chars127: String = (1..=127u8).map(char::from).collect();
  let mut value = TracedValue::create();
  value.set_string("a", string1);
  value.set_string("b", &chars127);

  let mut json = String::new();
  value.append_as_trace_format(&mut json);
  let expected = r##"{"a":"abc\"'\\\\x\"y'z\n\t\u0017","b":"\u0001\u0002\u0003\u0004\u0005\u0006\u0007\b\t\n\u000B\f\r\u000E\u000F\u0010\u0011\u0012\u0013\u0014\u0015\u0016\u0017\u0018\u0019\u001A\u001B\u001C\u001D\u001E\u001F !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u007F"}"##;
  assert_eq!(json, expected);
  OK
}

/// Utf8 (traced-value-unittest.cc:144-159)。
#[test]
fn traced_value_test_utf8() -> Void {
  let string1 = "Люблю тебя, Петра творенье";
  let string2 = "☀\u{2600}\u{26FF}";
  let mut value = TracedValue::create();
  value.set_string("a", string1);
  value.set_string("b", string2);
  // Surrogate pair test. Smile emoji === U+1F601 === \xf0\x9f\x98\x81
  value.set_string("c", "\u{1F601}");
  let mut json = String::new();
  value.append_as_trace_format(&mut json);
  assert_eq!(
    json,
    format!("{{\"a\":\"{string1}\",\"b\":\"{string2}\",\"c\":\"\u{1F601}\"}}")
  );
  OK
}

/// fmt_double ECMA-262 9.8.1 分段边界 (conversions.cc:1178-1207 同规则):
/// NaN/±Inf/-0/int32 快路径/定点/补零/小数前导零/指数 ±。
#[test]
fn traced_value_double_formatting_boundaries() -> Void {
  let cases: &[(&str, f64, &str)] = &[
    ("nan", f64::NAN, "NaN"),
    ("inf", f64::INFINITY, "Infinity"),
    ("-inf", f64::NEG_INFINITY, "-Infinity"),
    ("-0", -0.0, "0"),
    ("int32-max", 2147483647.0, "2147483647"),
    ("int32-min", -2147483648.0, "-2147483648"),
    // 2^31 越过 kMaxInt → 脱离 Int32Double 快路径, 走最短位数 (conversions-inl.h:200-204)
    ("int32-over", 2147483648.0, "2147483648"),
    ("frac", 10.5, "10.5"),
    ("-frac", -20.25, "-20.25"),
    ("half", 0.5, "0.5"),
    // 步骤 8 边界: point = -5 (0.000001) 仍在定点区, -6 起指数
    ("1e-6", 0.000001, "0.000001"),
    ("1e-7", 0.0000001, "1e-7"),
    // 步骤 6/7 边界: 1e20 补零, 1e21 起指数
    ("1e20", 1e20, "100000000000000000000"),
    ("1e21", 1e21, "1e+21"),
    ("1.5e21", 1.5e21, "1.5e+21"),
    // 最短表示舍入
    ("shortest", 123.456, "123.456"),
    ("neg-exp", 1.5e-7, "1.5e-7"),
  ];
  for &(name, v, want) in cases {
    let mut tv = TracedValue::create();
    tv.set_double("d", v);
    let mut json = String::new();
    tv.append_as_trace_format(&mut json);
    assert_eq!(json, format!("{{\"d\":{want}}}"), "case {name}");
  }
  OK
}

/// `Default` 与原版构造器同态: 根容器恒为字典 (traced-value.cc:78-80
/// `DEBUG_PUSH_CONTAINER(kStackTypeDict)`), 首项无逗号, 输出完整 `{...}`。
#[test]
fn traced_value_default_root_dict() -> Void {
  let mut value = TracedValue::default();
  value.set_integer("i", 1);
  let mut json = String::new();
  value.append_as_trace_format(&mut json);
  assert_eq!(json, "{\"i\":1}");
  OK
}
