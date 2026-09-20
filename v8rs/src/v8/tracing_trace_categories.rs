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
pub fn CountLeadingZeros_20(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        if (32_u32 == 64_u32) {
            (({ __builtin_clzll_21(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((32_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros_22(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        if (64_u32 == 64_u32) {
            (({ __builtin_clzll_21(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((64_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros32_23(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_20((*value.borrow())) });
}
pub fn CountLeadingZeros64_24(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_22((*value.borrow())) });
}
pub fn CountTrailingZeros_25(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        (if (32_u32 == 64_u32) {
            ({ __builtin_ctzll_26(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros_27(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        (if (64_u32 == 64_u32) {
            ({ __builtin_ctzll_26(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros32_28(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_25((*value.borrow())) });
}
pub fn CountTrailingZeros64_29(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_27((*value.borrow())) });
}
pub fn RoundUpToPowerOfTwo32_30(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u32 << ((32_u32).wrapping_sub(({ CountLeadingZeros_20((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo64_31(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u64 << ((64_u32).wrapping_sub(({ CountLeadingZeros_22((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo_32(value: usize) -> usize {
    let value: Value<usize> = Rc::new(RefCell::new(value));
    if (::std::mem::size_of::<usize>() == ::std::mem::size_of::<u64>()) {
        return (({ RoundUpToPowerOfTwo64_31(((*value.borrow()) as u64)) }) as usize);
    } else {
        return (({ RoundUpToPowerOfTwo32_30(((*value.borrow()) as u32)) }) as usize);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn RoundDownToPowerOfTwo32_33(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    if ((*value.borrow()) > 2147483648_u32) {
        return 2147483648_u32;
    }
    let result: Value<u32> = Rc::new(RefCell::new(
        ({ RoundUpToPowerOfTwo32_30((*value.borrow())) }),
    ));
    if ((*result.borrow()) > (*value.borrow())) {
        (*result.borrow_mut()) >>= 1;
    }
    return (*result.borrow());
}
pub fn RotateRight32_34(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateLeft32_35(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateRight64_36(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn RotateLeft64_37(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn ClearLsb_38(value: i32) -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(value));
    return ((*value.borrow()) & ((*value.borrow()) - 1));
}
pub fn SignedAddOverflow32_39(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_sadd_overflow_40(_arg0, _arg1, _arg2)
    });
}
pub fn SignedSubOverflow32_41(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_ssub_overflow_42(_arg0, _arg1, _arg2)
    });
}
pub fn SignedMulOverflow32_43(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_smul_overflow_44(_arg0, _arg1, _arg2)
    });
}
pub fn SignedAddOverflow64_45(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_add_overflow_46(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedSubOverflow64_47(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_sub_overflow_48(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedMulOverflow64_49(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return {
        let (val, ovf) = (*lhs.borrow()).overflowing_mul((*rhs.borrow()));
        (*val.borrow()).write(val);
        ovf
    };
}
pub fn UnsignedAddOverflow32_50(lhs: u32, rhs: u32, val: Ptr<u32>) -> bool {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<u32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: u32 = (*lhs.borrow());
        let _arg1: u32 = (*rhs.borrow());
        let _arg2: Ptr<u32> = (*val.borrow()).clone();
        __builtin_uadd_overflow_51(_arg0, _arg1, _arg2)
    });
}
pub fn UnsignedDiv32_52(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedDiv64_53(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn UnsignedMod32_54(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedMod64_55(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn WraparoundAdd32_56(lhs: i32, rhs: i32) -> i32 {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as u32).wrapping_add(((*rhs.borrow()) as u32))) as i32);
}
pub fn WraparoundNeg32_57(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (-((*x.borrow()) as u32) as i32);
}
pub fn ByteReverse16_58(value: u16) -> u16 {
    let value: Value<u16> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse32_59(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse64_60(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
thread_local!(
    pub static kMaxExponent_61: Value<i32> = Rc::new(RefCell::new(64));
);
thread_local!(
    pub static kMaxExponent_62: Value<i32> = Rc::new(RefCell::new(1024));
);
thread_local!(
    pub static kIntegerBitsPlusSign_63: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kIntegerBitsPlusSign_64: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kIntegerBitsPlusSign_65: Value<i32> = Rc::new(RefCell::new(16));
);
thread_local!(
    pub static kIntegerBitsPlusSign_66: Value<i32> = Rc::new(RefCell::new(16));
);
thread_local!(
    pub static kIntegerBitsPlusSign_67: Value<i32> = Rc::new(RefCell::new(32));
);
thread_local!(
    pub static kIntegerBitsPlusSign_68: Value<i32> = Rc::new(RefCell::new(32));
);
thread_local!(
    pub static kIntegerBitsPlusSign_69: Value<i32> = Rc::new(RefCell::new(64));
);
thread_local!(
    pub static kIntegerBitsPlusSign_70: Value<i32> = Rc::new(RefCell::new(64));
);
thread_local!(
    pub static kIntegerBitsPlusSign_71: Value<i32> = Rc::new(RefCell::new(64));
);
pub fn IsValueNegative_72(value: i64) -> bool {
    let value: Value<i64> = Rc::new(RefCell::new(value));
    if true {
        return ((*value.borrow()) < 0_i64);
    } else {
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ConditionalNegate_73(x: u64, is_negative: bool) -> i64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let is_negative: Value<bool> = Rc::new(RefCell::new(is_negative));
    return (((((*x.borrow()) as u64) ^ (-((*is_negative.borrow()) as i64) as u64))
        .wrapping_add(((*is_negative.borrow()) as u64))) as i64);
}
pub fn SafeUnsignedAbs_74(value: i64) -> u64 {
    let value: Value<i64> = Rc::new(RefCell::new(value));
    return if ({ IsValueNegative_72((*value.borrow())) }) {
        (0_u64).wrapping_sub(((*value.borrow()) as u64))
    } else {
        ((*value.borrow()) as u64)
    };
}
thread_local!(
    pub static kEnableAsmCode_75: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Default)]
pub struct v8_base_internal_CheckOnFailure {}
impl v8_base_internal_CheckOnFailure {
    pub fn HandleFailure_long_long() -> i64 {
        panic!("builtin trap");
        return <i64>::default();
    }
}
impl Clone for v8_base_internal_CheckOnFailure {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_CheckOnFailure> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_CheckOnFailure> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_CheckOnFailure {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub type v8_base_internal_IntegerRepresentation = i32;
pub const v8_base_internal_IntegerRepresentation_kUnsigned: v8_base_internal_IntegerRepresentation =
    0;
pub const v8_base_internal_IntegerRepresentation_kSigned: v8_base_internal_IntegerRepresentation =
    1;
pub type v8_base_internal_NumericRangeRepresentation = i32;
pub const v8_base_internal_NumericRangeRepresentation_kNotContained:
    v8_base_internal_NumericRangeRepresentation = 0;
pub const v8_base_internal_NumericRangeRepresentation_kContained:
    v8_base_internal_NumericRangeRepresentation = 1;
thread_local!(
    pub static kStaticDstRangeRelationToSrcRange_76: Value<
        v8_base_internal_NumericRangeRepresentation,
    > = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kStaticDstRangeRelationToSrcRange_78: Value<
        v8_base_internal_NumericRangeRepresentation,
    > = Rc::new(RefCell::new(0));
);
#[derive(Default)]
pub struct v8_base_internal_RangeCheck {
    is_underflow_: Value<bool>,
    is_overflow_: Value<bool>,
}
impl v8_base_internal_RangeCheck {
    pub fn v8_base_internal_RangeCheck1(is_in_lower_bound: bool, is_in_upper_bound: bool) -> Self {
        let is_in_lower_bound: Value<bool> = Rc::new(RefCell::new(is_in_lower_bound));
        let is_in_upper_bound: Value<bool> = Rc::new(RefCell::new(is_in_upper_bound));
        let __this: Value<v8_base_internal_RangeCheck> = Rc::new(RefCell::new(Self {
            is_underflow_: Rc::new(RefCell::new(!(*is_in_lower_bound.borrow()))),
            is_overflow_: Rc::new(RefCell::new(!(*is_in_upper_bound.borrow()))),
        }));
        let this: Ptr<v8_base_internal_RangeCheck> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for v8_base_internal_RangeCheck {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_internal_RangeCheckImpl::operator_eq(
                &Rc::new(RefCell::new(v8_base_internal_RangeCheck {
                    is_underflow_: self.is_underflow_.clone(),
                    is_overflow_: self.is_overflow_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(v8_base_internal_RangeCheck {
                    is_underflow_: other.is_underflow_.clone(),
                    is_overflow_: other.is_overflow_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for v8_base_internal_RangeCheck {}
impl Clone for v8_base_internal_RangeCheck {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_RangeCheck> = Rc::new(RefCell::new(Self {
            is_underflow_: Rc::new(RefCell::new((*self.is_underflow_.borrow()))),
            is_overflow_: Rc::new(RefCell::new((*self.is_overflow_.borrow()))),
        }));
        let this: Ptr<v8_base_internal_RangeCheck> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_RangeCheck {
    fn byte_size() -> usize {
        2
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.is_underflow_.borrow()).to_bytes(&mut buf[0..1]);
        (*self.is_overflow_.borrow()).to_bytes(&mut buf[1..2]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            is_underflow_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[0..1]))),
            is_overflow_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[1..2]))),
        }
    }
}
thread_local!(
    pub static kShift_79: Value<i32> = Rc::new(RefCell::new(10));
);
#[derive(Default)]
pub struct v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_
{}
impl v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_ {
    pub fn max() -> i64 {
        return ({
            v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_::Adjust ( <i64>::MAX   , )
        });
    }
    pub fn lowest() -> i64 {
        return ({
            v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_::Adjust ( (  { std_numeric_limits_long_long_::lowest ( ) } )   , )
        });
    }
    pub fn Adjust_i64__long_long(value: i64) -> i64 {
        let value: Value<i64> = Rc::new(RefCell::new(value));
        if true {
            return (({
                let _x: u64 = (({ SafeUnsignedAbs_74((*value.borrow())) })
                    & !(((1_u64 << 10) as u64).wrapping_sub((1_u64 as u64))));
                let _is_negative: bool = ({ IsValueNegative_72((*value.borrow())) });
                ConditionalNegate_73(_x, _is_negative)
            }) as i64);
        } else {
        }
        panic!("ub: non-void function does not return a value")
    }
}
impl Clone
    for v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_
{
    fn clone(&self) -> Self {
        let __this : Value<v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_> = Rc::new(RefCell::new(Self { } )) ;
        let this : Ptr<v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_> = __this.as_pointer() ;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr
    for v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_
{
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_internal_DstRangeRelationToSrcRangeImpl_long_long__double__v8_base_internal_SaturationDefaultLimits__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_NumericRangeRepresentation_kNotContained_
{}
impl v8_base_internal_DstRangeRelationToSrcRangeImpl_long_long__double__v8_base_internal_SaturationDefaultLimits__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_NumericRangeRepresentation_kNotContained_ { pub  fn Check ( value : f64  , ) -> v8_base_internal_RangeCheck  { let value : Value<f64 >  = Rc::new(RefCell::new(value)) ;
 ;
 ;
  return v8_base_internal_RangeCheck :: v8_base_internal_RangeCheck1 ( {  ( (*value.borrow()) >= ( ( (  { v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_::lowest ( ) } )  as f64 ) ) )   } , {  ( (*value.borrow()) <= ( ( (  { v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_::max ( ) } )  as f64 ) ) )   } , )   ;
 } }
impl Clone for v8_base_internal_DstRangeRelationToSrcRangeImpl_long_long__double__v8_base_internal_SaturationDefaultLimits__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_NumericRangeRepresentation_kNotContained_ { fn clone(&self) -> Self { let __this : Value<v8_base_internal_DstRangeRelationToSrcRangeImpl_long_long__double__v8_base_internal_SaturationDefaultLimits__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_NumericRangeRepresentation_kNotContained_> = Rc::new(RefCell::new(Self { } )) ;
 let this : Ptr<v8_base_internal_DstRangeRelationToSrcRangeImpl_long_long__double__v8_base_internal_SaturationDefaultLimits__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_NumericRangeRepresentation_kNotContained_> = __this.as_pointer() ;
 Rc::try_unwrap(__this).ok().unwrap().into_inner() } }
impl ByteRepr for  v8_base_internal_DstRangeRelationToSrcRangeImpl_long_long__double__v8_base_internal_SaturationDefaultLimits__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_NumericRangeRepresentation_kNotContained_ { fn byte_size() -> usize { 1 } fn to_bytes(&self, buf: &mut [u8]) { } fn from_bytes(buf: &[u8]) -> Self { Self { } } }
pub fn DstRangeRelationToSrcRange_80(value: f64) -> v8_base_internal_RangeCheck {
    let value: Value<f64> = Rc::new(RefCell::new(value));
    return ({
        v8_base_internal_DstRangeRelationToSrcRangeImpl_long_long__double__v8_base_internal_SaturationDefaultLimits__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_NumericRangeRepresentation_kNotContained_::Check ( (*value.borrow())  , )
    });
}
#[derive(Default)]
pub struct v8_base_internal_IntegerForDigitsAndSignImpl____true_ {}
impl Clone for v8_base_internal_IntegerForDigitsAndSignImpl____true_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_IntegerForDigitsAndSignImpl____true_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_IntegerForDigitsAndSignImpl____true_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_IntegerForDigitsAndSignImpl____true_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_internal_IntegerForDigitsAndSignImpl____false_ {}
impl Clone for v8_base_internal_IntegerForDigitsAndSignImpl____false_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_IntegerForDigitsAndSignImpl____false_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_IntegerForDigitsAndSignImpl____false_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_IntegerForDigitsAndSignImpl____false_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_internal_ArithmeticOrIntegralConstant_double_ {}
impl Clone for v8_base_internal_ArithmeticOrIntegralConstant_double_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_ArithmeticOrIntegralConstant_double_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_ArithmeticOrIntegralConstant_double_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_ArithmeticOrIntegralConstant_double_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static kIsCheckedNumeric_81: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static kIsClampedNumeric_82: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static kIsStrictNumeric_83: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Default)]
pub struct v8_base_internal_UnderlyingTypeImpl_double_ {}
impl Clone for v8_base_internal_UnderlyingTypeImpl_double_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_UnderlyingTypeImpl_double_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_UnderlyingTypeImpl_double_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_UnderlyingTypeImpl_double_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static kIsNumeric_84: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Default)]
pub struct v8_base_internal_SaturationDefaultLimits_long_long_ {}
impl v8_base_internal_SaturationDefaultLimits_long_long_ {
    pub fn NaN() -> i64 {
        if false {
        } else {
            return <i64>::default();
        }
        panic!("ub: non-void function does not return a value")
    }
    pub fn Overflow() -> i64 {
        if false {
        } else {
            return <i64>::MAX;
        }
        panic!("ub: non-void function does not return a value")
    }
    pub fn Underflow() -> i64 {
        if false {
        } else {
            return ({ std_numeric_limits_long_long_::lowest() });
        }
        panic!("ub: non-void function does not return a value")
    }
}
impl Clone for v8_base_internal_SaturationDefaultLimits_long_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_SaturationDefaultLimits_long_long_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_SaturationDefaultLimits_long_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_SaturationDefaultLimits_long_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn saturated_cast_impl_85(value: f64, constraint: v8_base_internal_RangeCheck) -> i64 {
    let value: Value<f64> = Rc::new(RefCell::new(value));
    let constraint: Value<v8_base_internal_RangeCheck> = Rc::new(RefCell::new(constraint));
    return if !({ v8_base_internal_RangeCheckImpl::IsOverflowFlagSet(&constraint.as_pointer()) }) {
        (if !({ v8_base_internal_RangeCheckImpl::IsUnderflowFlagSet(&constraint.as_pointer()) }) {
            ((*value.borrow()) as i64)
        } else {
            ({ v8_base_internal_SaturationDefaultLimits_long_long_::Underflow() })
        })
    } else {
        (if (false)
            || (!({
                v8_base_internal_RangeCheckImpl::IsUnderflowFlagSet(&constraint.as_pointer())
            }))
        {
            ({ v8_base_internal_SaturationDefaultLimits_long_long_::Overflow() })
        } else {
            ({ v8_base_internal_SaturationDefaultLimits_long_long_::NaN() })
        })
    };
}
thread_local!(
    pub static is_supported_86: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_internal_SaturateFastOp_long_long__double_ {}
impl v8_base_internal_SaturateFastOp_long_long__double_ {
    pub fn Do(_a0: f64) -> i64 {
        let _a0: Value<f64> = Rc::new(RefCell::new(_a0));
        return ({ v8_base_internal_CheckOnFailure::HandleFailure() });
    }
}
impl Clone for v8_base_internal_SaturateFastOp_long_long__double_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_SaturateFastOp_long_long__double_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_SaturateFastOp_long_long__double_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_SaturateFastOp_long_long__double_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn saturated_cast_87(value: f64) -> i64 {
    let value: Value<f64> = Rc::new(RefCell::new(value));
    let underlying_value: Value<f64> = Rc::new(RefCell::new(((*value.borrow()) as f64)));
    return if ((!({ is_constant_evaluated_88() })) && (false)) && (true) {
        ({ v8_base_internal_SaturateFastOp_long_long__double_::Do((*underlying_value.borrow())) })
    } else {
        ({
            let _value: f64 = (*underlying_value.borrow());
            let _constraint: v8_base_internal_RangeCheck =
                ({ DstRangeRelationToSrcRange_80((*underlying_value.borrow())) });
            saturated_cast_impl_85(_value, _constraint)
        })
    };
}
#[derive(Default)]
pub struct v8_base_time_internal_TimeBase_v8_base_Time_ {
    us_: Value<i64>,
}
impl v8_base_time_internal_TimeBase_v8_base_Time_ {
    fn v8_base_time_internal_TimeBase_v8_base_Time_(us: i64) -> Self {
        let us: Value<i64> = Rc::new(RefCell::new(us));
        let __this: Value<v8_base_time_internal_TimeBase_v8_base_Time_> =
            Rc::new(RefCell::new(Self {
                us_: Rc::new(RefCell::new((*us.borrow()))),
            }));
        let this: Ptr<v8_base_time_internal_TimeBase_v8_base_Time_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for v8_base_time_internal_TimeBase_v8_base_Time_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            v8_base_time_internal_TimeBase_v8_base_Time_Impl::operator_cmp(
                &Rc::new(RefCell::new(v8_base_time_internal_TimeBase_v8_base_Time_ {
                    us_: self.us_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(v8_base_time_internal_TimeBase_v8_base_Time_ {
                    us_: other.us_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for v8_base_time_internal_TimeBase_v8_base_Time_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for v8_base_time_internal_TimeBase_v8_base_Time_ {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_time_internal_TimeBase_v8_base_Time_Impl::operator_cmp(
                &Rc::new(RefCell::new(v8_base_time_internal_TimeBase_v8_base_Time_ {
                    us_: self.us_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(v8_base_time_internal_TimeBase_v8_base_Time_ {
                    us_: other.us_.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for v8_base_time_internal_TimeBase_v8_base_Time_ {}
impl Clone for v8_base_time_internal_TimeBase_v8_base_Time_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_time_internal_TimeBase_v8_base_Time_> =
            Rc::new(RefCell::new(Self {
                us_: Rc::new(RefCell::new((*self.us_.borrow()))),
            }));
        let this: Ptr<v8_base_time_internal_TimeBase_v8_base_Time_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_time_internal_TimeBase_v8_base_Time_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.us_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            us_: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
    us_: Value<i64>,
}
impl v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
    fn v8_base_time_internal_TimeBase_v8_base_TimeTicks_(us: i64) -> Self {
        let us: Value<i64> = Rc::new(RefCell::new(us));
        let __this: Value<v8_base_time_internal_TimeBase_v8_base_TimeTicks_> =
            Rc::new(RefCell::new(Self {
                us_: Rc::new(RefCell::new((*us.borrow()))),
            }));
        let this: Ptr<v8_base_time_internal_TimeBase_v8_base_TimeTicks_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            v8_base_time_internal_TimeBase_v8_base_TimeTicks_Impl::operator_cmp(
                &Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
                        us_: self.us_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
                        us_: other.us_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_time_internal_TimeBase_v8_base_TimeTicks_Impl::operator_cmp(
                &Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
                        us_: self.us_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
                        us_: other.us_.clone(),
                    },
                ))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {}
impl Clone for v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_time_internal_TimeBase_v8_base_TimeTicks_> =
            Rc::new(RefCell::new(Self {
                us_: Rc::new(RefCell::new((*self.us_.borrow()))),
            }));
        let this: Ptr<v8_base_time_internal_TimeBase_v8_base_TimeTicks_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.us_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            us_: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
    us_: Value<i64>,
}
impl v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
    fn v8_base_time_internal_TimeBase_v8_base_ThreadTicks_(us: i64) -> Self {
        let us: Value<i64> = Rc::new(RefCell::new(us));
        let __this: Value<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_> =
            Rc::new(RefCell::new(Self {
                us_: Rc::new(RefCell::new((*us.borrow()))),
            }));
        let this: Ptr<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            v8_base_time_internal_TimeBase_v8_base_ThreadTicks_Impl::operator_cmp(
                &Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
                        us_: self.us_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
                        us_: other.us_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_time_internal_TimeBase_v8_base_ThreadTicks_Impl::operator_cmp(
                &Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
                        us_: self.us_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
                        us_: other.us_.clone(),
                    },
                ))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {}
impl Clone for v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_> =
            Rc::new(RefCell::new(Self {
                us_: Rc::new(RefCell::new((*self.us_.borrow()))),
            }));
        let this: Ptr<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.us_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            us_: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
        }
    }
}
thread_local!(
    pub static kHoursPerDay_91: Value<i64> = Rc::new(RefCell::new(24));
);
thread_local!(
    pub static kMillisecondsPerSecond_92: Value<i64> = Rc::new(RefCell::new(1000));
);
thread_local!(
    pub static kMillisecondsPerDay_93: Value<i64> = Rc::new(RefCell::new(86400000));
);
thread_local!(
    pub static kMicrosecondsPerMillisecond_94: Value<i64> = Rc::new(RefCell::new(1000));
);
thread_local!(
    pub static kMicrosecondsPerSecond_95: Value<i64> = Rc::new(RefCell::new(1000000));
);
thread_local!(
    pub static kMicrosecondsPerMinute_96: Value<i64> = Rc::new(RefCell::new(60000000));
);
thread_local!(
    pub static kMicrosecondsPerHour_97: Value<i64> = Rc::new(RefCell::new(3600000000));
);
thread_local!(
    pub static kMicrosecondsPerDay_98: Value<i64> = Rc::new(RefCell::new(86400000000));
);
thread_local!(
    pub static kMicrosecondsPerWeek_99: Value<i64> = Rc::new(RefCell::new(604800000000));
);
thread_local!(
    pub static kNanosecondsPerMicrosecond_100: Value<i64> = Rc::new(RefCell::new(1000));
);
thread_local!(
    pub static kNanosecondsPerSecond_101: Value<i64> = Rc::new(RefCell::new(1000000000));
);
#[derive(Default)]
pub struct v8_base_TimeConstants {}
impl std::cmp::Ord for v8_base_TimeConstants {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            v8_base_TimeConstantsImpl::operator_cmp(
                &Rc::new(RefCell::new(v8_base_TimeConstants {})).as_pointer(),
                Rc::new(RefCell::new(v8_base_TimeConstants {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for v8_base_TimeConstants {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for v8_base_TimeConstants {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_TimeConstantsImpl::operator_cmp(
                &Rc::new(RefCell::new(v8_base_TimeConstants {})).as_pointer(),
                Rc::new(RefCell::new(v8_base_TimeConstants {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for v8_base_TimeConstants {}
impl Clone for v8_base_TimeConstants {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_TimeConstants> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_TimeConstants> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_TimeConstants {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn swap_102(a: v8_base_TimeDelta, b: v8_base_TimeDelta) {
    let a: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(a));
    let b: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(b));
    {
        let tmp = (*a.borrow()).delta_.as_pointer().read();
        (*a.borrow())
            .delta_
            .as_pointer()
            .write((*b.borrow()).delta_.as_pointer().read());
        (*b.borrow()).delta_.as_pointer().write(tmp);
    };
}
#[derive()]
pub struct v8_base_TimeDelta {
    delta_: Value<i64>,
}
impl v8_base_TimeDelta {
    pub fn v8_base_TimeDelta1() -> Self {
        let __this: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(Self {
            delta_: Rc::new(RefCell::new(0_i64)),
        }));
        let this: Ptr<v8_base_TimeDelta> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn FromDays(days: i32) -> v8_base_TimeDelta {
        let days: Value<i32> = Rc::new(RefCell::new(days));
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            (((*days.borrow()) as i64) * 86400000000)
        });
    }
    pub fn FromHours(hours: i32) -> v8_base_TimeDelta {
        let hours: Value<i32> = Rc::new(RefCell::new(hours));
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            (((*hours.borrow()) as i64) * 3600000000)
        });
    }
    pub fn FromMinutes(minutes: i32) -> v8_base_TimeDelta {
        let minutes: Value<i32> = Rc::new(RefCell::new(minutes));
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            (((*minutes.borrow()) as i64) * 60000000)
        });
    }
    pub fn FromSeconds(seconds: i64) -> v8_base_TimeDelta {
        let seconds: Value<i64> = Rc::new(RefCell::new(seconds));
        return v8_base_TimeDelta::v8_base_TimeDelta2({ ((*seconds.borrow()) * 1000000) });
    }
    pub fn FromMilliseconds(milliseconds: i64) -> v8_base_TimeDelta {
        let milliseconds: Value<i64> = Rc::new(RefCell::new(milliseconds));
        return v8_base_TimeDelta::v8_base_TimeDelta2({ ((*milliseconds.borrow()) * 1000) });
    }
    pub fn FromMicroseconds(microseconds: i64) -> v8_base_TimeDelta {
        let microseconds: Value<i64> = Rc::new(RefCell::new(microseconds));
        return v8_base_TimeDelta::v8_base_TimeDelta2({ (*microseconds.borrow()) });
    }
    pub fn FromNanoseconds(nanoseconds: i64) -> v8_base_TimeDelta {
        let nanoseconds: Value<i64> = Rc::new(RefCell::new(nanoseconds));
        return v8_base_TimeDelta::v8_base_TimeDelta2({ ((*nanoseconds.borrow()) / 1000) });
    }
    pub fn FromSecondsD(seconds: f64) -> v8_base_TimeDelta {
        let seconds: Value<f64> = Rc::new(RefCell::new(seconds));
        return ({ v8_base_TimeDelta::FromDouble(((*seconds.borrow()) * (1000000 as f64))) });
    }
    pub fn FromMillisecondsD(milliseconds: f64) -> v8_base_TimeDelta {
        let milliseconds: Value<f64> = Rc::new(RefCell::new(milliseconds));
        return ({ v8_base_TimeDelta::FromDouble(((*milliseconds.borrow()) * (1000 as f64))) });
    }
    fn v8_base_TimeDelta2(delta: i64) -> Self {
        let delta: Value<i64> = Rc::new(RefCell::new(delta));
        let __this: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(Self {
            delta_: Rc::new(RefCell::new((*delta.borrow()))),
        }));
        let this: Ptr<v8_base_TimeDelta> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for v8_base_TimeDelta {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            v8_base_TimeDeltaImpl::operator_cmp(
                &Rc::new(RefCell::new(v8_base_TimeDelta {
                    delta_: self.delta_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(v8_base_TimeDelta {
                    delta_: other.delta_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for v8_base_TimeDelta {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for v8_base_TimeDelta {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_TimeDeltaImpl::operator_cmp(
                &Rc::new(RefCell::new(v8_base_TimeDelta {
                    delta_: self.delta_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(v8_base_TimeDelta {
                    delta_: other.delta_.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for v8_base_TimeDelta {}
impl Clone for v8_base_TimeDelta {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(Self {
            delta_: Rc::new(RefCell::new((*self.delta_.borrow()))),
        }));
        let this: Ptr<v8_base_TimeDelta> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_TimeDelta {
    fn default() -> Self {
        { v8_base_TimeDelta::v8_base_TimeDelta1() }
    }
}
impl ByteRepr for v8_base_TimeDelta {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.delta_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            delta_: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
        }
    }
}
impl v8_base_TimeDelta {
    fn FromDouble(value: f64) -> v8_base_TimeDelta {
        let value: Value<f64> = Rc::new(RefCell::new(value));
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            ({ saturated_cast_87((*value.borrow())) })
        });
    }
}
impl v8_base_TimeDelta {
    pub fn Max() -> v8_base_TimeDelta {
        return v8_base_TimeDelta::v8_base_TimeDelta2({ <i64>::MAX });
    }
}
impl v8_base_TimeDelta {
    pub fn Min() -> v8_base_TimeDelta {
        return v8_base_TimeDelta::v8_base_TimeDelta2({ <i64>::MIN });
    }
}
pub fn Nanoseconds_103(nanoseconds: i64) -> v8_base_TimeDelta {
    let nanoseconds: Value<i64> = Rc::new(RefCell::new(nanoseconds));
    return ({ v8_base_TimeDelta::FromNanoseconds((*nanoseconds.borrow())) });
}
pub fn Microseconds_104(microseconds: i64) -> v8_base_TimeDelta {
    let microseconds: Value<i64> = Rc::new(RefCell::new(microseconds));
    return ({ v8_base_TimeDelta::FromMicroseconds((*microseconds.borrow())) });
}
pub fn Milliseconds_105(milliseconds: i64) -> v8_base_TimeDelta {
    let milliseconds: Value<i64> = Rc::new(RefCell::new(milliseconds));
    return ({ v8_base_TimeDelta::FromMilliseconds((*milliseconds.borrow())) });
}
pub fn Milliseconds_106(milliseconds: f64) -> v8_base_TimeDelta {
    let milliseconds: Value<f64> = Rc::new(RefCell::new(milliseconds));
    return ({ v8_base_TimeDelta::FromMillisecondsD((*milliseconds.borrow())) });
}
pub fn Seconds_107(seconds: i64) -> v8_base_TimeDelta {
    let seconds: Value<i64> = Rc::new(RefCell::new(seconds));
    return ({ v8_base_TimeDelta::FromSeconds((*seconds.borrow())) });
}
pub fn Seconds_108(seconds: f64) -> v8_base_TimeDelta {
    let seconds: Value<f64> = Rc::new(RefCell::new(seconds));
    return ({ v8_base_TimeDelta::FromSecondsD((*seconds.borrow())) });
}
pub fn Minutes_109(minutes: i32) -> v8_base_TimeDelta {
    let minutes: Value<i32> = Rc::new(RefCell::new(minutes));
    return ({ v8_base_TimeDelta::FromMinutes((*minutes.borrow())) });
}
pub fn Hours_110(hours: i32) -> v8_base_TimeDelta {
    let hours: Value<i32> = Rc::new(RefCell::new(hours));
    return ({ v8_base_TimeDelta::FromHours((*hours.borrow())) });
}
pub fn FromDays_111(days: i32) -> v8_base_TimeDelta {
    let days: Value<i32> = Rc::new(RefCell::new(days));
    return ({ v8_base_TimeDelta::FromDays((*days.borrow())) });
}
#[derive()]
pub struct v8_base_Time {}
impl v8_base_Time {
    pub fn v8_base_Time2() -> Self {
        let __this: Value<v8_base_Time> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_Time> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn UnixEpoch() -> v8_base_Time {
        return v8_base_Time::v8_base_Time1({ 0_i64 });
    }
    fn v8_base_Time1(us: i64) -> Self {
        let us: Value<i64> = Rc::new(RefCell::new(us));
        let __this: Value<v8_base_Time> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_Time> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_Time {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Time> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_Time> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_Time {
    fn default() -> Self {
        { v8_base_Time::v8_base_Time2() }
    }
}
impl ByteRepr for v8_base_Time {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn operator_add_112(delta: Ptr<v8_base_TimeDelta>, time: Ptr<v8_base_Time>) -> v8_base_Time {
    return ({
        let _delta: v8_base_TimeDelta = (*delta.upgrade().deref()).clone();
        v8_base_time_internal_TimeBase_v8_base_Time_Impl::operator_add(&time, _delta)
    });
}
#[derive()]
pub struct v8_base_TimeTicks {}
impl v8_base_TimeTicks {
    pub fn v8_base_TimeTicks2() -> Self {
        let __this: Value<v8_base_TimeTicks> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_TimeTicks> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn FromMsTicksForTesting(ticks: i64) -> v8_base_TimeTicks {
        let ticks: Value<i64> = Rc::new(RefCell::new(ticks));
        return v8_base_TimeTicks::v8_base_TimeTicks1({ ((*ticks.borrow()) * 1000) });
    }
    fn v8_base_TimeTicks1(ticks: i64) -> Self {
        let ticks: Value<i64> = Rc::new(RefCell::new(ticks));
        let __this: Value<v8_base_TimeTicks> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_TimeTicks> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_TimeTicks {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_TimeTicks> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_TimeTicks> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_TimeTicks {
    fn default() -> Self {
        { v8_base_TimeTicks::v8_base_TimeTicks2() }
    }
}
impl ByteRepr for v8_base_TimeTicks {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn operator_add_113(
    delta: Ptr<v8_base_TimeDelta>,
    ticks: Ptr<v8_base_TimeTicks>,
) -> v8_base_TimeTicks {
    return ({
        let _delta: v8_base_TimeDelta = (*delta.upgrade().deref()).clone();
        v8_base_time_internal_TimeBase_v8_base_TimeTicks_Impl::operator_add(&ticks, _delta)
    });
}
#[derive()]
pub struct v8_base_ThreadTicks {}
impl v8_base_ThreadTicks {
    pub fn v8_base_ThreadTicks1() -> Self {
        let __this: Value<v8_base_ThreadTicks> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_ThreadTicks> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn WaitUntilInitialized() {}
    fn v8_base_ThreadTicks2(ticks: i64) -> Self {
        let ticks: Value<i64> = Rc::new(RefCell::new(ticks));
        let __this: Value<v8_base_ThreadTicks> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_ThreadTicks> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_ThreadTicks {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_ThreadTicks> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_ThreadTicks> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_ThreadTicks {
    fn default() -> Self {
        { v8_base_ThreadTicks::v8_base_ThreadTicks1() }
    }
}
impl ByteRepr for v8_base_ThreadTicks {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub trait v8_base_TimeConstantsImpl {
    fn operator_cmp(&self, _a0: Ptr<v8_base_TimeConstants>) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<v8_base_TimeConstants>) -> bool;
}
impl v8_base_TimeConstantsImpl for Ptr<v8_base_TimeConstants> {
    fn operator_cmp(&self, _a0: Ptr<v8_base_TimeConstants>) -> std::cmp::Ordering {
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<v8_base_TimeConstants>) -> bool {
        return true;
    }
}
pub trait v8_base_TimeDeltaImpl {
    fn IsZero(&self) -> bool;
    fn IsMax(&self) -> bool;
    fn IsMin(&self) -> bool;
    fn operator_add(&self, other: Ptr<v8_base_TimeDelta>) -> v8_base_TimeDelta;
    fn operator_sub_pconstv8_base_TimeDelta_const(
        &self,
        other: Ptr<v8_base_TimeDelta>,
    ) -> v8_base_TimeDelta;
    fn operator_add_assign(&self, other: Ptr<v8_base_TimeDelta>) -> Ptr<v8_base_TimeDelta>;
    fn operator_sub_assign(&self, other: Ptr<v8_base_TimeDelta>) -> Ptr<v8_base_TimeDelta>;
    fn operator_neg_const(&self) -> v8_base_TimeDelta;
    fn TimesOf(&self, other: Ptr<v8_base_TimeDelta>) -> f64;
    fn PercentOf(&self, other: Ptr<v8_base_TimeDelta>) -> f64;
    fn operator_mul(&self, a: i64) -> v8_base_TimeDelta;
    fn operator_div_i64_const(&self, a: i64) -> v8_base_TimeDelta;
    fn operator_mul_assign(&self, a: i64) -> Ptr<v8_base_TimeDelta>;
    fn operator_div_assign(&self, a: i64) -> Ptr<v8_base_TimeDelta>;
    fn operator_div_pconstv8_base_TimeDelta_const(&self, other: Ptr<v8_base_TimeDelta>) -> i64;
    fn operator_cmp(&self, _a0: Ptr<v8_base_TimeDelta>) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<v8_base_TimeDelta>) -> bool;
}
impl v8_base_TimeDeltaImpl for Ptr<v8_base_TimeDelta> {
    fn IsZero(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).delta_.borrow()) == 0_i64);
    }
    fn IsMax(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).delta_.borrow()) == <i64>::MAX);
    }
    fn IsMin(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).delta_.borrow()) == <i64>::MIN);
    }
    fn operator_add(&self, other: Ptr<v8_base_TimeDelta>) -> v8_base_TimeDelta {
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            {
                let _lhs = (*(*(*self).upgrade().deref()).delta_.borrow());
                _lhs + (*(*other.upgrade().deref()).delta_.borrow())
            }
        });
    }
    fn operator_sub_pconstv8_base_TimeDelta_const(
        &self,
        other: Ptr<v8_base_TimeDelta>,
    ) -> v8_base_TimeDelta {
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            {
                let _lhs = (*(*(*self).upgrade().deref()).delta_.borrow());
                _lhs - (*(*other.upgrade().deref()).delta_.borrow())
            }
        });
    }
    fn operator_add_assign(&self, other: Ptr<v8_base_TimeDelta>) -> Ptr<v8_base_TimeDelta> {
        let __rhs = (*(*other.upgrade().deref()).delta_.borrow());
        (*(*(*self).upgrade().deref()).delta_.borrow_mut()) += __rhs;
        return (*self).clone();
    }
    fn operator_sub_assign(&self, other: Ptr<v8_base_TimeDelta>) -> Ptr<v8_base_TimeDelta> {
        let __rhs = (*(*other.upgrade().deref()).delta_.borrow());
        (*(*(*self).upgrade().deref()).delta_.borrow_mut()) -= __rhs;
        return (*self).clone();
    }
    fn operator_neg_const(&self) -> v8_base_TimeDelta {
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            -(*(*(*self).upgrade().deref()).delta_.borrow())
        });
    }
    fn TimesOf(&self, other: Ptr<v8_base_TimeDelta>) -> f64 {
        return {
            let _lhs = ((*(*(*self).upgrade().deref()).delta_.borrow()) as f64);
            _lhs / ((*(*other.upgrade().deref()).delta_.borrow()) as f64)
        };
    }
    fn PercentOf(&self, other: Ptr<v8_base_TimeDelta>) -> f64 {
        return (({ v8_base_TimeDeltaImpl::TimesOf(self, (other).clone()) }) * 1.0E+2);
    }
    fn operator_mul(&self, a: i64) -> v8_base_TimeDelta {
        let a: Value<i64> = Rc::new(RefCell::new(a));
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            ((*(*(*self).upgrade().deref()).delta_.borrow()) * (*a.borrow()))
        });
    }
    fn operator_div_i64_const(&self, a: i64) -> v8_base_TimeDelta {
        let a: Value<i64> = Rc::new(RefCell::new(a));
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            ((*(*(*self).upgrade().deref()).delta_.borrow()) / (*a.borrow()))
        });
    }
    fn operator_mul_assign(&self, a: i64) -> Ptr<v8_base_TimeDelta> {
        let a: Value<i64> = Rc::new(RefCell::new(a));
        (*(*(*self).upgrade().deref()).delta_.borrow_mut()) *= (*a.borrow());
        return (*self).clone();
    }
    fn operator_div_assign(&self, a: i64) -> Ptr<v8_base_TimeDelta> {
        let a: Value<i64> = Rc::new(RefCell::new(a));
        (*(*(*self).upgrade().deref()).delta_.borrow_mut()) /= (*a.borrow());
        return (*self).clone();
    }
    fn operator_div_pconstv8_base_TimeDelta_const(&self, other: Ptr<v8_base_TimeDelta>) -> i64 {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).delta_.borrow());
            _lhs / (*(*other.upgrade().deref()).delta_.borrow())
        };
    }
    fn operator_cmp(&self, _a0: Ptr<v8_base_TimeDelta>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).delta_.borrow())
                    .cmp(&(*(*_a0.upgrade().deref()).delta_.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<v8_base_TimeDelta>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).delta_.borrow());
            _lhs == (*(*_a0.upgrade().deref()).delta_.borrow())
        };
    }
}
pub trait v8_base_internal_RangeCheckImpl {
    fn operator_eq(&self, rhs: Ptr<v8_base_internal_RangeCheck>) -> bool;
    fn IsValid(&self) -> bool;
    fn IsInvalid(&self) -> bool;
    fn IsOverflow(&self) -> bool;
    fn IsUnderflow(&self) -> bool;
    fn IsOverflowFlagSet(&self) -> bool;
    fn IsUnderflowFlagSet(&self) -> bool;
}
impl v8_base_internal_RangeCheckImpl for Ptr<v8_base_internal_RangeCheck> {
    fn operator_eq(&self, rhs: Ptr<v8_base_internal_RangeCheck>) -> bool {
        return ({
            let _lhs = ((*(*(*self).upgrade().deref()).is_underflow_.borrow()) as i32);
            _lhs == ((*(*rhs.upgrade().deref()).is_underflow_.borrow()) as i32)
        }) && ({
            let _lhs = ((*(*(*self).upgrade().deref()).is_overflow_.borrow()) as i32);
            _lhs == ((*(*rhs.upgrade().deref()).is_overflow_.borrow()) as i32)
        });
    }
    fn IsValid(&self) -> bool {
        return (!(*(*(*self).upgrade().deref()).is_overflow_.borrow()))
            && (!(*(*(*self).upgrade().deref()).is_underflow_.borrow()));
    }
    fn IsInvalid(&self) -> bool {
        return (*(*(*self).upgrade().deref()).is_overflow_.borrow())
            && (*(*(*self).upgrade().deref()).is_underflow_.borrow());
    }
    fn IsOverflow(&self) -> bool {
        return (*(*(*self).upgrade().deref()).is_overflow_.borrow())
            && (!(*(*(*self).upgrade().deref()).is_underflow_.borrow()));
    }
    fn IsUnderflow(&self) -> bool {
        return (!(*(*(*self).upgrade().deref()).is_overflow_.borrow()))
            && (*(*(*self).upgrade().deref()).is_underflow_.borrow());
    }
    fn IsOverflowFlagSet(&self) -> bool {
        return (*(*(*self).upgrade().deref()).is_overflow_.borrow());
    }
    fn IsUnderflowFlagSet(&self) -> bool {
        return (*(*(*self).upgrade().deref()).is_underflow_.borrow());
    }
}
pub trait v8_base_time_internal_TimeBase_v8_base_ThreadTicks_Impl {
    fn operator_cmp(
        &self,
        _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_>,
    ) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_>) -> bool;
}
impl v8_base_time_internal_TimeBase_v8_base_ThreadTicks_Impl
    for Ptr<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_>
{
    fn operator_cmp(
        &self,
        _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_>,
    ) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                ({
                    let _arg0: Ptr<v8_base_TimeConstants> = (_a0).clone();
                    v8_base_TimeConstantsImpl::operator_cmp(&(*self), _arg0)
                }),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).us_.borrow())
                    .cmp(&(*(*_a0.upgrade().deref()).us_.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_>) -> bool {
        return ({
            let _arg0: Ptr<v8_base_TimeConstants> = (_a0).clone();
            v8_base_TimeConstantsImpl::operator_eq(&(*self), _arg0)
        }) && ({
            let _lhs = (*(*(*self).upgrade().deref()).us_.borrow());
            _lhs == (*(*_a0.upgrade().deref()).us_.borrow())
        });
    }
}
pub trait v8_base_time_internal_TimeBase_v8_base_TimeTicks_Impl {
    fn operator_add(&self, delta: v8_base_TimeDelta) -> v8_base_TimeTicks;
    fn operator_cmp(
        &self,
        _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_TimeTicks_>,
    ) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_TimeTicks_>) -> bool;
}
impl v8_base_time_internal_TimeBase_v8_base_TimeTicks_Impl
    for Ptr<v8_base_time_internal_TimeBase_v8_base_TimeTicks_>
{
    fn operator_add(&self, delta: v8_base_TimeDelta) -> v8_base_TimeTicks {
        let delta: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(delta));
        return v8_base_TimeTicks::v8_base_TimeTicks1({
            ({
                SignedSaturatedAdd64_90(
                    (*(*delta.borrow()).delta_.borrow()),
                    (*(*(*self).upgrade().deref()).us_.borrow()),
                )
            })
        });
    }
    fn operator_cmp(
        &self,
        _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_TimeTicks_>,
    ) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                ({
                    let _arg0: Ptr<v8_base_TimeConstants> = (_a0).clone();
                    v8_base_TimeConstantsImpl::operator_cmp(&(*self), _arg0)
                }),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).us_.borrow())
                    .cmp(&(*(*_a0.upgrade().deref()).us_.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_TimeTicks_>) -> bool {
        return ({
            let _arg0: Ptr<v8_base_TimeConstants> = (_a0).clone();
            v8_base_TimeConstantsImpl::operator_eq(&(*self), _arg0)
        }) && ({
            let _lhs = (*(*(*self).upgrade().deref()).us_.borrow());
            _lhs == (*(*_a0.upgrade().deref()).us_.borrow())
        });
    }
}
pub trait v8_base_time_internal_TimeBase_v8_base_Time_Impl {
    fn operator_add(&self, delta: v8_base_TimeDelta) -> v8_base_Time;
    fn operator_cmp(
        &self,
        _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_Time_>,
    ) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_Time_>) -> bool;
}
impl v8_base_time_internal_TimeBase_v8_base_Time_Impl
    for Ptr<v8_base_time_internal_TimeBase_v8_base_Time_>
{
    fn operator_add(&self, delta: v8_base_TimeDelta) -> v8_base_Time {
        let delta: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(delta));
        return v8_base_Time::v8_base_Time1({
            ({
                SignedSaturatedAdd64_90(
                    (*(*delta.borrow()).delta_.borrow()),
                    (*(*(*self).upgrade().deref()).us_.borrow()),
                )
            })
        });
    }
    fn operator_cmp(
        &self,
        _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_Time_>,
    ) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                ({
                    let _arg0: Ptr<v8_base_TimeConstants> = (_a0).clone();
                    v8_base_TimeConstantsImpl::operator_cmp(&(*self), _arg0)
                }),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).us_.borrow())
                    .cmp(&(*(*_a0.upgrade().deref()).us_.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_Time_>) -> bool {
        return ({
            let _arg0: Ptr<v8_base_TimeConstants> = (_a0).clone();
            v8_base_TimeConstantsImpl::operator_eq(&(*self), _arg0)
        }) && ({
            let _lhs = (*(*(*self).upgrade().deref()).us_.borrow());
            _lhs == (*(*_a0.upgrade().deref()).us_.borrow())
        });
    }
}
