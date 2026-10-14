// 由 tool/src/libplatform/tracing/trace-buffer.cc.js 从 v8/src/libplatform/tracing/ 11 缺口文件族 生成; 修订就地改, 复跑覆盖
// 追踪实现族骨架

/// 追踪族文件锚点 (11 项)
pub const TRACING_FILES: [&str; 11] = ["trace-buffer.h", "trace-buffer.cc", "trace-config.cc", "trace-object.cc", "trace-writer.h", "trace-writer.cc", "tracing-controller.cc", "recorder.h", "recorder-mac.cc", "recorder-win.cc", "trace-event-listener.h"];

pub mod tracing_impl_skel {
  /// 环形缓冲 (原版 TraceBuffer: 分块数组, 满则覆盖最旧 chunk)
  pub fn trace_buffer_skel() {
    unimplemented!("骨架")
  }
  /// 追踪配置 (原版 TraceConfig: 分类使能过滤)
  pub fn trace_config_skel() {
    unimplemented!("骨架")
  }
  /// 追踪对象 (原版 TraceObject: Initialize + set_arg* 事件字段, arg 数登记)
  pub const TRACE_OBJECT_ARG_SETTERS: usize = 0;
  /// JSON 输出写器 (原版 JSONTraceWriter: trace 事件流 → json 数组)
  pub fn json_trace_writer_skel() {
    unimplemented!("骨架")
  }
  /// 控制器 (原版 TracingController: writer 挂载 + 分类使能 + 采样线程)
  pub fn tracing_controller_skel() {
    unimplemented!("骨架")
  }
}

/// 平台分支登记: recorder-mac.cc (os_log) / recorder-win.cc (ETW) — 非 darwin-app/win 目标跳过
pub const RECORDER_PLATFORM_BRANCHES: [&str; 2] = ["mac-os_log", "win-ETW"];
