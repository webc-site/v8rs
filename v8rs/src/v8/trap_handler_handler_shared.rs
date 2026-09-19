use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static kReturnAddressStackSlotCount_0: Value<i32> =
        Rc::new(RefCell::new(if false { 1 } else { 0 }));
);
thread_local!(
    pub static kPageSizeBits_1: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
    pub static kRegularPageSize_2: Value<i32> = Rc::new(RefCell::new(
        (1 << (*kPageSizeBits_1.with(Value::clone).borrow())),
    ));
);
thread_local!(
    pub static kMinimumOSPageSize_3: Value<i32> = Rc::new(RefCell::new((16 * 1024)));
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
    pub static kInvalidIndex_4: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!();
thread_local!();
pub fn IsTrapHandlerEnabled_7() -> bool {
    &(0);
    if ({
        ((*g_can_enable_trap_handler_6.with(Value::clone).borrow())
            as std___atomic_base_bool__false_)
            .load_const(Some((*memory_order_relaxed_8.with(Value::clone).borrow())))
    }) {
        ({
            ((*g_can_enable_trap_handler_6.with(Value::clone).borrow())
                as std___atomic_base_bool__false_)
                .store_bool(
                    false,
                    Some((*memory_order_relaxed_8.with(Value::clone).borrow())),
                )
        });
    }
    return (*g_is_trap_handler_enabled_5.with(Value::clone).borrow());
}
thread_local!();
#[derive()]
pub struct v8_internal_trap_handler_TrapHandlerGuard {}
impl v8_internal_trap_handler_TrapHandlerGuard {
    pub fn v8_internal_trap_handler_TrapHandlerGuard() -> Self {
        let __this: Value<v8_internal_trap_handler_TrapHandlerGuard> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_internal_trap_handler_TrapHandlerGuard> = __this.as_pointer();
        if !(!(*is_active__9.with(Value::clone).borrow())) {
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
        (*is_active__9.with(Value::clone).borrow_mut()) = true;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn IsActiveOnCurrentThread() -> bool {
        return (*is_active__9.with(Value::clone).borrow());
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
thread_local!();
#[derive()]
pub struct v8_internal_trap_handler_MetadataLock {}
impl v8_internal_trap_handler_MetadataLock {
    pub fn v8_internal_trap_handler_MetadataLock() -> Self {
        let __this: Value<v8_internal_trap_handler_MetadataLock> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_internal_trap_handler_MetadataLock> = __this.as_pointer();
        if !({ v8_internal_trap_handler_TrapHandlerGuard::IsActiveOnCurrentThread() }) {
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
        'loop_: while ({
            (*spinlock__10.with(Value::clone).borrow())
                .test_and_set(Some((*memory_order_acquire_11.with(Value::clone).borrow())))
        }) {}
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_internal_trap_handler_MetadataLock {
    fn default() -> Self {
        { v8_internal_trap_handler_MetadataLock::v8_internal_trap_handler_MetadataLock() }
    }
}
impl ByteRepr for v8_internal_trap_handler_MetadataLock {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
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
#[derive()]
pub struct v8_internal_trap_handler_SandboxRecordsLock {}
impl v8_internal_trap_handler_SandboxRecordsLock {
    pub fn v8_internal_trap_handler_SandboxRecordsLock() -> Self {
        let __this: Value<v8_internal_trap_handler_SandboxRecordsLock> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_internal_trap_handler_SandboxRecordsLock> = __this.as_pointer();
        if !({ v8_internal_trap_handler_TrapHandlerGuard::IsActiveOnCurrentThread() }) {
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
        'loop_: while ({
            (*spinlock__14.with(Value::clone).borrow())
                .test_and_set(Some((*memory_order_acquire_11.with(Value::clone).borrow())))
        }) {}
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_internal_trap_handler_SandboxRecordsLock {
    fn default() -> Self {
        {
            v8_internal_trap_handler_SandboxRecordsLock::v8_internal_trap_handler_SandboxRecordsLock(
            )
        }
    }
}
impl ByteRepr for v8_internal_trap_handler_SandboxRecordsLock {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!();
#[derive()]
pub struct v8_internal_trap_handler_CoveredMemoryRecordsLock {}
impl v8_internal_trap_handler_CoveredMemoryRecordsLock {
    pub fn v8_internal_trap_handler_CoveredMemoryRecordsLock() -> Self {
        let __this: Value<v8_internal_trap_handler_CoveredMemoryRecordsLock> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_internal_trap_handler_CoveredMemoryRecordsLock> = __this.as_pointer();
        if !({ v8_internal_trap_handler_TrapHandlerGuard::IsActiveOnCurrentThread() }) {
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
        'loop_: while ({
            (*spinlock__15.with(Value::clone).borrow())
                .test_and_set(Some((*memory_order_acquire_11.with(Value::clone).borrow())))
        }) {}
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_internal_trap_handler_CoveredMemoryRecordsLock {
    fn default() -> Self {
        {
            v8_internal_trap_handler_CoveredMemoryRecordsLock :: v8_internal_trap_handler_CoveredMemoryRecordsLock ( )
        }
    }
}
impl ByteRepr for v8_internal_trap_handler_CoveredMemoryRecordsLock {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!(
    static is_active__9: Value<bool> = Rc::new(RefCell::new((0 != 0)));
);
thread_local!(
    pub static gNumCodeObjects_12: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static gCodeObjects_13: Value<Ptr<v8_internal_trap_handler_CodeProtectionInfoListEntry>> =
        Rc::new(RefCell::new(Ptr::<
            v8_internal_trap_handler_CodeProtectionInfoListEntry,
        >::null()));
);
thread_local!(
    pub static gSandboxRecordsHead_16: Value<Ptr<v8_internal_trap_handler_SandboxRecord>> = Rc::new(
        RefCell::new(Ptr::<v8_internal_trap_handler_SandboxRecord>::null()),
    );
);
thread_local!(
    pub static gCoveredMemoryRecordsHead_17: Value<
        Ptr<v8_internal_trap_handler_CoveredMemoryRecord>,
    > = Rc::new(RefCell::new(Ptr::<
        v8_internal_trap_handler_CoveredMemoryRecord,
    >::null()));
);
thread_local!(
    pub static gRecoveredTrapCount_18: Value<std_atomic_unsigned_long_> = Rc::new(RefCell::new(
        std_atomic_unsigned_long_::std_atomic_unsigned_long_1({ 0_u64 }),
    ));
);
thread_local!(
    pub static gLandingPad_19: Value<std_atomic_unsigned_long_> = Rc::new(RefCell::new(
        std_atomic_unsigned_long_::std_atomic_unsigned_long_1({ 0_u64 }),
    ));
);
thread_local!(
    static spinlock__10: Value<std_atomic_flag> =
        Rc::new(RefCell::new(std_atomic_flag::std_atomic_flag1()));
);
thread_local!(
    static spinlock__14: Value<std_atomic_flag> =
        Rc::new(RefCell::new(std_atomic_flag::std_atomic_flag1()));
);
thread_local!(
    static spinlock__15: Value<std_atomic_flag> =
        Rc::new(RefCell::new(std_atomic_flag::std_atomic_flag1()));
);
pub trait v8_internal_trap_handler_CoveredMemoryRecordsLockImpl {
    fn destructor(&self);
}
impl v8_internal_trap_handler_CoveredMemoryRecordsLockImpl
    for Ptr<v8_internal_trap_handler_CoveredMemoryRecordsLock>
{
    fn destructor(&self) {
        if !({ v8_internal_trap_handler_TrapHandlerGuard::IsActiveOnCurrentThread() }) {
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
        ({
            (*spinlock__15.with(Value::clone).borrow())
                .clear(Some((*memory_order_release_20.with(Value::clone).borrow())))
        });
    }
}
pub trait v8_internal_trap_handler_MetadataLockImpl {
    fn destructor(&self);
}
impl v8_internal_trap_handler_MetadataLockImpl for Ptr<v8_internal_trap_handler_MetadataLock> {
    fn destructor(&self) {
        if !({ v8_internal_trap_handler_TrapHandlerGuard::IsActiveOnCurrentThread() }) {
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
        ({
            (*spinlock__10.with(Value::clone).borrow())
                .clear(Some((*memory_order_release_20.with(Value::clone).borrow())))
        });
    }
}
pub trait v8_internal_trap_handler_SandboxRecordsLockImpl {
    fn destructor(&self);
}
impl v8_internal_trap_handler_SandboxRecordsLockImpl
    for Ptr<v8_internal_trap_handler_SandboxRecordsLock>
{
    fn destructor(&self) {
        if !({ v8_internal_trap_handler_TrapHandlerGuard::IsActiveOnCurrentThread() }) {
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
        ({
            (*spinlock__14.with(Value::clone).borrow())
                .clear(Some((*memory_order_release_20.with(Value::clone).borrow())))
        });
    }
}
pub trait v8_internal_trap_handler_TrapHandlerGuardImpl {
    fn destructor(&self);
}
impl v8_internal_trap_handler_TrapHandlerGuardImpl
    for Ptr<v8_internal_trap_handler_TrapHandlerGuard>
{
    fn destructor(&self) {
        (*is_active__9.with(Value::clone).borrow_mut()) = false;
    }
}
