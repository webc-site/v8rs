use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
pub fn HardeningAbort_1() {
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
pub fn is_constant_evaluated_2() -> bool {
    return ({ is_constant_evaluated_3() });
}
pub fn compare_result_as_less_than_4(r: std_weak_ordering) -> bool {
    let r: Value<std_weak_ordering> = Rc::new(RefCell::new(r));
    return operator_lt(
        (*r.borrow()).clone(),
        std__CmpUnspecifiedParam::std__CmpUnspecifiedParam({ 0 }),
    );
}
pub fn compare_result_as_ordering_5(c: std_weak_ordering) -> std_weak_ordering {
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
            lo_: Rc::new(RefCell::new(({ Int128Low64_6((*v.borrow()).clone()) }))),
            hi_: Rc::new(RefCell::new(
                (({ Int128High64_7((*v.borrow()).clone()) }) as u64),
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
            operator_cmp_8(
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
            operator_eq_9(
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
pub fn Uint128Max_10() -> absl_uint128 {
    return absl_uint128::absl_uint12810({ <u64>::MAX }, { <u64>::MAX });
}
thread_local!(
    pub static is_specialized_11: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_signed_12: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static is_integer_13: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_exact_14: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static has_infinity_15: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_quiet_NaN_16: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_signaling_NaN_17: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_denorm_18: Value = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static has_denorm_loss_19: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static round_style_20: Value = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static is_iec559_21: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static is_bounded_22: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_modulo_23: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static digits_24: Value<i32> = Rc::new(RefCell::new(128));
);
thread_local!(
    pub static digits10_25: Value<i32> = Rc::new(RefCell::new(38));
);
thread_local!(
    pub static max_digits10_26: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static radix_27: Value<i32> = Rc::new(RefCell::new(2));
);
thread_local!(
    pub static min_exponent_28: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static min_exponent10_29: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static max_exponent_30: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static max_exponent10_31: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static traps_32: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static tinyness_before_33: Value<bool> = Rc::new(RefCell::new(false));
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
        return ({ Uint128Max_10() });
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
                (({ BitCastToSigned_34((((*high.borrow()) as u128) << 64)) })
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
            operator_cmp_35(
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
            operator_eq_36(
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
pub fn Int128Max_37() -> absl_int128 {
    return absl_int128::absl_int12813({ <i64>::MAX }, { <u64>::MAX });
}
pub fn Int128Min_38() -> absl_int128 {
    return absl_int128::absl_int12813({ <i64>::MIN }, { 0_u64 });
}
thread_local!(
    pub static is_specialized_39: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_signed_40: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_integer_41: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_exact_42: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static has_infinity_43: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_quiet_NaN_44: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_signaling_NaN_45: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static has_denorm_46: Value = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static has_denorm_loss_47: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static round_style_48: Value = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static is_iec559_49: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static is_bounded_50: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_modulo_51: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static digits_52: Value<i32> = Rc::new(RefCell::new(127));
);
thread_local!(
    pub static digits10_53: Value<i32> = Rc::new(RefCell::new(38));
);
thread_local!(
    pub static max_digits10_54: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static radix_55: Value<i32> = Rc::new(RefCell::new(2));
);
thread_local!(
    pub static min_exponent_56: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static min_exponent10_57: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static max_exponent_58: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static max_exponent10_59: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static traps_60: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static tinyness_before_61: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct std_numeric_limits_absl_int128_ {}
impl std_numeric_limits_absl_int128_ {
    pub fn min() -> absl_int128 {
        return ({ Int128Min_38() });
    }
    pub fn lowest() -> absl_int128 {
        return ({ Int128Min_38() });
    }
    pub fn max() -> absl_int128 {
        return ({ Int128Max_37() });
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
pub fn MakeUint128_62(high: u64, low: u64) -> absl_uint128 {
    let high: Value<u64> = Rc::new(RefCell::new(high));
    let low: Value<u64> = Rc::new(RefCell::new(low));
    return absl_uint128::absl_uint12810({ (*high.borrow()) }, { (*low.borrow()) });
}
pub fn Uint128Low64_70(v: absl_uint128) -> u64 {
    let v: Value<absl_uint128> = Rc::new(RefCell::new(v));
    return (*(*v.borrow()).lo_.borrow());
}
pub fn Uint128High64_71(v: absl_uint128) -> u64 {
    let v: Value<absl_uint128> = Rc::new(RefCell::new(v));
    return (*(*v.borrow()).hi_.borrow());
}
pub fn operator_eq_9(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
        == ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }));
}
pub fn operator_ne_72(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_uint128 = (*lhs.borrow()).clone();
        operator_eq_9(_lhs, (*rhs.borrow()).clone())
    });
}
pub fn operator_lt_73(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
        < ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }));
}
pub fn operator_gt_74(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_uint128 = (*rhs.borrow()).clone();
        operator_lt_73(_lhs, (*lhs.borrow()).clone())
    });
}
pub fn operator_le_75(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_uint128 = (*rhs.borrow()).clone();
        operator_lt_73(_lhs, (*lhs.borrow()).clone())
    });
}
pub fn operator_ge_76(lhs: absl_uint128, rhs: absl_uint128) -> bool {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_uint128 = (*lhs.borrow()).clone();
        operator_lt_73(_lhs, (*rhs.borrow()).clone())
    });
}
pub fn operator_cmp_8(lhs: absl_uint128, rhs: absl_uint128) -> std::cmp::Ordering {
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
pub fn operator_pos_77(val: absl_uint128) -> absl_uint128 {
    let val: Value<absl_uint128> = Rc::new(RefCell::new(val));
    return (*val.borrow()).clone();
}
pub fn operator_pos_78(val: absl_int128) -> absl_int128 {
    let val: Value<absl_int128> = Rc::new(RefCell::new(val));
    return (*val.borrow()).clone();
}
pub fn operator_neg_79(val: absl_uint128) -> absl_uint128 {
    let val: Value<absl_uint128> = Rc::new(RefCell::new(val));
    return absl_uint128::absl_uint1288({
        -({ absl_uint128Impl::operator_unsigned___int128(&val.as_pointer()) })
    });
}
pub fn operator_not_80(val: absl_uint128) -> bool {
    let val: Value<absl_uint128> = Rc::new(RefCell::new(val));
    return !(({ absl_uint128Impl::operator_unsigned___int128(&val.as_pointer()) }) != 0);
}
pub fn operator_bitnot_81(val: absl_uint128) -> absl_uint128 {
    let val: Value<absl_uint128> = Rc::new(RefCell::new(val));
    return absl_uint128::absl_uint1288({
        !({ absl_uint128Impl::operator_unsigned___int128(&val.as_pointer()) })
    });
}
pub fn operator_bitor_82(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            | ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitand_83(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            & ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitxor_84(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            ^ ({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_shl_63(lhs: absl_uint128, amount: i32) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let amount: Value<i32> = Rc::new(RefCell::new(amount));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            << (*amount.borrow()))
    });
}
pub fn operator_shr_64(lhs: absl_uint128, amount: i32) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let amount: Value<i32> = Rc::new(RefCell::new(amount));
    return absl_uint128::absl_uint1288({
        (({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            >> (*amount.borrow()))
    });
}
pub fn operator_add_65(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_add(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_sub_66(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_sub(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_mul_67(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_mul(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_div_68(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_div(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_rem_69(lhs: absl_uint128, rhs: absl_uint128) -> absl_uint128 {
    let lhs: Value<absl_uint128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_uint128> = Rc::new(RefCell::new(rhs));
    return absl_uint128::absl_uint1288({
        ({ absl_uint128Impl::operator_unsigned___int128(&lhs.as_pointer()) })
            .wrapping_rem(({ absl_uint128Impl::operator_unsigned___int128(&rhs.as_pointer()) }))
    });
}
pub fn MakeInt128_85(high: i64, low: u64) -> absl_int128 {
    let high: Value<i64> = Rc::new(RefCell::new(high));
    let low: Value<u64> = Rc::new(RefCell::new(low));
    return absl_int128::absl_int12813({ (*high.borrow()) }, { (*low.borrow()) });
}
pub fn BitCastToSigned_96(v: u64) -> i64 {
    let v: Value<u64> = Rc::new(RefCell::new(v));
    return if (((*v.borrow()) & (1_u64 << 63)) != 0) {
        !(!(*v.borrow()) as i64)
    } else {
        ((*v.borrow()) as i64)
    };
}
pub fn BitCastToSigned_34(v: u128) -> i128 {
    let v: Value<u128> = Rc::new(RefCell::new(v));
    return if (((*v.borrow()) & (1_u128 << 127)) != 0) {
        !(!(*v.borrow()) as i128)
    } else {
        ((*v.borrow()) as i128)
    };
}
pub fn Int128Low64_6(v: absl_int128) -> u64 {
    let v: Value<absl_int128> = Rc::new(RefCell::new(v));
    return (((*(*v.borrow()).v_.borrow()) & (!0_u64 as i128)) as u64);
}
pub fn Int128High64_7(v: absl_int128) -> i64 {
    let v: Value<absl_int128> = Rc::new(RefCell::new(v));
    return ({ BitCastToSigned_96(((((*(*v.borrow()).v_.borrow()) as u128) >> 64) as u64)) });
}
pub fn operator_eq_36(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        == ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_ne_97(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        != ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_lt_98(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        < ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_gt_99(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        > ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_le_100(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        <= ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_ge_101(lhs: absl_int128, rhs: absl_int128) -> bool {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
        >= ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }));
}
pub fn operator_cmp_35(lhs: absl_int128, rhs: absl_int128) -> std::cmp::Ordering {
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
pub fn operator_neg_102(v: absl_int128) -> absl_int128 {
    let v: Value<absl_int128> = Rc::new(RefCell::new(v));
    return absl_int128::absl_int1288({
        -({ absl_int128Impl::operator___int128(&v.as_pointer()) })
    });
}
pub fn operator_not_103(v: absl_int128) -> bool {
    let v: Value<absl_int128> = Rc::new(RefCell::new(v));
    return !(({ absl_int128Impl::operator___int128(&v.as_pointer()) }) != 0);
}
pub fn operator_bitnot_104(val: absl_int128) -> absl_int128 {
    let val: Value<absl_int128> = Rc::new(RefCell::new(val));
    return absl_int128::absl_int1288({
        !({ absl_int128Impl::operator___int128(&val.as_pointer()) })
    });
}
pub fn operator_add_86(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            + ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_sub_87(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            - ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_mul_88(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            * ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_div_89(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            / ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_rem_90(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            % ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitor_91(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            | ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitand_92(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            & ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_bitxor_93(lhs: absl_int128, rhs: absl_int128) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_int128> = Rc::new(RefCell::new(rhs));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) })
            ^ ({ absl_int128Impl::operator___int128(&rhs.as_pointer()) }))
    });
}
pub fn operator_shl_94(lhs: absl_int128, amount: i32) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let amount: Value<i32> = Rc::new(RefCell::new(amount));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) }) << (*amount.borrow()))
    });
}
pub fn operator_shr_95(lhs: absl_int128, amount: i32) -> absl_int128 {
    let lhs: Value<absl_int128> = Rc::new(RefCell::new(lhs));
    let amount: Value<i32> = Rc::new(RefCell::new(amount));
    return absl_int128::absl_int1288({
        (({ absl_int128Impl::operator___int128(&lhs.as_pointer()) }) >> (*amount.borrow()))
    });
}
pub type v8_base_AbortMode = i32;
pub const v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures: v8_base_AbortMode = 0;
pub const v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures: v8_base_AbortMode = 1;
pub const v8_base_AbortMode_kExitIfNoSecurityImpact: v8_base_AbortMode = 2;
pub const v8_base_AbortMode_kImmediateCrash: v8_base_AbortMode = 3;
pub const v8_base_AbortMode_kDefault: v8_base_AbortMode = 4;
thread_local!();
pub fn ControlledCrashesAreHarmless_106() -> bool {
    return ((*g_abort_mode_105.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_105.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn DcheckFailuresAreIgnored_107() -> bool {
    return ((*g_abort_mode_105.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_105.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn FatalErrorsWithNoSecurityImpactShouldExit_108() -> bool {
    return ((*g_abort_mode_105.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitIfNoSecurityImpact);
}
thread_local!(
    pub static kReturnAddressStackSlotCount_109: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kPageSizeBits_110: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
    pub static kRegularPageSize_111: Value<i32> = Rc::new(RefCell::new(262144));
);
thread_local!(
    pub static kMinimumOSPageSize_112: Value<i32> = Rc::new(RefCell::new(16384));
);
thread_local!(
    pub static kUnimplementedCodeMessage_113: Value<Ptr<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"unimplemented code"),
    ));
);
thread_local!(
    pub static kUnreachableCodeMessage_114: Value<Ptr<u8>> =
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
    pub static is_enum_115: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static is_enum_116: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_117: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_118: Value<bool> = Rc::new(RefCell::new(true));
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
    pub static value_119: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_120: Value<bool> = Rc::new(RefCell::new(false));
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
thread_local!(
    pub static kSignificandSize_121: Value<i32> = Rc::new(RefCell::new(64));
);
thread_local!(
    static kUint64MSB_122: Value<u64> = Rc::new(RefCell::new((1_u64 << 63)));
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
                operator_mul_67(_lhs, _rhs)
            }),
        ));
        let hi: Value<u64> = Rc::new(RefCell::new(
            ({ Uint128High64_71((*mul.borrow()).clone()) }),
        ));
        let lo: Value<u64> = Rc::new(RefCell::new(({ Uint128Low64_70((*mul.borrow()).clone()) })));
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
pub fn make_uint64_126(high: u32, low: u32) -> u64 {
    let high: Value<u32> = Rc::new(RefCell::new(high));
    let low: Value<u32> = Rc::new(RefCell::new(low));
    return (((*high.borrow()) as u64) << 32).wrapping_add(((*low.borrow()) as u64));
}
pub fn RoundDown_127(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    return ((*x.borrow()) & (-(*m.borrow()) as u64));
}
pub fn RoundUp_128(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    (&(0));
    return ({
        let _x: u64 = ((*x.borrow()).wrapping_add((((*m.borrow()) - 1_i64) as u64)) as u64);
        let _m: i64 = (*m.borrow());
        RoundDown_127(_x, _m)
    });
}
pub fn IsAligned_129(value: u64, alignment: u64) -> bool {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let alignment: Value<u64> = Rc::new(RefCell::new(alignment));
    return (((*value.borrow()) & ((*alignment.borrow()).wrapping_sub(1_u64))) == 0_u64);
}
#[derive(Default)]
pub struct v8_base_CachedPower {
    pub significand: Value<u64>,
    pub binary_exponent: Value<i16>,
    pub decimal_exponent: Value<i16>,
}
impl Clone for v8_base_CachedPower {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_CachedPower> = Rc::new(RefCell::new(Self {
            significand: Rc::new(RefCell::new((*self.significand.borrow()))),
            binary_exponent: Rc::new(RefCell::new((*self.binary_exponent.borrow()))),
            decimal_exponent: Rc::new(RefCell::new((*self.decimal_exponent.borrow()))),
        }));
        let this: Ptr<v8_base_CachedPower> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_CachedPower {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.significand.borrow()).to_bytes(&mut buf[0..8]);
        (*self.binary_exponent.borrow()).to_bytes(&mut buf[8..10]);
        (*self.decimal_exponent.borrow()).to_bytes(&mut buf[10..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            significand: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
            binary_exponent: Rc::new(RefCell::new(<i16>::from_bytes(&buf[8..10]))),
            decimal_exponent: Rc::new(RefCell::new(<i16>::from_bytes(&buf[10..12]))),
        }
    }
}
thread_local!(
    pub static kCachedPowers_130: Value<Box<[v8_base_CachedPower]>> =
        Rc::new(RefCell::new(Box::new([
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(18054884314459144840_u64)),
                binary_exponent: Rc::new(RefCell::new((-1220_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-348_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(13451937075301367670_u64)),
                binary_exponent: Rc::new(RefCell::new((-1193_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-340_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(10022474136428063862_u64)),
                binary_exponent: Rc::new(RefCell::new((-1166_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-332_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(14934650266808366570_u64)),
                binary_exponent: Rc::new(RefCell::new((-1140_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-324_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(11127181549972568877_u64)),
                binary_exponent: Rc::new(RefCell::new((-1113_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-316_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(16580792590934885855_u64)),
                binary_exponent: Rc::new(RefCell::new((-1087_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-308_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(12353653155963782858_u64)),
                binary_exponent: Rc::new(RefCell::new((-1060_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-300_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(18408377700990114895_u64)),
                binary_exponent: Rc::new(RefCell::new((-1034_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-292_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(13715310171984221708_u64)),
                binary_exponent: Rc::new(RefCell::new((-1007_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-284_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(10218702384817765436_u64)),
                binary_exponent: Rc::new(RefCell::new((-980_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-276_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(15227053142812498563_u64)),
                binary_exponent: Rc::new(RefCell::new((-954_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-268_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(11345038669416679861_u64)),
                binary_exponent: Rc::new(RefCell::new((-927_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-260_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(16905424996341287883_u64)),
                binary_exponent: Rc::new(RefCell::new((-901_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-252_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(12595523146049147757_u64)),
                binary_exponent: Rc::new(RefCell::new((-874_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-244_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(9384396036005875287_u64)),
                binary_exponent: Rc::new(RefCell::new((-847_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-236_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(13983839803942852151_u64)),
                binary_exponent: Rc::new(RefCell::new((-821_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-228_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(10418772551374772303_u64)),
                binary_exponent: Rc::new(RefCell::new((-794_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-220_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(15525180923007089351_u64)),
                binary_exponent: Rc::new(RefCell::new((-768_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-212_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(11567161174868858868_u64)),
                binary_exponent: Rc::new(RefCell::new((-741_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-204_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(17236413322193710309_u64)),
                binary_exponent: Rc::new(RefCell::new((-715_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-196_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(12842128665889583758_u64)),
                binary_exponent: Rc::new(RefCell::new((-688_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-188_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(9568131466127621947_u64)),
                binary_exponent: Rc::new(RefCell::new((-661_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-180_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(14257626930069360058_u64)),
                binary_exponent: Rc::new(RefCell::new((-635_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-172_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(10622759856335341974_u64)),
                binary_exponent: Rc::new(RefCell::new((-608_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-164_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(15829145694278690180_u64)),
                binary_exponent: Rc::new(RefCell::new((-582_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-156_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(11793632577567316726_u64)),
                binary_exponent: Rc::new(RefCell::new((-555_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-148_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(17573882009934360870_u64)),
                binary_exponent: Rc::new(RefCell::new((-529_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-140_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(13093562431584567480_u64)),
                binary_exponent: Rc::new(RefCell::new((-502_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-132_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(9755464219737475723_u64)),
                binary_exponent: Rc::new(RefCell::new((-475_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-124_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(14536774485912137811_u64)),
                binary_exponent: Rc::new(RefCell::new((-449_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-116_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(10830740992659433045_u64)),
                binary_exponent: Rc::new(RefCell::new((-422_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-108_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(16139061738043178685_u64)),
                binary_exponent: Rc::new(RefCell::new((-396_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-100_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(12024538023802026127_u64)),
                binary_exponent: Rc::new(RefCell::new((-369_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-92_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(17917957937422433684_u64)),
                binary_exponent: Rc::new(RefCell::new((-343_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-84_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(13349918974505688015_u64)),
                binary_exponent: Rc::new(RefCell::new((-316_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-76_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(9946464728195732843_u64)),
                binary_exponent: Rc::new(RefCell::new((-289_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-68_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(14821387422376473014_u64)),
                binary_exponent: Rc::new(RefCell::new((-263_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-60_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(11042794154864902060_u64)),
                binary_exponent: Rc::new(RefCell::new((-236_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-52_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(16455045573212060422_u64)),
                binary_exponent: Rc::new(RefCell::new((-210_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-44_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(12259964326927110867_u64)),
                binary_exponent: Rc::new(RefCell::new((-183_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-36_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(18268770466636286478_u64)),
                binary_exponent: Rc::new(RefCell::new((-157_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-28_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(13611294676837538539_u64)),
                binary_exponent: Rc::new(RefCell::new((-130_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-20_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(10141204801825835212_u64)),
                binary_exponent: Rc::new(RefCell::new((-103_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-12_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(15111572745182864684_u64)),
                binary_exponent: Rc::new(RefCell::new((-77_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new((-4_i32 as i16))),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(11258999068426240000_u64)),
                binary_exponent: Rc::new(RefCell::new((-50_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new(4_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(16777216000000000000_u64)),
                binary_exponent: Rc::new(RefCell::new((-24_i32 as i16))),
                decimal_exponent: Rc::new(RefCell::new(12_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(12500000000000000000_u64)),
                binary_exponent: Rc::new(RefCell::new(3_i16)),
                decimal_exponent: Rc::new(RefCell::new(20_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(9313225746154785156_u64)),
                binary_exponent: Rc::new(RefCell::new(30_i16)),
                decimal_exponent: Rc::new(RefCell::new(28_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(13877787807814456755_u64)),
                binary_exponent: Rc::new(RefCell::new(56_i16)),
                decimal_exponent: Rc::new(RefCell::new(36_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(10339757656912845936_u64)),
                binary_exponent: Rc::new(RefCell::new(83_i16)),
                decimal_exponent: Rc::new(RefCell::new(44_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(15407439555097886824_u64)),
                binary_exponent: Rc::new(RefCell::new(109_i16)),
                decimal_exponent: Rc::new(RefCell::new(52_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(11479437019748901445_u64)),
                binary_exponent: Rc::new(RefCell::new(136_i16)),
                decimal_exponent: Rc::new(RefCell::new(60_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(17105694144590052135_u64)),
                binary_exponent: Rc::new(RefCell::new(162_i16)),
                decimal_exponent: Rc::new(RefCell::new(68_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(12744735289059618216_u64)),
                binary_exponent: Rc::new(RefCell::new(189_i16)),
                decimal_exponent: Rc::new(RefCell::new(76_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(9495567745759798747_u64)),
                binary_exponent: Rc::new(RefCell::new(216_i16)),
                decimal_exponent: Rc::new(RefCell::new(84_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(14149498560666738074_u64)),
                binary_exponent: Rc::new(RefCell::new(242_i16)),
                decimal_exponent: Rc::new(RefCell::new(92_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(10542197943230523224_u64)),
                binary_exponent: Rc::new(RefCell::new(269_i16)),
                decimal_exponent: Rc::new(RefCell::new(100_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(15709099088952724970_u64)),
                binary_exponent: Rc::new(RefCell::new(295_i16)),
                decimal_exponent: Rc::new(RefCell::new(108_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(11704190886730495818_u64)),
                binary_exponent: Rc::new(RefCell::new(322_i16)),
                decimal_exponent: Rc::new(RefCell::new(116_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(17440603504673385349_u64)),
                binary_exponent: Rc::new(RefCell::new(348_i16)),
                decimal_exponent: Rc::new(RefCell::new(124_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(12994262207056124023_u64)),
                binary_exponent: Rc::new(RefCell::new(375_i16)),
                decimal_exponent: Rc::new(RefCell::new(132_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(9681479787123295682_u64)),
                binary_exponent: Rc::new(RefCell::new(402_i16)),
                decimal_exponent: Rc::new(RefCell::new(140_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(14426529090290212157_u64)),
                binary_exponent: Rc::new(RefCell::new(428_i16)),
                decimal_exponent: Rc::new(RefCell::new(148_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(10748601772107342003_u64)),
                binary_exponent: Rc::new(RefCell::new(455_i16)),
                decimal_exponent: Rc::new(RefCell::new(156_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(16016664761464807395_u64)),
                binary_exponent: Rc::new(RefCell::new(481_i16)),
                decimal_exponent: Rc::new(RefCell::new(164_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(11933345169920330789_u64)),
                binary_exponent: Rc::new(RefCell::new(508_i16)),
                decimal_exponent: Rc::new(RefCell::new(172_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(17782069995880619868_u64)),
                binary_exponent: Rc::new(RefCell::new(534_i16)),
                decimal_exponent: Rc::new(RefCell::new(180_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(13248674568444952270_u64)),
                binary_exponent: Rc::new(RefCell::new(561_i16)),
                decimal_exponent: Rc::new(RefCell::new(188_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(9871031767461413346_u64)),
                binary_exponent: Rc::new(RefCell::new(588_i16)),
                decimal_exponent: Rc::new(RefCell::new(196_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(14708983551653345445_u64)),
                binary_exponent: Rc::new(RefCell::new(614_i16)),
                decimal_exponent: Rc::new(RefCell::new(204_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(10959046745042015199_u64)),
                binary_exponent: Rc::new(RefCell::new(641_i16)),
                decimal_exponent: Rc::new(RefCell::new(212_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(16330252207878254650_u64)),
                binary_exponent: Rc::new(RefCell::new(667_i16)),
                decimal_exponent: Rc::new(RefCell::new(220_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(12166986024289022870_u64)),
                binary_exponent: Rc::new(RefCell::new(694_i16)),
                decimal_exponent: Rc::new(RefCell::new(228_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(18130221999122236476_u64)),
                binary_exponent: Rc::new(RefCell::new(720_i16)),
                decimal_exponent: Rc::new(RefCell::new(236_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(13508068024458167312_u64)),
                binary_exponent: Rc::new(RefCell::new(747_i16)),
                decimal_exponent: Rc::new(RefCell::new(244_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(10064294952495520794_u64)),
                binary_exponent: Rc::new(RefCell::new(774_i16)),
                decimal_exponent: Rc::new(RefCell::new(252_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(14996968138956309548_u64)),
                binary_exponent: Rc::new(RefCell::new(800_i16)),
                decimal_exponent: Rc::new(RefCell::new(260_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(11173611982879273257_u64)),
                binary_exponent: Rc::new(RefCell::new(827_i16)),
                decimal_exponent: Rc::new(RefCell::new(268_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(16649979327439178909_u64)),
                binary_exponent: Rc::new(RefCell::new(853_i16)),
                decimal_exponent: Rc::new(RefCell::new(276_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(12405201291620119593_u64)),
                binary_exponent: Rc::new(RefCell::new(880_i16)),
                decimal_exponent: Rc::new(RefCell::new(284_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(9242595204427927429_u64)),
                binary_exponent: Rc::new(RefCell::new(907_i16)),
                decimal_exponent: Rc::new(RefCell::new(292_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(13772540099066387757_u64)),
                binary_exponent: Rc::new(RefCell::new(933_i16)),
                decimal_exponent: Rc::new(RefCell::new(300_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(10261342003245940623_u64)),
                binary_exponent: Rc::new(RefCell::new(960_i16)),
                decimal_exponent: Rc::new(RefCell::new(308_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(15290591125556738113_u64)),
                binary_exponent: Rc::new(RefCell::new(986_i16)),
                decimal_exponent: Rc::new(RefCell::new(316_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(11392378155556871081_u64)),
                binary_exponent: Rc::new(RefCell::new(1013_i16)),
                decimal_exponent: Rc::new(RefCell::new(324_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(16975966327722178521_u64)),
                binary_exponent: Rc::new(RefCell::new(1039_i16)),
                decimal_exponent: Rc::new(RefCell::new(332_i16)),
            },
            v8_base_CachedPower {
                significand: Rc::new(RefCell::new(12648080533535911531_u64)),
                binary_exponent: Rc::new(RefCell::new(1066_i16)),
                decimal_exponent: Rc::new(RefCell::new(340_i16)),
            },
        ])));
);
thread_local!(
    pub static kCachedPowersOffset_131: Value<i32> = Rc::new(RefCell::new(348));
);
thread_local!(
    pub static kD_1_LOG2_10_132: Value<f64> = Rc::new(RefCell::new(3.010299957E-1));
);
thread_local!(
    pub static kDecimalExponentDistance_123: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kMinDecimalExponent_124: Value<i32> = Rc::new(RefCell::new(-348_i32));
);
thread_local!(
    pub static kMaxDecimalExponent_125: Value<i32> = Rc::new(RefCell::new(340));
);
impl v8_base_PowersOfTenCache {
    pub fn GetCachedPowerForBinaryExponentRange(
        min_exponent: i32,
        max_exponent: i32,
        power: Ptr<v8_base_DiyFp>,
        decimal_exponent: Ptr<i32>,
    ) {
        let min_exponent: Value<i32> = Rc::new(RefCell::new(min_exponent));
        let max_exponent: Value<i32> = Rc::new(RefCell::new(max_exponent));
        let power: Value<Ptr<v8_base_DiyFp>> = Rc::new(RefCell::new(power));
        let decimal_exponent: Value<Ptr<i32>> = Rc::new(RefCell::new(decimal_exponent));
        let kQ: Value<i32> = Rc::new(RefCell::new(
            (*kSignificandSize_121.with(Value::clone).borrow()),
        ));
        let k: Value<f64> = Rc::new(RefCell::new(
            ({
                ceil_133(
                    (((((*min_exponent.borrow()) + (*kQ.borrow())) - 1) as f64)
                        * (*kD_1_LOG2_10_132.with(Value::clone).borrow())),
                )
            }),
        ));
        let foo: Value<i32> = Rc::new(RefCell::new(
            (*kCachedPowersOffset_131.with(Value::clone).borrow()),
        ));
        let index: Value<i32> = Rc::new(RefCell::new(
            (((((*foo.borrow()) + ((*k.borrow()) as i32)) - 1)
                / (*kDecimalExponentDistance_123.with(Value::clone).borrow()))
                + 1),
        ));
        (&(0));
        let cached_power: Value<v8_base_CachedPower> = Rc::new(RefCell::new(
            ((*kCachedPowers_130.with(Value::clone).borrow())[(*index.borrow()) as usize]).clone(),
        ));
        (&(0));
        (&(0));
        let __rhs = ((*(*cached_power.borrow()).decimal_exponent.borrow()) as i32);
        (*decimal_exponent.borrow()).write(__rhs);
        let __rhs =
            v8_base_DiyFp::v8_base_DiyFp2({ (*(*cached_power.borrow()).significand.borrow()) }, {
                ((*(*cached_power.borrow()).binary_exponent.borrow()) as i32)
            });
        (*power.borrow()).write(__rhs);
    }
}
impl v8_base_PowersOfTenCache {
    pub fn GetCachedPowerForDecimalExponent(
        requested_exponent: i32,
        power: Ptr<v8_base_DiyFp>,
        found_exponent: Ptr<i32>,
    ) {
        let requested_exponent: Value<i32> = Rc::new(RefCell::new(requested_exponent));
        let power: Value<Ptr<v8_base_DiyFp>> = Rc::new(RefCell::new(power));
        let found_exponent: Value<Ptr<i32>> = Rc::new(RefCell::new(found_exponent));
        (&(0));
        (&(0));
        let index: Value<i32> = Rc::new(RefCell::new(
            (((*requested_exponent.borrow())
                + (*kCachedPowersOffset_131.with(Value::clone).borrow()))
                / (*kDecimalExponentDistance_123.with(Value::clone).borrow())),
        ));
        let cached_power: Value<v8_base_CachedPower> = Rc::new(RefCell::new(
            ((*kCachedPowers_130.with(Value::clone).borrow())[(*index.borrow()) as usize]).clone(),
        ));
        let __rhs =
            v8_base_DiyFp::v8_base_DiyFp2({ (*(*cached_power.borrow()).significand.borrow()) }, {
                ((*(*cached_power.borrow()).binary_exponent.borrow()) as i32)
            });
        (*power.borrow()).write(__rhs);
        let __rhs = ((*(*cached_power.borrow()).decimal_exponent.borrow()) as i32);
        (*found_exponent.borrow()).write(__rhs);
        (&(0));
        (&(0));
    }
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
                operator_add_86(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_sub_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_sub_87(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_mul_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_mul_88(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_div_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_div_89(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_rem_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_rem_90(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitor_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_bitor_91(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitand_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_bitand_92(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitxor_assign(&self, other: absl_int128) -> Ptr<absl_int128> {
        let other: Value<absl_int128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_bitxor_93(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_shl_assign(&self, amount: i32) -> Ptr<absl_int128> {
        let amount: Value<i32> = Rc::new(RefCell::new(amount));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_shl_94(_lhs, (*amount.borrow()))
            }),
        );
        return (*self).clone();
    }
    fn operator_shr_assign(&self, amount: i32) -> Ptr<absl_int128> {
        let amount: Value<i32> = Rc::new(RefCell::new(amount));
        (*self).write(
            ({
                let _lhs: absl_int128 = (*(*self).upgrade().deref()).clone();
                operator_shr_95(_lhs, (*amount.borrow()))
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
                operator_shl_63(_lhs, (*amount.borrow()))
            }),
        );
        return (*self).clone();
    }
    fn operator_shr_assign(&self, amount: i32) -> Ptr<absl_uint128> {
        let amount: Value<i32> = Rc::new(RefCell::new(amount));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_shr_64(_lhs, (*amount.borrow()))
            }),
        );
        return (*self).clone();
    }
    fn operator_add_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_add_65(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_sub_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_sub_66(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_mul_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_mul_67(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_div_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_div_68(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_rem_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_rem_69(_lhs, (*other.borrow()).clone())
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
                operator_bitor_82(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitand_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_bitand_83(_lhs, (*other.borrow()).clone())
            }),
        );
        return (*self).clone();
    }
    fn operator_bitxor_assign(&self, other: absl_uint128) -> Ptr<absl_uint128> {
        let other: Value<absl_uint128> = Rc::new(RefCell::new(other));
        (*self).write(
            ({
                let _lhs: absl_uint128 = (*(*self).upgrade().deref()).clone();
                operator_bitxor_84(_lhs, (*other.borrow()).clone())
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
        'loop_: while (((*f.borrow()) & (*kUint64MSB_122.with(Value::clone).borrow())) == 0_u64) {
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
