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
    pub static value_17: Value<bool> = Rc::new(RefCell::new(false));
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
thread_local!(
    pub static value_18: Value<bool> = Rc::new(RefCell::new(true));
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
pub struct v8_base_is_unsigned_vs_signed_unsigned_long__int_ {}
impl Clone for v8_base_is_unsigned_vs_signed_unsigned_long__int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_unsigned_vs_signed_unsigned_long__int_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_unsigned_vs_signed_unsigned_long__int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_unsigned_vs_signed_unsigned_long__int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn CmpLEImpl_19(lhs: i32, rhs: u64) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return ((*lhs.borrow()) <= 0)
        || ((((*lhs.borrow()) as u32) as u64) <= ((*rhs.borrow()) as u64));
}
pub fn CmpGTImpl_20(lhs: i32, rhs: u64) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return !({ CmpLEImpl_19((*lhs.borrow()), (*rhs.borrow())) });
}
pub fn CmpLTImpl_21(lhs: u64, rhs: i32) -> bool {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    return ({ CmpGTImpl_20((*rhs.borrow()), (*lhs.borrow())) });
}
pub fn FormatBytesToHex_22(
    formatted: Ptr<u8>,
    size_of_formatted: usize,
    val: Ptr<u8>,
    size_of_val: usize,
) {
    let formatted: Value<Ptr<u8>> = Rc::new(RefCell::new(formatted));
    let size_of_formatted: Value<usize> = Rc::new(RefCell::new(size_of_formatted));
    let val: Value<Ptr<u8>> = Rc::new(RefCell::new(val));
    let size_of_val: Value<usize> = Rc::new(RefCell::new(size_of_val));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let _cmp: Value<bool> = Rc::new(RefCell::new(
            ({ CmpLTImpl_21(((*size_of_val.borrow()) as u64), (536870912)) }),
        ));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if ((!(!(!(*_cmp.borrow()))) as i64) != 0) {
                ({
                    V8_Fatal_23(
                        Ptr::from_string_literal(b"Check failed: %s."),
                        &[(Ptr::from_string_literal(b"size_of_val < 0x20000000")).into()],
                    )
                });
            }
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if ((!(!(!((*size_of_formatted.borrow())
            >= ((*size_of_val.borrow()).wrapping_mul(2_usize))))) as i64)
            != 0)
        {
            ({
                V8_Fatal_23(
                    Ptr::from_string_literal(b"Check failed: %s."),
                    &[
                        (Ptr::from_string_literal(b"size_of_formatted >= (size_of_val * 2)"))
                            .into(),
                    ],
                )
            });
        }
    }
    let index: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*index.borrow()) < (*size_of_val.borrow())) {
        let dest_index: Value<usize> = Rc::new(RefCell::new(((*index.borrow()) << 1)));
        {
            let __s = format_c(
                &Ptr::from_string_literal(b"%02x").to_rust_string(),
                &[(((*val.borrow()).offset((*index.borrow()) as isize).read()) as i32).into()],
            );
            let __b = __s.as_bytes();
            if (*size_of_formatted.borrow()).wrapping_sub((*dest_index.borrow())) > 0 {
                let __n = ::std::cmp::min(
                    __b.len(),
                    (*size_of_formatted.borrow()).wrapping_sub((*dest_index.borrow())) - 1,
                );
                ((*formatted.borrow()).offset((*dest_index.borrow()) as isize)).with_slice_mut(
                    __n + 1,
                    |__dst| {
                        __dst[..__n].copy_from_slice(&__b[..__n]);
                        __dst[__n] = 0;
                    },
                );
            }
            __b.len() as i32
        };
        (*index.borrow_mut()).postfix_inc();
    }
}
