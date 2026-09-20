use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static kReturnAddressStackSlotCount_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kPageSizeBits_1: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
    pub static kRegularPageSize_2: Value<i32> = Rc::new(RefCell::new(262144));
);
thread_local!(
    pub static kMinimumOSPageSize_3: Value<i32> = Rc::new(RefCell::new(16384));
);
thread_local!(
    static kMaxTraces_4: Value<i32> = Rc::new(RefCell::new(62));
);
#[derive()]
pub struct v8_base_debug_StackTrace {
    trace_: Value<Box<[AnyPtr]>>,
    count_: Value<usize>,
}
impl v8_base_debug_StackTrace {
    pub fn v8_base_debug_StackTrace1() -> Self {
        let __this: Value<v8_base_debug_StackTrace> = Rc::new(RefCell::new(Self {
            trace_: Rc::new(RefCell::new(
                (0..62)
                    .map(|_| AnyPtr::default())
                    .collect::<Box<[AnyPtr]>>(),
            )),
            count_: Rc::new(RefCell::new(0_usize)),
        }));
        let this: Ptr<v8_base_debug_StackTrace> = __this.as_pointer();
        (*(*this.upgrade().deref()).count_.borrow_mut()) = (({
            let _arg0: Ptr<AnyPtr> = ((*this.upgrade().deref()).trace_.as_pointer() as Ptr<AnyPtr>);
            let ___size: i32 = ((::std::mem::size_of::<[u8; 62]>()) as i32);
            backtrace_5(_arg0, ___size)
        }) as usize);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_debug_StackTrace {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_debug_StackTrace> = Rc::new(RefCell::new(Self {
            trace_: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 62, _>(
                |__i: usize| ((*self.trace_.borrow())[(__i) as usize]).clone(),
            )))),
            count_: Rc::new(RefCell::new((*self.count_.borrow()))),
        }));
        let this: Ptr<v8_base_debug_StackTrace> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_debug_StackTrace {
    fn default() -> Self {
        { v8_base_debug_StackTrace::v8_base_debug_StackTrace1() }
    }
}
impl ByteRepr for v8_base_debug_StackTrace {
    fn byte_size() -> usize {
        504
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.trace_.borrow()).to_bytes(&mut buf[0..496]);
        (*self.count_.borrow()).to_bytes(&mut buf[496..504]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            trace_: Rc::new(RefCell::new(<Box<[AnyPtr]>>::from_bytes(&buf[0..496]))),
            count_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[496..504]))),
        }
    }
}
pub type v8_base_AbortMode = i32;
pub const v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures: v8_base_AbortMode = 0;
pub const v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures: v8_base_AbortMode = 1;
pub const v8_base_AbortMode_kExitIfNoSecurityImpact: v8_base_AbortMode = 2;
pub const v8_base_AbortMode_kImmediateCrash: v8_base_AbortMode = 3;
pub const v8_base_AbortMode_kDefault: v8_base_AbortMode = 4;
thread_local!();
pub fn ControlledCrashesAreHarmless_7() -> bool {
    return ((*g_abort_mode_6.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_6.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn DcheckFailuresAreIgnored_8() -> bool {
    return ((*g_abort_mode_6.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_6.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn FatalErrorsWithNoSecurityImpactShouldExit_9() -> bool {
    return ((*g_abort_mode_6.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitIfNoSecurityImpact);
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
#[derive(Default)]
pub struct v8_base_pass_value_or_ref_unsigned_long__true_ {}
impl Clone for v8_base_pass_value_or_ref_unsigned_long__true_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_pass_value_or_ref_unsigned_long__true_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_pass_value_or_ref_unsigned_long__true_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_pass_value_or_ref_unsigned_long__true_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static kUnimplementedCodeMessage_10: Value<Ptr<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"unimplemented code"),
    ));
);
thread_local!(
    pub static kUnreachableCodeMessage_11: Value<Ptr<u8>> =
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
    pub static is_enum_12: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static is_enum_13: Value<bool> = Rc::new(RefCell::new(false));
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
pub type v8_base_comparison_underlying_type_unsigned_long__Dummy = u32;
thread_local!(
    pub static is_enum_14: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_comparison_underlying_type_unsigned_long_ {}
impl Clone for v8_base_comparison_underlying_type_unsigned_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_comparison_underlying_type_unsigned_long_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_comparison_underlying_type_unsigned_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_comparison_underlying_type_unsigned_long_ {
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
    pub static value_16: Value<bool> = Rc::new(RefCell::new(true));
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
    pub static value_17: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_18: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_19: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_int__unsigned_long_ {}
impl Clone for v8_base_is_signed_vs_unsigned_int__unsigned_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_signed_vs_unsigned_int__unsigned_long_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_signed_vs_unsigned_int__unsigned_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_signed_vs_unsigned_int__unsigned_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static value_20: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_unsigned_long__int_ {}
impl Clone for v8_base_is_signed_vs_unsigned_unsigned_long__int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_signed_vs_unsigned_unsigned_long__int_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_signed_vs_unsigned_unsigned_long__int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_signed_vs_unsigned_unsigned_long__int_ {
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
pub struct v8_base_is_unsigned_vs_signed_int__unsigned_long_ {}
impl Clone for v8_base_is_unsigned_vs_signed_int__unsigned_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_unsigned_vs_signed_int__unsigned_long_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_unsigned_vs_signed_int__unsigned_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_unsigned_vs_signed_int__unsigned_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn CmpEQImpl_21(lhs: i32, rhs: u64) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return ((*lhs.borrow()) >= 0)
        && ((((*lhs.borrow()) as u32) as u64) == ((*rhs.borrow()) as u64));
}
pub fn CmpNEImpl_22(lhs: i32, rhs: u64) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return !({ CmpEQImpl_21((*lhs.borrow()), (*rhs.borrow())) });
}
impl v8_base_Use {
    pub fn v8_base_Use1(_a0: Ptr<bool>) -> Self {
        let __this: Value<v8_base_Use> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_Use> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl v8_base_Use {
    pub fn v8_base_Use2(_a0: Ptr<i64>) -> Self {
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
pub fn make_uint64_23(high: u32, low: u32) -> u64 {
    let high: Value<u32> = Rc::new(RefCell::new(high));
    let low: Value<u32> = Rc::new(RefCell::new(low));
    return (((*high.borrow()) as u64) << 32).wrapping_add(((*low.borrow()) as u64));
}
pub fn RoundDown_24(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    return ((*x.borrow()) & (-(*m.borrow()) as u64));
}
pub fn RoundUp_25(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    (&(0));
    return ({
        let _x: u64 = ((*x.borrow()).wrapping_add((((*m.borrow()) - 1_i64) as u64)) as u64);
        let _m: i64 = (*m.borrow());
        RoundDown_24(_x, _m)
    });
}
pub fn IsAligned_26(value: u64, alignment: u64) -> bool {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let alignment: Value<u64> = Rc::new(RefCell::new(alignment));
    return (((*value.borrow()) & ((*alignment.borrow()).wrapping_sub(1_u64))) == 0_u64);
}
pub fn CountLeadingZeros_27(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        if (32_u32 == 64_u32) {
            (({ __builtin_clzll_28(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((32_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros_29(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        if (64_u32 == 64_u32) {
            (({ __builtin_clzll_28(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((64_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros32_30(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_27((*value.borrow())) });
}
pub fn CountLeadingZeros64_31(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_29((*value.borrow())) });
}
pub fn CountTrailingZeros_32(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        (if (32_u32 == 64_u32) {
            ({ __builtin_ctzll_33(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros_34(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        (if (64_u32 == 64_u32) {
            ({ __builtin_ctzll_33(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros32_35(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_32((*value.borrow())) });
}
pub fn CountTrailingZeros64_36(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_34((*value.borrow())) });
}
pub fn RoundUpToPowerOfTwo32_37(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u32 << ((32_u32).wrapping_sub(({ CountLeadingZeros_27((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo64_38(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u64 << ((64_u32).wrapping_sub(({ CountLeadingZeros_29((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo_39(value: usize) -> usize {
    let value: Value<usize> = Rc::new(RefCell::new(value));
    if (::std::mem::size_of::<usize>() == ::std::mem::size_of::<u64>()) {
        return (({ RoundUpToPowerOfTwo64_38(((*value.borrow()) as u64)) }) as usize);
    } else {
        return (({ RoundUpToPowerOfTwo32_37(((*value.borrow()) as u32)) }) as usize);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn RoundDownToPowerOfTwo32_40(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    if ((*value.borrow()) > 2147483648_u32) {
        return 2147483648_u32;
    }
    let result: Value<u32> = Rc::new(RefCell::new(
        ({ RoundUpToPowerOfTwo32_37((*value.borrow())) }),
    ));
    if ((*result.borrow()) > (*value.borrow())) {
        (*result.borrow_mut()) >>= 1;
    }
    return (*result.borrow());
}
pub fn RotateRight32_41(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateLeft32_42(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateRight64_43(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn RotateLeft64_44(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn ClearLsb_45(value: i32) -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(value));
    return ((*value.borrow()) & ((*value.borrow()) - 1));
}
pub fn SignedAddOverflow32_46(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_sadd_overflow_47(_arg0, _arg1, _arg2)
    });
}
pub fn SignedSubOverflow32_48(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_ssub_overflow_49(_arg0, _arg1, _arg2)
    });
}
pub fn SignedMulOverflow32_50(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_smul_overflow_51(_arg0, _arg1, _arg2)
    });
}
pub fn SignedAddOverflow64_52(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_add_overflow_53(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedSubOverflow64_54(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_sub_overflow_55(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedMulOverflow64_56(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return {
        let (val, ovf) = (*lhs.borrow()).overflowing_mul((*rhs.borrow()));
        (*val.borrow()).write(val);
        ovf
    };
}
pub fn UnsignedAddOverflow32_57(lhs: u32, rhs: u32, val: Ptr<u32>) -> bool {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<u32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: u32 = (*lhs.borrow());
        let _arg1: u32 = (*rhs.borrow());
        let _arg2: Ptr<u32> = (*val.borrow()).clone();
        __builtin_uadd_overflow_58(_arg0, _arg1, _arg2)
    });
}
pub fn UnsignedDiv32_59(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedDiv64_60(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn UnsignedMod32_61(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedMod64_62(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn WraparoundAdd32_63(lhs: i32, rhs: i32) -> i32 {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as u32).wrapping_add(((*rhs.borrow()) as u32))) as i32);
}
pub fn WraparoundNeg32_64(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (-((*x.borrow()) as u32) as i32);
}
pub fn ByteReverse16_65(value: u16) -> u16 {
    let value: Value<u16> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse32_66(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse64_67(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn Malloc_68(size: usize) -> AnyPtr {
    let size: Value<usize> = Rc::new(RefCell::new(size));
    return malloc_refcount((*size.borrow()));
}
pub fn Realloc_69(memory: AnyPtr, size: usize) -> AnyPtr {
    let memory: Value<AnyPtr> = Rc::new(RefCell::new(memory));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let _cmp: Value<bool> = Rc::new(RefCell::new(
            ({ CmpNEImpl_22((0), ((*size.borrow()) as u64)) }),
        ));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if ((!(!(!(*_cmp.borrow()))) as i64) != 0) {
                ({
                    V8_Fatal_70(
                        Ptr::from_string_literal(b"Check failed: %s."),
                        &[(Ptr::from_string_literal(b"0 != size")).into()],
                    )
                });
            }
        }
    }
    return realloc_refcount((*memory.borrow()).clone(), (*size.borrow()));
}
pub fn Free_71(memory: AnyPtr) {
    let memory: Value<AnyPtr> = Rc::new(RefCell::new(memory));
    free_refcount((*memory.borrow()).clone());
    return;
}
pub fn Calloc_72(count: usize, size: usize) -> AnyPtr {
    let count: Value<usize> = Rc::new(RefCell::new(count));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    return calloc_refcount((*count.borrow()), (*size.borrow()));
}
pub fn AlignedAlloc_73(size: usize, alignment: usize) -> AnyPtr {
    let size: Value<usize> = Rc::new(RefCell::new(size));
    let alignment: Value<usize> = Rc::new(RefCell::new(alignment));
    (&(0));
    (&(0));
    let ptr: Value<AnyPtr> = Rc::new(RefCell::new(AnyPtr::default()));
    if (({ posix_memalign_74((ptr.as_pointer()), (*alignment.borrow()), (*size.borrow())) }) != 0) {
        (*ptr.borrow_mut()) = AnyPtr::default();
    }
    return (*ptr.borrow()).clone();
}
pub fn AlignedFree_75(ptr: AnyPtr) {
    let ptr: Value<AnyPtr> = Rc::new(RefCell::new(ptr));
    ({ Free_71((*ptr.borrow()).clone()) });
}
pub fn MallocUsableSize_76(ptr: AnyPtr) -> usize {
    let ptr: Value<AnyPtr> = Rc::new(RefCell::new(ptr));
    return ({ malloc_size_77((*ptr.borrow()).clone()) });
}
#[derive(Default)]
pub struct v8_base_FreeDeleter {}
impl Clone for v8_base_FreeDeleter {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_FreeDeleter> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_FreeDeleter> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_FreeDeleter {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static in_signal_handler_78: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static dump_stack_in_signal_handler_79: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static kMangledSymbolPrefix_80: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::from(*b"_Z\0")));
);
thread_local!(
    pub static kSymbolCharacters_81: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(
        *b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_\0",
    )));
);
pub fn DemangleSymbols_82(text: Ptr<Vec<u8>>) {
    let text: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(text));
    let search_from: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*search_from.borrow());
        _lhs < ((*(*text.borrow()).upgrade().deref()).len() - 1)
    } {
        let mangled_start: Value<usize> = Rc::new(RefCell::new(
            ({
                (*(*text.borrow()).upgrade().deref()).find_Ptru8_usize_const(
                    (kMangledSymbolPrefix_80.with(Value::clone).as_pointer() as Ptr<u8>),
                    Some((*search_from.borrow())),
                )
            }),
        ));
        if ((*mangled_start.borrow()) == (*npos_83.with(Value::clone).borrow())) {
            break;
        }
        let mangled_end: Value<usize> = Rc::new(RefCell::new(
            ({
                (*(*text.borrow()).upgrade().deref()).find_first_not_of_Ptru8_usize_const(
                    (kSymbolCharacters_81.with(Value::clone).as_pointer() as Ptr<u8>),
                    Some((*mangled_start.borrow())),
                )
            }),
        ));
        if ((*mangled_end.borrow()) == (*npos_83.with(Value::clone).borrow())) {
            (*mangled_end.borrow_mut()) = ((*(*text.borrow()).upgrade().deref()).len() - 1);
        }
        let mangled_symbol: Value<Vec<u8>> = Rc::new(RefCell::new({
            let mut __tmp1 = (*(*text.borrow()).upgrade().deref())[(*mangled_start.borrow())
                as usize
                ..::std::cmp::min(
                    (*mangled_start.borrow()).saturating_add(
                        (*mangled_end.borrow()).wrapping_sub((*mangled_start.borrow())),
                    ),
                    (*(*text.borrow()).upgrade().deref())
                        .len()
                        .saturating_sub(1),
                )]
                .to_vec();
            __tmp1.push(0);
            __tmp1
        }));
        let status: Value<i32> = Rc::new(RefCell::new(0));
        let demangled_symbol: Value<Option<Value<u8>>> = Rc::new(RefCell::new(
            std_unique_ptr_char__v8_base_FreeDeleter_::std_unique_ptr_char__v8_base_FreeDeleter_({
                ({
                    __cxa_demangle_84(
                        (mangled_symbol.as_pointer() as Ptr<u8>),
                        Ptr::<u8>::null(),
                        Ptr::<usize>::null(),
                        (status.as_pointer()),
                    )
                })
            }),
        ));
        if ((*status.borrow()) == 0) {
            ({
                let ___pos: usize = (*mangled_start.borrow());
                let ___n: usize = (*mangled_end.borrow()).wrapping_sub((*mangled_start.borrow()));
                (*(*text.borrow()).upgrade().deref()).erase_usize_usize(Some(___pos), Some(___n))
            });
            ({
                (*(*text.borrow()).upgrade().deref()).insert_usize_Ptru8(
                    (*mangled_start.borrow()),
                    ({ (*demangled_symbol.borrow()).get() }),
                )
            });
            (*search_from.borrow_mut()) = (((*mangled_start.borrow()) as u64).wrapping_add(
                (({ (*demangled_symbol.borrow()).get() })
                    .to_c_string_iterator()
                    .count() as u64),
            ) as usize);
        } else {
            (*search_from.borrow_mut()) = (*mangled_start.borrow()).wrapping_add(2_usize);
        }
    }
}
pub trait v8_base_debug_BacktraceOutputHandler {
    fn HandleOutput(&self, output: Ptr<u8>);
    fn OutputFileDescriptor(&self) -> i32 {
        return 0;
    }
}
pub fn OutputPointer_85(
    pointer: AnyPtr,
    handler: PtrDyn<dyn v8_base_debug_BacktraceOutputHandler>,
) {
    let pointer: Value<AnyPtr> = Rc::new(RefCell::new(pointer));
    let handler: Value<PtrDyn<dyn v8_base_debug_BacktraceOutputHandler>> =
        Rc::new(RefCell::new(handler));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ('\0' as u8),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    ({ (*(*handler.borrow()).upgrade().deref()).HandleOutput(Ptr::from_string_literal(b"0x")) });
    ({
        let _buf: Ptr<u8> = (buf.as_pointer() as Ptr<u8>);
        let _sz: usize = ::std::mem::size_of::<[u8; 17]>();
        itoa_r_86((*pointer.borrow()).to_int(), _buf, _sz, 16, 12_usize)
    });
    ({ (*(*handler.borrow()).upgrade().deref()).HandleOutput((buf.as_pointer() as Ptr<u8>)) });
}
pub fn ProcessBacktrace_87(
    trace: Ptr<AnyPtr>,
    size: usize,
    handler: PtrDyn<dyn v8_base_debug_BacktraceOutputHandler>,
) {
    let trace: Value<Ptr<AnyPtr>> = Rc::new(RefCell::new(trace));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    let handler: Value<PtrDyn<dyn v8_base_debug_BacktraceOutputHandler>> =
        Rc::new(RefCell::new(handler));
    ({ (*(*handler.borrow()).upgrade().deref()).HandleOutput(Ptr::from_string_literal(b"\n")) });
    ({
        (*(*handler.borrow()).upgrade().deref()).HandleOutput(Ptr::from_string_literal(
            b"==== C stack trace ===============================\n",
        ))
    });
    ({ (*(*handler.borrow()).upgrade().deref()).HandleOutput(Ptr::from_string_literal(b"\n")) });
    let printed: Value<bool> = Rc::new(RefCell::new(false));
    if ((*in_signal_handler_78.with(Value::clone).borrow()) == 0) {
        let trace_symbols : Value<Option<Value<Ptr::<u8>>> > = Rc::new(RefCell::new(std_unique_ptr_char_ptr__v8_base_FreeDeleter_ :: std_unique_ptr_char_ptr__v8_base_FreeDeleter_ ( {  (  { backtrace_symbols_88 ( ((*trace.borrow()) ).clone() , ( ( (*size.borrow()) as i32 ) )  , ) } )    } , ) )) ;
        if ({ (*trace_symbols.borrow())() }) {
            let i: Value<usize> = Rc::new(RefCell::new(0_usize));
            'loop_: while ((*i.borrow()) < (*size.borrow())) {
                let trace_symbol: Value<Vec<u8>> = Rc::new(RefCell::new(
                    (({ (*trace_symbols.borrow()).get() })
                        .offset((*i.borrow()) as isize)
                        .read())
                    .to_c_string_iterator()
                    .chain(std::iter::once(0))
                    .collect::<Vec<u8>>(),
                ));
                ({ DemangleSymbols_82((trace_symbol.as_pointer())) });
                ({
                    (*(*handler.borrow()).upgrade().deref())
                        .HandleOutput(Ptr::from_string_literal(b"    "))
                });
                ({
                    (*(*handler.borrow()).upgrade().deref())
                        .HandleOutput((trace_symbol.as_pointer() as Ptr<u8>))
                });
                ({
                    (*(*handler.borrow()).upgrade().deref())
                        .HandleOutput(Ptr::from_string_literal(b"\n"))
                });
                (*i.borrow_mut()).prefix_inc();
            }
            (*printed.borrow_mut()) = true;
        }
    } else if (({ (*(*handler.borrow()).upgrade().deref()).OutputFileDescriptor() }) != 0) {
        ({
            backtrace_symbols_fd_89(
                (*trace.borrow()).clone(),
                ((*size.borrow()) as i32),
                ({ (*(*handler.borrow()).upgrade().deref()).OutputFileDescriptor() }),
            )
        });
        (*printed.borrow_mut()) = true;
    }
    if !(*printed.borrow()) {
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*size.borrow())) {
            ({
                (*(*handler.borrow()).upgrade().deref())
                    .HandleOutput(Ptr::from_string_literal(b" ["))
            });
            ({
                let _pointer: AnyPtr =
                    ((*trace.borrow()).offset((*i.borrow()) as isize).read()).clone();
                let _handler: PtrDyn<dyn v8_base_debug_BacktraceOutputHandler> =
                    (*handler.borrow()).clone();
                OutputPointer_85(_pointer, _handler)
            });
            ({
                (*(*handler.borrow()).upgrade().deref())
                    .HandleOutput(Ptr::from_string_literal(b"]\n"))
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
}
pub fn PrintToStderr_90(output: Ptr<u8>) {
    let output: Value<Ptr<u8>> = Rc::new(RefCell::new(output));
    let return_val: Value<isize> = Rc::new(RefCell::new(
        match FdRegistry::with_fd(2, |__fd| {
            ((*output.borrow()).clone() as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice((*output.borrow()).to_c_string_iterator().count(), |__buf| {
                    nix::unistd::write(__fd, __buf)
                })
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                cpp2rust_errno().write(__e as i32);
                -1
            }
        },
    ));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let unused_tmp_array_for_use_macro: Value<Box<[v8_base_Use]>> =
            Rc::new(RefCell::new(Box::new([v8_base_Use::v8_base_Use2({
                return_val.as_pointer()
            })])));
        &(*unused_tmp_array_for_use_macro.borrow_mut());
    }
}
pub fn StackDumpSignalHandler_91(signal: i32, info: Ptr<__siginfo>, void_context: AnyPtr) {
    let signal: Value<i32> = Rc::new(RefCell::new(signal));
    let info: Value<Ptr<__siginfo>> = Rc::new(RefCell::new(info));
    let void_context: Value<AnyPtr> = Rc::new(RefCell::new(void_context));
    (*in_signal_handler_78.with(Value::clone).borrow_mut()) = 1;
    ({ PrintToStderr_90(Ptr::from_string_literal(b"Received signal ")) });
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    ({
        let _buf: Ptr<u8> = (buf.as_pointer() as Ptr<u8>);
        let _sz: usize = ::std::mem::size_of::<[u8; 1024]>();
        itoa_r_86(((*signal.borrow()) as i64), _buf, _sz, 10, 0_usize)
    });
    ({ PrintToStderr_90((buf.as_pointer() as Ptr<u8>)) });
    if ((*signal.borrow()) == 10) {
        if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 1) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" BUS_ADRALN ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 2) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" BUS_ADRERR ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 3) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" BUS_OBJERR ")) });
        } else {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" <unknown> ")) });
        }
    } else if ((*signal.borrow()) == 8) {
        if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 1) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" FPE_FLTDIV ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 5) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" FPE_FLTINV ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 2) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" FPE_FLTOVF ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 4) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" FPE_FLTRES ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 6) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" FPE_FLTSUB ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 3) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" FPE_FLTUND ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 7) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" FPE_INTDIV ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 8) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" FPE_INTOVF ")) });
        } else {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" <unknown> ")) });
        }
    } else if ((*signal.borrow()) == 4) {
        if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 8) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" ILL_BADSTK ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 7) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" ILL_COPROC ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 4) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" ILL_ILLOPN ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 5) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" ILL_ILLADR ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 2) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" ILL_ILLTRP ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 3) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" ILL_PRVOPC ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 6) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" ILL_PRVREG ")) });
        } else {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" <unknown> ")) });
        }
    } else if ((*signal.borrow()) == 11) {
        if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 1) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" SEGV_MAPERR ")) });
        } else if ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) == 2) {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" SEGV_ACCERR ")) });
        } else {
            ({ PrintToStderr_90(Ptr::from_string_literal(b" <unknown> ")) });
        }
    }
    if ((((*signal.borrow()) == 10) || ((*signal.borrow()) == 8)) || ((*signal.borrow()) == 4))
        || ((*signal.borrow()) == 11)
    {
        ({
            let _buf: Ptr<u8> = (buf.as_pointer() as Ptr<u8>);
            let _sz: usize = ::std::mem::size_of::<[u8; 1024]>();
            itoa_r_86(
                (*(*(*info.borrow()).upgrade().deref()).si_addr.borrow()).to_int(),
                _buf,
                _sz,
                16,
                12_usize,
            )
        });
        ({ PrintToStderr_90((buf.as_pointer() as Ptr<u8>)) });
    }
    ({ PrintToStderr_90(Ptr::from_string_literal(b"\n")) });
    if (*dump_stack_in_signal_handler_79.with(Value::clone).borrow()) {
        ({
            v8_base_debug_StackTraceImpl::Print(
                &Rc::new(RefCell::new(
                    v8_base_debug_StackTrace::v8_base_debug_StackTrace1(),
                ))
                .as_pointer(),
            )
        });
        ({ PrintToStderr_90(Ptr::from_string_literal(b"[end of stack trace]\n")) });
    }
    if (({
        signal_92(
            (*signal.borrow()),
            FnPtr::<fn(i32)>::null().cast::<fn(i32)>(None),
        )
    }) == (<FnPtr<fn(i32)>>::from_int(-1_i32)))
    {
        ({ _exit_93(1) });
    }
}
#[derive(Default)]
pub struct v8_base_debug_PrintBacktraceOutputHandler {}
impl v8_base_debug_BacktraceOutputHandler for v8_base_debug_PrintBacktraceOutputHandler {
    fn HandleOutput(&self, output: Ptr<u8>) {
        let output: Value<Ptr<u8>> = Rc::new(RefCell::new(output));
        ({ PrintToStderr_90((*output.borrow()).clone()) });
    }
    fn OutputFileDescriptor(&self) -> i32 {
        return 2;
    }
}
impl ByteRepr for v8_base_debug_PrintBacktraceOutputHandler {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_debug_StreamBacktraceOutputHandler {
    os_: Value<Ptr<std::fs::File>>,
}
impl v8_base_debug_StreamBacktraceOutputHandler {
    pub fn v8_base_debug_StreamBacktraceOutputHandler(os: Ptr<std::fs::File>) -> Self {
        let os: Value<Ptr<std::fs::File>> = Rc::new(RefCell::new(os));
        let __this: Value<v8_base_debug_StreamBacktraceOutputHandler> =
            Rc::new(RefCell::new(Self {
                os_: Rc::new(RefCell::new((*os.borrow()).clone())),
            }));
        let this: Ptr<v8_base_debug_StreamBacktraceOutputHandler> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl v8_base_debug_BacktraceOutputHandler for v8_base_debug_StreamBacktraceOutputHandler {
    fn HandleOutput(&self, output: Ptr<u8>) {
        let output: Value<Ptr<u8>> = Rc::new(RefCell::new(output));
        write!((*self.os_.borrow()), "{:}", (*output.borrow()),);
    }
}
impl ByteRepr for v8_base_debug_StreamBacktraceOutputHandler {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.os_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            os_: Rc::new(RefCell::new(<Ptr<std::fs::File>>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn WarmUpBacktrace_94() {
    let stack_trace: Value<v8_base_debug_StackTrace> = Rc::new(RefCell::new(
        v8_base_debug_StackTrace::v8_base_debug_StackTrace1(),
    ));
}
pub fn EnableInProcessStackDumping_95() -> bool {
    let sigpipe_action: Value<sigaction> = Rc::new(RefCell::new(<sigaction>::default()));
    {
        ((sigpipe_action.as_pointer()) as Ptr<sigaction>)
            .to_any()
            .memset((0) as u8, 16usize as usize);
        ((sigpipe_action.as_pointer()) as Ptr<sigaction>).to_any()
    };
    (*(*sigpipe_action.borrow()).__sigaction_u.borrow_mut())
        .__sa_handler()
        .write(<FnPtr<fn(i32)>>::from_int(1));
    {
        ((*sigpipe_action.borrow()).sa_mask.as_pointer()).write(0_u32);
        0
    };
    let success: Value<bool> = Rc::new(RefCell::new(
        (libc::sigaction(13, (sigpipe_action.as_pointer()), Ptr::<sigaction>::null()) == 0),
    ));
    ({ WarmUpBacktrace_94() });
    let action: Value<sigaction> = Rc::new(RefCell::new(<sigaction>::default()));
    {
        ((action.as_pointer()) as Ptr<sigaction>)
            .to_any()
            .memset((0) as u8, 16usize as usize);
        ((action.as_pointer()) as Ptr<sigaction>).to_any()
    };
    (*(*action.borrow()).sa_flags.borrow_mut()) = ((4 | 64) | 1);
    (*(*action.borrow()).__sigaction_u.borrow_mut())
        .__sa_sigaction()
        .write((FnPtr::<fn(i32, Ptr<__siginfo>, AnyPtr)>::new(StackDumpSignalHandler_91)));
    {
        ((*action.borrow()).sa_mask.as_pointer()).write(0_u32);
        0
    };
    {
        let rhs_0 = (((*success.borrow()) as i32)
            & ((libc::sigaction(4, (action.as_pointer()), Ptr::<sigaction>::null()) == 0) as i32))
            != 0;
        (*success.borrow_mut()) = rhs_0
    };
    {
        let rhs_0 = (((*success.borrow()) as i32)
            & ((libc::sigaction(6, (action.as_pointer()), Ptr::<sigaction>::null()) == 0) as i32))
            != 0;
        (*success.borrow_mut()) = rhs_0
    };
    {
        let rhs_0 = (((*success.borrow()) as i32)
            & ((libc::sigaction(8, (action.as_pointer()), Ptr::<sigaction>::null()) == 0) as i32))
            != 0;
        (*success.borrow_mut()) = rhs_0
    };
    {
        let rhs_0 = (((*success.borrow()) as i32)
            & ((libc::sigaction(10, (action.as_pointer()), Ptr::<sigaction>::null()) == 0) as i32))
            != 0;
        (*success.borrow_mut()) = rhs_0
    };
    {
        let rhs_0 = (((*success.borrow()) as i32)
            & ((libc::sigaction(11, (action.as_pointer()), Ptr::<sigaction>::null()) == 0) as i32))
            != 0;
        (*success.borrow_mut()) = rhs_0
    };
    {
        let rhs_0 = (((*success.borrow()) as i32)
            & ((libc::sigaction(12, (action.as_pointer()), Ptr::<sigaction>::null()) == 0) as i32))
            != 0;
        (*success.borrow_mut()) = rhs_0
    };
    (*dump_stack_in_signal_handler_79
        .with(Value::clone)
        .borrow_mut()) = true;
    return (*success.borrow());
}
pub fn DisableSignalStackDump_96() {
    (*dump_stack_in_signal_handler_79
        .with(Value::clone)
        .borrow_mut()) = false;
}
pub fn itoa_r_86(i: i64, buf: Ptr<u8>, sz: usize, base: i32, padding: usize) -> Ptr<u8> {
    let i: Value<i64> = Rc::new(RefCell::new(i));
    let buf: Value<Ptr<u8>> = Rc::new(RefCell::new(buf));
    let sz: Value<usize> = Rc::new(RefCell::new(sz));
    let base: Value<i32> = Rc::new(RefCell::new(base));
    let padding: Value<usize> = Rc::new(RefCell::new(padding));
    let n: Value<usize> = Rc::new(RefCell::new(1_usize));
    if ((*n.borrow()) > (*sz.borrow())) {
        return Ptr::<u8>::null();
    }
    if ((*base.borrow()) < 2) || ((*base.borrow()) > 16) {
        (*buf.borrow()).offset((0) as isize).write(('\0' as u8));
        return Ptr::<u8>::null();
    }
    let start: Value<Ptr<u8>> = Rc::new(RefCell::new((*buf.borrow()).clone()));
    let j: Value<u64> = Rc::new(RefCell::new(((*i.borrow()) as u64)));
    if ((*i.borrow()) < 0_i64) && ((*base.borrow()) == 10) {
        (*j.borrow_mut()) = (-((*i.borrow()) + 1_i64) as u64).wrapping_add(1_u64);
        if ((*n.borrow_mut()).prefix_inc() > (*sz.borrow())) {
            (*buf.borrow()).offset((0) as isize).write(('\0' as u8));
            return Ptr::<u8>::null();
        }
        (*start.borrow_mut()).postfix_inc().write(('-' as u8));
    }
    let ptr: Value<Ptr<u8>> = Rc::new(RefCell::new((*start.borrow()).clone()));
    let mut __do_while = true;
    'loop_: while __do_while || (((*j.borrow()) > 0_u64) || ((*padding.borrow()) > 0_usize)) {
        __do_while = false;
        if ((*n.borrow_mut()).prefix_inc() > (*sz.borrow())) {
            (*buf.borrow()).offset((0) as isize).write(('\0' as u8));
            return Ptr::<u8>::null();
        }
        let __rhs =
            b"0123456789abcdef"[((*j.borrow()).wrapping_rem(((*base.borrow()) as u64))) as usize];
        (*ptr.borrow_mut()).postfix_inc().write(__rhs);
        {
            let rhs_0 = (*j.borrow()).wrapping_div(((*base.borrow()) as u64));
            (*j.borrow_mut()) = rhs_0
        };
        if ((*padding.borrow()) > 0_usize) {
            (*padding.borrow_mut()).postfix_dec();
        }
    }
    (*ptr.borrow()).write(('\0' as u8));
    'loop_: while {
        let _lhs = (*ptr.borrow_mut()).prefix_dec();
        _lhs > (*start.borrow()).clone()
    } {
        let ch: Value<u8> = Rc::new(RefCell::new(((*ptr.borrow()).read())));
        let __rhs = ((*start.borrow()).read());
        (*ptr.borrow()).write(__rhs);
        let __rhs = (*ch.borrow());
        (*start.borrow_mut()).postfix_inc().write(__rhs);
    }
    return (*buf.borrow()).clone();
}
pub trait v8_base_FreeDeleterImpl {
    fn operator_call(&self, ptr: AnyPtr);
}
impl v8_base_FreeDeleterImpl for Ptr<v8_base_FreeDeleter> {
    fn operator_call(&self, ptr: AnyPtr) {
        let ptr: Value<AnyPtr> = Rc::new(RefCell::new(ptr));
        ({ Free_71((*ptr.borrow()).clone()) });
    }
}
pub trait v8_base_debug_StackTraceImpl {
    fn Print(&self);
    fn OutputToStream(&self, os: Ptr<std::fs::File>);
}
impl v8_base_debug_StackTraceImpl for Ptr<v8_base_debug_StackTrace> {
    fn Print(&self) {
        let handler: Value<v8_base_debug_PrintBacktraceOutputHandler> = Rc::new(RefCell::new(
            <v8_base_debug_PrintBacktraceOutputHandler>::default(),
        ));
        ({
            let _trace: Ptr<AnyPtr> =
                ((*(*self).upgrade().deref()).trace_.as_pointer() as Ptr<AnyPtr>);
            let _size: usize = (*(*(*self).upgrade().deref()).count_.borrow());
            ProcessBacktrace_87(
                _trace,
                _size,
                ((handler.as_pointer()).to_strong()
                    as Value<dyn v8_base_debug_BacktraceOutputHandler>)
                    .as_pointer_dyn(),
            )
        });
    }
    fn OutputToStream(&self, os: Ptr<std::fs::File>) {
        let os: Value<Ptr<std::fs::File>> = Rc::new(RefCell::new(os));
        let handler: Value<v8_base_debug_StreamBacktraceOutputHandler> = Rc::new(RefCell::new(
            v8_base_debug_StreamBacktraceOutputHandler::v8_base_debug_StreamBacktraceOutputHandler(
                { (*os.borrow()).clone() },
            ),
        ));
        ({
            let _trace: Ptr<AnyPtr> =
                ((*(*self).upgrade().deref()).trace_.as_pointer() as Ptr<AnyPtr>);
            let _size: usize = (*(*(*self).upgrade().deref()).count_.borrow());
            ProcessBacktrace_87(
                _trace,
                _size,
                ((handler.as_pointer()).to_strong()
                    as Value<dyn v8_base_debug_BacktraceOutputHandler>)
                    .as_pointer_dyn(),
            )
        });
    }
}
