use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_ {
    value_: Value<bool>,
}
impl v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_ {
    pub fn v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_1(v: Ptr<bool>) -> Self {
        let __this: Value<v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_> =
            Rc::new(RefCell::new(Self {
                value_: Rc::new(RefCell::new((v.read()))),
            }));
        let this: Ptr<v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_> =
            Rc::new(RefCell::new(Self {
                value_: Rc::new(RefCell::new((*self.value_.borrow()))),
            }));
        let this: Ptr<v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.value_.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[0..1]))),
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
pub fn make_uint64_30(high: u32, low: u32) -> u64 {
    let high: Value<u32> = Rc::new(RefCell::new(high));
    let low: Value<u32> = Rc::new(RefCell::new(low));
    return (((*high.borrow()) as u64) << 32).wrapping_add(((*low.borrow()) as u64));
}
pub fn RoundDown_31(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    return ((*x.borrow()) & (-(*m.borrow()) as u64));
}
pub fn RoundUp_32(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    (&(0));
    return ({
        let _x: u64 = ((*x.borrow()).wrapping_add((((*m.borrow()) - 1_i64) as u64)) as u64);
        let _m: i64 = (*m.borrow());
        RoundDown_31(_x, _m)
    });
}
pub fn IsAligned_33(value: u64, alignment: u64) -> bool {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let alignment: Value<u64> = Rc::new(RefCell::new(alignment));
    return (((*value.borrow()) & ((*alignment.borrow()).wrapping_sub(1_u64))) == 0_u64);
}
pub fn CountLeadingZeros_34(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        if (32_u32 == 64_u32) {
            (({ __builtin_clzll_35(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((32_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros_36(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        if (64_u32 == 64_u32) {
            (({ __builtin_clzll_35(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((64_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros_37(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        if (64_u32 == 64_u32) {
            (({ __builtin_clzll_35((*value.borrow())) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((64_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros32_38(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_34((*value.borrow())) });
}
pub fn CountLeadingZeros64_39(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_36((*value.borrow())) });
}
pub fn CountTrailingZeros_40(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        (if (32_u32 == 64_u32) {
            ({ __builtin_ctzll_41(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros_42(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        (if (64_u32 == 64_u32) {
            ({ __builtin_ctzll_41(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros32_43(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_40((*value.borrow())) });
}
pub fn CountTrailingZeros64_44(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_42((*value.borrow())) });
}
pub fn RoundUpToPowerOfTwo32_45(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u32 << ((32_u32).wrapping_sub(({ CountLeadingZeros_34((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo64_46(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u64 << ((64_u32).wrapping_sub(({ CountLeadingZeros_36((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo_47(value: usize) -> usize {
    let value: Value<usize> = Rc::new(RefCell::new(value));
    if (::std::mem::size_of::<usize>() == ::std::mem::size_of::<u64>()) {
        return (({ RoundUpToPowerOfTwo64_46(((*value.borrow()) as u64)) }) as usize);
    } else {
        return (({ RoundUpToPowerOfTwo32_45(((*value.borrow()) as u32)) }) as usize);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn RoundDownToPowerOfTwo32_48(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    if ((*value.borrow()) > 2147483648_u32) {
        return 2147483648_u32;
    }
    let result: Value<u32> = Rc::new(RefCell::new(
        ({ RoundUpToPowerOfTwo32_45((*value.borrow())) }),
    ));
    if ((*result.borrow()) > (*value.borrow())) {
        (*result.borrow_mut()) >>= 1;
    }
    return (*result.borrow());
}
pub fn RotateRight32_49(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateLeft32_50(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateRight64_51(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn RotateLeft64_52(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn ClearLsb_53(value: i32) -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(value));
    return ((*value.borrow()) & ((*value.borrow()) - 1));
}
pub fn SignedAddOverflow32_54(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_sadd_overflow_55(_arg0, _arg1, _arg2)
    });
}
pub fn SignedSubOverflow32_56(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_ssub_overflow_57(_arg0, _arg1, _arg2)
    });
}
pub fn SignedMulOverflow32_58(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_smul_overflow_59(_arg0, _arg1, _arg2)
    });
}
pub fn SignedAddOverflow64_60(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_add_overflow_61(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedSubOverflow64_62(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_sub_overflow_63(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedMulOverflow64_64(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return {
        let (val, ovf) = (*lhs.borrow()).overflowing_mul((*rhs.borrow()));
        (*val.borrow()).write(val);
        ovf
    };
}
pub fn UnsignedAddOverflow32_65(lhs: u32, rhs: u32, val: Ptr<u32>) -> bool {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<u32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: u32 = (*lhs.borrow());
        let _arg1: u32 = (*rhs.borrow());
        let _arg2: Ptr<u32> = (*val.borrow()).clone();
        __builtin_uadd_overflow_66(_arg0, _arg1, _arg2)
    });
}
pub fn UnsignedDiv32_67(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedDiv64_68(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn UnsignedMod32_69(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedMod64_70(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn WraparoundAdd32_71(lhs: i32, rhs: i32) -> i32 {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as u32).wrapping_add(((*rhs.borrow()) as u32))) as i32);
}
pub fn WraparoundNeg32_72(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (-((*x.borrow()) as u32) as i32);
}
pub fn ByteReverse16_73(value: u16) -> u16 {
    let value: Value<u16> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse32_74(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse64_75(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn OverlappingWrites_76(dst: AnyPtr, src: AnyPtr, count: usize) {
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
pub fn OverlappingWrites_77(dst: AnyPtr, src: AnyPtr, count: usize) {
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
pub fn OverlappingWrites_78(dst: AnyPtr, src: AnyPtr, count: usize) {
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
pub fn SimdMemCopy_79(dst: AnyPtr, src: AnyPtr, count: usize) {
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
            (({ CountLeadingZeros_37(((*count.borrow()).wrapping_sub(1_usize) as u64)) }) as usize),
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
                    OverlappingWrites_76(
                        ((*dst_u.borrow()).clone() as Ptr<u8>).to_any(),
                        ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                        (*count.borrow()),
                    )
                });
                return;
            }
            __v if __v == 3 => {
                ({
                    OverlappingWrites_77(
                        ((*dst_u.borrow()).clone() as Ptr<u8>).to_any(),
                        ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                        (*count.borrow()),
                    )
                });
                return;
            }
            __v if __v == 4 => {
                ({
                    OverlappingWrites_78(
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
                            __builtin_neon_vld1q_v_80(
                                ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                                48,
                            )
                        });
                        let __result = (*__ret.borrow());
                        __result
                    }));
                    let __result = ({
                        __builtin_neon_vst1q_v_81(
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
                            __builtin_neon_vld1q_v_80(
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
                        __builtin_neon_vst1q_v_81(
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
                            __builtin_neon_vld1q_v_80(
                                ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                                48,
                            )
                        });
                        let __result = (*__ret.borrow());
                        __result
                    }));
                    let __result = ({
                        __builtin_neon_vst1q_v_81(
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
                                __builtin_neon_vld1q_v_80(
                                    ((*src_u.borrow()).offset((*i.borrow()) as isize) as Ptr<u8>)
                                        .to_any(),
                                    48,
                                )
                            });
                            let __result = (*__ret.borrow());
                            __result
                        }));
                        let __result = ({
                            __builtin_neon_vst1q_v_81(
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
pub fn MemCopy_82(dest: AnyPtr, src: AnyPtr, size: usize) {
    let dest: Value<AnyPtr> = Rc::new(RefCell::new(dest));
    let src: Value<AnyPtr> = Rc::new(RefCell::new(src));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    ({
        SimdMemCopy_79(
            (*dest.borrow()).clone(),
            (*src.borrow()).clone(),
            (*size.borrow()),
        )
    });
}
pub fn MemMove_83(dest: AnyPtr, src: AnyPtr, size: usize) {
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
    pub static kCompressionFactor_84: Value<u32> = Rc::new(RefCell::new(1));
);
thread_local!(
    pub static kExpansionFactor_85: Value<u32> = Rc::new(RefCell::new(1));
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
pub fn rapid_mul128_86(A: u64, B: u64) -> (Value<u64>, Value<u64>) {
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
pub fn rapid_mix_87(A: u64, B: u64) -> u64 {
    let A: Value<u64> = Rc::new(RefCell::new(A));
    let B: Value<u64> = Rc::new(RefCell::new(B));
    let __rhs = ({ rapid_mul128_86((*A.borrow()), (*B.borrow())) });
    ({ tie_88(A.as_pointer(), B.as_pointer()) }) = __rhs;
    return ((*A.borrow()) ^ (*B.borrow()));
}
thread_local!(
    pub static RAPIDHASH_DEFAULT_SECRET_89: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
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
pub fn hash_combine_92(seed: usize, hash: usize) -> usize {
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
    pub static kRapidhashSecret1_93: Value<u64> = Rc::new(RefCell::new(3257665815644502181));
);
thread_local!(
    pub static kRapidhashSecret2_94: Value<u64> = Rc::new(RefCell::new(10067880064238660809));
);
pub fn hash64_95(key: u64) -> u64 {
    let key: Value<u64> = Rc::new(RefCell::new(key));
    return ({
        let _A: u64 = ((*key.borrow()) ^ 3257665815644502181);
        let _B: u64 = ((*key.borrow()) ^ 10067880064238660809);
        rapid_mix_87(_A, _B)
    });
}
pub fn hash32_96(key: u32) -> u32 {
    let key: Value<u32> = Rc::new(RefCell::new(key));
    return (({ hash64_95(((*key.borrow()) as u64)) }) as u32);
}
pub fn hash_value_97(v: bool) -> usize {
    let v: Value<bool> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) as usize);
}
pub fn hash_value_98(v: u8) -> usize {
    let v: Value<u8> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) as usize);
}
pub fn hash_value_99(v: u16) -> usize {
    let v: Value<u16> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) as usize);
}
pub fn hash_value_90(v: u32) -> usize {
    let v: Value<u32> = Rc::new(RefCell::new(v));
    return (({ hash32_96((*v.borrow())) }) as usize);
}
pub fn hash_value_100(v: u64) -> usize {
    let v: Value<u64> = Rc::new(RefCell::new(v));
    if false {
        return (({ hash32_96(((*v.borrow()) as u32)) }) as usize);
    } else {
        return (({ hash64_95((*v.borrow())) }) as usize);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn hash_value_91(v: u64) -> usize {
    let v: Value<u64> = Rc::new(RefCell::new(v));
    return (({ hash64_95((*v.borrow())) }) as usize);
}
pub fn hash_value_101(v: i8) -> usize {
    let v: Value<i8> = Rc::new(RefCell::new(v));
    return ({ hash_value_98(({ bit_cast_16(v.as_pointer()) })) });
}
pub fn hash_value_102(v: i16) -> usize {
    let v: Value<i16> = Rc::new(RefCell::new(v));
    return ({ hash_value_99(({ bit_cast_18(v.as_pointer()) })) });
}
pub fn hash_value_103(v: i32) -> usize {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    return ({ hash_value_90(({ bit_cast_20(v.as_pointer()) })) });
}
pub fn hash_value_104(v: i64) -> usize {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    return ({ hash_value_100(({ bit_cast_22(v.as_pointer()) })) });
}
pub fn hash_value_105(v: i64) -> usize {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    return ({ hash_value_91(({ bit_cast_24(v.as_pointer()) })) });
}
pub fn hash_value_106(v: f32) -> usize {
    let v: Value<f32> = Rc::new(RefCell::new(v));
    return if ((*v.borrow()) != 0.0E+0) {
        ({ hash_value_90(({ bit_cast_26(v.as_pointer()) })) })
    } else {
        0_usize
    };
}
pub fn hash_value_107(v: f64) -> usize {
    let v: Value<f64> = Rc::new(RefCell::new(v));
    return if ((*v.borrow()) != 0.0E+0) {
        ({ hash_value_91(({ bit_cast_28(v.as_pointer()) })) })
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
pub struct v8_base_Vector_const_v8_internal_wasm_WellKnownImport_ {
    start_: Value<Ptr>,
    length_: Value<usize>,
}
impl v8_base_Vector_const_v8_internal_wasm_WellKnownImport_ {}
impl Clone for v8_base_Vector_const_v8_internal_wasm_WellKnownImport_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Vector_const_v8_internal_wasm_WellKnownImport_> =
            Rc::new(RefCell::new(Self {
                start_: Rc::new(RefCell::new((*self.start_.borrow()).clone())),
                length_: Rc::new(RefCell::new((*self.length_.borrow()))),
            }));
        let this: Ptr<v8_base_Vector_const_v8_internal_wasm_WellKnownImport_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_Vector_const_v8_internal_wasm_WellKnownImport_ {
    fn default() -> Self {
        {
            v8_base_Vector_const_v8_internal_wasm_WellKnownImport_ :: v8_base_Vector_const_v8_internal_wasm_WellKnownImport_5 ( )
        }
    }
}
impl ByteRepr for v8_base_Vector_const_v8_internal_wasm_WellKnownImport_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.start_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.length_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            start_: Rc::new(RefCell::new(<Ptr>::from_bytes(&buf[0..8]))),
            length_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive()]
pub struct v8_base_Vector_v8_internal_wasm_WellKnownImport_ {
    start_: Value<Ptr>,
    length_: Value<usize>,
}
impl v8_base_Vector_v8_internal_wasm_WellKnownImport_ {}
impl Clone for v8_base_Vector_v8_internal_wasm_WellKnownImport_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Vector_v8_internal_wasm_WellKnownImport_> =
            Rc::new(RefCell::new(Self {
                start_: Rc::new(RefCell::new((*self.start_.borrow()).clone())),
                length_: Rc::new(RefCell::new((*self.length_.borrow()))),
            }));
        let this: Ptr<v8_base_Vector_v8_internal_wasm_WellKnownImport_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_Vector_v8_internal_wasm_WellKnownImport_ {
    fn default() -> Self {
        {
            v8_base_Vector_v8_internal_wasm_WellKnownImport_ :: v8_base_Vector_v8_internal_wasm_WellKnownImport_6 ( )
        }
    }
}
impl ByteRepr for v8_base_Vector_v8_internal_wasm_WellKnownImport_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.start_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.length_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            start_: Rc::new(RefCell::new(<Ptr>::from_bytes(&buf[0..8]))),
            length_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
        }
    }
}
thread_local!(
    pub static enable_view_108: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static enable_borrowed_range_109: Value<bool> = Rc::new(RefCell::new(true));
);
pub fn CStrVector_110(data: Ptr<u8>) -> v8_base_Vector_const_char_ {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    return v8_base_Vector_const_char_::v8_base_Vector_const_char_1(
        { (*data.borrow()).clone() },
        { (*data.borrow()).to_c_string_iterator().count() },
    );
}
pub fn StrVector_111(str: Vec<u8>) -> v8_base_Vector_const_char_ {
    let str: Value<Vec<u8>> = Rc::new(RefCell::new(str));
    return v8_base_Vector_const_char_::v8_base_Vector_const_char_1(
        { ({ (*str.borrow()).data() }) },
        { (*str.borrow()).len() },
    );
}
pub fn OneByteVector_112(data: Ptr<u8>, length: usize) -> v8_base_Vector_const_unsigned_char_ {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    return v8_base_Vector_const_unsigned_char_::v8_base_Vector_const_unsigned_char_3(
        { (*data.borrow()).reinterpret_cast::<u8>() },
        { (*length.borrow()) },
    );
}
pub fn OneByteVector_113(data: Ptr<u8>) -> v8_base_Vector_const_unsigned_char_ {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    return ({
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _length: usize = (*data.borrow()).to_c_string_iterator().count();
        OneByteVector_112(_data, _length)
    });
}
pub type v8_internal_wasm_WellKnownImport = u8;
pub const v8_internal_wasm_WellKnownImport_kUninstantiated: v8_internal_wasm_WellKnownImport = 0;
pub const v8_internal_wasm_WellKnownImport_kGeneric: v8_internal_wasm_WellKnownImport = 1;
pub const v8_internal_wasm_WellKnownImport_kLinkError: v8_internal_wasm_WellKnownImport = 2;
pub const v8_internal_wasm_WellKnownImport_kFirstCompileTimeImport:
    v8_internal_wasm_WellKnownImport = 3;
pub const v8_internal_wasm_WellKnownImport_kStringCast: v8_internal_wasm_WellKnownImport = 3;
pub const v8_internal_wasm_WellKnownImport_kStringCastShared: v8_internal_wasm_WellKnownImport = 4;
pub const v8_internal_wasm_WellKnownImport_kStringCharCodeAt: v8_internal_wasm_WellKnownImport = 5;
pub const v8_internal_wasm_WellKnownImport_kStringCharCodeAtShared:
    v8_internal_wasm_WellKnownImport = 6;
pub const v8_internal_wasm_WellKnownImport_kStringCodePointAt: v8_internal_wasm_WellKnownImport = 7;
pub const v8_internal_wasm_WellKnownImport_kStringCodePointAtShared:
    v8_internal_wasm_WellKnownImport = 8;
pub const v8_internal_wasm_WellKnownImport_kStringCompare: v8_internal_wasm_WellKnownImport = 9;
pub const v8_internal_wasm_WellKnownImport_kStringCompareShared: v8_internal_wasm_WellKnownImport =
    10;
pub const v8_internal_wasm_WellKnownImport_kStringConcat: v8_internal_wasm_WellKnownImport = 11;
pub const v8_internal_wasm_WellKnownImport_kStringConcatShared: v8_internal_wasm_WellKnownImport =
    12;
pub const v8_internal_wasm_WellKnownImport_kStringEquals: v8_internal_wasm_WellKnownImport = 13;
pub const v8_internal_wasm_WellKnownImport_kStringEqualsShared: v8_internal_wasm_WellKnownImport =
    14;
pub const v8_internal_wasm_WellKnownImport_kStringFromCharCode: v8_internal_wasm_WellKnownImport =
    15;
pub const v8_internal_wasm_WellKnownImport_kStringFromCharCodeShared:
    v8_internal_wasm_WellKnownImport = 16;
pub const v8_internal_wasm_WellKnownImport_kStringFromCodePoint: v8_internal_wasm_WellKnownImport =
    17;
pub const v8_internal_wasm_WellKnownImport_kStringFromCodePointShared:
    v8_internal_wasm_WellKnownImport = 18;
pub const v8_internal_wasm_WellKnownImport_kStringFromUtf8Array: v8_internal_wasm_WellKnownImport =
    19;
pub const v8_internal_wasm_WellKnownImport_kStringFromUtf8ArrayShared:
    v8_internal_wasm_WellKnownImport = 20;
pub const v8_internal_wasm_WellKnownImport_kStringFromWtf16Array: v8_internal_wasm_WellKnownImport =
    21;
pub const v8_internal_wasm_WellKnownImport_kStringFromWtf16ArrayShared:
    v8_internal_wasm_WellKnownImport = 22;
pub const v8_internal_wasm_WellKnownImport_kStringIntoUtf8Array: v8_internal_wasm_WellKnownImport =
    23;
pub const v8_internal_wasm_WellKnownImport_kStringIntoUtf8ArrayShared:
    v8_internal_wasm_WellKnownImport = 24;
pub const v8_internal_wasm_WellKnownImport_kStringLength: v8_internal_wasm_WellKnownImport = 25;
pub const v8_internal_wasm_WellKnownImport_kStringLengthShared: v8_internal_wasm_WellKnownImport =
    26;
pub const v8_internal_wasm_WellKnownImport_kStringMeasureUtf8: v8_internal_wasm_WellKnownImport =
    27;
pub const v8_internal_wasm_WellKnownImport_kStringMeasureUtf8Shared:
    v8_internal_wasm_WellKnownImport = 28;
pub const v8_internal_wasm_WellKnownImport_kStringSubstring: v8_internal_wasm_WellKnownImport = 29;
pub const v8_internal_wasm_WellKnownImport_kStringSubstringShared:
    v8_internal_wasm_WellKnownImport = 30;
pub const v8_internal_wasm_WellKnownImport_kStringTest: v8_internal_wasm_WellKnownImport = 31;
pub const v8_internal_wasm_WellKnownImport_kStringTestShared: v8_internal_wasm_WellKnownImport = 32;
pub const v8_internal_wasm_WellKnownImport_kStringToUtf8Array: v8_internal_wasm_WellKnownImport =
    33;
pub const v8_internal_wasm_WellKnownImport_kStringToUtf8ArrayShared:
    v8_internal_wasm_WellKnownImport = 34;
pub const v8_internal_wasm_WellKnownImport_kStringToWtf16Array: v8_internal_wasm_WellKnownImport =
    35;
pub const v8_internal_wasm_WellKnownImport_kStringToWtf16ArrayShared:
    v8_internal_wasm_WellKnownImport = 36;
pub const v8_internal_wasm_WellKnownImport_kConfigureAllPrototypes:
    v8_internal_wasm_WellKnownImport = 37;
pub const v8_internal_wasm_WellKnownImport_kLastCompileTimeImport:
    v8_internal_wasm_WellKnownImport = 37;
pub const v8_internal_wasm_WellKnownImport_kDataViewGetBigInt64: v8_internal_wasm_WellKnownImport =
    38;
pub const v8_internal_wasm_WellKnownImport_kDataViewGetBigUint64: v8_internal_wasm_WellKnownImport =
    39;
pub const v8_internal_wasm_WellKnownImport_kDataViewGetFloat32: v8_internal_wasm_WellKnownImport =
    40;
pub const v8_internal_wasm_WellKnownImport_kDataViewGetFloat64: v8_internal_wasm_WellKnownImport =
    41;
pub const v8_internal_wasm_WellKnownImport_kDataViewGetInt8: v8_internal_wasm_WellKnownImport = 42;
pub const v8_internal_wasm_WellKnownImport_kDataViewGetInt16: v8_internal_wasm_WellKnownImport = 43;
pub const v8_internal_wasm_WellKnownImport_kDataViewGetInt32: v8_internal_wasm_WellKnownImport = 44;
pub const v8_internal_wasm_WellKnownImport_kDataViewGetUint8: v8_internal_wasm_WellKnownImport = 45;
pub const v8_internal_wasm_WellKnownImport_kDataViewGetUint16: v8_internal_wasm_WellKnownImport =
    46;
pub const v8_internal_wasm_WellKnownImport_kDataViewGetUint32: v8_internal_wasm_WellKnownImport =
    47;
pub const v8_internal_wasm_WellKnownImport_kDataViewSetBigInt64: v8_internal_wasm_WellKnownImport =
    48;
pub const v8_internal_wasm_WellKnownImport_kDataViewSetBigUint64: v8_internal_wasm_WellKnownImport =
    49;
pub const v8_internal_wasm_WellKnownImport_kDataViewSetFloat32: v8_internal_wasm_WellKnownImport =
    50;
pub const v8_internal_wasm_WellKnownImport_kDataViewSetFloat64: v8_internal_wasm_WellKnownImport =
    51;
pub const v8_internal_wasm_WellKnownImport_kDataViewSetInt8: v8_internal_wasm_WellKnownImport = 52;
pub const v8_internal_wasm_WellKnownImport_kDataViewSetInt16: v8_internal_wasm_WellKnownImport = 53;
pub const v8_internal_wasm_WellKnownImport_kDataViewSetInt32: v8_internal_wasm_WellKnownImport = 54;
pub const v8_internal_wasm_WellKnownImport_kDataViewSetUint8: v8_internal_wasm_WellKnownImport = 55;
pub const v8_internal_wasm_WellKnownImport_kDataViewSetUint16: v8_internal_wasm_WellKnownImport =
    56;
pub const v8_internal_wasm_WellKnownImport_kDataViewSetUint32: v8_internal_wasm_WellKnownImport =
    57;
pub const v8_internal_wasm_WellKnownImport_kDataViewByteLength: v8_internal_wasm_WellKnownImport =
    58;
pub const v8_internal_wasm_WellKnownImport_kMathF64Acos: v8_internal_wasm_WellKnownImport = 59;
pub const v8_internal_wasm_WellKnownImport_kMathF64Asin: v8_internal_wasm_WellKnownImport = 60;
pub const v8_internal_wasm_WellKnownImport_kMathF64Atan: v8_internal_wasm_WellKnownImport = 61;
pub const v8_internal_wasm_WellKnownImport_kMathF64Atan2: v8_internal_wasm_WellKnownImport = 62;
pub const v8_internal_wasm_WellKnownImport_kMathF64Cos: v8_internal_wasm_WellKnownImport = 63;
pub const v8_internal_wasm_WellKnownImport_kMathF64Sin: v8_internal_wasm_WellKnownImport = 64;
pub const v8_internal_wasm_WellKnownImport_kMathF64Tan: v8_internal_wasm_WellKnownImport = 65;
pub const v8_internal_wasm_WellKnownImport_kMathF64Exp: v8_internal_wasm_WellKnownImport = 66;
pub const v8_internal_wasm_WellKnownImport_kMathF64Log: v8_internal_wasm_WellKnownImport = 67;
pub const v8_internal_wasm_WellKnownImport_kMathF64Pow: v8_internal_wasm_WellKnownImport = 68;
pub const v8_internal_wasm_WellKnownImport_kMathF64Sqrt: v8_internal_wasm_WellKnownImport = 69;
pub const v8_internal_wasm_WellKnownImport_kDoubleToString: v8_internal_wasm_WellKnownImport = 70;
pub const v8_internal_wasm_WellKnownImport_kIntToString: v8_internal_wasm_WellKnownImport = 71;
pub const v8_internal_wasm_WellKnownImport_kParseFloat: v8_internal_wasm_WellKnownImport = 72;
pub const v8_internal_wasm_WellKnownImport_kStringIndexOf: v8_internal_wasm_WellKnownImport = 73;
pub const v8_internal_wasm_WellKnownImport_kStringIndexOfImported:
    v8_internal_wasm_WellKnownImport = 74;
pub const v8_internal_wasm_WellKnownImport_kStringToLocaleLowerCaseStringref:
    v8_internal_wasm_WellKnownImport = 75;
pub const v8_internal_wasm_WellKnownImport_kStringToLowerCaseStringref:
    v8_internal_wasm_WellKnownImport = 76;
pub const v8_internal_wasm_WellKnownImport_kStringToLowerCaseImported:
    v8_internal_wasm_WellKnownImport = 77;
pub const v8_internal_wasm_WellKnownImport_kFastAPICall: v8_internal_wasm_WellKnownImport = 78;
pub fn IsCompileTimeImport_114(wki: v8_internal_wasm_WellKnownImport) -> bool {
    let wki: Value<v8_internal_wasm_WellKnownImport> = Rc::new(RefCell::new(wki));
    let num: Value<u8> = Rc::new(RefCell::new(((*wki.borrow()) as u8)));
    let kFirst: Value<u8> = Rc::new(RefCell::new(3));
    let kLast: Value<u8> = Rc::new(RefCell::new(37));
    return ((3 as i32) <= ((*num.borrow()) as i32)) && (((*num.borrow()) as i32) <= (37 as i32));
}
thread_local!(
    pub static  kFoundIncompatibility_115 : Value<v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_ > = Rc::new(RefCell::new(v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_ :: v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_1 ( { static __tmp_0 : Value<bool > = Rc::new(RefCell::new(false )); __tmp_0.with(Value::as_pointer)  } , ) )) ;
);
thread_local!(
    pub static  kOK_116 : Value<v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_ > = Rc::new(RefCell::new(v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_ :: v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_1 ( { static __tmp_1 : Value<bool > = Rc::new(RefCell::new(true )); __tmp_1.with(Value::as_pointer)  } , ) )) ;
);
#[derive(Default)]
pub struct v8_internal_wasm_WellKnownImportsList {
    statuses_: Value<Option<Value<Box<[std_atomic_v8_internal_wasm_WellKnownImport_]>>>>,
}
impl v8_internal_wasm_WellKnownImportsList {
    pub fn WellKnownImportsList_pmutv8_internal_wasm_WellKnownImportsList(
        _a0: Ptr<v8_internal_wasm_WellKnownImportsList>,
    ) -> Self {
        let __this: Value<v8_internal_wasm_WellKnownImportsList> = Rc::new(RefCell::new(Self {
            statuses_: Rc::new(RefCell::new(
                (*(*_a0.upgrade().deref()).statuses_.borrow_mut()).take(),
            )),
        }));
        let this: Ptr<v8_internal_wasm_WellKnownImportsList> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_internal_wasm_WellKnownImportsList {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.statuses_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            statuses_: Rc::new(RefCell::new(<Option<
                Value<Box<[std_atomic_v8_internal_wasm_WellKnownImport_]>>,
            >>::from_bytes(&buf[0..8]))),
        }
    }
}
pub fn WellKnownImportName_117(wki: v8_internal_wasm_WellKnownImport) -> Ptr<u8> {
    let wki: Value<v8_internal_wasm_WellKnownImport> = Rc::new(RefCell::new(wki));
    'switch: {
        let __match_cond = (*wki.borrow());
        match __match_cond {
            __v if __v == 0 => {
                return Ptr::from_string_literal(b"uninstantiated");
            }
            __v if __v == 1 => {
                return Ptr::from_string_literal(b"generic");
            }
            __v if __v == 2 => {
                return Ptr::from_string_literal(b"LinkError");
            }
            __v if __v == 38 => {
                return Ptr::from_string_literal(b"DataView.getBigInt64");
            }
            __v if __v == 39 => {
                return Ptr::from_string_literal(b"DataView.getBigUint64");
            }
            __v if __v == 40 => {
                return Ptr::from_string_literal(b"DataView.getFloat32");
            }
            __v if __v == 41 => {
                return Ptr::from_string_literal(b"DataView.getFloat64");
            }
            __v if __v == 42 => {
                return Ptr::from_string_literal(b"DataView.getInt8");
            }
            __v if __v == 43 => {
                return Ptr::from_string_literal(b"DataView.getInt16");
            }
            __v if __v == 44 => {
                return Ptr::from_string_literal(b"DataView.getInt32");
            }
            __v if __v == 45 => {
                return Ptr::from_string_literal(b"DataView.getUint8");
            }
            __v if __v == 46 => {
                return Ptr::from_string_literal(b"DataView.getUint16");
            }
            __v if __v == 47 => {
                return Ptr::from_string_literal(b"DataView.getUint32");
            }
            __v if __v == 48 => {
                return Ptr::from_string_literal(b"DataView.setBigInt64");
            }
            __v if __v == 49 => {
                return Ptr::from_string_literal(b"DataView.setBigUint64");
            }
            __v if __v == 50 => {
                return Ptr::from_string_literal(b"DataView.setFloat32");
            }
            __v if __v == 51 => {
                return Ptr::from_string_literal(b"DataView.setFloat64");
            }
            __v if __v == 52 => {
                return Ptr::from_string_literal(b"DataView.setInt8");
            }
            __v if __v == 53 => {
                return Ptr::from_string_literal(b"DataView.setInt16");
            }
            __v if __v == 54 => {
                return Ptr::from_string_literal(b"DataView.setInt32");
            }
            __v if __v == 55 => {
                return Ptr::from_string_literal(b"DataView.setUint8");
            }
            __v if __v == 56 => {
                return Ptr::from_string_literal(b"DataView.setUint16");
            }
            __v if __v == 57 => {
                return Ptr::from_string_literal(b"DataView.setUint32");
            }
            __v if __v == 58 => {
                return Ptr::from_string_literal(b"DataView.byteLength");
            }
            __v if __v == 59 => {
                return Ptr::from_string_literal(b"Math.acos");
            }
            __v if __v == 60 => {
                return Ptr::from_string_literal(b"Math.asin");
            }
            __v if __v == 61 => {
                return Ptr::from_string_literal(b"Math.atan");
            }
            __v if __v == 62 => {
                return Ptr::from_string_literal(b"Math.atan2");
            }
            __v if __v == 63 => {
                return Ptr::from_string_literal(b"Math.cos");
            }
            __v if __v == 64 => {
                return Ptr::from_string_literal(b"Math.sin");
            }
            __v if __v == 65 => {
                return Ptr::from_string_literal(b"Math.tan");
            }
            __v if __v == 66 => {
                return Ptr::from_string_literal(b"Math.exp");
            }
            __v if __v == 67 => {
                return Ptr::from_string_literal(b"Math.log");
            }
            __v if __v == 68 => {
                return Ptr::from_string_literal(b"Math.pow");
            }
            __v if __v == 69 => {
                return Ptr::from_string_literal(b"Math.sqrt");
            }
            __v if __v == 70 => {
                return Ptr::from_string_literal(b"DoubleToString");
            }
            __v if __v == 71 => {
                return Ptr::from_string_literal(b"IntToString");
            }
            __v if __v == 72 => {
                return Ptr::from_string_literal(b"ParseFloat");
            }
            __v if __v == 73 || __v == 74 => {
                return Ptr::from_string_literal(b"String.indexOf");
            }
            __v if __v == 75 => {
                return Ptr::from_string_literal(b"String.toLocaleLowerCase");
            }
            __v if __v == 76 || __v == 77 => {
                return Ptr::from_string_literal(b"String.toLowerCase");
            }
            __v if __v == 3 || __v == 4 => {
                return Ptr::from_string_literal(b"js-string:cast");
            }
            __v if __v == 5 || __v == 6 => {
                return Ptr::from_string_literal(b"js-string:charCodeAt");
            }
            __v if __v == 7 || __v == 8 => {
                return Ptr::from_string_literal(b"js-string:codePointAt");
            }
            __v if __v == 9 || __v == 10 => {
                return Ptr::from_string_literal(b"js-string:compare");
            }
            __v if __v == 11 || __v == 12 => {
                return Ptr::from_string_literal(b"js-string:concat");
            }
            __v if __v == 13 || __v == 14 => {
                return Ptr::from_string_literal(b"js-string:equals");
            }
            __v if __v == 15 || __v == 16 => {
                return Ptr::from_string_literal(b"js-string:fromCharCode");
            }
            __v if __v == 17 || __v == 18 => {
                return Ptr::from_string_literal(b"js-string:fromCodePoint");
            }
            __v if __v == 19 || __v == 20 => {
                return Ptr::from_string_literal(b"text-decoder:decodeStringFromUTF8Array");
            }
            __v if __v == 21 || __v == 22 => {
                return Ptr::from_string_literal(b"js-string:fromCharCodeArray");
            }
            __v if __v == 23 || __v == 24 => {
                return Ptr::from_string_literal(b"text-encoder:encodeStringIntoUTF8Array");
            }
            __v if __v == 33 || __v == 34 => {
                return Ptr::from_string_literal(b"text-encoder:encodeStringToUTF8Array");
            }
            __v if __v == 25 || __v == 26 => {
                return Ptr::from_string_literal(b"js-string:length");
            }
            __v if __v == 27 || __v == 28 => {
                return Ptr::from_string_literal(b"text-encoder:measureStringAsUTF8");
            }
            __v if __v == 29 || __v == 30 => {
                return Ptr::from_string_literal(b"js-string:substring");
            }
            __v if __v == 31 || __v == 32 => {
                return Ptr::from_string_literal(b"js-string:test");
            }
            __v if __v == 35 || __v == 36 => {
                return Ptr::from_string_literal(b"js-string:intoCharCodeArray");
            }
            __v if __v == 37 => {
                return Ptr::from_string_literal(b"js-prototypes:configureAll");
            }
            __v if __v == 78 => {
                return Ptr::from_string_literal(b"fast API call");
            }
            _ => {}
        }
    };
    ({
        V8_Fatal_118(
            (*kUnreachableCodeMessage_9.with(Value::clone).borrow()).clone(),
            &[],
        )
    });
    panic!("ub: non-void function does not return a value")
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
            hash_combine_92(
                (*(*(*self).upgrade().deref()).hash_.borrow()),
                (*other_hash.borrow()),
            )
        });
        (*(*(*self).upgrade().deref()).hash_.borrow_mut()) = __rhs;
        return (*self).clone();
    }
}
pub trait v8_base_Vector_const_v8_internal_wasm_WellKnownImport_Impl {
    fn size(&self) -> usize;
    fn operator_index(&self, index: usize) -> Ptr;
}
impl v8_base_Vector_const_v8_internal_wasm_WellKnownImport_Impl
    for Ptr<v8_base_Vector_const_v8_internal_wasm_WellKnownImport_>
{
    fn size(&self) -> usize {
        return (*(*(*self).upgrade().deref()).length_.borrow());
    }
    fn operator_index(&self, index: usize) -> Ptr {
        let index: Value<usize> = Rc::new(RefCell::new(index));
        (&(0));
        return (*(*(*self).upgrade().deref()).start_.borrow()).offset((*index.borrow()) as isize);
    }
}
pub trait v8_base_Vector_v8_internal_wasm_WellKnownImport_Impl {
    fn size(&self) -> usize;
    fn operator_index(&self, index: usize) -> Ptr;
}
impl v8_base_Vector_v8_internal_wasm_WellKnownImport_Impl
    for Ptr<v8_base_Vector_v8_internal_wasm_WellKnownImport_>
{
    fn size(&self) -> usize {
        return (*(*(*self).upgrade().deref()).length_.borrow());
    }
    fn operator_index(&self, index: usize) -> Ptr {
        let index: Value<usize> = Rc::new(RefCell::new(index));
        (&(0));
        return (*(*(*self).upgrade().deref()).start_.borrow()).offset((*index.borrow()) as isize);
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
        return ({ hash_value_90((v.read())) });
    }
}
pub trait v8_base_hash_unsigned_long_long_Impl {
    fn operator_call(&self, v: Ptr<u64>) -> usize;
}
impl v8_base_hash_unsigned_long_long_Impl for Ptr<v8_base_hash_unsigned_long_long_> {
    fn operator_call(&self, v: Ptr<u64>) -> usize {
        return ({ hash_value_91((v.read())) });
    }
}
pub trait v8_internal_wasm_WellKnownImportsListImpl {
    fn Initialize_i32(&self, size: i32);
    fn Initialize_v8_base_Vector_const_v8_internal_wasm_WellKnownImport(
        &self,
        entries: v8_base_Vector_const_v8_internal_wasm_WellKnownImport_,
    );
    fn get(&self, index: i32) -> v8_internal_wasm_WellKnownImport;
    fn Update(
        &self,
        entries: v8_base_Vector_v8_internal_wasm_WellKnownImport_,
    ) -> v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_;
    fn operator_assign_pmutv8_internal_wasm_WellKnownImportsList(
        &self,
        _a0: Ptr<v8_internal_wasm_WellKnownImportsList>,
    ) -> Ptr<v8_internal_wasm_WellKnownImportsList>;
}
impl v8_internal_wasm_WellKnownImportsListImpl for Ptr<v8_internal_wasm_WellKnownImportsList> {
    fn Initialize_i32(&self, size: i32) {
        let size: Value<i32> = Rc::new(RefCell::new(size));
        ((*(*self).upgrade().deref()).statuses_.as_pointer()
            as Ptr<Option<Value<Box<[std_atomic_v8_internal_wasm_WellKnownImport_]>>>>)
            .write(
                Some(Rc::new(RefCell::new(
                    (0..((*size.borrow()) as usize))
                        .map(|_| <std_atomic_v8_internal_wasm_WellKnownImport_>::default())
                        .collect::<Box<[_]>>(),
                )))
                .take(),
            );
    }
    fn get(&self, index: i32) -> v8_internal_wasm_WellKnownImport {
        let index: Value<i32> = Rc::new(RefCell::new(index));
        (&(0));
        return ({
            (*(*(*self).upgrade().deref()).statuses_.borrow())
                .as_ref()
                .unwrap()
                .borrow()[((*index.borrow()) as usize) as usize]
                .load_const(Some(0))
        });
    }
    fn operator_assign_pmutv8_internal_wasm_WellKnownImportsList(
        &self,
        _a0: Ptr<v8_internal_wasm_WellKnownImportsList>,
    ) -> Ptr<v8_internal_wasm_WellKnownImportsList> {
        ((*(*self).upgrade().deref()).statuses_.as_pointer()
            as Ptr<Option<Value<Box<[std_atomic_v8_internal_wasm_WellKnownImport_]>>>>)
            .write((*(*_a0.upgrade().deref()).statuses_.borrow_mut()).take());
        return (*self).clone();
    }
    fn Update(
        &self,
        entries: v8_base_Vector_v8_internal_wasm_WellKnownImport_,
    ) -> v8_base_StrongAlias_v8_internal_wasm_UpdateResultTag__bool_ {
        let entries: Value<v8_base_Vector_v8_internal_wasm_WellKnownImport_> =
            Rc::new(RefCell::new(entries));
        (&(0));
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow())
            < ({
                v8_base_Vector_v8_internal_wasm_WellKnownImport_Impl::size(&entries.as_pointer())
            }))
        {
            let entry: Value<v8_internal_wasm_WellKnownImport> = Rc::new(RefCell::new(
                (({
                    v8_base_Vector_v8_internal_wasm_WellKnownImport_Impl::operator_index(
                        &entries.as_pointer(),
                        (*i.borrow()),
                    )
                })
                .read()),
            ));
            (&(0));
            let old: Value<v8_internal_wasm_WellKnownImport> = Rc::new(RefCell::new(
                ({
                    (*(*(*self).upgrade().deref()).statuses_.borrow())
                        .as_ref()
                        .unwrap()
                        .borrow()[(*i.borrow()) as usize]
                        .load_const(Some(0))
                }),
            ));
            if ((*old.borrow()) == v8_internal_wasm_WellKnownImport_kGeneric) {
                (*i.borrow_mut()).postfix_inc();
                continue 'loop_;
            }
            if ((*old.borrow()) == (*entry.borrow())) {
                (*i.borrow_mut()).postfix_inc();
                continue 'loop_;
            }
            if ((*old.borrow()) == v8_internal_wasm_WellKnownImport_kUninstantiated) {
                ({
                    (*(*(*self).upgrade().deref()).statuses_.borrow())
                        .as_ref()
                        .unwrap()
                        .borrow()[(*i.borrow()) as usize]
                        .store_v8_internal_wasm_WellKnownImport((*entry.borrow()), Some(0))
                });
            } else {
                let j: Value<usize> = Rc::new(RefCell::new(0_usize));
                'loop_: while ((*j.borrow())
                    < ({
                        v8_base_Vector_v8_internal_wasm_WellKnownImport_Impl::size(
                            &entries.as_pointer(),
                        )
                    }))
                {
                    if ({
                        IsCompileTimeImport_114(
                            ({
                                (*(*(*self).upgrade().deref()).statuses_.borrow())
                                    .as_ref()
                                    .unwrap()
                                    .borrow()[(*j.borrow()) as usize]
                                    .load_const(Some(0))
                            }),
                        )
                    }) {
                        (&(0));
                        (*j.borrow_mut()).postfix_inc();
                        continue 'loop_;
                    }
                    ({
                        (*(*(*self).upgrade().deref()).statuses_.borrow())
                            .as_ref()
                            .unwrap()
                            .borrow()[(*j.borrow()) as usize]
                            .store_v8_internal_wasm_WellKnownImport(
                                v8_internal_wasm_WellKnownImport_kGeneric,
                                Some(0),
                            )
                    });
                    (*j.borrow_mut()).postfix_inc();
                }
                return (*kFoundIncompatibility_115.with(Value::clone).borrow()).clone();
            }
            (*i.borrow_mut()).postfix_inc();
        }
        return (*kOK_116.with(Value::clone).borrow()).clone();
    }
    fn Initialize_v8_base_Vector_const_v8_internal_wasm_WellKnownImport(
        &self,
        entries: v8_base_Vector_const_v8_internal_wasm_WellKnownImport_,
    ) {
        let entries: Value<v8_base_Vector_const_v8_internal_wasm_WellKnownImport_> =
            Rc::new(RefCell::new(entries));
        (&(0));
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow())
            < ({
                v8_base_Vector_const_v8_internal_wasm_WellKnownImport_Impl::size(
                    &entries.as_pointer(),
                )
            }))
        {
            (&(0));
            ({
                let ___d: v8_internal_wasm_WellKnownImport = (({
                    v8_base_Vector_const_v8_internal_wasm_WellKnownImport_Impl::operator_index(
                        &entries.as_pointer(),
                        (*i.borrow()),
                    )
                })
                .read());
                (*(*(*self).upgrade().deref()).statuses_.borrow())
                    .as_ref()
                    .unwrap()
                    .borrow()[(*i.borrow()) as usize]
                    .store_v8_internal_wasm_WellKnownImport(___d, Some(0))
            });
            (*i.borrow_mut()).postfix_inc();
        }
    }
}
