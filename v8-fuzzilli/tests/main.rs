use std::str::FromStr;

use aok::{OK, Void};
use log::info;
use v8_fuzzilli::{
  BUFFER_SIZE, BoolFlagRegistry, COV_EXPORT_DECLARATIONS, CRASH_ARMS, CRASH_SIGNALS,
  CoverageBitmap, EXTENSION_NAME, Error, FuzzilliCrashKind, FuzzilliExtension, FuzzilliOperation,
  MAX_BOOL_FLAG_REGIONS, MAX_EDGES, REPRL_CRFD, REPRL_CWFD, REPRL_DRFD, REPRL_DWFD,
  SANITIZER_CALLBACKS, SHM_SIZE, SUPPORT_MULTI_DSO, WILD_WRITE_ADDRESS, cov_is_edge_set,
  cov_set_edge,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_cov_constants_and_bitops() -> Void {
  info!("> test_cov_constants_and_bitops");
  assert_eq!(SHM_SIZE, 0x200000);
  assert_eq!(MAX_EDGES, (0x200000 - 4) * 8);
  const {
    assert!(!SUPPORT_MULTI_DSO);
  }
  assert_eq!(MAX_BOOL_FLAG_REGIONS, 64);
  assert_eq!(CRASH_SIGNALS.len(), 6);

  // 纯位运算验证
  let mut edges = [0u8; 8];
  assert!(!cov_is_edge_set(&edges, 0));
  assert!(!cov_is_edge_set(&edges, 17));

  cov_set_edge(&mut edges, 0);
  assert!(cov_is_edge_set(&edges, 0));
  assert_eq!(edges[0], 0b0000_0001);

  cov_set_edge(&mut edges, 7);
  assert!(cov_is_edge_set(&edges, 7));
  assert_eq!(edges[0], 0b1000_0001);

  cov_set_edge(&mut edges, 17);
  assert!(cov_is_edge_set(&edges, 17));
  assert_eq!(edges[2], 0b0000_0010);

  // 越界安全防护
  assert!(!cov_is_edge_set(&edges, 1000));
  cov_set_edge(&mut edges, 1000); // 不越界 panic
  OK
}

#[test]
fn test_coverage_bitmap_lifecycle() -> Void {
  info!("> test_coverage_bitmap_lifecycle");
  let mut bitmap = CoverageBitmap::new(64);
  assert_eq!(bitmap.num_edges(), 64);
  assert!(!bitmap.has_builtins_edges());

  // 置位普通边
  bitmap.set_edge(5)?;
  bitmap.set_edge(12)?;
  assert!(bitmap.is_edge_set(5));
  assert!(bitmap.is_edge_set(12));
  assert!(!bitmap.is_edge_set(6));

  // 初始化 builtins 覆盖率
  bitmap.init_builtins_edges(32)?;
  assert!(bitmap.has_builtins_edges());
  assert_eq!(bitmap.builtins_start(), 65);
  assert_eq!(bitmap.builtins_edge_count(), 32);

  // 重复初始化应报错
  let err = bitmap.init_builtins_edges(10);
  assert!(matches!(err, Err(Error::BuiltinsAlreadyInitialized)));

  // 置位内建函数边
  bitmap.set_builtin_edge(0)?; // 对应 edge 65
  bitmap.set_builtin_edge(31)?; // 对应 edge 96
  assert!(bitmap.is_edge_set(65));
  assert!(bitmap.is_edge_set(96));

  // 内建块索引越界
  let err_oob = bitmap.set_builtin_edge(32);
  assert!(matches!(err_oob, Err(Error::InvalidEdgeIndex(32))));

  // 发现边计数: 统计 [1, 65) 范围内的边
  let discovered = bitmap.count_discovered_edges();
  assert_eq!(discovered, 2); // edge 5 和 edge 12

  // 守卫重置
  let mut guards = [0u32; 10];
  bitmap.reset_edgeguards(&mut guards);
  assert_eq!(guards[0], 1);
  assert_eq!(guards[9], 10);

  // 清空位图
  bitmap.clear_edges();
  assert!(!bitmap.is_edge_set(5));
  assert!(!bitmap.is_edge_set(65));
  OK
}

#[test]
fn test_bool_flag_registry() -> Void {
  info!("> test_bool_flag_registry");
  let mut registry = BoolFlagRegistry::new();
  let mut bitmap = CoverageBitmap::new(100);

  // 注册第 1 个区域 (50 个标志)
  let first_idx = registry.register_region(50, &mut bitmap)?;
  assert_eq!(first_idx, 101);
  assert_eq!(registry.region_count(), 1);

  // 模拟 DSO 内标志设置
  let mut flags = vec![false; 50];
  flags[0] = true;
  flags[7] = true;
  flags[49] = true;

  // 冲刷标志到共享位图
  let flushed = registry.flush_bool_flags(0, &mut flags, &mut bitmap)?;
  assert_eq!(flushed, 3);
  // 冲刷后标志必须清零
  assert!(!flags[0]);
  assert!(!flags[7]);
  assert!(!flags[49]);

  // 位图中对应的边应当被置位
  assert!(bitmap.is_edge_set(101));
  assert!(bitmap.is_edge_set(108));
  assert!(bitmap.is_edge_set(150));

  // 废弃标志
  flags[2] = true;
  registry.discard_bool_flags(&mut flags);
  assert!(!flags[2]);
  OK
}

#[test]
fn test_fuzzilli_constants_and_crash_kinds() -> Void {
  info!("> test_fuzzilli_constants_and_crash_kinds");
  assert_eq!(REPRL_CRFD, 100);
  assert_eq!(REPRL_CWFD, 101);
  assert_eq!(REPRL_DRFD, 102);
  assert_eq!(REPRL_DWFD, 103);
  assert_eq!(EXTENSION_NAME, "v8/fuzzilli");
  assert_eq!(BUFFER_SIZE, 50);
  assert_eq!(WILD_WRITE_ADDRESS, 0x414141414141);

  // 12 臂枚举原样映射
  for i in 0..12 {
    let kind = FuzzilliCrashKind::from_i32(i)?;
    assert_eq!(kind as u16, i as u16);
  }

  // 错误参数拦截
  assert!(matches!(
    FuzzilliCrashKind::from_i32(-1),
    Err(Error::InvalidCrashKind(-1))
  ));
  assert!(matches!(
    FuzzilliCrashKind::from_i32(12),
    Err(Error::InvalidCrashKind(12))
  ));

  // 谓词测试
  assert!(FuzzilliCrashKind::CheckFalse.is_assertion_failure());
  assert!(FuzzilliCrashKind::DCheckFalse.is_assertion_failure());
  assert!(!FuzzilliCrashKind::ImmediateCrash.is_assertion_failure());

  assert!(FuzzilliCrashKind::WildWrite.is_memory_safety_violation());
  assert!(FuzzilliCrashKind::UseAfterFree.is_memory_safety_violation());
  assert!(FuzzilliCrashKind::OutOfBounds1.is_memory_safety_violation());
  assert!(FuzzilliCrashKind::OutOfBounds2.is_memory_safety_violation());
  assert!(FuzzilliCrashKind::HoleFuzzing.is_memory_safety_violation());

  assert!(FuzzilliCrashKind::SigillUd2.is_sigill());
  assert!(FuzzilliCrashKind::SigillInvalidInsn.is_sigill());

  assert!(FuzzilliCrashKind::UseAfterFree.requires_asan());
  assert!(FuzzilliCrashKind::OutOfBounds2.requires_asan());
  assert!(!FuzzilliCrashKind::OutOfBounds1.requires_asan());

  assert!(FuzzilliCrashKind::HoleFuzzing.requires_hole_fuzzing());

  // 架构支持谓词
  assert!(FuzzilliCrashKind::SigillUd2.arch_supported("x64"));
  assert!(!FuzzilliCrashKind::SigillUd2.arch_supported("arm64"));
  assert!(FuzzilliCrashKind::SigillInvalidInsn.arch_supported("x64"));
  assert!(FuzzilliCrashKind::SigillInvalidInsn.arch_supported("arm64"));
  assert!(!FuzzilliCrashKind::SigillInvalidInsn.arch_supported("riscv64"));

  // Strum 序列化与反序列化
  let parsed = FuzzilliCrashKind::from_str("IMMEDIATE_CRASH")
    .map_err(|e| Error::InvalidOperation(e.to_string()))?;
  assert_eq!(parsed, FuzzilliCrashKind::ImmediateCrash);
  OK
}

#[test]
fn test_fuzzilli_operation_and_extension() -> Void {
  info!("> test_fuzzilli_operation_and_extension");
  let ext = FuzzilliExtension::new("fuzzilli");
  assert_eq!(ext.extension_name(), "v8/fuzzilli");
  assert_eq!(ext.function_name(), "fuzzilli");
  assert_eq!(ext.build_source(), "native function fuzzilli();");

  // 1. 分派 FUZZILLI_PRINT
  let mut output = Vec::new();
  let res_print = ext.dispatch("FUZZILLI_PRINT", Some("Hello Fuzzilli!"), &mut output)?;
  assert_eq!(res_print, None);
  let text = String::from_utf8(output).map_err(|e| Error::Io(e.to_string()))?;
  assert_eq!(text, "Hello Fuzzilli!\n");

  // 2. 分派 FUZZILLI_CRASH
  let res_crash = ext.dispatch("FUZZILLI_CRASH", Some("3"), &mut Vec::new())?;
  assert_eq!(res_crash, Some(FuzzilliCrashKind::WildWrite));

  // 3. 无效操作分派
  let res_invalid = ext.dispatch("UNKNOWN_OP", None, &mut Vec::new());
  assert!(matches!(res_invalid, Err(Error::InvalidOperation(_))));

  // 4. 解析测试
  let op = FuzzilliOperation::parse("FUZZILLI_CRASH", Some("0"))?;
  assert_eq!(
    op,
    FuzzilliOperation::Crash(FuzzilliCrashKind::ImmediateCrash)
  );
  OK
}

#[test]
fn test_table_and_declaration_ledgers() -> Void {
  info!("> test_table_and_declaration_ledgers");
  assert_eq!(COV_EXPORT_DECLARATIONS.len(), 9);
  assert!(COV_EXPORT_DECLARATIONS.contains(&"fuzzilli_cov_enable"));
  assert!(COV_EXPORT_DECLARATIONS.contains(&"cov_set_builtin_edge"));

  assert_eq!(SANITIZER_CALLBACKS.len(), 3);
  assert!(SANITIZER_CALLBACKS.contains(&"__sanitizer_cov_trace_pc_guard"));

  assert_eq!(CRASH_ARMS.len(), 12);
  assert_eq!(CRASH_ARMS[0].0, 0);
  assert_eq!(CRASH_ARMS[11].0, 11);
  OK
}
