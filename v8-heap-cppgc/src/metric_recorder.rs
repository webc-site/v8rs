// 由 tool/src/heap/cppgc-internal/metric-recorder.h.js 从 v8/src/heap/cppgc-internal/metric-recorder.h 生成; 修订就地改, 复跑覆盖
// MetricRecorder 事件结构体族 + 嵌入器上报 trait 骨架
// 推迟台账: AddMainThreadTraceEvent / MetricRecorderImpl 的 perfetto 落地 (metric-recorder.cc) 属
// trace 面裁剪, 随 trace_event/heap 批次; 本面仅落类型面与空 trait。

/// v8: src/heap/cppgc-internal/metric-recorder.h:24 GCCycle::Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GcCycleType {
  Minor,
  /// 原版 GCCycle 默认 kMajor。
  #[default]
  Major,
}

/// v8: src/heap/cppgc-internal/metric-recorder.h:25-28 GCCycle::IncrementalPhases; None = 未采样哨兵 (-1)
#[derive(Debug, Clone, Copy, Default)]
pub struct IncrementalPhases {
  /// int64_t mark_duration_us = -1
  pub mark_duration_us: Option<i64>,
  /// int64_t sweep_duration_us = -1
  pub sweep_duration_us: Option<i64>,
}

/// v8: src/heap/cppgc-internal/metric-recorder.h:29-32 GCCycle::Phases : IncrementalPhases (继承扁平化)
#[derive(Debug, Clone, Copy, Default)]
pub struct Phases {
  pub incremental: IncrementalPhases,
  /// int64_t weak_duration_us = -1
  pub weak_duration_us: Option<i64>,
  /// int64_t compact_duration_us = -1
  pub compact_duration_us: Option<i64>,
}

/// v8: src/heap/cppgc-internal/metric-recorder.h:33-37 GCCycle::Sizes; None = 未采样哨兵 (-1)
#[derive(Debug, Clone, Copy, Default)]
pub struct Sizes {
  pub before_bytes: Option<i64>,
  pub after_bytes: Option<i64>,
  pub freed_bytes: Option<i64>,
}

/// v8: src/heap/cppgc-internal/metric-recorder.h:23-49 GCCycle
/// 10 聚合字段 (type + 4 阶段 + 2 Sizes + 3 率)。
#[derive(Debug, Clone, Copy, Default)]
pub struct GcCycle {
  pub type_: GcCycleType,
  pub total: Phases,
  pub main_thread: Phases,
  pub main_thread_atomic: Phases,
  pub main_thread_incremental: IncrementalPhases,
  pub objects: Sizes,
  pub memory: Sizes,
  pub collection_rate_in_percent: f64,
  pub efficiency_in_bytes_per_us: f64,
  pub main_thread_efficiency_in_bytes_per_us: f64,
}

/// v8: src/heap/cppgc-internal/metric-recorder.h:51-53 MainThreadIncrementalMark
#[derive(Debug, Clone, Copy, Default)]
pub struct MainThreadIncrementalMark {
  /// int64_t duration_us = -1 未采样
  pub duration_us: Option<i64>,
}

/// v8: src/heap/cppgc-internal/metric-recorder.h:55-57 MainThreadIncrementalSweep
#[derive(Debug, Clone, Copy, Default)]
pub struct MainThreadIncrementalSweep {
  /// int64_t duration_us = -1 未采样
  pub duration_us: Option<i64>,
}

/// v8: src/heap/cppgc-internal/metric-recorder.h:21-64 MetricRecorder
/// 嵌入器实现 trait 上报 GC 直方图; 3 个虚 AddMainThreadEvent 重载 → Rust 拆名默认空实现。
pub trait MetricRecorder {
  /// virtual void AddMainThreadEvent(const GCCycle&) — metric-recorder.h:61
  fn add_main_thread_event_gc_cycle(&mut self, _event: &GcCycle) {}
  /// virtual void AddMainThreadEvent(const MainThreadIncrementalMark&) — metric-recorder.h:62
  fn add_main_thread_event_incremental_mark(&mut self, _event: &MainThreadIncrementalMark) {}
  /// virtual void AddMainThreadEvent(const MainThreadIncrementalSweep&) — metric-recorder.h:63
  fn add_main_thread_event_incremental_sweep(&mut self, _event: &MainThreadIncrementalSweep) {}
}
