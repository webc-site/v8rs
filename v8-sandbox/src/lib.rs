#![cfg_attr(docsrs, feature(doc_cfg))]

//! v8-sandbox: V8 `src/sandbox/` 第一梯队 (核心机制层) Rust 转写。
//!
//! 覆盖: Sandbox 地址空间预留 / SegmentedTable 段式表 / ExternalEntityTable
//! freelist 分配与 sweep / 外部指针 tag-Handle-payload 编码 / 有界大小 /
//! 沙箱内指针 / 硬件 (PKU) 支持。
//! 映射与推迟登记见 crate README。

pub mod bounded_size;
mod error;
mod external_entity_table;
mod external_pointer;
mod hardware_support;
mod internal;
mod sandbox;
pub mod sandboxed_pointer;
mod segmented_table;

pub use bounded_size::{read_bounded_size_field, write_bounded_size_field};
pub use error::{Error, Result};
pub use external_entity_table::{
  ExternalEntityTable, K_ENTRY_ALLOCATION_IS_FORBIDDEN_MARKER, K_INTERNAL_NULL_ENTRY_INDEX,
  K_NUM_READ_ONLY_SEGMENTS, Space,
};
pub use external_pointer::{
  ExternalPointerHandle, ExternalPointerMember, ExternalPointerPayload, ExternalPointerTag,
  ExternalPointerTagRange, K_ANY_EXTERNAL_POINTER_TAG_RANGE,
  K_ANY_FOREIGN_EXTERNAL_POINTER_TAG_RANGE, K_ANY_INTERCEPTOR_INFO_EXTERNAL_POINTER_TAG_RANGE,
  K_ANY_MANAGED_EXTERNAL_POINTER_TAG_RANGE, K_ANY_MANAGED_RESOURCE_EXTERNAL_POINTER_TAG_RANGE,
  K_ANY_MAYBE_READ_ONLY_EXTERNAL_POINTER_TAG_RANGE,
  K_ANY_SHARED_MANAGED_EXTERNAL_POINTER_TAG_RANGE, K_FIRST_EXTERNAL_POINTER_TAG,
  K_FIRST_INTERCEPTOR_INFO_EXTERNAL_POINTER_TAG, K_FIRST_MAYBE_READ_ONLY_EXTERNAL_POINTER_TAG,
  K_FIRST_SHARED_EXTERNAL_POINTER_TAG, K_LAST_EXTERNAL_POINTER_TAG, K_LAST_EXTERNAL_TYPE_TAG,
  K_LAST_INTERCEPTOR_INFO_EXTERNAL_POINTER_TAG, K_LAST_MANAGED_RESOURCE_TAG,
  K_LAST_MAYBE_READ_ONLY_EXTERNAL_POINTER_TAG, K_LAST_SHARED_EXTERNAL_POINTER_TAG,
  K_NULL_EXTERNAL_POINTER_HANDLE, external_pointer_can_be_empty, handle_to_index, index_to_handle,
  is_managed_external_pointer_type, is_maybe_read_only_external_pointer_type,
  is_shared_external_pointer_type, is_valid_handle,
};
pub use hardware_support::{
  AllowSandboxAccess, CodeSandboxingMode, DisallowSandboxAccess, EnterSandboxScope,
  ExitSandboxScope, SandboxHardwareSupport, enter_sandbox, exit_sandbox,
};
pub use internal::{
  Address, GB, K_ADDITIONAL_TRAILING_GUARD_REGION_SIZE, K_BOUNDED_SIZE_MASK, K_BOUNDED_SIZE_SHIFT,
  K_EXTERNAL_POINTER_INDEX_SHIFT, K_EXTERNAL_POINTER_MARK_BIT, K_EXTERNAL_POINTER_PAYLOAD_MASK,
  K_EXTERNAL_POINTER_SHIFTED_TAG_MASK, K_EXTERNAL_POINTER_TABLE_ENTRY_SIZE,
  K_EXTERNAL_POINTER_TABLE_ENTRY_SIZE_LOG2, K_EXTERNAL_POINTER_TABLE_RESERVATION_SIZE,
  K_EXTERNAL_POINTER_TAG_AND_MARKBIT_MASK, K_EXTERNAL_POINTER_TAG_MASK,
  K_EXTERNAL_POINTER_TAG_SHIFT, K_MAX_EXTERNAL_POINTERS, K_MAX_SAFE_BUFFER_SIZE_FOR_SANDBOX,
  K_PTR_COMPR_CAGE_BASE_ALIGNMENT, K_PTR_COMPR_CAGE_RESERVATION_SIZE,
  K_REQUIRES_TYPED_ARRAY_ACCESS_MASKS, K_SANDBOX_ALIGNMENT, K_SANDBOX_GUARD_REGION_SIZE,
  K_SANDBOX_MINIMUM_RESERVATION_SIZE, K_SANDBOX_SIZE, K_SANDBOX_SIZE_LOG2,
  K_SANDBOXED_POINTER_SHIFT, KB, MB, V8_EMBEDDER_DATA_TAG_COUNT, V8_EXTERNAL_POINTER_TAG_COUNT,
};
pub use sandbox::{
  InitConfig, K_FALLBACK_TO_PARTIALLY_RESERVED_SANDBOX_ALLOWED,
  K_MAX_VIRTUAL_ADDRESS_BITS_FOR_CRASH_FILTER, K_SANDBOX_ADDRESS_SPACE_NAME, K_SMI_ADDRESS_RANGE,
  K_SMI_ADDRESS_RANGE_PADDING, Sandbox, SandboxAddressSpace, SandboxedPointerConstants, current,
  determine_address_space_limit, empty_backing_store_buffer, initialize_once, inside_sandbox,
  outside_sandbox,
};
pub use sandboxed_pointer::{read_sandboxed_pointer_field, write_sandboxed_pointer_field};
pub use segmented_table::{
  AtomicFreelistHead, FreelistHead, K_ALIGNMENT, K_INTERNAL_READ_ONLY_SEGMENTS_OFFSET,
  K_SEGMENT_SIZE, K_USE_CONTIGUOUS_MEMORY, SegmentBase, SegmentedTable, SegmentedTableBase,
  TableEntry, UnsealReadOnlySegmentScope,
};
