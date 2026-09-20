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
pub type absl_ConstInitType = u32;
pub const absl_ConstInitType_kConstInit: absl_ConstInitType = 0;
thread_local!(
    pub static kLowZeroBits_21: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kAlignment_22: Value<i32> = Rc::new(RefCell::new(256));
);
pub type absl_base_internal_PerThreadSynch_State = u32;
pub const absl_base_internal_PerThreadSynch_State_kAvailable:
    absl_base_internal_PerThreadSynch_State = 0;
pub const absl_base_internal_PerThreadSynch_State_kQueued: absl_base_internal_PerThreadSynch_State =
    1;
#[derive(Default)]
pub struct absl_base_internal_PerThreadSynch {
    pub next: Value<Ptr<absl_base_internal_PerThreadSynch>>,
    pub skip: Value<Ptr<absl_base_internal_PerThreadSynch>>,
    pub may_skip: Value<bool>,
    pub wake: Value<bool>,
    pub cond_waiter: Value<bool>,
    pub maybe_unlocking: Value<bool>,
    pub suppress_fatal_errors: Value<bool>,
    pub priority: Value<i32>,
    pub state: Value<std_atomic_absl_base_internal_PerThreadSynch_State_>,
    pub waitp: Value<Ptr<absl_SynchWaitParams>>,
    pub readers: Value<i64>,
    pub next_priority_read_cycles: Value<i64>,
    pub all_locks: Value<Ptr<absl_SynchLocksHeld>>,
}
impl ByteRepr for absl_base_internal_PerThreadSynch {
    fn byte_size() -> usize {
        64
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.next.borrow()).to_bytes(&mut buf[0..8]);
        (*self.skip.borrow()).to_bytes(&mut buf[8..16]);
        (*self.may_skip.borrow()).to_bytes(&mut buf[16..17]);
        (*self.wake.borrow()).to_bytes(&mut buf[17..18]);
        (*self.cond_waiter.borrow()).to_bytes(&mut buf[18..19]);
        (*self.maybe_unlocking.borrow()).to_bytes(&mut buf[19..20]);
        (*self.suppress_fatal_errors.borrow()).to_bytes(&mut buf[20..21]);
        (*self.priority.borrow()).to_bytes(&mut buf[24..28]);
        (*self.state.borrow()).to_bytes(&mut buf[28..32]);
        (*self.waitp.borrow()).to_bytes(&mut buf[32..40]);
        (*self.readers.borrow()).to_bytes(&mut buf[40..48]);
        (*self.next_priority_read_cycles.borrow()).to_bytes(&mut buf[48..56]);
        (*self.all_locks.borrow()).to_bytes(&mut buf[56..64]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            next: Rc::new(RefCell::new(
                <Ptr<absl_base_internal_PerThreadSynch>>::from_bytes(&buf[0..8]),
            )),
            skip: Rc::new(RefCell::new(
                <Ptr<absl_base_internal_PerThreadSynch>>::from_bytes(&buf[8..16]),
            )),
            may_skip: Rc::new(RefCell::new(<bool>::from_bytes(&buf[16..17]))),
            wake: Rc::new(RefCell::new(<bool>::from_bytes(&buf[17..18]))),
            cond_waiter: Rc::new(RefCell::new(<bool>::from_bytes(&buf[18..19]))),
            maybe_unlocking: Rc::new(RefCell::new(<bool>::from_bytes(&buf[19..20]))),
            suppress_fatal_errors: Rc::new(RefCell::new(<bool>::from_bytes(&buf[20..21]))),
            priority: Rc::new(RefCell::new(<i32>::from_bytes(&buf[24..28]))),
            state: Rc::new(RefCell::new(
                <std_atomic_absl_base_internal_PerThreadSynch_State_>::from_bytes(&buf[28..32]),
            )),
            waitp: Rc::new(RefCell::new(<Ptr<absl_SynchWaitParams>>::from_bytes(
                &buf[32..40],
            ))),
            readers: Rc::new(RefCell::new(<i64>::from_bytes(&buf[40..48]))),
            next_priority_read_cycles: Rc::new(RefCell::new(<i64>::from_bytes(&buf[48..56]))),
            all_locks: Rc::new(RefCell::new(<Ptr<absl_SynchLocksHeld>>::from_bytes(
                &buf[56..64],
            ))),
        }
    }
}
pub type absl_base_internal_ThreadIdentity_WaitState = u8;
pub const absl_base_internal_ThreadIdentity_WaitState_kActive:
    absl_base_internal_ThreadIdentity_WaitState = 0;
pub const absl_base_internal_ThreadIdentity_WaitState_kWaitingForWork:
    absl_base_internal_ThreadIdentity_WaitState = 1;
thread_local!(
    pub static kToBePaddedSize_23: Value<usize> = Rc::new(RefCell::new(33));
);
#[derive(Default)]
pub struct absl_base_internal_ThreadIdentity_SchedulerState {
    pub bound_schedulable: Value<std_atomic_void_ptr_>,
    pub association_lock_word: Value<u32>,
    pub scheduling_disabled_depth: Value<std_atomic_int_>,
    pub potentially_blocking_depth: Value<i32>,
    pub schedule_next_state: Value<u32>,
    pub waking_designated_waker: Value<bool>,
}
impl ByteRepr for absl_base_internal_ThreadIdentity_SchedulerState {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.bound_schedulable.borrow()).to_bytes(&mut buf[0..8]);
        (*self.association_lock_word.borrow()).to_bytes(&mut buf[8..12]);
        (*self.scheduling_disabled_depth.borrow()).to_bytes(&mut buf[12..16]);
        (*self.potentially_blocking_depth.borrow()).to_bytes(&mut buf[16..20]);
        (*self.schedule_next_state.borrow()).to_bytes(&mut buf[20..24]);
        (*self.waking_designated_waker.borrow()).to_bytes(&mut buf[24..25]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            bound_schedulable: Rc::new(RefCell::new(<std_atomic_void_ptr_>::from_bytes(
                &buf[0..8],
            ))),
            association_lock_word: Rc::new(RefCell::new(<u32>::from_bytes(&buf[8..12]))),
            scheduling_disabled_depth: Rc::new(RefCell::new(<std_atomic_int_>::from_bytes(
                &buf[12..16],
            ))),
            potentially_blocking_depth: Rc::new(RefCell::new(<i32>::from_bytes(&buf[16..20]))),
            schedule_next_state: Rc::new(RefCell::new(<u32>::from_bytes(&buf[20..24]))),
            waking_designated_waker: Rc::new(RefCell::new(<bool>::from_bytes(&buf[24..25]))),
        }
    }
}
#[derive()]
pub struct absl_base_internal_ThreadIdentity_WaiterState {
    pub data: Value<Box<[u8]>>,
}
impl Clone for absl_base_internal_ThreadIdentity_WaiterState {
    fn clone(&self) -> Self {
        let __this: Value<absl_base_internal_ThreadIdentity_WaiterState> =
            Rc::new(RefCell::new(Self {
                data: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 256, _>(
                    |__i: usize| (*self.data.borrow())[(__i) as usize],
                )))),
            }));
        let this: Ptr<absl_base_internal_ThreadIdentity_WaiterState> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_base_internal_ThreadIdentity_WaiterState {
    fn default() -> Self {
        absl_base_internal_ThreadIdentity_WaiterState {
            data: Rc::new(RefCell::new(
                (0..256).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
        }
    }
}
impl ByteRepr for absl_base_internal_ThreadIdentity_WaiterState {
    fn byte_size() -> usize {
        256
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.data.borrow()).to_bytes(&mut buf[0..256]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[0..256]))),
        }
    }
}
#[derive()]
pub struct absl_base_internal_ThreadIdentity {
    pub per_thread_synch: Value<absl_base_internal_PerThreadSynch>,
    pub scheduler_state: Value<absl_base_internal_ThreadIdentity_SchedulerState>,
    pub wait_state: Value<std_atomic_absl_base_internal_ThreadIdentity_WaitState_>,
    pub padding: Value<Box<[u8]>>,
    pub waiter_state: Value<absl_base_internal_ThreadIdentity_WaiterState>,
    pub blocked_count_ptr: Value<Ptr<std_atomic_int_>>,
    pub ticker: Value<std_atomic_int_>,
    pub wait_start: Value<std_atomic_int_>,
    pub is_idle: Value<std_atomic_bool_>,
    pub static_initialization_depth: Value<i32>,
    pub next: Value<Ptr<absl_base_internal_ThreadIdentity>>,
}
impl Default for absl_base_internal_ThreadIdentity {
    fn default() -> Self {
        absl_base_internal_ThreadIdentity {
            per_thread_synch: <Value<absl_base_internal_PerThreadSynch>>::default(),
            scheduler_state: <Value<absl_base_internal_ThreadIdentity_SchedulerState>>::default(),
            wait_state: <Value<std_atomic_absl_base_internal_ThreadIdentity_WaitState_>>::default(),
            padding: Rc::new(RefCell::new(
                (0..31).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
            waiter_state: <Value<absl_base_internal_ThreadIdentity_WaiterState>>::default(),
            blocked_count_ptr: Rc::new(RefCell::new(Ptr::<std_atomic_int_>::null())),
            ticker: <Value<std_atomic_int_>>::default(),
            wait_start: <Value<std_atomic_int_>>::default(),
            is_idle: <Value<std_atomic_bool_>>::default(),
            static_initialization_depth: <Value<i32>>::default(),
            next: Rc::new(RefCell::new(
                Ptr::<absl_base_internal_ThreadIdentity>::null(),
            )),
        }
    }
}
impl ByteRepr for absl_base_internal_ThreadIdentity {
    fn byte_size() -> usize {
        416
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.per_thread_synch.borrow()).to_bytes(&mut buf[0..64]);
        (*self.scheduler_state.borrow()).to_bytes(&mut buf[64..96]);
        (*self.wait_state.borrow()).to_bytes(&mut buf[96..97]);
        (*self.padding.borrow()).to_bytes(&mut buf[97..128]);
        (*self.waiter_state.borrow()).to_bytes(&mut buf[128..384]);
        (*self.blocked_count_ptr.borrow()).to_bytes(&mut buf[384..392]);
        (*self.ticker.borrow()).to_bytes(&mut buf[392..396]);
        (*self.wait_start.borrow()).to_bytes(&mut buf[396..400]);
        (*self.is_idle.borrow()).to_bytes(&mut buf[400..401]);
        (*self.static_initialization_depth.borrow()).to_bytes(&mut buf[404..408]);
        (*self.next.borrow()).to_bytes(&mut buf[408..416]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            per_thread_synch: Rc::new(RefCell::new(
                <absl_base_internal_PerThreadSynch>::from_bytes(&buf[0..64]),
            )),
            scheduler_state: Rc::new(RefCell::new(
                <absl_base_internal_ThreadIdentity_SchedulerState>::from_bytes(&buf[64..96]),
            )),
            wait_state: Rc::new(RefCell::new(
                <std_atomic_absl_base_internal_ThreadIdentity_WaitState_>::from_bytes(&buf[96..97]),
            )),
            padding: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[97..128]))),
            waiter_state: Rc::new(RefCell::new(
                <absl_base_internal_ThreadIdentity_WaiterState>::from_bytes(&buf[128..384]),
            )),
            blocked_count_ptr: Rc::new(RefCell::new(<Ptr<std_atomic_int_>>::from_bytes(
                &buf[384..392],
            ))),
            ticker: Rc::new(RefCell::new(<std_atomic_int_>::from_bytes(&buf[392..396]))),
            wait_start: Rc::new(RefCell::new(<std_atomic_int_>::from_bytes(&buf[396..400]))),
            is_idle: Rc::new(RefCell::new(<std_atomic_bool_>::from_bytes(&buf[400..401]))),
            static_initialization_depth: Rc::new(RefCell::new(<i32>::from_bytes(&buf[404..408]))),
            next: Rc::new(RefCell::new(
                <Ptr<absl_base_internal_ThreadIdentity>>::from_bytes(&buf[408..416]),
            )),
        }
    }
}
thread_local!();
pub fn HardeningAbort_25() {
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
pub fn is_constant_evaluated_26() -> bool {
    return ({ is_constant_evaluated_27() });
}
pub type absl_LogSeverity = i32;
pub const absl_LogSeverity_kInfo: absl_LogSeverity = 0;
pub const absl_LogSeverity_kWarning: absl_LogSeverity = 1;
pub const absl_LogSeverity_kError: absl_LogSeverity = 2;
pub const absl_LogSeverity_kFatal: absl_LogSeverity = 3;
pub fn LogSeverities_28() -> Vec<absl_LogSeverity> {
    return vec![
        absl_LogSeverity_kInfo,
        absl_LogSeverity_kWarning,
        absl_LogSeverity_kError,
        absl_LogSeverity_kFatal,
    ];
}
thread_local!(
    pub static kLogDebugFatal_29: Value<absl_LogSeverity> = Rc::new(RefCell::new(2));
);
pub fn LogSeverityName_30(s: absl_LogSeverity) -> Ptr<u8> {
    let s: Value<absl_LogSeverity> = Rc::new(RefCell::new(s));
    'switch: {
        let __match_cond = (*s.borrow());
        match __match_cond {
            __v if __v == 0 => {
                return Ptr::from_string_literal(b"INFO");
            }
            __v if __v == 1 => {
                return Ptr::from_string_literal(b"WARNING");
            }
            __v if __v == 2 => {
                return Ptr::from_string_literal(b"ERROR");
            }
            __v if __v == 3 => {
                return Ptr::from_string_literal(b"FATAL");
            }
            _ => {}
        }
    };
    return Ptr::from_string_literal(b"UNKNOWN");
}
pub fn NormalizeLogSeverity_31(s: absl_LogSeverity) -> absl_LogSeverity {
    let s: Value<absl_LogSeverity> = Rc::new(RefCell::new(s));
    let n: Value<absl_LogSeverity> = Rc::new(RefCell::new((*s.borrow())));
    if ((*n.borrow()) < absl_LogSeverity_kInfo) {
        (*n.borrow_mut()) = absl_LogSeverity_kInfo;
    }
    if ((*n.borrow()) > absl_LogSeverity_kFatal) {
        (*n.borrow_mut()) = absl_LogSeverity_kError;
    }
    return (*n.borrow());
}
pub fn NormalizeLogSeverity_32(s: i32) -> absl_LogSeverity {
    let s: Value<i32> = Rc::new(RefCell::new(s));
    return ({ NormalizeLogSeverity_31(((*s.borrow()) as absl_LogSeverity)) });
}
pub type absl_LogSeverityAtLeast = i32;
pub const absl_LogSeverityAtLeast_kInfo: absl_LogSeverityAtLeast = 0;
pub const absl_LogSeverityAtLeast_kWarning: absl_LogSeverityAtLeast = 1;
pub const absl_LogSeverityAtLeast_kError: absl_LogSeverityAtLeast = 2;
pub const absl_LogSeverityAtLeast_kFatal: absl_LogSeverityAtLeast = 3;
pub const absl_LogSeverityAtLeast_kInfinity: absl_LogSeverityAtLeast = 1000;
pub type absl_LogSeverityAtMost = i32;
pub const absl_LogSeverityAtMost_kNegativeInfinity: absl_LogSeverityAtMost = -1000;
pub const absl_LogSeverityAtMost_kInfo: absl_LogSeverityAtMost = 0;
pub const absl_LogSeverityAtMost_kWarning: absl_LogSeverityAtMost = 1;
pub const absl_LogSeverityAtMost_kError: absl_LogSeverityAtMost = 2;
pub const absl_LogSeverityAtMost_kFatal: absl_LogSeverityAtMost = 3;
pub fn operator_gt_33(lhs: absl_LogSeverityAtLeast, rhs: absl_LogSeverity) -> bool {
    let lhs: Value<absl_LogSeverityAtLeast> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(rhs));
    return (((*lhs.borrow()) as absl_LogSeverity) > (*rhs.borrow()));
}
pub fn operator_lt_34(lhs: absl_LogSeverity, rhs: absl_LogSeverityAtLeast) -> bool {
    let lhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverityAtLeast> = Rc::new(RefCell::new(rhs));
    return ((*lhs.borrow()) < ((*rhs.borrow()) as absl_LogSeverity));
}
pub fn operator_le_35(lhs: absl_LogSeverityAtLeast, rhs: absl_LogSeverity) -> bool {
    let lhs: Value<absl_LogSeverityAtLeast> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(rhs));
    return (((*lhs.borrow()) as absl_LogSeverity) <= (*rhs.borrow()));
}
pub fn operator_ge_36(lhs: absl_LogSeverity, rhs: absl_LogSeverityAtLeast) -> bool {
    let lhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverityAtLeast> = Rc::new(RefCell::new(rhs));
    return ((*lhs.borrow()) >= ((*rhs.borrow()) as absl_LogSeverity));
}
pub fn operator_lt_37(lhs: absl_LogSeverityAtMost, rhs: absl_LogSeverity) -> bool {
    let lhs: Value<absl_LogSeverityAtMost> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(rhs));
    return (((*lhs.borrow()) as absl_LogSeverity) < (*rhs.borrow()));
}
pub fn operator_gt_38(lhs: absl_LogSeverity, rhs: absl_LogSeverityAtMost) -> bool {
    let lhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverityAtMost> = Rc::new(RefCell::new(rhs));
    return ((*lhs.borrow()) > ((*rhs.borrow()) as absl_LogSeverity));
}
pub fn operator_ge_39(lhs: absl_LogSeverityAtMost, rhs: absl_LogSeverity) -> bool {
    let lhs: Value<absl_LogSeverityAtMost> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(rhs));
    return (((*lhs.borrow()) as absl_LogSeverity) >= (*rhs.borrow()));
}
pub fn operator_le_40(lhs: absl_LogSeverity, rhs: absl_LogSeverityAtMost) -> bool {
    let lhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverityAtMost> = Rc::new(RefCell::new(rhs));
    return ((*lhs.borrow()) <= ((*rhs.borrow()) as absl_LogSeverity));
}
pub fn Basename_41(fname: Ptr<u8>, offset: i32) -> Ptr<u8> {
    let fname: Value<Ptr<u8>> = Rc::new(RefCell::new(fname));
    let offset: Value<i32> = Rc::new(RefCell::new(offset));
    return if (((*offset.borrow()) == 0)
        || ((((*fname.borrow())
            .offset(((*offset.borrow()) - 1) as isize)
            .read()) as i32)
            == (('/' as u8) as i32)))
        || ((((*fname.borrow())
            .offset(((*offset.borrow()) - 1) as isize)
            .read()) as i32)
            == (('\\' as u8) as i32))
    {
        (*fname.borrow()).offset((*offset.borrow()) as isize)
    } else {
        ({ Basename_41((*fname.borrow()).clone(), ((*offset.borrow()) - 1)) })
    };
}
thread_local!();
pub fn ClippedSubstr_43(s: Vec<u8>, pos: usize, n: Option<usize>) -> Vec<u8> {
    let s: Value<Vec<u8>> = Rc::new(RefCell::new(s));
    let pos: Value<usize> = Rc::new(RefCell::new(pos));
    let n: Value<usize> = Rc::new(RefCell::new(n.unwrap_or(18446744073709551615)));
    let __rhs = ({
        let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*pos.borrow()) as u64)));
        let __tmp_1: Value<u64> = Rc::new(RefCell::new((((*s.borrow()).len() as usize) as u64)));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    } as usize);
    (*pos.borrow_mut()) = __rhs;
    return (*s.borrow())[(*pos.borrow()) as usize
        ..::std::cmp::min(
            (*pos.borrow()).saturating_add((*n.borrow())),
            (*s.borrow()).len(),
        )]
        .to_vec();
}
pub fn NullSafeStringView_44(p: Ptr<u8>) -> Vec<u8> {
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(p));
    return if !(*p.borrow()).is_null() {
        (*p.borrow()).to_c_string_iterator().collect::<Vec<u8>>()
    } else {
        std_basic_string_view_char__std_char_traits_char__ :: std_basic_string_view_char__std_char_traits_char__1 ( )
    };
}
#[derive(Default)]
pub struct absl_time_internal_cctz_detail_fields {
    pub y: Value<i64>,
    pub m: Value<i8>,
    pub d: Value<i8>,
    pub hh: Value<i8>,
    pub mm: Value<i8>,
    pub ss: Value<i8>,
}
impl absl_time_internal_cctz_detail_fields {
    pub fn absl_time_internal_cctz_detail_fields(
        year: i64,
        month: i8,
        day: i8,
        hour: i8,
        minute: i8,
        second: i8,
    ) -> Self {
        let year: Value<i64> = Rc::new(RefCell::new(year));
        let month: Value<i8> = Rc::new(RefCell::new(month));
        let day: Value<i8> = Rc::new(RefCell::new(day));
        let hour: Value<i8> = Rc::new(RefCell::new(hour));
        let minute: Value<i8> = Rc::new(RefCell::new(minute));
        let second: Value<i8> = Rc::new(RefCell::new(second));
        let __this: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(Self {
            y: Rc::new(RefCell::new((*year.borrow()))),
            m: Rc::new(RefCell::new((*month.borrow()))),
            d: Rc::new(RefCell::new((*day.borrow()))),
            hh: Rc::new(RefCell::new((*hour.borrow()))),
            mm: Rc::new(RefCell::new((*minute.borrow()))),
            ss: Rc::new(RefCell::new((*second.borrow()))),
        }));
        let this: Ptr<absl_time_internal_cctz_detail_fields> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for absl_time_internal_cctz_detail_fields {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(Self {
            y: Rc::new(RefCell::new((*self.y.borrow()))),
            m: Rc::new(RefCell::new((*self.m.borrow()))),
            d: Rc::new(RefCell::new((*self.d.borrow()))),
            hh: Rc::new(RefCell::new((*self.hh.borrow()))),
            mm: Rc::new(RefCell::new((*self.mm.borrow()))),
            ss: Rc::new(RefCell::new((*self.ss.borrow()))),
        }));
        let this: Ptr<absl_time_internal_cctz_detail_fields> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_cctz_detail_fields {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.y.borrow()).to_bytes(&mut buf[0..8]);
        (*self.m.borrow()).to_bytes(&mut buf[8..9]);
        (*self.d.borrow()).to_bytes(&mut buf[9..10]);
        (*self.hh.borrow()).to_bytes(&mut buf[10..11]);
        (*self.mm.borrow()).to_bytes(&mut buf[11..12]);
        (*self.ss.borrow()).to_bytes(&mut buf[12..13]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            y: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
            m: Rc::new(RefCell::new(<i8>::from_bytes(&buf[8..9]))),
            d: Rc::new(RefCell::new(<i8>::from_bytes(&buf[9..10]))),
            hh: Rc::new(RefCell::new(<i8>::from_bytes(&buf[10..11]))),
            mm: Rc::new(RefCell::new(<i8>::from_bytes(&buf[11..12]))),
            ss: Rc::new(RefCell::new(<i8>::from_bytes(&buf[12..13]))),
        }
    }
}
#[derive(Default)]
pub struct absl_time_internal_cctz_detail_second_tag {}
impl Clone for absl_time_internal_cctz_detail_second_tag {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_detail_second_tag> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_time_internal_cctz_detail_second_tag> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_cctz_detail_second_tag {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct absl_time_internal_cctz_detail_minute_tag {}
impl Clone for absl_time_internal_cctz_detail_minute_tag {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_detail_minute_tag> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_time_internal_cctz_detail_minute_tag> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_cctz_detail_minute_tag {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct absl_time_internal_cctz_detail_hour_tag {}
impl Clone for absl_time_internal_cctz_detail_hour_tag {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_detail_hour_tag> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_time_internal_cctz_detail_hour_tag> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_cctz_detail_hour_tag {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct absl_time_internal_cctz_detail_day_tag {}
impl Clone for absl_time_internal_cctz_detail_day_tag {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_detail_day_tag> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_time_internal_cctz_detail_day_tag> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_cctz_detail_day_tag {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct absl_time_internal_cctz_detail_month_tag {}
impl Clone for absl_time_internal_cctz_detail_month_tag {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_detail_month_tag> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_time_internal_cctz_detail_month_tag> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_cctz_detail_month_tag {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct absl_time_internal_cctz_detail_year_tag {}
impl Clone for absl_time_internal_cctz_detail_year_tag {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_detail_year_tag> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_time_internal_cctz_detail_year_tag> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_cctz_detail_year_tag {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn is_leap_year_45(y: i64) -> bool {
    let y: Value<i64> = Rc::new(RefCell::new(y));
    return (((*y.borrow()) % 4_i64) == 0_i64)
        && ((((*y.borrow()) % 100_i64) != 0_i64) || (((*y.borrow()) % 400_i64) == 0_i64));
}
pub fn year_index_46(y: i64, m: i8) -> i32 {
    let y: Value<i64> = Rc::new(RefCell::new(y));
    let m: Value<i8> = Rc::new(RefCell::new(m));
    let yi: Value<i32> = Rc::new(RefCell::new(
        ((((*y.borrow()) + ((((*m.borrow()) as i32) > 2) as i64)) % 400_i64) as i32),
    ));
    return if ((*yi.borrow()) < 0) {
        ((*yi.borrow()) + 400)
    } else {
        (*yi.borrow())
    };
}
pub fn days_per_century_47(yi: i32) -> i32 {
    let yi: Value<i32> = Rc::new(RefCell::new(yi));
    return (36524 + ((((*yi.borrow()) == 0) || ((*yi.borrow()) > 300)) as i32));
}
pub fn days_per_4years_48(yi: i32) -> i32 {
    let yi: Value<i32> = Rc::new(RefCell::new(yi));
    return (1460
        + (((((*yi.borrow()) == 0) || ((*yi.borrow()) > 300))
            || ((((*yi.borrow()) - 1) % 100) < 96)) as i32));
}
pub fn days_per_year_49(y: i64, m: i8) -> i32 {
    let y: Value<i64> = Rc::new(RefCell::new(y));
    let m: Value<i8> = Rc::new(RefCell::new(m));
    return if ({ is_leap_year_45(((*y.borrow()) + ((((*m.borrow()) as i32) > 2) as i64))) }) {
        366
    } else {
        365
    };
}
pub fn days_per_month_50(y: i64, m: i8) -> i32 {
    let y: Value<i64> = Rc::new(RefCell::new(y));
    let m: Value<i8> = Rc::new(RefCell::new(m));
    let k_days_per_month: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        -1_i32, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    ])));
    return ((*k_days_per_month.borrow())[(*m.borrow()) as usize]
        + (((((*m.borrow()) as i32) == 2) && ({ is_leap_year_45((*y.borrow())) })) as i32));
}
pub fn n_day_51(
    y: i64,
    m: i8,
    d: i64,
    cd: i64,
    hh: i8,
    mm: i8,
    ss: i8,
) -> absl_time_internal_cctz_detail_fields {
    let y: Value<i64> = Rc::new(RefCell::new(y));
    let m: Value<i8> = Rc::new(RefCell::new(m));
    let d: Value<i64> = Rc::new(RefCell::new(d));
    let cd: Value<i64> = Rc::new(RefCell::new(cd));
    let hh: Value<i8> = Rc::new(RefCell::new(hh));
    let mm: Value<i8> = Rc::new(RefCell::new(mm));
    let ss: Value<i8> = Rc::new(RefCell::new(ss));
    let ey: Value<i64> = Rc::new(RefCell::new(((*y.borrow()) % 400_i64)));
    let oey: Value<i64> = Rc::new(RefCell::new((*ey.borrow())));
    (*ey.borrow_mut()) += (((*cd.borrow()) / 146097_i64) * 400_i64);
    (*cd.borrow_mut()) %= 146097_i64;
    if ((*cd.borrow()) < 0_i64) {
        (*ey.borrow_mut()) -= 400_i64;
        (*cd.borrow_mut()) += 146097_i64;
    }
    (*ey.borrow_mut()) += (((*d.borrow()) / 146097_i64) * 400_i64);
    let __rhs = (((*d.borrow()) % 146097_i64) + (*cd.borrow()));
    (*d.borrow_mut()) = __rhs;
    if ((*d.borrow()) > 0_i64) {
        if ((*d.borrow()) > 146097_i64) {
            (*ey.borrow_mut()) += 400_i64;
            (*d.borrow_mut()) -= 146097_i64;
        }
    } else {
        if ((*d.borrow()) > (-365_i32 as i64)) {
            (*ey.borrow_mut()) -= 1_i64;
            (*d.borrow_mut()) += (({ days_per_year_49((*ey.borrow()), (*m.borrow())) }) as i64);
        } else {
            (*ey.borrow_mut()) -= 400_i64;
            (*d.borrow_mut()) += 146097_i64;
        }
    }
    if ((*d.borrow()) > 365_i64) {
        let yi: Value<i32> = Rc::new(RefCell::new(
            ({ year_index_46((*ey.borrow()), (*m.borrow())) }),
        ));
        'loop_: while true {
            let n: Value<i32> = Rc::new(RefCell::new(({ days_per_century_47((*yi.borrow())) })));
            if ((*d.borrow()) <= ((*n.borrow()) as i64)) {
                break;
            }
            (*d.borrow_mut()) -= ((*n.borrow()) as i64);
            (*ey.borrow_mut()) += 100_i64;
            (*yi.borrow_mut()) += 100;
            if ((*yi.borrow()) >= 400) {
                (*yi.borrow_mut()) -= 400;
            };
        }
        'loop_: while true {
            let n: Value<i32> = Rc::new(RefCell::new(({ days_per_4years_48((*yi.borrow())) })));
            if ((*d.borrow()) <= ((*n.borrow()) as i64)) {
                break;
            }
            (*d.borrow_mut()) -= ((*n.borrow()) as i64);
            (*ey.borrow_mut()) += 4_i64;
            (*yi.borrow_mut()) += 4;
            if ((*yi.borrow()) >= 400) {
                (*yi.borrow_mut()) -= 400;
            };
        }
        'loop_: while true {
            let n: Value<i32> = Rc::new(RefCell::new(
                ({ days_per_year_49((*ey.borrow()), (*m.borrow())) }),
            ));
            if ((*d.borrow()) <= ((*n.borrow()) as i64)) {
                break;
            }
            (*d.borrow_mut()) -= ((*n.borrow()) as i64);
            (*ey.borrow_mut()).prefix_inc();
        }
    }
    if ((*d.borrow()) > 28_i64) {
        'loop_: while true {
            let n: Value<i32> = Rc::new(RefCell::new(
                ({ days_per_month_50((*ey.borrow()), (*m.borrow())) }),
            ));
            if ((*d.borrow()) <= ((*n.borrow()) as i64)) {
                break;
            }
            (*d.borrow_mut()) -= ((*n.borrow()) as i64);
            if (((*m.borrow_mut()).prefix_inc() as i32) > 12) {
                (*ey.borrow_mut()).prefix_inc();
                (*m.borrow_mut()) = 1_i8;
            };
        }
    }
    return absl_time_internal_cctz_detail_fields::absl_time_internal_cctz_detail_fields(
        { ((*y.borrow()) + ((*ey.borrow()) - (*oey.borrow()))) },
        { (*m.borrow()) },
        { ((*d.borrow()) as i8) },
        { (*hh.borrow()) },
        { (*mm.borrow()) },
        { (*ss.borrow()) },
    );
}
pub fn n_mon_52(
    y: i64,
    m: i64,
    d: i64,
    cd: i64,
    hh: i8,
    mm: i8,
    ss: i8,
) -> absl_time_internal_cctz_detail_fields {
    let y: Value<i64> = Rc::new(RefCell::new(y));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    let d: Value<i64> = Rc::new(RefCell::new(d));
    let cd: Value<i64> = Rc::new(RefCell::new(cd));
    let hh: Value<i8> = Rc::new(RefCell::new(hh));
    let mm: Value<i8> = Rc::new(RefCell::new(mm));
    let ss: Value<i8> = Rc::new(RefCell::new(ss));
    if ((*m.borrow()) != 12_i64) {
        (*y.borrow_mut()) += ((*m.borrow()) / 12_i64);
        (*m.borrow_mut()) %= 12_i64;
        if ((*m.borrow()) <= 0_i64) {
            (*y.borrow_mut()) -= 1_i64;
            (*m.borrow_mut()) += 12_i64;
        }
    }
    return ({
        n_day_51(
            (*y.borrow()),
            ((*m.borrow()) as i8),
            (*d.borrow()),
            (*cd.borrow()),
            (*hh.borrow()),
            (*mm.borrow()),
            (*ss.borrow()),
        )
    });
}
pub fn n_hour_53(
    y: i64,
    m: i64,
    d: i64,
    cd: i64,
    hh: i64,
    mm: i8,
    ss: i8,
) -> absl_time_internal_cctz_detail_fields {
    let y: Value<i64> = Rc::new(RefCell::new(y));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    let d: Value<i64> = Rc::new(RefCell::new(d));
    let cd: Value<i64> = Rc::new(RefCell::new(cd));
    let hh: Value<i64> = Rc::new(RefCell::new(hh));
    let mm: Value<i8> = Rc::new(RefCell::new(mm));
    let ss: Value<i8> = Rc::new(RefCell::new(ss));
    (*cd.borrow_mut()) += ((*hh.borrow()) / 24_i64);
    (*hh.borrow_mut()) %= 24_i64;
    if ((*hh.borrow()) < 0_i64) {
        (*cd.borrow_mut()) -= 1_i64;
        (*hh.borrow_mut()) += 24_i64;
    }
    return ({
        n_mon_52(
            (*y.borrow()),
            (*m.borrow()),
            (*d.borrow()),
            (*cd.borrow()),
            ((*hh.borrow()) as i8),
            (*mm.borrow()),
            (*ss.borrow()),
        )
    });
}
pub fn n_min_54(
    y: i64,
    m: i64,
    d: i64,
    hh: i64,
    ch: i64,
    mm: i64,
    ss: i8,
) -> absl_time_internal_cctz_detail_fields {
    let y: Value<i64> = Rc::new(RefCell::new(y));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    let d: Value<i64> = Rc::new(RefCell::new(d));
    let hh: Value<i64> = Rc::new(RefCell::new(hh));
    let ch: Value<i64> = Rc::new(RefCell::new(ch));
    let mm: Value<i64> = Rc::new(RefCell::new(mm));
    let ss: Value<i8> = Rc::new(RefCell::new(ss));
    (*ch.borrow_mut()) += ((*mm.borrow()) / 60_i64);
    (*mm.borrow_mut()) %= 60_i64;
    if ((*mm.borrow()) < 0_i64) {
        (*ch.borrow_mut()) -= 1_i64;
        (*mm.borrow_mut()) += 60_i64;
    }
    return ({
        let _cd: i64 = (((*hh.borrow()) / 24_i64) + ((*ch.borrow()) / 24_i64));
        let _hh: i64 = (((*hh.borrow()) % 24_i64) + ((*ch.borrow()) % 24_i64));
        n_hour_53(
            (*y.borrow()),
            (*m.borrow()),
            (*d.borrow()),
            _cd,
            _hh,
            ((*mm.borrow()) as i8),
            (*ss.borrow()),
        )
    });
}
pub fn n_sec_55(
    y: i64,
    m: i64,
    d: i64,
    hh: i64,
    mm: i64,
    ss: i64,
) -> absl_time_internal_cctz_detail_fields {
    let y: Value<i64> = Rc::new(RefCell::new(y));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    let d: Value<i64> = Rc::new(RefCell::new(d));
    let hh: Value<i64> = Rc::new(RefCell::new(hh));
    let mm: Value<i64> = Rc::new(RefCell::new(mm));
    let ss: Value<i64> = Rc::new(RefCell::new(ss));
    if (0_i64 <= (*ss.borrow())) && ((*ss.borrow()) < 60_i64) {
        let nss: Value<i8> = Rc::new(RefCell::new(((*ss.borrow()) as i8)));
        if (0_i64 <= (*mm.borrow())) && ((*mm.borrow()) < 60_i64) {
            let nmm: Value<i8> = Rc::new(RefCell::new(((*mm.borrow()) as i8)));
            if (0_i64 <= (*hh.borrow())) && ((*hh.borrow()) < 24_i64) {
                let nhh: Value<i8> = Rc::new(RefCell::new(((*hh.borrow()) as i8)));
                if (((1_i64 <= (*d.borrow())) && ((*d.borrow()) <= 28_i64))
                    && (1_i64 <= (*m.borrow())))
                    && ((*m.borrow()) <= 12_i64)
                {
                    let nd: Value<i8> = Rc::new(RefCell::new(((*d.borrow()) as i8)));
                    let nm: Value<i8> = Rc::new(RefCell::new(((*m.borrow()) as i8)));
                    return absl_time_internal_cctz_detail_fields :: absl_time_internal_cctz_detail_fields ( {  (*y.borrow())   } , {  (*nm.borrow())   } , {  (*nd.borrow())   } , {  (*nhh.borrow())   } , {  (*nmm.borrow())   } , {  (*nss.borrow())   } , )   ;
                }
                return ({
                    n_mon_52(
                        (*y.borrow()),
                        (*m.borrow()),
                        (*d.borrow()),
                        0_i64,
                        (*nhh.borrow()),
                        (*nmm.borrow()),
                        (*nss.borrow()),
                    )
                });
            }
            return ({
                let _cd: i64 = ((*hh.borrow()) / 24_i64);
                let _hh: i64 = ((*hh.borrow()) % 24_i64);
                n_hour_53(
                    (*y.borrow()),
                    (*m.borrow()),
                    (*d.borrow()),
                    _cd,
                    _hh,
                    (*nmm.borrow()),
                    (*nss.borrow()),
                )
            });
        }
        return ({
            let _ch: i64 = ((*mm.borrow()) / 60_i64);
            let _mm: i64 = ((*mm.borrow()) % 60_i64);
            n_min_54(
                (*y.borrow()),
                (*m.borrow()),
                (*d.borrow()),
                (*hh.borrow()),
                _ch,
                _mm,
                (*nss.borrow()),
            )
        });
    }
    let cm: Value<i64> = Rc::new(RefCell::new(((*ss.borrow()) / 60_i64)));
    (*ss.borrow_mut()) %= 60_i64;
    if ((*ss.borrow()) < 0_i64) {
        (*cm.borrow_mut()) -= 1_i64;
        (*ss.borrow_mut()) += 60_i64;
    }
    return ({
        let _ch: i64 = (((*mm.borrow()) / 60_i64) + ((*cm.borrow()) / 60_i64));
        let _mm: i64 = (((*mm.borrow()) % 60_i64) + ((*cm.borrow()) % 60_i64));
        n_min_54(
            (*y.borrow()),
            (*m.borrow()),
            (*d.borrow()),
            (*hh.borrow()),
            _ch,
            _mm,
            ((*ss.borrow()) as i8),
        )
    });
}
pub fn step_56(
    _a0: absl_time_internal_cctz_detail_second_tag,
    f: absl_time_internal_cctz_detail_fields,
    n: i64,
) -> absl_time_internal_cctz_detail_fields {
    let _a0: Value<absl_time_internal_cctz_detail_second_tag> = Rc::new(RefCell::new(_a0));
    let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({
        let _y: i64 = (*(*f.borrow()).y.borrow());
        let _m: i64 = ((*(*f.borrow()).m.borrow()) as i64);
        let _d: i64 = ((*(*f.borrow()).d.borrow()) as i64);
        let _hh: i64 = ((*(*f.borrow()).hh.borrow()) as i64);
        let _mm: i64 = (((*(*f.borrow()).mm.borrow()) as i64) + ((*n.borrow()) / 60_i64));
        let _ss: i64 = (((*(*f.borrow()).ss.borrow()) as i64) + ((*n.borrow()) % 60_i64));
        n_sec_55(_y, _m, _d, _hh, _mm, _ss)
    });
}
pub fn step_57(
    _a0: absl_time_internal_cctz_detail_minute_tag,
    f: absl_time_internal_cctz_detail_fields,
    n: i64,
) -> absl_time_internal_cctz_detail_fields {
    let _a0: Value<absl_time_internal_cctz_detail_minute_tag> = Rc::new(RefCell::new(_a0));
    let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({
        let _y: i64 = (*(*f.borrow()).y.borrow());
        let _m: i64 = ((*(*f.borrow()).m.borrow()) as i64);
        let _d: i64 = ((*(*f.borrow()).d.borrow()) as i64);
        let _hh: i64 = (((*(*f.borrow()).hh.borrow()) as i64) + ((*n.borrow()) / 60_i64));
        let _mm: i64 = (((*(*f.borrow()).mm.borrow()) as i64) + ((*n.borrow()) % 60_i64));
        let _ss: i8 = (*(*f.borrow()).ss.borrow());
        n_min_54(_y, _m, _d, _hh, 0_i64, _mm, _ss)
    });
}
pub fn step_58(
    _a0: absl_time_internal_cctz_detail_hour_tag,
    f: absl_time_internal_cctz_detail_fields,
    n: i64,
) -> absl_time_internal_cctz_detail_fields {
    let _a0: Value<absl_time_internal_cctz_detail_hour_tag> = Rc::new(RefCell::new(_a0));
    let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({
        let _y: i64 = (*(*f.borrow()).y.borrow());
        let _m: i64 = ((*(*f.borrow()).m.borrow()) as i64);
        let _d: i64 = (((*(*f.borrow()).d.borrow()) as i64) + ((*n.borrow()) / 24_i64));
        let _hh: i64 = (((*(*f.borrow()).hh.borrow()) as i64) + ((*n.borrow()) % 24_i64));
        let _mm: i8 = (*(*f.borrow()).mm.borrow());
        let _ss: i8 = (*(*f.borrow()).ss.borrow());
        n_hour_53(_y, _m, _d, 0_i64, _hh, _mm, _ss)
    });
}
pub fn step_59(
    _a0: absl_time_internal_cctz_detail_day_tag,
    f: absl_time_internal_cctz_detail_fields,
    n: i64,
) -> absl_time_internal_cctz_detail_fields {
    let _a0: Value<absl_time_internal_cctz_detail_day_tag> = Rc::new(RefCell::new(_a0));
    let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({
        let _y: i64 = (*(*f.borrow()).y.borrow());
        let _m: i8 = (*(*f.borrow()).m.borrow());
        let _d: i64 = ((*(*f.borrow()).d.borrow()) as i64);
        let _hh: i8 = (*(*f.borrow()).hh.borrow());
        let _mm: i8 = (*(*f.borrow()).mm.borrow());
        let _ss: i8 = (*(*f.borrow()).ss.borrow());
        n_day_51(_y, _m, _d, (*n.borrow()), _hh, _mm, _ss)
    });
}
pub fn step_60(
    _a0: absl_time_internal_cctz_detail_month_tag,
    f: absl_time_internal_cctz_detail_fields,
    n: i64,
) -> absl_time_internal_cctz_detail_fields {
    let _a0: Value<absl_time_internal_cctz_detail_month_tag> = Rc::new(RefCell::new(_a0));
    let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({
        let _y: i64 = ((*(*f.borrow()).y.borrow()) + ((*n.borrow()) / 12_i64));
        let _m: i64 = (((*(*f.borrow()).m.borrow()) as i64) + ((*n.borrow()) % 12_i64));
        let _d: i64 = ((*(*f.borrow()).d.borrow()) as i64);
        let _hh: i8 = (*(*f.borrow()).hh.borrow());
        let _mm: i8 = (*(*f.borrow()).mm.borrow());
        let _ss: i8 = (*(*f.borrow()).ss.borrow());
        n_mon_52(_y, _m, _d, 0_i64, _hh, _mm, _ss)
    });
}
pub fn step_61(
    _a0: absl_time_internal_cctz_detail_year_tag,
    f: absl_time_internal_cctz_detail_fields,
    n: i64,
) -> absl_time_internal_cctz_detail_fields {
    let _a0: Value<absl_time_internal_cctz_detail_year_tag> = Rc::new(RefCell::new(_a0));
    let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return absl_time_internal_cctz_detail_fields::absl_time_internal_cctz_detail_fields(
        { ((*(*f.borrow()).y.borrow()) + (*n.borrow())) },
        { (*(*f.borrow()).m.borrow()) },
        { (*(*f.borrow()).d.borrow()) },
        { (*(*f.borrow()).hh.borrow()) },
        { (*(*f.borrow()).mm.borrow()) },
        { (*(*f.borrow()).ss.borrow()) },
    );
}
pub fn scale_add_62(v: i64, f: i64, a: i64) -> i64 {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let f: Value<i64> = Rc::new(RefCell::new(f));
    let a: Value<i64> = Rc::new(RefCell::new(a));
    return if ((*v.borrow()) < 0_i64) {
        (((((*v.borrow()) + 1_i64) * (*f.borrow())) + (*a.borrow())) - (*f.borrow()))
    } else {
        (((((*v.borrow()) - 1_i64) * (*f.borrow())) + (*a.borrow())) + (*f.borrow()))
    };
}
pub fn ymd_ord_63(y: i64, m: i8, d: i8) -> i64 {
    let y: Value<i64> = Rc::new(RefCell::new(y));
    let m: Value<i8> = Rc::new(RefCell::new(m));
    let d: Value<i8> = Rc::new(RefCell::new(d));
    let eyear: Value<i64> = Rc::new(RefCell::new(if (((*m.borrow()) as i32) <= 2) {
        ((*y.borrow()) - 1_i64)
    } else {
        (*y.borrow())
    }));
    let era: Value<i64> = Rc::new(RefCell::new(
        ((if ((*eyear.borrow()) >= 0_i64) {
            (*eyear.borrow())
        } else {
            ((*eyear.borrow()) - 399_i64)
        }) / 400_i64),
    ));
    let yoe: Value<i64> = Rc::new(RefCell::new(
        ((*eyear.borrow()) - ((*era.borrow()) * 400_i64)),
    ));
    let doy: Value<i64> = Rc::new(RefCell::new(
        ((((((153
            * (((*m.borrow()) as i32)
                + (if (((*m.borrow()) as i32) > 2) {
                    -3_i32
                } else {
                    9
                })))
            + 2)
            / 5)
            + ((*d.borrow()) as i32))
            - 1) as i64),
    ));
    let doe: Value<i64> = Rc::new(RefCell::new(
        (((((*yoe.borrow()) * 365_i64) + ((*yoe.borrow()) / 4_i64)) - ((*yoe.borrow()) / 100_i64))
            + (*doy.borrow())),
    ));
    return ((((*era.borrow()) * 146097_i64) + (*doe.borrow())) - 719468_i64);
}
pub fn day_difference_64(y1: i64, m1: i8, d1: i8, y2: i64, m2: i8, d2: i8) -> i64 {
    let y1: Value<i64> = Rc::new(RefCell::new(y1));
    let m1: Value<i8> = Rc::new(RefCell::new(m1));
    let d1: Value<i8> = Rc::new(RefCell::new(d1));
    let y2: Value<i64> = Rc::new(RefCell::new(y2));
    let m2: Value<i8> = Rc::new(RefCell::new(m2));
    let d2: Value<i8> = Rc::new(RefCell::new(d2));
    let a_c4_off: Value<i64> = Rc::new(RefCell::new(((*y1.borrow()) % 400_i64)));
    let b_c4_off: Value<i64> = Rc::new(RefCell::new(((*y2.borrow()) % 400_i64)));
    let c4_diff: Value<i64> = Rc::new(RefCell::new(
        (((*y1.borrow()) - (*a_c4_off.borrow())) - ((*y2.borrow()) - (*b_c4_off.borrow()))),
    ));
    let delta: Value<i64> = Rc::new(RefCell::new(
        (({ ymd_ord_63((*a_c4_off.borrow()), (*m1.borrow()), (*d1.borrow())) })
            - ({ ymd_ord_63((*b_c4_off.borrow()), (*m2.borrow()), (*d2.borrow())) })),
    ));
    if ((*c4_diff.borrow()) > 0_i64) && ((*delta.borrow()) < 0_i64) {
        (*delta.borrow_mut()) += ((2 * 146097) as i64);
        (*c4_diff.borrow_mut()) -= ((2 * 400) as i64);
    } else if ((*c4_diff.borrow()) < 0_i64) && ((*delta.borrow()) > 0_i64) {
        (*delta.borrow_mut()) -= ((2 * 146097) as i64);
        (*c4_diff.borrow_mut()) += ((2 * 400) as i64);
    }
    return ((((*c4_diff.borrow()) / 400_i64) * 146097_i64) + (*delta.borrow()));
}
pub fn difference_65(
    _a0: absl_time_internal_cctz_detail_year_tag,
    f1: absl_time_internal_cctz_detail_fields,
    f2: absl_time_internal_cctz_detail_fields,
) -> i64 {
    let _a0: Value<absl_time_internal_cctz_detail_year_tag> = Rc::new(RefCell::new(_a0));
    let f1: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f1));
    let f2: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f2));
    return ((*(*f1.borrow()).y.borrow()) - (*(*f2.borrow()).y.borrow()));
}
pub fn difference_66(
    _a0: absl_time_internal_cctz_detail_month_tag,
    f1: absl_time_internal_cctz_detail_fields,
    f2: absl_time_internal_cctz_detail_fields,
) -> i64 {
    let _a0: Value<absl_time_internal_cctz_detail_month_tag> = Rc::new(RefCell::new(_a0));
    let f1: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f1));
    let f2: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f2));
    return ({
        let _v: i64 = ({
            difference_65(
                absl_time_internal_cctz_detail_year_tag {},
                (*f1.borrow()).clone(),
                (*f2.borrow()).clone(),
            )
        });
        let _a: i64 = ((((*(*f1.borrow()).m.borrow()) as i32)
            - ((*(*f2.borrow()).m.borrow()) as i32)) as i64);
        scale_add_62(_v, 12_i64, _a)
    });
}
pub fn difference_67(
    _a0: absl_time_internal_cctz_detail_day_tag,
    f1: absl_time_internal_cctz_detail_fields,
    f2: absl_time_internal_cctz_detail_fields,
) -> i64 {
    let _a0: Value<absl_time_internal_cctz_detail_day_tag> = Rc::new(RefCell::new(_a0));
    let f1: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f1));
    let f2: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f2));
    return ({
        let _y1: i64 = (*(*f1.borrow()).y.borrow());
        let _m1: i8 = (*(*f1.borrow()).m.borrow());
        let _d1: i8 = (*(*f1.borrow()).d.borrow());
        let _y2: i64 = (*(*f2.borrow()).y.borrow());
        let _m2: i8 = (*(*f2.borrow()).m.borrow());
        let _d2: i8 = (*(*f2.borrow()).d.borrow());
        day_difference_64(_y1, _m1, _d1, _y2, _m2, _d2)
    });
}
pub fn difference_68(
    _a0: absl_time_internal_cctz_detail_hour_tag,
    f1: absl_time_internal_cctz_detail_fields,
    f2: absl_time_internal_cctz_detail_fields,
) -> i64 {
    let _a0: Value<absl_time_internal_cctz_detail_hour_tag> = Rc::new(RefCell::new(_a0));
    let f1: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f1));
    let f2: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f2));
    return ({
        let _v: i64 = ({
            difference_67(
                absl_time_internal_cctz_detail_day_tag {},
                (*f1.borrow()).clone(),
                (*f2.borrow()).clone(),
            )
        });
        let _a: i64 = ((((*(*f1.borrow()).hh.borrow()) as i32)
            - ((*(*f2.borrow()).hh.borrow()) as i32)) as i64);
        scale_add_62(_v, 24_i64, _a)
    });
}
pub fn difference_69(
    _a0: absl_time_internal_cctz_detail_minute_tag,
    f1: absl_time_internal_cctz_detail_fields,
    f2: absl_time_internal_cctz_detail_fields,
) -> i64 {
    let _a0: Value<absl_time_internal_cctz_detail_minute_tag> = Rc::new(RefCell::new(_a0));
    let f1: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f1));
    let f2: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f2));
    return ({
        let _v: i64 = ({
            difference_68(
                absl_time_internal_cctz_detail_hour_tag {},
                (*f1.borrow()).clone(),
                (*f2.borrow()).clone(),
            )
        });
        let _a: i64 = ((((*(*f1.borrow()).mm.borrow()) as i32)
            - ((*(*f2.borrow()).mm.borrow()) as i32)) as i64);
        scale_add_62(_v, 60_i64, _a)
    });
}
pub fn difference_70(
    _a0: absl_time_internal_cctz_detail_second_tag,
    f1: absl_time_internal_cctz_detail_fields,
    f2: absl_time_internal_cctz_detail_fields,
) -> i64 {
    let _a0: Value<absl_time_internal_cctz_detail_second_tag> = Rc::new(RefCell::new(_a0));
    let f1: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f1));
    let f2: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f2));
    return ({
        let _v: i64 = ({
            difference_69(
                absl_time_internal_cctz_detail_minute_tag {},
                (*f1.borrow()).clone(),
                (*f2.borrow()).clone(),
            )
        });
        let _a: i64 = ((((*(*f1.borrow()).ss.borrow()) as i32)
            - ((*(*f2.borrow()).ss.borrow()) as i32)) as i64);
        scale_add_62(_v, 60_i64, _a)
    });
}
pub fn align_71(
    _a0: absl_time_internal_cctz_detail_second_tag,
    f: absl_time_internal_cctz_detail_fields,
) -> absl_time_internal_cctz_detail_fields {
    let _a0: Value<absl_time_internal_cctz_detail_second_tag> = Rc::new(RefCell::new(_a0));
    let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
    return (*f.borrow()).clone();
}
pub fn align_72(
    _a0: absl_time_internal_cctz_detail_minute_tag,
    f: absl_time_internal_cctz_detail_fields,
) -> absl_time_internal_cctz_detail_fields {
    let _a0: Value<absl_time_internal_cctz_detail_minute_tag> = Rc::new(RefCell::new(_a0));
    let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
    return absl_time_internal_cctz_detail_fields::absl_time_internal_cctz_detail_fields(
        { (*(*f.borrow()).y.borrow()) },
        { (*(*f.borrow()).m.borrow()) },
        { (*(*f.borrow()).d.borrow()) },
        { (*(*f.borrow()).hh.borrow()) },
        { (*(*f.borrow()).mm.borrow()) },
        { 0_i8 },
    );
}
pub fn align_73(
    _a0: absl_time_internal_cctz_detail_hour_tag,
    f: absl_time_internal_cctz_detail_fields,
) -> absl_time_internal_cctz_detail_fields {
    let _a0: Value<absl_time_internal_cctz_detail_hour_tag> = Rc::new(RefCell::new(_a0));
    let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
    return absl_time_internal_cctz_detail_fields::absl_time_internal_cctz_detail_fields(
        { (*(*f.borrow()).y.borrow()) },
        { (*(*f.borrow()).m.borrow()) },
        { (*(*f.borrow()).d.borrow()) },
        { (*(*f.borrow()).hh.borrow()) },
        { 0_i8 },
        { 0_i8 },
    );
}
pub fn align_74(
    _a0: absl_time_internal_cctz_detail_day_tag,
    f: absl_time_internal_cctz_detail_fields,
) -> absl_time_internal_cctz_detail_fields {
    let _a0: Value<absl_time_internal_cctz_detail_day_tag> = Rc::new(RefCell::new(_a0));
    let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
    return absl_time_internal_cctz_detail_fields::absl_time_internal_cctz_detail_fields(
        { (*(*f.borrow()).y.borrow()) },
        { (*(*f.borrow()).m.borrow()) },
        { (*(*f.borrow()).d.borrow()) },
        { 0_i8 },
        { 0_i8 },
        { 0_i8 },
    );
}
pub fn align_75(
    _a0: absl_time_internal_cctz_detail_month_tag,
    f: absl_time_internal_cctz_detail_fields,
) -> absl_time_internal_cctz_detail_fields {
    let _a0: Value<absl_time_internal_cctz_detail_month_tag> = Rc::new(RefCell::new(_a0));
    let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
    return absl_time_internal_cctz_detail_fields::absl_time_internal_cctz_detail_fields(
        { (*(*f.borrow()).y.borrow()) },
        { (*(*f.borrow()).m.borrow()) },
        { 1_i8 },
        { 0_i8 },
        { 0_i8 },
        { 0_i8 },
    );
}
pub fn align_76(
    _a0: absl_time_internal_cctz_detail_year_tag,
    f: absl_time_internal_cctz_detail_fields,
) -> absl_time_internal_cctz_detail_fields {
    let _a0: Value<absl_time_internal_cctz_detail_year_tag> = Rc::new(RefCell::new(_a0));
    let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
    return absl_time_internal_cctz_detail_fields::absl_time_internal_cctz_detail_fields(
        { (*(*f.borrow()).y.borrow()) },
        { 1_i8 },
        { 1_i8 },
        { 0_i8 },
        { 0_i8 },
        { 0_i8 },
    );
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ {
    pub fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_1(
        ct: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_>,
        _a1: Option<AnyPtr>,
    ) -> Self {
        let _a1: Value<AnyPtr> = Rc::new(RefCell::new(_a1.unwrap_or(AnyPtr::default())));
        let __this: Value<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
        > = Rc::new(RefCell::new(Self {
            f_: <Value<absl_time_internal_cctz_detail_fields>>::default(),
        }));
        let this: Ptr<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
        > = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn operator_add_77(
    a: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
    n: i64,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ {
    let a: Value<
        absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
    > = Rc::new(RefCell::new(a));
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_2 ( {  (  { step_59 ( absl_time_internal_cctz_detail_day_tag { }  , ((*(*a.borrow()) . f_ .borrow()) ).clone()  , (*n.borrow())  , ) } )    } , )   ;
}
pub fn operator_sub_78(
    a: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
    n: i64,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ {
    let a: Value<
        absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
    > = Rc::new(RefCell::new(a));
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return if ((*n.borrow()) != <i64>::MIN) {
        absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_2 ( {  (  { step_59 ( absl_time_internal_cctz_detail_day_tag { }  , ((*(*a.borrow()) . f_ .borrow()) ).clone()  , - (*n.borrow())  , ) } )    } , )
    } else {
        absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_2 ( {  (  { step_59 ( absl_time_internal_cctz_detail_day_tag { }  , (  { step_59 ( absl_time_internal_cctz_detail_day_tag { }  , ((*(*a.borrow()) . f_ .borrow()) ).clone()  , - ( ( (*n.borrow()) + 1_i64 ) )  , ) } )   , 1_i64  , ) } )    } , )
    };
}
#[derive()]
pub struct absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ {
    f_: Value<absl_time_internal_cctz_detail_fields>,
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ {
    fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_2(
        f: absl_time_internal_cctz_detail_fields,
    ) -> Self {
        let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
        let __this: Value<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
        > = Rc::new(RefCell::new(Self {
            f_: Rc::new(RefCell::new(
                ({
                    align_74(
                        absl_time_internal_cctz_detail_day_tag {},
                        (*f.borrow()).clone(),
                    )
                }),
            )),
        }));
        let this: Ptr<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
        > = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ {
    fn clone(&self) -> Self {
        let __this: Value<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
        > = Rc::new(RefCell::new(Self {
            f_: Rc::new(RefCell::new((*self.f_.borrow()).clone())),
        }));
        let this: Ptr<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
        > = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ {
    fn default() -> Self {
        {
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_3 ( )
        }
    }
}
impl ByteRepr
    for absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_
{
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.f_.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            f_: Rc::new(RefCell::new(
                <absl_time_internal_cctz_detail_fields>::from_bytes(&buf[0..16]),
            )),
        }
    }
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ {
    pub fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_4(
        ct: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_>,
        _a1: Option<AnyPtr>,
    ) -> Self {
        let _a1: Value<AnyPtr> = Rc::new(RefCell::new(_a1.unwrap_or(AnyPtr::default())));
        let __this: Value<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_,
        > = Rc::new(RefCell::new(Self {
            f_: <Value<absl_time_internal_cctz_detail_fields>>::default(),
        }));
        let this: Ptr<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_,
        > = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ {
    pub fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_5(
        ct: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_>,
        _a1: Option<AnyPtr>,
    ) -> Self {
        let _a1: Value<AnyPtr> = Rc::new(RefCell::new(_a1.unwrap_or(AnyPtr::default())));
        let __this: Value<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_,
        > = Rc::new(RefCell::new(Self {
            f_: <Value<absl_time_internal_cctz_detail_fields>>::default(),
        }));
        let this: Ptr<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_,
        > = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive()]
pub struct absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ {
    f_: Value<absl_time_internal_cctz_detail_fields>,
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ {
    fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_6(
        f: absl_time_internal_cctz_detail_fields,
    ) -> Self {
        let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
        let __this: Value<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_,
        > = Rc::new(RefCell::new(Self {
            f_: Rc::new(RefCell::new(
                ({
                    align_71(
                        absl_time_internal_cctz_detail_second_tag {},
                        (*f.borrow()).clone(),
                    )
                }),
            )),
        }));
        let this: Ptr<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_,
        > = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone
    for absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_
{
    fn clone(&self) -> Self {
        let __this: Value<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_,
        > = Rc::new(RefCell::new(Self {
            f_: <Value<absl_time_internal_cctz_detail_fields>>::default(),
        }));
        let this: Ptr<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_,
        > = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default
    for absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_
{
    fn default() -> Self {
        {
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_7 ( )
        }
    }
}
impl ByteRepr
    for absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_
{
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.f_.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            f_: Rc::new(RefCell::new(
                <absl_time_internal_cctz_detail_fields>::from_bytes(&buf[0..16]),
            )),
        }
    }
}
#[derive()]
pub struct absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_ {
    f_: Value<absl_time_internal_cctz_detail_fields>,
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_ {}
impl Clone for absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_ {
    fn clone(&self) -> Self {
        let __this: Value<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_,
        > = Rc::new(RefCell::new(Self {
            f_: Rc::new(RefCell::new((*self.f_.borrow()).clone())),
        }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_ {
    fn default() -> Self {
        {
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_8 ( )
        }
    }
}
impl ByteRepr for absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.f_.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            f_: Rc::new(RefCell::new(
                <absl_time_internal_cctz_detail_fields>::from_bytes(&buf[0..16]),
            )),
        }
    }
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_ {
    pub fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_9(
        ct: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_>,
        _a1: Option<AnyPtr>,
    ) -> Self {
        let _a1: Value<AnyPtr> = Rc::new(RefCell::new(_a1.unwrap_or(AnyPtr::default())));
        let __this: Value<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_,
        > = Rc::new(RefCell::new(Self {
            f_: <Value<absl_time_internal_cctz_detail_fields>>::default(),
        }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive()]
pub struct absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_ {
    f_: Value<absl_time_internal_cctz_detail_fields>,
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_ {
    fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_10(
        f: absl_time_internal_cctz_detail_fields,
    ) -> Self {
        let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
        let __this: Value<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_,
        > = Rc::new(RefCell::new(Self {
            f_: Rc::new(RefCell::new(
                ({ align_72(absl_time_internal_minute_tag {}, (*f.borrow()).clone()) }),
            )),
        }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_ {
    fn clone(&self) -> Self {
        let __this: Value<
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_,
        > = Rc::new(RefCell::new(Self {
            f_: Rc::new(RefCell::new((*self.f_.borrow()).clone())),
        }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_ {
    fn default() -> Self {
        {
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_11 ( )
        }
    }
}
impl ByteRepr for absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.f_.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            f_: Rc::new(RefCell::new(
                <absl_time_internal_cctz_detail_fields>::from_bytes(&buf[0..16]),
            )),
        }
    }
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_ {
    pub fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_12(
        ct: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_>,
        _a1: Option<AnyPtr>,
    ) -> Self {
        let _a1: Value<AnyPtr> = Rc::new(RefCell::new(_a1.unwrap_or(AnyPtr::default())));
        let __this: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_> =
            Rc::new(RefCell::new(Self {
                f_: <Value<absl_time_internal_cctz_detail_fields>>::default(),
            }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive()]
pub struct absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_ {
    f_: Value<absl_time_internal_cctz_detail_fields>,
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_ {
    fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_13(
        f: absl_time_internal_cctz_detail_fields,
    ) -> Self {
        let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
        let __this: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_> =
            Rc::new(RefCell::new(Self {
                f_: Rc::new(RefCell::new(
                    ({ align_73(absl_time_internal_hour_tag {}, (*f.borrow()).clone()) }),
                )),
            }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_ {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_> =
            Rc::new(RefCell::new(Self {
                f_: Rc::new(RefCell::new((*self.f_.borrow()).clone())),
            }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_ {
    fn default() -> Self {
        {
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_14 ( )
        }
    }
}
impl ByteRepr for absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.f_.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            f_: Rc::new(RefCell::new(
                <absl_time_internal_cctz_detail_fields>::from_bytes(&buf[0..16]),
            )),
        }
    }
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ {
    pub fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_15(
        ct: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_>,
        _a1: Option<AnyPtr>,
    ) -> Self {
        let _a1: Value<AnyPtr> = Rc::new(RefCell::new(_a1.unwrap_or(AnyPtr::default())));
        let __this: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_> =
            Rc::new(RefCell::new(Self {
                f_: <Value<absl_time_internal_cctz_detail_fields>>::default(),
            }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ {
    pub fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_16(
        ct: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_>,
        _a1: Option<AnyPtr>,
    ) -> Self {
        let _a1: Value<AnyPtr> = Rc::new(RefCell::new(_a1.unwrap_or(AnyPtr::default())));
        let __this: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_> =
            Rc::new(RefCell::new(Self {
                f_: <Value<absl_time_internal_cctz_detail_fields>>::default(),
            }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive()]
pub struct absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ {
    f_: Value<absl_time_internal_cctz_detail_fields>,
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ {
    fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_17(
        f: absl_time_internal_cctz_detail_fields,
    ) -> Self {
        let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
        let __this: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_> =
            Rc::new(RefCell::new(Self {
                f_: Rc::new(RefCell::new(
                    ({ align_74(absl_time_internal_day_tag {}, (*f.borrow()).clone()) }),
                )),
            }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_> =
            Rc::new(RefCell::new(Self {
                f_: Rc::new(RefCell::new((*self.f_.borrow()).clone())),
            }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ {
    fn default() -> Self {
        {
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_18 ( )
        }
    }
}
impl ByteRepr for absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.f_.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            f_: Rc::new(RefCell::new(
                <absl_time_internal_cctz_detail_fields>::from_bytes(&buf[0..16]),
            )),
        }
    }
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_ {
    pub fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_19(
        ct: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_>,
        _a1: Option<AnyPtr>,
    ) -> Self {
        let _a1: Value<AnyPtr> = Rc::new(RefCell::new(_a1.unwrap_or(AnyPtr::default())));
        let __this: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_> =
            Rc::new(RefCell::new(Self {
                f_: <Value<absl_time_internal_cctz_detail_fields>>::default(),
            }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive()]
pub struct absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_ {
    f_: Value<absl_time_internal_cctz_detail_fields>,
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_ {
    fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_20(
        f: absl_time_internal_cctz_detail_fields,
    ) -> Self {
        let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
        let __this: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_> =
            Rc::new(RefCell::new(Self {
                f_: Rc::new(RefCell::new(
                    ({ align_75(absl_time_internal_month_tag {}, (*f.borrow()).clone()) }),
                )),
            }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_ {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_> =
            Rc::new(RefCell::new(Self {
                f_: Rc::new(RefCell::new((*self.f_.borrow()).clone())),
            }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_ {
    fn default() -> Self {
        {
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_21 ( )
        }
    }
}
impl ByteRepr for absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.f_.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            f_: Rc::new(RefCell::new(
                <absl_time_internal_cctz_detail_fields>::from_bytes(&buf[0..16]),
            )),
        }
    }
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_ {
    pub fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_22(
        ct: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_>,
        _a1: Option<AnyPtr>,
    ) -> Self {
        let _a1: Value<AnyPtr> = Rc::new(RefCell::new(_a1.unwrap_or(AnyPtr::default())));
        let __this: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_> =
            Rc::new(RefCell::new(Self {
                f_: <Value<absl_time_internal_cctz_detail_fields>>::default(),
            }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive()]
pub struct absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_ {
    f_: Value<absl_time_internal_cctz_detail_fields>,
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_ {
    fn absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_23(
        f: absl_time_internal_cctz_detail_fields,
    ) -> Self {
        let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
        let __this: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_> =
            Rc::new(RefCell::new(Self {
                f_: Rc::new(RefCell::new(
                    ({ align_76(absl_time_internal_year_tag {}, (*f.borrow()).clone()) }),
                )),
            }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_ {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_> =
            Rc::new(RefCell::new(Self {
                f_: Rc::new(RefCell::new((*self.f_.borrow()).clone())),
            }));
        let this: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_ {
    fn default() -> Self {
        {
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_24 ( )
        }
    }
}
impl ByteRepr for absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.f_.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            f_: Rc::new(RefCell::new(
                <absl_time_internal_cctz_detail_fields>::from_bytes(&buf[0..16]),
            )),
        }
    }
}
pub type absl_time_internal_cctz_detail_weekday = i32;
pub const absl_time_internal_cctz_detail_weekday_monday: absl_time_internal_cctz_detail_weekday = 0;
pub const absl_time_internal_cctz_detail_weekday_tuesday: absl_time_internal_cctz_detail_weekday =
    1;
pub const absl_time_internal_cctz_detail_weekday_wednesday: absl_time_internal_cctz_detail_weekday =
    2;
pub const absl_time_internal_cctz_detail_weekday_thursday: absl_time_internal_cctz_detail_weekday =
    3;
pub const absl_time_internal_cctz_detail_weekday_friday: absl_time_internal_cctz_detail_weekday = 4;
pub const absl_time_internal_cctz_detail_weekday_saturday: absl_time_internal_cctz_detail_weekday =
    5;
pub const absl_time_internal_cctz_detail_weekday_sunday: absl_time_internal_cctz_detail_weekday = 6;
pub fn get_weekday_79(
    cs: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_>,
) -> absl_time_internal_cctz_detail_weekday {
    let k_weekday_by_mon_off: Value<Box<[absl_time_internal_cctz_detail_weekday]>> =
        Rc::new(RefCell::new(Box::new([
            absl_time_internal_cctz_detail_weekday_monday,
            absl_time_internal_cctz_detail_weekday_tuesday,
            absl_time_internal_cctz_detail_weekday_wednesday,
            absl_time_internal_cctz_detail_weekday_thursday,
            absl_time_internal_cctz_detail_weekday_friday,
            absl_time_internal_cctz_detail_weekday_saturday,
            absl_time_internal_cctz_detail_weekday_sunday,
            absl_time_internal_cctz_detail_weekday_monday,
            absl_time_internal_cctz_detail_weekday_tuesday,
            absl_time_internal_cctz_detail_weekday_wednesday,
            absl_time_internal_cctz_detail_weekday_thursday,
            absl_time_internal_cctz_detail_weekday_friday,
            absl_time_internal_cctz_detail_weekday_saturday,
        ])));
    let k_weekday_offsets: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        -1_i32, 0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4,
    ])));
    let wd: Value<i64> = Rc::new(RefCell::new({
        let _lhs = (2400_i64
            + (({
                absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_Impl :: year ( &cs  , )
            }) % 400_i64));
        _lhs - ((({
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_Impl :: month ( &cs  , )
        }) < 3) as i64)
    }));
    let __rhs =
        ((((*wd.borrow()) / 4_i64) - ((*wd.borrow()) / 100_i64)) + ((*wd.borrow()) / 400_i64));
    (*wd.borrow_mut()) += __rhs;
    (*wd.borrow_mut()) += (({
        let _lhs = (*k_weekday_offsets.borrow())[({
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_Impl :: month ( &cs  , )
        }) as usize];
        _lhs + ({
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_Impl :: day ( &cs  , )
        })
    }) as i64);
    return (*k_weekday_by_mon_off.borrow())[(((*wd.borrow()) % 7_i64) + 6_i64) as usize];
}
pub fn next_weekday_80(
    cd: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
    wd: absl_time_internal_cctz_detail_weekday,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ {
    let cd: Value<
        absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
    > = Rc::new(RefCell::new(cd));
    let wd: Value<absl_time_internal_cctz_detail_weekday> = Rc::new(RefCell::new(wd));
    let k_weekdays_forw: Value<Box<[absl_time_internal_cctz_detail_weekday]>> =
        Rc::new(RefCell::new(Box::new([
            absl_time_internal_cctz_detail_weekday_monday,
            absl_time_internal_cctz_detail_weekday_tuesday,
            absl_time_internal_cctz_detail_weekday_wednesday,
            absl_time_internal_cctz_detail_weekday_thursday,
            absl_time_internal_cctz_detail_weekday_friday,
            absl_time_internal_cctz_detail_weekday_saturday,
            absl_time_internal_cctz_detail_weekday_sunday,
            absl_time_internal_cctz_detail_weekday_monday,
            absl_time_internal_cctz_detail_weekday_tuesday,
            absl_time_internal_cctz_detail_weekday_wednesday,
            absl_time_internal_cctz_detail_weekday_thursday,
            absl_time_internal_cctz_detail_weekday_friday,
            absl_time_internal_cctz_detail_weekday_saturday,
            absl_time_internal_cctz_detail_weekday_sunday,
        ])));
    let base: Value<absl_time_internal_cctz_detail_weekday> = Rc::new(RefCell::new(
        ({
            let _cs : Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ > = Rc::new(RefCell::new(absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_4 ( {  cd .as_pointer()   } , None , ) ));
            get_weekday_79(_cs.as_pointer())
        }),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while true {
        if ((*base.borrow()) == (*k_weekdays_forw.borrow())[(*i.borrow()) as usize]) {
            let j: Value<i32> = Rc::new(RefCell::new(((*i.borrow()) + 1)));
            'loop_: while true {
                if ((*wd.borrow()) == (*k_weekdays_forw.borrow())[(*j.borrow()) as usize]) {
                    return ({
                        let _a: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_   = ((*cd.borrow()) ).clone()  ;
                        operator_add_77(_a, (((*j.borrow()) - (*i.borrow())) as i64))
                    });
                }
                (*j.borrow_mut()).prefix_inc();
            }
        }
        (*i.borrow_mut()).prefix_inc();
    }
    panic!("ub: non-void function does not return a value")
}
pub fn prev_weekday_81(
    cd: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
    wd: absl_time_internal_cctz_detail_weekday,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ {
    let cd: Value<
        absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
    > = Rc::new(RefCell::new(cd));
    let wd: Value<absl_time_internal_cctz_detail_weekday> = Rc::new(RefCell::new(wd));
    let k_weekdays_back: Value<Box<[absl_time_internal_cctz_detail_weekday]>> =
        Rc::new(RefCell::new(Box::new([
            absl_time_internal_cctz_detail_weekday_sunday,
            absl_time_internal_cctz_detail_weekday_saturday,
            absl_time_internal_cctz_detail_weekday_friday,
            absl_time_internal_cctz_detail_weekday_thursday,
            absl_time_internal_cctz_detail_weekday_wednesday,
            absl_time_internal_cctz_detail_weekday_tuesday,
            absl_time_internal_cctz_detail_weekday_monday,
            absl_time_internal_cctz_detail_weekday_sunday,
            absl_time_internal_cctz_detail_weekday_saturday,
            absl_time_internal_cctz_detail_weekday_friday,
            absl_time_internal_cctz_detail_weekday_thursday,
            absl_time_internal_cctz_detail_weekday_wednesday,
            absl_time_internal_cctz_detail_weekday_tuesday,
            absl_time_internal_cctz_detail_weekday_monday,
        ])));
    let base: Value<absl_time_internal_cctz_detail_weekday> = Rc::new(RefCell::new(
        ({
            let _cs : Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ > = Rc::new(RefCell::new(absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_4 ( {  cd .as_pointer()   } , None , ) ));
            get_weekday_79(_cs.as_pointer())
        }),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while true {
        if ((*base.borrow()) == (*k_weekdays_back.borrow())[(*i.borrow()) as usize]) {
            let j: Value<i32> = Rc::new(RefCell::new(((*i.borrow()) + 1)));
            'loop_: while true {
                if ((*wd.borrow()) == (*k_weekdays_back.borrow())[(*j.borrow()) as usize]) {
                    return ({
                        let _a: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_   = ((*cd.borrow()) ).clone()  ;
                        operator_sub_78(_a, (((*j.borrow()) - (*i.borrow())) as i64))
                    });
                }
                (*j.borrow_mut()).prefix_inc();
            }
        }
        (*i.borrow_mut()).prefix_inc();
    }
    panic!("ub: non-void function does not return a value")
}
pub fn get_yearday_82(
    cs: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_>,
) -> i32 {
    let k_month_offsets: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        -1_i32, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334,
    ])));
    let feb29: Value<i32> = Rc::new(RefCell::new(
        (((({
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_Impl :: month ( &cs  , )
        }) > 2)
            && ({
                is_leap_year_45(
                    ({
                        absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_Impl :: year ( &cs  , )
                    }),
                )
            })) as i32),
    ));
    return {
        let _lhs = {
            let _lhs = (*k_month_offsets.borrow())[({
                absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_Impl :: month ( &cs  , )
            }) as usize];
            _lhs + (*feb29.borrow())
        };
        _lhs + ({
            absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_Impl :: day ( &cs  , )
        })
    };
}
#[derive(Default)]
pub struct absl_time_internal_second_tag {}
impl Clone for absl_time_internal_second_tag {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_second_tag> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_time_internal_second_tag> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_second_tag {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct absl_time_internal_minute_tag {}
impl Clone for absl_time_internal_minute_tag {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_minute_tag> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_time_internal_minute_tag> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_minute_tag {
    fn byte_size() -> usize {
        2
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct absl_time_internal_hour_tag {}
impl Clone for absl_time_internal_hour_tag {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_hour_tag> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_time_internal_hour_tag> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_hour_tag {
    fn byte_size() -> usize {
        3
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct absl_time_internal_day_tag {}
impl Clone for absl_time_internal_day_tag {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_day_tag> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_time_internal_day_tag> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_day_tag {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct absl_time_internal_month_tag {}
impl Clone for absl_time_internal_month_tag {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_month_tag> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_time_internal_month_tag> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_month_tag {
    fn byte_size() -> usize {
        5
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct absl_time_internal_year_tag {}
impl Clone for absl_time_internal_year_tag {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_year_tag> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_time_internal_year_tag> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_year_tag {
    fn byte_size() -> usize {
        6
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn GetWeekday_83(
    cs: absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_,
) -> absl_time_internal_cctz_detail_weekday {
    let cs: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_> =
        Rc::new(RefCell::new(cs));
    return ({
        let _cs : Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ > = Rc::new(RefCell::new(absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_5 ( {  cs .as_pointer()   } , None , ) ));
        get_weekday_79(_cs.as_pointer())
    });
}
pub fn NextWeekday_84(
    cd: absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_,
    wd: absl_time_internal_cctz_detail_weekday,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ {
    let cd: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_> =
        Rc::new(RefCell::new(cd));
    let wd: Value<absl_time_internal_cctz_detail_weekday> = Rc::new(RefCell::new(wd));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_15 ( { let __tmp_0 : Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ > = Rc::new(RefCell::new((  { next_weekday_80 ( absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_1 ( {  cd .as_pointer()   } , None , )  , (*wd.borrow())  , ) } )  )); __tmp_0.as_pointer()  } , None , )   ;
}
pub fn PrevWeekday_85(
    cd: absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_,
    wd: absl_time_internal_cctz_detail_weekday,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ {
    let cd: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_> =
        Rc::new(RefCell::new(cd));
    let wd: Value<absl_time_internal_cctz_detail_weekday> = Rc::new(RefCell::new(wd));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_15 ( { let __tmp_1 : Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ > = Rc::new(RefCell::new((  { prev_weekday_81 ( absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_1 ( {  cd .as_pointer()   } , None , )  , (*wd.borrow())  , ) } )  )); __tmp_1.as_pointer()  } , None , )   ;
}
pub fn GetYearDay_86(
    cs: absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_,
) -> i32 {
    let cs: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_> =
        Rc::new(RefCell::new(cs));
    return ({
        let _cs : Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ > = Rc::new(RefCell::new(absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_5 ( {  cs .as_pointer()   } , None , ) ));
        get_yearday_82(_cs.as_pointer())
    });
}
pub fn operator_eq_87(
    lhs: absl_time_internal_cctz_time_zone,
    rhs: absl_time_internal_cctz_time_zone,
) -> bool {
    let lhs: Value<absl_time_internal_cctz_time_zone> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_time_internal_cctz_time_zone> = Rc::new(RefCell::new(rhs));
    return (({ absl_time_internal_cctz_time_zoneImpl::effective_impl(&lhs.as_pointer()) })
        == ({ absl_time_internal_cctz_time_zoneImpl::effective_impl(&rhs.as_pointer()) }));
}
pub fn operator_ne_88(
    lhs: absl_time_internal_cctz_time_zone,
    rhs: absl_time_internal_cctz_time_zone,
) -> bool {
    let lhs: Value<absl_time_internal_cctz_time_zone> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_time_internal_cctz_time_zone> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_time_internal_cctz_time_zone = (*lhs.borrow()).clone();
        operator_eq_87(_lhs, (*rhs.borrow()).clone())
    });
}
#[derive(Default)]
pub struct absl_time_internal_cctz_time_zone_absolute_lookup {
    pub cs:
        Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_>,
    pub offset: Value<i32>,
    pub is_dst: Value<bool>,
    pub abbr: Value<Ptr<u8>>,
}
impl Clone for absl_time_internal_cctz_time_zone_absolute_lookup {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_time_zone_absolute_lookup> =
            Rc::new(RefCell::new(Self {
                cs: Rc::new(RefCell::new((*self.cs.borrow()).clone())),
                offset: Rc::new(RefCell::new((*self.offset.borrow()))),
                is_dst: Rc::new(RefCell::new((*self.is_dst.borrow()))),
                abbr: Rc::new(RefCell::new((*self.abbr.borrow()).clone())),
            }));
        let this: Ptr<absl_time_internal_cctz_time_zone_absolute_lookup> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_cctz_time_zone_absolute_lookup {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.cs.borrow()).to_bytes(&mut buf[0..16]);
        (*self.offset.borrow()).to_bytes(&mut buf[16..20]);
        (*self.is_dst.borrow()).to_bytes(&mut buf[20..21]);
        (*self.abbr.borrow()).to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self { cs: Rc::new(RefCell::new(<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ >::from_bytes(&buf[0..16]))), offset: Rc::new(RefCell::new(<i32 >::from_bytes(&buf[16..20]))), is_dst: Rc::new(RefCell::new(<bool >::from_bytes(&buf[20..21]))), abbr: Rc::new(RefCell::new(<Ptr::<u8> >::from_bytes(&buf[24..32]))), }
    }
}
pub type absl_time_internal_cctz_time_zone_civil_lookup_civil_kind = u32;
pub const absl_time_internal_cctz_time_zone_civil_lookup_civil_kind_UNIQUE:
    absl_time_internal_cctz_time_zone_civil_lookup_civil_kind = 0;
pub const absl_time_internal_cctz_time_zone_civil_lookup_civil_kind_SKIPPED:
    absl_time_internal_cctz_time_zone_civil_lookup_civil_kind = 1;
pub const absl_time_internal_cctz_time_zone_civil_lookup_civil_kind_REPEATED:
    absl_time_internal_cctz_time_zone_civil_lookup_civil_kind = 2;
#[derive(Default)]
pub struct absl_time_internal_cctz_time_zone_civil_lookup { pub kind : Value<absl_time_internal_cctz_time_zone_civil_lookup_civil_kind > , pub pre : Value<std_chrono_time_point_std_chrono_system_clock__std_chrono_duration_long_long__std_ratio________ > , pub trans : Value<std_chrono_time_point_std_chrono_system_clock__std_chrono_duration_long_long__std_ratio________ > , pub post : Value<std_chrono_time_point_std_chrono_system_clock__std_chrono_duration_long_long__std_ratio________ > , }
impl Clone for absl_time_internal_cctz_time_zone_civil_lookup {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_time_zone_civil_lookup> =
            Rc::new(RefCell::new(Self {
                kind: Rc::new(RefCell::new((*self.kind.borrow()))),
                pre: Rc::new(RefCell::new((*self.pre.borrow()).clone())),
                trans: Rc::new(RefCell::new((*self.trans.borrow()).clone())),
                post: Rc::new(RefCell::new((*self.post.borrow()).clone())),
            }));
        let this: Ptr<absl_time_internal_cctz_time_zone_civil_lookup> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_cctz_time_zone_civil_lookup {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.kind.borrow()).to_bytes(&mut buf[0..4]);
        (*self.pre.borrow()).to_bytes(&mut buf[8..16]);
        (*self.trans.borrow()).to_bytes(&mut buf[16..24]);
        (*self.post.borrow()).to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self { kind: Rc::new(RefCell::new(<absl_time_internal_cctz_time_zone_civil_lookup_civil_kind >::from_bytes(&buf[0..4]))), pre: Rc::new(RefCell::new(<std_chrono_time_point_std_chrono_system_clock__std_chrono_duration_long_long__std_ratio________ >::from_bytes(&buf[8..16]))), trans: Rc::new(RefCell::new(<std_chrono_time_point_std_chrono_system_clock__std_chrono_duration_long_long__std_ratio________ >::from_bytes(&buf[16..24]))), post: Rc::new(RefCell::new(<std_chrono_time_point_std_chrono_system_clock__std_chrono_duration_long_long__std_ratio________ >::from_bytes(&buf[24..32]))), }
    }
}
#[derive(Default)]
pub struct absl_time_internal_cctz_time_zone_civil_transition {
    pub from:
        Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_>,
    pub to:
        Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_>,
}
impl Clone for absl_time_internal_cctz_time_zone_civil_transition {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_time_zone_civil_transition> =
            Rc::new(RefCell::new(Self {
                from: Rc::new(RefCell::new((*self.from.borrow()).clone())),
                to: Rc::new(RefCell::new((*self.to.borrow()).clone())),
            }));
        let this: Ptr<absl_time_internal_cctz_time_zone_civil_transition> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_time_internal_cctz_time_zone_civil_transition {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.from.borrow()).to_bytes(&mut buf[0..16]);
        (*self.to.borrow()).to_bytes(&mut buf[16..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self { from: Rc::new(RefCell::new(<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ >::from_bytes(&buf[0..16]))), to: Rc::new(RefCell::new(<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ >::from_bytes(&buf[16..32]))), }
    }
}
#[derive()]
pub struct absl_time_internal_cctz_time_zone {
    impl__: Value<Ptr<absl_time_internal_cctz_time_zone_Impl>>,
}
impl absl_time_internal_cctz_time_zone {
    pub fn absl_time_internal_cctz_time_zone1() -> Self {
        let __this: Value<absl_time_internal_cctz_time_zone> = Rc::new(RefCell::new(Self {
            impl__: Rc::new(RefCell::new(
                Ptr::<absl_time_internal_cctz_time_zone_Impl>::null(),
            )),
        }));
        let this: Ptr<absl_time_internal_cctz_time_zone> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    fn absl_time_internal_cctz_time_zone2(
        impl_: Ptr<absl_time_internal_cctz_time_zone_Impl>,
    ) -> Self {
        let impl_: Value<Ptr<absl_time_internal_cctz_time_zone_Impl>> =
            Rc::new(RefCell::new(impl_));
        let __this: Value<absl_time_internal_cctz_time_zone> = Rc::new(RefCell::new(Self {
            impl__: Rc::new(RefCell::new((*impl_.borrow()).clone())),
        }));
        let this: Ptr<absl_time_internal_cctz_time_zone> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for absl_time_internal_cctz_time_zone {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_87(
                Rc::new(RefCell::new(absl_time_internal_cctz_time_zone {
                    impl__: self.impl__.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(absl_time_internal_cctz_time_zone {
                    impl__: other.impl__.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for absl_time_internal_cctz_time_zone {}
impl Clone for absl_time_internal_cctz_time_zone {
    fn clone(&self) -> Self {
        let __this: Value<absl_time_internal_cctz_time_zone> = Rc::new(RefCell::new(Self {
            impl__: Rc::new(RefCell::new((*self.impl__.borrow()).clone())),
        }));
        let this: Ptr<absl_time_internal_cctz_time_zone> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_time_internal_cctz_time_zone {
    fn default() -> Self {
        { absl_time_internal_cctz_time_zone::absl_time_internal_cctz_time_zone1() }
    }
}
impl ByteRepr for absl_time_internal_cctz_time_zone {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.impl__.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            impl__: Rc::new(RefCell::new(
                <Ptr<absl_time_internal_cctz_time_zone_Impl>>::from_bytes(&buf[0..8]),
            )),
        }
    }
}
pub fn convert_89(
    cs: Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_>,
    tz: Ptr<absl_time_internal_cctz_time_zone>,
) -> std_chrono_time_point_std_chrono_system_clock__std_chrono_duration_long_long__std_ratio________
{
    let cl: Value<absl_time_internal_cctz_time_zone_civil_lookup> = Rc::new(RefCell::new(
        ({
            let _cs: Ptr< absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_  >  = (cs ).clone() ;
            absl_time_internal_cctz_time_zoneImpl :: lookup_pconstabsl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_const ( &tz  , _cs , )
        }),
    ));
    if (((*(*cl.borrow()).kind.borrow()) as i32)
        == (absl_time_internal_cctz_time_zone_civil_lookup_civil_kind_SKIPPED as i32))
    {
        return (*(*cl.borrow()).trans.borrow()).clone();
    }
    return (*(*cl.borrow()).pre.borrow()).clone();
}pub  fn split_seconds_90 ( tp : Ptr< std_chrono_time_point_std_chrono_system_clock__std_chrono_duration_long_long__std_ratio________  > , ) -> (Value<std_chrono_time_point_std_chrono_steady_clock__std_chrono_duration_long_long__std_ratio________>, Value<std_chrono_duration_long_long__std_ratio_______>){
    return (
        Rc::new(RefCell::new(
            (*tp.upgrade().deref())
                .clone()
                .try_into()
                .expect("failed conversion"),
        )),
        Rc::new(RefCell::new(
            ({ std_chrono_duration_long_long__std_ratio_______::zero() })
                .try_into()
                .expect("failed conversion"),
        )),
    );
}
pub fn join_seconds_91(
    sec : Ptr< std_chrono_time_point_std_chrono_system_clock__std_chrono_duration_long_long__std_ratio________  >,
    _a1: Ptr<std_chrono_duration_long_long__std_ratio_______>,
    tpp : Ptr< std_chrono_time_point_std_chrono_system_clock__std_chrono_duration_long_long__std_ratio________  >,
) -> bool {
    let tpp : Value<Ptr< std_chrono_time_point_std_chrono_system_clock__std_chrono_duration_long_long__std_ratio________  > >  = Rc::new(RefCell::new(tpp)) ;
    let __rhs = (*sec.upgrade().deref()).clone();
    (*tpp.borrow()).write(__rhs);
    return true;
}
thread_local!(
    pub static kTicksPerNanosecond_92: Value<i64> = Rc::new(RefCell::new(4));
);
thread_local!(
    pub static kTicksPerSecond_93: Value<i64> = Rc::new(RefCell::new(4000000000));
);
pub fn FromInt64_94(v: i64, _a1: std_ratio______) -> absl_Duration {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({
        let _sec: i64 = ((*v.borrow()) / 1000000000_i64);
        let _ticks: i64 = (((((((*v.borrow()) % 1000000000_i64) * 4) * 1000_i64) * 1000_i64)
            * 1000_i64)
            / 1000000000_i64);
        MakeNormalizedDuration_95(_sec, _ticks)
    });
}
pub fn FromInt64_96(v: i64, _a1: std_ratio______) -> absl_Duration {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({
        let _sec: i64 = ((*v.borrow()) / 1000000_i64);
        let _ticks: i64 = (((((((*v.borrow()) % 1000000_i64) * 4) * 1000_i64) * 1000_i64)
            * 1000_i64)
            / 1000000_i64);
        MakeNormalizedDuration_95(_sec, _ticks)
    });
}
pub fn FromInt64_97(v: i64, _a1: std_ratio______) -> absl_Duration {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({
        let _sec: i64 = ((*v.borrow()) / 1000_i64);
        let _ticks: i64 =
            (((((((*v.borrow()) % 1000_i64) * 4) * 1000_i64) * 1000_i64) * 1000_i64) / 1000_i64);
        MakeNormalizedDuration_95(_sec, _ticks)
    });
}
pub fn FromInt64_98(v: i64, _a1: std_ratio______) -> absl_Duration {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({
        let _sec: i64 = ((*v.borrow()) / 1_i64);
        let _ticks: i64 =
            (((((((*v.borrow()) % 1_i64) * 4) * 1000_i64) * 1000_i64) * 1000_i64) / 1_i64);
        MakeNormalizedDuration_95(_sec, _ticks)
    });
}
#[derive(Default)]
struct absl_Duration_HiRep {
    lo_: Value<u32>,
    hi_: Value<u32>,
}
impl absl_Duration_HiRep {
    pub fn absl_Duration_HiRep1(value: i64) -> Self {
        let value: Value<i64> = Rc::new(RefCell::new(value));
        let __this: Value<absl_Duration_HiRep> = Rc::new(RefCell::new(Self {
            lo_: Rc::new(RefCell::new(0_u32)),
            hi_: Rc::new(RefCell::new(0_u32)),
        }));
        let this: Ptr<absl_Duration_HiRep> = __this.as_pointer();
        ({ absl_Duration_HiRepImpl::operator_assign_i64(&this, (*value.borrow())) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for absl_Duration_HiRep {
    fn clone(&self) -> Self {
        let __this: Value<absl_Duration_HiRep> = Rc::new(RefCell::new(Self {
            lo_: Rc::new(RefCell::new((*self.lo_.borrow()))),
            hi_: Rc::new(RefCell::new((*self.hi_.borrow()))),
        }));
        let this: Ptr<absl_Duration_HiRep> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_Duration_HiRep {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.lo_.borrow()).to_bytes(&mut buf[0..4]);
        (*self.hi_.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            lo_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
            hi_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive()]
pub struct absl_Duration {
    rep_hi_: Value<absl_Duration_HiRep>,
    rep_lo_: Value<u32>,
}
impl absl_Duration {
    pub fn absl_Duration1() -> Self {
        let __this: Value<absl_Duration> = Rc::new(RefCell::new(Self {
            rep_hi_: Rc::new(RefCell::new(absl_Duration_HiRep::absl_Duration_HiRep1({
                0_i64
            }))),
            rep_lo_: Rc::new(RefCell::new(0_u32)),
        }));
        let this: Ptr<absl_Duration> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    fn absl_Duration2(hi: i64, lo: u32) -> Self {
        let hi: Value<i64> = Rc::new(RefCell::new(hi));
        let lo: Value<u32> = Rc::new(RefCell::new(lo));
        let __this: Value<absl_Duration> = Rc::new(RefCell::new(Self {
            rep_hi_: Rc::new(RefCell::new(absl_Duration_HiRep::absl_Duration_HiRep1({
                (*hi.borrow())
            }))),
            rep_lo_: Rc::new(RefCell::new((*lo.borrow()))),
        }));
        let this: Ptr<absl_Duration> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for absl_Duration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_99(
                Rc::new(RefCell::new(absl_Duration {
                    rep_hi_: self.rep_hi_.clone(),
                    rep_lo_: self.rep_lo_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(absl_Duration {
                    rep_hi_: other.rep_hi_.clone(),
                    rep_lo_: other.rep_lo_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for absl_Duration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for absl_Duration {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_100(
                Rc::new(RefCell::new(absl_Duration {
                    rep_hi_: self.rep_hi_.clone(),
                    rep_lo_: self.rep_lo_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(absl_Duration {
                    rep_hi_: other.rep_hi_.clone(),
                    rep_lo_: other.rep_lo_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for absl_Duration {}
impl Clone for absl_Duration {
    fn clone(&self) -> Self {
        let __this: Value<absl_Duration> = Rc::new(RefCell::new(Self {
            rep_hi_: Rc::new(RefCell::new(
                (*(*d.upgrade().deref()).rep_hi_.borrow()).clone(),
            )),
            rep_lo_: Rc::new(RefCell::new((*(*d.upgrade().deref()).rep_lo_.borrow()))),
        }));
        let this: Ptr<absl_Duration> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_Duration {
    fn default() -> Self {
        { absl_Duration::absl_Duration1() }
    }
}
impl ByteRepr for absl_Duration {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.rep_hi_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.rep_lo_.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            rep_hi_: Rc::new(RefCell::new(<absl_Duration_HiRep>::from_bytes(&buf[0..8]))),
            rep_lo_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[8..12]))),
        }
    }
}
pub fn operator_gt_101(lhs: absl_Duration, rhs: absl_Duration) -> bool {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_Duration = (*rhs.borrow()).clone();
        operator_lt_102(_lhs, (*lhs.borrow()).clone())
    });
}
pub fn operator_ge_103(lhs: absl_Duration, rhs: absl_Duration) -> bool {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_Duration = (*lhs.borrow()).clone();
        operator_lt_102(_lhs, (*rhs.borrow()).clone())
    });
}
pub fn operator_le_104(lhs: absl_Duration, rhs: absl_Duration) -> bool {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_Duration = (*rhs.borrow()).clone();
        operator_lt_102(_lhs, (*lhs.borrow()).clone())
    });
}
pub fn operator_ne_105(lhs: absl_Duration, rhs: absl_Duration) -> bool {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_Duration = (*lhs.borrow()).clone();
        operator_eq_100(_lhs, (*rhs.borrow()).clone())
    });
}
pub fn operator_add_106(lhs: absl_Duration, rhs: absl_Duration) -> absl_Duration {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return (*({
        absl_DurationImpl::operator_add_assign(&lhs.as_pointer(), (*rhs.borrow()).clone())
    })
    .upgrade()
    .deref())
    .clone();
}
pub fn operator_sub_107(lhs: absl_Duration, rhs: absl_Duration) -> absl_Duration {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return (*({
        absl_DurationImpl::operator_sub_assign(&lhs.as_pointer(), (*rhs.borrow()).clone())
    })
    .upgrade()
    .deref())
    .clone();
}
pub fn operator_div_108(lhs: absl_Duration, rhs: absl_Duration) -> i64 {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return ({
        let _num: absl_Duration = (*lhs.borrow()).clone();
        let _rem: Ptr<absl_Duration> = (lhs.as_pointer());
        IDivDuration_109(_num, (*rhs.borrow()).clone(), _rem)
    });
}
pub fn operator_rem_110(lhs: absl_Duration, rhs: absl_Duration) -> absl_Duration {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return (*({
        absl_DurationImpl::operator_rem_assign(&lhs.as_pointer(), (*rhs.borrow()).clone())
    })
    .upgrade()
    .deref())
    .clone();
}
pub fn ZeroDuration_111() -> absl_Duration {
    return absl_Duration::absl_Duration1();
}
pub fn AbsDuration_112(d: absl_Duration) -> absl_Duration {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return if ({
        let _lhs: absl_Duration = (*d.borrow()).clone();
        operator_lt_102(_lhs, ({ ZeroDuration_111() }))
    }) {
        ({
            let _d: absl_Duration = (*d.borrow()).clone();
            operator_neg_113(_d)
        })
    } else {
        (*d.borrow()).clone()
    };
}
pub fn Nanoseconds_114(n: i32) -> absl_Duration {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return ({ FromInt64_94(((*n.borrow()) as i64), std_ratio______ {}) });
}
pub fn Nanoseconds_115(n: i64) -> absl_Duration {
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({ FromInt64_94((*n.borrow()), std_ratio______ {}) });
}
pub fn Microseconds_116(n: i32) -> absl_Duration {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return ({ FromInt64_96(((*n.borrow()) as i64), std_ratio______ {}) });
}
pub fn Microseconds_117(n: i64) -> absl_Duration {
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({ FromInt64_96((*n.borrow()), std_ratio______ {}) });
}
pub fn Milliseconds_118(n: i32) -> absl_Duration {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return ({ FromInt64_97(((*n.borrow()) as i64), std_ratio______ {}) });
}
pub fn Milliseconds_119(n: i64) -> absl_Duration {
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({ FromInt64_97((*n.borrow()), std_ratio______ {}) });
}
pub fn Seconds_120(n: i64) -> absl_Duration {
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({ FromInt64_98((*n.borrow()), std_ratio______ {}) });
}
pub fn Seconds_121(n: i64) -> absl_Duration {
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({ FromInt64_98((*n.borrow()), std_ratio______ {}) });
}
pub fn Minutes_122(n: i32) -> absl_Duration {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return ({ FromInt64_123(((*n.borrow()) as i64), std_ratio______ {}) });
}
pub fn Hours_124(n: i32) -> absl_Duration {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return ({ FromInt64_125(((*n.borrow()) as i64), std_ratio______ {}) });
}
pub fn operator_shl_126(os: Ptr<std::fs::File>, d: absl_Duration) -> Ptr<std::fs::File> {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return os.write_all(
        &([(&({ FormatDuration_127((*d.borrow()).clone()) })
            .iter()
            .take(({ FormatDuration_127((*d.borrow()).clone()) }).len() - 1)
            .map(|&c| c as u8)
            .collect::<Vec<u8>>()[..] as &[u8])]
        .concat()),
    );
}
#[derive(Default)]
pub struct absl_Time_Breakdown {
    pub year: Value<i64>,
    pub month: Value<i32>,
    pub day: Value<i32>,
    pub hour: Value<i32>,
    pub minute: Value<i32>,
    pub second: Value<i32>,
    pub subsecond: Value<absl_Duration>,
    pub weekday: Value<i32>,
    pub yearday: Value<i32>,
    pub offset: Value<i32>,
    pub is_dst: Value<bool>,
    pub zone_abbr: Value<Ptr<u8>>,
}
impl Clone for absl_Time_Breakdown {
    fn clone(&self) -> Self {
        let __this: Value<absl_Time_Breakdown> = Rc::new(RefCell::new(Self {
            year: Rc::new(RefCell::new((*self.year.borrow()))),
            month: Rc::new(RefCell::new((*self.month.borrow()))),
            day: Rc::new(RefCell::new((*self.day.borrow()))),
            hour: Rc::new(RefCell::new((*self.hour.borrow()))),
            minute: Rc::new(RefCell::new((*self.minute.borrow()))),
            second: Rc::new(RefCell::new((*self.second.borrow()))),
            subsecond: Rc::new(RefCell::new((*self.subsecond.borrow()).clone())),
            weekday: Rc::new(RefCell::new((*self.weekday.borrow()))),
            yearday: Rc::new(RefCell::new((*self.yearday.borrow()))),
            offset: Rc::new(RefCell::new((*self.offset.borrow()))),
            is_dst: Rc::new(RefCell::new((*self.is_dst.borrow()))),
            zone_abbr: Rc::new(RefCell::new((*self.zone_abbr.borrow()).clone())),
        }));
        let this: Ptr<absl_Time_Breakdown> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_Time_Breakdown {
    fn byte_size() -> usize {
        64
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.year.borrow()).to_bytes(&mut buf[0..8]);
        (*self.month.borrow()).to_bytes(&mut buf[8..12]);
        (*self.day.borrow()).to_bytes(&mut buf[12..16]);
        (*self.hour.borrow()).to_bytes(&mut buf[16..20]);
        (*self.minute.borrow()).to_bytes(&mut buf[20..24]);
        (*self.second.borrow()).to_bytes(&mut buf[24..28]);
        (*self.subsecond.borrow()).to_bytes(&mut buf[28..40]);
        (*self.weekday.borrow()).to_bytes(&mut buf[40..44]);
        (*self.yearday.borrow()).to_bytes(&mut buf[44..48]);
        (*self.offset.borrow()).to_bytes(&mut buf[48..52]);
        (*self.is_dst.borrow()).to_bytes(&mut buf[52..53]);
        (*self.zone_abbr.borrow()).to_bytes(&mut buf[56..64]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            year: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
            month: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
            day: Rc::new(RefCell::new(<i32>::from_bytes(&buf[12..16]))),
            hour: Rc::new(RefCell::new(<i32>::from_bytes(&buf[16..20]))),
            minute: Rc::new(RefCell::new(<i32>::from_bytes(&buf[20..24]))),
            second: Rc::new(RefCell::new(<i32>::from_bytes(&buf[24..28]))),
            subsecond: Rc::new(RefCell::new(<absl_Duration>::from_bytes(&buf[28..40]))),
            weekday: Rc::new(RefCell::new(<i32>::from_bytes(&buf[40..44]))),
            yearday: Rc::new(RefCell::new(<i32>::from_bytes(&buf[44..48]))),
            offset: Rc::new(RefCell::new(<i32>::from_bytes(&buf[48..52]))),
            is_dst: Rc::new(RefCell::new(<bool>::from_bytes(&buf[52..53]))),
            zone_abbr: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[56..64]))),
        }
    }
}
#[derive(Default)]
pub struct absl_Time {
    rep_: Value<absl_Duration>,
}
impl absl_Time {
    fn absl_Time1(rep: absl_Duration) -> Self {
        let rep: Value<absl_Duration> = Rc::new(RefCell::new(rep));
        let __this: Value<absl_Time> = Rc::new(RefCell::new(Self {
            rep_: Rc::new(RefCell::new((*rep.borrow()).clone())),
        }));
        let this: Ptr<absl_Time> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for absl_Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_128(
                Rc::new(RefCell::new(absl_Time {
                    rep_: self.rep_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(absl_Time {
                    rep_: other.rep_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for absl_Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for absl_Time {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_129(
                Rc::new(RefCell::new(absl_Time {
                    rep_: self.rep_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(absl_Time {
                    rep_: other.rep_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for absl_Time {}
impl Clone for absl_Time {
    fn clone(&self) -> Self {
        let __this: Value<absl_Time> = Rc::new(RefCell::new(Self {
            rep_: Rc::new(RefCell::new(
                (*(*t.upgrade().deref()).rep_.borrow()).clone(),
            )),
        }));
        let this: Ptr<absl_Time> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_Time {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.rep_.borrow()).to_bytes(&mut buf[0..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            rep_: Rc::new(RefCell::new(<absl_Duration>::from_bytes(&buf[0..12]))),
        }
    }
}
pub fn operator_cmp_128(lhs: absl_Time, rhs: absl_Time) -> std::cmp::Ordering {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_Duration = (*(*lhs.borrow()).rep_.borrow()).clone();
        let _rhs: absl_Duration = (*(*rhs.borrow()).rep_.borrow()).clone();
        operator_cmp_99(_lhs, _rhs)
    });
}
pub fn operator_lt_130(lhs: absl_Time, rhs: absl_Time) -> bool {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_Duration = (*(*lhs.borrow()).rep_.borrow()).clone();
        let _rhs: absl_Duration = (*(*rhs.borrow()).rep_.borrow()).clone();
        operator_lt_102(_lhs, _rhs)
    });
}
pub fn operator_gt_131(lhs: absl_Time, rhs: absl_Time) -> bool {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_Time = (*rhs.borrow()).clone();
        operator_lt_130(_lhs, (*lhs.borrow()).clone())
    });
}
pub fn operator_ge_132(lhs: absl_Time, rhs: absl_Time) -> bool {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_Time = (*lhs.borrow()).clone();
        operator_lt_130(_lhs, (*rhs.borrow()).clone())
    });
}
pub fn operator_le_133(lhs: absl_Time, rhs: absl_Time) -> bool {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_Time = (*rhs.borrow()).clone();
        operator_lt_130(_lhs, (*lhs.borrow()).clone())
    });
}
pub fn operator_eq_129(lhs: absl_Time, rhs: absl_Time) -> bool {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_Duration = (*(*lhs.borrow()).rep_.borrow()).clone();
        let _rhs: absl_Duration = (*(*rhs.borrow()).rep_.borrow()).clone();
        operator_eq_100(_lhs, _rhs)
    });
}
pub fn operator_ne_134(lhs: absl_Time, rhs: absl_Time) -> bool {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_Time = (*lhs.borrow()).clone();
        operator_eq_129(_lhs, (*rhs.borrow()).clone())
    });
}
pub fn operator_add_135(lhs: absl_Time, rhs: absl_Duration) -> absl_Time {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return (*({ absl_TimeImpl::operator_add_assign(&lhs.as_pointer(), (*rhs.borrow()).clone()) })
        .upgrade()
        .deref())
    .clone();
}
pub fn operator_add_136(lhs: absl_Duration, rhs: absl_Time) -> absl_Time {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return (*({ absl_TimeImpl::operator_add_assign(&rhs.as_pointer(), (*lhs.borrow()).clone()) })
        .upgrade()
        .deref())
    .clone();
}
pub fn operator_sub_137(lhs: absl_Time, rhs: absl_Duration) -> absl_Time {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return (*({ absl_TimeImpl::operator_sub_assign(&lhs.as_pointer(), (*rhs.borrow()).clone()) })
        .upgrade()
        .deref())
    .clone();
}
pub fn operator_sub_138(lhs: absl_Time, rhs: absl_Time) -> absl_Duration {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_Duration = (*(*lhs.borrow()).rep_.borrow()).clone();
        let _rhs: absl_Duration = (*(*rhs.borrow()).rep_.borrow()).clone();
        operator_sub_107(_lhs, _rhs)
    });
}
pub fn UnixEpoch_139() -> absl_Time {
    return <absl_Time>::default();
}
pub fn UniversalEpoch_140() -> absl_Time {
    return absl_Time::absl_Time1({
        ({ MakeDuration_141((((-24_i32 * 719162) as i64) * 3600_i64), 0_u32) })
    });
}
pub fn InfiniteFuture_142() -> absl_Time {
    return absl_Time::absl_Time1({ ({ MakeDuration_141(<i64>::MAX, !0_u32) }) });
}
pub fn InfinitePast_143() -> absl_Time {
    return absl_Time::absl_Time1({ ({ MakeDuration_141(<i64>::MIN, !0_u32) }) });
}
pub fn operator_eq_144(a: absl_TimeZone, b: absl_TimeZone) -> bool {
    let a: Value<absl_TimeZone> = Rc::new(RefCell::new(a));
    let b: Value<absl_TimeZone> = Rc::new(RefCell::new(b));
    return ({
        let _lhs: absl_time_internal_cctz_time_zone = (*(*a.borrow()).cz_.borrow()).clone();
        let _rhs: absl_time_internal_cctz_time_zone = (*(*b.borrow()).cz_.borrow()).clone();
        operator_eq_87(_lhs, _rhs)
    });
}
pub fn operator_ne_145(a: absl_TimeZone, b: absl_TimeZone) -> bool {
    let a: Value<absl_TimeZone> = Rc::new(RefCell::new(a));
    let b: Value<absl_TimeZone> = Rc::new(RefCell::new(b));
    return ({
        let _lhs: absl_time_internal_cctz_time_zone = (*(*a.borrow()).cz_.borrow()).clone();
        let _rhs: absl_time_internal_cctz_time_zone = (*(*b.borrow()).cz_.borrow()).clone();
        operator_ne_88(_lhs, _rhs)
    });
}
pub fn operator_shl_146(os: Ptr<std::fs::File>, tz: absl_TimeZone) -> Ptr<std::fs::File> {
    let tz: Value<absl_TimeZone> = Rc::new(RefCell::new(tz));
    return os.write_all(
        &([(&({ absl_TimeZoneImpl::name(&tz.as_pointer()) })
            .iter()
            .take(({ absl_TimeZoneImpl::name(&tz.as_pointer()) }).len() - 1)
            .map(|&c| c as u8)
            .collect::<Vec<u8>>()[..] as &[u8])]
        .concat()),
    );
}
#[derive(Default)]
pub struct absl_TimeZone_CivilInfo {
    pub cs: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_>,
    pub subsecond: Value<absl_Duration>,
    pub offset: Value<i32>,
    pub is_dst: Value<bool>,
    pub zone_abbr: Value<Ptr<u8>>,
}
impl Clone for absl_TimeZone_CivilInfo {
    fn clone(&self) -> Self {
        let __this: Value<absl_TimeZone_CivilInfo> = Rc::new(RefCell::new(Self {
            cs: Rc::new(RefCell::new((*self.cs.borrow()).clone())),
            subsecond: Rc::new(RefCell::new((*self.subsecond.borrow()).clone())),
            offset: Rc::new(RefCell::new((*self.offset.borrow()))),
            is_dst: Rc::new(RefCell::new((*self.is_dst.borrow()))),
            zone_abbr: Rc::new(RefCell::new((*self.zone_abbr.borrow()).clone())),
        }));
        let this: Ptr<absl_TimeZone_CivilInfo> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_TimeZone_CivilInfo {
    fn byte_size() -> usize {
        48
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.cs.borrow()).to_bytes(&mut buf[0..16]);
        (*self.subsecond.borrow()).to_bytes(&mut buf[16..28]);
        (*self.offset.borrow()).to_bytes(&mut buf[28..32]);
        (*self.is_dst.borrow()).to_bytes(&mut buf[32..33]);
        (*self.zone_abbr.borrow()).to_bytes(&mut buf[40..48]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self { cs: Rc::new(RefCell::new(<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_ >::from_bytes(&buf[0..16]))), subsecond: Rc::new(RefCell::new(<absl_Duration >::from_bytes(&buf[16..28]))), offset: Rc::new(RefCell::new(<i32 >::from_bytes(&buf[28..32]))), is_dst: Rc::new(RefCell::new(<bool >::from_bytes(&buf[32..33]))), zone_abbr: Rc::new(RefCell::new(<Ptr::<u8> >::from_bytes(&buf[40..48]))), }
    }
}
pub type absl_TimeZone_TimeInfo_CivilKind = u32;
pub const absl_TimeZone_TimeInfo_CivilKind_UNIQUE: absl_TimeZone_TimeInfo_CivilKind = 0;
pub const absl_TimeZone_TimeInfo_CivilKind_SKIPPED: absl_TimeZone_TimeInfo_CivilKind = 1;
pub const absl_TimeZone_TimeInfo_CivilKind_REPEATED: absl_TimeZone_TimeInfo_CivilKind = 2;
#[derive(Default)]
pub struct absl_TimeZone_TimeInfo {
    pub kind: Value<absl_TimeZone_TimeInfo_CivilKind>,
    pub pre: Value<absl_Time>,
    pub trans: Value<absl_Time>,
    pub post: Value<absl_Time>,
}
impl Clone for absl_TimeZone_TimeInfo {
    fn clone(&self) -> Self {
        let __this: Value<absl_TimeZone_TimeInfo> = Rc::new(RefCell::new(Self {
            kind: Rc::new(RefCell::new((*self.kind.borrow()))),
            pre: Rc::new(RefCell::new((*self.pre.borrow()).clone())),
            trans: Rc::new(RefCell::new((*self.trans.borrow()).clone())),
            post: Rc::new(RefCell::new((*self.post.borrow()).clone())),
        }));
        let this: Ptr<absl_TimeZone_TimeInfo> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_TimeZone_TimeInfo {
    fn byte_size() -> usize {
        40
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.kind.borrow()).to_bytes(&mut buf[0..4]);
        (*self.pre.borrow()).to_bytes(&mut buf[4..16]);
        (*self.trans.borrow()).to_bytes(&mut buf[16..28]);
        (*self.post.borrow()).to_bytes(&mut buf[28..40]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            kind: Rc::new(RefCell::new(
                <absl_TimeZone_TimeInfo_CivilKind>::from_bytes(&buf[0..4]),
            )),
            pre: Rc::new(RefCell::new(<absl_Time>::from_bytes(&buf[4..16]))),
            trans: Rc::new(RefCell::new(<absl_Time>::from_bytes(&buf[16..28]))),
            post: Rc::new(RefCell::new(<absl_Time>::from_bytes(&buf[28..40]))),
        }
    }
}
#[derive(Default)]
pub struct absl_TimeZone_CivilTransition {
    pub from: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_>,
    pub to: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_>,
}
impl Clone for absl_TimeZone_CivilTransition {
    fn clone(&self) -> Self {
        let __this: Value<absl_TimeZone_CivilTransition> = Rc::new(RefCell::new(Self {
            from: Rc::new(RefCell::new((*self.from.borrow()).clone())),
            to: Rc::new(RefCell::new((*self.to.borrow()).clone())),
        }));
        let this: Ptr<absl_TimeZone_CivilTransition> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_TimeZone_CivilTransition {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.from.borrow()).to_bytes(&mut buf[0..16]);
        (*self.to.borrow()).to_bytes(&mut buf[16..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self { from: Rc::new(RefCell::new(<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_ >::from_bytes(&buf[0..16]))), to: Rc::new(RefCell::new(<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_ >::from_bytes(&buf[16..32]))), }
    }
}
#[derive(Default)]
pub struct absl_TimeZone {
    cz_: Value<absl_time_internal_cctz_time_zone>,
}
impl absl_TimeZone {
    pub fn absl_TimeZone1(tz: absl_time_internal_cctz_time_zone) -> Self {
        let tz: Value<absl_time_internal_cctz_time_zone> = Rc::new(RefCell::new(tz));
        let __this: Value<absl_TimeZone> = Rc::new(RefCell::new(Self {
            cz_: Rc::new(RefCell::new((*tz.borrow()).clone())),
        }));
        let this: Ptr<absl_TimeZone> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for absl_TimeZone {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_144(
                Rc::new(RefCell::new(absl_TimeZone {
                    cz_: self.cz_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(absl_TimeZone {
                    cz_: other.cz_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for absl_TimeZone {}
impl Clone for absl_TimeZone {
    fn clone(&self) -> Self {
        let __this: Value<absl_TimeZone> = Rc::new(RefCell::new(Self {
            cz_: Rc::new(RefCell::new((*self.cz_.borrow()).clone())),
        }));
        let this: Ptr<absl_TimeZone> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_TimeZone {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.cz_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            cz_: Rc::new(RefCell::new(
                <absl_time_internal_cctz_time_zone>::from_bytes(&buf[0..8]),
            )),
        }
    }
}
pub fn LoadTimeZone_147(name: Vec<u8>, tz: Ptr<absl_TimeZone>) -> bool {
    let name: Value<Vec<u8>> = Rc::new(RefCell::new(name));
    let tz: Value<Ptr<absl_TimeZone>> = Rc::new(RefCell::new(tz));
    if (*name.borrow()).clone()
        == Ptr::from_string_literal(b"localtime")
            .to_c_string_iterator()
            .collect::<Vec<u8>>()
    {
        let __rhs = absl_TimeZone::absl_TimeZone1({ ({ local_time_zone_148() }) });
        (*tz.borrow()).write(__rhs);
        return true;
    }
    let cz: Value<absl_time_internal_cctz_time_zone> = Rc::new(RefCell::new(
        absl_time_internal_cctz_time_zone::absl_time_internal_cctz_time_zone1(),
    ));
    let b: Value<bool> = Rc::new(RefCell::new(
        ({
            let _name : Value<Vec<u8> > = Rc::new(RefCell::new(std_basic_string_char__std_char_traits_char___std_allocator_char__ :: std_basic_string_char__std_char_traits_char___std_allocator_char__1 ( {  name .as_pointer()   } , None , ) ));
            load_time_zone_149(_name.as_pointer(), (cz.as_pointer()))
        }),
    ));
    let __rhs = absl_TimeZone::absl_TimeZone1({ (*cz.borrow()).clone() });
    (*tz.borrow()).write(__rhs);
    return (*b.borrow());
}
pub fn FixedTimeZone_150(seconds: i32) -> absl_TimeZone {
    let seconds: Value<i32> = Rc::new(RefCell::new(seconds));
    return absl_TimeZone::absl_TimeZone1({
        ({
            let _offset : Value<std_chrono_duration_long_long__std_ratio_______ > = Rc::new(RefCell::new(std_chrono_duration_long_long__std_ratio_______ :: std_chrono_duration_long_long__std_ratio_______1 ( {  seconds .as_pointer()   } , ) ));
            fixed_time_zone_151(_offset.as_pointer())
        })
    });
}
pub fn UTCTimeZone_152() -> absl_TimeZone {
    return absl_TimeZone::absl_TimeZone1({ ({ utc_time_zone_153() }) });
}
pub fn LocalTimeZone_154() -> absl_TimeZone {
    return absl_TimeZone::absl_TimeZone1({ ({ local_time_zone_148() }) });
}
pub fn ToCivilSecond_155(
    t: absl_Time,
    tz: absl_TimeZone,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_ {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    let tz: Value<absl_TimeZone> = Rc::new(RefCell::new(tz));
    return (*({ absl_TimeZoneImpl::At_absl_Time_const(&tz.as_pointer(), (*t.borrow()).clone()) })
        .cs
        .borrow())
    .clone();
}
pub fn ToCivilMinute_156(
    t: absl_Time,
    tz: absl_TimeZone,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_ {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    let tz: Value<absl_TimeZone> = Rc::new(RefCell::new(tz));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_9 ( {  (  { absl_TimeZoneImpl :: At_absl_Time_const ( &tz .as_pointer()  , ((*t.borrow()) ).clone()  , ) } )  . cs  .as_pointer()   } , None , )   ;
}
pub fn ToCivilHour_157(
    t: absl_Time,
    tz: absl_TimeZone,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_ {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    let tz: Value<absl_TimeZone> = Rc::new(RefCell::new(tz));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_12 ( {  (  { absl_TimeZoneImpl :: At_absl_Time_const ( &tz .as_pointer()  , ((*t.borrow()) ).clone()  , ) } )  . cs  .as_pointer()   } , None , )   ;
}
pub fn ToCivilDay_158(
    t: absl_Time,
    tz: absl_TimeZone,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    let tz: Value<absl_TimeZone> = Rc::new(RefCell::new(tz));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_16 ( {  (  { absl_TimeZoneImpl :: At_absl_Time_const ( &tz .as_pointer()  , ((*t.borrow()) ).clone()  , ) } )  . cs  .as_pointer()   } , None , )   ;
}
pub fn ToCivilMonth_159(
    t: absl_Time,
    tz: absl_TimeZone,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_ {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    let tz: Value<absl_TimeZone> = Rc::new(RefCell::new(tz));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_19 ( {  (  { absl_TimeZoneImpl :: At_absl_Time_const ( &tz .as_pointer()  , ((*t.borrow()) ).clone()  , ) } )  . cs  .as_pointer()   } , None , )   ;
}
pub fn ToCivilYear_160(
    t: absl_Time,
    tz: absl_TimeZone,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_ {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    let tz: Value<absl_TimeZone> = Rc::new(RefCell::new(tz));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_22 ( {  (  { absl_TimeZoneImpl :: At_absl_Time_const ( &tz .as_pointer()  , ((*t.borrow()) ).clone()  , ) } )  . cs  .as_pointer()   } , None , )   ;
}
pub fn FromCivil_161(
    ct: absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_,
    tz: absl_TimeZone,
) -> absl_Time {
    let ct: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_> =
        Rc::new(RefCell::new(ct));
    let tz: Value<absl_TimeZone> = Rc::new(RefCell::new(tz));
    let ti: Value<absl_TimeZone_TimeInfo> = Rc::new(RefCell::new(
        ({
            absl_TimeZoneImpl :: At_absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_const ( &tz .as_pointer()  , ((*ct.borrow()) ).clone()  , )
        }),
    ));
    if (((*(*ti.borrow()).kind.borrow()) as i32)
        == (absl_TimeZone_TimeInfo_CivilKind_SKIPPED as i32))
    {
        return (*(*ti.borrow()).trans.borrow()).clone();
    }
    return (*(*ti.borrow()).pre.borrow()).clone();
}
pub type absl_TimeConversion_Kind = u32;
pub const absl_TimeConversion_Kind_UNIQUE: absl_TimeConversion_Kind = 0;
pub const absl_TimeConversion_Kind_SKIPPED: absl_TimeConversion_Kind = 1;
pub const absl_TimeConversion_Kind_REPEATED: absl_TimeConversion_Kind = 2;
#[derive(Default)]
pub struct absl_TimeConversion {
    pub pre: Value<absl_Time>,
    pub trans: Value<absl_Time>,
    pub post: Value<absl_Time>,
    pub kind: Value<absl_TimeConversion_Kind>,
    pub normalized: Value<bool>,
}
impl Clone for absl_TimeConversion {
    fn clone(&self) -> Self {
        let __this: Value<absl_TimeConversion> = Rc::new(RefCell::new(Self {
            pre: Rc::new(RefCell::new((*self.pre.borrow()).clone())),
            trans: Rc::new(RefCell::new((*self.trans.borrow()).clone())),
            post: Rc::new(RefCell::new((*self.post.borrow()).clone())),
            kind: Rc::new(RefCell::new((*self.kind.borrow()))),
            normalized: Rc::new(RefCell::new((*self.normalized.borrow()))),
        }));
        let this: Ptr<absl_TimeConversion> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_TimeConversion {
    fn byte_size() -> usize {
        44
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.pre.borrow()).to_bytes(&mut buf[0..12]);
        (*self.trans.borrow()).to_bytes(&mut buf[12..24]);
        (*self.post.borrow()).to_bytes(&mut buf[24..36]);
        (*self.kind.borrow()).to_bytes(&mut buf[36..40]);
        (*self.normalized.borrow()).to_bytes(&mut buf[40..41]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            pre: Rc::new(RefCell::new(<absl_Time>::from_bytes(&buf[0..12]))),
            trans: Rc::new(RefCell::new(<absl_Time>::from_bytes(&buf[12..24]))),
            post: Rc::new(RefCell::new(<absl_Time>::from_bytes(&buf[24..36]))),
            kind: Rc::new(RefCell::new(<absl_TimeConversion_Kind>::from_bytes(
                &buf[36..40],
            ))),
            normalized: Rc::new(RefCell::new(<bool>::from_bytes(&buf[40..41]))),
        }
    }
}
pub fn FromDateTime_162(
    year: i64,
    mon: i32,
    day: i32,
    hour: i32,
    min: i32,
    sec: i32,
    tz: absl_TimeZone,
) -> absl_Time {
    let year: Value<i64> = Rc::new(RefCell::new(year));
    let mon: Value<i32> = Rc::new(RefCell::new(mon));
    let day: Value<i32> = Rc::new(RefCell::new(day));
    let hour: Value<i32> = Rc::new(RefCell::new(hour));
    let min: Value<i32> = Rc::new(RefCell::new(min));
    let sec: Value<i32> = Rc::new(RefCell::new(sec));
    let tz: Value<absl_TimeZone> = Rc::new(RefCell::new(tz));
    return (*({
        ConvertDateTime_163(
            (*year.borrow()),
            (*mon.borrow()),
            (*day.borrow()),
            (*hour.borrow()),
            (*min.borrow()),
            (*sec.borrow()),
            (*tz.borrow()).clone(),
        )
    })
    .pre
    .borrow())
    .clone();
}
thread_local!();
thread_local!();
thread_local!();
thread_local!();
pub fn operator_shl_168(os: Ptr<std::fs::File>, t: absl_Time) -> Ptr<std::fs::File> {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    return os.write_all(
        &([(&({ FormatTime_169((*t.borrow()).clone()) })
            .iter()
            .take(({ FormatTime_169((*t.borrow()).clone()) }).len() - 1)
            .map(|&c| c as u8)
            .collect::<Vec<u8>>()[..] as &[u8])]
        .concat()),
    );
}
pub fn MakeDuration_141(hi: i64, lo: Option<u32>) -> absl_Duration {
    let hi: Value<i64> = Rc::new(RefCell::new(hi));
    let lo: Value<u32> = Rc::new(RefCell::new(lo.unwrap_or(0_u32)));
    return absl_Duration::absl_Duration2({ (*hi.borrow()) }, { (*lo.borrow()) });
}
pub fn MakeDuration_170(hi: i64, lo: i64) -> absl_Duration {
    let hi: Value<i64> = Rc::new(RefCell::new(hi));
    let lo: Value<i64> = Rc::new(RefCell::new(lo));
    return ({ MakeDuration_141((*hi.borrow()), Some(((*lo.borrow()) as u32))) });
}
pub fn MakePosDoubleDuration_171(n: f64) -> absl_Duration {
    let n: Value<f64> = Rc::new(RefCell::new(n));
    let int_secs: Value<i64> = Rc::new(RefCell::new(((*n.borrow()) as i64)));
    let ticks: Value<u32> = Rc::new(RefCell::new(
        ((((*n.borrow()) - ((*int_secs.borrow()) as f64)) * (4000000000 as f64)).round() as u32),
    ));
    return if (((*ticks.borrow()) as i64) < 4000000000) {
        ({ MakeDuration_141((*int_secs.borrow()), Some((*ticks.borrow()))) })
    } else {
        ({
            MakeDuration_170(
                ((*int_secs.borrow()) + 1_i64),
                (((*ticks.borrow()) as i64) - 4000000000),
            )
        })
    };
}
pub fn MakeNormalizedDuration_95(sec: i64, ticks: i64) -> absl_Duration {
    let sec: Value<i64> = Rc::new(RefCell::new(sec));
    let ticks: Value<i64> = Rc::new(RefCell::new(ticks));
    return if ((*ticks.borrow()) < 0_i64) {
        ({ MakeDuration_170(((*sec.borrow()) - 1_i64), ((*ticks.borrow()) + 4000000000)) })
    } else {
        ({ MakeDuration_170((*sec.borrow()), (*ticks.borrow())) })
    };
}
pub fn GetRepHi_172(d: absl_Duration) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return ({ absl_Duration_HiRepImpl::Get(&(*d.borrow()).rep_hi_.as_pointer()) });
}
pub fn GetRepLo_173(d: absl_Duration) -> u32 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return (*(*d.borrow()).rep_lo_.borrow());
}
pub fn IsInfiniteDuration_174(d: absl_Duration) -> bool {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return (({ GetRepLo_173((*d.borrow()).clone()) }) == !0_u32);
}
pub fn OppositeInfinity_175(d: absl_Duration) -> absl_Duration {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return if (({ GetRepHi_172((*d.borrow()).clone()) }) < 0_i64) {
        ({ MakeDuration_141(<i64>::MAX, Some(!0_u32)) })
    } else {
        ({ MakeDuration_141(<i64>::MIN, Some(!0_u32)) })
    };
}
pub fn NegateAndSubtractOne_176(n: i64) -> i64 {
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return if ((*n.borrow()) < 0_i64) {
        -((*n.borrow()) + 1_i64)
    } else {
        ((-(*n.borrow())) - 1_i64)
    };
}
pub fn FromUnixDuration_177(d: absl_Duration) -> absl_Time {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return absl_Time::absl_Time1({ (*d.borrow()).clone() });
}
pub fn ToUnixDuration_178(t: absl_Time) -> absl_Duration {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    return (*(*t.borrow()).rep_.borrow()).clone();
}
pub fn FromInt64_123(v: i64, _a1: std_ratio______) -> absl_Duration {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return if (((*v.borrow()) <= (<i64>::MAX / 60_i64)) && ((*v.borrow()) >= (<i64>::MIN / 60_i64)))
    {
        ({ MakeDuration_141(((*v.borrow()) * 60_i64), None) })
    } else {
        if ((*v.borrow()) > 0_i64) {
            ({ InfiniteDuration_179() })
        } else {
            ({
                let _d: absl_Duration = ({ InfiniteDuration_179() });
                operator_neg_113(_d)
            })
        }
    };
}
pub fn FromInt64_125(v: i64, _a1: std_ratio______) -> absl_Duration {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return if (((*v.borrow()) <= (<i64>::MAX / 3600_i64))
        && ((*v.borrow()) >= (<i64>::MIN / 3600_i64)))
    {
        ({ MakeDuration_141(((*v.borrow()) * 3600_i64), None) })
    } else {
        if ((*v.borrow()) > 0_i64) {
            ({ InfiniteDuration_179() })
        } else {
            ({
                let _d: absl_Duration = ({ InfiniteDuration_179() });
                operator_neg_113(_d)
            })
        }
    };
}
pub fn IsValidRep64_180(_a0: i32) -> bool {
    let _a0: Value<i32> = Rc::new(RefCell::new(_a0));
    return true;
}
pub fn IsValidRep64_181(_a0: i32) -> bool {
    let _a0: Value<i32> = Rc::new(RefCell::new(_a0));
    return true;
}
pub fn FromChrono_182(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromInt64_94(({ (*d.upgrade().deref()).count() }), std_ratio______ {}) });
}
pub fn FromChrono_183(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromInt64_96(({ (*d.upgrade().deref()).count() }), std_ratio______ {}) });
}
pub fn FromChrono_184(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromInt64_97(({ (*d.upgrade().deref()).count() }), std_ratio______ {}) });
}
pub fn FromChrono_185(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromInt64_98(({ (*d.upgrade().deref()).count() }), std_ratio______ {}) });
}
pub fn FromChrono_186(d: Ptr<std_chrono_duration_long__std_ratio_______>) -> absl_Duration {
    return ({ FromInt64_123(({ (*d.upgrade().deref()).count() }), std_ratio______ {}) });
}
pub fn FromChrono_187(d: Ptr<std_chrono_duration_long__std_ratio_______>) -> absl_Duration {
    return ({ FromInt64_125(({ (*d.upgrade().deref()).count() }), std_ratio______ {}) });
}
pub fn ToInt64_188(d: absl_Duration, _a1: std_ratio______) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({ ToInt64Nanoseconds_189((*d.borrow()).clone()) });
}
pub fn ToInt64_190(d: absl_Duration, _a1: std_ratio______) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({ ToInt64Microseconds_191((*d.borrow()).clone()) });
}
pub fn ToInt64_192(d: absl_Duration, _a1: std_ratio______) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({ ToInt64Milliseconds_193((*d.borrow()).clone()) });
}
pub fn ToInt64_194(d: absl_Duration, _a1: std_ratio______) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({ ToInt64Seconds_195((*d.borrow()).clone()) });
}
pub fn ToInt64_196(d: absl_Duration, _a1: std_ratio______) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({ ToInt64Minutes_197((*d.borrow()).clone()) });
}
pub fn ToInt64_198(d: absl_Duration, _a1: std_ratio______) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({ ToInt64Hours_199((*d.borrow()).clone()) });
}
pub fn operator_lt_102(lhs: absl_Duration, rhs: absl_Duration) -> bool {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return if (({ GetRepHi_172((*lhs.borrow()).clone()) })
        != ({ GetRepHi_172((*rhs.borrow()).clone()) }))
    {
        (({ GetRepHi_172((*lhs.borrow()).clone()) }) < ({ GetRepHi_172((*rhs.borrow()).clone()) }))
    } else {
        if (({ GetRepHi_172((*lhs.borrow()).clone()) }) == <i64>::MIN) {
            (({ GetRepLo_173((*lhs.borrow()).clone()) }).wrapping_add(1_u32)
                < ({ GetRepLo_173((*rhs.borrow()).clone()) }).wrapping_add(1_u32))
        } else {
            (({ GetRepLo_173((*lhs.borrow()).clone()) })
                < ({ GetRepLo_173((*rhs.borrow()).clone()) }))
        }
    };
}
pub fn operator_cmp_99(lhs: absl_Duration, rhs: absl_Duration) -> std::cmp::Ordering {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    let lhs_hi: Value<i64> = Rc::new(RefCell::new(({ GetRepHi_172((*lhs.borrow()).clone()) })));
    let rhs_hi: Value<i64> = Rc::new(RefCell::new(({ GetRepHi_172((*rhs.borrow()).clone()) })));
    {
        let c: Value<std::cmp::Ordering> =
            Rc::new(RefCell::new((*lhs_hi.borrow()).cmp(&(*rhs_hi.borrow()))));
        if !((*c.borrow()) == std::cmp::Ordering::Equal) {
            return (*c.borrow_mut()).clone();
        }
    }
    let lhs_lo: Value<u32> = Rc::new(RefCell::new(({ GetRepLo_173((*lhs.borrow()).clone()) })));
    let rhs_lo: Value<u32> = Rc::new(RefCell::new(({ GetRepLo_173((*rhs.borrow()).clone()) })));
    return if ((*lhs_hi.borrow()) == <i64>::MIN) {
        ((*lhs_lo.borrow()).wrapping_add(1_u32)).cmp(&((*rhs_lo.borrow()).wrapping_add(1_u32)))
    } else {
        (*lhs_lo.borrow()).cmp(&(*rhs_lo.borrow()))
    };
}
pub fn operator_eq_100(lhs: absl_Duration, rhs: absl_Duration) -> bool {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return (({ GetRepHi_172((*lhs.borrow()).clone()) })
        == ({ GetRepHi_172((*rhs.borrow()).clone()) }))
        && (({ GetRepLo_173((*lhs.borrow()).clone()) })
            == ({ GetRepLo_173((*rhs.borrow()).clone()) }));
}
pub fn operator_neg_113(d: absl_Duration) -> absl_Duration {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return if (({ GetRepLo_173((*d.borrow()).clone()) }) == 0_u32) {
        if (({ GetRepHi_172((*d.borrow()).clone()) }) == <i64>::MIN) {
            ({ InfiniteDuration_179() })
        } else {
            ({ MakeDuration_141(-({ GetRepHi_172((*d.borrow()).clone()) }), None) })
        }
    } else {
        if ({ IsInfiniteDuration_174((*d.borrow()).clone()) }) {
            ({ OppositeInfinity_175((*d.borrow()).clone()) })
        } else {
            ({
                let _hi: i64 =
                    ({ NegateAndSubtractOne_176(({ GetRepHi_172((*d.borrow()).clone()) })) });
                let _lo: i64 = (4000000000 - (({ GetRepLo_173((*d.borrow()).clone()) }) as i64));
                MakeDuration_170(_hi, _lo)
            })
        }
    };
}
pub fn InfiniteDuration_179() -> absl_Duration {
    return ({ MakeDuration_141(<i64>::MAX, Some(!0_u32)) });
}
pub fn FromChrono_200(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromChrono_182((d).clone()) });
}
pub fn FromChrono_201(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromChrono_183((d).clone()) });
}
pub fn FromChrono_202(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromChrono_184((d).clone()) });
}
pub fn FromChrono_203(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromChrono_185((d).clone()) });
}
pub fn FromChrono_204(d: Ptr<std_chrono_duration_long__std_ratio_______>) -> absl_Duration {
    return ({ FromChrono_186((d).clone()) });
}
pub fn FromChrono_205(d: Ptr<std_chrono_duration_long__std_ratio_______>) -> absl_Duration {
    return ({ FromChrono_187((d).clone()) });
}
pub fn FromUnixNanos_206(ns: i64) -> absl_Time {
    let ns: Value<i64> = Rc::new(RefCell::new(ns));
    return ({ FromUnixDuration_177(({ Nanoseconds_115((*ns.borrow())) })) });
}
pub fn FromUnixMicros_207(us: i64) -> absl_Time {
    let us: Value<i64> = Rc::new(RefCell::new(us));
    return ({ FromUnixDuration_177(({ Microseconds_117((*us.borrow())) })) });
}
pub fn FromUnixMillis_208(ms: i64) -> absl_Time {
    let ms: Value<i64> = Rc::new(RefCell::new(ms));
    return ({ FromUnixDuration_177(({ Milliseconds_119((*ms.borrow())) })) });
}
pub fn FromUnixSeconds_209(s: i64) -> absl_Time {
    let s: Value<i64> = Rc::new(RefCell::new(s));
    return ({ FromUnixDuration_177(({ Seconds_120((*s.borrow())) })) });
}
pub fn FromTimeT_210(t: i64) -> absl_Time {
    let t: Value<i64> = Rc::new(RefCell::new(t));
    return ({ FromUnixDuration_177(({ Seconds_121((*t.borrow())) })) });
}
pub fn ToInt64Nanoseconds_189(d: absl_Duration) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    if (({ GetRepHi_172((*d.borrow()).clone()) }) >= 0_i64)
        && ((({ GetRepHi_172((*d.borrow()).clone()) }) >> 33) == 0_i64)
    {
        return ((((({ GetRepHi_172((*d.borrow()).clone()) }) * 1000_i64) * 1000_i64) * 1000_i64)
            + ((({ GetRepLo_173((*d.borrow()).clone()) }) as i64) / 4));
    } else {
        return ({
            let _lhs: absl_Duration = (*d.borrow()).clone();
            operator_div_108(_lhs, ({ Nanoseconds_114(1) }))
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ToInt64Microseconds_191(d: absl_Duration) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    if (({ GetRepHi_172((*d.borrow()).clone()) }) >= 0_i64)
        && ((({ GetRepHi_172((*d.borrow()).clone()) }) >> 43) == 0_i64)
    {
        return (((({ GetRepHi_172((*d.borrow()).clone()) }) * 1000_i64) * 1000_i64)
            + ((({ GetRepLo_173((*d.borrow()).clone()) }) as i64) / (4 * 1000_i64)));
    } else {
        return ({
            let _lhs: absl_Duration = (*d.borrow()).clone();
            operator_div_108(_lhs, ({ Microseconds_116(1) }))
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ToInt64Milliseconds_193(d: absl_Duration) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    if (({ GetRepHi_172((*d.borrow()).clone()) }) >= 0_i64)
        && ((({ GetRepHi_172((*d.borrow()).clone()) }) >> 53) == 0_i64)
    {
        return ((({ GetRepHi_172((*d.borrow()).clone()) }) * 1000_i64)
            + ((({ GetRepLo_173((*d.borrow()).clone()) }) as i64) / ((4 * 1000_i64) * 1000_i64)));
    } else {
        return ({
            let _lhs: absl_Duration = (*d.borrow()).clone();
            operator_div_108(_lhs, ({ Milliseconds_118(1) }))
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ToInt64Seconds_195(d: absl_Duration) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let hi: Value<i64> = Rc::new(RefCell::new(({ GetRepHi_172((*d.borrow()).clone()) })));
    if ({ IsInfiniteDuration_174((*d.borrow()).clone()) }) {
        return (*hi.borrow());
    }
    if ((*hi.borrow()) < 0_i64) && (({ GetRepLo_173((*d.borrow()).clone()) }) != 0_u32) {
        (*hi.borrow_mut()).prefix_inc();
    }
    return (*hi.borrow());
}
pub fn ToInt64Minutes_197(d: absl_Duration) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let hi: Value<i64> = Rc::new(RefCell::new(({ GetRepHi_172((*d.borrow()).clone()) })));
    if ({ IsInfiniteDuration_174((*d.borrow()).clone()) }) {
        return (*hi.borrow());
    }
    if ((*hi.borrow()) < 0_i64) && (({ GetRepLo_173((*d.borrow()).clone()) }) != 0_u32) {
        (*hi.borrow_mut()).prefix_inc();
    }
    return ((*hi.borrow()) / 60_i64);
}
pub fn ToInt64Hours_199(d: absl_Duration) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let hi: Value<i64> = Rc::new(RefCell::new(({ GetRepHi_172((*d.borrow()).clone()) })));
    if ({ IsInfiniteDuration_174((*d.borrow()).clone()) }) {
        return (*hi.borrow());
    }
    if ((*hi.borrow()) < 0_i64) && (({ GetRepLo_173((*d.borrow()).clone()) }) != 0_u32) {
        (*hi.borrow_mut()).prefix_inc();
    }
    return ((*hi.borrow()) / ((60 * 60) as i64));
}
pub fn SleepFor_211(duration: absl_Duration) {
    let duration: Value<absl_Duration> = Rc::new(RefCell::new(duration));
    ({ AbslInternalSleepFor_212((*duration.borrow()).clone()) });
}
thread_local!(
    static kNoTimeout_213: Value<u64> = Rc::new(RefCell::new(18446744073709551615));
);
thread_local!(
    static kMaxNanos_214: Value<i64> = Rc::new(RefCell::new(9223372036854775807));
);
#[derive()]
pub struct absl_synchronization_internal_KernelTimeout {
    rep_: Value<u64>,
}
impl absl_synchronization_internal_KernelTimeout {
    pub fn absl_synchronization_internal_KernelTimeout1() -> Self {
        let __this: Value<absl_synchronization_internal_KernelTimeout> =
            Rc::new(RefCell::new(Self {
                rep_: Rc::new(RefCell::new(18446744073709551615)),
            }));
        let this: Ptr<absl_synchronization_internal_KernelTimeout> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Never() -> absl_synchronization_internal_KernelTimeout {
        return absl_synchronization_internal_KernelTimeout :: absl_synchronization_internal_KernelTimeout1 ( )   ;
    }
    pub fn SupportsSteadyClock() -> bool {
        return true;
    }
}
impl Clone for absl_synchronization_internal_KernelTimeout {
    fn clone(&self) -> Self {
        let __this: Value<absl_synchronization_internal_KernelTimeout> =
            Rc::new(RefCell::new(Self {
                rep_: Rc::new(RefCell::new((*self.rep_.borrow()))),
            }));
        let this: Ptr<absl_synchronization_internal_KernelTimeout> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_synchronization_internal_KernelTimeout {
    fn default() -> Self {
        {
            absl_synchronization_internal_KernelTimeout :: absl_synchronization_internal_KernelTimeout1 ( )
        }
    }
}
impl ByteRepr for absl_synchronization_internal_KernelTimeout {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.rep_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            rep_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
        }
    }
}
pub fn GetOrCreateCurrentThreadIdentity_215() -> Ptr<absl_base_internal_ThreadIdentity> {
    let identity: Value<Ptr<absl_base_internal_ThreadIdentity>> =
        Rc::new(RefCell::new(({ CurrentThreadIdentityIfPresent_216() })));
    if ((((false) || ((*identity.borrow()).is_null())) as i64) != 0) {
        return ({ CreateThreadIdentity_217() });
    }
    return (*identity.borrow()).clone();
}
#[derive(Default)]
pub struct absl_synchronization_internal_PerThreadSem {}
impl ByteRepr for absl_synchronization_internal_PerThreadSem {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
impl absl_synchronization_internal_PerThreadSem {
    fn Init(identity: Ptr<absl_base_internal_ThreadIdentity>) {
        let identity: Value<Ptr<absl_base_internal_ThreadIdentity>> =
            Rc::new(RefCell::new(identity));
        ({ AbslInternalPerThreadSemInit_218((*identity.borrow()).clone()) });
    }
}
impl absl_synchronization_internal_PerThreadSem {
    fn Post(identity: Ptr<absl_base_internal_ThreadIdentity>) {
        let identity: Value<Ptr<absl_base_internal_ThreadIdentity>> =
            Rc::new(RefCell::new(identity));
        ({ AbslInternalPerThreadSemPost_219((*identity.borrow()).clone()) });
    }
}
impl absl_synchronization_internal_PerThreadSem {
    fn Wait(t: absl_synchronization_internal_KernelTimeout) -> bool {
        let t: Value<absl_synchronization_internal_KernelTimeout> = Rc::new(RefCell::new(t));
        return ({ AbslInternalPerThreadSemWait_220((*t.borrow()).clone()) });
    }
}
impl absl_synchronization_internal_PerThreadSem {
    fn WaitAbsolute(t: absl_Time) -> bool {
        let t: Value<absl_Time> = Rc::new(RefCell::new(t));
        return ({
            absl_synchronization_internal_PerThreadSem::Wait ( absl_synchronization_internal_KernelTimeout :: absl_synchronization_internal_KernelTimeout2 ( {  ((*t.borrow()) ).clone()   } , )  , )
        });
    }
}
#[derive()]
pub struct absl_Mutex {
    mu_: Value<std_atomic_long_>,
}
impl absl_Mutex {
    pub fn absl_Mutex1() -> Self {
        let __this: Value<absl_Mutex> = Rc::new(RefCell::new(Self {
            mu_: Rc::new(RefCell::new(std_atomic_long_::std_atomic_long_1({ 0_i64 }))),
        }));
        let this: Ptr<absl_Mutex> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_Mutex2(_a0: absl_ConstInitType) -> Self {
        let _a0: Value<absl_ConstInitType> = Rc::new(RefCell::new(_a0));
        let __this: Value<absl_Mutex> = Rc::new(RefCell::new(Self {
            mu_: Rc::new(RefCell::new(std_atomic_long_::std_atomic_long_1({ 0_i64 }))),
        }));
        let this: Ptr<absl_Mutex> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    fn absl_Mutex3(_a0: Ptr<absl_Mutex>) -> Self {
        let _a0: Value<Ptr<absl_Mutex>> = Rc::new(RefCell::new(_a0));
        let __this: Value<absl_Mutex> = Rc::new(RefCell::new(Self {
            mu_: Rc::new(RefCell::new(<std_atomic_long_>::default())),
        }));
        let this: Ptr<absl_Mutex> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_Mutex {
    fn default() -> Self {
        { absl_Mutex::absl_Mutex1() }
    }
}
impl ByteRepr for absl_Mutex {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.mu_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            mu_: Rc::new(RefCell::new(<std_atomic_long_>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct absl_MutexLock {
    mu_: Ptr<absl_Mutex>,
}
impl absl_MutexLock {
    pub fn absl_MutexLock1(mu: Ptr<absl_Mutex>) -> Self {
        let __this: Value<absl_MutexLock> = Rc::new(RefCell::new(Self { mu_: (mu).clone() }));
        let this: Ptr<absl_MutexLock> = __this.as_pointer();
        ({ absl_MutexImpl::lock(&(*this.upgrade().deref()).mu_) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_MutexLock2(mu: Ptr<absl_Mutex>) -> Self {
        let mu: Value<Ptr<absl_Mutex>> = Rc::new(RefCell::new(mu));
        let __this: Value<absl_MutexLock> = Rc::new(RefCell::new(Self {
            mu_: <Ptr<absl_Mutex>>::default(),
        }));
        let this: Ptr<absl_MutexLock> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_MutexLock3(mu: Ptr<absl_Mutex>, cond: Ptr<absl_Condition>) -> Self {
        let __this: Value<absl_MutexLock> = Rc::new(RefCell::new(Self { mu_: (mu).clone() }));
        let this: Ptr<absl_MutexLock> = __this.as_pointer();
        ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            absl_MutexImpl::LockWhen(&(*this.upgrade().deref()).mu_, _cond)
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_MutexLock4(mu: Ptr<absl_Mutex>, cond: Ptr<absl_Condition>) -> Self {
        let mu: Value<Ptr<absl_Mutex>> = Rc::new(RefCell::new(mu));
        let __this: Value<absl_MutexLock> = Rc::new(RefCell::new(Self {
            mu_: <Ptr<absl_Mutex>>::default(),
        }));
        let this: Ptr<absl_MutexLock> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_MutexLock {}
#[derive(Default)]
pub struct absl_ReaderMutexLock {
    mu_: Ptr<absl_Mutex>,
}
impl absl_ReaderMutexLock {
    pub fn absl_ReaderMutexLock1(mu: Ptr<absl_Mutex>) -> Self {
        let __this: Value<absl_ReaderMutexLock> = Rc::new(RefCell::new(Self { mu_: (mu).clone() }));
        let this: Ptr<absl_ReaderMutexLock> = __this.as_pointer();
        ({ absl_MutexImpl::lock_shared(&mu) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_ReaderMutexLock2(mu: Ptr<absl_Mutex>) -> Self {
        let mu: Value<Ptr<absl_Mutex>> = Rc::new(RefCell::new(mu));
        let __this: Value<absl_ReaderMutexLock> = Rc::new(RefCell::new(Self {
            mu_: <Ptr<absl_Mutex>>::default(),
        }));
        let this: Ptr<absl_ReaderMutexLock> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_ReaderMutexLock3(mu: Ptr<absl_Mutex>, cond: Ptr<absl_Condition>) -> Self {
        let __this: Value<absl_ReaderMutexLock> = Rc::new(RefCell::new(Self { mu_: (mu).clone() }));
        let this: Ptr<absl_ReaderMutexLock> = __this.as_pointer();
        ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            absl_MutexImpl::ReaderLockWhen(&mu, _cond)
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_ReaderMutexLock4(mu: Ptr<absl_Mutex>, cond: Ptr<absl_Condition>) -> Self {
        let mu: Value<Ptr<absl_Mutex>> = Rc::new(RefCell::new(mu));
        let __this: Value<absl_ReaderMutexLock> = Rc::new(RefCell::new(Self {
            mu_: <Ptr<absl_Mutex>>::default(),
        }));
        let this: Ptr<absl_ReaderMutexLock> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_ReaderMutexLock {}
#[derive(Default)]
pub struct absl_WriterMutexLock {
    mu_: Ptr<absl_Mutex>,
}
impl absl_WriterMutexLock {
    pub fn absl_WriterMutexLock1(mu: Ptr<absl_Mutex>) -> Self {
        let __this: Value<absl_WriterMutexLock> = Rc::new(RefCell::new(Self { mu_: (mu).clone() }));
        let this: Ptr<absl_WriterMutexLock> = __this.as_pointer();
        ({ absl_MutexImpl::lock(&mu) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_WriterMutexLock2(mu: Ptr<absl_Mutex>) -> Self {
        let mu: Value<Ptr<absl_Mutex>> = Rc::new(RefCell::new(mu));
        let __this: Value<absl_WriterMutexLock> = Rc::new(RefCell::new(Self {
            mu_: <Ptr<absl_Mutex>>::default(),
        }));
        let this: Ptr<absl_WriterMutexLock> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_WriterMutexLock3(mu: Ptr<absl_Mutex>, cond: Ptr<absl_Condition>) -> Self {
        let __this: Value<absl_WriterMutexLock> = Rc::new(RefCell::new(Self { mu_: (mu).clone() }));
        let this: Ptr<absl_WriterMutexLock> = __this.as_pointer();
        ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            absl_MutexImpl::WriterLockWhen(&mu, _cond)
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_WriterMutexLock4(mu: Ptr<absl_Mutex>, cond: Ptr<absl_Condition>) -> Self {
        let mu: Value<Ptr<absl_Mutex>> = Rc::new(RefCell::new(mu));
        let __this: Value<absl_WriterMutexLock> = Rc::new(RefCell::new(Self {
            mu_: <Ptr<absl_Mutex>>::default(),
        }));
        let this: Ptr<absl_WriterMutexLock> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_WriterMutexLock {}
thread_local!();
#[derive()]
pub struct absl_Condition {
    callback_: Value<Box<[u8]>>,
    eval_: Value<FnPtr<fn(Ptr<absl_Condition>) -> bool>>,
    arg_: Value<AnyPtr>,
}
impl absl_Condition {
    fn AlwaysTrue(_a0: Ptr<absl_Condition>) -> bool {
        let _a0: Value<Ptr<absl_Condition>> = Rc::new(RefCell::new(_a0));
        return true;
    }
    fn absl_Condition1() -> Self {
        let __this: Value<absl_Condition> = Rc::new(RefCell::new(Self {
            callback_: Rc::new(RefCell::new(Box::new([
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
            ]))),
            eval_: Rc::new(RefCell::new(FnPtr::<fn(Ptr<absl_Condition>) -> bool>::new(
                AlwaysTrue,
            ))),
            arg_: Rc::new(RefCell::new(AnyPtr::default())),
        }));
        let this: Ptr<absl_Condition> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for absl_Condition {
    fn clone(&self) -> Self {
        let __this: Value<absl_Condition> = Rc::new(RefCell::new(Self {
            callback_: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 16, _>(
                |__i: usize| (*self.callback_.borrow())[(__i) as usize],
            )))),
            eval_: Rc::new(RefCell::new((*self.eval_.borrow()).clone())),
            arg_: Rc::new(RefCell::new((*self.arg_.borrow()).clone())),
        }));
        let this: Ptr<absl_Condition> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_Condition {
    fn default() -> Self {
        { absl_Condition::absl_Condition1() }
    }
}
impl ByteRepr for absl_Condition {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.callback_.borrow()).to_bytes(&mut buf[0..16]);
        (*self.eval_.borrow()).to_bytes(&mut buf[16..24]);
        (*self.arg_.borrow()).to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            callback_: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[0..16]))),
            eval_: Rc::new(RefCell::new(
                <FnPtr<fn(Ptr<absl_Condition>) -> bool>>::from_bytes(&buf[16..24]),
            )),
            arg_: Rc::new(RefCell::new(<AnyPtr>::from_bytes(&buf[24..32]))),
        }
    }
}
#[derive()]
pub struct absl_CondVar {
    cv_: Value<std_atomic_long_>,
}
impl absl_CondVar {
    pub fn absl_CondVar() -> Self {
        let __this: Value<absl_CondVar> = Rc::new(RefCell::new(Self {
            cv_: Rc::new(RefCell::new(std_atomic_long_::std_atomic_long_1({ 0_i64 }))),
        }));
        let this: Ptr<absl_CondVar> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_CondVar {
    fn default() -> Self {
        { absl_CondVar::absl_CondVar() }
    }
}
impl ByteRepr for absl_CondVar {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.cv_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            cv_: Rc::new(RefCell::new(<std_atomic_long_>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct absl_MutexLockMaybe {
    mu_: Value<Ptr<absl_Mutex>>,
}
impl absl_MutexLockMaybe {
    pub fn absl_MutexLockMaybe1(mu: Ptr<absl_Mutex>) -> Self {
        let mu: Value<Ptr<absl_Mutex>> = Rc::new(RefCell::new(mu));
        let __this: Value<absl_MutexLockMaybe> = Rc::new(RefCell::new(Self {
            mu_: Rc::new(RefCell::new((*mu.borrow()).clone())),
        }));
        let this: Ptr<absl_MutexLockMaybe> = __this.as_pointer();
        if !((*(*this.upgrade().deref()).mu_.borrow()).is_null()) {
            ({ absl_MutexImpl::lock(&(*(*this.upgrade().deref()).mu_.borrow())) });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_MutexLockMaybe2(mu: Ptr<absl_Mutex>, cond: Ptr<absl_Condition>) -> Self {
        let mu: Value<Ptr<absl_Mutex>> = Rc::new(RefCell::new(mu));
        let __this: Value<absl_MutexLockMaybe> = Rc::new(RefCell::new(Self {
            mu_: Rc::new(RefCell::new((*mu.borrow()).clone())),
        }));
        let this: Ptr<absl_MutexLockMaybe> = __this.as_pointer();
        if !((*(*this.upgrade().deref()).mu_.borrow()).is_null()) {
            ({
                let _cond: Ptr<absl_Condition> = (cond).clone();
                absl_MutexImpl::LockWhen(&(*(*this.upgrade().deref()).mu_.borrow()), _cond)
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_MutexLockMaybe {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.mu_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            mu_: Rc::new(RefCell::new(<Ptr<absl_Mutex>>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct absl_ReleasableMutexLock {
    mu_: Value<Ptr<absl_Mutex>>,
}
impl absl_ReleasableMutexLock {
    pub fn absl_ReleasableMutexLock1(mu: Ptr<absl_Mutex>) -> Self {
        let __this: Value<absl_ReleasableMutexLock> = Rc::new(RefCell::new(Self {
            mu_: Rc::new(RefCell::new((mu).clone())),
        }));
        let this: Ptr<absl_ReleasableMutexLock> = __this.as_pointer();
        ({ absl_MutexImpl::lock(&(*(*this.upgrade().deref()).mu_.borrow())) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_ReleasableMutexLock2(mu: Ptr<absl_Mutex>) -> Self {
        let mu: Value<Ptr<absl_Mutex>> = Rc::new(RefCell::new(mu));
        let __this: Value<absl_ReleasableMutexLock> = Rc::new(RefCell::new(Self {
            mu_: Rc::new(RefCell::new(Ptr::<absl_Mutex>::null())),
        }));
        let this: Ptr<absl_ReleasableMutexLock> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_ReleasableMutexLock3(mu: Ptr<absl_Mutex>, cond: Ptr<absl_Condition>) -> Self {
        let __this: Value<absl_ReleasableMutexLock> = Rc::new(RefCell::new(Self {
            mu_: Rc::new(RefCell::new((mu).clone())),
        }));
        let this: Ptr<absl_ReleasableMutexLock> = __this.as_pointer();
        ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            absl_MutexImpl::LockWhen(&(*(*this.upgrade().deref()).mu_.borrow()), _cond)
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn absl_ReleasableMutexLock4(mu: Ptr<absl_Mutex>, cond: Ptr<absl_Condition>) -> Self {
        let mu: Value<Ptr<absl_Mutex>> = Rc::new(RefCell::new(mu));
        let __this: Value<absl_ReleasableMutexLock> = Rc::new(RefCell::new(Self {
            mu_: Rc::new(RefCell::new(Ptr::<absl_Mutex>::null())),
        }));
        let this: Ptr<absl_ReleasableMutexLock> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_ReleasableMutexLock {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.mu_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            mu_: Rc::new(RefCell::new(<Ptr<absl_Mutex>>::from_bytes(&buf[0..8]))),
        }
    }
}
pub type absl_OnDeadlockCycle = i32;
pub const absl_OnDeadlockCycle_kIgnore: absl_OnDeadlockCycle = 0;
pub const absl_OnDeadlockCycle_kReport: absl_OnDeadlockCycle = 1;
pub const absl_OnDeadlockCycle_kAbort: absl_OnDeadlockCycle = 2;
pub type anon_222 = u8;
pub const anon_222_ONCE_STATE_UNINITIALIZED: anon_222 = 0;
pub const anon_222_ONCE_STATE_EXECUTING_FUNCTION: anon_222 = 1;
pub const anon_222_ONCE_STATE_DONE: anon_222 = 2;
pub fn CallOnce_223(once: Ptr<std_atomic_unsigned_char_>, init_func: std_function_void____) {
    let once: Value<Ptr<std_atomic_unsigned_char_>> = Rc::new(RefCell::new(once));
    let init_func: Value<std_function_void____> = Rc::new(RefCell::new(init_func));
    if ((({ (*(*once.borrow()).upgrade().deref()).load_const(Some(2)) }) as i32)
        != (anon_222_ONCE_STATE_DONE as i32))
    {
        ({ CallOnceImpl_224((*once.borrow()).clone(), (*init_func.borrow()).clone()) });
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
pub struct v8_base_LazyStaticInstance_v8_base_Mutex__v8_base_DefaultConstructTrait_v8_base_Mutex___v8_base_ThreadSafeInitOnceTrait__v8_base_LeakyInstanceTrait_v8_base_Mutex__
{}
impl Clone for v8_base_LazyStaticInstance_v8_base_Mutex__v8_base_DefaultConstructTrait_v8_base_Mutex___v8_base_ThreadSafeInitOnceTrait__v8_base_LeakyInstanceTrait_v8_base_Mutex__ { fn clone(&self) -> Self { let __this : Value<v8_base_LazyStaticInstance_v8_base_Mutex__v8_base_DefaultConstructTrait_v8_base_Mutex___v8_base_ThreadSafeInitOnceTrait__v8_base_LeakyInstanceTrait_v8_base_Mutex__> = Rc::new(RefCell::new(Self { } )) ;
 let this : Ptr<v8_base_LazyStaticInstance_v8_base_Mutex__v8_base_DefaultConstructTrait_v8_base_Mutex___v8_base_ThreadSafeInitOnceTrait__v8_base_LeakyInstanceTrait_v8_base_Mutex__> = __this.as_pointer() ;
 Rc::try_unwrap(__this).ok().unwrap().into_inner() } }
impl ByteRepr for  v8_base_LazyStaticInstance_v8_base_Mutex__v8_base_DefaultConstructTrait_v8_base_Mutex___v8_base_ThreadSafeInitOnceTrait__v8_base_LeakyInstanceTrait_v8_base_Mutex__ { fn byte_size() -> usize { 1 } fn to_bytes(&self, buf: &mut [u8]) { } fn from_bytes(buf: &[u8]) -> Self { Self { } } }
#[derive(Default)]
pub struct v8_base_LazyStaticInstance_v8_base_RecursiveMutex__v8_base_DefaultConstructTrait_v8_base_RecursiveMutex___v8_base_ThreadSafeInitOnceTrait__v8_base_LeakyInstanceTrait_v8_base_RecursiveMutex__
{}
impl Clone for v8_base_LazyStaticInstance_v8_base_RecursiveMutex__v8_base_DefaultConstructTrait_v8_base_RecursiveMutex___v8_base_ThreadSafeInitOnceTrait__v8_base_LeakyInstanceTrait_v8_base_RecursiveMutex__ { fn clone(&self) -> Self { let __this : Value<v8_base_LazyStaticInstance_v8_base_RecursiveMutex__v8_base_DefaultConstructTrait_v8_base_RecursiveMutex___v8_base_ThreadSafeInitOnceTrait__v8_base_LeakyInstanceTrait_v8_base_RecursiveMutex__> = Rc::new(RefCell::new(Self { } )) ;
 let this : Ptr<v8_base_LazyStaticInstance_v8_base_RecursiveMutex__v8_base_DefaultConstructTrait_v8_base_RecursiveMutex___v8_base_ThreadSafeInitOnceTrait__v8_base_LeakyInstanceTrait_v8_base_RecursiveMutex__> = __this.as_pointer() ;
 Rc::try_unwrap(__this).ok().unwrap().into_inner() } }
impl ByteRepr for  v8_base_LazyStaticInstance_v8_base_RecursiveMutex__v8_base_DefaultConstructTrait_v8_base_RecursiveMutex___v8_base_ThreadSafeInitOnceTrait__v8_base_LeakyInstanceTrait_v8_base_RecursiveMutex__ { fn byte_size() -> usize { 1 } fn to_bytes(&self, buf: &mut [u8]) { } fn from_bytes(buf: &[u8]) -> Self { Self { } } }
#[derive()]
pub struct v8_base_Mutex {
    native_handle_: Value<absl_Mutex>,
}
impl v8_base_Mutex {}
impl Default for v8_base_Mutex {
    fn default() -> Self {
        { v8_base_Mutex::v8_base_Mutex() }
    }
}
impl ByteRepr for v8_base_Mutex {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.native_handle_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            native_handle_: Rc::new(RefCell::new(<absl_Mutex>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_RecursiveMutex {
    thread_id_: Value<std_atomic_int_>,
    level_: Value<i32>,
    mutex_: Value<v8_base_Mutex>,
}
impl ByteRepr for v8_base_RecursiveMutex {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.thread_id_.borrow()).to_bytes(&mut buf[0..4]);
        (*self.level_.borrow()).to_bytes(&mut buf[4..8]);
        (*self.mutex_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            thread_id_: Rc::new(RefCell::new(<std_atomic_int_>::from_bytes(&buf[0..4]))),
            level_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            mutex_: Rc::new(RefCell::new(<v8_base_Mutex>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_LockGuard_v8_base_Mutex_ {
    mutex_: Value<Ptr<v8_base_Mutex>>,
}
impl v8_base_LockGuard_v8_base_Mutex_ {
    pub fn v8_base_LockGuard_v8_base_Mutex_1(mutex: Ptr<v8_base_Mutex>) -> Self {
        let mutex: Value<Ptr<v8_base_Mutex>> = Rc::new(RefCell::new(mutex));
        let __this: Value<v8_base_LockGuard_v8_base_Mutex_> = Rc::new(RefCell::new(Self {
            mutex_: Rc::new(RefCell::new((*mutex.borrow()).clone())),
        }));
        let this: Ptr<v8_base_LockGuard_v8_base_Mutex_> = __this.as_pointer();
        (&(0));
        ({ v8_base_MutexImpl::Lock(&(*(*this.upgrade().deref()).mutex_.borrow())) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_LockGuard_v8_base_Mutex_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.mutex_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            mutex_: Rc::new(RefCell::new(<Ptr<v8_base_Mutex>>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_MutexGuardIf {
    mutex_: Value<std_optional_v8_base_LockGuard_v8_base_Mutex__>,
}
impl v8_base_MutexGuardIf {
    pub fn v8_base_MutexGuardIf(mutex: Ptr<v8_base_Mutex>, enable_mutex: bool) -> Self {
        let mutex: Value<Ptr<v8_base_Mutex>> = Rc::new(RefCell::new(mutex));
        let enable_mutex: Value<bool> = Rc::new(RefCell::new(enable_mutex));
        let __this : Value<v8_base_MutexGuardIf> = Rc::new(RefCell::new(Self { mutex_ : Rc::new(RefCell::new(std_optional_v8_base_LockGuard_v8_base_Mutex__ :: std_optional_v8_base_LockGuard_v8_base_Mutex__1 ( ) )) , } )) ;
        let this: Ptr<v8_base_MutexGuardIf> = __this.as_pointer();
        if (*enable_mutex.borrow()) {
            ({ (*(*this.upgrade().deref()).mutex_.borrow()).emplace(mutex.as_pointer()) });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_MutexGuardIf {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.mutex_.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            mutex_: Rc::new(RefCell::new(
                <std_optional_v8_base_LockGuard_v8_base_Mutex__>::from_bytes(&buf[0..16]),
            )),
        }
    }
}
#[derive(Default)]
pub struct v8_internal_IndexGenerator {  lock_ : Value<v8_base_Mutex > ,  first_use_ : Value<bool > ,  ranges_to_split_ : Value<std_queue_std_pair_unsigned_long__unsigned_long___std_deque_std_pair_unsigned_long__unsigned_long___std_allocator_std_pair_unsigned_long__unsigned_long____ > , }
impl v8_internal_IndexGenerator {
    pub fn v8_internal_IndexGenerator(size: usize) -> Self {
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let __this : Value<v8_internal_IndexGenerator> = Rc::new(RefCell::new(Self { lock_ : Rc::new(RefCell::new(v8_base_Mutex :: v8_base_Mutex ( ) )) , first_use_ : Rc::new(RefCell::new(( (*size.borrow()) > 0_usize ) )) , ranges_to_split_ : Rc::new(RefCell::new(std_queue_std_pair_unsigned_long__unsigned_long___std_deque_std_pair_unsigned_long__unsigned_long___std_allocator_std_pair_unsigned_long__unsigned_long____ :: std_queue_std_pair_unsigned_long__unsigned_long___std_deque_std_pair_unsigned_long__unsigned_long___std_allocator_std_pair_unsigned_long__unsigned_long____1 ( ) )) , } )) ;
        let this: Ptr<v8_internal_IndexGenerator> = __this.as_pointer();
        if ((*size.borrow()) == 0_usize) {
            return;
        }
        let guard: Value<v8_base_LockGuard_v8_base_Mutex_> = Rc::new(RefCell::new(
            v8_base_LockGuard_v8_base_Mutex_::v8_base_LockGuard_v8_base_Mutex_1({
                ((*this.upgrade().deref()).lock_.as_pointer())
            }),
        ));
        let _dtor_guard = ScopedDestructor::new(&guard, |__p| __p.destructor());
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(0));
            (*(*this.upgrade().deref()).ranges_to_split_.borrow())
                .emplace(___args.as_pointer(), size.as_pointer())
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_internal_IndexGenerator {
    fn byte_size() -> usize {
        64
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.lock_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.first_use_.borrow()).to_bytes(&mut buf[8..9]);
        (*self.ranges_to_split_.borrow()).to_bytes(&mut buf[16..64]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self { lock_: Rc::new(RefCell::new(<v8_base_Mutex >::from_bytes(&buf[0..8]))), first_use_: Rc::new(RefCell::new(<bool >::from_bytes(&buf[8..9]))), ranges_to_split_: Rc::new(RefCell::new(<std_queue_std_pair_unsigned_long__unsigned_long___std_deque_std_pair_unsigned_long__unsigned_long___std_allocator_std_pair_unsigned_long__unsigned_long____ >::from_bytes(&buf[16..64]))), }
    }
}
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct absl_SynchWaitParams;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct absl_time_internal_cctz_time_zone_Impl;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct absl_SynchLocksHeld;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct absl_base_internal_SpinLock;
pub trait absl_CondVarImpl {
    fn Wait(&self, mu: Ptr<absl_Mutex>);
    fn WaitWithTimeout(&self, mu: Ptr<absl_Mutex>, timeout: absl_Duration) -> bool;
    fn WaitWithDeadline(&self, mu: Ptr<absl_Mutex>, deadline: absl_Time) -> bool;
}
impl absl_CondVarImpl for Ptr<absl_CondVar> {
    fn Wait(&self, mu: Ptr<absl_Mutex>) {
        let mu: Value<Ptr<absl_Mutex>> = Rc::new(RefCell::new(mu));
        ({
            absl_CondVarImpl::WaitCommon(
                self,
                (*mu.borrow()).clone(),
                ({ absl_synchronization_internal_KernelTimeout::Never() }),
            )
        });
    }
    fn WaitWithTimeout(&self, mu: Ptr<absl_Mutex>, timeout: absl_Duration) -> bool {
        let mu: Value<Ptr<absl_Mutex>> = Rc::new(RefCell::new(mu));
        let timeout: Value<absl_Duration> = Rc::new(RefCell::new(timeout));
        return ({
            absl_CondVarImpl :: WaitCommon ( self , ((*mu.borrow()) ).clone() , absl_synchronization_internal_KernelTimeout :: absl_synchronization_internal_KernelTimeout3 ( {  ((*timeout.borrow()) ).clone()   } , )  , )
        });
    }
    fn WaitWithDeadline(&self, mu: Ptr<absl_Mutex>, deadline: absl_Time) -> bool {
        let mu: Value<Ptr<absl_Mutex>> = Rc::new(RefCell::new(mu));
        let deadline: Value<absl_Time> = Rc::new(RefCell::new(deadline));
        return ({
            absl_CondVarImpl :: WaitCommon ( self , ((*mu.borrow()) ).clone() , absl_synchronization_internal_KernelTimeout :: absl_synchronization_internal_KernelTimeout2 ( {  ((*deadline.borrow()) ).clone()   } , )  , )
        });
    }
}
pub trait absl_Duration_HiRepImpl {
    fn Get(&self) -> i64;
    fn operator_assign_i64(&self, value: i64) -> Ptr<absl_Duration_HiRep>;
}
impl absl_Duration_HiRepImpl for Ptr<absl_Duration_HiRep> {
    fn Get(&self) -> i64 {
        let unsigned_value: Value<u64> = Rc::new(RefCell::new(
            ((((*(*(*self).upgrade().deref()).hi_.borrow()) as u64) << 32)
                | ((*(*(*self).upgrade().deref()).lo_.borrow()) as u64)),
        ));
        return ((*unsigned_value.borrow()) as i64);
    }
    fn operator_assign_i64(&self, value: i64) -> Ptr<absl_Duration_HiRep> {
        let value: Value<i64> = Rc::new(RefCell::new(value));
        let unsigned_value: Value<u64> = Rc::new(RefCell::new(((*value.borrow()) as u64)));
        (*(*(*self).upgrade().deref()).hi_.borrow_mut()) =
            (((*unsigned_value.borrow()) >> 32) as u32);
        (*(*(*self).upgrade().deref()).lo_.borrow_mut()) = ((*unsigned_value.borrow()) as u32);
        return (*self).clone();
    }
}
pub trait absl_MutexImpl {
    fn Lock(&self);
    fn Unlock(&self);
    fn TryLock(&self) -> bool;
    fn ReaderLock(&self);
    fn ReaderUnlock(&self);
    fn ReaderTryLock(&self) -> bool;
    fn WriterLock(&self);
    fn WriterUnlock(&self);
    fn WriterTryLock(&self) -> bool;
    fn Await(&self, cond: Ptr<absl_Condition>);
    fn LockWhen(&self, cond: Ptr<absl_Condition>);
    fn ReaderLockWhen(&self, cond: Ptr<absl_Condition>);
    fn WriterLockWhen(&self, cond: Ptr<absl_Condition>);
    fn AwaitWithTimeout(&self, cond: Ptr<absl_Condition>, timeout: absl_Duration) -> bool;
    fn AwaitWithDeadline(&self, cond: Ptr<absl_Condition>, deadline: absl_Time) -> bool;
    fn LockWhenWithTimeout(&self, cond: Ptr<absl_Condition>, timeout: absl_Duration) -> bool;
    fn ReaderLockWhenWithTimeout(&self, cond: Ptr<absl_Condition>, timeout: absl_Duration) -> bool;
    fn WriterLockWhenWithTimeout(&self, cond: Ptr<absl_Condition>, timeout: absl_Duration) -> bool;
    fn LockWhenWithDeadline(&self, cond: Ptr<absl_Condition>, deadline: absl_Time) -> bool;
    fn ReaderLockWhenWithDeadline(&self, cond: Ptr<absl_Condition>, deadline: absl_Time) -> bool;
    fn WriterLockWhenWithDeadline(&self, cond: Ptr<absl_Condition>, deadline: absl_Time) -> bool;
    fn Dtor(&self);
}
impl absl_MutexImpl for Ptr<absl_Mutex> {
    fn Lock(&self) {
        ({ absl_MutexImpl::lock(self) });
    }
    fn Unlock(&self) {
        ({ absl_MutexImpl::unlock(self) });
    }
    fn TryLock(&self) -> bool {
        return ({ absl_MutexImpl::try_lock(self) });
    }
    fn ReaderLock(&self) {
        ({ absl_MutexImpl::lock_shared(self) });
    }
    fn ReaderUnlock(&self) {
        ({ absl_MutexImpl::unlock_shared(self) });
    }
    fn ReaderTryLock(&self) -> bool {
        return ({ absl_MutexImpl::try_lock_shared(self) });
    }
    fn WriterLock(&self) {
        ({ absl_MutexImpl::lock(self) });
    }
    fn WriterUnlock(&self) {
        ({ absl_MutexImpl::unlock(self) });
    }
    fn WriterTryLock(&self) -> bool {
        return ({ absl_MutexImpl::try_lock(self) });
    }
    fn Await(&self, cond: Ptr<absl_Condition>) {
        ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            let _t: absl_synchronization_internal_KernelTimeout =
                ({ absl_synchronization_internal_KernelTimeout::Never() });
            absl_MutexImpl::AwaitCommon(self, _cond, _t)
        });
    }
    fn LockWhen(&self, cond: Ptr<absl_Condition>) {
        ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            let _t: absl_synchronization_internal_KernelTimeout =
                ({ absl_synchronization_internal_KernelTimeout::Never() });
            absl_MutexImpl::LockWhenCommon(self, _cond, _t, true)
        });
    }
    fn ReaderLockWhen(&self, cond: Ptr<absl_Condition>) {
        ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            let _t: absl_synchronization_internal_KernelTimeout =
                ({ absl_synchronization_internal_KernelTimeout::Never() });
            absl_MutexImpl::LockWhenCommon(self, _cond, _t, false)
        });
    }
    fn WriterLockWhen(&self, cond: Ptr<absl_Condition>) {
        ({ absl_MutexImpl::LockWhen(self, (cond).clone()) });
    }
    fn AwaitWithTimeout(&self, cond: Ptr<absl_Condition>, timeout: absl_Duration) -> bool {
        let timeout: Value<absl_Duration> = Rc::new(RefCell::new(timeout));
        return ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            let _t: absl_synchronization_internal_KernelTimeout   = absl_synchronization_internal_KernelTimeout :: absl_synchronization_internal_KernelTimeout3 ( {  ((*timeout.borrow()) ).clone()   } , )  ;
            absl_MutexImpl::AwaitCommon(self, _cond, _t)
        });
    }
    fn AwaitWithDeadline(&self, cond: Ptr<absl_Condition>, deadline: absl_Time) -> bool {
        let deadline: Value<absl_Time> = Rc::new(RefCell::new(deadline));
        return ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            let _t: absl_synchronization_internal_KernelTimeout   = absl_synchronization_internal_KernelTimeout :: absl_synchronization_internal_KernelTimeout2 ( {  ((*deadline.borrow()) ).clone()   } , )  ;
            absl_MutexImpl::AwaitCommon(self, _cond, _t)
        });
    }
    fn LockWhenWithTimeout(&self, cond: Ptr<absl_Condition>, timeout: absl_Duration) -> bool {
        let timeout: Value<absl_Duration> = Rc::new(RefCell::new(timeout));
        return ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            let _t: absl_synchronization_internal_KernelTimeout   = absl_synchronization_internal_KernelTimeout :: absl_synchronization_internal_KernelTimeout3 ( {  ((*timeout.borrow()) ).clone()   } , )  ;
            absl_MutexImpl::LockWhenCommon(self, _cond, _t, true)
        });
    }
    fn ReaderLockWhenWithTimeout(&self, cond: Ptr<absl_Condition>, timeout: absl_Duration) -> bool {
        let timeout: Value<absl_Duration> = Rc::new(RefCell::new(timeout));
        return ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            let _t: absl_synchronization_internal_KernelTimeout   = absl_synchronization_internal_KernelTimeout :: absl_synchronization_internal_KernelTimeout3 ( {  ((*timeout.borrow()) ).clone()   } , )  ;
            absl_MutexImpl::LockWhenCommon(self, _cond, _t, false)
        });
    }
    fn WriterLockWhenWithTimeout(&self, cond: Ptr<absl_Condition>, timeout: absl_Duration) -> bool {
        let timeout: Value<absl_Duration> = Rc::new(RefCell::new(timeout));
        return ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            let _timeout: absl_Duration = (*timeout.borrow()).clone();
            absl_MutexImpl::LockWhenWithTimeout(self, _cond, _timeout)
        });
    }
    fn LockWhenWithDeadline(&self, cond: Ptr<absl_Condition>, deadline: absl_Time) -> bool {
        let deadline: Value<absl_Time> = Rc::new(RefCell::new(deadline));
        return ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            let _t: absl_synchronization_internal_KernelTimeout   = absl_synchronization_internal_KernelTimeout :: absl_synchronization_internal_KernelTimeout2 ( {  ((*deadline.borrow()) ).clone()   } , )  ;
            absl_MutexImpl::LockWhenCommon(self, _cond, _t, true)
        });
    }
    fn ReaderLockWhenWithDeadline(&self, cond: Ptr<absl_Condition>, deadline: absl_Time) -> bool {
        let deadline: Value<absl_Time> = Rc::new(RefCell::new(deadline));
        return ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            let _t: absl_synchronization_internal_KernelTimeout   = absl_synchronization_internal_KernelTimeout :: absl_synchronization_internal_KernelTimeout2 ( {  ((*deadline.borrow()) ).clone()   } , )  ;
            absl_MutexImpl::LockWhenCommon(self, _cond, _t, false)
        });
    }
    fn WriterLockWhenWithDeadline(&self, cond: Ptr<absl_Condition>, deadline: absl_Time) -> bool {
        let deadline: Value<absl_Time> = Rc::new(RefCell::new(deadline));
        return ({
            let _cond: Ptr<absl_Condition> = (cond).clone();
            let _deadline: absl_Time = (*deadline.borrow()).clone();
            absl_MutexImpl::LockWhenWithDeadline(self, _cond, _deadline)
        });
    }
    fn Dtor(&self) {}
}
pub trait absl_MutexLockImpl {
    fn destructor(&self);
}
impl absl_MutexLockImpl for Ptr<absl_MutexLock> {
    fn destructor(&self) {
        ({ absl_MutexImpl::unlock(&(*(*self).upgrade().deref()).mu_) });
    }
}
pub trait absl_MutexLockMaybeImpl {
    fn destructor(&self);
}
impl absl_MutexLockMaybeImpl for Ptr<absl_MutexLockMaybe> {
    fn destructor(&self) {
        if !((*(*(*self).upgrade().deref()).mu_.borrow()).is_null()) {
            ({ absl_MutexImpl::unlock(&(*(*(*self).upgrade().deref()).mu_.borrow())) });
        }
    }
}
pub trait absl_ReaderMutexLockImpl {
    fn destructor(&self);
}
impl absl_ReaderMutexLockImpl for Ptr<absl_ReaderMutexLock> {
    fn destructor(&self) {
        ({ absl_MutexImpl::unlock_shared(&(*(*self).upgrade().deref()).mu_) });
    }
}
pub trait absl_ReleasableMutexLockImpl {
    fn destructor(&self);
}
impl absl_ReleasableMutexLockImpl for Ptr<absl_ReleasableMutexLock> {
    fn destructor(&self) {
        if !((*(*(*self).upgrade().deref()).mu_.borrow()).is_null()) {
            ({ absl_MutexImpl::unlock(&(*(*(*self).upgrade().deref()).mu_.borrow())) });
        }
    }
}
pub trait absl_TimeImpl {
    fn operator_add_assign(&self, d: absl_Duration) -> Ptr<absl_Time>;
    fn operator_sub_assign(&self, d: absl_Duration) -> Ptr<absl_Time>;
}
impl absl_TimeImpl for Ptr<absl_Time> {
    fn operator_add_assign(&self, d: absl_Duration) -> Ptr<absl_Time> {
        let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
        ({
            absl_DurationImpl::operator_add_assign(
                &(*(*self).upgrade().deref()).rep_.as_pointer(),
                (*d.borrow()).clone(),
            )
        });
        return (*self).clone();
    }
    fn operator_sub_assign(&self, d: absl_Duration) -> Ptr<absl_Time> {
        let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
        ({
            absl_DurationImpl::operator_sub_assign(
                &(*(*self).upgrade().deref()).rep_.as_pointer(),
                (*d.borrow()).clone(),
            )
        });
        return (*self).clone();
    }
}
pub trait absl_TimeZoneImpl {
    fn operator_time_internal__cctz__time_zone(&self) -> absl_time_internal_cctz_time_zone;
    fn name(&self) -> Vec<u8>;
}
impl absl_TimeZoneImpl for Ptr<absl_TimeZone> {
    fn operator_time_internal__cctz__time_zone(&self) -> absl_time_internal_cctz_time_zone {
        return (*(*(*self).upgrade().deref()).cz_.borrow()).clone();
    }
    fn name(&self) -> Vec<u8> {
        return ({
            absl_time_internal_cctz_time_zoneImpl::name(
                &(*(*self).upgrade().deref()).cz_.as_pointer(),
            )
        });
    }
}
pub trait absl_WriterMutexLockImpl {
    fn destructor(&self);
}
impl absl_WriterMutexLockImpl for Ptr<absl_WriterMutexLock> {
    fn destructor(&self) {
        ({ absl_MutexImpl::unlock(&(*(*self).upgrade().deref()).mu_) });
    }
}
pub trait absl_base_internal_PerThreadSynchImpl {
    fn thread_identity(&self) -> Ptr<absl_base_internal_ThreadIdentity>;
}
impl absl_base_internal_PerThreadSynchImpl for Ptr<absl_base_internal_PerThreadSynch> {
    fn thread_identity(&self) -> Ptr<absl_base_internal_ThreadIdentity> {
        return (*self).reinterpret_cast::<absl_base_internal_ThreadIdentity>();
    }
}
pub trait absl_base_internal_ThreadIdentity_SchedulerStateImpl {
    fn association_lock(&self) -> Ptr<absl_base_internal_SpinLock>;
}
impl absl_base_internal_ThreadIdentity_SchedulerStateImpl
    for Ptr<absl_base_internal_ThreadIdentity_SchedulerState>
{
    fn association_lock(&self) -> Ptr<absl_base_internal_SpinLock> {
        return ((*(*self).upgrade().deref())
            .association_lock_word
            .as_pointer())
        .reinterpret_cast::<absl_base_internal_SpinLock>();
    }
}
pub trait absl_synchronization_internal_KernelTimeoutImpl {
    fn has_timeout(&self) -> bool;
    fn is_absolute_timeout(&self) -> bool;
    fn is_relative_timeout(&self) -> bool;
    fn RawAbsNanos(&self) -> i64;
}
impl absl_synchronization_internal_KernelTimeoutImpl
    for Ptr<absl_synchronization_internal_KernelTimeout>
{
    fn has_timeout(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).rep_.borrow()) != 18446744073709551615);
    }
    fn is_absolute_timeout(&self) -> bool {
        return (((*(*(*self).upgrade().deref()).rep_.borrow()) & 1_u64) == 0_u64);
    }
    fn is_relative_timeout(&self) -> bool {
        return (((*(*(*self).upgrade().deref()).rep_.borrow()) & 1_u64) == 1_u64);
    }
    fn RawAbsNanos(&self) -> i64 {
        return (((*(*(*self).upgrade().deref()).rep_.borrow()) >> 1) as i64);
    }
}
pub trait absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_Impl {
    fn year(&self) -> i64;
    fn month(&self) -> i32;
    fn day(&self) -> i32;
}
impl absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_Impl
    for Ptr<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_>
{
    fn year(&self) -> i64 {
        return (*(*(*(*self).upgrade().deref()).f_.borrow()).y.borrow());
    }
    fn month(&self) -> i32 {
        return ((*(*(*(*self).upgrade().deref()).f_.borrow()).m.borrow()) as i32);
    }
    fn day(&self) -> i32 {
        return ((*(*(*(*self).upgrade().deref()).f_.borrow()).d.borrow()) as i32);
    }
}
pub trait v8_base_LockGuard_v8_base_Mutex_Impl {
    fn destructor(&self);
}
impl v8_base_LockGuard_v8_base_Mutex_Impl for Ptr<v8_base_LockGuard_v8_base_Mutex_> {
    fn destructor(&self) {
        if !(*(*(*self).upgrade().deref()).mutex_.borrow()).is_null() {
            ({ v8_base_MutexImpl::Unlock(&(*(*(*self).upgrade().deref()).mutex_.borrow())) });
        }
    }
}
pub trait v8_base_MutexImpl {
    fn AssertHeld(&self);
    fn AssertHeldAndUnmark(&self);
    fn AssertUnheldAndMark(&self);
}
impl v8_base_MutexImpl for Ptr<v8_base_Mutex> {
    fn AssertHeld(&self) {
        (&(0));
    }
    fn AssertHeldAndUnmark(&self) {}
    fn AssertUnheldAndMark(&self) {}
}
pub trait v8_base_RecursiveMutexImpl {
    fn AssertHeld(&self);
}
impl v8_base_RecursiveMutexImpl for Ptr<v8_base_RecursiveMutex> {
    fn AssertHeld(&self) {
        (&(0));
    }
}
pub trait v8_internal_IndexGeneratorImpl {
    fn GetNext(&self) -> std_optional_unsigned_long_;
}
impl v8_internal_IndexGeneratorImpl for Ptr<v8_internal_IndexGenerator> {
    fn GetNext(&self) -> std_optional_unsigned_long_ {
        let guard: Value<v8_base_LockGuard_v8_base_Mutex_> = Rc::new(RefCell::new(
            v8_base_LockGuard_v8_base_Mutex_::v8_base_LockGuard_v8_base_Mutex_1({
                ((*(*self).upgrade().deref()).lock_.as_pointer())
            }),
        ));
        let _dtor_guard = ScopedDestructor::new(&guard, |__p| __p.destructor());
        if (*(*(*self).upgrade().deref()).first_use_.borrow()) {
            (*(*(*self).upgrade().deref()).first_use_.borrow_mut()) = false;
            return 0;
        }
        if ({ (*(*(*self).upgrade().deref()).ranges_to_split_.borrow()).empty() }) {
            return std_optional_unsigned_long_::std_optional_unsigned_long_2({
                (*nullopt_225.with(Value::clone).borrow()).clone()
            });
        }
        let range: Value<(Value<u64>, Value<u64>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(
                (*({ (*(*(*self).upgrade().deref()).ranges_to_split_.borrow()).front() })
                    .upgrade()
                    .deref())
                .0
                .borrow()
                .clone(),
            )),
            Rc::new(RefCell::new(
                (*({ (*(*(*self).upgrade().deref()).ranges_to_split_.borrow()).front() })
                    .upgrade()
                    .deref())
                .1
                .borrow()
                .clone(),
            )),
        )));
        ({ (*(*(*self).upgrade().deref()).ranges_to_split_.borrow()).pop() });
        let size: Value<usize> = Rc::new(RefCell::new(
            ((*(*range.borrow()).1.borrow()).wrapping_sub((*(*range.borrow()).0.borrow()))
                as usize),
        ));
        let mid: Value<usize> = Rc::new(RefCell::new(
            ((*(*range.borrow()).0.borrow())
                .wrapping_add(((*size.borrow()).wrapping_div(2_usize) as u64))
                as usize),
        ));
        if (((*mid.borrow()) as u64).wrapping_sub((*(*range.borrow()).0.borrow())) > 1_u64) {
            ({
                (*(*(*self).upgrade().deref()).ranges_to_split_.borrow())
                    .emplace((*range.borrow()).0.as_pointer(), mid.as_pointer())
            });
        }
        if ((*(*range.borrow()).1.borrow()).wrapping_sub(((*mid.borrow()) as u64)) > 1_u64) {
            ({
                (*(*(*self).upgrade().deref()).ranges_to_split_.borrow())
                    .emplace(mid.as_pointer(), (*range.borrow()).1.as_pointer())
            });
        }
        return (*mid.borrow_mut()).clone();
    }
}
