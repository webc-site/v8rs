use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
pub fn SharedMemoryHandleFromMachMemoryEntry_0(handle: u32) -> v8_SharedMemoryHandle {
    let handle: Value<u32> = Rc::new(RefCell::new(handle));
    return ({ v8_SharedMemoryHandle::FromPlatformHandle((*handle.borrow())) });
}
pub fn MachMemoryEntryFromSharedMemoryHandle_1(handle: v8_SharedMemoryHandle) -> u32 {
    let handle: Value<v8_SharedMemoryHandle> = Rc::new(RefCell::new(handle));
    return ({ v8_SharedMemoryHandleImpl::GetPlatformHandle(&handle.as_pointer()) });
}
thread_local!(
    pub static kInvalidSharedMemoryHandle_2: Value<std_optional_v8_SharedMemoryHandle_> =
        Rc::new(RefCell::new(
            std_optional_v8_SharedMemoryHandle_::std_optional_v8_SharedMemoryHandle_1({
                (*nullopt_3.with(Value::clone).borrow()).clone()
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
pub fn operator_bitor_4(lhs: v8_PagePermissions, rhs: v8_PagePermissions) -> v8_PagePermissions {
    let lhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(lhs));
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as i32) | ((*rhs.borrow()) as i32)) as v8_PagePermissions);
}
pub fn operator_bitand_5(lhs: v8_PagePermissions, rhs: v8_PagePermissions) -> v8_PagePermissions {
    let lhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(lhs));
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as i32) & ((*rhs.borrow()) as i32)) as v8_PagePermissions);
}
pub fn operator_bitor_assign_6(
    lhs: Ptr<v8_PagePermissions>,
    rhs: v8_PagePermissions,
) -> Ptr<v8_PagePermissions> {
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    let __rhs = ({
        let _lhs: v8_PagePermissions = (lhs.read());
        let _rhs: v8_PagePermissions = (*rhs.borrow());
        operator_bitor_4(_lhs, _rhs)
    });
    lhs.write(__rhs);
    return (lhs).clone();
}
pub fn IsSubset_7(lhs: v8_PagePermissions, rhs: v8_PagePermissions) -> bool {
    let lhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(lhs));
    let rhs: Value<v8_PagePermissions> = Rc::new(RefCell::new(rhs));
    return (({
        let _lhs: v8_PagePermissions = (*lhs.borrow());
        operator_bitand_5(_lhs, (*rhs.borrow()))
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
        return (({ floor_8(({ self.CurrentClockTimeMillis() })) }) as i64);
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
            static default_observer_9: Value<v8_HighAllocationThroughputObserver> = Rc::new(
                RefCell::new(<v8_HighAllocationThroughputObserver>::default()),
            );
        );
        return (default_observer_9.with(Value::clone).as_pointer());
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
#[derive()]
pub struct v8_base_PageAllocator {
    allocate_page_size_: Value<usize>,
    commit_page_size_: Value<usize>,
}
impl v8_base_PageAllocator {
    pub fn v8_base_PageAllocator() -> Self {
        let __this: Value<v8_base_PageAllocator> = Rc::new(RefCell::new(Self {
            allocate_page_size_: Rc::new(RefCell::new(({ v8_base_OS::AllocatePageSize() }))),
            commit_page_size_: Rc::new(RefCell::new(({ v8_base_OS::CommitPageSize() }))),
        }));
        let this: Ptr<v8_base_PageAllocator> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl v8_PageAllocator for v8_base_PageAllocator {
    fn AllocatePageSize(&self) -> usize {
        return (*self.allocate_page_size_.borrow());
    }
    fn CommitPageSize(&self) -> usize {
        return (*self.commit_page_size_.borrow());
    }
    fn SetRandomMmapSeed(&self, seed: i64) {
        let seed: Value<i64> = Rc::new(RefCell::new(seed));
        ({ v8_base_OS::SetRandomMmapSeed((*seed.borrow())) });
    }
    fn GetRandomMmapAddr(&self) -> AnyPtr {
        return ({ v8_base_OS::GetRandomMmapAddr() });
    }
    fn AllocatePages(
        &self,
        hint: AnyPtr,
        size: usize,
        alignment: usize,
        access: v8_PageAllocator_Permission,
    ) -> AnyPtr {
        let hint: Value<AnyPtr> = Rc::new(RefCell::new(hint));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let alignment: Value<usize> = Rc::new(RefCell::new(alignment));
        let access: Value<v8_PageAllocator_Permission> = Rc::new(RefCell::new(access));
        return ({
            v8_base_OS::Allocate(
                (*hint.borrow()).clone(),
                (*size.borrow()),
                (*alignment.borrow()),
                ((*access.borrow()) as v8_base_OS),
                None,
            )
        });
    }
    fn CanAllocateSharedPages(&self) -> bool {
        return false;
    }
    fn AllocateSharedPages(
        &self,
        size: usize,
        original_address: AnyPtr,
    ) -> Option<Value<v8_PageAllocator_SharedMemory>> {
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let original_address: Value<AnyPtr> = Rc::new(RefCell::new(original_address));
        return None;
    }
    fn FreePages(&self, address: AnyPtr, size: usize) -> bool {
        let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        ({ v8_base_OS::Free((*address.borrow()).clone(), (*size.borrow())) });
        return true;
    }
    fn ReleasePages(&self, address: AnyPtr, size: usize, new_size: usize) -> bool {
        let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let new_size: Value<usize> = Rc::new(RefCell::new(new_size));
        (&(0));
        ({
            let _address: AnyPtr = ((*address.borrow())
                .reinterpret_cast::<u8>()
                .offset((*new_size.borrow()) as isize)
                as Ptr<u8>)
                .to_any();
            let _size: usize = (*size.borrow()).wrapping_sub((*new_size.borrow()));
            v8_base_OS::Release(_address, _size)
        });
        return true;
    }
    fn SetPermissions(
        &self,
        address: AnyPtr,
        size: usize,
        access: v8_PageAllocator_Permission,
    ) -> bool {
        let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let access: Value<v8_PageAllocator_Permission> = Rc::new(RefCell::new(access));
        return ({
            v8_base_OS::SetPermissions(
                (*address.borrow()).clone(),
                (*size.borrow()),
                ((*access.borrow()) as v8_base_OS),
            )
        });
    }
    fn RecommitPages(
        &self,
        address: AnyPtr,
        size: usize,
        access: v8_PageAllocator_Permission,
    ) -> bool {
        let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let access: Value<v8_PageAllocator_Permission> = Rc::new(RefCell::new(access));
        return ({
            v8_base_OS::RecommitPages(
                (*address.borrow()).clone(),
                (*size.borrow()),
                ((*access.borrow()) as v8_base_OS),
            )
        });
    }
    fn DiscardSystemPages(&self, address: AnyPtr, size: usize) -> bool {
        let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        return ({ v8_base_OS::DiscardSystemPages((*address.borrow()).clone(), (*size.borrow())) });
    }
    fn DecommitPages(&self, address: AnyPtr, size: usize) -> bool {
        let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        return ({ v8_base_OS::DecommitPages((*address.borrow()).clone(), (*size.borrow())) });
    }
    fn SealPages(&self, address: AnyPtr, size: usize) -> bool {
        let address: Value<AnyPtr> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        return ({ v8_base_OS::SealPages((*address.borrow()).clone(), (*size.borrow())) });
    }
}
impl Clone for v8_base_PageAllocator {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_PageAllocator> = Rc::new(RefCell::new(Self {
            allocate_page_size_: Rc::new(RefCell::new((*self.allocate_page_size_.borrow()))),
            commit_page_size_: Rc::new(RefCell::new((*self.commit_page_size_.borrow()))),
        }));
        let this: Ptr<v8_base_PageAllocator> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_PageAllocator {
    fn default() -> Self {
        { v8_base_PageAllocator::v8_base_PageAllocator() }
    }
}
impl ByteRepr for v8_base_PageAllocator {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.allocate_page_size_.borrow()).to_bytes(&mut buf[8..16]);
        (*self.commit_page_size_.borrow()).to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            allocate_page_size_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
            commit_page_size_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[16..24]))),
        }
    }
}
pub type v8_base_AbortMode = i32;
pub const v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures: v8_base_AbortMode = 0;
pub const v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures: v8_base_AbortMode = 1;
pub const v8_base_AbortMode_kExitIfNoSecurityImpact: v8_base_AbortMode = 2;
pub const v8_base_AbortMode_kImmediateCrash: v8_base_AbortMode = 3;
pub const v8_base_AbortMode_kDefault: v8_base_AbortMode = 4;
thread_local!();
pub fn ControlledCrashesAreHarmless_11() -> bool {
    return ((*g_abort_mode_10.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_10.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn DcheckFailuresAreIgnored_12() -> bool {
    return ((*g_abort_mode_10.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_10.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn FatalErrorsWithNoSecurityImpactShouldExit_13() -> bool {
    return ((*g_abort_mode_10.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitIfNoSecurityImpact);
}
thread_local!(
    pub static kReturnAddressStackSlotCount_14: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kPageSizeBits_15: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
    pub static kRegularPageSize_16: Value<i32> = Rc::new(RefCell::new(262144));
);
thread_local!(
    pub static kMinimumOSPageSize_17: Value<i32> = Rc::new(RefCell::new(16384));
);
thread_local!(
    pub static kUnimplementedCodeMessage_18: Value<Ptr<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"unimplemented code"),
    ));
);
thread_local!(
    pub static kUnreachableCodeMessage_19: Value<Ptr<u8>> =
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
    pub static is_enum_20: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static is_enum_21: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_22: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_23: Value<bool> = Rc::new(RefCell::new(true));
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
    pub static value_24: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_25: Value<bool> = Rc::new(RefCell::new(false));
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
pub fn make_uint64_26(high: u32, low: u32) -> u64 {
    let high: Value<u32> = Rc::new(RefCell::new(high));
    let low: Value<u32> = Rc::new(RefCell::new(low));
    return (((*high.borrow()) as u64) << 32).wrapping_add(((*low.borrow()) as u64));
}
pub fn RoundDown_27(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    return ((*x.borrow()) & (-(*m.borrow()) as u64));
}
pub fn RoundUp_28(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    (&(0));
    return ({
        let _x: u64 = ((*x.borrow()).wrapping_add((((*m.borrow()) - 1_i64) as u64)) as u64);
        let _m: i64 = (*m.borrow());
        RoundDown_27(_x, _m)
    });
}
pub fn IsAligned_29(value: u64, alignment: u64) -> bool {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let alignment: Value<u64> = Rc::new(RefCell::new(alignment));
    return (((*value.borrow()) & ((*alignment.borrow()).wrapping_sub(1_u64))) == 0_u64);
}
pub type anon_30 = u8;
pub const anon_30_ONCE_STATE_UNINITIALIZED: anon_30 = 0;
pub const anon_30_ONCE_STATE_EXECUTING_FUNCTION: anon_30 = 1;
pub const anon_30_ONCE_STATE_DONE: anon_30 = 2;
pub fn CallOnce_31(once: Ptr<std_atomic_unsigned_char_>, init_func: std_function_void____) {
    let once: Value<Ptr<std_atomic_unsigned_char_>> = Rc::new(RefCell::new(once));
    let init_func: Value<std_function_void____> = Rc::new(RefCell::new(init_func));
    if ((({ (*(*once.borrow()).upgrade().deref()).load_const(Some(2)) }) as i32)
        != (anon_30_ONCE_STATE_DONE as i32))
    {
        ({ CallOnceImpl_32((*once.borrow()).clone(), (*init_func.borrow()).clone()) });
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
pub struct v8_base_Semaphore {
    native_handle_: Value<Ptr<dispatch_semaphore_s>>,
}
impl v8_base_Semaphore {}
impl ByteRepr for v8_base_Semaphore {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.native_handle_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            native_handle_: Rc::new(RefCell::new(<Ptr<dispatch_semaphore_s>>::from_bytes(
                &buf[0..8],
            ))),
        }
    }
}
thread_local!();
pub type v8_base_OS_MemoryPermission = i32;
pub const v8_base_OS_MemoryPermission_kNoAccess: v8_base_OS_MemoryPermission = 0;
pub const v8_base_OS_MemoryPermission_kRead: v8_base_OS_MemoryPermission = 1;
pub const v8_base_OS_MemoryPermission_kReadWrite: v8_base_OS_MemoryPermission = 2;
pub const v8_base_OS_MemoryPermission_kReadWriteExecute: v8_base_OS_MemoryPermission = 3;
pub const v8_base_OS_MemoryPermission_kReadExecute: v8_base_OS_MemoryPermission = 4;
pub const v8_base_OS_MemoryPermission_kNoAccessWillJitLater: v8_base_OS_MemoryPermission = 5;
thread_local!(
    pub static kStackWalkError_34: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static kStackWalkMaxNameLen_35: Value<i32> = Rc::new(RefCell::new(256));
);
thread_local!(
    pub static kStackWalkMaxTextLen_36: Value<i32> = Rc::new(RefCell::new(256));
);
thread_local!(
    static msPerSecond_37: Value<i32> = Rc::new(RefCell::new(1000));
);
#[derive()]
pub struct v8_base_OS_StackFrame {
    pub address: Value<AnyPtr>,
    pub text: Value<Box<[u8]>>,
}
impl Clone for v8_base_OS_StackFrame {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_OS_StackFrame> = Rc::new(RefCell::new(Self {
            address: Rc::new(RefCell::new((*self.address.borrow()).clone())),
            text: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 256, _>(
                |__i: usize| (*self.text.borrow())[(__i) as usize],
            )))),
        }));
        let this: Ptr<v8_base_OS_StackFrame> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_OS_StackFrame {
    fn default() -> Self {
        v8_base_OS_StackFrame {
            address: Rc::new(RefCell::new(AnyPtr::default())),
            text: Rc::new(RefCell::new(
                (0..256).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
        }
    }
}
impl ByteRepr for v8_base_OS_StackFrame {
    fn byte_size() -> usize {
        264
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.address.borrow()).to_bytes(&mut buf[0..8]);
        (*self.text.borrow()).to_bytes(&mut buf[8..264]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            address: Rc::new(RefCell::new(<AnyPtr>::from_bytes(&buf[0..8]))),
            text: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[8..264]))),
        }
    }
}
pub type v8_base_OS_MemoryMappedFile_FileMode = i32;
pub const v8_base_OS_MemoryMappedFile_FileMode_kReadOnly: v8_base_OS_MemoryMappedFile_FileMode = 0;
pub const v8_base_OS_MemoryMappedFile_FileMode_kReadWrite: v8_base_OS_MemoryMappedFile_FileMode = 1;
pub trait v8_base_OS_MemoryMappedFile {
    fn memory(&self) -> AnyPtr;
    fn size(&self) -> usize;
}
#[derive(Default)]
pub struct v8_base_OS_SharedLibraryAddress {
    pub library_path: Value<Vec<u8>>,
    pub start: Value<u64>,
    pub end: Value<u64>,
    pub aslr_slide: Value<i64>,
}
impl v8_base_OS_SharedLibraryAddress {
    pub fn v8_base_OS_SharedLibraryAddress1(
        library_path: Ptr<Vec<u8>>,
        start: u64,
        end: u64,
    ) -> Self {
        let start: Value<u64> = Rc::new(RefCell::new(start));
        let end: Value<u64> = Rc::new(RefCell::new(end));
        let __this: Value<v8_base_OS_SharedLibraryAddress> = Rc::new(RefCell::new(Self {
            library_path: Rc::new(RefCell::new((*library_path.upgrade().deref()).clone())),
            start: Rc::new(RefCell::new((*start.borrow()))),
            end: Rc::new(RefCell::new((*end.borrow()))),
            aslr_slide: Rc::new(RefCell::new(0_i64)),
        }));
        let this: Ptr<v8_base_OS_SharedLibraryAddress> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_base_OS_SharedLibraryAddress2(
        library_path: Ptr<Vec<u8>>,
        start: u64,
        end: u64,
        aslr_slide: i64,
    ) -> Self {
        let start: Value<u64> = Rc::new(RefCell::new(start));
        let end: Value<u64> = Rc::new(RefCell::new(end));
        let aslr_slide: Value<i64> = Rc::new(RefCell::new(aslr_slide));
        let __this: Value<v8_base_OS_SharedLibraryAddress> = Rc::new(RefCell::new(Self {
            library_path: Rc::new(RefCell::new((*library_path.upgrade().deref()).clone())),
            start: Rc::new(RefCell::new((*start.borrow()))),
            end: Rc::new(RefCell::new((*end.borrow()))),
            aslr_slide: Rc::new(RefCell::new((*aslr_slide.borrow()))),
        }));
        let this: Ptr<v8_base_OS_SharedLibraryAddress> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_OS_SharedLibraryAddress {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_OS_SharedLibraryAddress> = Rc::new(RefCell::new(Self {
            library_path: Rc::new(RefCell::new((*self.library_path.borrow()).clone())),
            start: Rc::new(RefCell::new((*self.start.borrow()))),
            end: Rc::new(RefCell::new((*self.end.borrow()))),
            aslr_slide: Rc::new(RefCell::new((*self.aslr_slide.borrow()))),
        }));
        let this: Ptr<v8_base_OS_SharedLibraryAddress> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_OS_SharedLibraryAddress {
    fn byte_size() -> usize {
        48
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.library_path.borrow()).to_bytes(&mut buf[0..24]);
        (*self.start.borrow()).to_bytes(&mut buf[24..32]);
        (*self.end.borrow()).to_bytes(&mut buf[32..40]);
        (*self.aslr_slide.borrow()).to_bytes(&mut buf[40..48]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            library_path: Rc::new(RefCell::new(<Vec<u8>>::from_bytes(&buf[0..24]))),
            start: Rc::new(RefCell::new(<u64>::from_bytes(&buf[24..32]))),
            end: Rc::new(RefCell::new(<u64>::from_bytes(&buf[32..40]))),
            aslr_slide: Rc::new(RefCell::new(<i64>::from_bytes(&buf[40..48]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_OS_MemoryRange {
    pub start: Value<u64>,
    pub end: Value<u64>,
}
impl Clone for v8_base_OS_MemoryRange {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_OS_MemoryRange> = Rc::new(RefCell::new(Self {
            start: Rc::new(RefCell::new((*self.start.borrow()))),
            end: Rc::new(RefCell::new((*self.end.borrow()))),
        }));
        let this: Ptr<v8_base_OS_MemoryRange> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_OS_MemoryRange {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.start.borrow()).to_bytes(&mut buf[0..8]);
        (*self.end.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            start: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
            end: Rc::new(RefCell::new(<u64>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_OS {}
impl v8_base_OS {
    pub fn IsRemapPageSupported() -> bool {
        return true;
    }
}
impl ByteRepr for v8_base_OS {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn EnsureConsoleOutput_38() {}
#[derive(Default)]
pub struct v8_base_AddressSpaceReservation {
    base_: Value<AnyPtr>,
    size_: Value<usize>,
}
impl v8_base_AddressSpaceReservation {
    fn v8_base_AddressSpaceReservation(base: AnyPtr, size: usize) -> Self {
        let base: Value<AnyPtr> = Rc::new(RefCell::new(base));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let __this: Value<v8_base_AddressSpaceReservation> = Rc::new(RefCell::new(Self {
            base_: Rc::new(RefCell::new((*base.borrow()).clone())),
            size_: Rc::new(RefCell::new((*size.borrow()))),
        }));
        let this: Ptr<v8_base_AddressSpaceReservation> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_AddressSpaceReservation {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_AddressSpaceReservation> = Rc::new(RefCell::new(Self {
            base_: Rc::new(RefCell::new((*self.base_.borrow()).clone())),
            size_: Rc::new(RefCell::new((*self.size_.borrow()))),
        }));
        let this: Ptr<v8_base_AddressSpaceReservation> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_AddressSpaceReservation {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.size_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_: Rc::new(RefCell::new(<AnyPtr>::from_bytes(&buf[0..8]))),
            size_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
        }
    }
}
pub type v8_base_Thread_Priority = i32;
pub const v8_base_Thread_Priority_kBestEffort: v8_base_Thread_Priority = 0;
pub const v8_base_Thread_Priority_kUserVisible: v8_base_Thread_Priority = 1;
pub const v8_base_Thread_Priority_kUserBlocking: v8_base_Thread_Priority = 2;
pub const v8_base_Thread_Priority_kDefault: v8_base_Thread_Priority = 3;
pub trait v8_base_Thread {
    fn StartSynchronously(&self) -> bool {
        (*(*(*self).upgrade().deref()).start_semaphore_.borrow_mut()) =
            Ptr::alloc(v8_base_Semaphore::v8_base_Semaphore({ 0 }));
        if !({ v8_base_ThreadImpl::Start(self) }) {
            return false;
        }
        ({
            v8_base_SemaphoreImpl::Wait(&(*(*(*self).upgrade().deref()).start_semaphore_.borrow()))
        });
        (*(*(*self).upgrade().deref()).start_semaphore_.borrow()).delete();
        (*(*(*self).upgrade().deref()).start_semaphore_.borrow_mut()) =
            Ptr::<v8_base_Semaphore>::null();
        return true;
    }
    fn name(&self) -> Ptr<u8> {
        return ((*(*self).upgrade().deref()).name_.as_pointer() as Ptr<u8>);
    }
    fn Run(&self);
    fn HasThreadLocal(key: i32) -> bool {
        let key: Value<i32> = Rc::new(RefCell::new(key));
        return !(({ v8_base_Thread::GetThreadLocal((*key.borrow())) }).is_null());
    }
    fn GetExistingThreadLocal(key: i32) -> AnyPtr {
        let key: Value<i32> = Rc::new(RefCell::new(key));
        return ({ v8_base_Thread::GetThreadLocal((*key.borrow())) });
    }
    fn data(&self) -> Ptr<v8_base_Thread_PlatformData> {
        return (*(*(*self).upgrade().deref()).data_.borrow()).clone();
    }
    fn priority(&self) -> v8_base_Thread_Priority {
        return (*(*(*self).upgrade().deref()).priority_.borrow());
    }
    fn NotifyStartedAndDispatch(&self) {
        if !(*self.start_semaphore_.borrow()).is_null() {
            ({ v8_base_SemaphoreImpl::Signal(&(*self.start_semaphore_.borrow())) });
        }
        ({ self.Dispatch() });
    }
    fn Dispatch(&self) {
        ({ self.Run() });
    }
}
#[derive(Default)]
struct v8_base_Stack_PreventNonDefaultParameters {}
impl Clone for v8_base_Stack_PreventNonDefaultParameters {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Stack_PreventNonDefaultParameters> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_Stack_PreventNonDefaultParameters> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_Stack_PreventNonDefaultParameters {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_Stack_StackSlot {
    pub value: Value<u64>,
}
impl v8_base_Stack_StackSlot {
    pub fn v8_base_Stack_StackSlot1(value: AnyPtr) -> Self {
        let value: Value<AnyPtr> = Rc::new(RefCell::new(value));
        let __this: Value<v8_base_Stack_StackSlot> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*value.borrow()).to_int())),
        }));
        let this: Ptr<v8_base_Stack_StackSlot> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_base_Stack_StackSlot2(value: u64) -> Self {
        let value: Value<u64> = Rc::new(RefCell::new(value));
        let __this: Value<v8_base_Stack_StackSlot> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*value.borrow()))),
        }));
        let this: Ptr<v8_base_Stack_StackSlot> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_Stack_StackSlot {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Stack_StackSlot> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        }));
        let this: Ptr<v8_base_Stack_StackSlot> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_Stack_StackSlot {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.value.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_Stack {}
impl v8_base_Stack {
    pub fn GetCurrentFrameAddress(
        _a0: Option<v8_base_Stack_PreventNonDefaultParameters>,
        frame_address: Option<AnyPtr>,
    ) -> v8_base_Stack_StackSlot {
        let _a0: Value<v8_base_Stack_PreventNonDefaultParameters> = Rc::new(RefCell::new(
            _a0.unwrap_or(<v8_base_Stack_PreventNonDefaultParameters>::default()),
        ));
        let frame_address: Value<AnyPtr> = Rc::new(RefCell::new(
            frame_address.unwrap_or(({ __builtin_frame_address_39(0_u32) })),
        ));
        return v8_base_Stack_StackSlot::v8_base_Stack_StackSlot1({
            (*frame_address.borrow()).clone()
        });
    }
    pub fn GetRealStackAddressForSlot(slot: v8_base_Stack_StackSlot) -> v8_base_Stack_StackSlot {
        let slot: Value<v8_base_Stack_StackSlot> = Rc::new(RefCell::new(slot));
        return (*slot.borrow()).clone();
    }
}
impl Clone for v8_base_Stack {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Stack> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_Stack> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_Stack {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_SharedMemoryMapping {
    page_allocator_: Value<Ptr<v8_base_PageAllocator>>,
    ptr_: Value<AnyPtr>,
    size_: Value<usize>,
}
impl v8_base_SharedMemoryMapping {
    pub fn v8_base_SharedMemoryMapping(
        page_allocator: Ptr<v8_base_PageAllocator>,
        ptr: AnyPtr,
        size: usize,
    ) -> Self {
        let page_allocator: Value<Ptr<v8_base_PageAllocator>> =
            Rc::new(RefCell::new(page_allocator));
        let ptr: Value<AnyPtr> = Rc::new(RefCell::new(ptr));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let __this: Value<v8_base_SharedMemoryMapping> = Rc::new(RefCell::new(Self {
            page_allocator_: Rc::new(RefCell::new((*page_allocator.borrow()).clone())),
            ptr_: Rc::new(RefCell::new((*ptr.borrow()).clone())),
            size_: Rc::new(RefCell::new((*size.borrow()))),
        }));
        let this: Ptr<v8_base_SharedMemoryMapping> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl v8_PageAllocator_SharedMemoryMapping for v8_base_SharedMemoryMapping {
    fn GetMemory(&self) -> AnyPtr {
        return (*self.ptr_.borrow()).clone();
    }
}
impl Clone for v8_base_SharedMemoryMapping {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_SharedMemoryMapping> = Rc::new(RefCell::new(Self {
            page_allocator_: Rc::new(RefCell::new((*self.page_allocator_.borrow()).clone())),
            ptr_: Rc::new(RefCell::new((*self.ptr_.borrow()).clone())),
            size_: Rc::new(RefCell::new((*self.size_.borrow()))),
        }));
        let this: Ptr<v8_base_SharedMemoryMapping> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_SharedMemoryMapping {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.page_allocator_.borrow()).to_bytes(&mut buf[8..16]);
        (*self.ptr_.borrow()).to_bytes(&mut buf[16..24]);
        (*self.size_.borrow()).to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            page_allocator_: Rc::new(RefCell::new(<Ptr<v8_base_PageAllocator>>::from_bytes(
                &buf[8..16],
            ))),
            ptr_: Rc::new(RefCell::new(<AnyPtr>::from_bytes(&buf[16..24]))),
            size_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[24..32]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_SharedMemory {
    allocator_: Value<Ptr<v8_base_PageAllocator>>,
    ptr_: Value<AnyPtr>,
    size_: Value<usize>,
}
impl v8_base_SharedMemory {
    pub fn v8_base_SharedMemory(
        allocator: Ptr<v8_base_PageAllocator>,
        memory: AnyPtr,
        size: usize,
    ) -> Self {
        let allocator: Value<Ptr<v8_base_PageAllocator>> = Rc::new(RefCell::new(allocator));
        let memory: Value<AnyPtr> = Rc::new(RefCell::new(memory));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let __this: Value<v8_base_SharedMemory> = Rc::new(RefCell::new(Self {
            allocator_: Rc::new(RefCell::new((*allocator.borrow()).clone())),
            ptr_: Rc::new(RefCell::new((*memory.borrow()).clone())),
            size_: Rc::new(RefCell::new((*size.borrow()))),
        }));
        let this: Ptr<v8_base_SharedMemory> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl v8_PageAllocator_SharedMemory for v8_base_SharedMemory {
    fn GetMemory(&self) -> AnyPtr {
        return (*self.ptr_.borrow()).clone();
    }
    fn GetSize(&self) -> usize {
        return (*self.size_.borrow());
    }
    fn RemapTo(&self, new_address: AnyPtr) -> Option<Value<v8_PageAllocator_SharedMemoryMapping>> {
        let new_address: Value<AnyPtr> = Rc::new(RefCell::new(new_address));
        if !({
            let _old_address: AnyPtr = (*self.ptr_.borrow()).clone();
            let _size: usize = (*self.size_.borrow());
            v8_base_PageAllocatorImpl::RemapShared(
                &(*self.allocator_.borrow()),
                _old_address,
                (*new_address.borrow()).clone(),
                _size,
            )
        })
        .is_null()
        {
            return ({
                let ___args: Ptr<Ptr<v8_base_PageAllocator>> = self.allocator_.as_pointer();
                let ___args: Ptr<u64> = self.size_.as_pointer();
                make_unique_40(___args, new_address.as_pointer(), ___args)
            });
        } else {
            return None;
        }
        panic!("ub: non-void function does not return a value")
    }
}
impl Clone for v8_base_SharedMemory {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_SharedMemory> = Rc::new(RefCell::new(Self {
            allocator_: Rc::new(RefCell::new((*self.allocator_.borrow()).clone())),
            ptr_: Rc::new(RefCell::new((*self.ptr_.borrow()).clone())),
            size_: Rc::new(RefCell::new((*self.size_.borrow()))),
        }));
        let this: Ptr<v8_base_SharedMemory> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_SharedMemory {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.allocator_.borrow()).to_bytes(&mut buf[8..16]);
        (*self.ptr_.borrow()).to_bytes(&mut buf[16..24]);
        (*self.size_.borrow()).to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            allocator_: Rc::new(RefCell::new(<Ptr<v8_base_PageAllocator>>::from_bytes(
                &buf[8..16],
            ))),
            ptr_: Rc::new(RefCell::new(<AnyPtr>::from_bytes(&buf[16..24]))),
            size_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[24..32]))),
        }
    }
}
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_PageAllocator_SharedMemory;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_PageAllocator_SharedMemoryMapping;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_base_Thread_PlatformData;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_Isolate;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_PageAllocator_AllocationHint;
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
pub trait v8_base_AddressSpaceReservationImpl {
    fn base(&self) -> AnyPtr;
    fn size(&self) -> usize;
    fn Contains(&self, region_addr: AnyPtr, region_size: usize) -> bool;
    fn SetName(&self, name: Ptr<u8>) -> bool;
}
impl v8_base_AddressSpaceReservationImpl for Ptr<v8_base_AddressSpaceReservation> {
    fn base(&self) -> AnyPtr {
        return (*(*(*self).upgrade().deref()).base_.borrow()).clone();
    }
    fn size(&self) -> usize {
        return (*(*(*self).upgrade().deref()).size_.borrow());
    }
    fn Contains(&self, region_addr: AnyPtr, region_size: usize) -> bool {
        let region_addr: Value<AnyPtr> = Rc::new(RefCell::new(region_addr));
        let region_size: Value<usize> = Rc::new(RefCell::new(region_size));
        let base: Value<u64> = Rc::new(RefCell::new(
            (*(*(*self).upgrade().deref()).base_.borrow()).to_int(),
        ));
        let region_base: Value<u64> = Rc::new(RefCell::new((*region_addr.borrow()).to_int()));
        return ((*region_base.borrow()) >= (*base.borrow()))
            && (((*region_base.borrow()).wrapping_add(((*region_size.borrow()) as u64)))
                <= ((*base.borrow())
                    .wrapping_add(((*(*(*self).upgrade().deref()).size_.borrow()) as u64))));
    }
    fn SetName(&self, name: Ptr<u8>) -> bool {
        let name: Value<Ptr<u8>> = Rc::new(RefCell::new(name));
        return ({
            let _address: AnyPtr = (*(*(*self).upgrade().deref()).base_.borrow()).clone();
            let _size: usize = (*(*(*self).upgrade().deref()).size_.borrow());
            v8_base_OS::SetMemoryRegionName(_address, _size, (*name.borrow()).clone())
        });
    }
}
pub trait v8_base_PageAllocatorImpl {
    fn RemapShared(&self, old_address: AnyPtr, new_address: AnyPtr, size: usize) -> AnyPtr;
}
impl v8_base_PageAllocatorImpl for Ptr<v8_base_PageAllocator> {
    fn RemapShared(&self, old_address: AnyPtr, new_address: AnyPtr, size: usize) -> AnyPtr {
        let old_address: Value<AnyPtr> = Rc::new(RefCell::new(old_address));
        let new_address: Value<AnyPtr> = Rc::new(RefCell::new(new_address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        return AnyPtr::default();
    }
}
pub trait v8_base_SemaphoreImpl {
    fn native_handle(&self) -> Ptr<Ptr<dispatch_semaphore_s>>;
    fn native_handle_const(&self) -> Ptr<Ptr<dispatch_semaphore_s>>;
}
impl v8_base_SemaphoreImpl for Ptr<v8_base_Semaphore> {
    fn native_handle(&self) -> Ptr<Ptr<dispatch_semaphore_s>> {
        return (*(*self).upgrade().deref()).native_handle_.as_pointer();
    }
    fn native_handle_const(&self) -> Ptr<Ptr<dispatch_semaphore_s>> {
        return (*(*self).upgrade().deref()).native_handle_.as_pointer();
    }
}
pub trait v8_base_Stack_StackSlotImpl {
    fn operator_void__(&self) -> AnyPtr;
    fn operator_uintptr_t(&self) -> u64;
}
impl v8_base_Stack_StackSlotImpl for Ptr<v8_base_Stack_StackSlot> {
    fn operator_void__(&self) -> AnyPtr {
        return (*(*(*self).upgrade().deref()).value.borrow()).reinterpret_cast::<::libc::c_void>();
    }
    fn operator_uintptr_t(&self) -> u64 {
        return (*(*(*self).upgrade().deref()).value.borrow());
    }
}
