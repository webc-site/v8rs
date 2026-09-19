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
    pub static gNextCodeObject_16: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static kEnableSlowChecks_17: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static kInitialCodeObjectSize_18: Value<usize> = Rc::new(RefCell::new(1024_usize));
);
thread_local!(
    pub static kCodeObjectGrowthFactor_19: Value<usize> = Rc::new(RefCell::new(2_usize));
);
pub fn HandlerDataSize_20(num_trapping_instructions: usize) -> usize {
    let num_trapping_instructions: Value<usize> = Rc::new(RefCell::new(num_trapping_instructions));
    return ((24_usize as u64)
        .wrapping_add(((*num_trapping_instructions.borrow()) as u64).wrapping_mul((4usize as u64)))
        as usize);
}
pub fn VerifyCodeRangeIsDisjoint_21(code_info: Ptr<v8_internal_trap_handler_CodeProtectionInfo>) {
    let code_info: Value<Ptr<v8_internal_trap_handler_CodeProtectionInfo>> =
        Rc::new(RefCell::new(code_info));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*gNumCodeObjects_10.with(Value::clone).borrow())) {
        &(0);
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn ValidateCodeObjects_22() {
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as usize) < (*gNumCodeObjects_10.with(Value::clone).borrow())) {
        let data: Value<Ptr<v8_internal_trap_handler_CodeProtectionInfo>> = Rc::new(RefCell::new(
            (*(*(*gCodeObjects_11.with(Value::clone).borrow())
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .code_info
            .borrow())
            .clone(),
        ));
        if (*data.borrow()).is_null() {
            (*i.borrow_mut()).prefix_inc();
            continue 'loop_;
        }
        let j: Value<u32> = Rc::new(RefCell::new(0_u32));
        'loop_: while {
            let _lhs = ((*j.borrow()) as usize);
            _lhs < (*(*(*data.borrow()).upgrade().deref())
                .num_trapping_instructions
                .borrow())
        } {
            &(0);
            &(0);
            (*j.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn CreateHandlerData_23(
    base: u64,
    size: usize,
    num_trapping_instructions: usize,
    trapping_instructions: Ptr<v8_internal_trap_handler_TrappingInstructionData>,
) -> Ptr<v8_internal_trap_handler_CodeProtectionInfo> {
    let base: Value<u64> = Rc::new(RefCell::new(base));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    let num_trapping_instructions: Value<usize> = Rc::new(RefCell::new(num_trapping_instructions));
    let trapping_instructions: Value<Ptr<v8_internal_trap_handler_TrappingInstructionData>> =
        Rc::new(RefCell::new(trapping_instructions));
    let alloc_size: Value<usize> = Rc::new(RefCell::new(
        ({ HandlerDataSize_20((*num_trapping_instructions.borrow())) }),
    ));
    let data: Value<Ptr<v8_internal_trap_handler_CodeProtectionInfo>> = Rc::new(RefCell::new(
        malloc_refcount((*alloc_size.borrow()))
            .reinterpret_cast::<v8_internal_trap_handler_CodeProtectionInfo>(),
    ));
    if (*data.borrow()).is_null() {
        return Ptr::<v8_internal_trap_handler_CodeProtectionInfo>::null();
    }
    (*(*(*data.borrow()).upgrade().deref()).base.borrow_mut()) = (*base.borrow());
    (*(*(*data.borrow()).upgrade().deref()).size.borrow_mut()) = (*size.borrow());
    (*(*(*data.borrow()).upgrade().deref())
        .num_trapping_instructions
        .borrow_mut()) = (*num_trapping_instructions.borrow());
    if ((*num_trapping_instructions.borrow()) > 0_usize) {
        {
            (((*(*data.borrow()).upgrade().deref())
                .instructions
                .as_pointer()
                as Ptr<v8_internal_trap_handler_TrappingInstructionData>)
                as Ptr<v8_internal_trap_handler_TrappingInstructionData>)
                .to_any()
                .memcpy(
                    &((*trapping_instructions.borrow()).clone()
                        as Ptr<v8_internal_trap_handler_TrappingInstructionData>)
                        .to_any(),
                    (((*num_trapping_instructions.borrow()) as u64).wrapping_mul((4usize as u64))
                        as usize) as usize,
                );
            (((*(*data.borrow()).upgrade().deref())
                .instructions
                .as_pointer()
                as Ptr<v8_internal_trap_handler_TrappingInstructionData>)
                as Ptr<v8_internal_trap_handler_TrappingInstructionData>)
                .to_any()
        };
    }
    return (*data.borrow()).clone();
}
pub fn RegisterHandlerData_24(
    base: u64,
    size: usize,
    num_trapping_instructions: usize,
    trapping_instructions: Ptr<v8_internal_trap_handler_TrappingInstructionData>,
) -> i32 {
    let base: Value<u64> = Rc::new(RefCell::new(base));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    let num_trapping_instructions: Value<usize> = Rc::new(RefCell::new(num_trapping_instructions));
    let trapping_instructions: Value<Ptr<v8_internal_trap_handler_TrappingInstructionData>> =
        Rc::new(RefCell::new(trapping_instructions));
    let data: Value<Ptr<v8_internal_trap_handler_CodeProtectionInfo>> = Rc::new(RefCell::new(
        ({
            CreateHandlerData_23(
                (*base.borrow()),
                (*size.borrow()),
                (*num_trapping_instructions.borrow()),
                (*trapping_instructions.borrow()).clone(),
            )
        }),
    ));
    if (*data.borrow()).is_null() {
        std::process::abort();
    }
    let active_guard: Value<v8_internal_trap_handler_TrapHandlerGuard> = Rc::new(RefCell::new(
        v8_internal_trap_handler_TrapHandlerGuard::v8_internal_trap_handler_TrapHandlerGuard(),
    ));
    let _dtor_active_guard = ScopedDestructor::new(&active_guard, |__p| __p.destructor());
    let lock: Value<v8_internal_trap_handler_MetadataLock> = Rc::new(RefCell::new(
        v8_internal_trap_handler_MetadataLock::v8_internal_trap_handler_MetadataLock(),
    ));
    if (*kEnableSlowChecks_17.with(Value::clone).borrow()) {
        ({ VerifyCodeRangeIsDisjoint_21((*data.borrow()).clone()) });
    }
    let i: Value<usize> = Rc::new(RefCell::new(
        (*gNextCodeObject_16.with(Value::clone).borrow()),
    ));
    let int_max: Value<usize> = Rc::new(RefCell::new((<i32>::MAX as usize)));
    if ((*i.borrow()) == (*gNumCodeObjects_10.with(Value::clone).borrow())) {
        let new_size: Value<usize> = Rc::new(RefCell::new(
            if ((*gNumCodeObjects_10.with(Value::clone).borrow()) > 0_usize) {
                (*gNumCodeObjects_10.with(Value::clone).borrow())
                    .wrapping_mul((*kCodeObjectGrowthFactor_19.with(Value::clone).borrow()))
            } else {
                (*kInitialCodeObjectSize_18.with(Value::clone).borrow())
            },
        ));
        if ((*new_size.borrow()) > (*int_max.borrow())) {
            (*new_size.borrow_mut()) = (*int_max.borrow());
        }
        if ((*new_size.borrow()) == (*gNumCodeObjects_10.with(Value::clone).borrow())) {
            free_refcount(
                ((*data.borrow()).clone() as Ptr<v8_internal_trap_handler_CodeProtectionInfo>)
                    .to_any(),
            );
            return (*kInvalidIndex_4.with(Value::clone).borrow());
        }
        let __rhs = realloc_refcount(
            ((*gCodeObjects_11.with(Value::clone).borrow()).clone()
                as Ptr<v8_internal_trap_handler_CodeProtectionInfoListEntry>)
                .to_any(),
            ((16usize as u64).wrapping_mul(((*new_size.borrow()) as u64)) as usize),
        )
        .reinterpret_cast::<v8_internal_trap_handler_CodeProtectionInfoListEntry>();
        (*gCodeObjects_11.with(Value::clone).borrow_mut()) = __rhs;
        if (*gCodeObjects_11.with(Value::clone).borrow()).is_null() {
            std::process::abort();
        }
        {
            ((*gCodeObjects_11.with(Value::clone).borrow())
                .offset((*gNumCodeObjects_10.with(Value::clone).borrow()) as isize)
                as Ptr<v8_internal_trap_handler_CodeProtectionInfoListEntry>)
                .to_any()
                .memset(
                    (0) as u8,
                    ((16usize as u64).wrapping_mul(
                        (((*new_size.borrow())
                            .wrapping_sub((*gNumCodeObjects_10.with(Value::clone).borrow())))
                            as u64),
                    ) as usize) as usize,
                );
            ((*gCodeObjects_11.with(Value::clone).borrow())
                .offset((*gNumCodeObjects_10.with(Value::clone).borrow()) as isize)
                as Ptr<v8_internal_trap_handler_CodeProtectionInfoListEntry>)
                .to_any()
        };
        let j: Value<usize> = Rc::new(RefCell::new(
            (*gNumCodeObjects_10.with(Value::clone).borrow()),
        ));
        'loop_: while ((*j.borrow()) < (*new_size.borrow())) {
            let __rhs = (*j.borrow()).wrapping_add(1_usize);
            (*(*(*gCodeObjects_11.with(Value::clone).borrow())
                .offset((*j.borrow()) as isize)
                .upgrade()
                .deref())
            .next_free
            .borrow_mut()) = __rhs;
            (*j.borrow_mut()).prefix_inc();
        }
        (*gNumCodeObjects_10.with(Value::clone).borrow_mut()) = (*new_size.borrow());
    }
    &(0);
    (*gNextCodeObject_16.with(Value::clone).borrow_mut()) =
        (*(*(*gCodeObjects_11.with(Value::clone).borrow())
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .next_free
        .borrow());
    if ((*i.borrow()) <= (*int_max.borrow())) {
        (*(*(*gCodeObjects_11.with(Value::clone).borrow())
            .offset((*i.borrow()) as isize)
            .upgrade()
            .deref())
        .code_info
        .borrow_mut()) = (*data.borrow()).clone();
        if (*kEnableSlowChecks_17.with(Value::clone).borrow()) {
            ({ ValidateCodeObjects_22() });
        }
        return ((*i.borrow()) as i32);
    } else {
        free_refcount(
            ((*data.borrow()).clone() as Ptr<v8_internal_trap_handler_CodeProtectionInfo>).to_any(),
        );
        return (*kInvalidIndex_4.with(Value::clone).borrow());
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ReleaseHandlerData_25(index: i32) {
    let index: Value<i32> = Rc::new(RefCell::new(index));
    if ((*index.borrow()) == (*kInvalidIndex_4.with(Value::clone).borrow())) {
        return;
    }
    &(0);
    let data: Value<Ptr<v8_internal_trap_handler_CodeProtectionInfo>> =
        Rc::new(RefCell::new(Ptr::<
            v8_internal_trap_handler_CodeProtectionInfo,
        >::null()));
    {
        let active_guard: Value<v8_internal_trap_handler_TrapHandlerGuard> = Rc::new(RefCell::new(
            v8_internal_trap_handler_TrapHandlerGuard::v8_internal_trap_handler_TrapHandlerGuard(),
        ));
        let _dtor_active_guard = ScopedDestructor::new(&active_guard, |__p| __p.destructor());
        let lock: Value<v8_internal_trap_handler_MetadataLock> = Rc::new(RefCell::new(
            v8_internal_trap_handler_MetadataLock::v8_internal_trap_handler_MetadataLock(),
        ));
        (*data.borrow_mut()) = (*(*(*gCodeObjects_11.with(Value::clone).borrow())
            .offset((*index.borrow()) as isize)
            .upgrade()
            .deref())
        .code_info
        .borrow())
        .clone();
        (*(*(*gCodeObjects_11.with(Value::clone).borrow())
            .offset((*index.borrow()) as isize)
            .upgrade()
            .deref())
        .code_info
        .borrow_mut()) = Ptr::<v8_internal_trap_handler_CodeProtectionInfo>::null();
        (*(*(*gCodeObjects_11.with(Value::clone).borrow())
            .offset((*index.borrow()) as isize)
            .upgrade()
            .deref())
        .next_free
        .borrow_mut()) = (*gNextCodeObject_16.with(Value::clone).borrow());
        (*gNextCodeObject_16.with(Value::clone).borrow_mut()) = ((*index.borrow()) as usize);
        if (*kEnableSlowChecks_17.with(Value::clone).borrow()) {
            ({ ValidateCodeObjects_22() });
        }
    }
    &(0);
    free_refcount(
        ((*data.borrow()).clone() as Ptr<v8_internal_trap_handler_CodeProtectionInfo>).to_any(),
    );
}
pub fn RegisterV8Sandbox_26(base: u64, size: usize) -> bool {
    let base: Value<u64> = Rc::new(RefCell::new(base));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    let active_guard: Value<v8_internal_trap_handler_TrapHandlerGuard> = Rc::new(RefCell::new(
        v8_internal_trap_handler_TrapHandlerGuard::v8_internal_trap_handler_TrapHandlerGuard(),
    ));
    let _dtor_active_guard = ScopedDestructor::new(&active_guard, |__p| __p.destructor());
    let lock: Value<v8_internal_trap_handler_SandboxRecordsLock> = Rc::new(RefCell::new(
        v8_internal_trap_handler_SandboxRecordsLock::v8_internal_trap_handler_SandboxRecordsLock(),
    ));
    let new_record: Value<Ptr<v8_internal_trap_handler_SandboxRecord>> = Rc::new(RefCell::new(
        malloc_refcount(24usize)
            .reinterpret_cast::<v8_internal_trap_handler_SandboxRecord>(),
    ));
    if (*new_record.borrow()).is_null() {
        return false;
    }
    (*(*(*new_record.borrow()).upgrade().deref())
        .base
        .borrow_mut()) = (*base.borrow());
    (*(*(*new_record.borrow()).upgrade().deref())
        .size
        .borrow_mut()) = (*size.borrow());
    (*(*(*new_record.borrow()).upgrade().deref())
        .next
        .borrow_mut()) = (*gSandboxRecordsHead_12.with(Value::clone).borrow()).clone();
    (*gSandboxRecordsHead_12.with(Value::clone).borrow_mut()) = (*new_record.borrow()).clone();
    return true;
}
pub fn UnregisterV8Sandbox_27(base: u64, size: usize) {
    let base: Value<u64> = Rc::new(RefCell::new(base));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    let active_guard: Value<v8_internal_trap_handler_TrapHandlerGuard> = Rc::new(RefCell::new(
        v8_internal_trap_handler_TrapHandlerGuard::v8_internal_trap_handler_TrapHandlerGuard(),
    ));
    let _dtor_active_guard = ScopedDestructor::new(&active_guard, |__p| __p.destructor());
    let lock: Value<v8_internal_trap_handler_SandboxRecordsLock> = Rc::new(RefCell::new(
        v8_internal_trap_handler_SandboxRecordsLock::v8_internal_trap_handler_SandboxRecordsLock(),
    ));
    let current: Value<Ptr<v8_internal_trap_handler_SandboxRecord>> = Rc::new(RefCell::new(
        (*gSandboxRecordsHead_12.with(Value::clone).borrow()).clone(),
    ));
    let previous: Value<Ptr<v8_internal_trap_handler_SandboxRecord>> = Rc::new(RefCell::new(
        Ptr::<v8_internal_trap_handler_SandboxRecord>::null(),
    ));
    'loop_: while !((*current.borrow()).is_null()) {
        if {
            let _lhs = (*(*(*current.borrow()).upgrade().deref()).base.borrow());
            _lhs == (*base.borrow())
        } {
            break;
        }
        (*previous.borrow_mut()) = (*current.borrow()).clone();
        let __rhs = (*(*(*current.borrow()).upgrade().deref()).next.borrow()).clone();
        (*current.borrow_mut()) = __rhs;
    }
    if !(!((*current.borrow()).is_null())) {
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
    if !({
        let _lhs = (*(*(*current.borrow()).upgrade().deref()).size.borrow());
        _lhs == (*size.borrow())
    }) {
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
    if !(*previous.borrow()).is_null() {
        let __rhs = (*(*(*current.borrow()).upgrade().deref()).next.borrow()).clone();
        (*(*(*previous.borrow()).upgrade().deref()).next.borrow_mut()) = __rhs;
    } else {
        (*gSandboxRecordsHead_12.with(Value::clone).borrow_mut()) =
            (*(*(*current.borrow()).upgrade().deref()).next.borrow()).clone();
    }
    free_refcount(
        ((*current.borrow()).clone() as Ptr<v8_internal_trap_handler_SandboxRecord>).to_any(),
    );
}
pub fn RegisterCoveredMemory_28(base: u64, reserved_size: usize) -> bool {
    let base: Value<u64> = Rc::new(RefCell::new(base));
    let reserved_size: Value<usize> = Rc::new(RefCell::new(reserved_size));
    let active_guard: Value<v8_internal_trap_handler_TrapHandlerGuard> = Rc::new(RefCell::new(
        v8_internal_trap_handler_TrapHandlerGuard::v8_internal_trap_handler_TrapHandlerGuard(),
    ));
    let _dtor_active_guard = ScopedDestructor::new(&active_guard, |__p| __p.destructor());
    if !((*base.borrow()).wrapping_add(((*reserved_size.borrow()) as u64)) > (*base.borrow())) {
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
    {
        let sandbox_lock : Value<v8_internal_trap_handler_SandboxRecordsLock > = Rc::new(RefCell::new(v8_internal_trap_handler_SandboxRecordsLock :: v8_internal_trap_handler_SandboxRecordsLock ( ) )) ;
        if !((*gSandboxRecordsHead_12.with(Value::clone).borrow()).is_null()) {
            let within_sandbox: Value<bool> = Rc::new(RefCell::new(false));
            let s: Value<Ptr<v8_internal_trap_handler_SandboxRecord>> = Rc::new(RefCell::new(
                (*gSandboxRecordsHead_12.with(Value::clone).borrow()).clone(),
            ));
            'loop_: while (!((*s.borrow()).is_null())) && (!(*within_sandbox.borrow())) {
                (*within_sandbox.borrow_mut()) = ({
                    let _lhs = (*base.borrow());
                    _lhs >= (*(*(*s.borrow()).upgrade().deref()).base.borrow())
                }) && ({
                    let _lhs = (*base.borrow()).wrapping_add(((*reserved_size.borrow()) as u64));
                    _lhs <= (*(*(*s.borrow()).upgrade().deref()).base.borrow())
                        .wrapping_add(((*(*(*s.borrow()).upgrade().deref()).size.borrow()) as u64))
                });
                let __rhs = (*(*(*s.borrow()).upgrade().deref()).next.borrow()).clone();
                (*s.borrow_mut()) = __rhs;
            }
            if !(*within_sandbox.borrow()) {
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
        }
    }
    let lock : Value<v8_internal_trap_handler_CoveredMemoryRecordsLock > = Rc::new(RefCell::new(v8_internal_trap_handler_CoveredMemoryRecordsLock :: v8_internal_trap_handler_CoveredMemoryRecordsLock ( ) )) ;
    let current: Value<Ptr<v8_internal_trap_handler_CoveredMemoryRecord>> = Rc::new(RefCell::new(
        (*gCoveredMemoryRecordsHead_13.with(Value::clone).borrow()).clone(),
    ));
    'loop_: while !((*current.borrow()).is_null()) {
        let disjoint: Value<bool> = Rc::new(RefCell::new(
            ({
                let _lhs = (*base.borrow());
                _lhs >= (*(*(*current.borrow()).upgrade().deref()).base.borrow()).wrapping_add(
                    ((*(*(*current.borrow()).upgrade().deref()).size.borrow()) as u64),
                )
            }) || ({
                let _lhs = (*base.borrow()).wrapping_add(((*reserved_size.borrow()) as u64));
                _lhs <= (*(*(*current.borrow()).upgrade().deref()).base.borrow())
            }),
        ));
        if !(*disjoint.borrow()) {
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
        let __rhs = (*(*(*current.borrow()).upgrade().deref()).next.borrow()).clone();
        (*current.borrow_mut()) = __rhs;
    }
    let new_record: Value<Ptr<v8_internal_trap_handler_CoveredMemoryRecord>> =
        Rc::new(RefCell::new(
            malloc_refcount(24usize)
                .reinterpret_cast::<v8_internal_trap_handler_CoveredMemoryRecord>(),
        ));
    if (*new_record.borrow()).is_null() {
        return false;
    }
    (*(*(*new_record.borrow()).upgrade().deref())
        .base
        .borrow_mut()) = (*base.borrow());
    (*(*(*new_record.borrow()).upgrade().deref())
        .size
        .borrow_mut()) = (*reserved_size.borrow());
    (*(*(*new_record.borrow()).upgrade().deref())
        .next
        .borrow_mut()) = (*gCoveredMemoryRecordsHead_13.with(Value::clone).borrow()).clone();
    (*gCoveredMemoryRecordsHead_13.with(Value::clone).borrow_mut()) =
        (*new_record.borrow()).clone();
    return true;
}
pub fn UnregisterCoveredMemory_29(base: u64, reserved_size: usize) {
    let base: Value<u64> = Rc::new(RefCell::new(base));
    let reserved_size: Value<usize> = Rc::new(RefCell::new(reserved_size));
    let active_guard: Value<v8_internal_trap_handler_TrapHandlerGuard> = Rc::new(RefCell::new(
        v8_internal_trap_handler_TrapHandlerGuard::v8_internal_trap_handler_TrapHandlerGuard(),
    ));
    let _dtor_active_guard = ScopedDestructor::new(&active_guard, |__p| __p.destructor());
    let lock : Value<v8_internal_trap_handler_CoveredMemoryRecordsLock > = Rc::new(RefCell::new(v8_internal_trap_handler_CoveredMemoryRecordsLock :: v8_internal_trap_handler_CoveredMemoryRecordsLock ( ) )) ;
    let current: Value<Ptr<v8_internal_trap_handler_CoveredMemoryRecord>> = Rc::new(RefCell::new(
        (*gCoveredMemoryRecordsHead_13.with(Value::clone).borrow()).clone(),
    ));
    let previous: Value<Ptr<v8_internal_trap_handler_CoveredMemoryRecord>> =
        Rc::new(RefCell::new(Ptr::<
            v8_internal_trap_handler_CoveredMemoryRecord,
        >::null()));
    'loop_: while (!((*current.borrow()).is_null()))
        && ({
            let _lhs = (*(*(*current.borrow()).upgrade().deref()).base.borrow());
            _lhs != (*base.borrow())
        })
    {
        (*previous.borrow_mut()) = (*current.borrow()).clone();
        let __rhs = (*(*(*current.borrow()).upgrade().deref()).next.borrow()).clone();
        (*current.borrow_mut()) = __rhs;
    }
    if !(!((*current.borrow()).is_null())) {
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
    if !({
        let _lhs = (*(*(*current.borrow()).upgrade().deref()).size.borrow());
        _lhs == (*reserved_size.borrow())
    }) {
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
    if !(*previous.borrow()).is_null() {
        let __rhs = (*(*(*current.borrow()).upgrade().deref()).next.borrow()).clone();
        (*(*(*previous.borrow()).upgrade().deref()).next.borrow_mut()) = __rhs;
    } else {
        (*gCoveredMemoryRecordsHead_13.with(Value::clone).borrow_mut()) =
            (*(*(*current.borrow()).upgrade().deref()).next.borrow()).clone();
    }
    free_refcount(
        ((*current.borrow()).clone() as Ptr<v8_internal_trap_handler_CoveredMemoryRecord>).to_any(),
    );
}
pub fn GetRecoveredTrapCount_30() -> usize {
    return (({
        ((*gRecoveredTrapCount_14.with(Value::clone).borrow())
            as std___atomic_base_unsigned_long__false_)
            .load_const(Some((*memory_order_relaxed_8.with(Value::clone).borrow())))
    }) as usize);
}
thread_local!(
    pub static g_is_trap_handler_enabled_5: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static g_can_enable_trap_handler_6: Value<std_atomic_bool_> =
        Rc::new(RefCell::new(std_atomic_bool_::std_atomic_bool_1({ true })));
);
pub fn EnableTrapHandler_31(use_v8_handler: bool) -> bool {
    let use_v8_handler: Value<bool> = Rc::new(RefCell::new(use_v8_handler));
    let can_enable: Value<bool> = Rc::new(RefCell::new(
        ({
            ((*g_can_enable_trap_handler_6.with(Value::clone).borrow())
                as std___atomic_base_bool__false_)
                .exchange_bool(
                    false,
                    Some((*memory_order_relaxed_8.with(Value::clone).borrow())),
                )
        }),
    ));
    if !(*can_enable.borrow()) {
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
    if !(true) {
        return false;
    }
    let active_guard: Value<v8_internal_trap_handler_TrapHandlerGuard> = Rc::new(RefCell::new(
        v8_internal_trap_handler_TrapHandlerGuard::v8_internal_trap_handler_TrapHandlerGuard(),
    ));
    let _dtor_active_guard = ScopedDestructor::new(&active_guard, |__p| __p.destructor());
    if (*use_v8_handler.borrow()) {
        (*g_is_trap_handler_enabled_5.with(Value::clone).borrow_mut()) =
            ({ RegisterDefaultTrapHandler_32() });
        return (*g_is_trap_handler_enabled_5.with(Value::clone).borrow());
    }
    (*g_is_trap_handler_enabled_5.with(Value::clone).borrow_mut()) = true;
    return true;
}
pub fn SetLandingPad_33(landing_pad: u64) {
    let landing_pad: Value<u64> = Rc::new(RefCell::new(landing_pad));
    ({
        ((*gLandingPad_15.with(Value::clone).borrow()) as std___atomic_base_unsigned_long__false_)
            .store_u64((*landing_pad.borrow()), None)
    });
}
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_internal_trap_handler_CoveredMemoryRecordsLock;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_internal_trap_handler_SandboxRecordsLock;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_internal_trap_handler_MetadataLock;
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
