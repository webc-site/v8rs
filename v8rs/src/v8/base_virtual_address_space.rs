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
thread_local!(
    pub static kReturnAddressStackSlotCount_10: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kPageSizeBits_11: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
    pub static kRegularPageSize_12: Value<i32> = Rc::new(RefCell::new(262144));
);
thread_local!(
    pub static kMinimumOSPageSize_13: Value<i32> = Rc::new(RefCell::new(16384));
);
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
    pub static kLowZeroBits_15: Value<i32> = Rc::new(RefCell::new(8));
);
thread_local!(
    pub static kAlignment_16: Value<i32> = Rc::new(RefCell::new(256));
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
    pub static kToBePaddedSize_17: Value<usize> = Rc::new(RefCell::new(33));
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
pub fn HardeningAbort_19() {
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
pub fn is_constant_evaluated_20() -> bool {
    return ({ is_constant_evaluated_21() });
}
pub type absl_LogSeverity = i32;
pub const absl_LogSeverity_kInfo: absl_LogSeverity = 0;
pub const absl_LogSeverity_kWarning: absl_LogSeverity = 1;
pub const absl_LogSeverity_kError: absl_LogSeverity = 2;
pub const absl_LogSeverity_kFatal: absl_LogSeverity = 3;
pub fn LogSeverities_22() -> Vec<absl_LogSeverity> {
    return vec![
        absl_LogSeverity_kInfo,
        absl_LogSeverity_kWarning,
        absl_LogSeverity_kError,
        absl_LogSeverity_kFatal,
    ];
}
thread_local!(
    pub static kLogDebugFatal_23: Value<absl_LogSeverity> = Rc::new(RefCell::new(2));
);
pub fn LogSeverityName_24(s: absl_LogSeverity) -> Ptr<u8> {
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
pub fn NormalizeLogSeverity_25(s: absl_LogSeverity) -> absl_LogSeverity {
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
pub fn NormalizeLogSeverity_26(s: i32) -> absl_LogSeverity {
    let s: Value<i32> = Rc::new(RefCell::new(s));
    return ({ NormalizeLogSeverity_25(((*s.borrow()) as absl_LogSeverity)) });
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
pub fn operator_gt_27(lhs: absl_LogSeverityAtLeast, rhs: absl_LogSeverity) -> bool {
    let lhs: Value<absl_LogSeverityAtLeast> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(rhs));
    return (((*lhs.borrow()) as absl_LogSeverity) > (*rhs.borrow()));
}
pub fn operator_lt_28(lhs: absl_LogSeverity, rhs: absl_LogSeverityAtLeast) -> bool {
    let lhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverityAtLeast> = Rc::new(RefCell::new(rhs));
    return ((*lhs.borrow()) < ((*rhs.borrow()) as absl_LogSeverity));
}
pub fn operator_le_29(lhs: absl_LogSeverityAtLeast, rhs: absl_LogSeverity) -> bool {
    let lhs: Value<absl_LogSeverityAtLeast> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(rhs));
    return (((*lhs.borrow()) as absl_LogSeverity) <= (*rhs.borrow()));
}
pub fn operator_ge_30(lhs: absl_LogSeverity, rhs: absl_LogSeverityAtLeast) -> bool {
    let lhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverityAtLeast> = Rc::new(RefCell::new(rhs));
    return ((*lhs.borrow()) >= ((*rhs.borrow()) as absl_LogSeverity));
}
pub fn operator_lt_31(lhs: absl_LogSeverityAtMost, rhs: absl_LogSeverity) -> bool {
    let lhs: Value<absl_LogSeverityAtMost> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(rhs));
    return (((*lhs.borrow()) as absl_LogSeverity) < (*rhs.borrow()));
}
pub fn operator_gt_32(lhs: absl_LogSeverity, rhs: absl_LogSeverityAtMost) -> bool {
    let lhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverityAtMost> = Rc::new(RefCell::new(rhs));
    return ((*lhs.borrow()) > ((*rhs.borrow()) as absl_LogSeverity));
}
pub fn operator_ge_33(lhs: absl_LogSeverityAtMost, rhs: absl_LogSeverity) -> bool {
    let lhs: Value<absl_LogSeverityAtMost> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(rhs));
    return (((*lhs.borrow()) as absl_LogSeverity) >= (*rhs.borrow()));
}
pub fn operator_le_34(lhs: absl_LogSeverity, rhs: absl_LogSeverityAtMost) -> bool {
    let lhs: Value<absl_LogSeverity> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_LogSeverityAtMost> = Rc::new(RefCell::new(rhs));
    return ((*lhs.borrow()) <= ((*rhs.borrow()) as absl_LogSeverity));
}
pub fn Basename_35(fname: Ptr<u8>, offset: i32) -> Ptr<u8> {
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
        ({ Basename_35((*fname.borrow()).clone(), ((*offset.borrow()) - 1)) })
    };
}
thread_local!();
pub fn ClippedSubstr_37(s: Vec<u8>, pos: usize, n: Option<usize>) -> Vec<u8> {
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
pub fn NullSafeStringView_38(p: Ptr<u8>) -> Vec<u8> {
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
pub fn is_leap_year_39(y: i64) -> bool {
    let y: Value<i64> = Rc::new(RefCell::new(y));
    return (((*y.borrow()) % 4_i64) == 0_i64)
        && ((((*y.borrow()) % 100_i64) != 0_i64) || (((*y.borrow()) % 400_i64) == 0_i64));
}
pub fn year_index_40(y: i64, m: i8) -> i32 {
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
pub fn days_per_century_41(yi: i32) -> i32 {
    let yi: Value<i32> = Rc::new(RefCell::new(yi));
    return (36524 + ((((*yi.borrow()) == 0) || ((*yi.borrow()) > 300)) as i32));
}
pub fn days_per_4years_42(yi: i32) -> i32 {
    let yi: Value<i32> = Rc::new(RefCell::new(yi));
    return (1460
        + (((((*yi.borrow()) == 0) || ((*yi.borrow()) > 300))
            || ((((*yi.borrow()) - 1) % 100) < 96)) as i32));
}
pub fn days_per_year_43(y: i64, m: i8) -> i32 {
    let y: Value<i64> = Rc::new(RefCell::new(y));
    let m: Value<i8> = Rc::new(RefCell::new(m));
    return if ({ is_leap_year_39(((*y.borrow()) + ((((*m.borrow()) as i32) > 2) as i64))) }) {
        366
    } else {
        365
    };
}
pub fn days_per_month_44(y: i64, m: i8) -> i32 {
    let y: Value<i64> = Rc::new(RefCell::new(y));
    let m: Value<i8> = Rc::new(RefCell::new(m));
    let k_days_per_month: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        -1_i32, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    ])));
    return ((*k_days_per_month.borrow())[(*m.borrow()) as usize]
        + (((((*m.borrow()) as i32) == 2) && ({ is_leap_year_39((*y.borrow())) })) as i32));
}
pub fn n_day_45(
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
            (*d.borrow_mut()) += (({ days_per_year_43((*ey.borrow()), (*m.borrow())) }) as i64);
        } else {
            (*ey.borrow_mut()) -= 400_i64;
            (*d.borrow_mut()) += 146097_i64;
        }
    }
    if ((*d.borrow()) > 365_i64) {
        let yi: Value<i32> = Rc::new(RefCell::new(
            ({ year_index_40((*ey.borrow()), (*m.borrow())) }),
        ));
        'loop_: while true {
            let n: Value<i32> = Rc::new(RefCell::new(({ days_per_century_41((*yi.borrow())) })));
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
            let n: Value<i32> = Rc::new(RefCell::new(({ days_per_4years_42((*yi.borrow())) })));
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
                ({ days_per_year_43((*ey.borrow()), (*m.borrow())) }),
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
                ({ days_per_month_44((*ey.borrow()), (*m.borrow())) }),
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
pub fn n_mon_46(
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
        n_day_45(
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
pub fn n_hour_47(
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
        n_mon_46(
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
pub fn n_min_48(
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
        n_hour_47(
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
pub fn n_sec_49(
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
                    n_mon_46(
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
                n_hour_47(
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
            n_min_48(
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
        n_min_48(
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
pub fn step_50(
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
        n_sec_49(_y, _m, _d, _hh, _mm, _ss)
    });
}
pub fn step_51(
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
        n_min_48(_y, _m, _d, _hh, 0_i64, _mm, _ss)
    });
}
pub fn step_52(
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
        n_hour_47(_y, _m, _d, 0_i64, _hh, _mm, _ss)
    });
}
pub fn step_53(
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
        n_day_45(_y, _m, _d, (*n.borrow()), _hh, _mm, _ss)
    });
}
pub fn step_54(
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
        n_mon_46(_y, _m, _d, 0_i64, _hh, _mm, _ss)
    });
}
pub fn step_55(
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
pub fn scale_add_56(v: i64, f: i64, a: i64) -> i64 {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let f: Value<i64> = Rc::new(RefCell::new(f));
    let a: Value<i64> = Rc::new(RefCell::new(a));
    return if ((*v.borrow()) < 0_i64) {
        (((((*v.borrow()) + 1_i64) * (*f.borrow())) + (*a.borrow())) - (*f.borrow()))
    } else {
        (((((*v.borrow()) - 1_i64) * (*f.borrow())) + (*a.borrow())) + (*f.borrow()))
    };
}
pub fn ymd_ord_57(y: i64, m: i8, d: i8) -> i64 {
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
pub fn day_difference_58(y1: i64, m1: i8, d1: i8, y2: i64, m2: i8, d2: i8) -> i64 {
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
        (({ ymd_ord_57((*a_c4_off.borrow()), (*m1.borrow()), (*d1.borrow())) })
            - ({ ymd_ord_57((*b_c4_off.borrow()), (*m2.borrow()), (*d2.borrow())) })),
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
pub fn difference_59(
    _a0: absl_time_internal_cctz_detail_year_tag,
    f1: absl_time_internal_cctz_detail_fields,
    f2: absl_time_internal_cctz_detail_fields,
) -> i64 {
    let _a0: Value<absl_time_internal_cctz_detail_year_tag> = Rc::new(RefCell::new(_a0));
    let f1: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f1));
    let f2: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f2));
    return ((*(*f1.borrow()).y.borrow()) - (*(*f2.borrow()).y.borrow()));
}
pub fn difference_60(
    _a0: absl_time_internal_cctz_detail_month_tag,
    f1: absl_time_internal_cctz_detail_fields,
    f2: absl_time_internal_cctz_detail_fields,
) -> i64 {
    let _a0: Value<absl_time_internal_cctz_detail_month_tag> = Rc::new(RefCell::new(_a0));
    let f1: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f1));
    let f2: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f2));
    return ({
        let _v: i64 = ({
            difference_59(
                absl_time_internal_cctz_detail_year_tag {},
                (*f1.borrow()).clone(),
                (*f2.borrow()).clone(),
            )
        });
        let _a: i64 = ((((*(*f1.borrow()).m.borrow()) as i32)
            - ((*(*f2.borrow()).m.borrow()) as i32)) as i64);
        scale_add_56(_v, 12_i64, _a)
    });
}
pub fn difference_61(
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
        day_difference_58(_y1, _m1, _d1, _y2, _m2, _d2)
    });
}
pub fn difference_62(
    _a0: absl_time_internal_cctz_detail_hour_tag,
    f1: absl_time_internal_cctz_detail_fields,
    f2: absl_time_internal_cctz_detail_fields,
) -> i64 {
    let _a0: Value<absl_time_internal_cctz_detail_hour_tag> = Rc::new(RefCell::new(_a0));
    let f1: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f1));
    let f2: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f2));
    return ({
        let _v: i64 = ({
            difference_61(
                absl_time_internal_cctz_detail_day_tag {},
                (*f1.borrow()).clone(),
                (*f2.borrow()).clone(),
            )
        });
        let _a: i64 = ((((*(*f1.borrow()).hh.borrow()) as i32)
            - ((*(*f2.borrow()).hh.borrow()) as i32)) as i64);
        scale_add_56(_v, 24_i64, _a)
    });
}
pub fn difference_63(
    _a0: absl_time_internal_cctz_detail_minute_tag,
    f1: absl_time_internal_cctz_detail_fields,
    f2: absl_time_internal_cctz_detail_fields,
) -> i64 {
    let _a0: Value<absl_time_internal_cctz_detail_minute_tag> = Rc::new(RefCell::new(_a0));
    let f1: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f1));
    let f2: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f2));
    return ({
        let _v: i64 = ({
            difference_62(
                absl_time_internal_cctz_detail_hour_tag {},
                (*f1.borrow()).clone(),
                (*f2.borrow()).clone(),
            )
        });
        let _a: i64 = ((((*(*f1.borrow()).mm.borrow()) as i32)
            - ((*(*f2.borrow()).mm.borrow()) as i32)) as i64);
        scale_add_56(_v, 60_i64, _a)
    });
}
pub fn difference_64(
    _a0: absl_time_internal_cctz_detail_second_tag,
    f1: absl_time_internal_cctz_detail_fields,
    f2: absl_time_internal_cctz_detail_fields,
) -> i64 {
    let _a0: Value<absl_time_internal_cctz_detail_second_tag> = Rc::new(RefCell::new(_a0));
    let f1: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f1));
    let f2: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f2));
    return ({
        let _v: i64 = ({
            difference_63(
                absl_time_internal_cctz_detail_minute_tag {},
                (*f1.borrow()).clone(),
                (*f2.borrow()).clone(),
            )
        });
        let _a: i64 = ((((*(*f1.borrow()).ss.borrow()) as i32)
            - ((*(*f2.borrow()).ss.borrow()) as i32)) as i64);
        scale_add_56(_v, 60_i64, _a)
    });
}
pub fn align_65(
    _a0: absl_time_internal_cctz_detail_second_tag,
    f: absl_time_internal_cctz_detail_fields,
) -> absl_time_internal_cctz_detail_fields {
    let _a0: Value<absl_time_internal_cctz_detail_second_tag> = Rc::new(RefCell::new(_a0));
    let f: Value<absl_time_internal_cctz_detail_fields> = Rc::new(RefCell::new(f));
    return (*f.borrow()).clone();
}
pub fn align_66(
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
pub fn align_67(
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
pub fn align_68(
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
pub fn align_69(
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
pub fn align_70(
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
pub fn operator_add_71(
    a: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
    n: i64,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ {
    let a: Value<
        absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
    > = Rc::new(RefCell::new(a));
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_2 ( {  (  { step_53 ( absl_time_internal_cctz_detail_day_tag { }  , ((*(*a.borrow()) . f_ .borrow()) ).clone()  , (*n.borrow())  , ) } )    } , )   ;
}
pub fn operator_sub_72(
    a: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
    n: i64,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ {
    let a: Value<
        absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_,
    > = Rc::new(RefCell::new(a));
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return if ((*n.borrow()) != <i64>::MIN) {
        absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_2 ( {  (  { step_53 ( absl_time_internal_cctz_detail_day_tag { }  , ((*(*a.borrow()) . f_ .borrow()) ).clone()  , - (*n.borrow())  , ) } )    } , )
    } else {
        absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_2 ( {  (  { step_53 ( absl_time_internal_cctz_detail_day_tag { }  , (  { step_53 ( absl_time_internal_cctz_detail_day_tag { }  , ((*(*a.borrow()) . f_ .borrow()) ).clone()  , - ( ( (*n.borrow()) + 1_i64 ) )  , ) } )   , 1_i64  , ) } )    } , )
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
                    align_68(
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
                    align_65(
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
                ({ align_66(absl_time_internal_minute_tag {}, (*f.borrow()).clone()) }),
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
                    ({ align_67(absl_time_internal_hour_tag {}, (*f.borrow()).clone()) }),
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
                    ({ align_68(absl_time_internal_day_tag {}, (*f.borrow()).clone()) }),
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
                    ({ align_69(absl_time_internal_month_tag {}, (*f.borrow()).clone()) }),
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
                    ({ align_70(absl_time_internal_year_tag {}, (*f.borrow()).clone()) }),
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
pub fn get_weekday_73(
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
pub fn next_weekday_74(
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
            get_weekday_73(_cs.as_pointer())
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
                        operator_add_71(_a, (((*j.borrow()) - (*i.borrow())) as i64))
                    });
                }
                (*j.borrow_mut()).prefix_inc();
            }
        }
        (*i.borrow_mut()).prefix_inc();
    }
    panic!("ub: non-void function does not return a value")
}
pub fn prev_weekday_75(
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
            get_weekday_73(_cs.as_pointer())
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
                        operator_sub_72(_a, (((*j.borrow()) - (*i.borrow())) as i64))
                    });
                }
                (*j.borrow_mut()).prefix_inc();
            }
        }
        (*i.borrow_mut()).prefix_inc();
    }
    panic!("ub: non-void function does not return a value")
}
pub fn get_yearday_76(
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
                is_leap_year_39(
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
pub fn GetWeekday_77(
    cs: absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_,
) -> absl_time_internal_cctz_detail_weekday {
    let cs: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_> =
        Rc::new(RefCell::new(cs));
    return ({
        let _cs : Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ > = Rc::new(RefCell::new(absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_5 ( {  cs .as_pointer()   } , None , ) ));
        get_weekday_73(_cs.as_pointer())
    });
}
pub fn NextWeekday_78(
    cd: absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_,
    wd: absl_time_internal_cctz_detail_weekday,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ {
    let cd: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_> =
        Rc::new(RefCell::new(cd));
    let wd: Value<absl_time_internal_cctz_detail_weekday> = Rc::new(RefCell::new(wd));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_15 ( { let __tmp_0 : Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ > = Rc::new(RefCell::new((  { next_weekday_74 ( absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_1 ( {  cd .as_pointer()   } , None , )  , (*wd.borrow())  , ) } )  )); __tmp_0.as_pointer()  } , None , )   ;
}
pub fn PrevWeekday_79(
    cd: absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_,
    wd: absl_time_internal_cctz_detail_weekday,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ {
    let cd: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_> =
        Rc::new(RefCell::new(cd));
    let wd: Value<absl_time_internal_cctz_detail_weekday> = Rc::new(RefCell::new(wd));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_15 ( { let __tmp_1 : Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ > = Rc::new(RefCell::new((  { prev_weekday_75 ( absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_day_tag_1 ( {  cd .as_pointer()   } , None , )  , (*wd.borrow())  , ) } )  )); __tmp_1.as_pointer()  } , None , )   ;
}
pub fn GetYearDay_80(
    cs: absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_,
) -> i32 {
    let cs: Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_second_tag_> =
        Rc::new(RefCell::new(cs));
    return ({
        let _cs : Value<absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ > = Rc::new(RefCell::new(absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_cctz_detail_second_tag_5 ( {  cs .as_pointer()   } , None , ) ));
        get_yearday_76(_cs.as_pointer())
    });
}
pub fn operator_eq_81(
    lhs: absl_time_internal_cctz_time_zone,
    rhs: absl_time_internal_cctz_time_zone,
) -> bool {
    let lhs: Value<absl_time_internal_cctz_time_zone> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_time_internal_cctz_time_zone> = Rc::new(RefCell::new(rhs));
    return (({ absl_time_internal_cctz_time_zoneImpl::effective_impl(&lhs.as_pointer()) })
        == ({ absl_time_internal_cctz_time_zoneImpl::effective_impl(&rhs.as_pointer()) }));
}
pub fn operator_ne_82(
    lhs: absl_time_internal_cctz_time_zone,
    rhs: absl_time_internal_cctz_time_zone,
) -> bool {
    let lhs: Value<absl_time_internal_cctz_time_zone> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_time_internal_cctz_time_zone> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_time_internal_cctz_time_zone = (*lhs.borrow()).clone();
        operator_eq_81(_lhs, (*rhs.borrow()).clone())
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
            operator_eq_81(
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
pub fn convert_83(
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
}pub  fn split_seconds_84 ( tp : Ptr< std_chrono_time_point_std_chrono_system_clock__std_chrono_duration_long_long__std_ratio________  > , ) -> (Value<std_chrono_time_point_std_chrono_steady_clock__std_chrono_duration_long_long__std_ratio________>, Value<std_chrono_duration_long_long__std_ratio_______>){
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
pub fn join_seconds_85(
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
    pub static kTicksPerNanosecond_86: Value<i64> = Rc::new(RefCell::new(4));
);
thread_local!(
    pub static kTicksPerSecond_87: Value<i64> = Rc::new(RefCell::new(4000000000));
);
pub fn FromInt64_88(v: i64, _a1: std_ratio______) -> absl_Duration {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({
        let _sec: i64 = ((*v.borrow()) / 1000000000_i64);
        let _ticks: i64 = (((((((*v.borrow()) % 1000000000_i64) * 4) * 1000_i64) * 1000_i64)
            * 1000_i64)
            / 1000000000_i64);
        MakeNormalizedDuration_89(_sec, _ticks)
    });
}
pub fn FromInt64_90(v: i64, _a1: std_ratio______) -> absl_Duration {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({
        let _sec: i64 = ((*v.borrow()) / 1000000_i64);
        let _ticks: i64 = (((((((*v.borrow()) % 1000000_i64) * 4) * 1000_i64) * 1000_i64)
            * 1000_i64)
            / 1000000_i64);
        MakeNormalizedDuration_89(_sec, _ticks)
    });
}
pub fn FromInt64_91(v: i64, _a1: std_ratio______) -> absl_Duration {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({
        let _sec: i64 = ((*v.borrow()) / 1000_i64);
        let _ticks: i64 =
            (((((((*v.borrow()) % 1000_i64) * 4) * 1000_i64) * 1000_i64) * 1000_i64) / 1000_i64);
        MakeNormalizedDuration_89(_sec, _ticks)
    });
}
pub fn FromInt64_92(v: i64, _a1: std_ratio______) -> absl_Duration {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({
        let _sec: i64 = ((*v.borrow()) / 1_i64);
        let _ticks: i64 =
            (((((((*v.borrow()) % 1_i64) * 4) * 1000_i64) * 1000_i64) * 1000_i64) / 1_i64);
        MakeNormalizedDuration_89(_sec, _ticks)
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
            operator_cmp_93(
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
            operator_eq_94(
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
pub fn operator_gt_95(lhs: absl_Duration, rhs: absl_Duration) -> bool {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_Duration = (*rhs.borrow()).clone();
        operator_lt_96(_lhs, (*lhs.borrow()).clone())
    });
}
pub fn operator_ge_97(lhs: absl_Duration, rhs: absl_Duration) -> bool {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_Duration = (*lhs.borrow()).clone();
        operator_lt_96(_lhs, (*rhs.borrow()).clone())
    });
}
pub fn operator_le_98(lhs: absl_Duration, rhs: absl_Duration) -> bool {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_Duration = (*rhs.borrow()).clone();
        operator_lt_96(_lhs, (*lhs.borrow()).clone())
    });
}
pub fn operator_ne_99(lhs: absl_Duration, rhs: absl_Duration) -> bool {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_Duration = (*lhs.borrow()).clone();
        operator_eq_94(_lhs, (*rhs.borrow()).clone())
    });
}
pub fn operator_add_100(lhs: absl_Duration, rhs: absl_Duration) -> absl_Duration {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return (*({
        absl_DurationImpl::operator_add_assign(&lhs.as_pointer(), (*rhs.borrow()).clone())
    })
    .upgrade()
    .deref())
    .clone();
}
pub fn operator_sub_101(lhs: absl_Duration, rhs: absl_Duration) -> absl_Duration {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return (*({
        absl_DurationImpl::operator_sub_assign(&lhs.as_pointer(), (*rhs.borrow()).clone())
    })
    .upgrade()
    .deref())
    .clone();
}
pub fn operator_div_102(lhs: absl_Duration, rhs: absl_Duration) -> i64 {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return ({
        let _num: absl_Duration = (*lhs.borrow()).clone();
        let _rem: Ptr<absl_Duration> = (lhs.as_pointer());
        IDivDuration_103(_num, (*rhs.borrow()).clone(), _rem)
    });
}
pub fn operator_rem_104(lhs: absl_Duration, rhs: absl_Duration) -> absl_Duration {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return (*({
        absl_DurationImpl::operator_rem_assign(&lhs.as_pointer(), (*rhs.borrow()).clone())
    })
    .upgrade()
    .deref())
    .clone();
}
pub fn ZeroDuration_105() -> absl_Duration {
    return absl_Duration::absl_Duration1();
}
pub fn AbsDuration_106(d: absl_Duration) -> absl_Duration {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return if ({
        let _lhs: absl_Duration = (*d.borrow()).clone();
        operator_lt_96(_lhs, ({ ZeroDuration_105() }))
    }) {
        ({
            let _d: absl_Duration = (*d.borrow()).clone();
            operator_neg_107(_d)
        })
    } else {
        (*d.borrow()).clone()
    };
}
pub fn Nanoseconds_108(n: i32) -> absl_Duration {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return ({ FromInt64_88(((*n.borrow()) as i64), std_ratio______ {}) });
}
pub fn Nanoseconds_109(n: i64) -> absl_Duration {
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({ FromInt64_88((*n.borrow()), std_ratio______ {}) });
}
pub fn Microseconds_110(n: i32) -> absl_Duration {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return ({ FromInt64_90(((*n.borrow()) as i64), std_ratio______ {}) });
}
pub fn Microseconds_111(n: i64) -> absl_Duration {
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({ FromInt64_90((*n.borrow()), std_ratio______ {}) });
}
pub fn Milliseconds_112(n: i32) -> absl_Duration {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return ({ FromInt64_91(((*n.borrow()) as i64), std_ratio______ {}) });
}
pub fn Milliseconds_113(n: i64) -> absl_Duration {
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({ FromInt64_91((*n.borrow()), std_ratio______ {}) });
}
pub fn Seconds_114(n: i64) -> absl_Duration {
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({ FromInt64_92((*n.borrow()), std_ratio______ {}) });
}
pub fn Seconds_115(n: i64) -> absl_Duration {
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return ({ FromInt64_92((*n.borrow()), std_ratio______ {}) });
}
pub fn Minutes_116(n: i32) -> absl_Duration {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return ({ FromInt64_117(((*n.borrow()) as i64), std_ratio______ {}) });
}
pub fn Hours_118(n: i32) -> absl_Duration {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return ({ FromInt64_119(((*n.borrow()) as i64), std_ratio______ {}) });
}
pub fn operator_shl_120(os: Ptr<std::fs::File>, d: absl_Duration) -> Ptr<std::fs::File> {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return os.write_all(
        &([(&({ FormatDuration_121((*d.borrow()).clone()) })
            .iter()
            .take(({ FormatDuration_121((*d.borrow()).clone()) }).len() - 1)
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
            operator_cmp_122(
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
            operator_eq_123(
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
pub fn operator_cmp_122(lhs: absl_Time, rhs: absl_Time) -> std::cmp::Ordering {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_Duration = (*(*lhs.borrow()).rep_.borrow()).clone();
        let _rhs: absl_Duration = (*(*rhs.borrow()).rep_.borrow()).clone();
        operator_cmp_93(_lhs, _rhs)
    });
}
pub fn operator_lt_124(lhs: absl_Time, rhs: absl_Time) -> bool {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_Duration = (*(*lhs.borrow()).rep_.borrow()).clone();
        let _rhs: absl_Duration = (*(*rhs.borrow()).rep_.borrow()).clone();
        operator_lt_96(_lhs, _rhs)
    });
}
pub fn operator_gt_125(lhs: absl_Time, rhs: absl_Time) -> bool {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_Time = (*rhs.borrow()).clone();
        operator_lt_124(_lhs, (*lhs.borrow()).clone())
    });
}
pub fn operator_ge_126(lhs: absl_Time, rhs: absl_Time) -> bool {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_Time = (*lhs.borrow()).clone();
        operator_lt_124(_lhs, (*rhs.borrow()).clone())
    });
}
pub fn operator_le_127(lhs: absl_Time, rhs: absl_Time) -> bool {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_Time = (*rhs.borrow()).clone();
        operator_lt_124(_lhs, (*lhs.borrow()).clone())
    });
}
pub fn operator_eq_123(lhs: absl_Time, rhs: absl_Time) -> bool {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_Duration = (*(*lhs.borrow()).rep_.borrow()).clone();
        let _rhs: absl_Duration = (*(*rhs.borrow()).rep_.borrow()).clone();
        operator_eq_94(_lhs, _rhs)
    });
}
pub fn operator_ne_128(lhs: absl_Time, rhs: absl_Time) -> bool {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return !({
        let _lhs: absl_Time = (*lhs.borrow()).clone();
        operator_eq_123(_lhs, (*rhs.borrow()).clone())
    });
}
pub fn operator_add_129(lhs: absl_Time, rhs: absl_Duration) -> absl_Time {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return (*({ absl_TimeImpl::operator_add_assign(&lhs.as_pointer(), (*rhs.borrow()).clone()) })
        .upgrade()
        .deref())
    .clone();
}
pub fn operator_add_130(lhs: absl_Duration, rhs: absl_Time) -> absl_Time {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return (*({ absl_TimeImpl::operator_add_assign(&rhs.as_pointer(), (*lhs.borrow()).clone()) })
        .upgrade()
        .deref())
    .clone();
}
pub fn operator_sub_131(lhs: absl_Time, rhs: absl_Duration) -> absl_Time {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return (*({ absl_TimeImpl::operator_sub_assign(&lhs.as_pointer(), (*rhs.borrow()).clone()) })
        .upgrade()
        .deref())
    .clone();
}
pub fn operator_sub_132(lhs: absl_Time, rhs: absl_Time) -> absl_Duration {
    let lhs: Value<absl_Time> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Time> = Rc::new(RefCell::new(rhs));
    return ({
        let _lhs: absl_Duration = (*(*lhs.borrow()).rep_.borrow()).clone();
        let _rhs: absl_Duration = (*(*rhs.borrow()).rep_.borrow()).clone();
        operator_sub_101(_lhs, _rhs)
    });
}
pub fn UnixEpoch_133() -> absl_Time {
    return <absl_Time>::default();
}
pub fn UniversalEpoch_134() -> absl_Time {
    return absl_Time::absl_Time1({
        ({ MakeDuration_135((((-24_i32 * 719162) as i64) * 3600_i64), 0_u32) })
    });
}
pub fn InfiniteFuture_136() -> absl_Time {
    return absl_Time::absl_Time1({ ({ MakeDuration_135(<i64>::MAX, !0_u32) }) });
}
pub fn InfinitePast_137() -> absl_Time {
    return absl_Time::absl_Time1({ ({ MakeDuration_135(<i64>::MIN, !0_u32) }) });
}
pub fn operator_eq_138(a: absl_TimeZone, b: absl_TimeZone) -> bool {
    let a: Value<absl_TimeZone> = Rc::new(RefCell::new(a));
    let b: Value<absl_TimeZone> = Rc::new(RefCell::new(b));
    return ({
        let _lhs: absl_time_internal_cctz_time_zone = (*(*a.borrow()).cz_.borrow()).clone();
        let _rhs: absl_time_internal_cctz_time_zone = (*(*b.borrow()).cz_.borrow()).clone();
        operator_eq_81(_lhs, _rhs)
    });
}
pub fn operator_ne_139(a: absl_TimeZone, b: absl_TimeZone) -> bool {
    let a: Value<absl_TimeZone> = Rc::new(RefCell::new(a));
    let b: Value<absl_TimeZone> = Rc::new(RefCell::new(b));
    return ({
        let _lhs: absl_time_internal_cctz_time_zone = (*(*a.borrow()).cz_.borrow()).clone();
        let _rhs: absl_time_internal_cctz_time_zone = (*(*b.borrow()).cz_.borrow()).clone();
        operator_ne_82(_lhs, _rhs)
    });
}
pub fn operator_shl_140(os: Ptr<std::fs::File>, tz: absl_TimeZone) -> Ptr<std::fs::File> {
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
            operator_eq_138(
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
pub fn LoadTimeZone_141(name: Vec<u8>, tz: Ptr<absl_TimeZone>) -> bool {
    let name: Value<Vec<u8>> = Rc::new(RefCell::new(name));
    let tz: Value<Ptr<absl_TimeZone>> = Rc::new(RefCell::new(tz));
    if (*name.borrow()).clone()
        == Ptr::from_string_literal(b"localtime")
            .to_c_string_iterator()
            .collect::<Vec<u8>>()
    {
        let __rhs = absl_TimeZone::absl_TimeZone1({ ({ local_time_zone_142() }) });
        (*tz.borrow()).write(__rhs);
        return true;
    }
    let cz: Value<absl_time_internal_cctz_time_zone> = Rc::new(RefCell::new(
        absl_time_internal_cctz_time_zone::absl_time_internal_cctz_time_zone1(),
    ));
    let b: Value<bool> = Rc::new(RefCell::new(
        ({
            let _name : Value<Vec<u8> > = Rc::new(RefCell::new(std_basic_string_char__std_char_traits_char___std_allocator_char__ :: std_basic_string_char__std_char_traits_char___std_allocator_char__1 ( {  name .as_pointer()   } , None , ) ));
            load_time_zone_143(_name.as_pointer(), (cz.as_pointer()))
        }),
    ));
    let __rhs = absl_TimeZone::absl_TimeZone1({ (*cz.borrow()).clone() });
    (*tz.borrow()).write(__rhs);
    return (*b.borrow());
}
pub fn FixedTimeZone_144(seconds: i32) -> absl_TimeZone {
    let seconds: Value<i32> = Rc::new(RefCell::new(seconds));
    return absl_TimeZone::absl_TimeZone1({
        ({
            let _offset : Value<std_chrono_duration_long_long__std_ratio_______ > = Rc::new(RefCell::new(std_chrono_duration_long_long__std_ratio_______ :: std_chrono_duration_long_long__std_ratio_______1 ( {  seconds .as_pointer()   } , ) ));
            fixed_time_zone_145(_offset.as_pointer())
        })
    });
}
pub fn UTCTimeZone_146() -> absl_TimeZone {
    return absl_TimeZone::absl_TimeZone1({ ({ utc_time_zone_147() }) });
}
pub fn LocalTimeZone_148() -> absl_TimeZone {
    return absl_TimeZone::absl_TimeZone1({ ({ local_time_zone_142() }) });
}
pub fn ToCivilSecond_149(
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
pub fn ToCivilMinute_150(
    t: absl_Time,
    tz: absl_TimeZone,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_ {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    let tz: Value<absl_TimeZone> = Rc::new(RefCell::new(tz));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_minute_tag_9 ( {  (  { absl_TimeZoneImpl :: At_absl_Time_const ( &tz .as_pointer()  , ((*t.borrow()) ).clone()  , ) } )  . cs  .as_pointer()   } , None , )   ;
}
pub fn ToCivilHour_151(
    t: absl_Time,
    tz: absl_TimeZone,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_ {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    let tz: Value<absl_TimeZone> = Rc::new(RefCell::new(tz));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_hour_tag_12 ( {  (  { absl_TimeZoneImpl :: At_absl_Time_const ( &tz .as_pointer()  , ((*t.borrow()) ).clone()  , ) } )  . cs  .as_pointer()   } , None , )   ;
}
pub fn ToCivilDay_152(
    t: absl_Time,
    tz: absl_TimeZone,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    let tz: Value<absl_TimeZone> = Rc::new(RefCell::new(tz));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_day_tag_16 ( {  (  { absl_TimeZoneImpl :: At_absl_Time_const ( &tz .as_pointer()  , ((*t.borrow()) ).clone()  , ) } )  . cs  .as_pointer()   } , None , )   ;
}
pub fn ToCivilMonth_153(
    t: absl_Time,
    tz: absl_TimeZone,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_ {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    let tz: Value<absl_TimeZone> = Rc::new(RefCell::new(tz));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_month_tag_19 ( {  (  { absl_TimeZoneImpl :: At_absl_Time_const ( &tz .as_pointer()  , ((*t.borrow()) ).clone()  , ) } )  . cs  .as_pointer()   } , None , )   ;
}
pub fn ToCivilYear_154(
    t: absl_Time,
    tz: absl_TimeZone,
) -> absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_ {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    let tz: Value<absl_TimeZone> = Rc::new(RefCell::new(tz));
    return absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_ :: absl_time_internal_cctz_detail_civil_time_absl_time_internal_year_tag_22 ( {  (  { absl_TimeZoneImpl :: At_absl_Time_const ( &tz .as_pointer()  , ((*t.borrow()) ).clone()  , ) } )  . cs  .as_pointer()   } , None , )   ;
}
pub fn FromCivil_155(
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
pub fn FromDateTime_156(
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
        ConvertDateTime_157(
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
pub fn operator_shl_162(os: Ptr<std::fs::File>, t: absl_Time) -> Ptr<std::fs::File> {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    return os.write_all(
        &([(&({ FormatTime_163((*t.borrow()).clone()) })
            .iter()
            .take(({ FormatTime_163((*t.borrow()).clone()) }).len() - 1)
            .map(|&c| c as u8)
            .collect::<Vec<u8>>()[..] as &[u8])]
        .concat()),
    );
}
pub fn MakeDuration_135(hi: i64, lo: Option<u32>) -> absl_Duration {
    let hi: Value<i64> = Rc::new(RefCell::new(hi));
    let lo: Value<u32> = Rc::new(RefCell::new(lo.unwrap_or(0_u32)));
    return absl_Duration::absl_Duration2({ (*hi.borrow()) }, { (*lo.borrow()) });
}
pub fn MakeDuration_164(hi: i64, lo: i64) -> absl_Duration {
    let hi: Value<i64> = Rc::new(RefCell::new(hi));
    let lo: Value<i64> = Rc::new(RefCell::new(lo));
    return ({ MakeDuration_135((*hi.borrow()), Some(((*lo.borrow()) as u32))) });
}
pub fn MakePosDoubleDuration_165(n: f64) -> absl_Duration {
    let n: Value<f64> = Rc::new(RefCell::new(n));
    let int_secs: Value<i64> = Rc::new(RefCell::new(((*n.borrow()) as i64)));
    let ticks: Value<u32> = Rc::new(RefCell::new(
        ((((*n.borrow()) - ((*int_secs.borrow()) as f64)) * (4000000000 as f64)).round() as u32),
    ));
    return if (((*ticks.borrow()) as i64) < 4000000000) {
        ({ MakeDuration_135((*int_secs.borrow()), Some((*ticks.borrow()))) })
    } else {
        ({
            MakeDuration_164(
                ((*int_secs.borrow()) + 1_i64),
                (((*ticks.borrow()) as i64) - 4000000000),
            )
        })
    };
}
pub fn MakeNormalizedDuration_89(sec: i64, ticks: i64) -> absl_Duration {
    let sec: Value<i64> = Rc::new(RefCell::new(sec));
    let ticks: Value<i64> = Rc::new(RefCell::new(ticks));
    return if ((*ticks.borrow()) < 0_i64) {
        ({ MakeDuration_164(((*sec.borrow()) - 1_i64), ((*ticks.borrow()) + 4000000000)) })
    } else {
        ({ MakeDuration_164((*sec.borrow()), (*ticks.borrow())) })
    };
}
pub fn GetRepHi_166(d: absl_Duration) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return ({ absl_Duration_HiRepImpl::Get(&(*d.borrow()).rep_hi_.as_pointer()) });
}
pub fn GetRepLo_167(d: absl_Duration) -> u32 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return (*(*d.borrow()).rep_lo_.borrow());
}
pub fn IsInfiniteDuration_168(d: absl_Duration) -> bool {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return (({ GetRepLo_167((*d.borrow()).clone()) }) == !0_u32);
}
pub fn OppositeInfinity_169(d: absl_Duration) -> absl_Duration {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return if (({ GetRepHi_166((*d.borrow()).clone()) }) < 0_i64) {
        ({ MakeDuration_135(<i64>::MAX, Some(!0_u32)) })
    } else {
        ({ MakeDuration_135(<i64>::MIN, Some(!0_u32)) })
    };
}
pub fn NegateAndSubtractOne_170(n: i64) -> i64 {
    let n: Value<i64> = Rc::new(RefCell::new(n));
    return if ((*n.borrow()) < 0_i64) {
        -((*n.borrow()) + 1_i64)
    } else {
        ((-(*n.borrow())) - 1_i64)
    };
}
pub fn FromUnixDuration_171(d: absl_Duration) -> absl_Time {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return absl_Time::absl_Time1({ (*d.borrow()).clone() });
}
pub fn ToUnixDuration_172(t: absl_Time) -> absl_Duration {
    let t: Value<absl_Time> = Rc::new(RefCell::new(t));
    return (*(*t.borrow()).rep_.borrow()).clone();
}
pub fn FromInt64_117(v: i64, _a1: std_ratio______) -> absl_Duration {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return if (((*v.borrow()) <= (<i64>::MAX / 60_i64)) && ((*v.borrow()) >= (<i64>::MIN / 60_i64)))
    {
        ({ MakeDuration_135(((*v.borrow()) * 60_i64), None) })
    } else {
        if ((*v.borrow()) > 0_i64) {
            ({ InfiniteDuration_173() })
        } else {
            ({
                let _d: absl_Duration = ({ InfiniteDuration_173() });
                operator_neg_107(_d)
            })
        }
    };
}
pub fn FromInt64_119(v: i64, _a1: std_ratio______) -> absl_Duration {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return if (((*v.borrow()) <= (<i64>::MAX / 3600_i64))
        && ((*v.borrow()) >= (<i64>::MIN / 3600_i64)))
    {
        ({ MakeDuration_135(((*v.borrow()) * 3600_i64), None) })
    } else {
        if ((*v.borrow()) > 0_i64) {
            ({ InfiniteDuration_173() })
        } else {
            ({
                let _d: absl_Duration = ({ InfiniteDuration_173() });
                operator_neg_107(_d)
            })
        }
    };
}
pub fn IsValidRep64_174(_a0: i32) -> bool {
    let _a0: Value<i32> = Rc::new(RefCell::new(_a0));
    return true;
}
pub fn IsValidRep64_175(_a0: i32) -> bool {
    let _a0: Value<i32> = Rc::new(RefCell::new(_a0));
    return true;
}
pub fn FromChrono_176(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromInt64_88(({ (*d.upgrade().deref()).count() }), std_ratio______ {}) });
}
pub fn FromChrono_177(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromInt64_90(({ (*d.upgrade().deref()).count() }), std_ratio______ {}) });
}
pub fn FromChrono_178(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromInt64_91(({ (*d.upgrade().deref()).count() }), std_ratio______ {}) });
}
pub fn FromChrono_179(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromInt64_92(({ (*d.upgrade().deref()).count() }), std_ratio______ {}) });
}
pub fn FromChrono_180(d: Ptr<std_chrono_duration_long__std_ratio_______>) -> absl_Duration {
    return ({ FromInt64_117(({ (*d.upgrade().deref()).count() }), std_ratio______ {}) });
}
pub fn FromChrono_181(d: Ptr<std_chrono_duration_long__std_ratio_______>) -> absl_Duration {
    return ({ FromInt64_119(({ (*d.upgrade().deref()).count() }), std_ratio______ {}) });
}
pub fn ToInt64_182(d: absl_Duration, _a1: std_ratio______) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({ ToInt64Nanoseconds_183((*d.borrow()).clone()) });
}
pub fn ToInt64_184(d: absl_Duration, _a1: std_ratio______) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({ ToInt64Microseconds_185((*d.borrow()).clone()) });
}
pub fn ToInt64_186(d: absl_Duration, _a1: std_ratio______) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({ ToInt64Milliseconds_187((*d.borrow()).clone()) });
}
pub fn ToInt64_188(d: absl_Duration, _a1: std_ratio______) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({ ToInt64Seconds_189((*d.borrow()).clone()) });
}
pub fn ToInt64_190(d: absl_Duration, _a1: std_ratio______) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({ ToInt64Minutes_191((*d.borrow()).clone()) });
}
pub fn ToInt64_192(d: absl_Duration, _a1: std_ratio______) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let _a1: Value<std_ratio______> = Rc::new(RefCell::new(_a1));
    return ({ ToInt64Hours_193((*d.borrow()).clone()) });
}
pub fn operator_lt_96(lhs: absl_Duration, rhs: absl_Duration) -> bool {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return if (({ GetRepHi_166((*lhs.borrow()).clone()) })
        != ({ GetRepHi_166((*rhs.borrow()).clone()) }))
    {
        (({ GetRepHi_166((*lhs.borrow()).clone()) }) < ({ GetRepHi_166((*rhs.borrow()).clone()) }))
    } else {
        if (({ GetRepHi_166((*lhs.borrow()).clone()) }) == <i64>::MIN) {
            (({ GetRepLo_167((*lhs.borrow()).clone()) }).wrapping_add(1_u32)
                < ({ GetRepLo_167((*rhs.borrow()).clone()) }).wrapping_add(1_u32))
        } else {
            (({ GetRepLo_167((*lhs.borrow()).clone()) })
                < ({ GetRepLo_167((*rhs.borrow()).clone()) }))
        }
    };
}
pub fn operator_cmp_93(lhs: absl_Duration, rhs: absl_Duration) -> std::cmp::Ordering {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    let lhs_hi: Value<i64> = Rc::new(RefCell::new(({ GetRepHi_166((*lhs.borrow()).clone()) })));
    let rhs_hi: Value<i64> = Rc::new(RefCell::new(({ GetRepHi_166((*rhs.borrow()).clone()) })));
    {
        let c: Value<std::cmp::Ordering> =
            Rc::new(RefCell::new((*lhs_hi.borrow()).cmp(&(*rhs_hi.borrow()))));
        if !((*c.borrow()) == std::cmp::Ordering::Equal) {
            return (*c.borrow_mut()).clone();
        }
    }
    let lhs_lo: Value<u32> = Rc::new(RefCell::new(({ GetRepLo_167((*lhs.borrow()).clone()) })));
    let rhs_lo: Value<u32> = Rc::new(RefCell::new(({ GetRepLo_167((*rhs.borrow()).clone()) })));
    return if ((*lhs_hi.borrow()) == <i64>::MIN) {
        ((*lhs_lo.borrow()).wrapping_add(1_u32)).cmp(&((*rhs_lo.borrow()).wrapping_add(1_u32)))
    } else {
        (*lhs_lo.borrow()).cmp(&(*rhs_lo.borrow()))
    };
}
pub fn operator_eq_94(lhs: absl_Duration, rhs: absl_Duration) -> bool {
    let lhs: Value<absl_Duration> = Rc::new(RefCell::new(lhs));
    let rhs: Value<absl_Duration> = Rc::new(RefCell::new(rhs));
    return (({ GetRepHi_166((*lhs.borrow()).clone()) })
        == ({ GetRepHi_166((*rhs.borrow()).clone()) }))
        && (({ GetRepLo_167((*lhs.borrow()).clone()) })
            == ({ GetRepLo_167((*rhs.borrow()).clone()) }));
}
pub fn operator_neg_107(d: absl_Duration) -> absl_Duration {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    return if (({ GetRepLo_167((*d.borrow()).clone()) }) == 0_u32) {
        if (({ GetRepHi_166((*d.borrow()).clone()) }) == <i64>::MIN) {
            ({ InfiniteDuration_173() })
        } else {
            ({ MakeDuration_135(-({ GetRepHi_166((*d.borrow()).clone()) }), None) })
        }
    } else {
        if ({ IsInfiniteDuration_168((*d.borrow()).clone()) }) {
            ({ OppositeInfinity_169((*d.borrow()).clone()) })
        } else {
            ({
                let _hi: i64 =
                    ({ NegateAndSubtractOne_170(({ GetRepHi_166((*d.borrow()).clone()) })) });
                let _lo: i64 = (4000000000 - (({ GetRepLo_167((*d.borrow()).clone()) }) as i64));
                MakeDuration_164(_hi, _lo)
            })
        }
    };
}
pub fn InfiniteDuration_173() -> absl_Duration {
    return ({ MakeDuration_135(<i64>::MAX, Some(!0_u32)) });
}
pub fn FromChrono_194(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromChrono_176((d).clone()) });
}
pub fn FromChrono_195(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromChrono_177((d).clone()) });
}
pub fn FromChrono_196(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromChrono_178((d).clone()) });
}
pub fn FromChrono_197(d: Ptr<std_chrono_duration_long_long__std_ratio_______>) -> absl_Duration {
    return ({ FromChrono_179((d).clone()) });
}
pub fn FromChrono_198(d: Ptr<std_chrono_duration_long__std_ratio_______>) -> absl_Duration {
    return ({ FromChrono_180((d).clone()) });
}
pub fn FromChrono_199(d: Ptr<std_chrono_duration_long__std_ratio_______>) -> absl_Duration {
    return ({ FromChrono_181((d).clone()) });
}
pub fn FromUnixNanos_200(ns: i64) -> absl_Time {
    let ns: Value<i64> = Rc::new(RefCell::new(ns));
    return ({ FromUnixDuration_171(({ Nanoseconds_109((*ns.borrow())) })) });
}
pub fn FromUnixMicros_201(us: i64) -> absl_Time {
    let us: Value<i64> = Rc::new(RefCell::new(us));
    return ({ FromUnixDuration_171(({ Microseconds_111((*us.borrow())) })) });
}
pub fn FromUnixMillis_202(ms: i64) -> absl_Time {
    let ms: Value<i64> = Rc::new(RefCell::new(ms));
    return ({ FromUnixDuration_171(({ Milliseconds_113((*ms.borrow())) })) });
}
pub fn FromUnixSeconds_203(s: i64) -> absl_Time {
    let s: Value<i64> = Rc::new(RefCell::new(s));
    return ({ FromUnixDuration_171(({ Seconds_114((*s.borrow())) })) });
}
pub fn FromTimeT_204(t: i64) -> absl_Time {
    let t: Value<i64> = Rc::new(RefCell::new(t));
    return ({ FromUnixDuration_171(({ Seconds_115((*t.borrow())) })) });
}
pub fn ToInt64Nanoseconds_183(d: absl_Duration) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    if (({ GetRepHi_166((*d.borrow()).clone()) }) >= 0_i64)
        && ((({ GetRepHi_166((*d.borrow()).clone()) }) >> 33) == 0_i64)
    {
        return ((((({ GetRepHi_166((*d.borrow()).clone()) }) * 1000_i64) * 1000_i64) * 1000_i64)
            + ((({ GetRepLo_167((*d.borrow()).clone()) }) as i64) / 4));
    } else {
        return ({
            let _lhs: absl_Duration = (*d.borrow()).clone();
            operator_div_102(_lhs, ({ Nanoseconds_108(1) }))
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ToInt64Microseconds_185(d: absl_Duration) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    if (({ GetRepHi_166((*d.borrow()).clone()) }) >= 0_i64)
        && ((({ GetRepHi_166((*d.borrow()).clone()) }) >> 43) == 0_i64)
    {
        return (((({ GetRepHi_166((*d.borrow()).clone()) }) * 1000_i64) * 1000_i64)
            + ((({ GetRepLo_167((*d.borrow()).clone()) }) as i64) / (4 * 1000_i64)));
    } else {
        return ({
            let _lhs: absl_Duration = (*d.borrow()).clone();
            operator_div_102(_lhs, ({ Microseconds_110(1) }))
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ToInt64Milliseconds_187(d: absl_Duration) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    if (({ GetRepHi_166((*d.borrow()).clone()) }) >= 0_i64)
        && ((({ GetRepHi_166((*d.borrow()).clone()) }) >> 53) == 0_i64)
    {
        return ((({ GetRepHi_166((*d.borrow()).clone()) }) * 1000_i64)
            + ((({ GetRepLo_167((*d.borrow()).clone()) }) as i64) / ((4 * 1000_i64) * 1000_i64)));
    } else {
        return ({
            let _lhs: absl_Duration = (*d.borrow()).clone();
            operator_div_102(_lhs, ({ Milliseconds_112(1) }))
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ToInt64Seconds_189(d: absl_Duration) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let hi: Value<i64> = Rc::new(RefCell::new(({ GetRepHi_166((*d.borrow()).clone()) })));
    if ({ IsInfiniteDuration_168((*d.borrow()).clone()) }) {
        return (*hi.borrow());
    }
    if ((*hi.borrow()) < 0_i64) && (({ GetRepLo_167((*d.borrow()).clone()) }) != 0_u32) {
        (*hi.borrow_mut()).prefix_inc();
    }
    return (*hi.borrow());
}
pub fn ToInt64Minutes_191(d: absl_Duration) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let hi: Value<i64> = Rc::new(RefCell::new(({ GetRepHi_166((*d.borrow()).clone()) })));
    if ({ IsInfiniteDuration_168((*d.borrow()).clone()) }) {
        return (*hi.borrow());
    }
    if ((*hi.borrow()) < 0_i64) && (({ GetRepLo_167((*d.borrow()).clone()) }) != 0_u32) {
        (*hi.borrow_mut()).prefix_inc();
    }
    return ((*hi.borrow()) / 60_i64);
}
pub fn ToInt64Hours_193(d: absl_Duration) -> i64 {
    let d: Value<absl_Duration> = Rc::new(RefCell::new(d));
    let hi: Value<i64> = Rc::new(RefCell::new(({ GetRepHi_166((*d.borrow()).clone()) })));
    if ({ IsInfiniteDuration_168((*d.borrow()).clone()) }) {
        return (*hi.borrow());
    }
    if ((*hi.borrow()) < 0_i64) && (({ GetRepLo_167((*d.borrow()).clone()) }) != 0_u32) {
        (*hi.borrow_mut()).prefix_inc();
    }
    return ((*hi.borrow()) / ((60 * 60) as i64));
}
pub fn SleepFor_205(duration: absl_Duration) {
    let duration: Value<absl_Duration> = Rc::new(RefCell::new(duration));
    ({ AbslInternalSleepFor_206((*duration.borrow()).clone()) });
}
thread_local!(
    static kNoTimeout_207: Value<u64> = Rc::new(RefCell::new(18446744073709551615));
);
thread_local!(
    static kMaxNanos_208: Value<i64> = Rc::new(RefCell::new(9223372036854775807));
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
pub fn GetOrCreateCurrentThreadIdentity_209() -> Ptr<absl_base_internal_ThreadIdentity> {
    let identity: Value<Ptr<absl_base_internal_ThreadIdentity>> =
        Rc::new(RefCell::new(({ CurrentThreadIdentityIfPresent_210() })));
    if ((((false) || ((*identity.borrow()).is_null())) as i64) != 0) {
        return ({ CreateThreadIdentity_211() });
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
        ({ AbslInternalPerThreadSemInit_212((*identity.borrow()).clone()) });
    }
}
impl absl_synchronization_internal_PerThreadSem {
    fn Post(identity: Ptr<absl_base_internal_ThreadIdentity>) {
        let identity: Value<Ptr<absl_base_internal_ThreadIdentity>> =
            Rc::new(RefCell::new(identity));
        ({ AbslInternalPerThreadSemPost_213((*identity.borrow()).clone()) });
    }
}
impl absl_synchronization_internal_PerThreadSem {
    fn Wait(t: absl_synchronization_internal_KernelTimeout) -> bool {
        let t: Value<absl_synchronization_internal_KernelTimeout> = Rc::new(RefCell::new(t));
        return ({ AbslInternalPerThreadSemWait_214((*t.borrow()).clone()) });
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
pub type v8_base_AbortMode = i32;
pub const v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures: v8_base_AbortMode = 0;
pub const v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures: v8_base_AbortMode = 1;
pub const v8_base_AbortMode_kExitIfNoSecurityImpact: v8_base_AbortMode = 2;
pub const v8_base_AbortMode_kImmediateCrash: v8_base_AbortMode = 3;
pub const v8_base_AbortMode_kDefault: v8_base_AbortMode = 4;
thread_local!();
pub fn ControlledCrashesAreHarmless_217() -> bool {
    return ((*g_abort_mode_216.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_216.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn DcheckFailuresAreIgnored_218() -> bool {
    return ((*g_abort_mode_216.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_216.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn FatalErrorsWithNoSecurityImpactShouldExit_219() -> bool {
    return ((*g_abort_mode_216.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitIfNoSecurityImpact);
}
#[derive(Default)]
pub struct v8_base_pass_value_or_ref_unsigned_long__true_ {}
impl Clone for v8_base_pass_value_or_ref_unsigned_long__true_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_pass_value_or_ref_unsigned_long__true_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_pass_value_or_ref_unsigned_long__true_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_pass_value_or_ref_unsigned_long__true_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static kUnimplementedCodeMessage_220: Value<Ptr<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"unimplemented code"),
    ));
);
thread_local!(
    pub static kUnreachableCodeMessage_221: Value<Ptr<u8>> =
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
    pub static is_enum_222: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static is_enum_223: Value<bool> = Rc::new(RefCell::new(false));
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
pub type v8_base_comparison_underlying_type_unsigned_long__Dummy = u32;
thread_local!(
    pub static is_enum_224: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_comparison_underlying_type_unsigned_long_ {}
impl Clone for v8_base_comparison_underlying_type_unsigned_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_comparison_underlying_type_unsigned_long_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_comparison_underlying_type_unsigned_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_comparison_underlying_type_unsigned_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static value_225: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_226: Value<bool> = Rc::new(RefCell::new(true));
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
    pub static value_227: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_228: Value<bool> = Rc::new(RefCell::new(false));
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
thread_local!(
    pub static value_229: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_unsigned_long__unsigned_long_ {}
impl Clone for v8_base_is_signed_vs_unsigned_unsigned_long__unsigned_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_signed_vs_unsigned_unsigned_long__unsigned_long_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_signed_vs_unsigned_unsigned_long__unsigned_long_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_signed_vs_unsigned_unsigned_long__unsigned_long_ {
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
pub struct v8_base_is_unsigned_vs_signed_unsigned_long__unsigned_long_ {}
impl Clone for v8_base_is_unsigned_vs_signed_unsigned_long__unsigned_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_unsigned_vs_signed_unsigned_long__unsigned_long_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_unsigned_vs_signed_unsigned_long__unsigned_long_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_unsigned_vs_signed_unsigned_long__unsigned_long_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn CmpEQImpl_230(lhs: u64, rhs: u64) -> bool {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return ((*lhs.borrow()) == (*rhs.borrow()));
}
thread_local!(
    pub static value_231: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Default)]
pub struct v8_base_is_trivially_copyable_v8_base_AddressRegion_ {}
impl Clone for v8_base_is_trivially_copyable_v8_base_AddressRegion_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_trivially_copyable_v8_base_AddressRegion_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_trivially_copyable_v8_base_AddressRegion_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_trivially_copyable_v8_base_AddressRegion_ {
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
pub fn make_uint64_232(high: u32, low: u32) -> u64 {
    let high: Value<u32> = Rc::new(RefCell::new(high));
    let low: Value<u32> = Rc::new(RefCell::new(low));
    return (((*high.borrow()) as u64) << 32).wrapping_add(((*low.borrow()) as u64));
}
pub fn RoundDown_233(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    return ((*x.borrow()) & (-(*m.borrow()) as u64));
}
pub fn RoundUp_234(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    (&(0));
    return ({
        let _x: u64 = ((*x.borrow()).wrapping_add((((*m.borrow()) - 1_i64) as u64)) as u64);
        let _m: i64 = (*m.borrow());
        RoundDown_233(_x, _m)
    });
}
pub fn IsAligned_235(value: u64, alignment: u64) -> bool {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let alignment: Value<u64> = Rc::new(RefCell::new(alignment));
    return (((*value.borrow()) & ((*alignment.borrow()).wrapping_sub(1_u64))) == 0_u64);
}
pub type anon_236 = u8;
pub const anon_236_ONCE_STATE_UNINITIALIZED: anon_236 = 0;
pub const anon_236_ONCE_STATE_EXECUTING_FUNCTION: anon_236 = 1;
pub const anon_236_ONCE_STATE_DONE: anon_236 = 2;
pub fn CallOnce_237(once: Ptr<std_atomic_unsigned_char_>, init_func: std_function_void____) {
    let once: Value<Ptr<std_atomic_unsigned_char_>> = Rc::new(RefCell::new(once));
    let init_func: Value<std_function_void____> = Rc::new(RefCell::new(init_func));
    if ((({ (*(*once.borrow()).upgrade().deref()).load_const(Some(2)) }) as i32)
        != (anon_236_ONCE_STATE_DONE as i32))
    {
        ({ CallOnceImpl_238((*once.borrow()).clone(), (*init_func.borrow()).clone()) });
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
        let __this : Value<v8_base_MutexGuardIf> = Rc::new(RefCell::new(Self { mutex_ : Rc::new(RefCell::new(std_optional_v8_base_LockGuard_v8_base_Mutex__ :: std_optional_v8_base_LockGuard_v8_base_Mutex__2 ( ) )) , } )) ;
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
    pub static kStackWalkError_240: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static kStackWalkMaxNameLen_241: Value<i32> = Rc::new(RefCell::new(256));
);
thread_local!(
    pub static kStackWalkMaxTextLen_242: Value<i32> = Rc::new(RefCell::new(256));
);
thread_local!(
    static msPerSecond_243: Value<i32> = Rc::new(RefCell::new(1000));
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
pub fn EnsureConsoleOutput_244() {}
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
            frame_address.unwrap_or(({ __builtin_frame_address_245(0_u32) })),
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
pub struct v8_base_AddressRegion_StartAddressLess {}
impl Clone for v8_base_AddressRegion_StartAddressLess {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_AddressRegion_StartAddressLess> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_AddressRegion_StartAddressLess> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_AddressRegion_StartAddressLess {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_AddressRegion {
    address_: Value<u64>,
    size_: Value<usize>,
}
impl v8_base_AddressRegion {
    pub fn v8_base_AddressRegion1(address: u64, size: usize) -> Self {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let __this: Value<v8_base_AddressRegion> = Rc::new(RefCell::new(Self {
            address_: Rc::new(RefCell::new((*address.borrow()))),
            size_: Rc::new(RefCell::new((*size.borrow()))),
        }));
        let this: Ptr<v8_base_AddressRegion> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for v8_base_AddressRegion {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_AddressRegionImpl::operator_eq(
                &Rc::new(RefCell::new(v8_base_AddressRegion {
                    address_: self.address_.clone(),
                    size_: self.size_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(v8_base_AddressRegion {
                    address_: other.address_.clone(),
                    size_: other.size_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for v8_base_AddressRegion {}
impl Clone for v8_base_AddressRegion {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_AddressRegion> = Rc::new(RefCell::new(Self {
            address_: Rc::new(RefCell::new((*self.address_.borrow()))),
            size_: Rc::new(RefCell::new((*self.size_.borrow()))),
        }));
        let this: Ptr<v8_base_AddressRegion> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_AddressRegion {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.address_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.size_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            address_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
            size_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn operator_shl_246(
    out: Ptr<std::fs::File>,
    region: v8_base_AddressRegion,
) -> Ptr<std::fs::File> {
    let region: Value<v8_base_AddressRegion> = Rc::new(RefCell::new(region));
    return write!(
        out,
        "[{:}+{:}]",
        Rc::new(RefCell::new(
            ({ v8_base_AddressRegionImpl::begin(&region.as_pointer(),) })
        ))
        .as_pointer()
        .reinterpret_cast::<::libc::c_void>(),
        Rc::new(RefCell::new(
            ({ v8_base_AddressRegionImpl::size(&region.as_pointer(),) })
        ))
        .as_pointer(),
    );
}
thread_local!(
    static kMultiplier_247: Value<i64> = Rc::new(RefCell::new(25214903917_i64));
);
thread_local!(
    static kAddend_248: Value<i64> = Rc::new(RefCell::new(11_i64));
);
thread_local!(
    static kMask_249: Value<i64> = Rc::new(RefCell::new(281474976710655_i64));
);
#[derive()]
pub struct v8_base_RandomNumberGenerator {
    initial_seed_: Value<i64>,
    state0_: Value<u64>,
    state1_: Value<u64>,
}
impl v8_base_RandomNumberGenerator {
    pub fn v8_base_RandomNumberGenerator1(seed: i64) -> Self {
        let seed: Value<i64> = Rc::new(RefCell::new(seed));
        let __this: Value<v8_base_RandomNumberGenerator> = Rc::new(RefCell::new(Self {
            initial_seed_: <Value<i64>>::default(),
            state0_: <Value<u64>>::default(),
            state1_: <Value<u64>>::default(),
        }));
        let this: Ptr<v8_base_RandomNumberGenerator> = __this.as_pointer();
        ({ v8_base_RandomNumberGeneratorImpl::SetSeed(&this, (*seed.borrow())) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn ToDouble(random: u64) -> f64 {
        let random: Value<u64> = Rc::new(RefCell::new(random));
        let random_0_to_2_53: Value<f64> =
            Rc::new(RefCell::new((((*random.borrow()) >> 11) as f64)));
        let k2_53: Value<f64> = Rc::new(RefCell::new(((1_u64 << 53) as f64)));
        return ((*random_0_to_2_53.borrow()) / (*k2_53.borrow()));
    }
    pub fn XorShift128(state0: Ptr<u64>, state1: Ptr<u64>) -> u64 {
        let state0: Value<Ptr<u64>> = Rc::new(RefCell::new(state0));
        let state1: Value<Ptr<u64>> = Rc::new(RefCell::new(state1));
        let s1: Value<u64> = Rc::new(RefCell::new(((*state0.borrow()).read())));
        let s0: Value<u64> = Rc::new(RefCell::new(((*state1.borrow()).read())));
        let __rhs = (*s0.borrow());
        (*state0.borrow()).write(__rhs);
        let __rhs = ((*s1.borrow()) << 23);
        (*s1.borrow_mut()) ^= __rhs;
        let __rhs = ((*s1.borrow()) >> 17);
        (*s1.borrow_mut()) ^= __rhs;
        (*s1.borrow_mut()) ^= (*s0.borrow());
        (*s1.borrow_mut()) ^= ((*s0.borrow()) >> 26);
        let __rhs = (*s1.borrow());
        (*state1.borrow()).write(__rhs);
        return (*s0.borrow()).wrapping_add((*s1.borrow()));
    }
    pub fn min() -> u32 {
        return 0_u32;
    }
    pub fn max() -> u32 {
        return <u32>::MAX;
    }
}
impl Clone for v8_base_RandomNumberGenerator {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_RandomNumberGenerator> = Rc::new(RefCell::new(Self {
            initial_seed_: Rc::new(RefCell::new((*self.initial_seed_.borrow()))),
            state0_: Rc::new(RefCell::new((*self.state0_.borrow()))),
            state1_: Rc::new(RefCell::new((*self.state1_.borrow()))),
        }));
        let this: Ptr<v8_base_RandomNumberGenerator> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_RandomNumberGenerator {
    fn default() -> Self {
        { v8_base_RandomNumberGenerator::v8_base_RandomNumberGenerator2() }
    }
}
impl ByteRepr for v8_base_RandomNumberGenerator {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.initial_seed_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.state0_.borrow()).to_bytes(&mut buf[8..16]);
        (*self.state1_.borrow()).to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            initial_seed_: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
            state0_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[8..16]))),
            state1_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[16..24]))),
        }
    }
}
thread_local!(
    pub static kAllocationFailure_250: Value<u64> = Rc::new(RefCell::new(18446744073709551615));
);
pub type v8_base_RegionAllocator_RegionState = i32;
pub const v8_base_RegionAllocator_RegionState_kFree: v8_base_RegionAllocator_RegionState = 0;
pub const v8_base_RegionAllocator_RegionState_kExcluded: v8_base_RegionAllocator_RegionState = 1;
pub const v8_base_RegionAllocator_RegionState_kAllocated: v8_base_RegionAllocator_RegionState = 2;
pub type v8_base_RegionAllocator_AllocationStrategy = i32;
pub const v8_base_RegionAllocator_AllocationStrategy_kFirstFit:
    v8_base_RegionAllocator_AllocationStrategy = 0;
pub const v8_base_RegionAllocator_AllocationStrategy_kLargestFit:
    v8_base_RegionAllocator_AllocationStrategy = 1;
#[derive(Default)]
struct v8_base_RegionAllocator_Region {
    state_: Value<v8_base_RegionAllocator_RegionState>,
}
impl v8_base_RegionAllocator_Region {
    pub fn v8_base_RegionAllocator_Region(
        address: u64,
        size: usize,
        state: v8_base_RegionAllocator_RegionState,
    ) -> Self {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let state: Value<v8_base_RegionAllocator_RegionState> = Rc::new(RefCell::new(state));
        let __this: Value<v8_base_RegionAllocator_Region> = Rc::new(RefCell::new(Self {
            state_: Rc::new(RefCell::new((*state.borrow()))),
        }));
        let this: Ptr<v8_base_RegionAllocator_Region> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_RegionAllocator_Region {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_RegionAllocator_Region> = Rc::new(RefCell::new(Self {
            state_: Rc::new(RefCell::new((*self.state_.borrow()))),
        }));
        let this: Ptr<v8_base_RegionAllocator_Region> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_RegionAllocator_Region {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.state_.borrow()).to_bytes(&mut buf[16..20]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            state_: Rc::new(RefCell::new(
                <v8_base_RegionAllocator_RegionState>::from_bytes(&buf[16..20]),
            )),
        }
    }
}
#[derive(Default)]
struct v8_base_RegionAllocator_AddressEndOrder {}
impl Clone for v8_base_RegionAllocator_AddressEndOrder {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_RegionAllocator_AddressEndOrder> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_RegionAllocator_AddressEndOrder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_RegionAllocator_AddressEndOrder {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
struct v8_base_RegionAllocator_SizeAddressOrder {}
impl Clone for v8_base_RegionAllocator_SizeAddressOrder {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_RegionAllocator_SizeAddressOrder> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_RegionAllocator_SizeAddressOrder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_RegionAllocator_SizeAddressOrder {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_RegionAllocator {  whole_region_ : Value<v8_base_RegionAllocator_Region > ,  region_size_in_pages_ : Value<usize > ,  max_load_for_randomization_ : Value<usize > ,  free_size_ : Value<usize > ,  page_size_ : Value<usize > ,  all_regions_ : Value<std_set_v8_base_RegionAllocator_Region_ptr__v8_base_RegionAllocator_AddressEndOrder__std_allocator_v8_base_RegionAllocator_Region_ptr__ > ,  free_regions_ : Value<std_set_v8_base_RegionAllocator_Region_ptr__v8_base_RegionAllocator_SizeAddressOrder__std_allocator_v8_base_RegionAllocator_Region_ptr__ > ,  on_split_ : Value<std_function_void__unsigned_long__unsigned_long__ > ,  on_merge_ : Value<std_function_void__unsigned_long__unsigned_long__ > , }
impl v8_base_RegionAllocator {}
impl ByteRepr for v8_base_RegionAllocator {
    fn byte_size() -> usize {
        168
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.whole_region_.borrow()).to_bytes(&mut buf[0..24]);
        (*self.region_size_in_pages_.borrow()).to_bytes(&mut buf[24..32]);
        (*self.max_load_for_randomization_.borrow()).to_bytes(&mut buf[32..40]);
        (*self.free_size_.borrow()).to_bytes(&mut buf[40..48]);
        (*self.page_size_.borrow()).to_bytes(&mut buf[48..56]);
        (*self.all_regions_.borrow()).to_bytes(&mut buf[56..80]);
        (*self.free_regions_.borrow()).to_bytes(&mut buf[80..104]);
        (*self.on_split_.borrow()).to_bytes(&mut buf[104..136]);
        (*self.on_merge_.borrow()).to_bytes(&mut buf[136..168]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self { whole_region_: Rc::new(RefCell::new(<v8_base_RegionAllocator_Region >::from_bytes(&buf[0..24]))), region_size_in_pages_: Rc::new(RefCell::new(<usize >::from_bytes(&buf[24..32]))), max_load_for_randomization_: Rc::new(RefCell::new(<usize >::from_bytes(&buf[32..40]))), free_size_: Rc::new(RefCell::new(<usize >::from_bytes(&buf[40..48]))), page_size_: Rc::new(RefCell::new(<usize >::from_bytes(&buf[48..56]))), all_regions_: Rc::new(RefCell::new(<std_set_v8_base_RegionAllocator_Region_ptr__v8_base_RegionAllocator_AddressEndOrder__std_allocator_v8_base_RegionAllocator_Region_ptr__ >::from_bytes(&buf[56..80]))), free_regions_: Rc::new(RefCell::new(<std_set_v8_base_RegionAllocator_Region_ptr__v8_base_RegionAllocator_SizeAddressOrder__std_allocator_v8_base_RegionAllocator_Region_ptr__ >::from_bytes(&buf[80..104]))), on_split_: Rc::new(RefCell::new(<std_function_void__unsigned_long__unsigned_long__ >::from_bytes(&buf[104..136]))), on_merge_: Rc::new(RefCell::new(<std_function_void__unsigned_long__unsigned_long__ >::from_bytes(&buf[136..168]))), }
    }
}
thread_local!(
    pub static kNullAddress_251: Value<u64> = Rc::new(RefCell::new(0));
);
pub trait v8_base_VirtualAddressSpaceBase {
    fn FreeSubspace(&self, subspace: Ptr<v8_base_VirtualAddressSubspace>);
}
#[derive()]
pub struct v8_base_VirtualAddressSpace {}
impl v8_base_VirtualAddressSpace {
    pub fn v8_base_VirtualAddressSpace() -> Self {
        let __this: Value<v8_base_VirtualAddressSpace> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_VirtualAddressSpace> = __this.as_pointer();
        (&(0));
        (&(0));
        (&(0));
        (&(0));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl v8_base_VirtualAddressSpaceBase for v8_base_VirtualAddressSpace {
    fn SetRandomSeed(&self, seed: i64) {
        let seed: Value<i64> = Rc::new(RefCell::new(seed));
        ({ v8_base_OS::SetRandomMmapSeed((*seed.borrow())) });
    }
    fn RandomPageAddress(&self) -> u64 {
        return ({ v8_base_OS::GetRandomMmapAddr() }).to_int();
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
        (&(0));
        (&(0));
        (&(0));
        return ({
            v8_base_OS::Allocate(
                (*hint.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
                (*alignment.borrow()),
                ({ ToMemoryPermission_252((*permissions.borrow())) }),
                None,
            )
        })
        .to_int();
    }
    fn FreePages(&self, address: u64, size: usize) {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        (&(0));
        (&(0));
        ({
            v8_base_OS::Free(
                (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
            )
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
        (&(0));
        (&(0));
        return ({
            v8_base_OS::SetPermissions(
                (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
                ({ ToMemoryPermission_252((*permissions.borrow())) }),
            )
        });
    }
    fn AllocateGuardRegion(&self, address: u64, size: usize) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        (&(0));
        (&(0));
        let hint: Value<AnyPtr> = Rc::new(RefCell::new(
            (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
        ));
        let result: Value<AnyPtr> = Rc::new(RefCell::new(
            ({
                v8_base_OS::Allocate(
                    (*hint.borrow()).clone(),
                    (*size.borrow()),
                    ({
                        v8_VirtualAddressSpaceImpl::allocation_granularity(
                            &((*self).to_strong() as Value<dyn v8_VirtualAddressSpace>)
                                .as_pointer_dyn(),
                        )
                    }),
                    v8_base_OS_MemoryPermission_kNoAccess,
                    None,
                )
            }),
        ));
        if (!(*result.borrow()).is_null())
            && ({
                let _lhs = (*result.borrow()).clone();
                _lhs != (*hint.borrow()).clone()
            })
        {
            ({ v8_base_OS::Free((*result.borrow()).clone(), (*size.borrow())) });
        }
        return {
            let _lhs = (*result.borrow()).clone();
            _lhs == (*hint.borrow()).clone()
        };
    }
    fn FreeGuardRegion(&self, address: u64, size: usize) {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        (&(0));
        (&(0));
        ({
            v8_base_OS::Free(
                (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
            )
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
        (&(0));
        (&(0));
        (&(0));
        return ({
            v8_base_OS::AllocateShared(
                (*hint.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
                ({ ToMemoryPermission_252((*permissions.borrow())) }),
                (*handle.borrow()).clone(),
                (*offset.borrow()),
            )
        })
        .to_int();
    }
    fn FreeSharedPages(&self, address: u64, size: usize) {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        (&(0));
        (&(0));
        ({
            v8_base_OS::FreeShared(
                (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
            )
        });
    }
    fn ActiveMemoryProtectionKey(&self) -> std_optional_int_ {
        return std_optional_int_::std_optional_int_3({
            (*nullopt_3.with(Value::clone).borrow()).clone()
        });
    }
    fn CanAllocateSubspaces(&self) -> bool {
        return ({ v8_base_OS::CanReserveAddressSpace() });
    }
    fn AllocateSubspace(
        &self,
        hint: u64,
        size: usize,
        alignment: usize,
        max_page_permissions: v8_PagePermissions,
        key: Option<std_optional_int_>,
        handle: Option<std_optional_v8_SharedMemoryHandle_>,
    ) -> Option<Value<v8_VirtualAddressSpace>> {
        let hint: Value<u64> = Rc::new(RefCell::new(hint));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let alignment: Value<usize> = Rc::new(RefCell::new(alignment));
        let max_page_permissions: Value<v8_PagePermissions> =
            Rc::new(RefCell::new(max_page_permissions));
        let key: Value<std_optional_int_> = Rc::new(RefCell::new(key.unwrap_or(
            std_optional_int_::std_optional_int_3({
                (*nullopt_3.with(Value::clone).borrow()).clone()
            }),
        )));
        let handle: Value<std_optional_v8_SharedMemoryHandle_> =
            Rc::new(RefCell::new(handle.unwrap_or(
                std_optional_v8_SharedMemoryHandle_::std_optional_v8_SharedMemoryHandle_1({
                    (*nullopt_3.with(Value::clone).borrow()).clone()
                }),
            )));
        (&(0));
        (&(0));
        (&(0));
        let reservation: Value<std_optional_v8_base_AddressSpaceReservation_> =
            Rc::new(RefCell::new(
                ({
                    v8_base_OS::CreateAddressSpaceReservation(
                        (*hint.borrow()).reinterpret_cast::<::libc::c_void>(),
                        (*size.borrow()),
                        (*alignment.borrow()),
                        ({ ToMemoryPermission_252((*max_page_permissions.borrow())) }),
                        Some((*handle.borrow()).clone()),
                    )
                }),
            ));
        if !({ (*reservation.borrow()).has_value() }) {
            return None;
        }
        return Ptr::alloc(
            v8_base_VirtualAddressSubspace::v8_base_VirtualAddressSubspace(
                { (*(*reservation.borrow()).upgrade().deref()).clone() },
                {
                    ((*self).to_strong() as Value<dyn v8_base_VirtualAddressSpaceBase>)
                        .as_pointer_dyn()
                },
                { (*max_page_permissions.borrow()) },
                { (*key.borrow()).clone() },
            ),
        )
        .to_owned_opt();
    }
    fn RecommitPages(&self, address: u64, size: usize, permissions: v8_PagePermissions) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let permissions: Value<v8_PagePermissions> = Rc::new(RefCell::new(permissions));
        (&(0));
        (&(0));
        return ({
            v8_base_OS::RecommitPages(
                (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
                ({ ToMemoryPermission_252((*permissions.borrow())) }),
            )
        });
    }
    fn DiscardSystemPages(&self, address: u64, size: usize) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        (&(0));
        (&(0));
        return ({
            v8_base_OS::DiscardSystemPages(
                (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
            )
        });
    }
    fn DecommitPages(&self, address: u64, size: usize) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        (&(0));
        (&(0));
        return ({
            v8_base_OS::DecommitPages(
                (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
            )
        });
    }
    fn SetName(&self, name: Ptr<Vec<u8>>) -> bool {
        return false;
    }
    fn FreeSubspace(&self, subspace: Ptr<v8_base_VirtualAddressSubspace>) {
        let subspace: Value<Ptr<v8_base_VirtualAddressSubspace>> = Rc::new(RefCell::new(subspace));
        ({
            v8_base_OS::FreeAddressSpaceReservation(
                (*(*(*subspace.borrow()).upgrade().deref())
                    .reservation_
                    .borrow())
                .clone(),
            )
        });
    }
}
impl Clone for v8_base_VirtualAddressSpace {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_VirtualAddressSpace> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_VirtualAddressSpace> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_VirtualAddressSpace {
    fn default() -> Self {
        { v8_base_VirtualAddressSpace::v8_base_VirtualAddressSpace() }
    }
}
impl ByteRepr for v8_base_VirtualAddressSpace {
    fn byte_size() -> usize {
        48
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_VirtualAddressSubspace {
    reservation_: Value<v8_base_AddressSpaceReservation>,
    mutex_: Value<v8_base_Mutex>,
    region_allocator_: Value<v8_base_RegionAllocator>,
    rng_: Value<v8_base_RandomNumberGenerator>,
    parent_space_: Value<PtrDyn<dyn v8_base_VirtualAddressSpaceBase>>,
    pkey_: Value<std_optional_int_>,
    name_: Value<Vec<u8>>,
}
impl v8_base_VirtualAddressSubspace {
    fn v8_base_VirtualAddressSubspace(
        reservation: v8_base_AddressSpaceReservation,
        parent_space: PtrDyn<dyn v8_base_VirtualAddressSpaceBase>,
        max_page_permissions: v8_PagePermissions,
        key: std_optional_int_,
    ) -> Self {
        let reservation: Value<v8_base_AddressSpaceReservation> =
            Rc::new(RefCell::new(reservation));
        let parent_space: Value<PtrDyn<dyn v8_base_VirtualAddressSpaceBase>> =
            Rc::new(RefCell::new(parent_space));
        let max_page_permissions: Value<v8_PagePermissions> =
            Rc::new(RefCell::new(max_page_permissions));
        let key: Value<std_optional_int_> = Rc::new(RefCell::new(key));
        let __this: Value<v8_base_VirtualAddressSubspace> = Rc::new(RefCell::new(Self {
            reservation_: Rc::new(RefCell::new((*reservation.borrow()).clone())),
            mutex_: Rc::new(RefCell::new(v8_base_Mutex::v8_base_Mutex())),
            region_allocator_: Rc::new(RefCell::new(
                v8_base_RegionAllocator::v8_base_RegionAllocator(
                    {
                        ({ v8_base_AddressSpaceReservationImpl::base(&reservation.as_pointer()) })
                            .to_int()
                    },
                    { ({ v8_base_AddressSpaceReservationImpl::size(&reservation.as_pointer()) }) },
                    {
                        ({
                            v8_VirtualAddressSpaceImpl::allocation_granularity(
                                &((*parent_space.borrow()).to_strong()
                                    as Value<dyn v8_VirtualAddressSpace>)
                                    .as_pointer_dyn(),
                            )
                        })
                    },
                ),
            )),
            rng_: Rc::new(RefCell::new(
                v8_base_RandomNumberGenerator::v8_base_RandomNumberGenerator2(),
            )),
            parent_space_: Rc::new(RefCell::new((*parent_space.borrow()).clone())),
            pkey_: Rc::new(RefCell::new((*key.borrow()).clone())),
            name_: Rc::new(RefCell::new(vec![0])),
        }));
        let this: Ptr<v8_base_VirtualAddressSubspace> = __this.as_pointer();
        if ({ (*(*this.upgrade().deref()).pkey_.borrow())() }) {
            ({
                V8_Fatal_253(
                    (*kUnreachableCodeMessage_221.with(Value::clone).borrow()).clone(),
                    &[],
                )
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl v8_base_VirtualAddressSpaceBase for v8_base_VirtualAddressSubspace {
    fn SetRandomSeed(&self, seed: i64) {
        let seed: Value<i64> = Rc::new(RefCell::new(seed));
        let guard: Value<v8_base_LockGuard_v8_base_Mutex_> = Rc::new(RefCell::new(
            v8_base_LockGuard_v8_base_Mutex_::v8_base_LockGuard_v8_base_Mutex_1({
                (self.mutex_.as_pointer())
            }),
        ));
        let _dtor_guard = ScopedDestructor::new(&guard, |__p| __p.destructor());
        ({ v8_base_RandomNumberGeneratorImpl::SetSeed(&self.rng_.as_pointer(), (*seed.borrow())) });
    }
    fn RandomPageAddress(&self) -> u64 {
        let guard: Value<v8_base_LockGuard_v8_base_Mutex_> = Rc::new(RefCell::new(
            v8_base_LockGuard_v8_base_Mutex_::v8_base_LockGuard_v8_base_Mutex_1({
                (self.mutex_.as_pointer())
            }),
        ));
        let _dtor_guard = ScopedDestructor::new(&guard, |__p| __p.destructor());
        let addr: Value<u64> = Rc::new(RefCell::new(
            ({
                v8_VirtualAddressSpaceImpl::base(
                    &((*self).to_strong() as Value<dyn v8_VirtualAddressSpace>).as_pointer_dyn(),
                )
            })
            .wrapping_add(
                ((({ v8_base_RandomNumberGeneratorImpl::NextInt64(&self.rng_.as_pointer()) })
                    as u64)
                    .wrapping_rem(
                        (({
                            v8_VirtualAddressSpaceImpl::size(
                                &((*self).to_strong() as Value<dyn v8_VirtualAddressSpace>)
                                    .as_pointer_dyn(),
                            )
                        }) as u64),
                    )),
            ),
        ));
        return ({
            RoundDown_233(
                (*addr.borrow()),
                (({
                    v8_VirtualAddressSpaceImpl::allocation_granularity(
                        &((*self).to_strong() as Value<dyn v8_VirtualAddressSpace>)
                            .as_pointer_dyn(),
                    )
                }) as i64),
            )
        });
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
        (&(0));
        (&(0));
        (&(0));
        (&(0));
        let guard: Value<v8_base_LockGuard_v8_base_Mutex_> = Rc::new(RefCell::new(
            v8_base_LockGuard_v8_base_Mutex_::v8_base_LockGuard_v8_base_Mutex_1({
                (self.mutex_.as_pointer())
            }),
        ));
        let _dtor_guard = ScopedDestructor::new(&guard, |__p| __p.destructor());
        let address: Value<u64> = Rc::new(RefCell::new(
            ({
                v8_base_RegionAllocatorImpl::AllocateRegion_u64_usize_usize(
                    &self.region_allocator_.as_pointer(),
                    (*hint.borrow()),
                    (*size.borrow()),
                    (*alignment.borrow()),
                )
            }),
        ));
        if ((*address.borrow()) == 18446744073709551615) {
            return 0;
        }
        if !({
            v8_base_AddressSpaceReservationImpl::Allocate(
                &self.reservation_.as_pointer(),
                (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
                ({ ToMemoryPermission_252((*permissions.borrow())) }),
            )
        }) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                let _cmp: Value<bool> = Rc::new(RefCell::new(
                    ({
                        CmpEQImpl_230(
                            ((*size.borrow()) as u64),
                            (({
                                v8_base_RegionAllocatorImpl::FreeRegion(
                                    &self.region_allocator_.as_pointer(),
                                    (*address.borrow()),
                                )
                            }) as u64),
                        )
                    }),
                ));
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if ((!(!(!(*_cmp.borrow()))) as i64) != 0) {
                        ({
                            V8_Fatal_253(
                                Ptr::from_string_literal(b"Check failed: %s."),
                                &[(Ptr::from_string_literal(
                                    b"size == region_allocator_.FreeRegion(address)",
                                ))
                                .into()],
                            )
                        });
                    }
                }
            }
            return 0;
        }
        return (*address.borrow());
    }
    fn FreePages(&self, address: u64, size: usize) {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        (&(0));
        (&(0));
        let guard: Value<v8_base_LockGuard_v8_base_Mutex_> = Rc::new(RefCell::new(
            v8_base_LockGuard_v8_base_Mutex_::v8_base_LockGuard_v8_base_Mutex_1({
                (self.mutex_.as_pointer())
            }),
        ));
        let _dtor_guard = ScopedDestructor::new(&guard, |__p| __p.destructor());
        if !({
            v8_base_AddressSpaceReservationImpl::Free(
                &self.reservation_.as_pointer(),
                (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
            )
        }) {
            ({
                FatalOOM_254(
                    v8_base_OOMType_kProcess,
                    Ptr::from_string_literal(b"VirtualAddressSubspace::FreePages"),
                )
            });
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            let _cmp: Value<bool> = Rc::new(RefCell::new(
                ({
                    CmpEQImpl_230(
                        ((*size.borrow()) as u64),
                        (({
                            v8_base_RegionAllocatorImpl::FreeRegion(
                                &self.region_allocator_.as_pointer(),
                                (*address.borrow()),
                            )
                        }) as u64),
                    )
                }),
            ));
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if ((!(!(!(*_cmp.borrow()))) as i64) != 0) {
                    ({
                        V8_Fatal_253(
                            Ptr::from_string_literal(b"Check failed: %s."),
                            &[(Ptr::from_string_literal(
                                b"size == region_allocator_.FreeRegion(address)",
                            ))
                            .into()],
                        )
                    });
                }
            }
        }
        if !((*self.name_.borrow()).len() <= 1) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if ((!(!(!({
                    v8_base_OS::SetMemoryRegionName(
                        (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                        (*size.borrow()),
                        (self.name_.as_pointer() as Ptr<u8>),
                    )
                }))) as i64)
                    != 0)
                {
                    ({
                        V8_Fatal_253 ( Ptr::from_string_literal(b"Check failed: %s." )  , & [ ( Ptr::from_string_literal(b"OS::SetMemoryRegionName(reinterpret_cast<void*>(address), size, name_.c_str())" )  ) .into() , ] )
                    });
                }
            }
        }
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
        (&(0));
        (&(0));
        (&(0));
        return ({
            v8_base_AddressSpaceReservationImpl::SetPermissions(
                &self.reservation_.as_pointer(),
                (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
                ({ ToMemoryPermission_252((*permissions.borrow())) }),
            )
        });
    }
    fn AllocateGuardRegion(&self, address: u64, size: usize) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        (&(0));
        (&(0));
        let guard: Value<v8_base_LockGuard_v8_base_Mutex_> = Rc::new(RefCell::new(
            v8_base_LockGuard_v8_base_Mutex_::v8_base_LockGuard_v8_base_Mutex_1({
                (self.mutex_.as_pointer())
            }),
        ));
        let _dtor_guard = ScopedDestructor::new(&guard, |__p| __p.destructor());
        return ({
            v8_base_RegionAllocatorImpl::AllocateRegionAt(
                &self.region_allocator_.as_pointer(),
                (*address.borrow()),
                (*size.borrow()),
                None,
            )
        });
    }
    fn FreeGuardRegion(&self, address: u64, size: usize) {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        (&(0));
        (&(0));
        let guard: Value<v8_base_LockGuard_v8_base_Mutex_> = Rc::new(RefCell::new(
            v8_base_LockGuard_v8_base_Mutex_::v8_base_LockGuard_v8_base_Mutex_1({
                (self.mutex_.as_pointer())
            }),
        ));
        let _dtor_guard = ScopedDestructor::new(&guard, |__p| __p.destructor());
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            let _cmp: Value<bool> = Rc::new(RefCell::new(
                ({
                    CmpEQImpl_230(
                        ((*size.borrow()) as u64),
                        (({
                            v8_base_RegionAllocatorImpl::FreeRegion(
                                &self.region_allocator_.as_pointer(),
                                (*address.borrow()),
                            )
                        }) as u64),
                    )
                }),
            ));
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if ((!(!(!(*_cmp.borrow()))) as i64) != 0) {
                    ({
                        V8_Fatal_253(
                            Ptr::from_string_literal(b"Check failed: %s."),
                            &[(Ptr::from_string_literal(
                                b"size == region_allocator_.FreeRegion(address)",
                            ))
                            .into()],
                        )
                    });
                }
            }
        }
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
        (&(0));
        (&(0));
        (&(0));
        let guard: Value<v8_base_LockGuard_v8_base_Mutex_> = Rc::new(RefCell::new(
            v8_base_LockGuard_v8_base_Mutex_::v8_base_LockGuard_v8_base_Mutex_1({
                (self.mutex_.as_pointer())
            }),
        ));
        let _dtor_guard = ScopedDestructor::new(&guard, |__p| __p.destructor());
        let address: Value<u64> = Rc::new(RefCell::new(
            ({
                let _alignment: usize = ({
                    v8_VirtualAddressSpaceImpl::allocation_granularity(
                        &((*self).to_strong() as Value<dyn v8_VirtualAddressSpace>)
                            .as_pointer_dyn(),
                    )
                });
                v8_base_RegionAllocatorImpl::AllocateRegion_u64_usize_usize(
                    &self.region_allocator_.as_pointer(),
                    (*hint.borrow()),
                    (*size.borrow()),
                    _alignment,
                )
            }),
        ));
        if ((*address.borrow()) == 18446744073709551615) {
            return 0;
        }
        if !({
            v8_base_AddressSpaceReservationImpl::AllocateShared(
                &self.reservation_.as_pointer(),
                (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
                ({ ToMemoryPermission_252((*permissions.borrow())) }),
                (*handle.borrow()).clone(),
                (*offset.borrow()),
            )
        }) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                let _cmp: Value<bool> = Rc::new(RefCell::new(
                    ({
                        CmpEQImpl_230(
                            ((*size.borrow()) as u64),
                            (({
                                v8_base_RegionAllocatorImpl::FreeRegion(
                                    &self.region_allocator_.as_pointer(),
                                    (*address.borrow()),
                                )
                            }) as u64),
                        )
                    }),
                ));
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if ((!(!(!(*_cmp.borrow()))) as i64) != 0) {
                        ({
                            V8_Fatal_253(
                                Ptr::from_string_literal(b"Check failed: %s."),
                                &[(Ptr::from_string_literal(
                                    b"size == region_allocator_.FreeRegion(address)",
                                ))
                                .into()],
                            )
                        });
                    }
                }
            }
            return 0;
        }
        return (*address.borrow());
    }
    fn FreeSharedPages(&self, address: u64, size: usize) {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        (&(0));
        (&(0));
        let guard: Value<v8_base_LockGuard_v8_base_Mutex_> = Rc::new(RefCell::new(
            v8_base_LockGuard_v8_base_Mutex_::v8_base_LockGuard_v8_base_Mutex_1({
                (self.mutex_.as_pointer())
            }),
        ));
        let _dtor_guard = ScopedDestructor::new(&guard, |__p| __p.destructor());
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if ((!(!(!({
                v8_base_AddressSpaceReservationImpl::FreeShared(
                    &self.reservation_.as_pointer(),
                    (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                    (*size.borrow()),
                )
            }))) as i64)
                != 0)
            {
                ({
                    V8_Fatal_253(
                        Ptr::from_string_literal(b"Check failed: %s."),
                        &[(Ptr::from_string_literal(
                            b"reservation_.FreeShared(reinterpret_cast<void*>(address), size)",
                        ))
                        .into()],
                    )
                });
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            let _cmp: Value<bool> = Rc::new(RefCell::new(
                ({
                    CmpEQImpl_230(
                        ((*size.borrow()) as u64),
                        (({
                            v8_base_RegionAllocatorImpl::FreeRegion(
                                &self.region_allocator_.as_pointer(),
                                (*address.borrow()),
                            )
                        }) as u64),
                    )
                }),
            ));
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if ((!(!(!(*_cmp.borrow()))) as i64) != 0) {
                    ({
                        V8_Fatal_253(
                            Ptr::from_string_literal(b"Check failed: %s."),
                            &[(Ptr::from_string_literal(
                                b"size == region_allocator_.FreeRegion(address)",
                            ))
                            .into()],
                        )
                    });
                }
            }
        }
        if !((*self.name_.borrow()).len() <= 1) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if ((!(!(!({
                    v8_base_OS::SetMemoryRegionName(
                        (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                        (*size.borrow()),
                        (self.name_.as_pointer() as Ptr<u8>),
                    )
                }))) as i64)
                    != 0)
                {
                    ({
                        V8_Fatal_253 ( Ptr::from_string_literal(b"Check failed: %s." )  , & [ ( Ptr::from_string_literal(b"OS::SetMemoryRegionName(reinterpret_cast<void*>(address), size, name_.c_str())" )  ) .into() , ] )
                    });
                }
            }
        }
    }
    fn ActiveMemoryProtectionKey(&self) -> std_optional_int_ {
        return (*self.pkey_.borrow()).clone();
    }
    fn CanAllocateSubspaces(&self) -> bool {
        return true;
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
        (&(0));
        (&(0));
        (&(0));
        (&(0));
        (&(0));
        let guard: Value<v8_base_LockGuard_v8_base_Mutex_> = Rc::new(RefCell::new(
            v8_base_LockGuard_v8_base_Mutex_::v8_base_LockGuard_v8_base_Mutex_1({
                (self.mutex_.as_pointer())
            }),
        ));
        let _dtor_guard = ScopedDestructor::new(&guard, |__p| __p.destructor());
        let address: Value<u64> = Rc::new(RefCell::new(
            ({
                v8_base_RegionAllocatorImpl::AllocateRegion_u64_usize_usize(
                    &self.region_allocator_.as_pointer(),
                    (*hint.borrow()),
                    (*size.borrow()),
                    (*alignment.borrow()),
                )
            }),
        ));
        if ((*address.borrow()) == 18446744073709551615) {
            return None;
        }
        let reservation: Value<std_optional_v8_base_AddressSpaceReservation_> =
            Rc::new(RefCell::new(
                ({
                    v8_base_AddressSpaceReservationImpl::CreateSubReservation(
                        &self.reservation_.as_pointer(),
                        (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                        (*size.borrow()),
                        ({ ToMemoryPermission_252((*max_page_permissions.borrow())) }),
                    )
                }),
            ));
        if !({ (*reservation.borrow()).has_value() }) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                let _cmp: Value<bool> = Rc::new(RefCell::new(
                    ({
                        CmpEQImpl_230(
                            ((*size.borrow()) as u64),
                            (({
                                v8_base_RegionAllocatorImpl::FreeRegion(
                                    &self.region_allocator_.as_pointer(),
                                    (*address.borrow()),
                                )
                            }) as u64),
                        )
                    }),
                ));
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if ((!(!(!(*_cmp.borrow()))) as i64) != 0) {
                        ({
                            V8_Fatal_253(
                                Ptr::from_string_literal(b"Check failed: %s."),
                                &[(Ptr::from_string_literal(
                                    b"size == region_allocator_.FreeRegion(address)",
                                ))
                                .into()],
                            )
                        });
                    }
                }
            }
            return std_unique_ptr_v8_VirtualAddressSpace__std_default_delete_v8_VirtualAddressSpace__ :: std_unique_ptr_v8_VirtualAddressSpace__std_default_delete_v8_VirtualAddressSpace__1 ( {  Default::default()   } , )   ;
        }
        return Ptr::alloc(
            v8_base_VirtualAddressSubspace::v8_base_VirtualAddressSubspace(
                { (*(*reservation.borrow()).upgrade().deref()).clone() },
                {
                    ((*self).to_strong() as Value<dyn v8_base_VirtualAddressSpaceBase>)
                        .as_pointer_dyn()
                },
                { (*max_page_permissions.borrow()) },
                { (*key.borrow()).clone() },
            ),
        )
        .to_owned_opt();
    }
    fn RecommitPages(&self, address: u64, size: usize, permissions: v8_PagePermissions) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let permissions: Value<v8_PagePermissions> = Rc::new(RefCell::new(permissions));
        (&(0));
        (&(0));
        (&(0));
        return ({
            v8_base_AddressSpaceReservationImpl::RecommitPages(
                &self.reservation_.as_pointer(),
                (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
                ({ ToMemoryPermission_252((*permissions.borrow())) }),
            )
        });
    }
    fn DiscardSystemPages(&self, address: u64, size: usize) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        (&(0));
        (&(0));
        return ({
            v8_base_AddressSpaceReservationImpl::DiscardSystemPages(
                &self.reservation_.as_pointer(),
                (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                (*size.borrow()),
            )
        });
    }
    fn DecommitPages(&self, address: u64, size: usize) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        (&(0));
        (&(0));
        let success: Value<bool> = Rc::new(RefCell::new(
            ({
                v8_base_AddressSpaceReservationImpl::DecommitPages(
                    &self.reservation_.as_pointer(),
                    (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                    (*size.borrow()),
                )
            }),
        ));
        if (*success.borrow()) && (!((*self.name_.borrow()).len() <= 1)) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if ((!(!(!({
                    v8_base_OS::SetMemoryRegionName(
                        (*address.borrow()).reinterpret_cast::<::libc::c_void>(),
                        (*size.borrow()),
                        (self.name_.as_pointer() as Ptr<u8>),
                    )
                }))) as i64)
                    != 0)
                {
                    ({
                        V8_Fatal_253 ( Ptr::from_string_literal(b"Check failed: %s." )  , & [ ( Ptr::from_string_literal(b"OS::SetMemoryRegionName(reinterpret_cast<void*>(address), size, name_.c_str())" )  ) .into() , ] )
                    });
                }
            }
        }
        return (*success.borrow());
    }
    fn SetName(&self, name: Ptr<Vec<u8>>) -> bool {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if ((!(!(!({ IsValidMappingName_255((name).clone()) }))) as i64) != 0) {
                ({
                    V8_Fatal_253(
                        Ptr::from_string_literal(b"Check failed: %s."),
                        &[(Ptr::from_string_literal(b"IsValidMappingName(name)")).into()],
                    )
                });
            }
        }
        if ({
            v8_base_AddressSpaceReservationImpl::SetName(
                &self.reservation_.as_pointer(),
                (name.to_strong().as_pointer() as Ptr<u8>),
            )
        }) {
            (self.name_.as_pointer() as Ptr<Vec<u8>>).write((*name.upgrade().deref()).clone());
            return true;
        }
        return false;
    }
    fn FreeSubspace(&self, subspace: Ptr<v8_base_VirtualAddressSubspace>) {
        let subspace: Value<Ptr<v8_base_VirtualAddressSubspace>> = Rc::new(RefCell::new(subspace));
        let guard: Value<v8_base_LockGuard_v8_base_Mutex_> = Rc::new(RefCell::new(
            v8_base_LockGuard_v8_base_Mutex_::v8_base_LockGuard_v8_base_Mutex_1({
                (self.mutex_.as_pointer())
            }),
        ));
        let _dtor_guard = ScopedDestructor::new(&guard, |__p| __p.destructor());
        let reservation: Value<v8_base_AddressSpaceReservation> = Rc::new(RefCell::new(
            (*(*(*subspace.borrow()).upgrade().deref())
                .reservation_
                .borrow())
            .clone(),
        ));
        let base: Value<u64> = Rc::new(RefCell::new(
            ({ v8_base_AddressSpaceReservationImpl::base(&reservation.as_pointer()) }).to_int(),
        ));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            let _cmp: Value<bool> = Rc::new(RefCell::new(
                ({
                    CmpEQImpl_230(
                        (({ v8_base_AddressSpaceReservationImpl::size(&reservation.as_pointer()) })
                            as u64),
                        (({
                            v8_base_RegionAllocatorImpl::FreeRegion(
                                &self.region_allocator_.as_pointer(),
                                (*base.borrow()),
                            )
                        }) as u64),
                    )
                }),
            ));
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if ((!(!(!(*_cmp.borrow()))) as i64) != 0) {
                    ({
                        V8_Fatal_253(
                            Ptr::from_string_literal(b"Check failed: %s."),
                            &[(Ptr::from_string_literal(
                                b"reservation.size() == region_allocator_.FreeRegion(base)",
                            ))
                            .into()],
                        )
                    });
                }
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if ((!(!(!({
                (*self.reservation_.borrow()).FreeSubReservation((*reservation.borrow()).clone())
            }))) as i64)
                != 0)
            {
                ({
                    V8_Fatal_253(
                        Ptr::from_string_literal(b"Check failed: %s."),
                        &[(Ptr::from_string_literal(
                            b"reservation_.FreeSubReservation(reservation)",
                        ))
                        .into()],
                    )
                });
            }
        }
    }
}
impl ByteRepr for v8_base_VirtualAddressSubspace {
    fn byte_size() -> usize {
        304
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.reservation_.borrow()).to_bytes(&mut buf[48..64]);
        (*self.mutex_.borrow()).to_bytes(&mut buf[64..72]);
        (*self.region_allocator_.borrow()).to_bytes(&mut buf[72..240]);
        (*self.rng_.borrow()).to_bytes(&mut buf[240..264]);
        (*self.parent_space_.borrow()).to_bytes(&mut buf[264..272]);
        (*self.pkey_.borrow()).to_bytes(&mut buf[272..280]);
        (*self.name_.borrow()).to_bytes(&mut buf[280..304]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            reservation_: Rc::new(RefCell::new(<v8_base_AddressSpaceReservation>::from_bytes(
                &buf[48..64],
            ))),
            mutex_: Rc::new(RefCell::new(<v8_base_Mutex>::from_bytes(&buf[64..72]))),
            region_allocator_: Rc::new(RefCell::new(<v8_base_RegionAllocator>::from_bytes(
                &buf[72..240],
            ))),
            rng_: Rc::new(RefCell::new(<v8_base_RandomNumberGenerator>::from_bytes(
                &buf[240..264],
            ))),
            parent_space_: Rc::new(RefCell::new(
                <PtrDyn<dyn v8_base_VirtualAddressSpaceBase>>::from_bytes(&buf[264..272]),
            )),
            pkey_: Rc::new(RefCell::new(<std_optional_int_>::from_bytes(
                &buf[272..280],
            ))),
            name_: Rc::new(RefCell::new(<Vec<u8>>::from_bytes(&buf[280..304]))),
        }
    }
}
pub fn CountLeadingZeros_256(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        if (32_u32 == 64_u32) {
            (({ __builtin_clzll_257(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((32_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros_258(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        if (64_u32 == 64_u32) {
            (({ __builtin_clzll_257(((*value.borrow()) as u64)) }) as u32)
        } else {
            (((*value.borrow()) as u32).leading_zeros() as i32 as u32)
                .wrapping_sub((((32_u32).wrapping_sub((64_u32 as u32))) as u32))
        }
    };
}
pub fn CountLeadingZeros32_259(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_256((*value.borrow())) });
}
pub fn CountLeadingZeros64_260(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountLeadingZeros_258((*value.borrow())) });
}
pub fn CountTrailingZeros_261(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32_u32
    } else {
        (if (32_u32 == 64_u32) {
            ({ __builtin_ctzll_262(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros_263(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64_u32
    } else {
        (if (64_u32 == 64_u32) {
            ({ __builtin_ctzll_262(((*value.borrow()) as u64)) })
        } else {
            ((*value.borrow()) as u32).trailing_zeros() as i32
        } as u32)
    };
}
pub fn CountTrailingZeros32_264(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_261((*value.borrow())) });
}
pub fn CountTrailingZeros64_265(value: u64) -> u32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return ({ CountTrailingZeros_263((*value.borrow())) });
}
pub fn RoundUpToPowerOfTwo32_266(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u32 << ((32_u32).wrapping_sub(({ CountLeadingZeros_256((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo64_267(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    (&(0));
    if ((*value.borrow()) != 0) {
        (*value.borrow_mut()).prefix_dec();
    }
    return (1_u64 << ((64_u32).wrapping_sub(({ CountLeadingZeros_258((*value.borrow())) }))));
}
pub fn RoundUpToPowerOfTwo_268(value: usize) -> usize {
    let value: Value<usize> = Rc::new(RefCell::new(value));
    if (::std::mem::size_of::<usize>() == ::std::mem::size_of::<u64>()) {
        return (({ RoundUpToPowerOfTwo64_267(((*value.borrow()) as u64)) }) as usize);
    } else {
        return (({ RoundUpToPowerOfTwo32_266(((*value.borrow()) as u32)) }) as usize);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn RoundDownToPowerOfTwo32_269(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    if ((*value.borrow()) > 2147483648_u32) {
        return 2147483648_u32;
    }
    let result: Value<u32> = Rc::new(RefCell::new(
        ({ RoundUpToPowerOfTwo32_266((*value.borrow())) }),
    ));
    if ((*result.borrow()) > (*value.borrow())) {
        (*result.borrow_mut()) >>= 1;
    }
    return (*result.borrow());
}
pub fn RotateRight32_270(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateLeft32_271(value: u32, shift: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let shift: Value<u32> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((32_u32).wrapping_sub((*shift.borrow()))) & 31_u32)));
}
pub fn RotateRight64_272(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) >> (*shift.borrow()))
        | ((*value.borrow()) << (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn RotateLeft64_273(value: u64, shift: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    return (((*value.borrow()) << (*shift.borrow()))
        | ((*value.borrow()) >> (((64_u64).wrapping_sub((*shift.borrow()))) & 63_u64)));
}
pub fn ClearLsb_274(value: i32) -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(value));
    return ((*value.borrow()) & ((*value.borrow()) - 1));
}
pub fn SignedAddOverflow32_275(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_sadd_overflow_276(_arg0, _arg1, _arg2)
    });
}
pub fn SignedSubOverflow32_277(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_ssub_overflow_278(_arg0, _arg1, _arg2)
    });
}
pub fn SignedMulOverflow32_279(lhs: i32, rhs: i32, val: Ptr<i32>) -> bool {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: i32 = (*lhs.borrow());
        let _arg1: i32 = (*rhs.borrow());
        let _arg2: Ptr<i32> = (*val.borrow()).clone();
        __builtin_smul_overflow_280(_arg0, _arg1, _arg2)
    });
}
pub fn SignedAddOverflow64_281(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_add_overflow_282(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedSubOverflow64_283(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return ({
        __builtin_sub_overflow_284(&[
            (*lhs.borrow()).into(),
            (*rhs.borrow()).into(),
            ((*val.borrow()).clone()).into(),
        ])
    });
}
pub fn SignedMulOverflow64_285(lhs: i64, rhs: i64, val: Ptr<i64>) -> bool {
    let lhs: Value<i64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i64> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<i64>> = Rc::new(RefCell::new(val));
    return {
        let (val, ovf) = (*lhs.borrow()).overflowing_mul((*rhs.borrow()));
        (*val.borrow()).write(val);
        ovf
    };
}
pub fn UnsignedAddOverflow32_286(lhs: u32, rhs: u32, val: Ptr<u32>) -> bool {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    let val: Value<Ptr<u32>> = Rc::new(RefCell::new(val));
    return ({
        let _arg0: u32 = (*lhs.borrow());
        let _arg1: u32 = (*rhs.borrow());
        let _arg2: Ptr<u32> = (*val.borrow()).clone();
        __builtin_uadd_overflow_287(_arg0, _arg1, _arg2)
    });
}
pub fn UnsignedDiv32_288(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedDiv64_289(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_div((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn UnsignedMod32_290(lhs: u32, rhs: u32) -> u32 {
    let lhs: Value<u32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u32> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u32
    };
}
pub fn UnsignedMod64_291(lhs: u64, rhs: u64) -> u64 {
    let lhs: Value<u64> = Rc::new(RefCell::new(lhs));
    let rhs: Value<u64> = Rc::new(RefCell::new(rhs));
    return if ((*rhs.borrow()) != 0) {
        (*lhs.borrow()).wrapping_rem((*rhs.borrow()))
    } else {
        0_u64
    };
}
pub fn WraparoundAdd32_292(lhs: i32, rhs: i32) -> i32 {
    let lhs: Value<i32> = Rc::new(RefCell::new(lhs));
    let rhs: Value<i32> = Rc::new(RefCell::new(rhs));
    return ((((*lhs.borrow()) as u32).wrapping_add(((*rhs.borrow()) as u32))) as i32);
}
pub fn WraparoundNeg32_293(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (-((*x.borrow()) as u32) as i32);
}
pub fn ByteReverse16_294(value: u16) -> u16 {
    let value: Value<u16> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse32_295(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ByteReverse64_296(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return (*value.borrow()).swap_bytes();
}
pub fn ToMemoryPermission_252(permissions: v8_PagePermissions) -> v8_base_OS_MemoryPermission {
    let permissions: Value<v8_PagePermissions> = Rc::new(RefCell::new(permissions));
    switch!(match (*permissions.borrow()) {
        __v if __v == 0 => {
            return v8_base_OS_MemoryPermission_kNoAccess;
        }
        __v if __v == 1 => {
            return v8_base_OS_MemoryPermission_kRead;
        }
        __v if __v == 3 => {
            return v8_base_OS_MemoryPermission_kReadWrite;
        }
        __v if __v == 7 => {
            return v8_base_OS_MemoryPermission_kReadWriteExecute;
        }
        __v if __v == 5 => {
            return v8_base_OS_MemoryPermission_kReadExecute;
        }
        _ => {
            ({
                V8_Fatal_253(
                    (*kUnreachableCodeMessage_221.with(Value::clone).borrow()).clone(),
                    &[],
                )
            });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn IsValidMappingName_255(name: Ptr<Vec<u8>>) -> bool {
    return ({
        let ___first: Ptr<u8> = ({ (*name.upgrade().deref()).begin_const() });
        let ___last: Ptr<u8> = ({ (*name.upgrade().deref()).end_const() });
        all_of_297(
            ___first,
            ___last,
            (|c: u8| {
                let c: Value<u8> = Rc::new(RefCell::new(c));
                return (({ isalnum_298(((*c.borrow()) as i32)) }) != 0)
                    || (((*c.borrow()) as i32) == (('-' as u8) as i32));
            }),
        )
    });
}
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_base_Thread_PlatformData;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_Isolate;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct absl_SynchWaitParams;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct absl_SynchLocksHeld;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_PageAllocator_AllocationHint;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct absl_base_internal_SpinLock;
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct absl_time_internal_cctz_time_zone_Impl;
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
pub trait v8_base_AddressRegionImpl {
    fn begin(&self) -> u64;
    fn end(&self) -> u64;
    fn size(&self) -> usize;
    fn set_size(&self, size: usize);
    fn is_empty(&self) -> bool;
    fn contains_u64_const(&self, address: u64) -> bool;
    fn contains_u64_usize_const(&self, address: u64, size: usize) -> bool;
    fn contains_v8_base_AddressRegion_const(&self, region: v8_base_AddressRegion) -> bool;
    fn GetOverlap(&self, region: v8_base_AddressRegion) -> v8_base_AddressRegion;
    fn operator_eq(&self, other: v8_base_AddressRegion) -> bool;
}
impl v8_base_AddressRegionImpl for Ptr<v8_base_AddressRegion> {
    fn begin(&self) -> u64 {
        return (*(*(*self).upgrade().deref()).address_.borrow());
    }
    fn end(&self) -> u64 {
        return (*(*(*self).upgrade().deref()).address_.borrow())
            .wrapping_add(((*(*(*self).upgrade().deref()).size_.borrow()) as u64));
    }
    fn size(&self) -> usize {
        return (*(*(*self).upgrade().deref()).size_.borrow());
    }
    fn set_size(&self, size: usize) {
        let size: Value<usize> = Rc::new(RefCell::new(size));
        (*(*(*self).upgrade().deref()).size_.borrow_mut()) = (*size.borrow());
    }
    fn is_empty(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).size_.borrow()) == 0_usize);
    }
    fn contains_u64_const(&self, address: u64) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        return ((((*address.borrow()).wrapping_sub(({ v8_base_AddressRegionImpl::begin(self) })))
            as usize)
            < ({ v8_base_AddressRegionImpl::size(self) }));
    }
    fn contains_u64_usize_const(&self, address: u64, size: usize) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        let offset: Value<u64> = Rc::new(RefCell::new(
            (*address.borrow()).wrapping_sub(({ v8_base_AddressRegionImpl::begin(self) })),
        ));
        return (((*offset.borrow()) as usize) < (*(*(*self).upgrade().deref()).size_.borrow()))
            && (((*offset.borrow()).wrapping_add(((*size.borrow()) as u64)) as usize)
                <= (*(*(*self).upgrade().deref()).size_.borrow()));
    }
    fn contains_v8_base_AddressRegion_const(&self, region: v8_base_AddressRegion) -> bool {
        let region: Value<v8_base_AddressRegion> = Rc::new(RefCell::new(region));
        return ({
            let _address: u64 = (*(*region.borrow()).address_.borrow());
            let _size: usize = (*(*region.borrow()).size_.borrow());
            v8_base_AddressRegionImpl::contains_u64_usize_const(self, _address, _size)
        });
    }
    fn GetOverlap(&self, region: v8_base_AddressRegion) -> v8_base_AddressRegion {
        let region: Value<v8_base_AddressRegion> = Rc::new(RefCell::new(region));
        let overlap_start: Value<u64> = Rc::new(RefCell::new({
            let __tmp_0: Value<u64> =
                Rc::new(RefCell::new(({ v8_base_AddressRegionImpl::begin(self) })));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                ({ v8_base_AddressRegionImpl::begin(&region.as_pointer()) }),
            ));
            (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        }));
        let overlap_end: Value<u64> = Rc::new(RefCell::new(
            (if overlap_start.as_pointer().read()
                >= if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                    __tmp_0.as_pointer()
                } else {
                    __tmp_1.as_pointer()
                }
                .to_strong()
                .as_pointer()
                .read()
            {
                overlap_start.as_pointer()
            } else {
                if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                    __tmp_0.as_pointer()
                } else {
                    __tmp_1.as_pointer()
                }
                .to_strong()
                .as_pointer()
            }
            .read()),
        ));
        return v8_base_AddressRegion::v8_base_AddressRegion1({ (*overlap_start.borrow()) }, {
            ((*overlap_end.borrow()).wrapping_sub((*overlap_start.borrow())) as usize)
        });
    }
    fn operator_eq(&self, other: v8_base_AddressRegion) -> bool {
        let other: Value<v8_base_AddressRegion> = Rc::new(RefCell::new(other));
        return ((*(*(*self).upgrade().deref()).address_.borrow())
            == (*(*other.borrow()).address_.borrow()))
            && ((*(*(*self).upgrade().deref()).size_.borrow())
                == (*(*other.borrow()).size_.borrow()));
    }
}
pub trait v8_base_AddressRegion_StartAddressLessImpl {
    fn operator_call(&self, a: v8_base_AddressRegion, b: v8_base_AddressRegion) -> bool;
}
impl v8_base_AddressRegion_StartAddressLessImpl for Ptr<v8_base_AddressRegion_StartAddressLess> {
    fn operator_call(&self, a: v8_base_AddressRegion, b: v8_base_AddressRegion) -> bool {
        let a: Value<v8_base_AddressRegion> = Rc::new(RefCell::new(a));
        let b: Value<v8_base_AddressRegion> = Rc::new(RefCell::new(b));
        return (({ v8_base_AddressRegionImpl::begin(&a.as_pointer()) })
            < ({ v8_base_AddressRegionImpl::begin(&b.as_pointer()) }));
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
pub trait v8_base_RandomNumberGeneratorImpl {
    fn NextInt(&self) -> i32;
    fn NextBool(&self) -> bool;
    fn initial_seed(&self) -> i64;
    fn operator_call(&self) -> u32;
}
impl v8_base_RandomNumberGeneratorImpl for Ptr<v8_base_RandomNumberGenerator> {
    fn NextInt(&self) -> i32 {
        return ({ v8_base_RandomNumberGeneratorImpl::Next(self, 32) });
    }
    fn NextBool(&self) -> bool {
        return (({ v8_base_RandomNumberGeneratorImpl::Next(self, 1) }) != 0);
    }
    fn initial_seed(&self) -> i64 {
        return (*(*(*self).upgrade().deref()).initial_seed_.borrow());
    }
    fn operator_call(&self) -> u32 {
        return (({ v8_base_RandomNumberGeneratorImpl::NextInt(self) }) as u32);
    }
}
pub trait v8_base_RecursiveMutexImpl {
    fn AssertHeld(&self);
}
impl v8_base_RecursiveMutexImpl for Ptr<v8_base_RecursiveMutex> {
    fn AssertHeld(&self) {
        (&(0));
    }
}
pub trait v8_base_RegionAllocatorImpl {
    fn set_on_split_callback(&self, callback: std_function_void__unsigned_long__unsigned_long__);
    fn set_on_merge_callback(&self, callback: std_function_void__unsigned_long__unsigned_long__);
    fn FreeRegion(&self, address: u64) -> usize;
    fn begin(&self) -> u64;
    fn end(&self) -> u64;
    fn size(&self) -> usize;
    fn contains_u64_const(&self, address: u64) -> bool;
    fn contains_u64_usize_const(&self, address: u64, size: usize) -> bool;
    fn free_size(&self) -> usize;
    fn page_size(&self) -> usize;
}
impl v8_base_RegionAllocatorImpl for Ptr<v8_base_RegionAllocator> {
    fn set_on_split_callback(&self, callback: std_function_void__unsigned_long__unsigned_long__) {
        let callback: Value<std_function_void__unsigned_long__unsigned_long__> =
            Rc::new(RefCell::new(callback));
        (*(*(*self).upgrade().deref()).on_split_.borrow_mut()) = (*callback.borrow()).clone();
    }
    fn set_on_merge_callback(&self, callback: std_function_void__unsigned_long__unsigned_long__) {
        let callback: Value<std_function_void__unsigned_long__unsigned_long__> =
            Rc::new(RefCell::new(callback));
        (*(*(*self).upgrade().deref()).on_merge_.borrow_mut()) = (*callback.borrow()).clone();
    }
    fn FreeRegion(&self, address: u64) -> usize {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        return ({ v8_base_RegionAllocatorImpl::TrimRegion(self, (*address.borrow()), 0_usize) });
    }
    fn begin(&self) -> u64 {
        return ({
            v8_base_AddressRegionImpl::begin(
                &(*(*self).upgrade().deref()).whole_region_.as_pointer(),
            )
        });
    }
    fn end(&self) -> u64 {
        return ({
            v8_base_AddressRegionImpl::end(&(*(*self).upgrade().deref()).whole_region_.as_pointer())
        });
    }
    fn size(&self) -> usize {
        return ({
            v8_base_AddressRegionImpl::size(
                &(*(*self).upgrade().deref()).whole_region_.as_pointer(),
            )
        });
    }
    fn contains_u64_const(&self, address: u64) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        return ({
            v8_base_AddressRegionImpl::contains_u64_const(
                &(*(*self).upgrade().deref()).whole_region_.as_pointer(),
                (*address.borrow()),
            )
        });
    }
    fn contains_u64_usize_const(&self, address: u64, size: usize) -> bool {
        let address: Value<u64> = Rc::new(RefCell::new(address));
        let size: Value<usize> = Rc::new(RefCell::new(size));
        return ({
            v8_base_AddressRegionImpl::contains_u64_usize_const(
                &(*(*self).upgrade().deref()).whole_region_.as_pointer(),
                (*address.borrow()),
                (*size.borrow()),
            )
        });
    }
    fn free_size(&self) -> usize {
        return (*(*(*self).upgrade().deref()).free_size_.borrow());
    }
    fn page_size(&self) -> usize {
        return (*(*(*self).upgrade().deref()).page_size_.borrow());
    }
}
pub trait v8_base_RegionAllocator_AddressEndOrderImpl {
    fn operator_call(
        &self,
        a: Ptr<v8_base_RegionAllocator_Region>,
        b: Ptr<v8_base_RegionAllocator_Region>,
    ) -> bool;
}
impl v8_base_RegionAllocator_AddressEndOrderImpl for Ptr<v8_base_RegionAllocator_AddressEndOrder> {
    fn operator_call(
        &self,
        a: Ptr<v8_base_RegionAllocator_Region>,
        b: Ptr<v8_base_RegionAllocator_Region>,
    ) -> bool {
        let a: Value<Ptr<v8_base_RegionAllocator_Region>> = Rc::new(RefCell::new(a));
        let b: Value<Ptr<v8_base_RegionAllocator_Region>> = Rc::new(RefCell::new(b));
        return {
            let _lhs = ({ v8_base_AddressRegionImpl::end(&(*a.borrow())) });
            _lhs < ({ v8_base_AddressRegionImpl::end(&(*b.borrow())) })
        };
    }
}
pub trait v8_base_RegionAllocator_RegionImpl {
    fn is_free(&self) -> bool;
    fn is_allocated(&self) -> bool;
    fn is_excluded(&self) -> bool;
    fn state(&self) -> v8_base_RegionAllocator_RegionState;
    fn set_state(&self, state: v8_base_RegionAllocator_RegionState);
}
impl v8_base_RegionAllocator_RegionImpl for Ptr<v8_base_RegionAllocator_Region> {
    fn is_free(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).state_.borrow())
            == v8_base_RegionAllocator_RegionState_kFree);
    }
    fn is_allocated(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).state_.borrow())
            == v8_base_RegionAllocator_RegionState_kAllocated);
    }
    fn is_excluded(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).state_.borrow())
            == v8_base_RegionAllocator_RegionState_kExcluded);
    }
    fn state(&self) -> v8_base_RegionAllocator_RegionState {
        return (*(*(*self).upgrade().deref()).state_.borrow());
    }
    fn set_state(&self, state: v8_base_RegionAllocator_RegionState) {
        let state: Value<v8_base_RegionAllocator_RegionState> = Rc::new(RefCell::new(state));
        (*(*(*self).upgrade().deref()).state_.borrow_mut()) = (*state.borrow());
    }
}
pub trait v8_base_RegionAllocator_SizeAddressOrderImpl {
    fn operator_call(
        &self,
        a: Ptr<v8_base_RegionAllocator_Region>,
        b: Ptr<v8_base_RegionAllocator_Region>,
    ) -> bool;
}
impl v8_base_RegionAllocator_SizeAddressOrderImpl
    for Ptr<v8_base_RegionAllocator_SizeAddressOrder>
{
    fn operator_call(
        &self,
        a: Ptr<v8_base_RegionAllocator_Region>,
        b: Ptr<v8_base_RegionAllocator_Region>,
    ) -> bool {
        let a: Value<Ptr<v8_base_RegionAllocator_Region>> = Rc::new(RefCell::new(a));
        let b: Value<Ptr<v8_base_RegionAllocator_Region>> = Rc::new(RefCell::new(b));
        if {
            let _lhs = ({ v8_base_AddressRegionImpl::size(&(*a.borrow())) });
            _lhs != ({ v8_base_AddressRegionImpl::size(&(*b.borrow())) })
        } {
            return {
                let _lhs = ({ v8_base_AddressRegionImpl::size(&(*a.borrow())) });
                _lhs < ({ v8_base_AddressRegionImpl::size(&(*b.borrow())) })
            };
        }
        return {
            let _lhs = ({ v8_base_AddressRegionImpl::begin(&(*a.borrow())) });
            _lhs < ({ v8_base_AddressRegionImpl::begin(&(*b.borrow())) })
        };
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
