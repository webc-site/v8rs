#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
mod handles;
mod local_handles;
mod maybe_handles;
mod persistent_handles;
mod shared_object_conveyor_handles;

pub use error::{Error, Result};
pub use handles::{
  DirectHandle, DirectHandleSmallVector, DirectHandleVector, Handle, HandleScope, HandleScopeData,
  HeapObject, IndirectHandle, K_CHECK_HANDLE_THRESHOLD, K_HANDLE_BLOCK_SIZE, K_HANDLE_ZAP_VALUE,
  K_PERSISTENT_HANDLE_ZAP_VALUE, K_TAGGED_NULL_ADDRESS, Object, SealHandleScope, SlotPtr, Smi,
  Tagged, is_smi,
};
pub use local_handles::{LocalHandleScope, LocalHandles, LocalHeapRef};
pub use maybe_handles::{
  MaybeDirectHandle, MaybeHandle, MaybeObjectDirectHandle, MaybeObjectHandle, is_weak, make_weak,
};
pub use persistent_handles::{PersistentHandles, PersistentHandlesList, PersistentHandlesScope};
pub use shared_object_conveyor_handles::SharedObjectConveyorHandles;
// 引用强度枚举定义在原版 common/globals.h, 归一 v8_common 单一真相 (禁本地重复)
pub use v8_common::HeapObjectReferenceType;
