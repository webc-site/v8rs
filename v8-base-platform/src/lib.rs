//! v8-base-platform：V8 `src/base/platform` 的 Rust 转写。
//!
//! 依赖策略：自含于 std / libc / parking_lot / coarsetime / memmap2；
//! 已回收两处 `v8-base` 依赖（`lazy_instance` 懒实例、`page_allocator` 进程级
//! RNG——方向与原版 platform → lazy-instance / random-number-generator 的 include 一致）。
//! 其余待回收点在 readme 中登记。

#![cfg_attr(docsrs, feature(doc_cfg))]

mod allocation;
mod condition_variable;
mod elapsed_timer;
mod error;
mod memory_mapped_file;
mod memory_protection_key;
mod monotonic;
mod mutex;
mod os;
mod reservation;
mod semaphore;
mod stack;
mod thread;
mod thread_id;
mod time;
mod yield_processor;

pub use allocation::{
  AllocationResult, aligned_alloc, aligned_free, allocate_at_least, calloc, free, malloc,
  malloc_usable_size, realloc,
};
pub use condition_variable::{ConditionVariable, LazyConditionVariable};
pub use elapsed_timer::{ElapsedTimer, ScopedTimer};
pub use error::{Error, Result};
pub use memory_mapped_file::{FileMode, MemoryMappedFile, MemoryMappedFileHandle};
pub use memory_protection_key::{
  K_DEFAULT_PROTECTION_KEY, K_NO_MEMORY_PROTECTION_KEY, MemoryProtectionKey, Permission,
};
pub use mutex::{
  LazyMutex, LazyRecursiveMutex, LockGuard, Lockable, Mutex, MutexGuard, MutexGuardIf,
  RecursiveMutex, RecursiveMutexGuard,
};
pub use os::{MemoryPermission, MemoryRange, OS};
pub use reservation::AddressSpaceReservation;
pub use semaphore::Semaphore;
pub use stack::{Stack, StackSlot};
pub use thread::{
  K_MAX_THREAD_NAME_LENGTH, LocalStorageKey, Options, Priority, Thread, create_thread_local_key,
  delete_thread_local_key, get_existing_thread_local, get_thread_local, has_thread_local,
  set_thread_local,
};
pub use thread_id::current_thread_id;
#[cfg(target_os = "macos")]
pub use time::MachTimeSpec;
pub use time::{ThreadTicks, Time, TimeConstants, TimeDelta, TimeTicks};
pub use yield_processor::{YieldProcessor, yield_processor};
