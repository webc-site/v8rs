// 由 tool/src/tracing/trace-event-no-perfetto.h.js 从 v8/src/tracing/trace-event-no-perfetto.h 生成; 修订就地改, 复跑覆盖
// X-macro 展开: TRACE_EVENT* 宏族 → 显式 Rust 函数, 逐个登记
//! trace-event 宏体系展开登记表 (trace-event-no-perfetto.h, 982 行)。
//!
//! 原版宏 `TRACE_EVENT*` 经 `INTERNAL_TRACE_EVENT_*` 两层转发到
//! `v8::internal::tracing::AddTraceEvent[WithTimestamp]` (trace-event.h);
//! 本模块把每个宏机械展开为同名 snake_case 函数, 参数/phase/flags 1:1。
//! 展开规则 (trace-event.h:181-297):
//! - `ADD`: 类别启用检查 → `emit(phase, ...)`
//! - `ADD_WITH_ID`: `flags |= HAS_ID` + TraceID → `emit_id`
//! - `ADD_WITH_TIMESTAMP` / `ADD_WITH_ID_TID_AND_TIMESTAMP`: 同上加 timestamp (tid 丢弃, 原版重定向)
//! - `ADD_SCOPED`: COMPLETE 相位 + `ScopedTracer` (drop 时补 end 时长)
//! - 计数器 value 一律 `static_cast<int>` (i32 截断, 原版行为)

use crate::trace_event::{
  ScopedTracer, TraceEventArg, TraceId, TraceValue, category_enabled, emit, emit_id, emit_id_ts,
  emit_ts, scoped_add, scoped_add_with_flow, ticks_now_ns, time_now_ns,
};

/// `TRACE_DISABLED_BY_DEFAULT` 前缀 ("disabled-by-default-")。
pub const DISABLED_BY_DEFAULT_PREFIX: &str = "disabled-by-default-";

// ---- TRACE_EVENT_PHASE_* (23) ----
pub const TRACE_EVENT_PHASE_BEGIN: u8 = b'B';
pub const TRACE_EVENT_PHASE_END: u8 = b'E';
pub const TRACE_EVENT_PHASE_COMPLETE: u8 = b'X';
pub const TRACE_EVENT_PHASE_INSTANT: u8 = b'I';
pub const TRACE_EVENT_PHASE_ASYNC_BEGIN: u8 = b'S';
pub const TRACE_EVENT_PHASE_ASYNC_STEP_INTO: u8 = b'T';
pub const TRACE_EVENT_PHASE_ASYNC_STEP_PAST: u8 = b'p';
pub const TRACE_EVENT_PHASE_ASYNC_END: u8 = b'F';
pub const TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN: u8 = b'b';
pub const TRACE_EVENT_PHASE_NESTABLE_ASYNC_END: u8 = b'e';
pub const TRACE_EVENT_PHASE_NESTABLE_ASYNC_INSTANT: u8 = b'n';
pub const TRACE_EVENT_PHASE_FLOW_BEGIN: u8 = b's';
pub const TRACE_EVENT_PHASE_FLOW_STEP: u8 = b't';
pub const TRACE_EVENT_PHASE_FLOW_END: u8 = b'f';
pub const TRACE_EVENT_PHASE_METADATA: u8 = b'M';
pub const TRACE_EVENT_PHASE_COUNTER: u8 = b'C';
pub const TRACE_EVENT_PHASE_SAMPLE: u8 = b'P';
pub const TRACE_EVENT_PHASE_CREATE_OBJECT: u8 = b'N';
pub const TRACE_EVENT_PHASE_SNAPSHOT_OBJECT: u8 = b'O';
pub const TRACE_EVENT_PHASE_DELETE_OBJECT: u8 = b'D';
pub const TRACE_EVENT_PHASE_MEMORY_DUMP: u8 = b'v';
pub const TRACE_EVENT_PHASE_MARK: u8 = b'R';
pub const TRACE_EVENT_PHASE_CLOCK_SYNC: u8 = b'c';

// ---- TRACE_EVENT_FLAG_* 位标志 (14) ----
pub const TRACE_EVENT_FLAG_COPY: u32 = 1 << 0;
pub const TRACE_EVENT_FLAG_HAS_ID: u32 = 1 << 1;
pub const TRACE_EVENT_FLAG_SCOPE_OFFSET: u32 = 1 << 2;
pub const TRACE_EVENT_FLAG_SCOPE_EXTRA: u32 = 1 << 3;
pub const TRACE_EVENT_FLAG_EXPLICIT_TIMESTAMP: u32 = 1 << 4;
pub const TRACE_EVENT_FLAG_ASYNC_TTS: u32 = 1 << 5;
pub const TRACE_EVENT_FLAG_BIND_TO_ENCLOSING: u32 = 1 << 6;
pub const TRACE_EVENT_FLAG_FLOW_IN: u32 = 1 << 7;
pub const TRACE_EVENT_FLAG_FLOW_OUT: u32 = 1 << 8;
pub const TRACE_EVENT_FLAG_HAS_CONTEXT_ID: u32 = 1 << 9;
pub const TRACE_EVENT_FLAG_HAS_PROCESS_ID: u32 = 1 << 10;
pub const TRACE_EVENT_FLAG_HAS_LOCAL_ID: u32 = 1 << 11;
pub const TRACE_EVENT_FLAG_HAS_GLOBAL_ID: u32 = 1 << 12;
pub const TRACE_EVENT_FLAG_JAVA_STRING_LITERALS: u32 = 1 << 16;

/// TRACE_EVENT_FLAG_NONE (unsigned int 字面 0)。
pub const TRACE_EVENT_FLAG_NONE: u32 = 0;

/// TRACE_EVENT_FLAG_SCOPE_MASK = SCOPE_OFFSET | SCOPE_EXTRA (复合, 原版逐字组合)。
pub const TRACE_EVENT_FLAG_SCOPE_MASK: u32 =
  TRACE_EVENT_FLAG_SCOPE_OFFSET | TRACE_EVENT_FLAG_SCOPE_EXTRA;

// ---- TRACE_VALUE_TYPE_* (9) ----
pub const TRACE_VALUE_TYPE_BOOL: u8 = 1;
pub const TRACE_VALUE_TYPE_UINT: u8 = 2;
pub const TRACE_VALUE_TYPE_INT: u8 = 3;
pub const TRACE_VALUE_TYPE_DOUBLE: u8 = 4;
pub const TRACE_VALUE_TYPE_POINTER: u8 = 5;
pub const TRACE_VALUE_TYPE_STRING: u8 = 6;
pub const TRACE_VALUE_TYPE_COPY_STRING: u8 = 7;
pub const TRACE_VALUE_TYPE_CONVERTABLE: u8 = 8;
pub const TRACE_VALUE_TYPE_PROTO: u8 = 9;

// ---- TRACE_EVENT_SCOPE_* (3, 须落在 SCOPE_MASK 内) ----
pub const TRACE_EVENT_SCOPE_GLOBAL: u8 = 0 << 2;
pub const TRACE_EVENT_SCOPE_PROCESS: u8 = 1 << 2;
pub const TRACE_EVENT_SCOPE_THREAD: u8 = 2 << 2;

// ---- TRACE_EVENT_SCOPE_NAME_* (3) ----
pub const TRACE_EVENT_SCOPE_NAME_GLOBAL: u8 = b'g';
pub const TRACE_EVENT_SCOPE_NAME_PROCESS: u8 = b'p';
pub const TRACE_EVENT_SCOPE_NAME_THREAD: u8 = b't';

/// `TRACE_DISABLED_BY_DEFAULT(name)` 展开 (trace-event-no-perfetto.h:138)。
pub fn trace_disabled_by_default(name: &str) -> String {
  format!("{DISABLED_BY_DEFAULT_PREFIX}{name}")
}

/// `TRACE_EVENT0(category_group, name)` 展开 (trace-event-no-perfetto.h:145)。
pub fn trace_event0(category_group: &'static str, name: &'static str) -> ScopedTracer {
  scoped_add(category_group, name, vec![])
}

/// `TRACE_EVENT_WITH_FLOW0(category_group, name, bind_id, flow_flags)` 展开 (trace-event-no-perfetto.h:147)。
pub fn trace_event_with_flow0(
  category_group: &'static str,
  name: &'static str,
  bind_id: impl Into<TraceId>,
  flow_flags: u32,
) -> ScopedTracer {
  scoped_add_with_flow(category_group, name, bind_id.into(), flow_flags, vec![])
}

/// `TRACE_EVENT1(category_group, name, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:150)。
pub fn trace_event1(
  category_group: &'static str,
  name: &'static str,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) -> ScopedTracer {
  scoped_add(
    category_group,
    name,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  )
}

/// `TRACE_EVENT_WITH_FLOW1(category_group, name, bind_id, flow_flags, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:152)。
pub fn trace_event_with_flow1(
  category_group: &'static str,
  name: &'static str,
  bind_id: impl Into<TraceId>,
  flow_flags: u32,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) -> ScopedTracer {
  scoped_add_with_flow(
    category_group,
    name,
    bind_id.into(),
    flow_flags,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  )
}

/// `TRACE_EVENT2(category_group, name, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:156)。
pub fn trace_event2(
  category_group: &'static str,
  name: &'static str,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) -> ScopedTracer {
  scoped_add(
    category_group,
    name,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  )
}

/// `TRACE_EVENT_WITH_FLOW2(category_group, name, bind_id, flow_flags, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:160)。
pub fn trace_event_with_flow2(
  category_group: &'static str,
  name: &'static str,
  bind_id: impl Into<TraceId>,
  flow_flags: u32,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) -> ScopedTracer {
  scoped_add_with_flow(
    category_group,
    name,
    bind_id.into(),
    flow_flags,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  )
}

/// `TRACE_EVENT_INSTANT0(category_group, name, scope)` 展开 (trace-event-no-perfetto.h:171)。
pub fn trace_event_instant0(category_group: &'static str, name: &'static str, scope: u8) {
  emit(
    TRACE_EVENT_PHASE_INSTANT,
    category_group,
    name,
    TRACE_EVENT_FLAG_NONE | u32::from(scope),
    vec![],
  );
}

/// `TRACE_EVENT_INSTANT1(category_group, name, scope, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:174)。
pub fn trace_event_instant1(
  category_group: &'static str,
  name: &'static str,
  scope: u8,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit(
    TRACE_EVENT_PHASE_INSTANT,
    category_group,
    name,
    TRACE_EVENT_FLAG_NONE | u32::from(scope),
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_INSTANT2(category_group, name, scope, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:177)。
pub fn trace_event_instant2(
  category_group: &'static str,
  name: &'static str,
  scope: u8,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit(
    TRACE_EVENT_PHASE_INSTANT,
    category_group,
    name,
    TRACE_EVENT_FLAG_NONE | u32::from(scope),
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_COPY_INSTANT0(category_group, name, scope)` 展开 (trace-event-no-perfetto.h:182)。
pub fn trace_event_copy_instant0(category_group: &'static str, name: &'static str, scope: u8) {
  emit(
    TRACE_EVENT_PHASE_INSTANT,
    category_group,
    name,
    TRACE_EVENT_FLAG_COPY | u32::from(scope),
    vec![],
  );
}

/// `TRACE_EVENT_COPY_INSTANT1(category_group, name, scope, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:185)。
pub fn trace_event_copy_instant1(
  category_group: &'static str,
  name: &'static str,
  scope: u8,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit(
    TRACE_EVENT_PHASE_INSTANT,
    category_group,
    name,
    TRACE_EVENT_FLAG_COPY | u32::from(scope),
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_COPY_INSTANT2(category_group, name, scope, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:189)。
pub fn trace_event_copy_instant2(
  category_group: &'static str,
  name: &'static str,
  scope: u8,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit(
    TRACE_EVENT_PHASE_INSTANT,
    category_group,
    name,
    TRACE_EVENT_FLAG_COPY | u32::from(scope),
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_INSTANT_WITH_FLAGS0(category_group, name, scope_and_flags)` 展开 (trace-event-no-perfetto.h:194)。
pub fn trace_event_instant_with_flags0(
  category_group: &'static str,
  name: &'static str,
  scope_and_flags: u32,
) {
  emit(
    TRACE_EVENT_PHASE_INSTANT,
    category_group,
    name,
    scope_and_flags,
    vec![],
  );
}

/// `TRACE_EVENT_INSTANT_WITH_FLAGS1(category_group, name, scope_and_flags, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:197)。
pub fn trace_event_instant_with_flags1(
  category_group: &'static str,
  name: &'static str,
  scope_and_flags: u32,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit(
    TRACE_EVENT_PHASE_INSTANT,
    category_group,
    name,
    scope_and_flags,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_INSTANT_WITH_TIMESTAMP0(category_group, name, scope, timestamp)` 展开 (trace-event-no-perfetto.h:202)。
pub fn trace_event_instant_with_timestamp0(
  category_group: &'static str,
  name: &'static str,
  scope: u8,
  timestamp: i64,
) {
  emit_ts(
    TRACE_EVENT_PHASE_INSTANT,
    category_group,
    name,
    timestamp,
    TRACE_EVENT_FLAG_NONE | u32::from(scope),
    vec![],
  );
}

/// `TRACE_EVENT_INSTANT_WITH_TIMESTAMP1(category_group, name, scope, timestamp, arg_name, arg_val)` 展开 (trace-event-no-perfetto.h:208)。
pub fn trace_event_instant_with_timestamp1(
  category_group: &'static str,
  name: &'static str,
  scope: u8,
  timestamp: i64,
  arg_name: &'static str,
  arg_val: impl Into<TraceValue>,
) {
  emit_ts(
    TRACE_EVENT_PHASE_INSTANT,
    category_group,
    name,
    timestamp,
    TRACE_EVENT_FLAG_NONE | u32::from(scope),
    vec![TraceEventArg::new(arg_name, arg_val)],
  );
}

/// `TRACE_EVENT_BEGIN0(category_group, name)` 展开 (trace-event-no-perfetto.h:219)。
pub fn trace_event_begin0(category_group: &'static str, name: &'static str) {
  emit(
    TRACE_EVENT_PHASE_BEGIN,
    category_group,
    name,
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_BEGIN1(category_group, name, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:222)。
pub fn trace_event_begin1(
  category_group: &'static str,
  name: &'static str,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit(
    TRACE_EVENT_PHASE_BEGIN,
    category_group,
    name,
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_BEGIN2(category_group, name, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:225)。
pub fn trace_event_begin2(
  category_group: &'static str,
  name: &'static str,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit(
    TRACE_EVENT_PHASE_BEGIN,
    category_group,
    name,
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_BEGIN_WITH_FLAGS0(category_group, name, flags)` 展开 (trace-event-no-perfetto.h:230)。
pub fn trace_event_begin_with_flags0(category_group: &'static str, name: &'static str, flags: u32) {
  emit(TRACE_EVENT_PHASE_BEGIN, category_group, name, flags, vec![]);
}

/// `TRACE_EVENT_BEGIN_WITH_FLAGS1(category_group, name, flags, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:232)。
pub fn trace_event_begin_with_flags1(
  category_group: &'static str,
  name: &'static str,
  flags: u32,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit(
    TRACE_EVENT_PHASE_BEGIN,
    category_group,
    name,
    flags,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_COPY_BEGIN2(category_group, name, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:236)。
pub fn trace_event_copy_begin2(
  category_group: &'static str,
  name: &'static str,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit(
    TRACE_EVENT_PHASE_BEGIN,
    category_group,
    name,
    TRACE_EVENT_FLAG_COPY,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_BEGIN_WITH_ID_TID_AND_TIMESTAMP0(category_group, name, id, thread_id, timestamp)` 展开 (trace-event-no-perfetto.h:255)。
pub fn trace_event_begin_with_id_tid_and_timestamp0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_COPY_BEGIN_WITH_ID_TID_AND_TIMESTAMP0(category_group, name, id, thread_id, timestamp)` 展开 (trace-event-no-perfetto.h:260)。
pub fn trace_event_copy_begin_with_id_tid_and_timestamp0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_COPY,
    vec![],
  );
}

/// `TRACE_EVENT_COPY_BEGIN_WITH_ID_TID_AND_TIMESTAMP1(category_group, name, id, thread_id, timestamp, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:265)。
pub fn trace_event_copy_begin_with_id_tid_and_timestamp1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_COPY,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_COPY_BEGIN_WITH_ID_TID_AND_TIMESTAMP2(category_group, name, id, thread_id, timestamp, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:270)。
pub fn trace_event_copy_begin_with_id_tid_and_timestamp2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_COPY,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_END0(category_group, name)` 展开 (trace-event-no-perfetto.h:282)。
pub fn trace_event_end0(category_group: &'static str, name: &'static str) {
  emit(
    TRACE_EVENT_PHASE_END,
    category_group,
    name,
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_END1(category_group, name, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:285)。
pub fn trace_event_end1(
  category_group: &'static str,
  name: &'static str,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit(
    TRACE_EVENT_PHASE_END,
    category_group,
    name,
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_END2(category_group, name, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:288)。
pub fn trace_event_end2(
  category_group: &'static str,
  name: &'static str,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit(
    TRACE_EVENT_PHASE_END,
    category_group,
    name,
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_END_WITH_FLAGS0(category_group, name, flags)` 展开 (trace-event-no-perfetto.h:293)。
pub fn trace_event_end_with_flags0(category_group: &'static str, name: &'static str, flags: u32) {
  emit(TRACE_EVENT_PHASE_END, category_group, name, flags, vec![]);
}

/// `TRACE_EVENT_END_WITH_FLAGS1(category_group, name, flags, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:295)。
pub fn trace_event_end_with_flags1(
  category_group: &'static str,
  name: &'static str,
  flags: u32,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit(
    TRACE_EVENT_PHASE_END,
    category_group,
    name,
    flags,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_COPY_END2(category_group, name, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:299)。
pub fn trace_event_copy_end2(
  category_group: &'static str,
  name: &'static str,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit(
    TRACE_EVENT_PHASE_END,
    category_group,
    name,
    TRACE_EVENT_FLAG_COPY,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_MARK_WITH_TIMESTAMP0(category_group, name, timestamp)` 展开 (trace-event-no-perfetto.h:309)。
pub fn trace_event_mark_with_timestamp0(
  category_group: &'static str,
  name: &'static str,
  timestamp: i64,
) {
  emit_ts(
    TRACE_EVENT_PHASE_MARK,
    category_group,
    name,
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_MARK_WITH_TIMESTAMP1(category_group, name, timestamp, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:314)。
pub fn trace_event_mark_with_timestamp1(
  category_group: &'static str,
  name: &'static str,
  timestamp: i64,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_ts(
    TRACE_EVENT_PHASE_MARK,
    category_group,
    name,
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_MARK_WITH_TIMESTAMP2(category_group, name, timestamp, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:320)。
pub fn trace_event_mark_with_timestamp2(
  category_group: &'static str,
  name: &'static str,
  timestamp: i64,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_ts(
    TRACE_EVENT_PHASE_MARK,
    category_group,
    name,
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_COPY_MARK(category_group, name)` 展开 (trace-event-no-perfetto.h:326)。
pub fn trace_event_copy_mark(category_group: &'static str, name: &'static str) {
  emit(
    TRACE_EVENT_PHASE_MARK,
    category_group,
    name,
    TRACE_EVENT_FLAG_COPY,
    vec![],
  );
}

/// `TRACE_EVENT_COPY_MARK1(category_group, name, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:330)。
pub fn trace_event_copy_mark1(
  category_group: &'static str,
  name: &'static str,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit(
    TRACE_EVENT_PHASE_MARK,
    category_group,
    name,
    TRACE_EVENT_FLAG_COPY,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_COPY_MARK_WITH_TIMESTAMP(category_group, name, timestamp)` 展开 (trace-event-no-perfetto.h:334)。
pub fn trace_event_copy_mark_with_timestamp(
  category_group: &'static str,
  name: &'static str,
  timestamp: i64,
) {
  emit_ts(
    TRACE_EVENT_PHASE_MARK,
    category_group,
    name,
    timestamp,
    TRACE_EVENT_FLAG_COPY,
    vec![],
  );
}

/// `TRACE_EVENT_END_WITH_ID_TID_AND_TIMESTAMP0(category_group, name, id, thread_id, timestamp)` 展开 (trace-event-no-perfetto.h:348)。
pub fn trace_event_end_with_id_tid_and_timestamp0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_END,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_COPY_END_WITH_ID_TID_AND_TIMESTAMP0(category_group, name, id, thread_id, timestamp)` 展开 (trace-event-no-perfetto.h:353)。
pub fn trace_event_copy_end_with_id_tid_and_timestamp0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_END,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_COPY,
    vec![],
  );
}

/// `TRACE_EVENT_COPY_END_WITH_ID_TID_AND_TIMESTAMP1(category_group, name, id, thread_id, timestamp, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:358)。
pub fn trace_event_copy_end_with_id_tid_and_timestamp1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_END,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_COPY,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_COPY_END_WITH_ID_TID_AND_TIMESTAMP2(category_group, name, id, thread_id, timestamp, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:363)。
pub fn trace_event_copy_end_with_id_tid_and_timestamp2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_END,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_COPY,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_COUNTER1(category_group, name, value)` 展开 (trace-event-no-perfetto.h:375)。
pub fn trace_counter1(category_group: &'static str, name: &'static str, value: i32) {
  emit(
    TRACE_EVENT_PHASE_COUNTER,
    category_group,
    name,
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new("value", value)],
  );
}

/// `TRACE_COUNTER_WITH_FLAG1(category_group, name, flag, value)` 展开 (trace-event-no-perfetto.h:379)。
pub fn trace_counter_with_flag1(
  category_group: &'static str,
  name: &'static str,
  flag: u32,
  value: i32,
) {
  emit(
    TRACE_EVENT_PHASE_COUNTER,
    category_group,
    name,
    flag,
    vec![TraceEventArg::new("value", value)],
  );
}

/// `TRACE_COPY_COUNTER1(category_group, name, value)` 展开 (trace-event-no-perfetto.h:382)。
pub fn trace_copy_counter1(category_group: &'static str, name: &'static str, value: i32) {
  emit(
    TRACE_EVENT_PHASE_COUNTER,
    category_group,
    name,
    TRACE_EVENT_FLAG_COPY,
    vec![TraceEventArg::new("value", value)],
  );
}

/// `TRACE_COUNTER2(category_group, name, value1_name, value1_val, value2_name, value2_val)` 展开 (trace-event-no-perfetto.h:392)。
pub fn trace_counter2(
  category_group: &'static str,
  name: &'static str,
  value1_name: &'static str,
  value1_val: i32,
  value2_name: &'static str,
  value2_val: i32,
) {
  emit(
    TRACE_EVENT_PHASE_COUNTER,
    category_group,
    name,
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new(value1_name, value1_val),
      TraceEventArg::new(value2_name, value2_val),
    ],
  );
}

/// `TRACE_COPY_COUNTER2(category_group, name, value1_name, value1_val, value2_name, value2_val)` 展开 (trace-event-no-perfetto.h:398)。
pub fn trace_copy_counter2(
  category_group: &'static str,
  name: &'static str,
  value1_name: &'static str,
  value1_val: i32,
  value2_name: &'static str,
  value2_val: i32,
) {
  emit(
    TRACE_EVENT_PHASE_COUNTER,
    category_group,
    name,
    TRACE_EVENT_FLAG_COPY,
    vec![
      TraceEventArg::new(value1_name, value1_val),
      TraceEventArg::new(value2_name, value2_val),
    ],
  );
}

/// `TRACE_COUNTER_WITH_TIMESTAMP1(category_group, name, timestamp, value)` 展开 (trace-event-no-perfetto.h:409)。
pub fn trace_counter_with_timestamp1(
  category_group: &'static str,
  name: &'static str,
  timestamp: i64,
  value: i32,
) {
  emit_ts(
    TRACE_EVENT_PHASE_COUNTER,
    category_group,
    name,
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new("value", value)],
  );
}

/// `TRACE_COUNTER_WITH_TIMESTAMP2(category_group, name, timestamp, value1_name, value1_val, value2_name, value2_val)` 展开 (trace-event-no-perfetto.h:414)。
pub fn trace_counter_with_timestamp2(
  category_group: &'static str,
  name: &'static str,
  timestamp: i64,
  value1_name: &'static str,
  value1_val: i32,
  value2_name: &'static str,
  value2_val: i32,
) {
  emit_ts(
    TRACE_EVENT_PHASE_COUNTER,
    category_group,
    name,
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new(value1_name, value1_val),
      TraceEventArg::new(value2_name, value2_val),
    ],
  );
}

/// `TRACE_COUNTER_ID1(category_group, name, id, value)` 展开 (trace-event-no-perfetto.h:430)。
pub fn trace_counter_id1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  value: i32,
) {
  emit_id(
    TRACE_EVENT_PHASE_COUNTER,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new("value", value)],
  );
}

/// `TRACE_COPY_COUNTER_ID1(category_group, name, id, value)` 展开 (trace-event-no-perfetto.h:434)。
pub fn trace_copy_counter_id1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  value: i32,
) {
  emit_id(
    TRACE_EVENT_PHASE_COUNTER,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_COPY,
    vec![TraceEventArg::new("value", value)],
  );
}

/// `TRACE_COUNTER_ID2(category_group, name, id, value1_name, value1_val, value2_name, value2_val)` 展开 (trace-event-no-perfetto.h:448)。
pub fn trace_counter_id2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  value1_name: &'static str,
  value1_val: i32,
  value2_name: &'static str,
  value2_val: i32,
) {
  emit_id(
    TRACE_EVENT_PHASE_COUNTER,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new(value1_name, value1_val),
      TraceEventArg::new(value2_name, value2_val),
    ],
  );
}

/// `TRACE_COPY_COUNTER_ID2(category_group, name, id, value1_name, value1_val, value2_name, value2_val)` 展开 (trace-event-no-perfetto.h:454)。
pub fn trace_copy_counter_id2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  value1_name: &'static str,
  value1_val: i32,
  value2_name: &'static str,
  value2_val: i32,
) {
  emit_id(
    TRACE_EVENT_PHASE_COUNTER,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_COPY,
    vec![
      TraceEventArg::new(value1_name, value1_val),
      TraceEventArg::new(value2_name, value2_val),
    ],
  );
}

/// `TRACE_EVENT_SAMPLE_WITH_ID1(category_group, name, id, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:461)。
pub fn trace_event_sample_with_id1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_SAMPLE,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_ASYNC_BEGIN0(category_group, name, id)` 展开 (trace-event-no-perfetto.h:495)。
pub fn trace_event_async_begin0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_ASYNC_BEGIN1(category_group, name, id, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:499)。
pub fn trace_event_async_begin1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_ASYNC_BEGIN2(category_group, name, id, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:504)。
pub fn trace_event_async_begin2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_COPY_ASYNC_BEGIN0(category_group, name, id)` 展开 (trace-event-no-perfetto.h:509)。
pub fn trace_event_copy_async_begin0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_COPY,
    vec![],
  );
}

/// `TRACE_EVENT_COPY_ASYNC_BEGIN1(category_group, name, id, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:513)。
pub fn trace_event_copy_async_begin1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_COPY,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_COPY_ASYNC_BEGIN2(category_group, name, id, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:518)。
pub fn trace_event_copy_async_begin2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_COPY,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_ASYNC_BEGIN_WITH_TIMESTAMP0(category_group, name, id, timestamp)` 展开 (trace-event-no-perfetto.h:526)。
pub fn trace_event_async_begin_with_timestamp0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_ASYNC_BEGIN_WITH_TIMESTAMP1(category_group, name, id, timestamp, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:531)。
pub fn trace_event_async_begin_with_timestamp1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_ASYNC_BEGIN_WITH_TIMESTAMP2(category_group, name, id, timestamp, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:537)。
pub fn trace_event_async_begin_with_timestamp2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_COPY_ASYNC_BEGIN_WITH_TIMESTAMP0(category_group, name, id, timestamp)` 展开 (trace-event-no-perfetto.h:544)。
pub fn trace_event_copy_async_begin_with_timestamp0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_COPY,
    vec![],
  );
}

/// `TRACE_EVENT_ASYNC_STEP_INTO0(category_group, name, id, step)` 展开 (trace-event-no-perfetto.h:556)。
pub fn trace_event_async_step_into0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  step: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_STEP_INTO,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new("step", step)],
  );
}

/// `TRACE_EVENT_ASYNC_STEP_INTO1(category_group, name, id, step, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:560)。
pub fn trace_event_async_step_into1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  step: impl Into<TraceValue>,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_STEP_INTO,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new("step", step),
      TraceEventArg::new(arg1_name, arg1_val),
    ],
  );
}

/// `TRACE_EVENT_ASYNC_STEP_INTO_WITH_TIMESTAMP0(category_group, name, id, step, timestamp)` 展开 (trace-event-no-perfetto.h:568)。
pub fn trace_event_async_step_into_with_timestamp0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  step: impl Into<TraceValue>,
  timestamp: i64,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_STEP_INTO,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new("step", step)],
  );
}

/// `TRACE_EVENT_ASYNC_STEP_PAST0(category_group, name, id, step)` 展开 (trace-event-no-perfetto.h:581)。
pub fn trace_event_async_step_past0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  step: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_STEP_PAST,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new("step", step)],
  );
}

/// `TRACE_EVENT_ASYNC_STEP_PAST1(category_group, name, id, step, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:585)。
pub fn trace_event_async_step_past1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  step: impl Into<TraceValue>,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_STEP_PAST,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new("step", step),
      TraceEventArg::new(arg1_name, arg1_val),
    ],
  );
}

/// `TRACE_EVENT_ASYNC_END0(category_group, name, id)` 展开 (trace-event-no-perfetto.h:593)。
pub fn trace_event_async_end0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_END,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_ASYNC_END1(category_group, name, id, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:597)。
pub fn trace_event_async_end1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_END,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_ASYNC_END2(category_group, name, id, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:601)。
pub fn trace_event_async_end2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_END,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_COPY_ASYNC_END0(category_group, name, id)` 展开 (trace-event-no-perfetto.h:606)。
pub fn trace_event_copy_async_end0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_END,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_COPY,
    vec![],
  );
}

/// `TRACE_EVENT_COPY_ASYNC_END1(category_group, name, id, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:610)。
pub fn trace_event_copy_async_end1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_END,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_COPY,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_COPY_ASYNC_END2(category_group, name, id, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:615)。
pub fn trace_event_copy_async_end2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id(
    TRACE_EVENT_PHASE_ASYNC_END,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_COPY,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_ASYNC_END_WITH_TIMESTAMP0(category_group, name, id, timestamp)` 展开 (trace-event-no-perfetto.h:622)。
pub fn trace_event_async_end_with_timestamp0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_END,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_ASYNC_END_WITH_TIMESTAMP1(category_group, name, id, timestamp, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:627)。
pub fn trace_event_async_end_with_timestamp1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_END,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_ASYNC_END_WITH_TIMESTAMP2(category_group, name, id, timestamp, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:633)。
pub fn trace_event_async_end_with_timestamp2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_END,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_COPY_ASYNC_END_WITH_TIMESTAMP0(category_group, name, id, timestamp)` 展开 (trace-event-no-perfetto.h:640)。
pub fn trace_event_copy_async_end_with_timestamp0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_ASYNC_END,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_COPY,
    vec![],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_BEGIN0(category_group, name, id)` 展开 (trace-event-no-perfetto.h:669)。
pub fn trace_event_nestable_async_begin0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_BEGIN1(category_group, name, id, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:673)。
pub fn trace_event_nestable_async_begin1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_BEGIN2(category_group, name, id, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:678)。
pub fn trace_event_nestable_async_begin2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_BEGIN_WITH_FLAGS0(category_group, name, id, flags)` 展开 (trace-event-no-perfetto.h:683)。
pub fn trace_event_nestable_async_begin_with_flags0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  flags: u32,
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    flags,
    vec![],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_END0(category_group, name, id)` 展开 (trace-event-no-perfetto.h:690)。
pub fn trace_event_nestable_async_end0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_END,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_END1(category_group, name, id, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:696)。
pub fn trace_event_nestable_async_end1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_END,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_END2(category_group, name, id, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:701)。
pub fn trace_event_nestable_async_end2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_END,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_END_WITH_FLAGS0(category_group, name, id, flags)` 展开 (trace-event-no-perfetto.h:706)。
pub fn trace_event_nestable_async_end_with_flags0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  flags: u32,
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_END,
    category_group,
    name,
    id.into(),
    flags,
    vec![],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_INSTANT0(category_group, name, id)` 展开 (trace-event-no-perfetto.h:714)。
pub fn trace_event_nestable_async_instant0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_INSTANT,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_INSTANT1(category_group, name, id, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:719)。
pub fn trace_event_nestable_async_instant1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_INSTANT,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_INSTANT2(category_group, name, id, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:725)。
pub fn trace_event_nestable_async_instant2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_INSTANT,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN_WITH_TTS2(category_group, name, id, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:731)。
pub fn trace_event_copy_nestable_async_begin_with_tts2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_ASYNC_TTS | TRACE_EVENT_FLAG_COPY,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_COPY_NESTABLE_ASYNC_END_WITH_TTS2(category_group, name, id, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:737)。
pub fn trace_event_copy_nestable_async_end_with_tts2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_END,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_ASYNC_TTS | TRACE_EVENT_FLAG_COPY,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP0(category_group, name, id, timestamp)` 展开 (trace-event-no-perfetto.h:746)。
pub fn trace_event_nestable_async_begin_with_timestamp0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP1(category_group, name, id, timestamp, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:751)。
pub fn trace_event_nestable_async_begin_with_timestamp1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP_AND_FLAGS0(category_group, name, id, timestamp, flags)` 展开 (trace-event-no-perfetto.h:757)。
pub fn trace_event_nestable_async_begin_with_timestamp_and_flags0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
  flags: u32,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    timestamp,
    flags,
    vec![],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_END_WITH_TIMESTAMP0(category_group, name, id, timestamp)` 展开 (trace-event-no-perfetto.h:762)。
pub fn trace_event_nestable_async_end_with_timestamp0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_END,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_END_WITH_TIMESTAMP1(category_group, name, id, timestamp, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:767)。
pub fn trace_event_nestable_async_end_with_timestamp1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_END,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_END_WITH_TIMESTAMP2(category_group, name, id, timestamp, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:773)。
pub fn trace_event_nestable_async_end_with_timestamp2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_END,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_END_WITH_TIMESTAMP_AND_FLAGS0(category_group, name, id, timestamp, flags)` 展开 (trace-event-no-perfetto.h:780)。
pub fn trace_event_nestable_async_end_with_timestamp_and_flags0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
  flags: u32,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_END,
    category_group,
    name,
    id.into(),
    timestamp,
    flags,
    vec![],
  );
}

/// `TRACE_EVENT_NESTABLE_ASYNC_INSTANT_WITH_TIMESTAMP0(category_group, name, id, timestamp)` 展开 (trace-event-no-perfetto.h:785)。
pub fn trace_event_nestable_async_instant_with_timestamp0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_INSTANT,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN0(category_group, name, id)` 展开 (trace-event-no-perfetto.h:790)。
pub fn trace_event_copy_nestable_async_begin0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_COPY,
    vec![],
  );
}

/// `TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN1(category_group, name, id, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:794)。
pub fn trace_event_copy_nestable_async_begin1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_COPY,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN2(category_group, name, id, arg1_name, arg1_val, arg2_name, arg2_val)` 展开 (trace-event-no-perfetto.h:799)。
pub fn trace_event_copy_nestable_async_begin2(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1: (&'static str, impl Into<TraceValue>),
  arg2: (&'static str, impl Into<TraceValue>),
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_COPY,
    vec![
      TraceEventArg::new(arg1.0, arg1.1),
      TraceEventArg::new(arg2.0, arg2.1),
    ],
  );
}

/// `TRACE_EVENT_COPY_NESTABLE_ASYNC_END0(category_group, name, id)` 展开 (trace-event-no-perfetto.h:804)。
pub fn trace_event_copy_nestable_async_end0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_END,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_COPY,
    vec![],
  );
}

/// `TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP0(category_group, name, id, timestamp)` 展开 (trace-event-no-perfetto.h:808)。
pub fn trace_event_copy_nestable_async_begin_with_timestamp0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_COPY,
    vec![],
  );
}

/// `TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP1(category_group, name, id, timestamp, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:813)。
pub fn trace_event_copy_nestable_async_begin_with_timestamp1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_COPY,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_COPY_NESTABLE_ASYNC_END_WITH_TIMESTAMP0(category_group, name, id, timestamp)` 展开 (trace-event-no-perfetto.h:819)。
pub fn trace_event_copy_nestable_async_end_with_timestamp0(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  timestamp: i64,
) {
  emit_id_ts(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_END,
    category_group,
    name,
    id.into(),
    timestamp,
    TRACE_EVENT_FLAG_COPY,
    vec![],
  );
}

/// `TRACE_EVENT_COPY_NESTABLE_ASYNC_END1(category_group, name, id, arg1_name, arg1_val)` 展开 (trace-event-no-perfetto.h:824)。
pub fn trace_event_copy_nestable_async_end1(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  arg1_name: &'static str,
  arg1_val: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_NESTABLE_ASYNC_END,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_COPY,
    vec![TraceEventArg::new(arg1_name, arg1_val)],
  );
}

/// `TRACE_EVENT_CLOCK_SYNC_RECEIVER(sync_id)` 展开 (trace-event-no-perfetto.h:836)。
pub fn trace_event_clock_sync_receiver(sync_id: impl Into<TraceValue>) {
  emit(
    TRACE_EVENT_PHASE_CLOCK_SYNC,
    "__metadata",
    "clock_sync",
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new("sync_id", sync_id)],
  );
}

/// `TRACE_EVENT_CLOCK_SYNC_ISSUER(sync_id, issue_ts, issue_end_ts)` 展开 (trace-event-no-perfetto.h:840)。
pub fn trace_event_clock_sync_issuer(
  sync_id: impl Into<TraceValue>,
  issue_ts: i64,
  issue_end_ts: i64,
) {
  emit_ts(
    TRACE_EVENT_PHASE_CLOCK_SYNC,
    "__metadata",
    "clock_sync",
    issue_end_ts,
    TRACE_EVENT_FLAG_NONE,
    vec![
      TraceEventArg::new("sync_id", sync_id),
      TraceEventArg::new("issue_ts", issue_ts),
    ],
  );
}

/// `TRACE_EVENT_OBJECT_CREATED_WITH_ID(category_group, name, id)` 展开 (trace-event-no-perfetto.h:848)。
pub fn trace_event_object_created_with_id(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
) {
  emit_id(
    TRACE_EVENT_PHASE_CREATE_OBJECT,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_OBJECT_SNAPSHOT_WITH_ID(category_group, name, id, snapshot)` 展开 (trace-event-no-perfetto.h:853)。
pub fn trace_event_object_snapshot_with_id(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
  snapshot: impl Into<TraceValue>,
) {
  emit_id(
    TRACE_EVENT_PHASE_SNAPSHOT_OBJECT,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![TraceEventArg::new("snapshot", snapshot)],
  );
}

/// `TRACE_EVENT_OBJECT_DELETED_WITH_ID(category_group, name, id)` 展开 (trace-event-no-perfetto.h:859)。
pub fn trace_event_object_deleted_with_id(
  category_group: &'static str,
  name: &'static str,
  id: impl Into<TraceId>,
) {
  emit_id(
    TRACE_EVENT_PHASE_DELETE_OBJECT,
    category_group,
    name,
    id.into(),
    TRACE_EVENT_FLAG_NONE,
    vec![],
  );
}

/// `TRACE_EVENT_CATEGORY_ENABLED(category)` 展开 (trace-event-no-perfetto.h:865)。
pub fn trace_event_category_enabled(category_group: &'static str) -> bool {
  category_enabled(category_group)
}

/// `TRACE_EVENT_CATEGORY_GROUP_ENABLED(category_group, ret)` 展开 (trace-event-no-perfetto.h:871)。
pub fn trace_event_category_group_enabled(category_group: &'static str) -> bool {
  category_enabled(category_group)
}

/// `TRACE_TIME_TICKS_NOW()` 展开 (trace-event-no-perfetto.h:898)。
pub fn trace_time_ticks_now() -> i64 {
  ticks_now_ns()
}

/// `TRACE_TIME_NOW()` 展开 (trace-event-no-perfetto.h:902)。
pub fn trace_time_now() -> i64 {
  time_now_ns()
}

// ---- perfetto typed 宏族展开 (trace-event-no-perfetto.h:26-32) ----
//
// 原版 5 个 typed 宏把实参整体擦除 → `trace_event_internal::Ignore(...)`
// (trace-event-no-perfetto.h:14-18, 空变参函数) — no-perfetto 构建下即 no-op。
// C 变参在 Rust 侧无可表达的匿名擦除形态, 机械展开为 (category, name) 两参
// no-op, 其余实参由原版宏本体擦除, 调用方 (L4+ 引擎代码) 适配时不传。
/// `TRACE_EVENT_BEGIN(...)` 展开 (trace-event-no-perfetto.h:26)。
#[inline]
pub fn trace_event_begin(category: &'static str, name: &'static str) {
  let _ = (category, name); // Ignore(category, name)
}

/// `TRACE_EVENT_END(...)` 展开 (trace-event-no-perfetto.h:28)。
#[inline]
pub fn trace_event_end(category: &'static str) {
  let _ = category; // Ignore(category)
}

/// `TRACE_EVENT(...)` 展开 (trace-event-no-perfetto.h:29)。
#[inline]
pub fn trace_event(category: &'static str, name: &'static str) {
  let _ = (category, name); // Ignore(category, name)
}

/// `TRACE_EVENT_INSTANT(...)` 展开 (trace-event-no-perfetto.h:30)。
#[inline]
pub fn trace_event_instant(category: &'static str, name: &'static str) {
  let _ = (category, name); // Ignore(category, name)
}

/// `TRACE_COUNTER(...)` 展开 (trace-event-no-perfetto.h:32)。
#[inline]
pub fn trace_counter(category: &'static str, name: &'static str) {
  let _ = (category, name); // Ignore(category, name)
}

// ---- 登记不展开 (2 项, 实测本 checkout 无可展开语义) ----
//
// 1. `TRACE_EVENT_METADATA1(category_group, name, arg1_name, arg1_val)`
//    (trace-event-no-perfetto.h:832-834) → `INTERNAL_TRACE_EVENT_METADATA_ADD`;
//    该 INTERNAL 宏在 include/ 与 src/ 全 checkout 无定义 (仅此一处使用),
//    no-perfetto 分支缺失, 被调用即编译失败 — 无可转写语义。
// 2. `TRACE_EVENT_IS_NEW_TRACE(ret)` (trace-event-no-perfetto.h:882-892) →
//    `TRACE_EVENT_API_GET_NUM_TRACES_RECORDED` = `UNIMPLEMENTED()`
//    (trace-event.h:86-87), no-perfetto 构建下调用即 abort — 无可转写语义。
//
// ---- perfetto stub 类型登记 (trace-event-no-perfetto.h:42-109) ----
//
// `v8_stub::EventContext / StaticString / DynamicString / Track / ThreadTrack /
// NamedTrack / CounterTrack / Flow / TerminatingFlow`: 全部方法体为空 (typed
// 宏的 C++ ABI 占位), 无可观察行为; typed 宏已展开为上述 no-op 函数,
// 空壳类型不生成 (零死代码), 此处登记备查。
