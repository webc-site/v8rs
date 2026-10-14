use aok::{OK, Void};
use v8_api::{
  FCA_API_ARGS_LENGTH, FCA_ARGS_LENGTH, FCA_FIRST_API_ARG_INDEX, FCA_RECEIVER_INDEX,
  FastIterateResult, MapAsArrayKind, OPEN_HANDLE_LIST, PCA_FIRST_API_ARG_INDEX,
  PCA_FULL_API_ARGS_LENGTH, PCA_FULL_ARGS_LENGTH, PCA_MANDATORY_API_ARGS_LENGTH,
  PCA_MANDATORY_ARGS_LENGTH, PCA_OPTIONAL_ARGS_LENGTH, SetAsArrayKind, TO_LOCAL_LIST,
  TO_LOCAL_NAMES, find_exported_static_root, find_to_local_rule,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_to_local_rules_and_handles() -> Void {
  // 关键映射条目规则查询
  assert_eq!(
    find_to_local_rule("Value"),
    Some(("ToLocal", "Object", false))
  );
  assert_eq!(
    find_to_local_rule("Array"),
    Some(("ToLocal", "JSArray", false))
  );
  assert_eq!(
    find_to_local_rule("WasmMemoryObject"),
    Some(("ToLocal", "WasmMemoryObject", true))
  );
  assert_eq!(find_to_local_rule("UnknownType"), None);

  // 验证所有 ToLocal 转换函数名均属于标准清单
  for (fn_name, ..) in &TO_LOCAL_LIST {
    assert!(TO_LOCAL_NAMES.contains(fn_name));
  }

  // 验证句柄开放列表条目非空
  for &(api_from, internal_to, _) in &OPEN_HANDLE_LIST {
    assert!(!api_from.is_empty());
    assert!(!internal_to.is_empty());
  }

  OK
}

#[test]
fn test_function_callback_arguments_invariants() -> Void {
  // 编译期派生不变量与布局关系断言
  const {
    assert!(FCA_ARGS_LENGTH == FCA_RECEIVER_INDEX + 1);
    assert!(FCA_API_ARGS_LENGTH == FCA_RECEIVER_INDEX - FCA_FIRST_API_ARG_INDEX);
    assert!(FCA_API_ARGS_LENGTH > 0);
    assert!(FCA_FIRST_API_ARG_INDEX >= 0);
  };

  OK
}

#[test]
fn test_property_callback_arguments_invariants() -> Void {
  // 编译期派生不变量与栈帧参数布局算术一致性
  const {
    assert!(PCA_FULL_ARGS_LENGTH == PCA_MANDATORY_ARGS_LENGTH + PCA_OPTIONAL_ARGS_LENGTH);
    assert!(PCA_MANDATORY_API_ARGS_LENGTH == PCA_MANDATORY_ARGS_LENGTH - PCA_FIRST_API_ARG_INDEX);
    assert!(PCA_FULL_API_ARGS_LENGTH == PCA_FULL_ARGS_LENGTH - PCA_FIRST_API_ARG_INDEX);
    assert!(PCA_FULL_API_ARGS_LENGTH > PCA_MANDATORY_API_ARGS_LENGTH);
  };

  OK
}

#[test]
fn test_api_static_roots_and_enums() -> Void {
  // 静态根导出表达式查询
  assert_eq!(
    find_exported_static_root("UndefinedValue"),
    Some("i::StaticReadOnlyRoot::kUndefinedValue")
  );
  assert_eq!(
    find_exported_static_root("NullValue"),
    Some("i::StaticReadOnlyRoot::kNullValue")
  );
  assert_eq!(
    find_exported_static_root("TrueValue"),
    Some("i::StaticReadOnlyRoot::kTrueValue")
  );
  assert_eq!(
    find_exported_static_root("FalseValue"),
    Some("i::StaticReadOnlyRoot::kFalseValue")
  );
  assert_eq!(find_exported_static_root("UnknownRoot"), None);

  // 迭代结果枚举互斥性
  assert_ne!(FastIterateResult::Exception, FastIterateResult::Finished);
  assert_ne!(FastIterateResult::Break, FastIterateResult::SlowPath);

  // 视图枚举互斥性
  assert_ne!(MapAsArrayKind::Entries as u8, MapAsArrayKind::Values as u8);
  assert_ne!(SetAsArrayKind::Entries as u8, SetAsArrayKind::Values as u8);

  OK
}
