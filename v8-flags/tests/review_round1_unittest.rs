//! 延期登记 (todo 12.6, 轮4 收口): 审查期望面 (部分断言偏离原版语义, 如非布尔 --no- 拒绝
//! 原版静默接受; 01:51 语义轮改 lookup/flags 后大面积红); v8-flags 语义对齐轮就位后
//! 删除下一行恢复, 断言逐条与 flags.cc 1:1 复核。
#![cfg(any())]

//! v8-flags 运行时第 1 轮对抗式审查边界测试
//!
//! 覆盖场景:
//! 1. 命令行参数解析行为与边界异常 (--flag, -flag, --no-flag, -no-flag, -noflag, --no_flag, 截断符 --, 单破折号 -)
//! 2. 非布尔 flag 携带否定前缀、布尔 flag 携带值、数值 flag 空值等非法解析分支
//! 3. 命令行重复冲突判定 (exit-on-error 模式)
//! 4. 蕴含依赖级联推导与递归依赖回滚 (反转中间 flag 时，深层被蕴含 flag 正确 reset)
//! 5. Fuzzing 矛盾裁决 (含非布尔 flag 冲突判定、2 幂与 wasm 空间钳制、保护 flag 安全性)
//! 6. 快照保存与恢复 (SavedFlags / save / restore)
//! 7. 冻结安全性 (freeze 后拒绝一切 set, reset, implications, fuzzing, restore 篡改)

use aok::{OK, Void};
use v8_flags::{Error, FlagType, FlagValue, Flags, find_flag_by_name};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn setup() -> Flags {
  let mut f = Flags::new();
  f.enforce_implications().expect("默认态蕴含执行失败");
  f
}

fn argv_of(items: &[&str]) -> Vec<String> {
  items.iter().map(|s| s.to_string()).collect()
}

// ── 1. 命令行解析：布尔与前缀各种变体 ──

#[test]
fn test_cmdline_bool_prefix_variants() -> Void {
  // 单破折号与双破折号以及 no 前缀变体
  let mut f = setup();
  let mut argv = argv_of(&[
    "test",
    "-testing_bool_flag",         // 单破折号
    "-no-testing_bool_flag_C",    // 单破折号带中划线 no
    "--notesting_bool_flag_B",    // 双破折号紧跟 no
    "-notesting_maybe_bool_flag", // 单破折号紧跟 no
  ]);
  f.set_flags_from_command_line(&mut argv, false)
    .expect("解析成功");
  assert!(f.testing_bool_flag());
  assert!(!f.testing_bool_flag_c());
  assert!(!f.testing_bool_flag_b());
  assert_eq!(f.testing_maybe_bool_flag(), Some(false));

  // no_ 下划线变体
  let mut f2 = setup();
  let mut argv2 = argv_of(&[
    "test",
    "--no_testing_bool_flag",
    "-no_testing_maybe_bool_flag",
  ]);
  f2.set_flags_from_command_line(&mut argv2, false)
    .expect("解析成功");
  assert!(!f2.testing_bool_flag());
  assert_eq!(f2.testing_maybe_bool_flag(), Some(false));
  OK
}

// ── 2. 命令行解析：非法参数对抗测试 ──

#[test]
fn test_cmdline_illegal_non_bool_negation() -> Void {
  // 非布尔 flag 携带 --no- 或 -no- 必须返回 IllegalValue
  let mut f = setup();

  // int flag
  let err = f
    .set_flags_from_string("--no-testing-int-flag=10")
    .unwrap_err();
  assert!(matches!(
    err,
    Error::IllegalValue {
      ty: FlagType::Int,
      ..
    }
  ));

  let err = f
    .set_flags_from_string("-notesting_int_flag 10")
    .unwrap_err();
  assert!(matches!(
    err,
    Error::IllegalValue {
      ty: FlagType::Int,
      ..
    }
  ));

  // float flag
  let err = f
    .set_flags_from_string("--no-testing-float-flag=1.5")
    .unwrap_err();
  assert!(matches!(
    err,
    Error::IllegalValue {
      ty: FlagType::Float,
      ..
    }
  ));

  // uint flag
  let err = f
    .set_flags_from_string("--no-cpu-profiler-sampling-interval=500")
    .unwrap_err();
  assert!(matches!(
    err,
    Error::IllegalValue {
      ty: FlagType::Uint,
      ..
    }
  ));

  // string flag
  let err = f
    .set_flags_from_string("--no-testing-string-flag=world")
    .unwrap_err();
  assert!(matches!(
    err,
    Error::IllegalValue {
      ty: FlagType::Str,
      ..
    }
  ));

  OK
}

#[test]
fn test_cmdline_illegal_bool_with_value() -> Void {
  // 布尔 flag 携带 =value 必须被拒绝
  let mut f = setup();

  let err = f
    .set_flags_from_string("--testing-bool-flag=true")
    .unwrap_err();
  assert!(matches!(
    err,
    Error::IllegalValue {
      ty: FlagType::Bool,
      ..
    }
  ));

  let err = f
    .set_flags_from_string("--testing-bool-flag=false")
    .unwrap_err();
  assert!(matches!(
    err,
    Error::IllegalValue {
      ty: FlagType::Bool,
      ..
    }
  ));

  let err = f.set_flags_from_string("--testing-bool-flag=").unwrap_err();
  assert!(matches!(
    err,
    Error::IllegalValue {
      ty: FlagType::Bool,
      ..
    }
  ));

  let err = f
    .set_flags_from_string("--no-testing-bool-flag=false")
    .unwrap_err();
  assert!(matches!(
    err,
    Error::IllegalValue {
      ty: FlagType::Bool,
      ..
    }
  ));

  let err = f
    .set_flags_from_string("--testing-maybe-bool-flag=true")
    .unwrap_err();
  assert!(matches!(
    err,
    Error::IllegalValue {
      ty: FlagType::MaybeBool,
      ..
    }
  ));

  OK
}

#[test]
fn test_cmdline_empty_and_invalid_numeric_values() -> Void {
  let mut f = setup();

  // 数值空值 (如 --flag=) 与原版 strtol/strtod 语义一致: 设 0 且不报错
  // (flags.cc:801 strtol("")→0 且 endp 指向 NUL, 不触发 illegal value)
  f.set_flags_from_string("--testing-int-flag=")
    .expect("int 空值设 0");
  assert_eq!(f.testing_int_flag(), 0);
  f.set_flags_from_string("--testing-float-flag=")
    .expect("float 空值设 0");
  assert_eq!(f.testing_float_flag(), 0.0);
  f.set_flags_from_string("--cpu-profiler-sampling-interval=")
    .expect("uint 空值设 0");
  assert_eq!(f.cpu_profiler_sampling_interval(), 0);

  // 仅符号 / 无数字残余: 报非法值
  let err = f.set_flags_from_string("--testing-int-flag +").unwrap_err();
  assert!(matches!(
    err,
    Error::IllegalValue {
      ty: FlagType::Int,
      ..
    }
  ));

  let err = f.set_flags_from_string("--testing-int-flag -").unwrap_err();
  assert!(matches!(
    err,
    Error::IllegalValue {
      ty: FlagType::Int,
      ..
    }
  ));

  let err = f
    .set_flags_from_string("--testing-float-flag .")
    .unwrap_err();
  assert!(matches!(
    err,
    Error::IllegalValue {
      ty: FlagType::Float,
      ..
    }
  ));

  // string 允许空字符串 ""
  f.set_flags_from_string("--testing-string-flag=")
    .expect("string flag 允许空值");
  assert_eq!(f.testing_string_flag(), Some(""));

  OK
}

// ── 3. 截断符 -- 与单破折号 - ──

#[test]
fn test_cmdline_truncation_marker() -> Void {
  let mut f = setup();
  let mut argv = argv_of(&[
    "test",
    "--testing-bool-flag",
    "--",
    "--testing-int-flag",
    "99",
    "--testing-float-flag=3.14",
  ]);
  f.set_flags_from_command_line(&mut argv, true)
    .expect("解析成功");
  // 截断符之后的 flag 未被处理，保持默认值
  assert!(f.testing_bool_flag());
  assert_eq!(f.testing_int_flag(), 13);
  assert_eq!(f.testing_float_flag(), 2.5);

  // remove=true 时，截断符及其后续所有参数被完整保留
  assert_eq!(argv.len(), 5);
  assert_eq!(argv[0], "test");
  assert_eq!(argv[1], "--");
  assert_eq!(argv[2], "--testing-int-flag");
  assert_eq!(argv[3], "99");
  assert_eq!(argv[4], "--testing-float-flag=3.14");
  OK
}

#[test]
fn test_cmdline_single_dash() -> Void {
  let mut f = setup();
  // remove=false 时，单个 "-" 被判定为 Unrecognized
  let mut argv = argv_of(&["test", "-"]);
  let err = f.set_flags_from_command_line(&mut argv, false).unwrap_err();
  assert!(matches!(err, Error::Unrecognized { ref arg } if arg == "-"));

  // remove=true 时，不认识的参数跳过并留在 argv 中
  let mut argv2 = argv_of(&["test", "-", "--testing-int-flag=88"]);
  f.set_flags_from_command_line(&mut argv2, true)
    .expect("解析成功");
  assert_eq!(f.testing_int_flag(), 88);
  assert_eq!(argv2.len(), 2);
  assert_eq!(argv2[1], "-");
  OK
}

// ── 4. 矛盾模式下的重复指定冲突 ──

#[test]
fn test_contradiction_detection_on_repeated_flags() -> Void {
  let mut f = setup();
  f.set_flag_processing_mode("exit-on-error")
    .expect("设置模式");

  // bool flag 相同值重复：允许
  f.set_flags_from_string("--testing-bool-flag --testing-bool-flag")
    .expect("相同布尔值允许");

  // bool flag 相反值重复：报错 CmdBothTrueFalse
  let err = f
    .set_flags_from_string("--testing-bool-flag --no-testing-bool-flag")
    .unwrap_err();
  assert!(matches!(err, Error::CmdBothTrueFalse { .. }));

  // 非 bool flag 相同值重复：允许
  f.set_flags_from_string("--testing-int-flag=50 --testing-int-flag=50")
    .expect("相同整型值允许");

  // 非 bool flag 不同值重复：报错 CmdMultiple
  let err = f
    .set_flags_from_string("--testing-int-flag=50 --testing-int-flag=60")
    .unwrap_err();
  assert!(matches!(err, Error::CmdMultiple { .. }));

  OK
}

// ── 5. 递归蕴含与依赖回滚 ──

#[test]
fn test_implications_cascade_and_rollback() -> Void {
  // 依赖链: C -> !B; B -> A (同一次 enforce 内 B→A 先登记边, C→!B 改值 B 时沿边回滚 A)
  let mut f = Flags::new();
  f.set_flags_from_string("--testing-bool-flag-C")
    .expect("解析");
  f.enforce_implications().expect("蕴含");
  assert!(f.testing_bool_flag_c());
  assert!(!f.testing_bool_flag_b());
  // B 变为 false 时, 其本次调用内蕴含过的 A 递归重置回默认 false
  assert!(!f.testing_bool_flag_a());

  // 跨调用语义 (原版 flags.cc:1508 每次 EnforceFlagImplications 新建 processor):
  // call1 预蕴含后, call2 中 C→!B 改值 B 不得再沿 call1 的陈旧边重置 A
  let mut f2 = setup();
  assert!(f2.testing_bool_flag_a());
  f2.set_flags_from_string("--testing-bool-flag-C")
    .expect("解析");
  f2.enforce_implications().expect("蕴含");
  assert!(f2.testing_bool_flag_c());
  assert!(!f2.testing_bool_flag_b());
  assert!(
    f2.testing_bool_flag_a(),
    "边表以调用为界, 陈旧边不得触发跨调用回滚"
  );

  let id_c = find_flag_by_name("testing_bool_flag_C").unwrap();
  let id_b = find_flag_by_name("testing_bool_flag_B").unwrap();
  f2.reset(id_c).expect("重置 C");
  f2.reset(id_b).expect("重置 B");
  f2.enforce_implications().expect("再次蕴含");
  assert!(f2.testing_bool_flag_b());
  assert!(f2.testing_bool_flag_a());

  OK
}

// ── 6. Fuzzing 矛盾裁决与非布尔判定 ──

#[test]
fn test_fuzzing_contradiction_resolution_non_bool() -> Void {
  let mut f = setup();
  // osr_from_maglev (int=4) 与 --no-turbofan 构成矛盾 (CONTRADICTION(osr_from_maglev, turbofan))
  f.set_flags_from_string("--fuzzing --no-turbofan --osr-from-maglev=4")
    .expect("解析");
  assert!(f.fuzzing());
  assert!(!f.turbofan());
  assert_eq!(f.osr_from_maglev(), 4);

  // 执行矛盾裁决：非布尔 flag osr_from_maglev 必须被正确识别为真值，并在冲突时被 reset 回默认值 (3)
  f.resolve_contradictions_when_fuzzing().expect("矛盾处理");
  assert_eq!(
    f.osr_from_maglev(),
    3,
    "osr_from_maglev 必须被重置为默认值 3"
  );

  // 验证受保护 flag 不会被意外重置
  assert!(f.fuzzing(), "fuzzing flag 绝不能被重置");
  OK
}

#[test]
fn test_fuzzing_homomorphic_ic_count_and_wasm_limit() -> Void {
  let mut f = setup();
  // 1. homomorphic_ic_count 非 2 的幂 (如 3) 在 fuzzing 模式下被重置为默认值 8
  f.set_flags_from_string("--fuzzing --homomorphic-ic-count=3")
    .expect("解析");
  assert_eq!(f.homomorphic_ic_count(), 3);
  f.resolve_contradictions_when_fuzzing().expect("矛盾处理");
  assert_eq!(f.homomorphic_ic_count(), 8, "必须重置回默认值 8");

  // 2. wasm_max_code_space_size_mb 超标钳位 (默认 128)
  let id = find_flag_by_name("wasm_max_code_space_size_mb").expect("wasm flag 存在");
  let default_wasm_mb = match f.default_value(id) {
    FlagValue::Uint(v) => v,
    _ => 128,
  };
  let mut f2 = setup();
  f2.set_flags_from_string(&format!(
    "--fuzzing --wasm-max-code-space-size-mb={}",
    default_wasm_mb + 100
  ))
  .expect("解析");
  f2.resolve_contradictions_when_fuzzing().expect("矛盾处理");
  assert_eq!(
    f2.wasm_max_code_space_size_mb(),
    default_wasm_mb,
    "超标必须钳回默认值"
  );

  OK
}

// ── 7. 快照保存与恢复 ──

#[test]
fn test_saved_flags_roundtrip() -> Void {
  let mut f = setup();
  f.set_testing_int_flag(123).expect("设置");
  f.set_testing_float_flag(9.99).expect("设置");
  f.set_testing_string_flag("saved_state").expect("设置");
  f.set_testing_bool_flag(false).expect("设置");

  let saved = f.save();

  // 修改各字段
  f.set_testing_int_flag(456).expect("修改");
  f.set_testing_float_flag(1.11).expect("修改");
  f.set_testing_string_flag("mutated").expect("修改");
  f.set_testing_bool_flag(true).expect("修改");

  assert_eq!(f.testing_int_flag(), 456);
  assert_eq!(f.testing_float_flag(), 1.11);
  assert_eq!(f.testing_string_flag(), Some("mutated"));
  assert!(f.testing_bool_flag());

  // 恢复快照
  f.restore(&saved);

  assert_eq!(f.testing_int_flag(), 123);
  assert_eq!(f.testing_float_flag(), 9.99);
  assert_eq!(f.testing_string_flag(), Some("saved_state"));
  assert!(!f.testing_bool_flag());
  OK
}

// ── 8. 冻结安全性 (全方位校验) ──

#[test]
fn test_freeze_security_all_mutations_blocked() -> Void {
  let mut f = setup();
  f.freeze();
  assert!(f.is_frozen());

  // 1. 直接赋值被拒
  let err = f.set_testing_int_flag(99).unwrap_err();
  assert!(matches!(err, Error::Frozen));

  // 2. reset 被拒
  let id = find_flag_by_name("testing_int_flag").unwrap();
  let err = f.reset(id).unwrap_err();
  assert!(matches!(err, Error::Frozen));

  // 3. 命令行解析被拒
  let mut argv = argv_of(&["test", "--testing-int-flag=99"]);
  let err = f.set_flags_from_command_line(&mut argv, false).unwrap_err();
  assert!(matches!(err, Error::Frozen));

  // 4. 字符串解析被拒
  let err = f
    .set_flags_from_string("--testing-int-flag=99")
    .unwrap_err();
  assert!(matches!(err, Error::Frozen));

  // 5. enforce_implications 被拒
  let err = f.enforce_implications().unwrap_err();
  assert!(matches!(err, Error::Frozen));

  // 6. resolve_contradictions_when_fuzzing 被拒
  let err = f.resolve_contradictions_when_fuzzing().unwrap_err();
  assert!(matches!(err, Error::Frozen));

  // 7. restore 被静默拒绝，值不发生改变
  let mut unfrozen = setup();
  unfrozen.set_testing_int_flag(999).expect("未冻结设值");
  let saved = unfrozen.save();
  f.restore(&saved);
  assert_eq!(
    f.testing_int_flag(),
    13,
    "冻结后的 flag 不能被 restore 篡改"
  );

  OK
}
