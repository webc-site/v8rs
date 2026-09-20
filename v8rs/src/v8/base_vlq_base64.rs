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
thread_local!(
  pub static kCharToDigit_16: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    62_i8,
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    63_i8,
    52_i8,
    53_i8,
    54_i8,
    55_i8,
    56_i8,
    57_i8,
    58_i8,
    59_i8,
    60_i8,
    61_i8,
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    0_i8,
    1_i8,
    2_i8,
    3_i8,
    4_i8,
    5_i8,
    6_i8,
    7_i8,
    8_i8,
    9_i8,
    10_i8,
    11_i8,
    12_i8,
    13_i8,
    14_i8,
    15_i8,
    16_i8,
    17_i8,
    18_i8,
    19_i8,
    20_i8,
    21_i8,
    22_i8,
    23_i8,
    24_i8,
    25_i8,
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    26_i8,
    27_i8,
    28_i8,
    29_i8,
    30_i8,
    31_i8,
    32_i8,
    33_i8,
    34_i8,
    35_i8,
    36_i8,
    37_i8,
    38_i8,
    39_i8,
    40_i8,
    41_i8,
    42_i8,
    43_i8,
    44_i8,
    45_i8,
    46_i8,
    47_i8,
    48_i8,
    49_i8,
    50_i8,
    51_i8,
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
    (-1_i32 as i8),
  ])));
);
thread_local!(
  pub static kContinueShift_17: Value<u32> = Rc::new(RefCell::new(5));
);
thread_local!(
  pub static kContinueMask_18: Value<u32> = Rc::new(RefCell::new(32));
);
thread_local!(
  pub static kDataMask_19: Value<u32> = Rc::new(RefCell::new(31));
);
pub fn charToDigitDecode_20(c: u8) -> i8 {
  let c: Value<u8> = Rc::new(RefCell::new(c));
  return (if (((*c.borrow()) as u32) < 128_u32) {
    ((*kCharToDigit_16.with(Value::clone).borrow())[(*c.borrow()) as usize] as i32)
  } else {
    -1_i32
  } as i8);
}
pub fn charToDigitDecodeForTesting_21(c: u8) -> i8 {
  let c: Value<u8> = Rc::new(RefCell::new(c));
  return ({ charToDigitDecode_20((*c.borrow())) });
}
pub fn VLQBase64Decode_22(start: Ptr<u8>, sz: usize, pos: Ptr<usize>) -> i32 {
  let start: Value<Ptr<u8>> = Rc::new(RefCell::new(start));
  let sz: Value<usize> = Rc::new(RefCell::new(sz));
  let pos: Value<Ptr<usize>> = Rc::new(RefCell::new(pos));
  let res: Value<u32> = Rc::new(RefCell::new(0_u32));
  let shift: Value<u64> = Rc::new(RefCell::new(0_u64));
  let digit: Value<i32> = <Value<i32>>::default();
  let mut __do_while = true;
  'loop_: while __do_while || ((((*digit.borrow()) as u32) & 32) != 0) {
    __do_while = false;
    if {
      let _lhs = ((*pos.borrow()).read());
      _lhs >= (*sz.borrow())
    } {
      return <i32>::MIN;
    }
    (*digit.borrow_mut()) = (({
      charToDigitDecode_20(
        ((*start.borrow())
          .offset(((*pos.borrow()).read()) as isize)
          .read()),
      )
    }) as i32);
    let is_last_byte: Value<bool> = Rc::new(RefCell::new(
      ((*shift.borrow()).wrapping_add((5 as u64)) >= 32_u64),
    ));
    if ((*digit.borrow()) == -1_i32)
      || ((*is_last_byte.borrow()) && (((*digit.borrow()) >> 2) != 0))
    {
      return <i32>::MIN;
    }
    {
      let rhs_0 =
        (*res.borrow()).wrapping_add(((((*digit.borrow()) as u32) & 31) << (*shift.borrow())));
      (*res.borrow_mut()) = rhs_0
    };
    {
      let rhs_0 = (*shift.borrow()).wrapping_add((5 as u64));
      (*shift.borrow_mut()) = rhs_0
    };
    (*pos.borrow()).with_mut(|__v| __v.postfix_inc());
  }
  return (if (((*res.borrow()) & 1_u32) != 0) {
    (-(((*res.borrow()) >> 1) as i32) as u32)
  } else {
    ((*res.borrow()) >> 1)
  } as i32);
}
