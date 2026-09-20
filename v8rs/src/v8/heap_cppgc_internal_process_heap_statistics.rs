use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!();
thread_local!();
#[derive(Default)]
pub struct cppgc_ProcessHeapStatistics {}
impl cppgc_ProcessHeapStatistics {
    pub fn TotalAllocatedObjectSize() -> usize {
        return (({
            (*total_allocated_object_size__1.with(Value::clone).borrow()).load_const(Some(0))
        }) as usize);
    }
    pub fn TotalAllocatedSpace() -> usize {
        return (({ (*total_allocated_space__0.with(Value::clone).borrow()).load_const(Some(0)) })
            as usize);
    }
}
impl Clone for cppgc_ProcessHeapStatistics {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_ProcessHeapStatistics> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<cppgc_ProcessHeapStatistics> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_ProcessHeapStatistics {
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
pub fn SharedMemoryHandleFromMachMemoryEntry_2(handle: u32) -> v8_SharedMemoryHandle {
    let handle: Value<u32> = Rc::new(RefCell::new(handle));
    return ({ v8_SharedMemoryHandle::FromPlatformHandle((*handle.borrow())) });
}
pub fn MachMemoryEntryFromSharedMemoryHandle_3(handle: v8_SharedMemoryHandle) -> u32 {
    let handle: Value<v8_SharedMemoryHandle> = Rc::new(RefCell::new(handle));
    return ({ v8_SharedMemoryHandleImpl::GetPlatformHandle(&handle.as_pointer()) });
}
thread_local!(
    pub static kInvalidSharedMemoryHandle_4: Value<std_optional_v8_SharedMemoryHandle_> =
        Rc::new(RefCell::new(
            std_optional_v8_SharedMemoryHandle_::std_optional_v8_SharedMemoryHandle_1({
                (*nullopt_5.with(Value::clone).borrow()).clone()
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
pub fn operator_bitor_6(lhs: v8_PagePermissions, rhs: v8_PagePermissions) -> v8_PagePermissions {
    let lhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(lhs));
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as i32) | ((*rhs.borrow()) as i32)) as v8_PagePermissions);
}
pub fn operator_bitand_7(lhs: v8_PagePermissions, rhs: v8_PagePermissions) -> v8_PagePermissions {
    let lhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(lhs));
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as i32) & ((*rhs.borrow()) as i32)) as v8_PagePermissions);
}
pub fn operator_bitor_assign_8(
    lhs: Ptr<v8_PagePermissions>,
    rhs: v8_PagePermissions,
) -> Ptr<v8_PagePermissions> {
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    let __rhs = ({
        let _lhs: v8_PagePermissions = (lhs.read());
        let _rhs: v8_PagePermissions = (*rhs.borrow());
        operator_bitor_6(_lhs, _rhs)
    });
    lhs.write(__rhs);
    return (lhs).clone();
}
pub fn IsSubset_9(lhs: v8_PagePermissions, rhs: v8_PagePermissions) -> bool {
    let lhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(lhs));
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    return (({
        let _lhs: v8_PagePermissions = (*lhs.borrow());
        operator_bitand_7(_lhs, (*rhs.borrow()))
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
        return (({ floor_10(({ self.CurrentClockTimeMillis() })) }) as i64);
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
            static default_observer_11: Value<v8_HighAllocationThroughputObserver> = Rc::new(
                RefCell::new(<v8_HighAllocationThroughputObserver>::default()),
            );
        );
        return (default_observer_11.with(Value::clone).as_pointer());
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
pub fn CountLeadingZeros_32(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        if (32_u32 == 64_u32) {
            (({ __builtin_clzll_33(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((32_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros_34(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        if (64_u32 == 64_u32) {
            (({ __builtin_clzll_33(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((64_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros32_35(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_32((*value.borrow())) });
}
pub fn CountLeadingZeros64_36(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_34((*value.borrow())) });
}
pub fn CountTrailingZeros_37(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        (if (32_u32 == 64_u32) {
            ({ __builtin_ctzll_38(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros_39(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        (if (64_u32 == 64_u32) {
            ({ __builtin_ctzll_38(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros32_40(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_37((*value.borrow())) });
}
pub fn CountTrailingZeros64_41(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_39((*value.borrow())) });
}
pub fn RoundUpToPowerOfTwo32_42(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u32 << ((32_u32).wrapping_sub(({ CountLeadingZeros_32((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo64_43(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u64 << ((64_u32).wrapping_sub(({ CountLeadingZeros_34((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo_44(value: usize) -> usize {
    let value: Value<usize> = Rc::new(RefCell::new(value));
    if (::std::mem::size_of::<usize>() == ::std::mem::size_of::<u64>()) {
        return (({ RoundUpToPowerOfTwo64_43(((*value.borrow()) as u64)) }) as usize);
    } else {
        return (({ RoundUpToPowerOfTwo32_42(((*value.borrow()) as u32)) }) as usize);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn RoundDownToPowerOfTwo32_45(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    if ((*value.borrow()) > 2147483648_u32) {
        return 2147483648_u32;
    }
    let result: Value<u32> = Rc::new(RefCell::new(
        ({ RoundUpToPowerOfTwo32_42((*value.borrow())) }),
    ));
    if ((*result.borrow()) > (*value.borrow())) {
        (*result.borrow_mut()) >>= 1;
    }
    return (*result.borrow());
}
pub fn RotateRight32_46(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateLeft32_47(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateRight64_48(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn RotateLeft64_49(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn ClearLsb_50(value: i32) -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(value));
    return ((*value.borrow()) & ((*value.borrow()) - 1));
}
pub fn SignedAddOverflow32_51(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_sadd_overflow_52(_arg0, _arg1, _arg2)
    });
}
pub fn SignedSubOverflow32_53(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_ssub_overflow_54(_arg0, _arg1, _arg2)
    });
}
pub fn SignedMulOverflow32_55(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_smul_overflow_56(_arg0, _arg1, _arg2)
    });
}
pub fn SignedAddOverflow64_57(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_add_overflow_58(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedSubOverflow64_59(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_sub_overflow_60(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedMulOverflow64_61(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return {
        let (val, ovf) = (*lhs.borrow()).overflowing_mul((*rhs.borrow()));
        (*val.borrow()).write(val);
        ovf
    };
}
pub fn UnsignedAddOverflow32_62(lhs: u32, rhs: u32, val: Ptr<u32>) -> bool {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<u32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: u32 = (*lhs.borrow());
        let _arg1: u32 = (*rhs.borrow());
        let _arg2: Ptr<u32> = (*val.borrow()).clone();
        __builtin_uadd_overflow_63(_arg0, _arg1, _arg2)
    });
}
pub fn UnsignedDiv32_64(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedDiv64_65(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn UnsignedMod32_66(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedMod64_67(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn WraparoundAdd32_68(lhs: i32, rhs: i32) -> i32 {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as u32).wrapping_add(((*rhs.borrow()) as u32))) as i32);
}
pub fn WraparoundNeg32_69(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (-((*x.borrow()) as u32) as i32);
}
pub fn ByteReverse16_70(value: u16) -> u16 {
    let value: Value<u16> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse32_71(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse64_72(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
thread_local!(
    pub static kMaxExponent_73: Value<i32> = Rc::new(RefCell::new(64));
);
thread_local!(
    pub static kMaxExponent_74: Value<i32> = Rc::new(RefCell::new(1024));
);
thread_local!(
    pub static kIntegerBitsPlusSign_75: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kIntegerBitsPlusSign_76: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kIntegerBitsPlusSign_77: Value<i32> = Rc::new(RefCell::new(16));
);
thread_local!(
    pub static kIntegerBitsPlusSign_78: Value<i32> = Rc::new(RefCell::new(16));
);
thread_local!(
    pub static kIntegerBitsPlusSign_79: Value<i32> = Rc::new(RefCell::new(32));
);
thread_local!(
    pub static kIntegerBitsPlusSign_80: Value<i32> = Rc::new(RefCell::new(32));
);
thread_local!(
    pub static kIntegerBitsPlusSign_81: Value<i32> = Rc::new(RefCell::new(64));
);
thread_local!(
    pub static kIntegerBitsPlusSign_82: Value<i32> = Rc::new(RefCell::new(64));
);
thread_local!(
    pub static kIntegerBitsPlusSign_83: Value<i32> = Rc::new(RefCell::new(64));
);
pub fn IsValueNegative_84(value: i64) -> bool {
    let value: Value<i64> = Rc::new(RefCell::new(value));
    if true {
        return ((*value.borrow()) < 0_i64);
    } else {
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ConditionalNegate_85(x: u64, is_negative: bool) -> i64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let is_negative: Value<bool> = Rc::new(RefCell::new(is_negative));
    return (((((*x.borrow()) as u64) ^ (-((*is_negative.borrow()) as i64) as u64))
        .wrapping_add(((*is_negative.borrow()) as u64))) as i64);
}
pub fn SafeUnsignedAbs_86(value: i64) -> u64 {
    let value: Value<i64> = Rc::new(RefCell::new(value));
    return if ({ IsValueNegative_84((*value.borrow())) }) {
        (0_u64).wrapping_sub(((*value.borrow()) as u64))
    } else {
        ((*value.borrow()) as u64)
    };
}
thread_local!(
    pub static kEnableAsmCode_87: Value<bool> = Rc::new(RefCell::new(true));
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
    pub static kStaticDstRangeRelationToSrcRange_88: Value<
        v8_base_internal_NumericRangeRepresentation,
    > = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kStaticDstRangeRelationToSrcRange_90: Value<
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
    pub static kShift_91: Value<i32> = Rc::new(RefCell::new(10));
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
                let _x: u64 = (({ SafeUnsignedAbs_86((*value.borrow())) })
                    & !(((1_u64 << 10) as u64).wrapping_sub((1_u64 as u64))));
                let _is_negative: bool = ({ IsValueNegative_84((*value.borrow())) });
                ConditionalNegate_85(_x, _is_negative)
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
pub fn DstRangeRelationToSrcRange_92(value: f64) -> v8_base_internal_RangeCheck {
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
    pub static kIsCheckedNumeric_93: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static kIsClampedNumeric_94: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static kIsStrictNumeric_95: Value<bool> = Rc::new(RefCell::new(true));
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
    pub static kIsNumeric_96: Value<bool> = Rc::new(RefCell::new(true));
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
pub fn saturated_cast_impl_97(value: f64, constraint: v8_base_internal_RangeCheck) -> i64 {
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
    pub static is_supported_98: Value<bool> = Rc::new(RefCell::new(false));
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
pub fn saturated_cast_99(value: f64) -> i64 {
    let value: Value<f64> = Rc::new(RefCell::new(value));
    let underlying_value: Value<f64> = Rc::new(RefCell::new(((*value.borrow()) as f64)));
    return if ((!({ is_constant_evaluated_100() })) && (false)) && (true) {
        ({ v8_base_internal_SaturateFastOp_long_long__double_::Do((*underlying_value.borrow())) })
    } else {
        ({
            let _value: f64 = (*underlying_value.borrow());
            let _constraint: v8_base_internal_RangeCheck =
                ({ DstRangeRelationToSrcRange_92((*underlying_value.borrow())) });
            saturated_cast_impl_97(_value, _constraint)
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
    pub static kHoursPerDay_103: Value<i64> = Rc::new(RefCell::new(24));
);
thread_local!(
    pub static kMillisecondsPerSecond_104: Value<i64> = Rc::new(RefCell::new(1000));
);
thread_local!(
    pub static kMillisecondsPerDay_105: Value<i64> = Rc::new(RefCell::new(86400000));
);
thread_local!(
    pub static kMicrosecondsPerMillisecond_106: Value<i64> = Rc::new(RefCell::new(1000));
);
thread_local!(
    pub static kMicrosecondsPerSecond_107: Value<i64> = Rc::new(RefCell::new(1000000));
);
thread_local!(
    pub static kMicrosecondsPerMinute_108: Value<i64> = Rc::new(RefCell::new(60000000));
);
thread_local!(
    pub static kMicrosecondsPerHour_109: Value<i64> = Rc::new(RefCell::new(3600000000));
);
thread_local!(
    pub static kMicrosecondsPerDay_110: Value<i64> = Rc::new(RefCell::new(86400000000));
);
thread_local!(
    pub static kMicrosecondsPerWeek_111: Value<i64> = Rc::new(RefCell::new(604800000000));
);
thread_local!(
    pub static kNanosecondsPerMicrosecond_112: Value<i64> = Rc::new(RefCell::new(1000));
);
thread_local!(
    pub static kNanosecondsPerSecond_113: Value<i64> = Rc::new(RefCell::new(1000000000));
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
pub fn swap_114(a: v8_base_TimeDelta, b: v8_base_TimeDelta) {
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
            ({ saturated_cast_99((*value.borrow())) })
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
pub fn Nanoseconds_115(nanoseconds: i64) -> v8_base_TimeDelta {
    let nanoseconds: Value<i64> = Rc::new(RefCell::new(nanoseconds));
    return ({ v8_base_TimeDelta::FromNanoseconds((*nanoseconds.borrow())) });
}
pub fn Microseconds_116(microseconds: i64) -> v8_base_TimeDelta {
    let microseconds: Value<i64> = Rc::new(RefCell::new(microseconds));
    return ({ v8_base_TimeDelta::FromMicroseconds((*microseconds.borrow())) });
}
pub fn Milliseconds_117(milliseconds: i64) -> v8_base_TimeDelta {
    let milliseconds: Value<i64> = Rc::new(RefCell::new(milliseconds));
    return ({ v8_base_TimeDelta::FromMilliseconds((*milliseconds.borrow())) });
}
pub fn Milliseconds_118(milliseconds: f64) -> v8_base_TimeDelta {
    let milliseconds: Value<f64> = Rc::new(RefCell::new(milliseconds));
    return ({ v8_base_TimeDelta::FromMillisecondsD((*milliseconds.borrow())) });
}
pub fn Seconds_119(seconds: i64) -> v8_base_TimeDelta {
    let seconds: Value<i64> = Rc::new(RefCell::new(seconds));
    return ({ v8_base_TimeDelta::FromSeconds((*seconds.borrow())) });
}
pub fn Seconds_120(seconds: f64) -> v8_base_TimeDelta {
    let seconds: Value<f64> = Rc::new(RefCell::new(seconds));
    return ({ v8_base_TimeDelta::FromSecondsD((*seconds.borrow())) });
}
pub fn Minutes_121(minutes: i32) -> v8_base_TimeDelta {
    let minutes: Value<i32> = Rc::new(RefCell::new(minutes));
    return ({ v8_base_TimeDelta::FromMinutes((*minutes.borrow())) });
}
pub fn Hours_122(hours: i32) -> v8_base_TimeDelta {
    let hours: Value<i32> = Rc::new(RefCell::new(hours));
    return ({ v8_base_TimeDelta::FromHours((*hours.borrow())) });
}
pub fn FromDays_123(days: i32) -> v8_base_TimeDelta {
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
pub fn operator_add_124(delta: Ptr<v8_base_TimeDelta>, time: Ptr<v8_base_Time>) -> v8_base_Time {
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
pub fn operator_add_125(
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
pub type cppgc_EmbedderStackState = i32;
pub const cppgc_EmbedderStackState_kMayContainHeapPointers: cppgc_EmbedderStackState = 0;
pub const cppgc_EmbedderStackState_kNoHeapPointers: cppgc_EmbedderStackState = 1;
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
pub fn HardeningAbort_127() {
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
pub fn is_constant_evaluated_128() -> bool {
    return ({ is_constant_evaluated_100() });
}
thread_local!(
    pub static in_place_129: Ptr<std_in_place_t> = in_place_130.with(Value::clone).as_pointer();
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
pub struct anon_131 {
    pub target: Value<AnyPtr>,
    pub size: Value<usize>,
}
impl Clone for anon_131 {
    fn clone(&self) -> Self {
        let __this: Value<anon_131> = Rc::new(RefCell::new(Self {
            target: Rc::new(RefCell::new((*self.target.borrow()).clone())),
            size: Rc::new(RefCell::new((*self.size.borrow()))),
        }));
        let this: Ptr<anon_131> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for anon_131 {
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
    pub fn remote(&self) -> Ptr<anon_131> {
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
pub fn EmptyManager_132(
    _a0: absl_internal_any_invocable_FunctionToCall,
    _a1: Ptr<absl_internal_any_invocable_TypeErasedState>,
    _a2: Ptr<absl_internal_any_invocable_TypeErasedState>,
) {
    let _a0: Value<absl_internal_any_invocable_FunctionToCall> = Rc::new(RefCell::new(_a0));
    let _a1: Value<Ptr<absl_internal_any_invocable_TypeErasedState>> = Rc::new(RefCell::new(_a1));
    let _a2: Value<Ptr<absl_internal_any_invocable_TypeErasedState>> = Rc::new(RefCell::new(_a2));
}
pub fn LocalManagerTrivial_133(
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
pub fn RemoteManagerTrivial_134(
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
pub fn IsCompatibleConversion_135(_a0: AnyPtr, _a1: AnyPtr) -> bool {
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
thread_local!(
    pub static is_instantiation_v_136: Value<bool> = Rc::new(RefCell::new(true));
);
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
    pub static value_137: Value<bool> = Rc::new(RefCell::new(false));
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
#[derive()]
pub struct cppgc_StackStartMarker {
    stack_start_: Value<AnyPtr>,
}
impl cppgc_StackStartMarker {
    pub fn cppgc_StackStartMarker() -> Self {
        let __this: Value<cppgc_StackStartMarker> = Rc::new(RefCell::new(Self {
            stack_start_: Rc::new(RefCell::new(({ __builtin_frame_address_138(0_u32) }))),
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
                    (*nullopt_5.with(Value::clone).borrow()).clone()
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
pub type cppgc_internal_MetricRecorder_GCCycle_Type = i32;
pub const cppgc_internal_MetricRecorder_GCCycle_Type_kMinor:
    cppgc_internal_MetricRecorder_GCCycle_Type = 0;
pub const cppgc_internal_MetricRecorder_GCCycle_Type_kMajor:
    cppgc_internal_MetricRecorder_GCCycle_Type = 1;
#[derive(Default)]
pub struct cppgc_internal_MetricRecorder_GCCycle_IncrementalPhases {
    pub mark_duration_us: Value<i64>,
    pub sweep_duration_us: Value<i64>,
}
impl Clone for cppgc_internal_MetricRecorder_GCCycle_IncrementalPhases {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_MetricRecorder_GCCycle_IncrementalPhases> =
            Rc::new(RefCell::new(Self {
                mark_duration_us: Rc::new(RefCell::new((*self.mark_duration_us.borrow()))),
                sweep_duration_us: Rc::new(RefCell::new((*self.sweep_duration_us.borrow()))),
            }));
        let this: Ptr<cppgc_internal_MetricRecorder_GCCycle_IncrementalPhases> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_MetricRecorder_GCCycle_IncrementalPhases {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.mark_duration_us.borrow()).to_bytes(&mut buf[0..8]);
        (*self.sweep_duration_us.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            mark_duration_us: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
            sweep_duration_us: Rc::new(RefCell::new(<i64>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Default)]
pub struct cppgc_internal_MetricRecorder_GCCycle_Phases {
    pub weak_duration_us: Value<i64>,
    pub compact_duration_us: Value<i64>,
}
impl Clone for cppgc_internal_MetricRecorder_GCCycle_Phases {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_MetricRecorder_GCCycle_Phases> =
            Rc::new(RefCell::new(Self {
                weak_duration_us: Rc::new(RefCell::new((*self.weak_duration_us.borrow()))),
                compact_duration_us: Rc::new(RefCell::new((*self.compact_duration_us.borrow()))),
            }));
        let this: Ptr<cppgc_internal_MetricRecorder_GCCycle_Phases> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_MetricRecorder_GCCycle_Phases {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.weak_duration_us.borrow()).to_bytes(&mut buf[16..24]);
        (*self.compact_duration_us.borrow()).to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            weak_duration_us: Rc::new(RefCell::new(<i64>::from_bytes(&buf[16..24]))),
            compact_duration_us: Rc::new(RefCell::new(<i64>::from_bytes(&buf[24..32]))),
        }
    }
}
#[derive(Default)]
pub struct cppgc_internal_MetricRecorder_GCCycle_Sizes {
    pub before_bytes: Value<i64>,
    pub after_bytes: Value<i64>,
    pub freed_bytes: Value<i64>,
}
impl Clone for cppgc_internal_MetricRecorder_GCCycle_Sizes {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_MetricRecorder_GCCycle_Sizes> =
            Rc::new(RefCell::new(Self {
                before_bytes: Rc::new(RefCell::new((*self.before_bytes.borrow()))),
                after_bytes: Rc::new(RefCell::new((*self.after_bytes.borrow()))),
                freed_bytes: Rc::new(RefCell::new((*self.freed_bytes.borrow()))),
            }));
        let this: Ptr<cppgc_internal_MetricRecorder_GCCycle_Sizes> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_MetricRecorder_GCCycle_Sizes {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.before_bytes.borrow()).to_bytes(&mut buf[0..8]);
        (*self.after_bytes.borrow()).to_bytes(&mut buf[8..16]);
        (*self.freed_bytes.borrow()).to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            before_bytes: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
            after_bytes: Rc::new(RefCell::new(<i64>::from_bytes(&buf[8..16]))),
            freed_bytes: Rc::new(RefCell::new(<i64>::from_bytes(&buf[16..24]))),
        }
    }
}
#[derive(Default)]
pub struct cppgc_internal_MetricRecorder_GCCycle {
    pub type_: Value<cppgc_internal_MetricRecorder_GCCycle_Type>,
    pub total: Value<cppgc_internal_MetricRecorder_GCCycle_Phases>,
    pub main_thread: Value<cppgc_internal_MetricRecorder_GCCycle_Phases>,
    pub main_thread_atomic: Value<cppgc_internal_MetricRecorder_GCCycle_Phases>,
    pub main_thread_incremental: Value<cppgc_internal_MetricRecorder_GCCycle_IncrementalPhases>,
    pub objects: Value<cppgc_internal_MetricRecorder_GCCycle_Sizes>,
    pub memory: Value<cppgc_internal_MetricRecorder_GCCycle_Sizes>,
    pub collection_rate_in_percent: Value<f64>,
    pub efficiency_in_bytes_per_us: Value<f64>,
    pub main_thread_efficiency_in_bytes_per_us: Value<f64>,
}
impl Clone for cppgc_internal_MetricRecorder_GCCycle {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_MetricRecorder_GCCycle> = Rc::new(RefCell::new(Self {
            type_: Rc::new(RefCell::new((*self.type_.borrow()))),
            total: Rc::new(RefCell::new((*self.total.borrow()).clone())),
            main_thread: Rc::new(RefCell::new((*self.main_thread.borrow()).clone())),
            main_thread_atomic: Rc::new(RefCell::new((*self.main_thread_atomic.borrow()).clone())),
            main_thread_incremental: Rc::new(RefCell::new(
                (*self.main_thread_incremental.borrow()).clone(),
            )),
            objects: Rc::new(RefCell::new((*self.objects.borrow()).clone())),
            memory: Rc::new(RefCell::new((*self.memory.borrow()).clone())),
            collection_rate_in_percent: Rc::new(RefCell::new(
                (*self.collection_rate_in_percent.borrow()),
            )),
            efficiency_in_bytes_per_us: Rc::new(RefCell::new(
                (*self.efficiency_in_bytes_per_us.borrow()),
            )),
            main_thread_efficiency_in_bytes_per_us: Rc::new(RefCell::new(
                (*self.main_thread_efficiency_in_bytes_per_us.borrow()),
            )),
        }));
        let this: Ptr<cppgc_internal_MetricRecorder_GCCycle> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_MetricRecorder_GCCycle {
    fn byte_size() -> usize {
        192
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.type_.borrow()).to_bytes(&mut buf[0..4]);
        (*self.total.borrow()).to_bytes(&mut buf[8..40]);
        (*self.main_thread.borrow()).to_bytes(&mut buf[40..72]);
        (*self.main_thread_atomic.borrow()).to_bytes(&mut buf[72..104]);
        (*self.main_thread_incremental.borrow()).to_bytes(&mut buf[104..120]);
        (*self.objects.borrow()).to_bytes(&mut buf[120..144]);
        (*self.memory.borrow()).to_bytes(&mut buf[144..168]);
        (*self.collection_rate_in_percent.borrow()).to_bytes(&mut buf[168..176]);
        (*self.efficiency_in_bytes_per_us.borrow()).to_bytes(&mut buf[176..184]);
        (*self.main_thread_efficiency_in_bytes_per_us.borrow()).to_bytes(&mut buf[184..192]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            type_: Rc::new(RefCell::new(
                <cppgc_internal_MetricRecorder_GCCycle_Type>::from_bytes(&buf[0..4]),
            )),
            total: Rc::new(RefCell::new(
                <cppgc_internal_MetricRecorder_GCCycle_Phases>::from_bytes(&buf[8..40]),
            )),
            main_thread: Rc::new(RefCell::new(
                <cppgc_internal_MetricRecorder_GCCycle_Phases>::from_bytes(&buf[40..72]),
            )),
            main_thread_atomic: Rc::new(RefCell::new(
                <cppgc_internal_MetricRecorder_GCCycle_Phases>::from_bytes(&buf[72..104]),
            )),
            main_thread_incremental: Rc::new(RefCell::new(
                <cppgc_internal_MetricRecorder_GCCycle_IncrementalPhases>::from_bytes(
                    &buf[104..120],
                ),
            )),
            objects: Rc::new(RefCell::new(
                <cppgc_internal_MetricRecorder_GCCycle_Sizes>::from_bytes(&buf[120..144]),
            )),
            memory: Rc::new(RefCell::new(
                <cppgc_internal_MetricRecorder_GCCycle_Sizes>::from_bytes(&buf[144..168]),
            )),
            collection_rate_in_percent: Rc::new(RefCell::new(<f64>::from_bytes(&buf[168..176]))),
            efficiency_in_bytes_per_us: Rc::new(RefCell::new(<f64>::from_bytes(&buf[176..184]))),
            main_thread_efficiency_in_bytes_per_us: Rc::new(RefCell::new(<f64>::from_bytes(
                &buf[184..192],
            ))),
        }
    }
}
#[derive(Default)]
pub struct cppgc_internal_MetricRecorder_MainThreadIncrementalMark {
    pub duration_us: Value<i64>,
}
impl Clone for cppgc_internal_MetricRecorder_MainThreadIncrementalMark {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_MetricRecorder_MainThreadIncrementalMark> =
            Rc::new(RefCell::new(Self {
                duration_us: Rc::new(RefCell::new((*self.duration_us.borrow()))),
            }));
        let this: Ptr<cppgc_internal_MetricRecorder_MainThreadIncrementalMark> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_MetricRecorder_MainThreadIncrementalMark {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.duration_us.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            duration_us: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct cppgc_internal_MetricRecorder_MainThreadIncrementalSweep {
    pub duration_us: Value<i64>,
}
impl Clone for cppgc_internal_MetricRecorder_MainThreadIncrementalSweep {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_MetricRecorder_MainThreadIncrementalSweep> =
            Rc::new(RefCell::new(Self {
                duration_us: Rc::new(RefCell::new((*self.duration_us.borrow()))),
            }));
        let this: Ptr<cppgc_internal_MetricRecorder_MainThreadIncrementalSweep> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_MetricRecorder_MainThreadIncrementalSweep {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.duration_us.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            duration_us: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct cppgc_internal_MetricRecorder {}
impl Clone for cppgc_internal_MetricRecorder {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_MetricRecorder> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<cppgc_internal_MetricRecorder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_MetricRecorder {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn Ignore_139(args_0: Ptr<Box<[u8]>>, args_1: Ptr<perfetto_DynamicString>) {}
#[derive(Default)]
pub struct perfetto_StaticString {}
impl Clone for perfetto_StaticString {
    fn clone(&self) -> Self {
        let __this: Value<perfetto_StaticString> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<perfetto_StaticString> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for perfetto_StaticString {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
impl perfetto_DynamicString {
    pub fn perfetto_DynamicString(_a0: Ptr<u8>) -> Self {
        let _a0: Value<Ptr<u8>> = Rc::new(RefCell::new(_a0));
        let __this: Value<perfetto_DynamicString> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<perfetto_DynamicString> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Default)]
pub struct perfetto_DynamicString {}
impl Clone for perfetto_DynamicString {
    fn clone(&self) -> Self {
        let __this: Value<perfetto_DynamicString> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<perfetto_DynamicString> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for perfetto_DynamicString {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct perfetto_Track {}
impl perfetto_Track {
    pub fn perfetto_Track1(id: u64) -> Self {
        let id: Value<u64> = Rc::new(RefCell::new(id));
        let __this: Value<perfetto_Track> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<perfetto_Track> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for perfetto_Track {
    fn clone(&self) -> Self {
        let __this: Value<perfetto_Track> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<perfetto_Track> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for perfetto_Track {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct perfetto_ThreadTrack {}
impl perfetto_ThreadTrack {
    pub fn Current() -> perfetto_ThreadTrack {
        return <perfetto_ThreadTrack>::default();
    }
}
impl Clone for perfetto_ThreadTrack {
    fn clone(&self) -> Self {
        let __this: Value<perfetto_ThreadTrack> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<perfetto_ThreadTrack> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for perfetto_ThreadTrack {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct perfetto_NamedTrack {}
impl Clone for perfetto_NamedTrack {
    fn clone(&self) -> Self {
        let __this: Value<perfetto_NamedTrack> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<perfetto_NamedTrack> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for perfetto_NamedTrack {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct perfetto_CounterTrack {}
impl Clone for perfetto_CounterTrack {
    fn clone(&self) -> Self {
        let __this: Value<perfetto_CounterTrack> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<perfetto_CounterTrack> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for perfetto_CounterTrack {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct perfetto_Flow {}
impl perfetto_Flow {
    pub fn ProcessScoped(flow_id: u64) -> perfetto_Flow {
        let flow_id: Value<u64> = Rc::new(RefCell::new(flow_id));
        return <perfetto_Flow>::default();
    }
    pub fn FromPointer(ptr: AnyPtr) -> perfetto_Flow {
        let ptr: Value<AnyPtr> = Rc::new(RefCell::new(ptr));
        return <perfetto_Flow>::default();
    }
    pub fn Global(flow_id: u64) -> perfetto_Flow {
        let flow_id: Value<u64> = Rc::new(RefCell::new(flow_id));
        return <perfetto_Flow>::default();
    }
}
impl Clone for perfetto_Flow {
    fn clone(&self) -> Self {
        let __this: Value<perfetto_Flow> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<perfetto_Flow> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for perfetto_Flow {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct perfetto_TerminatingFlow {}
impl perfetto_TerminatingFlow {
    pub fn ProcessScoped(flow_id: u64) -> perfetto_TerminatingFlow {
        let flow_id: Value<u64> = Rc::new(RefCell::new(flow_id));
        return <perfetto_TerminatingFlow>::default();
    }
    pub fn FromPointer(ptr: AnyPtr) -> perfetto_TerminatingFlow {
        let ptr: Value<AnyPtr> = Rc::new(RefCell::new(ptr));
        return <perfetto_TerminatingFlow>::default();
    }
    pub fn Global(flow_id: u64) -> perfetto_TerminatingFlow {
        let flow_id: Value<u64> = Rc::new(RefCell::new(flow_id));
        return <perfetto_TerminatingFlow>::default();
    }
}
impl Clone for perfetto_TerminatingFlow {
    fn clone(&self) -> Self {
        let __this: Value<perfetto_TerminatingFlow> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<perfetto_TerminatingFlow> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for perfetto_TerminatingFlow {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn SeqCst_MemoryFence_140() {
    ({ atomic_thread_fence_141(5) });
}
pub fn Relaxed_Store_142(ptr: Ptr<u8>, value: u8) {
    let ptr: Value<Ptr<u8>> = Rc::new(RefCell::new(ptr));
    let value: Value<u8> = Rc::new(RefCell::new(value));
    ({
        let ___desired: u8 = (*value.borrow());
        std_atomic_ref_char_::std_atomic_ref_char_1({ (*ptr.borrow()).clone() })
            .store(___desired, Some(0))
    });
}
pub fn Relaxed_Store_143(ptr: Ptr<i64>, value: i64) {
    let ptr: Value<Ptr<i64>> = Rc::new(RefCell::new(ptr));
    let value: Value<i64> = Rc::new(RefCell::new(value));
    ({
        let ___desired: i64 = (*value.borrow());
        std_atomic_ref_long_::std_atomic_ref_long_2({ (*ptr.borrow()).clone() })
            .store(___desired, Some(0))
    });
}
pub fn Relaxed_Load_144(ptr: Ptr<u8>) -> u8 {
    let ptr: Value<Ptr<u8>> = Rc::new(RefCell::new(ptr));
    return ({
        std_atomic_ref_char_::std_atomic_ref_char_1({ (*ptr.borrow()).clone() }).load(Some(0))
    });
}
pub fn Relaxed_Load_145(ptr: Ptr<i64>) -> i64 {
    let ptr: Value<Ptr<i64>> = Rc::new(RefCell::new(ptr));
    return ({
        std_atomic_ref_long_::std_atomic_ref_long_2({ (*ptr.borrow()).clone() }).load(Some(0))
    });
}
pub fn Relaxed_Memcpy_146(dst: Ptr<u8>, src: Ptr<u8>, bytes: usize) {
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let src: Value<Ptr<u8>> = Rc::new(RefCell::new(src));
    let bytes: Value<usize> = Rc::new(RefCell::new(bytes));
    let kAtomicWordSize: Value<usize> = Rc::new(RefCell::new(8));
    'loop_: while ((*bytes.borrow()) > 0_usize)
        && (!({ IsAligned_31((*dst.borrow()).to_int(), (8 as u64)) }))
    {
        ({
            let _ptr: Ptr<u8> = (*dst.borrow_mut()).postfix_inc();
            let _value: u8 = ({ Relaxed_Load_144((*src.borrow_mut()).postfix_inc()) });
            Relaxed_Store_142(_ptr, _value)
        });
        (*bytes.borrow_mut()).prefix_dec();
    }
    if ({ IsAligned_31((*src.borrow()).to_int(), (8 as u64)) })
        && ({ IsAligned_31((*dst.borrow()).to_int(), (8 as u64)) })
    {
        'loop_: while ((*bytes.borrow()) >= 8) {
            ({
                Relaxed_Store_143(
                    (*dst.borrow()).reinterpret_cast::<i64>(),
                    ({ Relaxed_Load_145((*src.borrow()).reinterpret_cast::<i64>()) }),
                )
            });
            (*dst.borrow_mut()) += 8;
            (*src.borrow_mut()) += 8;
            {
                let rhs_0 = (*bytes.borrow()).wrapping_sub(8);
                (*bytes.borrow_mut()) = rhs_0
            };
        }
    }
    'loop_: while ((*bytes.borrow()) > 0_usize) {
        ({
            let _ptr: Ptr<u8> = (*dst.borrow_mut()).postfix_inc();
            let _value: u8 = ({ Relaxed_Load_144((*src.borrow_mut()).postfix_inc()) });
            Relaxed_Store_142(_ptr, _value)
        });
        (*bytes.borrow_mut()).prefix_dec();
    }
}
pub fn Relaxed_Memmove_147(dst: Ptr<u8>, src: Ptr<u8>, bytes: usize) {
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let src: Value<Ptr<u8>> = Rc::new(RefCell::new(src));
    let bytes: Value<usize> = Rc::new(RefCell::new(bytes));
    if {
        let _lhs = (((*dst.borrow()).to_int()).wrapping_sub((*src.borrow()).to_int()) as usize);
        _lhs >= (*bytes.borrow())
    } {
        ({
            Relaxed_Memcpy_146(
                (*dst.borrow()).clone(),
                (*src.borrow()).clone(),
                (*bytes.borrow()),
            )
        });
        return;
    }
    (*dst.borrow_mut()) += (*bytes.borrow());
    (*src.borrow_mut()) += (*bytes.borrow());
    let kAtomicWordSize: Value<usize> = Rc::new(RefCell::new(8));
    'loop_: while ((*bytes.borrow()) > 0_usize)
        && (!({ IsAligned_31((*dst.borrow()).to_int(), (8 as u64)) }))
    {
        ({
            let _ptr: Ptr<u8> = (*dst.borrow_mut()).prefix_dec();
            let _value: u8 = ({ Relaxed_Load_144((*src.borrow_mut()).prefix_dec()) });
            Relaxed_Store_142(_ptr, _value)
        });
        (*bytes.borrow_mut()).prefix_dec();
    }
    if ({ IsAligned_31((*src.borrow()).to_int(), (8 as u64)) })
        && ({ IsAligned_31((*dst.borrow()).to_int(), (8 as u64)) })
    {
        'loop_: while ((*bytes.borrow()) >= 8) {
            (*dst.borrow_mut()) -= 8;
            (*src.borrow_mut()) -= 8;
            {
                let rhs_0 = (*bytes.borrow()).wrapping_sub(8);
                (*bytes.borrow_mut()) = rhs_0
            };
            ({
                Relaxed_Store_143(
                    (*dst.borrow()).reinterpret_cast::<i64>(),
                    ({ Relaxed_Load_145((*src.borrow()).reinterpret_cast::<i64>()) }),
                )
            });
        }
    }
    'loop_: while ((*bytes.borrow()) > 0_usize) {
        ({
            let _ptr: Ptr<u8> = (*dst.borrow_mut()).prefix_dec();
            let _value: u8 = ({ Relaxed_Load_144((*src.borrow_mut()).prefix_dec()) });
            Relaxed_Store_142(_ptr, _value)
        });
        (*bytes.borrow_mut()).prefix_dec();
    }
}
pub fn MemcmpNotEqualFundamental_148(u1: u8, u2: u8) -> i32 {
    let u1: Value<u8> = Rc::new(RefCell::new(u1));
    let u2: Value<u8> = Rc::new(RefCell::new(u2));
    (&(0));
    return if (((*u1.borrow()) as i32) < ((*u2.borrow()) as i32)) {
        -1_i32
    } else {
        1
    };
}
pub fn MemcmpNotEqualFundamental_149(u1: i64, u2: i64) -> i32 {
    let u1: Value<i64> = Rc::new(RefCell::new(u1));
    let u2: Value<i64> = Rc::new(RefCell::new(u2));
    (&(0));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < ::std::mem::size_of::<i64>()) {
        let byte1: Value<u8> = Rc::new(RefCell::new((((*u1.borrow()) & 255_i64) as u8)));
        let byte2: Value<u8> = Rc::new(RefCell::new((((*u2.borrow()) & 255_i64) as u8)));
        if (((*byte1.borrow()) as i32) != ((*byte2.borrow()) as i32)) {
            return if (((*byte1.borrow()) as i32) < ((*byte2.borrow()) as i32)) {
                -1_i32
            } else {
                1
            };
        }
        (*u1.borrow_mut()) >>= 8;
        (*u2.borrow_mut()) >>= 8;
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        V8_Fatal_150(
            (*kUnreachableCodeMessage_21.with(Value::clone).borrow()).clone(),
            &[],
        )
    });
    panic!("ub: non-void function does not return a value")
}
pub fn Relaxed_Memcmp_151(s1: Ptr<u8>, s2: Ptr<u8>, len: usize) -> i32 {
    let s1: Value<Ptr<u8>> = Rc::new(RefCell::new(s1));
    let s2: Value<Ptr<u8>> = Rc::new(RefCell::new(s2));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let kAtomicWordSize: Value<usize> = Rc::new(RefCell::new(8));
    'loop_: while ((*len.borrow()) > 0_usize)
        && (!(({ IsAligned_31((*s1.borrow()).to_int(), (8 as u64)) })
            && ({ IsAligned_31((*s2.borrow()).to_int(), (8 as u64)) })))
    {
        let u1: Value<u8> = Rc::new(RefCell::new(
            ({ Relaxed_Load_144((*s1.borrow_mut()).postfix_inc()) }),
        ));
        let u2: Value<u8> = Rc::new(RefCell::new(
            ({ Relaxed_Load_144((*s2.borrow_mut()).postfix_inc()) }),
        ));
        if (((*u1.borrow()) as i32) != ((*u2.borrow()) as i32)) {
            return ({ MemcmpNotEqualFundamental_148((*u1.borrow()), (*u2.borrow())) });
        }
        (*len.borrow_mut()).prefix_dec();
    }
    if ({ IsAligned_31((*s1.borrow()).to_int(), (8 as u64)) })
        && ({ IsAligned_31((*s2.borrow()).to_int(), (8 as u64)) })
    {
        'loop_: while ((*len.borrow()) >= 8) {
            let u1: Value<i64> = Rc::new(RefCell::new(
                ({ Relaxed_Load_145((*s1.borrow()).reinterpret_cast::<i64>()) }),
            ));
            let u2: Value<i64> = Rc::new(RefCell::new(
                ({ Relaxed_Load_145((*s2.borrow()).reinterpret_cast::<i64>()) }),
            ));
            if ((*u1.borrow()) != (*u2.borrow())) {
                return ({ MemcmpNotEqualFundamental_149((*u1.borrow()), (*u2.borrow())) });
            }
            (*s1.borrow_mut()) += 8;
            (*s2.borrow_mut()) += 8;
            {
                let rhs_0 = (*len.borrow()).wrapping_sub(8);
                (*len.borrow_mut()) = rhs_0
            };
        }
    }
    'loop_: while ((*len.borrow()) > 0_usize) {
        let u1: Value<u8> = Rc::new(RefCell::new(
            ({ Relaxed_Load_144((*s1.borrow_mut()).postfix_inc()) }),
        ));
        let u2: Value<u8> = Rc::new(RefCell::new(
            ({ Relaxed_Load_144((*s2.borrow_mut()).postfix_inc()) }),
        ));
        if (((*u1.borrow()) as i32) != ((*u2.borrow()) as i32)) {
            return ({ MemcmpNotEqualFundamental_148((*u1.borrow()), (*u2.borrow())) });
        }
        (*len.borrow_mut()).prefix_dec();
    }
    return 0;
}
pub type CategoryGroupEnabledFlags = u32;
pub const CategoryGroupEnabledFlags_kEnabledForRecording_CategoryGroupEnabledFlags:
    CategoryGroupEnabledFlags = 1;
pub const CategoryGroupEnabledFlags_kEnabledForEventCallback_CategoryGroupEnabledFlags:
    CategoryGroupEnabledFlags = 4;
pub const CategoryGroupEnabledFlags_kEnabledForETWExport_CategoryGroupEnabledFlags:
    CategoryGroupEnabledFlags = 8;
thread_local!(
    pub static kZeroNumArgs_152: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kGlobalScope_153: Value<Value<AnyPtr>> = Rc::new(RefCell::new(Default::default()));
);
thread_local!(
    pub static kNoId_154: Value<u64> = Rc::new(RefCell::new(0_u64));
);
#[derive(Default)]
pub struct v8_internal_tracing_TraceEventHelper {}
impl Clone for v8_internal_tracing_TraceEventHelper {
    fn clone(&self) -> Self {
        let __this: Value<v8_internal_tracing_TraceEventHelper> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_internal_tracing_TraceEventHelper> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_internal_tracing_TraceEventHelper {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_internal_tracing_TraceID_WithScope {
    scope_: Value<Ptr<u8>>,
    raw_id_: Value<u64>,
}
impl v8_internal_tracing_TraceID_WithScope {
    pub fn v8_internal_tracing_TraceID_WithScope(scope: Ptr<u8>, raw_id: u64) -> Self {
        let scope: Value<Ptr<u8>> = Rc::new(RefCell::new(scope));
        let raw_id: Value<u64> = Rc::new(RefCell::new(raw_id));
        let __this: Value<v8_internal_tracing_TraceID_WithScope> = Rc::new(RefCell::new(Self {
            scope_: Rc::new(RefCell::new((*scope.borrow()).clone())),
            raw_id_: Rc::new(RefCell::new((*raw_id.borrow()))),
        }));
        let this: Ptr<v8_internal_tracing_TraceID_WithScope> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_internal_tracing_TraceID_WithScope {
    fn clone(&self) -> Self {
        let __this: Value<v8_internal_tracing_TraceID_WithScope> = Rc::new(RefCell::new(Self {
            scope_: Rc::new(RefCell::new((*self.scope_.borrow()).clone())),
            raw_id_: Rc::new(RefCell::new((*self.raw_id_.borrow()))),
        }));
        let this: Ptr<v8_internal_tracing_TraceID_WithScope> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_internal_tracing_TraceID_WithScope {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.scope_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.raw_id_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            scope_: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[0..8]))),
            raw_id_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Default)]
pub struct v8_internal_tracing_TraceID {
    scope_: Value<Ptr<u8>>,
    raw_id_: Value<u64>,
}
impl v8_internal_tracing_TraceID {
    pub fn v8_internal_tracing_TraceID1(raw_id: AnyPtr, flags: Ptr<u32>) -> Self {
        let raw_id: Value<AnyPtr> = Rc::new(RefCell::new(raw_id));
        let flags: Value<Ptr<u32>> = Rc::new(RefCell::new(flags));
        let __this: Value<v8_internal_tracing_TraceID> = Rc::new(RefCell::new(Self {
            scope_: Rc::new(RefCell::new(Ptr::<u8>::null())),
            raw_id_: Rc::new(RefCell::new((*raw_id.borrow()).to_int())),
        }));
        let this: Ptr<v8_internal_tracing_TraceID> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_internal_tracing_TraceID2(raw_id: u64, flags: Ptr<u32>) -> Self {
        let raw_id: Value<u64> = Rc::new(RefCell::new(raw_id));
        let flags: Value<Ptr<u32>> = Rc::new(RefCell::new(flags));
        let __this: Value<v8_internal_tracing_TraceID> = Rc::new(RefCell::new(Self {
            scope_: Rc::new(RefCell::new(Ptr::<u8>::null())),
            raw_id_: Rc::new(RefCell::new((*raw_id.borrow()))),
        }));
        let this: Ptr<v8_internal_tracing_TraceID> = __this.as_pointer();
        &(*flags.borrow_mut());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_internal_tracing_TraceID3(raw_id: u32, flags: Ptr<u32>) -> Self {
        let raw_id: Value<u32> = Rc::new(RefCell::new(raw_id));
        let flags: Value<Ptr<u32>> = Rc::new(RefCell::new(flags));
        let __this: Value<v8_internal_tracing_TraceID> = Rc::new(RefCell::new(Self {
            scope_: Rc::new(RefCell::new(Ptr::<u8>::null())),
            raw_id_: Rc::new(RefCell::new(((*raw_id.borrow()) as u64))),
        }));
        let this: Ptr<v8_internal_tracing_TraceID> = __this.as_pointer();
        &(*flags.borrow_mut());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_internal_tracing_TraceID4(raw_id: u16, flags: Ptr<u32>) -> Self {
        let raw_id: Value<u16> = Rc::new(RefCell::new(raw_id));
        let flags: Value<Ptr<u32>> = Rc::new(RefCell::new(flags));
        let __this: Value<v8_internal_tracing_TraceID> = Rc::new(RefCell::new(Self {
            scope_: Rc::new(RefCell::new(Ptr::<u8>::null())),
            raw_id_: Rc::new(RefCell::new(((*raw_id.borrow()) as u64))),
        }));
        let this: Ptr<v8_internal_tracing_TraceID> = __this.as_pointer();
        &(*flags.borrow_mut());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_internal_tracing_TraceID5(raw_id: u8, flags: Ptr<u32>) -> Self {
        let raw_id: Value<u8> = Rc::new(RefCell::new(raw_id));
        let flags: Value<Ptr<u32>> = Rc::new(RefCell::new(flags));
        let __this: Value<v8_internal_tracing_TraceID> = Rc::new(RefCell::new(Self {
            scope_: Rc::new(RefCell::new(Ptr::<u8>::null())),
            raw_id_: Rc::new(RefCell::new(((*raw_id.borrow()) as u64))),
        }));
        let this: Ptr<v8_internal_tracing_TraceID> = __this.as_pointer();
        &(*flags.borrow_mut());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_internal_tracing_TraceID6(raw_id: i64, flags: Ptr<u32>) -> Self {
        let raw_id: Value<i64> = Rc::new(RefCell::new(raw_id));
        let flags: Value<Ptr<u32>> = Rc::new(RefCell::new(flags));
        let __this: Value<v8_internal_tracing_TraceID> = Rc::new(RefCell::new(Self {
            scope_: Rc::new(RefCell::new(Ptr::<u8>::null())),
            raw_id_: Rc::new(RefCell::new(((*raw_id.borrow()) as u64))),
        }));
        let this: Ptr<v8_internal_tracing_TraceID> = __this.as_pointer();
        &(*flags.borrow_mut());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_internal_tracing_TraceID7(raw_id: i32, flags: Ptr<u32>) -> Self {
        let raw_id: Value<i32> = Rc::new(RefCell::new(raw_id));
        let flags: Value<Ptr<u32>> = Rc::new(RefCell::new(flags));
        let __this: Value<v8_internal_tracing_TraceID> = Rc::new(RefCell::new(Self {
            scope_: Rc::new(RefCell::new(Ptr::<u8>::null())),
            raw_id_: Rc::new(RefCell::new(((*raw_id.borrow()) as u64))),
        }));
        let this: Ptr<v8_internal_tracing_TraceID> = __this.as_pointer();
        &(*flags.borrow_mut());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_internal_tracing_TraceID8(raw_id: i16, flags: Ptr<u32>) -> Self {
        let raw_id: Value<i16> = Rc::new(RefCell::new(raw_id));
        let flags: Value<Ptr<u32>> = Rc::new(RefCell::new(flags));
        let __this: Value<v8_internal_tracing_TraceID> = Rc::new(RefCell::new(Self {
            scope_: Rc::new(RefCell::new(Ptr::<u8>::null())),
            raw_id_: Rc::new(RefCell::new(((*raw_id.borrow()) as u64))),
        }));
        let this: Ptr<v8_internal_tracing_TraceID> = __this.as_pointer();
        &(*flags.borrow_mut());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_internal_tracing_TraceID9(raw_id: i8, flags: Ptr<u32>) -> Self {
        let raw_id: Value<i8> = Rc::new(RefCell::new(raw_id));
        let flags: Value<Ptr<u32>> = Rc::new(RefCell::new(flags));
        let __this: Value<v8_internal_tracing_TraceID> = Rc::new(RefCell::new(Self {
            scope_: Rc::new(RefCell::new(Ptr::<u8>::null())),
            raw_id_: Rc::new(RefCell::new(((*raw_id.borrow()) as u64))),
        }));
        let this: Ptr<v8_internal_tracing_TraceID> = __this.as_pointer();
        &(*flags.borrow_mut());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_internal_tracing_TraceID10(
        scoped_id: v8_internal_tracing_TraceID_WithScope,
        flags: Ptr<u32>,
    ) -> Self {
        let scoped_id: Value<v8_internal_tracing_TraceID_WithScope> =
            Rc::new(RefCell::new(scoped_id));
        let flags: Value<Ptr<u32>> = Rc::new(RefCell::new(flags));
        let __this: Value<v8_internal_tracing_TraceID> = Rc::new(RefCell::new(Self {
            scope_: Rc::new(RefCell::new(
                ({ v8_internal_tracing_TraceID_WithScopeImpl::scope(&scoped_id.as_pointer()) }),
            )),
            raw_id_: Rc::new(RefCell::new(
                ({ v8_internal_tracing_TraceID_WithScopeImpl::raw_id(&scoped_id.as_pointer()) }),
            )),
        }));
        let this: Ptr<v8_internal_tracing_TraceID> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_internal_tracing_TraceID {
    fn clone(&self) -> Self {
        let __this: Value<v8_internal_tracing_TraceID> = Rc::new(RefCell::new(Self {
            scope_: Rc::new(RefCell::new((*self.scope_.borrow()).clone())),
            raw_id_: Rc::new(RefCell::new((*self.raw_id_.borrow()))),
        }));
        let this: Ptr<v8_internal_tracing_TraceID> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_internal_tracing_TraceID {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.scope_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.raw_id_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            scope_: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[0..8]))),
            raw_id_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Default)]
pub struct v8_internal_tracing_TraceStringWithCopy {
    str_: Value<Ptr<u8>>,
}
impl v8_internal_tracing_TraceStringWithCopy {
    pub fn v8_internal_tracing_TraceStringWithCopy(str: Ptr<u8>) -> Self {
        let str: Value<Ptr<u8>> = Rc::new(RefCell::new(str));
        let __this: Value<v8_internal_tracing_TraceStringWithCopy> = Rc::new(RefCell::new(Self {
            str_: Rc::new(RefCell::new((*str.borrow()).clone())),
        }));
        let this: Ptr<v8_internal_tracing_TraceStringWithCopy> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_internal_tracing_TraceStringWithCopy {
    fn clone(&self) -> Self {
        let __this: Value<v8_internal_tracing_TraceStringWithCopy> = Rc::new(RefCell::new(Self {
            str_: Rc::new(RefCell::new((*self.str_.borrow()).clone())),
        }));
        let this: Ptr<v8_internal_tracing_TraceStringWithCopy> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_internal_tracing_TraceStringWithCopy {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.str_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            str_: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[0..8]))),
        }
    }
}
pub fn AddTraceEventImpl_155(
    phase: u8,
    category_group_enabled: Ptr<u8>,
    name: Ptr<u8>,
    scope: Ptr<u8>,
    id: u64,
    bind_id: u64,
    num_args: i32,
    arg_names: Ptr<Ptr<u8>>,
    arg_types: Ptr<u8>,
    arg_values: Ptr<u64>,
    flags: u32,
) -> u64 {
    let phase: Value<u8> = Rc::new(RefCell::new(phase));
    let category_group_enabled: Value<Ptr<u8>> = Rc::new(RefCell::new(category_group_enabled));
    let name: Value<Ptr<u8>> = Rc::new(RefCell::new(name));
    let scope: Value<Ptr<u8>> = Rc::new(RefCell::new(scope));
    let id: Value<u64> = Rc::new(RefCell::new(id));
    let bind_id: Value<u64> = Rc::new(RefCell::new(bind_id));
    let num_args: Value<i32> = Rc::new(RefCell::new(num_args));
    let arg_names: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(arg_names));
    let arg_types: Value<Ptr<u8>> = Rc::new(RefCell::new(arg_types));
    let arg_values: Value<Ptr<u64>> = Rc::new(RefCell::new(arg_values));
    let flags: Value<u32> = Rc::new(RefCell::new(flags));
    let arg_convertables: Value<Box<[Option<Value<v8_ConvertableToTraceFormat>>]>> =
        Rc::new(RefCell::new(None));
    if ((*num_args.borrow()) > 0)
        && ((((*arg_types.borrow()).offset((0) as isize).read()) as i32) == ((8_u8) as i32))
    {
        {
            let _p: Ptr<_> = (((*arg_values.borrow()).offset((0) as isize).read()) as i64)
                .reinterpret_cast::<v8_ConvertableToTraceFormat>();
            (*arg_convertables.borrow_mut())[(0) as usize] = _p.to_owned_opt()
        };
    }
    if ((*num_args.borrow()) > 1)
        && ((((*arg_types.borrow()).offset((1) as isize).read()) as i32) == ((8_u8) as i32))
    {
        {
            let _p: Ptr<_> = (((*arg_values.borrow()).offset((1) as isize).read()) as i64)
                .reinterpret_cast::<v8_ConvertableToTraceFormat>();
            (*arg_convertables.borrow_mut())[(1) as usize] = _p.to_owned_opt()
        };
    }
    (&(0));
    let controller: Value<Ptr<v8_TracingController>> = Rc::new(RefCell::new(
        ({ v8_internal_tracing_TraceEventHelper::GetTracingController() }),
    ));
    return ({
        let _phase: u8 = (*phase.borrow());
        let _name: Ptr<u8> = (*name.borrow()).clone();
        let _scope: Ptr<u8> = (*scope.borrow()).clone();
        let _id: u64 = (*id.borrow());
        let _bind_id: u64 = (*bind_id.borrow());
        let _arg_names: Ptr<Ptr<u8>> = (*arg_names.borrow()).clone();
        let _arg_values: Ptr<u64> = (*arg_values.borrow()).clone();
        (*(*controller.borrow()).upgrade().deref()).AddTraceEvent(
            _phase,
            (*category_group_enabled.borrow()).clone(),
            _name,
            _scope,
            _id,
            _bind_id,
            (*num_args.borrow()),
            _arg_names,
            (*arg_types.borrow()).clone(),
            _arg_values,
            (arg_convertables.as_pointer() as Ptr<Option<Value<v8_ConvertableToTraceFormat>>>),
            (*flags.borrow()),
        )
    });
}
pub fn AddTraceEventWithTimestampImpl_156(
    phase: u8,
    category_group_enabled: Ptr<u8>,
    name: Ptr<u8>,
    scope: Ptr<u8>,
    id: u64,
    bind_id: u64,
    num_args: i32,
    arg_names: Ptr<Ptr<u8>>,
    arg_types: Ptr<u8>,
    arg_values: Ptr<u64>,
    flags: u32,
    timestamp: i64,
) -> u64 {
    let phase: Value<u8> = Rc::new(RefCell::new(phase));
    let category_group_enabled: Value<Ptr<u8>> = Rc::new(RefCell::new(category_group_enabled));
    let name: Value<Ptr<u8>> = Rc::new(RefCell::new(name));
    let scope: Value<Ptr<u8>> = Rc::new(RefCell::new(scope));
    let id: Value<u64> = Rc::new(RefCell::new(id));
    let bind_id: Value<u64> = Rc::new(RefCell::new(bind_id));
    let num_args: Value<i32> = Rc::new(RefCell::new(num_args));
    let arg_names: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(arg_names));
    let arg_types: Value<Ptr<u8>> = Rc::new(RefCell::new(arg_types));
    let arg_values: Value<Ptr<u64>> = Rc::new(RefCell::new(arg_values));
    let flags: Value<u32> = Rc::new(RefCell::new(flags));
    let timestamp: Value<i64> = Rc::new(RefCell::new(timestamp));
    let arg_convertables: Value<Box<[Option<Value<v8_ConvertableToTraceFormat>>]>> =
        Rc::new(RefCell::new(None));
    if ((*num_args.borrow()) > 0)
        && ((((*arg_types.borrow()).offset((0) as isize).read()) as i32) == ((8_u8) as i32))
    {
        {
            let _p: Ptr<_> = (((*arg_values.borrow()).offset((0) as isize).read()) as i64)
                .reinterpret_cast::<v8_ConvertableToTraceFormat>();
            (*arg_convertables.borrow_mut())[(0) as usize] = _p.to_owned_opt()
        };
    }
    if ((*num_args.borrow()) > 1)
        && ((((*arg_types.borrow()).offset((1) as isize).read()) as i32) == ((8_u8) as i32))
    {
        {
            let _p: Ptr<_> = (((*arg_values.borrow()).offset((1) as isize).read()) as i64)
                .reinterpret_cast::<v8_ConvertableToTraceFormat>();
            (*arg_convertables.borrow_mut())[(1) as usize] = _p.to_owned_opt()
        };
    }
    (&(0));
    let controller: Value<Ptr<v8_TracingController>> = Rc::new(RefCell::new(
        ({ v8_internal_tracing_TraceEventHelper::GetTracingController() }),
    ));
    return ({
        let _phase: u8 = (*phase.borrow());
        let _name: Ptr<u8> = (*name.borrow()).clone();
        let _scope: Ptr<u8> = (*scope.borrow()).clone();
        let _id: u64 = (*id.borrow());
        let _bind_id: u64 = (*bind_id.borrow());
        let _arg_names: Ptr<Ptr<u8>> = (*arg_names.borrow()).clone();
        let _arg_values: Ptr<u64> = (*arg_values.borrow()).clone();
        (*(*controller.borrow()).upgrade().deref()).AddTraceEventWithTimestamp(
            _phase,
            (*category_group_enabled.borrow()).clone(),
            _name,
            _scope,
            _id,
            _bind_id,
            (*num_args.borrow()),
            _arg_names,
            (*arg_types.borrow()).clone(),
            _arg_values,
            (arg_convertables.as_pointer() as Ptr<Option<Value<v8_ConvertableToTraceFormat>>>),
            (*flags.borrow()),
            (*timestamp.borrow()),
        )
    });
}
pub fn SetTraceValue_157(arg: f64, type_: Ptr<u8>, value: Ptr<u64>) {
    let arg: Value<f64> = Rc::new(RefCell::new(arg));
    let type_: Value<Ptr<u8>> = Rc::new(RefCell::new(type_));
    let value: Value<Ptr<u64>> = Rc::new(RefCell::new(value));
    (*type_.borrow()).write((4_u8));
    (*value.borrow()).write(0_u64);
    {
        ((*value.borrow()).clone() as Ptr<u64>).to_any().memcpy(
            &((arg.as_pointer()) as Ptr<f64>).to_any(),
            ::std::mem::size_of::<f64>() as usize,
        );
        ((*value.borrow()).clone() as Ptr<u64>).to_any()
    };
}
pub fn SetTraceValue_158(arg: AnyPtr, type_: Ptr<u8>, value: Ptr<u64>) {
    let arg: Value<AnyPtr> = Rc::new(RefCell::new(arg));
    let type_: Value<Ptr<u8>> = Rc::new(RefCell::new(type_));
    let value: Value<Ptr<u64>> = Rc::new(RefCell::new(value));
    (*type_.borrow()).write((5_u8));
    (*value.borrow()).write(0_u64);
    {
        ((*value.borrow()).clone() as Ptr<u64>).to_any().memcpy(
            &((arg.as_pointer()) as Ptr<AnyPtr>).to_any(),
            ::std::mem::size_of::<AnyPtr>() as usize,
        );
        ((*value.borrow()).clone() as Ptr<u64>).to_any()
    };
}
pub fn SetTraceValue_159(arg: Ptr<u8>, type_: Ptr<u8>, value: Ptr<u64>) {
    let arg: Value<Ptr<u8>> = Rc::new(RefCell::new(arg));
    let type_: Value<Ptr<u8>> = Rc::new(RefCell::new(type_));
    let value: Value<Ptr<u64>> = Rc::new(RefCell::new(value));
    (*type_.borrow()).write((6_u8));
    (*value.borrow()).write(0_u64);
    {
        ((*value.borrow()).clone() as Ptr<u64>).to_any().memcpy(
            &((arg.as_pointer()) as Ptr<Ptr<u8>>).to_any(),
            ::std::mem::size_of::<Ptr<u8>>() as usize,
        );
        ((*value.borrow()).clone() as Ptr<u64>).to_any()
    };
}
pub fn SetTraceValue_160(
    arg: Ptr<v8_internal_tracing_TraceStringWithCopy>,
    type_: Ptr<u8>,
    value: Ptr<u64>,
) {
    let type_: Value<Ptr<u8>> = Rc::new(RefCell::new(type_));
    let value: Value<Ptr<u64>> = Rc::new(RefCell::new(value));
    (*type_.borrow()).write((7_u8));
    (*value.borrow()).write(0_u64);
    {
        ((*value.borrow()).clone() as Ptr<u64>).to_any().memcpy(
            &((arg).clone() as Ptr<v8_internal_tracing_TraceStringWithCopy>).to_any(),
            8usize as usize,
        );
        ((*value.borrow()).clone() as Ptr<u64>).to_any()
    };
}
pub fn SetTraceValue_161(
    convertable_value: PtrDyn<dyn v8_ConvertableToTraceFormat>,
    type_: Ptr<u8>,
    value: Ptr<u64>,
) {
    let convertable_value: Value<PtrDyn<dyn v8_ConvertableToTraceFormat>> =
        Rc::new(RefCell::new(convertable_value));
    let type_: Value<Ptr<u8>> = Rc::new(RefCell::new(type_));
    let value: Value<Ptr<u64>> = Rc::new(RefCell::new(value));
    (*type_.borrow()).write((8_u8));
    let __rhs = ((*convertable_value.borrow()).to_int() as u64);
    (*value.borrow()).write(__rhs);
}
pub fn AddTraceEvent_162(
    phase: u8,
    category_group_enabled: Ptr<u8>,
    name: Ptr<u8>,
    scope: Ptr<u8>,
    id: u64,
    bind_id: u64,
    flags: u32,
) -> u64 {
    let phase: Value<u8> = Rc::new(RefCell::new(phase));
    let category_group_enabled: Value<Ptr<u8>> = Rc::new(RefCell::new(category_group_enabled));
    let name: Value<Ptr<u8>> = Rc::new(RefCell::new(name));
    let scope: Value<Ptr<u8>> = Rc::new(RefCell::new(scope));
    let id: Value<u64> = Rc::new(RefCell::new(id));
    let bind_id: Value<u64> = Rc::new(RefCell::new(bind_id));
    let flags: Value<u32> = Rc::new(RefCell::new(flags));
    return ({
        let _phase: u8 = (*phase.borrow());
        let _name: Ptr<u8> = (*name.borrow()).clone();
        let _scope: Ptr<u8> = (*scope.borrow()).clone();
        AddTraceEventImpl_155(
            _phase,
            (*category_group_enabled.borrow()).clone(),
            _name,
            _scope,
            (*id.borrow()),
            (*bind_id.borrow()),
            (*kZeroNumArgs_152.with(Value::clone).borrow()),
            Ptr::<Ptr<u8>>::null(),
            Ptr::<u8>::null(),
            Ptr::<u64>::null(),
            (*flags.borrow()),
        )
    });
}
pub fn AddTraceEventWithTimestamp_163(
    phase: u8,
    category_group_enabled: Ptr<u8>,
    name: Ptr<u8>,
    scope: Ptr<u8>,
    id: u64,
    bind_id: u64,
    flags: u32,
    timestamp: i64,
) -> u64 {
    let phase: Value<u8> = Rc::new(RefCell::new(phase));
    let category_group_enabled: Value<Ptr<u8>> = Rc::new(RefCell::new(category_group_enabled));
    let name: Value<Ptr<u8>> = Rc::new(RefCell::new(name));
    let scope: Value<Ptr<u8>> = Rc::new(RefCell::new(scope));
    let id: Value<u64> = Rc::new(RefCell::new(id));
    let bind_id: Value<u64> = Rc::new(RefCell::new(bind_id));
    let flags: Value<u32> = Rc::new(RefCell::new(flags));
    let timestamp: Value<i64> = Rc::new(RefCell::new(timestamp));
    return ({
        let _phase: u8 = (*phase.borrow());
        let _name: Ptr<u8> = (*name.borrow()).clone();
        let _scope: Ptr<u8> = (*scope.borrow()).clone();
        AddTraceEventWithTimestampImpl_156(
            _phase,
            (*category_group_enabled.borrow()).clone(),
            _name,
            _scope,
            (*id.borrow()),
            (*bind_id.borrow()),
            (*kZeroNumArgs_152.with(Value::clone).borrow()),
            Ptr::<Ptr<u8>>::null(),
            Ptr::<u8>::null(),
            Ptr::<u64>::null(),
            (*flags.borrow()),
            (*timestamp.borrow()),
        )
    });
}
#[derive(Default)]
struct v8_internal_tracing_ScopedTracer_Data {
    pub category_group_enabled: Value<Ptr<u8>>,
    pub name: Value<Ptr<u8>>,
    pub event_handle: Value<u64>,
}
impl Clone for v8_internal_tracing_ScopedTracer_Data {
    fn clone(&self) -> Self {
        let __this: Value<v8_internal_tracing_ScopedTracer_Data> = Rc::new(RefCell::new(Self {
            category_group_enabled: Rc::new(RefCell::new(
                (*self.category_group_enabled.borrow()).clone(),
            )),
            name: Rc::new(RefCell::new((*self.name.borrow()).clone())),
            event_handle: Rc::new(RefCell::new((*self.event_handle.borrow()))),
        }));
        let this: Ptr<v8_internal_tracing_ScopedTracer_Data> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_internal_tracing_ScopedTracer_Data {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.category_group_enabled.borrow()).to_bytes(&mut buf[0..8]);
        (*self.name.borrow()).to_bytes(&mut buf[8..16]);
        (*self.event_handle.borrow()).to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            category_group_enabled: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[0..8]))),
            name: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[8..16]))),
            event_handle: Rc::new(RefCell::new(<u64>::from_bytes(&buf[16..24]))),
        }
    }
}
#[derive()]
pub struct v8_internal_tracing_ScopedTracer {
    p_data_: Value<Ptr<v8_internal_tracing_ScopedTracer_Data>>,
    data_: Value<v8_internal_tracing_ScopedTracer_Data>,
}
impl v8_internal_tracing_ScopedTracer {
    pub fn v8_internal_tracing_ScopedTracer() -> Self {
        let __this: Value<v8_internal_tracing_ScopedTracer> = Rc::new(RefCell::new(Self {
            p_data_: Rc::new(RefCell::new(
                Ptr::<v8_internal_tracing_ScopedTracer_Data>::null(),
            )),
            data_: Rc::new(RefCell::new(
                <v8_internal_tracing_ScopedTracer_Data>::default(),
            )),
        }));
        let this: Ptr<v8_internal_tracing_ScopedTracer> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_internal_tracing_ScopedTracer {
    fn clone(&self) -> Self {
        let __this: Value<v8_internal_tracing_ScopedTracer> = Rc::new(RefCell::new(Self {
            p_data_: Rc::new(RefCell::new((*self.p_data_.borrow()).clone())),
            data_: Rc::new(RefCell::new((*self.data_.borrow()).clone())),
        }));
        let this: Ptr<v8_internal_tracing_ScopedTracer> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_internal_tracing_ScopedTracer {
    fn default() -> Self {
        { v8_internal_tracing_ScopedTracer::v8_internal_tracing_ScopedTracer() }
    }
}
impl ByteRepr for v8_internal_tracing_ScopedTracer {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.p_data_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.data_.borrow()).to_bytes(&mut buf[8..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            p_data_: Rc::new(RefCell::new(
                <Ptr<v8_internal_tracing_ScopedTracer_Data>>::from_bytes(&buf[0..8]),
            )),
            data_: Rc::new(RefCell::new(
                <v8_internal_tracing_ScopedTracer_Data>::from_bytes(&buf[8..32]),
            )),
        }
    }
}
pub type cppgc_internal_StatsCollector_ScopeId = u32;
pub const cppgc_internal_StatsCollector_ScopeId_kAtomicMark: cppgc_internal_StatsCollector_ScopeId =
    0;
pub const cppgc_internal_StatsCollector_ScopeId_kAtomicWeak: cppgc_internal_StatsCollector_ScopeId =
    1;
pub const cppgc_internal_StatsCollector_ScopeId_kAtomicCompact:
    cppgc_internal_StatsCollector_ScopeId = 2;
pub const cppgc_internal_StatsCollector_ScopeId_kAtomicSweep:
    cppgc_internal_StatsCollector_ScopeId = 3;
pub const cppgc_internal_StatsCollector_ScopeId_kIncrementalMark:
    cppgc_internal_StatsCollector_ScopeId = 4;
pub const cppgc_internal_StatsCollector_ScopeId_kIncrementalSweep:
    cppgc_internal_StatsCollector_ScopeId = 5;
pub const cppgc_internal_StatsCollector_ScopeId_kNumHistogramScopeIds:
    cppgc_internal_StatsCollector_ScopeId = 6;
pub const cppgc_internal_StatsCollector_ScopeId_kUnmark: cppgc_internal_StatsCollector_ScopeId = 7;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkIncrementalStart:
    cppgc_internal_StatsCollector_ScopeId = 8;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkIncrementalFinalize:
    cppgc_internal_StatsCollector_ScopeId = 9;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkAtomicPrologue:
    cppgc_internal_StatsCollector_ScopeId = 10;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkAtomicEpilogue:
    cppgc_internal_StatsCollector_ScopeId = 11;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkTransitiveClosure:
    cppgc_internal_StatsCollector_ScopeId = 12;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkTransitiveClosureWithDeadline:
    cppgc_internal_StatsCollector_ScopeId = 13;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkFlushEphemerons:
    cppgc_internal_StatsCollector_ScopeId = 14;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkOnAllocation:
    cppgc_internal_StatsCollector_ScopeId = 15;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkProcessBailOutObjects:
    cppgc_internal_StatsCollector_ScopeId = 16;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkProcessMarkingWorklist:
    cppgc_internal_StatsCollector_ScopeId = 17;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkProcessRetraceWorklist:
    cppgc_internal_StatsCollector_ScopeId = 18;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkProcessWriteBarrierWorklist:
    cppgc_internal_StatsCollector_ScopeId = 19;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkProcessNotFullyconstructedWorklist:
    cppgc_internal_StatsCollector_ScopeId = 20;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkProcessEphemerons:
    cppgc_internal_StatsCollector_ScopeId = 21;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkVisitRoots:
    cppgc_internal_StatsCollector_ScopeId = 22;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkVisitNotFullyConstructedObjects:
    cppgc_internal_StatsCollector_ScopeId = 23;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkVisitPersistents:
    cppgc_internal_StatsCollector_ScopeId = 24;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkVisitCrossThreadPersistents:
    cppgc_internal_StatsCollector_ScopeId = 25;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkVisitStack:
    cppgc_internal_StatsCollector_ScopeId = 26;
pub const cppgc_internal_StatsCollector_ScopeId_kMarkVisitRememberedSets:
    cppgc_internal_StatsCollector_ScopeId = 27;
pub const cppgc_internal_StatsCollector_ScopeId_kWeakContainerCallbacksProcessing:
    cppgc_internal_StatsCollector_ScopeId = 28;
pub const cppgc_internal_StatsCollector_ScopeId_kCustomCallbacksProcessing:
    cppgc_internal_StatsCollector_ScopeId = 29;
pub const cppgc_internal_StatsCollector_ScopeId_kSweepEmptyPages:
    cppgc_internal_StatsCollector_ScopeId = 30;
pub const cppgc_internal_StatsCollector_ScopeId_kSweepFinish:
    cppgc_internal_StatsCollector_ScopeId = 31;
pub const cppgc_internal_StatsCollector_ScopeId_kSweepFinalizeEmptyPages:
    cppgc_internal_StatsCollector_ScopeId = 32;
pub const cppgc_internal_StatsCollector_ScopeId_kSweepFinalizeSweptPages:
    cppgc_internal_StatsCollector_ScopeId = 33;
pub const cppgc_internal_StatsCollector_ScopeId_kSweepFinishIfOutOfWork:
    cppgc_internal_StatsCollector_ScopeId = 34;
pub const cppgc_internal_StatsCollector_ScopeId_kSweepInvokePreFinalizers:
    cppgc_internal_StatsCollector_ScopeId = 35;
pub const cppgc_internal_StatsCollector_ScopeId_kSweepInLowPriorityTask:
    cppgc_internal_StatsCollector_ScopeId = 36;
pub const cppgc_internal_StatsCollector_ScopeId_kSweepInTask:
    cppgc_internal_StatsCollector_ScopeId = 37;
pub const cppgc_internal_StatsCollector_ScopeId_kSweepInTaskForStatistics:
    cppgc_internal_StatsCollector_ScopeId = 38;
pub const cppgc_internal_StatsCollector_ScopeId_kSweepOnAllocation:
    cppgc_internal_StatsCollector_ScopeId = 39;
pub const cppgc_internal_StatsCollector_ScopeId_kSweepPages: cppgc_internal_StatsCollector_ScopeId =
    40;
pub const cppgc_internal_StatsCollector_ScopeId_kNumScopeIds:
    cppgc_internal_StatsCollector_ScopeId = 41;
pub type cppgc_internal_StatsCollector_ConcurrentScopeId = u32;
pub const cppgc_internal_StatsCollector_ConcurrentScopeId_kConcurrentMark:
    cppgc_internal_StatsCollector_ConcurrentScopeId = 0;
pub const cppgc_internal_StatsCollector_ConcurrentScopeId_kConcurrentSweep:
    cppgc_internal_StatsCollector_ConcurrentScopeId = 1;
pub const cppgc_internal_StatsCollector_ConcurrentScopeId_kConcurrentWeakCallback:
    cppgc_internal_StatsCollector_ConcurrentScopeId = 2;
pub const cppgc_internal_StatsCollector_ConcurrentScopeId_kNumHistogramConcurrentScopeIds:
    cppgc_internal_StatsCollector_ConcurrentScopeId = 3;
pub const cppgc_internal_StatsCollector_ConcurrentScopeId_kConcurrentMarkProcessEphemeronWorklist: cppgc_internal_StatsCollector_ConcurrentScopeId = 4;
pub const cppgc_internal_StatsCollector_ConcurrentScopeId_kConcurrentMarkProcessMarkingWorklist:
    cppgc_internal_StatsCollector_ConcurrentScopeId = 5;
pub const cppgc_internal_StatsCollector_ConcurrentScopeId_kConcurrentMarkProcessNotFullyconstructedWorklist: cppgc_internal_StatsCollector_ConcurrentScopeId = 6;
pub const cppgc_internal_StatsCollector_ConcurrentScopeId_kConcurrentMarkProcessWriteBarrierWorklist: cppgc_internal_StatsCollector_ConcurrentScopeId = 7;
pub const cppgc_internal_StatsCollector_ConcurrentScopeId_kNumConcurrentScopeIds:
    cppgc_internal_StatsCollector_ConcurrentScopeId = 8;
pub type cppgc_internal_StatsCollector_TraceCategory = u32;
pub const cppgc_internal_StatsCollector_TraceCategory_kEnabled:
    cppgc_internal_StatsCollector_TraceCategory = 0;
pub const cppgc_internal_StatsCollector_TraceCategory_kDisabled:
    cppgc_internal_StatsCollector_TraceCategory = 1;
pub type cppgc_internal_StatsCollector_ScopeContext = u32;
pub const cppgc_internal_StatsCollector_ScopeContext_kMutatorThread:
    cppgc_internal_StatsCollector_ScopeContext = 0;
pub const cppgc_internal_StatsCollector_ScopeContext_kConcurrentThread:
    cppgc_internal_StatsCollector_ScopeContext = 1;
thread_local!(
    pub static kAllocationThresholdBytes_164: Value<usize> = Rc::new(RefCell::new(1024));
);
pub type cppgc_internal_StatsCollector_GarbageCollectionState = u8;
pub const cppgc_internal_StatsCollector_GarbageCollectionState_kNotRunning:
    cppgc_internal_StatsCollector_GarbageCollectionState = 0;
pub const cppgc_internal_StatsCollector_GarbageCollectionState_kUnmarking:
    cppgc_internal_StatsCollector_GarbageCollectionState = 1;
pub const cppgc_internal_StatsCollector_GarbageCollectionState_kMarking:
    cppgc_internal_StatsCollector_GarbageCollectionState = 2;
pub const cppgc_internal_StatsCollector_GarbageCollectionState_kSweeping:
    cppgc_internal_StatsCollector_GarbageCollectionState = 3;
#[derive()]
pub struct cppgc_internal_StatsCollector_Event {
    pub scope_data: Value<Box<[v8_base_TimeDelta]>>,
    pub concurrent_scope_data: Value<Box<[i64]>>,
    pub epoch: Value<usize>,
    pub collection_type: Value<cppgc_internal_CollectionType>,
    pub marking_type: Value<cppgc_Heap_MarkingType>,
    pub sweeping_type: Value<cppgc_Heap_SweepingType>,
    pub is_forced_gc: Value<cppgc_internal_MarkingConfig_IsForcedGC>,
    pub marked_bytes: Value<usize>,
    pub object_size_before_sweep_bytes: Value<usize>,
    pub memory_size_before_sweep_bytes: Value<usize>,
}
impl cppgc_internal_StatsCollector_Event {}
impl Clone for cppgc_internal_StatsCollector_Event {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_StatsCollector_Event> = Rc::new(RefCell::new(Self {
            scope_data: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 6, _>(
                |__i: usize| ((*self.scope_data.borrow())[(__i) as usize]).clone(),
            )))),
            concurrent_scope_data: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 3, _>(
                |__i: usize| (*self.concurrent_scope_data.borrow())[(__i) as usize],
            )))),
            epoch: Rc::new(RefCell::new((*self.epoch.borrow()))),
            collection_type: Rc::new(RefCell::new((*self.collection_type.borrow()))),
            marking_type: Rc::new(RefCell::new((*self.marking_type.borrow()))),
            sweeping_type: Rc::new(RefCell::new((*self.sweeping_type.borrow()))),
            is_forced_gc: Rc::new(RefCell::new((*self.is_forced_gc.borrow()))),
            marked_bytes: Rc::new(RefCell::new((*self.marked_bytes.borrow()))),
            object_size_before_sweep_bytes: Rc::new(RefCell::new(
                (*self.object_size_before_sweep_bytes.borrow()),
            )),
            memory_size_before_sweep_bytes: Rc::new(RefCell::new(
                (*self.memory_size_before_sweep_bytes.borrow()),
            )),
        }));
        let this: Ptr<cppgc_internal_StatsCollector_Event> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for cppgc_internal_StatsCollector_Event {
    fn default() -> Self {
        { cppgc_internal_StatsCollector_Event::cppgc_internal_StatsCollector_Event() }
    }
}
impl ByteRepr for cppgc_internal_StatsCollector_Event {
    fn byte_size() -> usize {
        112
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.scope_data.borrow()).to_bytes(&mut buf[0..48]);
        (*self.concurrent_scope_data.borrow()).to_bytes(&mut buf[48..72]);
        (*self.epoch.borrow()).to_bytes(&mut buf[72..80]);
        (*self.collection_type.borrow()).to_bytes(&mut buf[80..81]);
        (*self.marking_type.borrow()).to_bytes(&mut buf[81..82]);
        (*self.sweeping_type.borrow()).to_bytes(&mut buf[82..83]);
        (*self.is_forced_gc.borrow()).to_bytes(&mut buf[83..84]);
        (*self.marked_bytes.borrow()).to_bytes(&mut buf[88..96]);
        (*self.object_size_before_sweep_bytes.borrow()).to_bytes(&mut buf[96..104]);
        (*self.memory_size_before_sweep_bytes.borrow()).to_bytes(&mut buf[104..112]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            scope_data: Rc::new(RefCell::new(<Box<[v8_base_TimeDelta]>>::from_bytes(
                &buf[0..48],
            ))),
            concurrent_scope_data: Rc::new(RefCell::new(<Box<[i64]>>::from_bytes(&buf[48..72]))),
            epoch: Rc::new(RefCell::new(<usize>::from_bytes(&buf[72..80]))),
            collection_type: Rc::new(RefCell::new(<cppgc_internal_CollectionType>::from_bytes(
                &buf[80..81],
            ))),
            marking_type: Rc::new(RefCell::new(<cppgc_Heap_MarkingType>::from_bytes(
                &buf[81..82],
            ))),
            sweeping_type: Rc::new(RefCell::new(<cppgc_Heap_SweepingType>::from_bytes(
                &buf[82..83],
            ))),
            is_forced_gc: Rc::new(RefCell::new(
                <cppgc_internal_MarkingConfig_IsForcedGC>::from_bytes(&buf[83..84]),
            )),
            marked_bytes: Rc::new(RefCell::new(<usize>::from_bytes(&buf[88..96]))),
            object_size_before_sweep_bytes: Rc::new(RefCell::new(<usize>::from_bytes(
                &buf[96..104],
            ))),
            memory_size_before_sweep_bytes: Rc::new(RefCell::new(<usize>::from_bytes(
                &buf[104..112],
            ))),
        }
    }
}
#[derive(Default)]
pub struct cppgc_internal_StatsCollector_AllocationObserver {}
impl Clone for cppgc_internal_StatsCollector_AllocationObserver {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_StatsCollector_AllocationObserver> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<cppgc_internal_StatsCollector_AllocationObserver> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_StatsCollector_AllocationObserver {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct cppgc_internal_StatsCollector {
    allocated_bytes_since_end_of_marking_: Value<i64>,
    time_of_last_end_of_marking_: Value<v8_base_TimeTicks>,
    allocated_bytes_since_safepoint_: Value<i64>,
    explicitly_freed_bytes_since_safepoint_: Value<i64>,
    marked_bytes_so_far_: Value<usize>,
    memory_allocated_bytes_: Value<i64>,
    memory_freed_bytes_since_end_of_marking_: Value<i64>,
    discarded_bytes_: Value<std_atomic_unsigned_long_>,
    allocation_observers_: Value<Vec<Ptr<cppgc_internal_StatsCollector_AllocationObserver>>>,
    allocation_observer_deleted_: Value<bool>,
    gc_state_: Value<cppgc_internal_StatsCollector_GarbageCollectionState>,
    current_: Value<cppgc_internal_StatsCollector_Event>,
    previous_: Value<cppgc_internal_StatsCollector_Event>,
    metric_recorder_: Value<Option<Value<cppgc_internal_MetricRecorder>>>,
    platform_: Value<PtrDyn<dyn cppgc_Platform>>,
}
impl cppgc_internal_StatsCollector {
    fn GetScopeName_cppgc_internal_StatsCollector_ScopeId_cppgc_internal_CollectionType(
        id: cppgc_internal_StatsCollector_ScopeId,
        type_: cppgc_internal_CollectionType,
    ) -> Ptr<u8> {
        let id: Value<cppgc_internal_StatsCollector_ScopeId> = Rc::new(RefCell::new(id));
        let type_: Value<cppgc_internal_CollectionType> = Rc::new(RefCell::new(type_));
        'switch: {
            let __match_cond = ((*id.borrow()) as i32);
            match __match_cond {
                __v if __v == 0 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.AtomicMark")
                    } else {
                        Ptr::from_string_literal(b"CppGC.AtomicMark.Minor")
                    };
                }
                __v if __v == 1 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.AtomicWeak")
                    } else {
                        Ptr::from_string_literal(b"CppGC.AtomicWeak.Minor")
                    };
                }
                __v if __v == 2 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.AtomicCompact")
                    } else {
                        Ptr::from_string_literal(b"CppGC.AtomicCompact.Minor")
                    };
                }
                __v if __v == 3 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.AtomicSweep")
                    } else {
                        Ptr::from_string_literal(b"CppGC.AtomicSweep.Minor")
                    };
                }
                __v if __v == 4 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.IncrementalMark")
                    } else {
                        Ptr::from_string_literal(b"CppGC.IncrementalMark.Minor")
                    };
                }
                __v if __v == 5 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.IncrementalSweep")
                    } else {
                        Ptr::from_string_literal(b"CppGC.IncrementalSweep.Minor")
                    };
                }
                __v if __v == 7 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.Unmark")
                    } else {
                        Ptr::from_string_literal(b"CppGC.Unmark.Minor")
                    };
                }
                __v if __v == 8 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkIncrementalStart")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkIncrementalStart.Minor")
                    };
                }
                __v if __v == 9 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkIncrementalFinalize")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkIncrementalFinalize.Minor")
                    };
                }
                __v if __v == 10 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkAtomicPrologue")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkAtomicPrologue.Minor")
                    };
                }
                __v if __v == 11 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkAtomicEpilogue")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkAtomicEpilogue.Minor")
                    };
                }
                __v if __v == 12 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkTransitiveClosure")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkTransitiveClosure.Minor")
                    };
                }
                __v if __v == 13 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkTransitiveClosureWithDeadline")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkTransitiveClosureWithDeadline.Minor")
                    };
                }
                __v if __v == 14 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkFlushEphemerons")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkFlushEphemerons.Minor")
                    };
                }
                __v if __v == 15 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkOnAllocation")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkOnAllocation.Minor")
                    };
                }
                __v if __v == 16 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkProcessBailOutObjects")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkProcessBailOutObjects.Minor")
                    };
                }
                __v if __v == 17 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkProcessMarkingWorklist")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkProcessMarkingWorklist.Minor")
                    };
                }
                __v if __v == 18 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkProcessRetraceWorklist")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkProcessRetraceWorklist.Minor")
                    };
                }
                __v if __v == 19 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkProcessWriteBarrierWorklist")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkProcessWriteBarrierWorklist.Minor")
                    };
                }
                __v if __v == 20 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkProcessNotFullyconstructedWorklist")
                    } else {
                        Ptr::from_string_literal(
                            b"CppGC.MarkProcessNotFullyconstructedWorklist.Minor",
                        )
                    };
                }
                __v if __v == 21 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkProcessEphemerons")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkProcessEphemerons.Minor")
                    };
                }
                __v if __v == 22 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkVisitRoots")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkVisitRoots.Minor")
                    };
                }
                __v if __v == 23 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkVisitNotFullyConstructedObjects")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkVisitNotFullyConstructedObjects.Minor")
                    };
                }
                __v if __v == 24 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkVisitPersistents")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkVisitPersistents.Minor")
                    };
                }
                __v if __v == 25 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkVisitCrossThreadPersistents")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkVisitCrossThreadPersistents.Minor")
                    };
                }
                __v if __v == 26 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkVisitStack")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkVisitStack.Minor")
                    };
                }
                __v if __v == 27 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.MarkVisitRememberedSets")
                    } else {
                        Ptr::from_string_literal(b"CppGC.MarkVisitRememberedSets.Minor")
                    };
                }
                __v if __v == 28 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.WeakContainerCallbacksProcessing")
                    } else {
                        Ptr::from_string_literal(b"CppGC.WeakContainerCallbacksProcessing.Minor")
                    };
                }
                __v if __v == 29 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.CustomCallbacksProcessing")
                    } else {
                        Ptr::from_string_literal(b"CppGC.CustomCallbacksProcessing.Minor")
                    };
                }
                __v if __v == 30 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.SweepEmptyPages")
                    } else {
                        Ptr::from_string_literal(b"CppGC.SweepEmptyPages.Minor")
                    };
                }
                __v if __v == 31 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.SweepFinish")
                    } else {
                        Ptr::from_string_literal(b"CppGC.SweepFinish.Minor")
                    };
                }
                __v if __v == 32 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.SweepFinalizeEmptyPages")
                    } else {
                        Ptr::from_string_literal(b"CppGC.SweepFinalizeEmptyPages.Minor")
                    };
                }
                __v if __v == 33 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.SweepFinalizeSweptPages")
                    } else {
                        Ptr::from_string_literal(b"CppGC.SweepFinalizeSweptPages.Minor")
                    };
                }
                __v if __v == 34 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.SweepFinishIfOutOfWork")
                    } else {
                        Ptr::from_string_literal(b"CppGC.SweepFinishIfOutOfWork.Minor")
                    };
                }
                __v if __v == 35 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.SweepInvokePreFinalizers")
                    } else {
                        Ptr::from_string_literal(b"CppGC.SweepInvokePreFinalizers.Minor")
                    };
                }
                __v if __v == 36 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.SweepInLowPriorityTask")
                    } else {
                        Ptr::from_string_literal(b"CppGC.SweepInLowPriorityTask.Minor")
                    };
                }
                __v if __v == 37 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.SweepInTask")
                    } else {
                        Ptr::from_string_literal(b"CppGC.SweepInTask.Minor")
                    };
                }
                __v if __v == 38 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.SweepInTaskForStatistics")
                    } else {
                        Ptr::from_string_literal(b"CppGC.SweepInTaskForStatistics.Minor")
                    };
                }
                __v if __v == 39 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.SweepOnAllocation")
                    } else {
                        Ptr::from_string_literal(b"CppGC.SweepOnAllocation.Minor")
                    };
                }
                __v if __v == 40 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.SweepPages")
                    } else {
                        Ptr::from_string_literal(b"CppGC.SweepPages.Minor")
                    };
                }
                _ => {
                    return Ptr::<u8>::null();
                }
            }
        };
        panic!("ub: non-void function does not return a value")
    }
    fn GetScopeName_cppgc_internal_StatsCollector_ConcurrentScopeId_cppgc_internal_CollectionType(
        id: cppgc_internal_StatsCollector_ConcurrentScopeId,
        type_: cppgc_internal_CollectionType,
    ) -> Ptr<u8> {
        let id: Value<cppgc_internal_StatsCollector_ConcurrentScopeId> = Rc::new(RefCell::new(id));
        let type_: Value<cppgc_internal_CollectionType> = Rc::new(RefCell::new(type_));
        'switch: {
            let __match_cond = ((*id.borrow()) as i32);
            match __match_cond {
                __v if __v == 0 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.ConcurrentMark")
                    } else {
                        Ptr::from_string_literal(b"CppGC.ConcurrentMark.Minor")
                    };
                }
                __v if __v == 1 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.ConcurrentSweep")
                    } else {
                        Ptr::from_string_literal(b"CppGC.ConcurrentSweep.Minor")
                    };
                }
                __v if __v == 2 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.ConcurrentWeakCallback")
                    } else {
                        Ptr::from_string_literal(b"CppGC.ConcurrentWeakCallback.Minor")
                    };
                }
                __v if __v == 4 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.ConcurrentMarkProcessEphemeronWorklist")
                    } else {
                        Ptr::from_string_literal(
                            b"CppGC.ConcurrentMarkProcessEphemeronWorklist.Minor",
                        )
                    };
                }
                __v if __v == 5 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.ConcurrentMarkProcessMarkingWorklist")
                    } else {
                        Ptr::from_string_literal(
                            b"CppGC.ConcurrentMarkProcessMarkingWorklist.Minor",
                        )
                    };
                }
                __v if __v == 6 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(
                            b"CppGC.ConcurrentMarkProcessNotFullyconstructedWorklist",
                        )
                    } else {
                        Ptr::from_string_literal(
                            b"CppGC.ConcurrentMarkProcessNotFullyconstructedWorklist.Minor",
                        )
                    };
                }
                __v if __v == 7 => {
                    return if ((*type_.borrow()) == cppgc_internal_CollectionType_kMajor) {
                        Ptr::from_string_literal(b"CppGC.ConcurrentMarkProcessWriteBarrierWorklist")
                    } else {
                        Ptr::from_string_literal(
                            b"CppGC.ConcurrentMarkProcessWriteBarrierWorklist.Minor",
                        )
                    };
                }
                _ => {
                    return Ptr::<u8>::null();
                }
            }
        };
        panic!("ub: non-void function does not return a value")
    }
    pub fn Note(note: Ptr<u8>) {
        let note: Value<Ptr<u8>> = Rc::new(RefCell::new(note));
        (if false {
            ({
                let _args: Value<perfetto_DynamicString> = Rc::new(RefCell::new(
                    perfetto_DynamicString::perfetto_DynamicString({ (*note.borrow()).clone() }),
                ));
                Ignore_139(
                    Ptr::from_string_literal_array(b"disabled-by-default-cppgc"),
                    _args.as_pointer(),
                )
            })
        } else {
            &(0)
        });
    }
}
impl ByteRepr for cppgc_internal_StatsCollector {
    fn byte_size() -> usize {
        336
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.allocated_bytes_since_end_of_marking_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.time_of_last_end_of_marking_.borrow()).to_bytes(&mut buf[8..16]);
        (*self.allocated_bytes_since_safepoint_.borrow()).to_bytes(&mut buf[16..24]);
        (*self.explicitly_freed_bytes_since_safepoint_.borrow()).to_bytes(&mut buf[24..32]);
        (*self.marked_bytes_so_far_.borrow()).to_bytes(&mut buf[32..40]);
        (*self.memory_allocated_bytes_.borrow()).to_bytes(&mut buf[40..48]);
        (*self.memory_freed_bytes_since_end_of_marking_.borrow()).to_bytes(&mut buf[48..56]);
        (*self.discarded_bytes_.borrow()).to_bytes(&mut buf[56..64]);
        (*self.allocation_observers_.borrow()).to_bytes(&mut buf[64..88]);
        (*self.allocation_observer_deleted_.borrow()).to_bytes(&mut buf[88..89]);
        (*self.gc_state_.borrow()).to_bytes(&mut buf[89..90]);
        (*self.current_.borrow()).to_bytes(&mut buf[96..208]);
        (*self.previous_.borrow()).to_bytes(&mut buf[208..320]);
        (*self.metric_recorder_.borrow()).to_bytes(&mut buf[320..328]);
        (*self.platform_.borrow()).to_bytes(&mut buf[328..336]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            allocated_bytes_since_end_of_marking_: Rc::new(RefCell::new(<i64>::from_bytes(
                &buf[0..8],
            ))),
            time_of_last_end_of_marking_: Rc::new(RefCell::new(<v8_base_TimeTicks>::from_bytes(
                &buf[8..16],
            ))),
            allocated_bytes_since_safepoint_: Rc::new(RefCell::new(<i64>::from_bytes(
                &buf[16..24],
            ))),
            explicitly_freed_bytes_since_safepoint_: Rc::new(RefCell::new(<i64>::from_bytes(
                &buf[24..32],
            ))),
            marked_bytes_so_far_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[32..40]))),
            memory_allocated_bytes_: Rc::new(RefCell::new(<i64>::from_bytes(&buf[40..48]))),
            memory_freed_bytes_since_end_of_marking_: Rc::new(RefCell::new(<i64>::from_bytes(
                &buf[48..56],
            ))),
            discarded_bytes_: Rc::new(RefCell::new(<std_atomic_unsigned_long_>::from_bytes(
                &buf[56..64],
            ))),
            allocation_observers_: Rc::new(RefCell::new(<Vec<
                Ptr<cppgc_internal_StatsCollector_AllocationObserver>,
            >>::from_bytes(&buf[64..88]))),
            allocation_observer_deleted_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[88..89]))),
            gc_state_: Rc::new(RefCell::new(
                <cppgc_internal_StatsCollector_GarbageCollectionState>::from_bytes(&buf[89..90]),
            )),
            current_: Rc::new(RefCell::new(
                <cppgc_internal_StatsCollector_Event>::from_bytes(&buf[96..208]),
            )),
            previous_: Rc::new(RefCell::new(
                <cppgc_internal_StatsCollector_Event>::from_bytes(&buf[208..320]),
            )),
            metric_recorder_: Rc::new(RefCell::new(
                <Option<Value<cppgc_internal_MetricRecorder>>>::from_bytes(&buf[320..328]),
            )),
            platform_: Rc::new(RefCell::new(<PtrDyn<dyn cppgc_Platform>>::from_bytes(
                &buf[328..336],
            ))),
        }
    }
}
impl cppgc_internal_StatsCollector_InternalScope_cppgc_internal_StatsCollector_InternalScope_trace_category__cppgc_internal_StatsCollector_InternalScope_scope_category_ {   fn TraceCategory ( ) -> Ptr::<u8>  { 'switch: { let __match_cond = ( ( (*trace_category.borrow()) as i32 ) ) ; match __match_cond { __v if __v ==  0  =>  {  return Ptr::from_string_literal(b"cppgc" )   ;
 }, __v if __v ==  1  =>  {  return Ptr::from_string_literal(b"disabled-by-default-cppgc" )   ;
 },  _ => {} } } ;
 (  { V8_Fatal_150 ( ((*kUnreachableCodeMessage_21.with(Value::clone).borrow()) ).clone() , & [ ] ) } )  ;
 panic!("ub: non-void function does not return a value") } }
#[derive(Default)]
pub struct cppgc_internal_ProcessHeapStatisticsUpdater_AllocationObserverImpl {
    object_size_changes_since_last_reset_: Value<usize>,
}
impl cppgc_internal_StatsCollector_AllocationObserver
    for cppgc_internal_ProcessHeapStatisticsUpdater_AllocationObserverImpl
{
    fn AllocatedObjectSizeIncreased(&self, bytes: usize) {
        let bytes: Value<usize> = Rc::new(RefCell::new(bytes));
        ({
            cppgc_internal_ProcessHeapStatisticsUpdater::IncreaseTotalAllocatedObjectSize(
                (*bytes.borrow()),
            )
        });
        {
            let rhs_0 = (*self.object_size_changes_since_last_reset_.borrow())
                .wrapping_add((*bytes.borrow()));
            (*self.object_size_changes_since_last_reset_.borrow_mut()) = rhs_0
        };
    }
    fn AllocatedObjectSizeDecreased(&self, bytes: usize) {
        let bytes: Value<usize> = Rc::new(RefCell::new(bytes));
        ({
            cppgc_internal_ProcessHeapStatisticsUpdater::DecreaseTotalAllocatedObjectSize(
                (*bytes.borrow()),
            )
        });
        {
            let rhs_0 = (*self.object_size_changes_since_last_reset_.borrow())
                .wrapping_sub((*bytes.borrow()));
            (*self.object_size_changes_since_last_reset_.borrow_mut()) = rhs_0
        };
    }
    fn ResetAllocatedObjectSize(&self, bytes: usize) {
        let bytes: Value<usize> = Rc::new(RefCell::new(bytes));
        ({
            cppgc_internal_ProcessHeapStatisticsUpdater::DecreaseTotalAllocatedObjectSize(
                (*self.object_size_changes_since_last_reset_.borrow()),
            )
        });
        ({
            cppgc_internal_ProcessHeapStatisticsUpdater::IncreaseTotalAllocatedObjectSize(
                (*bytes.borrow()),
            )
        });
        (*self.object_size_changes_since_last_reset_.borrow_mut()) = (*bytes.borrow());
    }
    fn AllocatedSizeIncreased(&self, bytes: usize) {
        let bytes: Value<usize> = Rc::new(RefCell::new(bytes));
        ({
            cppgc_internal_ProcessHeapStatisticsUpdater::IncreaseTotalAllocatedSpace(
                (*bytes.borrow()),
            )
        });
    }
    fn AllocatedSizeDecreased(&self, bytes: usize) {
        let bytes: Value<usize> = Rc::new(RefCell::new(bytes));
        ({
            cppgc_internal_ProcessHeapStatisticsUpdater::DecreaseTotalAllocatedSpace(
                (*bytes.borrow()),
            )
        });
    }
}
impl Clone for cppgc_internal_ProcessHeapStatisticsUpdater_AllocationObserverImpl {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_ProcessHeapStatisticsUpdater_AllocationObserverImpl> =
            Rc::new(RefCell::new(Self {
                object_size_changes_since_last_reset_: Rc::new(RefCell::new(
                    (*self.object_size_changes_since_last_reset_.borrow()),
                )),
            }));
        let this: Ptr<cppgc_internal_ProcessHeapStatisticsUpdater_AllocationObserverImpl> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_ProcessHeapStatisticsUpdater_AllocationObserverImpl {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.object_size_changes_since_last_reset_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            object_size_changes_since_last_reset_: Rc::new(RefCell::new(<usize>::from_bytes(
                &buf[8..16],
            ))),
        }
    }
}
#[derive(Default)]
pub struct cppgc_internal_ProcessHeapStatisticsUpdater {}
impl cppgc_internal_ProcessHeapStatisticsUpdater {
    pub fn IncreaseTotalAllocatedObjectSize(delta: usize) {
        let delta: Value<usize> = Rc::new(RefCell::new(delta));
        ({
            (*total_allocated_object_size__1.with(Value::clone).borrow())
                .fetch_add_u64(((*delta.borrow()) as u64), Some(0))
        });
    }
    pub fn DecreaseTotalAllocatedObjectSize(delta: usize) {
        let delta: Value<usize> = Rc::new(RefCell::new(delta));
        ({
            (*total_allocated_object_size__1.with(Value::clone).borrow())
                .fetch_sub_u64(((*delta.borrow()) as u64), Some(0))
        });
    }
    pub fn IncreaseTotalAllocatedSpace(delta: usize) {
        let delta: Value<usize> = Rc::new(RefCell::new(delta));
        ({
            (*total_allocated_space__0.with(Value::clone).borrow())
                .fetch_add_u64(((*delta.borrow()) as u64), Some(0))
        });
    }
    pub fn DecreaseTotalAllocatedSpace(delta: usize) {
        let delta: Value<usize> = Rc::new(RefCell::new(delta));
        ({
            (*total_allocated_space__0.with(Value::clone).borrow())
                .fetch_sub_u64(((*delta.borrow()) as u64), Some(0))
        });
    }
}
impl Clone for cppgc_internal_ProcessHeapStatisticsUpdater {
    fn clone(&self) -> Self {
        let __this: Value<cppgc_internal_ProcessHeapStatisticsUpdater> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<cppgc_internal_ProcessHeapStatisticsUpdater> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for cppgc_internal_ProcessHeapStatisticsUpdater {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    static total_allocated_space__0: Value<std_atomic_unsigned_long_> = Rc::new(RefCell::new(
        std_atomic_unsigned_long_::std_atomic_unsigned_long_1({ 0_u64 }),
    ));
);
thread_local!(
    static total_allocated_object_size__1: Value<std_atomic_unsigned_long_> =
        Rc::new(RefCell::new(
            std_atomic_unsigned_long_::std_atomic_unsigned_long_1({ 0_u64 }),
        ));
);
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_base_FunctionRef_bool____;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_Isolate;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_PageAllocator_AllocationHint;
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
pub trait cppgc_internal_StatsCollectorImpl {
    fn GetPreviousEventForTesting(&self) -> Ptr<cppgc_internal_StatsCollector_Event>;
    fn SetMetricRecorder(&self, histogram_recorder: Option<Value<cppgc_internal_MetricRecorder>>);
    fn GetMetricRecorder(&self) -> Ptr<cppgc_internal_MetricRecorder>;
    fn RecordHistogramSample_cppgc_internal_StatsCollector_ConcurrentScopeId_v8_base_TimeDelta(
        &self,
        _a0: cppgc_internal_StatsCollector_ConcurrentScopeId,
        _a1: v8_base_TimeDelta,
    );
}
impl cppgc_internal_StatsCollectorImpl for Ptr<cppgc_internal_StatsCollector> {
    fn GetPreviousEventForTesting(&self) -> Ptr<cppgc_internal_StatsCollector_Event> {
        return (*(*self).upgrade().deref()).previous_.as_pointer();
    }
    fn SetMetricRecorder(&self, histogram_recorder: Option<Value<cppgc_internal_MetricRecorder>>) {
        let histogram_recorder: Value<Option<Value<cppgc_internal_MetricRecorder>>> =
            Rc::new(RefCell::new(histogram_recorder));
        ((*(*self).upgrade().deref()).metric_recorder_.as_pointer()
            as Ptr<Option<Value<cppgc_internal_MetricRecorder>>>)
            .write((*histogram_recorder.borrow_mut()).take());
    }
    fn GetMetricRecorder(&self) -> Ptr<cppgc_internal_MetricRecorder> {
        return (*(*(*self).upgrade().deref()).metric_recorder_.borrow()).as_pointer();
    }
    fn RecordHistogramSample_cppgc_internal_StatsCollector_ConcurrentScopeId_v8_base_TimeDelta(
        &self,
        _a0: cppgc_internal_StatsCollector_ConcurrentScopeId,
        _a1: v8_base_TimeDelta,
    ) {
        let _a0: Value<cppgc_internal_StatsCollector_ConcurrentScopeId> =
            Rc::new(RefCell::new(_a0));
        let _a1: Value<v8_base_TimeDelta> = Rc::new(RefCell::new(_a1));
    }
}
pub trait cppgc_internal_StatsCollector_InternalScope_cppgc_internal_StatsCollector_InternalScope_trace_category__cppgc_internal_StatsCollector_InternalScope_scope_category_Impl
{
}
impl cppgc_internal_StatsCollector_InternalScope_cppgc_internal_StatsCollector_InternalScope_trace_category__cppgc_internal_StatsCollector_InternalScope_scope_category_Impl for Ptr<cppgc_internal_StatsCollector_InternalScope_cppgc_internal_StatsCollector_InternalScope_trace_category__cppgc_internal_StatsCollector_InternalScope_scope_category_> {
 fn StopTrace ( &self , ) { (  { cppgc_internal_StatsCollector_InternalScope_cppgc_internal_StatsCollector_InternalScope_trace_category__cppgc_internal_StatsCollector_InternalScope_scope_category_Impl :: StopTraceImpl ( self , ) } )  ;
 }  fn StopTraceImpl ( &self , ) { ( if false { (  { let _args : Value<Ptr::<u8> > = Rc::new(RefCell::new((  { cppgc_internal_StatsCollector_InternalScope_cppgc_internal_StatsCollector_InternalScope_trace_category__cppgc_internal_StatsCollector_InternalScope_scope_category_::TraceCategory ( ) } )  )); Ignore_165 ( _args.as_pointer() , ) } )   } else { & ( 0 )  } ) ;
 } }
pub trait perfetto_NamedTrackImpl {
    fn disable_sibling_merge(&self) -> perfetto_NamedTrack;
}
impl perfetto_NamedTrackImpl for Ptr<perfetto_NamedTrack> {
    fn disable_sibling_merge(&self) -> perfetto_NamedTrack {
        return (*(*self).upgrade().deref()).clone();
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
                SignedSaturatedAdd64_102(
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
                SignedSaturatedAdd64_102(
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
pub trait v8_internal_tracing_ScopedTracerImpl {
    fn destructor(&self);
    fn Initialize(&self, category_group_enabled: Ptr<u8>, name: Ptr<u8>, event_handle: u64);
}
impl v8_internal_tracing_ScopedTracerImpl for Ptr<v8_internal_tracing_ScopedTracer> {
    fn destructor(&self) {
        if (!(*(*(*self).upgrade().deref()).p_data_.borrow()).is_null())
            && (({
                Relaxed_Load_144(
                    (*(*(*(*self).upgrade().deref()).data_.borrow())
                        .category_group_enabled
                        .borrow())
                    .reinterpret_cast::<u8>(),
                )
            }) != 0)
        {
            ({
                let _category_enabled_flag: Ptr<u8> =
                    (*(*(*(*self).upgrade().deref()).data_.borrow())
                        .category_group_enabled
                        .borrow())
                    .clone();
                let _name: Ptr<u8> =
                    (*(*(*(*self).upgrade().deref()).data_.borrow()).name.borrow()).clone();
                let _handle: u64 = (*(*(*(*self).upgrade().deref()).data_.borrow())
                    .event_handle
                    .borrow());
                (*({ v8_internal_tracing_TraceEventHelper::GetTracingController() })
                    .upgrade()
                    .deref())
                .UpdateTraceEventDuration(_category_enabled_flag, _name, _handle)
            });
        }
    }
    fn Initialize(&self, category_group_enabled: Ptr<u8>, name: Ptr<u8>, event_handle: u64) {
        let category_group_enabled: Value<Ptr<u8>> = Rc::new(RefCell::new(category_group_enabled));
        let name: Value<Ptr<u8>> = Rc::new(RefCell::new(name));
        let event_handle: Value<u64> = Rc::new(RefCell::new(event_handle));
        (*(*(*(*self).upgrade().deref()).data_.borrow())
            .category_group_enabled
            .borrow_mut()) = (*category_group_enabled.borrow()).clone();
        (*(*(*(*self).upgrade().deref()).data_.borrow())
            .name
            .borrow_mut()) = (*name.borrow()).clone();
        (*(*(*(*self).upgrade().deref()).data_.borrow())
            .event_handle
            .borrow_mut()) = (*event_handle.borrow());
        (*(*(*self).upgrade().deref()).p_data_.borrow_mut()) =
            ((*(*self).upgrade().deref()).data_.as_pointer());
    }
}
pub trait v8_internal_tracing_TraceIDImpl {
    fn raw_id(&self) -> u64;
    fn scope(&self) -> Ptr<u8>;
}
impl v8_internal_tracing_TraceIDImpl for Ptr<v8_internal_tracing_TraceID> {
    fn raw_id(&self) -> u64 {
        return (*(*(*self).upgrade().deref()).raw_id_.borrow());
    }
    fn scope(&self) -> Ptr<u8> {
        return (*(*(*self).upgrade().deref()).scope_.borrow()).clone();
    }
}
pub trait v8_internal_tracing_TraceID_WithScopeImpl {
    fn raw_id(&self) -> u64;
    fn scope(&self) -> Ptr<u8>;
}
impl v8_internal_tracing_TraceID_WithScopeImpl for Ptr<v8_internal_tracing_TraceID_WithScope> {
    fn raw_id(&self) -> u64 {
        return (*(*(*self).upgrade().deref()).raw_id_.borrow());
    }
    fn scope(&self) -> Ptr<u8> {
        return (*(*(*self).upgrade().deref()).scope_.borrow()).clone();
    }
}
pub trait v8_internal_tracing_TraceStringWithCopyImpl {
    fn operator_const_char__(&self) -> Ptr<u8>;
}
impl v8_internal_tracing_TraceStringWithCopyImpl for Ptr<v8_internal_tracing_TraceStringWithCopy> {
    fn operator_const_char__(&self) -> Ptr<u8> {
        return (*(*(*self).upgrade().deref()).str_.borrow()).clone();
    }
}
