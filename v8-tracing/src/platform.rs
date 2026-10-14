//! embedder 追踪面, 1:1 对应 `include/v8-platform.h` 的追踪部分 (389-468 行)。
//!
//! - `ConvertableToTraceFormat` (v8-platform.h:389): 追踪事件复杂参数的 JSON 序列化口。
//! - `TracingController` (v8-platform.h:397): embedder 挂接点 — 类别启用查询 /
//!   事件写入 / COMPLETE 事件时长补记 / 追踪状态观察者; 默认实现与原版 inline
//!   空实现逐条对应 (GetCategoryGroupEnabled → 静态 0, AddTraceEvent → 0,
//!   UpdateTraceEventDuration → 空操作, observer 增删 → 空操作)。
//! - `TraceStateObserver` (v8-platform.h:452): OnTraceEnabled/OnTraceDisabled。
//!
//! 原版 C++ 通过虚函数 + 平台指针分发; Rust 以 trait 对象等价承接
//! (`V8_USE_PERFETTO` 分支的 TrackEvent 直写路径【推迟登记: 依赖 perfetto SDK】)。

use std::sync::Arc;

use crate::trace_event::TraceEvent;

/// `v8::ConvertableToTraceFormat` (v8-platform.h:389)。
/// 追加的内容必须是合法 JSON 对象 (字符串须已转义)。
pub trait ConvertableToTraceFormat: Send + Sync {
  /// 对应 `AppendAsTraceFormat(std::string* out) const`。
  fn append_as_trace_format(&self, out: &mut String);
}

/// `v8::TracingController::TraceStateObserver` (v8-platform.h:452)。
pub trait TraceStateObserver: Send + Sync {
  /// 对应 `OnTraceEnabled`。
  fn on_trace_enabled(&self);
  /// 对应 `OnTraceDisabled`。
  fn on_trace_disabled(&self);
}

/// `v8::TracingController` (v8-platform.h:397) — embedder 实现此 trait 记录 V8 事件。
///
/// 追踪事件经 trace_event_macros 宏展开函数 → trace_event::emit 核
/// → 本 trait; 默认实现全部为原版 inline 空实现 (no-op embedder)。
pub trait TracingController: Send + Sync {
  /// 对应 `GetCategoryGroupEnabled` (v8-platform.h:413): 返回非 0 表示类别组启用;
  /// 字节位含义见 `crate::trace_event::ENABLED_FOR_RECORDING` 等。
  fn get_category_group_enabled(&self, _name: &str) -> u8 {
    0
  }

  /// 对应 `AddTraceEvent` (v8-platform.h:424): 返回句柄供 `update_trace_event_duration`
  /// 补记 COMPLETE 事件时长。arg 类型打标语义 = 原版 arg_types/arg_values 平面数组
  /// (Rust 以 `TraceValue` 类型化承载, 见 `TraceValue::type_tag`)。
  fn add_trace_event(&self, _ev: &TraceEvent) -> u64 {
    0
  }

  /// 对应 `AddTraceEventWithTimestamp` (v8-platform.h:438): 显式时间戳变体。
  fn add_trace_event_with_timestamp(&self, _ev: &TraceEvent, _timestamp: i64) -> u64 {
    0
  }

  /// 对应 `UpdateTraceEventDuration` (v8-platform.h:450)。
  fn update_trace_event_duration(&self, _category_enabled_flag: u8, _name: &str, _handle: u64) {}

  /// 对应 `AddTraceStateObserver` (v8-platform.h:461)。
  fn add_trace_state_observer(&self, _observer: Arc<dyn TraceStateObserver>) {}

  /// 对应 `RemoveTraceStateObserver` (v8-platform.h:467): 以 `Arc::ptr_eq` 判同。
  fn remove_trace_state_observer(&self, _observer: &Arc<dyn TraceStateObserver>) {}
}

/// 原版默认 no-op controller (v8-platform.h 内联空实现的等价物)。
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopController;

impl TracingController for NoopController {}
