#!/usr/bin/env bun
// tool/src/logging/log.h.js
// 机械提取 v8/src/logging/log.{h,cc} + log-inl.h + log-file.{h,cc}:
// 日志分隔符/文件名常量/监听器面/消息构造协议

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/logging/log.h");
const ccSrc = readV8("src/logging/log.cc");
const inlSrc = readV8("src/logging/log-inl.h");
const fhSrc = readV8("src/logging/log-file.h");
const fcSrc = readV8("src/logging/log-file.cc");

// 1. LogSeparator 枚举
const sepM = fhSrc.match(/enum class LogSeparator \{ (kSeparator) \};/);
if (!sepM) throw new Error("LogSeparator 漂移");

// 2. log-file.h 常量文件名
const constNames = [...fhSrc.matchAll(/static const char\* const (k\w+);/g)].map((m) => m[1]);
if (constNames.length < 2) throw new Error(`log-file.h 常量文件名应 ≥2, 实得 ${constNames.join(",")}`);
// cc 侧取实值
const constVals = constNames.map((n) => {
  const m = fcSrc.match(new RegExp(`const char\\* const LogFile::${n} = "([^"]+)"`));
  if (!m) throw new Error(`log-file.cc 未定义 ${n}`);
  return { name: n, val: m[1] };
});

// 3. LogEventListener 覆写面 (log.h: CodeCreateEvent 等, 大多为虚; LogRecordedBuffer 归 wasm 监听)
const listenerFns = [...hSrc.matchAll(/\n  virtual void (\w+)\(/g)].map((m) => m[1]);
// V8FileLogger 事件面单独取 (非虚)
const loggerFns = [...hSrc.matchAll(/\n  void (\w+Event)\(/g)].map((m) => m[1]);
if (listenerFns.join() !== "LogRecordedBuffer,LogRecordedBuffer") {
  throw new Error(`监听器虚面漂移: ${listenerFns.join()}`);
}
if (loggerFns.length < 10) throw new Error(`V8FileLogger 事件应 ≥10, 实得 ${loggerFns.length}`);

// 5. MessageBuilder 协议
for (const k of ["MessageBuilder", "operator<<"]) {
  if (!fhSrc.includes(k)) throw new Error(`log-file.h 缺 ${k}`);
}
// log-inl: CodeTag 归一 + TimerEventScope 落盘
if (!inlSrc.includes("kNativeFunction") || !inlSrc.includes("TimerEventScope")) {
  throw new Error("log-inl 结构漂移");
}

let rust = banner(
  "src/logging/log.h.js",
  "src/logging/log.{h,cc,inl.h} + log-file.{h,cc}",
  "日志器: 分隔符/文件名/监听器面",
);

rust += `
/// 日志行分隔符 (原版 enum class LogSeparator)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LogSeparator {
  Separator = 0,
}

/// 日志目标常量文件名 (原版 log-file.h 声明 + log-file.cc 定义)
pub mod log_file_const {
${constVals
  .map((c) => {
    const n = c.name.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
    return `  /// 原版 ${c.name}\n  pub const ${n}: &str = "${c.val}";`;
  })
  .join("\n")}
}

/// LogEventListener 虚面 (原版 log.h LogEventListener 派生的 wasm 监听虚函数对数)
pub const EVENT_LISTENER_METHODS: [&str; ${listenerFns.length}] = [${listenerFns.map((f) => `"${f}"`).join(", ")}];

/// V8FileLogger 事件写入面 (原版 log.h, ${loggerFns.length} 项 *Event 方法, 顺序登记)
pub const LOGGER_EVENT_METHODS: [&str; ${loggerFns.length}] = [${loggerFns.map((f) => `"${f}"`).join(", ")}];

/// 签名骨架 (语义见手写 log.rs / log_file.rs)
pub mod log_skel {
  /// 消息构造协议 (原版 LogFile::MessageBuilder): 流式 << 拼接, 析构时整行落盘并转义
  pub fn message_builder_skel() {
    unimplemented!("骨架")
  }
  /// Native 标签归一 (原版 log-inl: kFunction/kScript 落盘前转 Native 标签)
  pub fn normalize_native_tag() {
    unimplemented!("骨架")
  }
  /// 计时事件落盘 (原版 TimerEventScope::LogTimerEvent → CallEventLogger)
  pub fn log_timer_event() {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-logging", "log_generated.rs", rust);
console.log(`written ${p}; 文件名=${constVals.length}, 监听器=${listenerFns.length}, 事件=${loggerFns.length}`);
