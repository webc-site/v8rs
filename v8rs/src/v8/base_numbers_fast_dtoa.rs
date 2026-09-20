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
pub fn bit_cast_16(source: Ptr<i8>) -> u8 {
    return ({ bit_cast_17((source).clone()) });
}
pub fn bit_cast_18(source: Ptr<i16>) -> u16 {
    return ({ bit_cast_19((source).clone()) });
}
pub fn bit_cast_20(source: Ptr<i32>) -> u32 {
    return ({ bit_cast_21((source).clone()) });
}
pub fn bit_cast_22(source: Ptr<i64>) -> u64 {
    return ({ bit_cast_23((source).clone()) });
}
pub fn bit_cast_24(source: Ptr<i64>) -> u64 {
    return ({ bit_cast_25((source).clone()) });
}
pub fn bit_cast_26(source: Ptr<f32>) -> u32 {
    return ({ bit_cast_27((source).clone()) });
}
pub fn bit_cast_28(source: Ptr<f64>) -> u64 {
    return ({ bit_cast_29((source).clone()) });
}
pub fn bit_cast_30(source: Ptr<u64>) -> f64 {
    return ({ bit_cast_31((source).clone()) });
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
pub fn make_uint64_32(high: u32, low: u32) -> u64 {
    let high: Value<u32> = Rc::new(RefCell::new(high));
    let low: Value<u32> = Rc::new(RefCell::new(low));
    return (((*high.borrow()) as u64) << 32).wrapping_add(((*low.borrow()) as u64));
}
pub fn RoundDown_33(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    return ((*x.borrow()) & (-(*m.borrow()) as u64));
}
pub fn RoundUp_34(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    (&(0));
    return ({
        let _x: u64 = ((*x.borrow()).wrapping_add((((*m.borrow()) - 1_i64) as u64)) as u64);
        let _m: i64 = (*m.borrow());
        RoundDown_33(_x, _m)
    });
}
pub fn IsAligned_35(value: u64, alignment: u64) -> bool {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let alignment: Value<u64> = Rc::new(RefCell::new(alignment));
    return (((*value.borrow()) & ((*alignment.borrow()).wrapping_sub(1_u64))) == 0_u64);
}
pub fn CountLeadingZeros_36(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        if (32_u32 == 64_u32) {
            (({ __builtin_clzll_37(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((32_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros_38(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        if (64_u32 == 64_u32) {
            (({ __builtin_clzll_37(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((64_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros_39(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        if (64_u32 == 64_u32) {
            (({ __builtin_clzll_37((*value.borrow())) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((64_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros32_40(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_36((*value.borrow())) });
}
pub fn CountLeadingZeros64_41(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_38((*value.borrow())) });
}
pub fn CountTrailingZeros_42(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        (if (32_u32 == 64_u32) {
            ({ __builtin_ctzll_43(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros_44(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        (if (64_u32 == 64_u32) {
            ({ __builtin_ctzll_43(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros32_45(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_42((*value.borrow())) });
}
pub fn CountTrailingZeros64_46(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_44((*value.borrow())) });
}
pub fn RoundUpToPowerOfTwo32_47(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u32 << ((32_u32).wrapping_sub(({ CountLeadingZeros_36((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo64_48(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u64 << ((64_u32).wrapping_sub(({ CountLeadingZeros_38((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo_49(value: usize) -> usize {
    let value: Value<usize> = Rc::new(RefCell::new(value));
    if (::std::mem::size_of::<usize>() == ::std::mem::size_of::<u64>()) {
        return (({ RoundUpToPowerOfTwo64_48(((*value.borrow()) as u64)) }) as usize);
    } else {
        return (({ RoundUpToPowerOfTwo32_47(((*value.borrow()) as u32)) }) as usize);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn RoundDownToPowerOfTwo32_50(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    if ((*value.borrow()) > 2147483648_u32) {
        return 2147483648_u32;
    }
    let result: Value<u32> = Rc::new(RefCell::new(
        ({ RoundUpToPowerOfTwo32_47((*value.borrow())) }),
    ));
    if ((*result.borrow()) > (*value.borrow())) {
        (*result.borrow_mut()) >>= 1;
    }
    return (*result.borrow());
}
pub fn RotateRight32_51(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateLeft32_52(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateRight64_53(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn RotateLeft64_54(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn ClearLsb_55(value: i32) -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(value));
    return ((*value.borrow()) & ((*value.borrow()) - 1));
}
pub fn SignedAddOverflow32_56(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_sadd_overflow_57(_arg0, _arg1, _arg2)
    });
}
pub fn SignedSubOverflow32_58(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_ssub_overflow_59(_arg0, _arg1, _arg2)
    });
}
pub fn SignedMulOverflow32_60(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_smul_overflow_61(_arg0, _arg1, _arg2)
    });
}
pub fn SignedAddOverflow64_62(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_add_overflow_63(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedSubOverflow64_64(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_sub_overflow_65(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedMulOverflow64_66(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return {
        let (val, ovf) = (*lhs.borrow()).overflowing_mul((*rhs.borrow()));
        (*val.borrow()).write(val);
        ovf
    };
}
pub fn UnsignedAddOverflow32_67(lhs: u32, rhs: u32, val: Ptr<u32>) -> bool {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<u32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: u32 = (*lhs.borrow());
        let _arg1: u32 = (*rhs.borrow());
        let _arg2: Ptr<u32> = (*val.borrow()).clone();
        __builtin_uadd_overflow_68(_arg0, _arg1, _arg2)
    });
}
pub fn UnsignedDiv32_69(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedDiv64_70(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn UnsignedMod32_71(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedMod64_72(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn WraparoundAdd32_73(lhs: i32, rhs: i32) -> i32 {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as u32).wrapping_add(((*rhs.borrow()) as u32))) as i32);
}
pub fn WraparoundNeg32_74(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (-((*x.borrow()) as u32) as i32);
}
pub fn ByteReverse16_75(value: u16) -> u16 {
    let value: Value<u16> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse32_76(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse64_77(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn OverlappingWrites_78(dst: AnyPtr, src: AnyPtr, count: usize) {
    let dst: Value<AnyPtr> = Rc::new(RefCell::new(dst));
    let src: Value<AnyPtr> = Rc::new(RefCell::new(src));
    let count: Value<usize> = Rc::new(RefCell::new(count));
    (*dst.borrow())
        .reinterpret_cast::<u16>()
        .write(((*src.borrow()).reinterpret_cast::<u16>().read()));
    let __rhs = ((*src.borrow())
        .reinterpret_cast::<u8>()
        .offset((*count.borrow()) as isize)
        .offset(-((::std::mem::size_of::<u16>()) as isize))
        .reinterpret_cast::<u16>()
        .read());
    (*dst.borrow())
        .reinterpret_cast::<u8>()
        .offset((*count.borrow()) as isize)
        .offset(-((::std::mem::size_of::<u16>()) as isize))
        .reinterpret_cast::<u16>()
        .write(__rhs);
}
pub fn OverlappingWrites_79(dst: AnyPtr, src: AnyPtr, count: usize) {
    let dst: Value<AnyPtr> = Rc::new(RefCell::new(dst));
    let src: Value<AnyPtr> = Rc::new(RefCell::new(src));
    let count: Value<usize> = Rc::new(RefCell::new(count));
    (*dst.borrow())
        .reinterpret_cast::<u32>()
        .write(((*src.borrow()).reinterpret_cast::<u32>().read()));
    let __rhs = ((*src.borrow())
        .reinterpret_cast::<u8>()
        .offset((*count.borrow()) as isize)
        .offset(-((::std::mem::size_of::<u32>()) as isize))
        .reinterpret_cast::<u32>()
        .read());
    (*dst.borrow())
        .reinterpret_cast::<u8>()
        .offset((*count.borrow()) as isize)
        .offset(-((::std::mem::size_of::<u32>()) as isize))
        .reinterpret_cast::<u32>()
        .write(__rhs);
}
pub fn OverlappingWrites_80(dst: AnyPtr, src: AnyPtr, count: usize) {
    let dst: Value<AnyPtr> = Rc::new(RefCell::new(dst));
    let src: Value<AnyPtr> = Rc::new(RefCell::new(src));
    let count: Value<usize> = Rc::new(RefCell::new(count));
    (*dst.borrow())
        .reinterpret_cast::<u64>()
        .write(((*src.borrow()).reinterpret_cast::<u64>().read()));
    let __rhs = ((*src.borrow())
        .reinterpret_cast::<u8>()
        .offset((*count.borrow()) as isize)
        .offset(-((::std::mem::size_of::<u64>()) as isize))
        .reinterpret_cast::<u64>()
        .read());
    (*dst.borrow())
        .reinterpret_cast::<u8>()
        .offset((*count.borrow()) as isize)
        .offset(-((::std::mem::size_of::<u64>()) as isize))
        .reinterpret_cast::<u64>()
        .write(__rhs);
}
pub fn SimdMemCopy_81(dst: AnyPtr, src: AnyPtr, count: usize) {
    let dst: Value<AnyPtr> = Rc::new(RefCell::new(dst));
    let src: Value<AnyPtr> = Rc::new(RefCell::new(src));
    let count: Value<usize> = Rc::new(RefCell::new(count));
    let dst_u: Value<Ptr<u8>> = Rc::new(RefCell::new((*dst.borrow()).reinterpret_cast::<u8>()));
    let src_u: Value<Ptr<u8>> = Rc::new(RefCell::new((*src.borrow()).reinterpret_cast::<u8>()));
    if ((*count.borrow()) == 0_usize) {
        return;
    }
    if ((*count.borrow()) == 1_usize) {
        let __rhs = ((*src_u.borrow()).read());
        (*dst_u.borrow()).write(__rhs);
        return;
    }
    let order: Value<usize> = Rc::new(RefCell::new(
        ((::std::mem::size_of::<usize>() as usize).wrapping_mul(8_usize) as usize).wrapping_sub(
            (({ CountLeadingZeros_39(((*count.borrow()).wrapping_sub(1_usize) as u64)) }) as usize),
        ),
    ));
    'switch: {
        let __match_cond = (*order.borrow());
        match __match_cond {
            __v if __v == 1 => {
                (*dst_u.borrow())
                    .reinterpret_cast::<u16>()
                    .write(((*src_u.borrow()).reinterpret_cast::<u16>().read()));
                return;
            }
            __v if __v == 2 => {
                ({
                    OverlappingWrites_78(
                        ((*dst_u.borrow()).clone() as Ptr<u8>).to_any(),
                        ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                        (*count.borrow()),
                    )
                });
                return;
            }
            __v if __v == 3 => {
                ({
                    OverlappingWrites_79(
                        ((*dst_u.borrow()).clone() as Ptr<u8>).to_any(),
                        ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                        (*count.borrow()),
                    )
                });
                return;
            }
            __v if __v == 4 => {
                ({
                    OverlappingWrites_80(
                        ((*dst_u.borrow()).clone() as Ptr<u8>).to_any(),
                        ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                        (*count.borrow()),
                    )
                });
                return;
            }
            __v if __v == 5 => {
                {
                    let __s1: Value<Value<u8>> = Rc::new(RefCell::new({
                        let __ret: Value<Value<u8>> = <Value<Value<u8>>>::default();
                        (*__ret.borrow_mut()) = ({
                            __builtin_neon_vld1q_v_82(
                                ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                                48,
                            )
                        });
                        let __result = (*__ret.borrow());
                        __result
                    }));
                    let __result = ({
                        __builtin_neon_vst1q_v_83(
                            ((*dst_u.borrow()).clone() as Ptr<u8>).to_any(),
                            (*__s1.borrow()).clone(),
                            48,
                        )
                    });
                    __result
                };
                {
                    let __s1: Value<Value<u8>> = Rc::new(RefCell::new({
                        let __ret: Value<Value<u8>> = <Value<Value<u8>>>::default();
                        (*__ret.borrow_mut()) = ({
                            __builtin_neon_vld1q_v_82(
                                ((*src_u.borrow())
                                    .offset((*count.borrow()) as isize)
                                    .offset(-((::std::mem::size_of::<u8>()) as isize))
                                    as Ptr<u8>)
                                    .to_any(),
                                48,
                            )
                        });
                        let __result = (*__ret.borrow());
                        __result
                    }));
                    let __result = ({
                        __builtin_neon_vst1q_v_83(
                            ((*dst_u.borrow())
                                .offset((*count.borrow()) as isize)
                                .offset(-((::std::mem::size_of::<u8>()) as isize))
                                as Ptr<u8>)
                                .to_any(),
                            (*__s1.borrow()).clone(),
                            48,
                        )
                    });
                    __result
                };
                return;
            }
            _ => {
                {
                    let __s1: Value<Value<u8>> = Rc::new(RefCell::new({
                        let __ret: Value<Value<u8>> = <Value<Value<u8>>>::default();
                        (*__ret.borrow_mut()) = ({
                            __builtin_neon_vld1q_v_82(
                                ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                                48,
                            )
                        });
                        let __result = (*__ret.borrow());
                        __result
                    }));
                    let __result = ({
                        __builtin_neon_vst1q_v_83(
                            ((*dst_u.borrow()).clone() as Ptr<u8>).to_any(),
                            (*__s1.borrow()).clone(),
                            48,
                        )
                    });
                    __result
                };
                let i: Value<usize> = Rc::new(RefCell::new(
                    (((*count.borrow()) as u64).wrapping_rem((::std::mem::size_of::<u8>() as u64))
                        as usize),
                ));
                'loop_: while ((*i.borrow()) < (*count.borrow())) {
                    {
                        let __s1: Value<Value<u8>> = Rc::new(RefCell::new({
                            let __ret: Value<Value<u8>> = <Value<Value<u8>>>::default();
                            (*__ret.borrow_mut()) = ({
                                __builtin_neon_vld1q_v_82(
                                    ((*src_u.borrow()).offset((*i.borrow()) as isize) as Ptr<u8>)
                                        .to_any(),
                                    48,
                                )
                            });
                            let __result = (*__ret.borrow());
                            __result
                        }));
                        let __result = ({
                            __builtin_neon_vst1q_v_83(
                                ((*dst_u.borrow()).offset((*i.borrow()) as isize) as Ptr<u8>)
                                    .to_any(),
                                (*__s1.borrow()).clone(),
                                48,
                            )
                        });
                        __result
                    };
                    {
                        let rhs_0 = (((*i.borrow()) as u64)
                            .wrapping_add((::std::mem::size_of::<u8>() as u64)))
                            as usize;
                        (*i.borrow_mut()) = rhs_0
                    };
                }
                return;
            }
        }
    };
}
pub fn MemCopy_84(dest: AnyPtr, src: AnyPtr, size: usize) {
    let dest: Value<AnyPtr> = Rc::new(RefCell::new(dest));
    let src: Value<AnyPtr> = Rc::new(RefCell::new(src));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    ({
        SimdMemCopy_81(
            (*dest.borrow()).clone(),
            (*src.borrow()).clone(),
            (*size.borrow()),
        )
    });
}
pub fn MemMove_85(dest: AnyPtr, src: AnyPtr, size: usize) {
    let dest: Value<AnyPtr> = Rc::new(RefCell::new(dest));
    let src: Value<AnyPtr> = Rc::new(RefCell::new(src));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    'switch: {
        let __match_cond = (*size.borrow());
        match __match_cond {
            __v if __v == 0 => {
                return;
            }
            __v if __v == 1 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 1_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 2 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 2_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 3 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 3_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 4 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 4_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 5 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 5_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 6 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 6_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 7 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 7_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 8 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 8_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 9 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 9_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 10 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 10_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 11 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 11_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 12 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 12_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 13 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 13_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 14 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 14_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 15 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 15_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            __v if __v == 16 => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), 16_usize as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
            _ => {
                {
                    (*dest.borrow()).memcpy(&(*src.borrow()), (*size.borrow()) as usize);
                    (*dest.borrow()).clone()
                };
                return;
            }
        }
    };
}
thread_local!(
    pub static kCompressionFactor_86: Value<u32> = Rc::new(RefCell::new(1));
);
thread_local!(
    pub static kExpansionFactor_87: Value<u32> = Rc::new(RefCell::new(1));
);
#[derive(Default)]
pub struct PlainHashReader {}
impl PlainHashReader {
    pub fn Read64(p: Ptr<u8>) -> u64 {
        let p: Value<Ptr<u8>> = Rc::new(RefCell::new(p));
        let v: Value<u64> = <Value<u64>>::default();
        {
            ((v.as_pointer()) as Ptr<u64>).to_any().memcpy(
                &((*p.borrow()).clone() as Ptr<u8>).to_any(),
                8_usize as usize,
            );
            ((v.as_pointer()) as Ptr<u64>).to_any()
        };
        return (*v.borrow());
    }
    pub fn Read32(p: Ptr<u8>) -> u64 {
        let p: Value<Ptr<u8>> = Rc::new(RefCell::new(p));
        let v: Value<u32> = <Value<u32>>::default();
        {
            ((v.as_pointer()) as Ptr<u32>).to_any().memcpy(
                &((*p.borrow()).clone() as Ptr<u8>).to_any(),
                4_usize as usize,
            );
            ((v.as_pointer()) as Ptr<u32>).to_any()
        };
        return ((*v.borrow()) as u64);
    }
    pub fn ReadSmall(p: Ptr<u8>, k: usize) -> u64 {
        let p: Value<Ptr<u8>> = Rc::new(RefCell::new(p));
        let k: Value<usize> = Rc::new(RefCell::new(k));
        return ({
            let _lhs = ((((*p.borrow()).offset((0) as isize).read()) as u64) << 56);
            _lhs | ((((*p.borrow()).offset(((*k.borrow()) >> 1) as isize).read()) as u64) << 32)
        } | (((*p.borrow())
            .offset(((*k.borrow()).wrapping_sub(1_usize)) as isize)
            .read()) as u64));
    }
}
impl Clone for PlainHashReader {
    fn clone(&self) -> Self {
        let __this: Value<PlainHashReader> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<PlainHashReader> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for PlainHashReader {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn rapid_mul128_88(A: u64, B: u64) -> (Value<u64>, Value<u64>) {
    let A: Value<u64> = Rc::new(RefCell::new(A));
    let B: Value<u64> = Rc::new(RefCell::new(B));
    let r: Value<u128> = Rc::new(RefCell::new(((*A.borrow()) as u128)));
    {
        let rhs_0 = (*r.borrow()).wrapping_mul(((*B.borrow()) as u128));
        (*r.borrow_mut()) = rhs_0
    };
    return (
        Rc::new(RefCell::new(
            ((*r.borrow()) as u64)
                .try_into()
                .expect("failed conversion"),
        )),
        Rc::new(RefCell::new(
            (((*r.borrow()) >> 64) as u64)
                .try_into()
                .expect("failed conversion"),
        )),
    );
}
pub fn rapid_mix_89(A: u64, B: u64) -> u64 {
    let A: Value<u64> = Rc::new(RefCell::new(A));
    let B: Value<u64> = Rc::new(RefCell::new(B));
    let __rhs = ({ rapid_mul128_88((*A.borrow()), (*B.borrow())) });
    ({ tie_90(A.as_pointer(), B.as_pointer()) }) = __rhs;
    return ((*A.borrow()) ^ (*B.borrow()));
}
thread_local!(
    pub static RAPIDHASH_DEFAULT_SECRET_91: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        3257665815644502181_u64,
        10067880064238660809_u64,
        5418857496715711651_u64,
    ])));
);
#[derive(Default)]
pub struct v8_base_hash_signed_char_ {}
impl Clone for v8_base_hash_signed_char_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_hash_signed_char_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_hash_signed_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_hash_signed_char_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_hash_unsigned_char_ {}
impl Clone for v8_base_hash_unsigned_char_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_hash_unsigned_char_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_hash_unsigned_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_hash_unsigned_char_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_hash_short_ {}
impl Clone for v8_base_hash_short_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_hash_short_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_hash_short_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_hash_short_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_hash_unsigned_short_ {}
impl Clone for v8_base_hash_unsigned_short_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_hash_unsigned_short_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_hash_unsigned_short_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_hash_unsigned_short_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_hash_int_ {}
impl Clone for v8_base_hash_int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_hash_int_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_hash_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_hash_int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_hash_unsigned_int_ {}
impl Clone for v8_base_hash_unsigned_int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_hash_unsigned_int_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_hash_unsigned_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_hash_unsigned_int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_hash_long_ {}
impl Clone for v8_base_hash_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_hash_long_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_hash_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_hash_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_hash_unsigned_long_ {}
impl Clone for v8_base_hash_unsigned_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_hash_unsigned_long_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_hash_unsigned_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_hash_unsigned_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_hash_long_long_ {}
impl Clone for v8_base_hash_long_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_hash_long_long_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_hash_long_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_hash_long_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_hash_unsigned_long_long_ {}
impl Clone for v8_base_hash_unsigned_long_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_hash_unsigned_long_long_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_hash_unsigned_long_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_hash_unsigned_long_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn hash_combine_94(seed: usize, hash: usize) -> usize {
    let seed: Value<usize> = Rc::new(RefCell::new(seed));
    let hash: Value<usize> = Rc::new(RefCell::new(hash));
    let m: Value<u64> = Rc::new(RefCell::new(14313749767032793493_u64));
    let r: Value<u32> = Rc::new(RefCell::new(47_u32));
    {
        let rhs_0 = (((*hash.borrow()) as u64).wrapping_mul((*m.borrow()))) as usize;
        (*hash.borrow_mut()) = rhs_0
    };
    let __rhs = ((*hash.borrow()) >> (*r.borrow()));
    (*hash.borrow_mut()) ^= __rhs;
    {
        let rhs_0 = (((*hash.borrow()) as u64).wrapping_mul((*m.borrow()))) as usize;
        (*hash.borrow_mut()) = rhs_0
    };
    (*seed.borrow_mut()) ^= (*hash.borrow());
    {
        let rhs_0 = (((*seed.borrow()) as u64).wrapping_mul((*m.borrow()))) as usize;
        (*seed.borrow_mut()) = rhs_0
    };
    return (*seed.borrow());
}
#[derive(Default)]
pub struct v8_base_Hasher {
    hash_: Value<usize>,
}
impl v8_base_Hasher {
    pub fn v8_base_Hasher1(seed: usize) -> Self {
        let seed: Value<usize> = Rc::new(RefCell::new(seed));
        let __this: Value<v8_base_Hasher> = Rc::new(RefCell::new(Self {
            hash_: Rc::new(RefCell::new((*seed.borrow()))),
        }));
        let this: Ptr<v8_base_Hasher> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_Hasher {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Hasher> = Rc::new(RefCell::new(Self {
            hash_: Rc::new(RefCell::new((*self.hash_.borrow()))),
        }));
        let this: Ptr<v8_base_Hasher> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_Hasher {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.hash_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            hash_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[0..8]))),
        }
    }
}
thread_local!(
    pub static kRapidhashSecret1_95: Value<u64> = Rc::new(RefCell::new(3257665815644502181));
);
thread_local!(
    pub static kRapidhashSecret2_96: Value<u64> = Rc::new(RefCell::new(10067880064238660809));
);
pub fn hash64_97(key: u64) -> u64 {
    let key: Value<u64> = Rc::new(RefCell::new(key));
    return ({
        let _A: u64 = ((*key.borrow()) ^ 3257665815644502181);
        let _B: u64 = ((*key.borrow()) ^ 10067880064238660809);
        rapid_mix_89(_A, _B)
    });
}
pub fn hash32_98(key: u32) -> u32 {
    let key: Value<u32> = Rc::new(RefCell::new(key));
    return (({ hash64_97(((*key.borrow()) as u64)) }) as u32);
}
pub fn hash_value_99(v: bool) -> usize {
    let v: Value<bool> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) as usize);
}
pub fn hash_value_100(v: u8) -> usize {
    let v: Value<u8> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) as usize);
}
pub fn hash_value_101(v: u16) -> usize {
    let v: Value<u16> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) as usize);
}
pub fn hash_value_92(v: u32) -> usize {
    let v: Value<u32> = Rc::new(RefCell::new(v));
    return (({ hash32_98((*v.borrow())) }) as usize);
}
pub fn hash_value_102(v: u64) -> usize {
    let v: Value<u64> = Rc::new(RefCell::new(v));
    if false {
        return (({ hash32_98(((*v.borrow()) as u32)) }) as usize);
    } else {
        return (({ hash64_97((*v.borrow())) }) as usize);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn hash_value_93(v: u64) -> usize {
    let v: Value<u64> = Rc::new(RefCell::new(v));
    return (({ hash64_97((*v.borrow())) }) as usize);
}
pub fn hash_value_103(v: i8) -> usize {
    let v: Value<i8> = Rc::new(RefCell::new(v));
    return ({ hash_value_100(({ bit_cast_16(v.as_pointer()) })) });
}
pub fn hash_value_104(v: i16) -> usize {
    let v: Value<i16> = Rc::new(RefCell::new(v));
    return ({ hash_value_101(({ bit_cast_18(v.as_pointer()) })) });
}
pub fn hash_value_105(v: i32) -> usize {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    return ({ hash_value_92(({ bit_cast_20(v.as_pointer()) })) });
}
pub fn hash_value_106(v: i64) -> usize {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    return ({ hash_value_102(({ bit_cast_22(v.as_pointer()) })) });
}
pub fn hash_value_107(v: i64) -> usize {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    return ({ hash_value_93(({ bit_cast_24(v.as_pointer()) })) });
}
pub fn hash_value_108(v: f32) -> usize {
    let v: Value<f32> = Rc::new(RefCell::new(v));
    return if ((*v.borrow()) != 0.0E+0) {
        ({ hash_value_92(({ bit_cast_26(v.as_pointer()) })) })
    } else {
        0_usize
    };
}
pub fn hash_value_109(v: f64) -> usize {
    let v: Value<f64> = Rc::new(RefCell::new(v));
    return if ((*v.borrow()) != 0.0E+0) {
        ({ hash_value_93(({ bit_cast_28(v.as_pointer()) })) })
    } else {
        0_usize
    };
}
#[derive(Default)]
pub struct v8_base_bit_equal_to_signed_char_ {}
impl Clone for v8_base_bit_equal_to_signed_char_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_equal_to_signed_char_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_equal_to_signed_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_equal_to_signed_char_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_equal_to_unsigned_char_ {}
impl Clone for v8_base_bit_equal_to_unsigned_char_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_equal_to_unsigned_char_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_equal_to_unsigned_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_equal_to_unsigned_char_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_equal_to_short_ {}
impl Clone for v8_base_bit_equal_to_short_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_equal_to_short_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_equal_to_short_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_equal_to_short_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_equal_to_unsigned_short_ {}
impl Clone for v8_base_bit_equal_to_unsigned_short_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_equal_to_unsigned_short_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_equal_to_unsigned_short_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_equal_to_unsigned_short_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_equal_to_int_ {}
impl Clone for v8_base_bit_equal_to_int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_equal_to_int_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_equal_to_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_equal_to_int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_equal_to_unsigned_int_ {}
impl Clone for v8_base_bit_equal_to_unsigned_int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_equal_to_unsigned_int_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_equal_to_unsigned_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_equal_to_unsigned_int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_equal_to_long_ {}
impl Clone for v8_base_bit_equal_to_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_equal_to_long_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_equal_to_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_equal_to_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_equal_to_unsigned_long_ {}
impl Clone for v8_base_bit_equal_to_unsigned_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_equal_to_unsigned_long_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_equal_to_unsigned_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_equal_to_unsigned_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_equal_to_long_long_ {}
impl Clone for v8_base_bit_equal_to_long_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_equal_to_long_long_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_equal_to_long_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_equal_to_long_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_equal_to_unsigned_long_long_ {}
impl Clone for v8_base_bit_equal_to_unsigned_long_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_equal_to_unsigned_long_long_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_equal_to_unsigned_long_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_equal_to_unsigned_long_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_equal_to_float_ {}
impl Clone for v8_base_bit_equal_to_float_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_equal_to_float_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_equal_to_float_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_equal_to_float_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_equal_to_double_ {}
impl Clone for v8_base_bit_equal_to_double_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_equal_to_double_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_equal_to_double_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_equal_to_double_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_hash_signed_char_ {}
impl Clone for v8_base_bit_hash_signed_char_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_hash_signed_char_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_hash_signed_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_hash_signed_char_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_hash_unsigned_char_ {}
impl Clone for v8_base_bit_hash_unsigned_char_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_hash_unsigned_char_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_hash_unsigned_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_hash_unsigned_char_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_hash_short_ {}
impl Clone for v8_base_bit_hash_short_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_hash_short_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_hash_short_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_hash_short_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_hash_unsigned_short_ {}
impl Clone for v8_base_bit_hash_unsigned_short_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_hash_unsigned_short_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_hash_unsigned_short_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_hash_unsigned_short_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_hash_int_ {}
impl Clone for v8_base_bit_hash_int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_hash_int_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_hash_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_hash_int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_hash_unsigned_int_ {}
impl Clone for v8_base_bit_hash_unsigned_int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_hash_unsigned_int_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_hash_unsigned_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_hash_unsigned_int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_hash_long_ {}
impl Clone for v8_base_bit_hash_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_hash_long_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_hash_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_hash_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_hash_unsigned_long_ {}
impl Clone for v8_base_bit_hash_unsigned_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_hash_unsigned_long_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_hash_unsigned_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_hash_unsigned_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_hash_long_long_ {}
impl Clone for v8_base_bit_hash_long_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_hash_long_long_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_hash_long_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_hash_long_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_hash_unsigned_long_long_ {}
impl Clone for v8_base_bit_hash_unsigned_long_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_hash_unsigned_long_long_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_hash_unsigned_long_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_hash_unsigned_long_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_hash_float_ {}
impl Clone for v8_base_bit_hash_float_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_hash_float_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_hash_float_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_hash_float_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_bit_hash_double_ {}
impl Clone for v8_base_bit_hash_double_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_bit_hash_double_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_bit_hash_double_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_bit_hash_double_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive()]
pub struct v8_base_Vector_const_char_ {
    start_: Value<Ptr<u8>>,
    length_: Value<usize>,
}
impl v8_base_Vector_const_char_ {
    pub fn v8_base_Vector_const_char_1(data: Ptr<u8>, length: usize) -> Self {
        let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
        let length: Value<usize> = Rc::new(RefCell::new(length));
        let __this: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(Self {
            start_: Rc::new(RefCell::new((*data.borrow()).clone())),
            length_: Rc::new(RefCell::new((*length.borrow()))),
        }));
        let this: Ptr<v8_base_Vector_const_char_> = __this.as_pointer();
        (&(0));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_Vector_const_char_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(Self {
            start_: Rc::new(RefCell::new((*self.start_.borrow()).clone())),
            length_: Rc::new(RefCell::new((*self.length_.borrow()))),
        }));
        let this: Ptr<v8_base_Vector_const_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_Vector_const_char_ {
    fn default() -> Self {
        { v8_base_Vector_const_char_::v8_base_Vector_const_char_2() }
    }
}
impl ByteRepr for v8_base_Vector_const_char_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.start_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.length_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            start_: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[0..8]))),
            length_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive()]
pub struct v8_base_Vector_const_unsigned_char_ {
    start_: Value<Ptr<u8>>,
    length_: Value<usize>,
}
impl v8_base_Vector_const_unsigned_char_ {
    pub fn v8_base_Vector_const_unsigned_char_3(data: Ptr<u8>, length: usize) -> Self {
        let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
        let length: Value<usize> = Rc::new(RefCell::new(length));
        let __this: Value<v8_base_Vector_const_unsigned_char_> = Rc::new(RefCell::new(Self {
            start_: Rc::new(RefCell::new((*data.borrow()).clone())),
            length_: Rc::new(RefCell::new((*length.borrow()))),
        }));
        let this: Ptr<v8_base_Vector_const_unsigned_char_> = __this.as_pointer();
        (&(0));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_Vector_const_unsigned_char_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Vector_const_unsigned_char_> = Rc::new(RefCell::new(Self {
            start_: Rc::new(RefCell::new((*self.start_.borrow()).clone())),
            length_: Rc::new(RefCell::new((*self.length_.borrow()))),
        }));
        let this: Ptr<v8_base_Vector_const_unsigned_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_Vector_const_unsigned_char_ {
    fn default() -> Self {
        { v8_base_Vector_const_unsigned_char_::v8_base_Vector_const_unsigned_char_4() }
    }
}
impl ByteRepr for v8_base_Vector_const_unsigned_char_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.start_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.length_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            start_: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[0..8]))),
            length_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive()]
pub struct v8_base_Vector_char_ {
    start_: Value<Ptr<u8>>,
    length_: Value<usize>,
}
impl v8_base_Vector_char_ {}
impl Clone for v8_base_Vector_char_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Vector_char_> = Rc::new(RefCell::new(Self {
            start_: Rc::new(RefCell::new((*self.start_.borrow()).clone())),
            length_: Rc::new(RefCell::new((*self.length_.borrow()))),
        }));
        let this: Ptr<v8_base_Vector_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_Vector_char_ {
    fn default() -> Self {
        { v8_base_Vector_char_::v8_base_Vector_char_5() }
    }
}
impl ByteRepr for v8_base_Vector_char_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.start_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.length_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            start_: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[0..8]))),
            length_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
        }
    }
}
thread_local!(
    pub static enable_view_110: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static enable_borrowed_range_111: Value<bool> = Rc::new(RefCell::new(true));
);
pub fn CStrVector_112(data: Ptr<u8>) -> v8_base_Vector_const_char_ {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    return v8_base_Vector_const_char_::v8_base_Vector_const_char_1(
        { (*data.borrow()).clone() },
        { (*data.borrow()).to_c_string_iterator().count() },
    );
}
pub fn StrVector_113(str: Vec<u8>) -> v8_base_Vector_const_char_ {
    let str: Value<Vec<u8>> = Rc::new(RefCell::new(str));
    return v8_base_Vector_const_char_::v8_base_Vector_const_char_1(
        { ({ (*str.borrow()).data() }) },
        { (*str.borrow()).len() },
    );
}
pub fn OneByteVector_114(data: Ptr<u8>, length: usize) -> v8_base_Vector_const_unsigned_char_ {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    return v8_base_Vector_const_unsigned_char_::v8_base_Vector_const_unsigned_char_3(
        { (*data.borrow()).reinterpret_cast::<u8>() },
        { (*length.borrow()) },
    );
}
pub fn OneByteVector_115(data: Ptr<u8>) -> v8_base_Vector_const_unsigned_char_ {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    return ({
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _length: usize = (*data.borrow()).to_c_string_iterator().count();
        OneByteVector_114(_data, _length)
    });
}
pub type v8_base_FastDtoaMode = u32;
pub const v8_base_FastDtoaMode_FAST_DTOA_SHORTEST: v8_base_FastDtoaMode = 0;
pub const v8_base_FastDtoaMode_FAST_DTOA_PRECISION: v8_base_FastDtoaMode = 1;
thread_local!(
    pub static kFastDtoaMaximalLength_116: Value<i32> = Rc::new(RefCell::new(17));
);
thread_local!();
#[derive(Default)]
pub struct AbslInternal_YouForgotToExplicitlyInitializeAField {}
impl Clone for AbslInternal_YouForgotToExplicitlyInitializeAField {
    fn clone(&self) -> Self {
        let __this: Value<AbslInternal_YouForgotToExplicitlyInitializeAField> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<AbslInternal_YouForgotToExplicitlyInitializeAField> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for AbslInternal_YouForgotToExplicitlyInitializeAField {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn HardeningAbort_118() {
    panic!("builtin trap");
    unreachable!();
}
#[derive(Default)]
struct absl_type_traits_internal_AssertHashEnabledHelper_NAT {}
impl Clone for absl_type_traits_internal_AssertHashEnabledHelper_NAT {
    fn clone(&self) -> Self {
        let __this: Value<absl_type_traits_internal_AssertHashEnabledHelper_NAT> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_type_traits_internal_AssertHashEnabledHelper_NAT> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_type_traits_internal_AssertHashEnabledHelper_NAT {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct absl_type_traits_internal_AssertHashEnabledHelper {}
impl absl_type_traits_internal_AssertHashEnabledHelper {
    fn Sink(__args: &[VaArg]) {}
}
impl Clone for absl_type_traits_internal_AssertHashEnabledHelper {
    fn clone(&self) -> Self {
        let __this: Value<absl_type_traits_internal_AssertHashEnabledHelper> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_type_traits_internal_AssertHashEnabledHelper> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_type_traits_internal_AssertHashEnabledHelper {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn is_constant_evaluated_119() -> bool {
    return ({ is_constant_evaluated_120() });
}
pub fn compare_result_as_less_than_121(r: std_weak_ordering) -> bool {
    let r: Value<std_weak_ordering> = Rc::new(RefCell::new(r));
    return operator_lt(
        (*r.borrow()).clone(),
        std__CmpUnspecifiedParam::std__CmpUnspecifiedParam({ 0 }),
    );
}
pub fn compare_result_as_ordering_122(c: std_weak_ordering) -> std_weak_ordering {
    let c: Value<std_weak_ordering> = Rc::new(RefCell::new(c));
    return (*c.borrow()).clone();
}
#[derive(Default)]
pub struct absl_uint128 {
    lo_: Value<u64>,
    hi_: Value<u64>,
}
impl absl_uint128 {
    pub fn absl_uint1281(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<absl_uint128> = Rc::new(RefCell::new(Self {
            lo_: Rc::new(RefCell::new(((*v.borrow()) as u64))),
            hi_: Rc::new(RefCell::new(if ((*v.borrow()) < 0) {
                <u64>::MAX
            } else {
                0_u64
            })),
        }));
        let this: Ptr<absl_uint128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_uint1282(v: u32) -> Self {
        let v: Value<u32> = Rc::new(RefCell::new(v));
        let __this: Value<absl_uint128> = Rc::new(RefCell::new(Self {
            lo_: Rc::new(RefCell::new(((*v.borrow()) as u64))),
            hi_: Rc::new(RefCell::new(0_u64)),
        }));
        let this: Ptr<absl_uint128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_uint1283(v: i64) -> Self {
        let v: Value<i64> = Rc::new(RefCell::new(v));
        let __this: Value<absl_uint128> = Rc::new(RefCell::new(Self {
            lo_: Rc::new(RefCell::new(((*v.borrow()) as u64))),
            hi_: Rc::new(RefCell::new(if ((*v.borrow()) < 0_i64) {
                <u64>::MAX
            } else {
                0_u64
            })),
        }));
        let this: Ptr<absl_uint128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_uint1284(v: u64) -> Self {
        let v: Value<u64> = Rc::new(RefCell::new(v));
        let __this: Value<absl_uint128> = Rc::new(RefCell::new(Self {
            lo_: Rc::new(RefCell::new((*v.borrow()))),
            hi_: Rc::new(RefCell::new(0_u64)),
        }));
        let this: Ptr<absl_uint128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_uint1285(v: i64) -> Self {
        let v: Value<i64> = Rc::new(RefCell::new(v));
        let __this: Value<absl_uint128> = Rc::new(RefCell::new(Self {
            lo_: Rc::new(RefCell::new(((*v.borrow()) as u64))),
            hi_: Rc::new(RefCell::new(if ((*v.borrow()) < 0_i64) {
                <u64>::MAX
            } else {
                0_u64
            })),
        }));
        let this: Ptr<absl_uint128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_uint1286(v: u64) -> Self {
        let v: Value<u64> = Rc::new(RefCell::new(v));
        let __this: Value<absl_uint128> = Rc::new(RefCell::new(Self {
            lo_: Rc::new(RefCell::new((*v.borrow()))),
            hi_: Rc::new(RefCell::new(0_u64)),
        }));
        let this: Ptr<absl_uint128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_uint1287(v: i128) -> Self {
        let v: Value<i128> = Rc::new(RefCell::new(v));
        let __this: Value<absl_uint128> = Rc::new(RefCell::new(Self {
            lo_: Rc::new(RefCell::new((((*v.borrow()) & (!0_u64 as i128)) as u64))),
            hi_: Rc::new(RefCell::new(((((*v.borrow()) as u128) >> 64) as u64))),
        }));
        let this: Ptr<absl_uint128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_uint1288(v: u128) -> Self {
        let v: Value<u128> = Rc::new(RefCell::new(v));
        let __this: Value<absl_uint128> = Rc::new(RefCell::new(Self {
            lo_: Rc::new(RefCell::new((((*v.borrow()) & (!0_u64 as u128)) as u64))),
            hi_: Rc::new(RefCell::new((((*v.borrow()) >> 64) as u64))),
        }));
        let this: Ptr<absl_uint128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_uint1289(v: absl_int128) -> Self {
        let v: Value<absl_int128> = Rc::new(RefCell::new(v));
        let __this: Value<absl_uint128> = Rc::new(RefCell::new(Self {
            lo_: Rc::new(RefCell::new(({ Int128Low64_123((*v.borrow()).clone()) }))),
            hi_: Rc::new(RefCell::new(
                (({ Int128High64_124((*v.borrow()).clone()) }) as u64),
            )),
        }));
        let this: Ptr<absl_uint128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    fn absl_uint12810(high: u64, low: u64) -> Self {
        let high: Value<u64> = Rc::new(RefCell::new(high));
        let low: Value<u64> = Rc::new(RefCell::new(low));
        let __this: Value<absl_uint128> = Rc::new(RefCell::new(Self {
            lo_: Rc::new(RefCell::new((*low.borrow()))),
            hi_: Rc::new(RefCell::new((*high.borrow()))),
        }));
        let this: Ptr<absl_uint128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for absl_uint128 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_125(
                Rc::new(RefCell::new(absl_uint128 {
                    lo_: self.lo_.clone(),
                    hi_: self.hi_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(absl_uint128 {
                    lo_: other.lo_.clone(),
                    hi_: other.hi_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for absl_uint128 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for absl_uint128 {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_126(
                Rc::new(RefCell::new(absl_uint128 {
                    lo_: self.lo_.clone(),
                    hi_: self.hi_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(absl_uint128 {
                    lo_: other.lo_.clone(),
                    hi_: other.hi_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for absl_uint128 {}
impl Clone for absl_uint128 {
    fn clone(&self) -> Self {
        let __this: Value<absl_uint128> = Rc::new(RefCell::new(Self {
            lo_: Rc::new(RefCell::new((*self.lo_.borrow()))),
            hi_: Rc::new(RefCell::new((*self.hi_.borrow()))),
        }));
        let this: Ptr<absl_uint128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_uint128 {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.lo_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.hi_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            lo_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
            hi_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn Uint128Max_127() -> absl_uint128 {
    return absl_uint128::absl_uint12810({ <u64>::MAX }, { <u64>::MAX });
}
thread_local!(
    pub static is_specialized_128: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_signed_129: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static is_integer_130: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_exact_131: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static has_infinity_132: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_quiet_NaN_133: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_signaling_NaN_134: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_denorm_135: Value = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static has_denorm_loss_136: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static round_style_137: Value = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static is_iec559_138: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static is_bounded_139: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_modulo_140: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static digits_141: Value<i32> = Rc::new(RefCell::new(128));
);
thread_local!(
    pub static digits10_142: Value<i32> = Rc::new(RefCell::new(38));
);
thread_local!(
    pub static max_digits10_143: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static radix_144: Value<i32> = Rc::new(RefCell::new(2));
);
thread_local!(
    pub static min_exponent_145: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static min_exponent10_146: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static max_exponent_147: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static max_exponent10_148: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static traps_149: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static tinyness_before_150: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct std_numeric_limits_absl_uint128_ {}
impl std_numeric_limits_absl_uint128_ {
    pub fn min() -> absl_uint128 {
        return absl_uint128::absl_uint1281({ 0 });
    }
    pub fn lowest() -> absl_uint128 {
        return absl_uint128::absl_uint1281({ 0 });
    }
    pub fn max() -> absl_uint128 {
        return ({ Uint128Max_127() });
    }
    pub fn epsilon() -> absl_uint128 {
        return absl_uint128::absl_uint1281({ 0 });
    }
    pub fn round_error() -> absl_uint128 {
        return absl_uint128::absl_uint1281({ 0 });
    }
    pub fn infinity() -> absl_uint128 {
        return absl_uint128::absl_uint1281({ 0 });
    }
    pub fn quiet_NaN() -> absl_uint128 {
        return absl_uint128::absl_uint1281({ 0 });
    }
    pub fn signaling_NaN() -> absl_uint128 {
        return absl_uint128::absl_uint1281({ 0 });
    }
    pub fn denorm_min() -> absl_uint128 {
        return absl_uint128::absl_uint1281({ 0 });
    }
}
impl Clone for std_numeric_limits_absl_uint128_ {
    fn clone(&self) -> Self {
        let __this: Value<std_numeric_limits_absl_uint128_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<std_numeric_limits_absl_uint128_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for std_numeric_limits_absl_uint128_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct absl_int128 {
    v_: Value<i128>,
}
impl absl_int128 {
    pub fn absl_int1281(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<absl_int128> = Rc::new(RefCell::new(Self {
            v_: Rc::new(RefCell::new(((*v.borrow()) as i128))),
        }));
        let this: Ptr<absl_int128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_int1282(v: u32) -> Self {
        let v: Value<u32> = Rc::new(RefCell::new(v));
        let __this: Value<absl_int128> = Rc::new(RefCell::new(Self {
            v_: Rc::new(RefCell::new(((*v.borrow()) as i128))),
        }));
        let this: Ptr<absl_int128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_int1283(v: i64) -> Self {
        let v: Value<i64> = Rc::new(RefCell::new(v));
        let __this: Value<absl_int128> = Rc::new(RefCell::new(Self {
            v_: Rc::new(RefCell::new(((*v.borrow()) as i128))),
        }));
        let this: Ptr<absl_int128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_int1284(v: u64) -> Self {
        let v: Value<u64> = Rc::new(RefCell::new(v));
        let __this: Value<absl_int128> = Rc::new(RefCell::new(Self {
            v_: Rc::new(RefCell::new(((*v.borrow()) as i128))),
        }));
        let this: Ptr<absl_int128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_int1285(v: i64) -> Self {
        let v: Value<i64> = Rc::new(RefCell::new(v));
        let __this: Value<absl_int128> = Rc::new(RefCell::new(Self {
            v_: Rc::new(RefCell::new(((*v.borrow()) as i128))),
        }));
        let this: Ptr<absl_int128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_int1286(v: u64) -> Self {
        let v: Value<u64> = Rc::new(RefCell::new(v));
        let __this: Value<absl_int128> = Rc::new(RefCell::new(Self {
            v_: Rc::new(RefCell::new(((*v.borrow()) as i128))),
        }));
        let this: Ptr<absl_int128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_int1287(v: absl_uint128) -> Self {
        let v: Value<absl_uint128> = Rc::new(RefCell::new(v));
        let __this: Value<absl_int128> = Rc::new(RefCell::new(Self {
            v_: Rc::new(RefCell::new(
                ({ absl_uint128Impl::operator___int128(&v.as_pointer()) }),
            )),
        }));
        let this: Ptr<absl_int128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_int1288(v: i128) -> Self {
        let v: Value<i128> = Rc::new(RefCell::new(v));
        let __this: Value<absl_int128> = Rc::new(RefCell::new(Self {
            v_: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<absl_int128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_int1289(v: u128) -> Self {
        let v: Value<u128> = Rc::new(RefCell::new(v));
        let __this: Value<absl_int128> = Rc::new(RefCell::new(Self {
            v_: Rc::new(RefCell::new(((*v.borrow()) as i128))),
        }));
        let this: Ptr<absl_int128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_int12810(v: f32) -> Self {
        let v: Value<f32> = Rc::new(RefCell::new(v));
        let __this: Value<absl_int128> = Rc::new(RefCell::new(Self {
            v_: Rc::new(RefCell::new(((*v.borrow()) as i128))),
        }));
        let this: Ptr<absl_int128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_int12811(v: f64) -> Self {
        let v: Value<f64> = Rc::new(RefCell::new(v));
        let __this: Value<absl_int128> = Rc::new(RefCell::new(Self {
            v_: Rc::new(RefCell::new(((*v.borrow()) as i128))),
        }));
        let this: Ptr<absl_int128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_int12812(v: f64) -> Self {
        let v: Value<f64> = Rc::new(RefCell::new(v));
        let __this: Value<absl_int128> = Rc::new(RefCell::new(Self {
            v_: Rc::new(RefCell::new(((*v.borrow()) as i128))),
        }));
        let this: Ptr<absl_int128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    fn absl_int12813(high: i64, low: u64) -> Self {
        let high: Value<i64> = Rc::new(RefCell::new(high));
        let low: Value<u64> = Rc::new(RefCell::new(low));
        let __this: Value<absl_int128> = Rc::new(RefCell::new(Self {
            v_: Rc::new(RefCell::new(
                (({ BitCastToSigned_151((((*high.borrow()) as u128) << 64)) })
                    | ((*low.borrow()) as i128)),
            )),
        }));
        let this: Ptr<absl_int128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for absl_int128 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_152(
                Rc::new(RefCell::new(absl_int128 {
                    v_: self.v_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(absl_int128 {
                    v_: other.v_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for absl_int128 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for absl_int128 {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_153(
                Rc::new(RefCell::new(absl_int128 {
                    v_: self.v_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(absl_int128 {
                    v_: other.v_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for absl_int128 {}
impl Clone for absl_int128 {
    fn clone(&self) -> Self {
        let __this: Value<absl_int128> = Rc::new(RefCell::new(Self {
            v_: Rc::new(RefCell::new((*self.v_.borrow()))),
        }));
        let this: Ptr<absl_int128> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_int128 {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v_.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v_: Rc::new(RefCell::new(<i128>::from_bytes(&buf[0..16]))),
        }
    }
}
pub fn Int128Max_154() -> absl_int128 {
    return absl_int128::absl_int12813({ <i64>::MAX }, { <u64>::MAX });
}
pub fn Int128Min_155() -> absl_int128 {
    return absl_int128::absl_int12813({ <i64>::MIN }, { 0_u64 });
}
thread_local!(
    pub static is_specialized_156: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_signed_157: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_integer_158: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_exact_159: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static has_infinity_160: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_quiet_NaN_161: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_signaling_NaN_162: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_denorm_163: Value = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static has_denorm_loss_164: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static round_style_165: Value = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static is_iec559_166: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static is_bounded_167: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_modulo_168: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static digits_169: Value<i32> = Rc::new(RefCell::new(127));
);
thread_local!(
    pub static digits10_170: Value<i32> = Rc::new(RefCell::new(38));
);
thread_local!(
    pub static max_digits10_171: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static radix_172: Value<i32> = Rc::new(RefCell::new(2));
);
thread_local!(
    pub static min_exponent_173: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static min_exponent10_174: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static max_exponent_175: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static max_exponent10_176: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static traps_177: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static tinyness_before_178: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct std_numeric_limits_absl_int128_ {}
impl std_numeric_limits_absl_int128_ {
    pub fn min() -> absl_int128 {
        return ({ Int128Min_155() });
    }
    pub fn lowest() -> absl_int128 {
        return ({ Int128Min_155() });
    }
    pub fn max() -> absl_int128 {
        return ({ Int128Max_154() });
    }
    pub fn epsilon() -> absl_int128 {
        return absl_int128::absl_int1281({ 0 });
    }
    pub fn round_error() -> absl_int128 {
        return absl_int128::absl_int1281({ 0 });
    }
    pub fn infinity() -> absl_int128 {
        return absl_int128::absl_int1281({ 0 });
    }
    pub fn quiet_NaN() -> absl_int128 {
        return absl_int128::absl_int1281({ 0 });
    }
    pub fn signaling_NaN() -> absl_int128 {
        return absl_int128::absl_int1281({ 0 });
    }
    pub fn denorm_min() -> absl_int128 {
        return absl_int128::absl_int1281({ 0 });
    }
}
impl Clone for std_numeric_limits_absl_int128_ {
    fn clone(&self) -> Self {
        let __this: Value<std_numeric_limits_absl_int128_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<std_numeric_limits_absl_int128_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for std_numeric_limits_absl_int128_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn MakeUint128_179(high: u64, low: u64) -> absl_uint128 {
    let high: Value<u64> = Rc::new(RefCell::new(high));
    let low: Value<u64> = Rc::new(RefCell::new(low));
    return absl_uint128::absl_uint12810({ (*high.borrow()) }, { (*low.borrow()) });
}
pub fn Uint128Low64_187(v: absl_uint128) -> u64 {
    let v: Value<absl_uint128> = Rc::new(RefCell::new(v));
    return (*(*v.borrow()).lo_.borrow());
}
pub fn Uint128High64_188(v: absl_uint128) -> u64 {
    let v: Value<absl_uint128> = Rc::new(RefCell::new(v));
    return (*(*v.borrow()).hi_.borrow());
}
pub fn operator_eq_126(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
        == ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }));
}
pub fn operator_ne_189(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_uint128 = (*lhs.borrow()).clone();
        operator_eq_126(_lhs, (*rhs.borrow()).clone())
    });
}
pub fn operator_lt_190(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
        < ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }));
}
pub fn operator_gt_191(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_uint128 = (*rhs.borrow()).clone();
        operator_lt_190(_lhs, (*lhs.borrow()).clone())
    });
}
pub fn operator_le_192(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_uint128 = (*rhs.borrow()).clone();
        operator_lt_190(_lhs, (*lhs.borrow()).clone())
    });
}
pub fn operator_ge_193(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_uint128 = (*lhs.borrow()).clone();
        operator_lt_190(_lhs, (*rhs.borrow()).clone())
    });
}
pub fn operator_cmp_125(lhs: absl_uint128, rhs: absl_uint128) -> std::cmp::Ordering {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    {
        let lhs_128: Value<u128> = Rc::new(RefCell::new(
            ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) }),
        ));
        let rhs_128: Value<u128> = Rc::new(RefCell::new(
            ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }),
        ));
        if ((*lhs_128.borrow()) < (*rhs_128.borrow())) {
            return std::cmp::Ordering::Less;
        } else if ((*lhs_128.borrow()) > (*rhs_128.borrow())) {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Equal;
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub fn operator_pos_194(val: absl_uint128) -> absl_uint128 {
    let val: Value<absl_uint128> = Rc::new(RefCell::new(val));
    return (*val.borrow()).clone();
}
pub fn operator_pos_195(val: absl_int128) -> absl_int128 {
    let val: Value<absl_int128> = Rc::new(RefCell::new(val));
    return (*val.borrow()).clone();
}
pub fn operator_neg_196(val: absl_uint128) -> absl_uint128 {
    let val: Value<absl_uint128> = Rc::new(RefCell::new(val));
    return absl_uint128::absl_uint1288({
        -({ absl_uint128Impl::operator_unsigned___int128(&val.as_pointer()) })
    });
}
pub fn operator_not_197(val: absl_uint128) -> bool {
    let val: Value<absl_uint128> = Rc::new(RefCell::new(val));
    return !(({ absl_uint128Impl::operator_unsigned___int128(&val.as_pointer()) }) != 0);
}
pub fn operator_bitnot_198(val: absl_uint128) -> absl_uint128 {
    let val: Value<absl_uint128> = Rc::new(RefCell::new(val));
    return absl_uint128::absl_uint1288({
        !({ absl_uint128Impl::operator_unsigned___int128(&val.as_pointer()) })
    });
}
pub fn operator_bitor_199(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            | ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitand_200(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            & ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitxor_201(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            ^ ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_shl_180(lhs: absl_uint128, amount: i32) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let amount: Value<i32> = Rc::new(RefCell::new(amount));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            << (*amount.borrow()))
    });
}
pub fn operator_shr_181(lhs: absl_uint128, amount: i32) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let amount: Value<i32> = Rc::new(RefCell::new(amount));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            >> (*amount.borrow()))
    });
}
pub fn operator_add_182(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_add(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_sub_183(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_sub(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_mul_184(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_mul(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_div_185(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_div(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_rem_186(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_rem(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn MakeInt128_202(high: i64, low: u64) -> absl_int128 {
    let high: Value<i64> = Rc::new(RefCell::new(high));
    let low: Value<u64> = Rc::new(RefCell::new(low));
    return absl_int128::absl_int12813({ (*high.borrow()) }, { (*low.borrow()) });
}
pub fn BitCastToSigned_213(v: u64) -> i64 {
    let v: Value<u64> = Rc::new(RefCell::new(v));
    return if (((*v.borrow()) & (1_u64 << 63)) != 0) {
        !(!(*v.borrow()) as i64)
    } else {
        ((*v.borrow()) as i64)
    };
}
pub fn BitCastToSigned_151(v: u128) -> i128 {
    let v: Value<u128> = Rc::new(RefCell::new(v));
    return if (((*v.borrow()) & (1_u128 << 127)) != 0) {
        !(!(*v.borrow()) as i128)
    } else {
        ((*v.borrow()) as i128)
    };
}
pub fn Int128Low64_123(v: absl_int128) -> u64 {
    let v: Value<absl_int128> = Rc::new(RefCell::new(v));
    return (((*(*v.borrow()).v_.borrow()) & (!0_u64 as i128)) as u64);
}
pub fn Int128High64_124(v: absl_int128) -> i64 {
    let v: Value<absl_int128> = Rc::new(RefCell::new(v));
    return ({ BitCastToSigned_213(((((*(*v.borrow()).v_.borrow()) as u128) >> 64) as u64)) });
}
pub fn operator_eq_153(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        == ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_ne_214(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        != ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_lt_215(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        < ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_gt_216(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        > ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_le_217(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        <= ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_ge_218(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        >= ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_cmp_152(lhs: absl_int128, rhs: absl_int128) -> std::cmp::Ordering {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    {
        let lhs_128: Value<i128> = Rc::new(RefCell::new(
            ({ absl_int128Impl::operator___int128(&lhs.as_pointer()) }),
        ));
        let rhs_128: Value<i128> = Rc::new(RefCell::new(
            ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }),
        ));
        if ((*lhs_128.borrow()) < (*rhs_128.borrow())) {
            return std::cmp::Ordering::Less;
        } else if ((*lhs_128.borrow()) > (*rhs_128.borrow())) {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Equal;
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub fn operator_neg_219(v: absl_int128) -> absl_int128 {
    let v: Value<absl_int128> = Rc::new(RefCell::new(v));
    return absl_int128::absl_int1288({
        -({ absl_int128Impl::operator___int128(&v.as_pointer()) })
    });
}
pub fn operator_not_220(v: absl_int128) -> bool {
    let v: Value<absl_int128> = Rc::new(RefCell::new(v));
    return !(({ absl_int128Impl::operator___int128(&v.as_pointer()) }) != 0);
}
pub fn operator_bitnot_221(val: absl_int128) -> absl_int128 {
    let val: Value<absl_int128> = Rc::new(RefCell::new(val));
    return absl_int128::absl_int1288({
        !({ absl_int128Impl::operator___int128(&val.as_pointer()) })
    });
}
pub fn operator_add_203(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            + ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_sub_204(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            - ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_mul_205(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            * ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_div_206(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            / ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_rem_207(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            % ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitor_208(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            | ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitand_209(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            & ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitxor_210(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            ^ ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_shl_211(lhs: absl_int128, amount: i32) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let amount: Value<i32> = Rc::new(RefCell::new(amount));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) }) << (*amount.borrow()))
    });
}
pub fn operator_shr_212(lhs: absl_int128, amount: i32) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let amount: Value<i32> = Rc::new(RefCell::new(amount));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) }) >> (*amount.borrow()))
    });
}
thread_local!(
    pub static kSignificandSize_222: Value<i32> = Rc::new(RefCell::new(64));
);
thread_local!(
    static kUint64MSB_223: Value<u64> = Rc::new(RefCell::new((1_u64 << 63)));
);
#[derive()]
pub struct v8_base_DiyFp {
    f_: Value<u64>,
    e_: Value<i32>,
}
impl v8_base_DiyFp {
    pub fn v8_base_DiyFp1() -> Self {
        let __this: Value<v8_base_DiyFp> = Rc::new(RefCell::new(Self {
            f_: Rc::new(RefCell::new(0_u64)),
            e_: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<v8_base_DiyFp> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_base_DiyFp2(f: u64, e: i32) -> Self {
        let f: Value<u64> = Rc::new(RefCell::new(f));
        let e: Value<i32> = Rc::new(RefCell::new(e));
        let __this: Value<v8_base_DiyFp> = Rc::new(RefCell::new(Self {
            f_: Rc::new(RefCell::new((*f.borrow()))),
            e_: Rc::new(RefCell::new((*e.borrow()))),
        }));
        let this: Ptr<v8_base_DiyFp> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Minus(a: Ptr<v8_base_DiyFp>, b: Ptr<v8_base_DiyFp>) -> v8_base_DiyFp {
        let result: Value<v8_base_DiyFp> = Rc::new(RefCell::new((*a.upgrade().deref()).clone()));
        ({
            let _other: Ptr<v8_base_DiyFp> = (b).clone();
            v8_base_DiyFpImpl::Subtract(&result.as_pointer(), _other)
        });
        return (*result.borrow()).clone();
    }
    pub fn Times(a: Ptr<v8_base_DiyFp>, b: Ptr<v8_base_DiyFp>) -> v8_base_DiyFp {
        let mul: Value<absl_uint128> = Rc::new(RefCell::new(
            ({
                let _lhs: absl_uint128 =
                    absl_uint128::absl_uint1286({ (*(*a.upgrade().deref()).f_.borrow()) });
                let _rhs: absl_uint128 =
                    absl_uint128::absl_uint1286({ (*(*b.upgrade().deref()).f_.borrow()) });
                operator_mul_184(_lhs, _rhs)
            }),
        ));
        let hi: Value<u64> = Rc::new(RefCell::new(
            ({ Uint128High64_188((*mul.borrow()).clone()) }),
        ));
        let lo: Value<u64> = Rc::new(RefCell::new(
            ({ Uint128Low64_187((*mul.borrow()).clone()) }),
        ));
        return v8_base_DiyFp::v8_base_DiyFp2(
            { (*hi.borrow()).wrapping_add(((*lo.borrow()) >> 63)) },
            {
                ({
                    let _lhs = (*(*a.upgrade().deref()).e_.borrow());
                    _lhs + (*(*b.upgrade().deref()).e_.borrow())
                } + 64)
            },
        );
    }
    pub fn Normalize_pconstv8_base_DiyFp(a: Ptr<v8_base_DiyFp>) -> v8_base_DiyFp {
        let result: Value<v8_base_DiyFp> = Rc::new(RefCell::new((*a.upgrade().deref()).clone()));
        ({ v8_base_DiyFpImpl::Normalize(&result.as_pointer()) });
        return (*result.borrow()).clone();
    }
}
impl Clone for v8_base_DiyFp {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_DiyFp> = Rc::new(RefCell::new(Self {
            f_: Rc::new(RefCell::new((*self.f_.borrow()))),
            e_: Rc::new(RefCell::new((*self.e_.borrow()))),
        }));
        let this: Ptr<v8_base_DiyFp> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_DiyFp {
    fn default() -> Self {
        { v8_base_DiyFp::v8_base_DiyFp1() }
    }
}
impl ByteRepr for v8_base_DiyFp {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.f_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.e_.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            f_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
            e_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
thread_local!();
thread_local!();
thread_local!();
#[derive(Default)]
pub struct v8_base_PowersOfTenCache {}
impl Clone for v8_base_PowersOfTenCache {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_PowersOfTenCache> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_PowersOfTenCache> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_PowersOfTenCache {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn double_to_uint64_227(d: f64) -> u64 {
    let d: Value<f64> = Rc::new(RefCell::new(d));
    return ({ bit_cast_28(d.as_pointer()) });
}
pub fn uint64_to_double_228(d64: u64) -> f64 {
    let d64: Value<u64> = Rc::new(RefCell::new(d64));
    return ({ bit_cast_30(d64.as_pointer()) });
}
thread_local!(
    pub static kSignMask_229: Value<u64> = Rc::new(RefCell::new(9223372036854775808));
);
thread_local!(
    pub static kExponentMask_230: Value<u64> = Rc::new(RefCell::new(9218868437227405312));
);
thread_local!(
    pub static kSignificandMask_231: Value<u64> = Rc::new(RefCell::new(4503599627370495));
);
thread_local!(
    pub static kHiddenBit_232: Value<u64> = Rc::new(RefCell::new(4503599627370496));
);
thread_local!(
    pub static kPhysicalSignificandSize_233: Value<i32> = Rc::new(RefCell::new(52));
);
thread_local!(
    pub static kSignificandSize_234: Value<i32> = Rc::new(RefCell::new(53));
);
thread_local!(
    static kExponentBias_235: Value<i32> = Rc::new(RefCell::new(1075));
);
thread_local!(
    static kDenormalExponent_236: Value<i32> = Rc::new(RefCell::new(-1074));
);
thread_local!(
    static kMaxExponent_237: Value<i32> = Rc::new(RefCell::new(972));
);
thread_local!(
    static kInfinity_238: Value<u64> = Rc::new(RefCell::new(9218868437227405312));
);
#[derive()]
pub struct v8_base_Double {
    d64_: Value<u64>,
}
impl v8_base_Double {
    pub fn v8_base_Double1() -> Self {
        let __this: Value<v8_base_Double> = Rc::new(RefCell::new(Self {
            d64_: Rc::new(RefCell::new(0_u64)),
        }));
        let this: Ptr<v8_base_Double> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_base_Double2(d: f64) -> Self {
        let d: Value<f64> = Rc::new(RefCell::new(d));
        let __this: Value<v8_base_Double> = Rc::new(RefCell::new(Self {
            d64_: Rc::new(RefCell::new(({ double_to_uint64_227((*d.borrow())) }))),
        }));
        let this: Ptr<v8_base_Double> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_base_Double3(d64: u64) -> Self {
        let d64: Value<u64> = Rc::new(RefCell::new(d64));
        let __this: Value<v8_base_Double> = Rc::new(RefCell::new(Self {
            d64_: Rc::new(RefCell::new((*d64.borrow()))),
        }));
        let this: Ptr<v8_base_Double> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_base_Double4(diy_fp: v8_base_DiyFp) -> Self {
        let diy_fp: Value<v8_base_DiyFp> = Rc::new(RefCell::new(diy_fp));
        let __this: Value<v8_base_Double> = Rc::new(RefCell::new(Self {
            d64_: Rc::new(RefCell::new(
                ({ v8_base_Double::DiyFpToUint64((*diy_fp.borrow()).clone()) }),
            )),
        }));
        let this: Ptr<v8_base_Double> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn SignificandSizeForOrderOfMagnitude(order: i32) -> i32 {
        let order: Value<i32> = Rc::new(RefCell::new(order));
        if ((*order.borrow()) >= (-1074 + 53)) {
            return 53;
        }
        if ((*order.borrow()) <= -1074) {
            return 0;
        }
        return ((*order.borrow()) - -1074);
    }
    fn DiyFpToUint64(diy_fp: v8_base_DiyFp) -> u64 {
        let diy_fp: Value<v8_base_DiyFp> = Rc::new(RefCell::new(diy_fp));
        let significand: Value<u64> = Rc::new(RefCell::new(
            ({ v8_base_DiyFpImpl::f(&diy_fp.as_pointer()) }),
        ));
        let exponent: Value<i32> = Rc::new(RefCell::new(
            ({ v8_base_DiyFpImpl::e(&diy_fp.as_pointer()) }),
        ));
        'loop_: while ((*significand.borrow()) > (4503599627370496).wrapping_add(4503599627370495))
        {
            (*significand.borrow_mut()) >>= 1;
            (*exponent.borrow_mut()).postfix_inc();
        }
        if ((*exponent.borrow()) >= 972) {
            return 9218868437227405312;
        }
        if ((*exponent.borrow()) < -1074) {
            return 0_u64;
        }
        'loop_: while ((*exponent.borrow()) > -1074)
            && (((*significand.borrow()) & 4503599627370496) == 0_u64)
        {
            (*significand.borrow_mut()) <<= 1;
            (*exponent.borrow_mut()).postfix_dec();
        }
        let biased_exponent: Value<u64> = <Value<u64>>::default();
        if ((*exponent.borrow()) == -1074)
            && (((*significand.borrow()) & 4503599627370496) == 0_u64)
        {
            (*biased_exponent.borrow_mut()) = 0_u64;
        } else {
            (*biased_exponent.borrow_mut()) = (((*exponent.borrow()) + 1075) as u64);
        }
        return (((*significand.borrow()) & 4503599627370495)
            | ((*biased_exponent.borrow()) << 52));
    }
}
impl Clone for v8_base_Double {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Double> = Rc::new(RefCell::new(Self {
            d64_: Rc::new(RefCell::new((*self.d64_.borrow()))),
        }));
        let this: Ptr<v8_base_Double> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_Double {
    fn default() -> Self {
        { v8_base_Double::v8_base_Double1() }
    }
}
impl ByteRepr for v8_base_Double {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.d64_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            d64_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
        }
    }
}
thread_local!(
    pub static kMinimalTargetExponent_239: Value<i32> = Rc::new(RefCell::new(-60_i32));
);
thread_local!(
    pub static kMaximalTargetExponent_240: Value<i32> = Rc::new(RefCell::new(-32_i32));
);
pub fn RoundWeed_241(
    last_digit: Ptr<u8>,
    distance_too_high_w: u64,
    unsafe_interval: u64,
    rest: u64,
    ten_kappa: u64,
    unit: u64,
) -> bool {
    let last_digit: Value<Ptr<u8>> = Rc::new(RefCell::new(last_digit));
    let distance_too_high_w: Value<u64> = Rc::new(RefCell::new(distance_too_high_w));
    let unsafe_interval: Value<u64> = Rc::new(RefCell::new(unsafe_interval));
    let rest: Value<u64> = Rc::new(RefCell::new(rest));
    let ten_kappa: Value<u64> = Rc::new(RefCell::new(ten_kappa));
    let unit: Value<u64> = Rc::new(RefCell::new(unit));
    let small_distance: Value<u64> = Rc::new(RefCell::new(
        (*distance_too_high_w.borrow()).wrapping_sub((*unit.borrow())),
    ));
    let big_distance: Value<u64> = Rc::new(RefCell::new(
        (*distance_too_high_w.borrow()).wrapping_add((*unit.borrow())),
    ));
    (&(0));
    'loop_: while (((*rest.borrow()) < (*small_distance.borrow()))
        && ((*unsafe_interval.borrow()).wrapping_sub((*rest.borrow())) >= (*ten_kappa.borrow())))
        && (((*rest.borrow()).wrapping_add((*ten_kappa.borrow())) < (*small_distance.borrow()))
            || ((*small_distance.borrow()).wrapping_sub((*rest.borrow()))
                >= ((*rest.borrow()).wrapping_add((*ten_kappa.borrow())))
                    .wrapping_sub((*small_distance.borrow()))))
    {
        (*last_digit.borrow()).with_mut(|__v| __v.prefix_dec());
        {
            let rhs_0 = (*rest.borrow()).wrapping_add((*ten_kappa.borrow()));
            (*rest.borrow_mut()) = rhs_0
        };
    }
    if (((*rest.borrow()) < (*big_distance.borrow()))
        && ((*unsafe_interval.borrow()).wrapping_sub((*rest.borrow())) >= (*ten_kappa.borrow())))
        && (((*rest.borrow()).wrapping_add((*ten_kappa.borrow())) < (*big_distance.borrow()))
            || ((*big_distance.borrow()).wrapping_sub((*rest.borrow()))
                > ((*rest.borrow()).wrapping_add((*ten_kappa.borrow())))
                    .wrapping_sub((*big_distance.borrow()))))
    {
        return false;
    }
    return ((2_u64).wrapping_mul((*unit.borrow())) <= (*rest.borrow()))
        && ((*rest.borrow())
            <= (*unsafe_interval.borrow()).wrapping_sub((4_u64).wrapping_mul((*unit.borrow()))));
}
pub fn RoundWeedCounted_242(
    buffer: v8_base_Vector_char_,
    length: i32,
    rest: u64,
    ten_kappa: u64,
    unit: u64,
    kappa: Ptr<i32>,
) -> bool {
    let buffer: Value<v8_base_Vector_char_> = Rc::new(RefCell::new(buffer));
    let length: Value<i32> = Rc::new(RefCell::new(length));
    let rest: Value<u64> = Rc::new(RefCell::new(rest));
    let ten_kappa: Value<u64> = Rc::new(RefCell::new(ten_kappa));
    let unit: Value<u64> = Rc::new(RefCell::new(unit));
    let kappa: Value<Ptr<i32>> = Rc::new(RefCell::new(kappa));
    (&(0));
    if ((*unit.borrow()) >= (*ten_kappa.borrow())) {
        return false;
    }
    if ((*ten_kappa.borrow()).wrapping_sub((*unit.borrow())) <= (*unit.borrow())) {
        return false;
    }
    if ((*ten_kappa.borrow()).wrapping_sub((*rest.borrow())) > (*rest.borrow()))
        && ((*ten_kappa.borrow()).wrapping_sub((2_u64).wrapping_mul((*rest.borrow())))
            >= (2_u64).wrapping_mul((*unit.borrow())))
    {
        return true;
    }
    if ((*rest.borrow()) > (*unit.borrow()))
        && ((*ten_kappa.borrow()).wrapping_sub(((*rest.borrow()).wrapping_sub((*unit.borrow()))))
            <= ((*rest.borrow()).wrapping_sub((*unit.borrow()))))
    {
        ({
            v8_base_Vector_char_Impl::operator_index(
                &buffer.as_pointer(),
                (((*length.borrow()) - 1) as usize),
            )
        })
        .with_mut(|__v| __v.postfix_inc());
        let i: Value<i32> = Rc::new(RefCell::new(((*length.borrow()) - 1)));
        'loop_: while ((*i.borrow()) > 0) {
            if (((({
                v8_base_Vector_char_Impl::operator_index(
                    &buffer.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .read()) as i32)
                != ((('0' as u8) as i32) + 10))
            {
                break;
            }
            ({
                v8_base_Vector_char_Impl::operator_index(
                    &buffer.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .write(('0' as u8));
            ({
                v8_base_Vector_char_Impl::operator_index(
                    &buffer.as_pointer(),
                    (((*i.borrow()) - 1) as usize),
                )
            })
            .with_mut(|__v| __v.postfix_inc());
            (*i.borrow_mut()).prefix_dec();
        }
        if (((({ v8_base_Vector_char_Impl::operator_index(&buffer.as_pointer(), 0_usize) }).read())
            as i32)
            == ((('0' as u8) as i32) + 10))
        {
            ({ v8_base_Vector_char_Impl::operator_index(&buffer.as_pointer(), 0_usize) })
                .write(('1' as u8));
            {
                let _ptr = (*kappa.borrow()).clone();
                _ptr.write(_ptr.read() + 1)
            };
        }
        return true;
    }
    return false;
}
thread_local!(
    pub static kTen4_243: Value<u32> = Rc::new(RefCell::new(10000_u32));
);
thread_local!(
    pub static kTen5_244: Value<u32> = Rc::new(RefCell::new(100000_u32));
);
thread_local!(
    pub static kTen6_245: Value<u32> = Rc::new(RefCell::new(1000000_u32));
);
thread_local!(
    pub static kTen7_246: Value<u32> = Rc::new(RefCell::new(10000000_u32));
);
thread_local!(
    pub static kTen8_247: Value<u32> = Rc::new(RefCell::new(100000000_u32));
);
thread_local!(
    pub static kTen9_248: Value<u32> = Rc::new(RefCell::new(1000000000_u32));
);
#[derive(Default)]
pub struct v8_base_DivMagic {
    pub mul: Value<u32>,
    pub shift: Value<u32>,
}
impl Clone for v8_base_DivMagic {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_DivMagic> = Rc::new(RefCell::new(Self {
            mul: Rc::new(RefCell::new((*self.mul.borrow()))),
            shift: Rc::new(RefCell::new((*self.shift.borrow()))),
        }));
        let this: Ptr<v8_base_DivMagic> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_DivMagic {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.mul.borrow()).to_bytes(&mut buf[0..4]);
        (*self.shift.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            mul: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
            shift: Rc::new(RefCell::new(<u32>::from_bytes(&buf[4..8]))),
        }
    }
}
thread_local!(
    pub static div_249: Value<Box<[v8_base_DivMagic]>> = Rc::new(RefCell::new(Box::new([
        v8_base_DivMagic {
            mul: Rc::new(RefCell::new(0_u32)),
            shift: Rc::new(RefCell::new(0_u32)),
        },
        v8_base_DivMagic {
            mul: Rc::new(RefCell::new(2576980378_u32)),
            shift: Rc::new(RefCell::new(3_u32)),
        },
        v8_base_DivMagic {
            mul: Rc::new(RefCell::new(1202590843_u32)),
            shift: Rc::new(RefCell::new(6_u32)),
        },
        v8_base_DivMagic {
            mul: Rc::new(RefCell::new(103079216_u32)),
            shift: Rc::new(RefCell::new(9_u32)),
        },
        v8_base_DivMagic {
            mul: Rc::new(RefCell::new(2741907122_u32)),
            shift: Rc::new(RefCell::new(13_u32)),
        },
        v8_base_DivMagic {
            mul: Rc::new(RefCell::new(1334532239_u32)),
            shift: Rc::new(RefCell::new(16_u32)),
        },
        v8_base_DivMagic {
            mul: Rc::new(RefCell::new(208632332_u32)),
            shift: Rc::new(RefCell::new(19_u32)),
        },
        v8_base_DivMagic {
            mul: Rc::new(RefCell::new(2910792108_u32)),
            shift: Rc::new(RefCell::new(23_u32)),
        },
        v8_base_DivMagic {
            mul: Rc::new(RefCell::new(1469640228_u32)),
            shift: Rc::new(RefCell::new(26_u32)),
        },
    ])));
);
pub fn fast_divmod_250(val: Ptr<u32>, divisor: u32, d: Ptr<v8_base_DivMagic>) -> u32 {
    let val: Value<Ptr<u32>> = Rc::new(RefCell::new(val));
    let divisor: Value<u32> = Rc::new(RefCell::new(divisor));
    if ((*divisor.borrow()) == 1_u32) {
        let digit: Value<u32> = Rc::new(RefCell::new(((*val.borrow()).read())));
        (*val.borrow()).write(0_u32);
        return (*digit.borrow());
    } else {
        let q: Value<u32> = Rc::new(RefCell::new(
            ((((((*val.borrow()).read()) as u64)
                .wrapping_mul(((*(*d.upgrade().deref()).mul.borrow()) as u64)))
                >> 32) as u32),
        ));
        let t: Value<u32> = Rc::new(RefCell::new(
            ((((*val.borrow()).read()).wrapping_sub((*q.borrow()))) >> 1)
                .wrapping_add((*q.borrow())),
        ));
        let digit: Value<u32> = Rc::new(RefCell::new({
            let _lhs = (*t.borrow());
            _lhs >> (*(*d.upgrade().deref()).shift.borrow())
        }));
        {
            let rhs_0 = ((*val.borrow()).read())
                .wrapping_sub((*digit.borrow()).wrapping_mul((*divisor.borrow())));
            (*val.borrow()).write(rhs_0)
        };
        return (*digit.borrow());
    }
    panic!("ub: non-void function does not return a value")
}
pub fn BiggestPowerTen_251(number: u32, number_bits: i32, power: Ptr<u32>, exponent: Ptr<u32>) {
    let number: Value<u32> = Rc::new(RefCell::new(number));
    let number_bits: Value<i32> = Rc::new(RefCell::new(number_bits));
    let power: Value<Ptr<u32>> = Rc::new(RefCell::new(power));
    let exponent: Value<Ptr<u32>> = Rc::new(RefCell::new(exponent));
    switch!(match (*number_bits.borrow()) {
        __v if __v == 32 || __v == 31 || __v == 30 => {
            if ((*kTen9_248.with(Value::clone).borrow()) <= (*number.borrow())) {
                let __rhs = (*kTen9_248.with(Value::clone).borrow());
                (*power.borrow()).write(__rhs);
                (*exponent.borrow()).write(9_u32);
                break;
            };
        }
        __v if __v == 29 || __v == 28 || __v == 27 => {
            if ((*kTen8_247.with(Value::clone).borrow()) <= (*number.borrow())) {
                let __rhs = (*kTen8_247.with(Value::clone).borrow());
                (*power.borrow()).write(__rhs);
                (*exponent.borrow()).write(8_u32);
                break;
            };
        }
        __v if __v == 26 || __v == 25 || __v == 24 => {
            if ((*kTen7_246.with(Value::clone).borrow()) <= (*number.borrow())) {
                let __rhs = (*kTen7_246.with(Value::clone).borrow());
                (*power.borrow()).write(__rhs);
                (*exponent.borrow()).write(7_u32);
                break;
            };
        }
        __v if __v == 23 || __v == 22 || __v == 21 || __v == 20 => {
            if ((*kTen6_245.with(Value::clone).borrow()) <= (*number.borrow())) {
                let __rhs = (*kTen6_245.with(Value::clone).borrow());
                (*power.borrow()).write(__rhs);
                (*exponent.borrow()).write(6_u32);
                break;
            };
        }
        __v if __v == 19 || __v == 18 || __v == 17 => {
            if ((*kTen5_244.with(Value::clone).borrow()) <= (*number.borrow())) {
                let __rhs = (*kTen5_244.with(Value::clone).borrow());
                (*power.borrow()).write(__rhs);
                (*exponent.borrow()).write(5_u32);
                break;
            };
        }
        __v if __v == 16 || __v == 15 || __v == 14 => {
            if ((*kTen4_243.with(Value::clone).borrow()) <= (*number.borrow())) {
                let __rhs = (*kTen4_243.with(Value::clone).borrow());
                (*power.borrow()).write(__rhs);
                (*exponent.borrow()).write(4_u32);
                break;
            };
        }
        __v if __v == 13 || __v == 12 || __v == 11 || __v == 10 => {
            if (1000_u32 <= (*number.borrow())) {
                (*power.borrow()).write(1000_u32);
                (*exponent.borrow()).write(3_u32);
                break;
            };
        }
        __v if __v == 9 || __v == 8 || __v == 7 => {
            if (100_u32 <= (*number.borrow())) {
                (*power.borrow()).write(100_u32);
                (*exponent.borrow()).write(2_u32);
                break;
            };
        }
        __v if __v == 6 || __v == 5 || __v == 4 => {
            if (10_u32 <= (*number.borrow())) {
                (*power.borrow()).write(10_u32);
                (*exponent.borrow()).write(1_u32);
                break;
            };
        }
        __v if __v == 3 || __v == 2 || __v == 1 => {
            if (1_u32 <= (*number.borrow())) {
                (*power.borrow()).write(1_u32);
                (*exponent.borrow()).write(0_u32);
                break;
            };
        }
        __v if __v == 0 => {
            (*power.borrow()).write(0_u32);
            (*exponent.borrow()).write((-1_i32 as u32));
            break;
        }
        _ => {
            (*power.borrow()).write(0_u32);
            (*exponent.borrow()).write(0_u32);
            ({
                V8_Fatal_252(
                    (*kUnreachableCodeMessage_9.with(Value::clone).borrow()).clone(),
                    &[],
                )
            });
        }
    });
}
pub fn DigitGen_253(
    low: v8_base_DiyFp,
    w: v8_base_DiyFp,
    high: v8_base_DiyFp,
    outptr: Ptr<Ptr<u8>>,
    kappa: Ptr<i32>,
) -> bool {
    let low: Value<v8_base_DiyFp> = Rc::new(RefCell::new(low));
    let w: Value<v8_base_DiyFp> = Rc::new(RefCell::new(w));
    let high: Value<v8_base_DiyFp> = Rc::new(RefCell::new(high));
    let outptr: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(outptr));
    let kappa: Value<Ptr<i32>> = Rc::new(RefCell::new(kappa));
    (&(0));
    (&(0));
    (&(0));
    let unit: Value<u64> = Rc::new(RefCell::new(1_u64));
    let too_low: Value<v8_base_DiyFp> = Rc::new(RefCell::new(v8_base_DiyFp::v8_base_DiyFp2(
        { ({ v8_base_DiyFpImpl::f(&low.as_pointer()) }).wrapping_sub((*unit.borrow())) },
        { ({ v8_base_DiyFpImpl::e(&low.as_pointer()) }) },
    )));
    let too_high: Value<v8_base_DiyFp> = Rc::new(RefCell::new(v8_base_DiyFp::v8_base_DiyFp2(
        { ({ v8_base_DiyFpImpl::f(&high.as_pointer()) }).wrapping_add((*unit.borrow())) },
        { ({ v8_base_DiyFpImpl::e(&high.as_pointer()) }) },
    )));
    let unsafe_interval: Value<v8_base_DiyFp> = Rc::new(RefCell::new(
        ({ v8_base_DiyFp::Minus(too_high.as_pointer(), too_low.as_pointer()) }),
    ));
    let one: Value<v8_base_DiyFp> = Rc::new(RefCell::new(v8_base_DiyFp::v8_base_DiyFp2(
        { (1_u64 << -({ v8_base_DiyFpImpl::e(&w.as_pointer()) })) },
        { ({ v8_base_DiyFpImpl::e(&w.as_pointer()) }) },
    )));
    let integrals: Value<u32> = Rc::new(RefCell::new(
        ((({ v8_base_DiyFpImpl::f(&too_high.as_pointer()) })
            >> -({ v8_base_DiyFpImpl::e(&one.as_pointer()) })) as u32),
    ));
    let fractionals: Value<u64> = Rc::new(RefCell::new(
        (({ v8_base_DiyFpImpl::f(&too_high.as_pointer()) })
            & (({ v8_base_DiyFpImpl::f(&one.as_pointer()) }).wrapping_sub(1_u64))),
    ));
    let divisor: Value<u32> = <Value<u32>>::default();
    let divisor_exponent: Value<u32> = <Value<u32>>::default();
    ({
        BiggestPowerTen_251(
            (*integrals.borrow()),
            ((*kSignificandSize_222.with(Value::clone).borrow())
                - (-({ v8_base_DiyFpImpl::e(&one.as_pointer()) }))),
            (divisor.as_pointer()),
            (divisor_exponent.as_pointer()),
        )
    });
    let __rhs = (((*divisor_exponent.borrow()).wrapping_add(1_u32)) as i32);
    (*kappa.borrow()).write(__rhs);
    'loop_: while (((*kappa.borrow()).read()) > 0) {
        let digit: Value<u32> = Rc::new(RefCell::new(
            ({
                fast_divmod_250(
                    (integrals.as_pointer()),
                    (*divisor.borrow()),
                    (div_249.with(Value::clone).as_pointer() as Ptr<v8_base_DivMagic>)
                        .offset((*divisor_exponent.borrow())),
                )
            }),
        ));
        ((*outptr.borrow()).read())
            .write((((('0' as u8) as u32).wrapping_add((*digit.borrow()))) as u8));
        (*outptr.borrow()).with_mut(|__v| __v.postfix_inc());
        (*kappa.borrow()).with_mut(|__v| __v.postfix_dec());
        let rest: Value<u64> = Rc::new(RefCell::new(
            (((*integrals.borrow()) as u64) << -({ v8_base_DiyFpImpl::e(&one.as_pointer()) }))
                .wrapping_add((*fractionals.borrow())),
        ));
        if ((*rest.borrow()) < ({ v8_base_DiyFpImpl::f(&unsafe_interval.as_pointer()) })) {
            return ({
                let _last_digit: Ptr<u8> = ((*outptr.borrow()).read()).offset(-((1) as isize));
                let _distance_too_high_w: u64 = ({
                    v8_base_DiyFpImpl::f(
                        &Rc::new(RefCell::new(
                            ({ v8_base_DiyFp::Minus(too_high.as_pointer(), w.as_pointer()) }),
                        ))
                        .as_pointer(),
                    )
                });
                let _unsafe_interval: u64 =
                    ({ v8_base_DiyFpImpl::f(&unsafe_interval.as_pointer()) });
                let _rest: u64 = (*rest.borrow());
                let _ten_kappa: u64 = (((*divisor.borrow()) as u64)
                    << -({ v8_base_DiyFpImpl::e(&one.as_pointer()) }));
                let _unit: u64 = (*unit.borrow());
                RoundWeed_241(
                    _last_digit,
                    _distance_too_high_w,
                    _unsafe_interval,
                    _rest,
                    _ten_kappa,
                    _unit,
                )
            });
        }
        if (((*kappa.borrow()).read()) <= 0) {
            break;
        }
        {
            let rhs_0 = (*divisor.borrow()).wrapping_div(10_u32);
            (*divisor.borrow_mut()) = rhs_0
        };
        (*divisor_exponent.borrow_mut()).prefix_dec();
    }
    (&(0));
    (&(0));
    (&(0));
    'loop_: while true {
        {
            let rhs_0 = (*fractionals.borrow()).wrapping_mul(10_u64);
            (*fractionals.borrow_mut()) = rhs_0
        };
        {
            let rhs_0 = (*unit.borrow()).wrapping_mul(10_u64);
            (*unit.borrow_mut()) = rhs_0
        };
        ({
            let _new_value: u64 =
                ({ v8_base_DiyFpImpl::f(&unsafe_interval.as_pointer()) }).wrapping_mul(10_u64);
            v8_base_DiyFpImpl::set_f(&unsafe_interval.as_pointer(), _new_value)
        });
        let digit: Value<i32> = Rc::new(RefCell::new(
            (((*fractionals.borrow()) >> -({ v8_base_DiyFpImpl::e(&one.as_pointer()) })) as i32),
        ));
        ((*outptr.borrow()).read()).write((((('0' as u8) as i32) + (*digit.borrow())) as u8));
        (*outptr.borrow()).with_mut(|__v| __v.postfix_inc());
        (*fractionals.borrow_mut()) &=
            ({ v8_base_DiyFpImpl::f(&one.as_pointer()) }).wrapping_sub(1_u64);
        (*kappa.borrow()).with_mut(|__v| __v.postfix_dec());
        if ((*fractionals.borrow()) < ({ v8_base_DiyFpImpl::f(&unsafe_interval.as_pointer()) })) {
            return ({
                let _last_digit: Ptr<u8> = ((*outptr.borrow()).read()).offset(-((1) as isize));
                let _distance_too_high_w: u64 = ({
                    v8_base_DiyFpImpl::f(
                        &Rc::new(RefCell::new(
                            ({ v8_base_DiyFp::Minus(too_high.as_pointer(), w.as_pointer()) }),
                        ))
                        .as_pointer(),
                    )
                })
                .wrapping_mul((*unit.borrow()));
                let _unsafe_interval: u64 =
                    ({ v8_base_DiyFpImpl::f(&unsafe_interval.as_pointer()) });
                let _rest: u64 = (*fractionals.borrow());
                let _ten_kappa: u64 = ({ v8_base_DiyFpImpl::f(&one.as_pointer()) });
                let _unit: u64 = (*unit.borrow());
                RoundWeed_241(
                    _last_digit,
                    _distance_too_high_w,
                    _unsafe_interval,
                    _rest,
                    _ten_kappa,
                    _unit,
                )
            });
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub fn DigitGenCounted_254(
    w: v8_base_DiyFp,
    requested_digits: i32,
    buffer: v8_base_Vector_char_,
    length: Ptr<i32>,
    kappa: Ptr<i32>,
) -> bool {
    let w: Value<v8_base_DiyFp> = Rc::new(RefCell::new(w));
    let requested_digits: Value<i32> = Rc::new(RefCell::new(requested_digits));
    let buffer: Value<v8_base_Vector_char_> = Rc::new(RefCell::new(buffer));
    let length: Value<Ptr<i32>> = Rc::new(RefCell::new(length));
    let kappa: Value<Ptr<i32>> = Rc::new(RefCell::new(kappa));
    (&(0));
    (&(0));
    (&(0));
    let w_error: Value<u64> = Rc::new(RefCell::new(1_u64));
    let one: Value<v8_base_DiyFp> = Rc::new(RefCell::new(v8_base_DiyFp::v8_base_DiyFp2(
        { (1_u64 << -({ v8_base_DiyFpImpl::e(&w.as_pointer()) })) },
        { ({ v8_base_DiyFpImpl::e(&w.as_pointer()) }) },
    )));
    let integrals: Value<u32> = Rc::new(RefCell::new(
        ((({ v8_base_DiyFpImpl::f(&w.as_pointer()) })
            >> -({ v8_base_DiyFpImpl::e(&one.as_pointer()) })) as u32),
    ));
    let fractionals: Value<u64> = Rc::new(RefCell::new(
        (({ v8_base_DiyFpImpl::f(&w.as_pointer()) })
            & (({ v8_base_DiyFpImpl::f(&one.as_pointer()) }).wrapping_sub(1_u64))),
    ));
    let divisor: Value<u32> = <Value<u32>>::default();
    let divisor_exponent: Value<u32> = <Value<u32>>::default();
    ({
        BiggestPowerTen_251(
            (*integrals.borrow()),
            ((*kSignificandSize_222.with(Value::clone).borrow())
                - (-({ v8_base_DiyFpImpl::e(&one.as_pointer()) }))),
            (divisor.as_pointer()),
            (divisor_exponent.as_pointer()),
        )
    });
    let __rhs = (((*divisor_exponent.borrow()).wrapping_add(1_u32)) as i32);
    (*kappa.borrow()).write(__rhs);
    (*length.borrow()).write(0);
    'loop_: while (((*kappa.borrow()).read()) > 0) {
        let digit: Value<u32> = Rc::new(RefCell::new(
            ({
                fast_divmod_250(
                    (integrals.as_pointer()),
                    (*divisor.borrow()),
                    (div_249.with(Value::clone).as_pointer() as Ptr<v8_base_DivMagic>)
                        .offset((*divisor_exponent.borrow())),
                )
            }),
        ));
        ({
            v8_base_Vector_char_Impl::operator_index(
                &buffer.as_pointer(),
                (((*length.borrow()).read()) as usize),
            )
        })
        .write((((('0' as u8) as u32).wrapping_add((*digit.borrow()))) as u8));
        (*length.borrow()).with_mut(|__v| __v.postfix_inc());
        (*requested_digits.borrow_mut()).postfix_dec();
        (*kappa.borrow()).with_mut(|__v| __v.postfix_dec());
        if ((*requested_digits.borrow()) == 0) {
            break;
        }
        {
            let rhs_0 = (*divisor.borrow()).wrapping_div(10_u32);
            (*divisor.borrow_mut()) = rhs_0
        };
        (*divisor_exponent.borrow_mut()).prefix_dec();
    }
    if ((*requested_digits.borrow()) == 0) {
        let rest: Value<u64> = Rc::new(RefCell::new(
            (((*integrals.borrow()) as u64) << -({ v8_base_DiyFpImpl::e(&one.as_pointer()) }))
                .wrapping_add((*fractionals.borrow())),
        ));
        return ({
            let _buffer: v8_base_Vector_char_ = (*buffer.borrow()).clone();
            let _length: i32 = ((*length.borrow()).read());
            let _rest: u64 = (*rest.borrow());
            let _ten_kappa: u64 =
                (((*divisor.borrow()) as u64) << -({ v8_base_DiyFpImpl::e(&one.as_pointer()) }));
            let _unit: u64 = (*w_error.borrow());
            let _kappa: Ptr<i32> = (*kappa.borrow()).clone();
            RoundWeedCounted_242(_buffer, _length, _rest, _ten_kappa, _unit, _kappa)
        });
    }
    (&(0));
    (&(0));
    (&(0));
    'loop_: while ((*requested_digits.borrow()) > 0)
        && ((*fractionals.borrow()) > (*w_error.borrow()))
    {
        {
            let rhs_0 = (*fractionals.borrow()).wrapping_mul(10_u64);
            (*fractionals.borrow_mut()) = rhs_0
        };
        {
            let rhs_0 = (*w_error.borrow()).wrapping_mul(10_u64);
            (*w_error.borrow_mut()) = rhs_0
        };
        let digit: Value<i32> = Rc::new(RefCell::new(
            (((*fractionals.borrow()) >> -({ v8_base_DiyFpImpl::e(&one.as_pointer()) })) as i32),
        ));
        ({
            v8_base_Vector_char_Impl::operator_index(
                &buffer.as_pointer(),
                (((*length.borrow()).read()) as usize),
            )
        })
        .write((((('0' as u8) as i32) + (*digit.borrow())) as u8));
        (*length.borrow()).with_mut(|__v| __v.postfix_inc());
        (*requested_digits.borrow_mut()).postfix_dec();
        (*fractionals.borrow_mut()) &=
            ({ v8_base_DiyFpImpl::f(&one.as_pointer()) }).wrapping_sub(1_u64);
        (*kappa.borrow()).with_mut(|__v| __v.postfix_dec());
    }
    if ((*requested_digits.borrow()) != 0) {
        return false;
    }
    return ({
        let _buffer: v8_base_Vector_char_ = (*buffer.borrow()).clone();
        let _length: i32 = ((*length.borrow()).read());
        let _rest: u64 = (*fractionals.borrow());
        let _ten_kappa: u64 = ({ v8_base_DiyFpImpl::f(&one.as_pointer()) });
        let _unit: u64 = (*w_error.borrow());
        let _kappa: Ptr<i32> = (*kappa.borrow()).clone();
        RoundWeedCounted_242(_buffer, _length, _rest, _ten_kappa, _unit, _kappa)
    });
}
pub fn Grisu3_255(v: f64, outptr: Ptr<Ptr<u8>>, decimal_exponent: Ptr<i32>) -> bool {
    let v: Value<f64> = Rc::new(RefCell::new(v));
    let outptr: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(outptr));
    let decimal_exponent: Value<Ptr<i32>> = Rc::new(RefCell::new(decimal_exponent));
    let w: Value<v8_base_DiyFp> = Rc::new(RefCell::new(
        ({
            v8_base_DoubleImpl::AsNormalizedDiyFp(
                &Rc::new(RefCell::new(v8_base_Double::v8_base_Double2({
                    (*v.borrow())
                })))
                .as_pointer(),
            )
        }),
    ));
    let boundary_minus: Value<v8_base_DiyFp> =
        Rc::new(RefCell::new(v8_base_DiyFp::v8_base_DiyFp1()));
    let boundary_plus: Value<v8_base_DiyFp> =
        Rc::new(RefCell::new(v8_base_DiyFp::v8_base_DiyFp1()));
    ({
        v8_base_DoubleImpl::NormalizedBoundaries(
            &Rc::new(RefCell::new(v8_base_Double::v8_base_Double2({
                (*v.borrow())
            })))
            .as_pointer(),
            (boundary_minus.as_pointer()),
            (boundary_plus.as_pointer()),
        )
    });
    (&(0));
    let ten_mk: Value<v8_base_DiyFp> = Rc::new(RefCell::new(v8_base_DiyFp::v8_base_DiyFp1()));
    let mk: Value<i32> = <Value<i32>>::default();
    let ten_mk_minimal_binary_exponent: Value<i32> = Rc::new(RefCell::new(
        ((*kMinimalTargetExponent_239.with(Value::clone).borrow())
            - (({ v8_base_DiyFpImpl::e(&w.as_pointer()) })
                + (*kSignificandSize_222.with(Value::clone).borrow()))),
    ));
    let ten_mk_maximal_binary_exponent: Value<i32> = Rc::new(RefCell::new(
        ((*kMaximalTargetExponent_240.with(Value::clone).borrow())
            - (({ v8_base_DiyFpImpl::e(&w.as_pointer()) })
                + (*kSignificandSize_222.with(Value::clone).borrow()))),
    ));
    ({
        v8_base_PowersOfTenCache::GetCachedPowerForBinaryExponentRange(
            (*ten_mk_minimal_binary_exponent.borrow()),
            (*ten_mk_maximal_binary_exponent.borrow()),
            (ten_mk.as_pointer()),
            (mk.as_pointer()),
        )
    });
    (&(0));
    let scaled_w: Value<v8_base_DiyFp> = Rc::new(RefCell::new(
        ({ v8_base_DiyFp::Times(w.as_pointer(), ten_mk.as_pointer()) }),
    ));
    (&(0));
    let scaled_boundary_minus: Value<v8_base_DiyFp> = Rc::new(RefCell::new(
        ({ v8_base_DiyFp::Times(boundary_minus.as_pointer(), ten_mk.as_pointer()) }),
    ));
    let scaled_boundary_plus: Value<v8_base_DiyFp> = Rc::new(RefCell::new(
        ({ v8_base_DiyFp::Times(boundary_plus.as_pointer(), ten_mk.as_pointer()) }),
    ));
    let kappa: Value<i32> = <Value<i32>>::default();
    let result: Value<bool> = Rc::new(RefCell::new(
        ({
            DigitGen_253(
                (*scaled_boundary_minus.borrow()).clone(),
                (*scaled_w.borrow()).clone(),
                (*scaled_boundary_plus.borrow()).clone(),
                (*outptr.borrow()).clone(),
                (kappa.as_pointer()),
            )
        }),
    ));
    let __rhs = (-(*mk.borrow()) + (*kappa.borrow()));
    (*decimal_exponent.borrow()).write(__rhs);
    return (*result.borrow());
}
pub fn Grisu3Counted_256(
    v: f64,
    requested_digits: i32,
    buffer: v8_base_Vector_char_,
    length: Ptr<i32>,
    decimal_exponent: Ptr<i32>,
) -> bool {
    let v: Value<f64> = Rc::new(RefCell::new(v));
    let requested_digits: Value<i32> = Rc::new(RefCell::new(requested_digits));
    let buffer: Value<v8_base_Vector_char_> = Rc::new(RefCell::new(buffer));
    let length: Value<Ptr<i32>> = Rc::new(RefCell::new(length));
    let decimal_exponent: Value<Ptr<i32>> = Rc::new(RefCell::new(decimal_exponent));
    let w: Value<v8_base_DiyFp> = Rc::new(RefCell::new(
        ({
            v8_base_DoubleImpl::AsNormalizedDiyFp(
                &Rc::new(RefCell::new(v8_base_Double::v8_base_Double2({
                    (*v.borrow())
                })))
                .as_pointer(),
            )
        }),
    ));
    let ten_mk: Value<v8_base_DiyFp> = Rc::new(RefCell::new(v8_base_DiyFp::v8_base_DiyFp1()));
    let mk: Value<i32> = <Value<i32>>::default();
    let ten_mk_minimal_binary_exponent: Value<i32> = Rc::new(RefCell::new(
        ((*kMinimalTargetExponent_239.with(Value::clone).borrow())
            - (({ v8_base_DiyFpImpl::e(&w.as_pointer()) })
                + (*kSignificandSize_222.with(Value::clone).borrow()))),
    ));
    let ten_mk_maximal_binary_exponent: Value<i32> = Rc::new(RefCell::new(
        ((*kMaximalTargetExponent_240.with(Value::clone).borrow())
            - (({ v8_base_DiyFpImpl::e(&w.as_pointer()) })
                + (*kSignificandSize_222.with(Value::clone).borrow()))),
    ));
    ({
        v8_base_PowersOfTenCache::GetCachedPowerForBinaryExponentRange(
            (*ten_mk_minimal_binary_exponent.borrow()),
            (*ten_mk_maximal_binary_exponent.borrow()),
            (ten_mk.as_pointer()),
            (mk.as_pointer()),
        )
    });
    (&(0));
    let scaled_w: Value<v8_base_DiyFp> = Rc::new(RefCell::new(
        ({ v8_base_DiyFp::Times(w.as_pointer(), ten_mk.as_pointer()) }),
    ));
    let kappa: Value<i32> = <Value<i32>>::default();
    let result: Value<bool> = Rc::new(RefCell::new(
        ({
            let _requested_digits: i32 = (*requested_digits.borrow());
            let _length: Ptr<i32> = (*length.borrow()).clone();
            DigitGenCounted_254(
                (*scaled_w.borrow()).clone(),
                _requested_digits,
                (*buffer.borrow()).clone(),
                _length,
                (kappa.as_pointer()),
            )
        }),
    ));
    let __rhs = (-(*mk.borrow()) + (*kappa.borrow()));
    (*decimal_exponent.borrow()).write(__rhs);
    return (*result.borrow());
}
pub fn FastDtoa_257(
    v: f64,
    mode: v8_base_FastDtoaMode,
    requested_digits: i32,
    buffer: v8_base_Vector_char_,
    length: Ptr<i32>,
    decimal_point: Ptr<i32>,
) -> bool {
    let v: Value<f64> = Rc::new(RefCell::new(v));
    let mode: Value<v8_base_FastDtoaMode> = Rc::new(RefCell::new(mode));
    let requested_digits: Value<i32> = Rc::new(RefCell::new(requested_digits));
    let buffer: Value<v8_base_Vector_char_> = Rc::new(RefCell::new(buffer));
    let length: Value<Ptr<i32>> = Rc::new(RefCell::new(length));
    let decimal_point: Value<Ptr<i32>> = Rc::new(RefCell::new(decimal_point));
    (&(0));
    (&(0));
    let result: Value<bool> = Rc::new(RefCell::new(false));
    let outptr: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ({ v8_base_Vector_char_Impl::data(&buffer.as_pointer()) }),
    ));
    let decimal_exponent: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match ((*mode.borrow()) as i32) {
        __v if __v == 0 => {
            (*result.borrow_mut()) = ({
                Grisu3_255(
                    (*v.borrow()),
                    (outptr.as_pointer()),
                    (decimal_exponent.as_pointer()),
                )
            });
            let __rhs = ((((*outptr.borrow()).clone()
                - ({ v8_base_Vector_char_Impl::data(&buffer.as_pointer()) }))
                as i64) as i32);
            (*length.borrow()).write(__rhs);
            break;
        }
        __v if __v == 1 => {
            {
                let local_length: Value<i32> = Rc::new(RefCell::new(0));
                (*result.borrow_mut()) = ({
                    Grisu3Counted_256(
                        (*v.borrow()),
                        (*requested_digits.borrow()),
                        (*buffer.borrow()).clone(),
                        (local_length.as_pointer()),
                        (decimal_exponent.as_pointer()),
                    )
                });
                let __rhs = (*local_length.borrow());
                (*length.borrow()).write(__rhs);
                break;
            }
        }
        _ => {
            ({
                V8_Fatal_252(
                    (*kUnreachableCodeMessage_9.with(Value::clone).borrow()).clone(),
                    &[],
                )
            });
        }
    });
    if (*result.borrow()) {
        let __rhs = {
            let _lhs = ((*length.borrow()).read());
            _lhs + (*decimal_exponent.borrow())
        };
        (*decimal_point.borrow()).write(__rhs);
        ({
            v8_base_Vector_char_Impl::operator_index(
                &buffer.as_pointer(),
                (((*length.borrow()).read()) as usize),
            )
        })
        .write(('\0' as u8));
    }
    return (*result.borrow());
}
pub trait absl_int128Impl {
    fn operator_assign_i32(&self, v: i32) -> Ptr<absl_int128>;
    fn operator_assign_u32(&self, v: u32) -> Ptr<absl_int128>;
    fn operator_assign_i64(&self, v: i64) -> Ptr<absl_int128>;
    fn operator_assign_u64(&self, v: u64) -> Ptr<absl_int128>;
    fn operator_assign_i64(&self, v: i64) -> Ptr<absl_int128>;
    fn operator_assign_u64(&self, v: u64) -> Ptr<absl_int128>;
    fn operator_assign_i128(&self, v: i128) -> Ptr<absl_int128>;
    fn operator__Bool(&self) -> bool;
    fn operator_char(&self) -> u8;
    fn operator_signed_char(&self) -> i8;
    fn operator_unsigned_char(&self) -> u8;
    fn operator_char16_t(&self) -> u16;
    fn operator_char32_t(&self) -> u32;
    fn operator_wchar_t(&self) -> i32;
    fn operator_short(&self) -> i16;
    fn operator_unsigned_short(&self) -> u16;
    fn operator_int(&self) -> i32;
    fn operator_unsigned_int(&self) -> u32;
    fn operator_long(&self) -> i64;
    fn operator_unsigned_long(&self) -> u64;
    fn operator_long_long(&self) -> i64;
    fn operator_unsigned_long_long(&self) -> u64;
    fn operator___int128(&self) -> i128;
    fn operator_unsigned___int128(&self) -> u128;
    fn operator_float(&self) -> f32;
    fn operator_double(&self) -> f64;
    fn operator_long_double(&self) -> f64;
    fn operator_add_assign(&self, other: absl_int128) -> Ptr<absl_int128>;
    fn operator_sub_assign(&self, other: absl_int128) -> Ptr<absl_int128>;
    fn operator_mul_assign(&self, other: absl_int128) -> Ptr<absl_int128>;
    fn operator_div_assign(&self, other: absl_int128) -> Ptr<absl_int128>;
    fn operator_rem_assign(&self, other: absl_int128) -> Ptr<absl_int128>;
    fn operator_post_inc_i32(&self, _a0: i32) -> absl_int128;
    fn operator_post_dec_i32(&self, _a0: i32) -> absl_int128;
    fn operator_inc(&self) -> Ptr<absl_int128>;
    fn operator_dec(&self) -> Ptr<absl_int128>;
    fn operator_bitand_assign(&self, other: absl_int128) -> Ptr<absl_int128>;
    fn operator_bitor_assign(&self, other: absl_int128) -> Ptr<absl_int128>;
    fn operator_bitxor_assign(&self, other: absl_int128) -> Ptr<absl_int128>;
    fn operator_shl_assign(&self, amount: i32) -> Ptr<absl_int128>;
    fn operator_shr_assign(&self, amount: i32) -> Ptr<absl_int128>;
}
impl absl_int128Impl for Ptr<absl_int128> {
    fn operator_assign_i32(&self, v: i32) -> Ptr<absl_int128> {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        return (*self).write(absl_int128::absl_int1281({ (*v.borrow()) }));
    }
    fn operator_assign_u32(&self, v: u32) -> Ptr<absl_int128> {
        let v: Value<u32> = Rc::new(RefCell::new(v));
        return (*self).write(absl_int128::absl_int1282({ (*v.borrow()) }));
    }
    fn operator_assign_i64(&self, v: i64) -> Ptr<absl_int128> {
        let v: Value<i64> = Rc::new(RefCell::new(v));
        return (*self).write(absl_int128::absl_int1283({ (*v.borrow()) }));
    }
    fn operator_assign_u64(&self, v: u64) -> Ptr<absl_int128> {
        let v: Value<u64> = Rc::new(RefCell::new(v));
        return (*self).write(absl_int128::absl_int1284({ (*v.borrow()) }));
    }
    fn operator_assign_i64(&self, v: i64) -> Ptr<absl_int128> {
        let v: Value<i64> = Rc::new(RefCell::new(v));
        return (*self).write(absl_int128::absl_int1285({ (*v.borrow()) }));
    }
    fn operator_assign_u64(&self, v: u64) -> Ptr<absl_int128> {
        let v: Value<u64> = Rc::new(RefCell::new(v));
        return (*self).write(absl_int128::absl_int1286({ (*v.borrow()) }));
    }
    fn operator_add_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_add_203(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_sub_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_sub_204(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_mul_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_mul_205(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_div_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_div_206(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_rem_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_rem_207(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitor_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_bitor_208(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitand_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_bitand_209(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitxor_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_bitxor_210(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_shl_assign(&self, amount: i32) -> Ptr<absl_int128> {
        let amount: Value<i32> = Rc::new(RefCell::new(amount));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_shl_211(_lhs, (*amount.borrow()))
            }),
        );
        return (*self).clone();
    }
    fn operator_shr_assign(&self, amount: i32) -> Ptr<absl_int128> {
        let amount: Value<i32> = Rc::new(RefCell::new(amount));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_shr_212(_lhs, (*amount.borrow()))
            }),
        );
        return (*self).clone();
    }
    fn operator_assign_i128(&self, v: i128) -> Ptr<absl_int128> {
        let v: Value<i128> = Rc::new(RefCell::new(v));
        (*(*(*self).upgrade().deref()).v_.borrow_mut()) = (*v.borrow());
        return (*self).clone();
    }
    fn operator__Bool(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) != 0);
    }
    fn operator_char(&self) -> u8 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as u8);
    }
    fn operator_signed_char(&self) -> i8 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as i8);
    }
    fn operator_unsigned_char(&self) -> u8 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as u8);
    }
    fn operator_char16_t(&self) -> u16 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as u16);
    }
    fn operator_char32_t(&self) -> u32 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as u32);
    }
    fn operator_wchar_t(&self) -> i32 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as i32);
    }
    fn operator_short(&self) -> i16 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as i16);
    }
    fn operator_unsigned_short(&self) -> u16 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as u16);
    }
    fn operator_int(&self) -> i32 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as i32);
    }
    fn operator_unsigned_int(&self) -> u32 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as u32);
    }
    fn operator_long(&self) -> i64 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as i64);
    }
    fn operator_unsigned_long(&self) -> u64 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as u64);
    }
    fn operator_long_long(&self) -> i64 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as i64);
    }
    fn operator_unsigned_long_long(&self) -> u64 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as u64);
    }
    fn operator___int128(&self) -> i128 {
        return (*(*(*self).upgrade().deref()).v_.borrow());
    }
    fn operator_unsigned___int128(&self) -> u128 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as u128);
    }
    fn operator_float(&self) -> f32 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as f32);
    }
    fn operator_double(&self) -> f64 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as f64);
    }
    fn operator_long_double(&self) -> f64 {
        return ((*(*(*self).upgrade().deref()).v_.borrow()) as f64);
    }
    fn operator_post_inc_i32(&self, _a0: i32) -> absl_int128 {
        let _a0: Value<i32> = Rc::new(RefCell::new(_a0));
        let tmp: Value<absl_int128> = Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        (*(*(*self).upgrade().deref()).v_.borrow_mut()).prefix_inc();
        return (*tmp.borrow()).clone();
    }
    fn operator_post_dec_i32(&self, _a0: i32) -> absl_int128 {
        let _a0: Value<i32> = Rc::new(RefCell::new(_a0));
        let tmp: Value<absl_int128> = Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        (*(*(*self).upgrade().deref()).v_.borrow_mut()).prefix_dec();
        return (*tmp.borrow()).clone();
    }
    fn operator_inc(&self) -> Ptr<absl_int128> {
        (*(*(*self).upgrade().deref()).v_.borrow_mut()).prefix_inc();
        return (*self).clone();
    }
    fn operator_dec(&self) -> Ptr<absl_int128> {
        (*(*(*self).upgrade().deref()).v_.borrow_mut()).prefix_dec();
        return (*self).clone();
    }
}
pub trait absl_uint128Impl {
    fn operator_assign_i32(&self, v: i32) -> Ptr<absl_uint128>;
    fn operator_assign_u32(&self, v: u32) -> Ptr<absl_uint128>;
    fn operator_assign_i64(&self, v: i64) -> Ptr<absl_uint128>;
    fn operator_assign_u64(&self, v: u64) -> Ptr<absl_uint128>;
    fn operator_assign_i64(&self, v: i64) -> Ptr<absl_uint128>;
    fn operator_assign_u64(&self, v: u64) -> Ptr<absl_uint128>;
    fn operator_assign_i128(&self, v: i128) -> Ptr<absl_uint128>;
    fn operator_assign_u128(&self, v: u128) -> Ptr<absl_uint128>;
    fn operator_assign_absl_int128(&self, v: absl_int128) -> Ptr<absl_uint128>;
    fn operator__Bool(&self) -> bool;
    fn operator_char(&self) -> u8;
    fn operator_signed_char(&self) -> i8;
    fn operator_unsigned_char(&self) -> u8;
    fn operator_char16_t(&self) -> u16;
    fn operator_char32_t(&self) -> u32;
    fn operator_wchar_t(&self) -> i32;
    fn operator_short(&self) -> i16;
    fn operator_unsigned_short(&self) -> u16;
    fn operator_int(&self) -> i32;
    fn operator_unsigned_int(&self) -> u32;
    fn operator_long(&self) -> i64;
    fn operator_unsigned_long(&self) -> u64;
    fn operator_long_long(&self) -> i64;
    fn operator_unsigned_long_long(&self) -> u64;
    fn operator___int128(&self) -> i128;
    fn operator_unsigned___int128(&self) -> u128;
    fn operator_float(&self) -> f32;
    fn operator_double(&self) -> f64;
    fn operator_long_double(&self) -> f64;
    fn operator_add_assign(&self, other: absl_uint128) -> Ptr<absl_uint128>;
    fn operator_sub_assign(&self, other: absl_uint128) -> Ptr<absl_uint128>;
    fn operator_mul_assign(&self, other: absl_uint128) -> Ptr<absl_uint128>;
    fn operator_div_assign(&self, other: absl_uint128) -> Ptr<absl_uint128>;
    fn operator_rem_assign(&self, other: absl_uint128) -> Ptr<absl_uint128>;
    fn operator_post_inc_i32(&self, _a0: i32) -> absl_uint128;
    fn operator_post_dec_i32(&self, _a0: i32) -> absl_uint128;
    fn operator_shl_assign(&self, amount: i32) -> Ptr<absl_uint128>;
    fn operator_shr_assign(&self, amount: i32) -> Ptr<absl_uint128>;
    fn operator_bitand_assign(&self, other: absl_uint128) -> Ptr<absl_uint128>;
    fn operator_bitor_assign(&self, other: absl_uint128) -> Ptr<absl_uint128>;
    fn operator_bitxor_assign(&self, other: absl_uint128) -> Ptr<absl_uint128>;
    fn operator_inc(&self) -> Ptr<absl_uint128>;
    fn operator_dec(&self) -> Ptr<absl_uint128>;
}
impl absl_uint128Impl for Ptr<absl_uint128> {
    fn operator_assign_i32(&self, v: i32) -> Ptr<absl_uint128> {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        return (*self).write(absl_uint128::absl_uint1281({ (*v.borrow()) }));
    }
    fn operator_assign_u32(&self, v: u32) -> Ptr<absl_uint128> {
        let v: Value<u32> = Rc::new(RefCell::new(v));
        return (*self).write(absl_uint128::absl_uint1282({ (*v.borrow()) }));
    }
    fn operator_assign_i64(&self, v: i64) -> Ptr<absl_uint128> {
        let v: Value<i64> = Rc::new(RefCell::new(v));
        return (*self).write(absl_uint128::absl_uint1283({ (*v.borrow()) }));
    }
    fn operator_assign_u64(&self, v: u64) -> Ptr<absl_uint128> {
        let v: Value<u64> = Rc::new(RefCell::new(v));
        return (*self).write(absl_uint128::absl_uint1284({ (*v.borrow()) }));
    }
    fn operator_assign_i64(&self, v: i64) -> Ptr<absl_uint128> {
        let v: Value<i64> = Rc::new(RefCell::new(v));
        return (*self).write(absl_uint128::absl_uint1285({ (*v.borrow()) }));
    }
    fn operator_assign_u64(&self, v: u64) -> Ptr<absl_uint128> {
        let v: Value<u64> = Rc::new(RefCell::new(v));
        return (*self).write(absl_uint128::absl_uint1286({ (*v.borrow()) }));
    }
    fn operator_assign_i128(&self, v: i128) -> Ptr<absl_uint128> {
        let v: Value<i128> = Rc::new(RefCell::new(v));
        return (*self).write(absl_uint128::absl_uint1287({ (*v.borrow()) }));
    }
    fn operator_assign_u128(&self, v: u128) -> Ptr<absl_uint128> {
        let v: Value<u128> = Rc::new(RefCell::new(v));
        return (*self).write(absl_uint128::absl_uint1288({ (*v.borrow()) }));
    }
    fn operator_assign_absl_int128(&self, v: absl_int128) -> Ptr<absl_uint128> {
        let v: Value<absl_int128> = Rc::new(RefCell::new(v));
        return (*self).write(absl_uint128::absl_uint1289({ (*v.borrow()).clone() }));
    }
    fn operator_shl_assign(&self, amount: i32) -> Ptr<absl_uint128> {
        let amount: Value<i32> = Rc::new(RefCell::new(amount));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_shl_180(_lhs, (*amount.borrow()))
            }),
        );
        return (*self).clone();
    }
    fn operator_shr_assign(&self, amount: i32) -> Ptr<absl_uint128> {
        let amount: Value<i32> = Rc::new(RefCell::new(amount));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_shr_181(_lhs, (*amount.borrow()))
            }),
        );
        return (*self).clone();
    }
    fn operator_add_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_add_182(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_sub_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_sub_183(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_mul_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_mul_184(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_div_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_div_185(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_rem_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_rem_186(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator__Bool(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).lo_.borrow()) != 0)
            || ((*(*(*self).upgrade().deref()).hi_.borrow()) != 0);
    }
    fn operator_char(&self) -> u8 {
        return ((*(*(*self).upgrade().deref()).lo_.borrow()) as u8);
    }
    fn operator_signed_char(&self) -> i8 {
        return ((*(*(*self).upgrade().deref()).lo_.borrow()) as i8);
    }
    fn operator_unsigned_char(&self) -> u8 {
        return ((*(*(*self).upgrade().deref()).lo_.borrow()) as u8);
    }
    fn operator_char16_t(&self) -> u16 {
        return ((*(*(*self).upgrade().deref()).lo_.borrow()) as u16);
    }
    fn operator_char32_t(&self) -> u32 {
        return ((*(*(*self).upgrade().deref()).lo_.borrow()) as u32);
    }
    fn operator_wchar_t(&self) -> i32 {
        return ((*(*(*self).upgrade().deref()).lo_.borrow()) as i32);
    }
    fn operator_short(&self) -> i16 {
        return ((*(*(*self).upgrade().deref()).lo_.borrow()) as i16);
    }
    fn operator_unsigned_short(&self) -> u16 {
        return ((*(*(*self).upgrade().deref()).lo_.borrow()) as u16);
    }
    fn operator_int(&self) -> i32 {
        return ((*(*(*self).upgrade().deref()).lo_.borrow()) as i32);
    }
    fn operator_unsigned_int(&self) -> u32 {
        return ((*(*(*self).upgrade().deref()).lo_.borrow()) as u32);
    }
    fn operator_long(&self) -> i64 {
        return ((*(*(*self).upgrade().deref()).lo_.borrow()) as i64);
    }
    fn operator_unsigned_long(&self) -> u64 {
        return (*(*(*self).upgrade().deref()).lo_.borrow());
    }
    fn operator_long_long(&self) -> i64 {
        return ((*(*(*self).upgrade().deref()).lo_.borrow()) as i64);
    }
    fn operator_unsigned_long_long(&self) -> u64 {
        return ((*(*(*self).upgrade().deref()).lo_.borrow()) as u64);
    }
    fn operator___int128(&self) -> i128 {
        return ((((*(*(*self).upgrade().deref()).hi_.borrow()) as i128) << 64)
            + ((*(*(*self).upgrade().deref()).lo_.borrow()) as i128));
    }
    fn operator_unsigned___int128(&self) -> u128 {
        return (((*(*(*self).upgrade().deref()).hi_.borrow()) as u128) << 64)
            .wrapping_add(((*(*(*self).upgrade().deref()).lo_.borrow()) as u128));
    }
    fn operator_float(&self) -> f32 {
        let pow_2_64: Value<f32> = Rc::new(RefCell::new(1.844674407E+19));
        return (((*(*(*self).upgrade().deref()).lo_.borrow()) as f32)
            + (((*(*(*self).upgrade().deref()).hi_.borrow()) as f32) * (*pow_2_64.borrow())));
    }
    fn operator_double(&self) -> f64 {
        let pow_2_64: Value<f64> = Rc::new(RefCell::new(1.844674407E+19));
        return (((*(*(*self).upgrade().deref()).lo_.borrow()) as f64)
            + (((*(*(*self).upgrade().deref()).hi_.borrow()) as f64) * (*pow_2_64.borrow())));
    }
    fn operator_long_double(&self) -> f64 {
        let pow_2_64: Value<f64> = Rc::new(RefCell::new(1.844674407E+19));
        return (((*(*(*self).upgrade().deref()).lo_.borrow()) as f64)
            + (((*(*(*self).upgrade().deref()).hi_.borrow()) as f64) * (*pow_2_64.borrow())));
    }
    fn operator_bitor_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_bitor_199(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitand_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_bitand_200(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitxor_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_bitxor_201(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_post_inc_i32(&self, _a0: i32) -> absl_uint128 {
        let _a0: Value<i32> = Rc::new(RefCell::new(_a0));
        let tmp: Value<absl_uint128> = Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({ absl_uint128Impl::operator_add_assign(&(*self), absl_uint128::absl_uint1281({ 1 })) });
        return (*tmp.borrow()).clone();
    }
    fn operator_post_dec_i32(&self, _a0: i32) -> absl_uint128 {
        let _a0: Value<i32> = Rc::new(RefCell::new(_a0));
        let tmp: Value<absl_uint128> = Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({ absl_uint128Impl::operator_sub_assign(&(*self), absl_uint128::absl_uint1281({ 1 })) });
        return (*tmp.borrow()).clone();
    }
    fn operator_inc(&self) -> Ptr<absl_uint128> {
        ({ absl_uint128Impl::operator_add_assign(&(*self), absl_uint128::absl_uint1281({ 1 })) });
        return (*self).clone();
    }
    fn operator_dec(&self) -> Ptr<absl_uint128> {
        ({ absl_uint128Impl::operator_sub_assign(&(*self), absl_uint128::absl_uint1281({ 1 })) });
        return (*self).clone();
    }
}
pub trait v8_base_DiyFpImpl {
    fn Subtract(&self, other: Ptr<v8_base_DiyFp>);
    fn Multiply(&self, other: Ptr<v8_base_DiyFp>);
    fn Normalize(&self);
    fn f(&self) -> u64;
    fn e(&self) -> i32;
    fn set_f(&self, new_value: u64);
    fn set_e(&self, new_value: i32);
}
impl v8_base_DiyFpImpl for Ptr<v8_base_DiyFp> {
    fn Subtract(&self, other: Ptr<v8_base_DiyFp>) {
        (&(0));
        (&(0));
        {
            let rhs_0 = (*(*(*self).upgrade().deref()).f_.borrow())
                .wrapping_sub((*(*other.upgrade().deref()).f_.borrow()));
            (*(*(*self).upgrade().deref()).f_.borrow_mut()) = rhs_0
        };
    }
    fn Multiply(&self, other: Ptr<v8_base_DiyFp>) {
        let __rhs = ({
            let _a: Ptr<v8_base_DiyFp> = (*self).clone();
            let _b: Ptr<v8_base_DiyFp> = (other).clone();
            v8_base_DiyFp::Times(_a, _b)
        });
        (*self).write(__rhs);
    }
    fn Normalize(&self) {
        (&(0));
        let f: Value<u64> = Rc::new(RefCell::new((*(*(*self).upgrade().deref()).f_.borrow())));
        let e: Value<i32> = Rc::new(RefCell::new((*(*(*self).upgrade().deref()).e_.borrow())));
        let k10MSBits: Value<u64> = Rc::new(RefCell::new((1023_u64 << 54)));
        'loop_: while (((*f.borrow()) & (*k10MSBits.borrow())) == 0_u64) {
            (*f.borrow_mut()) <<= 10;
            (*e.borrow_mut()) -= 10;
        }
        'loop_: while (((*f.borrow()) & (*kUint64MSB_223.with(Value::clone).borrow())) == 0_u64) {
            (*f.borrow_mut()) <<= 1;
            (*e.borrow_mut()).postfix_dec();
        }
        (*(*(*self).upgrade().deref()).f_.borrow_mut()) = (*f.borrow());
        (*(*(*self).upgrade().deref()).e_.borrow_mut()) = (*e.borrow());
    }
    fn f(&self) -> u64 {
        return (*(*(*self).upgrade().deref()).f_.borrow());
    }
    fn e(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).e_.borrow());
    }
    fn set_f(&self, new_value: u64) {
        let new_value: Value<u64> = Rc::new(RefCell::new(new_value));
        (*(*(*self).upgrade().deref()).f_.borrow_mut()) = (*new_value.borrow());
    }
    fn set_e(&self, new_value: i32) {
        let new_value: Value<i32> = Rc::new(RefCell::new(new_value));
        (*(*(*self).upgrade().deref()).e_.borrow_mut()) = (*new_value.borrow());
    }
}
pub trait v8_base_DoubleImpl {
    fn AsDiyFp(&self) -> v8_base_DiyFp;
    fn AsNormalizedDiyFp(&self) -> v8_base_DiyFp;
    fn AsUint64(&self) -> u64;
    fn NextDouble(&self) -> f64;
    fn Exponent(&self) -> i32;
    fn Significand(&self) -> u64;
    fn IsDenormal(&self) -> bool;
    fn IsSpecial(&self) -> bool;
    fn IsInfinite(&self) -> bool;
    fn Sign(&self) -> i32;
    fn UpperBoundary(&self) -> v8_base_DiyFp;
    fn NormalizedBoundaries(&self, out_m_minus: Ptr<v8_base_DiyFp>, out_m_plus: Ptr<v8_base_DiyFp>);
    fn value(&self) -> f64;
}
impl v8_base_DoubleImpl for Ptr<v8_base_Double> {
    fn AsDiyFp(&self) -> v8_base_DiyFp {
        (&(0));
        (&(0));
        return v8_base_DiyFp::v8_base_DiyFp2({ ({ v8_base_DoubleImpl::Significand(self) }) }, {
            ({ v8_base_DoubleImpl::Exponent(self) })
        });
    }
    fn AsNormalizedDiyFp(&self) -> v8_base_DiyFp {
        (&(0));
        let f: Value<u64> = Rc::new(RefCell::new(({ v8_base_DoubleImpl::Significand(self) })));
        let e: Value<i32> = Rc::new(RefCell::new(({ v8_base_DoubleImpl::Exponent(self) })));
        'loop_: while (((*f.borrow()) & 4503599627370496) == 0_u64) {
            (*f.borrow_mut()) <<= 1;
            (*e.borrow_mut()).postfix_dec();
        }
        (*f.borrow_mut()) <<= ((*kSignificandSize_222.with(Value::clone).borrow()) - 53);
        (*e.borrow_mut()) -= ((*kSignificandSize_222.with(Value::clone).borrow()) - 53);
        return v8_base_DiyFp::v8_base_DiyFp2({ (*f.borrow()) }, { (*e.borrow()) });
    }
    fn AsUint64(&self) -> u64 {
        return (*(*(*self).upgrade().deref()).d64_.borrow());
    }
    fn NextDouble(&self) -> f64 {
        if ((*(*(*self).upgrade().deref()).d64_.borrow()) == 9218868437227405312) {
            return ({
                v8_base_DoubleImpl::value(
                    &Rc::new(RefCell::new(v8_base_Double::v8_base_Double3({
                        9218868437227405312
                    })))
                    .as_pointer(),
                )
            });
        }
        if (({ v8_base_DoubleImpl::Sign(self) }) < 0)
            && (({ v8_base_DoubleImpl::Significand(self) }) == 0_u64)
        {
            return 0.0E+0;
        }
        if (({ v8_base_DoubleImpl::Sign(self) }) < 0) {
            return ({
                v8_base_DoubleImpl::value(
                    &Rc::new(RefCell::new(v8_base_Double::v8_base_Double3({
                        (*(*(*self).upgrade().deref()).d64_.borrow()).wrapping_sub(1_u64)
                    })))
                    .as_pointer(),
                )
            });
        } else {
            return ({
                v8_base_DoubleImpl::value(
                    &Rc::new(RefCell::new(v8_base_Double::v8_base_Double3({
                        (*(*(*self).upgrade().deref()).d64_.borrow()).wrapping_add(1_u64)
                    })))
                    .as_pointer(),
                )
            });
        }
        panic!("ub: non-void function does not return a value")
    }
    fn Exponent(&self) -> i32 {
        if ({ v8_base_DoubleImpl::IsDenormal(self) }) {
            return -1074;
        }
        let d64: Value<u64> = Rc::new(RefCell::new(({ v8_base_DoubleImpl::AsUint64(self) })));
        let biased_e: Value<i32> = Rc::new(RefCell::new(
            ((((*d64.borrow()) & 9218868437227405312) >> 52) as i32),
        ));
        return ((*biased_e.borrow()) - 1075);
    }
    fn Significand(&self) -> u64 {
        let d64: Value<u64> = Rc::new(RefCell::new(({ v8_base_DoubleImpl::AsUint64(self) })));
        let significand: Value<u64> = Rc::new(RefCell::new(((*d64.borrow()) & 4503599627370495)));
        if !({ v8_base_DoubleImpl::IsDenormal(self) }) {
            return (*significand.borrow()).wrapping_add(4503599627370496);
        } else {
            return (*significand.borrow());
        }
        panic!("ub: non-void function does not return a value")
    }
    fn IsDenormal(&self) -> bool {
        let d64: Value<u64> = Rc::new(RefCell::new(({ v8_base_DoubleImpl::AsUint64(self) })));
        return (((*d64.borrow()) & 9218868437227405312) == 0_u64);
    }
    fn IsSpecial(&self) -> bool {
        let d64: Value<u64> = Rc::new(RefCell::new(({ v8_base_DoubleImpl::AsUint64(self) })));
        return (((*d64.borrow()) & 9218868437227405312) == 9218868437227405312);
    }
    fn IsInfinite(&self) -> bool {
        let d64: Value<u64> = Rc::new(RefCell::new(({ v8_base_DoubleImpl::AsUint64(self) })));
        return (((*d64.borrow()) & 9218868437227405312) == 9218868437227405312)
            && (((*d64.borrow()) & 4503599627370495) == 0_u64);
    }
    fn Sign(&self) -> i32 {
        let d64: Value<u64> = Rc::new(RefCell::new(({ v8_base_DoubleImpl::AsUint64(self) })));
        return if (((*d64.borrow()) & 9223372036854775808) == 0_u64) {
            1
        } else {
            -1_i32
        };
    }
    fn UpperBoundary(&self) -> v8_base_DiyFp {
        (&(0));
        return v8_base_DiyFp::v8_base_DiyFp2(
            {
                (({ v8_base_DoubleImpl::Significand(self) }).wrapping_mul(2_u64))
                    .wrapping_add(1_u64)
            },
            { (({ v8_base_DoubleImpl::Exponent(self) }) - 1) },
        );
    }
    fn NormalizedBoundaries(
        &self,
        out_m_minus: Ptr<v8_base_DiyFp>,
        out_m_plus: Ptr<v8_base_DiyFp>,
    ) {
        let out_m_minus: Value<Ptr<v8_base_DiyFp>> = Rc::new(RefCell::new(out_m_minus));
        let out_m_plus: Value<Ptr<v8_base_DiyFp>> = Rc::new(RefCell::new(out_m_plus));
        (&(0));
        let v: Value<v8_base_DiyFp> =
            Rc::new(RefCell::new(({ v8_base_DoubleImpl::AsDiyFp(self) })));
        let m_plus: Value<v8_base_DiyFp> = Rc::new(RefCell::new(
            ({
                let _a: Value<v8_base_DiyFp> =
                    Rc::new(RefCell::new(v8_base_DiyFp::v8_base_DiyFp2(
                        { (({ v8_base_DiyFpImpl::f(&v.as_pointer()) }) << 1).wrapping_add(1_u64) },
                        { (({ v8_base_DiyFpImpl::e(&v.as_pointer()) }) - 1) },
                    )));
                v8_base_DiyFp::Normalize(_a.as_pointer())
            }),
        ));
        let m_minus: Value<v8_base_DiyFp> = Rc::new(RefCell::new(v8_base_DiyFp::v8_base_DiyFp1()));
        if ((({ v8_base_DoubleImpl::AsUint64(self) }) & 4503599627370495) == 0_u64)
            && (({ v8_base_DiyFpImpl::e(&v.as_pointer()) }) != -1074)
        {
            (*m_minus.borrow_mut()) = v8_base_DiyFp::v8_base_DiyFp2(
                { (({ v8_base_DiyFpImpl::f(&v.as_pointer()) }) << 2).wrapping_sub(1_u64) },
                { (({ v8_base_DiyFpImpl::e(&v.as_pointer()) }) - 2) },
            );
        } else {
            (*m_minus.borrow_mut()) = v8_base_DiyFp::v8_base_DiyFp2(
                { (({ v8_base_DiyFpImpl::f(&v.as_pointer()) }) << 1).wrapping_sub(1_u64) },
                { (({ v8_base_DiyFpImpl::e(&v.as_pointer()) }) - 1) },
            );
        }
        ({
            let _new_value: u64 = (({ v8_base_DiyFpImpl::f(&m_minus.as_pointer()) })
                << (({ v8_base_DiyFpImpl::e(&m_minus.as_pointer()) })
                    - ({ v8_base_DiyFpImpl::e(&m_plus.as_pointer()) })));
            v8_base_DiyFpImpl::set_f(&m_minus.as_pointer(), _new_value)
        });
        ({
            v8_base_DiyFpImpl::set_e(
                &m_minus.as_pointer(),
                ({ v8_base_DiyFpImpl::e(&m_plus.as_pointer()) }),
            )
        });
        let __rhs = (*m_plus.borrow()).clone();
        (*out_m_plus.borrow()).write(__rhs);
        let __rhs = (*m_minus.borrow()).clone();
        (*out_m_minus.borrow()).write(__rhs);
    }
    fn value(&self) -> f64 {
        return ({ uint64_to_double_228((*(*(*self).upgrade().deref()).d64_.borrow())) });
    }
}
pub trait v8_base_HasherImpl {
    fn hash(&self) -> usize;
    fn AddHash(&self, other_hash: usize) -> Ptr<v8_base_Hasher>;
}
impl v8_base_HasherImpl for Ptr<v8_base_Hasher> {
    fn hash(&self) -> usize {
        return (*(*(*self).upgrade().deref()).hash_.borrow());
    }
    fn AddHash(&self, other_hash: usize) -> Ptr<v8_base_Hasher> {
        let other_hash: Value<usize> = Rc::new(RefCell::new(other_hash));
        let __rhs = ({
            hash_combine_94(
                (*(*(*self).upgrade().deref()).hash_.borrow()),
                (*other_hash.borrow()),
            )
        });
        (*(*(*self).upgrade().deref()).hash_.borrow_mut()) = __rhs;
        return (*self).clone();
    }
}
pub trait v8_base_Vector_char_Impl {
    fn operator_index(&self, index: usize) -> Ptr<u8>;
    fn data(&self) -> Ptr<u8>;
}
impl v8_base_Vector_char_Impl for Ptr<v8_base_Vector_char_> {
    fn operator_index(&self, index: usize) -> Ptr<u8> {
        let index: Value<usize> = Rc::new(RefCell::new(index));
        (&(0));
        return (*(*(*self).upgrade().deref()).start_.borrow()).offset((*index.borrow()) as isize);
    }
    fn data(&self) -> Ptr<u8> {
        return (*(*(*self).upgrade().deref()).start_.borrow()).clone();
    }
}
pub trait v8_base_bit_equal_to_double_Impl {
    fn operator_call(&self, lhs: f64, rhs: f64) -> bool;
}
impl v8_base_bit_equal_to_double_Impl for Ptr<v8_base_bit_equal_to_double_> {
    fn operator_call(&self, lhs: f64, rhs: f64) -> bool {
        let lhs: Value<f64> = Rc::new(RefCell::new(lhs));
        let rhs: Value<f64> = Rc::new(RefCell::new(rhs));
        return (({ bit_cast_28(lhs.as_pointer()) }) == ({ bit_cast_28(rhs.as_pointer()) }));
    }
}
pub trait v8_base_bit_equal_to_float_Impl {
    fn operator_call(&self, lhs: f32, rhs: f32) -> bool;
}
impl v8_base_bit_equal_to_float_Impl for Ptr<v8_base_bit_equal_to_float_> {
    fn operator_call(&self, lhs: f32, rhs: f32) -> bool {
        let lhs: Value<f32> = Rc::new(RefCell::new(lhs));
        let rhs: Value<f32> = Rc::new(RefCell::new(rhs));
        return (({ bit_cast_26(lhs.as_pointer()) }) == ({ bit_cast_26(rhs.as_pointer()) }));
    }
}
pub trait v8_base_bit_hash_double_Impl {
    fn operator_call(&self, v: f64) -> usize;
}
impl v8_base_bit_hash_double_Impl for Ptr<v8_base_bit_hash_double_> {
    fn operator_call(&self, v: f64) -> usize {
        let v: Value<f64> = Rc::new(RefCell::new(v));
        let h: Value<v8_base_hash_unsigned_long_long_> =
            Rc::new(RefCell::new(<v8_base_hash_unsigned_long_long_>::default()));
        return ({
            let _v: Value<u64> = Rc::new(RefCell::new(({ bit_cast_28(v.as_pointer()) })));
            v8_base_hash_unsigned_long_long_Impl::operator_call(&h.as_pointer(), _v.as_pointer())
        });
    }
}
pub trait v8_base_bit_hash_float_Impl {
    fn operator_call(&self, v: f32) -> usize;
}
impl v8_base_bit_hash_float_Impl for Ptr<v8_base_bit_hash_float_> {
    fn operator_call(&self, v: f32) -> usize {
        let v: Value<f32> = Rc::new(RefCell::new(v));
        let h: Value<v8_base_hash_unsigned_int_> =
            Rc::new(RefCell::new(<v8_base_hash_unsigned_int_>::default()));
        return ({
            let _v: Value<u32> = Rc::new(RefCell::new(({ bit_cast_26(v.as_pointer()) })));
            v8_base_hash_unsigned_int_Impl::operator_call(&h.as_pointer(), _v.as_pointer())
        });
    }
}
pub trait v8_base_hash_unsigned_int_Impl {
    fn operator_call(&self, v: Ptr<u32>) -> usize;
}
impl v8_base_hash_unsigned_int_Impl for Ptr<v8_base_hash_unsigned_int_> {
    fn operator_call(&self, v: Ptr<u32>) -> usize {
        return ({ hash_value_92((v.read())) });
    }
}
pub trait v8_base_hash_unsigned_long_long_Impl {
    fn operator_call(&self, v: Ptr<u64>) -> usize;
}
impl v8_base_hash_unsigned_long_long_Impl for Ptr<v8_base_hash_unsigned_long_long_> {
    fn operator_call(&self, v: Ptr<u64>) -> usize {
        return ({ hash_value_93((v.read())) });
    }
}
