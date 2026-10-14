//! v8-heap-gc — 代 GC 算法 1:1 迁移语义面。
//!
//! 族: `v8/src/heap/` 中 GC 内核 (mark-compact / sweeper / scavenger / minor-ms /
//! incremental-marking(+job) / memory-reducer / marking(+state,barrier,worklist,visitor) /
//! gc-tracer(+cc))。
//! 分工: 本 crate 只承载 GC 算法纯语义与规则面; Heap/Tagged/Map/NormalPage/MutablePage/
//! SlotSet 等对象模型属 v8-heap-core, 相关方法体在 heap-core 冻结前于 `readme/zh.md`
//! 推迟台账登记。
//! 破环: 事件名/常量/状态机/位图/进度追踪等纯逻辑无外部依赖; marking bitmap 手写
//! arena 前置于本 crate, 后续 visitors 挂载时以手写访问器绕开 Arc/Weak 环。

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// —— 底座 (依赖最浅) ——
mod error;
mod heap_layout;
mod marking;
mod marking_progress_tracker;

// —— 追踪/规则枚举与常量族 ——
mod gc_tracer;
mod gc_tracer_cc;
mod incremental_marking;
mod incremental_marking_job;
mod mark_compact;
mod mark_compact_enums;
mod memory_reducer;
mod memory_reducer_enums;
mod minor_mark_sweep;
mod scavenger;
mod sweeper;
mod sweeper_enums;

pub use error::{Error, Result};
// gc_tracer: ScopeId/Event 枚举面 (EventType/EventState/Mode/MarkingType/YoungGenerationSpeedMode)
pub use gc_tracer::{
  EventState, EventType as GcEventType, MarkingType, Mode as GcTracerMode, ScopeId,
  YoungGenerationSpeedMode,
};
// gc_tracer_cc: 事件名表 + 速度界 (Priority 独立避免与 isolate::Priority 冲名)
pub use gc_tracer_cc::{
  K_MAX_SPEED_IN_BYTES_PER_MS, K_MIN_NON_EMPTY_SPEED_IN_BYTES_PER_MS, Priority as GcTracerPriority,
  bounded_average_speed, bounded_throughput, event_state_to_string, event_type_to_string,
  priority_to_string,
};
// heap_layout: 全局地址/页/槽宽常量
pub use heap_layout::{
  Address, K_ALL_BITS_SET_IN_CELL_VALUE, K_BITS_PER_BYTE, K_BITS_PER_BYTE_LOG2,
  K_MAX_REGULAR_HEAP_OBJECT_SIZE, K_PAGE_ALIGNMENT_MASK, K_PAGE_SIZE_BITS, K_REGULAR_PAGE_SIZE,
  K_TAGGED_SIZE_LOG2,
};
// incremental_marking: StepOrigin + 阈值纯式
pub use incremental_marking::{
  K_GLOBAL_ACTIVATION_THRESHOLD, K_GLOBAL_ACTIVATION_THRESHOLD_DEBUG,
  K_MAJOR_GC_OLD_GENERATION_ALLOCATION_OBSERVER_STEP,
  K_MAJOR_GC_YOUNG_GENERATION_ALLOCATION_OBSERVER_STEP, K_MAX_STEP_SIZE_ON_ALLOCATION_MS,
  K_MAX_STEP_SIZE_ON_TASK_MS, K_V8_ACTIVATION_THRESHOLD, K_V8_ACTIVATION_THRESHOLD_DEBUG,
  StepOrigin, get_max_duration_ms, is_below_activation_thresholds,
};
// incremental_marking_job: 前台任务调度骨架
pub use incremental_marking_job::IncrementalMarkingJobSkeleton;
// mark_compact_enums + mark_compact
pub use mark_compact::{
  EvacuationCandidatePage, EvacuationMode, EvacuationSelectionResult, K_DEADLINE_CHECK_INTERVAL,
  K_MAX_EVACUATED_BYTES_MB, K_MAX_POINTER_UPDATE_TASKS, K_MAX_STRING_TABLE_CHUNKS,
  K_MIN_STRING_TABLE_CHUNK_SIZE, K_STRESS_ABORT_FRACTION, K_TARGET_FRAGMENTATION_PERCENT,
  K_TARGET_FRAGMENTATION_PERCENT_REDUCE_MEMORY, K_TARGET_MS_PER_AREA, MB,
  compute_evacuation_heuristics, compute_evacuation_mode, compute_free_bytes_threshold,
  estimated_new_pages, select_evacuation_candidates,
};
pub use mark_compact_enums::{
  CallOrigin, CollectorState, EphemeronResult, MarkingWorklistProcessingMode, StartCompactionMode,
};
// marking: MarkBit / MarkingBitmap 数据结构 (非原子, 面向单元测试)
pub use marking::{
  K_BIT_INDEX_MASK, K_BITS_PER_CELL, K_BITS_PER_CELL_LOG2, K_BYTES_PER_CELL, K_BYTES_PER_CELL_LOG2,
  K_CELLS_COUNT, K_LENGTH, K_SIZE, LivenessMode, MarkBit, MarkBitCell, MarkingBitmap,
  WorklistTarget, address_to_index, cell_align_index, cell_to_base, index_in_cell,
  index_in_cell_mask, index_to_address_offset, index_to_cell, limit_address_to_index,
};
// marking_progress_tracker: 增量标记进度分块
pub use marking_progress_tracker::MarkingProgressTracker;
// memory_reducer_enums + memory_reducer
pub use memory_reducer::{
  Event as MemoryReducerEvent, K_COMMITTED_MEMORY_DELTA, K_COMMITTED_MEMORY_FACTOR,
  K_SHORT_DELAY_MS, K_SLACK_MS, K_WATCHDOG_DELAY_MS, State as MemoryReducerState,
  max_number_of_gcs, schedule_delay_seconds, step as memory_reducer_step,
  watchdog_gc as memory_reducer_watchdog_gc,
};
pub use memory_reducer_enums::{EventType as MemoryReducerEventType, Id as MemoryReducerId};
// minor_mark_sweep 纯面
pub use minor_mark_sweep::{K_MAX_PARALLEL_TASKS as K_MINOR_MS_MAX_PARALLEL_TASKS, SlotsType};
// scavenger 纯面
pub use scavenger::{
  MigrationOutcome, ObjectAge, ScavengePreference, ScavengerTaskStats,
  determine_scavenge_preference, is_address_below_age_mark, number_of_scavenge_tasks,
  scavenger_consts,
};
// sweeper_enums + sweeper
pub use sweeper::{
  K_CODE_SPACE, K_FIRST_SWEEPABLE_SPACE, K_LAST_SWEEPABLE_SPACE, K_LO_SPACE, K_MAX_TASKS_MAJOR,
  K_MAX_TASKS_MINOR, K_NEW_SPACE, K_NUMBER_OF_MAJOR_SWEEPING_SPACES, K_NUMBER_OF_SWEEPING_SPACES,
  K_OLD_SPACE, K_PAGE_PER_TASK, K_SHARED_SPACE, K_SHARED_TRUSTED_SPACE, K_TRUSTED_SPACE,
  K_ZAP_TAGGED, PageSweepingState, SweepingJobStatus, SweepingPhase, SweepingStateMachine,
  ZappingMode, compute_sweeper_trace_ids, concurrent_minor_space, get_max_concurrency,
  should_zap_dead_objects_on_page,
};
pub use sweeper_enums::{
  FreeSpaceTreatmentMode, IncludeMainThread, SafepointKind, SweepingMode, SweepingScope,
};
use v8_heap_core::linear_allocation_area::LinearAllocationArea;

/// 桥接: 在垃圾回收前重置线性分配区并统计待回收碎片 (真实消费 LinearAllocationArea)
pub fn gc_prepare_linear_allocation_area(area: &mut LinearAllocationArea) -> usize {
  let wasted_bytes = area.limit().saturating_sub(area.top());
  area.reset(0, 0);
  wasted_bytes
}
