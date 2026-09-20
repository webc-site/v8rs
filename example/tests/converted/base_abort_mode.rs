// 复刻 v8/src/base/abort-mode.cc 语义（v8 无对应 gtest 文件）
// abort-mode.h 被多个 TU 包含，同名符号会经 glob 重导出多次，故走精确模块路径
use aok::{OK, Void};
use v8rs::{
  Value,
  v8::base_abort_mode::{
    ControlledCrashesAreHarmless_1, DcheckFailuresAreIgnored_2,
    FatalErrorsWithNoSecurityImpactShouldExit_3, g_abort_mode_0, v8_base_AbortMode,
    v8_base_AbortMode_kDefault, v8_base_AbortMode_kExitIfNoSecurityImpact,
    v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures,
    v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures, v8_base_AbortMode_kImmediateCrash,
  },
};

fn set_mode(m: v8_base_AbortMode) {
  *g_abort_mode_0.with(Value::clone).borrow_mut() = m;
}

#[test]
fn mode_constants() -> Void {
  assert_eq!(v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures, 0);
  assert_eq!(v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures, 1);
  assert_eq!(v8_base_AbortMode_kExitIfNoSecurityImpact, 2);
  assert_eq!(v8_base_AbortMode_kImmediateCrash, 3);
  assert_eq!(v8_base_AbortMode_kDefault, 4);
  OK
}

#[test]
fn default_mode_all_predicates_false() -> Void {
  assert_eq!(
    *g_abort_mode_0.with(Value::clone).borrow(),
    v8_base_AbortMode_kDefault
  );
  assert!(!ControlledCrashesAreHarmless_1());
  assert!(!DcheckFailuresAreIgnored_2());
  assert!(!FatalErrorsWithNoSecurityImpactShouldExit_3());
  OK
}

#[test]
fn mode_switches_predicates() -> Void {
  set_mode(v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures);
  assert!(ControlledCrashesAreHarmless_1() && DcheckFailuresAreIgnored_2());
  assert!(!FatalErrorsWithNoSecurityImpactShouldExit_3());

  set_mode(v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
  assert!(ControlledCrashesAreHarmless_1() && DcheckFailuresAreIgnored_2());
  assert!(!FatalErrorsWithNoSecurityImpactShouldExit_3());

  set_mode(v8_base_AbortMode_kExitIfNoSecurityImpact);
  assert!(!ControlledCrashesAreHarmless_1() && !DcheckFailuresAreIgnored_2());
  assert!(FatalErrorsWithNoSecurityImpactShouldExit_3());

  set_mode(v8_base_AbortMode_kImmediateCrash);
  assert!(!ControlledCrashesAreHarmless_1());
  assert!(!DcheckFailuresAreIgnored_2());
  assert!(!FatalErrorsWithNoSecurityImpactShouldExit_3());
  OK
}
