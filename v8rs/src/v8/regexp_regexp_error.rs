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
  pub static kReturnAddressStackSlotCount_4: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
  pub static kPageSizeBits_5: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
  pub static kRegularPageSize_6: Value<i32> = Rc::new(RefCell::new(262144));
);
thread_local!(
  pub static kMinimumOSPageSize_7: Value<i32> = Rc::new(RefCell::new(16384));
);
thread_local!(
  pub static kUnimplementedCodeMessage_8: Value<Ptr<u8>> = Rc::new(RefCell::new(
    Ptr::from_string_literal(b"unimplemented code"),
  ));
);
thread_local!(
  pub static kUnreachableCodeMessage_9: Value<Ptr<u8>> =
    Rc::new(RefCell::new(Ptr::from_string_literal(b"unreachable code")));
);
#[derive(Default)]
pub struct v8_base_CheckMessageStream {}
impl ByteRepr for v8_base_CheckMessageStream {
  fn byte_size() -> usize {
    264
  }
  fn to_bytes(&self, buf: &mut [u8]) {}
  fn from_bytes(buf: &[u8]) -> Self {
    Self {}
  }
}
pub type v8_base_OOMType = i32;
pub const v8_base_OOMType_kJavaScript: v8_base_OOMType = 0;
pub const v8_base_OOMType_kProcess: v8_base_OOMType = 1;
pub type v8_base_comparison_underlying_type_unsigned_int__Dummy = u32;
thread_local!(
  pub static is_enum_10: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_comparison_underlying_type_unsigned_int_ {}
impl Clone for v8_base_comparison_underlying_type_unsigned_int_ {
  fn clone(&self) -> Self {
    let __this: Value<v8_base_comparison_underlying_type_unsigned_int_> =
      Rc::new(RefCell::new(Self {}));
    let this: Ptr<v8_base_comparison_underlying_type_unsigned_int_> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl ByteRepr for v8_base_comparison_underlying_type_unsigned_int_ {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {}
  fn from_bytes(buf: &[u8]) -> Self {
    Self {}
  }
}
pub type v8_base_comparison_underlying_type_int__Dummy = u32;
thread_local!(
  pub static is_enum_11: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_comparison_underlying_type_int_ {}
impl Clone for v8_base_comparison_underlying_type_int_ {
  fn clone(&self) -> Self {
    let __this: Value<v8_base_comparison_underlying_type_int_> = Rc::new(RefCell::new(Self {}));
    let this: Ptr<v8_base_comparison_underlying_type_int_> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl ByteRepr for v8_base_comparison_underlying_type_int_ {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {}
  fn from_bytes(buf: &[u8]) -> Self {
    Self {}
  }
}
thread_local!(
  pub static value_12: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_unsigned_int__int_ {}
impl Clone for v8_base_is_signed_vs_unsigned_unsigned_int__int_ {
  fn clone(&self) -> Self {
    let __this: Value<v8_base_is_signed_vs_unsigned_unsigned_int__int_> =
      Rc::new(RefCell::new(Self {}));
    let this: Ptr<v8_base_is_signed_vs_unsigned_unsigned_int__int_> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl ByteRepr for v8_base_is_signed_vs_unsigned_unsigned_int__int_ {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {}
  fn from_bytes(buf: &[u8]) -> Self {
    Self {}
  }
}
thread_local!(
  pub static value_13: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_int__unsigned_int_ {}
impl Clone for v8_base_is_signed_vs_unsigned_int__unsigned_int_ {
  fn clone(&self) -> Self {
    let __this: Value<v8_base_is_signed_vs_unsigned_int__unsigned_int_> =
      Rc::new(RefCell::new(Self {}));
    let this: Ptr<v8_base_is_signed_vs_unsigned_int__unsigned_int_> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl ByteRepr for v8_base_is_signed_vs_unsigned_int__unsigned_int_ {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {}
  fn from_bytes(buf: &[u8]) -> Self {
    Self {}
  }
}
thread_local!(
  pub static value_14: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_unsigned_int__unsigned_int_ {}
impl Clone for v8_base_is_signed_vs_unsigned_unsigned_int__unsigned_int_ {
  fn clone(&self) -> Self {
    let __this: Value<v8_base_is_signed_vs_unsigned_unsigned_int__unsigned_int_> =
      Rc::new(RefCell::new(Self {}));
    let this: Ptr<v8_base_is_signed_vs_unsigned_unsigned_int__unsigned_int_> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl ByteRepr for v8_base_is_signed_vs_unsigned_unsigned_int__unsigned_int_ {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {}
  fn from_bytes(buf: &[u8]) -> Self {
    Self {}
  }
}
thread_local!(
  pub static value_15: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_int__int_ {}
impl Clone for v8_base_is_signed_vs_unsigned_int__int_ {
  fn clone(&self) -> Self {
    let __this: Value<v8_base_is_signed_vs_unsigned_int__int_> = Rc::new(RefCell::new(Self {}));
    let this: Ptr<v8_base_is_signed_vs_unsigned_int__int_> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl ByteRepr for v8_base_is_signed_vs_unsigned_int__int_ {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {}
  fn from_bytes(buf: &[u8]) -> Self {
    Self {}
  }
}
#[derive(Default)]
pub struct v8_base_is_unsigned_vs_signed_unsigned_int__int_ {}
impl Clone for v8_base_is_unsigned_vs_signed_unsigned_int__int_ {
  fn clone(&self) -> Self {
    let __this: Value<v8_base_is_unsigned_vs_signed_unsigned_int__int_> =
      Rc::new(RefCell::new(Self {}));
    let this: Ptr<v8_base_is_unsigned_vs_signed_unsigned_int__int_> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl ByteRepr for v8_base_is_unsigned_vs_signed_unsigned_int__int_ {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {}
  fn from_bytes(buf: &[u8]) -> Self {
    Self {}
  }
}
#[derive(Default)]
pub struct v8_base_is_unsigned_vs_signed_int__unsigned_int_ {}
impl Clone for v8_base_is_unsigned_vs_signed_int__unsigned_int_ {
  fn clone(&self) -> Self {
    let __this: Value<v8_base_is_unsigned_vs_signed_int__unsigned_int_> =
      Rc::new(RefCell::new(Self {}));
    let this: Ptr<v8_base_is_unsigned_vs_signed_int__unsigned_int_> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl ByteRepr for v8_base_is_unsigned_vs_signed_int__unsigned_int_ {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {}
  fn from_bytes(buf: &[u8]) -> Self {
    Self {}
  }
}
impl v8_base_Use {
  pub fn v8_base_Use(_a0: Ptr<bool>) -> Self {
    let __this: Value<v8_base_Use> = Rc::new(RefCell::new(Self {}));
    let this: Ptr<v8_base_Use> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
#[derive(Default)]
pub struct v8_base_Use {}
impl Clone for v8_base_Use {
  fn clone(&self) -> Self {
    let __this: Value<v8_base_Use> = Rc::new(RefCell::new(Self {}));
    let this: Ptr<v8_base_Use> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl ByteRepr for v8_base_Use {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {}
  fn from_bytes(buf: &[u8]) -> Self {
    Self {}
  }
}
pub fn make_uint64_16(high: u32, low: u32) -> u64 {
  let high: Value<u32> = Rc::new(RefCell::new(high));
  let low: Value<u32> = Rc::new(RefCell::new(low));
  return (((*high.borrow()) as u64) << 32).wrapping_add(((*low.borrow()) as u64));
}
pub fn RoundDown_17(x: u64, m: i64) -> u64 {
  let x: Value<u64> = Rc::new(RefCell::new(x));
  let m: Value<i64> = Rc::new(RefCell::new(m));
  (&(0));
  return ((*x.borrow()) & (-(*m.borrow()) as u64));
}
pub fn RoundUp_18(x: u64, m: i64) -> u64 {
  let x: Value<u64> = Rc::new(RefCell::new(x));
  let m: Value<i64> = Rc::new(RefCell::new(m));
  (&(0));
  (&(0));
  return ({
    let _x: u64 = ((*x.borrow()).wrapping_add((((*m.borrow()) - 1_i64) as u64)) as u64);
    let _m: i64 = (*m.borrow());
    RoundDown_17(_x, _m)
  });
}
pub fn IsAligned_19(value: u64, alignment: u64) -> bool {
  let value: Value<u64> = Rc::new(RefCell::new(value));
  let alignment: Value<u64> = Rc::new(RefCell::new(alignment));
  return (((*value.borrow()) & ((*alignment.borrow()).wrapping_sub(1_u64))) == 0_u64);
}
pub type v8_internal_regexp_Error = u32;
pub const v8_internal_regexp_Error_kNone: v8_internal_regexp_Error = 0;
pub const v8_internal_regexp_Error_kStackOverflow: v8_internal_regexp_Error = 1;
pub const v8_internal_regexp_Error_kAnalysisStackOverflow: v8_internal_regexp_Error = 2;
pub const v8_internal_regexp_Error_kTooLarge: v8_internal_regexp_Error = 3;
pub const v8_internal_regexp_Error_kUnterminatedGroup: v8_internal_regexp_Error = 4;
pub const v8_internal_regexp_Error_kUnmatchedParen: v8_internal_regexp_Error = 5;
pub const v8_internal_regexp_Error_kEscapeAtEndOfPattern: v8_internal_regexp_Error = 6;
pub const v8_internal_regexp_Error_kInvalidPropertyName: v8_internal_regexp_Error = 7;
pub const v8_internal_regexp_Error_kInvalidEscape: v8_internal_regexp_Error = 8;
pub const v8_internal_regexp_Error_kInvalidDecimalEscape: v8_internal_regexp_Error = 9;
pub const v8_internal_regexp_Error_kInvalidUnicodeEscape: v8_internal_regexp_Error = 10;
pub const v8_internal_regexp_Error_kNothingToRepeat: v8_internal_regexp_Error = 11;
pub const v8_internal_regexp_Error_kLoneQuantifierBrackets: v8_internal_regexp_Error = 12;
pub const v8_internal_regexp_Error_kRangeOutOfOrder: v8_internal_regexp_Error = 13;
pub const v8_internal_regexp_Error_kIncompleteQuantifier: v8_internal_regexp_Error = 14;
pub const v8_internal_regexp_Error_kInvalidQuantifier: v8_internal_regexp_Error = 15;
pub const v8_internal_regexp_Error_kInvalidGroup: v8_internal_regexp_Error = 16;
pub const v8_internal_regexp_Error_kMultipleFlagDashes: v8_internal_regexp_Error = 17;
pub const v8_internal_regexp_Error_kNotLinear: v8_internal_regexp_Error = 18;
pub const v8_internal_regexp_Error_kRepeatedFlag: v8_internal_regexp_Error = 19;
pub const v8_internal_regexp_Error_kInvalidFlagGroup: v8_internal_regexp_Error = 20;
pub const v8_internal_regexp_Error_kTooManyCaptures: v8_internal_regexp_Error = 21;
pub const v8_internal_regexp_Error_kInvalidCaptureGroupName: v8_internal_regexp_Error = 22;
pub const v8_internal_regexp_Error_kDuplicateCaptureGroupName: v8_internal_regexp_Error = 23;
pub const v8_internal_regexp_Error_kInvalidNamedReference: v8_internal_regexp_Error = 24;
pub const v8_internal_regexp_Error_kInvalidNamedCaptureReference: v8_internal_regexp_Error = 25;
pub const v8_internal_regexp_Error_kInvalidClassPropertyName: v8_internal_regexp_Error = 26;
pub const v8_internal_regexp_Error_kInvalidCharacterClass: v8_internal_regexp_Error = 27;
pub const v8_internal_regexp_Error_kUnterminatedCharacterClass: v8_internal_regexp_Error = 28;
pub const v8_internal_regexp_Error_kOutOfOrderCharacterClass: v8_internal_regexp_Error = 29;
pub const v8_internal_regexp_Error_kInvalidClassSetOperation: v8_internal_regexp_Error = 30;
pub const v8_internal_regexp_Error_kInvalidCharacterInClass: v8_internal_regexp_Error = 31;
pub const v8_internal_regexp_Error_kNegatedCharacterClassWithStrings: v8_internal_regexp_Error = 32;
pub const v8_internal_regexp_Error_kUnsupportedBytecode: v8_internal_regexp_Error = 33;
pub const v8_internal_regexp_Error_NumErrors: v8_internal_regexp_Error = 34;
pub fn ErrorIsStackOverflow_20(error: v8_internal_regexp_Error) -> bool {
  let error: Value<v8_internal_regexp_Error> = Rc::new(RefCell::new(error));
  return (((*error.borrow()) == v8_internal_regexp_Error_kStackOverflow)
    || ((*error.borrow()) == v8_internal_regexp_Error_kAnalysisStackOverflow))
    || ((*error.borrow()) == v8_internal_regexp_Error_kTooLarge);
}
thread_local!(
  pub static kErrorStrings_21: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
    Ptr::from_string_literal(b""),
    Ptr::from_string_literal(b"Maximum call stack size exceeded"),
    Ptr::from_string_literal(b"Stack overflow"),
    Ptr::from_string_literal(b"Regular expression too large"),
    Ptr::from_string_literal(b"Unterminated group"),
    Ptr::from_string_literal(b"Unmatched \')\'"),
    Ptr::from_string_literal(b"\\ at end of pattern"),
    Ptr::from_string_literal(b"Invalid property name"),
    Ptr::from_string_literal(b"Invalid escape"),
    Ptr::from_string_literal(b"Invalid decimal escape"),
    Ptr::from_string_literal(b"Invalid Unicode escape"),
    Ptr::from_string_literal(b"Nothing to repeat"),
    Ptr::from_string_literal(b"Lone quantifier brackets"),
    Ptr::from_string_literal(b"numbers out of order in {} quantifier"),
    Ptr::from_string_literal(b"Incomplete quantifier"),
    Ptr::from_string_literal(b"Invalid quantifier"),
    Ptr::from_string_literal(b"Invalid group"),
    Ptr::from_string_literal(b"Multiple dashes in flag group"),
    Ptr::from_string_literal(b"Cannot be executed in linear time"),
    Ptr::from_string_literal(b"Repeated flag in flag group"),
    Ptr::from_string_literal(b"Invalid flag group"),
    Ptr::from_string_literal(b"Too many captures"),
    Ptr::from_string_literal(b"Invalid capture group name"),
    Ptr::from_string_literal(b"Duplicate capture group name"),
    Ptr::from_string_literal(b"Invalid named reference"),
    Ptr::from_string_literal(b"Invalid named capture referenced"),
    Ptr::from_string_literal(b"Invalid property name in character class"),
    Ptr::from_string_literal(b"Invalid character class"),
    Ptr::from_string_literal(b"Unterminated character class"),
    Ptr::from_string_literal(b"Range out of order in character class"),
    Ptr::from_string_literal(b"Invalid set operation in character class"),
    Ptr::from_string_literal(b"Invalid character in character class"),
    Ptr::from_string_literal(b"Negated character class may contain strings"),
    Ptr::from_string_literal(b"Unsupported Bytecode"),
  ])));
);
pub fn ErrorString_22(error: v8_internal_regexp_Error) -> Ptr<u8> {
  let error: Value<v8_internal_regexp_Error> = Rc::new(RefCell::new(error));
  (&(0));
  return ((*kErrorStrings_21.with(Value::clone).borrow())[((*error.borrow()) as i32) as usize])
    .clone();
}
