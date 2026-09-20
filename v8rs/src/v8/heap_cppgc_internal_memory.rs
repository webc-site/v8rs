use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
#[derive(Default)]
pub struct v8_base_pass_value_or_ref_const_unsigned_char__true_ {}
impl Clone for v8_base_pass_value_or_ref_const_unsigned_char__true_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_pass_value_or_ref_const_unsigned_char__true_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_pass_value_or_ref_const_unsigned_char__true_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_pass_value_or_ref_const_unsigned_char__true_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_pass_value_or_ref_const_unsigned_char_ref__true_ {}
impl Clone for v8_base_pass_value_or_ref_const_unsigned_char_ref__true_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_pass_value_or_ref_const_unsigned_char_ref__true_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_pass_value_or_ref_const_unsigned_char_ref__true_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_pass_value_or_ref_const_unsigned_char_ref__true_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_pass_value_or_ref_int__true_ {}
impl Clone for v8_base_pass_value_or_ref_int__true_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_pass_value_or_ref_int__true_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_pass_value_or_ref_int__true_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_pass_value_or_ref_int__true_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
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
pub type v8_base_comparison_underlying_type_unsigned_char__Dummy = u32;
thread_local!(
    pub static is_enum_12: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_comparison_underlying_type_unsigned_char_ {}
impl Clone for v8_base_comparison_underlying_type_unsigned_char_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_comparison_underlying_type_unsigned_char_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_comparison_underlying_type_unsigned_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_comparison_underlying_type_unsigned_char_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static value_13: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_14: Value<bool> = Rc::new(RefCell::new(true));
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
    pub static value_15: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_unsigned_int__unsigned_int_ {}
impl Clone for v8_base_is_signed_vs_unsigned_unsigned_int__unsigned_int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_signed_vs_unsigned_unsigned_int__unsigned_int_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_signed_vs_unsigned_unsigned_int__unsigned_int_> =
            __this.as_pointer();
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
    pub static value_16: Value<bool> = Rc::new(RefCell::new(false));
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
thread_local!(
    pub static value_17: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_unsigned_char__unsigned_char_ {}
impl Clone for v8_base_is_signed_vs_unsigned_unsigned_char__unsigned_char_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_signed_vs_unsigned_unsigned_char__unsigned_char_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_signed_vs_unsigned_unsigned_char__unsigned_char_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_signed_vs_unsigned_unsigned_char__unsigned_char_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static value_18: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_int__unsigned_char_ {}
impl Clone for v8_base_is_signed_vs_unsigned_int__unsigned_char_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_signed_vs_unsigned_int__unsigned_char_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_signed_vs_unsigned_int__unsigned_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_signed_vs_unsigned_int__unsigned_char_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static value_19: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_unsigned_char__int_ {}
impl Clone for v8_base_is_signed_vs_unsigned_unsigned_char__int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_signed_vs_unsigned_unsigned_char__int_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_signed_vs_unsigned_unsigned_char__int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_signed_vs_unsigned_unsigned_char__int_ {
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
#[derive(Default)]
pub struct v8_base_is_unsigned_vs_signed_unsigned_char__unsigned_char_ {}
impl Clone for v8_base_is_unsigned_vs_signed_unsigned_char__unsigned_char_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_unsigned_vs_signed_unsigned_char__unsigned_char_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_unsigned_vs_signed_unsigned_char__unsigned_char_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_unsigned_vs_signed_unsigned_char__unsigned_char_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_is_unsigned_vs_signed_int__unsigned_char_ {}
impl Clone for v8_base_is_unsigned_vs_signed_int__unsigned_char_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_unsigned_vs_signed_int__unsigned_char_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_unsigned_vs_signed_int__unsigned_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_unsigned_vs_signed_int__unsigned_char_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn CmpEQImpl_20(lhs: u8, rhs: u8) -> bool {
    let lhs: Value<u8> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u8> = Rc::new(RefCell::new(rhs));
    return (((*lhs.borrow()) as i32) == ((*rhs.borrow()) as i32));
}
pub fn CmpEQImpl_21(lhs: i32, rhs: u8) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u8> = Rc::new(RefCell::new(rhs));
    return ((*lhs.borrow()) >= 0)
        && (((*lhs.borrow()) as u32) == (((*rhs.borrow()) as u8) as u32));
}
impl v8_base_Use {
    pub fn v8_base_Use1(_a0: Ptr<bool>) -> Self {
        let __this: Value<v8_base_Use> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_Use> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl v8_base_Use {
    pub fn v8_base_Use2(_a0: Ptr<u8>) -> Self {
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
pub fn make_uint64_22(high: u32, low: u32) -> u64 {
    let high: Value<u32> = Rc::new(RefCell::new(high));
    let low: Value<u32> = Rc::new(RefCell::new(low));
    return (((*high.borrow()) as u64) << 32).wrapping_add(((*low.borrow()) as u64));
}
pub fn RoundDown_23(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    return ((*x.borrow()) & (-(*m.borrow()) as u64));
}
pub fn RoundUp_24(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    (&(0));
    return ({
        let _x: u64 = ((*x.borrow()).wrapping_add((((*m.borrow()) - 1_i64) as u64)) as u64);
        let _m: i64 = (*m.borrow());
        RoundDown_23(_x, _m)
    });
}
pub fn IsAligned_25(value: u64, alignment: u64) -> bool {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let alignment: Value<u64> = Rc::new(RefCell::new(alignment));
    return (((*value.borrow()) & ((*alignment.borrow()).wrapping_sub(1_u64))) == 0_u64);
}
#[derive(Default)]
pub struct AsanUnpoisonScope {}
impl AsanUnpoisonScope {
    pub fn AsanUnpoisonScope(_a0: AnyPtr, _a1: usize) -> Self {
        let _a0: Value<AnyPtr> = Rc::new(RefCell::new(_a0));
        let _a1: Value<usize> = Rc::new(RefCell::new(_a1));
        let __this: Value<AsanUnpoisonScope> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<AsanUnpoisonScope> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for AsanUnpoisonScope {
    fn clone(&self) -> Self {
        let __this: Value<AsanUnpoisonScope> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<AsanUnpoisonScope> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for AsanUnpoisonScope {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn NoSanitizeProbeMemory_26(address: u64) {
    let address: Value<u64> = Rc::new(RefCell::new(address));
    let v: Value<u8> = Rc::new(RefCell::new(
        ((*address.borrow()).reinterpret_cast::<u8>().read()),
    ));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let unused_tmp_array_for_use_macro: Value<Box<[v8_base_Use]>> =
            Rc::new(RefCell::new(Box::new([v8_base_Use::v8_base_Use2({
                v.as_pointer()
            })])));
        &(*unused_tmp_array_for_use_macro.borrow_mut());
    }
}
thread_local!(
    pub static IsAnyMemberTypeV_27: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!();
#[derive(Default)]
pub struct v8_SourceLocation {
    loc_: Value<std_source_location>,
}
impl v8_SourceLocation {
    pub fn Current(loc: Option<Ptr<std_source_location>>) -> v8_SourceLocation {
        return v8_SourceLocation::v8_SourceLocation1({ (loc).clone() });
    }
    pub fn CurrentIfDebug() -> v8_SourceLocation {
        return <v8_SourceLocation>::default();
    }
    fn v8_SourceLocation1(loc: Ptr<std_source_location>) -> Self {
        let __this: Value<v8_SourceLocation> = Rc::new(RefCell::new(Self {
            loc_: Rc::new(RefCell::new((*loc.upgrade().deref()).clone())),
        }));
        let this: Ptr<v8_SourceLocation> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_SourceLocation {
    fn clone(&self) -> Self {
        let __this: Value<v8_SourceLocation> = Rc::new(RefCell::new(Self {
            loc_: Rc::new(RefCell::new((*self.loc_.borrow()).clone())),
        }));
        let this: Ptr<v8_SourceLocation> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_SourceLocation {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.loc_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            loc_: Rc::new(RefCell::new(<std_source_location>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct cppgc_internal_EatParams_const_char__ref_arr_arr_ {}
impl Clone for cppgc_internal_EatParams_const_char__ref_arr_arr_ {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_EatParams_const_char__ref_arr_arr_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<cppgc_internal_EatParams_const_char__ref_arr_arr_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_EatParams_const_char__ref_arr_arr_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub trait cppgc_NameProvider {
    fn SupportsCppClassNamesAsObjectNames() -> bool {
        return true;
    }
    fn GetHumanReadableName(&self) -> Ptr<u8>;
}
#[derive(Default)]
pub struct cppgc_internal_HeapObjectName {
    pub value: Value<Ptr<u8>>,
    pub name_was_hidden: Value<bool>,
}
impl Clone for cppgc_internal_HeapObjectName {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_HeapObjectName> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*self.value.borrow()).clone())),
            name_was_hidden: Rc::new(RefCell::new((*self.name_was_hidden.borrow()))),
        }));
        let this: Ptr<cppgc_internal_HeapObjectName> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_HeapObjectName {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.value.borrow()).to_bytes(&mut buf[0..8]);
        (*self.name_was_hidden.borrow()).to_bytes(&mut buf[8..9]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[0..8]))),
            name_was_hidden: Rc::new(RefCell::new(<bool>::from_bytes(&buf[8..9]))),
        }
    }
}
pub type cppgc_internal_HeapObjectNameForUnnamedObject = u8;
pub const cppgc_internal_HeapObjectNameForUnnamedObject_kUseClassNameIfSupported:
    cppgc_internal_HeapObjectNameForUnnamedObject = 0;
pub const cppgc_internal_HeapObjectNameForUnnamedObject_kUseHiddenName:
    cppgc_internal_HeapObjectNameForUnnamedObject = 1;
#[derive(Default)]
pub struct cppgc_internal_NameTraitBase {}
impl Clone for cppgc_internal_NameTraitBase {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_NameTraitBase> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<cppgc_internal_NameTraitBase> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_NameTraitBase {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive()]
pub struct cppgc_TraceDescriptor {
    pub base_object_payload: Value<AnyPtr>,
    pub callback: Value<FnPtr<fn(Ptr<cppgc_Visitor>, AnyPtr)>>,
}
impl Clone for cppgc_TraceDescriptor {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_TraceDescriptor> = Rc::new(RefCell::new(Self {
            base_object_payload: Rc::new(RefCell::new(
                (*self.base_object_payload.borrow()).clone(),
            )),
            callback: Rc::new(RefCell::new((*self.callback.borrow()).clone())),
        }));
        let this: Ptr<cppgc_TraceDescriptor> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for cppgc_TraceDescriptor {
    fn default() -> Self {
        cppgc_TraceDescriptor {
            base_object_payload: Rc::new(RefCell::new(AnyPtr::default())),
            callback: Rc::new(RefCell::new(FnPtr::<fn(Ptr<cppgc_Visitor>, AnyPtr)>::null())),
        }
    }
}
impl ByteRepr for cppgc_TraceDescriptor {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_object_payload.borrow()).to_bytes(&mut buf[0..8]);
        (*self.callback.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_object_payload: Rc::new(RefCell::new(<AnyPtr>::from_bytes(&buf[0..8]))),
            callback: Rc::new(RefCell::new(
                <FnPtr<fn(Ptr<cppgc_Visitor>, AnyPtr)>>::from_bytes(&buf[8..16]),
            )),
        }
    }
}
#[derive(Default)]
pub struct cppgc_internal_TraceTraitFromInnerAddressImpl {}
impl Clone for cppgc_internal_TraceTraitFromInnerAddressImpl {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_TraceTraitFromInnerAddressImpl> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<cppgc_internal_TraceTraitFromInnerAddressImpl> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_TraceTraitFromInnerAddressImpl {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static kMaxGCInfoIndex_29: Value<u16> = Rc::new(RefCell::new(16384));
);
thread_local!(
    pub static kMinGCInfoIndex_30: Value<u16> = Rc::new(RefCell::new(1));
);
#[derive()]
pub struct cppgc_internal_GCInfo {
    pub finalize: Value<FnPtr<fn(AnyPtr)>>,
    pub trace: Value<FnPtr<fn(Ptr<cppgc_Visitor>, AnyPtr)>>,
    pub name: Value<
        FnPtr<
            fn(
                AnyPtr,
                cppgc_internal_HeapObjectNameForUnnamedObject,
            ) -> cppgc_internal_HeapObjectName,
        >,
    >,
    pub padding: Value<usize>,
}
impl cppgc_internal_GCInfo {
    pub fn cppgc_internal_GCInfo(
        finalize: FnPtr<fn(AnyPtr)>,
        trace: FnPtr<fn(Ptr<cppgc_Visitor>, AnyPtr)>,
        name: FnPtr<
            fn(
                AnyPtr,
                cppgc_internal_HeapObjectNameForUnnamedObject,
            ) -> cppgc_internal_HeapObjectName,
        >,
    ) -> Self {
        let finalize: Value<FnPtr<fn(AnyPtr)>> = Rc::new(RefCell::new(finalize));
        let trace: Value<FnPtr<fn(Ptr<cppgc_Visitor>, AnyPtr)>> = Rc::new(RefCell::new(trace));
        let name: Value<
            FnPtr<
                fn(
                    AnyPtr,
                    cppgc_internal_HeapObjectNameForUnnamedObject,
                ) -> cppgc_internal_HeapObjectName,
            >,
        > = Rc::new(RefCell::new(name));
        let __this: Value<cppgc_internal_GCInfo> = Rc::new(RefCell::new(Self {
            finalize: Rc::new(RefCell::new((*finalize.borrow()).clone())),
            trace: Rc::new(RefCell::new((*trace.borrow()).clone())),
            name: Rc::new(RefCell::new((*name.borrow()).clone())),
            padding: Rc::new(RefCell::new(0_usize)),
        }));
        let this: Ptr<cppgc_internal_GCInfo> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for cppgc_internal_GCInfo {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_GCInfo> = Rc::new(RefCell::new(Self {
            finalize: Rc::new(RefCell::new((*self.finalize.borrow()).clone())),
            trace: Rc::new(RefCell::new((*self.trace.borrow()).clone())),
            name: Rc::new(RefCell::new((*self.name.borrow()).clone())),
            padding: Rc::new(RefCell::new((*self.padding.borrow()))),
        }));
        let this: Ptr<cppgc_internal_GCInfo> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for cppgc_internal_GCInfo {
    fn default() -> Self {
        cppgc_internal_GCInfo {
            finalize: Rc::new(RefCell::new(FnPtr::<fn(AnyPtr)>::null())),
            trace: Rc::new(RefCell::new(FnPtr::<fn(Ptr<cppgc_Visitor>, AnyPtr)>::null())),
            name: Rc::new(RefCell::new(FnPtr::<
                fn(
                    AnyPtr,
                    cppgc_internal_HeapObjectNameForUnnamedObject,
                ) -> cppgc_internal_HeapObjectName,
            >::null())),
            padding: Rc::new(RefCell::new(0_usize)),
        }
    }
}
impl ByteRepr for cppgc_internal_GCInfo {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.finalize.borrow()).to_bytes(&mut buf[0..8]);
        (*self.trace.borrow()).to_bytes(&mut buf[8..16]);
        (*self.name.borrow()).to_bytes(&mut buf[16..24]);
        (*self.padding.borrow()).to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            finalize: Rc::new(RefCell::new(<FnPtr<fn(AnyPtr)>>::from_bytes(&buf[0..8]))),
            trace: Rc::new(RefCell::new(
                <FnPtr<fn(Ptr<cppgc_Visitor>, AnyPtr)>>::from_bytes(&buf[8..16]),
            )),
            name: Rc::new(RefCell::new(<FnPtr<
                fn(
                    AnyPtr,
                    cppgc_internal_HeapObjectNameForUnnamedObject,
                ) -> cppgc_internal_HeapObjectName,
            >>::from_bytes(&buf[16..24]))),
            padding: Rc::new(RefCell::new(<usize>::from_bytes(&buf[24..32]))),
        }
    }
}
pub fn GetHiddenName_31(
    _a0: AnyPtr,
    name_retrieval_mode: cppgc_internal_HeapObjectNameForUnnamedObject,
) -> cppgc_internal_HeapObjectName {
    let _a0: Value<AnyPtr> = Rc::new(RefCell::new(_a0));
    let name_retrieval_mode: Value<cppgc_internal_HeapObjectNameForUnnamedObject> =
        Rc::new(RefCell::new(name_retrieval_mode));
    return cppgc_internal_HeapObjectName {
        value: Rc::new(RefCell::new(
            (kHiddenName_32.with(Value::clone).as_pointer() as Ptr<u8>),
        )),
        name_was_hidden: Rc::new(RefCell::new(
            ((*name_retrieval_mode.borrow())
                == cppgc_internal_HeapObjectNameForUnnamedObject_kUseHiddenName),
        )),
    };
}
thread_local!();
thread_local!();
#[derive(Default)]
pub struct cppgc_internal_GCInfoTableSection {}
impl cppgc_internal_GCInfoTableSection {
    pub fn Index(info: Ptr<cppgc_internal_GCInfo>) -> u16 {
        return ((((info).clone()
            - (__start_gc_info_section_33.with(Value::clone).as_pointer()
                as Ptr<cppgc_internal_GCInfo>)) as i64
            + 1_i64) as u16);
    }
    pub fn GCInfoFromIndex(index: u16) -> Ptr<cppgc_internal_GCInfo> {
        let index: Value<u16> = Rc::new(RefCell::new(index));
        (&(cppgc_internal_EatParams_const_char__ref_arr_arr_ {}));
        (&(cppgc_internal_EatParams_const_char__ref_arr_arr_ {}));
        return (__start_gc_info_section_33.with(Value::clone).as_pointer()
            as Ptr<Box<[cppgc_internal_GCInfo]>>)
            .offset((((*index.borrow()) as i32) - 1));
    }
}
impl Clone for cppgc_internal_GCInfoTableSection {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_GCInfoTableSection> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<cppgc_internal_GCInfoTableSection> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_GCInfoTableSection {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static kKB_36: Value<usize> = Rc::new(RefCell::new(1024));
);
thread_local!(
    pub static kMB_37: Value<usize> = Rc::new(RefCell::new(1048576));
);
thread_local!(
    pub static kGB_38: Value<usize> = Rc::new(RefCell::new(1073741824));
);
pub type cppgc_internal_AccessMode = u8;
pub const cppgc_internal_AccessMode_kNonAtomic: cppgc_internal_AccessMode = 0;
pub const cppgc_internal_AccessMode_kAtomic: cppgc_internal_AccessMode = 1;
thread_local!(
    pub static kAllocationGranularity_39: Value<usize> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kAllocationMask_40: Value<usize> = Rc::new(RefCell::new(7));
);
thread_local!(
    pub static kPageSizeLog2_41: Value<usize> = Rc::new(RefCell::new(17));
);
thread_local!(
    pub static kPageSize_42: Value<usize> = Rc::new(RefCell::new(131072));
);
thread_local!(
    pub static kPageOffsetMask_43: Value<usize> = Rc::new(RefCell::new(131071));
);
thread_local!(
    pub static kPageBaseMask_44: Value<usize> = Rc::new(RefCell::new(18446744073709420544));
);
thread_local!(
    pub static kLargeObjectSizeThreshold_45: Value<usize> = Rc::new(RefCell::new(65536));
);
thread_local!(
    pub static kFreeListGCInfoIndex_46: Value<u16> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kFreeListEntrySize_47: Value<usize> = Rc::new(RefCell::new(16));
);
thread_local!(
    pub static kSlotSize_48: Value<usize> = Rc::new(RefCell::new(4));
);
thread_local!(
    pub static kZappedValue_49: Value<u8> = Rc::new(RefCell::new(220));
);
pub fn ZapMemory_50(address: AnyPtr, size: usize) {
    let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    {
        (*address.borrow()).memset((220 as i32) as u8, (*size.borrow()) as usize);
        (*address.borrow()).clone()
    };
}
pub fn CheckMemoryIsZapped_51(address: AnyPtr, size: usize) {
    let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*size.borrow())) {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            let _cmp: Value<bool> = Rc::new(RefCell::new(
                ({
                    CmpEQImpl_20(
                        (220),
                        ((*address.borrow())
                            .reinterpret_cast::<u8>()
                            .offset((*i.borrow()) as isize)
                            .read()),
                    )
                }),
            ));
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if ((!(!(!(*_cmp.borrow()))) as i64) != 0) {
                    ({
                        V8_Fatal_52(
                            Ptr::from_string_literal(b"Check failed: %s."),
                            &[(Ptr::from_string_literal(
                                b"kZappedValue == reinterpret_cast<ConstAddress>(address)[i]",
                            ))
                            .into()],
                        )
                    });
                }
            }
        }
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn CheckMemoryIsZero_53(address: AnyPtr, size: usize) {
    let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*size.borrow())) {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            let _cmp: Value<bool> = Rc::new(RefCell::new(
                ({
                    CmpEQImpl_21(
                        (0),
                        ((*address.borrow())
                            .reinterpret_cast::<u8>()
                            .offset((*i.borrow()) as isize)
                            .read()),
                    )
                }),
            ));
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if ((!(!(!(*_cmp.borrow()))) as i64) != 0) {
                    ({
                        V8_Fatal_52(
                            Ptr::from_string_literal(b"Check failed: %s."),
                            &[(Ptr::from_string_literal(
                                b"0 == reinterpret_cast<ConstAddress>(address)[i]",
                            ))
                            .into()],
                        )
                    });
                }
            }
        }
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn SetMemoryAccessible_54(address: AnyPtr, size: usize) {
    let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
    let size: Value<usize> = Rc::new(RefCell::new(size));
}
pub fn CheckMemoryIsInaccessible_55(address: AnyPtr, size: usize) {
    let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
    let size: Value<usize> = Rc::new(RefCell::new(size));
}
pub fn CheckMemoryIsInaccessibleIsNoop_56() -> bool {
    return true;
}
pub fn SetMemoryInaccessible_57(address: AnyPtr, size: usize) {
    let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    {
        (*address.borrow()).memset((0) as u8, (*size.borrow()) as usize);
        (*address.borrow()).clone()
    };
}
pub fn NoSanitizeMemset_58(address: AnyPtr, c: u8, bytes: usize) {
    let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
    let c: Value<u8> = Rc::new(RefCell::new(c));
    let bytes: Value<usize> = Rc::new(RefCell::new(bytes));
    let base: Value<Ptr<u8>> = Rc::new(RefCell::new((*address.borrow()).reinterpret_cast::<u8>()));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*bytes.borrow())) {
        let __rhs = (*c.borrow());
        (*base.borrow()).offset((*i.borrow()) as isize).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
}
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct cppgc_Visitor;
pub trait v8_SourceLocationImpl {
    fn Function(&self) -> Ptr<u8>;
    fn FileName(&self) -> Ptr<u8>;
    fn Line(&self) -> usize;
    fn operator__Bool(&self) -> bool;
}
impl v8_SourceLocationImpl for Ptr<v8_SourceLocation> {
    fn Function(&self) -> Ptr<u8> {
        return ({ (*(*(*self).upgrade().deref()).loc_.borrow()).function_name() });
    }
    fn FileName(&self) -> Ptr<u8> {
        return ({ (*(*(*self).upgrade().deref()).loc_.borrow()).file_name() });
    }
    fn Line(&self) -> usize {
        return (({ (*(*(*self).upgrade().deref()).loc_.borrow()).line() }) as usize);
    }
    fn operator__Bool(&self) -> bool {
        return (({ (*(*(*self).upgrade().deref()).loc_.borrow()).line() }) != 0_u32);
    }
}
