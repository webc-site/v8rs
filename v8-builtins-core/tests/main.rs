use std::str::FromStr;

use aok::{OK, Void};
use log::info;
use strum::IntoEnumIterator;
use v8_builtins_core::{
  BUILTIN_LIST_BASE_TIER0_USE_COUNT, BUILTIN_LIST_BASE_TIER1_USE_COUNT,
  BUILTIN_LIST_INTL_USE_COUNT, BUILTIN_LIST_TEMPORAL_USE_COUNT, BUILTIN_TABLE, BUILTIN_TABLE_LEN,
  BuiltinKind, CC_BUILTIN_COUNT, Error, TQ_INFRA_DECLS, count_by_kind, descriptor_name,
  find_builtin, get_builtin, is_tier0,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_builtin_table_length_and_boundaries() -> Void {
  // 1. 验证内建表长度契约
  assert_eq!(BUILTIN_TABLE_LEN, 1111);
  assert_eq!(BUILTIN_TABLE.len(), BUILTIN_TABLE_LEN);

  // 2. 表首项契约: DeoptimizationEntry_Eager (ASM, line 77)
  let (kind_0, name_0, line_0) = BUILTIN_TABLE[0];
  assert_eq!(kind_0, "ASM");
  assert_eq!(name_0, "DeoptimizationEntry_Eager");
  assert_eq!(line_0, 77);

  // 3. 表末项契约: StringPrototypeToUpperCase (CPP, line 2020)
  let (kind_last, name_last, line_last) = BUILTIN_TABLE[BUILTIN_TABLE_LEN - 1];
  assert_eq!(kind_last, "CPP");
  assert_eq!(name_last, "StringPrototypeToUpperCase");
  assert_eq!(line_last, 2020);

  // 4. get_builtin 边界测试
  assert_eq!(
    get_builtin(0),
    Some(("ASM", "DeoptimizationEntry_Eager", 77))
  );
  assert_eq!(
    get_builtin(BUILTIN_TABLE_LEN - 1),
    Some(("CPP", "StringPrototypeToUpperCase", 2020))
  );
  assert_eq!(get_builtin(BUILTIN_TABLE_LEN), None);
  assert_eq!(get_builtin(BUILTIN_TABLE_LEN + 100), None);

  info!("内建表长度与边界项契约验证通过");
  OK
}

#[test]
fn test_tier0_builtins() -> Void {
  // Tier0 内建是保证靠近根寄存器以实现 root-relative 高效调用的核心内建 (前 7 个)
  let expected_tier0 = [
    ("ASM", "DeoptimizationEntry_Eager", 77),
    ("ASM", "DeoptimizationEntry_Lazy", 78),
    ("ASM", "DeoptimizationEntry_LazyAfterFastCall", 79),
    ("TFC", "RecordWriteSaveFP", 82),
    ("TFC", "RecordWriteIgnoreFP", 83),
    ("TFC", "EphemeronKeyBarrierSaveFP", 84),
    ("TFC", "EphemeronKeyBarrierIgnoreFP", 85),
  ];

  for (i, (expected_k, expected_n, expected_l)) in expected_tier0.into_iter().enumerate() {
    assert!(is_tier0(i), "索引 {i} 必须属于 Tier0");
    let (kind, name, line) = BUILTIN_TABLE[i];
    assert_eq!(kind, expected_k);
    assert_eq!(name, expected_n);
    assert_eq!(line, expected_l);
  }

  // 索引 >= 7 不再属于 Tier0
  assert!(!is_tier0(7));
  assert!(!is_tier0(100));
  assert!(!is_tier0(BUILTIN_TABLE_LEN));

  // 机械使用计数断言
  assert_eq!(BUILTIN_LIST_BASE_TIER0_USE_COUNT, 3);
  assert_eq!(BUILTIN_LIST_BASE_TIER1_USE_COUNT, 3);
  assert_eq!(BUILTIN_LIST_TEMPORAL_USE_COUNT, 4);
  assert_eq!(BUILTIN_LIST_INTL_USE_COUNT, 4);

  info!("Tier0 核心内建名单与判定契约验证通过");
  OK
}

#[test]
fn test_builtin_kind_and_strum() -> Void {
  // 1. 枚举遍历与 strum 序列化/反序列化
  let mut kinds_seen = 0;
  for kind in BuiltinKind::iter() {
    kinds_seen += 1;
    let s = kind.to_string();
    let parsed = BuiltinKind::from_str(&s)?;
    assert_eq!(kind, parsed);
  }
  assert_eq!(kinds_seen, 9); // CPP, TFJ_TSA, TFJ, TFC_TSA, TFC, TFS, TFH, ASM, BCH

  // 2. 特殊种类谓词测试
  assert!(BuiltinKind::CPP.is_cpp());
  assert!(!BuiltinKind::ASM.is_cpp());

  assert!(BuiltinKind::TFJ.has_js_linkage());
  assert!(BuiltinKind::TFJTSA.has_js_linkage());
  assert!(!BuiltinKind::TFS.has_js_linkage());

  assert!(BuiltinKind::TFC.is_code_stub());
  assert!(BuiltinKind::TFCTSA.is_code_stub());
  assert!(BuiltinKind::TFS.is_code_stub());
  assert!(!BuiltinKind::CPP.is_code_stub());

  assert!(BuiltinKind::ASM.is_asm());
  assert!(!BuiltinKind::TFH.is_asm());

  assert!(BuiltinKind::BCH.is_bytecode_handler());
  assert!(!BuiltinKind::CPP.is_bytecode_handler());

  info!("BuiltinKind 种类枚举与谓词逻辑验证通过");
  OK
}

#[test]
fn test_descriptor_names() -> Void {
  assert_eq!(
    descriptor_name("RecordWriteSaveFP"),
    "Builtin_RecordWriteSaveFP_InterfaceDescriptor"
  );
  assert_eq!(
    descriptor_name("CallFunction"),
    "Builtin_CallFunction_InterfaceDescriptor"
  );
  assert_eq!(CC_BUILTIN_COUNT, 0);

  info!("descriptor_name 映射生成测试通过");
  OK
}

#[test]
fn test_find_builtin_and_count_by_kind() -> Void {
  // 1. 查找有效内建
  assert_eq!(find_builtin("DeoptimizationEntry_Eager"), Some(0));
  assert_eq!(find_builtin("RecordWriteSaveFP"), Some(3));
  assert_eq!(find_builtin("CallBoundFunction"), Some(26));

  // 2. 查找不存在的内建
  assert_eq!(find_builtin("NonExistentBuiltin_XYZ"), None);
  assert_eq!(find_builtin(""), None);

  // 3. 各 kind 计数统计与总数守恒
  let mut total = 0;
  for k in [
    "CPP", "TFJ_TSA", "TFJ", "TFC_TSA", "TFC", "TFS", "TFH", "ASM", "BCH",
  ] {
    let c = count_by_kind(k);
    total += c;
  }
  assert_eq!(total, BUILTIN_TABLE_LEN);

  info!("内建检索与种类频次统计验证通过");
  OK
}

#[test]
fn test_tq_infra_declarations() -> Void {
  // 验证 .tq 基建声明条目有效性
  assert_eq!(TQ_INFRA_DECLS.len(), 607);

  // 抽查前若干项有效性
  for (kind, name, file) in TQ_INFRA_DECLS.iter().take(20) {
    assert!(!kind.is_empty());
    assert!(!name.is_empty());
    assert!(file.ends_with(".tq"));
  }

  info!("TQ 基建声明元数据验证通过");
  OK
}

#[test]
fn test_builtin_error_types() -> Void {
  let err_name = Error::UnknownBuiltin("NoSuchBuiltin".into());
  assert_eq!(format!("{err_name}"), "Unknown builtin name: NoSuchBuiltin");

  let err_id = Error::InvalidBuiltinId(9999);
  assert_eq!(format!("{err_id}"), "Invalid builtin id: 9999");

  assert_eq!(err_name, Error::UnknownBuiltin("NoSuchBuiltin".into()));
  assert_ne!(err_name, Error::UnknownBuiltin("Other".into()));

  info!("内建错误类型与格式化契约验证通过");
  OK
}
