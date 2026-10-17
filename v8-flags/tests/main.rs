//! v8/test/unittests/flags/flag-definitions-unittest.cc 规约迁移 (测试名保留原名)
//! 差异备注 (相对原版 C++):
//! - 出错路径 Err 化 (原版 return_code 索引 / FlagError 终止); 出错时 argv 不收缩
//!   (原版部分收缩), 已解析的 flag 变更不回滚 (与原版一致)
//! - FreezeFlags 的内存页保护未实现, 冻结后 API 赋值返回 Err (原版 CHECK 失败)
//! - V8_FUZZ_TEST (FlagHelpersFuzzTest) 以代表性样例代替

use std::cmp::Ordering;

use aok::{OK, Void};
use log::info;
use sonic_rs::{JsonValueTrait, Value};
use v8_flags::{
  ALL_ENTRIES, CANON_ALIASES, CANON_ALL_ENTRIES, CANON_CONTRADICTIONS,
  CANON_CONTRADICTIONS_WASM_SCOPED, CANON_FEATURES, CANON_IMPLICATIONS, CANON_KIND_GENERIC,
  CANON_KIND_REQUIRE, CANON_KIND_VALUE, CANON_KIND_WARN, CANON_PRIMARY, CANON_READONLY,
  CANON_RESET_CORRECTNESS, CANON_RESET_FUZZING, CANON_TYPE_BOOL, CANON_TYPE_FLOAT, CANON_TYPE_INT,
  CANON_TYPE_MAYBE_BOOL, CANON_TYPE_SIZE_T, CANON_TYPE_STRING, CANON_TYPE_UINT, CANON_TYPE_UINT64,
  Error, FLAGS, FlagProcessingMode, FlagType, Flags, NUM_ALL_FLAGS, NUM_CONTRADICTIONS,
  NUM_FEATURES, NUM_FLAGS, NUM_GENERICS, NUM_IMPLICATIONS, NUM_REQUIRES, NUM_WARNINGS, SetBy,
  cmp_names, equal_names, feature_flags_json, features, find_flag_by_name, find_implication_flag,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 全默认 + 蕴含执行后的新实例 (对应原版 FlagDefinitionsTest::SetUp)
fn setup() -> Flags {
  let mut f = Flags::new();
  f.enforce_implications().expect("默认态蕴含执行失败");
  f
}

fn argv_of(items: &[&str]) -> Vec<String> {
  items.iter().map(|s| s.to_string()).collect()
}

// ── 对数门禁 (必做): 表实数 == counts 常量 == stats.json 三方核对 ──

#[test]
fn counts_consistency() -> Void {
  let stats: Value = sonic_rs::from_str(include_str!("../../tool/src/flags/stats.json"))
    .expect("stats.json 解析失败");
  let canon = &stats["canonical"];
  let num = |k: &str| canon[k].as_i64().expect("数值字段") as usize;

  // 主 flag: 三方一致
  assert_eq!(num("primary"), CANON_PRIMARY);
  assert_eq!(FLAGS.len(), CANON_PRIMARY);
  assert_eq!(NUM_FLAGS, CANON_PRIMARY);

  // 别名与全条目
  assert_eq!(num("aliases"), CANON_ALIASES);
  assert_eq!(ALL_ENTRIES.len() - FLAGS.len(), CANON_ALIASES);
  assert_eq!(num("all_entries"), CANON_ALL_ENTRIES);
  assert_eq!(ALL_ENTRIES.len(), CANON_ALL_ENTRIES);
  assert_eq!(NUM_ALL_FLAGS, CANON_ALL_ENTRIES);

  // 只读
  assert_eq!(num("readonly"), CANON_READONLY);
  assert_eq!(FLAGS.iter().filter(|f| f.readonly).count(), CANON_READONLY);

  // 类型细分
  let by_type = &canon["by_type"];
  let t = |k: &str| by_type[k].as_i64().expect("类型字段") as usize;
  let ty_count = |want: FlagType| FLAGS.iter().filter(|f| f.ty == want).count();
  assert_eq!(t("BOOL"), CANON_TYPE_BOOL);
  assert_eq!(ty_count(FlagType::Bool), CANON_TYPE_BOOL);
  assert_eq!(t("STRING"), CANON_TYPE_STRING);
  assert_eq!(ty_count(FlagType::Str), CANON_TYPE_STRING);
  assert_eq!(t("UINT"), CANON_TYPE_UINT);
  assert_eq!(ty_count(FlagType::Uint), CANON_TYPE_UINT);
  assert_eq!(t("INT"), CANON_TYPE_INT);
  assert_eq!(ty_count(FlagType::Int), CANON_TYPE_INT);
  assert_eq!(t("FLOAT"), CANON_TYPE_FLOAT);
  assert_eq!(ty_count(FlagType::Float), CANON_TYPE_FLOAT);
  assert_eq!(t("MAYBE_BOOL"), CANON_TYPE_MAYBE_BOOL);
  assert_eq!(ty_count(FlagType::MaybeBool), CANON_TYPE_MAYBE_BOOL);
  assert_eq!(t("SIZE_T"), CANON_TYPE_SIZE_T);
  assert_eq!(ty_count(FlagType::SizeT), CANON_TYPE_SIZE_T);
  assert_eq!(t("UINT64"), CANON_TYPE_UINT64);
  assert_eq!(ty_count(FlagType::Uint64), CANON_TYPE_UINT64);

  // 蕴含总数与种类细分 (编译期断言已核表实数 == counts, 此处核 stats.json)
  assert_eq!(num("implications"), CANON_IMPLICATIONS);
  assert_eq!(
    NUM_IMPLICATIONS + NUM_WARNINGS + NUM_GENERICS + NUM_REQUIRES,
    CANON_IMPLICATIONS
  );
  let by_kind = &canon["by_kind"];
  assert_eq!(
    by_kind["value"].as_i64().unwrap() as usize,
    NUM_IMPLICATIONS
  );
  assert_eq!(NUM_IMPLICATIONS, CANON_KIND_VALUE);
  assert_eq!(by_kind["warn"].as_i64().unwrap() as usize, NUM_WARNINGS);
  assert_eq!(NUM_WARNINGS, CANON_KIND_WARN);
  assert_eq!(by_kind["generic"].as_i64().unwrap() as usize, NUM_GENERICS);
  assert_eq!(NUM_GENERICS, CANON_KIND_GENERIC);
  assert_eq!(by_kind["require"].as_i64().unwrap() as usize, NUM_REQUIRES);
  assert_eq!(NUM_REQUIRES, CANON_KIND_REQUIRE);

  // 特性与矛盾
  assert_eq!(num("features"), CANON_FEATURES);
  assert_eq!(features().len(), CANON_FEATURES);
  assert_eq!(NUM_FEATURES, CANON_FEATURES);
  assert_eq!(num("contradictions"), CANON_CONTRADICTIONS);
  assert_eq!(NUM_CONTRADICTIONS, CANON_CONTRADICTIONS);
  assert_eq!(
    num("contradictions_wasm_scoped"),
    CANON_CONTRADICTIONS_WASM_SCOPED
  );
  assert_eq!(num("reset_fuzzing"), CANON_RESET_FUZZING);
  assert_eq!(num("reset_correctness"), CANON_RESET_CORRECTNESS);

  info!(
    "三方对数通过: primary={CANON_PRIMARY} implications={CANON_IMPLICATIONS} features={CANON_FEATURES} contradictions={CANON_CONTRADICTIONS}"
  );
  OK
}

// ── FlagDefinitionsTest ──

#[test]
fn default() -> Void {
  let f = setup();
  assert!(f.testing_bool_flag());
  assert_eq!(f.testing_int_flag(), 13);
  assert_eq!(f.testing_float_flag(), 2.5);
  assert_eq!(f.testing_string_flag(), Some("Hello, world!"));
  OK
}

#[test]
fn flags1() -> Void {
  // 原版 PrintHelp 冒烟: 输出到缓冲不崩溃, 语法头完整
  let f = setup();
  let mut out = String::new();
  f.print_help(&mut out).expect("print_help");
  assert!(out.starts_with("The following syntax for options is accepted"));
  assert!(out.contains("--flag        (bool flags only)"));
  let mut vals = String::new();
  f.print_values(&mut vals).expect("print_values");
  assert!(vals.contains("--testing-bool-flag"));
  OK
}

#[test]
fn flags2() -> Void {
  let mut f = setup();
  let mut argv = argv_of(&[
    "Test2",
    "-notesting-bool-flag",
    "--notesting-maybe-bool-flag",
    "notaflag",
    "--testing_int_flag=77",
    "-testing_float_flag=.25",
    "--testing_string_flag",
    "no way!",
  ]);
  f.set_flags_from_command_line(&mut argv, false)
    .expect("解析成功");
  // remove=false: argv 保持不变 (原版 CHECK_EQ(8, argc))
  assert_eq!(argv.len(), 8);
  assert!(!f.testing_bool_flag());
  assert_eq!(f.testing_maybe_bool_flag(), Some(false));
  assert_eq!(f.testing_int_flag(), 77);
  assert_eq!(f.testing_float_flag(), 0.25);
  assert_eq!(f.testing_string_flag(), Some("no way!"));
  OK
}

#[test]
fn flags2b() -> Void {
  let mut f = setup();
  f.set_flags_from_string(
    " -notesting-bool-flag notaflag   --testing_int_flag=77 -notesting-maybe-bool-flag   -testing_float_flag=.25  --testing_string_flag   no_way!  ",
  )
  .expect("解析成功");
  assert!(!f.testing_bool_flag());
  assert_eq!(f.testing_maybe_bool_flag(), Some(false));
  assert_eq!(f.testing_int_flag(), 77);
  assert_eq!(f.testing_float_flag(), 0.25);
  assert_eq!(f.testing_string_flag(), Some("no_way!"));
  OK
}

#[test]
fn flags3() -> Void {
  let mut f = setup();
  let mut argv = argv_of(&[
    "Test3",
    "--testing_bool_flag",
    "--testing-maybe-bool-flag",
    "notaflag",
    "--testing_int_flag",
    "-666",
    "--testing_float_flag",
    "-12E10",
    "-testing-string-flag=foo-bar",
  ]);
  f.set_flags_from_command_line(&mut argv, true)
    .expect("解析成功");
  // remove=true: 仅剩程序名与 "notaflag" (原版 CHECK_EQ(2, argc))
  assert_eq!(argv.len(), 2);
  assert_eq!(argv[1], "notaflag");
  assert!(f.testing_bool_flag());
  assert_eq!(f.testing_maybe_bool_flag(), Some(true));
  assert_eq!(f.testing_int_flag(), -666);
  assert_eq!(f.testing_float_flag(), -12e10);
  assert_eq!(f.testing_string_flag(), Some("foo-bar"));
  OK
}

#[test]
fn flags3b() -> Void {
  let mut f = setup();
  f.set_flags_from_string(
    "--testing_bool_flag --testing-maybe-bool-flag notaflag --testing_int_flag -666 --testing_float_flag -12E10 -testing-string-flag=foo-bar",
  )
  .expect("解析成功");
  assert!(f.testing_bool_flag());
  assert_eq!(f.testing_maybe_bool_flag(), Some(true));
  assert_eq!(f.testing_int_flag(), -666);
  assert_eq!(f.testing_float_flag(), -12e10);
  assert_eq!(f.testing_string_flag(), Some("foo-bar"));
  OK
}

#[test]
fn flags4() -> Void {
  let mut f = setup();
  let mut argv = argv_of(&["Test4", "--testing_bool_flag", "--foo"]);
  // remove=true: 未识别 flag 交给后续处理器, 不报错 (原版 rc=0)
  f.set_flags_from_command_line(&mut argv, true)
    .expect("解析成功");
  assert_eq!(argv.len(), 2); // 原版 CHECK_EQ(2, argc)
  assert_eq!(f.testing_maybe_bool_flag(), None);
  OK
}

#[test]
fn flags4b() -> Void {
  let mut f = setup();
  // remove=false: 未识别 flag 报错 (原版 rc=2)
  let err = f
    .set_flags_from_string("--testing_bool_flag --foo")
    .unwrap_err();
  assert!(matches!(err, Error::Unrecognized { arg } if arg == "--foo"));
  assert_eq!(f.testing_maybe_bool_flag(), None);
  OK
}

#[test]
fn flags5() -> Void {
  let mut f = setup();
  let mut argv = argv_of(&["Test5", "--testing_int_flag=\"foobar\""]);
  // 值非法: 数字解析零消耗 → 尾部残余报错 (原版 rc=1); argv 不变
  let err = f.set_flags_from_command_line(&mut argv, true).unwrap_err();
  assert!(matches!(err, Error::IllegalValue { .. }));
  assert_eq!(argv.len(), 2);
  OK
}

#[test]
fn flags5b() -> Void {
  let mut f = setup();
  let err = f
    .set_flags_from_string("                     --testing_int_flag=\"foobar\"")
    .unwrap_err();
  assert!(matches!(err, Error::IllegalValue { .. }));
  OK
}

#[test]
fn flags6() -> Void {
  let mut f = setup();
  let mut argv = argv_of(&["Test5", "--testing-int-flag", "0", "--testing_float_flag"]);
  // 末尾缺值 (原版 rc=3); 已解析部分生效, argv 不收缩 (与原版微差, 见文件头)
  let err = f.set_flags_from_command_line(&mut argv, true).unwrap_err();
  assert!(matches!(err, Error::MissingValue { .. }));
  assert_eq!(f.testing_int_flag(), 0);
  assert_eq!(argv.len(), 4);
  OK
}

#[test]
fn flags6b() -> Void {
  let mut f = setup();
  let err = f
    .set_flags_from_string("       --testing-int-flag 0      --testing_float_flag    ")
    .unwrap_err();
  assert!(matches!(err, Error::MissingValue { .. }));
  assert_eq!(f.testing_int_flag(), 0);
  OK
}

#[test]
fn assign_read_only_string_flag() -> Void {
  let mut f = setup();
  // 只读 string flag 赋值: 默认模式 (忽略矛盾) 下静默忽略 (原版 rc=0)
  f.set_flags_from_string("--print-opt-code-filter=MEH")
    .expect("只读赋值被忽略");
  // 只读 flag 值保持默认 "*"
  assert_eq!(f.print_opt_code_filter(), Some("*"));
  OK
}

#[test]
fn reject_negative_unsigned_flag_and_keep_default() -> Void {
  let mut f = setup();
  let mut argv = argv_of(&["Test", "--cpu-profiler-sampling-interval=-100"]);
  let err = f.set_flags_from_command_line(&mut argv, true).unwrap_err();
  assert!(matches!(err, Error::OutOfBounds { .. }));
  assert_eq!(f.cpu_profiler_sampling_interval(), 1000);
  OK
}

#[test]
fn flags_remove_incomplete() -> Void {
  let mut f = setup();
  let mut argv = argv_of(&["", "--testing-bool-flag", "--expose-gc-as"]);
  // 参数意外截断: 已处理参数生效 (bool flag 已设置)
  let err = f.set_flags_from_command_line(&mut argv, true).unwrap_err();
  assert!(matches!(err, Error::MissingValue { .. }));
  assert!(f.testing_bool_flag());
  // 出错时 argv 不变: 截断参数仍留在原位
  assert_eq!(argv.len(), 3);
  assert_eq!(argv[2], "--expose-gc-as");
  OK
}

#[test]
fn flags_jitless_implications() -> Void {
  let mut f = setup();
  f.set_flags_from_string("--jitless").expect("解析");
  f.enforce_implications().expect("蕴含");
  assert!(!f.turbofan());
  assert!(!f.maglev());
  assert!(!f.sparkplug());
  // wasm_lazy_compilation 仅在 V8_ENABLE_DRUMBRAKE 构建由 jitless 蕴含 (规范配置关),
  // 原版断言受该变体保护, 此处不迁移
  OK
}

#[test]
fn flags_disable_optimizing_compilers_implications() -> Void {
  let mut f = setup();
  f.set_flags_from_string("--disable-optimizing-compilers")
    .expect("解析");
  f.enforce_implications().expect("蕴含");
  assert!(!f.turbofan());
  assert!(!f.turboshaft());
  assert!(!f.maglev());
  assert!(!f.wasm_tier_up());
  assert!(!f.wasm_dynamic_tiering());
  OK
}

#[test]
fn freeze_flags() -> Void {
  let mut f = setup();
  assert_eq!(f.testing_int_flag(), 13); // 默认值
  f.set_testing_int_flag(27).expect("冻结前赋值");
  assert_eq!(f.testing_int_flag(), 27);
  f.freeze();
  assert!(f.is_frozen());
  // 冻结后 API 赋值被拒 (原版 CHECK 失败点)
  let err = f.set_testing_int_flag(41).unwrap_err();
  assert!(matches!(err, Error::Frozen));
  // 读取不受影响
  assert_eq!(f.testing_int_flag(), 27);
  OK
}

#[test]
fn unsafe_flag_implies_test_only_unsafe() -> Void {
  // --expose-trigger-failure
  {
    let mut f = setup();
    let saved = f.save();
    f.set_flags_from_string("--expose-trigger-failure")
      .expect("解析");
    f.enforce_implications().expect("蕴含");
    assert!(f.expose_trigger_failure());
    assert!(f.test_only_unsafe());
    f.restore(&saved);
  }
  // --no-wasm-bounds-checks
  {
    let mut f = setup();
    let saved = f.save();
    f.set_flags_from_string("--no-wasm-bounds-checks")
      .expect("解析");
    f.enforce_implications().expect("蕴含");
    assert!(!f.wasm_bounds_checks());
    assert!(f.test_only_unsafe());
    f.restore(&saved);
  }
  // --gc-fake-mmap=/tmp/foo (原版块尾 FindFlagByName("gc_fake_mmap")->Reset())
  {
    let mut f = setup();
    let saved = f.save();
    f.set_flags_from_string("--gc-fake-mmap=/tmp/foo")
      .expect("解析");
    f.enforce_implications().expect("蕴含");
    assert_eq!(f.gc_fake_mmap(), Some("/tmp/foo"));
    assert!(f.test_only_unsafe());
    let id = find_flag_by_name("gc_fake_mmap").unwrap();
    f.reset(id).expect("重置");
    f.restore(&saved);
  }
  // --stack-size=100: 合法值不蕴含 unsafe (DISALLOW_UNSAFE_FLAG_IF 条件式)
  {
    let mut f = setup();
    let saved = f.save();
    f.set_flags_from_string("--stack-size=100").expect("解析");
    f.enforce_implications().expect("蕴含");
    assert_eq!(f.stack_size(), 100);
    assert!(!f.test_only_unsafe());
    f.restore(&saved);
  }
  // --stack-size=40: 非法值蕴含 unsafe
  {
    let mut f = setup();
    let saved = f.save();
    f.set_flags_from_string("--stack-size=40").expect("解析");
    f.enforce_implications().expect("蕴含");
    assert_eq!(f.stack_size(), 40);
    assert!(f.test_only_unsafe());
    f.restore(&saved);
  }
  OK
}

#[test]
fn fuzzing_implies_disallow_unsafe_flags() -> Void {
  // 单设 --disallow-unsafe-flags 不蕴含 --fuzzing
  {
    let mut f = setup();
    let saved = f.save();
    f.set_flags_from_string("--disallow-unsafe-flags")
      .expect("解析");
    f.enforce_implications().expect("蕴含");
    assert!(f.disallow_unsafe_flags());
    assert!(!f.fuzzing());
    f.restore(&saved);
  }
  // --fuzzing 蕴含 disallow_unsafe_flags, 并把 unsafe flag 重置回默认 (非 DEBUG 配置)
  {
    let mut f = setup();
    let saved = f.save();
    f.set_flags_from_string(
      "--fuzzing --mock-arraybuffer-allocator --gc-fake-mmap=/tmp/x --csa-trap-on-node=stub,1 --expose-trigger-failure --stack-size=40",
    )
    .expect("解析");
    f.resolve_contradictions_when_fuzzing().expect("矛盾处理");
    f.enforce_implications().expect("蕴含");
    assert!(f.fuzzing());
    assert!(f.disallow_unsafe_flags());
    assert!(!f.mock_arraybuffer_allocator());
    assert_eq!(f.gc_fake_mmap(), Some("/tmp/__v8_gc__"));
    assert_eq!(f.csa_trap_on_node(), None);
    assert!(!f.expose_trigger_failure());
    // V8_DEFAULT_STACK_SIZE_KB (规范配置 984)
    assert_eq!(f.stack_size(), 984);
    assert!(!f.test_only_unsafe());
    f.restore(&saved);
  }
  OK
}

#[test]
fn array_destructure_bytecode_implications() -> Void {
  {
    let mut f = setup();
    let saved = f.save();
    f.set_flags_from_string("--array-destructure-bytecode")
      .expect("解析");
    f.enforce_implications().expect("蕴含");
    assert!(f.array_destructure_bytecode());
    assert!(f.turbolev());
    f.restore(&saved);
  }
  {
    let mut f = setup();
    let saved = f.save();
    f.set_flags_from_string("--fuzzing --array-destructure-bytecode --no-turbolev")
      .expect("解析");
    f.resolve_contradictions_when_fuzzing().expect("矛盾处理");
    f.enforce_implications().expect("蕴含");
    assert!(f.turbofan());
    assert!(!f.turbolev());
    assert!(!f.array_destructure_bytecode());
    f.restore(&saved);
  }
  {
    let mut f = setup();
    let saved = f.save();
    f.set_flags_from_string(
      "--fuzzing --array-destructure-bytecode --no-turbofan --stress-concurrent-inlining --no-turbolev",
    )
    .expect("解析");
    f.resolve_contradictions_when_fuzzing().expect("矛盾处理");
    f.enforce_implications().expect("蕴含");
    assert!(f.turbofan());
    assert!(!f.turbolev());
    assert!(!f.array_destructure_bytecode());
    f.restore(&saved);
  }
  OK
}

#[test]
fn correctness_fuzzer_suppressions_is_developer_only() -> Void {
  let mut f = setup();
  let saved = f.save();
  f.set_flags_from_string("--correctness-fuzzer-suppressions")
    .expect("解析");
  f.enforce_implications().expect("蕴含");
  assert!(f.correctness_fuzzer_suppressions());
  assert!(f.developer_only_features());
  f.restore(&saved);
  OK
}

// ── FlagContradictionsTest ──

#[test]
fn resolves_contradictions() -> Void {
  let mut f = setup();
  f.set_flags_from_string("--fuzzing --stress-maglev --jitless")
    .expect("解析");
  assert!(f.fuzzing());
  assert!(f.jitless());
  assert!(f.stress_maglev());
  f.resolve_contradictions_when_fuzzing().expect("矛盾处理");
  f.enforce_implications().expect("蕴含");
  assert!(f.fuzzing());
  // jitless 与 stress_maglev 矛盾: 先列者保留, jitless 被重置
  assert!(!f.jitless());
  assert!(f.stress_maglev());
  OK
}

#[test]
fn resolves_neg_contradictions() -> Void {
  {
    let mut f = setup();
    f.set_flags_from_string("--fuzzing --no-turbofan --always-osr-from-maglev")
      .expect("解析");
    assert!(f.fuzzing());
    assert!(!f.turbofan());
    assert!(f.always_osr_from_maglev());
    f.resolve_contradictions_when_fuzzing().expect("矛盾处理");
    f.enforce_implications().expect("蕴含");
    assert!(f.fuzzing());
    assert!(!f.turbofan());
    assert!(!f.always_osr_from_maglev());
    assert_eq!(f.osr_from_maglev(), 0);
  }
  {
    let mut f = setup();
    f.set_flags_from_string("--fuzzing --no-turbofan --osr-from-maglev=4")
      .expect("解析");
    assert!(f.fuzzing());
    assert!(!f.turbofan());
    assert_eq!(f.osr_from_maglev(), 4);
    f.resolve_contradictions_when_fuzzing().expect("矛盾处理");
    f.enforce_implications().expect("蕴含");
    assert!(f.fuzzing());
    assert!(!f.turbofan());
    assert_eq!(f.osr_from_maglev(), 0);
  }
  OK
}

// ── FlagHelpersTest ──

const SMALLER_VALUES: &[&str] = &["", "--a", "--a-b-c", "--a_b_c"];
const LARGER_VALUES: &[&str] = &[
  "--a-c-b",
  "--a_c_b",
  "--a_b_d",
  "--a-b-d",
  "--a_b_c_d",
  "--a-b-c-d",
];

#[test]
fn compare_different_flags() -> Void {
  for small in SMALLER_VALUES {
    for large in LARGER_VALUES {
      assert_eq!(cmp_names(small, large), Ordering::Less, "{small} < {large}");
      assert_eq!(
        cmp_names(large, small),
        Ordering::Greater,
        "{large} > {small}"
      );
    }
  }
  OK
}

#[test]
fn compare_same_flags() -> Void {
  let check = |a: &str, b: &str| {
    assert!(equal_names(a, b), "{a} == {b}");
    assert!(equal_names(b, a), "{b} == {a}");
  };
  check("", "");
  check("--a", "--a");
  check("--a-b-c", "--a_b_c");
  check("--a-b-c", "--a-b-c");
  // 不变量 (原版 V8_FUZZ_TEST 的代表性样例): 反对称 + 自反
  let all: Vec<&str> = SMALLER_VALUES
    .iter()
    .chain(LARGER_VALUES)
    .copied()
    .collect();
  for a in &all {
    for b in &all {
      assert_eq!(cmp_names(a, b), cmp_names(b, a).reverse());
      assert!(equal_names(a, a));
    }
  }
  OK
}

// ── FlagInternalsTest ──

#[test]
fn lookup_flag_by_name() -> Void {
  let f = setup();
  let id = find_flag_by_name("trace_opt").expect("下划线名");
  assert_eq!(f.name(id), "trace_opt");
  let id2 = find_flag_by_name("trace-opt").expect("中划线名");
  assert_eq!(id, id2);
  assert!(find_flag_by_name("trace?opt").is_none());
  OK
}

#[test]
fn lookup_all_flags_by_name() -> Void {
  let f = setup();
  for (i, d) in FLAGS.iter().enumerate() {
    let id = find_flag_by_name(d.name).expect("主 flag 必可查");
    assert_eq!(id as usize, i, "{} 查表错位", d.name);
    assert_eq!(f.name(id), d.name);
  }
  OK
}

#[test]
fn lookup_all_implication_flags_by_name() -> Void {
  let f = setup();
  for (i, d) in FLAGS.iter().enumerate() {
    let id = find_implication_flag(d.name).expect("精确名");
    assert_eq!(id as usize, i);
    // 蕴含条件名带后缀 ("flag < 3") 亦可命中
    let with_suffix = format!("{} < 3", d.name);
    let id2 = find_implication_flag(&with_suffix).expect("后缀名");
    assert_eq!(id2 as usize, i);
    assert_eq!(f.name(id2), d.name);
  }
  OK
}

#[test]
fn implication_order_should_not_matter() -> Void {
  // 原版 TEST(FlagInternalsTest, ...) 无 fixture SetUp — 单次 enforce 内:
  // B→A 先改值 A 并登记边; 随后 C→!B 改值 B 时沿边递归回滚 A
  // (flags.cc:1508 每次调用新建 processor, 边表以调用为界, 不跨调用存活)
  let mut f = Flags::new();
  f.set_flags_from_string("--testing-bool-flag-C")
    .expect("解析");
  f.enforce_implications().expect("蕴含");
  assert!(f.testing_bool_flag_c());
  assert!(!f.testing_bool_flag_b());
  assert!(!f.testing_bool_flag_a());
  OK
}

// ── ExperimentalFlagImplicationTest (参数化 → 逐变体) ──

/// 特性 flag 打开后 --experimental / --test-only-unsafe 仍不应被蕴含 (对应原 TEST_P 两用例)
fn check_feature_not_enabling(flag: Option<&str>) {
  let mut f = setup();
  let saved = f.save();
  if let Some(name) = flag {
    f.set_flags_from_string(name).expect("解析特性 flag");
    let underscore = name.trim_start_matches('-').replace('-', "_");
    let id = find_flag_by_name(&underscore).expect("特性 flag 存在");
    // 原版 CHECK(*flag_value): 该 bool flag 已打开
    assert_eq!(f.value(id).as_bool(), Some(true), "{name} 应已打开");
  }
  f.enforce_implications().expect("蕴含");
  assert!(!f.experimental(), "--experimental 不应被 {flag:?} 蕴含");
  assert!(
    !f.test_only_unsafe(),
    "--test-only-unsafe 不应被 {flag:?} 蕴含"
  );
  f.restore(&saved);
}

#[test]
fn experimental_not_enabled_default() -> Void {
  check_feature_not_enabling(None);
  OK
}

#[test]
fn experimental_not_enabled_future() -> Void {
  check_feature_not_enabling(Some("--future"));
  OK
}

#[test]
fn experimental_not_enabled_wasm_staging() -> Void {
  check_feature_not_enabling(Some("--wasm-staging"));
  OK
}

#[test]
fn experimental_not_enabled_harmony() -> Void {
  check_feature_not_enabling(Some("--harmony"));
  OK
}

// ── 补充: 模式 / 快照语义 / 特性 JSON ──

#[test]
fn processing_modes() -> Void {
  let mut f = setup();
  // 默认: 忽略矛盾 (历史行为)
  assert_eq!(
    f.processing_mode().unwrap(),
    FlagProcessingMode::IgnoreContradictions
  );
  assert!(!f.should_check_contradictions().unwrap());

  // exit-on-error: 开启矛盾检查; 只读 flag 显式改值报错
  f.set_flag_processing_mode("exit-on-error")
    .expect("设置模式");
  assert_eq!(
    f.processing_mode().unwrap(),
    FlagProcessingMode::ExitOnError
  );
  assert!(f.should_check_contradictions().unwrap());
  let readonly_name = FLAGS.iter().find(|d| d.readonly).unwrap().name;
  let err = f
    .set_flags_from_string(&format!("--{readonly_name}"))
    .unwrap_err();
  assert!(matches!(err, Error::ReadonlyConflict { .. }));

  // abort-on-error + fuzzing: 回落忽略矛盾 (历史行为)
  let mut fz = setup();
  fz.set_flag_processing_mode("abort-on-error").expect("模式");
  assert_eq!(
    fz.processing_mode().unwrap(),
    FlagProcessingMode::AbortOnError
  );
  fz.set_fuzzing(true).expect("fuzzing");
  assert_eq!(
    fz.processing_mode().unwrap(),
    FlagProcessingMode::IgnoreContradictions
  );

  // 非法模式值
  let mut bad = setup();
  bad
    .set_flag_processing_mode("nonsense")
    .expect("赋值本身合法");
  assert!(matches!(
    bad.processing_mode(),
    Err(Error::InvalidProcessingMode(_))
  ));
  OK
}

#[test]
fn flag_ops_save_restore() -> Void {
  let mut f = setup();
  // 赋值来源与蕴含来源追踪
  let exp = find_flag_by_name("experimental").unwrap();
  assert_eq!(f.set_by(exp), SetBy::Default);
  f.set_experimental(true).expect("赋值");
  assert_eq!(f.set_by(exp), SetBy::CommandLine);
  f.reset(exp).expect("重置");
  assert_eq!(f.set_by(exp), SetBy::Default);

  // 蕴含来源记录 (前提名 = 蕴含定义中的 flag 名)
  f.set_array_destructure_bytecode(true).expect("赋值");
  f.enforce_implications().expect("蕴含");
  assert!(f.experimental());
  assert_eq!(f.set_by(exp), SetBy::Implication);
  assert_eq!(f.implied_by(exp), Some("array_destructure_bytecode"));

  // 快照恢复 (SaveFlags 语义: 只回值)
  let saved = f.save();
  f.set_maglev_max_small_graph_size(77).expect("赋值");
  assert_eq!(f.maglev_max_small_graph_size(), 77);
  f.restore(&saved);
  assert_eq!(f.maglev_max_small_graph_size(), 10);
  OK
}

#[test]
fn feature_flags_json_shape() -> Void {
  let json = feature_flags_json();
  let v: Value = sonic_rs::from_str(&json).expect("特性 JSON 解析");
  assert!(v["js"].is_object());
  assert!(v["harmony"].is_object());
  assert!(v["wasm"].is_object());
  for group in ["js", "harmony", "wasm"] {
    assert!(v[group]["in-progress"].is_array());
    assert!(v[group]["staged"].is_array());
    assert!(v[group]["shipping"].is_array());
  }
  // 已知归属: harmony_temporal (SHIPPED) 在 harmony.shipping; wasm_stringref (PRE_STAGED) 在 wasm.in-progress
  let harmony_shipping: Vec<String> =
    sonic_rs::from_value(&v["harmony"]["shipping"]).expect("数组");
  assert!(harmony_shipping.contains(&"harmony_temporal".to_string()));
  let wasm_inprogress: Vec<String> = sonic_rs::from_value(&v["wasm"]["in-progress"]).expect("数组");
  assert!(wasm_inprogress.contains(&"wasm_stringref".to_string()));
  info!("feature_flags_json:\n{json}");
  OK
}

// ── round2 对抗审查回归 ──

/// 蕴含未改值时不得登记重置边 (原版 implied_by_map_ 仅在改值时 push;
/// 旧实现按 implied_by 标签重置, 会把"等值蕴含"的 flag 误重置)
#[test]
fn implication_no_change_registers_no_reset_edge() -> Void {
  // 不用 setup(): 预蕴含会让 B→A 真实改值 A (登记边), 需从全新实例出发
  let mut f = Flags::new();
  // A=true (cmdline, 先于任何蕴含), B=true (=默认): 首轮 B→A 触发但等值, 无改值
  f.set_flags_from_string("--testing-bool-flag-A --testing-bool-flag-B")
    .expect("解析");
  f.enforce_implications().expect("蕴含");
  assert!(f.testing_bool_flag_a());
  assert!(f.testing_bool_flag_b());
  // C→!B 改值: B 的重置边为空, A 不得被连带重置 (旧实现此处 A 被误重置为 false)
  f.set_flags_from_string("--testing-bool-flag-C")
    .expect("解析");
  f.enforce_implications().expect("蕴含");
  assert!(f.testing_bool_flag_c());
  assert!(!f.testing_bool_flag_b());
  assert!(f.testing_bool_flag_a(), "等值蕴含不得登记重置边");
  OK
}

/// wasm 代码空间上限 > 默认 128 时钳回默认 (矛盾处理前置块)
#[test]
fn resolve_wasm_code_space_clamp() -> Void {
  let mut f = setup();
  f.set_flags_from_string("--fuzzing --wasm-max-code-space-size-mb=999")
    .expect("解析");
  assert_eq!(f.wasm_max_code_space_size_mb(), 999);
  f.resolve_contradictions_when_fuzzing().expect("矛盾处理");
  assert_eq!(f.wasm_max_code_space_size_mb(), 128);
  // 合法值不钳制
  let mut f2 = setup();
  f2.set_flags_from_string("--fuzzing --wasm-max-code-space-size-mb=64")
    .expect("解析");
  f2.resolve_contradictions_when_fuzzing().expect("矛盾处理");
  assert_eq!(f2.wasm_max_code_space_size_mb(), 64);
  OK
}

/// 带 '=' 的参数 flag 名超 1KB 解析缓冲 → NameTooLong (原版 SplitArgument FlagError;
/// 无 '=' 的超长名不走缓冲, 仍是 Unrecognized)
#[test]
fn split_argument_name_too_long() -> Void {
  let mut f = setup();
  let long = format!("--{}=5", "x".repeat(1024));
  let err = f.set_flags_from_string(&long).unwrap_err();
  assert!(matches!(err, Error::NameTooLong { .. }));
  // 无 '=': 原版不做缓冲拷贝, 按未识别处理
  let bare = format!("--{}", "x".repeat(1024));
  let err = f.set_flags_from_string(&bare).unwrap_err();
  assert!(matches!(err, Error::Unrecognized { .. }));
  OK
}
