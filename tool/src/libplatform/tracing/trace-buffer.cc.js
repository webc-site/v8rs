#!/usr/bin/env bun
// tool/src/libplatform/tracing/trace-buffer.cc.js
// 机械提取 v8/src/libplatform/tracing/ 追踪族: trace-buffer.{h,cc} + trace-config.cc +
// trace-object.cc + trace-writer.{h,cc} + tracing-controller.cc + recorder.h + recorder-{mac,win}.cc +
// trace-event-listener.h

import { readV8, writeSrc, banner } from "../../../lib.js";

const files = [
  ["src/libplatform/tracing/trace-buffer.h", "TraceBuffer"],
  ["src/libplatform/tracing/trace-buffer.cc", "TraceBuffer::"],
  ["src/libplatform/tracing/trace-config.cc", "TraceConfig::"],
  ["src/libplatform/tracing/trace-object.cc", "TraceObject::"],
  ["src/libplatform/tracing/trace-writer.h", "TraceWriter"],
  ["src/libplatform/tracing/trace-writer.cc", "TraceWriter::"],
  ["src/libplatform/tracing/tracing-controller.cc", "TracingController::"],
  ["src/libplatform/tracing/recorder.h", "Recorder"],
  ["src/libplatform/tracing/recorder-mac.cc", "recorder"],
  ["src/libplatform/tracing/recorder-win.cc", "recorder"],
  ["src/libplatform/tracing/trace-event-listener.h", "TraceEventListener"],
];

const anchors = [];
for (const [file, sym] of files) {
  const src = readV8(file);
  if (!src.includes(sym)) throw new Error(`${file} 缺 ${sym}`);
  anchors.push(file.split("/").pop());
}

// TraceObject 字段面 (category/phase/name/id/duration 等经典 trace 字段)
const toCc = readV8("src/libplatform/tracing/trace-object.cc");
const fieldM = toCc.match(/Initialize\(([\s\S]*?)\)/);
const argCount = (toCc.match(/set_arg\w*\(/g) || []).length;

// 平台分支: recorder-mac (os_log) / recorder-win (ETW) — 当前目标非 mac app target 时仅登记
let rust = banner(
  "src/libplatform/tracing/trace-buffer.cc.js",
  "src/libplatform/tracing/ 11 缺口文件族",
  "追踪实现族骨架",
);

rust += `
/// 追踪族文件锚点 (11 项)
pub const TRACING_FILES: [&str; ${anchors.length}] = [${anchors.map((a) => `"${a}"`).join(", ")}];

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
  pub const TRACE_OBJECT_ARG_SETTERS: usize = ${argCount};
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
`;

const p = writeSrc("v8-libplatform", "tracing/tracing_impl_generated.rs", rust);
console.log(`written ${p}; 锚点=${anchors.length}, arg setter=${argCount}`);
