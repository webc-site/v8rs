//! 原版 v8/test/unittests/torque/torque-utils-unittest.cc 的 1:1 用例 + utils.h|cc 语义校验
//!
//! 断言规约清单 (先规约后断言, 强度守恒):
//! - torque_utils_file_uri_decode_illegal: 非 file:// 前缀、残缺 %XX 转义一律 None
//!   (torque-utils-unittest.cc:14-19 + utils.cc:95-124)
//! - torque_utils_file_uri_decode: 前缀剥离 + %XX 解码 (torque-utils-unittest.cc:21-31,
//!   Windows/POSIX 前缀差异见 utils.cc:87-91, 以 cfg!(windows) 复刻原 #ifdef)
//! - utils_string_literal_quote_unquote: 转义集 \\n \\r \\t \" \\\\ 双向
//!   (utils.cc:24-53/55-77)
//! - utils_naming_convention_predicates: IsLowerCamelCase / IsUpperCamelCase /
//!   IsSnakeCase / IsValidNamespaceConstName / IsValidTypeName (utils.cc:167-254)
//! - utils_case_transform_helpers: CapifyStringWithUnderscores (JS 特例) / Camelify /
//!   Snakeify / Dashify / UnderlinifyPath (utils.cc:256-323)
//! - utils_starts_with_single_underscore: 恰一个前导下划线 (utils.cc:325-327)
//! - utils_residue_class_arithmetic: 构造归一化、AlignmentLog2、加减乘、渲染
//!   (utils.h:423-492 + utils.cc:385-394)
//! - utils_stack_range_and_delete: TopRange / PushMany / Extend / DeleteRange / PopMany
//!   (utils.h:171-292)
//! - utils_worklist_and_deduplicator: 去重入队与出队后可重入、intern 稳定下标
//!   (utils.h:494-560 区间)

use aok::{OK, Void};
use log::info;
use v8_torque::{
  BottomOffset, Deduplicator, ResidueClass, Stack, StackRange, Worklist, camelify_string,
  capify_string_with_underscores, dashify_string, file_uri_decode, is_lower_camel_case,
  is_snake_case, is_upper_camel_case, is_valid_namespace_const_name, is_valid_type_name,
  snakeify_string, starts_with_single_underscore, string_literal_quote, string_literal_unquote,
  underlinify_path,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn bo(offset: usize) -> BottomOffset {
  BottomOffset { offset }
}

/// 原版 TEST(TorqueUtils, FileUriDecodeIllegal) 1:1
#[test]
fn torque_utils_file_uri_decode_illegal() -> Void {
  info!("> torque_utils_file_uri_decode_illegal");
  assert_eq!(file_uri_decode("http://wrong.scheme"), None);
  assert_eq!(file_uri_decode("file://wrong-escape%"), None);
  assert_eq!(file_uri_decode("file://another-wrong-escape%a"), None);
  assert_eq!(file_uri_decode("file://no-hex-escape%0g"), None);
  OK
}

/// 原版 TEST(TorqueUtils, FileUriDecode) 1:1 (#ifdef V8_OS_WIN → cfg!(windows))
#[test]
fn torque_utils_file_uri_decode() -> Void {
  info!("> torque_utils_file_uri_decode");
  if cfg!(windows) {
    assert_eq!(
      file_uri_decode("file:///c%3A/torque/base.tq").as_deref(),
      Some("c:/torque/base.tq")
    );
    assert_eq!(
      file_uri_decode("file:///d%3a/lower/hex.txt").as_deref(),
      Some("d:/lower/hex.txt")
    );
  } else {
    assert_eq!(
      file_uri_decode("file:///some/src/file.tq").as_deref(),
      Some("/some/src/file.tq")
    );
  }
  OK
}

/// 原版 StringLiteralQuote / StringLiteralUnquote (utils.cc:24-77)
#[test]
fn utils_string_literal_quote_unquote() -> Void {
  info!("> utils_string_literal_quote_unquote");
  assert_eq!(string_literal_quote("plain"), "\"plain\"");
  assert_eq!(string_literal_quote("a\nb"), "\"a\\nb\"");
  assert_eq!(string_literal_quote("a\r\nb"), "\"a\\r\\nb\"");
  assert_eq!(string_literal_quote("a\tb"), "\"a\\tb\"");
  assert_eq!(string_literal_quote("say \"hi\""), "\"say \\\"hi\\\"\"");
  assert_eq!(string_literal_quote("back\\slash"), "\"back\\\\slash\"");
  assert_eq!(string_literal_unquote("\"a\\nb\""), "a\nb");
  assert_eq!(string_literal_unquote("'it\\'s'"), "it's");
  assert_eq!(string_literal_unquote("\"\\\"x\\\\\""), "\"x\\");
  assert_eq!(string_literal_unquote("\"tab\\there\""), "tab\there");
  // 往返不变量: quote 的像必可 unquote 回原值 (原版即此互逆对)
  for raw in ["", "abc", "a\nb\rc\td", "q\"z", "back\\slash"] {
    assert_eq!(string_literal_unquote(&string_literal_quote(raw)), raw);
  }
  OK
}

/// 原版命名规范谓词 (utils.cc:223-254)
#[test]
fn utils_naming_convention_predicates() -> Void {
  info!("> utils_naming_convention_predicates");
  // IsLowerCamelCase: 首字符小写且余段无下划线, 允许一个前导下划线
  assert!(is_lower_camel_case("foo"));
  assert!(is_lower_camel_case("fooBar"));
  assert!(is_lower_camel_case("_fooBar"));
  assert!(!is_lower_camel_case("FooBar"));
  assert!(!is_lower_camel_case("foo_bar"));
  assert!(!is_lower_camel_case("_foo_bar"));
  assert!(!is_lower_camel_case(""));
  // IsUpperCamelCase: 仅查首字符大写 (允许前导下划线)
  assert!(is_upper_camel_case("FooBar"));
  assert!(is_upper_camel_case("F"));
  assert!(is_upper_camel_case("_Foo"));
  assert!(!is_upper_camel_case("fooBar"));
  assert!(!is_upper_camel_case(""));
  // IsSnakeCase: 无大写即 snake
  assert!(is_snake_case("foo_bar"));
  assert!(is_snake_case("foo1"));
  assert!(!is_snake_case("Foo"));
  assert!(!is_snake_case("fooBar"));
  assert!(!is_snake_case(""));
  // IsValidTypeName: 机器类型白名单 或 UpperCamelCase
  assert!(is_valid_type_name("int32"));
  assert!(is_valid_type_name("void"));
  assert!(is_valid_type_name("never"));
  assert!(is_valid_type_name("float64_or_undefined_or_hole"));
  assert!(is_valid_type_name("JSObject"));
  assert!(!is_valid_type_name("fooBar"));
  assert!(!is_valid_type_name("kFoo"));
  assert!(!is_valid_type_name(""));
  // IsValidNamespaceConstName: 关键字样名 或 k + UpperCamelCase
  assert!(is_valid_namespace_const_name("kFooBar"));
  assert!(is_valid_namespace_const_name("kFoo_Bar"));
  assert!(is_valid_namespace_const_name("True"));
  assert!(is_valid_namespace_const_name("Undefined"));
  assert!(is_valid_namespace_const_name("TheHole"));
  assert!(!is_valid_namespace_const_name("fooBar"));
  assert!(!is_valid_namespace_const_name("KFoo"));
  assert!(!is_valid_namespace_const_name("k"));
  assert!(!is_valid_namespace_const_name(""));
  OK
}

/// 原版大小写/分隔符变换 (utils.cc:256-323)
#[test]
fn utils_case_transform_helpers() -> Void {
  info!("> utils_case_transform_helpers");
  // CapifyStringWithUnderscores: 小写/数字后接大写断词, "JS" 后强制断词, . 与 - 归一为 _
  assert_eq!(capify_string_with_underscores("fooBar"), "FOO_BAR");
  assert_eq!(capify_string_with_underscores("JSObject"), "JS_OBJECT");
  assert_eq!(capify_string_with_underscores("foo-bar.baz"), "FOO_BAR_BAZ");
  assert_eq!(
    capify_string_with_underscores("base26Value"),
    "BASE26_VALUE"
  );
  // CamelifyString: 下划线/连字符断词首字母大写
  assert_eq!(camelify_string("foo_bar"), "FooBar");
  assert_eq!(camelify_string("a-b_c"), "ABC");
  assert_eq!(camelify_string(""), "");
  // SnakeifyString: 小写后接大写插入下划线
  assert_eq!(snakeify_string("FooBarBaz"), "foo_bar_baz");
  assert_eq!(snakeify_string("ABC"), "abc");
  assert_eq!(snakeify_string(""), "");
  // DashifyString / UnderlinifyPath
  assert_eq!(dashify_string("foo_bar"), "foo-bar");
  assert_eq!(underlinify_path("src/torque/base.tq"), "SRC_TORQUE_BASE_TQ");
  assert_eq!(underlinify_path("a-b/c.d\\e"), "A_B_C_D_E");
  OK
}

/// 原版 StartsWithSingleUnderscore (utils.cc:325-327)
#[test]
fn utils_starts_with_single_underscore() -> Void {
  info!("> utils_starts_with_single_underscore");
  assert!(starts_with_single_underscore("_foo"));
  assert!(starts_with_single_underscore("_a"));
  assert!(!starts_with_single_underscore("__foo"));
  assert!(!starts_with_single_underscore("_"));
  assert!(!starts_with_single_underscore("foo"));
  assert!(!starts_with_single_underscore(""));
  OK
}

/// 原版 ResidueClass 模运算与渲染 (utils.h:423-492 + utils.cc:385-394)
#[test]
fn utils_residue_class_arithmetic() -> Void {
  info!("> utils_residue_class_arithmetic");
  // 构造: 模数钳到 kMaxModulusLog2, 小于全模时 value 归一化
  let mod3 = ResidueClass::new(8, 3);
  assert_eq!(mod3.value(), 0);
  assert_eq!(mod3.modulus_log_2(), 3);
  assert_eq!(mod3.single_value(), None);
  // value==0 → AlignmentLog2 = modulus_log_2 (utils.cc:385-388)
  assert_eq!(mod3.alignment_log2(), 3);
  assert_eq!(mod3.alignment(), 8);
  assert_eq!(mod3.to_display_string(), "[0 mod 2^3]");
  // 全模即具体值
  let concrete = ResidueClass::new(4, ResidueClass::K_MAX_MODULUS_LOG2);
  assert_eq!(concrete.single_value(), Some(4));
  assert_eq!(concrete.alignment_log2(), 2);
  assert_eq!(concrete.to_display_string(), "4");
  // Unknown = 0 mod 1 = 全体整数
  let unknown = ResidueClass::unknown();
  assert_eq!(unknown.modulus_log_2(), 0);
  assert_eq!(unknown.alignment_log2(), 0);
  assert_eq!(unknown.to_display_string(), "[0 mod 2^0]");
  // 加法: 值相加、模数取小, 再归一化
  assert_eq!(
    ResidueClass::new(3, 3)
      .add(&ResidueClass::new(5, 3))
      .to_display_string(),
    "[0 mod 2^3]"
  );
  // 乘法: 新模数 = min(a.m + b.align, b.m + a.align)
  assert_eq!(
    ResidueClass::new(3, 3)
      .mul(&ResidueClass::new(5, 3))
      .to_display_string(),
    "[7 mod 2^3]"
  );
  // 未知模乘具体值仍为未知
  assert_eq!(unknown.mul(&concrete).alignment_log2(), 2);
  OK
}

/// 原版 Stack / StackRange / BottomOffset 槽位操作 (utils.h:171-292)
#[test]
fn utils_stack_range_and_delete() -> Void {
  info!("> utils_stack_range_and_delete");
  let mut stack: Stack<i32> = Stack::from_vec(vec![1, 2, 3, 4]);
  assert_eq!(stack.size(), 4);
  assert_eq!(stack.above_top(), bo(4));
  // TopRange: [AboveTop - n, AboveTop)
  let range = stack.top_range(2);
  assert_eq!(range, StackRange::new(bo(2), bo(4)));
  assert_eq!(range.size(), 2);
  assert_eq!(*stack.peek(bo(2)), 3);
  assert_eq!(*stack.top(), 4);
  // PushMany 返回新增区间
  let pushed = stack.push_many(&[5, 6]);
  assert_eq!(pushed, StackRange::new(bo(4), bo(6)));
  assert_eq!(stack.size(), 6);
  // Extend: 相邻区间并接
  let mut head = StackRange::new(bo(2), bo(4));
  head.extend(StackRange::new(bo(4), bo(6)));
  assert_eq!(head.size(), 4);
  // DeleteRange: 删中段, 高位槽下移补位
  stack.delete_range(range);
  assert_eq!(stack.pop_many(2), vec![5, 6]);
  assert_eq!(stack.size(), 2);
  assert_eq!(stack.pop(), Some(2));
  assert_eq!(stack.pop(), Some(1));
  assert_eq!(stack.pop(), None);
  // Poke 就地改写
  let mut s2: Stack<String> = Stack::from_vec(vec!["a".to_owned(), "b".to_owned()]);
  s2.poke(bo(0), "z".to_owned());
  assert_eq!(s2.peek(bo(0)), "z");
  // BottomOffset 算术 (原版 operator+/-, 下溢为契约错误)
  assert_eq!(bo(3).plus(2), bo(5));
  assert_eq!(bo(3).minus(1), bo(2));
  OK
}

/// 原版 Worklist 去重队列 + Deduplicator 稳定下标 (utils.h 模板类)
#[test]
fn utils_worklist_and_deduplicator() -> Void {
  info!("> utils_worklist_and_deduplicator");
  let mut worklist: Worklist<u32> = Worklist::default();
  assert!(worklist.is_empty());
  assert!(worklist.enqueue(7));
  assert!(!worklist.enqueue(7));
  assert!(!worklist.is_empty());
  assert_eq!(worklist.dequeue(), Some(7));
  // 原版 Dequeue 会 erase contained_, 出队后可再次入队
  assert!(worklist.enqueue(7));
  assert_eq!(worklist.dequeue(), Some(7));
  assert!(worklist.is_empty());
  assert_eq!(worklist.dequeue(), None);

  let mut dedup: Deduplicator<String> = Deduplicator::default();
  let a = dedup.intern("alpha".to_owned());
  let b = dedup.intern("beta".to_owned());
  assert_eq!(a, 0);
  assert_eq!(b, 1);
  // 重复入池复用既有下标 (原版 unordered_set::insert)
  assert_eq!(dedup.intern("alpha".to_owned()), a);
  assert_eq!(dedup.get(b), "beta");
  assert_eq!(dedup.intern("gamma".to_owned()), 2);
  assert_eq!(dedup.get(2), "gamma");
  OK
}
