#!/usr/bin/env bun
// tool/src/logging/metrics.h.js
// 机械提取 v8/src/logging/metrics.{h,cc} + local-logger.{h,cc} + tracing-flags.{h,cc}:
// 嵌入式 Recorder 面, 局部日志器, 追踪开关组

import { readV8, writeSrc, banner } from "../../lib.js";

const mH = readV8("src/logging/metrics.h");
const mCc = readV8("src/logging/metrics.cc");
const lH = readV8("src/logging/local-logger.h");
const lCc = readV8("src/logging/local-logger.cc");
const tH = readV8("src/logging/tracing-flags.h");
const tCc = readV8("src/logging/tracing-flags.cc");

// 1. metrics Recorder 面
const recorderFns = [...mH.matchAll(/\n  (?:V8_EXPORT_PRIVATE )?(?:void|bool) (\w+)\(/g)].map((m) => m[1]);
if (!recorderFns.includes("SetEmbedderRecorder") || !recorderFns.includes("HasEmbedderRecorder")) {
  throw new Error(`Recorder 面漂移: ${recorderFns.join(",")}`);
}
// AddMainThreadEvent / DelayMainThreadEvent / AddBackgroundEvent 模板事件族
for (const k of ["AddMainThreadEvent", "DelayMainThreadEvent"]) {
  if (!mH.includes(k)) throw new Error(`metrics.h 缺 ${k}`);
}

// 2. local-logger 面
const localFns = [...lH.matchAll(/\n  void (\w+)\(/g)].map((m) => m[1]);
if (localFns.length < 4) throw new Error(`LocalLogger 方法应 ≥4, 实得 ${localFns.length}`);
for (const k of ["is_logging", "is_listening_to_code_events"]) {
  if (!lH.includes(k)) throw new Error(`LocalLogger 缺 ${k}`);
}

// 3. tracing-flags 开关组
const flagFns = [...tH.matchAll(/static bool (is_\w+)\(\)/g)].map((m) => m[1]);
if (flagFns.join() !== "is_runtime_stats_enabled,is_gc_enabled,is_gc_stats_enabled,is_ic_stats_enabled,is_zone_stats_enabled") {
  throw new Error(`追踪开关漂移: ${flagFns.join()}`);
}
// 开关为独立原子量 (tracing-flags.cc 定义 TracingFlags::runtime_stats 等)
if (!tCc.includes("std::atomic_uint TracingFlags::runtime_stats")) throw new Error("tracing-flags.cc 原子开关漂移");

let rust = banner(
  "src/logging/metrics.h.js",
  "src/logging/metrics.{h,cc} + local-logger.{h,cc} + tracing-flags.{h,cc}",
  "指标 Recorder / 局部日志器 / 追踪开关",
);

rust += `
/// 嵌入式指标 Recorder 面 (原版 metrics.h class Recorder)
pub mod metrics_skel {
  /// 方法清单 (对数: ${recorderFns.length})
  pub const RECORDER_METHODS: [&str; ${recorderFns.length}] = [${recorderFns.map((f) => `"${f}"`).join(", ")}];

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
  /// 方法清单 (对数: ${localFns.length})
  pub const METHODS: [&str; ${localFns.length}] = [${localFns.map((f) => `"${f}"`).join(", ")}];

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

/// 追踪开关组 (原版 tracing-flags.h, ${flagFns.length} 项, 联动命令行旗标)
pub mod tracing_flags_skel {
  pub const FLAGS: [&str; ${flagFns.length}] = [${flagFns.map((f) => `"${f}"`).join(", ")}];

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
`;

const p = writeSrc("v8-logging", "logging_misc_generated.rs", rust);
console.log(`written ${p}; Recorder=${recorderFns.length}, Local=${localFns.length}, 开关=${flagFns.length}`);
