//! `v8::internal` logging 层的 Rust 移植crate。
//!
//! 对应源目录 `v8/src/logging/`(实际现存文件):
//! `log.h/.cc`(`code_events`/`events` 可移植部分)、`log-file.h/.cc`、
//! `counters.h/.cc`、`counters-definitions.h`、`counters-scopes.h`、
//! `runtime-call-stats.h/.cc`、`runtime-call-stats-scope.h`、
//! `tracing-flags.h/.cc`。
//! `logutils`(DebugServer)在现存源码树中不存在;推迟/映射登记见
//! `readme/zh.md`。日志门面统一走 `log` crate(`info!/debug!/trace!/warn!`)。
//!
//! 导出面(AGENTS.md:禁 `pub mod`,以 `pub use` 摊平):

mod code_events;
mod counters;
mod error;
mod events;
mod flags_lite;
mod local_logger;
mod log;
mod log_file;
mod log_inl;
mod metrics;
mod rcs_names;
mod runtime_call_stats;
mod time_base;
mod tracing_flags;

// --- code_events.rs(code-events.h 可移植部分)---
pub use code_events::{
  CODE_TAG_NAMES, CodeEvent, CodeEventType, CodeTag, LOG_EVENT_NAMES, code_tag_name, event_name,
  get_code_event_type_for_tag,
};
// `define_counters!` 生成的聚合体(全部访问器随结构导出):
pub use counters::Counters;
// --- counters.rs(counters.h/.cc + counters-definitions.h + counters-scopes.h)---
pub use counters::{
  AggregatableHistogramTimer, AggregatedHistogramTimerScope, AggregatedMemoryHistogram,
  AggregatingHistogramTimerScope, AnyUpdate, CounterLookupCallback, CreateHistogramCallback,
  DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS, DelayedCounterUpdates, GB, GetHistogramFn,
  GetStatsCounterFn, GetTimedHistogramFn, Histogram, HistogramHandle,
  K_GARBAGE_COLLECTION_REASON_MAX_VALUE, K_MAX_CPP_HEAP_POINTERS, K_MAX_EXTERNAL_POINTERS,
  K_MAX_JS_DISPATCH_ENTRIES, K_MAX_TRUSTED_POINTERS, KB, LazyTimedHistogramScope,
  LegacyMemoryHistogram, MB, MICROSECOND, MILLISECOND, NestedTimedHistogram,
  NestedTimedHistogramScope, OptionalTimedHistogramScope, OptionalTimedHistogramScopeMode,
  PauseNestedTimedHistogramScope, PercentageHistogram, SampleSink, StatsCounter, StatsTable,
  TimedHistogram, TimedHistogramResolution, TimedHistogramScope,
};
// --- error.rs ---
pub use error::{Error, Result};
// --- events.rs(log.h/.cc 可移植事件函数)---
pub use events::{
  K_UTF8_BUFFER_SIZE, K_UTF16_BUFFER_SIZE, LogEventStatus, NameBuffer, TIMER_EVENTS,
  TimerEventName, append_code_create_header, current_time_event, delete_event, file_logger_time,
  intptr_t_event, new_event, profiler_begin_event, shared_library_end_event, shared_library_event,
  string_event, timer_event, unchecked_string_event,
};
// --- flags_lite.rs(【待回收】v8-flags)---
pub use flags_lite::{
  DEFAULT_HISTOGRAM_INTERVAL, DEFAULT_LOGFILE, DEFAULT_PROF_SAMPLING_INTERVAL, histogram_interval,
  interpreted_frames_native_stack, log, log_all, log_code, log_code_disassemble, log_deopt,
  log_feedback_vector, log_function_events, log_ic, log_maps, log_maps_details, log_source_code,
  log_source_position, log_timer_events, logfile, logfile_per_isolate, predictable, prof_cpp,
  prof_include_idle, prof_sampling_interval, rcs_cpu_time, reset_defaults, set_histogram_interval,
  set_interpreted_frames_native_stack, set_log, set_log_all, set_log_code,
  set_log_code_disassemble, set_log_deopt, set_log_feedback_vector, set_log_function_events,
  set_log_ic, set_log_maps, set_log_maps_details, set_log_source_code, set_log_source_position,
  set_log_timer_events, set_logfile, set_logfile_per_isolate, set_predictable, set_prof_cpp,
  set_prof_include_idle, set_prof_sampling_interval, set_rcs_cpu_time, set_slow_histograms,
  set_verify_predictable, slow_histograms, verify_predictable,
};
// --- local_logger.rs(local-logger.h/.cc)---
pub use local_logger::LocalLogger;
// --- log.rs(log.cc 函数/脚本事件核心门面, 解耦 Isolate)---
pub use log::{
  CompilationCacheEventData, FunctionEventData, ScriptEventType, append_function_message,
  code_name_event, compilation_cache_event, format_g, function_event, move_event_internal,
  script_event, script_event_name,
};
// --- log_file.rs(log-file.h/.cc)---
// 注: `close`/`new_message_builder`/`write_log_header` 是 `LogFile` 关联方法
// (log_file.rs:167/175/186), 非自由函数, 不经 `pub use` 摊平, 经 `LogFile::` 使用。
pub use log_file::{
  K_LOG_TO_CONSOLE, K_LOG_TO_TEMPORARY_FILE, K_MESSAGE_BUFFER_SIZE, K_NEXT, LogFile, LogSeparator,
  LogVersionInfo, MessageBuilder, OutputHandle, create_output_handle, is_logging_to_console,
  is_logging_to_temporary_file, open_temporary_file,
};
// --- log_inl.rs(log-inl.h)---
pub use log_inl::to_native_by_script;
// --- metrics.rs(metrics.h/.cc `Recorder` 签名骨架)---
pub use metrics::{ContextId, DelayedEventBase, MetricsRecorder, Recorder};
// --- runtime_call_stats.rs(runtime-call-stats.h/.cc + scope)---
pub use runtime_call_stats::{
  CounterMode, K_JS_EXECUTION, K_NUMBER_OF_COUNTERS, K_TEST_COUNTER1, K_TEST_COUNTER2,
  K_TEST_COUNTER3, NowFn, RuntimeCallCounter, RuntimeCallCounterId, RuntimeCallStats,
  RuntimeCallTimer, RuntimeCallTimerScope, ThreadType, WorkerThreadRuntimeCallStats,
  change_current_runtime_counter, now, now_cpu_time, rcs_scope, reset_now, set_now,
  trace_handler_stats,
};
// --- time_base.rs(【待回收】v8-base)---
pub use time_base::{
  ElapsedTimer, TimeDelta, TimeTicks, monotonically_increasing_time_in_ms, thread_ticks_supported,
};
// --- tracing_flags.rs(【待回收】v8-tracing)---
pub use tracing_flags::{
  ENABLED_BY_NATIVE, ENABLED_BY_SAMPLING, ENABLED_BY_TRACING, GC, GC_STATS, IC_STATS,
  RUNTIME_STATS, TracingFlags, ZONE_STATS,
};
