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
pub fn bit_cast_35(source: Ptr<u64>) -> f64 {
    return ({ bit_cast_36((source).clone()) });
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
pub fn make_uint64_37(high: u32, low: u32) -> u64 {
    let high: Value<u32> = Rc::new(RefCell::new(high));
    let low: Value<u32> = Rc::new(RefCell::new(low));
    return (((*high.borrow()) as u64) << 32).wrapping_add(((*low.borrow()) as u64));
}
pub fn RoundDown_38(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    return ((*x.borrow()) & (-(*m.borrow()) as u64));
}
pub fn RoundUp_39(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    (&(0));
    return ({
        let _x: u64 = ((*x.borrow()).wrapping_add((((*m.borrow()) - 1_i64) as u64)) as u64);
        let _m: i64 = (*m.borrow());
        RoundDown_38(_x, _m)
    });
}
pub fn IsAligned_40(value: u64, alignment: u64) -> bool {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let alignment: Value<u64> = Rc::new(RefCell::new(alignment));
    return (((*value.borrow()) & ((*alignment.borrow()).wrapping_sub(1_u64))) == 0_u64);
}
pub fn CountLeadingZeros_41(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        if (32_u32 == 64_u32) {
            (({ __builtin_clzll_42(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((32_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros_43(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        if (64_u32 == 64_u32) {
            (({ __builtin_clzll_42(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((64_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros_44(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        if (64_u32 == 64_u32) {
            (({ __builtin_clzll_42((*value.borrow())) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((64_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros32_45(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_41((*value.borrow())) });
}
pub fn CountLeadingZeros64_46(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_43((*value.borrow())) });
}
pub fn CountTrailingZeros_47(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        (if (32_u32 == 64_u32) {
            ({ __builtin_ctzll_48(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros_49(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        (if (64_u32 == 64_u32) {
            ({ __builtin_ctzll_48(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros32_50(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_47((*value.borrow())) });
}
pub fn CountTrailingZeros64_51(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_49((*value.borrow())) });
}
pub fn RoundUpToPowerOfTwo32_52(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u32 << ((32_u32).wrapping_sub(({ CountLeadingZeros_41((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo64_53(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u64 << ((64_u32).wrapping_sub(({ CountLeadingZeros_43((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo_54(value: usize) -> usize {
    let value: Value<usize> = Rc::new(RefCell::new(value));
    if (::std::mem::size_of::<usize>() == ::std::mem::size_of::<u64>()) {
        return (({ RoundUpToPowerOfTwo64_53(((*value.borrow()) as u64)) }) as usize);
    } else {
        return (({ RoundUpToPowerOfTwo32_52(((*value.borrow()) as u32)) }) as usize);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn RoundDownToPowerOfTwo32_55(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    if ((*value.borrow()) > 2147483648_u32) {
        return 2147483648_u32;
    }
    let result: Value<u32> = Rc::new(RefCell::new(
        ({ RoundUpToPowerOfTwo32_52((*value.borrow())) }),
    ));
    if ((*result.borrow()) > (*value.borrow())) {
        (*result.borrow_mut()) >>= 1;
    }
    return (*result.borrow());
}
pub fn RotateRight32_56(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateLeft32_57(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateRight64_58(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn RotateLeft64_59(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn ClearLsb_60(value: i32) -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(value));
    return ((*value.borrow()) & ((*value.borrow()) - 1));
}
pub fn SignedAddOverflow32_61(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_sadd_overflow_62(_arg0, _arg1, _arg2)
    });
}
pub fn SignedSubOverflow32_63(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_ssub_overflow_64(_arg0, _arg1, _arg2)
    });
}
pub fn SignedMulOverflow32_65(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_smul_overflow_66(_arg0, _arg1, _arg2)
    });
}
pub fn SignedAddOverflow64_67(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_add_overflow_68(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedSubOverflow64_69(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_sub_overflow_70(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedMulOverflow64_71(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return {
        let (val, ovf) = (*lhs.borrow()).overflowing_mul((*rhs.borrow()));
        (*val.borrow()).write(val);
        ovf
    };
}
pub fn UnsignedAddOverflow32_72(lhs: u32, rhs: u32, val: Ptr<u32>) -> bool {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<u32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: u32 = (*lhs.borrow());
        let _arg1: u32 = (*rhs.borrow());
        let _arg2: Ptr<u32> = (*val.borrow()).clone();
        __builtin_uadd_overflow_73(_arg0, _arg1, _arg2)
    });
}
pub fn UnsignedDiv32_74(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedDiv64_75(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn UnsignedMod32_76(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedMod64_77(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn WraparoundAdd32_78(lhs: i32, rhs: i32) -> i32 {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as u32).wrapping_add(((*rhs.borrow()) as u32))) as i32);
}
pub fn WraparoundNeg32_79(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (-((*x.borrow()) as u32) as i32);
}
pub fn ByteReverse16_80(value: u16) -> u16 {
    let value: Value<u16> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse32_81(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse64_82(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn OverlappingWrites_83(dst: AnyPtr, src: AnyPtr, count: usize) {
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
pub fn OverlappingWrites_84(dst: AnyPtr, src: AnyPtr, count: usize) {
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
pub fn OverlappingWrites_85(dst: AnyPtr, src: AnyPtr, count: usize) {
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
pub fn SimdMemCopy_86(dst: AnyPtr, src: AnyPtr, count: usize) {
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
            (({ CountLeadingZeros_44(((*count.borrow()).wrapping_sub(1_usize) as u64)) }) as usize),
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
                    OverlappingWrites_83(
                        ((*dst_u.borrow()).clone() as Ptr<u8>).to_any(),
                        ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                        (*count.borrow()),
                    )
                });
                return;
            }
            __v if __v == 3 => {
                ({
                    OverlappingWrites_84(
                        ((*dst_u.borrow()).clone() as Ptr<u8>).to_any(),
                        ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                        (*count.borrow()),
                    )
                });
                return;
            }
            __v if __v == 4 => {
                ({
                    OverlappingWrites_85(
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
                            __builtin_neon_vld1q_v_87(
                                ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                                48,
                            )
                        });
                        let __result = (*__ret.borrow());
                        __result
                    }));
                    let __result = ({
                        __builtin_neon_vst1q_v_88(
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
                            __builtin_neon_vld1q_v_87(
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
                        __builtin_neon_vst1q_v_88(
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
                            __builtin_neon_vld1q_v_87(
                                ((*src_u.borrow()).clone() as Ptr<u8>).to_any(),
                                48,
                            )
                        });
                        let __result = (*__ret.borrow());
                        __result
                    }));
                    let __result = ({
                        __builtin_neon_vst1q_v_88(
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
                                __builtin_neon_vld1q_v_87(
                                    ((*src_u.borrow()).offset((*i.borrow()) as isize) as Ptr<u8>)
                                        .to_any(),
                                    48,
                                )
                            });
                            let __result = (*__ret.borrow());
                            __result
                        }));
                        let __result = ({
                            __builtin_neon_vst1q_v_88(
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
pub fn MemCopy_89(dest: AnyPtr, src: AnyPtr, size: usize) {
    let dest: Value<AnyPtr> = Rc::new(RefCell::new(dest));
    let src: Value<AnyPtr> = Rc::new(RefCell::new(src));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    ({
        SimdMemCopy_86(
            (*dest.borrow()).clone(),
            (*src.borrow()).clone(),
            (*size.borrow()),
        )
    });
}
pub fn MemMove_90(dest: AnyPtr, src: AnyPtr, size: usize) {
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
    pub static kCompressionFactor_91: Value<u32> = Rc::new(RefCell::new(1));
);
thread_local!(
    pub static kExpansionFactor_92: Value<u32> = Rc::new(RefCell::new(1));
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
pub fn rapid_mul128_93(A: u64, B: u64) -> (Value<u64>, Value<u64>) {
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
pub fn rapid_mix_94(A: u64, B: u64) -> u64 {
    let A: Value<u64> = Rc::new(RefCell::new(A));
    let B: Value<u64> = Rc::new(RefCell::new(B));
    let __rhs = ({ rapid_mul128_93((*A.borrow()), (*B.borrow())) });
    ({ tie_95(A.as_pointer(), B.as_pointer()) }) = __rhs;
    return ((*A.borrow()) ^ (*B.borrow()));
}
thread_local!(
    pub static RAPIDHASH_DEFAULT_SECRET_96: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
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
pub fn hash_combine_99(seed: usize, hash: usize) -> usize {
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
    pub static kRapidhashSecret1_100: Value<u64> = Rc::new(RefCell::new(3257665815644502181));
);
thread_local!(
    pub static kRapidhashSecret2_101: Value<u64> = Rc::new(RefCell::new(10067880064238660809));
);
pub fn hash64_102(key: u64) -> u64 {
    let key: Value<u64> = Rc::new(RefCell::new(key));
    return ({
        let _A: u64 = ((*key.borrow()) ^ 3257665815644502181);
        let _B: u64 = ((*key.borrow()) ^ 10067880064238660809);
        rapid_mix_94(_A, _B)
    });
}
pub fn hash32_103(key: u32) -> u32 {
    let key: Value<u32> = Rc::new(RefCell::new(key));
    return (({ hash64_102(((*key.borrow()) as u64)) }) as u32);
}
pub fn hash_value_104(v: bool) -> usize {
    let v: Value<bool> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) as usize);
}
pub fn hash_value_105(v: u8) -> usize {
    let v: Value<u8> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) as usize);
}
pub fn hash_value_106(v: u16) -> usize {
    let v: Value<u16> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) as usize);
}
pub fn hash_value_97(v: u32) -> usize {
    let v: Value<u32> = Rc::new(RefCell::new(v));
    return (({ hash32_103((*v.borrow())) }) as usize);
}
pub fn hash_value_107(v: u64) -> usize {
    let v: Value<u64> = Rc::new(RefCell::new(v));
    if false {
        return (({ hash32_103(((*v.borrow()) as u32)) }) as usize);
    } else {
        return (({ hash64_102((*v.borrow())) }) as usize);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn hash_value_98(v: u64) -> usize {
    let v: Value<u64> = Rc::new(RefCell::new(v));
    return (({ hash64_102((*v.borrow())) }) as usize);
}
pub fn hash_value_108(v: i8) -> usize {
    let v: Value<i8> = Rc::new(RefCell::new(v));
    return ({ hash_value_105(({ bit_cast_21(v.as_pointer()) })) });
}
pub fn hash_value_109(v: i16) -> usize {
    let v: Value<i16> = Rc::new(RefCell::new(v));
    return ({ hash_value_106(({ bit_cast_23(v.as_pointer()) })) });
}
pub fn hash_value_110(v: i32) -> usize {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    return ({ hash_value_97(({ bit_cast_25(v.as_pointer()) })) });
}
pub fn hash_value_111(v: i64) -> usize {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    return ({ hash_value_107(({ bit_cast_27(v.as_pointer()) })) });
}
pub fn hash_value_112(v: i64) -> usize {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    return ({ hash_value_98(({ bit_cast_29(v.as_pointer()) })) });
}
pub fn hash_value_113(v: f32) -> usize {
    let v: Value<f32> = Rc::new(RefCell::new(v));
    return if ((*v.borrow()) != 0.0E+0) {
        ({ hash_value_97(({ bit_cast_31(v.as_pointer()) })) })
    } else {
        0_usize
    };
}
pub fn hash_value_114(v: f64) -> usize {
    let v: Value<f64> = Rc::new(RefCell::new(v));
    return if ((*v.borrow()) != 0.0E+0) {
        ({ hash_value_98(({ bit_cast_33(v.as_pointer()) })) })
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
impl v8_base_Vector_unsigned_int_ {}
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
        { v8_base_Vector_unsigned_int_::v8_base_Vector_unsigned_int_5() }
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
    pub static enable_view_116: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static enable_borrowed_range_117: Value<bool> = Rc::new(RefCell::new(true));
);
pub fn CStrVector_118(data: Ptr<u8>) -> v8_base_Vector_const_char_ {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    return v8_base_Vector_const_char_::v8_base_Vector_const_char_1(
        { (*data.borrow()).clone() },
        { (*data.borrow()).to_c_string_iterator().count() },
    );
}
pub fn StrVector_119(str: Vec<u8>) -> v8_base_Vector_const_char_ {
    let str: Value<Vec<u8>> = Rc::new(RefCell::new(str));
    return v8_base_Vector_const_char_::v8_base_Vector_const_char_1(
        { ({ (*str.borrow()).data() }) },
        { (*str.borrow()).len() },
    );
}
pub fn OneByteVector_120(data: Ptr<u8>, length: usize) -> v8_base_Vector_const_unsigned_char_ {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    return v8_base_Vector_const_unsigned_char_::v8_base_Vector_const_unsigned_char_3(
        { (*data.borrow()).reinterpret_cast::<u8>() },
        { (*length.borrow()) },
    );
}
pub fn OneByteVector_121(data: Ptr<u8>) -> v8_base_Vector_const_unsigned_char_ {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    return ({
        let _data: Ptr<u8> = (*data.borrow()).clone();
        let _length: usize = (*data.borrow()).to_c_string_iterator().count();
        OneByteVector_120(_data, _length)
    });
}
thread_local!(
    pub static kMaxSignificantBits_122: Value<i32> = Rc::new(RefCell::new(3584));
);
thread_local!(
    static kChunkSize_123: Value<i32> = Rc::new(RefCell::new(
        (((::std::mem::size_of::<u32>() as usize).wrapping_mul(8_usize)) as i32),
    ));
);
thread_local!(
    static kDoubleChunkSize_124: Value<i32> = Rc::new(RefCell::new(
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)) as i32),
    ));
);
thread_local!(
    static kBigitSize_125: Value<i32> = Rc::new(RefCell::new(28));
);
thread_local!(
    static kBigitMask_126: Value<u32> = Rc::new(RefCell::new(
        (((1 << (*kBigitSize_125.with(Value::clone).borrow())) - 1) as u32),
    ));
);
thread_local!(
    static kBigitCapacity_127: Value<i32> = Rc::new(RefCell::new(
        ((*kMaxSignificantBits_122.with(Value::clone).borrow())
            / (*kBigitSize_125.with(Value::clone).borrow())),
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
pub fn HardeningAbort_129() {
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
pub fn is_constant_evaluated_130() -> bool {
    return ({ is_constant_evaluated_131() });
}
pub fn compare_result_as_less_than_132(r: std_weak_ordering) -> bool {
    let r: Value<std_weak_ordering> = Rc::new(RefCell::new(r));
    return operator_lt(
        (*r.borrow()).clone(),
        std__CmpUnspecifiedParam::std__CmpUnspecifiedParam({ 0 }),
    );
}
pub fn compare_result_as_ordering_133(c: std_weak_ordering) -> std_weak_ordering {
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
            lo_: Rc::new(RefCell::new(({ Int128Low64_134((*v.borrow()).clone()) }))),
            hi_: Rc::new(RefCell::new(
                (({ Int128High64_135((*v.borrow()).clone()) }) as u64),
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
            operator_cmp_136(
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
            operator_eq_137(
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
pub fn Uint128Max_138() -> absl_uint128 {
    return absl_uint128::absl_uint12810({ <u64>::MAX }, { <u64>::MAX });
}
thread_local!(
    pub static is_specialized_139: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_signed_140: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static is_integer_141: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_exact_142: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static has_infinity_143: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_quiet_NaN_144: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_signaling_NaN_145: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_denorm_146: Value = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static has_denorm_loss_147: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static round_style_148: Value = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static is_iec559_149: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static is_bounded_150: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_modulo_151: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static digits_152: Value<i32> = Rc::new(RefCell::new(128));
);
thread_local!(
    pub static digits10_153: Value<i32> = Rc::new(RefCell::new(38));
);
thread_local!(
    pub static max_digits10_154: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static radix_155: Value<i32> = Rc::new(RefCell::new(2));
);
thread_local!(
    pub static min_exponent_156: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static min_exponent10_157: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static max_exponent_158: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static max_exponent10_159: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static traps_160: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static tinyness_before_161: Value<bool> = Rc::new(RefCell::new(false));
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
        return ({ Uint128Max_138() });
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
                (({ BitCastToSigned_162((((*high.borrow()) as u128) << 64)) })
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
            operator_cmp_163(
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
            operator_eq_164(
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
pub fn Int128Max_165() -> absl_int128 {
    return absl_int128::absl_int12813({ <i64>::MAX }, { <u64>::MAX });
}
pub fn Int128Min_166() -> absl_int128 {
    return absl_int128::absl_int12813({ <i64>::MIN }, { 0_u64 });
}
thread_local!(
    pub static is_specialized_167: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_signed_168: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_integer_169: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_exact_170: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static has_infinity_171: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_quiet_NaN_172: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_signaling_NaN_173: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_denorm_174: Value = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static has_denorm_loss_175: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static round_style_176: Value = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static is_iec559_177: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static is_bounded_178: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_modulo_179: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static digits_180: Value<i32> = Rc::new(RefCell::new(127));
);
thread_local!(
    pub static digits10_181: Value<i32> = Rc::new(RefCell::new(38));
);
thread_local!(
    pub static max_digits10_182: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static radix_183: Value<i32> = Rc::new(RefCell::new(2));
);
thread_local!(
    pub static min_exponent_184: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static min_exponent10_185: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static max_exponent_186: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static max_exponent10_187: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static traps_188: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static tinyness_before_189: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct std_numeric_limits_absl_int128_ {}
impl std_numeric_limits_absl_int128_ {
    pub fn min() -> absl_int128 {
        return ({ Int128Min_166() });
    }
    pub fn lowest() -> absl_int128 {
        return ({ Int128Min_166() });
    }
    pub fn max() -> absl_int128 {
        return ({ Int128Max_165() });
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
pub fn MakeUint128_190(high: u64, low: u64) -> absl_uint128 {
    let high: Value<u64> = Rc::new(RefCell::new(high));
    let low: Value<u64> = Rc::new(RefCell::new(low));
    return absl_uint128::absl_uint12810({ (*high.borrow()) }, { (*low.borrow()) });
}
pub fn Uint128Low64_198(v: absl_uint128) -> u64 {
    let v: Value<absl_uint128> = Rc::new(RefCell::new(v));
    return (*(*v.borrow()).lo_.borrow());
}
pub fn Uint128High64_199(v: absl_uint128) -> u64 {
    let v: Value<absl_uint128> = Rc::new(RefCell::new(v));
    return (*(*v.borrow()).hi_.borrow());
}
pub fn operator_eq_137(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
        == ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }));
}
pub fn operator_ne_200(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_uint128 = (*lhs.borrow()).clone();
        operator_eq_137(_lhs, (*rhs.borrow()).clone())
    });
}
pub fn operator_lt_201(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
        < ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }));
}
pub fn operator_gt_202(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_uint128 = (*rhs.borrow()).clone();
        operator_lt_201(_lhs, (*lhs.borrow()).clone())
    });
}
pub fn operator_le_203(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_uint128 = (*rhs.borrow()).clone();
        operator_lt_201(_lhs, (*lhs.borrow()).clone())
    });
}
pub fn operator_ge_204(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_uint128 = (*lhs.borrow()).clone();
        operator_lt_201(_lhs, (*rhs.borrow()).clone())
    });
}
pub fn operator_cmp_136(lhs: absl_uint128, rhs: absl_uint128) -> std::cmp::Ordering {
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
pub fn operator_pos_205(val: absl_uint128) -> absl_uint128 {
    let val: Value<absl_uint128> = Rc::new(RefCell::new(val));
    return (*val.borrow()).clone();
}
pub fn operator_pos_206(val: absl_int128) -> absl_int128 {
    let val: Value<absl_int128> = Rc::new(RefCell::new(val));
    return (*val.borrow()).clone();
}
pub fn operator_neg_207(val: absl_uint128) -> absl_uint128 {
    let val: Value<absl_uint128> = Rc::new(RefCell::new(val));
    return absl_uint128::absl_uint1288({
        -({ absl_uint128Impl::operator_unsigned___int128(&val.as_pointer()) })
    });
}
pub fn operator_not_208(val: absl_uint128) -> bool {
    let val: Value<absl_uint128> = Rc::new(RefCell::new(val));
    return !(({ absl_uint128Impl::operator_unsigned___int128(&val.as_pointer()) }) != 0);
}
pub fn operator_bitnot_209(val: absl_uint128) -> absl_uint128 {
    let val: Value<absl_uint128> = Rc::new(RefCell::new(val));
    return absl_uint128::absl_uint1288({
        !({ absl_uint128Impl::operator_unsigned___int128(&val.as_pointer()) })
    });
}
pub fn operator_bitor_210(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            | ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitand_211(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            & ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitxor_212(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            ^ ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_shl_191(lhs: absl_uint128, amount: i32) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let amount: Value<i32> = Rc::new(RefCell::new(amount));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            << (*amount.borrow()))
    });
}
pub fn operator_shr_192(lhs: absl_uint128, amount: i32) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let amount: Value<i32> = Rc::new(RefCell::new(amount));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            >> (*amount.borrow()))
    });
}
pub fn operator_add_193(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_add(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_sub_194(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_sub(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_mul_195(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_mul(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_div_196(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_div(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_rem_197(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_rem(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn MakeInt128_213(high: i64, low: u64) -> absl_int128 {
    let high: Value<i64> = Rc::new(RefCell::new(high));
    let low: Value<u64> = Rc::new(RefCell::new(low));
    return absl_int128::absl_int12813({ (*high.borrow()) }, { (*low.borrow()) });
}
pub fn BitCastToSigned_224(v: u64) -> i64 {
    let v: Value<u64> = Rc::new(RefCell::new(v));
    return if (((*v.borrow()) & (1_u64 << 63)) != 0) {
        !(!(*v.borrow()) as i64)
    } else {
        ((*v.borrow()) as i64)
    };
}
pub fn BitCastToSigned_162(v: u128) -> i128 {
    let v: Value<u128> = Rc::new(RefCell::new(v));
    return if (((*v.borrow()) & (1_u128 << 127)) != 0) {
        !(!(*v.borrow()) as i128)
    } else {
        ((*v.borrow()) as i128)
    };
}
pub fn Int128Low64_134(v: absl_int128) -> u64 {
    let v: Value<absl_int128> = Rc::new(RefCell::new(v));
    return (((*(*v.borrow()).v_.borrow()) & (!0_u64 as i128)) as u64);
}
pub fn Int128High64_135(v: absl_int128) -> i64 {
    let v: Value<absl_int128> = Rc::new(RefCell::new(v));
    return ({ BitCastToSigned_224(((((*(*v.borrow()).v_.borrow()) as u128) >> 64) as u64)) });
}
pub fn operator_eq_164(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        == ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_ne_225(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        != ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_lt_226(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        < ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_gt_227(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        > ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_le_228(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        <= ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_ge_229(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        >= ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_cmp_163(lhs: absl_int128, rhs: absl_int128) -> std::cmp::Ordering {
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
pub fn operator_neg_230(v: absl_int128) -> absl_int128 {
    let v: Value<absl_int128> = Rc::new(RefCell::new(v));
    return absl_int128::absl_int1288({
        -({ absl_int128Impl::operator___int128(&v.as_pointer()) })
    });
}
pub fn operator_not_231(v: absl_int128) -> bool {
    let v: Value<absl_int128> = Rc::new(RefCell::new(v));
    return !(({ absl_int128Impl::operator___int128(&v.as_pointer()) }) != 0);
}
pub fn operator_bitnot_232(val: absl_int128) -> absl_int128 {
    let val: Value<absl_int128> = Rc::new(RefCell::new(val));
    return absl_int128::absl_int1288({
        !({ absl_int128Impl::operator___int128(&val.as_pointer()) })
    });
}
pub fn operator_add_214(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            + ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_sub_215(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            - ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_mul_216(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            * ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_div_217(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            / ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_rem_218(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            % ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitor_219(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            | ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitand_220(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            & ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitxor_221(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            ^ ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_shl_222(lhs: absl_int128, amount: i32) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let amount: Value<i32> = Rc::new(RefCell::new(amount));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) }) << (*amount.borrow()))
    });
}
pub fn operator_shr_223(lhs: absl_int128, amount: i32) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let amount: Value<i32> = Rc::new(RefCell::new(amount));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) }) >> (*amount.borrow()))
    });
}
thread_local!(
    pub static kSignificandSize_233: Value<i32> = Rc::new(RefCell::new(64));
);
thread_local!(
    static kUint64MSB_234: Value<u64> = Rc::new(RefCell::new((1_u64 << 63)));
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
                operator_mul_195(_lhs, _rhs)
            }),
        ));
        let hi: Value<u64> = Rc::new(RefCell::new(
            ({ Uint128High64_199((*mul.borrow()).clone()) }),
        ));
        let lo: Value<u64> = Rc::new(RefCell::new(
            ({ Uint128Low64_198((*mul.borrow()).clone()) }),
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
pub fn double_to_uint64_238(d: f64) -> u64 {
    let d: Value<f64> = Rc::new(RefCell::new(d));
    return ({ bit_cast_33(d.as_pointer()) });
}
pub fn uint64_to_double_239(d64: u64) -> f64 {
    let d64: Value<u64> = Rc::new(RefCell::new(d64));
    return ({ bit_cast_35(d64.as_pointer()) });
}
thread_local!(
    pub static kSignMask_240: Value<u64> = Rc::new(RefCell::new(9223372036854775808));
);
thread_local!(
    pub static kExponentMask_241: Value<u64> = Rc::new(RefCell::new(9218868437227405312));
);
thread_local!(
    pub static kSignificandMask_242: Value<u64> = Rc::new(RefCell::new(4503599627370495));
);
thread_local!(
    pub static kHiddenBit_243: Value<u64> = Rc::new(RefCell::new(4503599627370496));
);
thread_local!(
    pub static kPhysicalSignificandSize_244: Value<i32> = Rc::new(RefCell::new(52));
);
thread_local!(
    pub static kSignificandSize_245: Value<i32> = Rc::new(RefCell::new(53));
);
thread_local!(
    static kExponentBias_246: Value<i32> = Rc::new(RefCell::new(1075));
);
thread_local!(
    static kDenormalExponent_247: Value<i32> = Rc::new(RefCell::new(-1074));
);
thread_local!(
    static kMaxExponent_248: Value<i32> = Rc::new(RefCell::new(972));
);
thread_local!(
    static kInfinity_249: Value<u64> = Rc::new(RefCell::new(9218868437227405312));
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
            d64_: Rc::new(RefCell::new(({ double_to_uint64_238((*d.borrow())) }))),
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
    pub static kMaxExactDoubleIntegerDecimalDigits_250: Value<i32> = Rc::new(RefCell::new(15));
);
thread_local!(
    pub static kMaxUint64DecimalDigits_251: Value<i32> = Rc::new(RefCell::new(19));
);
thread_local!(
    pub static kMaxDecimalPower_252: Value<i32> = Rc::new(RefCell::new(309));
);
thread_local!(
    pub static kMinDecimalPower_253: Value<i32> = Rc::new(RefCell::new(-324_i32));
);
thread_local!(
    pub static kMaxUint64_254: Value<u64> = Rc::new(RefCell::new(18446744073709551615_u64));
);
thread_local!(
    pub static exact_powers_of_ten_255: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([
        1.0E+0, 1.0E+1, 1.0E+2, 1.0E+3, 1.0E+4, 1.0E+5, 1.0E+6, 1.0E+7, 1.0E+8, 1.0E+9, 1.0E+10,
        1.0E+11, 1.0E+12, 1.0E+13, 1.0E+14, 1.0E+15, 1.0E+16, 1.0E+17, 1.0E+18, 1.0E+19, 1.0E+20,
        1.0E+21, 1.0E+22,
    ])));
);
thread_local!(
    pub static kExactPowersOfTenSize_256: Value<i32> =
        Rc::new(RefCell::new(((::std::mem::size_of::<[u8; 23]>()) as i32)));
);
thread_local!(
    pub static kMaxSignificantDecimalDigits_257: Value<i32> = Rc::new(RefCell::new(780));
);
pub fn TrimLeadingZeros_258(buffer: v8_base_Vector_const_char_) -> v8_base_Vector_const_char_ {
    let buffer: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(buffer));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow())
        < ({ v8_base_Vector_const_char_Impl::length(&buffer.as_pointer()) }))
    {
        if (((({
            v8_base_Vector_const_char_Impl::operator_index(
                &buffer.as_pointer(),
                ((*i.borrow()) as usize),
            )
        })
        .read()) as i32)
            != (('0' as u8) as i32))
        {
            return ({
                let _to: usize =
                    (({ v8_base_Vector_const_char_Impl::length(&buffer.as_pointer()) }) as usize);
                v8_base_Vector_const_char_Impl::SubVector(
                    &buffer.as_pointer(),
                    ((*i.borrow()) as usize),
                    _to,
                )
            });
        }
        (*i.borrow_mut()).postfix_inc();
    }
    return v8_base_Vector_const_char_::v8_base_Vector_const_char_1(
        { ({ v8_base_Vector_const_char_Impl::begin(&buffer.as_pointer()) }) },
        { 0_usize },
    );
}
pub fn TrimTrailingZeros_259(buffer: v8_base_Vector_const_char_) -> v8_base_Vector_const_char_ {
    let buffer: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(buffer));
    let i: Value<i32> = Rc::new(RefCell::new(
        (({ v8_base_Vector_const_char_Impl::length(&buffer.as_pointer()) }) - 1),
    ));
    'loop_: while ((*i.borrow()) >= 0) {
        if (((({
            v8_base_Vector_const_char_Impl::operator_index(
                &buffer.as_pointer(),
                ((*i.borrow()) as usize),
            )
        })
        .read()) as i32)
            != (('0' as u8) as i32))
        {
            return ({
                v8_base_Vector_const_char_Impl::SubVector(
                    &buffer.as_pointer(),
                    0_usize,
                    (((*i.borrow()) + 1) as usize),
                )
            });
        }
        (*i.borrow_mut()).prefix_dec();
    }
    return v8_base_Vector_const_char_::v8_base_Vector_const_char_1(
        { ({ v8_base_Vector_const_char_Impl::begin(&buffer.as_pointer()) }) },
        { 0_usize },
    );
}
pub fn TrimToMaxSignificantDigits_260(
    buffer: v8_base_Vector_const_char_,
    exponent: i32,
    significant_buffer: Ptr<u8>,
    significant_exponent: Ptr<i32>,
) {
    let buffer: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(buffer));
    let exponent: Value<i32> = Rc::new(RefCell::new(exponent));
    let significant_buffer: Value<Ptr<u8>> = Rc::new(RefCell::new(significant_buffer));
    let significant_exponent: Value<Ptr<i32>> = Rc::new(RefCell::new(significant_exponent));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow())
        < ((*kMaxSignificantDecimalDigits_257.with(Value::clone).borrow()) - 1))
    {
        let __rhs = (({
            v8_base_Vector_const_char_Impl::operator_index(
                &buffer.as_pointer(),
                ((*i.borrow()) as usize),
            )
        })
        .read());
        (*significant_buffer.borrow())
            .offset((*i.borrow()) as isize)
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    (&(0));
    (*significant_buffer.borrow())
        .offset(((*kMaxSignificantDecimalDigits_257.with(Value::clone).borrow()) - 1) as isize)
        .write(('1' as u8));
    let __rhs = ((*exponent.borrow())
        + (({ v8_base_Vector_const_char_Impl::length(&buffer.as_pointer()) })
            - (*kMaxSignificantDecimalDigits_257.with(Value::clone).borrow())));
    (*significant_exponent.borrow()).write(__rhs);
}
pub fn ReadUint64_261(buffer: v8_base_Vector_const_char_, number_of_read_digits: Ptr<i32>) -> u64 {
    let buffer: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(buffer));
    let number_of_read_digits: Value<Ptr<i32>> = Rc::new(RefCell::new(number_of_read_digits));
    let result: Value<u64> = Rc::new(RefCell::new(0_u64));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow())
        < ({ v8_base_Vector_const_char_Impl::length(&buffer.as_pointer()) }))
        && ((*result.borrow())
            <= (((*kMaxUint64_254.with(Value::clone).borrow()).wrapping_div(10_u64) as u64)
                .wrapping_sub(1_u64)))
    {
        let digit: Value<i32> = Rc::new(RefCell::new(
            (((({
                v8_base_Vector_const_char_Impl::operator_index(
                    &buffer.as_pointer(),
                    ((*i.borrow_mut()).postfix_inc() as usize),
                )
            })
            .read()) as i32)
                - (('0' as u8) as i32)),
        ));
        (&(0));
        let __rhs =
            ((10_u64).wrapping_mul((*result.borrow()))).wrapping_add(((*digit.borrow()) as u64));
        (*result.borrow_mut()) = __rhs;
    }
    let __rhs = (*i.borrow());
    (*number_of_read_digits.borrow()).write(__rhs);
    return (*result.borrow());
}
pub fn ReadDiyFp_262(
    buffer: v8_base_Vector_const_char_,
    result: Ptr<v8_base_DiyFp>,
    remaining_decimals: Ptr<i32>,
) {
    let buffer: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(buffer));
    let result: Value<Ptr<v8_base_DiyFp>> = Rc::new(RefCell::new(result));
    let remaining_decimals: Value<Ptr<i32>> = Rc::new(RefCell::new(remaining_decimals));
    let read_digits: Value<i32> = <Value<i32>>::default();
    let significand: Value<u64> = Rc::new(RefCell::new(
        ({ ReadUint64_261((*buffer.borrow()).clone(), (read_digits.as_pointer())) }),
    ));
    if (({ v8_base_Vector_const_char_Impl::length(&buffer.as_pointer()) })
        == (*read_digits.borrow()))
    {
        let __rhs = v8_base_DiyFp::v8_base_DiyFp2({ (*significand.borrow()) }, { 0 });
        (*result.borrow()).write(__rhs);
        (*remaining_decimals.borrow()).write(0);
    } else {
        if (((({
            v8_base_Vector_const_char_Impl::operator_index(
                &buffer.as_pointer(),
                ((*read_digits.borrow()) as usize),
            )
        })
        .read()) as i32)
            >= (('5' as u8) as i32))
        {
            (*significand.borrow_mut()).postfix_inc();
        }
        let exponent: Value<i32> = Rc::new(RefCell::new(0));
        let __rhs =
            v8_base_DiyFp::v8_base_DiyFp2({ (*significand.borrow()) }, { (*exponent.borrow()) });
        (*result.borrow()).write(__rhs);
        let __rhs = (({ v8_base_Vector_const_char_Impl::length(&buffer.as_pointer()) })
            - (*read_digits.borrow()));
        (*remaining_decimals.borrow()).write(__rhs);
    }
}
pub fn DoubleStrtod_263(
    trimmed: v8_base_Vector_const_char_,
    exponent: i32,
    result: Ptr<f64>,
) -> bool {
    let trimmed: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(trimmed));
    let exponent: Value<i32> = Rc::new(RefCell::new(exponent));
    let result: Value<Ptr<f64>> = Rc::new(RefCell::new(result));
    if (({ v8_base_Vector_const_char_Impl::length(&trimmed.as_pointer()) })
        <= (*kMaxExactDoubleIntegerDecimalDigits_250
            .with(Value::clone)
            .borrow()))
    {
        let read_digits: Value<i32> = <Value<i32>>::default();
        if ((*exponent.borrow()) < 0)
            && (-(*exponent.borrow()) < (*kExactPowersOfTenSize_256.with(Value::clone).borrow()))
        {
            let __rhs =
                (({ ReadUint64_261((*trimmed.borrow()).clone(), (read_digits.as_pointer())) })
                    as f64);
            (*result.borrow()).write(__rhs);
            (&(0));
            let __rhs = (*exact_powers_of_ten_255.with(Value::clone).borrow())
                [(-(*exponent.borrow())) as usize];
            {
                let _ptr = (*result.borrow()).clone();
                _ptr.write(_ptr.read() / __rhs)
            };
            return true;
        }
        if (0 <= (*exponent.borrow()))
            && ((*exponent.borrow()) < (*kExactPowersOfTenSize_256.with(Value::clone).borrow()))
        {
            let __rhs =
                (({ ReadUint64_261((*trimmed.borrow()).clone(), (read_digits.as_pointer())) })
                    as f64);
            (*result.borrow()).write(__rhs);
            (&(0));
            let __rhs = (*exact_powers_of_ten_255.with(Value::clone).borrow())
                [(*exponent.borrow()) as usize];
            {
                let _ptr = (*result.borrow()).clone();
                _ptr.write(_ptr.read() * __rhs)
            };
            return true;
        }
        let remaining_digits: Value<i32> = Rc::new(RefCell::new(
            ((*kMaxExactDoubleIntegerDecimalDigits_250
                .with(Value::clone)
                .borrow())
                - ({ v8_base_Vector_const_char_Impl::length(&trimmed.as_pointer()) })),
        ));
        if (0 <= (*exponent.borrow()))
            && (((*exponent.borrow()) - (*remaining_digits.borrow()))
                < (*kExactPowersOfTenSize_256.with(Value::clone).borrow()))
        {
            let __rhs =
                (({ ReadUint64_261((*trimmed.borrow()).clone(), (read_digits.as_pointer())) })
                    as f64);
            (*result.borrow()).write(__rhs);
            (&(0));
            let __rhs = (*exact_powers_of_ten_255.with(Value::clone).borrow())
                [(*remaining_digits.borrow()) as usize];
            {
                let _ptr = (*result.borrow()).clone();
                _ptr.write(_ptr.read() * __rhs)
            };
            let __rhs = (*exact_powers_of_ten_255.with(Value::clone).borrow())
                [((*exponent.borrow()) - (*remaining_digits.borrow())) as usize];
            {
                let _ptr = (*result.borrow()).clone();
                _ptr.write(_ptr.read() * __rhs)
            };
            return true;
        }
    }
    return false;
}
pub fn AdjustmentPowerOfTen_264(exponent: i32) -> v8_base_DiyFp {
    let exponent: Value<i32> = Rc::new(RefCell::new(exponent));
    (&(0));
    (&(0));
    (&(0));
    switch!(match (*exponent.borrow()) {
        __v if __v == 1 => {
            return v8_base_DiyFp::v8_base_DiyFp2({ 11529215046068469760_u64 }, { -60_i32 });
        }
        __v if __v == 2 => {
            return v8_base_DiyFp::v8_base_DiyFp2({ 14411518807585587200_u64 }, { -57_i32 });
        }
        __v if __v == 3 => {
            return v8_base_DiyFp::v8_base_DiyFp2({ 18014398509481984000_u64 }, { -54_i32 });
        }
        __v if __v == 4 => {
            return v8_base_DiyFp::v8_base_DiyFp2({ 11258999068426240000_u64 }, { -50_i32 });
        }
        __v if __v == 5 => {
            return v8_base_DiyFp::v8_base_DiyFp2({ 14073748835532800000_u64 }, { -47_i32 });
        }
        __v if __v == 6 => {
            return v8_base_DiyFp::v8_base_DiyFp2({ 17592186044416000000_u64 }, { -44_i32 });
        }
        __v if __v == 7 => {
            return v8_base_DiyFp::v8_base_DiyFp2({ 10995116277760000000_u64 }, { -40_i32 });
        }
        _ => {
            ({
                V8_Fatal_115(
                    (*kUnreachableCodeMessage_9.with(Value::clone).borrow()).clone(),
                    &[],
                )
            });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn DiyFpStrtod_265(
    buffer: v8_base_Vector_const_char_,
    exponent: i32,
    result: Ptr<f64>,
) -> bool {
    let buffer: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(buffer));
    let exponent: Value<i32> = Rc::new(RefCell::new(exponent));
    let result: Value<Ptr<f64>> = Rc::new(RefCell::new(result));
    let input: Value<v8_base_DiyFp> = Rc::new(RefCell::new(v8_base_DiyFp::v8_base_DiyFp1()));
    let remaining_decimals: Value<i32> = <Value<i32>>::default();
    ({
        ReadDiyFp_262(
            (*buffer.borrow()).clone(),
            (input.as_pointer()),
            (remaining_decimals.as_pointer()),
        )
    });
    let kDenominatorLog: Value<i32> = Rc::new(RefCell::new(3));
    let kDenominator: Value<i32> = Rc::new(RefCell::new((1 << (*kDenominatorLog.borrow()))));
    (*exponent.borrow_mut()) += (*remaining_decimals.borrow());
    let error: Value<i64> = Rc::new(RefCell::new(
        ((if ((*remaining_decimals.borrow()) == 0) {
            0
        } else {
            ((*kDenominator.borrow()) / 2)
        }) as i64),
    ));
    let old_e: Value<i32> = Rc::new(RefCell::new(
        ({ v8_base_DiyFpImpl::e(&input.as_pointer()) }),
    ));
    ({ v8_base_DiyFpImpl::Normalize(&input.as_pointer()) });
    (*error.borrow_mut()) <<= ((*old_e.borrow()) - ({ v8_base_DiyFpImpl::e(&input.as_pointer()) }));
    (&(0));
    if ((*exponent.borrow()) < (*kMinDecimalExponent_236.with(Value::clone).borrow())) {
        (*result.borrow()).write(0.0E+0);
        return true;
    }
    let cached_power: Value<v8_base_DiyFp> = Rc::new(RefCell::new(v8_base_DiyFp::v8_base_DiyFp1()));
    let cached_decimal_exponent: Value<i32> = <Value<i32>>::default();
    ({
        v8_base_PowersOfTenCache::GetCachedPowerForDecimalExponent(
            (*exponent.borrow()),
            (cached_power.as_pointer()),
            (cached_decimal_exponent.as_pointer()),
        )
    });
    if ((*cached_decimal_exponent.borrow()) != (*exponent.borrow())) {
        let adjustment_exponent: Value<i32> = Rc::new(RefCell::new(
            ((*exponent.borrow()) - (*cached_decimal_exponent.borrow())),
        ));
        let adjustment_power: Value<v8_base_DiyFp> = Rc::new(RefCell::new(
            ({ AdjustmentPowerOfTen_264((*adjustment_exponent.borrow())) }),
        ));
        ({ v8_base_DiyFpImpl::Multiply(&input.as_pointer(), adjustment_power.as_pointer()) });
        if (((*kMaxUint64DecimalDigits_251.with(Value::clone).borrow())
            - ({ v8_base_Vector_const_char_Impl::length(&buffer.as_pointer()) }))
            >= (*adjustment_exponent.borrow()))
        {
            (&(0));
        } else {
            (*error.borrow_mut()) += (((*kDenominator.borrow()) / 2) as i64);
        }
    }
    ({ v8_base_DiyFpImpl::Multiply(&input.as_pointer(), cached_power.as_pointer()) });
    let error_b: Value<i32> = Rc::new(RefCell::new(((*kDenominator.borrow()) / 2)));
    let error_ab: Value<i32> = Rc::new(RefCell::new(
        (if ((*error.borrow()) == 0_i64) { 0 } else { 1 }),
    ));
    let fixed_error: Value<i32> = Rc::new(RefCell::new(((*kDenominator.borrow()) / 2)));
    (*error.borrow_mut()) +=
        ((((*error_b.borrow()) + (*error_ab.borrow())) + (*fixed_error.borrow())) as i64);
    (*old_e.borrow_mut()) = ({ v8_base_DiyFpImpl::e(&input.as_pointer()) });
    ({ v8_base_DiyFpImpl::Normalize(&input.as_pointer()) });
    (*error.borrow_mut()) <<= ((*old_e.borrow()) - ({ v8_base_DiyFpImpl::e(&input.as_pointer()) }));
    let order_of_magnitude: Value<i32> = Rc::new(RefCell::new(
        ((*kSignificandSize_233.with(Value::clone).borrow())
            + ({ v8_base_DiyFpImpl::e(&input.as_pointer()) })),
    ));
    let effective_significand_size: Value<i32> = Rc::new(RefCell::new(
        ({ v8_base_Double::SignificandSizeForOrderOfMagnitude((*order_of_magnitude.borrow())) }),
    ));
    let precision_digits_count: Value<i32> = Rc::new(RefCell::new(
        ((*kSignificandSize_233.with(Value::clone).borrow())
            - (*effective_significand_size.borrow())),
    ));
    if (((*precision_digits_count.borrow()) + (*kDenominatorLog.borrow()))
        >= (*kSignificandSize_233.with(Value::clone).borrow()))
    {
        let shift_amount: Value<i32> = Rc::new(RefCell::new(
            ((((*precision_digits_count.borrow()) + (*kDenominatorLog.borrow()))
                - (*kSignificandSize_233.with(Value::clone).borrow()))
                + 1),
        ));
        ({
            let _new_value: u64 =
                (({ v8_base_DiyFpImpl::f(&input.as_pointer()) }) >> (*shift_amount.borrow()));
            v8_base_DiyFpImpl::set_f(&input.as_pointer(), _new_value)
        });
        ({
            let _new_value: i32 =
                (({ v8_base_DiyFpImpl::e(&input.as_pointer()) }) + (*shift_amount.borrow()));
            v8_base_DiyFpImpl::set_e(&input.as_pointer(), _new_value)
        });
        let __rhs = ((((*error.borrow()) >> (*shift_amount.borrow())) + 1_i64)
            + ((*kDenominator.borrow()) as i64));
        (*error.borrow_mut()) = __rhs;
        (*precision_digits_count.borrow_mut()) -= (*shift_amount.borrow());
    }
    (&(0));
    (&(0));
    let one64: Value<u64> = Rc::new(RefCell::new(1_u64));
    let precision_bits_mask: Value<u64> = Rc::new(RefCell::new(
        ((*one64.borrow()) << (*precision_digits_count.borrow())).wrapping_sub(1_u64),
    ));
    let precision_bits: Value<u64> = Rc::new(RefCell::new(
        (({ v8_base_DiyFpImpl::f(&input.as_pointer()) }) & (*precision_bits_mask.borrow())),
    ));
    let half_way: Value<u64> = Rc::new(RefCell::new(
        ((*one64.borrow()) << ((*precision_digits_count.borrow()) - 1)),
    ));
    {
        let rhs_0 = (*precision_bits.borrow()).wrapping_mul(((*kDenominator.borrow()) as u64));
        (*precision_bits.borrow_mut()) = rhs_0
    };
    {
        let rhs_0 = (*half_way.borrow()).wrapping_mul(((*kDenominator.borrow()) as u64));
        (*half_way.borrow_mut()) = rhs_0
    };
    let rounded_input: Value<v8_base_DiyFp> = Rc::new(RefCell::new(v8_base_DiyFp::v8_base_DiyFp2(
        { (({ v8_base_DiyFpImpl::f(&input.as_pointer()) }) >> (*precision_digits_count.borrow())) },
        { (({ v8_base_DiyFpImpl::e(&input.as_pointer()) }) + (*precision_digits_count.borrow())) },
    )));
    if ((*precision_bits.borrow()) >= (*half_way.borrow()).wrapping_add(((*error.borrow()) as u64)))
    {
        ({
            let _new_value: u64 =
                ({ v8_base_DiyFpImpl::f(&rounded_input.as_pointer()) }).wrapping_add(1_u64);
            v8_base_DiyFpImpl::set_f(&rounded_input.as_pointer(), _new_value)
        });
    }
    let __rhs = ({
        v8_base_DoubleImpl::value(
            &Rc::new(RefCell::new(v8_base_Double::v8_base_Double4({
                (*rounded_input.borrow()).clone()
            })))
            .as_pointer(),
        )
    });
    (*result.borrow()).write(__rhs);
    if ((*half_way.borrow()).wrapping_sub(((*error.borrow()) as u64)) < (*precision_bits.borrow()))
        && ((*precision_bits.borrow())
            < (*half_way.borrow()).wrapping_add(((*error.borrow()) as u64)))
    {
        return false;
    } else {
        return true;
    }
    panic!("ub: non-void function does not return a value")
}
pub fn BignumStrtod_266(buffer: v8_base_Vector_const_char_, exponent: i32, guess: f64) -> f64 {
    let buffer: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(buffer));
    let exponent: Value<i32> = Rc::new(RefCell::new(exponent));
    let guess: Value<f64> = Rc::new(RefCell::new(guess));
    if ((*guess.borrow()) == ({ std_numeric_limits_double_::infinity() })) {
        return (*guess.borrow());
    }
    let upper_boundary: Value<v8_base_DiyFp> = Rc::new(RefCell::new(
        ({
            v8_base_DoubleImpl::UpperBoundary(
                &Rc::new(RefCell::new(v8_base_Double::v8_base_Double2({
                    (*guess.borrow())
                })))
                .as_pointer(),
            )
        }),
    ));
    (&(0));
    (&(0));
    (&(0));
    (&(0));
    let input: Value<v8_base_Bignum> = Rc::new(RefCell::new(v8_base_Bignum::v8_base_Bignum()));
    let boundary: Value<v8_base_Bignum> = Rc::new(RefCell::new(v8_base_Bignum::v8_base_Bignum()));
    ({ v8_base_BignumImpl::AssignDecimalString(&input.as_pointer(), (*buffer.borrow()).clone()) });
    ({
        v8_base_BignumImpl::AssignUInt64(
            &boundary.as_pointer(),
            ({ v8_base_DiyFpImpl::f(&upper_boundary.as_pointer()) }),
        )
    });
    if ((*exponent.borrow()) >= 0) {
        ({ v8_base_BignumImpl::MultiplyByPowerOfTen(&input.as_pointer(), (*exponent.borrow())) });
    } else {
        ({
            v8_base_BignumImpl::MultiplyByPowerOfTen(&boundary.as_pointer(), -(*exponent.borrow()))
        });
    }
    if (({ v8_base_DiyFpImpl::e(&upper_boundary.as_pointer()) }) > 0) {
        ({
            v8_base_BignumImpl::ShiftLeft(
                &boundary.as_pointer(),
                ({ v8_base_DiyFpImpl::e(&upper_boundary.as_pointer()) }),
            )
        });
    } else {
        ({
            v8_base_BignumImpl::ShiftLeft(
                &input.as_pointer(),
                -({ v8_base_DiyFpImpl::e(&upper_boundary.as_pointer()) }),
            )
        });
    }
    let comparison: Value<i32> = Rc::new(RefCell::new(
        ({ v8_base_Bignum::Compare(input.as_pointer(), boundary.as_pointer()) }),
    ));
    if ((*comparison.borrow()) < 0) {
        return (*guess.borrow());
    } else if ((*comparison.borrow()) > 0) {
        return ({
            v8_base_DoubleImpl::NextDouble(
                &Rc::new(RefCell::new(v8_base_Double::v8_base_Double2({
                    (*guess.borrow())
                })))
                .as_pointer(),
            )
        });
    } else if ((({
        v8_base_DoubleImpl::Significand(
            &Rc::new(RefCell::new(v8_base_Double::v8_base_Double2({
                (*guess.borrow())
            })))
            .as_pointer(),
        )
    }) & 1_u64)
        == 0_u64)
    {
        return (*guess.borrow());
    } else {
        return ({
            v8_base_DoubleImpl::NextDouble(
                &Rc::new(RefCell::new(v8_base_Double::v8_base_Double2({
                    (*guess.borrow())
                })))
                .as_pointer(),
            )
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn Strtod_267(buffer: v8_base_Vector_const_char_, exponent: i32) -> f64 {
    let buffer: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(buffer));
    let exponent: Value<i32> = Rc::new(RefCell::new(exponent));
    let left_trimmed: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(
        ({ TrimLeadingZeros_258((*buffer.borrow()).clone()) }),
    ));
    let trimmed: Value<v8_base_Vector_const_char_> = Rc::new(RefCell::new(
        ({ TrimTrailingZeros_259((*left_trimmed.borrow()).clone()) }),
    ));
    (*exponent.borrow_mut()) +=
        (({ v8_base_Vector_const_char_Impl::length(&left_trimmed.as_pointer()) })
            - ({ v8_base_Vector_const_char_Impl::length(&trimmed.as_pointer()) }));
    if ({ v8_base_Vector_const_char_Impl::empty(&trimmed.as_pointer()) }) {
        return 0.0E+0;
    }
    if (({ v8_base_Vector_const_char_Impl::length(&trimmed.as_pointer()) })
        > (*kMaxSignificantDecimalDigits_257.with(Value::clone).borrow()))
    {
        let significant_buffer: Value<Box<[u8]>> = Rc::new(RefCell::new(
            (0..780).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
        ));
        let significant_exponent: Value<i32> = <Value<i32>>::default();
        ({
            TrimToMaxSignificantDigits_260(
                (*trimmed.borrow()).clone(),
                (*exponent.borrow()),
                (significant_buffer.as_pointer() as Ptr<u8>),
                (significant_exponent.as_pointer()),
            )
        });
        return ({
            Strtod_267(
                v8_base_Vector_const_char_::v8_base_Vector_const_char_1(
                    { (significant_buffer.as_pointer() as Ptr<u8>) },
                    { ((*kMaxSignificantDecimalDigits_257.with(Value::clone).borrow()) as usize) },
                ),
                (*significant_exponent.borrow()),
            )
        });
    }
    if ((((*exponent.borrow())
        + ({ v8_base_Vector_const_char_Impl::length(&trimmed.as_pointer()) }))
        - 1)
        >= (*kMaxDecimalPower_252.with(Value::clone).borrow()))
    {
        return ({ std_numeric_limits_double_::infinity() });
    }
    if (((*exponent.borrow())
        + ({ v8_base_Vector_const_char_Impl::length(&trimmed.as_pointer()) }))
        <= (*kMinDecimalPower_253.with(Value::clone).borrow()))
    {
        return 0.0E+0;
    }
    let guess: Value<f64> = <Value<f64>>::default();
    if ({
        DoubleStrtod_263(
            (*trimmed.borrow()).clone(),
            (*exponent.borrow()),
            (guess.as_pointer()),
        )
    }) || ({
        DiyFpStrtod_265(
            (*trimmed.borrow()).clone(),
            (*exponent.borrow()),
            (guess.as_pointer()),
        )
    }) {
        return (*guess.borrow());
    }
    return ({
        BignumStrtod_266(
            (*trimmed.borrow()).clone(),
            (*exponent.borrow()),
            (*guess.borrow()),
        )
    });
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
                operator_add_214(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_sub_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_sub_215(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_mul_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_mul_216(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_div_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_div_217(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_rem_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_rem_218(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitor_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_bitor_219(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitand_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_bitand_220(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitxor_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_bitxor_221(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_shl_assign(&self, amount: i32) -> Ptr<absl_int128> {
        let amount: Value<i32> = Rc::new(RefCell::new(amount));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_shl_222(_lhs, (*amount.borrow()))
            }),
        );
        return (*self).clone();
    }
    fn operator_shr_assign(&self, amount: i32) -> Ptr<absl_int128> {
        let amount: Value<i32> = Rc::new(RefCell::new(amount));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_shr_223(_lhs, (*amount.borrow()))
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
                operator_shl_191(_lhs, (*amount.borrow()))
            }),
        );
        return (*self).clone();
    }
    fn operator_shr_assign(&self, amount: i32) -> Ptr<absl_uint128> {
        let amount: Value<i32> = Rc::new(RefCell::new(amount));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_shr_192(_lhs, (*amount.borrow()))
            }),
        );
        return (*self).clone();
    }
    fn operator_add_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_add_193(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_sub_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_sub_194(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_mul_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_mul_195(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_div_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_div_196(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_rem_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_rem_197(_lhs, (*other.borrow()).clone())
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
                operator_bitor_210(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitand_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_bitand_211(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitxor_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_bitxor_212(_lhs, (*other.borrow()).clone())
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
pub trait v8_base_BignumImpl {
    fn Times10(&self);
    fn EnsureCapacity(&self, size: i32);
    fn BigitLength(&self) -> i32;
}
impl v8_base_BignumImpl for Ptr<v8_base_Bignum> {
    fn Times10(&self) {
        ({ v8_base_BignumImpl::MultiplyByUInt32(self, 10_u32) });
        return;
    }
    fn EnsureCapacity(&self, size: i32) {
        let size: Value<i32> = Rc::new(RefCell::new(size));
        if ((*size.borrow()) > (*kBigitCapacity_127.with(Value::clone).borrow())) {
            ({
                V8_Fatal_115(
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
        'loop_: while (((*f.borrow()) & (*kUint64MSB_234.with(Value::clone).borrow())) == 0_u64) {
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
        (*f.borrow_mut()) <<= ((*kSignificandSize_233.with(Value::clone).borrow()) - 53);
        (*e.borrow_mut()) -= ((*kSignificandSize_233.with(Value::clone).borrow()) - 53);
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
        return ({ uint64_to_double_239((*(*(*self).upgrade().deref()).d64_.borrow())) });
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
            hash_combine_99(
                (*(*(*self).upgrade().deref()).hash_.borrow()),
                (*other_hash.borrow()),
            )
        });
        (*(*(*self).upgrade().deref()).hash_.borrow_mut()) = __rhs;
        return (*self).clone();
    }
}
pub trait v8_base_Vector_const_char_Impl {
    fn SubVector(&self, from: usize, to: usize) -> v8_base_Vector_const_char_;
    fn length(&self) -> i32;
    fn empty(&self) -> bool;
    fn operator_index(&self, index: usize) -> Ptr<u8>;
    fn begin(&self) -> Ptr<u8>;
}
impl v8_base_Vector_const_char_Impl for Ptr<v8_base_Vector_const_char_> {
    fn SubVector(&self, from: usize, to: usize) -> v8_base_Vector_const_char_ {
        let from: Value<usize> = Rc::new(RefCell::new(from));
        let to: Value<usize> = Rc::new(RefCell::new(to));
        (&(0));
        (&(0));
        return v8_base_Vector_const_char_::v8_base_Vector_const_char_1(
            { ({ v8_base_Vector_const_char_Impl::begin(self) }).offset((*from.borrow()) as isize) },
            { (*to.borrow()).wrapping_sub((*from.borrow())) },
        );
    }
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
                        V8_Fatal_115(
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
    fn empty(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).length_.borrow()) == 0_usize);
    }
    fn operator_index(&self, index: usize) -> Ptr<u8> {
        let index: Value<usize> = Rc::new(RefCell::new(index));
        (&(0));
        return (*(*(*self).upgrade().deref()).start_.borrow()).offset((*index.borrow()) as isize);
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
        return ({ hash_value_97((v.read())) });
    }
}
pub trait v8_base_hash_unsigned_long_long_Impl {
    fn operator_call(&self, v: Ptr<u64>) -> usize;
}
impl v8_base_hash_unsigned_long_long_Impl for Ptr<v8_base_hash_unsigned_long_long_> {
    fn operator_call(&self, v: Ptr<u64>) -> usize {
        return ({ hash_value_98((v.read())) });
    }
}
