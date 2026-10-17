#!/usr/bin/env bun
// tool/src/diagnostics/etw-isolate-capture-state-monitor-win.h.js
// 机械提取 v8/src/diagnostics/etw-isolate-capture-state-monitor-win.{h,cc} —
// CaptureState 同步监视器面 (WaitFor/Notify 骨架)。
// Windows 平台模块; 条件变量等待实现登记跳过。
// 产物: v8-diagnostics/src/etw_isolate_capture_state_monitor_win.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/diagnostics/etw-isolate-capture-state-monitor-win.h";
const CC_REL = "src/diagnostics/etw-isolate-capture-state-monitor-win.cc";
const h = readV8(H_REL);
const cc = readV8(CC_REL);
const loadCc = readV8("src/diagnostics/etw-isolate-load-script-data-win.cc"); // kCaptureStateTimeout 所在

if (!h.includes("bool WaitFor(const base::TimeDelta& delta);")) throw new Error("缺 WaitFor 声明锚");
if (!h.includes("void Notify();")) throw new Error("缺 Notify 声明锚");
if (!h.includes("pending_isolate_count_")) throw new Error("缺 pending_isolate_count 字段锚");
const timeoutM = loadCc.match(/constexpr auto kCaptureStateTimeout = base::TimeDelta::FromSeconds\((\d+)\);/);
if (!timeoutM) throw new Error("未找到 kCaptureStateTimeout");

let rust = banner("src/diagnostics/etw-isolate-capture-state-monitor-win.h.js", "src/diagnostics/etw-isolate-capture-state-monitor-win.{h,cc}", "ETW CaptureState 监视器骨架");
rust += `
// Windows 平台模块 (ETW), 接入时建议 #[cfg(windows)]。
// 语义面: WaitFor 条件变量等待/超时恢复实现登记跳过。

/// CaptureState 超时秒数 (原 kCaptureStateTimeout, etw-isolate-load-script-data-win.cc)
pub const CAPTURE_STATE_TIMEOUT_SECS: u64 = ${timeoutM[1]};

/// 原 class EtwIsolateCaptureStateMonitor (不可拷贝)
/// - mutex_: WaitFor 前必须持有, 亦保护 isolate_count_ 读写
/// - pending_isolate_count_: 待通知 isolate 计数
/// - isolates_ready_cv_: 就绪条件变量
/// - wait_started_: 记录等待起点, 供虚假唤醒后恢复剩余超时
pub struct EtwIsolateCaptureStateMonitor {
  pub pending_isolate_count: usize,
}

impl EtwIsolateCaptureStateMonitor {
  // 原签名: bool WaitFor(const base::TimeDelta& delta)
  //   ETW 回调线程调用, 等待指定时长或 Notify 被 pending_isolate_count 次调用
  // 原签名: void Notify()
  //   isolate 线程调用, 解除 WaitFor 阻塞
  // TODO(dev): 以 Mutex+Condvar 补实现。
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(CAPTURE_STATE_TIMEOUT_SECS == 10);
};
`;

const p = writeSrc("v8-diagnostics", "etw_isolate_capture_state_monitor_win.rs", rust);
console.log(`written ${p}`);
