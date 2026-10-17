// 由 tool/src/diagnostics/etw-jit-metadata-win.h.js 从 v8/src/diagnostics/etw-jit-metadata-win.{h,cc} 生成; 修订就地改, 复跑覆盖
// ETW JIT 事件元数据常量

// Windows 平台模块 (ETW), 接入时建议 #[cfg(windows)]。
// 语义面: str_bytes/MakeStrBytes/JoinBytes/Field/Header/JoinFields/EventFields/
// EventMetadata constexpr 模板元编程不属规则代码, 登记跳过。

/// 原 kEtwControlDisable
pub const ETWCONTROLDISABLE: u32 = 0;
/// 原 kEtwControlEnable
pub const ETWCONTROLENABLE: u32 = 1;
/// 原 kEtwControlCaptureState
pub const ETWCONTROLCAPTURESTATE: u32 = 2;
/// 原 kJScriptRuntimeKeyword
pub const JSCRIPTRUNTIMEKEYWORD: u64 = 1;
/// 原 kSourceLoadEventID
pub const SOURCELOADEVENTID: u16 = 41;
/// 原 kMethodLoadEventID
pub const METHODLOADEVENTID: u16 = 9;
/// 原 kSourceDCStartEventID
pub const SOURCEDCSTARTEVENTID: u16 = 39;
/// 原 kMethodDCStartEventID
pub const METHODDCSTARTEVENTID: u16 = 5;

/// 原 kTraceLevel = TRACE_LEVEL_INFORMATION (值由 Windows SDK 头定义, 接入时绑定)
pub const TRACE_LEVEL: u8 = TRACE_LEVEL_INFORMATION;

// —— 提取不变式断言 ——
const _: () = {
  assert!(ETW_CONTROL_DISABLE == 0);
  assert!(ETW_CONTROL_ENABLE == 1);
  assert!(ETW_CONTROL_CAPTURE_STATE == 2);
  assert!(JSCRIPT_RUNTIME_KEYWORD == 1);
  assert!(SOURCE_LOAD_EVENT_ID == 41);
  assert!(METHOD_LOAD_EVENT_ID == 9);
  assert!(SOURCE_DC_START_EVENT_ID == 39);
  assert!(METHOD_DC_START_EVENT_ID == 5);
};
