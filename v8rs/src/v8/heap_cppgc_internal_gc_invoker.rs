use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type cppgc_EmbedderStackState = i32;
pub const cppgc_EmbedderStackState_kMayContainHeapPointers: cppgc_EmbedderStackState = 0;
pub const cppgc_EmbedderStackState_kNoHeapPointers: cppgc_EmbedderStackState = 1;
#[derive(Default)]
pub struct cppgc_CustomSpaceIndex {
    pub value: Value<usize>,
}
impl cppgc_CustomSpaceIndex {
    pub fn cppgc_CustomSpaceIndex(value: usize) -> Self {
        let value: Value<usize> = Rc::new(RefCell::new(value));
        let __this: Value<cppgc_CustomSpaceIndex> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*value.borrow()))),
        }));
        let this: Ptr<cppgc_CustomSpaceIndex> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for cppgc_CustomSpaceIndex {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_CustomSpaceIndex> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        }));
        let this: Ptr<cppgc_CustomSpaceIndex> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_CustomSpaceIndex {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.value.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: Rc::new(RefCell::new(<usize>::from_bytes(&buf[0..8]))),
        }
    }
}
pub trait cppgc_CustomSpaceBase {
    fn GetCustomSpaceIndex(&self) -> cppgc_CustomSpaceIndex;
    fn IsCompactable(&self) -> bool;
}
thread_local!(
    pub static value_0: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct cppgc_internal_IsAllocatedOnCompactableSpaceImpl_void_ {}
impl Clone for cppgc_internal_IsAllocatedOnCompactableSpaceImpl_void_ {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_IsAllocatedOnCompactableSpaceImpl_void_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<cppgc_internal_IsAllocatedOnCompactableSpaceImpl_void_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_IsAllocatedOnCompactableSpaceImpl_void_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
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
pub fn SharedMemoryHandleFromMachMemoryEntry_1(handle: u32) -> v8_SharedMemoryHandle {
    let handle: Value<u32> = Rc::new(RefCell::new(handle));
    return ({ v8_SharedMemoryHandle::FromPlatformHandle((*handle.borrow())) });
}
pub fn MachMemoryEntryFromSharedMemoryHandle_2(handle: v8_SharedMemoryHandle) -> u32 {
    let handle: Value<v8_SharedMemoryHandle> = Rc::new(RefCell::new(handle));
    return ({ v8_SharedMemoryHandleImpl::GetPlatformHandle(&handle.as_pointer()) });
}
thread_local!(
    pub static kInvalidSharedMemoryHandle_3: Value<std_optional_v8_SharedMemoryHandle_> =
        Rc::new(RefCell::new(
            std_optional_v8_SharedMemoryHandle_::std_optional_v8_SharedMemoryHandle_1({
                (*nullopt_4.with(Value::clone).borrow()).clone()
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
pub fn operator_bitor_5(lhs: v8_PagePermissions, rhs: v8_PagePermissions) -> v8_PagePermissions {
    let lhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(lhs));
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as i32) | ((*rhs.borrow()) as i32)) as v8_PagePermissions);
}
pub fn operator_bitand_6(lhs: v8_PagePermissions, rhs: v8_PagePermissions) -> v8_PagePermissions {
    let lhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(lhs));
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as i32) & ((*rhs.borrow()) as i32)) as v8_PagePermissions);
}
pub fn operator_bitor_assign_7(
    lhs: Ptr<v8_PagePermissions>,
    rhs: v8_PagePermissions,
) -> Ptr<v8_PagePermissions> {
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    let __rhs = ({
        let _lhs: v8_PagePermissions = (lhs.read());
        let _rhs: v8_PagePermissions = (*rhs.borrow());
        operator_bitor_5(_lhs, _rhs)
    });
    lhs.write(__rhs);
    return (lhs).clone();
}
pub fn IsSubset_8(lhs: v8_PagePermissions, rhs: v8_PagePermissions) -> bool {
    let lhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(lhs));
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    return (({
        let _lhs: v8_PagePermissions = (*lhs.borrow());
        operator_bitand_6(_lhs, (*rhs.borrow()))
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
        return (({ floor_9(({ self.CurrentClockTimeMillis() })) }) as i64);
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
            static default_observer_10: Value<v8_HighAllocationThroughputObserver> = Rc::new(
                RefCell::new(<v8_HighAllocationThroughputObserver>::default()),
            );
        );
        return (default_observer_10.with(Value::clone).as_pointer());
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
pub trait cppgc_Platform {
    fn GetPageAllocator(&self) -> PtrDyn<dyn v8_PageAllocator>;
    fn MonotonicallyIncreasingTime(&self) -> f64;
    fn GetForegroundTaskRunner(&self) -> std_shared_ptr_v8_TaskRunner_ {
        return ({ self.GetForegroundTaskRunner_v8_TaskPriority(v8_TaskPriority_kUserBlocking) });
    }
    fn GetForegroundTaskRunner_v8_TaskPriority(
        &self,
        priority: v8_TaskPriority,
    ) -> std_shared_ptr_v8_TaskRunner_ {
        let priority: Value<v8_TaskPriority> = Rc::new(RefCell::new(priority));
        return std_shared_ptr_v8_TaskRunner_::std_shared_ptr_v8_TaskRunner_1({
            Default::default()
        });
    }
    fn PostJob(
        &self,
        priority: v8_TaskPriority,
        job_task: Option<Value<v8_JobTask>>,
    ) -> Option<Value<v8_JobHandle>> {
        let priority: Value<v8_TaskPriority> = Rc::new(RefCell::new(priority));
        let job_task: Value<Option<Value<v8_JobTask>>> = Rc::new(RefCell::new(job_task));
        return std_unique_ptr_v8_JobHandle__std_default_delete_v8_JobHandle__ :: std_unique_ptr_v8_JobHandle__std_default_delete_v8_JobHandle__ ( {  Default::default()   } , )   ;
    }
}
#[derive()]
pub struct cppgc_StackStartMarker {
    stack_start_: Value<AnyPtr>,
}
impl cppgc_StackStartMarker {
    pub fn cppgc_StackStartMarker() -> Self {
        let __this: Value<cppgc_StackStartMarker> = Rc::new(RefCell::new(Self {
            stack_start_: Rc::new(RefCell::new(({ __builtin_frame_address_11(0_u32) }))),
        }));
        let this: Ptr<cppgc_StackStartMarker> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for cppgc_StackStartMarker {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_StackStartMarker> = Rc::new(RefCell::new(Self {
            stack_start_: Rc::new(RefCell::new((*self.stack_start_.borrow()).clone())),
        }));
        let this: Ptr<cppgc_StackStartMarker> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for cppgc_StackStartMarker {
    fn default() -> Self {
        { cppgc_StackStartMarker::cppgc_StackStartMarker() }
    }
}
impl ByteRepr for cppgc_StackStartMarker {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.stack_start_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            stack_start_: Rc::new(RefCell::new(<AnyPtr>::from_bytes(&buf[0..8]))),
        }
    }
}
pub type cppgc_Heap_StackSupport = u8;
pub const cppgc_Heap_StackSupport_kSupportsConservativeStackScan: cppgc_Heap_StackSupport = 0;
pub const cppgc_Heap_StackSupport_kNoConservativeStackScan: cppgc_Heap_StackSupport = 1;
pub type cppgc_Heap_MarkingType = u8;
pub const cppgc_Heap_MarkingType_kAtomic: cppgc_Heap_MarkingType = 0;
pub const cppgc_Heap_MarkingType_kIncremental: cppgc_Heap_MarkingType = 1;
pub const cppgc_Heap_MarkingType_kIncrementalAndConcurrent: cppgc_Heap_MarkingType = 2;
pub type cppgc_Heap_SweepingType = u8;
pub const cppgc_Heap_SweepingType_kAtomic: cppgc_Heap_SweepingType = 0;
pub const cppgc_Heap_SweepingType_kIncremental: cppgc_Heap_SweepingType = 1;
pub const cppgc_Heap_SweepingType_kIncrementalAndConcurrent: cppgc_Heap_SweepingType = 2;
#[derive(Default)]
pub struct cppgc_Heap_ResourceConstraints {
    pub initial_heap_size_bytes: Value<usize>,
}
impl Clone for cppgc_Heap_ResourceConstraints {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_Heap_ResourceConstraints> = Rc::new(RefCell::new(Self {
            initial_heap_size_bytes: Rc::new(RefCell::new(
                (*self.initial_heap_size_bytes.borrow()),
            )),
        }));
        let this: Ptr<cppgc_Heap_ResourceConstraints> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_Heap_ResourceConstraints {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.initial_heap_size_bytes.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            initial_heap_size_bytes: Rc::new(RefCell::new(<usize>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct cppgc_Heap_HeapOptions {
    pub custom_spaces: Value<Vec<Option<Value<cppgc_CustomSpaceBase>>>>,
    pub stack_support: Value<cppgc_Heap_StackSupport>,
    pub marking_support: Value<cppgc_Heap_MarkingType>,
    pub sweeping_support: Value<cppgc_Heap_SweepingType>,
    pub resource_constraints: Value<cppgc_Heap_ResourceConstraints>,
    pub stack_start_marker: Value<std_optional_cppgc_StackStartMarker_>,
}
impl cppgc_Heap_HeapOptions {
    pub fn Default() -> cppgc_Heap_HeapOptions {
        return cppgc_Heap_HeapOptions {
            custom_spaces: Rc::new(RefCell::new(Vec::new())),
            stack_support: Rc::new(RefCell::new(
                cppgc_Heap_StackSupport_kSupportsConservativeStackScan,
            )),
            marking_support: Rc::new(RefCell::new(
                cppgc_Heap_MarkingType_kIncrementalAndConcurrent,
            )),
            sweeping_support: Rc::new(RefCell::new(
                cppgc_Heap_SweepingType_kIncrementalAndConcurrent,
            )),
            resource_constraints: Rc::new(RefCell::new(cppgc_Heap_ResourceConstraints {
                initial_heap_size_bytes: Rc::new(RefCell::new(0_usize)),
            })),
            stack_start_marker: Rc::new(RefCell::new(
                std_optional_cppgc_StackStartMarker_::std_optional_cppgc_StackStartMarker_2({
                    (*nullopt_4.with(Value::clone).borrow()).clone()
                }),
            )),
        };
    }
}
impl Clone for cppgc_Heap_HeapOptions {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_Heap_HeapOptions> = Rc::new(RefCell::new(Self {
            custom_spaces: Rc::new(RefCell::new((*self.custom_spaces.borrow()).clone())),
            stack_support: Rc::new(RefCell::new((*self.stack_support.borrow()))),
            marking_support: Rc::new(RefCell::new((*self.marking_support.borrow()))),
            sweeping_support: Rc::new(RefCell::new((*self.sweeping_support.borrow()))),
            resource_constraints: Rc::new(RefCell::new(
                (*self.resource_constraints.borrow()).clone(),
            )),
            stack_start_marker: Rc::new(RefCell::new((*self.stack_start_marker.borrow()).clone())),
        }));
        let this: Ptr<cppgc_Heap_HeapOptions> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_Heap_HeapOptions {
    fn byte_size() -> usize {
        56
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.custom_spaces.borrow()).to_bytes(&mut buf[0..24]);
        (*self.stack_support.borrow()).to_bytes(&mut buf[24..25]);
        (*self.marking_support.borrow()).to_bytes(&mut buf[25..26]);
        (*self.sweeping_support.borrow()).to_bytes(&mut buf[26..27]);
        (*self.resource_constraints.borrow()).to_bytes(&mut buf[32..40]);
        (*self.stack_start_marker.borrow()).to_bytes(&mut buf[40..56]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            custom_spaces: Rc::new(RefCell::new(
                <Vec<Option<Value<cppgc_CustomSpaceBase>>>>::from_bytes(&buf[0..24]),
            )),
            stack_support: Rc::new(RefCell::new(<cppgc_Heap_StackSupport>::from_bytes(
                &buf[24..25],
            ))),
            marking_support: Rc::new(RefCell::new(<cppgc_Heap_MarkingType>::from_bytes(
                &buf[25..26],
            ))),
            sweeping_support: Rc::new(RefCell::new(<cppgc_Heap_SweepingType>::from_bytes(
                &buf[26..27],
            ))),
            resource_constraints: Rc::new(RefCell::new(
                <cppgc_Heap_ResourceConstraints>::from_bytes(&buf[32..40]),
            )),
            stack_start_marker: Rc::new(RefCell::new(
                <std_optional_cppgc_StackStartMarker_>::from_bytes(&buf[40..56]),
            )),
        }
    }
}
#[derive(Default)]
pub struct cppgc_Heap {}
impl Clone for cppgc_Heap {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_Heap> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<cppgc_Heap> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_Heap {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub type v8_base_AbortMode = i32;
pub const v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures: v8_base_AbortMode = 0;
pub const v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures: v8_base_AbortMode = 1;
pub const v8_base_AbortMode_kExitIfNoSecurityImpact: v8_base_AbortMode = 2;
pub const v8_base_AbortMode_kImmediateCrash: v8_base_AbortMode = 3;
pub const v8_base_AbortMode_kDefault: v8_base_AbortMode = 4;
thread_local!();
pub fn ControlledCrashesAreHarmless_13() -> bool {
    return ((*g_abort_mode_12.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_12.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn DcheckFailuresAreIgnored_14() -> bool {
    return ((*g_abort_mode_12.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_12.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn FatalErrorsWithNoSecurityImpactShouldExit_15() -> bool {
    return ((*g_abort_mode_12.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitIfNoSecurityImpact);
}
thread_local!(
    pub static kReturnAddressStackSlotCount_16: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kPageSizeBits_17: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
    pub static kRegularPageSize_18: Value<i32> = Rc::new(RefCell::new(262144));
);
thread_local!(
    pub static kMinimumOSPageSize_19: Value<i32> = Rc::new(RefCell::new(16384));
);
thread_local!(
    pub static kUnimplementedCodeMessage_20: Value<Ptr<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"unimplemented code"),
    ));
);
thread_local!(
    pub static kUnreachableCodeMessage_21: Value<Ptr<u8>> =
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
    pub static is_enum_22: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static is_enum_23: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_24: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_25: Value<bool> = Rc::new(RefCell::new(true));
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
    pub static value_26: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_27: Value<bool> = Rc::new(RefCell::new(false));
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
pub fn make_uint64_28(high: u32, low: u32) -> u64 {
    let high: Value<u32> = Rc::new(RefCell::new(high));
    let low: Value<u32> = Rc::new(RefCell::new(low));
    return (((*high.borrow()) as u64) << 32).wrapping_add(((*low.borrow()) as u64));
}
pub fn RoundDown_29(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    return ((*x.borrow()) & (-(*m.borrow()) as u64));
}
pub fn RoundUp_30(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    (&(0));
    return ({
        let _x: u64 = ((*x.borrow()).wrapping_add((((*m.borrow()) - 1_i64) as u64)) as u64);
        let _m: i64 = (*m.borrow());
        RoundDown_29(_x, _m)
    });
}
pub fn IsAligned_31(value: u64, alignment: u64) -> bool {
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
pub fn HardeningAbort_33() {
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
pub fn is_constant_evaluated_34() -> bool {
    return ({ is_constant_evaluated_35() });
}
thread_local!(
    pub static in_place_36: Ptr<std_in_place_t> = in_place_37.with(Value::clone).as_pointer();
);
pub type absl_internal_any_invocable_StorageProperty = usize;
pub const absl_internal_any_invocable_StorageProperty_kAlignment:
    absl_internal_any_invocable_StorageProperty = 8;
pub const absl_internal_any_invocable_StorageProperty_kStorageSize:
    absl_internal_any_invocable_StorageProperty = 16;
pub type absl_internal_any_invocable_FunctionToCall = u8;
pub const absl_internal_any_invocable_FunctionToCall_dispose:
    absl_internal_any_invocable_FunctionToCall = 0;
pub const absl_internal_any_invocable_FunctionToCall_relocate_from_to:
    absl_internal_any_invocable_FunctionToCall = 1;
pub const absl_internal_any_invocable_FunctionToCall_relocate_from_to_and_query_rust:
    absl_internal_any_invocable_FunctionToCall = 2;
#[derive(Default)]
pub struct anon_38 {
    pub target: Value<AnyPtr>,
    pub size: Value<usize>,
}
impl Clone for anon_38 {
    fn clone(&self) -> Self {
        let __this: Value<anon_38> = Rc::new(RefCell::new(Self {
            target: Rc::new(RefCell::new((*self.target.borrow()).clone())),
            size: Rc::new(RefCell::new((*self.size.borrow()))),
        }));
        let this: Ptr<anon_38> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for anon_38 {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.target.borrow()).to_bytes(&mut buf[0..8]);
        (*self.size.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            target: Rc::new(RefCell::new(<AnyPtr>::from_bytes(&buf[0..8]))),
            size: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
        }
    }
}
pub struct absl_internal_any_invocable_TypeErasedState {
    __bytes: Value<Box<[u8]>>,
}
impl absl_internal_any_invocable_TypeErasedState {
    pub fn remote(&self) -> Ptr<anon_38> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn storage(&self) -> Ptr<u8> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for absl_internal_any_invocable_TypeErasedState {
    fn clone(&self) -> Self {
        absl_internal_any_invocable_TypeErasedState {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for absl_internal_any_invocable_TypeErasedState {
    fn default() -> Self {
        absl_internal_any_invocable_TypeErasedState {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 16]))),
        }
    }
}
impl ByteRepr for absl_internal_any_invocable_TypeErasedState {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        absl_internal_any_invocable_TypeErasedState {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
pub fn EmptyManager_39(
    _a0: absl_internal_any_invocable_FunctionToCall,
    _a1: Ptr<absl_internal_any_invocable_TypeErasedState>,
    _a2: Ptr<absl_internal_any_invocable_TypeErasedState>,
) {
    let _a0: Value<absl_internal_any_invocable_FunctionToCall> = Rc::new(RefCell::new(_a0));
    let _a1: Value<Ptr<absl_internal_any_invocable_TypeErasedState>> = Rc::new(RefCell::new(_a1));
    let _a2: Value<Ptr<absl_internal_any_invocable_TypeErasedState>> = Rc::new(RefCell::new(_a2));
}
pub fn LocalManagerTrivial_40(
    _a0: absl_internal_any_invocable_FunctionToCall,
    from: Ptr<absl_internal_any_invocable_TypeErasedState>,
    to: Ptr<absl_internal_any_invocable_TypeErasedState>,
) {
    let _a0: Value<absl_internal_any_invocable_FunctionToCall> = Rc::new(RefCell::new(_a0));
    let from: Value<Ptr<absl_internal_any_invocable_TypeErasedState>> = Rc::new(RefCell::new(from));
    let to: Value<Ptr<absl_internal_any_invocable_TypeErasedState>> = Rc::new(RefCell::new(to));
    let __rhs = (*(*from.borrow()).upgrade().deref()).clone();
    (*to.borrow()).write(__rhs);
}
pub fn RemoteManagerTrivial_41(
    operation: absl_internal_any_invocable_FunctionToCall,
    from: Ptr<absl_internal_any_invocable_TypeErasedState>,
    to: Ptr<absl_internal_any_invocable_TypeErasedState>,
) {
    let operation: Value<absl_internal_any_invocable_FunctionToCall> =
        Rc::new(RefCell::new(operation));
    let from: Value<Ptr<absl_internal_any_invocable_TypeErasedState>> = Rc::new(RefCell::new(from));
    let to: Value<Ptr<absl_internal_any_invocable_TypeErasedState>> = Rc::new(RefCell::new(to));
    'switch: {
        let __match_cond = (*operation.borrow());
        match __match_cond {
            __v if __v == 1 || __v == 2 => {
                let __rhs = (*(*(*from.borrow()).upgrade().deref())
                    .remote()
                    .upgrade()
                    .deref())
                .clone();
                (*(*to.borrow()).upgrade().deref()).remote().write(__rhs);
                return;
            }
            __v if __v == 0 => {
                free_refcount(
                    (*(*(*(*from.borrow()).upgrade().deref())
                        .remote()
                        .upgrade()
                        .deref())
                    .target
                    .borrow())
                    .clone(),
                );
                return;
            }
            _ => {}
        }
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        panic!("builtin trap");
        unreachable!();
    }
}
#[derive(Default)]
pub struct absl_internal_any_invocable_TrivialDeleter {
    size_: Value<usize>,
}
impl absl_internal_any_invocable_TrivialDeleter {
    pub fn absl_internal_any_invocable_TrivialDeleter(size: usize) -> Self {
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let __this: Value<absl_internal_any_invocable_TrivialDeleter> =
            Rc::new(RefCell::new(Self {
                size_: Rc::new(RefCell::new((*size.borrow()))),
            }));
        let this: Ptr<absl_internal_any_invocable_TrivialDeleter> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for absl_internal_any_invocable_TrivialDeleter {
    fn clone(&self) -> Self {
        let __this: Value<absl_internal_any_invocable_TrivialDeleter> =
            Rc::new(RefCell::new(Self {
                size_: Rc::new(RefCell::new((*self.size_.borrow()))),
            }));
        let this: Ptr<absl_internal_any_invocable_TrivialDeleter> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_internal_any_invocable_TrivialDeleter {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.size_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            size_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[0..8]))),
        }
    }
}
pub fn IsCompatibleConversion_42(_a0: AnyPtr, _a1: AnyPtr) -> bool {
    let _a0: Value<AnyPtr> = Rc::new(RefCell::new(_a0));
    let _a1: Value<AnyPtr> = Rc::new(RefCell::new(_a1));
    return false;
}
#[derive(Default)]
pub struct absl_internal_any_invocable_ConversionConstruct {}
impl Clone for absl_internal_any_invocable_ConversionConstruct {
    fn clone(&self) -> Self {
        let __this: Value<absl_internal_any_invocable_ConversionConstruct> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<absl_internal_any_invocable_ConversionConstruct> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for absl_internal_any_invocable_ConversionConstruct {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub struct absl_functional_internal_VoidPtr {
    __bytes: Value<Box<[u8]>>,
}
impl absl_functional_internal_VoidPtr {
    pub fn obj(&self) -> Ptr<AnyPtr> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn fun(&self) -> Ptr<FnPtr<fn()>> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for absl_functional_internal_VoidPtr {
    fn clone(&self) -> Self {
        absl_functional_internal_VoidPtr {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for absl_functional_internal_VoidPtr {
    fn default() -> Self {
        absl_functional_internal_VoidPtr {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
impl ByteRepr for absl_functional_internal_VoidPtr {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        absl_functional_internal_VoidPtr {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
#[derive()]
pub struct absl_FunctionRef_bool____ {
    ptr_: Value<absl_functional_internal_VoidPtr>,
    invoker_: Value<FnPtr<fn(absl_functional_internal_VoidPtr) -> bool>>,
}
impl Clone for absl_FunctionRef_bool____ {
    fn clone(&self) -> Self {
        let __this: Value<absl_FunctionRef_bool____> = Rc::new(RefCell::new(Self {
            ptr_: Rc::new(RefCell::new((*self.ptr_.borrow()).clone())),
            invoker_: Rc::new(RefCell::new((*self.invoker_.borrow()).clone())),
        }));
        let this: Ptr<absl_FunctionRef_bool____> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for absl_FunctionRef_bool____ {
    fn default() -> Self {
        absl_FunctionRef_bool____ {
            ptr_: <Value<absl_functional_internal_VoidPtr>>::default(),
            invoker_: Rc::new(RefCell::new(FnPtr::<
                fn(absl_functional_internal_VoidPtr) -> bool,
            >::null())),
        }
    }
}
impl ByteRepr for absl_FunctionRef_bool____ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.ptr_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.invoker_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            ptr_: Rc::new(RefCell::new(
                <absl_functional_internal_VoidPtr>::from_bytes(&buf[0..8]),
            )),
            invoker_: Rc::new(RefCell::new(<FnPtr<
                fn(absl_functional_internal_VoidPtr) -> bool,
            >>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_internal_ExtractArgsImpl_bool____ {}
impl Clone for v8_base_internal_ExtractArgsImpl_bool____ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_ExtractArgsImpl_bool____> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_ExtractArgsImpl_bool____> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_ExtractArgsImpl_bool____ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_internal_ExtractCallableRunTypeImpl_v8_base_FunctionRef_bool______bool__v8_base_FunctionRef_bool_____ptr____const_
{}
impl Clone for v8_base_internal_ExtractCallableRunTypeImpl_v8_base_FunctionRef_bool______bool__v8_base_FunctionRef_bool_____ptr____const_ { fn clone(&self) -> Self { let __this : Value<v8_base_internal_ExtractCallableRunTypeImpl_v8_base_FunctionRef_bool______bool__v8_base_FunctionRef_bool_____ptr____const_> = Rc::new(RefCell::new(Self { } )) ;
 let this : Ptr<v8_base_internal_ExtractCallableRunTypeImpl_v8_base_FunctionRef_bool______bool__v8_base_FunctionRef_bool_____ptr____const_> = __this.as_pointer() ;
 Rc::try_unwrap(__this).ok().unwrap().into_inner() } }
impl ByteRepr for  v8_base_internal_ExtractCallableRunTypeImpl_v8_base_FunctionRef_bool______bool__v8_base_FunctionRef_bool_____ptr____const_ { fn byte_size() -> usize { 1 } fn to_bytes(&self, buf: &mut [u8]) { } fn from_bytes(buf: &[u8]) -> Self { Self { } } }
#[derive(Default)]
pub struct v8_base_internal_FunctorTraits_v8_base_FunctionRef_bool_____ {}
impl Clone for v8_base_internal_FunctorTraits_v8_base_FunctionRef_bool_____ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_FunctorTraits_v8_base_FunctionRef_bool_____> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_FunctorTraits_v8_base_FunctionRef_bool_____> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_FunctorTraits_v8_base_FunctionRef_bool_____ {
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
#[derive(Default)]
pub struct v8_base_internal_DecayedFunctorTraits_v8_base_FunctionRef_bool_____ {}
impl Clone for v8_base_internal_DecayedFunctorTraits_v8_base_FunctionRef_bool_____ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_DecayedFunctorTraits_v8_base_FunctionRef_bool_____> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_DecayedFunctorTraits_v8_base_FunctionRef_bool_____> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_DecayedFunctorTraits_v8_base_FunctionRef_bool_____ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static is_instantiation_v_47: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static is_instantiation_v_48: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    static kCompatibleFunctor_49: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Default)]
pub struct v8_base_FunctionRef_bool____ {
    wrapped_func_ref_: Value<absl_FunctionRef_bool____>,
}
impl Clone for v8_base_FunctionRef_bool____ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_FunctionRef_bool____> = Rc::new(RefCell::new(Self {
            wrapped_func_ref_: Rc::new(RefCell::new((*self.wrapped_func_ref_.borrow()).clone())),
        }));
        let this: Ptr<v8_base_FunctionRef_bool____> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_FunctionRef_bool____ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.wrapped_func_ref_.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            wrapped_func_ref_: Rc::new(RefCell::new(<absl_FunctionRef_bool____>::from_bytes(
                &buf[0..16],
            ))),
        }
    }
}
pub fn CountLeadingZeros_50(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        if (32_u32 == 64_u32) {
            (({ __builtin_clzll_51(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((32_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros_52(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        if (64_u32 == 64_u32) {
            (({ __builtin_clzll_51(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((64_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros32_53(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_50((*value.borrow())) });
}
pub fn CountLeadingZeros64_54(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_52((*value.borrow())) });
}
pub fn CountTrailingZeros_55(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        (if (32_u32 == 64_u32) {
            ({ __builtin_ctzll_56(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros_57(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        (if (64_u32 == 64_u32) {
            ({ __builtin_ctzll_56(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros32_58(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_55((*value.borrow())) });
}
pub fn CountTrailingZeros64_59(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_57((*value.borrow())) });
}
pub fn RoundUpToPowerOfTwo32_60(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u32 << ((32_u32).wrapping_sub(({ CountLeadingZeros_50((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo64_61(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u64 << ((64_u32).wrapping_sub(({ CountLeadingZeros_52((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo_62(value: usize) -> usize {
    let value: Value<usize> = Rc::new(RefCell::new(value));
    if (::std::mem::size_of::<usize>() == ::std::mem::size_of::<u64>()) {
        return (({ RoundUpToPowerOfTwo64_61(((*value.borrow()) as u64)) }) as usize);
    } else {
        return (({ RoundUpToPowerOfTwo32_60(((*value.borrow()) as u32)) }) as usize);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn RoundDownToPowerOfTwo32_63(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    if ((*value.borrow()) > 2147483648_u32) {
        return 2147483648_u32;
    }
    let result: Value<u32> = Rc::new(RefCell::new(
        ({ RoundUpToPowerOfTwo32_60((*value.borrow())) }),
    ));
    if ((*result.borrow()) > (*value.borrow())) {
        (*result.borrow_mut()) >>= 1;
    }
    return (*result.borrow());
}
pub fn RotateRight32_64(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateLeft32_65(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateRight64_66(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn RotateLeft64_67(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn ClearLsb_68(value: i32) -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(value));
    return ((*value.borrow()) & ((*value.borrow()) - 1));
}
pub fn SignedAddOverflow32_69(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_sadd_overflow_70(_arg0, _arg1, _arg2)
    });
}
pub fn SignedSubOverflow32_71(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_ssub_overflow_72(_arg0, _arg1, _arg2)
    });
}
pub fn SignedMulOverflow32_73(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_smul_overflow_74(_arg0, _arg1, _arg2)
    });
}
pub fn SignedAddOverflow64_75(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_add_overflow_76(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedSubOverflow64_77(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_sub_overflow_78(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedMulOverflow64_79(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return {
        let (val, ovf) = (*lhs.borrow()).overflowing_mul((*rhs.borrow()));
        (*val.borrow()).write(val);
        ovf
    };
}
pub fn UnsignedAddOverflow32_80(lhs: u32, rhs: u32, val: Ptr<u32>) -> bool {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<u32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: u32 = (*lhs.borrow());
        let _arg1: u32 = (*rhs.borrow());
        let _arg2: Ptr<u32> = (*val.borrow()).clone();
        __builtin_uadd_overflow_81(_arg0, _arg1, _arg2)
    });
}
pub fn UnsignedDiv32_82(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedDiv64_83(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn UnsignedMod32_84(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedMod64_85(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn WraparoundAdd32_86(lhs: i32, rhs: i32) -> i32 {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as u32).wrapping_add(((*rhs.borrow()) as u32))) as i32);
}
pub fn WraparoundNeg32_87(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (-((*x.borrow()) as u32) as i32);
}
pub fn ByteReverse16_88(value: u16) -> u16 {
    let value: Value<u16> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse32_89(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse64_90(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
thread_local!(
    pub static kMaxExponent_91: Value<i32> = Rc::new(RefCell::new(64));
);
thread_local!(
    pub static kMaxExponent_92: Value<i32> = Rc::new(RefCell::new(1024));
);
thread_local!(
    pub static kIntegerBitsPlusSign_93: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kIntegerBitsPlusSign_94: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kIntegerBitsPlusSign_95: Value<i32> = Rc::new(RefCell::new(16));
);
thread_local!(
    pub static kIntegerBitsPlusSign_96: Value<i32> = Rc::new(RefCell::new(16));
);
thread_local!(
    pub static kIntegerBitsPlusSign_97: Value<i32> = Rc::new(RefCell::new(32));
);
thread_local!(
    pub static kIntegerBitsPlusSign_98: Value<i32> = Rc::new(RefCell::new(32));
);
thread_local!(
    pub static kIntegerBitsPlusSign_99: Value<i32> = Rc::new(RefCell::new(64));
);
thread_local!(
    pub static kIntegerBitsPlusSign_100: Value<i32> = Rc::new(RefCell::new(64));
);
thread_local!(
    pub static kIntegerBitsPlusSign_101: Value<i32> = Rc::new(RefCell::new(64));
);
pub fn IsValueNegative_102(value: i64) -> bool {
    let value: Value<i64> = Rc::new(RefCell::new(value));
    if true {
        return ((*value.borrow()) < 0_i64);
    } else {
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ConditionalNegate_103(x: u64, is_negative: bool) -> i64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let is_negative: Value<bool> = Rc::new(RefCell::new(is_negative));
    return (((((*x.borrow()) as u64) ^ (-((*is_negative.borrow()) as i64) as u64))
        .wrapping_add(((*is_negative.borrow()) as u64))) as i64);
}
pub fn SafeUnsignedAbs_104(value: i64) -> u64 {
    let value: Value<i64> = Rc::new(RefCell::new(value));
    return if ({ IsValueNegative_102((*value.borrow())) }) {
        (0_u64).wrapping_sub(((*value.borrow()) as u64))
    } else {
        ((*value.borrow()) as u64)
    };
}
thread_local!(
    pub static kEnableAsmCode_105: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Default)]
pub struct v8_base_internal_CheckOnFailure {}
impl v8_base_internal_CheckOnFailure {
    pub fn HandleFailure_long_long() -> i64 {
        panic!("builtin trap");
        return <i64>::default();
    }
}
impl Clone for v8_base_internal_CheckOnFailure {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_CheckOnFailure> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_CheckOnFailure> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_CheckOnFailure {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub type v8_base_internal_IntegerRepresentation = i32;
pub const v8_base_internal_IntegerRepresentation_kUnsigned: v8_base_internal_IntegerRepresentation =
    0;
pub const v8_base_internal_IntegerRepresentation_kSigned: v8_base_internal_IntegerRepresentation =
    1;
pub type v8_base_internal_NumericRangeRepresentation = i32;
pub const v8_base_internal_NumericRangeRepresentation_kNotContained:
    v8_base_internal_NumericRangeRepresentation = 0;
pub const v8_base_internal_NumericRangeRepresentation_kContained:
    v8_base_internal_NumericRangeRepresentation = 1;
thread_local!(
    pub static kStaticDstRangeRelationToSrcRange_106: Value<
        v8_base_internal_NumericRangeRepresentation,
    > = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kStaticDstRangeRelationToSrcRange_108: Value<
        v8_base_internal_NumericRangeRepresentation,
    > = Rc::new(RefCell::new(0));
);
#[derive(Default)]
pub struct v8_base_internal_RangeCheck {
    is_underflow_: Value<bool>,
    is_overflow_: Value<bool>,
}
impl v8_base_internal_RangeCheck {
    pub fn v8_base_internal_RangeCheck1(is_in_lower_bound: bool, is_in_upper_bound: bool) -> Self {
        let is_in_lower_bound: Value<bool> = Rc::new(RefCell::new(is_in_lower_bound));
        let is_in_upper_bound: Value<bool> = Rc::new(RefCell::new(is_in_upper_bound));
        let __this: Value<v8_base_internal_RangeCheck> = Rc::new(RefCell::new(Self {
            is_underflow_: Rc::new(RefCell::new(!(*is_in_lower_bound.borrow()))),
            is_overflow_: Rc::new(RefCell::new(!(*is_in_upper_bound.borrow()))),
        }));
        let this: Ptr<v8_base_internal_RangeCheck> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for v8_base_internal_RangeCheck {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_internal_RangeCheckImpl::operator_eq(
                &Rc::new(RefCell::new(v8_base_internal_RangeCheck {
                    is_underflow_: self.is_underflow_.clone(),
                    is_overflow_: self.is_overflow_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(v8_base_internal_RangeCheck {
                    is_underflow_: other.is_underflow_.clone(),
                    is_overflow_: other.is_overflow_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for v8_base_internal_RangeCheck {}
impl Clone for v8_base_internal_RangeCheck {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_RangeCheck> = Rc::new(RefCell::new(Self {
            is_underflow_: Rc::new(RefCell::new((*self.is_underflow_.borrow()))),
            is_overflow_: Rc::new(RefCell::new((*self.is_overflow_.borrow()))),
        }));
        let this: Ptr<v8_base_internal_RangeCheck> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_RangeCheck {
    fn byte_size() -> usize {
        2
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.is_underflow_.borrow()).to_bytes(&mut buf[0..1]);
        (*self.is_overflow_.borrow()).to_bytes(&mut buf[1..2]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            is_underflow_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[0..1]))),
            is_overflow_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[1..2]))),
        }
    }
}
thread_local!(
    pub static kShift_109: Value<i32> = Rc::new(RefCell::new(10));
);
#[derive(Default)]
pub struct v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_
{}
impl v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_ {
    pub fn max() -> i64 {
        return ({
            v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_::Adjust ( <i64>::MAX   , )
        });
    }
    pub fn lowest() -> i64 {
        return ({
            v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_::Adjust ( (  { std_numeric_limits_long_long_::lowest ( ) } )   , )
        });
    }
    pub fn Adjust_i64__long_long(value: i64) -> i64 {
        let value: Value<i64> = Rc::new(RefCell::new(value));
        if true {
            return (({
                let _x: u64 = (({ SafeUnsignedAbs_104((*value.borrow())) })
                    & !(((1_u64 << 10) as u64).wrapping_sub((1_u64 as u64))));
                let _is_negative: bool = ({ IsValueNegative_102((*value.borrow())) });
                ConditionalNegate_103(_x, _is_negative)
            }) as i64);
        } else {
        }
        panic!("ub: non-void function does not return a value")
    }
}
impl Clone
    for v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_
{
    fn clone(&self) -> Self {
        let __this : Value<v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_> = Rc::new(RefCell::new(Self { } )) ;
        let this : Ptr<v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_> = __this.as_pointer() ;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr
    for v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_
{
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_internal_DstRangeRelationToSrcRangeImpl_long_long__double__v8_base_internal_SaturationDefaultLimits__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_NumericRangeRepresentation_kNotContained_
{}
impl v8_base_internal_DstRangeRelationToSrcRangeImpl_long_long__double__v8_base_internal_SaturationDefaultLimits__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_NumericRangeRepresentation_kNotContained_ { pub  fn Check ( value : f64  , ) -> v8_base_internal_RangeCheck  { let value : Value<f64 >  = Rc::new(RefCell::new(value)) ;
 ;
 ;
  return v8_base_internal_RangeCheck :: v8_base_internal_RangeCheck1 ( {  ( (*value.borrow()) >= ( ( (  { v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_::lowest ( ) } )  as f64 ) ) )   } , {  ( (*value.borrow()) <= ( ( (  { v8_base_internal_NarrowingRange_long_long__double__v8_base_internal_SaturationDefaultLimits_::max ( ) } )  as f64 ) ) )   } , )   ;
 } }
impl Clone for v8_base_internal_DstRangeRelationToSrcRangeImpl_long_long__double__v8_base_internal_SaturationDefaultLimits__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_NumericRangeRepresentation_kNotContained_ { fn clone(&self) -> Self { let __this : Value<v8_base_internal_DstRangeRelationToSrcRangeImpl_long_long__double__v8_base_internal_SaturationDefaultLimits__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_NumericRangeRepresentation_kNotContained_> = Rc::new(RefCell::new(Self { } )) ;
 let this : Ptr<v8_base_internal_DstRangeRelationToSrcRangeImpl_long_long__double__v8_base_internal_SaturationDefaultLimits__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_NumericRangeRepresentation_kNotContained_> = __this.as_pointer() ;
 Rc::try_unwrap(__this).ok().unwrap().into_inner() } }
impl ByteRepr for  v8_base_internal_DstRangeRelationToSrcRangeImpl_long_long__double__v8_base_internal_SaturationDefaultLimits__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_NumericRangeRepresentation_kNotContained_ { fn byte_size() -> usize { 1 } fn to_bytes(&self, buf: &mut [u8]) { } fn from_bytes(buf: &[u8]) -> Self { Self { } } }
pub fn DstRangeRelationToSrcRange_110(value: f64) -> v8_base_internal_RangeCheck {
    let value: Value<f64> = Rc::new(RefCell::new(value));
    return ({
        v8_base_internal_DstRangeRelationToSrcRangeImpl_long_long__double__v8_base_internal_SaturationDefaultLimits__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_IntegerRepresentation_kSigned__v8_base_internal_NumericRangeRepresentation_kNotContained_::Check ( (*value.borrow())  , )
    });
}
#[derive(Default)]
pub struct v8_base_internal_IntegerForDigitsAndSignImpl____true_ {}
impl Clone for v8_base_internal_IntegerForDigitsAndSignImpl____true_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_IntegerForDigitsAndSignImpl____true_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_IntegerForDigitsAndSignImpl____true_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_IntegerForDigitsAndSignImpl____true_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_internal_IntegerForDigitsAndSignImpl____false_ {}
impl Clone for v8_base_internal_IntegerForDigitsAndSignImpl____false_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_IntegerForDigitsAndSignImpl____false_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_IntegerForDigitsAndSignImpl____false_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_IntegerForDigitsAndSignImpl____false_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_internal_ArithmeticOrIntegralConstant_double_ {}
impl Clone for v8_base_internal_ArithmeticOrIntegralConstant_double_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_ArithmeticOrIntegralConstant_double_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_ArithmeticOrIntegralConstant_double_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_ArithmeticOrIntegralConstant_double_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static kIsCheckedNumeric_111: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static kIsClampedNumeric_112: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static kIsStrictNumeric_113: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Default)]
pub struct v8_base_internal_UnderlyingTypeImpl_double_ {}
impl Clone for v8_base_internal_UnderlyingTypeImpl_double_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_UnderlyingTypeImpl_double_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_UnderlyingTypeImpl_double_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_UnderlyingTypeImpl_double_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static kIsNumeric_114: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Default)]
pub struct v8_base_internal_SaturationDefaultLimits_long_long_ {}
impl v8_base_internal_SaturationDefaultLimits_long_long_ {
    pub fn NaN() -> i64 {
        if false {
        } else {
            return <i64>::default();
        }
        panic!("ub: non-void function does not return a value")
    }
    pub fn Overflow() -> i64 {
        if false {
        } else {
            return <i64>::MAX;
        }
        panic!("ub: non-void function does not return a value")
    }
    pub fn Underflow() -> i64 {
        if false {
        } else {
            return ({ std_numeric_limits_long_long_::lowest() });
        }
        panic!("ub: non-void function does not return a value")
    }
}
impl Clone for v8_base_internal_SaturationDefaultLimits_long_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_SaturationDefaultLimits_long_long_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_SaturationDefaultLimits_long_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_SaturationDefaultLimits_long_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn saturated_cast_impl_115(value: f64, constraint: v8_base_internal_RangeCheck) -> i64 {
    let value: Value<f64> = Rc::new(RefCell::new(value));
    let constraint: Value<v8_base_internal_RangeCheck> = Rc::new(RefCell::new(constraint));
    return if !({ v8_base_internal_RangeCheckImpl::IsOverflowFlagSet(&constraint.as_pointer()) }) {
        (if !({ v8_base_internal_RangeCheckImpl::IsUnderflowFlagSet(&constraint.as_pointer()) }) {
            ((*value.borrow()) as i64)
        } else {
            ({ v8_base_internal_SaturationDefaultLimits_long_long_::Underflow() })
        })
    } else {
        (if (false)
            || (!({
                v8_base_internal_RangeCheckImpl::IsUnderflowFlagSet(&constraint.as_pointer())
            }))
        {
            ({ v8_base_internal_SaturationDefaultLimits_long_long_::Overflow() })
        } else {
            ({ v8_base_internal_SaturationDefaultLimits_long_long_::NaN() })
        })
    };
}
thread_local!(
    pub static is_supported_116: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_internal_SaturateFastOp_long_long__double_ {}
impl v8_base_internal_SaturateFastOp_long_long__double_ {
    pub fn Do(_a0: f64) -> i64 {
        let _a0: Value<f64> = Rc::new(RefCell::new(_a0));
        return ({ v8_base_internal_CheckOnFailure::HandleFailure() });
    }
}
impl Clone for v8_base_internal_SaturateFastOp_long_long__double_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_internal_SaturateFastOp_long_long__double_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_internal_SaturateFastOp_long_long__double_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_internal_SaturateFastOp_long_long__double_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn saturated_cast_117(value: f64) -> i64 {
    let value: Value<f64> = Rc::new(RefCell::new(value));
    let underlying_value: Value<f64> = Rc::new(RefCell::new(((*value.borrow()) as f64)));
    return if ((!({ is_constant_evaluated_35() })) && (false)) && (true) {
        ({ v8_base_internal_SaturateFastOp_long_long__double_::Do((*underlying_value.borrow())) })
    } else {
        ({
            let _value: f64 = (*underlying_value.borrow());
            let _constraint: v8_base_internal_RangeCheck =
                ({ DstRangeRelationToSrcRange_110((*underlying_value.borrow())) });
            saturated_cast_impl_115(_value, _constraint)
        })
    };
}
#[derive(Default)]
pub struct v8_base_time_internal_TimeBase_v8_base_Time_ {
    us_: Value<i64>,
}
impl v8_base_time_internal_TimeBase_v8_base_Time_ {
    fn v8_base_time_internal_TimeBase_v8_base_Time_(us: i64) -> Self {
        let us: Value<i64> = Rc::new(RefCell::new(us));
        let __this: Value<v8_base_time_internal_TimeBase_v8_base_Time_> =
            Rc::new(RefCell::new(Self {
                us_: Rc::new(RefCell::new((*us.borrow()))),
            }));
        let this: Ptr<v8_base_time_internal_TimeBase_v8_base_Time_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for v8_base_time_internal_TimeBase_v8_base_Time_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            v8_base_time_internal_TimeBase_v8_base_Time_Impl::operator_cmp(
                &Rc::new(RefCell::new(v8_base_time_internal_TimeBase_v8_base_Time_ {
                    us_: self.us_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(v8_base_time_internal_TimeBase_v8_base_Time_ {
                    us_: other.us_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for v8_base_time_internal_TimeBase_v8_base_Time_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for v8_base_time_internal_TimeBase_v8_base_Time_ {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_time_internal_TimeBase_v8_base_Time_Impl::operator_cmp(
                &Rc::new(RefCell::new(v8_base_time_internal_TimeBase_v8_base_Time_ {
                    us_: self.us_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(v8_base_time_internal_TimeBase_v8_base_Time_ {
                    us_: other.us_.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for v8_base_time_internal_TimeBase_v8_base_Time_ {}
impl Clone for v8_base_time_internal_TimeBase_v8_base_Time_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_time_internal_TimeBase_v8_base_Time_> =
            Rc::new(RefCell::new(Self {
                us_: Rc::new(RefCell::new((*self.us_.borrow()))),
            }));
        let this: Ptr<v8_base_time_internal_TimeBase_v8_base_Time_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_time_internal_TimeBase_v8_base_Time_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.us_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            us_: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
    us_: Value<i64>,
}
impl v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
    fn v8_base_time_internal_TimeBase_v8_base_TimeTicks_(us: i64) -> Self {
        let us: Value<i64> = Rc::new(RefCell::new(us));
        let __this: Value<v8_base_time_internal_TimeBase_v8_base_TimeTicks_> =
            Rc::new(RefCell::new(Self {
                us_: Rc::new(RefCell::new((*us.borrow()))),
            }));
        let this: Ptr<v8_base_time_internal_TimeBase_v8_base_TimeTicks_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            v8_base_time_internal_TimeBase_v8_base_TimeTicks_Impl::operator_cmp(
                &Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
                        us_: self.us_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
                        us_: other.us_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_time_internal_TimeBase_v8_base_TimeTicks_Impl::operator_cmp(
                &Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
                        us_: self.us_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
                        us_: other.us_.clone(),
                    },
                ))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {}
impl Clone for v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_time_internal_TimeBase_v8_base_TimeTicks_> =
            Rc::new(RefCell::new(Self {
                us_: Rc::new(RefCell::new((*self.us_.borrow()))),
            }));
        let this: Ptr<v8_base_time_internal_TimeBase_v8_base_TimeTicks_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_time_internal_TimeBase_v8_base_TimeTicks_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.us_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            us_: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
    us_: Value<i64>,
}
impl v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
    fn v8_base_time_internal_TimeBase_v8_base_ThreadTicks_(us: i64) -> Self {
        let us: Value<i64> = Rc::new(RefCell::new(us));
        let __this: Value<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_> =
            Rc::new(RefCell::new(Self {
                us_: Rc::new(RefCell::new((*us.borrow()))),
            }));
        let this: Ptr<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            v8_base_time_internal_TimeBase_v8_base_ThreadTicks_Impl::operator_cmp(
                &Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
                        us_: self.us_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
                        us_: other.us_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_time_internal_TimeBase_v8_base_ThreadTicks_Impl::operator_cmp(
                &Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
                        us_: self.us_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
                        us_: other.us_.clone(),
                    },
                ))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {}
impl Clone for v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_> =
            Rc::new(RefCell::new(Self {
                us_: Rc::new(RefCell::new((*self.us_.borrow()))),
            }));
        let this: Ptr<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_time_internal_TimeBase_v8_base_ThreadTicks_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.us_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            us_: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
        }
    }
}
thread_local!(
    pub static kHoursPerDay_120: Value<i64> = Rc::new(RefCell::new(24));
);
thread_local!(
    pub static kMillisecondsPerSecond_121: Value<i64> = Rc::new(RefCell::new(1000));
);
thread_local!(
    pub static kMillisecondsPerDay_122: Value<i64> = Rc::new(RefCell::new(86400000));
);
thread_local!(
    pub static kMicrosecondsPerMillisecond_123: Value<i64> = Rc::new(RefCell::new(1000));
);
thread_local!(
    pub static kMicrosecondsPerSecond_124: Value<i64> = Rc::new(RefCell::new(1000000));
);
thread_local!(
    pub static kMicrosecondsPerMinute_125: Value<i64> = Rc::new(RefCell::new(60000000));
);
thread_local!(
    pub static kMicrosecondsPerHour_126: Value<i64> = Rc::new(RefCell::new(3600000000));
);
thread_local!(
    pub static kMicrosecondsPerDay_127: Value<i64> = Rc::new(RefCell::new(86400000000));
);
thread_local!(
    pub static kMicrosecondsPerWeek_128: Value<i64> = Rc::new(RefCell::new(604800000000));
);
thread_local!(
    pub static kNanosecondsPerMicrosecond_129: Value<i64> = Rc::new(RefCell::new(1000));
);
thread_local!(
    pub static kNanosecondsPerSecond_130: Value<i64> = Rc::new(RefCell::new(1000000000));
);
#[derive(Default)]
pub struct v8_base_TimeConstants {}
impl std::cmp::Ord for v8_base_TimeConstants {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            v8_base_TimeConstantsImpl::operator_cmp(
                &Rc::new(RefCell::new(v8_base_TimeConstants {})).as_pointer(),
                Rc::new(RefCell::new(v8_base_TimeConstants {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for v8_base_TimeConstants {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for v8_base_TimeConstants {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_TimeConstantsImpl::operator_cmp(
                &Rc::new(RefCell::new(v8_base_TimeConstants {})).as_pointer(),
                Rc::new(RefCell::new(v8_base_TimeConstants {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for v8_base_TimeConstants {}
impl Clone for v8_base_TimeConstants {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_TimeConstants> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_TimeConstants> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_TimeConstants {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn swap_131(a: v8_base_TimeDelta, b: v8_base_TimeDelta) {
    let a: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(a));
    let b: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(b));
    {
        let tmp = (*a.borrow()).delta_.as_pointer().read();
        (*a.borrow())
            .delta_
            .as_pointer()
            .write((*b.borrow()).delta_.as_pointer().read());
        (*b.borrow()).delta_.as_pointer().write(tmp);
    };
}
#[derive()]
pub struct v8_base_TimeDelta {
    delta_: Value<i64>,
}
impl v8_base_TimeDelta {
    pub fn v8_base_TimeDelta1() -> Self {
        let __this: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(Self {
            delta_: Rc::new(RefCell::new(0_i64)),
        }));
        let this: Ptr<v8_base_TimeDelta> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn FromDays(days: i32) -> v8_base_TimeDelta {
        let days: Value<i32> = Rc::new(RefCell::new(days));
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            (((*days.borrow()) as i64) * 86400000000)
        });
    }
    pub fn FromHours(hours: i32) -> v8_base_TimeDelta {
        let hours: Value<i32> = Rc::new(RefCell::new(hours));
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            (((*hours.borrow()) as i64) * 3600000000)
        });
    }
    pub fn FromMinutes(minutes: i32) -> v8_base_TimeDelta {
        let minutes: Value<i32> = Rc::new(RefCell::new(minutes));
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            (((*minutes.borrow()) as i64) * 60000000)
        });
    }
    pub fn FromSeconds(seconds: i64) -> v8_base_TimeDelta {
        let seconds: Value<i64> = Rc::new(RefCell::new(seconds));
        return v8_base_TimeDelta::v8_base_TimeDelta2({ ((*seconds.borrow()) * 1000000) });
    }
    pub fn FromMilliseconds(milliseconds: i64) -> v8_base_TimeDelta {
        let milliseconds: Value<i64> = Rc::new(RefCell::new(milliseconds));
        return v8_base_TimeDelta::v8_base_TimeDelta2({ ((*milliseconds.borrow()) * 1000) });
    }
    pub fn FromMicroseconds(microseconds: i64) -> v8_base_TimeDelta {
        let microseconds: Value<i64> = Rc::new(RefCell::new(microseconds));
        return v8_base_TimeDelta::v8_base_TimeDelta2({ (*microseconds.borrow()) });
    }
    pub fn FromNanoseconds(nanoseconds: i64) -> v8_base_TimeDelta {
        let nanoseconds: Value<i64> = Rc::new(RefCell::new(nanoseconds));
        return v8_base_TimeDelta::v8_base_TimeDelta2({ ((*nanoseconds.borrow()) / 1000) });
    }
    pub fn FromSecondsD(seconds: f64) -> v8_base_TimeDelta {
        let seconds: Value<f64> = Rc::new(RefCell::new(seconds));
        return ({ v8_base_TimeDelta::FromDouble(((*seconds.borrow()) * (1000000 as f64))) });
    }
    pub fn FromMillisecondsD(milliseconds: f64) -> v8_base_TimeDelta {
        let milliseconds: Value<f64> = Rc::new(RefCell::new(milliseconds));
        return ({ v8_base_TimeDelta::FromDouble(((*milliseconds.borrow()) * (1000 as f64))) });
    }
    fn v8_base_TimeDelta2(delta: i64) -> Self {
        let delta: Value<i64> = Rc::new(RefCell::new(delta));
        let __this: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(Self {
            delta_: Rc::new(RefCell::new((*delta.borrow()))),
        }));
        let this: Ptr<v8_base_TimeDelta> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for v8_base_TimeDelta {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            v8_base_TimeDeltaImpl::operator_cmp(
                &Rc::new(RefCell::new(v8_base_TimeDelta {
                    delta_: self.delta_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(v8_base_TimeDelta {
                    delta_: other.delta_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for v8_base_TimeDelta {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for v8_base_TimeDelta {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_TimeDeltaImpl::operator_cmp(
                &Rc::new(RefCell::new(v8_base_TimeDelta {
                    delta_: self.delta_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(v8_base_TimeDelta {
                    delta_: other.delta_.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for v8_base_TimeDelta {}
impl Clone for v8_base_TimeDelta {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(Self {
            delta_: Rc::new(RefCell::new((*self.delta_.borrow()))),
        }));
        let this: Ptr<v8_base_TimeDelta> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_TimeDelta {
    fn default() -> Self {
        { v8_base_TimeDelta::v8_base_TimeDelta1() }
    }
}
impl ByteRepr for v8_base_TimeDelta {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.delta_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            delta_: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
        }
    }
}
impl v8_base_TimeDelta {
    fn FromDouble(value: f64) -> v8_base_TimeDelta {
        let value: Value<f64> = Rc::new(RefCell::new(value));
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            ({ saturated_cast_117((*value.borrow())) })
        });
    }
}
impl v8_base_TimeDelta {
    pub fn Max() -> v8_base_TimeDelta {
        return v8_base_TimeDelta::v8_base_TimeDelta2({ <i64>::MAX });
    }
}
impl v8_base_TimeDelta {
    pub fn Min() -> v8_base_TimeDelta {
        return v8_base_TimeDelta::v8_base_TimeDelta2({ <i64>::MIN });
    }
}
pub fn Nanoseconds_132(nanoseconds: i64) -> v8_base_TimeDelta {
    let nanoseconds: Value<i64> = Rc::new(RefCell::new(nanoseconds));
    return ({ v8_base_TimeDelta::FromNanoseconds((*nanoseconds.borrow())) });
}
pub fn Microseconds_133(microseconds: i64) -> v8_base_TimeDelta {
    let microseconds: Value<i64> = Rc::new(RefCell::new(microseconds));
    return ({ v8_base_TimeDelta::FromMicroseconds((*microseconds.borrow())) });
}
pub fn Milliseconds_134(milliseconds: i64) -> v8_base_TimeDelta {
    let milliseconds: Value<i64> = Rc::new(RefCell::new(milliseconds));
    return ({ v8_base_TimeDelta::FromMilliseconds((*milliseconds.borrow())) });
}
pub fn Milliseconds_135(milliseconds: f64) -> v8_base_TimeDelta {
    let milliseconds: Value<f64> = Rc::new(RefCell::new(milliseconds));
    return ({ v8_base_TimeDelta::FromMillisecondsD((*milliseconds.borrow())) });
}
pub fn Seconds_136(seconds: i64) -> v8_base_TimeDelta {
    let seconds: Value<i64> = Rc::new(RefCell::new(seconds));
    return ({ v8_base_TimeDelta::FromSeconds((*seconds.borrow())) });
}
pub fn Seconds_137(seconds: f64) -> v8_base_TimeDelta {
    let seconds: Value<f64> = Rc::new(RefCell::new(seconds));
    return ({ v8_base_TimeDelta::FromSecondsD((*seconds.borrow())) });
}
pub fn Minutes_138(minutes: i32) -> v8_base_TimeDelta {
    let minutes: Value<i32> = Rc::new(RefCell::new(minutes));
    return ({ v8_base_TimeDelta::FromMinutes((*minutes.borrow())) });
}
pub fn Hours_139(hours: i32) -> v8_base_TimeDelta {
    let hours: Value<i32> = Rc::new(RefCell::new(hours));
    return ({ v8_base_TimeDelta::FromHours((*hours.borrow())) });
}
pub fn FromDays_140(days: i32) -> v8_base_TimeDelta {
    let days: Value<i32> = Rc::new(RefCell::new(days));
    return ({ v8_base_TimeDelta::FromDays((*days.borrow())) });
}
#[derive()]
pub struct v8_base_Time {}
impl v8_base_Time {
    pub fn v8_base_Time2() -> Self {
        let __this: Value<v8_base_Time> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_Time> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn UnixEpoch() -> v8_base_Time {
        return v8_base_Time::v8_base_Time1({ 0_i64 });
    }
    fn v8_base_Time1(us: i64) -> Self {
        let us: Value<i64> = Rc::new(RefCell::new(us));
        let __this: Value<v8_base_Time> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_Time> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_Time {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Time> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_Time> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_Time {
    fn default() -> Self {
        { v8_base_Time::v8_base_Time2() }
    }
}
impl ByteRepr for v8_base_Time {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn operator_add_141(delta: Ptr<v8_base_TimeDelta>, time: Ptr<v8_base_Time>) -> v8_base_Time {
    return ({
        let _delta: v8_base_TimeDelta = (*delta.upgrade().deref()).clone();
        v8_base_time_internal_TimeBase_v8_base_Time_Impl::operator_add(&time, _delta)
    });
}
#[derive()]
pub struct v8_base_TimeTicks {}
impl v8_base_TimeTicks {
    pub fn v8_base_TimeTicks2() -> Self {
        let __this: Value<v8_base_TimeTicks> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_TimeTicks> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn FromMsTicksForTesting(ticks: i64) -> v8_base_TimeTicks {
        let ticks: Value<i64> = Rc::new(RefCell::new(ticks));
        return v8_base_TimeTicks::v8_base_TimeTicks1({ ((*ticks.borrow()) * 1000) });
    }
    fn v8_base_TimeTicks1(ticks: i64) -> Self {
        let ticks: Value<i64> = Rc::new(RefCell::new(ticks));
        let __this: Value<v8_base_TimeTicks> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_TimeTicks> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_TimeTicks {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_TimeTicks> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_TimeTicks> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_TimeTicks {
    fn default() -> Self {
        { v8_base_TimeTicks::v8_base_TimeTicks2() }
    }
}
impl ByteRepr for v8_base_TimeTicks {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn operator_add_142(
    delta: Ptr<v8_base_TimeDelta>,
    ticks: Ptr<v8_base_TimeTicks>,
) -> v8_base_TimeTicks {
    return ({
        let _delta: v8_base_TimeDelta = (*delta.upgrade().deref()).clone();
        v8_base_time_internal_TimeBase_v8_base_TimeTicks_Impl::operator_add(&ticks, _delta)
    });
}
#[derive()]
pub struct v8_base_ThreadTicks {}
impl v8_base_ThreadTicks {
    pub fn v8_base_ThreadTicks1() -> Self {
        let __this: Value<v8_base_ThreadTicks> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_ThreadTicks> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn WaitUntilInitialized() {}
    fn v8_base_ThreadTicks2(ticks: i64) -> Self {
        let ticks: Value<i64> = Rc::new(RefCell::new(ticks));
        let __this: Value<v8_base_ThreadTicks> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_ThreadTicks> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_ThreadTicks {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_ThreadTicks> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_ThreadTicks> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_ThreadTicks {
    fn default() -> Self {
        { v8_base_ThreadTicks::v8_base_ThreadTicks1() }
    }
}
impl ByteRepr for v8_base_ThreadTicks {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub type cppgc_internal_CollectionType = u8;
pub const cppgc_internal_CollectionType_kMinor: cppgc_internal_CollectionType = 0;
pub const cppgc_internal_CollectionType_kMajor: cppgc_internal_CollectionType = 1;
pub type cppgc_internal_FreeMemoryHandling = u8;
pub const cppgc_internal_FreeMemoryHandling_kRetainMemory: cppgc_internal_FreeMemoryHandling = 0;
pub const cppgc_internal_FreeMemoryHandling_kReleaseMemory: cppgc_internal_FreeMemoryHandling = 1;
pub type cppgc_internal_MarkingConfig_IsForcedGC = u8;
pub const cppgc_internal_MarkingConfig_IsForcedGC_kNotForced:
    cppgc_internal_MarkingConfig_IsForcedGC = 0;
pub const cppgc_internal_MarkingConfig_IsForcedGC_kForced: cppgc_internal_MarkingConfig_IsForcedGC =
    1;
#[derive(Default)]
pub struct cppgc_internal_MarkingConfig {
    pub collection_type: Value<cppgc_internal_CollectionType>,
    pub stack_state: Value<cppgc_EmbedderStackState>,
    pub marking_type: Value<cppgc_Heap_MarkingType>,
    pub is_forced_gc: Value<cppgc_internal_MarkingConfig_IsForcedGC>,
}
impl cppgc_internal_MarkingConfig {
    pub fn Default() -> cppgc_internal_MarkingConfig {
        return cppgc_internal_MarkingConfig {
            collection_type: Rc::new(RefCell::new(cppgc_internal_CollectionType_kMajor)),
            stack_state: Rc::new(RefCell::new(
                cppgc_EmbedderStackState_kMayContainHeapPointers,
            )),
            marking_type: Rc::new(RefCell::new(cppgc_Heap_MarkingType_kIncremental)),
            is_forced_gc: Rc::new(RefCell::new(
                cppgc_internal_MarkingConfig_IsForcedGC_kNotForced,
            )),
        };
    }
}
impl Clone for cppgc_internal_MarkingConfig {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_MarkingConfig> = Rc::new(RefCell::new(Self {
            collection_type: Rc::new(RefCell::new((*self.collection_type.borrow()))),
            stack_state: Rc::new(RefCell::new((*self.stack_state.borrow()))),
            marking_type: Rc::new(RefCell::new((*self.marking_type.borrow()))),
            is_forced_gc: Rc::new(RefCell::new((*self.is_forced_gc.borrow()))),
        }));
        let this: Ptr<cppgc_internal_MarkingConfig> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_MarkingConfig {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.collection_type.borrow()).to_bytes(&mut buf[0..1]);
        (*self.stack_state.borrow()).to_bytes(&mut buf[4..8]);
        (*self.marking_type.borrow()).to_bytes(&mut buf[8..9]);
        (*self.is_forced_gc.borrow()).to_bytes(&mut buf[9..10]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            collection_type: Rc::new(RefCell::new(<cppgc_internal_CollectionType>::from_bytes(
                &buf[0..1],
            ))),
            stack_state: Rc::new(RefCell::new(<cppgc_EmbedderStackState>::from_bytes(
                &buf[4..8],
            ))),
            marking_type: Rc::new(RefCell::new(<cppgc_Heap_MarkingType>::from_bytes(
                &buf[8..9],
            ))),
            is_forced_gc: Rc::new(RefCell::new(
                <cppgc_internal_MarkingConfig_IsForcedGC>::from_bytes(&buf[9..10]),
            )),
        }
    }
}
pub type cppgc_internal_SweepingConfig_CompactableSpaceHandling = i32;
pub const cppgc_internal_SweepingConfig_CompactableSpaceHandling_kSweep:
    cppgc_internal_SweepingConfig_CompactableSpaceHandling = 0;
pub const cppgc_internal_SweepingConfig_CompactableSpaceHandling_kIgnore:
    cppgc_internal_SweepingConfig_CompactableSpaceHandling = 1;
#[derive(Default)]
pub struct cppgc_internal_SweepingConfig {
    pub sweeping_type: Value<cppgc_Heap_SweepingType>,
    pub compactable_space_handling: Value<cppgc_internal_SweepingConfig_CompactableSpaceHandling>,
    pub free_memory_handling: Value<cppgc_internal_FreeMemoryHandling>,
}
impl Clone for cppgc_internal_SweepingConfig {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_SweepingConfig> = Rc::new(RefCell::new(Self {
            sweeping_type: Rc::new(RefCell::new((*self.sweeping_type.borrow()))),
            compactable_space_handling: Rc::new(RefCell::new(
                (*self.compactable_space_handling.borrow()),
            )),
            free_memory_handling: Rc::new(RefCell::new((*self.free_memory_handling.borrow()))),
        }));
        let this: Ptr<cppgc_internal_SweepingConfig> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_SweepingConfig {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.sweeping_type.borrow()).to_bytes(&mut buf[0..1]);
        (*self.compactable_space_handling.borrow()).to_bytes(&mut buf[4..8]);
        (*self.free_memory_handling.borrow()).to_bytes(&mut buf[8..9]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            sweeping_type: Rc::new(RefCell::new(<cppgc_Heap_SweepingType>::from_bytes(
                &buf[0..1],
            ))),
            compactable_space_handling: Rc::new(RefCell::new(
                <cppgc_internal_SweepingConfig_CompactableSpaceHandling>::from_bytes(&buf[4..8]),
            )),
            free_memory_handling: Rc::new(RefCell::new(
                <cppgc_internal_FreeMemoryHandling>::from_bytes(&buf[8..9]),
            )),
        }
    }
}
#[derive(Default)]
pub struct cppgc_internal_GCConfig {
    pub collection_type: Value<cppgc_internal_CollectionType>,
    pub stack_state: Value<cppgc_EmbedderStackState>,
    pub marking_type: Value<cppgc_Heap_MarkingType>,
    pub sweeping_type: Value<cppgc_Heap_SweepingType>,
    pub free_memory_handling: Value<cppgc_internal_FreeMemoryHandling>,
    pub is_forced_gc: Value<cppgc_internal_MarkingConfig_IsForcedGC>,
}
impl cppgc_internal_GCConfig {
    pub fn ConservativeAtomicConfig() -> cppgc_internal_GCConfig {
        return cppgc_internal_GCConfig {
            collection_type: Rc::new(RefCell::new(cppgc_internal_CollectionType_kMajor)),
            stack_state: Rc::new(RefCell::new(
                cppgc_EmbedderStackState_kMayContainHeapPointers,
            )),
            marking_type: Rc::new(RefCell::new(cppgc_Heap_MarkingType_kAtomic)),
            sweeping_type: Rc::new(RefCell::new(cppgc_Heap_SweepingType_kAtomic)),
            free_memory_handling: Rc::new(RefCell::new(
                cppgc_internal_FreeMemoryHandling_kRetainMemory,
            )),
            is_forced_gc: Rc::new(RefCell::new(
                cppgc_internal_MarkingConfig_IsForcedGC_kNotForced,
            )),
        };
    }
    pub fn PreciseAtomicConfig() -> cppgc_internal_GCConfig {
        return cppgc_internal_GCConfig {
            collection_type: Rc::new(RefCell::new(cppgc_internal_CollectionType_kMajor)),
            stack_state: Rc::new(RefCell::new(cppgc_EmbedderStackState_kNoHeapPointers)),
            marking_type: Rc::new(RefCell::new(cppgc_Heap_MarkingType_kAtomic)),
            sweeping_type: Rc::new(RefCell::new(cppgc_Heap_SweepingType_kAtomic)),
            free_memory_handling: Rc::new(RefCell::new(
                cppgc_internal_FreeMemoryHandling_kRetainMemory,
            )),
            is_forced_gc: Rc::new(RefCell::new(
                cppgc_internal_MarkingConfig_IsForcedGC_kNotForced,
            )),
        };
    }
    pub fn ConservativeIncrementalConfig() -> cppgc_internal_GCConfig {
        return cppgc_internal_GCConfig {
            collection_type: Rc::new(RefCell::new(cppgc_internal_CollectionType_kMajor)),
            stack_state: Rc::new(RefCell::new(
                cppgc_EmbedderStackState_kMayContainHeapPointers,
            )),
            marking_type: Rc::new(RefCell::new(cppgc_Heap_MarkingType_kIncremental)),
            sweeping_type: Rc::new(RefCell::new(cppgc_Heap_SweepingType_kAtomic)),
            free_memory_handling: Rc::new(RefCell::new(
                cppgc_internal_FreeMemoryHandling_kRetainMemory,
            )),
            is_forced_gc: Rc::new(RefCell::new(
                cppgc_internal_MarkingConfig_IsForcedGC_kNotForced,
            )),
        };
    }
    pub fn PreciseIncrementalConfig() -> cppgc_internal_GCConfig {
        return cppgc_internal_GCConfig {
            collection_type: Rc::new(RefCell::new(cppgc_internal_CollectionType_kMajor)),
            stack_state: Rc::new(RefCell::new(cppgc_EmbedderStackState_kNoHeapPointers)),
            marking_type: Rc::new(RefCell::new(cppgc_Heap_MarkingType_kIncremental)),
            sweeping_type: Rc::new(RefCell::new(cppgc_Heap_SweepingType_kAtomic)),
            free_memory_handling: Rc::new(RefCell::new(
                cppgc_internal_FreeMemoryHandling_kRetainMemory,
            )),
            is_forced_gc: Rc::new(RefCell::new(
                cppgc_internal_MarkingConfig_IsForcedGC_kNotForced,
            )),
        };
    }
    pub fn PreciseIncrementalMarkingConcurrentSweepingConfig() -> cppgc_internal_GCConfig {
        return cppgc_internal_GCConfig {
            collection_type: Rc::new(RefCell::new(cppgc_internal_CollectionType_kMajor)),
            stack_state: Rc::new(RefCell::new(cppgc_EmbedderStackState_kNoHeapPointers)),
            marking_type: Rc::new(RefCell::new(cppgc_Heap_MarkingType_kIncremental)),
            sweeping_type: Rc::new(RefCell::new(
                cppgc_Heap_SweepingType_kIncrementalAndConcurrent,
            )),
            free_memory_handling: Rc::new(RefCell::new(
                cppgc_internal_FreeMemoryHandling_kRetainMemory,
            )),
            is_forced_gc: Rc::new(RefCell::new(
                cppgc_internal_MarkingConfig_IsForcedGC_kNotForced,
            )),
        };
    }
    pub fn PreciseConcurrentConfig() -> cppgc_internal_GCConfig {
        return cppgc_internal_GCConfig {
            collection_type: Rc::new(RefCell::new(cppgc_internal_CollectionType_kMajor)),
            stack_state: Rc::new(RefCell::new(cppgc_EmbedderStackState_kNoHeapPointers)),
            marking_type: Rc::new(RefCell::new(
                cppgc_Heap_MarkingType_kIncrementalAndConcurrent,
            )),
            sweeping_type: Rc::new(RefCell::new(
                cppgc_Heap_SweepingType_kIncrementalAndConcurrent,
            )),
            free_memory_handling: Rc::new(RefCell::new(
                cppgc_internal_FreeMemoryHandling_kRetainMemory,
            )),
            is_forced_gc: Rc::new(RefCell::new(
                cppgc_internal_MarkingConfig_IsForcedGC_kNotForced,
            )),
        };
    }
    pub fn MinorPreciseAtomicConfig() -> cppgc_internal_GCConfig {
        return cppgc_internal_GCConfig {
            collection_type: Rc::new(RefCell::new(cppgc_internal_CollectionType_kMinor)),
            stack_state: Rc::new(RefCell::new(cppgc_EmbedderStackState_kNoHeapPointers)),
            marking_type: Rc::new(RefCell::new(cppgc_Heap_MarkingType_kAtomic)),
            sweeping_type: Rc::new(RefCell::new(cppgc_Heap_SweepingType_kAtomic)),
            free_memory_handling: Rc::new(RefCell::new(
                cppgc_internal_FreeMemoryHandling_kRetainMemory,
            )),
            is_forced_gc: Rc::new(RefCell::new(
                cppgc_internal_MarkingConfig_IsForcedGC_kNotForced,
            )),
        };
    }
    pub fn MinorConservativeAtomicConfig() -> cppgc_internal_GCConfig {
        return cppgc_internal_GCConfig {
            collection_type: Rc::new(RefCell::new(cppgc_internal_CollectionType_kMinor)),
            stack_state: Rc::new(RefCell::new(
                cppgc_EmbedderStackState_kMayContainHeapPointers,
            )),
            marking_type: Rc::new(RefCell::new(cppgc_Heap_MarkingType_kAtomic)),
            sweeping_type: Rc::new(RefCell::new(cppgc_Heap_SweepingType_kAtomic)),
            free_memory_handling: Rc::new(RefCell::new(
                cppgc_internal_FreeMemoryHandling_kRetainMemory,
            )),
            is_forced_gc: Rc::new(RefCell::new(
                cppgc_internal_MarkingConfig_IsForcedGC_kNotForced,
            )),
        };
    }
}
impl Clone for cppgc_internal_GCConfig {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_GCConfig> = Rc::new(RefCell::new(Self {
            collection_type: Rc::new(RefCell::new((*self.collection_type.borrow()))),
            stack_state: Rc::new(RefCell::new((*self.stack_state.borrow()))),
            marking_type: Rc::new(RefCell::new((*self.marking_type.borrow()))),
            sweeping_type: Rc::new(RefCell::new((*self.sweeping_type.borrow()))),
            free_memory_handling: Rc::new(RefCell::new((*self.free_memory_handling.borrow()))),
            is_forced_gc: Rc::new(RefCell::new((*self.is_forced_gc.borrow()))),
        }));
        let this: Ptr<cppgc_internal_GCConfig> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_GCConfig {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.collection_type.borrow()).to_bytes(&mut buf[0..1]);
        (*self.stack_state.borrow()).to_bytes(&mut buf[4..8]);
        (*self.marking_type.borrow()).to_bytes(&mut buf[8..9]);
        (*self.sweeping_type.borrow()).to_bytes(&mut buf[9..10]);
        (*self.free_memory_handling.borrow()).to_bytes(&mut buf[10..11]);
        (*self.is_forced_gc.borrow()).to_bytes(&mut buf[11..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            collection_type: Rc::new(RefCell::new(<cppgc_internal_CollectionType>::from_bytes(
                &buf[0..1],
            ))),
            stack_state: Rc::new(RefCell::new(<cppgc_EmbedderStackState>::from_bytes(
                &buf[4..8],
            ))),
            marking_type: Rc::new(RefCell::new(<cppgc_Heap_MarkingType>::from_bytes(
                &buf[8..9],
            ))),
            sweeping_type: Rc::new(RefCell::new(<cppgc_Heap_SweepingType>::from_bytes(
                &buf[9..10],
            ))),
            free_memory_handling: Rc::new(RefCell::new(
                <cppgc_internal_FreeMemoryHandling>::from_bytes(&buf[10..11]),
            )),
            is_forced_gc: Rc::new(RefCell::new(
                <cppgc_internal_MarkingConfig_IsForcedGC>::from_bytes(&buf[11..12]),
            )),
        }
    }
}
pub trait cppgc_internal_GarbageCollector {
    fn CollectGarbage(&self, _a0: cppgc_internal_GCConfig);
    fn StartIncrementalGarbageCollection(&self, _a0: cppgc_internal_GCConfig);
    fn RetryAllocate(&self, allocate: v8_base_FunctionRef_bool____) -> bool;
    fn epoch(&self) -> usize;
    fn overridden_stack_state(&self) -> std_optional_cppgc_EmbedderStackState_;
    fn set_override_stack_state(&self, state: cppgc_EmbedderStackState);
    fn clear_overridden_stack_state(&self);
}
#[derive(Default)]
pub struct cppgc_internal_GCInvoker {
    impl__: Value<Option<Value<cppgc_internal_GCInvoker_GCInvokerImpl>>>,
}
impl cppgc_internal_GCInvoker {
    pub fn cppgc_internal_GCInvoker(
        collector: PtrDyn<dyn cppgc_internal_GarbageCollector>,
        platform: PtrDyn<dyn cppgc_Platform>,
        stack_support: cppgc_Heap_StackSupport,
    ) -> Self {
        let collector: Value<PtrDyn<dyn cppgc_internal_GarbageCollector>> =
            Rc::new(RefCell::new(collector));
        let platform: Value<PtrDyn<dyn cppgc_Platform>> = Rc::new(RefCell::new(platform));
        let stack_support: Value<cppgc_Heap_StackSupport> = Rc::new(RefCell::new(stack_support));
        let __this: Value<cppgc_internal_GCInvoker> = Rc::new(RefCell::new(Self {
            impl__: Rc::new(RefCell::new(
                ({
                    make_unique_143(
                        collector.as_pointer(),
                        platform.as_pointer(),
                        stack_support.as_pointer(),
                    )
                }),
            )),
        }));
        let this: Ptr<cppgc_internal_GCInvoker> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl cppgc_internal_GarbageCollector for cppgc_internal_GCInvoker {
    fn CollectGarbage(&self, config: cppgc_internal_GCConfig) {
        let config: Value<cppgc_internal_GCConfig> = Rc::new(RefCell::new(config));
        ({
            (*(*self.impl__.borrow()).as_ref().unwrap().borrow())
                .CollectGarbage((*config.borrow()).clone())
        });
    }
    fn StartIncrementalGarbageCollection(&self, config: cppgc_internal_GCConfig) {
        let config: Value<cppgc_internal_GCConfig> = Rc::new(RefCell::new(config));
        ({
            (*(*self.impl__.borrow()).as_ref().unwrap().borrow())
                .StartIncrementalGarbageCollection((*config.borrow()).clone())
        });
    }
    fn RetryAllocate(&self, allocate: v8_base_FunctionRef_bool____) -> bool {
        let allocate: Value<v8_base_FunctionRef_bool____> = Rc::new(RefCell::new(allocate));
        return ({
            (*(*self.impl__.borrow()).as_ref().unwrap().borrow())
                .RetryAllocate((*allocate.borrow()).clone())
        });
    }
    fn epoch(&self) -> usize {
        return ({
            (*(*(*self.impl__.borrow()).as_ref().unwrap().borrow())
                .upgrade()
                .deref())
            .epoch()
        });
    }
    fn overridden_stack_state(&self) -> std_optional_cppgc_EmbedderStackState_ {
        return ({
            (*(*(*self.impl__.borrow()).as_ref().unwrap().borrow())
                .upgrade()
                .deref())
            .overridden_stack_state()
        });
    }
    fn set_override_stack_state(&self, state: cppgc_EmbedderStackState) {
        let state: Value<cppgc_EmbedderStackState> = Rc::new(RefCell::new(state));
        ({
            (*(*self.impl__.borrow()).as_ref().unwrap().borrow())
                .set_override_stack_state((*state.borrow()))
        });
    }
    fn clear_overridden_stack_state(&self) {
        ({ (*(*self.impl__.borrow()).as_ref().unwrap().borrow()).clear_overridden_stack_state() });
    }
}
impl ByteRepr for cppgc_internal_GCInvoker {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.impl__.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            impl__: Rc::new(RefCell::new(<Option<
                Value<cppgc_internal_GCInvoker_GCInvokerImpl>,
            >>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Default)]
pub struct cppgc_internal_SingleThreadedHandle_NonEmptyTag {}
impl Clone for cppgc_internal_SingleThreadedHandle_NonEmptyTag {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_SingleThreadedHandle_NonEmptyTag> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<cppgc_internal_SingleThreadedHandle_NonEmptyTag> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_SingleThreadedHandle_NonEmptyTag {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct cppgc_internal_SingleThreadedHandle {
    is_cancelled_: Value<std_shared_ptr_bool_>,
}
impl cppgc_internal_SingleThreadedHandle {
    pub fn cppgc_internal_SingleThreadedHandle1(
        _a0: cppgc_internal_SingleThreadedHandle_NonEmptyTag,
    ) -> Self {
        let _a0: Value<cppgc_internal_SingleThreadedHandle_NonEmptyTag> =
            Rc::new(RefCell::new(_a0));
        let __this: Value<cppgc_internal_SingleThreadedHandle> = Rc::new(RefCell::new(Self {
            is_cancelled_: Rc::new(RefCell::new(
                ({
                    let ___args: Value<bool> = Rc::new(RefCell::new(false));
                    make_shared_144(___args.as_pointer())
                }),
            )),
        }));
        let this: Ptr<cppgc_internal_SingleThreadedHandle> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for cppgc_internal_SingleThreadedHandle {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_SingleThreadedHandle> = Rc::new(RefCell::new(Self {
            is_cancelled_: Rc::new(RefCell::new((*self.is_cancelled_.borrow()).clone())),
        }));
        let this: Ptr<cppgc_internal_SingleThreadedHandle> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_SingleThreadedHandle {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.is_cancelled_.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            is_cancelled_: Rc::new(RefCell::new(<std_shared_ptr_bool_>::from_bytes(
                &buf[0..16],
            ))),
        }
    }
}
#[derive(Default)]
struct cppgc_internal_GCInvoker_GCInvokerImpl_GCTask {
    collector_: Value<PtrDyn<dyn cppgc_internal_GarbageCollector>>,
    config_: Value<cppgc_internal_GCConfig>,
    handle_: Value<cppgc_internal_SingleThreadedHandle>,
    saved_epoch_: Value<usize>,
}
impl cppgc_internal_GCInvoker_GCInvokerImpl_GCTask {
    pub fn Post(
        collector: PtrDyn<dyn cppgc_internal_GarbageCollector>,
        runner: PtrDyn<dyn v8_TaskRunner>,
        config: cppgc_internal_GCConfig,
    ) -> cppgc_internal_SingleThreadedHandle {
        let collector: Value<PtrDyn<dyn cppgc_internal_GarbageCollector>> =
            Rc::new(RefCell::new(collector));
        let runner: Value<PtrDyn<dyn v8_TaskRunner>> = Rc::new(RefCell::new(runner));
        let config: Value<cppgc_internal_GCConfig> = Rc::new(RefCell::new(config));
        let task: Value<Option<Value<cppgc_internal_GCInvoker_GCInvokerImpl_GCTask>>> = Rc::new(
            RefCell::new(({ make_unique_145(collector.as_pointer(), config.as_pointer()) })),
        );
        let handle: Value<cppgc_internal_SingleThreadedHandle> = Rc::new(RefCell::new(
            ({
                cppgc_internal_GCInvoker_GCInvokerImpl_GCTaskImpl::GetHandle(
                    &(*task.borrow()).as_pointer(),
                )
            }),
        ));
        ({
            v8_TaskRunnerImpl::PostNonNestableTask(
                &(*runner.borrow()),
                (*task.borrow_mut()).clone(),
                None,
            )
        });
        return (*handle.borrow()).clone();
    }
    pub fn cppgc_internal_GCInvoker_GCInvokerImpl_GCTask(
        collector: PtrDyn<dyn cppgc_internal_GarbageCollector>,
        config: cppgc_internal_GCConfig,
    ) -> Self {
        let collector: Value<PtrDyn<dyn cppgc_internal_GarbageCollector>> =
            Rc::new(RefCell::new(collector));
        let config: Value<cppgc_internal_GCConfig> = Rc::new(RefCell::new(config));
        let __this: Value<cppgc_internal_GCInvoker_GCInvokerImpl_GCTask> =
            Rc::new(RefCell::new(Self {
                collector_: Rc::new(RefCell::new((*collector.borrow()).clone())),
                config_: Rc::new(RefCell::new((*config.borrow()).clone())),
                handle_: Rc::new(RefCell::new(
                    cppgc_internal_SingleThreadedHandle::cppgc_internal_SingleThreadedHandle1({
                        cppgc_internal_SingleThreadedHandle_NonEmptyTag {}
                    }),
                )),
                saved_epoch_: Rc::new(RefCell::new(
                    ({ (*(*collector.borrow()).upgrade().deref()).epoch() }),
                )),
            }));
        let this: Ptr<cppgc_internal_GCInvoker_GCInvokerImpl_GCTask> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl v8_Task for cppgc_internal_GCInvoker_GCInvokerImpl_GCTask {
    fn Run(&self) {
        if ({ cppgc_internal_SingleThreadedHandleImpl::IsCanceled(&self.handle_.as_pointer()) })
            || ({
                let _lhs = ({ (*(*self.collector_.borrow()).upgrade().deref()).epoch() });
                _lhs != (*self.saved_epoch_.borrow())
            })
        {
            return;
        }
        ({
            (*(*self.collector_.borrow()).upgrade().deref())
                .set_override_stack_state(cppgc_EmbedderStackState_kNoHeapPointers)
        });
        ({
            let _arg0: cppgc_internal_GCConfig = (*self.config_.borrow()).clone();
            (*(*self.collector_.borrow()).upgrade().deref()).CollectGarbage(_arg0)
        });
        ({ (*(*self.collector_.borrow()).upgrade().deref()).clear_overridden_stack_state() });
        ({ cppgc_internal_SingleThreadedHandleImpl::Cancel(&self.handle_.as_pointer()) });
    }
}
impl Clone for cppgc_internal_GCInvoker_GCInvokerImpl_GCTask {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_GCInvoker_GCInvokerImpl_GCTask> =
            Rc::new(RefCell::new(Self {
                collector_: Rc::new(RefCell::new((*self.collector_.borrow()).clone())),
                config_: Rc::new(RefCell::new((*self.config_.borrow()).clone())),
                handle_: Rc::new(RefCell::new((*self.handle_.borrow()).clone())),
                saved_epoch_: Rc::new(RefCell::new((*self.saved_epoch_.borrow()))),
            }));
        let this: Ptr<cppgc_internal_GCInvoker_GCInvokerImpl_GCTask> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_GCInvoker_GCInvokerImpl_GCTask {
    fn byte_size() -> usize {
        56
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.collector_.borrow()).to_bytes(&mut buf[8..16]);
        (*self.config_.borrow()).to_bytes(&mut buf[16..28]);
        (*self.handle_.borrow()).to_bytes(&mut buf[32..48]);
        (*self.saved_epoch_.borrow()).to_bytes(&mut buf[48..56]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            collector_: Rc::new(RefCell::new(
                <PtrDyn<dyn cppgc_internal_GarbageCollector>>::from_bytes(&buf[8..16]),
            )),
            config_: Rc::new(RefCell::new(<cppgc_internal_GCConfig>::from_bytes(
                &buf[16..28],
            ))),
            handle_: Rc::new(RefCell::new(
                <cppgc_internal_SingleThreadedHandle>::from_bytes(&buf[32..48]),
            )),
            saved_epoch_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[48..56]))),
        }
    }
}
#[derive(Default)]
struct cppgc_internal_GCInvoker_GCInvokerImpl {
    collector_: Value<PtrDyn<dyn cppgc_internal_GarbageCollector>>,
    platform_: Value<PtrDyn<dyn cppgc_Platform>>,
    stack_support_: Value<cppgc_Heap_StackSupport>,
    gc_task_handle_: Value<cppgc_internal_SingleThreadedHandle>,
}
impl cppgc_internal_GCInvoker_GCInvokerImpl {
    pub fn cppgc_internal_GCInvoker_GCInvokerImpl(
        collector: PtrDyn<dyn cppgc_internal_GarbageCollector>,
        platform: PtrDyn<dyn cppgc_Platform>,
        stack_support: cppgc_Heap_StackSupport,
    ) -> Self {
        let collector: Value<PtrDyn<dyn cppgc_internal_GarbageCollector>> =
            Rc::new(RefCell::new(collector));
        let platform: Value<PtrDyn<dyn cppgc_Platform>> = Rc::new(RefCell::new(platform));
        let stack_support: Value<cppgc_Heap_StackSupport> = Rc::new(RefCell::new(stack_support));
        let __this: Value<cppgc_internal_GCInvoker_GCInvokerImpl> = Rc::new(RefCell::new(Self {
            collector_: Rc::new(RefCell::new((*collector.borrow()).clone())),
            platform_: Rc::new(RefCell::new((*platform.borrow()).clone())),
            stack_support_: Rc::new(RefCell::new((*stack_support.borrow()))),
            gc_task_handle_: Rc::new(RefCell::new(
                <cppgc_internal_SingleThreadedHandle>::default(),
            )),
        }));
        let this: Ptr<cppgc_internal_GCInvoker_GCInvokerImpl> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl cppgc_internal_GarbageCollector for cppgc_internal_GCInvoker_GCInvokerImpl {
    fn CollectGarbage(&self, config: cppgc_internal_GCConfig) {
        let config: Value<cppgc_internal_GCConfig> = Rc::new(RefCell::new(config));
        (&(0));
        if ((*(*config.borrow()).stack_state.borrow()) == cppgc_EmbedderStackState_kNoHeapPointers)
            || ((*self.stack_support_.borrow())
                == cppgc_Heap_StackSupport_kSupportsConservativeStackScan)
        {
            ({
                (*(*self.collector_.borrow()).upgrade().deref())
                    .CollectGarbage((*config.borrow()).clone())
            });
        } else if ({
            ({ (*(*self.platform_.borrow()).upgrade().deref()).GetForegroundTaskRunner() })()
        }) && ({
            (*(({ (*(*self.platform_.borrow()).upgrade().deref()).GetForegroundTaskRunner() })
                .read())
            .upgrade()
            .deref())
            .NonNestableTasksEnabled()
        }) {
            if !({
                cppgc_internal_SingleThreadedHandleImpl::operator__Bool(
                    &self.gc_task_handle_.as_pointer(),
                )
            }) {
                (*(*config.borrow()).stack_state.borrow_mut()) =
                    cppgc_EmbedderStackState_kNoHeapPointers;
                (&(0));
                (*self.gc_task_handle_.borrow_mut()) = ({
                    let _collector: PtrDyn<dyn cppgc_internal_GarbageCollector> =
                        (*self.collector_.borrow()).clone();
                    let _runner: PtrDyn<dyn v8_TaskRunner> = ({
                        ({
                            (*(*self.platform_.borrow()).upgrade().deref())
                                .GetForegroundTaskRunner()
                        })
                        .get()
                    });
                    cppgc_internal_GCInvoker_GCInvokerImpl_GCTask::Post(
                        _collector,
                        _runner,
                        (*config.borrow()).clone(),
                    )
                });
            }
        }
    }
    fn StartIncrementalGarbageCollection(&self, config: cppgc_internal_GCConfig) {
        let config: Value<cppgc_internal_GCConfig> = Rc::new(RefCell::new(config));
        (&(0));
        if ((*self.stack_support_.borrow())
            != cppgc_Heap_StackSupport_kSupportsConservativeStackScan)
            && ((!({
                ({ (*(*self.platform_.borrow()).upgrade().deref()).GetForegroundTaskRunner() })()
            })) || (!({
                (*(({ (*(*self.platform_.borrow()).upgrade().deref()).GetForegroundTaskRunner() })
                    .read())
                .upgrade()
                .deref())
                .NonNestableTasksEnabled()
            })))
        {
            return;
        }
        ({
            (*(*self.collector_.borrow()).upgrade().deref())
                .StartIncrementalGarbageCollection((*config.borrow()).clone())
        });
    }
    fn RetryAllocate(&self, allocate: v8_base_FunctionRef_bool____) -> bool {
        let allocate: Value<v8_base_FunctionRef_bool____> = Rc::new(RefCell::new(allocate));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 2) {
            ({
                self.CollectGarbage(cppgc_internal_GCConfig {
                    collection_type: Rc::new(RefCell::new(cppgc_internal_CollectionType_kMajor)),
                    stack_state: Rc::new(RefCell::new(
                        cppgc_EmbedderStackState_kMayContainHeapPointers,
                    )),
                    marking_type: Rc::new(RefCell::new(cppgc_Heap_MarkingType_kAtomic)),
                    sweeping_type: Rc::new(RefCell::new(
                        cppgc_Heap_SweepingType_kIncrementalAndConcurrent,
                    )),
                    free_memory_handling: Rc::new(RefCell::new(
                        cppgc_internal_FreeMemoryHandling_kReleaseMemory,
                    )),
                    is_forced_gc: Rc::new(RefCell::new(
                        cppgc_internal_MarkingConfig_IsForcedGC_kNotForced,
                    )),
                })
            });
            let result: Value<bool> = Rc::new(RefCell::new(
                ({ v8_base_FunctionRef_bool____Impl::operator_call(&allocate.as_pointer()) }),
            ));
            if (*result.borrow()) {
                return true;
            }
            (*i.borrow_mut()).postfix_inc();
        }
        return false;
    }
    fn epoch(&self) -> usize {
        return ({ (*(*self.collector_.borrow()).upgrade().deref()).epoch() });
    }
    fn overridden_stack_state(&self) -> std_optional_cppgc_EmbedderStackState_ {
        return ({ (*(*self.collector_.borrow()).upgrade().deref()).overridden_stack_state() });
    }
    fn set_override_stack_state(&self, state: cppgc_EmbedderStackState) {
        let state: Value<cppgc_EmbedderStackState> = Rc::new(RefCell::new(state));
        ({
            (*(*self.collector_.borrow()).upgrade().deref())
                .set_override_stack_state((*state.borrow()))
        });
    }
    fn clear_overridden_stack_state(&self) {
        ({ (*(*self.collector_.borrow()).upgrade().deref()).clear_overridden_stack_state() });
    }
}
impl ByteRepr for cppgc_internal_GCInvoker_GCInvokerImpl {
    fn byte_size() -> usize {
        48
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.collector_.borrow()).to_bytes(&mut buf[8..16]);
        (*self.platform_.borrow()).to_bytes(&mut buf[16..24]);
        (*self.stack_support_.borrow()).to_bytes(&mut buf[24..25]);
        (*self.gc_task_handle_.borrow()).to_bytes(&mut buf[32..48]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            collector_: Rc::new(RefCell::new(
                <PtrDyn<dyn cppgc_internal_GarbageCollector>>::from_bytes(&buf[8..16]),
            )),
            platform_: Rc::new(RefCell::new(<PtrDyn<dyn cppgc_Platform>>::from_bytes(
                &buf[16..24],
            ))),
            stack_support_: Rc::new(RefCell::new(<cppgc_Heap_StackSupport>::from_bytes(
                &buf[24..25],
            ))),
            gc_task_handle_: Rc::new(RefCell::new(
                <cppgc_internal_SingleThreadedHandle>::from_bytes(&buf[32..48]),
            )),
        }
    }
}
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_PageAllocator_AllocationHint;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_Isolate;
pub trait absl_FunctionRef_bool____Impl {
    fn operator_call(&self) -> bool;
}
impl absl_FunctionRef_bool____Impl for Ptr<absl_FunctionRef_bool____> {
    fn operator_call(&self) -> bool {
        return ({
            let _arg0: absl_functional_internal_VoidPtr =
                (*(*(*self).upgrade().deref()).ptr_.borrow()).clone();
            (*(*(*(*self).upgrade().deref()).invoker_.borrow()))(_arg0)
        });
    }
}
pub trait absl_internal_any_invocable_TrivialDeleterImpl {
    fn operator_call(&self, target: AnyPtr);
}
impl absl_internal_any_invocable_TrivialDeleterImpl
    for Ptr<absl_internal_any_invocable_TrivialDeleter>
{
    fn operator_call(&self, target: AnyPtr) {
        let target: Value<AnyPtr> = Rc::new(RefCell::new(target));
        free_refcount((*target.borrow()).clone());
    }
}
pub trait cppgc_StackStartMarkerImpl {
    fn stack_start(&self) -> AnyPtr;
}
impl cppgc_StackStartMarkerImpl for Ptr<cppgc_StackStartMarker> {
    fn stack_start(&self) -> AnyPtr {
        return (*(*(*self).upgrade().deref()).stack_start_.borrow()).clone();
    }
}
pub trait cppgc_internal_GCInvoker_GCInvokerImplImpl {
    fn destructor(&self);
}
impl cppgc_internal_GCInvoker_GCInvokerImplImpl for Ptr<cppgc_internal_GCInvoker_GCInvokerImpl> {
    fn destructor(&self) {
        if ({
            cppgc_internal_SingleThreadedHandleImpl::operator__Bool(
                &(*(*self).upgrade().deref()).gc_task_handle_.as_pointer(),
            )
        }) {
            ({
                cppgc_internal_SingleThreadedHandleImpl::Cancel(
                    &(*(*self).upgrade().deref()).gc_task_handle_.as_pointer(),
                )
            });
        }
    }
}
pub trait cppgc_internal_GCInvoker_GCInvokerImpl_GCTaskImpl {
    fn GetHandle(&self) -> cppgc_internal_SingleThreadedHandle;
}
impl cppgc_internal_GCInvoker_GCInvokerImpl_GCTaskImpl
    for Ptr<cppgc_internal_GCInvoker_GCInvokerImpl_GCTask>
{
    fn GetHandle(&self) -> cppgc_internal_SingleThreadedHandle {
        return (*(*(*self).upgrade().deref()).handle_.borrow()).clone();
    }
}
pub trait cppgc_internal_SingleThreadedHandleImpl {
    fn Cancel(&self);
    fn CancelIfNonEmpty(&self);
    fn IsCanceled(&self) -> bool;
    fn operator__Bool(&self) -> bool;
}
impl cppgc_internal_SingleThreadedHandleImpl for Ptr<cppgc_internal_SingleThreadedHandle> {
    fn Cancel(&self) {
        (&(0));
        (*(*(*self).upgrade().deref()).is_cancelled_.borrow_mut()).write(true);
    }
    fn CancelIfNonEmpty(&self) {
        if ({ (*(*(*self).upgrade().deref()).is_cancelled_.borrow())() }) {
            (*(*(*self).upgrade().deref()).is_cancelled_.borrow_mut()).write(true);
        }
    }
    fn IsCanceled(&self) -> bool {
        (&(0));
        return ((*(*(*self).upgrade().deref()).is_cancelled_.borrow()).read());
    }
    fn operator__Bool(&self) -> bool {
        return (!({ (*(*(*self).upgrade().deref()).is_cancelled_.borrow()).get() }).is_null())
            && (!(({ (*(*(*self).upgrade().deref()).is_cancelled_.borrow()).get() }).read()));
    }
}
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
pub trait v8_base_FunctionRef_bool____Impl {
    fn operator_call(&self) -> bool;
}
impl v8_base_FunctionRef_bool____Impl for Ptr<v8_base_FunctionRef_bool____> {
    fn operator_call(&self) -> bool {
        return ({
            absl_FunctionRef_bool____Impl::operator_call(
                &(*(*self).upgrade().deref()).wrapped_func_ref_.as_pointer(),
            )
        });
    }
}
pub trait v8_base_TimeConstantsImpl {
    fn operator_cmp(&self, _a0: Ptr<v8_base_TimeConstants>) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<v8_base_TimeConstants>) -> bool;
}
impl v8_base_TimeConstantsImpl for Ptr<v8_base_TimeConstants> {
    fn operator_cmp(&self, _a0: Ptr<v8_base_TimeConstants>) -> std::cmp::Ordering {
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<v8_base_TimeConstants>) -> bool {
        return true;
    }
}
pub trait v8_base_TimeDeltaImpl {
    fn IsZero(&self) -> bool;
    fn IsMax(&self) -> bool;
    fn IsMin(&self) -> bool;
    fn operator_add(&self, other: Ptr<v8_base_TimeDelta>) -> v8_base_TimeDelta;
    fn operator_sub_pconstv8_base_TimeDelta_const(
        &self,
        other: Ptr<v8_base_TimeDelta>,
    ) -> v8_base_TimeDelta;
    fn operator_add_assign(&self, other: Ptr<v8_base_TimeDelta>) -> Ptr<v8_base_TimeDelta>;
    fn operator_sub_assign(&self, other: Ptr<v8_base_TimeDelta>) -> Ptr<v8_base_TimeDelta>;
    fn operator_neg_const(&self) -> v8_base_TimeDelta;
    fn TimesOf(&self, other: Ptr<v8_base_TimeDelta>) -> f64;
    fn PercentOf(&self, other: Ptr<v8_base_TimeDelta>) -> f64;
    fn operator_mul(&self, a: i64) -> v8_base_TimeDelta;
    fn operator_div_i64_const(&self, a: i64) -> v8_base_TimeDelta;
    fn operator_mul_assign(&self, a: i64) -> Ptr<v8_base_TimeDelta>;
    fn operator_div_assign(&self, a: i64) -> Ptr<v8_base_TimeDelta>;
    fn operator_div_pconstv8_base_TimeDelta_const(&self, other: Ptr<v8_base_TimeDelta>) -> i64;
    fn operator_cmp(&self, _a0: Ptr<v8_base_TimeDelta>) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<v8_base_TimeDelta>) -> bool;
}
impl v8_base_TimeDeltaImpl for Ptr<v8_base_TimeDelta> {
    fn IsZero(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).delta_.borrow()) == 0_i64);
    }
    fn IsMax(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).delta_.borrow()) == <i64>::MAX);
    }
    fn IsMin(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).delta_.borrow()) == <i64>::MIN);
    }
    fn operator_add(&self, other: Ptr<v8_base_TimeDelta>) -> v8_base_TimeDelta {
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            {
                let _lhs = (*(*(*self).upgrade().deref()).delta_.borrow());
                _lhs + (*(*other.upgrade().deref()).delta_.borrow())
            }
        });
    }
    fn operator_sub_pconstv8_base_TimeDelta_const(
        &self,
        other: Ptr<v8_base_TimeDelta>,
    ) -> v8_base_TimeDelta {
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            {
                let _lhs = (*(*(*self).upgrade().deref()).delta_.borrow());
                _lhs - (*(*other.upgrade().deref()).delta_.borrow())
            }
        });
    }
    fn operator_add_assign(&self, other: Ptr<v8_base_TimeDelta>) -> Ptr<v8_base_TimeDelta> {
        let __rhs = (*(*other.upgrade().deref()).delta_.borrow());
        (*(*(*self).upgrade().deref()).delta_.borrow_mut()) += __rhs;
        return (*self).clone();
    }
    fn operator_sub_assign(&self, other: Ptr<v8_base_TimeDelta>) -> Ptr<v8_base_TimeDelta> {
        let __rhs = (*(*other.upgrade().deref()).delta_.borrow());
        (*(*(*self).upgrade().deref()).delta_.borrow_mut()) -= __rhs;
        return (*self).clone();
    }
    fn operator_neg_const(&self) -> v8_base_TimeDelta {
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            -(*(*(*self).upgrade().deref()).delta_.borrow())
        });
    }
    fn TimesOf(&self, other: Ptr<v8_base_TimeDelta>) -> f64 {
        return {
            let _lhs = ((*(*(*self).upgrade().deref()).delta_.borrow()) as f64);
            _lhs / ((*(*other.upgrade().deref()).delta_.borrow()) as f64)
        };
    }
    fn PercentOf(&self, other: Ptr<v8_base_TimeDelta>) -> f64 {
        return (({ v8_base_TimeDeltaImpl::TimesOf(self, (other).clone()) }) * 1.0E+2);
    }
    fn operator_mul(&self, a: i64) -> v8_base_TimeDelta {
        let a: Value<i64> = Rc::new(RefCell::new(a));
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            ((*(*(*self).upgrade().deref()).delta_.borrow()) * (*a.borrow()))
        });
    }
    fn operator_div_i64_const(&self, a: i64) -> v8_base_TimeDelta {
        let a: Value<i64> = Rc::new(RefCell::new(a));
        return v8_base_TimeDelta::v8_base_TimeDelta2({
            ((*(*(*self).upgrade().deref()).delta_.borrow()) / (*a.borrow()))
        });
    }
    fn operator_mul_assign(&self, a: i64) -> Ptr<v8_base_TimeDelta> {
        let a: Value<i64> = Rc::new(RefCell::new(a));
        (*(*(*self).upgrade().deref()).delta_.borrow_mut()) *= (*a.borrow());
        return (*self).clone();
    }
    fn operator_div_assign(&self, a: i64) -> Ptr<v8_base_TimeDelta> {
        let a: Value<i64> = Rc::new(RefCell::new(a));
        (*(*(*self).upgrade().deref()).delta_.borrow_mut()) /= (*a.borrow());
        return (*self).clone();
    }
    fn operator_div_pconstv8_base_TimeDelta_const(&self, other: Ptr<v8_base_TimeDelta>) -> i64 {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).delta_.borrow());
            _lhs / (*(*other.upgrade().deref()).delta_.borrow())
        };
    }
    fn operator_cmp(&self, _a0: Ptr<v8_base_TimeDelta>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).delta_.borrow())
                    .cmp(&(*(*_a0.upgrade().deref()).delta_.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<v8_base_TimeDelta>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).delta_.borrow());
            _lhs == (*(*_a0.upgrade().deref()).delta_.borrow())
        };
    }
}
pub trait v8_base_internal_RangeCheckImpl {
    fn operator_eq(&self, rhs: Ptr<v8_base_internal_RangeCheck>) -> bool;
    fn IsValid(&self) -> bool;
    fn IsInvalid(&self) -> bool;
    fn IsOverflow(&self) -> bool;
    fn IsUnderflow(&self) -> bool;
    fn IsOverflowFlagSet(&self) -> bool;
    fn IsUnderflowFlagSet(&self) -> bool;
}
impl v8_base_internal_RangeCheckImpl for Ptr<v8_base_internal_RangeCheck> {
    fn operator_eq(&self, rhs: Ptr<v8_base_internal_RangeCheck>) -> bool {
        return ({
            let _lhs = ((*(*(*self).upgrade().deref()).is_underflow_.borrow()) as i32);
            _lhs == ((*(*rhs.upgrade().deref()).is_underflow_.borrow()) as i32)
        }) && ({
            let _lhs = ((*(*(*self).upgrade().deref()).is_overflow_.borrow()) as i32);
            _lhs == ((*(*rhs.upgrade().deref()).is_overflow_.borrow()) as i32)
        });
    }
    fn IsValid(&self) -> bool {
        return (!(*(*(*self).upgrade().deref()).is_overflow_.borrow()))
            && (!(*(*(*self).upgrade().deref()).is_underflow_.borrow()));
    }
    fn IsInvalid(&self) -> bool {
        return (*(*(*self).upgrade().deref()).is_overflow_.borrow())
            && (*(*(*self).upgrade().deref()).is_underflow_.borrow());
    }
    fn IsOverflow(&self) -> bool {
        return (*(*(*self).upgrade().deref()).is_overflow_.borrow())
            && (!(*(*(*self).upgrade().deref()).is_underflow_.borrow()));
    }
    fn IsUnderflow(&self) -> bool {
        return (!(*(*(*self).upgrade().deref()).is_overflow_.borrow()))
            && (*(*(*self).upgrade().deref()).is_underflow_.borrow());
    }
    fn IsOverflowFlagSet(&self) -> bool {
        return (*(*(*self).upgrade().deref()).is_overflow_.borrow());
    }
    fn IsUnderflowFlagSet(&self) -> bool {
        return (*(*(*self).upgrade().deref()).is_underflow_.borrow());
    }
}
pub trait v8_base_time_internal_TimeBase_v8_base_ThreadTicks_Impl {
    fn operator_cmp(
        &self,
        _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_>,
    ) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_>) -> bool;
}
impl v8_base_time_internal_TimeBase_v8_base_ThreadTicks_Impl
    for Ptr<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_>
{
    fn operator_cmp(
        &self,
        _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_>,
    ) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                ({
                    let _arg0: Ptr<v8_base_TimeConstants> = (_a0).clone();
                    v8_base_TimeConstantsImpl::operator_cmp(&(*self), _arg0)
                }),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).us_.borrow())
                    .cmp(&(*(*_a0.upgrade().deref()).us_.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_ThreadTicks_>) -> bool {
        return ({
            let _arg0: Ptr<v8_base_TimeConstants> = (_a0).clone();
            v8_base_TimeConstantsImpl::operator_eq(&(*self), _arg0)
        }) && ({
            let _lhs = (*(*(*self).upgrade().deref()).us_.borrow());
            _lhs == (*(*_a0.upgrade().deref()).us_.borrow())
        });
    }
}
pub trait v8_base_time_internal_TimeBase_v8_base_TimeTicks_Impl {
    fn operator_add(&self, delta: v8_base_TimeDelta) -> v8_base_TimeTicks;
    fn operator_cmp(
        &self,
        _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_TimeTicks_>,
    ) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_TimeTicks_>) -> bool;
}
impl v8_base_time_internal_TimeBase_v8_base_TimeTicks_Impl
    for Ptr<v8_base_time_internal_TimeBase_v8_base_TimeTicks_>
{
    fn operator_add(&self, delta: v8_base_TimeDelta) -> v8_base_TimeTicks {
        let delta: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(delta));
        return v8_base_TimeTicks::v8_base_TimeTicks1({
            ({
                SignedSaturatedAdd64_119(
                    (*(*delta.borrow()).delta_.borrow()),
                    (*(*(*self).upgrade().deref()).us_.borrow()),
                )
            })
        });
    }
    fn operator_cmp(
        &self,
        _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_TimeTicks_>,
    ) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                ({
                    let _arg0: Ptr<v8_base_TimeConstants> = (_a0).clone();
                    v8_base_TimeConstantsImpl::operator_cmp(&(*self), _arg0)
                }),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).us_.borrow())
                    .cmp(&(*(*_a0.upgrade().deref()).us_.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_TimeTicks_>) -> bool {
        return ({
            let _arg0: Ptr<v8_base_TimeConstants> = (_a0).clone();
            v8_base_TimeConstantsImpl::operator_eq(&(*self), _arg0)
        }) && ({
            let _lhs = (*(*(*self).upgrade().deref()).us_.borrow());
            _lhs == (*(*_a0.upgrade().deref()).us_.borrow())
        });
    }
}
pub trait v8_base_time_internal_TimeBase_v8_base_Time_Impl {
    fn operator_add(&self, delta: v8_base_TimeDelta) -> v8_base_Time;
    fn operator_cmp(
        &self,
        _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_Time_>,
    ) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_Time_>) -> bool;
}
impl v8_base_time_internal_TimeBase_v8_base_Time_Impl
    for Ptr<v8_base_time_internal_TimeBase_v8_base_Time_>
{
    fn operator_add(&self, delta: v8_base_TimeDelta) -> v8_base_Time {
        let delta: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(delta));
        return v8_base_Time::v8_base_Time1({
            ({
                SignedSaturatedAdd64_119(
                    (*(*delta.borrow()).delta_.borrow()),
                    (*(*(*self).upgrade().deref()).us_.borrow()),
                )
            })
        });
    }
    fn operator_cmp(
        &self,
        _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_Time_>,
    ) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                ({
                    let _arg0: Ptr<v8_base_TimeConstants> = (_a0).clone();
                    v8_base_TimeConstantsImpl::operator_cmp(&(*self), _arg0)
                }),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).us_.borrow())
                    .cmp(&(*(*_a0.upgrade().deref()).us_.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<v8_base_time_internal_TimeBase_v8_base_Time_>) -> bool {
        return ({
            let _arg0: Ptr<v8_base_TimeConstants> = (_a0).clone();
            v8_base_TimeConstantsImpl::operator_eq(&(*self), _arg0)
        }) && ({
            let _lhs = (*(*(*self).upgrade().deref()).us_.borrow());
            _lhs == (*(*_a0.upgrade().deref()).us_.borrow())
        });
    }
}
