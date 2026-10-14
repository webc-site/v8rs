//! v8-heap-cppgc — Oilpan/cppgc 内部机制（heap/cppgc-internal + heap/cppgc-js）。
//!
//! 归属看板：todo.md 三 · `v8-heap-cppgc`。

#![cfg_attr(docsrs, feature(doc_cfg))]

mod compaction_worklists;
mod error;
mod gc_info;
mod gc_info_table;
mod globals;
mod heap_object_header;
mod liveness_broker;
mod memory;
mod metric_recorder;
mod name_trait;
mod object_size_trait;
mod object_start_bitmap;
mod object_view;
mod process_heap_statistics;
mod task_handle;
mod trace_trait;
mod visitor;

pub use compaction_worklists::{CompactionWorklists, MovableReference, MovableReferencesWorklist};
pub use error::{Error, Result};
pub use gc_info::{
  EnsureGcInfoIndexTrait, FinalizationCallback, GcInfo, GcInfoIndex, K_MAX_GC_INFO_INDEX,
  K_MIN_GC_INFO_INDEX, get_hidden_name,
};
pub use gc_info_table::{GcInfoTable, GlobalGcInfoTable, K_MAX_INDEX, K_MIN_INDEX};
pub use globals::{
  AccessMode, Address, ConstAddress, K_ALLOCATION_GRANULARITY, K_ALLOCATION_MASK,
  K_FREE_LIST_ENTRY_SIZE, K_FREE_LIST_GC_INFO_INDEX, K_GB, K_KB, K_LARGE_OBJECT_SIZE_THRESHOLD,
  K_MB, K_PAGE_BASE_MASK, K_PAGE_OFFSET_MASK, K_PAGE_SIZE, K_PAGE_SIZE_LOG2, K_SLOT_SIZE,
};
pub use heap_object_header::{
  HeapObjectHeader, K_LARGE_OBJECT_SIZE_IN_HEADER, K_MAX_SIZE, K_SIZE_LOG2,
};
pub use liveness_broker::{LivenessBroker, LivenessBrokerFactory, is_heap_object_alive_impl};
pub use memory::{
  CHECK_MEMORY_IS_INACCESSIBLE_IS_NOOP, CPPGC_SHOULD_ZAP_MEMORY, K_ZAPPED_VALUE,
  check_memory_is_inaccessible, check_memory_is_zapped, check_memory_is_zero, no_sanitize_memset,
  set_memory_accessible, set_memory_inaccessible, zap_memory,
};
pub use metric_recorder::{
  GcCycle, GcCycleType, IncrementalPhases, MainThreadIncrementalMark, MainThreadIncrementalSweep,
  MetricRecorder, Phases, Sizes,
};
pub use name_trait::{
  HeapObjectName, HeapObjectNameForUnnamedObject, K_HIDDEN_NAME, K_NO_NAME_DEDUCIBLE, NameCallback,
  NameProvider, NameTrait, NameTraitBase, supports_cpp_class_names_as_object_names,
};
pub use object_size_trait::BaseObjectSizeTrait;
pub use object_start_bitmap::{
  K_BITMAP_SIZE, K_RESERVED_FOR_BITMAP, ObjectStartBitmap, PlatformAwareObjectStartBitmap,
};
pub use object_view::ObjectView;
pub use process_heap_statistics::{
  AllocationObserverImpl, ProcessHeapStatistics, ProcessHeapStatisticsUpdater,
};
pub use task_handle::{NonEmptyTag, SingleThreadedHandle};
pub use trace_trait::{TraceCallback, TraceDescriptor};
pub use visitor::{Visitor, VisitorFactory, VisitorKey};
