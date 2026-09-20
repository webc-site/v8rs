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
#[derive(Default)]
pub struct v8_base_LsanVirtualAddressSpace {
    vas_: Value<Option<Value<v8_VirtualAddressSpace>>>,
}
impl v8_base_LsanVirtualAddressSpace {
    pub fn v8_base_LsanVirtualAddressSpace(vas: Option<Value<v8_VirtualAddressSpace>>) -> Self {
        let vas: Value<Option<Value<v8_VirtualAddressSpace>>> = Rc::new(RefCell::new(vas));
        let __this: Value<v8_base_LsanVirtualAddressSpace> = Rc::new(RefCell::new(Self {
            vas_: Rc::new(RefCell::new((*vas.borrow_mut()).take())),
        }));
        let this: Ptr<v8_base_LsanVirtualAddressSpace> = __this.as_pointer();
        (&(0));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl v8_VirtualAddressSpace for v8_base_LsanVirtualAddressSpace {
    fn SetRandomSeed(&self, seed: i64) {
        let seed: Value<i64> = Rc::new(RefCell::new(seed));
        ({ (*(*self.vas_.borrow()).as_ref().unwrap().borrow()).SetRandomSeed((*seed.borrow())) });
        return;
    }
    fn RandomPageAddress(&self) -> u64 {
        return ({ (*(*self.vas_.borrow()).as_ref().unwrap().borrow()).RandomPageAddress() });
    }
    fn AllocatePages(
        &self,
        hint: u64,
        size: usize,
        alignment: usize,
        permissions: v8_PagePermissions,
    ) -> u64 {
        let hint: Value<u64> = Rc::new(RefCell::new(hint));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let alignment: Value<usize> = Rc::new(RefCell::new(alignment));
        let permissions: Value<v8_PagePermissions> = Rc::new(RefCell::new(permissions));
        let result: Value<u64> = Rc::new(RefCell::new(
            ({
                (*(*self.vas_.borrow()).as_ref().unwrap().borrow()).AllocatePages(
                    (*hint.borrow()),
                    (*size.borrow()),
                    (*alignment.borrow()),
                    (*permissions.borrow()),
                )
            }),
        ));
        return (*result.borrow());
    }
    fn FreePages(&self, address: u64, size: usize) {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        ({
            (*(*self.vas_.borrow()).as_ref().unwrap().borrow())
                .FreePages((*address.borrow()), (*size.borrow()))
        });
    }
    fn AllocateSharedPages(
        &self,
        hint: u64,
        size: usize,
        permissions: v8_PagePermissions,
        handle: v8_SharedMemoryHandle,
        offset: u64,
    ) -> u64 {
        let hint: Value<u64> = Rc::new(RefCell::new(hint));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let permissions: Value<v8_PagePermissions> = Rc::new(RefCell::new(permissions));
        let handle: Value<v8_SharedMemoryHandle> = Rc::new(RefCell::new(handle));
        let offset: Value<u64> = Rc::new(RefCell::new(offset));
        let result: Value<u64> = Rc::new(RefCell::new(
            ({
                (*(*self.vas_.borrow()).as_ref().unwrap().borrow())
                    .AllocateSharedPages_u64_usize_v8_PagePermissions_v8_SharedMemoryHandle_u64(
                        (*hint.borrow()),
                        (*size.borrow()),
                        (*permissions.borrow()),
                        (*handle.borrow()).clone(),
                        (*offset.borrow()),
                    )
            }),
        ));
        return (*result.borrow());
    }
    fn FreeSharedPages(&self, address: u64, size: usize) {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        ({
            (*(*self.vas_.borrow()).as_ref().unwrap().borrow())
                .FreeSharedPages((*address.borrow()), (*size.borrow()))
        });
    }
    fn SetPagePermissions(
        &self,
        address: u64,
        size: usize,
        permissions: v8_PagePermissions,
    ) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let permissions: Value<v8_PagePermissions> = Rc::new(RefCell::new(permissions));
        return ({
            (*(*self.vas_.borrow()).as_ref().unwrap().borrow()).SetPagePermissions(
                (*address.borrow()),
                (*size.borrow()),
                (*permissions.borrow()),
            )
        });
    }
    fn RecommitPages(&self, address: u64, size: usize, permissions: v8_PagePermissions) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let permissions: Value<v8_PagePermissions> = Rc::new(RefCell::new(permissions));
        return ({
            (*(*self.vas_.borrow()).as_ref().unwrap().borrow()).RecommitPages(
                (*address.borrow()),
                (*size.borrow()),
                (*permissions.borrow()),
            )
        });
    }
    fn AllocateGuardRegion(&self, address: u64, size: usize) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        return ({
            (*(*self.vas_.borrow()).as_ref().unwrap().borrow())
                .AllocateGuardRegion((*address.borrow()), (*size.borrow()))
        });
    }
    fn FreeGuardRegion(&self, address: u64, size: usize) {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        ({
            (*(*self.vas_.borrow()).as_ref().unwrap().borrow())
                .FreeGuardRegion((*address.borrow()), (*size.borrow()))
        });
    }
    fn CanAllocateSubspaces(&self) -> bool {
        return ({ (*(*self.vas_.borrow()).as_ref().unwrap().borrow()).CanAllocateSubspaces() });
    }
    fn ActiveMemoryProtectionKey(&self) -> std_optional_int_ {
        return std_optional_int_::std_optional_int_2({
            (*nullopt_3.with(Value::clone).borrow()).clone()
        });
    }
    fn AllocateSubspace(
        &self,
        hint: u64,
        size: usize,
        alignment: usize,
        max_page_permissions: v8_PagePermissions,
        key: std_optional_int_,
        handle: std_optional_v8_SharedMemoryHandle_,
    ) -> Option<Value<v8_VirtualAddressSpace>> {
        let hint: Value<u64> = Rc::new(RefCell::new(hint));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let alignment: Value<usize> = Rc::new(RefCell::new(alignment));
        let max_page_permissions: Value<v8_PagePermissions> =
            Rc::new(RefCell::new(max_page_permissions));
        let key: Value<std_optional_int_> = Rc::new(RefCell::new(key));
        let handle: Value<std_optional_v8_SharedMemoryHandle_> = Rc::new(RefCell::new(handle));
        let subspace: Value<Option<Value<v8_VirtualAddressSpace>>> = Rc::new(RefCell::new(
            ({
                (*(*self.vas_.borrow()).as_ref().unwrap().borrow()).AllocateSubspace(
                    (*hint.borrow()),
                    (*size.borrow()),
                    (*alignment.borrow()),
                    (*max_page_permissions.borrow()),
                    Some((*key.borrow()).clone()),
                    Some((*handle.borrow()).clone()),
                )
            }),
        ));
        return (*subspace.borrow_mut()).take();
    }
    fn DiscardSystemPages(&self, address: u64, size: usize) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        return ({
            (*(*self.vas_.borrow()).as_ref().unwrap().borrow())
                .DiscardSystemPages((*address.borrow()), (*size.borrow()))
        });
    }
    fn DecommitPages(&self, address: u64, size: usize) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        return ({
            (*(*self.vas_.borrow()).as_ref().unwrap().borrow())
                .DecommitPages((*address.borrow()), (*size.borrow()))
        });
    }
}
impl ByteRepr for v8_base_LsanVirtualAddressSpace {
    fn byte_size() -> usize {
        56
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.vas_.borrow()).to_bytes(&mut buf[48..56]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            vas_: Rc::new(RefCell::new(
                <Option<Value<v8_VirtualAddressSpace>>>::from_bytes(&buf[48..56]),
            )),
        }
    }
}
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
