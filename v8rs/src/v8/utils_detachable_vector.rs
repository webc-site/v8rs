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
thread_local!();
thread_local!();
thread_local!();
thread_local!();
#[derive(Default)]
pub struct v8_internal_DetachableVectorBase {
  data_: Value<AnyPtr>,
  capacity_: Value<usize>,
  size_: Value<usize>,
}
impl Clone for v8_internal_DetachableVectorBase {
  fn clone(&self) -> Self {
    let __this: Value<v8_internal_DetachableVectorBase> = Rc::new(RefCell::new(Self {
      data_: Rc::new(RefCell::new((*self.data_.borrow()).clone())),
      capacity_: Rc::new(RefCell::new((*self.capacity_.borrow()))),
      size_: Rc::new(RefCell::new((*self.size_.borrow()))),
    }));
    let this: Ptr<v8_internal_DetachableVectorBase> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl ByteRepr for v8_internal_DetachableVectorBase {
  fn byte_size() -> usize {
    24
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    (*self.data_.borrow()).to_bytes(&mut buf[0..8]);
    (*self.capacity_.borrow()).to_bytes(&mut buf[8..16]);
    (*self.size_.borrow()).to_bytes(&mut buf[16..24]);
  }
  fn from_bytes(buf: &[u8]) -> Self {
    Self {
      data_: Rc::new(RefCell::new(<AnyPtr>::from_bytes(&buf[0..8]))),
      capacity_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
      size_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[16..24]))),
    }
  }
}
thread_local!(
  pub static kMinimumCapacity_20: Value<usize> = Rc::new(RefCell::new(8_usize));
);
thread_local!(
  pub static kDataOffset_21: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
  pub static kCapacityOffset_22: Value<usize> = Rc::new(RefCell::new(8_usize));
);
thread_local!(
  pub static kSizeOffset_23: Value<usize> = Rc::new(RefCell::new(16_usize));
);
pub trait v8_internal_DetachableVectorBaseImpl {
  fn detach(&self);
  fn pop_back(&self);
  fn capacity(&self) -> usize;
  fn size(&self) -> usize;
  fn empty(&self) -> bool;
}
impl v8_internal_DetachableVectorBaseImpl for Ptr<v8_internal_DetachableVectorBase> {
  fn detach(&self) {
    (*(*(*self).upgrade().deref()).data_.borrow_mut()) = AnyPtr::default();
    (*(*(*self).upgrade().deref()).capacity_.borrow_mut()) = 0_usize;
    (*(*(*self).upgrade().deref()).size_.borrow_mut()) = 0_usize;
  }
  fn pop_back(&self) {
    (*(*(*self).upgrade().deref()).size_.borrow_mut()).prefix_dec();
  }
  fn capacity(&self) -> usize {
    return (*(*(*self).upgrade().deref()).capacity_.borrow());
  }
  fn size(&self) -> usize {
    return (*(*(*self).upgrade().deref()).size_.borrow());
  }
  fn empty(&self) -> bool {
    return ((*(*(*self).upgrade().deref()).size_.borrow()) == 0_usize);
  }
}
