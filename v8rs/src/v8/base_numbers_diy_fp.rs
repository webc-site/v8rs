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
