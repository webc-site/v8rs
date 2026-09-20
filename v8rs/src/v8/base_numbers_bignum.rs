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
pub type v8_base_comparison_underlying_type_unsigned_long__Dummy = u32;
thread_local!(
    pub static is_enum_12: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_17: Value<bool> = Rc::new(RefCell::new(true));
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
    pub static value_18: Value<bool> = Rc::new(RefCell::new(false));
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
pub fn CmpLTImpl_19(lhs: i32, rhs: u64) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return ((*lhs.borrow()) < 0) || ((((*lhs.borrow()) as u32) as u64) < ((*rhs.borrow()) as u64));
}
pub fn CmpGEImpl_20(lhs: i32, rhs: u64) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return !({ CmpLTImpl_19((*lhs.borrow()), (*rhs.borrow())) });
}
pub fn bit_cast_21(source: Ptr<i8>) -> u8 {
    return ({ bit_cast_22((source).clone()) });
}
pub fn bit_cast_23(source: Ptr<i16>) -> u16 {
    return ({ bit_cast_24((source).clone()) });
}
pub fn bit_cast_25(source: Ptr<i32>) -> u32 {
    return ({ bit_cast_26((source).clone()) });
}
pub fn bit_cast_27(source: Ptr<i64>) -> u64 {
    return ({ bit_cast_28((source).clone()) });
}
pub fn bit_cast_29(source: Ptr<i64>) -> u64 {
    return ({ bit_cast_30((source).clone()) });
}
pub fn bit_cast_31(source: Ptr<f32>) -> u32 {
    return ({ bit_cast_32((source).clone()) });
}
pub fn bit_cast_33(source: Ptr<f64>) -> u64 {
    return ({ bit_cast_34((source).clone()) });
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
pub fn make_uint64_35(high: u32, low: u32) -> u64 {
    let high: Value<u32> = Rc::new(RefCell::new(high));
    let low: Value<u32> = Rc::new(RefCell::new(low));
    return (((*high.borrow()) as u64) << 32).wrapping_add(((*low.borrow()) as u64));
}
pub fn RoundDown_36(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    return ((*x.borrow()) & (-(*m.borrow()) as u64));
}
pub fn RoundUp_37(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    (&(0));
    return ({
        let _x: u64 = ((*x.borrow()).wrapping_add((((*m.borrow()) - 1_i64) as u64)) as u64);
        let _m: i64 = (*m.borrow());
        RoundDown_36(_x, _m)
    });
}
pub fn IsAligned_38(value: u64, alignment: u64) -> bool {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let alignment: Value<u64> = Rc::new(RefCell::new(alignment));
    return (((*value.borrow()) & ((*alignment.borrow()).wrapping_sub(1_u64))) == 0_u64);
}
pub fn CountLeadingZeros_39(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        if (32_u32 == 64_u32) {
            (({ __builtin_clzll_40(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((32_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros_41(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        if (64_u32 == 64_u32) {
            (({ __builtin_clzll_40(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((64_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros_42(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        if (64_u32 == 64_u32) {
            (({ __builtin_clzll_40((*value.borrow())) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((64_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros32_43(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_39((*value.borrow())) });
}
pub fn CountLeadingZeros64_44(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_41((*value.borrow())) });
}
pub fn CountTrailingZeros_45(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        (if (32_u32 == 64_u32) {
            ({ __builtin_ctzll_46(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros_47(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        (if (64_u32 == 64_u32) {
            ({ __builtin_ctzll_46(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros32_48(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_45((*value.borrow())) });
}
pub fn CountTrailingZeros64_49(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_47((*value.borrow())) });
}
pub fn RoundUpToPowerOfTwo32_50(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u32 << ((32_u32).wrapping_sub(({ CountLeadingZeros_39((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo64_51(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u64 << ((64_u32).wrapping_sub(({ CountLeadingZeros_41((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo_52(value: usize) -> usize {
    let value: Value<usize> = Rc::new(RefCell::new(value));
    if (::std::mem::size_of::<usize>() == ::std::mem::size_of::<u64>()) {
        return (({ RoundUpToPowerOfTwo64_51(((*value.borrow()) as u64)) }) as usize);
    } else {
        return (({ RoundUpToPowerOfTwo32_50(((*value.borrow()) as u32)) }) as usize);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn RoundDownToPowerOfTwo32_53(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    if ((*value.borrow()) > 2147483648_u32) {
        return 2147483648_u32;
    }
    let result: Value<u32> = Rc::new(RefCell::new(
        ({ RoundUpToPowerOfTwo32_50((*value.borrow())) }),
    ));
    if ((*result.borrow()) > (*value.borrow())) {
        (*result.borrow_mut()) >>= 1;
    }
    return (*result.borrow());
}
pub fn RotateRight32_54(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateLeft32_55(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateRight64_56(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn RotateLeft64_57(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn ClearLsb_58(value: i32) -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(value));
    return ((*value.borrow()) & ((*value.borrow()) - 1));
}
pub fn SignedAddOverflow32_59(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_sadd_overflow_60(_arg0, _arg1, _arg2)
    });
}
pub fn SignedSubOverflow32_61(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_ssub_overflow_62(_arg0, _arg1, _arg2)
    });
}
pub fn SignedMulOverflow32_63(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_smul_overflow_64(_arg0, _arg1, _arg2)
    });
}
pub fn SignedAddOverflow64_65(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_add_overflow_66(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedSubOverflow64_67(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_sub_overflow_68(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedMulOverflow64_69(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return {
        let (val, ovf) = (*lhs.borrow()).overflowing_mul((*rhs.borrow()));
        (*val.borrow()).write(val);
        ovf
    };
}
pub fn UnsignedAddOverflow32_70(lhs: u32, rhs: u32, val: Ptr<u32>) -> bool {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<u32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: u32 = (*lhs.borrow());
        let _arg1: u32 = (*rhs.borrow());
        let _arg2: Ptr<u32> = (*val.borrow()).clone();
        __builtin_uadd_overflow_71(_arg0, _arg1, _arg2)
    });
}
pub fn UnsignedDiv32_72(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedDiv64_73(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn UnsignedMod32_74(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedMod64_75(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn WraparoundAdd32_76(lhs: i32, rhs: i32) -> i32 {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as u32).wrapping_add(((*rhs.borrow()) as u32))) as i32);
}
pub fn WraparoundNeg32_77(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (-((*x.borrow()) as u32) as i32);
}
pub fn ByteReverse16_78(value: u16) -> u16 {
    let value: Value<u16> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse32_79(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse64_80(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn OverlappingWrites_81(dst: AnyPtr, src: AnyPtr, count: usize) {
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
pub fn OverlappingWrites_82(dst: AnyPtr, src: AnyPtr, count: usize) {
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
pub fn OverlappingWrites_83(dst: AnyPtr, src: AnyPtr, count: usize) {
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
pub fn SimdMemCopy_84(dst: AnyPtr, src: AnyPtr, count: usize) {
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
            (({ CountLeadingZeros_42(((*count.borrow()).wrapping_sub(1_usize) as u64)) }) as usize),
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
                    OverlappingWrites_81(
                        ((*dst_u.borrow()).clone() as Ptr<u8>).to_any(),
                        ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                        (*count.borrow()),
                    )
                });
                return;
            }
            __v if __v == 3 => {
                ({
                    OverlappingWrites_82(
                        ((*dst_u.borrow()).clone() as Ptr<u8>).to_any(),
                        ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                        (*count.borrow()),
                    )
                });
                return;
            }
            __v if __v == 4 => {
                ({
                    OverlappingWrites_83(
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
                            __builtin_neon_vld1q_v_85(
                                ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                                48,
                            )
                        });
                        let __result = (*__ret.borrow());
                        __result
                    }));
                    let __result = ({
                        __builtin_neon_vst1q_v_86(
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
                            __builtin_neon_vld1q_v_85(
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
                        __builtin_neon_vst1q_v_86(
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
                            __builtin_neon_vld1q_v_85(
                                ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                                48,
                            )
                        });
                        let __result = (*__ret.borrow());
                        __result
                    }));
                    let __result = ({
                        __builtin_neon_vst1q_v_86(
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
                                __builtin_neon_vld1q_v_85(
                                    ((*src_u.borrow()).offset((*i.borrow()) as isize) as Ptr<u8>)
                                        .to_any(),
                                    48,
                                )
                            });
                            let __result = (*__ret.borrow());
                            __result
                        }));
                        let __result = ({
                            __builtin_neon_vst1q_v_86(
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
pub fn MemCopy_87(dest: AnyPtr, src: AnyPtr, size: usize) {
    let dest: Value<AnyPtr> = Rc::new(RefCell::new(dest));
    let src: Value<AnyPtr> = Rc::new(RefCell::new(src));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    ({
        SimdMemCopy_84(
            (*dest.borrow()).clone(),
            (*src.borrow()).clone(),
            (*size.borrow()),
        )
    });
}
pub fn MemMove_88(dest: AnyPtr, src: AnyPtr, size: usize) {
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
    pub static kCompressionFactor_89: Value<u32> = Rc::new(RefCell::new(1));
);
thread_local!(
    pub static kExpansionFactor_90: Value<u32> = Rc::new(RefCell::new(1));
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
pub fn rapid_mul128_91(A: u64, B: u64) -> (Value<u64>, Value<u64>) {
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
pub fn rapid_mix_92(A: u64, B: u64) -> u64 {
    let A: Value<u64> = Rc::new(RefCell::new(A));
    let B: Value<u64> = Rc::new(RefCell::new(B));
    let __rhs = ({ rapid_mul128_91((*A.borrow()), (*B.borrow())) });
    ({ tie_93(A.as_pointer(), B.as_pointer()) }) = __rhs;
    return ((*A.borrow()) ^ (*B.borrow()));
}
thread_local!(
    pub static RAPIDHASH_DEFAULT_SECRET_94: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
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
pub fn hash_combine_97(seed: usize, hash: usize) -> usize {
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
    pub static kRapidhashSecret1_98: Value<u64> = Rc::new(RefCell::new(3257665815644502181));
);
thread_local!(
    pub static kRapidhashSecret2_99: Value<u64> = Rc::new(RefCell::new(10067880064238660809));
);
pub fn hash64_100(key: u64) -> u64 {
    let key: Value<u64> = Rc::new(RefCell::new(key));
    return ({
        let _A: u64 = ((*key.borrow()) ^ 3257665815644502181);
        let _B: u64 = ((*key.borrow()) ^ 10067880064238660809);
        rapid_mix_92(_A, _B)
    });
}
pub fn hash32_101(key: u32) -> u32 {
    let key: Value<u32> = Rc::new(RefCell::new(key));
    return (({ hash64_100(((*key.borrow()) as u64)) }) as u32);
}
pub fn hash_value_102(v: bool) -> usize {
    let v: Value<bool> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) as usize);
}
pub fn hash_value_103(v: u8) -> usize {
    let v: Value<u8> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) as usize);
}
pub fn hash_value_104(v: u16) -> usize {
    let v: Value<u16> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) as usize);
}
pub fn hash_value_95(v: u32) -> usize {
    let v: Value<u32> = Rc::new(RefCell::new(v));
    return (({ hash32_101((*v.borrow())) }) as usize);
}
pub fn hash_value_105(v: u64) -> usize {
    let v: Value<u64> = Rc::new(RefCell::new(v));
    if false {
        return (({ hash32_101(((*v.borrow()) as u32)) }) as usize);
    } else {
        return (({ hash64_100((*v.borrow())) }) as usize);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn hash_value_96(v: u64) -> usize {
    let v: Value<u64> = Rc::new(RefCell::new(v));
    return (({ hash64_100((*v.borrow())) }) as usize);
}
pub fn hash_value_106(v: i8) -> usize {
    let v: Value<i8> = Rc::new(RefCell::new(v));
    return ({ hash_value_103(({ bit_cast_21(v.as_pointer()) })) });
}
pub fn hash_value_107(v: i16) -> usize {
    let v: Value<i16> = Rc::new(RefCell::new(v));
    return ({ hash_value_104(({ bit_cast_23(v.as_pointer()) })) });
}
pub fn hash_value_108(v: i32) -> usize {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    return ({ hash_value_95(({ bit_cast_25(v.as_pointer()) })) });
}
pub fn hash_value_109(v: i64) -> usize {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    return ({ hash_value_105(({ bit_cast_27(v.as_pointer()) })) });
}
pub fn hash_value_110(v: i64) -> usize {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    return ({ hash_value_96(({ bit_cast_29(v.as_pointer()) })) });
}
pub fn hash_value_111(v: f32) -> usize {
    let v: Value<f32> = Rc::new(RefCell::new(v));
    return if ((*v.borrow()) != 0.0E+0) {
        ({ hash_value_95(({ bit_cast_31(v.as_pointer()) })) })
    } else {
        0_usize
    };
}
pub fn hash_value_112(v: f64) -> usize {
    let v: Value<f64> = Rc::new(RefCell::new(v));
    return if ((*v.borrow()) != 0.0E+0) {
        ({ hash_value_96(({ bit_cast_33(v.as_pointer()) })) })
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
pub struct v8_base_Vector_unsigned_int_ {
    start_: Value<Ptr<u32>>,
    length_: Value<usize>,
}
impl v8_base_Vector_unsigned_int_ {
    pub fn v8_base_Vector_unsigned_int_5(data: Ptr<u32>, length: usize) -> Self {
        let data: Value<Ptr<u32>> = Rc::new(RefCell::new(data));
        let length: Value<usize> = Rc::new(RefCell::new(length));
        let __this: Value<v8_base_Vector_unsigned_int_> = Rc::new(RefCell::new(Self {
            start_: Rc::new(RefCell::new((*data.borrow()).clone())),
            length_: Rc::new(RefCell::new((*length.borrow()))),
        }));
        let this: Ptr<v8_base_Vector_unsigned_int_> = __this.as_pointer();
        (&(0));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_Vector_unsigned_int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Vector_unsigned_int_> = Rc::new(RefCell::new(Self {
            start_: Rc::new(RefCell::new((*self.start_.borrow()).clone())),
            length_: Rc::new(RefCell::new((*self.length_.borrow()))),
        }));
        let this: Ptr<v8_base_Vector_unsigned_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_Vector_unsigned_int_ {
    fn default() -> Self {
        { v8_base_Vector_unsigned_int_::v8_base_Vector_unsigned_int_6() }
    }
}
impl ByteRepr for v8_base_Vector_unsigned_int_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.start_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.length_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            start_: Rc::new(RefCell::new(<Ptr<u32>>::from_bytes(&buf[0..8]))),
            length_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
        }
    }
}
thread_local!(
    pub static enable_view_114: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static enable_borrowed_range_115: Value<bool> = Rc::new(RefCell::new(true));
);
pub fn CStrVector_116(data: Ptr<u8>) -> v8_base_Vector_const_char_ {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    return v8_base_Vector_const_char_::v8_base_Vector_const_char_1(
        { (*data.borrow()).clone() },
        { (*data.borrow()).to_c_string_iterator().count() },
    );
}
pub fn StrVector_117(str: Vec<u8>) -> v8_base_Vector_const_char_ {
    let str: Value<Vec<u8>> = Rc::new(RefCell::new(str));
    return v8_base_Vector_const_char_::v8_base_Vector_const_char_1(
        { ({ (*str.borrow()).data() }) },
        { (*str.borrow()).len() },
    );
}
pub fn OneByteVector_118(data: Ptr<u8>, length: usize) -> v8_base_Vector_const_unsigned_char_ {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    return v8_base_Vector_const_unsigned_char_::v8_base_Vector_const_unsigned_char_3(
        { (*data.borrow()).reinterpret_cast::<u8>() },
        { (*length.borrow()) },
    );
}
pub fn OneByteVector_119(data: Ptr<u8>) -> v8_base_Vector_const_unsigned_char_ {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    return ({
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _length: usize = (*data.borrow()).to_c_string_iterator().count();
        OneByteVector_118(_data, _length)
    });
}
thread_local!(
    pub static kMaxSignificantBits_120: Value<i32> = Rc::new(RefCell::new(3584));
);
thread_local!(
    static kChunkSize_121: Value<i32> = Rc::new(RefCell::new(
        (((::std::mem::size_of::<u32>() as usize).wrapping_mul(8_usize)) as i32),
    ));
);
thread_local!(
    static kDoubleChunkSize_122: Value<i32> = Rc::new(RefCell::new(
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)) as i32),
    ));
);
thread_local!(
    static kBigitSize_123: Value<i32> = Rc::new(RefCell::new(28));
);
thread_local!(
    static kBigitMask_124: Value<u32> = Rc::new(RefCell::new(
        (((1 << (*kBigitSize_123.with(Value::clone).borrow())) - 1) as u32),
    ));
);
thread_local!(
    static kBigitCapacity_125: Value<i32> = Rc::new(RefCell::new(
        ((*kMaxSignificantBits_120.with(Value::clone).borrow())
            / (*kBigitSize_123.with(Value::clone).borrow())),
    ));
);
#[derive()]
pub struct v8_base_Bignum {
    bigits_buffer_: Value<Box<[u32]>>,
    bigits_: Value<v8_base_Vector_unsigned_int_>,
    used_digits_: Value<i32>,
    exponent_: Value<i32>,
}
impl v8_base_Bignum {
    pub fn v8_base_Bignum() -> Self {
        let __this: Value<v8_base_Bignum> = Rc::new(RefCell::new(Self {
            bigits_buffer_: Rc::new(RefCell::new(
                (0..128).map(|_| <u32>::default()).collect::<Box<[u32]>>(),
            )),
            bigits_: Rc::new(RefCell::new(
                v8_base_Vector_unsigned_int_::v8_base_Vector_unsigned_int_5(
                    { ((*this.upgrade().deref()).bigits_buffer_.as_pointer() as Ptr<u32>) },
                    { ((*kBigitCapacity_125.with(Value::clone).borrow()) as usize) },
                ),
            )),
            used_digits_: Rc::new(RefCell::new(0)),
            exponent_: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<v8_base_Bignum> = __this.as_pointer();
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*kBigitCapacity_125.with(Value::clone).borrow())) {
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*this.upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .write(0_u32);
            (*i.borrow_mut()).prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Equal(a: Ptr<v8_base_Bignum>, b: Ptr<v8_base_Bignum>) -> bool {
        return (({
            let _a: Ptr<v8_base_Bignum> = (a).clone();
            let _b: Ptr<v8_base_Bignum> = (b).clone();
            v8_base_Bignum::Compare(_a, _b)
        }) == 0);
    }
    pub fn LessEqual(a: Ptr<v8_base_Bignum>, b: Ptr<v8_base_Bignum>) -> bool {
        return (({
            let _a: Ptr<v8_base_Bignum> = (a).clone();
            let _b: Ptr<v8_base_Bignum> = (b).clone();
            v8_base_Bignum::Compare(_a, _b)
        }) <= 0);
    }
    pub fn Less(a: Ptr<v8_base_Bignum>, b: Ptr<v8_base_Bignum>) -> bool {
        return (({
            let _a: Ptr<v8_base_Bignum> = (a).clone();
            let _b: Ptr<v8_base_Bignum> = (b).clone();
            v8_base_Bignum::Compare(_a, _b)
        }) < 0);
    }
    pub fn PlusEqual(
        a: Ptr<v8_base_Bignum>,
        b: Ptr<v8_base_Bignum>,
        c: Ptr<v8_base_Bignum>,
    ) -> bool {
        return (({
            let _a: Ptr<v8_base_Bignum> = (a).clone();
            let _b: Ptr<v8_base_Bignum> = (b).clone();
            let _c: Ptr<v8_base_Bignum> = (c).clone();
            v8_base_Bignum::PlusCompare(_a, _b, _c)
        }) == 0);
    }
    pub fn PlusLessEqual(
        a: Ptr<v8_base_Bignum>,
        b: Ptr<v8_base_Bignum>,
        c: Ptr<v8_base_Bignum>,
    ) -> bool {
        return (({
            let _a: Ptr<v8_base_Bignum> = (a).clone();
            let _b: Ptr<v8_base_Bignum> = (b).clone();
            let _c: Ptr<v8_base_Bignum> = (c).clone();
            v8_base_Bignum::PlusCompare(_a, _b, _c)
        }) <= 0);
    }
    pub fn PlusLess(
        a: Ptr<v8_base_Bignum>,
        b: Ptr<v8_base_Bignum>,
        c: Ptr<v8_base_Bignum>,
    ) -> bool {
        return (({
            let _a: Ptr<v8_base_Bignum> = (a).clone();
            let _b: Ptr<v8_base_Bignum> = (b).clone();
            let _c: Ptr<v8_base_Bignum> = (c).clone();
            v8_base_Bignum::PlusCompare(_a, _b, _c)
        }) < 0);
    }
}
impl Default for v8_base_Bignum {
    fn default() -> Self {
        { v8_base_Bignum::v8_base_Bignum() }
    }
}
impl ByteRepr for v8_base_Bignum {
    fn byte_size() -> usize {
        536
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.bigits_buffer_.borrow()).to_bytes(&mut buf[0..512]);
        (*self.bigits_.borrow()).to_bytes(&mut buf[512..528]);
        (*self.used_digits_.borrow()).to_bytes(&mut buf[528..532]);
        (*self.exponent_.borrow()).to_bytes(&mut buf[532..536]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            bigits_buffer_: Rc::new(RefCell::new(<Box<[u32]>>::from_bytes(&buf[0..512]))),
            bigits_: Rc::new(RefCell::new(<v8_base_Vector_unsigned_int_>::from_bytes(
                &buf[512..528],
            ))),
            used_digits_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[528..532]))),
            exponent_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[532..536]))),
        }
    }
}
thread_local!(
    pub static kUC16Size_126: Value<i32> = Rc::new(RefCell::new(2));
);
pub fn HexValue_127(c: u32) -> i32 {
    let c: Value<u32> = Rc::new(RefCell::new(c));
    {
        let rhs_0 = (*c.borrow()).wrapping_sub((('0' as u8) as u32));
        (*c.borrow_mut()) = rhs_0
    };
    if (((*c.borrow()) as u32) <= 9_u32) {
        return ((*c.borrow()) as i32);
    }
    let __rhs = ((*c.borrow()) | 32_u32)
        .wrapping_sub((((('a' as u8) as i32) - (('0' as u8) as i32)) as u32));
    (*c.borrow_mut()) = __rhs;
    if (((*c.borrow()) as u32) <= 5_u32) {
        return (((*c.borrow()).wrapping_add(10_u32)) as i32);
    }
    return -1_i32;
}
pub fn HexCharOfValue_128(value: i32) -> u8 {
    let value: Value<i32> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) < 10) {
        return (((*value.borrow()) + (('0' as u8) as i32)) as u8);
    }
    return ((((*value.borrow()) - 10) + (('A' as u8) as i32)) as u8);
}
pub fn ReadUInt64_129(buffer: v8_base_Vector_const_char_, from: i32, digits_to_read: i32) -> u64 {
    let buffer: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(buffer));
    let from: Value<i32> = Rc::new(RefCell::new(from));
    let digits_to_read: Value<i32> = Rc::new(RefCell::new(digits_to_read));
    let result: Value<u64> = Rc::new(RefCell::new(0_u64));
    let to: Value<i32> = Rc::new(RefCell::new(
        ((*from.borrow()) + (*digits_to_read.borrow())),
    ));
    let i: Value<i32> = Rc::new(RefCell::new((*from.borrow())));
    'loop_: while ((*i.borrow()) < (*to.borrow())) {
        let digit: Value<i32> = Rc::new(RefCell::new(
            (((({
                v8_base_Vector_const_char_Impl::operator_index(
                    &buffer.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .read()) as i32)
                - (('0' as u8) as i32)),
        ));
        (&(0));
        let __rhs =
            ((*result.borrow()).wrapping_mul(10_u64)).wrapping_add(((*digit.borrow()) as u64));
        (*result.borrow_mut()) = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    return (*result.borrow());
}
pub fn HexCharValue_130(c: u8) -> i32 {
    let c: Value<u8> = Rc::new(RefCell::new(c));
    if ((('0' as u8) as i32) <= ((*c.borrow()) as i32))
        && (((*c.borrow()) as i32) <= (('9' as u8) as i32))
    {
        return (((*c.borrow()) as i32) - (('0' as u8) as i32));
    }
    if ((('a' as u8) as i32) <= ((*c.borrow()) as i32))
        && (((*c.borrow()) as i32) <= (('f' as u8) as i32))
    {
        return ((10 + ((*c.borrow()) as i32)) - (('a' as u8) as i32));
    }
    if ((('A' as u8) as i32) <= ((*c.borrow()) as i32))
        && (((*c.borrow()) as i32) <= (('F' as u8) as i32))
    {
        return ((10 + ((*c.borrow()) as i32)) - (('A' as u8) as i32));
    }
    ({
        V8_Fatal_113(
            (*kUnreachableCodeMessage_9.with(Value::clone).borrow()).clone(),
            &[],
        )
    });
    panic!("ub: non-void function does not return a value")
}
pub fn SizeInHexChars_132(number: u32) -> i32 {
    let number: Value<u32> = Rc::new(RefCell::new(number));
    (&(0));
    let result: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*number.borrow()) != 0_u32) {
        (*number.borrow_mut()) >>= 4;
        (*result.borrow_mut()).postfix_inc();
    }
    return (*result.borrow());
}
impl v8_base_Bignum {
    pub fn Compare(a: Ptr<v8_base_Bignum>, b: Ptr<v8_base_Bignum>) -> i32 {
        (&(0));
        (&(0));
        let bigit_length_a: Value<i32> =
            Rc::new(RefCell::new(({ v8_base_BignumImpl::BigitLength(&a) })));
        let bigit_length_b: Value<i32> =
            Rc::new(RefCell::new(({ v8_base_BignumImpl::BigitLength(&b) })));
        if ((*bigit_length_a.borrow()) < (*bigit_length_b.borrow())) {
            return -1_i32;
        }
        if ((*bigit_length_a.borrow()) > (*bigit_length_b.borrow())) {
            return + 1;
        }
        let i: Value<i32> = Rc::new(RefCell::new(((*bigit_length_a.borrow()) - 1)));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs >= (if (*a.upgrade().deref()).exponent_.as_pointer().read()
                <= (*b.upgrade().deref()).exponent_.as_pointer().read()
            {
                (*a.upgrade().deref()).exponent_.as_pointer()
            } else {
                (*b.upgrade().deref()).exponent_.as_pointer()
            }
            .read())
        } {
            let bigit_a: Value<u32> = Rc::new(RefCell::new(
                ({
                    let _index: i32 = (*i.borrow());
                    v8_base_BignumImpl::BigitAt(&a, _index)
                }),
            ));
            let bigit_b: Value<u32> = Rc::new(RefCell::new(
                ({
                    let _index: i32 = (*i.borrow());
                    v8_base_BignumImpl::BigitAt(&b, _index)
                }),
            ));
            if ((*bigit_a.borrow()) < (*bigit_b.borrow())) {
                return -1_i32;
            }
            if ((*bigit_a.borrow()) > (*bigit_b.borrow())) {
                return + 1;
            }
            (*i.borrow_mut()).prefix_dec();
        }
        return 0;
    }
}
impl v8_base_Bignum {
    pub fn PlusCompare(
        a: Ptr<v8_base_Bignum>,
        b: Ptr<v8_base_Bignum>,
        c: Ptr<v8_base_Bignum>,
    ) -> i32 {
        (&(0));
        (&(0));
        (&(0));
        if {
            let _lhs = ({ v8_base_BignumImpl::BigitLength(&a) });
            _lhs < ({ v8_base_BignumImpl::BigitLength(&b) })
        } {
            return ({
                let _a: Ptr<v8_base_Bignum> = (b).clone();
                let _b: Ptr<v8_base_Bignum> = (a).clone();
                let _c: Ptr<v8_base_Bignum> = (c).clone();
                v8_base_Bignum::PlusCompare(_a, _b, _c)
            });
        }
        if {
            let _lhs = (({ v8_base_BignumImpl::BigitLength(&a) }) + 1);
            _lhs < ({ v8_base_BignumImpl::BigitLength(&c) })
        } {
            return -1_i32;
        }
        if {
            let _lhs = ({ v8_base_BignumImpl::BigitLength(&a) });
            _lhs > ({ v8_base_BignumImpl::BigitLength(&c) })
        } {
            return + 1;
        }
        if ({
            let _lhs = (*(*a.upgrade().deref()).exponent_.borrow());
            _lhs >= ({ v8_base_BignumImpl::BigitLength(&b) })
        }) && ({
            let _lhs = ({ v8_base_BignumImpl::BigitLength(&a) });
            _lhs < ({ v8_base_BignumImpl::BigitLength(&c) })
        }) {
            return -1_i32;
        }
        let borrow: Value<u32> = Rc::new(RefCell::new(0_u32));
        let min_exponent: Value<i32> = Rc::new(RefCell::new(
            ({
                min_133(vec![
                    (*(*a.upgrade().deref()).exponent_.borrow()),
                    (*(*b.upgrade().deref()).exponent_.borrow()),
                    (*(*c.upgrade().deref()).exponent_.borrow()),
                ])
            }),
        ));
        let i: Value<i32> = Rc::new(RefCell::new(
            (({ v8_base_BignumImpl::BigitLength(&c) }) - 1),
        ));
        'loop_: while ((*i.borrow()) >= (*min_exponent.borrow())) {
            let chunk_a: Value<u32> = Rc::new(RefCell::new(
                ({
                    let _index: i32 = (*i.borrow());
                    v8_base_BignumImpl::BigitAt(&a, _index)
                }),
            ));
            let chunk_b: Value<u32> = Rc::new(RefCell::new(
                ({
                    let _index: i32 = (*i.borrow());
                    v8_base_BignumImpl::BigitAt(&b, _index)
                }),
            ));
            let chunk_c: Value<u32> = Rc::new(RefCell::new(
                ({
                    let _index: i32 = (*i.borrow());
                    v8_base_BignumImpl::BigitAt(&c, _index)
                }),
            ));
            let sum: Value<u32> = Rc::new(RefCell::new(
                (*chunk_a.borrow()).wrapping_add((*chunk_b.borrow())),
            ));
            if ((*sum.borrow()) > (*chunk_c.borrow()).wrapping_add((*borrow.borrow()))) {
                return + 1;
            } else {
                let __rhs = ((*chunk_c.borrow()).wrapping_add((*borrow.borrow())))
                    .wrapping_sub((*sum.borrow()));
                (*borrow.borrow_mut()) = __rhs;
                if ((*borrow.borrow()) > 1_u32) {
                    return -1_i32;
                }
                (*borrow.borrow_mut()) <<= (*kBigitSize_123.with(Value::clone).borrow());
            }
            (*i.borrow_mut()).prefix_dec();
        }
        if ((*borrow.borrow()) == 0_u32) {
            return 0;
        }
        return -1_i32;
    }
}
pub trait v8_base_BignumImpl {
    fn AssignUInt16(&self, value: u16);
    fn AssignUInt64(&self, value: u64);
    fn AssignBignum(&self, other: Ptr<v8_base_Bignum>);
    fn AssignDecimalString(&self, value: v8_base_Vector_const_char_);
    fn AssignHexString(&self, value: v8_base_Vector_const_char_);
    fn AssignPowerUInt16(&self, base: u16, power_exponent: i32);
    fn AddUInt64(&self, operand: u64);
    fn AddBignum(&self, other: Ptr<v8_base_Bignum>);
    fn SubtractBignum(&self, other: Ptr<v8_base_Bignum>);
    fn Square(&self);
    fn ShiftLeft(&self, shift_amount: i32);
    fn MultiplyByUInt32(&self, factor: u32);
    fn MultiplyByUInt64(&self, factor: u64);
    fn MultiplyByPowerOfTen(&self, exponent: i32);
    fn Times10(&self);
    fn DivideModuloIntBignum(&self, other: Ptr<v8_base_Bignum>) -> u16;
    fn ToHexString(&self, buffer: Ptr<u8>, buffer_size: i32) -> bool;
    fn EnsureCapacity(&self, size: i32);
    fn Align(&self, other: Ptr<v8_base_Bignum>);
    fn Clamp(&self);
    fn IsClamped(&self) -> bool;
    fn Zero(&self);
    fn BigitsShiftLeft(&self, shift_amount: i32);
    fn BigitLength(&self) -> i32;
    fn BigitAt(&self, index: i32) -> u32;
    fn SubtractTimes(&self, other: Ptr<v8_base_Bignum>, factor: i32);
}
impl v8_base_BignumImpl for Ptr<v8_base_Bignum> {
    fn Times10(&self) {
        ({ v8_base_BignumImpl::MultiplyByUInt32(self, 10_u32) });
        return;
    }
    fn EnsureCapacity(&self, size: i32) {
        let size: Value<i32> = Rc::new(RefCell::new(size));
        if ((*size.borrow()) > (*kBigitCapacity_125.with(Value::clone).borrow())) {
            ({
                V8_Fatal_113(
                    (*kUnreachableCodeMessage_9.with(Value::clone).borrow()).clone(),
                    &[],
                )
            });
        }
    }
    fn BigitLength(&self) -> i32 {
        return ((*(*(*self).upgrade().deref()).used_digits_.borrow())
            + (*(*(*self).upgrade().deref()).exponent_.borrow()));
    }
    fn AssignUInt16(&self, value: u16) {
        let value: Value<u16> = Rc::new(RefCell::new(value));
        (&(0));
        ({ v8_base_BignumImpl::Zero(self) });
        if (((*value.borrow()) as i32) == 0) {
            return;
        }
        ({ v8_base_BignumImpl::EnsureCapacity(self, 1) });
        ({
            v8_base_Vector_unsigned_int_Impl::operator_index(
                &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                0_usize,
            )
        })
        .write(((*value.borrow()) as u32));
        (*(*(*self).upgrade().deref()).used_digits_.borrow_mut()) = 1;
    }
    fn AssignUInt64(&self, value: u64) {
        let value: Value<u64> = Rc::new(RefCell::new(value));
        let kUInt64Size: Value<i32> = Rc::new(RefCell::new(64));
        ({ v8_base_BignumImpl::Zero(self) });
        if ((*value.borrow()) == 0_u64) {
            return;
        }
        let needed_bigits: Value<i32> = Rc::new(RefCell::new(
            (((*kUInt64Size.borrow()) / (*kBigitSize_123.with(Value::clone).borrow())) + 1),
        ));
        ({ v8_base_BignumImpl::EnsureCapacity(self, (*needed_bigits.borrow())) });
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*needed_bigits.borrow())) {
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .write(
                (((*value.borrow()) & ((*kBigitMask_124.with(Value::clone).borrow()) as u64))
                    as u32),
            );
            let __rhs = ((*value.borrow()) >> (*kBigitSize_123.with(Value::clone).borrow()));
            (*value.borrow_mut()) = __rhs;
            (*i.borrow_mut()).prefix_inc();
        }
        (*(*(*self).upgrade().deref()).used_digits_.borrow_mut()) = (*needed_bigits.borrow());
        ({ v8_base_BignumImpl::Clamp(self) });
    }
    fn AssignBignum(&self, other: Ptr<v8_base_Bignum>) {
        let __rhs = (*(*other.upgrade().deref()).exponent_.borrow());
        (*(*(*self).upgrade().deref()).exponent_.borrow_mut()) = __rhs;
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < (*(*other.upgrade().deref()).used_digits_.borrow())
        } {
            let __rhs = (({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*other.upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .read());
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new(
            (*(*other.upgrade().deref()).used_digits_.borrow()),
        ));
        'loop_: while ((*i.borrow()) < (*(*(*self).upgrade().deref()).used_digits_.borrow())) {
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .write(0_u32);
            (*i.borrow_mut()).prefix_inc();
        }
        let __rhs = (*(*other.upgrade().deref()).used_digits_.borrow());
        (*(*(*self).upgrade().deref()).used_digits_.borrow_mut()) = __rhs;
    }
    fn AssignDecimalString(&self, value: v8_base_Vector_const_char_) {
        let value: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(value));
        let kMaxUint64DecimalDigits: Value<i32> = Rc::new(RefCell::new(19));
        ({ v8_base_BignumImpl::Zero(self) });
        let length: Value<i32> = Rc::new(RefCell::new(
            ({ v8_base_Vector_const_char_Impl::length(&value.as_pointer()) }),
        ));
        let pos: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*length.borrow()) >= (*kMaxUint64DecimalDigits.borrow())) {
            let digits: Value<u64> = Rc::new(RefCell::new(
                ({
                    ReadUInt64_129(
                        (*value.borrow()).clone(),
                        (*pos.borrow()),
                        (*kMaxUint64DecimalDigits.borrow()),
                    )
                }),
            ));
            (*pos.borrow_mut()) += (*kMaxUint64DecimalDigits.borrow());
            (*length.borrow_mut()) -= (*kMaxUint64DecimalDigits.borrow());
            ({
                v8_base_BignumImpl::MultiplyByPowerOfTen(self, (*kMaxUint64DecimalDigits.borrow()))
            });
            ({ v8_base_BignumImpl::AddUInt64(self, (*digits.borrow())) });
        }
        let digits: Value<u64> = Rc::new(RefCell::new(
            ({
                ReadUInt64_129(
                    (*value.borrow()).clone(),
                    (*pos.borrow()),
                    (*length.borrow()),
                )
            }),
        ));
        ({ v8_base_BignumImpl::MultiplyByPowerOfTen(self, (*length.borrow())) });
        ({ v8_base_BignumImpl::AddUInt64(self, (*digits.borrow())) });
        ({ v8_base_BignumImpl::Clamp(self) });
    }
    fn AssignHexString(&self, value: v8_base_Vector_const_char_) {
        let value: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(value));
        ({ v8_base_BignumImpl::Zero(self) });
        let length: Value<i32> = Rc::new(RefCell::new(
            ({ v8_base_Vector_const_char_Impl::length(&value.as_pointer()) }),
        ));
        let needed_bigits: Value<i32> = Rc::new(RefCell::new(
            ((((*length.borrow()) * 4) / (*kBigitSize_123.with(Value::clone).borrow())) + 1),
        ));
        ({ v8_base_BignumImpl::EnsureCapacity(self, (*needed_bigits.borrow())) });
        let string_index: Value<i32> = Rc::new(RefCell::new(((*length.borrow()) - 1)));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < ((*needed_bigits.borrow()) - 1)) {
            let current_bigit: Value<u32> = Rc::new(RefCell::new(0_u32));
            let j: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*j.borrow()) < ((*kBigitSize_123.with(Value::clone).borrow()) / 4)) {
                {
                    let rhs_0 = (*current_bigit.borrow()).wrapping_add(
                        ((({
                            HexCharValue_130(
                                (({
                                    v8_base_Vector_const_char_Impl::operator_index(
                                        &value.as_pointer(),
                                        ((*string_index.borrow_mut()).postfix_dec() as usize),
                                    )
                                })
                                .read()),
                            )
                        }) << ((*j.borrow()) * 4)) as u32),
                    );
                    (*current_bigit.borrow_mut()) = rhs_0
                };
                (*j.borrow_mut()).postfix_inc();
            }
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .write((*current_bigit.borrow()));
            (*i.borrow_mut()).prefix_inc();
        }
        (*(*(*self).upgrade().deref()).used_digits_.borrow_mut()) = ((*needed_bigits.borrow()) - 1);
        let most_significant_bigit: Value<u32> = Rc::new(RefCell::new(0_u32));
        let j: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*j.borrow()) <= (*string_index.borrow())) {
            (*most_significant_bigit.borrow_mut()) <<= 4;
            {
                let rhs_0 = (*most_significant_bigit.borrow()).wrapping_add(
                    (({
                        HexCharValue_130(
                            (({
                                v8_base_Vector_const_char_Impl::operator_index(
                                    &value.as_pointer(),
                                    ((*j.borrow()) as usize),
                                )
                            })
                            .read()),
                        )
                    }) as u32),
                );
                (*most_significant_bigit.borrow_mut()) = rhs_0
            };
            (*j.borrow_mut()).prefix_inc();
        }
        if ((*most_significant_bigit.borrow()) != 0_u32) {
            ({
                let _index: usize =
                    ((*(*(*self).upgrade().deref()).used_digits_.borrow()) as usize);
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    _index,
                )
            })
            .write((*most_significant_bigit.borrow()));
            (*(*(*self).upgrade().deref()).used_digits_.borrow_mut()).postfix_inc();
        }
        ({ v8_base_BignumImpl::Clamp(self) });
    }
    fn AddUInt64(&self, operand: u64) {
        let operand: Value<u64> = Rc::new(RefCell::new(operand));
        if ((*operand.borrow()) == 0_u64) {
            return;
        }
        let other: Value<v8_base_Bignum> = Rc::new(RefCell::new(v8_base_Bignum::v8_base_Bignum()));
        ({ v8_base_BignumImpl::AssignUInt64(&other.as_pointer(), (*operand.borrow())) });
        ({ v8_base_BignumImpl::AddBignum(self, other.as_pointer()) });
    }
    fn AddBignum(&self, other: Ptr<v8_base_Bignum>) {
        (&(0));
        (&(0));
        ({ v8_base_BignumImpl::Align(self, (other).clone()) });
        ({
            let _size: i32 = {
                let _lhs = (1 + {
                    let __tmp_0: Value<i32> =
                        Rc::new(RefCell::new(({ v8_base_BignumImpl::BigitLength(self) })));
                    let __tmp_1: Value<i32> =
                        Rc::new(RefCell::new(({ v8_base_BignumImpl::BigitLength(&other) })));
                    (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
                        __tmp_0.as_pointer()
                    } else {
                        __tmp_1.as_pointer()
                    }
                    .read())
                });
                _lhs - (*(*(*self).upgrade().deref()).exponent_.borrow())
            };
            v8_base_BignumImpl::EnsureCapacity(self, _size)
        });
        let carry: Value<u32> = Rc::new(RefCell::new(0_u32));
        let bigit_pos: Value<i32> = Rc::new(RefCell::new({
            let _lhs = (*(*other.upgrade().deref()).exponent_.borrow());
            _lhs - (*(*(*self).upgrade().deref()).exponent_.borrow())
        }));
        (&(0));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < (*(*other.upgrade().deref()).used_digits_.borrow())
        } {
            let sum: Value<u32> = Rc::new(RefCell::new(
                ((({
                    v8_base_Vector_unsigned_int_Impl::operator_index(
                        &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                        ((*bigit_pos.borrow()) as usize),
                    )
                })
                .read())
                .wrapping_add(
                    (({
                        v8_base_Vector_unsigned_int_Impl::operator_index(
                            &(*other.upgrade().deref()).bigits_.as_pointer(),
                            ((*i.borrow()) as usize),
                        )
                    })
                    .read()),
                ))
                .wrapping_add((*carry.borrow())),
            ));
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*bigit_pos.borrow()) as usize),
                )
            })
            .write(((*sum.borrow()) & (*kBigitMask_124.with(Value::clone).borrow())));
            (*carry.borrow_mut()) =
                ((*sum.borrow()) >> (*kBigitSize_123.with(Value::clone).borrow()));
            (*bigit_pos.borrow_mut()).postfix_inc();
            (*i.borrow_mut()).prefix_inc();
        }
        'loop_: while ((*carry.borrow()) != 0_u32) {
            let sum: Value<u32> = Rc::new(RefCell::new(
                (({
                    v8_base_Vector_unsigned_int_Impl::operator_index(
                        &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                        ((*bigit_pos.borrow()) as usize),
                    )
                })
                .read())
                .wrapping_add((*carry.borrow())),
            ));
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*bigit_pos.borrow()) as usize),
                )
            })
            .write(((*sum.borrow()) & (*kBigitMask_124.with(Value::clone).borrow())));
            (*carry.borrow_mut()) =
                ((*sum.borrow()) >> (*kBigitSize_123.with(Value::clone).borrow()));
            (*bigit_pos.borrow_mut()).postfix_inc();
        }
        let __rhs = (if bigit_pos.as_pointer().read()
            >= (*(*self).upgrade().deref())
                .used_digits_
                .as_pointer()
                .read()
        {
            bigit_pos.as_pointer()
        } else {
            (*(*self).upgrade().deref()).used_digits_.as_pointer()
        }
        .read());
        (*(*(*self).upgrade().deref()).used_digits_.borrow_mut()) = __rhs;
        (&(0));
    }
    fn SubtractBignum(&self, other: Ptr<v8_base_Bignum>) {
        (&(0));
        (&(0));
        (&(0));
        ({ v8_base_BignumImpl::Align(self, (other).clone()) });
        let offset: Value<i32> = Rc::new(RefCell::new({
            let _lhs = (*(*other.upgrade().deref()).exponent_.borrow());
            _lhs - (*(*(*self).upgrade().deref()).exponent_.borrow())
        }));
        let borrow: Value<u32> = Rc::new(RefCell::new(0_u32));
        let i: Value<i32> = <Value<i32>>::default();
        (*i.borrow_mut()) = 0;
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < (*(*other.upgrade().deref()).used_digits_.borrow())
        } {
            (&(0));
            let difference: Value<u32> = Rc::new(RefCell::new(
                ((({
                    v8_base_Vector_unsigned_int_Impl::operator_index(
                        &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                        (((*i.borrow()) + (*offset.borrow())) as usize),
                    )
                })
                .read())
                .wrapping_sub(
                    (({
                        v8_base_Vector_unsigned_int_Impl::operator_index(
                            &(*other.upgrade().deref()).bigits_.as_pointer(),
                            ((*i.borrow()) as usize),
                        )
                    })
                    .read()),
                ))
                .wrapping_sub((*borrow.borrow())),
            ));
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    (((*i.borrow()) + (*offset.borrow())) as usize),
                )
            })
            .write(((*difference.borrow()) & (*kBigitMask_124.with(Value::clone).borrow())));
            (*borrow.borrow_mut()) =
                ((*difference.borrow()) >> ((*kChunkSize_121.with(Value::clone).borrow()) - 1));
            (*i.borrow_mut()).prefix_inc();
        }
        'loop_: while ((*borrow.borrow()) != 0_u32) {
            let difference: Value<u32> = Rc::new(RefCell::new(
                (({
                    v8_base_Vector_unsigned_int_Impl::operator_index(
                        &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                        (((*i.borrow()) + (*offset.borrow())) as usize),
                    )
                })
                .read())
                .wrapping_sub((*borrow.borrow())),
            ));
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    (((*i.borrow()) + (*offset.borrow())) as usize),
                )
            })
            .write(((*difference.borrow()) & (*kBigitMask_124.with(Value::clone).borrow())));
            (*borrow.borrow_mut()) =
                ((*difference.borrow()) >> ((*kChunkSize_121.with(Value::clone).borrow()) - 1));
            (*i.borrow_mut()).prefix_inc();
        }
        ({ v8_base_BignumImpl::Clamp(self) });
    }
    fn ShiftLeft(&self, shift_amount: i32) {
        let shift_amount: Value<i32> = Rc::new(RefCell::new(shift_amount));
        if ((*(*(*self).upgrade().deref()).used_digits_.borrow()) == 0) {
            return;
        }
        (*(*(*self).upgrade().deref()).exponent_.borrow_mut()) +=
            ((*shift_amount.borrow()) / (*kBigitSize_123.with(Value::clone).borrow()));
        let local_shift: Value<i32> = Rc::new(RefCell::new(
            ((*shift_amount.borrow()) % (*kBigitSize_123.with(Value::clone).borrow())),
        ));
        ({
            let _size: i32 = ((*(*(*self).upgrade().deref()).used_digits_.borrow()) + 1);
            v8_base_BignumImpl::EnsureCapacity(self, _size)
        });
        ({ v8_base_BignumImpl::BigitsShiftLeft(self, (*local_shift.borrow())) });
    }
    fn MultiplyByUInt32(&self, factor: u32) {
        let factor: Value<u32> = Rc::new(RefCell::new(factor));
        if ((*factor.borrow()) == 1_u32) {
            return;
        }
        if ((*factor.borrow()) == 0_u32) {
            ({ v8_base_BignumImpl::Zero(self) });
            return;
        }
        if ((*(*(*self).upgrade().deref()).used_digits_.borrow()) == 0) {
            return;
        }
        (&(0));
        let carry: Value<u64> = Rc::new(RefCell::new(0_u64));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*(*(*self).upgrade().deref()).used_digits_.borrow())) {
            let product: Value<u64> = Rc::new(RefCell::new(
                (((*factor.borrow()) as u64).wrapping_mul(
                    ((({
                        v8_base_Vector_unsigned_int_Impl::operator_index(
                            &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                            ((*i.borrow()) as usize),
                        )
                    })
                    .read()) as u64),
                ))
                .wrapping_add((*carry.borrow())),
            ));
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .write(
                (((*product.borrow()) & ((*kBigitMask_124.with(Value::clone).borrow()) as u64))
                    as u32),
            );
            (*carry.borrow_mut()) =
                ((*product.borrow()) >> (*kBigitSize_123.with(Value::clone).borrow()));
            (*i.borrow_mut()).prefix_inc();
        }
        'loop_: while ((*carry.borrow()) != 0_u64) {
            ({
                let _size: i32 = ((*(*(*self).upgrade().deref()).used_digits_.borrow()) + 1);
                v8_base_BignumImpl::EnsureCapacity(self, _size)
            });
            ({
                let _index: usize =
                    ((*(*(*self).upgrade().deref()).used_digits_.borrow()) as usize);
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    _index,
                )
            })
            .write(
                (((*carry.borrow()) & ((*kBigitMask_124.with(Value::clone).borrow()) as u64))
                    as u32),
            );
            (*(*(*self).upgrade().deref()).used_digits_.borrow_mut()).postfix_inc();
            (*carry.borrow_mut()) >>= (*kBigitSize_123.with(Value::clone).borrow());
        }
    }
    fn MultiplyByUInt64(&self, factor: u64) {
        let factor: Value<u64> = Rc::new(RefCell::new(factor));
        if ((*factor.borrow()) == 1_u64) {
            return;
        }
        if ((*factor.borrow()) == 0_u64) {
            ({ v8_base_BignumImpl::Zero(self) });
            return;
        }
        (&(0));
        let carry: Value<u64> = Rc::new(RefCell::new(0_u64));
        let low: Value<u64> = Rc::new(RefCell::new(((*factor.borrow()) & 4294967295_u64)));
        let high: Value<u64> = Rc::new(RefCell::new(((*factor.borrow()) >> 32)));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*(*(*self).upgrade().deref()).used_digits_.borrow())) {
            let product_low: Value<u64> = Rc::new(RefCell::new(
                (*low.borrow()).wrapping_mul(
                    ((({
                        v8_base_Vector_unsigned_int_Impl::operator_index(
                            &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                            ((*i.borrow()) as usize),
                        )
                    })
                    .read()) as u64),
                ),
            ));
            let product_high: Value<u64> = Rc::new(RefCell::new(
                (*high.borrow()).wrapping_mul(
                    ((({
                        v8_base_Vector_unsigned_int_Impl::operator_index(
                            &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                            ((*i.borrow()) as usize),
                        )
                    })
                    .read()) as u64),
                ),
            ));
            let tmp: Value<u64> = Rc::new(RefCell::new(
                ((*carry.borrow()) & ((*kBigitMask_124.with(Value::clone).borrow()) as u64))
                    .wrapping_add((*product_low.borrow())),
            ));
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .write(
                (((*tmp.borrow()) & ((*kBigitMask_124.with(Value::clone).borrow()) as u64)) as u32),
            );
            let __rhs = (((*carry.borrow()) >> (*kBigitSize_123.with(Value::clone).borrow()))
                .wrapping_add(((*tmp.borrow()) >> (*kBigitSize_123.with(Value::clone).borrow()))))
            .wrapping_add(
                ((*product_high.borrow()) << (32 - (*kBigitSize_123.with(Value::clone).borrow()))),
            );
            (*carry.borrow_mut()) = __rhs;
            (*i.borrow_mut()).prefix_inc();
        }
        'loop_: while ((*carry.borrow()) != 0_u64) {
            ({
                let _size: i32 = ((*(*(*self).upgrade().deref()).used_digits_.borrow()) + 1);
                v8_base_BignumImpl::EnsureCapacity(self, _size)
            });
            ({
                let _index: usize =
                    ((*(*(*self).upgrade().deref()).used_digits_.borrow()) as usize);
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    _index,
                )
            })
            .write(
                (((*carry.borrow()) & ((*kBigitMask_124.with(Value::clone).borrow()) as u64))
                    as u32),
            );
            (*(*(*self).upgrade().deref()).used_digits_.borrow_mut()).postfix_inc();
            (*carry.borrow_mut()) >>= (*kBigitSize_123.with(Value::clone).borrow());
        }
    }
    fn MultiplyByPowerOfTen(&self, exponent: i32) {
        let exponent: Value<i32> = Rc::new(RefCell::new(exponent));
        let kFive27: Value<u64> = Rc::new(RefCell::new(7450580596923828125_u64));
        let kFive1: Value<u16> = Rc::new(RefCell::new(5_u16));
        let kFive2: Value<u16> = Rc::new(RefCell::new(((((*kFive1.borrow()) as i32) * 5) as u16)));
        let kFive3: Value<u16> = Rc::new(RefCell::new(((((*kFive2.borrow()) as i32) * 5) as u16)));
        let kFive4: Value<u16> = Rc::new(RefCell::new(((((*kFive3.borrow()) as i32) * 5) as u16)));
        let kFive5: Value<u16> = Rc::new(RefCell::new(((((*kFive4.borrow()) as i32) * 5) as u16)));
        let kFive6: Value<u16> = Rc::new(RefCell::new(((((*kFive5.borrow()) as i32) * 5) as u16)));
        let kFive7: Value<u32> = Rc::new(RefCell::new(((((*kFive6.borrow()) as i32) * 5) as u32)));
        let kFive8: Value<u32> = Rc::new(RefCell::new((*kFive7.borrow()).wrapping_mul(5_u32)));
        let kFive9: Value<u32> = Rc::new(RefCell::new((*kFive8.borrow()).wrapping_mul(5_u32)));
        let kFive10: Value<u32> = Rc::new(RefCell::new((*kFive9.borrow()).wrapping_mul(5_u32)));
        let kFive11: Value<u32> = Rc::new(RefCell::new((*kFive10.borrow()).wrapping_mul(5_u32)));
        let kFive12: Value<u32> = Rc::new(RefCell::new((*kFive11.borrow()).wrapping_mul(5_u32)));
        let kFive13: Value<u32> = Rc::new(RefCell::new((*kFive12.borrow()).wrapping_mul(5_u32)));
        let kFive1_to_12: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
            ((*kFive1.borrow()) as u32),
            ((*kFive2.borrow()) as u32),
            ((*kFive3.borrow()) as u32),
            ((*kFive4.borrow()) as u32),
            ((*kFive5.borrow()) as u32),
            ((*kFive6.borrow()) as u32),
            (*kFive7.borrow()),
            (*kFive8.borrow()),
            (*kFive9.borrow()),
            (*kFive10.borrow()),
            (*kFive11.borrow()),
            (*kFive12.borrow()),
        ])));
        (&(0));
        if ((*exponent.borrow()) == 0) {
            return;
        }
        if ((*(*(*self).upgrade().deref()).used_digits_.borrow()) == 0) {
            return;
        }
        let remaining_exponent: Value<i32> = Rc::new(RefCell::new((*exponent.borrow())));
        'loop_: while ((*remaining_exponent.borrow()) >= 27) {
            ({ v8_base_BignumImpl::MultiplyByUInt64(self, (*kFive27.borrow())) });
            (*remaining_exponent.borrow_mut()) -= 27;
        }
        'loop_: while ((*remaining_exponent.borrow()) >= 13) {
            ({ v8_base_BignumImpl::MultiplyByUInt32(self, (*kFive13.borrow())) });
            (*remaining_exponent.borrow_mut()) -= 13;
        }
        if ((*remaining_exponent.borrow()) > 0) {
            ({
                v8_base_BignumImpl::MultiplyByUInt32(
                    self,
                    (*kFive1_to_12.borrow())[((*remaining_exponent.borrow()) - 1) as usize],
                )
            });
        }
        ({ v8_base_BignumImpl::ShiftLeft(self, (*exponent.borrow())) });
    }
    fn Square(&self) {
        (&(0));
        let product_length: Value<i32> = Rc::new(RefCell::new(
            (2 * (*(*(*self).upgrade().deref()).used_digits_.borrow())),
        ));
        ({ v8_base_BignumImpl::EnsureCapacity(self, (*product_length.borrow())) });
        if ((1
            << (2
                * ((*kChunkSize_121.with(Value::clone).borrow())
                    - (*kBigitSize_123.with(Value::clone).borrow()))))
            <= (*(*(*self).upgrade().deref()).used_digits_.borrow()))
        {
            ({
                FatalNoSecurityImpact_131(
                    (*kUnimplementedCodeMessage_8.with(Value::clone).borrow()).clone(),
                    &[],
                )
            });
        }
        let accumulator: Value<u64> = Rc::new(RefCell::new(0_u64));
        let copy_offset: Value<i32> = Rc::new(RefCell::new(
            (*(*(*self).upgrade().deref()).used_digits_.borrow()),
        ));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*(*(*self).upgrade().deref()).used_digits_.borrow())) {
            let __rhs = (({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .read());
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    (((*copy_offset.borrow()) + (*i.borrow())) as usize),
                )
            })
            .write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*(*(*self).upgrade().deref()).used_digits_.borrow())) {
            let bigit_index1: Value<i32> = Rc::new(RefCell::new((*i.borrow())));
            let bigit_index2: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*bigit_index1.borrow()) >= 0) {
                let chunk1: Value<u32> = Rc::new(RefCell::new(
                    (({
                        v8_base_Vector_unsigned_int_Impl::operator_index(
                            &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                            (((*copy_offset.borrow()) + (*bigit_index1.borrow())) as usize),
                        )
                    })
                    .read()),
                ));
                let chunk2: Value<u32> = Rc::new(RefCell::new(
                    (({
                        v8_base_Vector_unsigned_int_Impl::operator_index(
                            &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                            (((*copy_offset.borrow()) + (*bigit_index2.borrow())) as usize),
                        )
                    })
                    .read()),
                ));
                {
                    let rhs_0 = (*accumulator.borrow()).wrapping_add(
                        ((*chunk1.borrow()) as u64).wrapping_mul(((*chunk2.borrow()) as u64)),
                    );
                    (*accumulator.borrow_mut()) = rhs_0
                };
                (*bigit_index1.borrow_mut()).postfix_dec();
                (*bigit_index2.borrow_mut()).postfix_inc();
            }
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .write(
                (((*accumulator.borrow()) as u32) & (*kBigitMask_124.with(Value::clone).borrow())),
            );
            (*accumulator.borrow_mut()) >>= (*kBigitSize_123.with(Value::clone).borrow());
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new(
            (*(*(*self).upgrade().deref()).used_digits_.borrow()),
        ));
        'loop_: while ((*i.borrow()) < (*product_length.borrow())) {
            let bigit_index1: Value<i32> = Rc::new(RefCell::new(
                ((*(*(*self).upgrade().deref()).used_digits_.borrow()) - 1),
            ));
            let bigit_index2: Value<i32> =
                Rc::new(RefCell::new(((*i.borrow()) - (*bigit_index1.borrow()))));
            'loop_: while ((*bigit_index2.borrow())
                < (*(*(*self).upgrade().deref()).used_digits_.borrow()))
            {
                let chunk1: Value<u32> = Rc::new(RefCell::new(
                    (({
                        v8_base_Vector_unsigned_int_Impl::operator_index(
                            &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                            (((*copy_offset.borrow()) + (*bigit_index1.borrow())) as usize),
                        )
                    })
                    .read()),
                ));
                let chunk2: Value<u32> = Rc::new(RefCell::new(
                    (({
                        v8_base_Vector_unsigned_int_Impl::operator_index(
                            &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                            (((*copy_offset.borrow()) + (*bigit_index2.borrow())) as usize),
                        )
                    })
                    .read()),
                ));
                {
                    let rhs_0 = (*accumulator.borrow()).wrapping_add(
                        ((*chunk1.borrow()) as u64).wrapping_mul(((*chunk2.borrow()) as u64)),
                    );
                    (*accumulator.borrow_mut()) = rhs_0
                };
                (*bigit_index1.borrow_mut()).postfix_dec();
                (*bigit_index2.borrow_mut()).postfix_inc();
            }
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .write(
                (((*accumulator.borrow()) as u32) & (*kBigitMask_124.with(Value::clone).borrow())),
            );
            (*accumulator.borrow_mut()) >>= (*kBigitSize_123.with(Value::clone).borrow());
            (*i.borrow_mut()).prefix_inc();
        }
        (&(0));
        (*(*(*self).upgrade().deref()).used_digits_.borrow_mut()) = (*product_length.borrow());
        (*(*(*self).upgrade().deref()).exponent_.borrow_mut()) *= 2;
        ({ v8_base_BignumImpl::Clamp(self) });
    }
    fn AssignPowerUInt16(&self, base: u16, power_exponent: i32) {
        let base: Value<u16> = Rc::new(RefCell::new(base));
        let power_exponent: Value<i32> = Rc::new(RefCell::new(power_exponent));
        (&(0));
        (&(0));
        if ((*power_exponent.borrow()) == 0) {
            ({ v8_base_BignumImpl::AssignUInt16(self, 1_u16) });
            return;
        }
        ({ v8_base_BignumImpl::Zero(self) });
        let shifts: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((((*base.borrow()) as i32) & 1) == 0) {
            {
                let rhs_0 = (((*base.borrow()) as i32) >> 1) as u16;
                (*base.borrow_mut()) = rhs_0
            };
            (*shifts.borrow_mut()).postfix_inc();
        }
        let bit_size: Value<i32> = Rc::new(RefCell::new(0));
        let tmp_base: Value<i32> = Rc::new(RefCell::new(((*base.borrow()) as i32)));
        'loop_: while ((*tmp_base.borrow()) != 0) {
            (*tmp_base.borrow_mut()) >>= 1;
            (*bit_size.borrow_mut()).postfix_inc();
        }
        let final_size: Value<i32> = Rc::new(RefCell::new(
            ((*bit_size.borrow()) * (*power_exponent.borrow())),
        ));
        ({
            v8_base_BignumImpl::EnsureCapacity(
                self,
                (((*final_size.borrow()) / (*kBigitSize_123.with(Value::clone).borrow())) + 2),
            )
        });
        let mask: Value<i32> = Rc::new(RefCell::new(1));
        'loop_: while ((*power_exponent.borrow()) >= (*mask.borrow())) {
            (*mask.borrow_mut()) <<= 1;
        }
        (*mask.borrow_mut()) >>= 2;
        let this_value: Value<u64> = Rc::new(RefCell::new(((*base.borrow()) as u64)));
        let delayed_multipliciation: Value<bool> = Rc::new(RefCell::new(false));
        let max_32bits: Value<u64> = Rc::new(RefCell::new(4294967295_u64));
        'loop_: while ((*mask.borrow()) != 0) && ((*this_value.borrow()) <= (*max_32bits.borrow()))
        {
            let __rhs = (*this_value.borrow()).wrapping_mul((*this_value.borrow()));
            (*this_value.borrow_mut()) = __rhs;
            if (((*power_exponent.borrow()) & (*mask.borrow())) != 0) {
                let base_bits_mask: Value<u64> = Rc::new(RefCell::new(
                    !((1_u64 << (64 - (*bit_size.borrow()))).wrapping_sub(1_u64)),
                ));
                let high_bits_zero: Value<bool> = Rc::new(RefCell::new(
                    (((*this_value.borrow()) & (*base_bits_mask.borrow())) == 0_u64),
                ));
                if (*high_bits_zero.borrow()) {
                    {
                        let rhs_0 = (*this_value.borrow()).wrapping_mul(((*base.borrow()) as u64));
                        (*this_value.borrow_mut()) = rhs_0
                    };
                } else {
                    (*delayed_multipliciation.borrow_mut()) = true;
                }
            }
            (*mask.borrow_mut()) >>= 1;
        }
        ({ v8_base_BignumImpl::AssignUInt64(self, (*this_value.borrow())) });
        if (*delayed_multipliciation.borrow()) {
            ({ v8_base_BignumImpl::MultiplyByUInt32(self, ((*base.borrow()) as u32)) });
        }
        'loop_: while ((*mask.borrow()) != 0) {
            ({ v8_base_BignumImpl::Square(self) });
            if (((*power_exponent.borrow()) & (*mask.borrow())) != 0) {
                ({ v8_base_BignumImpl::MultiplyByUInt32(self, ((*base.borrow()) as u32)) });
            }
            (*mask.borrow_mut()) >>= 1;
        }
        ({
            v8_base_BignumImpl::ShiftLeft(self, ((*shifts.borrow()) * (*power_exponent.borrow())))
        });
    }
    fn DivideModuloIntBignum(&self, other: Ptr<v8_base_Bignum>) -> u16 {
        (&(0));
        (&(0));
        (&(0));
        if {
            let _lhs = ({ v8_base_BignumImpl::BigitLength(self) });
            _lhs < ({ v8_base_BignumImpl::BigitLength(&other) })
        } {
            return 0_u16;
        }
        ({ v8_base_BignumImpl::Align(self, (other).clone()) });
        let result: Value<u16> = Rc::new(RefCell::new(0_u16));
        'loop_: while {
            let _lhs = ({ v8_base_BignumImpl::BigitLength(self) });
            _lhs > ({ v8_base_BignumImpl::BigitLength(&other) })
        } {
            (&(0));
            {
                let rhs_0 = (((*result.borrow()) as u32).wrapping_add(
                    (({
                        let _index: usize =
                            (((*(*(*self).upgrade().deref()).used_digits_.borrow()) - 1) as usize);
                        v8_base_Vector_unsigned_int_Impl::operator_index(
                            &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                            _index,
                        )
                    })
                    .read()),
                )) as u16;
                (*result.borrow_mut()) = rhs_0
            };
            ({
                let _other: Ptr<v8_base_Bignum> = (other).clone();
                let _factor: i32 = ((({
                    let _index: usize =
                        (((*(*(*self).upgrade().deref()).used_digits_.borrow()) - 1) as usize);
                    v8_base_Vector_unsigned_int_Impl::operator_index(
                        &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                        _index,
                    )
                })
                .read()) as i32);
                v8_base_BignumImpl::SubtractTimes(self, _other, _factor)
            });
        }
        (&(0));
        let this_bigit: Value<u32> = Rc::new(RefCell::new(
            (({
                let _index: usize =
                    (((*(*(*self).upgrade().deref()).used_digits_.borrow()) - 1) as usize);
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    _index,
                )
            })
            .read()),
        ));
        let other_bigit: Value<u32> = Rc::new(RefCell::new(
            (({
                let _index: usize =
                    (((*(*other.upgrade().deref()).used_digits_.borrow()) - 1) as usize);
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*other.upgrade().deref()).bigits_.as_pointer(),
                    _index,
                )
            })
            .read()),
        ));
        if ((*(*other.upgrade().deref()).used_digits_.borrow()) == 1) {
            let quotient: Value<i32> = Rc::new(RefCell::new(
                (((*this_bigit.borrow()).wrapping_div((*other_bigit.borrow()))) as i32),
            ));
            ({
                let _index: usize =
                    (((*(*(*self).upgrade().deref()).used_digits_.borrow()) - 1) as usize);
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    _index,
                )
            })
            .write(
                (*this_bigit.borrow()).wrapping_sub(
                    (*other_bigit.borrow()).wrapping_mul(((*quotient.borrow()) as u32)),
                ),
            );
            {
                let rhs_0 = (((*result.borrow()) as i32) + (*quotient.borrow())) as u16;
                (*result.borrow_mut()) = rhs_0
            };
            ({ v8_base_BignumImpl::Clamp(self) });
            return (*result.borrow());
        }
        let division_estimate: Value<i32> = Rc::new(RefCell::new(
            (((*this_bigit.borrow()).wrapping_div(((*other_bigit.borrow()).wrapping_add(1_u32))))
                as i32),
        ));
        {
            let rhs_0 = (((*result.borrow()) as i32) + (*division_estimate.borrow())) as u16;
            (*result.borrow_mut()) = rhs_0
        };
        ({
            let _other: Ptr<v8_base_Bignum> = (other).clone();
            let _factor: i32 = (*division_estimate.borrow());
            v8_base_BignumImpl::SubtractTimes(self, _other, _factor)
        });
        if ((*other_bigit.borrow()).wrapping_mul((((*division_estimate.borrow()) + 1) as u32))
            > (*this_bigit.borrow()))
        {
            return (*result.borrow());
        }
        'loop_: while ({
            let _a: Ptr<v8_base_Bignum> = (other).clone();
            let _b: Ptr<v8_base_Bignum> = (*self).clone();
            v8_base_Bignum::LessEqual(_a, _b)
        }) {
            ({ v8_base_BignumImpl::SubtractBignum(self, (other).clone()) });
            (*result.borrow_mut()).postfix_inc();
        }
        return (*result.borrow());
    }
    fn ToHexString(&self, buffer: Ptr<u8>, buffer_size: i32) -> bool {
        let buffer: Value<Ptr<u8>> = Rc::new(RefCell::new(buffer));
        let buffer_size: Value<i32> = Rc::new(RefCell::new(buffer_size));
        (&(0));
        (&(0));
        let kHexCharsPerBigit: Value<i32> = Rc::new(RefCell::new(
            ((*kBigitSize_123.with(Value::clone).borrow()) / 4),
        ));
        if ((*(*(*self).upgrade().deref()).used_digits_.borrow()) == 0) {
            if ((*buffer_size.borrow()) < 2) {
                return false;
            }
            (*buffer.borrow()).offset((0) as isize).write(('0' as u8));
            (*buffer.borrow()).offset((1) as isize).write(('\0' as u8));
            return true;
        }
        let needed_chars: Value<i32> = Rc::new(RefCell::new(
            ((((({ v8_base_BignumImpl::BigitLength(self) }) - 1) * (*kHexCharsPerBigit.borrow()))
                + ({
                    SizeInHexChars_132(
                        (({
                            let _index: usize =
                                (((*(*(*self).upgrade().deref()).used_digits_.borrow()) - 1)
                                    as usize);
                            v8_base_Vector_unsigned_int_Impl::operator_index(
                                &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                                _index,
                            )
                        })
                        .read()),
                    )
                }))
                + 1),
        ));
        if ((*needed_chars.borrow()) > (*buffer_size.borrow())) {
            return false;
        }
        let string_index: Value<i32> = Rc::new(RefCell::new(((*needed_chars.borrow()) - 1)));
        (*buffer.borrow())
            .offset(((*string_index.borrow_mut()).postfix_dec()) as isize)
            .write(('\0' as u8));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*(*(*self).upgrade().deref()).exponent_.borrow())) {
            let j: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*j.borrow()) < (*kHexCharsPerBigit.borrow())) {
                (*buffer.borrow())
                    .offset(((*string_index.borrow_mut()).postfix_dec()) as isize)
                    .write(('0' as u8));
                (*j.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < ((*(*(*self).upgrade().deref()).used_digits_.borrow()) - 1))
        {
            let current_bigit: Value<u32> = Rc::new(RefCell::new(
                (({
                    v8_base_Vector_unsigned_int_Impl::operator_index(
                        &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                        ((*i.borrow()) as usize),
                    )
                })
                .read()),
            ));
            let j: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*j.borrow()) < (*kHexCharsPerBigit.borrow())) {
                let __rhs = ({ HexCharOfValue_128((((*current_bigit.borrow()) & 15_u32) as i32)) });
                (*buffer.borrow())
                    .offset(((*string_index.borrow_mut()).postfix_dec()) as isize)
                    .write(__rhs);
                (*current_bigit.borrow_mut()) >>= 4;
                (*j.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let most_significant_bigit: Value<u32> = Rc::new(RefCell::new(
            (({
                let _index: usize =
                    (((*(*(*self).upgrade().deref()).used_digits_.borrow()) - 1) as usize);
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    _index,
                )
            })
            .read()),
        ));
        'loop_: while ((*most_significant_bigit.borrow()) != 0_u32) {
            let __rhs =
                ({ HexCharOfValue_128((((*most_significant_bigit.borrow()) & 15_u32) as i32)) });
            (*buffer.borrow())
                .offset(((*string_index.borrow_mut()).postfix_dec()) as isize)
                .write(__rhs);
            (*most_significant_bigit.borrow_mut()) >>= 4;
        }
        return true;
    }
    fn BigitAt(&self, index: i32) -> u32 {
        let index: Value<i32> = Rc::new(RefCell::new(index));
        if ((*index.borrow()) >= ({ v8_base_BignumImpl::BigitLength(self) })) {
            return 0_u32;
        }
        if ((*index.borrow()) < (*(*(*self).upgrade().deref()).exponent_.borrow())) {
            return 0_u32;
        }
        return (({
            let _index: usize =
                (((*index.borrow()) - (*(*(*self).upgrade().deref()).exponent_.borrow())) as usize);
            v8_base_Vector_unsigned_int_Impl::operator_index(
                &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                _index,
            )
        })
        .read());
    }
    fn Clamp(&self) {
        'loop_: while ((*(*(*self).upgrade().deref()).used_digits_.borrow()) > 0)
            && ((({
                let _index: usize =
                    (((*(*(*self).upgrade().deref()).used_digits_.borrow()) - 1) as usize);
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    _index,
                )
            })
            .read())
                == 0_u32)
        {
            (*(*(*self).upgrade().deref()).used_digits_.borrow_mut()).postfix_dec();
        }
        if ((*(*(*self).upgrade().deref()).used_digits_.borrow()) == 0) {
            (*(*(*self).upgrade().deref()).exponent_.borrow_mut()) = 0;
        }
    }
    fn IsClamped(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).used_digits_.borrow()) == 0)
            || ((({
                let _index: usize =
                    (((*(*(*self).upgrade().deref()).used_digits_.borrow()) - 1) as usize);
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    _index,
                )
            })
            .read())
                != 0_u32);
    }
    fn Zero(&self) {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*(*(*self).upgrade().deref()).used_digits_.borrow())) {
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .write(0_u32);
            (*i.borrow_mut()).prefix_inc();
        }
        (*(*(*self).upgrade().deref()).used_digits_.borrow_mut()) = 0;
        (*(*(*self).upgrade().deref()).exponent_.borrow_mut()) = 0;
    }
    fn Align(&self, other: Ptr<v8_base_Bignum>) {
        if {
            let _lhs = (*(*(*self).upgrade().deref()).exponent_.borrow());
            _lhs > (*(*other.upgrade().deref()).exponent_.borrow())
        } {
            let zero_digits: Value<i32> = Rc::new(RefCell::new({
                let _lhs = (*(*(*self).upgrade().deref()).exponent_.borrow());
                _lhs - (*(*other.upgrade().deref()).exponent_.borrow())
            }));
            ({
                let _size: i32 = ((*(*(*self).upgrade().deref()).used_digits_.borrow())
                    + (*zero_digits.borrow()));
                v8_base_BignumImpl::EnsureCapacity(self, _size)
            });
            let i: Value<i32> = Rc::new(RefCell::new(
                ((*(*(*self).upgrade().deref()).used_digits_.borrow()) - 1),
            ));
            'loop_: while ((*i.borrow()) >= 0) {
                let __rhs = (({
                    v8_base_Vector_unsigned_int_Impl::operator_index(
                        &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                        ((*i.borrow()) as usize),
                    )
                })
                .read());
                ({
                    v8_base_Vector_unsigned_int_Impl::operator_index(
                        &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                        (((*i.borrow()) + (*zero_digits.borrow())) as usize),
                    )
                })
                .write(__rhs);
                (*i.borrow_mut()).prefix_dec();
            }
            let i: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*i.borrow()) < (*zero_digits.borrow())) {
                ({
                    v8_base_Vector_unsigned_int_Impl::operator_index(
                        &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                        ((*i.borrow()) as usize),
                    )
                })
                .write(0_u32);
                (*i.borrow_mut()).prefix_inc();
            }
            (*(*(*self).upgrade().deref()).used_digits_.borrow_mut()) += (*zero_digits.borrow());
            (*(*(*self).upgrade().deref()).exponent_.borrow_mut()) -= (*zero_digits.borrow());
            (&(0));
            (&(0));
        }
    }
    fn BigitsShiftLeft(&self, shift_amount: i32) {
        let shift_amount: Value<i32> = Rc::new(RefCell::new(shift_amount));
        (&(0));
        (&(0));
        let carry: Value<u32> = Rc::new(RefCell::new(0_u32));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*(*(*self).upgrade().deref()).used_digits_.borrow())) {
            let new_carry: Value<u32> = Rc::new(RefCell::new(
                ((({
                    v8_base_Vector_unsigned_int_Impl::operator_index(
                        &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                        ((*i.borrow()) as usize),
                    )
                })
                .read())
                    >> ((*kBigitSize_123.with(Value::clone).borrow()) - (*shift_amount.borrow()))),
            ));
            let __rhs = ((((({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .read())
                << (*shift_amount.borrow()))
            .wrapping_add((*carry.borrow())))
                & (*kBigitMask_124.with(Value::clone).borrow()));
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .write(__rhs);
            (*carry.borrow_mut()) = (*new_carry.borrow());
            (*i.borrow_mut()).prefix_inc();
        }
        if ((*carry.borrow()) != 0_u32) {
            ({
                let _index: usize =
                    ((*(*(*self).upgrade().deref()).used_digits_.borrow()) as usize);
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    _index,
                )
            })
            .write((*carry.borrow()));
            (*(*(*self).upgrade().deref()).used_digits_.borrow_mut()).postfix_inc();
        }
    }
    fn SubtractTimes(&self, other: Ptr<v8_base_Bignum>, factor: i32) {
        let factor: Value<i32> = Rc::new(RefCell::new(factor));
        (&(0));
        if ((*factor.borrow()) < 3) {
            let i: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*i.borrow()) < (*factor.borrow())) {
                ({ v8_base_BignumImpl::SubtractBignum(self, (other).clone()) });
                (*i.borrow_mut()).prefix_inc();
            }
            return;
        }
        let borrow: Value<u32> = Rc::new(RefCell::new(0_u32));
        let exponent_diff: Value<i32> = Rc::new(RefCell::new({
            let _lhs = (*(*other.upgrade().deref()).exponent_.borrow());
            _lhs - (*(*(*self).upgrade().deref()).exponent_.borrow())
        }));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < (*(*other.upgrade().deref()).used_digits_.borrow())
        } {
            let product: Value<u64> = Rc::new(RefCell::new(
                ((*factor.borrow()) as u64).wrapping_mul(
                    ((({
                        v8_base_Vector_unsigned_int_Impl::operator_index(
                            &(*other.upgrade().deref()).bigits_.as_pointer(),
                            ((*i.borrow()) as usize),
                        )
                    })
                    .read()) as u64),
                ),
            ));
            let remove: Value<u64> = Rc::new(RefCell::new(
                ((*borrow.borrow()) as u64).wrapping_add((*product.borrow())),
            ));
            let difference: Value<u32> = Rc::new(RefCell::new(
                (({
                    v8_base_Vector_unsigned_int_Impl::operator_index(
                        &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                        (((*i.borrow()) + (*exponent_diff.borrow())) as usize),
                    )
                })
                .read())
                .wrapping_sub(
                    (((*remove.borrow()) & ((*kBigitMask_124.with(Value::clone).borrow()) as u64))
                        as u32),
                ),
            ));
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    (((*i.borrow()) + (*exponent_diff.borrow())) as usize),
                )
            })
            .write(((*difference.borrow()) & (*kBigitMask_124.with(Value::clone).borrow())));
            (*borrow.borrow_mut()) = (((((*difference.borrow())
                >> ((*kChunkSize_121.with(Value::clone).borrow()) - 1))
                as u64)
                .wrapping_add(
                    ((*remove.borrow()) >> (*kBigitSize_123.with(Value::clone).borrow())),
                )) as u32);
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new({
            let _lhs = (*(*other.upgrade().deref()).used_digits_.borrow());
            _lhs + (*exponent_diff.borrow())
        }));
        'loop_: while ((*i.borrow()) < (*(*(*self).upgrade().deref()).used_digits_.borrow())) {
            if ((*borrow.borrow()) == 0_u32) {
                return;
            }
            let difference: Value<u32> = Rc::new(RefCell::new(
                (({
                    v8_base_Vector_unsigned_int_Impl::operator_index(
                        &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                        ((*i.borrow()) as usize),
                    )
                })
                .read())
                .wrapping_sub((*borrow.borrow())),
            ));
            ({
                v8_base_Vector_unsigned_int_Impl::operator_index(
                    &(*(*self).upgrade().deref()).bigits_.as_pointer(),
                    ((*i.borrow()) as usize),
                )
            })
            .write(((*difference.borrow()) & (*kBigitMask_124.with(Value::clone).borrow())));
            (*borrow.borrow_mut()) =
                ((*difference.borrow()) >> ((*kChunkSize_121.with(Value::clone).borrow()) - 1));
            (*i.borrow_mut()).prefix_inc();
        }
        ({ v8_base_BignumImpl::Clamp(self) });
        (&(0));
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
            hash_combine_97(
                (*(*(*self).upgrade().deref()).hash_.borrow()),
                (*other_hash.borrow()),
            )
        });
        (*(*(*self).upgrade().deref()).hash_.borrow_mut()) = __rhs;
        return (*self).clone();
    }
}
pub trait v8_base_Vector_const_char_Impl {
    fn length(&self) -> i32;
    fn operator_index(&self, index: usize) -> Ptr<u8>;
}
impl v8_base_Vector_const_char_Impl for Ptr<v8_base_Vector_const_char_> {
    fn length(&self) -> i32 {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            let _cmp: Value<bool> = Rc::new(RefCell::new(
                ({
                    CmpGEImpl_20(
                        (<i32>::MAX),
                        ((*(*(*self).upgrade().deref()).length_.borrow()) as u64),
                    )
                }),
            ));
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if ((!(!(!(*_cmp.borrow()))) as i64) != 0) {
                    ({
                        V8_Fatal_113(
                            Ptr::from_string_literal(b"Check failed: %s."),
                            &[(Ptr::from_string_literal(
                                b"std::numeric_limits<int>::max() >= length_",
                            ))
                            .into()],
                        )
                    });
                }
            }
        }
        return ((*(*(*self).upgrade().deref()).length_.borrow()) as i32);
    }
    fn operator_index(&self, index: usize) -> Ptr<u8> {
        let index: Value<usize> = Rc::new(RefCell::new(index));
        (&(0));
        return (*(*(*self).upgrade().deref()).start_.borrow()).offset((*index.borrow()) as isize);
    }
}
pub trait v8_base_Vector_unsigned_int_Impl {
    fn operator_index(&self, index: usize) -> Ptr<u32>;
}
impl v8_base_Vector_unsigned_int_Impl for Ptr<v8_base_Vector_unsigned_int_> {
    fn operator_index(&self, index: usize) -> Ptr<u32> {
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
        return (({ bit_cast_33(lhs.as_pointer()) }) == ({ bit_cast_33(rhs.as_pointer()) }));
    }
}
pub trait v8_base_bit_equal_to_float_Impl {
    fn operator_call(&self, lhs: f32, rhs: f32) -> bool;
}
impl v8_base_bit_equal_to_float_Impl for Ptr<v8_base_bit_equal_to_float_> {
    fn operator_call(&self, lhs: f32, rhs: f32) -> bool {
        let lhs: Value<f32> = Rc::new(RefCell::new(lhs));
        let rhs: Value<f32> = Rc::new(RefCell::new(rhs));
        return (({ bit_cast_31(lhs.as_pointer()) }) == ({ bit_cast_31(rhs.as_pointer()) }));
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
            let _v: Value<u64> = Rc::new(RefCell::new(({ bit_cast_33(v.as_pointer()) })));
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
            let _v: Value<u32> = Rc::new(RefCell::new(({ bit_cast_31(v.as_pointer()) })));
            v8_base_hash_unsigned_int_Impl::operator_call(&h.as_pointer(), _v.as_pointer())
        });
    }
}
pub trait v8_base_hash_unsigned_int_Impl {
    fn operator_call(&self, v: Ptr<u32>) -> usize;
}
impl v8_base_hash_unsigned_int_Impl for Ptr<v8_base_hash_unsigned_int_> {
    fn operator_call(&self, v: Ptr<u32>) -> usize {
        return ({ hash_value_95((v.read())) });
    }
}
pub trait v8_base_hash_unsigned_long_long_Impl {
    fn operator_call(&self, v: Ptr<u64>) -> usize;
}
impl v8_base_hash_unsigned_long_long_Impl for Ptr<v8_base_hash_unsigned_long_long_> {
    fn operator_call(&self, v: Ptr<u64>) -> usize {
        return ({ hash_value_96((v.read())) });
    }
}
