use std::str::from_utf8;

use aok::{OK, Void};
use log::info;
use v8_json::{Error, JsonValue, parse, parse_with_depth, stringify, stringify_indent};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_json_primitives() -> Void {
  info!("测试基础原始值解析与序列化");

  // null
  let v = parse("  null  ")?;
  assert_eq!(v, JsonValue::Null);
  assert!(v.is_null());
  assert_eq!(stringify(&v)?, "null");

  // boolean
  let v_true = parse("true")?;
  assert_eq!(v_true, JsonValue::Bool(true));
  assert_eq!(v_true.as_bool(), Some(true));
  assert_eq!(stringify(&v_true)?, "true");

  let v_false = parse("false")?;
  assert_eq!(v_false, JsonValue::Bool(false));
  assert_eq!(v_false.as_bool(), Some(false));
  assert_eq!(stringify(&v_false)?, "false");

  // numbers
  let v_num1 = parse("0")?;
  assert_eq!(v_num1.as_f64(), Some(0.0));
  assert_eq!(stringify(&v_num1)?, "0");

  let v_num2 = parse("-0")?;
  assert_eq!(v_num2.as_f64(), Some(0.0));
  assert!(v_num2.as_f64().unwrap().is_sign_negative());
  assert_eq!(stringify(&v_num2)?, "-0");

  let v_num3 = parse("123456789")?;
  assert_eq!(v_num3.as_f64(), Some(123456789.0));
  assert_eq!(stringify(&v_num3)?, "123456789");

  let v_num4 = parse("-42.5")?;
  assert_eq!(v_num4.as_f64(), Some(-42.5));
  assert_eq!(stringify(&v_num4)?, "-42.5");

  let v_num5 = parse("1.25e2")?;
  assert_eq!(v_num5.as_f64(), Some(125.0));

  let v_num6 = parse("-3.5E-2")?;
  assert_eq!(v_num6.as_f64(), Some(-0.035));

  // 特殊数值序列化: NaN 与 Infinity 按 JSON 规范输出 null
  assert_eq!(stringify(&JsonValue::Number(f64::NAN))?, "null");
  assert_eq!(stringify(&JsonValue::Number(f64::INFINITY))?, "null");

  OK
}

#[test]
fn test_json_strings_and_escapes() -> Void {
  info!("测试字符串与各种转义序列");

  // 普通字符与空字符串
  assert_eq!(parse("\"\"")?, JsonValue::String("".to_string()));
  assert_eq!(
    parse("\"hello world\"")?,
    JsonValue::String("hello world".to_string())
  );

  // 转义字符
  let json_esc = r#""\" \\ \/ \b \f \n \r \t""#;
  let parsed_esc = parse(json_esc)?;
  assert_eq!(
    parsed_esc,
    JsonValue::String("\" \\ / \u{0008} \u{000c} \n \r \t".to_string())
  );
  // 正斜杠在序列化时不需要转义
  assert_eq!(stringify(&parsed_esc)?, r#""\" \\ / \b \f \n \r \t""#);

  // 4位 16 进制 Unicode 转义
  assert_eq!(
    parse(r#""\u0041\u0042""#)?,
    JsonValue::String("AB".to_string())
  );

  // Unicode 代理对 (Surrogate Pairs, 如 😀 \uD83D\uDE00)
  assert_eq!(
    parse(r#""\uD83D\uDE00""#)?,
    JsonValue::String("😀".to_string())
  );

  // 中文多字节 UTF-8
  let chinese_json = r#""你好，V8 引擎！""#;
  assert_eq!(
    parse(chinese_json)?,
    JsonValue::String("你好，V8 引擎！".to_string())
  );

  OK
}

#[test]
fn test_json_arrays() -> Void {
  info!("测试数组解析与嵌套");

  // 空数组
  let empty = parse("[]")?;
  assert_eq!(empty, JsonValue::Array(vec![]));
  assert_eq!(stringify(&empty)?, "[]");

  // 混合元素数组
  let src = r#"[1, "abc", true, null, [2, 3]]"#;
  let arr = parse(src)?;
  assert_eq!(
    arr,
    JsonValue::Array(vec![
      JsonValue::Number(1.0),
      JsonValue::String("abc".to_string()),
      JsonValue::Bool(true),
      JsonValue::Null,
      JsonValue::Array(vec![JsonValue::Number(2.0), JsonValue::Number(3.0)]),
    ])
  );

  assert_eq!(arr.get_index(0), Some(&JsonValue::Number(1.0)));
  assert_eq!(arr.get_index(3), Some(&JsonValue::Null));
  assert_eq!(arr.get_index(5), None);

  // 序列化紧凑格式
  assert_eq!(stringify(&arr)?, r#"[1,"abc",true,null,[2,3]]"#);

  OK
}

#[test]
fn test_json_objects() -> Void {
  info!("测试对象解析与键值查询");

  // 空对象
  let empty = parse(" {  } ")?;
  assert_eq!(empty, JsonValue::Object(vec![]));
  assert_eq!(stringify(&empty)?, "{}");

  // 复杂嵌套对象
  let src = r#"{
    "name": "v8rs",
    "version": 1,
    "active": true,
    "items": [10, 20],
    "sub": {
      "key": "val"
    }
  }"#;

  let obj = parse(src)?;
  assert_eq!(
    obj.get("name"),
    Some(&JsonValue::String("v8rs".to_string()))
  );
  assert_eq!(obj.get("version"), Some(&JsonValue::Number(1.0)));
  assert_eq!(obj.get("active"), Some(&JsonValue::Bool(true)));
  assert_eq!(obj.get("not_exist"), None);

  if let Some(sub) = obj.get("sub") {
    assert_eq!(sub.get("key"), Some(&JsonValue::String("val".to_string())));
  } else {
    panic!("缺少 sub 字段");
  }

  OK
}

#[test]
fn test_json_stringify_indentation() -> Void {
  info!("测试带缩进的格式化序列化");

  let val = JsonValue::Object(vec![
    ("a".to_string(), JsonValue::Number(1.0)),
    (
      "b".to_string(),
      JsonValue::Array(vec![JsonValue::Bool(true), JsonValue::Null]),
    ),
  ]);

  let formatted = stringify_indent(&val, "  ")?;
  let expected = "{\n  \"a\": 1,\n  \"b\": [\n    true,\n    null\n  ]\n}";
  assert_eq!(formatted, expected);

  // 验证重新反序列化相等
  let reparsed = parse(&formatted)?;
  assert_eq!(reparsed, val);

  OK
}

#[test]
fn test_json_syntax_error_rejections() -> Void {
  info!("测试各种非法 JSON 输入的拦截与报错定位");

  // 1. 空或纯空白
  assert_eq!(parse("   "), Err(Error::UnexpectedEndOfInput));

  // 2. 非法前导零
  assert!(matches!(parse("01"), Err(Error::UnexpectedNumber { .. })));
  assert!(matches!(parse("-00"), Err(Error::UnexpectedNumber { .. })));

  // 3. 数字格式错误
  assert!(matches!(parse("1."), Err(Error::UnexpectedNumber { .. })));
  assert!(matches!(parse("1e"), Err(Error::UnexpectedNumber { .. })));
  assert!(matches!(parse("-"), Err(Error::UnexpectedNumber { .. })));
  assert!(matches!(parse("+1"), Err(Error::UnexpectedToken { .. })));

  // 4. 字符串控制字符未转义 (ASCII < 0x20)
  assert!(matches!(
    parse("\"\x01\""),
    Err(Error::BadControlCharacter { .. })
  ));

  // 5. 非法转义字符
  assert!(matches!(
    parse(r#""\a""#),
    Err(Error::BadEscapeCharacter { .. })
  ));

  // 6. 非法 Unicode 序列
  assert!(matches!(
    parse(r#""\u123z""#),
    Err(Error::BadUnicodeEscape { .. })
  ));

  // 7. 尾逗号拒绝
  assert!(matches!(
    parse("[1, 2,]"),
    Err(Error::UnexpectedToken { .. })
  ));
  assert!(matches!(
    parse(r#"{"a": 1,}"#),
    Err(Error::UnexpectedToken { .. })
  ));

  // 8. 键名缺少引号
  assert!(matches!(
    parse("{a: 1}"),
    Err(Error::UnexpectedToken { .. })
  ));

  // 9. 冒号缺失
  assert!(matches!(
    parse(r#"{"a" 1}"#),
    Err(Error::UnexpectedToken { .. })
  ));

  // 10. 输入尾部多余字符
  assert!(matches!(
    parse(r#""hello" world"#),
    Err(Error::UnexpectedToken { .. })
  ));

  // 11. 字面量过早结束返回 UnexpectedEndOfInput
  assert_eq!(parse("nul"), Err(Error::UnexpectedEndOfInput));
  assert_eq!(parse("tru"), Err(Error::UnexpectedEndOfInput));
  assert_eq!(parse("fals"), Err(Error::UnexpectedEndOfInput));

  // 12. 字面量字符不匹配精准定位
  assert_eq!(
    parse("nulk"),
    Err(Error::UnexpectedToken {
      token: "k".to_string(),
      position: 3,
    })
  );

  OK
}

#[test]
fn test_json_stack_depth_overflow() -> Void {
  info!("测试嵌套层级深度限制与防栈溢出");

  // 构造深度为 10 的嵌套数组 [[[[[[[[[[0]]]]]]]]]]
  let mut deeply_nested = "0".to_string();
  for _ in 0..10 {
    deeply_nested = format!("[{deeply_nested}]");
  }

  // 允许深度 12: 正常解析
  let parsed = parse_with_depth(&deeply_nested, 12)?;
  assert!(parsed.is_array());

  // 限制深度 5: 必须报 StackOverflow 且 limit=5
  let err = parse_with_depth(&deeply_nested, 5);
  assert_eq!(err, Err(Error::StackOverflow { limit: 5 }));

  OK
}

#[test]
fn test_json_token_contract_and_value_predicates() -> Void {
  info!("测试 JsonToken 契约与 JsonValue 类型判定");

  use v8_json::{EscapeKind, JsonToken};

  // 1. JsonToken 契约
  assert_eq!(JsonToken::COUNT, 14);
  assert_eq!(JsonToken::Lbrace.to_char(), Some('{'));
  assert_eq!(JsonToken::Rbrace.to_char(), Some('}'));
  assert_eq!(JsonToken::Lbrack.to_char(), Some('['));
  assert_eq!(JsonToken::Rbrack.to_char(), Some(']'));
  assert_eq!(JsonToken::Colon.to_char(), Some(':'));
  assert_eq!(JsonToken::Comma.to_char(), Some(','));
  assert_eq!(JsonToken::String.to_char(), Some('"'));
  assert_eq!(JsonToken::TrueLiteral.to_char(), Some('t'));
  assert_eq!(JsonToken::FalseLiteral.to_char(), Some('f'));
  assert_eq!(JsonToken::NullLiteral.to_char(), Some('n'));
  assert_eq!(JsonToken::Number.to_char(), None);
  assert_eq!(JsonToken::Whitespace.to_char(), None);
  assert_eq!(JsonToken::Illegal.to_char(), None);
  assert_eq!(JsonToken::Eos.to_char(), None);

  assert_eq!(JsonToken::from_repr(2), Some(JsonToken::Lbrace));
  assert_eq!(JsonToken::from_repr(255), None);

  // 2. EscapeKind 契约
  assert_eq!(EscapeKind::from_repr(0), EscapeKind::Illegal);
  assert_eq!(EscapeKind::from_repr(1), EscapeKind::SelfEscape);
  assert_eq!(EscapeKind::from_repr(2), EscapeKind::Backspace);
  assert_eq!(EscapeKind::from_repr(3), EscapeKind::Tab);
  assert_eq!(EscapeKind::from_repr(4), EscapeKind::NewLine);
  assert_eq!(EscapeKind::from_repr(5), EscapeKind::FormFeed);
  assert_eq!(EscapeKind::from_repr(6), EscapeKind::CarriageReturn);
  assert_eq!(EscapeKind::from_repr(7), EscapeKind::Unicode);

  // 3. JsonValue 类型判定方法
  let b = JsonValue::Bool(true);
  assert!(b.is_bool());
  assert!(!b.is_number());
  assert!(!b.is_string());

  let num = JsonValue::Number(42.0);
  assert!(num.is_number());
  assert!(!num.is_bool());

  let s = JsonValue::String("test".to_string());
  assert!(s.is_string());
  assert!(!s.is_number());

  // 4. \u0000 空字符序列化转义
  let null_char_str = parse(r#""foo\u0000bar""#)?;
  assert_eq!(null_char_str, JsonValue::String("foo\0bar".to_string()));
  assert_eq!(stringify(&null_char_str)?, r#""foo\u0000bar""#);

  OK
}

#[test]
fn test_stringifier_api_skel() -> Void {
  use v8_json::stringifier_api_skel::{self, StringifyResult};

  // 1. 空对象指针返回 Ok
  assert_eq!(
    unsafe { stringifier_api_skel::json_stringify(0, 0, 0, 0) },
    StringifyResult::Ok
  );

  // 2. 有效对象序列化
  let val = JsonValue::Array(vec![
    JsonValue::Number(1.0),
    JsonValue::String("test".to_string()),
  ]);
  let val_ptr = &val as *const JsonValue as usize;
  assert_eq!(
    unsafe { stringifier_api_skel::json_stringify(0, val_ptr, 0, 0) },
    StringifyResult::Ok
  );

  // 3. 安全序列化辅助
  let json_str = stringifier_api_skel::stringify_value(&val, None).unwrap();
  assert_eq!(json_str, r#"[1,"test"]"#);

  let json_indent = stringifier_api_skel::stringify_value(&val, Some("  ")).unwrap();
  assert!(json_indent.contains('\n'));

  OK
}

#[test]
fn test_fuzz_bad_unicode_escape_boundary() -> Void {
  // 覆盖 fuzz 捕获的非 ASCII 字节破坏 4 字节 hex 切片边界情况
  let payloads: &[&[u8]] = &[
    &[
      34, 92, 117, 49, 49, 240, 191, 139, 139, 124, 91, 92, 117, 50, 55, 51, 55, 55, 55,
    ],
    &[
      91, 48, 32, 32, 44, 34, 48, 92, 117, 48, 48, 51, 221, 162, 32, 46, 0, 0,
    ],
  ];
  for payload in payloads {
    if let Ok(s) = from_utf8(payload) {
      let res = parse(s);
      assert!(res.is_err());
    }
  }
  OK
}
