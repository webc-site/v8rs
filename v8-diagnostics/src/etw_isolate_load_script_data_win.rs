// 由 tool/src/diagnostics/etw-isolate-load-script-data-win.h.js 从 v8/src/diagnostics/etw-isolate-load-script-data-win.{h,cc} 生成; 修订就地改, 复跑覆盖
// ETW 脚本加载日志骨架

// Windows 平台模块 (ETW), 接入时建议 #[cfg(windows)]。
// 语义面: 日志启用/中断排队/过滤数据下发实现登记跳过。

use std::{collections::HashMap, sync::Mutex};

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
  // 原签名: static void AddIsolate(Isolate* isolate)
  // 原签名: static void RemoveIsolate(Isolate* isolate)
  // 原签名: static void UpdateAllIsolates(bool etw_enabled, uint32_t options)
  // 原签名: static bool MaybeAddLoadedScript(Isolate* isolate, int script_id)
  // 原签名: static void EnableLog( Isolate* isolate, size_t event_id, std::weak_ptr<EtwIsolateCaptureStateMonitor> weak_monitor, uint32_t options)
  // 原签名: static void DisableLog(Isolate* isolate, size_t event_id)
  // 原签名: static void EnableLogWithFilterDataOnAllIsolates(const uint8_t* data, size_t size, uint32_t options)
  // 原签名: static void EnableLogWithFilterData( Isolate* isolate, size_t event_id, const std::string& EnableLogWithFilterData, std::weak_ptr<EtwIsolateCaptureStateMonitor> weak_monitor, uint32_t options)
  // 原签名: void EnqueueEnableLog(weak_ptr monitor, options)
  // 原签名: void EnqueueEnableLogWithFilterData(payload, monitor, options)
  // 原签名: void EnqueueDisableLog()
  // 原签名: bool IsScriptLoaded(int script_id) const
  // 原签名: void AddLoadedScript(int script_id)
  // 原签名: void RemoveAllLoadedScripts()
  // 原签名: size_t CurrentEventId() const
}

/// 原 extern base::LazyMutex isolates_mutex
pub static ISOLATES_MUTEX: Mutex<()> = Mutex::new(());

/// 原 isolate_map: Isolate* → IsolateLoadScriptData (LazyInstance)
pub type IsolateMap = HashMap<usize, IsolateLoadScriptData>;

/// 原 etw_filter_payload_glob (LazyInstance<FilterDataType>, FilterDataType = String)
pub type FilterData = String;

// —— 提取不变式断言 ——
const _: () = {
  // 静态接口 8 条登记
};
