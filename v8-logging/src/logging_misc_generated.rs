// 由 tool/src/logging/metrics.h.js 从 v8/src/logging/metrics.{h,cc} + local-logger.{h,cc} + tracing-flags.{h,cc} 生成; 修订就地改, 复跑覆盖
// 指标 Recorder / 局部日志器 / 追踪开关

/// 嵌入式指标 Recorder 面 (原版 metrics.h class Recorder)
pub mod metrics_skel {
  /// 方法清单 (对数: 7)
  pub const RECORDER_METHODS: [&str; 7] = ["SetEmbedderRecorder", "HasEmbedderRecorder", "NotifyIsolateDisposal", "AddMainThreadEvent", "DelayMainThreadEvent", "AddThreadSafeEvent", "Delay"];

  /// 主线程事件上报 (原版 AddMainThreadEvent<T>)
  pub fn add_main_thread_event<T>(_event: &T, _timestamp: u64) {
    unimplemented!("骨架")
  }
  /// 延时事件上报 (原版 DelayMainThreadEvent<T>)
  pub fn delay_main_thread_event<T>(_event: &T, _timestamp: u64) {
    unimplemented!("骨架")
  }
  /// 嵌入器 Recorder 挂载 (原版 SetEmbedderRecorder)
  pub fn set_embedder_recorder(_isolate_key: usize, _recorder: usize) {
    unimplemented!("骨架")
  }
}

/// LocalLogger — 线程安全日志快照 (原版 local-logger.h)
pub mod local_logger_skel {
  /// 方法清单 (对数: 5)
  pub const METHODS: [&str; 5] = ["ScriptDetails", "ScriptEvent", "CodeLinePosInfoRecordEvent", "MapCreate", "MapDetails"];

  /// 日志开关快照 (原版 is_logging)
  pub fn is_logging() -> bool {
    unimplemented!("骨架")
  }
  /// 代码事件监听快照 (原版 is_listening_to_code_events)
  pub fn is_listening_to_code_events() -> bool {
    unimplemented!("骨架")
  }
  /// 脚本事件 (原版 ScriptEvent/ScriptDetails)
  pub fn script_event(_type: u8, _script_id: i32) {
    unimplemented!("骨架")
  }
  /// Map 创建事件 (原版 MapCreate)
  pub fn map_create(_map: usize) {
    unimplemented!("骨架")
  }
}

/// 追踪开关组 (原版 tracing-flags.h, 5 项, 联动命令行旗标)
pub mod tracing_flags_skel {
  pub const FLAGS: [&str; 5] = ["is_runtime_stats_enabled", "is_gc_enabled", "is_gc_stats_enabled", "is_ic_stats_enabled", "is_zone_stats_enabled"];

  /// 运行时统计开关 (原版 is_runtime_stats_enabled → v8_flags.runtime_call_stats)
  pub fn is_runtime_stats_enabled() -> bool {
    unimplemented!("骨架")
  }
  /// GC 事件开关 (原版 is_gc_enabled)
  pub fn is_gc_enabled() -> bool {
    unimplemented!("骨架")
  }
  /// GC 统计开关 (原版 is_gc_stats_enabled)
  pub fn is_gc_stats_enabled() -> bool {
    unimplemented!("骨架")
  }
  /// IC 统计开关 (原版 is_ic_stats_enabled)
  pub fn is_ic_stats_enabled() -> bool {
    unimplemented!("骨架")
  }
  /// Zone 统计开关 (原版 is_zone_stats_enabled)
  pub fn is_zone_stats_enabled() -> bool {
    unimplemented!("骨架")
  }
}
