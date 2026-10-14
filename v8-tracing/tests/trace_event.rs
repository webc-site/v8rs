//! trace-event 宏展开登记面测试: 以录制 controller 验证宏函数的
//! phase/flags/固定实参展开 (X-macro 展开正确性 = 本 crate 核心)。
//! 依赖 nextest 进程级隔离 (全局 controller 安装点每测试独占)。

use std::sync::Arc;
mod common;

use std::sync::atomic::Ordering;

use aok::{OK, Void};
use v8_logging::RUNTIME_STATS;
use v8_tracing::{
  NoopController, TRACE_EVENT_FLAG_ASYNC_TTS, TRACE_EVENT_FLAG_COPY, TRACE_EVENT_FLAG_HAS_ID,
  TRACE_EVENT_FLAG_NONE, TRACE_EVENT_PHASE_ASYNC_STEP_INTO, TRACE_EVENT_PHASE_BEGIN,
  TRACE_EVENT_PHASE_CLOCK_SYNC, TRACE_EVENT_PHASE_COUNTER, TRACE_EVENT_PHASE_DELETE_OBJECT,
  TRACE_EVENT_PHASE_MARK, TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN, TRACE_EVENT_SCOPE_PROCESS,
  TRACE_VALUE_TYPE_BOOL, TRACE_VALUE_TYPE_CONVERTABLE, TRACE_VALUE_TYPE_COPY_STRING,
  TRACE_VALUE_TYPE_DOUBLE, TRACE_VALUE_TYPE_INT, TRACE_VALUE_TYPE_POINTER, TRACE_VALUE_TYPE_STRING,
  TRACE_VALUE_TYPE_UINT, TraceId, TraceValue, TracedValue, TracingCategoryObserver,
  add_trace_state_observer, trace_counter_id2, trace_counter1, trace_event_async_step_into1,
  trace_event_category_enabled, trace_event_clock_sync_receiver,
  trace_event_copy_nestable_async_begin_with_tts2, trace_event_instant0,
  trace_event_mark_with_timestamp0, trace_event_object_deleted_with_id, trace_event_with_flow1,
  trace_event0, trace_time_ticks_now, tracing_controller,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 计数器宏: phase C + 固定 "value" 实参 + i32 值 (trace-event-no-perfetto.h:375-378)。
#[test]
fn trace_event_macros_counter_expansion() -> Void {
  let rec = common::Recorder::install(&["cat"]);
  trace_counter1("cat", "c1", 2014);
  let evs = rec.events.lock().unwrap();
  assert_eq!(evs.len(), 1);
  let ev = &evs[0];
  assert_eq!(ev.phase, TRACE_EVENT_PHASE_COUNTER);
  assert_eq!(ev.flags, TRACE_EVENT_FLAG_NONE);
  assert_eq!(ev.args.len(), 1);
  assert_eq!(ev.args[0].name, "value");
  assert_eq!(ev.args[0].value.type_tag(), TRACE_VALUE_TYPE_INT);
  assert!(matches!(ev.args[0].value, TraceValue::Int(2014)));
  OK
}

/// INSTANT 宏 scope 并入 flags (NONE | TRACE_EVENT_SCOPE_PROCESS)。
#[test]
fn trace_event_macros_instant_scope_flags() -> Void {
  let rec = common::Recorder::install(&["cat"]);
  trace_event_instant0("cat", "i0", TRACE_EVENT_SCOPE_PROCESS);
  let evs = rec.events.lock().unwrap();
  assert_eq!(evs[0].phase, b'I');
  assert_eq!(
    evs[0].flags,
    TRACE_EVENT_FLAG_NONE | u32::from(TRACE_EVENT_SCOPE_PROCESS)
  );
  OK
}

/// WITH_ID 宏: HAS_ID 置位 + id/scope 传递; 未启用类别零写入。
#[test]
fn trace_event_macros_with_id_and_disabled() -> Void {
  let rec = common::Recorder::install(&["on"]);
  trace_counter_id2("on", "c2", 7u64, "a", 1, "b", 2);
  trace_counter1("off", "x", 1); // 未启用类别 → 早退
  let evs = rec.events.lock().unwrap();
  assert_eq!(evs.len(), 1);
  assert_eq!(
    evs[0].flags & TRACE_EVENT_FLAG_HAS_ID,
    TRACE_EVENT_FLAG_HAS_ID
  );
  assert_eq!(evs[0].id, 7);
  assert_eq!(evs[0].scope, None);
  assert_eq!(evs[0].args.len(), 2);
  OK
}

/// ASYNC_STEP_INTO 固定 "step" 实参 (trace-event-no-perfetto.h:560-564)。
#[test]
fn trace_event_macros_step_into_fixed_arg() -> Void {
  let rec = common::Recorder::install(&["cat"]);
  trace_event_async_step_into1("cat", "s", 5u32, "s1", "phase1", 9i64);
  let evs = rec.events.lock().unwrap();
  assert_eq!(evs[0].phase, TRACE_EVENT_PHASE_ASYNC_STEP_INTO);
  assert_eq!(evs[0].args[0].name, "step");
  assert_eq!(evs[0].args[1].name, "phase1");
  OK
}

/// CLOCK_SYNC 固定 "__metadata"/"clock_sync" (trace-event-no-perfetto.h:836-839)。
#[test]
fn trace_event_macros_clock_sync_fixed_names() -> Void {
  let rec = common::Recorder::install(&["__metadata"]);
  trace_event_clock_sync_receiver(42i64);
  let evs = rec.events.lock().unwrap();
  assert_eq!(evs[0].phase, TRACE_EVENT_PHASE_CLOCK_SYNC);
  assert_eq!(evs[0].category_group, "__metadata");
  assert_eq!(evs[0].name, "clock_sync");
  OK
}

/// WITH_TTS: ASYNC_TTS|COPY 组合 + HAS_ID (trace-event-no-perfetto.h:731-736)。
#[test]
fn trace_event_macros_tts_flags() -> Void {
  let rec = common::Recorder::install(&["cat"]);
  trace_event_copy_nestable_async_begin_with_tts2("cat", "t", 1u16, ("a", 1i32), ("b", 2i32));
  let evs = rec.events.lock().unwrap();
  assert_eq!(evs[0].phase, TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN);
  assert_eq!(
    evs[0].flags,
    TRACE_EVENT_FLAG_ASYNC_TTS | TRACE_EVENT_FLAG_COPY | TRACE_EVENT_FLAG_HAS_ID
  );
  OK
}

/// OBJECT_DELETED_WITH_ID → phase D; MARK_WITH_TIMESTAMP 走 with_timestamp 路径。
#[test]
fn trace_event_macros_object_and_timestamp() -> Void {
  let rec = common::Recorder::install(&["cat"]);
  trace_event_object_deleted_with_id("cat", "o", 9usize);
  trace_event_mark_with_timestamp0("cat", "m", 12345);
  let evs = rec.events.lock().unwrap();
  assert_eq!(evs[0].phase, TRACE_EVENT_PHASE_DELETE_OBJECT);
  assert_eq!(evs[0].id, 9);
  assert_eq!(evs[1].phase, TRACE_EVENT_PHASE_MARK);
  assert_eq!(evs[1].name, "m");
  OK
}

/// SCOPED: COMPLETE 相位 + Drop 补 UpdateTraceEventDuration (trace-event.h:195-207)。
#[test]
fn trace_event_scoped_guard_duration() -> Void {
  let rec = common::Recorder::install(&["cat"]);
  {
    let _g = trace_event0("cat", "scope");
    assert_eq!(rec.events.lock().unwrap().len(), 1);
    assert_eq!(rec.events.lock().unwrap()[0].phase, b'X');
    assert!(rec.durations.lock().unwrap().is_empty());
  }
  let durs = rec.durations.lock().unwrap();
  assert_eq!(durs.len(), 1);
  // Drop 透传 drop 时重读的启用字节 (Recorder 返回 RECORDING|CALLBACK) + 事件句柄
  assert_eq!(
    durs[0],
    (
      "scope".to_string(),
      v8_tracing::ENABLED_FOR_RECORDING | v8_tracing::ENABLED_FOR_EVENT_CALLBACK,
      1
    )
  );
  OK
}

/// 未启用类别的 SCOPED: 零事件 + Drop 零时长 (p_data_ = nullptr 语义)。
#[test]
fn trace_event_scoped_disabled_noop() -> Void {
  let rec = common::Recorder::install(&["on"]);
  {
    let _g = v8_tracing::trace_event1("off", "s", "a", 1);
  }
  assert!(rec.events.lock().unwrap().is_empty());
  assert!(rec.durations.lock().unwrap().is_empty());
  OK
}

/// WITH_FLOW: bind_id 归一传递 + flow flags 原样 (trace-event.h:209-225)。
#[test]
fn trace_event_with_flow_bind_id() -> Void {
  let rec = common::Recorder::install(&["cat"]);
  let _g = trace_event_with_flow1(
    "cat",
    "f",
    0xdead_u64,
    v8_tracing::TRACE_EVENT_FLAG_FLOW_IN,
    "k",
    "v",
  );
  let evs = rec.events.lock().unwrap();
  assert_eq!(evs[0].bind_id, 0xdead);
  assert_eq!(evs[0].flags, v8_tracing::TRACE_EVENT_FLAG_FLOW_IN);
  OK
}

/// Convertable 实参: TracedValue 经 Box<dyn> 装入事件 (SetTraceValue CONVERTABLE 路径)。
#[test]
fn trace_event_convertable_arg() -> Void {
  let rec = common::Recorder::install(&["cat"]);
  let mut tv = TracedValue::create();
  tv.set_integer("i", 3);
  let payload: Box<dyn v8_tracing::ConvertableToTraceFormat> = Box::new(*tv);
  let _g = v8_tracing::trace_event1("cat", "e", "payload", payload);
  let evs = rec.events.lock().unwrap();
  let arg = &evs[0].args[0];
  assert_eq!(arg.name, "payload");
  // 录制件把 Convertable 序列化为 JSON 串占位
  let expected = TraceValue::from(String::from("{\"i\":3}"));
  assert!(
    matches!((&arg.value, &expected), (TraceValue::CopyStr(a), TraceValue::CopyStr(b)) if a == b)
  );
  OK
}

/// TraceValue 类型标签全表 (SetTraceValue, trace-event.h:431-468)。
#[test]
fn trace_value_type_tags() -> Void {
  assert_eq!(TraceValue::from(true).type_tag(), TRACE_VALUE_TYPE_BOOL);
  assert_eq!(TraceValue::from(3u8).type_tag(), TRACE_VALUE_TYPE_UINT);
  assert_eq!(TraceValue::from(3i32).type_tag(), TRACE_VALUE_TYPE_INT);
  assert_eq!(TraceValue::from(2.5f64).type_tag(), TRACE_VALUE_TYPE_DOUBLE);
  assert_eq!(
    TraceValue::pointer(0x10).type_tag(),
    TRACE_VALUE_TYPE_POINTER
  );
  assert_eq!(TraceValue::from("s").type_tag(), TRACE_VALUE_TYPE_STRING);
  assert_eq!(
    TraceValue::from(String::from("s")).type_tag(),
    TRACE_VALUE_TYPE_COPY_STRING
  );
  let tv: Box<dyn v8_tracing::ConvertableToTraceFormat> = Box::new(*TracedValue::create());
  assert_eq!(
    TraceValue::from(tv).type_tag(),
    TRACE_VALUE_TYPE_CONVERTABLE
  );
  OK
}

/// TraceID 10 重载语义: 符号扩展 + WithScope (trace-event.h:332-363)。
#[test]
fn trace_id_conversions() -> Void {
  assert_eq!(TraceId::from(-1i32).raw_id, u64::MAX); // 符号扩展
  assert_eq!(TraceId::from(7u16).raw_id, 7);
  let scoped = TraceId::with_scope("scope", -2i8);
  assert_eq!(scoped.scope, Some("scope"));
  assert_eq!(scoped.raw_id, 0xFFFF_FFFF_FFFF_FFFE);
  OK
}

/// CATEGORY_ENABLED 宏查询 (trace-event-no-perfetto.h:865-879)。
#[test]
fn trace_event_category_enabled_query() -> Void {
  let _rec = common::Recorder::install(&["on"]);
  assert!(trace_event_category_enabled("on"));
  assert!(!trace_event_category_enabled("off"));
  OK
}

/// 未安装 controller → NOOP 默认行为 (v8-platform.h inline 空实现)。
#[test]
fn noop_controller_default() -> Void {
  use v8_tracing::TracingController;
  let ctrl = tracing_controller();
  trace_counter1("any", "n", 1); // 零事件零崩溃
  assert_eq!(ctrl.get_category_group_enabled("any"), 0);
  // NoopController 独立实例行为 (默认 trait 方法)
  let noop = NoopController;
  assert_eq!(noop.get_category_group_enabled("x"), 0);
  assert_eq!(noop.add_trace_event(&dummy_event()), 0);
  noop.update_trace_event_duration(0, "n", 0);
  OK
}

fn dummy_event() -> v8_tracing::TraceEvent {
  v8_tracing::TraceEvent {
    phase: TRACE_EVENT_PHASE_BEGIN,
    category_group: "c",
    name: "n",
    scope: None,
    id: 0,
    bind_id: 0,
    flags: TRACE_EVENT_FLAG_NONE,
    args: Vec::new(),
  }
}

/// TracingCategoryObserver: 挂接/启停位翻转/摘除 (tracing-category-observer.cc)。
#[test]
fn category_observer_lifecycle() -> Void {
  let rec = common::Recorder::install(&["disabled-by-default-v8.runtime_stats"]);
  RUNTIME_STATS.store(0, Ordering::Relaxed);
  TracingCategoryObserver::set_up().expect("setUp");
  assert_eq!(rec.observers.lock().unwrap().len(), 1);
  // 触发 OnTraceEnabled (原版由 controller 在 tracing 启动时回调)
  rec.observers.lock().unwrap()[0].on_trace_enabled();
  assert_eq!(
    RUNTIME_STATS.load(Ordering::Relaxed),
    v8_tracing::ENABLED_BY_TRACING
  );
  rec.observers.lock().unwrap()[0].on_trace_disabled();
  assert_eq!(RUNTIME_STATS.load(Ordering::Relaxed), 0);
  TracingCategoryObserver::tear_down();
  assert!(rec.observers.lock().unwrap().is_empty());
  // TearDown 后可重挂
  assert!(TracingCategoryObserver::set_up().is_ok());
  TracingCategoryObserver::tear_down();
  OK
}

/// 未启用 runtime_stats 类别 → 启用回调不置位。
#[test]
fn category_observer_category_gated() -> Void {
  let rec = common::Recorder::install(&["other"]);
  RUNTIME_STATS.store(0, Ordering::Relaxed);
  TracingCategoryObserver::set_up().expect("setUp");
  rec.observers.lock().unwrap()[0].on_trace_enabled();
  assert_eq!(RUNTIME_STATS.load(Ordering::Relaxed), 0);
  TracingCategoryObserver::tear_down();
  OK
}

/// TRACE_TIME_TICKS_NOW: 单调不减。
#[test]
fn trace_time_ticks_monotonic() -> Void {
  let a = trace_time_ticks_now();
  let b = trace_time_ticks_now();
  assert!(b >= a);
  OK
}

/// 观察者便捷转发 (v8-platform.h:461/467)。
#[test]
fn observer_forwarding() -> Void {
  let rec = common::Recorder::install(&[]);
  struct Marker;
  impl v8_tracing::TraceStateObserver for Marker {
    fn on_trace_enabled(&self) {}
    fn on_trace_disabled(&self) {}
  }
  let obs: Arc<dyn v8_tracing::TraceStateObserver> = Arc::new(Marker);
  add_trace_state_observer(Arc::clone(&obs));
  assert_eq!(rec.observers.lock().unwrap().len(), 1);
  v8_tracing::remove_trace_state_observer(&obs);
  assert!(rec.observers.lock().unwrap().is_empty());
  OK
}
