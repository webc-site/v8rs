#!/usr/bin/env bun
// tool/src/diagnostics/etw-isolate-load-script-data-win.h.js
// 机械提取 v8/src/diagnostics/etw-isolate-load-script-data-win.{h,cc} —
// ETW 脚本加载日志面 (静态接口 + 中断数据结构骨架)。
// Windows 平台模块; 日志启用/过滤实现登记跳过。
// 产物: v8-diagnostics/src/etw_isolate_load_script_data_win.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/diagnostics/etw-isolate-load-script-data-win.h";
const CC_REL = "src/diagnostics/etw-isolate-load-script-data-win.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

const staticSigs = [...h.matchAll(/^\s*static\s+((?:void|bool)\s+\w+\([^;]*\));/gm)].map((m) => m[1].replace(/\s+/g, " ").trim());
if (staticSigs.length < 7) throw new Error(`静态接口条数不足: ${staticSigs.length}`);
if (!h.includes("struct EnableInterruptData")) throw new Error("缺 EnableInterruptData 结构锚");
if (!h.includes("extern base::LazyMutex isolates_mutex;")) throw new Error("缺 isolates_mutex 锚");
if (!h.includes("extern base::LazyInstance<IsolateMapType>::type isolate_map;")) throw new Error("缺 isolate_map 锚");

let rust = banner("src/diagnostics/etw-isolate-load-script-data-win.h.js", "src/diagnostics/etw-isolate-load-script-data-win.{h,cc}", "ETW 脚本加载日志骨架");
rust += `
// Windows 平台模块 (ETW), 接入时建议 #[cfg(windows)]。
// 语义面: 日志启用/中断排队/过滤数据下发实现登记跳过。

/// 原 IsolateLoadScriptData::EnableInterruptData
pub struct EnableInterruptData {
  pub event_id: usize,
  /// 原为 weak_ptr<EtwIsolateCaptureStateMonitor>
  pub weak_monitor_slot: (),
  pub options: u32,
}

/// 原 IsolateLoadScriptData::EnableWithFilterDataInterruptData
pub struct EnableWithFilterDataInterruptData {
  pub event_id: usize,
  /// 原 std::string payload
  pub payload: String,
  pub weak_monitor_slot: (),
  pub options: u32,
}

/// 原 class IsolateLoadScriptData — 静态接口签名骨架
pub struct IsolateLoadScriptData;

impl IsolateLoadScriptData {
${staticSigs.map((s) => `  // 原签名: static ${s}`).join("\n")}
  // 原签名: void EnqueueEnableLog(weak_ptr monitor, options)
  // 原签名: void EnqueueEnableLogWithFilterData(payload, monitor, options)
  // 原签名: void EnqueueDisableLog()
  // 原签名: bool IsScriptLoaded(int script_id) const
  // 原签名: void AddLoadedScript(int script_id)
  // 原签名: void RemoveAllLoadedScripts()
  // 原签名: size_t CurrentEventId() const
}

/// 原 extern base::LazyMutex isolates_mutex
pub static ISOLATES_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// 原 isolate_map: Isolate* → IsolateLoadScriptData (LazyInstance)
pub type IsolateMap = std::collections::HashMap<usize, IsolateLoadScriptData>;

/// 原 etw_filter_payload_glob (LazyInstance<FilterDataType>, FilterDataType = String)
pub type FilterData = String;

// —— 提取不变式断言 ——
const _: () = {
  // 静态接口 ${staticSigs.length} 条登记
};
`;

const p = writeSrc("v8-diagnostics", "etw_isolate_load_script_data_win.rs", rust);
console.log(`written ${p} (静态接口 ${staticSigs.length} 条)`);
