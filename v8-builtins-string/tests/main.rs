use std::str::FromStr;

use aok::{OK, Void};
use log::info;
use strum::IntoEnumIterator;
use v8_builtins_core::{BUILTIN_TABLE_LEN, get_builtin};
use v8_builtins_string::{
  Error, StringBuiltin, string_compare, string_equal, string_prototype_replace, wasm_string_compare,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_string_builtins_iteration_and_strum() -> Void {
  let mut count = 0;
  for b in StringBuiltin::iter() {
    count += 1;
    let s = b.to_string();
    assert_eq!(s, b.name());

    let parsed = StringBuiltin::from_str(&s)?;
    assert_eq!(parsed, b);

    // 验证描述符符号规范
    let desc = b.descriptor_name();
    assert_eq!(desc, format!("Builtin_{}_InterfaceDescriptor", b.name()));
  }
  assert_eq!(count, 26);

  info!("StringBuiltin 枚举遍历与 strum 反序列化验证通过 (共 26 项)");
  OK
}

#[test]
fn test_string_builtins_global_table_link() -> Void {
  // 验证每个字符串内建在 v8-builtins-core 全局 BUILTIN_TABLE 的映射关系:
  // 基础表包含 21 项基础内建; 其余 5 项为原版 IF_TSA / IF_WASM 条件宏内建
  let mut found_count = 0;
  let mut conditional_count = 0;

  for b in StringBuiltin::iter() {
    if let Some(global_id) = b.global_id() {
      found_count += 1;
      assert!(global_id < BUILTIN_TABLE_LEN);

      let (kind, name, line) = get_builtin(global_id).expect("有效索引必定返回内建元数据");
      assert_eq!(name, b.name());
      assert!(!kind.is_empty());
      assert!(line > 0);
    } else {
      conditional_count += 1;
      // 5 项属于原版 IF_TSA / IF_WASM 宏内建
      assert!(matches!(
        b,
        StringBuiltin::StringFromCharCode
          | StringBuiltin::StringFromCodePointAt
          | StringBuiltin::WasmJSStringEqual
          | StringBuiltin::WasmStringAddNoMapCheck
          | StringBuiltin::WasmStringCompare
      ));
    }
  }

  assert_eq!(found_count, 21);
  assert_eq!(conditional_count, 5);

  info!("StringBuiltin 全局内建注册表映射与条件宏分流验证通过 (21 基础 + 5 条件)");
  OK
}

#[test]
fn test_string_builtins_categories() -> Void {
  // 原型方法分类
  assert!(StringBuiltin::StringPrototypeReplace.is_prototype_method());
  assert!(StringBuiltin::StringPrototypeSplit.is_prototype_method());
  assert!(StringBuiltin::StringPrototypeToLowerCase.is_prototype_method());
  assert!(!StringBuiltin::StringCompare.is_prototype_method());
  assert!(!StringBuiltin::StringRaw.is_prototype_method());

  // 比较类方法分类
  assert!(StringBuiltin::StringCompare.is_comparison());
  assert!(StringBuiltin::StringEqual.is_comparison());
  assert!(StringBuiltin::StringGreaterThan.is_comparison());
  assert!(StringBuiltin::StringLessThanOrEqual.is_comparison());
  assert!(StringBuiltin::WasmJSStringEqual.is_comparison());
  assert!(!StringBuiltin::StringRaw.is_comparison());

  // WASM 字符串互操作分类
  assert!(StringBuiltin::WasmJSStringEqual.is_wasm_string());
  assert!(StringBuiltin::WasmStringAddNoMapCheck.is_wasm_string());
  assert!(StringBuiltin::WasmStringCompare.is_wasm_string());
  assert!(!StringBuiltin::StringEqual.is_wasm_string());

  // 大小写转换分类
  assert!(StringBuiltin::StringPrototypeToLowerCase.is_case_conversion());
  assert!(StringBuiltin::StringPrototypeToUpperCase.is_case_conversion());
  assert!(StringBuiltin::StringPrototypeToLocaleLowerCase.is_case_conversion());
  assert!(!StringBuiltin::StringCompare.is_case_conversion());

  // 调度入口直接构造
  assert_eq!(string_compare(), StringBuiltin::StringCompare);
  assert_eq!(string_equal(), StringBuiltin::StringEqual);
  assert_eq!(
    string_prototype_replace(),
    StringBuiltin::StringPrototypeReplace
  );
  assert_eq!(wasm_string_compare(), StringBuiltin::WasmStringCompare);

  info!("StringBuiltin 语义分类判定逻辑验证通过");
  OK
}

#[test]
fn test_string_error_types() -> Void {
  let err1 = Error::InvalidCharCode(0xd800);
  assert_eq!(format!("{err1}"), "Invalid character code: 55296");

  let err2 = Error::IndexOutOfBounds {
    index: 10,
    length: 5,
  };
  assert_eq!(format!("{err2}"), "Index out of bounds: index 10, length 5");

  let err3 = Error::BuiltinNotFound("UnknownStringBuiltin".into());
  assert_eq!(
    format!("{err3}"),
    "Builtin not found in global table: UnknownStringBuiltin"
  );

  info!("Error 强类型错误枚举格式化验证通过");
  OK
}
