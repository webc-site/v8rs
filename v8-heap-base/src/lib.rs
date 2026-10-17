#![cfg_attr(docsrs, feature(doc_cfg))]
//! v8-heap-base — V8 `src/heap/base` 的 Rust 转写 (GC 基础件层)。
//!
//! 覆盖: Worklist 并发工作队列 / Stack 纯栈抽象 / BasicSlotSet 记录集 /
//! ActiveSystemPages 活跃页位集 / IncrementalMarkingSchedule 增量标记调度 /
//! bytes 吞吐统计 / UnsafeJsonEmitter / CachedUnorderedMap / MTE 挂起域。
//! 映射表与推迟登记见 crate README。

mod active_system_pages;
mod basic_slot_set;
mod bytes;
mod cached_unordered_map;
mod error;
mod incremental_marking_schedule;
mod memory_tagging;
mod stack;
mod unsafe_json_emitter;
mod worklist;

pub use active_system_pages::{ActiveSystemPages, K_MAX_PAGES};
pub use basic_slot_set::{
  AccessMode, BasicSlotSet, EmptyBucketMode, K_BITS_PER_BUCKET, K_BITS_PER_BUCKET_LOG2,
  K_BITS_PER_CELL, K_BITS_PER_CELL_LOG2, K_CELL_SIZE_BYTES, K_CELL_SIZE_BYTES_LOG2,
  K_CELLS_PER_BUCKET, K_CELLS_PER_BUCKET_LOG2, SlotCallbackResult,
};
pub use bytes::{
  ByteSize, BytesAndDuration, BytesAndDurationBuffer, RingBuffer, SmoothedBytesAndDuration,
  average_speed, average_speed_unbounded,
};
pub use cached_unordered_map::CachedUnorderedMap;
pub use error::{Error, Result};
pub use incremental_marking_schedule::{
  IncrementalMarkingSchedule, K_EPHEMERON_PAIRS_FLUSHING_RATIO_INCREMENTS,
  K_ESTIMATED_MARKING_TIME, K_STEP_SIZE_WHEN_NOT_MAKING_PROGRESS, StepInfo,
};
pub use memory_tagging::SuspendTagCheckingScope;
pub use stack::{
  Segment as StackSegment, Stack, StackVisitor, StackVisitorCallback, ThreadId, is_on_current_stack,
};
pub use unsafe_json_emitter::{JsonValue, UnsafeJsonEmitter};
pub use worklist::{Local as WorklistLocal, Segment as WorklistSegment, Worklist, WorklistBase};
