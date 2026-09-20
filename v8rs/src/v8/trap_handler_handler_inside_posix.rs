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
    pub static kReturnAddressStackSlotCount_1: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kPageSizeBits_2: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
    pub static kRegularPageSize_3: Value<i32> = Rc::new(RefCell::new(262144));
);
thread_local!(
    pub static kMinimumOSPageSize_4: Value<i32> = Rc::new(RefCell::new(16384));
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
    if ({ (*g_can_enable_trap_handler_7.with(Value::clone).borrow()).load_const(Some(0)) }) {
        ({ (*g_can_enable_trap_handler_7.with(Value::clone).borrow()).store_bool(false, Some(0)) });
    }
    return (*g_is_trap_handler_enabled_6.with(Value::clone).borrow());
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
impl v8_internal_trap_handler_MetadataLock {}
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
impl v8_internal_trap_handler_SandboxRecordsLock {}
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
impl v8_internal_trap_handler_CoveredMemoryRecordsLock {}
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
pub fn IsKernelGeneratedSignal_19(info: Ptr<__siginfo>) -> bool {
    let info: Value<Ptr<__siginfo>> = Rc::new(RefCell::new(info));
    return ((((((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) > 0)
        && ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) != 65537))
        && ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) != 65538))
        && ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) != 65539))
        && ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) != 65540))
        && ((*(*(*info.borrow()).upgrade().deref()).si_code.borrow()) != 65541);
}
#[derive()]
pub struct v8_internal_trap_handler_UnmaskOobSignalScope {
    old_mask_: Value<u32>,
}
impl v8_internal_trap_handler_UnmaskOobSignalScope {
    pub fn v8_internal_trap_handler_UnmaskOobSignalScope() -> Self {
        let __this: Value<v8_internal_trap_handler_UnmaskOobSignalScope> =
            Rc::new(RefCell::new(Self {
                old_mask_: <Value<u32>>::default(),
            }));
        let this: Ptr<v8_internal_trap_handler_UnmaskOobSignalScope> = __this.as_pointer();
        let sigs: Value<u32> = <Value<u32>>::default();
        {
            (sigs.as_pointer()).write(0_u32);
            0
        };
        {
            {
                let _ptr = (sigs.as_pointer());
                _ptr.write(_ptr.read() | ({ __sigbits_20(10) }))
            };
            0
        };
        ({
            pthread_sigmask_21(
                2,
                (sigs.as_pointer()),
                ((*this.upgrade().deref()).old_mask_.as_pointer()),
            )
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_internal_trap_handler_UnmaskOobSignalScope {
    fn default() -> Self {
        {
            v8_internal_trap_handler_UnmaskOobSignalScope :: v8_internal_trap_handler_UnmaskOobSignalScope ( )
        }
    }
}
impl ByteRepr for v8_internal_trap_handler_UnmaskOobSignalScope {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.old_mask_.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            old_mask_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn TryHandleSignal_22(signum: i32, info: Ptr<__siginfo>, context: AnyPtr) -> bool {
    let signum: Value<i32> = Rc::new(RefCell::new(signum));
    let info: Value<Ptr<__siginfo>> = Rc::new(RefCell::new(info));
    let context: Value<AnyPtr> = Rc::new(RefCell::new(context));
    if ({ v8_internal_trap_handler_TrapHandlerGuard::IsActiveOnCurrentThread() }) {
        return false;
    }
    let active_guard: Value<v8_internal_trap_handler_TrapHandlerGuard> = Rc::new(RefCell::new(
        v8_internal_trap_handler_TrapHandlerGuard::v8_internal_trap_handler_TrapHandlerGuard(),
    ));
    let _dtor_active_guard = ScopedDestructor::new(&active_guard, |__p| __p.destructor());
    if ((*signum.borrow()) != 10) {
        return false;
    }
    if !({ IsKernelGeneratedSignal_19((*info.borrow()).clone()) }) {
        return false;
    }
    let access_addr: Value<u64> = Rc::new(RefCell::new(
        (*(*(*info.borrow()).upgrade().deref()).si_addr.borrow()).to_int(),
    ));
    if !({ IsAccessedMemoryCovered_23((*access_addr.borrow())) }) {
        return false;
    }
    {
        let unmask_oob_signal : Value<v8_internal_trap_handler_UnmaskOobSignalScope > = Rc::new(RefCell::new(v8_internal_trap_handler_UnmaskOobSignalScope :: v8_internal_trap_handler_UnmaskOobSignalScope ( ) )) ;
        let _dtor_unmask_oob_signal =
            ScopedDestructor::new(&unmask_oob_signal, |__p| __p.destructor());
        let uc: Value<Ptr<__darwin_ucontext>> = Rc::new(RefCell::new(
            (*context.borrow()).reinterpret_cast::<__darwin_ucontext>(),
        ));
        let context_ip: Value<Ptr<u64>> = Rc::new(RefCell::new(
            ((*(*(*(*(*uc.borrow()).upgrade().deref()).uc_mcontext.borrow())
                .upgrade()
                .deref())
            .__ss
            .borrow())
            .__pc
            .as_pointer()),
        ));
        let fault_addr: Value<u64> = Rc::new(RefCell::new(((*context_ip.borrow()).read())));
        if !({ IsFaultAddressCovered_24((*fault_addr.borrow())) }) {
            return false;
        }
        &(0);
        let __rhs =
            ({ (*gLandingPad_18.with(Value::clone).borrow()).operator_unsigned_long_const() });
        (*context_ip.borrow()).write(__rhs);
        let fault_address_reg: Value<Ptr<u64>> = Rc::new(RefCell::new(
            (((*(*(*(*(*uc.borrow()).upgrade().deref()).uc_mcontext.borrow())
                .upgrade()
                .deref())
            .__ss
            .borrow())
            .__x
            .as_pointer() as Ptr<u64>)
                .offset(16)),
        ));
        let __rhs = (*fault_addr.borrow());
        (*fault_address_reg.borrow()).write(__rhs);
    }
    return true;
}
pub fn HandleSignal_25(signum: i32, info: Ptr<__siginfo>, context: AnyPtr) {
    let signum: Value<i32> = Rc::new(RefCell::new(signum));
    let info: Value<Ptr<__siginfo>> = Rc::new(RefCell::new(info));
    let context: Value<AnyPtr> = Rc::new(RefCell::new(context));
    if !({
        TryHandleSignal_22(
            (*signum.borrow()),
            (*info.borrow()).clone(),
            (*context.borrow()).clone(),
        )
    }) {
        ({ RemoveTrapHandler_26() });
        if !({ IsKernelGeneratedSignal_19((*info.borrow()).clone()) }) {
            ({ raise_27((*signum.borrow())) });
        }
    }
}
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_internal_trap_handler_TrapHandlerGuard;
pub trait v8_internal_trap_handler_UnmaskOobSignalScopeImpl {
    fn destructor(&self);
}
impl v8_internal_trap_handler_UnmaskOobSignalScopeImpl
    for Ptr<v8_internal_trap_handler_UnmaskOobSignalScope>
{
    fn destructor(&self) {
        ({
            pthread_sigmask_21(
                3,
                ((*(*self).upgrade().deref()).old_mask_.as_pointer()),
                Ptr::<u32>::null(),
            )
        });
    }
}
