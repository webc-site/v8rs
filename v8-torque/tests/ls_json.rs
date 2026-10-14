//! 原版 v8/test/unittests/torque/ls-json-unittest.cc 1:1
//!
//! 断言规约清单 (先规约后断言, 强度守恒):
//! - language_server_json_primitives: true/false → Bool, null → Null,
//!   42 → Number(42) (ls-json-unittest.cc:17-32)
//! - language_server_json_strings: "\"basic\"" → "basic", "\"'\"" → "'"
//!   (ls-json-unittest.cc:34-42)
//! - language_server_json_arrays: 空数组 0 元素; [1,2,3,4] 4 元素且 [1] 为
//!   Number(2); ["a","b"] 2 元素且 [1] 为 "b" (ls-json-unittest.cc:44-64)
//! - language_server_json_objects: 空对象 0 键; {"flag":true,"id":5} 原始字段;
//!   {"array":[],"object":{"name":"torque"}} 嵌套字段 (ls-json-unittest.cc:66-96)
//! - language_server_json_parser_error: "{]" → error 含 "Parser Error:
//!   unexpected token" (ls-json-unittest.cc:107-112)
//! - language_server_json_lexer_error: "{ noquoteskey: null }" → error 含
//!   "Lexer Error: unknown token" (ls-json-unittest.cc:114-118)
//!
//! 附带序列化往返规约 (原版 json.cc SerializeToString 的强类型对偶):
//! - 值 → 文本 → parse 回同值 (对象 BTreeMap 序与原版 std::map 有序一致)

use aok::{OK, Void};
use log::info;
use v8_torque::{JsonValue, parse_json, serialize_to_string};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 解析成功路径 (失败即测试构造错误)
fn parsed(input: &str) -> JsonValue {
  match parse_json(input) {
    Ok(r) => match r.value {
      Some(v) => v,
      None => panic!("规约内输入必解析成功: {input}"),
    },
    Err(e) => panic!("规约内输入必解析成功: {input}: {e}"),
  }
}

/// 原版 TEST(LanguageServerJson, TestJsonPrimitives) 1:1
#[test]
fn language_server_json_primitives() -> Void {
  info!("> language_server_json_primitives");
  assert!(matches!(parsed("true"), JsonValue::Bool(true)));
  assert!(matches!(parsed("false"), JsonValue::Bool(false)));
  assert!(matches!(parsed("null"), JsonValue::Null));
  let JsonValue::Number(n) = parsed("42") else {
    panic!("42 必为 Number");
  };
  assert_eq!(n, 42.0);
  OK
}

/// 原版 TEST(LanguageServerJson, TestJsonStrings) 1:1
#[test]
fn language_server_json_strings() -> Void {
  info!("> language_server_json_strings");
  assert_eq!(parsed("\"basic\""), JsonValue::String("basic".to_owned()));
  // 双引号包裹的单引号字符
  assert_eq!(parsed("\"'\""), JsonValue::String("'".to_owned()));
  OK
}

/// 原版 TEST(LanguageServerJson, TestJsonArrays) 1:1
#[test]
fn language_server_json_arrays() -> Void {
  info!("> language_server_json_arrays");
  let JsonValue::Array(empty) = parsed("[]") else {
    panic!("[] 必为 Array");
  };
  assert_eq!(empty.len(), 0);

  let JsonValue::Array(numbers) = parsed("[1, 2, 3, 4]") else {
    panic!("[1,2,3,4] 必为 Array");
  };
  assert_eq!(numbers.len(), 4);
  let JsonValue::Number(n1) = &numbers[1] else {
    panic!("numbers[1] 必为 Number");
  };
  assert_eq!(*n1, 2.0);

  let JsonValue::Array(strings) = parsed("[\"a\", \"b\"]") else {
    panic!("[a,b] 必为 Array");
  };
  assert_eq!(strings.len(), 2);
  assert_eq!(strings[1], JsonValue::String("b".to_owned()));
  OK
}

/// 原版 TEST(LanguageServerJson, TestJsonObjects) 1:1
#[test]
fn language_server_json_objects() -> Void {
  info!("> language_server_json_objects");
  let JsonValue::Object(empty) = parsed("{}") else {
    panic!("空对象必为 Object");
  };
  assert_eq!(empty.len(), 0);

  let JsonValue::Object(fields) = parsed("{ \"flag\": true, \"id\": 5}") else {
    panic!("flag/id 必为 Object");
  };
  assert!(matches!(fields.get("flag"), Some(JsonValue::Bool(true))));
  let Some(JsonValue::Number(id)) = fields.get("id") else {
    panic!("id 必为 Number");
  };
  assert_eq!(*id, 5.0);

  let JsonValue::Object(complex) =
    parsed("{ \"array\": [], \"object\": { \"name\": \"torque\" } }")
  else {
    panic!("嵌套输入必为 Object");
  };
  let Some(JsonValue::Array(array)) = complex.get("array") else {
    panic!("array 必为 Array");
  };
  assert_eq!(array.len(), 0);
  let Some(JsonValue::Object(object)) = complex.get("object") else {
    panic!("object 必为 Object");
  };
  assert_eq!(
    object.get("name"),
    Some(&JsonValue::String("torque".to_owned()))
  );
  OK
}

/// 原版 TEST(LanguageServerJson, ParserError) 1:1
#[test]
fn language_server_json_parser_error() -> Void {
  info!("> language_server_json_parser_error");
  let result = parse_json("{]")?;
  let error = result.error.as_deref().unwrap_or("");
  assert!(result.error.is_some());
  assert!(error.contains("Parser Error: unexpected token"));
  OK
}

/// 原版 TEST(LanguageServerJson, LexerError) 1:1
#[test]
fn language_server_json_lexer_error() -> Void {
  info!("> language_server_json_lexer_error");
  let result = parse_json("{ noquoteskey: null }")?;
  assert!(result.error.is_some());
  let error = result.error.as_deref().unwrap_or("");
  assert!(error.contains("Lexer Error: unknown token"));
  OK
}

/// 序列化往返 (json.cc SerializeToString + ParseJson 的互逆性)
#[test]
fn language_server_json_serialize_roundtrip() -> Void {
  info!("> language_server_json_serialize_roundtrip");
  // 原版序列化格式: 无空白紧凑形态
  let v = parsed(
    "{ \"flag\": true, \"id\": 5, \"list\": [1, 2], \"nested\": { \"name\": \"torque\" }, \"nil\": null }",
  );
  assert_eq!(
    serialize_to_string(&v),
    "{\"flag\":true,\"id\":5,\"list\":[1,2],\"nested\":{\"name\":\"torque\"},\"nil\":null}"
  );
  // 往返不变量
  let text = serialize_to_string(&v);
  assert_eq!(parsed(&text), v);
  OK
}
