// 由 tool/src/diagnostics/etw-jit-win.h.js 从 v8/src/diagnostics/etw-jit-win.{h,cc} 生成; 修订就地改, 复跑覆盖
// ETW JIT 接入骨架

// Windows 平台模块 (ETW), 接入时建议 #[cfg(windows)]。
// 语义面: 事件发送/元数据注册/回调实现登记跳过, 由 Developer 修订补齐。

use std::sync::atomic::AtomicBool;

/// Rundown 事件位掩码 (原 kEtwRundown; CaptureState 回调触发时对事件 ID 加此位)
pub const ETW_RUNDOWN: u32 = 0xf0000000;

/// 原 ETWJITInterface::has_active_etw_tracing_session_or_custom_filter (原子开关)
pub static HAS_ACTIVE_ETW_TRACING_SESSION_OR_CUSTOM_FILTER: AtomicBool = AtomicBool::new(false);

/// 原 namespace ETWJITInterface 接口函数签名骨架
pub struct EtwJitInterface;

impl EtwJitInterface {
  // 原签名: void Register()
  // 原签名: void Unregister()
  // 原签名: void AddIsolate(Isolate* isolate)
  // 原签名: void RemoveIsolate(Isolate* isolate)
  // 原签名: void EventHandler(const v8::JitCodeEvent* event)
  // 原签名: void MaybeSetHandlerNow(Isolate* isolate)
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(ETW_RUNDOWN == 0xF0000000);
};
