use std::{
  cell::RefCell,
  collections::BTreeMap,
  io::{Read, Seek, Write, prelude::*},
  os::fd::AsFd,
  rc::{Rc, Weak},
};

use crate::*;
pub type v8_base_AbortMode = i32;
pub const v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures: v8_base_AbortMode = 0;
pub const v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures: v8_base_AbortMode = 1;
pub const v8_base_AbortMode_kExitIfNoSecurityImpact: v8_base_AbortMode = 2;
pub const v8_base_AbortMode_kImmediateCrash: v8_base_AbortMode = 3;
pub const v8_base_AbortMode_kDefault: v8_base_AbortMode = 4;
thread_local!();
pub fn ControlledCrashesAreHarmless_1() -> bool {
  return ((*g_abort_mode_0.with(Value::clone).borrow())
    == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
    || ((*g_abort_mode_0.with(Value::clone).borrow())
      == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn DcheckFailuresAreIgnored_2() -> bool {
  return ((*g_abort_mode_0.with(Value::clone).borrow())
    == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
    || ((*g_abort_mode_0.with(Value::clone).borrow())
      == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn FatalErrorsWithNoSecurityImpactShouldExit_3() -> bool {
  return ((*g_abort_mode_0.with(Value::clone).borrow())
    == v8_base_AbortMode_kExitIfNoSecurityImpact);
}
thread_local!(
  pub static g_abort_mode_0: Value<v8_base_AbortMode> =
    Rc::new(RefCell::new(v8_base_AbortMode_kDefault));
);
