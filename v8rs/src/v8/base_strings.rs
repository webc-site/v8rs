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
    pub static kUC16Size_120: Value<i32> = Rc::new(RefCell::new(2));
);
pub fn HexValue_121(c: u32) -> i32 {
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
pub fn HexCharOfValue_122(value: i32) -> u8 {
    let value: Value<i32> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) < 10) {
        return (((*value.borrow()) + (('0' as u8) as i32)) as u8);
    }
    return ((((*value.borrow()) - 10) + (('A' as u8) as i32)) as u8);
}
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
pub type v8_TaskPriority = u8;
pub const v8_TaskPriority_kBestEffort: v8_TaskPriority = 0;
pub const v8_TaskPriority_kUserVisible: v8_TaskPriority = 1;
pub const v8_TaskPriority_kUserBlocking: v8_TaskPriority = 2;
pub const v8_TaskPriority_kMaxPriority: v8_TaskPriority = 2;
pub trait v8_Task {
    fn Run(&self);
}
pub trait v8_IdleTask {
    fn Run(&self, deadline_in_seconds: f64);
}
pub trait v8_TaskRunner {
    fn PostTask(&self, task: Option<Value<v8_Task>>, location: Option<v8_SourceLocation>) {
        let task: Value<Option<Value<v8_Task>>> = Rc::new(RefCell::new(task));
        let location: Value<v8_SourceLocation> = Rc::new(RefCell::new(
            location.unwrap_or(({ v8_SourceLocation::Current(None) })),
        ));
        ({
            (*(*self).upgrade().deref())
                .PostTaskImpl((*task.borrow_mut()).take(), location.as_pointer())
        });
    }
    fn PostNonNestableTask(
        &self,
        task: Option<Value<v8_Task>>,
        location: Option<v8_SourceLocation>,
    ) {
        let task: Value<Option<Value<v8_Task>>> = Rc::new(RefCell::new(task));
        let location: Value<v8_SourceLocation> = Rc::new(RefCell::new(
            location.unwrap_or(({ v8_SourceLocation::Current(None) })),
        ));
        ({
            (*(*self).upgrade().deref())
                .PostNonNestableTaskImpl((*task.borrow_mut()).take(), location.as_pointer())
        });
    }
    fn PostDelayedTask(
        &self,
        task: Option<Value<v8_Task>>,
        delay_in_seconds: f64,
        location: Option<v8_SourceLocation>,
    ) {
        let task: Value<Option<Value<v8_Task>>> = Rc::new(RefCell::new(task));
        let delay_in_seconds: Value<f64> = Rc::new(RefCell::new(delay_in_seconds));
        let location: Value<v8_SourceLocation> = Rc::new(RefCell::new(
            location.unwrap_or(({ v8_SourceLocation::Current(None) })),
        ));
        ({
            (*(*self).upgrade().deref()).PostDelayedTaskImpl(
                (*task.borrow_mut()).take(),
                (*delay_in_seconds.borrow()),
                location.as_pointer(),
            )
        });
    }
    fn PostNonNestableDelayedTask(
        &self,
        task: Option<Value<v8_Task>>,
        delay_in_seconds: f64,
        location: Option<v8_SourceLocation>,
    ) {
        let task: Value<Option<Value<v8_Task>>> = Rc::new(RefCell::new(task));
        let delay_in_seconds: Value<f64> = Rc::new(RefCell::new(delay_in_seconds));
        let location: Value<v8_SourceLocation> = Rc::new(RefCell::new(
            location.unwrap_or(({ v8_SourceLocation::Current(None) })),
        ));
        ({
            (*(*self).upgrade().deref()).PostNonNestableDelayedTaskImpl(
                (*task.borrow_mut()).take(),
                (*delay_in_seconds.borrow()),
                location.as_pointer(),
            )
        });
    }
    fn PostIdleTask(&self, task: Option<Value<v8_IdleTask>>, location: Option<v8_SourceLocation>) {
        let task: Value<Option<Value<v8_IdleTask>>> = Rc::new(RefCell::new(task));
        let location: Value<v8_SourceLocation> = Rc::new(RefCell::new(
            location.unwrap_or(({ v8_SourceLocation::Current(None) })),
        ));
        ({
            (*(*self).upgrade().deref())
                .PostIdleTaskImpl((*task.borrow_mut()).take(), location.as_pointer())
        });
    }
    fn IdleTasksEnabled(&self) -> bool;
    fn NonNestableTasksEnabled(&self) -> bool {
        return false;
    }
    fn NonNestableDelayedTasksEnabled(&self) -> bool {
        return false;
    }
    fn PostTaskImpl(&self, task: Option<Value<v8_Task>>, location: Ptr<v8_SourceLocation>) {
        let task: Value<Option<Value<v8_Task>>> = Rc::new(RefCell::new(task));
    }
    fn PostNonNestableTaskImpl(
        &self,
        task: Option<Value<v8_Task>>,
        location: Ptr<v8_SourceLocation>,
    ) {
        let task: Value<Option<Value<v8_Task>>> = Rc::new(RefCell::new(task));
    }
    fn PostDelayedTaskImpl(
        &self,
        task: Option<Value<v8_Task>>,
        delay_in_seconds: f64,
        location: Ptr<v8_SourceLocation>,
    ) {
        let task: Value<Option<Value<v8_Task>>> = Rc::new(RefCell::new(task));
        let delay_in_seconds: Value<f64> = Rc::new(RefCell::new(delay_in_seconds));
    }
    fn PostNonNestableDelayedTaskImpl(
        &self,
        task: Option<Value<v8_Task>>,
        delay_in_seconds: f64,
        location: Ptr<v8_SourceLocation>,
    ) {
        let task: Value<Option<Value<v8_Task>>> = Rc::new(RefCell::new(task));
        let delay_in_seconds: Value<f64> = Rc::new(RefCell::new(delay_in_seconds));
    }
    fn PostIdleTaskImpl(&self, task: Option<Value<v8_IdleTask>>, location: Ptr<v8_SourceLocation>) {
        let task: Value<Option<Value<v8_IdleTask>>> = Rc::new(RefCell::new(task));
    }
}
pub trait v8_JobDelegate {
    fn ShouldYield(&self) -> bool;
    fn NotifyConcurrencyIncrease(&self);
    fn GetTaskId(&self) -> u8;
    fn IsJoiningThread(&self) -> bool;
}
pub trait v8_JobHandle {
    fn NotifyConcurrencyIncrease(&self);
    fn Join(&self);
    fn Cancel(&self);
    fn CancelAndDetach(&self);
    fn IsActive(&self) -> bool;
    fn IsValid(&self) -> bool;
    fn UpdatePriorityEnabled(&self) -> bool {
        return false;
    }
    fn UpdatePriority(&self, new_priority: v8_TaskPriority) {
        let new_priority: Value<v8_TaskPriority> = Rc::new(RefCell::new(new_priority));
    }
}
pub trait v8_JobTask {
    fn Run(&self, delegate: PtrDyn<dyn v8_JobDelegate>);
    fn GetMaxConcurrency(&self, worker_count: usize) -> usize;
}
pub trait v8_ScopedBoostablePriority {
    fn BoostPriority(&self) -> bool;
    fn Reset(&self);
}
pub type v8_BlockingType = i32;
pub const v8_BlockingType_kMayBlock: v8_BlockingType = 0;
pub const v8_BlockingType_kWillBlock: v8_BlockingType = 1;
#[derive(Default)]
pub struct v8_ScopedBlockingCall {}
impl Clone for v8_ScopedBlockingCall {
    fn clone(&self) -> Self {
        let __this: Value<v8_ScopedBlockingCall> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_ScopedBlockingCall> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_ScopedBlockingCall {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub trait v8_ConvertableToTraceFormat {
    fn AppendAsTraceFormat(&self, out: Ptr<Vec<u8>>);
}
pub trait v8_TracingController_TraceStateObserver {
    fn OnTraceEnabled(&self);
    fn OnTraceDisabled(&self);
}
#[derive(Default)]
pub struct v8_TracingController {}
impl Clone for v8_TracingController {
    fn clone(&self) -> Self {
        let __this: Value<v8_TracingController> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_TracingController> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_TracingController {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_SharedMemoryHandle {
    handle_: Value<u32>,
}
impl v8_SharedMemoryHandle {
    pub fn FromPlatformHandle(handle: u32) -> v8_SharedMemoryHandle {
        let handle: Value<u32> = Rc::new(RefCell::new(handle));
        return v8_SharedMemoryHandle::v8_SharedMemoryHandle1({ (*handle.borrow()) });
    }
    fn v8_SharedMemoryHandle1(handle: u32) -> Self {
        let handle: Value<u32> = Rc::new(RefCell::new(handle));
        let __this: Value<v8_SharedMemoryHandle> = Rc::new(RefCell::new(Self {
            handle_: Rc::new(RefCell::new((*handle.borrow()))),
        }));
        let this: Ptr<v8_SharedMemoryHandle> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_SharedMemoryHandle {
    fn clone(&self) -> Self {
        let __this: Value<v8_SharedMemoryHandle> = Rc::new(RefCell::new(Self {
            handle_: Rc::new(RefCell::new((*self.handle_.borrow()))),
        }));
        let this: Ptr<v8_SharedMemoryHandle> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_SharedMemoryHandle {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.handle_.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            handle_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn SharedMemoryHandleFromMachMemoryEntry_123(handle: u32) -> v8_SharedMemoryHandle {
    let handle: Value<u32> = Rc::new(RefCell::new(handle));
    return ({ v8_SharedMemoryHandle::FromPlatformHandle((*handle.borrow())) });
}
pub fn MachMemoryEntryFromSharedMemoryHandle_124(handle: v8_SharedMemoryHandle) -> u32 {
    let handle: Value<v8_SharedMemoryHandle> = Rc::new(RefCell::new(handle));
    return ({ v8_SharedMemoryHandleImpl::GetPlatformHandle(&handle.as_pointer()) });
}
thread_local!(
    pub static kInvalidSharedMemoryHandle_125: Value<std_optional_v8_SharedMemoryHandle_> =
        Rc::new(RefCell::new(
            std_optional_v8_SharedMemoryHandle_::std_optional_v8_SharedMemoryHandle_1({
                (*nullopt_126.with(Value::clone).borrow()).clone()
            }),
        ));
);
pub type v8_PageAllocator_Permission = u32;
pub const v8_PageAllocator_Permission_kNoAccess: v8_PageAllocator_Permission = 0;
pub const v8_PageAllocator_Permission_kRead: v8_PageAllocator_Permission = 1;
pub const v8_PageAllocator_Permission_kReadWrite: v8_PageAllocator_Permission = 2;
pub const v8_PageAllocator_Permission_kReadWriteExecute: v8_PageAllocator_Permission = 3;
pub const v8_PageAllocator_Permission_kReadExecute: v8_PageAllocator_Permission = 4;
pub const v8_PageAllocator_Permission_kNoAccessWillJitLater: v8_PageAllocator_Permission = 5;
pub trait v8_PageAllocator {
    fn AllocatePageSize(&self) -> usize;
    fn CommitPageSize(&self) -> usize;
    fn SetRandomMmapSeed(&self, seed: i64);
    fn GetRandomMmapAddr(&self) -> AnyPtr;
    fn AllocatePages_AnyPtr_usize_usize_v8_PageAllocator_Permission(
        &self,
        address: AnyPtr,
        length: usize,
        alignment: usize,
        permissions: v8_PageAllocator_Permission,
    ) -> AnyPtr;
    fn AllocatePages_usize_usize_v8_PageAllocator_Permission_v8_PageAllocator_AllocationHint(
        &self,
        length: usize,
        alignment: usize,
        permissions: v8_PageAllocator_Permission,
        hint: v8_PageAllocator_AllocationHint,
    ) -> AnyPtr {
        let length: Value<usize> = Rc::new(RefCell::new(length));
        let alignment: Value<usize> = Rc::new(RefCell::new(alignment));
        let permissions: Value<v8_PageAllocator_Permission> = Rc::new(RefCell::new(permissions));
        let hint: Value<v8_PageAllocator_AllocationHint> = Rc::new(RefCell::new(hint));
        return ({
            self.AllocatePages_AnyPtr_usize_usize_v8_PageAllocator_Permission(
                ({ v8_PageAllocator_AllocationHintImpl::Address(&hint.as_pointer()) }),
                (*length.borrow()),
                (*alignment.borrow()),
                (*permissions.borrow()),
            )
        });
    }
    fn ResizeAllocationAt(
        &self,
        address: AnyPtr,
        old_length: usize,
        new_length: usize,
        permissions: v8_PageAllocator_Permission,
    ) -> bool {
        let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
        let old_length: Value<usize> = Rc::new(RefCell::new(old_length));
        let new_length: Value<usize> = Rc::new(RefCell::new(new_length));
        let permissions: Value<v8_PageAllocator_Permission> = Rc::new(RefCell::new(permissions));
        return false;
    }
    fn FreePages(&self, address: AnyPtr, length: usize) -> bool;
    fn ReleasePages(&self, address: AnyPtr, length: usize, new_length: usize) -> bool;
    fn SetPermissions(
        &self,
        address: AnyPtr,
        length: usize,
        permissions: v8_PageAllocator_Permission,
    ) -> bool;
    fn RecommitPages(
        &self,
        address: AnyPtr,
        length: usize,
        permissions: v8_PageAllocator_Permission,
    ) -> bool {
        let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
        let length: Value<usize> = Rc::new(RefCell::new(length));
        let permissions: Value<v8_PageAllocator_Permission> = Rc::new(RefCell::new(permissions));
        return false;
    }
    fn DiscardSystemPages(&self, address: AnyPtr, size: usize) -> bool {
        let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        return true;
    }
    fn DecommitPages(&self, address: AnyPtr, size: usize) -> bool;
    fn SealPages(&self, address: AnyPtr, length: usize) -> bool {
        let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
        let length: Value<usize> = Rc::new(RefCell::new(length));
        return false;
    }
    fn ReserveForSharedMemoryMapping(&self, address: AnyPtr, size: usize) -> bool {
        let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        return false;
    }
    fn AllocateSharedPages(
        &self,
        length: usize,
        original_address: AnyPtr,
    ) -> Option<Value<v8_PageAllocator_SharedMemory>> {
        let length: Value<usize> = Rc::new(RefCell::new(length));
        let original_address: Value<AnyPtr> = Rc::new(RefCell::new(original_address));
        return None;
    }
    fn CanAllocateSharedPages(&self) -> bool {
        return false;
    }
}
pub trait v8_Allocator {
    fn Allocate(&self, size: usize) -> AnyPtr;
    fn AllocateUninitialized(&self, size: usize) -> AnyPtr;
    fn AllocateUninitializedOrCrash(&self, size: usize) -> AnyPtr;
    fn Free(&self, ptr: AnyPtr);
}
pub type v8_ThreadIsolatedAllocator_Type = i32;
pub const v8_ThreadIsolatedAllocator_Type_kPkey: v8_ThreadIsolatedAllocator_Type = 0;
pub trait v8_ThreadIsolatedAllocator {
    fn Allocate(&self, size: usize) -> AnyPtr;
    fn Free(&self, object: AnyPtr);
    fn Type(&self) -> v8_ThreadIsolatedAllocator_Type;
    fn Pkey(&self) -> i32 {
        return -1_i32;
    }
}
pub type v8_PagePermissions = i32;
pub const v8_PagePermissions_kNoAccess: v8_PagePermissions = 0;
pub const v8_PagePermissions_kRead: v8_PagePermissions = 1;
pub const v8_PagePermissions_kWrite: v8_PagePermissions = 2;
pub const v8_PagePermissions_kExecute: v8_PagePermissions = 4;
pub const v8_PagePermissions_kReadWrite: v8_PagePermissions = 3;
pub const v8_PagePermissions_kReadExecute: v8_PagePermissions = 5;
pub const v8_PagePermissions_kWriteExecute: v8_PagePermissions = 6;
pub const v8_PagePermissions_kReadWriteExecute: v8_PagePermissions = 7;
pub fn operator_bitor_127(lhs: v8_PagePermissions, rhs: v8_PagePermissions) -> v8_PagePermissions {
    let lhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(lhs));
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as i32) | ((*rhs.borrow()) as i32)) as v8_PagePermissions);
}
pub fn operator_bitand_128(lhs: v8_PagePermissions, rhs: v8_PagePermissions) -> v8_PagePermissions {
    let lhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(lhs));
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as i32) & ((*rhs.borrow()) as i32)) as v8_PagePermissions);
}
pub fn operator_bitor_assign_129(
    lhs: Ptr<v8_PagePermissions>,
    rhs: v8_PagePermissions,
) -> Ptr<v8_PagePermissions> {
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    let __rhs = ({
        let _lhs: v8_PagePermissions = (lhs.read());
        let _rhs: v8_PagePermissions = (*rhs.borrow());
        operator_bitor_127(_lhs, _rhs)
    });
    lhs.write(__rhs);
    return (lhs).clone();
}
pub fn IsSubset_130(lhs: v8_PagePermissions, rhs: v8_PagePermissions) -> bool {
    let lhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(lhs));
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    return (({
        let _lhs: v8_PagePermissions = (*lhs.borrow());
        operator_bitand_128(_lhs, (*rhs.borrow()))
    }) == (*lhs.borrow()));
}
pub trait v8_VirtualAddressSpace {
    fn page_size(&self) -> usize {
        return (*(*(*self).upgrade().deref()).page_size_.borrow());
    }
    fn allocation_granularity(&self) -> usize {
        return (*(*(*self).upgrade().deref())
            .allocation_granularity_
            .borrow());
    }
    fn base(&self) -> u64 {
        return (*(*(*self).upgrade().deref()).base_.borrow());
    }
    fn size(&self) -> usize {
        return (*(*(*self).upgrade().deref()).size_.borrow());
    }
    fn max_page_permissions(&self) -> v8_PagePermissions {
        return (*(*(*self).upgrade().deref()).max_page_permissions_.borrow());
    }
    fn Contains(&self, address: u64) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        return ((*address.borrow()) >= ({ v8_VirtualAddressSpaceImpl::base(self) }))
            && ((*address.borrow())
                < ({ v8_VirtualAddressSpaceImpl::base(self) })
                    .wrapping_add((({ v8_VirtualAddressSpaceImpl::size(self) }) as u64)));
    }
    fn SetRandomSeed(&self, seed: i64);
    fn RandomPageAddress(&self) -> u64;
    fn AllocatePages(
        &self,
        hint: u64,
        size: usize,
        alignment: usize,
        permissions: v8_PagePermissions,
    ) -> u64;
    fn FreePages(&self, address: u64, size: usize);
    fn SetPagePermissions(
        &self,
        address: u64,
        size: usize,
        permissions: v8_PagePermissions,
    ) -> bool;
    fn AllocateGuardRegion(&self, address: u64, size: usize) -> bool;
    fn FreeGuardRegion(&self, address: u64, size: usize);
    fn AllocateSharedPages_u64_usize_v8_PagePermissions_v8_SharedMemoryHandle_u64(
        &self,
        hint: u64,
        size: usize,
        permissions: v8_PagePermissions,
        handle: v8_SharedMemoryHandle,
        offset: u64,
    ) -> u64;
    fn AllocateSharedPages_u64_usize_v8_PagePermissions_std_optional_v8_SharedMemoryHandle__u64(
        &self,
        hint: u64,
        size: usize,
        permissions: v8_PagePermissions,
        handle: std_optional_v8_SharedMemoryHandle_,
        offset: u64,
    ) -> u64 {
        let hint: Value<u64> = Rc::new(RefCell::new(hint));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let permissions: Value<v8_PagePermissions> = Rc::new(RefCell::new(permissions));
        let handle: Value<std_optional_v8_SharedMemoryHandle_> = Rc::new(RefCell::new(handle));
        let offset: Value<u64> = Rc::new(RefCell::new(offset));
        return ({
            (*(*self).upgrade().deref())
                .AllocateSharedPages_u64_usize_v8_PagePermissions_v8_SharedMemoryHandle_u64(
                    (*hint.borrow()),
                    (*size.borrow()),
                    (*permissions.borrow()),
                    (*(*handle.borrow()).upgrade().deref()).clone(),
                    (*offset.borrow()),
                )
        });
    }
    fn FreeSharedPages(&self, address: u64, size: usize);
    fn ActiveMemoryProtectionKey(&self) -> std_optional_int_;
    fn CanAllocateSubspaces(&self) -> bool;
    fn AllocateSubspace(
        &self,
        hint: u64,
        size: usize,
        alignment: usize,
        max_page_permissions: v8_PagePermissions,
        key: Option<std_optional_int_>,
        handle: Option<std_optional_v8_SharedMemoryHandle_>,
    ) -> Option<Value<v8_VirtualAddressSpace>>;
    fn RecommitPages(&self, address: u64, size: usize, permissions: v8_PagePermissions) -> bool;
    fn DiscardSystemPages(&self, address: u64, size: usize) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        return true;
    }
    fn DecommitPages(&self, address: u64, size: usize) -> bool;
    fn SetName(&self, name: Ptr<Vec<u8>>) -> bool {
        return false;
    }
}
#[derive(Default)]
pub struct v8_HighAllocationThroughputObserver {}
impl Clone for v8_HighAllocationThroughputObserver {
    fn clone(&self) -> Self {
        let __this: Value<v8_HighAllocationThroughputObserver> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_HighAllocationThroughputObserver> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_HighAllocationThroughputObserver {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub trait v8_Platform {
    fn GetPageAllocator(&self) -> PtrDyn<dyn v8_PageAllocator> {
        return Ptr::<dyn v8_PageAllocator>::null();
    }
    fn GetThreadIsolatedAllocator(&self) -> PtrDyn<dyn v8_ThreadIsolatedAllocator> {
        return Ptr::<dyn v8_ThreadIsolatedAllocator>::null();
    }
    fn GetZeroSegmentSize(&self) -> usize {
        return 0_usize;
    }
    fn OnCriticalMemoryPressure(&self) {}
    fn NumberOfWorkerThreads(&self) -> i32;
    fn GetForegroundTaskRunner_pmutv8_Isolate(
        &self,
        isolate: Ptr<v8_Isolate>,
    ) -> std_shared_ptr_v8_TaskRunner_ {
        let isolate: Value<Ptr<v8_Isolate>> = Rc::new(RefCell::new(isolate));
        return ({
            (*(*self).upgrade().deref()).GetForegroundTaskRunner_pmutv8_Isolate_v8_TaskPriority(
                (*isolate.borrow()).clone(),
                v8_TaskPriority_kUserBlocking,
            )
        });
    }
    fn GetForegroundTaskRunner_pmutv8_Isolate_v8_TaskPriority(
        &self,
        isolate: Ptr<v8_Isolate>,
        priority: v8_TaskPriority,
    ) -> std_shared_ptr_v8_TaskRunner_;
    fn CallOnWorkerThread(
        &self,
        task: Option<Value<v8_Task>>,
        location: Option<v8_SourceLocation>,
    ) {
        let task: Value<Option<Value<v8_Task>>> = Rc::new(RefCell::new(task));
        let location: Value<v8_SourceLocation> = Rc::new(RefCell::new(
            location.unwrap_or(({ v8_SourceLocation::Current(None) })),
        ));
        ({
            (*(*self).upgrade().deref()).PostTaskOnWorkerThreadImpl(
                v8_TaskPriority_kUserVisible,
                (*task.borrow_mut()).take(),
                location.as_pointer(),
            )
        });
    }
    fn CallBlockingTaskOnWorkerThread(
        &self,
        task: Option<Value<v8_Task>>,
        location: Option<v8_SourceLocation>,
    ) {
        let task: Value<Option<Value<v8_Task>>> = Rc::new(RefCell::new(task));
        let location: Value<v8_SourceLocation> = Rc::new(RefCell::new(
            location.unwrap_or(({ v8_SourceLocation::Current(None) })),
        ));
        ({
            (*(*self).upgrade().deref()).PostTaskOnWorkerThreadImpl(
                v8_TaskPriority_kUserBlocking,
                (*task.borrow_mut()).take(),
                location.as_pointer(),
            )
        });
    }
    fn CallLowPriorityTaskOnWorkerThread(
        &self,
        task: Option<Value<v8_Task>>,
        location: Option<v8_SourceLocation>,
    ) {
        let task: Value<Option<Value<v8_Task>>> = Rc::new(RefCell::new(task));
        let location: Value<v8_SourceLocation> = Rc::new(RefCell::new(
            location.unwrap_or(({ v8_SourceLocation::Current(None) })),
        ));
        ({
            (*(*self).upgrade().deref()).PostTaskOnWorkerThreadImpl(
                v8_TaskPriority_kBestEffort,
                (*task.borrow_mut()).take(),
                location.as_pointer(),
            )
        });
    }
    fn CallDelayedOnWorkerThread(
        &self,
        task: Option<Value<v8_Task>>,
        delay_in_seconds: f64,
        location: Option<v8_SourceLocation>,
    ) {
        let task: Value<Option<Value<v8_Task>>> = Rc::new(RefCell::new(task));
        let delay_in_seconds: Value<f64> = Rc::new(RefCell::new(delay_in_seconds));
        let location: Value<v8_SourceLocation> = Rc::new(RefCell::new(
            location.unwrap_or(({ v8_SourceLocation::Current(None) })),
        ));
        ({
            (*(*self).upgrade().deref()).PostDelayedTaskOnWorkerThreadImpl(
                v8_TaskPriority_kUserVisible,
                (*task.borrow_mut()).take(),
                (*delay_in_seconds.borrow()),
                location.as_pointer(),
            )
        });
    }
    fn PostTaskOnWorkerThread(
        &self,
        priority: v8_TaskPriority,
        task: Option<Value<v8_Task>>,
        location: Option<v8_SourceLocation>,
    ) {
        let priority: Value<v8_TaskPriority> = Rc::new(RefCell::new(priority));
        let task: Value<Option<Value<v8_Task>>> = Rc::new(RefCell::new(task));
        let location: Value<v8_SourceLocation> = Rc::new(RefCell::new(
            location.unwrap_or(({ v8_SourceLocation::Current(None) })),
        ));
        ({
            (*(*self).upgrade().deref()).PostTaskOnWorkerThreadImpl(
                (*priority.borrow()),
                (*task.borrow_mut()).take(),
                location.as_pointer(),
            )
        });
    }
    fn PostDelayedTaskOnWorkerThread(
        &self,
        priority: v8_TaskPriority,
        task: Option<Value<v8_Task>>,
        delay_in_seconds: f64,
        location: Option<v8_SourceLocation>,
    ) {
        let priority: Value<v8_TaskPriority> = Rc::new(RefCell::new(priority));
        let task: Value<Option<Value<v8_Task>>> = Rc::new(RefCell::new(task));
        let delay_in_seconds: Value<f64> = Rc::new(RefCell::new(delay_in_seconds));
        let location: Value<v8_SourceLocation> = Rc::new(RefCell::new(
            location.unwrap_or(({ v8_SourceLocation::Current(None) })),
        ));
        ({
            (*(*self).upgrade().deref()).PostDelayedTaskOnWorkerThreadImpl(
                (*priority.borrow()),
                (*task.borrow_mut()).take(),
                (*delay_in_seconds.borrow()),
                location.as_pointer(),
            )
        });
    }
    fn IdleTasksEnabled(&self, isolate: Ptr<v8_Isolate>) -> bool {
        let isolate: Value<Ptr<v8_Isolate>> = Rc::new(RefCell::new(isolate));
        return false;
    }
    fn PostJob(
        &self,
        priority: v8_TaskPriority,
        job_task: Option<Value<v8_JobTask>>,
        location: Option<v8_SourceLocation>,
    ) -> Option<Value<v8_JobHandle>> {
        let priority: Value<v8_TaskPriority> = Rc::new(RefCell::new(priority));
        let job_task: Value<Option<Value<v8_JobTask>>> = Rc::new(RefCell::new(job_task));
        let location: Value<v8_SourceLocation> = Rc::new(RefCell::new(
            location.unwrap_or(({ v8_SourceLocation::Current(None) })),
        ));
        let handle: Value<Option<Value<v8_JobHandle>>> = Rc::new(RefCell::new(
            ({
                v8_PlatformImpl::CreateJob(
                    self,
                    (*priority.borrow()),
                    (*job_task.borrow_mut()).take(),
                    Some((*location.borrow()).clone()),
                )
            }),
        ));
        ({ (*(*handle.borrow()).as_ref().unwrap().borrow()).NotifyConcurrencyIncrease() });
        return (*handle.borrow_mut()).take();
    }
    fn CreateJob(
        &self,
        priority: v8_TaskPriority,
        job_task: Option<Value<v8_JobTask>>,
        location: Option<v8_SourceLocation>,
    ) -> Option<Value<v8_JobHandle>> {
        let priority: Value<v8_TaskPriority> = Rc::new(RefCell::new(priority));
        let job_task: Value<Option<Value<v8_JobTask>>> = Rc::new(RefCell::new(job_task));
        let location: Value<v8_SourceLocation> = Rc::new(RefCell::new(
            location.unwrap_or(({ v8_SourceLocation::Current(None) })),
        ));
        return ({
            (*(*self).upgrade().deref()).CreateJobImpl(
                (*priority.borrow()),
                (*job_task.borrow_mut()).take(),
                location.as_pointer(),
            )
        });
    }
    fn CreateBoostablePriorityScope(&self) -> Option<Value<v8_ScopedBoostablePriority>> {
        return std_unique_ptr_v8_ScopedBoostablePriority__std_default_delete_v8_ScopedBoostablePriority__ :: std_unique_ptr_v8_ScopedBoostablePriority__std_default_delete_v8_ScopedBoostablePriority__ ( {  Default::default()   } , )   ;
    }
    fn CreateBlockingScope(
        &self,
        blocking_type: v8_BlockingType,
    ) -> Option<Value<v8_ScopedBlockingCall>> {
        let blocking_type: Value<v8_BlockingType> = Rc::new(RefCell::new(blocking_type));
        return std_unique_ptr_v8_ScopedBlockingCall__std_default_delete_v8_ScopedBlockingCall__ :: std_unique_ptr_v8_ScopedBlockingCall__std_default_delete_v8_ScopedBlockingCall__ ( {  Default::default()   } , )   ;
    }
    fn MonotonicallyIncreasingTime(&self) -> f64;
    fn CurrentClockTimeMilliseconds(&self) -> i64 {
        return (({ floor_131(({ self.CurrentClockTimeMillis() })) }) as i64);
    }
    fn CurrentClockTimeMillis(&self) -> f64;
    fn CurrentClockTimeMillisecondsHighResolution(&self) -> f64 {
        return ({ self.CurrentClockTimeMillis() });
    }
    fn GetStackTracePrinter(&self) -> FnPtr<fn()> {
        return FnPtr::<fn()>::null();
    }
    fn GetTracingController(&self) -> Ptr<v8_TracingController>;
    fn DumpWithoutCrashing(&self) {}
    fn GetHighAllocationThroughputObserver(&self) -> Ptr<v8_HighAllocationThroughputObserver> {
        thread_local!(
            static default_observer_132: Value<v8_HighAllocationThroughputObserver> = Rc::new(
                RefCell::new(<v8_HighAllocationThroughputObserver>::default()),
            );
        );
        return (default_observer_132.with(Value::clone).as_pointer());
    }
    fn CreateJobImpl(
        &self,
        priority: v8_TaskPriority,
        job_task: Option<Value<v8_JobTask>>,
        location: Ptr<v8_SourceLocation>,
    ) -> Option<Value<v8_JobHandle>>;
    fn PostTaskOnWorkerThreadImpl(
        &self,
        priority: v8_TaskPriority,
        task: Option<Value<v8_Task>>,
        location: Ptr<v8_SourceLocation>,
    );
    fn PostDelayedTaskOnWorkerThreadImpl(
        &self,
        priority: v8_TaskPriority,
        task: Option<Value<v8_Task>>,
        delay_in_seconds: f64,
        location: Ptr<v8_SourceLocation>,
    );
}
pub type anon_133 = u8;
pub const anon_133_ONCE_STATE_UNINITIALIZED: anon_133 = 0;
pub const anon_133_ONCE_STATE_EXECUTING_FUNCTION: anon_133 = 1;
pub const anon_133_ONCE_STATE_DONE: anon_133 = 2;
pub fn CallOnce_134(once: Ptr<std_atomic_unsigned_char_>, init_func: std_function_void____) {
    let once: Value<Ptr<std_atomic_unsigned_char_>> = Rc::new(RefCell::new(once));
    let init_func: Value<std_function_void____> = Rc::new(RefCell::new(init_func));
    if ((({ (*(*once.borrow()).upgrade().deref()).load_const(Some(2)) }) as i32)
        != (anon_133_ONCE_STATE_DONE as i32))
    {
        ({ CallOnceImpl_135((*once.borrow()).clone(), (*init_func.borrow()).clone()) });
    }
}
#[derive(Default)]
pub struct v8_base_ThreadSafeInitOnceTrait {}
impl Clone for v8_base_ThreadSafeInitOnceTrait {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_ThreadSafeInitOnceTrait> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_ThreadSafeInitOnceTrait> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_ThreadSafeInitOnceTrait {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_SingleThreadInitOnceTrait {}
impl Clone for v8_base_SingleThreadInitOnceTrait {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_SingleThreadInitOnceTrait> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_SingleThreadInitOnceTrait> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_SingleThreadInitOnceTrait {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_Semaphore {
    native_handle_: Value<Ptr<dispatch_semaphore_s>>,
}
impl v8_base_Semaphore {}
impl ByteRepr for v8_base_Semaphore {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.native_handle_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            native_handle_: Rc::new(RefCell::new(<Ptr<dispatch_semaphore_s>>::from_bytes(
                &buf[0..8],
            ))),
        }
    }
}
thread_local!();
pub type v8_base_OS_MemoryPermission = i32;
pub const v8_base_OS_MemoryPermission_kNoAccess: v8_base_OS_MemoryPermission = 0;
pub const v8_base_OS_MemoryPermission_kRead: v8_base_OS_MemoryPermission = 1;
pub const v8_base_OS_MemoryPermission_kReadWrite: v8_base_OS_MemoryPermission = 2;
pub const v8_base_OS_MemoryPermission_kReadWriteExecute: v8_base_OS_MemoryPermission = 3;
pub const v8_base_OS_MemoryPermission_kReadExecute: v8_base_OS_MemoryPermission = 4;
pub const v8_base_OS_MemoryPermission_kNoAccessWillJitLater: v8_base_OS_MemoryPermission = 5;
thread_local!(
    pub static kStackWalkError_137: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static kStackWalkMaxNameLen_138: Value<i32> = Rc::new(RefCell::new(256));
);
thread_local!(
    pub static kStackWalkMaxTextLen_139: Value<i32> = Rc::new(RefCell::new(256));
);
thread_local!(
    static msPerSecond_140: Value<i32> = Rc::new(RefCell::new(1000));
);
#[derive()]
pub struct v8_base_OS_StackFrame {
    pub address: Value<AnyPtr>,
    pub text: Value<Box<[u8]>>,
}
impl Clone for v8_base_OS_StackFrame {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_OS_StackFrame> = Rc::new(RefCell::new(Self {
            address: Rc::new(RefCell::new((*self.address.borrow()).clone())),
            text: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 256, _>(
                |__i: usize| (*self.text.borrow())[(__i) as usize],
            )))),
        }));
        let this: Ptr<v8_base_OS_StackFrame> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_OS_StackFrame {
    fn default() -> Self {
        v8_base_OS_StackFrame {
            address: Rc::new(RefCell::new(AnyPtr::default())),
            text: Rc::new(RefCell::new(
                (0..256).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
        }
    }
}
impl ByteRepr for v8_base_OS_StackFrame {
    fn byte_size() -> usize {
        264
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.address.borrow()).to_bytes(&mut buf[0..8]);
        (*self.text.borrow()).to_bytes(&mut buf[8..264]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            address: Rc::new(RefCell::new(<AnyPtr>::from_bytes(&buf[0..8]))),
            text: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[8..264]))),
        }
    }
}
pub type v8_base_OS_MemoryMappedFile_FileMode = i32;
pub const v8_base_OS_MemoryMappedFile_FileMode_kReadOnly: v8_base_OS_MemoryMappedFile_FileMode = 0;
pub const v8_base_OS_MemoryMappedFile_FileMode_kReadWrite: v8_base_OS_MemoryMappedFile_FileMode = 1;
pub trait v8_base_OS_MemoryMappedFile {
    fn memory(&self) -> AnyPtr;
    fn size(&self) -> usize;
}
#[derive(Default)]
pub struct v8_base_OS_SharedLibraryAddress {
    pub library_path: Value<Vec<u8>>,
    pub start: Value<u64>,
    pub end: Value<u64>,
    pub aslr_slide: Value<i64>,
}
impl v8_base_OS_SharedLibraryAddress {
    pub fn v8_base_OS_SharedLibraryAddress1(
        library_path: Ptr<Vec<u8>>,
        start: u64,
        end: u64,
    ) -> Self {
        let start: Value<u64> = Rc::new(RefCell::new(start));
        let end: Value<u64> = Rc::new(RefCell::new(end));
        let __this: Value<v8_base_OS_SharedLibraryAddress> = Rc::new(RefCell::new(Self {
            library_path: Rc::new(RefCell::new((*library_path.upgrade().deref()).clone())),
            start: Rc::new(RefCell::new((*start.borrow()))),
            end: Rc::new(RefCell::new((*end.borrow()))),
            aslr_slide: Rc::new(RefCell::new(0_i64)),
        }));
        let this: Ptr<v8_base_OS_SharedLibraryAddress> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_base_OS_SharedLibraryAddress2(
        library_path: Ptr<Vec<u8>>,
        start: u64,
        end: u64,
        aslr_slide: i64,
    ) -> Self {
        let start: Value<u64> = Rc::new(RefCell::new(start));
        let end: Value<u64> = Rc::new(RefCell::new(end));
        let aslr_slide: Value<i64> = Rc::new(RefCell::new(aslr_slide));
        let __this: Value<v8_base_OS_SharedLibraryAddress> = Rc::new(RefCell::new(Self {
            library_path: Rc::new(RefCell::new((*library_path.upgrade().deref()).clone())),
            start: Rc::new(RefCell::new((*start.borrow()))),
            end: Rc::new(RefCell::new((*end.borrow()))),
            aslr_slide: Rc::new(RefCell::new((*aslr_slide.borrow()))),
        }));
        let this: Ptr<v8_base_OS_SharedLibraryAddress> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_OS_SharedLibraryAddress {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_OS_SharedLibraryAddress> = Rc::new(RefCell::new(Self {
            library_path: Rc::new(RefCell::new((*self.library_path.borrow()).clone())),
            start: Rc::new(RefCell::new((*self.start.borrow()))),
            end: Rc::new(RefCell::new((*self.end.borrow()))),
            aslr_slide: Rc::new(RefCell::new((*self.aslr_slide.borrow()))),
        }));
        let this: Ptr<v8_base_OS_SharedLibraryAddress> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_OS_SharedLibraryAddress {
    fn byte_size() -> usize {
        48
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.library_path.borrow()).to_bytes(&mut buf[0..24]);
        (*self.start.borrow()).to_bytes(&mut buf[24..32]);
        (*self.end.borrow()).to_bytes(&mut buf[32..40]);
        (*self.aslr_slide.borrow()).to_bytes(&mut buf[40..48]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            library_path: Rc::new(RefCell::new(<Vec<u8>>::from_bytes(&buf[0..24]))),
            start: Rc::new(RefCell::new(<u64>::from_bytes(&buf[24..32]))),
            end: Rc::new(RefCell::new(<u64>::from_bytes(&buf[32..40]))),
            aslr_slide: Rc::new(RefCell::new(<i64>::from_bytes(&buf[40..48]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_OS_MemoryRange {
    pub start: Value<u64>,
    pub end: Value<u64>,
}
impl Clone for v8_base_OS_MemoryRange {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_OS_MemoryRange> = Rc::new(RefCell::new(Self {
            start: Rc::new(RefCell::new((*self.start.borrow()))),
            end: Rc::new(RefCell::new((*self.end.borrow()))),
        }));
        let this: Ptr<v8_base_OS_MemoryRange> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_OS_MemoryRange {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.start.borrow()).to_bytes(&mut buf[0..8]);
        (*self.end.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            start: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
            end: Rc::new(RefCell::new(<u64>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_OS {}
impl v8_base_OS {
    pub fn IsRemapPageSupported() -> bool {
        return true;
    }
}
impl ByteRepr for v8_base_OS {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn EnsureConsoleOutput_141() {}
#[derive(Default)]
pub struct v8_base_AddressSpaceReservation {
    base_: Value<AnyPtr>,
    size_: Value<usize>,
}
impl v8_base_AddressSpaceReservation {
    fn v8_base_AddressSpaceReservation(base: AnyPtr, size: usize) -> Self {
        let base: Value<AnyPtr> = Rc::new(RefCell::new(base));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let __this: Value<v8_base_AddressSpaceReservation> = Rc::new(RefCell::new(Self {
            base_: Rc::new(RefCell::new((*base.borrow()).clone())),
            size_: Rc::new(RefCell::new((*size.borrow()))),
        }));
        let this: Ptr<v8_base_AddressSpaceReservation> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_AddressSpaceReservation {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_AddressSpaceReservation> = Rc::new(RefCell::new(Self {
            base_: Rc::new(RefCell::new((*self.base_.borrow()).clone())),
            size_: Rc::new(RefCell::new((*self.size_.borrow()))),
        }));
        let this: Ptr<v8_base_AddressSpaceReservation> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_AddressSpaceReservation {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.size_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_: Rc::new(RefCell::new(<AnyPtr>::from_bytes(&buf[0..8]))),
            size_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
        }
    }
}
pub type v8_base_Thread_Priority = i32;
pub const v8_base_Thread_Priority_kBestEffort: v8_base_Thread_Priority = 0;
pub const v8_base_Thread_Priority_kUserVisible: v8_base_Thread_Priority = 1;
pub const v8_base_Thread_Priority_kUserBlocking: v8_base_Thread_Priority = 2;
pub const v8_base_Thread_Priority_kDefault: v8_base_Thread_Priority = 3;
pub trait v8_base_Thread {
    fn StartSynchronously(&self) -> bool {
        (*(*(*self).upgrade().deref()).start_semaphore_.borrow_mut()) =
            Ptr::alloc(v8_base_Semaphore::v8_base_Semaphore({ 0 }));
        if !({ v8_base_ThreadImpl::Start(self) }) {
            return false;
        }
        ({
            v8_base_SemaphoreImpl::Wait(&(*(*(*self).upgrade().deref()).start_semaphore_.borrow()))
        });
        (*(*(*self).upgrade().deref()).start_semaphore_.borrow()).delete();
        (*(*(*self).upgrade().deref()).start_semaphore_.borrow_mut()) =
            Ptr::<v8_base_Semaphore>::null();
        return true;
    }
    fn name(&self) -> Ptr<u8> {
        return ((*(*self).upgrade().deref()).name_.as_pointer() as Ptr<u8>);
    }
    fn Run(&self);
    fn HasThreadLocal(key: i32) -> bool {
        let key: Value<i32> = Rc::new(RefCell::new(key));
        return !(({ v8_base_Thread::GetThreadLocal((*key.borrow())) }).is_null());
    }
    fn GetExistingThreadLocal(key: i32) -> AnyPtr {
        let key: Value<i32> = Rc::new(RefCell::new(key));
        return ({ v8_base_Thread::GetThreadLocal((*key.borrow())) });
    }
    fn data(&self) -> Ptr<v8_base_Thread_PlatformData> {
        return (*(*(*self).upgrade().deref()).data_.borrow()).clone();
    }
    fn priority(&self) -> v8_base_Thread_Priority {
        return (*(*(*self).upgrade().deref()).priority_.borrow());
    }
    fn NotifyStartedAndDispatch(&self) {
        if !(*self.start_semaphore_.borrow()).is_null() {
            ({ v8_base_SemaphoreImpl::Signal(&(*self.start_semaphore_.borrow())) });
        }
        ({ self.Dispatch() });
    }
    fn Dispatch(&self) {
        ({ self.Run() });
    }
}
#[derive(Default)]
struct v8_base_Stack_PreventNonDefaultParameters {}
impl Clone for v8_base_Stack_PreventNonDefaultParameters {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Stack_PreventNonDefaultParameters> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_Stack_PreventNonDefaultParameters> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_Stack_PreventNonDefaultParameters {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_Stack_StackSlot {
    pub value: Value<u64>,
}
impl v8_base_Stack_StackSlot {
    pub fn v8_base_Stack_StackSlot1(value: AnyPtr) -> Self {
        let value: Value<AnyPtr> = Rc::new(RefCell::new(value));
        let __this: Value<v8_base_Stack_StackSlot> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*value.borrow()).to_int())),
        }));
        let this: Ptr<v8_base_Stack_StackSlot> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_base_Stack_StackSlot2(value: u64) -> Self {
        let value: Value<u64> = Rc::new(RefCell::new(value));
        let __this: Value<v8_base_Stack_StackSlot> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*value.borrow()))),
        }));
        let this: Ptr<v8_base_Stack_StackSlot> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_Stack_StackSlot {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Stack_StackSlot> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        }));
        let this: Ptr<v8_base_Stack_StackSlot> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_Stack_StackSlot {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.value.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_Stack {}
impl v8_base_Stack {
    pub fn GetCurrentFrameAddress(
        _a0: Option<v8_base_Stack_PreventNonDefaultParameters>,
        frame_address: Option<AnyPtr>,
    ) -> v8_base_Stack_StackSlot {
        let _a0: Value<v8_base_Stack_PreventNonDefaultParameters> = Rc::new(RefCell::new(
            _a0.unwrap_or(<v8_base_Stack_PreventNonDefaultParameters>::default()),
        ));
        let frame_address: Value<AnyPtr> = Rc::new(RefCell::new(
            frame_address.unwrap_or(({ __builtin_frame_address_142(0_u32) })),
        ));
        return v8_base_Stack_StackSlot::v8_base_Stack_StackSlot1({
            (*frame_address.borrow()).clone()
        });
    }
    pub fn GetRealStackAddressForSlot(slot: v8_base_Stack_StackSlot) -> v8_base_Stack_StackSlot {
        let slot: Value<v8_base_Stack_StackSlot> = Rc::new(RefCell::new(slot));
        return (*slot.borrow()).clone();
    }
}
impl Clone for v8_base_Stack {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Stack> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_Stack> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_Stack {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn VSNPrintF_143(str: v8_base_Vector_char_, format: Ptr<u8>, args: VaList) -> i32 {
    let str: Value<v8_base_Vector_char_> = Rc::new(RefCell::new(str));
    let format: Value<Ptr<u8>> = Rc::new(RefCell::new(format));
    let args: Value<VaList> = Rc::new(RefCell::new(args));
    return ({
        let _str: Ptr<u8> = ({ v8_base_Vector_char_Impl::begin(&str.as_pointer()) });
        let _length: i32 = ({ v8_base_Vector_char_Impl::length(&str.as_pointer()) });
        v8_base_OS::VSNPrintF(
            _str,
            _length,
            (*format.borrow()).clone(),
            (*args.borrow()).clone(),
        )
    });
}
pub fn SNPrintF_144(str: v8_base_Vector_char_, format: Ptr<u8>, __args: &[VaArg]) -> i32 {
    let str: Value<v8_base_Vector_char_> = Rc::new(RefCell::new(str));
    let format: Value<Ptr<u8>> = Rc::new(RefCell::new(format));
    let args: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*args.borrow_mut()) = VaList::new(__args);
    let result: Value<i32> = Rc::new(RefCell::new(
        ({
            VSNPrintF_143(
                (*str.borrow()).clone(),
                (*format.borrow()).clone(),
                (*args.borrow()).clone(),
            )
        }),
    ));
    return (*result.borrow());
}
pub fn StrNCpy_145(dest: v8_base_Vector_char_, src: Ptr<u8>, n: usize) {
    let dest: Value<v8_base_Vector_char_> = Rc::new(RefCell::new(dest));
    let src: Value<Ptr<u8>> = Rc::new(RefCell::new(src));
    let n: Value<usize> = Rc::new(RefCell::new(n));
    ({
        let _dest: Ptr<u8> = ({ v8_base_Vector_char_Impl::begin(&dest.as_pointer()) });
        let _length: i32 = ({ v8_base_Vector_char_Impl::length(&dest.as_pointer()) });
        v8_base_OS::StrNCpy(_dest, _length, (*src.borrow()).clone(), (*n.borrow()))
    });
}
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_PageAllocator_AllocationHint;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_Isolate;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_base_Thread_PlatformData;
pub trait v8_SharedMemoryHandleImpl {
    fn GetPlatformHandle(&self) -> u32;
}
impl v8_SharedMemoryHandleImpl for Ptr<v8_SharedMemoryHandle> {
    fn GetPlatformHandle(&self) -> u32 {
        return (*(*(*self).upgrade().deref()).handle_.borrow());
    }
}
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
pub trait v8_base_AddressSpaceReservationImpl {
    fn base(&self) -> AnyPtr;
    fn size(&self) -> usize;
    fn Contains(&self, region_addr: AnyPtr, region_size: usize) -> bool;
    fn SetName(&self, name: Ptr<u8>) -> bool;
}
impl v8_base_AddressSpaceReservationImpl for Ptr<v8_base_AddressSpaceReservation> {
    fn base(&self) -> AnyPtr {
        return (*(*(*self).upgrade().deref()).base_.borrow()).clone();
    }
    fn size(&self) -> usize {
        return (*(*(*self).upgrade().deref()).size_.borrow());
    }
    fn Contains(&self, region_addr: AnyPtr, region_size: usize) -> bool {
        let region_addr: Value<AnyPtr> = Rc::new(RefCell::new(region_addr));
        let region_size: Value<usize> = Rc::new(RefCell::new(region_size));
        let base: Value<u64> = Rc::new(RefCell::new(
            (*(*(*self).upgrade().deref()).base_.borrow()).to_int(),
        ));
        let region_base: Value<u64> = Rc::new(RefCell::new((*region_addr.borrow()).to_int()));
        return ((*region_base.borrow()) >= (*base.borrow()))
            && (((*region_base.borrow()).wrapping_add(((*region_size.borrow()) as u64)))
                <= ((*base.borrow())
                    .wrapping_add(((*(*(*self).upgrade().deref()).size_.borrow()) as u64))));
    }
    fn SetName(&self, name: Ptr<u8>) -> bool {
        let name: Value<Ptr<u8>> = Rc::new(RefCell::new(name));
        return ({
            let _address: AnyPtr = (*(*(*self).upgrade().deref()).base_.borrow()).clone();
            let _size: usize = (*(*(*self).upgrade().deref()).size_.borrow());
            v8_base_OS::SetMemoryRegionName(_address, _size, (*name.borrow()).clone())
        });
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
pub trait v8_base_SemaphoreImpl {
    fn native_handle(&self) -> Ptr<Ptr<dispatch_semaphore_s>>;
    fn native_handle_const(&self) -> Ptr<Ptr<dispatch_semaphore_s>>;
}
impl v8_base_SemaphoreImpl for Ptr<v8_base_Semaphore> {
    fn native_handle(&self) -> Ptr<Ptr<dispatch_semaphore_s>> {
        return (*(*self).upgrade().deref()).native_handle_.as_pointer();
    }
    fn native_handle_const(&self) -> Ptr<Ptr<dispatch_semaphore_s>> {
        return (*(*self).upgrade().deref()).native_handle_.as_pointer();
    }
}
pub trait v8_base_Stack_StackSlotImpl {
    fn operator_void__(&self) -> AnyPtr;
    fn operator_uintptr_t(&self) -> u64;
}
impl v8_base_Stack_StackSlotImpl for Ptr<v8_base_Stack_StackSlot> {
    fn operator_void__(&self) -> AnyPtr {
        return (*(*(*self).upgrade().deref()).value.borrow()).reinterpret_cast::<::libc::c_void>();
    }
    fn operator_uintptr_t(&self) -> u64 {
        return (*(*(*self).upgrade().deref()).value.borrow());
    }
}
pub trait v8_base_Vector_char_Impl {
    fn length(&self) -> i32;
    fn begin(&self) -> Ptr<u8>;
}
impl v8_base_Vector_char_Impl for Ptr<v8_base_Vector_char_> {
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
    fn begin(&self) -> Ptr<u8> {
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
