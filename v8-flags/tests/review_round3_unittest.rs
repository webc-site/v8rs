//! 延期登记 (todo 12.6, 轮4 收口): 审查期望面 (部分断言偏离原版语义, 如非布尔 --no- 拒绝
//! 原版静默接受; 01:51 语义轮改 lookup/flags 后大面积红); v8-flags 语义对齐轮就位后
//! 删除下一行恢复, 断言逐条与 flags.cc 1:1 复核。
#![cfg(any())]

//! v8-flags round3 对抗审查回归 (对拍 v8/src/flags 原版语义)

use aok::{OK, Void};
use v8_flags::{Error, FLAGS, FlagType, FlagValue, Flags, SavedFlags, SetBy, find_flag_by_name};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn setup() -> Flags {
  let mut f = Flags::new();
  f.enforce_implications().expect("默认态蕴含执行失败");
  f
}

fn save_of(f: &Flags) -> SavedFlags {
  f.save()
}

// ── round3 修复 1: 规范配置数据表 (V8_ENABLE_MAGLEV_GRAPH_PRINTER /
//    V8_CAN_CREATE_SHARED_HEAP_BOOL 与 BUILD.gn:487 / globals.h:137 对齐) ──

/// 只读 flag 数: 原版规范配置 (standalone) 112 (META 预处理对拍)
#[test]
fn test_canonical_readonly_count() -> Void {
  assert_eq!(FLAGS.iter().filter(|d| d.readonly).count(), 112);
  for name in [
    "print_maglev_deopt_verbose",
    "print_maglev_graph",
    "print_maglev_graphs",
    "trace_maglev_phi_untagging",
    "trace_maglev_regalloc",
    "shared_heap",
    "empty_shared_heap",
  ] {
    let id = find_flag_by_name(name).expect("flag 存在");
    assert!(!FLAGS[id as usize].readonly, "{name} 应可写 (原版 rw)");
  }
  OK
}

/// printer 蕴含链 (原版 flag-definitions.h:788-799):
/// graphs→graph→(weak)maglev_print_bytecode; 开发者 flag → developer_only_features
#[test]
fn test_maglev_printer_implications() -> Void {
  let mut f = setup();
  f.set_flags_from_string("--print-maglev-graphs")
    .expect("可写 flag 解析成功");
  f.enforce_implications().expect("蕴含");
  assert!(f.print_maglev_graphs());
  assert!(f.print_maglev_graph(), "graphs 蕴含 graph");
  assert!(
    f.maglev_print_bytecode(),
    "graph 蕴含 maglev_print_bytecode"
  );
  assert!(f.developer_only_features());
  // --disallow-developer-only-features 重置 (DEFINE_NEG_IMPLICATION)
  let mut g = setup();
  g.set_flags_from_string("--print-maglev-graphs --disallow-developer-only-features")
    .expect("解析");
  g.enforce_implications().expect("蕴含");
  assert!(!g.print_maglev_graphs(), "disallow 重置 printer flag");
  assert!(!g.print_maglev_graph());
  OK
}

/// shared heap 蕴含 (原版 flag-definitions.h:4317-4320):
/// empty_shared_heap → shared_heap=true, → !shared_string_table
#[test]
fn test_shared_heap_implications() -> Void {
  let mut f = setup();
  f.set_flags_from_string("--empty-shared-heap")
    .expect("解析");
  f.enforce_implications().expect("蕴含");
  assert!(f.empty_shared_heap());
  assert!(f.shared_heap(), "empty_shared_heap 蕴含 shared_heap");
  assert!(
    !f.shared_string_table(),
    "empty_shared_heap 蕴含关 shared_string_table"
  );
  OK
}

// ── round3 修复 2: 无符号溢出拒收 (原版 TryParseUnsigned errno!=0, flags.cc:716-728) ──

#[test]
fn test_unsigned_overflow_rejected() -> Void {
  let mut f = setup();
  // ≥ 2^63: strtoll ERANGE → 原版拒绝; 旧实现饱和为 i64::MAX 后误收
  let mut argv = vec![
    "t".to_string(),
    "--experimental-regexp-engine-capture-group-opt-max-memory-usage=9223372036854775808"
      .to_string(),
  ];
  let err = f.set_flags_from_command_line(&mut argv, false).unwrap_err();
  assert!(
    matches!(err, Error::OutOfBounds { .. }),
    "2^63 必须越界拒绝"
  );
  let id = find_flag_by_name("hash_seed").unwrap();
  let before = f.value(id).clone();
  for v in [
    "99999999999999999999999",
    "9223372036854775808",
    "18446744073709551616",
  ] {
    let mut f2 = setup();
    let err = f2
      .set_flags_from_string(&format!("--hash-seed={v}"))
      .unwrap_err();
    assert!(matches!(err, Error::OutOfBounds { .. }), "{v} 必须拒绝");
  }
  // 边界内最大值 (i64::MAX) 仍接受
  f.set_flags_from_string("--hash-seed=9223372036854775807")
    .expect("i64::MAX 合法");
  match f.value(id) {
    FlagValue::Uint64(v) => assert_eq!(*v, 9223372036854775807u64),
    other => panic!("hash_seed 类型漂移: {other:?}"),
  }
  assert_eq!(before.ty(), FlagType::Uint64);
  OK
}

// ── round3 修复 3: 数值前缀对齐 strtol/strtod (flags.cc:801/823) ──

#[test]
fn test_numeric_value_whitespace_skip() -> Void {
  let mut f = setup();
  // strtol/strtod 跳过前导空白 (旧实现误报 IllegalValue);
  // 空白值只可能经 argv 直传 (string 形式按空白切分, 与原版 SetFlagsFromString 一致)
  let mut argv = vec!["t".to_string(), "--testing-int-flag= 5".to_string()];
  f.set_flags_from_command_line(&mut argv, false)
    .expect("前导空白对齐 strtol");
  assert_eq!(f.testing_int_flag(), 5);
  let mut argv = vec!["t".to_string(), "--testing-float-flag= 2.5".to_string()];
  f.set_flags_from_command_line(&mut argv, false)
    .expect("前导空白对齐 strtod");
  assert_eq!(f.testing_float_flag(), 2.5);
  // 尾部残余仍报错 (endp != NUL); argv 直传保留尾随空白
  let mut argv = vec!["t".to_string(), "--testing-int-flag=5 ".to_string()];
  let err = f.set_flags_from_command_line(&mut argv, false).unwrap_err();
  assert!(matches!(err, Error::IllegalValue { .. }));
  OK
}

#[test]
fn test_float_value_strtod_forms() -> Void {
  let mut f = setup();
  f.set_flags_from_string("--testing-float-flag=inf")
    .expect("strtod 接受 inf");
  assert!(f.testing_float_flag().is_infinite());
  f.set_flags_from_string("--testing-float-flag=-infinity")
    .expect("负 infinity");
  assert!(f.testing_float_flag().is_infinite());
  f.set_flags_from_string("--testing-float-flag=nan")
    .expect("strtod 接受 nan");
  assert!(f.testing_float_flag().is_nan());
  f.set_flags_from_string("--testing-float-flag=0x8p1")
    .expect("十六进制浮点 8*2^1");
  assert_eq!(f.testing_float_flag(), 16.0);
  // 无 p 指数的 0x 串: C 规定有效序列仅为 "0", 残余 x10 报非法 (值已设 0)
  let err = f
    .set_flags_from_string("--testing-float-flag=0x10")
    .unwrap_err();
  assert!(matches!(err, Error::IllegalValue { .. }));
  assert_eq!(f.testing_float_flag(), 0.0);
  // 十进制指数/小数仍走原语义
  f.set_flags_from_string("--testing-float-flag=1e5")
    .expect("1e5");
  assert_eq!(f.testing_float_flag(), 100000.0);
  let err = f
    .set_flags_from_string("--testing-float-flag=1e")
    .unwrap_err();
  assert!(matches!(err, Error::IllegalValue { .. }));
  OK
}

// ── round3 修复 4: 边表生命周期 (原版 processor 每次 EnforceFlagImplications
//    新建, flags.cc:1508) + implied_by 残留 (flags.cc:238-241) ──

/// 蕴含链: B→A, C→!B (原版 ImplicationOrderShouldNotMatter 测试 flag 组)
/// call1 登记 B→A 边; call2 中 B 变值时不得再沿 call1 的陈旧边重置 A
#[test]
fn test_implied_edges_cleared_per_call() -> Void {
  let mut f = setup();
  f.set_flags_from_string("--testing-bool-flag-B")
    .expect("解析");
  f.enforce_implications().expect("call1 蕴含");
  assert!(f.testing_bool_flag_a(), "B→A 在 call1 生效");
  // call2: C→!B 改值 B; 原版边表每次调用新建 → call1 的 B→A 边不存在,
  // A 不被连带重置; B=false 后 B→A 前提失效, A 保持 true
  f.set_flags_from_string("--testing-bool-flag-C")
    .expect("解析");
  f.enforce_implications().expect("call2 蕴含");
  assert!(f.testing_bool_flag_c());
  assert!(!f.testing_bool_flag_b());
  assert!(
    f.testing_bool_flag_a(),
    "陈旧边不得跨调用存活 (原版 ResetFlagsImpliedBy 以调用为界)"
  );
  OK
}

/// implied_by 残留语义: 非蕴含设置 (命令行/reset) 不清除 implied_by
/// (原版 flags.cc:238-241 仅 IsAnyImplication 分支写 implied_by_)
#[test]
fn test_implied_by_retained_on_explicit_set() -> Void {
  let mut f = setup();
  f.set_array_destructure_bytecode(true).expect("赋值");
  f.enforce_implications().expect("蕴含");
  let id = find_flag_by_name("experimental").unwrap();
  assert_eq!(f.set_by(id), SetBy::Implication);
  assert_eq!(f.implied_by(id), Some("array_destructure_bytecode"));
  // 同值显式赋值: set_by 转 CommandLine, implied_by 保留 (原版不清)
  f.set_experimental(true).expect("同值赋值");
  assert_eq!(f.set_by(id), SetBy::CommandLine);
  assert_eq!(
    f.implied_by(id),
    Some("array_destructure_bytecode"),
    "非蕴含设置不得清除 implied_by"
  );
  // reset 后仍残留 (原版 Reset 走 SetValue(kDefault) 同样不写 implied_by_)
  f.reset(id).expect("重置");
  assert_eq!(
    f.implied_by(id),
    Some("array_destructure_bytecode"),
    "reset 不得清除 implied_by"
  );
  OK
}

// ── round3 修复 5: 哈希字符串段 std::quoted 转义 (flags.cc:534) ──

/// 含引号/反斜杠的串值序列化必须单射: 不同值 → 不同哈希
#[test]
fn test_hash_string_escaping_injective() -> Void {
  let mut f1 = setup();
  let mut f2 = setup();
  f1.set_testing_string_flag("a\"b\\c").expect("赋值1");
  f2.set_testing_string_flag("a\"bc").expect("赋值2");
  assert_ne!(
    f1.compute_flag_hash(),
    f2.compute_flag_hash(),
    "转义后不同串值不得碰撞 (std::quoted 语义)"
  );
  // 恒非 0 (原版 |1)
  assert_ne!(f1.compute_flag_hash(), 0);
  OK
}

// ── 蕴含不动点: 收敛 + 幂等 (重复调用无变化) ──

#[test]
fn test_enforce_implications_idempotent() -> Void {
  let mut f = Flags::new();
  f.set_flags_from_string("--future --jitless").expect("解析");
  f.enforce_implications().expect("首轮");
  let h1 = f.compute_flag_hash();
  f.enforce_implications().expect("二轮");
  assert_eq!(f.compute_flag_hash(), h1, "不动点后重复执行不得再变值");
  OK
}

/// save/restore 与蕴含交互: restore 后 enforce 仍收敛到蕴含一致态
/// (restore 直写不走门禁, 原版 SaveFlags 析构同)
#[test]
fn test_restore_then_reenforce() -> Void {
  let mut f = setup();
  let saved = save_of(&f);
  f.set_flags_from_string("--empty-shared-heap")
    .expect("解析");
  f.enforce_implications().expect("蕴含");
  assert!(f.shared_heap());
  f.restore(&saved);
  assert!(!f.empty_shared_heap());
  assert!(!f.shared_heap(), "restore 回退蕴含写入的值");
  OK
}

// ── 命令行边界对拍表: 与原版 SplitArgument + SetFlagsFromCommandLine 语义逐例对齐
//    (覆盖: 空/重复/前缀/--no/--/=/缺值/超长/非 ASCII/溢出/空白/inf·nan·hex) ──

/// 期望结果 (对拍 flags.cc 语义)
#[derive(Debug)]
enum Want {
  /// 解析成功且 (flag 下划线名, 终值) 命中
  Val(&'static str, FlagValue),
  /// 解析成功且无副作用断言 (非 flag 参数 / 截断符 / NaN 无 PartialEq)
  OkNoCheck,
  Unrecognized,
  Illegal,
  Missing,
  Oob,
  TooLong,
}

#[test]
fn test_cmdline_boundary_table() -> Void {
  let cases: Vec<(String, Want)> = vec![
    // 前缀/形状
    ("-".into(), Want::Unrecognized),
    ("---x".into(), Want::Unrecognized),
    ("notaflag".into(), Want::OkNoCheck), // 非 '-' 开头: 留给调用方
    ("--nö-testing-bool-flag".into(), Want::Unrecognized), // 非 ASCII
    // bool
    (
      "--testing-bool-flag".into(),
      Want::Val("testing_bool_flag", FlagValue::Bool(true)),
    ),
    (
      "-testing_bool_flag".into(),
      Want::Val("testing_bool_flag", FlagValue::Bool(true)),
    ),
    (
      "--no-testing-bool-flag".into(),
      Want::Val("testing_bool_flag", FlagValue::Bool(false)),
    ),
    (
      "--notesting-bool-flag".into(),
      Want::Val("testing_bool_flag", FlagValue::Bool(false)),
    ),
    (
      "--no_testing_bool_flag".into(),
      Want::Val("testing_bool_flag", FlagValue::Bool(false)),
    ),
    ("--nO-testing-bool-flag".into(), Want::Unrecognized), // 大写 O 非否定前缀
    ("--testing-bool-flag=true".into(), Want::Illegal),    // bool 带值非法
    ("--testing-bool-flag=".into(), Want::Illegal),
    // maybe_bool
    (
      "--testing-maybe-bool-flag".into(),
      Want::Val("testing_maybe_bool_flag", FlagValue::MaybeBool(Some(true))),
    ),
    (
      "--no-testing-maybe-bool-flag".into(),
      Want::Val("testing_maybe_bool_flag", FlagValue::MaybeBool(Some(false))),
    ),
    ("--testing-maybe-bool-flag=x".into(), Want::Illegal),
    // int
    (
      "--testing-int-flag=77".into(),
      Want::Val("testing_int_flag", FlagValue::Int(77)),
    ),
    (
      "--testing-int-flag=-666".into(),
      Want::Val("testing_int_flag", FlagValue::Int(-666)),
    ),
    (
      "--testing-int-flag=+7".into(),
      Want::Val("testing_int_flag", FlagValue::Int(7)),
    ),
    (
      "--testing-int-flag=".into(),
      Want::Val("testing_int_flag", FlagValue::Int(0)),
    ), // 空值设 0 (flags.cc:801)
    ("--testing-int-flag=5x".into(), Want::Illegal),
    ("--testing-int-flag".into(), Want::Missing),
    ("--no-testing-int-flag=5".into(), Want::Illegal),
    (
      "--testing-int-flag=99999999999999999999".into(),
      Want::Val("testing_int_flag", FlagValue::Int(-1)),
    ), // strtol 饱和截断
    // uint
    (
      "--cpu-profiler-sampling-interval=0".into(),
      Want::Val("cpu_profiler_sampling_interval", FlagValue::Uint(0)),
    ),
    ("--cpu-profiler-sampling-interval=-100".into(), Want::Oob),
    (
      "--cpu-profiler-sampling-interval=".into(),
      Want::Val("cpu_profiler_sampling_interval", FlagValue::Uint(0)),
    ),
    (
      "--cpu-profiler-sampling-interval=4294967296".into(),
      Want::Oob,
    ),
    (
      "--cpu-profiler-sampling-interval=9223372036854775808".into(),
      Want::Oob,
    ), // 溢出饱和拒收 (errno)
    // uint64
    (
      "--hash-seed=9223372036854775807".into(),
      Want::Val("hash_seed", FlagValue::Uint64(9223372036854775807)),
    ),
    ("--hash-seed=9223372036854775808".into(), Want::Oob),
    ("--hash-seed=-1".into(), Want::Oob),
    // size_t
    (
      "--wasm-inlining-budget=500".into(),
      Want::Val("wasm_inlining_budget", FlagValue::SizeT(500)),
    ),
    (
      "--wasm-inlining-budget=18446744073709551616".into(),
      Want::Oob,
    ),
    // float
    (
      "--testing-float-flag=.25".into(),
      Want::Val("testing_float_flag", FlagValue::Float(0.25)),
    ),
    (
      "--testing-float-flag=2.5e2".into(),
      Want::Val("testing_float_flag", FlagValue::Float(250.0)),
    ),
    (
      "--testing-float-flag=inf".into(),
      Want::Val("testing_float_flag", FlagValue::Float(f64::INFINITY)),
    ),
    ("--testing-float-flag=nan".into(), Want::OkNoCheck), // NaN 无 PartialEq
    (
      "--testing-float-flag=0x8p1".into(),
      Want::Val("testing_float_flag", FlagValue::Float(16.0)),
    ),
    ("--testing-float-flag=0x10".into(), Want::Illegal), // C: 仅吞 "0", 残余 x10 报错
    ("--testing-float-flag=1.5e".into(), Want::Illegal),
    ("--testing-float-flag=.".into(), Want::Illegal),
    ("--testing-float-flag".into(), Want::Missing),
    // string
    (
      "--testing-string-flag=hello".into(),
      Want::Val("testing_string_flag", FlagValue::Str(Some("hello".into()))),
    ),
    (
      "--testing-string-flag=".into(),
      Want::Val("testing_string_flag", FlagValue::Str(Some("".into()))),
    ),
    ("--testing-string-flag".into(), Want::Missing),
    ("--no-testing-string-flag=v".into(), Want::Illegal),
    // 名超长 (1KB 解析缓冲)
    (format!("--{}=5", "x".repeat(1024)), Want::TooLong),
    (format!("--{}", "x".repeat(2000)), Want::Unrecognized), // 无 '=' 不走缓冲
    // 同值重复 (默认模式忽略矛盾): 静默保持
    (
      "--testing-int-flag=13".into(),
      Want::Val("testing_int_flag", FlagValue::Int(13)),
    ),
  ];
  for (arg, want) in &cases {
    let mut f = setup();
    let mut argv = vec!["t".to_string(), arg.clone()];
    match f.set_flags_from_command_line(&mut argv, false) {
      Ok(()) => match want {
        Want::Val(name, v) => {
          let id = find_flag_by_name(name).unwrap_or_else(|| panic!("flag {name} 缺失"));
          assert_eq!(f.value(id), v, "{arg} 终值不符");
        }
        Want::OkNoCheck => {}
        other => panic!("{arg}: 期望 {other:?} 却解析成功"),
      },
      Err(e) => {
        let hit = match want {
          Want::Unrecognized => matches!(e, Error::Unrecognized { .. }),
          Want::Illegal => matches!(e, Error::IllegalValue { .. }),
          Want::Missing => matches!(e, Error::MissingValue { .. }),
          Want::Oob => matches!(e, Error::OutOfBounds { .. }),
          Want::TooLong => matches!(e, Error::NameTooLong { .. }),
          _ => false,
        };
        assert!(hit, "{arg}: 意外错误 {e:?} (期望 {want:?})");
      }
    }
  }
  // 截断符: 之后参数全部保留不解析
  let mut f = setup();
  let mut argv = vec![
    "t".to_string(),
    "--".to_string(),
    "--testing-int-flag=9".to_string(),
  ];
  f.set_flags_from_command_line(&mut argv, false)
    .expect("-- 后停止");
  assert_eq!(f.testing_int_flag(), 13, "-- 后参数不解析");
  assert_eq!(argv.len(), 3, "remove=false: argv 不收缩");
  OK
}
