#!/usr/bin/env bun
// tool/src/diagnostics/etw-debug-win.h.js
// 机械提取 v8/src/diagnostics/etw-debug-win.{h,cc} — ETW 调试输出面。
// Windows 平台模块 (非架构差异), 语义面登记跳过, 宏分支 + 类面登记。
// 产物: v8-diagnostics/src/etw_debug_win.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/diagnostics/etw-debug-win.h";
const CC_REL = "src/diagnostics/etw-debug-win.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

// DEBUG / release 两分支宏
const dbgM = h.match(/#ifdef DEBUG\n#define ETWTRACEDBG (.+)\n#else\n#define ETWTRACEDBG (.+)\n#endif/s);
if (!dbgM) throw new Error("未找到 ETWTRACEDBG 宏双分支");
if (!h.includes("static EtwTraceDebug info;")) throw new Error("缺 EtwTraceDebug::info 声明锚");
if (!h.includes("std::ostream& operator<<(std::ostream& os, const EtwTraceDebug&);")) {
  throw new Error("缺 operator<< 声明锚");
}

let rust = banner("src/diagnostics/etw-debug-win.h.js", "src/diagnostics/etw-debug-win.{h,cc}", "ETW 调试输出骨架");
rust += `
// Windows 平台模块 (ETW 事件追踪调试), 接入时建议 #[cfg(windows)]。
// 语义面: operator<< 流式输出实现登记跳过, 由 Developer 修订补齐。

/// 原 ETWTRACEDBG 宏 — DEBUG 分支 (v8_flags.etw_trace_debug 短路 + 流式输出)
pub const ETW_TRACE_DBG_MACRO_DEBUG: &str = ${JSON.stringify(dbgM[1].trim())};

/// 原 ETWTRACEDBG 宏 — release 分支 (编译期常量折叠为空操作)
pub const ETW_TRACE_DBG_MACRO_RELEASE: &str = ${JSON.stringify(dbgM[2].trim())};

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
`;

const p = writeSrc("v8-diagnostics", "etw_debug_win.rs", rust);
console.log(`written ${p}`);
