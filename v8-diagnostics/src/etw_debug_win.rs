// 由 tool/src/diagnostics/etw-debug-win.h.js 从 v8/src/diagnostics/etw-debug-win.{h,cc} 生成; 修订就地改, 复跑覆盖
// ETW 调试输出骨架

// Windows 平台模块 (ETW 事件追踪调试), 接入时建议 #[cfg(windows)]。
// 语义面: operator<< 流式输出实现登记跳过, 由 Developer 修订补齐。

/// 原 ETWTRACEDBG 宏 — DEBUG 分支 (v8_flags.etw_trace_debug 短路 + 流式输出)
pub const ETW_TRACE_DBG_MACRO_DEBUG: &str = "v8_flags.etw_trace_debug&& std::cout << EtwTraceDebug::info";

/// 原 ETWTRACEDBG 宏 — release 分支 (编译期常量折叠为空操作)
pub const ETW_TRACE_DBG_MACRO_RELEASE: &str = "0 && std::cout\n#endif\n\n}  // namespace internal\n}  // namespace v8";

/// 原 class EtwTraceDebug — 全局调试流标记单例 (唯一成员 static info)
pub struct EtwTraceDebug;

/// 原文件静态实例声明: EtwTraceDebug::info
pub static INFO: EtwTraceDebug = EtwTraceDebug;

// 原签名: std::ostream& operator<<(std::ostream&, const EtwTraceDebug&)
// TODO(dev): 补 Debug 输出实现。

// —— 提取不变式断言 ——
const _: () = {
  assert!(ETW_TRACE_DBG_MACRO_DEBUG.contains("etw_trace_debug"));
  assert!(ETW_TRACE_DBG_MACRO_RELEASE.contains("0 && std::cout"));
};
