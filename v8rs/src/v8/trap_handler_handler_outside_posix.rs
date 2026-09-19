use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static kOobSignal_0: Value<i32> = Rc::new(RefCell::new(10));
);
thread_local!(
    pub static kReturnAddressStackSlotCount_1: Value<i32> =
        Rc::new(RefCell::new(if false { 1 } else { 0 }));
);
thread_local!(
    pub static kPageSizeBits_2: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
    pub static kRegularPageSize_3: Value<i32> = Rc::new(RefCell::new(
        (1 << (*kPageSizeBits_2.with(Value::clone).borrow())),
    ));
);
thread_local!(
    pub static kMinimumOSPageSize_4: Value<i32> = Rc::new(RefCell::new((16 * 1024)));
);
#[derive(Default)]
pub struct v8_internal_trap_handler_TrappingInstructionData {
    pub instr_offset: Value<u32>,
}
impl Clone for v8_internal_trap_handler_TrappingInstructionData {
    fn clone(&self) -> Self {
        let __this: Value<v8_internal_trap_handler_TrappingInstructionData> =
            Rc::new(RefCell::new(Self {
                instr_offset: Rc::new(RefCell::new((*self.instr_offset.borrow()))),
            }));
        let this: Ptr<v8_internal_trap_handler_TrappingInstructionData> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_internal_trap_handler_TrappingInstructionData {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.instr_offset.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            instr_offset: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
        }
    }
}
thread_local!(
    pub static kInvalidIndex_5: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!();
thread_local!();
pub fn IsTrapHandlerEnabled_8() -> bool {
    &(0);
    if ({
        ((*g_can_enable_trap_handler_7.with(Value::clone).borrow())
            as std___atomic_base_bool__false_)
            .load_const(Some((*memory_order_relaxed_9.with(Value::clone).borrow())))
    }) {
        ({
            ((*g_can_enable_trap_handler_7.with(Value::clone).borrow())
                as std___atomic_base_bool__false_)
                .store_bool(
                    false,
                    Some((*memory_order_relaxed_9.with(Value::clone).borrow())),
                )
        });
    }
    return (*g_is_trap_handler_enabled_6.with(Value::clone).borrow());
}
thread_local!();
#[derive()]
pub struct v8_internal_trap_handler_TrapHandlerGuard {}
impl v8_internal_trap_handler_TrapHandlerGuard {
    pub fn v8_internal_trap_handler_TrapHandlerGuard() -> Self {
        let __this: Value<v8_internal_trap_handler_TrapHandlerGuard> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_internal_trap_handler_TrapHandlerGuard> = __this.as_pointer();
        if !(!(*is_active__10.with(Value::clone).borrow())) {
            {
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    ({
                        (|| {
                            let mut __do_while = true;
                            'loop_: while __do_while || (false) {
                                __do_while = false;
                                b"brk #0";
                                b"hlt #0";
                            }
                        })()
                    });
                }
                let __result = unreachable!();
                __result
            };
        };
        (*is_active__10.with(Value::clone).borrow_mut()) = true;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn IsActiveOnCurrentThread() -> bool {
        return (*is_active__10.with(Value::clone).borrow());
    }
}
impl Default for v8_internal_trap_handler_TrapHandlerGuard {
    fn default() -> Self {
        { v8_internal_trap_handler_TrapHandlerGuard::v8_internal_trap_handler_TrapHandlerGuard() }
    }
}
impl ByteRepr for v8_internal_trap_handler_TrapHandlerGuard {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive()]
pub struct v8_internal_trap_handler_CodeProtectionInfo {
    pub base: Value<u64>,
    pub size: Value<usize>,
    pub num_trapping_instructions: Value<usize>,
    pub instructions: Value<Box<[v8_internal_trap_handler_TrappingInstructionData]>>,
}
impl Clone for v8_internal_trap_handler_CodeProtectionInfo {
    fn clone(&self) -> Self {
        let __this: Value<v8_internal_trap_handler_CodeProtectionInfo> =
            Rc::new(RefCell::new(Self {
                base: Rc::new(RefCell::new((*self.base.borrow()))),
                size: Rc::new(RefCell::new((*self.size.borrow()))),
                num_trapping_instructions: Rc::new(RefCell::new(
                    (*self.num_trapping_instructions.borrow()),
                )),
                instructions: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 1, _>(
                    |__i: usize| ((*self.instructions.borrow())[(__i) as usize]).clone(),
                )))),
            }));
        let this: Ptr<v8_internal_trap_handler_CodeProtectionInfo> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_internal_trap_handler_CodeProtectionInfo {
    fn default() -> Self {
        v8_internal_trap_handler_CodeProtectionInfo {
            base: <Value<u64>>::default(),
            size: Rc::new(RefCell::new(0_usize)),
            num_trapping_instructions: Rc::new(RefCell::new(0_usize)),
            instructions: Rc::new(RefCell::new(
                (0..1)
                    .map(|_| <v8_internal_trap_handler_TrappingInstructionData>::default())
                    .collect::<Box<[v8_internal_trap_handler_TrappingInstructionData]>>(),
            )),
        }
    }
}
impl ByteRepr for v8_internal_trap_handler_CodeProtectionInfo {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base.borrow()).to_bytes(&mut buf[0..8]);
        (*self.size.borrow()).to_bytes(&mut buf[8..16]);
        (*self.num_trapping_instructions.borrow()).to_bytes(&mut buf[16..24]);
        (*self.instructions.borrow()).to_bytes(&mut buf[24..28]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
            size: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
            num_trapping_instructions: Rc::new(RefCell::new(<usize>::from_bytes(&buf[16..24]))),
            instructions: Rc::new(RefCell::new(<Box<
                [v8_internal_trap_handler_TrappingInstructionData],
            >>::from_bytes(&buf[24..28]))),
        }
    }
}
#[derive(Default)]
pub struct v8_internal_trap_handler_CodeProtectionInfoListEntry {
    pub code_info: Value<Ptr<v8_internal_trap_handler_CodeProtectionInfo>>,
    pub next_free: Value<usize>,
}
impl Clone for v8_internal_trap_handler_CodeProtectionInfoListEntry {
    fn clone(&self) -> Self {
        let __this: Value<v8_internal_trap_handler_CodeProtectionInfoListEntry> =
            Rc::new(RefCell::new(Self {
                code_info: Rc::new(RefCell::new((*self.code_info.borrow()).clone())),
                next_free: Rc::new(RefCell::new((*self.next_free.borrow()))),
            }));
        let this: Ptr<v8_internal_trap_handler_CodeProtectionInfoListEntry> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_internal_trap_handler_CodeProtectionInfoListEntry {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.code_info.borrow()).to_bytes(&mut buf[0..8]);
        (*self.next_free.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            code_info: Rc::new(RefCell::new(<Ptr<
                v8_internal_trap_handler_CodeProtectionInfo,
            >>::from_bytes(&buf[0..8]))),
            next_free: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
        }
    }
}
thread_local!();
thread_local!();
#[derive(Default)]
pub struct v8_internal_trap_handler_SandboxRecord {
    pub base: Value<u64>,
    pub size: Value<usize>,
    pub next: Value<Ptr<v8_internal_trap_handler_SandboxRecord>>,
}
impl Clone for v8_internal_trap_handler_SandboxRecord {
    fn clone(&self) -> Self {
        let __this: Value<v8_internal_trap_handler_SandboxRecord> = Rc::new(RefCell::new(Self {
            base: Rc::new(RefCell::new((*self.base.borrow()))),
            size: Rc::new(RefCell::new((*self.size.borrow()))),
            next: Rc::new(RefCell::new((*self.next.borrow()).clone())),
        }));
        let this: Ptr<v8_internal_trap_handler_SandboxRecord> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_internal_trap_handler_SandboxRecord {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base.borrow()).to_bytes(&mut buf[0..8]);
        (*self.size.borrow()).to_bytes(&mut buf[8..16]);
        (*self.next.borrow()).to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
            size: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
            next: Rc::new(RefCell::new(
                <Ptr<v8_internal_trap_handler_SandboxRecord>>::from_bytes(&buf[16..24]),
            )),
        }
    }
}
#[derive(Default)]
pub struct v8_internal_trap_handler_CoveredMemoryRecord {
    pub base: Value<u64>,
    pub size: Value<usize>,
    pub next: Value<Ptr<v8_internal_trap_handler_CoveredMemoryRecord>>,
}
impl Clone for v8_internal_trap_handler_CoveredMemoryRecord {
    fn clone(&self) -> Self {
        let __this: Value<v8_internal_trap_handler_CoveredMemoryRecord> =
            Rc::new(RefCell::new(Self {
                base: Rc::new(RefCell::new((*self.base.borrow()))),
                size: Rc::new(RefCell::new((*self.size.borrow()))),
                next: Rc::new(RefCell::new((*self.next.borrow()).clone())),
            }));
        let this: Ptr<v8_internal_trap_handler_CoveredMemoryRecord> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_internal_trap_handler_CoveredMemoryRecord {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base.borrow()).to_bytes(&mut buf[0..8]);
        (*self.size.borrow()).to_bytes(&mut buf[8..16]);
        (*self.next.borrow()).to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
            size: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
            next: Rc::new(RefCell::new(<Ptr<
                v8_internal_trap_handler_CoveredMemoryRecord,
            >>::from_bytes(&buf[16..24]))),
        }
    }
}
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!(
    pub static g_old_handler_17: Value<sigaction> = Rc::new(RefCell::new(<sigaction>::default()));
);
thread_local!(
    pub static g_is_default_signal_handler_registered_18: Value<bool> = <Value<bool>>::default();
);
pub fn RegisterDefaultTrapHandler_19() -> bool {
    if !(!(*g_is_default_signal_handler_registered_18
        .with(Value::clone)
        .borrow()))
    {
        {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                ({
                    (|| {
                        let mut __do_while = true;
                        'loop_: while __do_while || (false) {
                            __do_while = false;
                            b"brk #0";
                            b"hlt #0";
                        }
                    })()
                });
            }
            let __result = unreachable!();
            __result
        };
    };
    let action: Value<sigaction> = Rc::new(RefCell::new(<sigaction>::default()));
    (*(*action.borrow()).__sigaction_u.borrow_mut())
        .__sa_sigaction()
        .write(FnPtr::<fn(i32, Ptr<__siginfo>, AnyPtr)>::new(
            HandleSignal_20,
        ));
    (*(*action.borrow()).sa_flags.borrow_mut()) = (64 | 1);
    {
        ((*action.borrow()).sa_mask.as_pointer()).write(0_u32);
        0
    };
    if (libc::sigaction(
        (*kOobSignal_0.with(Value::clone).borrow()),
        (action.as_pointer()),
        (g_old_handler_17.with(Value::clone).as_pointer()),
    ) != 0)
    {
        return false;
    }
    (*g_is_default_signal_handler_registered_18
        .with(Value::clone)
        .borrow_mut()) = true;
    return true;
}
pub fn RemoveTrapHandler_21() {
    if (*g_is_default_signal_handler_registered_18
        .with(Value::clone)
        .borrow())
    {
        if (libc::sigaction(
            (*kOobSignal_0.with(Value::clone).borrow()),
            (g_old_handler_17.with(Value::clone).as_pointer()),
            Ptr::<sigaction>::null(),
        ) == 0)
        {
            (*g_is_default_signal_handler_registered_18
                .with(Value::clone)
                .borrow_mut()) = false;
        }
    }
}
pub trait v8_internal_trap_handler_TrapHandlerGuardImpl {
    fn destructor(&self);
}
impl v8_internal_trap_handler_TrapHandlerGuardImpl
    for Ptr<v8_internal_trap_handler_TrapHandlerGuard>
{
    fn destructor(&self) {
        (*is_active__10.with(Value::clone).borrow_mut()) = false;
    }
}
