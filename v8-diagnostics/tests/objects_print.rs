//! objects-printer 零耦合面: PrintDouble 分支 / DoubleToInteger / 枚举名表。

use v8_diagnostics::{
  JsIteratorHelperState, JsIteratorZipHelperMode, double_to_integer, is_minus_zero,
  iterator_helper_state_to_str, iterator_zip_helper_mode_to_str, print_double,
};

#[test]
fn minus_zero_and_integer() {
  assert!(is_minus_zero(-0.0));
  assert!(!is_minus_zero(0.0));
  assert!(!is_minus_zero(-1.0));

  assert_eq!(double_to_integer(f64::NAN), 0.0);
  assert_eq!(double_to_integer(-0.0), 0.0);
  assert!(
    double_to_integer(-0.0).is_sign_positive(),
    "NaN/±0 归 +0 (上游早退)"
  );
  assert_eq!(double_to_integer(2.7), 2.0);
  assert_eq!(double_to_integer(-2.3), -2.0, "负数向零取整 = ceil");
  assert!(double_to_integer(f64::INFINITY).is_sign_positive());
  assert!(double_to_integer(f64::NEG_INFINITY).is_sign_negative());
}

/// PrintDouble 四分支: 负零 / 安全整数满精度 / NaN 附位模式 / 默认 Display
#[test]
fn print_double_branches() {
  assert_eq!(print_double(-0.0), "-0.0");
  assert_eq!(print_double(0.0), "0.0");
  assert_eq!(print_double(1.0), "1.0");
  assert_eq!(print_double(-12345.0), "-12345.0");
  assert_eq!(print_double(9007199254740991.0), "9007199254740991.0");
  assert_eq!(print_double(-9007199254740991.0), "-9007199254740991.0");

  let s = print_double(f64::NAN);
  assert_eq!(
    s, "nan (0x7ff8000000000000)",
    "NaN 附位模式 (上游 double_to_uint64)"
  );

  // 非安全整数/小数走默认分支: Rust Display, 与 C++ %g 有意偏离 (见 readme)
  assert_eq!(print_double(2.5), "2.5");
  assert_eq!(print_double(1e16), "10000000000000000");
  assert_eq!(print_double(f64::INFINITY), "inf");
}

/// round2 手验 6 值 (已有测试外): 逐分支对 C++ ostream 格式 (符号/小数点/nan 小写字面量)
#[test]
fn print_double_manual_six() {
  // 安全整数分支: C++ os << (int64_t)val << ".0", 负号保留
  assert_eq!(print_double(-1.0), "-1.0");
  assert_eq!(
    print_double(4503599627370496.0),
    "4503599627370496.0",
    "2^52 界内满精度"
  );
  // 默认分支与 C++ %g 一致的具体值: 6 位有效内最短表示, 负号/小数点直出
  assert_eq!(print_double(-0.5), "-0.5");
  assert_eq!(print_double(123.5), "123.5");
  // NaN 分支: ostream 小写 nan/-nan + "(0x{位模式:x})", 符号位独立于 payload
  assert_eq!(
    print_double(f64::from_bits(0xfff8000000000000)),
    "-nan (0xfff8000000000000)",
    "负 quiet NaN"
  );
  assert_eq!(
    print_double(f64::from_bits(0x7ff0000000000001)),
    "nan (0x7ff0000000000001)",
    "正 signaling NaN 带 payload"
  );
}

#[test]
fn enum_name_tables() {
  use JsIteratorHelperState::*;
  assert_eq!(
    iterator_helper_state_to_str(SuspendedStart),
    "SUSPENDED_START"
  );
  assert_eq!(
    iterator_helper_state_to_str(SuspendedYield),
    "SUSPENDED_YIELD"
  );
  assert_eq!(iterator_helper_state_to_str(Executing), "EXECUTING");
  assert_eq!(iterator_helper_state_to_str(Completed), "COMPLETED");

  use JsIteratorZipHelperMode::*;
  assert_eq!(iterator_zip_helper_mode_to_str(Shortest), "SHORTEST");
  assert_eq!(iterator_zip_helper_mode_to_str(Longest), "LONGEST");
  assert_eq!(iterator_zip_helper_mode_to_str(Strict), "STRICT");
}
