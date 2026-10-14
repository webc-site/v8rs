#![cfg_attr(docsrs, feature(doc_cfg))]

//! `v8/src/tracing` 的 Rust 移植 crate (18 文件 3654 行 + include/v8-platform.h 追踪面 → 9 模块)。
//!
//! 模块映射与推迟登记详见 `readme/zh.md`:
//! - `trace_event_macros` — trace-event-no-perfetto.h 的 X-macro 展开
//!   (123 宏函数 + 2 项登记不展开 + 55 常量 + 9 个 perfetto 空壳类型登记)
//! - `trace_event` — trace-event.h/.cc 核心管道 + 全局 controller 安装点
//! - `platform` — include/v8-platform.h 的 embedder 追踪面
//! - `traced_value` — TracedValue JSON 构造器 (原版单测 1:1)
//! - `trace_categories` — 32 类别注册表 (27 单类别 + 5 组)
//! - `category_observer` — TracingCategoryObserver 启停位翻转
//! - `trace_id` / `perfetto_utils` — 序号发生器 / proto 字符串数据面
//!
//! perfetto 硬绑定四件 (perfetto-logger / code-data-source / code-trace-context /
//! perfetto-sdk) 依赖 perfetto protos 与 L3 对象层, 推迟登记, 不产占位代码。

mod category_observer;
mod error;
mod perfetto_utils;
mod platform;
mod trace_categories;
mod trace_event;
mod trace_event_macros;
mod trace_id;
mod traced_value;

// --- error.rs ---
// --- category_observer.rs ---
pub use category_observer::{
  ENABLED_BY_NATIVE, ENABLED_BY_SAMPLING, ENABLED_BY_TRACING, TracingCategoryObserver,
};
pub use error::{Error, Result};
// --- perfetto_utils.rs ---
pub use perfetto_utils::{PerfettoV8String, StringEncoding, UC16_SIZE};
// --- platform.rs (include/v8-platform.h 追踪面) ---
pub use platform::{
  ConvertableToTraceFormat, NoopController, TraceStateObserver, TracingController,
};
// --- trace_categories.rs ---
pub use trace_categories::{TEST_CATEGORY_PREFIXES, TRACE_CATEGORIES, TraceCategoryDef};
// --- trace_event.rs (trace-event.h/.cc) ---
pub use trace_event::{
  ENABLED_FOR_ETW_EXPORT, ENABLED_FOR_EVENT_CALLBACK, ENABLED_FOR_RECORDING, K_GLOBAL_SCOPE,
  K_NO_ID, K_ZERO_NUM_ARGS, ScopedTracer, TraceEvent, TraceEventArg, TraceId, TraceStringWithCopy,
  TraceValue, add_trace_state_observer, remove_trace_state_observer, set_tracing_controller,
  tracing_controller,
};
// --- trace_event_macros.rs (trace-event-no-perfetto.h X-macro 展开: 55 常量 + 123 宏函数) ---
pub use trace_event_macros::{
  DISABLED_BY_DEFAULT_PREFIX, TRACE_EVENT_FLAG_ASYNC_TTS, TRACE_EVENT_FLAG_BIND_TO_ENCLOSING,
  TRACE_EVENT_FLAG_COPY, TRACE_EVENT_FLAG_EXPLICIT_TIMESTAMP, TRACE_EVENT_FLAG_FLOW_IN,
  TRACE_EVENT_FLAG_FLOW_OUT, TRACE_EVENT_FLAG_HAS_CONTEXT_ID, TRACE_EVENT_FLAG_HAS_GLOBAL_ID,
  TRACE_EVENT_FLAG_HAS_ID, TRACE_EVENT_FLAG_HAS_LOCAL_ID, TRACE_EVENT_FLAG_HAS_PROCESS_ID,
  TRACE_EVENT_FLAG_JAVA_STRING_LITERALS, TRACE_EVENT_FLAG_NONE, TRACE_EVENT_FLAG_SCOPE_EXTRA,
  TRACE_EVENT_FLAG_SCOPE_MASK, TRACE_EVENT_FLAG_SCOPE_OFFSET, TRACE_EVENT_PHASE_ASYNC_BEGIN,
  TRACE_EVENT_PHASE_ASYNC_END, TRACE_EVENT_PHASE_ASYNC_STEP_INTO,
  TRACE_EVENT_PHASE_ASYNC_STEP_PAST, TRACE_EVENT_PHASE_BEGIN, TRACE_EVENT_PHASE_CLOCK_SYNC,
  TRACE_EVENT_PHASE_COMPLETE, TRACE_EVENT_PHASE_COUNTER, TRACE_EVENT_PHASE_CREATE_OBJECT,
  TRACE_EVENT_PHASE_DELETE_OBJECT, TRACE_EVENT_PHASE_END, TRACE_EVENT_PHASE_FLOW_BEGIN,
  TRACE_EVENT_PHASE_FLOW_END, TRACE_EVENT_PHASE_FLOW_STEP, TRACE_EVENT_PHASE_INSTANT,
  TRACE_EVENT_PHASE_MARK, TRACE_EVENT_PHASE_MEMORY_DUMP, TRACE_EVENT_PHASE_METADATA,
  TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN, TRACE_EVENT_PHASE_NESTABLE_ASYNC_END,
  TRACE_EVENT_PHASE_NESTABLE_ASYNC_INSTANT, TRACE_EVENT_PHASE_SAMPLE,
  TRACE_EVENT_PHASE_SNAPSHOT_OBJECT, TRACE_EVENT_SCOPE_GLOBAL, TRACE_EVENT_SCOPE_NAME_GLOBAL,
  TRACE_EVENT_SCOPE_NAME_PROCESS, TRACE_EVENT_SCOPE_NAME_THREAD, TRACE_EVENT_SCOPE_PROCESS,
  TRACE_EVENT_SCOPE_THREAD, TRACE_VALUE_TYPE_BOOL, TRACE_VALUE_TYPE_CONVERTABLE,
  TRACE_VALUE_TYPE_COPY_STRING, TRACE_VALUE_TYPE_DOUBLE, TRACE_VALUE_TYPE_INT,
  TRACE_VALUE_TYPE_POINTER, TRACE_VALUE_TYPE_PROTO, TRACE_VALUE_TYPE_STRING, TRACE_VALUE_TYPE_UINT,
};
pub use trace_event_macros::{
  trace_copy_counter_id1, trace_copy_counter_id2, trace_copy_counter1, trace_copy_counter2,
  trace_counter, trace_counter_id1, trace_counter_id2, trace_counter_with_flag1,
  trace_counter_with_timestamp1, trace_counter_with_timestamp2, trace_counter1, trace_counter2,
  trace_disabled_by_default, trace_event, trace_event_async_begin_with_timestamp0,
  trace_event_async_begin_with_timestamp1, trace_event_async_begin_with_timestamp2,
  trace_event_async_begin0, trace_event_async_begin1, trace_event_async_begin2,
  trace_event_async_end_with_timestamp0, trace_event_async_end_with_timestamp1,
  trace_event_async_end_with_timestamp2, trace_event_async_end0, trace_event_async_end1,
  trace_event_async_end2, trace_event_async_step_into_with_timestamp0,
  trace_event_async_step_into0, trace_event_async_step_into1, trace_event_async_step_past0,
  trace_event_async_step_past1, trace_event_begin, trace_event_begin_with_flags0,
  trace_event_begin_with_flags1, trace_event_begin_with_id_tid_and_timestamp0, trace_event_begin0,
  trace_event_begin1, trace_event_begin2, trace_event_category_enabled,
  trace_event_category_group_enabled, trace_event_clock_sync_issuer,
  trace_event_clock_sync_receiver, trace_event_copy_async_begin_with_timestamp0,
  trace_event_copy_async_begin0, trace_event_copy_async_begin1, trace_event_copy_async_begin2,
  trace_event_copy_async_end_with_timestamp0, trace_event_copy_async_end0,
  trace_event_copy_async_end1, trace_event_copy_async_end2,
  trace_event_copy_begin_with_id_tid_and_timestamp0,
  trace_event_copy_begin_with_id_tid_and_timestamp1,
  trace_event_copy_begin_with_id_tid_and_timestamp2, trace_event_copy_begin2,
  trace_event_copy_end_with_id_tid_and_timestamp0, trace_event_copy_end_with_id_tid_and_timestamp1,
  trace_event_copy_end_with_id_tid_and_timestamp2, trace_event_copy_end2,
  trace_event_copy_instant0, trace_event_copy_instant1, trace_event_copy_instant2,
  trace_event_copy_mark, trace_event_copy_mark_with_timestamp, trace_event_copy_mark1,
  trace_event_copy_nestable_async_begin_with_timestamp0,
  trace_event_copy_nestable_async_begin_with_timestamp1,
  trace_event_copy_nestable_async_begin_with_tts2, trace_event_copy_nestable_async_begin0,
  trace_event_copy_nestable_async_begin1, trace_event_copy_nestable_async_begin2,
  trace_event_copy_nestable_async_end_with_timestamp0,
  trace_event_copy_nestable_async_end_with_tts2, trace_event_copy_nestable_async_end0,
  trace_event_copy_nestable_async_end1, trace_event_end, trace_event_end_with_flags0,
  trace_event_end_with_flags1, trace_event_end_with_id_tid_and_timestamp0, trace_event_end0,
  trace_event_end1, trace_event_end2, trace_event_instant, trace_event_instant_with_flags0,
  trace_event_instant_with_flags1, trace_event_instant_with_timestamp0,
  trace_event_instant_with_timestamp1, trace_event_instant0, trace_event_instant1,
  trace_event_instant2, trace_event_mark_with_timestamp0, trace_event_mark_with_timestamp1,
  trace_event_mark_with_timestamp2, trace_event_nestable_async_begin_with_flags0,
  trace_event_nestable_async_begin_with_timestamp_and_flags0,
  trace_event_nestable_async_begin_with_timestamp0,
  trace_event_nestable_async_begin_with_timestamp1, trace_event_nestable_async_begin0,
  trace_event_nestable_async_begin1, trace_event_nestable_async_begin2,
  trace_event_nestable_async_end_with_flags0,
  trace_event_nestable_async_end_with_timestamp_and_flags0,
  trace_event_nestable_async_end_with_timestamp0, trace_event_nestable_async_end_with_timestamp1,
  trace_event_nestable_async_end_with_timestamp2, trace_event_nestable_async_end0,
  trace_event_nestable_async_end1, trace_event_nestable_async_end2,
  trace_event_nestable_async_instant_with_timestamp0, trace_event_nestable_async_instant0,
  trace_event_nestable_async_instant1, trace_event_nestable_async_instant2,
  trace_event_object_created_with_id, trace_event_object_deleted_with_id,
  trace_event_object_snapshot_with_id, trace_event_sample_with_id1, trace_event_with_flow0,
  trace_event_with_flow1, trace_event_with_flow2, trace_event0, trace_event1, trace_event2,
  trace_time_now, trace_time_ticks_now,
};
// --- trace_id.rs ---
pub use trace_id::next_trace_id;
// --- traced_value.rs ---
pub use traced_value::TracedValue;
