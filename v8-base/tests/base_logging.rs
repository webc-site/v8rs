//! v8-base/src/logging.rs 集成测试 — 1:1 对应 logging-unittest.cc 中可转写部分
//!
//! 原版含大量 `ASSERT_DEATH_IF_SUPPORTED(...)` 用例 (`FatalKills` / `DcheckIsOnlyFatalInDebug`
//! / `V8_DcheckCanBeOverridden` / `OutputEnumValues` / `OutputSingleCharEnum` /
//! `OutputLongValues` / `CheckChars` / `Collections` / `CompareWithDifferentSignedness`
//! 后半段等) — 按术语表红线一律**跳过**, 原因登记.
//!
//! 转写范围:
//! - `LoggingTest.CheckEQImpl` — float 相等比较的 `check_op_eq` 无失败
//! - `LoggingTest.CompareSignedMismatch` — Rust 无跨符号 ==/PartialOrd, 逐条转为同型比较
//! - `LoggingTest.CompareEnumTypes` — 保留 enum 分支的 `check_op_eq/ne/lt` 结构
//! - 匿名 `PrettyPrintChar` → `pretty_print_char`, 覆盖 10 类特殊字符 + 可打印 + 非打印
//! - `MakeCheckOpString` 短/长消息两分支 (原版 `kMaxInlineLength = 50`)
//! - `SetPrintStackTrace`/`SetDcheckFunction` 钩子安装/生效/恢复

use std::{
  fmt,
  sync::atomic::{AtomicU32, Ordering},
};

use aok::{OK, Void};
use v8_base::logging::{
  CmpOp, check_op_eq, check_op_ne, check_op_partial_ord, make_check_op_string, pretty_print_char,
  print_stack_trace_if_available, set_dcheck_function, set_print_stack_trace, v8_dcheck,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 `LoggingTest.CheckEQImpl`: 0.0/-0.0 相互等价的 IEEE754 语义.
#[test]
fn logging_test_check_eq_impl() -> Void {
  assert!(check_op_eq(&0.0f64, &0.0f64, "0.0 == 0.0").is_none());
  assert!(check_op_eq(&0.0f64, &-0.0f64, "0.0 == -0.0").is_none());
  assert!(check_op_eq(&-0.0f64, &0.0f64, "-0.0 == 0.0").is_none());
  assert!(check_op_eq(&-0.0f64, &-0.0f64, "-0.0 == -0.0").is_none());
  OK
}

/// 原版 `LoggingTest.CompareSignedMismatch`: C++ 侧跨 signed/unsigned 会做隐式转换.
///
/// Rust 无隐式转换 (`i32: !PartialEq<u32>`), 因此等价语义在此显式统一到 i64/u64 后逐条断言,
/// 保持 check_op_eq/ne/partial_ord 的成败分类与原用例一致.
#[test]
fn logging_test_compare_signed_mismatch() -> Void {
  // 14 == 14u → true; 14 == 15u → false
  assert!(check_op_eq(&(14i32 as i64), &(14u32 as i64), "eq14").is_none());
  assert!(check_op_eq(&(14i32 as i64), &(15u32 as i64), "eq15").is_some());

  // -1 vs u32::MAX (原版视为 -1): Rust 显式 i64 转换, 两者不相等
  assert!(check_op_ne(&(-1i32 as i64), &(u32::MAX as i64), "neg1_vs_max").is_none());

  // 依原版语义显式转 u64 复现 (Rust `-1i32 as u64` = u64::MAX):
  assert!(check_op_partial_ord(&(-1i32 as u64), &0u64, "lt_neg1_u_max", CmpOp::Lt).is_some());
  assert!(
    check_op_partial_ord(&(-1i32 as u64), &(u32::MAX as u64), "lt_int_max", CmpOp::Lt).is_some()
  );
  assert!(check_op_partial_ord(&(-1i32 as u64), &0u64, "le", CmpOp::Le).is_some());
  assert!(check_op_partial_ord(&(55i32 as u64), &(55u32 as u64), "le55", CmpOp::Le).is_none());
  assert!(
    check_op_partial_ord(&(55i32 as u64), &(0x7FFF_FFF0u32 as u64), "lt", CmpOp::Lt).is_none()
  );
  assert!(
    check_op_partial_ord(&(55i32 as u64), &(0x7FFF_FFF0u32 as u64), "le", CmpOp::Le).is_none()
  );
  assert!(
    check_op_partial_ord(&(0x7FFF_FFF0u32 as u64), &(55i32 as u64), "ge", CmpOp::Ge).is_none()
  );
  assert!(
    check_op_partial_ord(&(0x7FFF_FFF0u32 as u64), &(55i32 as u64), "gt", CmpOp::Gt).is_none()
  );
  assert!(
    check_op_partial_ord(
      &(u32::MAX as u64),
      &(-1i32 as u64),
      "gt_u_max_vs_neg",
      CmpOp::Gt
    )
    .is_some()
  ); // C++: u32::MAX(=4294967295) 与 -1(promote → 4294967295) 相等, 非严格大于
  assert!(check_op_partial_ord(&0u64, &(-1i32 as u64), "ge_u0_vs_neg1", CmpOp::Ge).is_some());
  assert!(check_op_partial_ord(&(-1i8 as u64), &0u64, "lt_i8_u32_promote", CmpOp::Lt).is_some());
  assert!(check_op_partial_ord(&0x7F01_0101_0101_0101u64, &0u64, "gt_u64", CmpOp::Gt).is_none());
  // 原版最后一条: 0xFF01_0101_0101_0101 (u64 bit63=1 → as i64 负值) <= 13 (u8) 为 true.
  assert!(
    check_op_partial_ord(
      &(0xFF01_0101_0101_0101u64 as i64),
      &(13u8 as i64),
      "le_i64_u8",
      CmpOp::Le
    )
    .is_none()
  );
  OK
}

/// 原版 `LoggingTest.CompareEnumTypes` — Rust enum 需 `PartialEq/PartialOrd + Display`.
#[test]
fn logging_test_compare_enum_types() -> Void {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  #[repr(u8)]
  enum E {
    One = 0,
    Two = 1,
  }
  impl fmt::Display for E {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{:?}", *self)
    }
  }
  assert!(check_op_eq(&E::One, &E::One, "eq").is_none());
  assert!(check_op_ne(&E::One, &E::Two, "ne").is_none());
  assert!(check_op_partial_ord(&E::One, &E::Two, "lt", CmpOp::Lt).is_none());
  OK
}

/// 原版 `PrettyPrintChar` (定义于 logging.cc 匿名 ns) 的 10 类字面串化 + 可打印/非打印兜底.
#[test]
fn logging_test_pretty_print_char() -> Void {
  // 特殊控制字符按源码字面串 (即 #ch 展开的 4 字符)
  assert_eq!(pretty_print_char(0), "'\\0'");
  assert_eq!(pretty_print_char(7), "'\\a'");
  assert_eq!(pretty_print_char(8), "'\\b'");
  assert_eq!(pretty_print_char(9), "'\\t'");
  assert_eq!(pretty_print_char(10), "'\\n'");
  assert_eq!(pretty_print_char(11), "'\\v'");
  assert_eq!(pretty_print_char(12), "'\\f'");
  assert_eq!(pretty_print_char(13), "'\\r'");
  assert_eq!(pretty_print_char(39), "'\\''"); // '\''
  assert_eq!(pretty_print_char(92), "'\\\\'"); // '\\'

  // 原版 `CheckChars`: CHECK_EQ('a','b') → "'97'" (C++ 侧 ch 是 int, 打印数字)
  assert_eq!(pretty_print_char(97), "'97'");
  assert_eq!(pretty_print_char(98), "'98'");

  // 非可打印 (< 0x20 且不在特殊表, >= 0x7f): \x 十六进制
  assert_eq!(pretty_print_char(1), "\\x1");
  assert_eq!(pretty_print_char(0x7f), "\\x7f");
  OK
}

/// 原版 `MakeCheckOpString`: 短消息一行, 长消息三行 (阈值 `kMaxInlineLength = 50`).
#[test]
fn logging_test_make_check_op_string() -> Void {
  let short = make_check_op_string(&"abc", &"def", "Check failed: x == y");
  assert_eq!(short, "Check failed: x == y (abc vs. def)");

  // 长消息: max(lhs.len, rhs.len) > 50
  let long_lhs = "a".repeat(51);
  let long = make_check_op_string(&long_lhs, &"def", "Check failed: x == y");
  assert!(long.contains("\n   "));
  assert!(long.contains("\n vs.\n   "));
  OK
}

static DCOUNT: AtomicU32 = AtomicU32::new(0);
fn counting_dcheck(_file: &str, _line: i32, _msg: &str) {
  DCOUNT.fetch_add(1, Ordering::SeqCst);
}

/// 原版 `SetDcheckFunction` + `LogFunctionPointers` 分支 — 覆写后 V8_Dcheck 调用计数, 恢复默认.
///
/// 注意: 默认 dcheck handler 会走 `v8_fatal` 触发 abort, 因此本用例**只在覆写下运行**;
/// 原版 `V8_DcheckCanBeOverridden` 是 DEATH 测试, 依红线跳过 (术语表 gtest→Rust 表).
#[test]
fn logging_test_set_dcheck_function() -> Void {
  DCOUNT.store(0, Ordering::SeqCst);
  set_dcheck_function(Some(counting_dcheck));
  v8_dcheck("benign message 1");
  v8_dcheck("benign message 2");
  assert_eq!(DCOUNT.load(Ordering::SeqCst), 2);

  // 复原: 传 None 回到默认 handler; 本用例之后**不再触发** v8_dcheck, 避免 abort.
  set_dcheck_function(None);
  OK
}

static PCOUNT: AtomicU32 = AtomicU32::new(0);
fn counting_print() {
  PCOUNT.fetch_add(1, Ordering::SeqCst);
}

/// 原版 `SetPrintStackTrace`/`PrintStackTraceIfAvailable`.
#[test]
fn logging_test_set_print_stack_trace() -> Void {
  PCOUNT.store(0, Ordering::SeqCst);
  // 未注册时: 不 panic, 静默
  print_stack_trace_if_available();
  assert_eq!(PCOUNT.load(Ordering::SeqCst), 0);

  set_print_stack_trace(Some(counting_print));
  print_stack_trace_if_available();
  print_stack_trace_if_available();
  assert_eq!(PCOUNT.load(Ordering::SeqCst), 2);

  // 显式恢复 None
  set_print_stack_trace(None);
  print_stack_trace_if_available();
  assert_eq!(PCOUNT.load(Ordering::SeqCst), 2);
  OK
}

/// 术语表红线: EXPECT_DEATH_IF_SUPPORTED → 跳过, 记录理由; 本用例只做登记不断言.
#[test]
fn logging_death_tests_skipped() -> Void {
  // 覆盖原版: FatalKills / DcheckIsOnlyFatalInDebug / V8_DcheckCanBeOverridden /
  //          OutputEnumValues / OutputSingleCharEnum / OutputLongValues /
  //          CheckChars / Collections / CollectionsOfUnprintable /
  //          CompareWithDifferentSignedness 后半段 / OutputEnumWithOutputOperator
  // 理由: 依赖 ASSERT_DEATH_IF_SUPPORTED 进程终止路径, Rust 集成测试不便直接 abort;
  //      v8-logging crate 桥接 fatal 钩子后, 由该 crate 提供 death harness.
  OK
}
