#!/usr/bin/env bun
// tool/src/base/debug/stack_trace_zos.cc.js — 族: src/base/debug/stack_trace_zos.cc
// 覆盖: z/OS 后端 — 信号线程注册 8 信号 + zoslib __display_backtrace
// → v8-base/src/stack_trace_zos.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/base/debug/stack_trace_zos.cc");

// —— 锚点断言 ——
if (!src.includes("bool IsDumpStackInSignalHandler = true;")) throw new Error("缺全局开关");
if (!src.includes("constexpr size_t kDefaultStackSize = 4 * 1024 * 1024;")) throw new Error("缺默认栈尺寸");
if (!src.includes("void StackDumpSignalHandler(int signal, siginfo_t* info, void* void_context)")) throw new Error("缺信号处理器");
if (!src.includes("if (IsDumpStackInSignalHandler) __display_backtrace(STDERR_FILENO);")) throw new Error("缺 zoslib 回溯");
if (!src.includes("action.sa_flags = SA_RESETHAND | SA_SIGINFO | SA_ONSTACK;")) throw new Error("缺 sa_flags");
if (!src.includes("__set_backtrace_on_abort(false);")) throw new Error("缺 abort 回溯关闭");
if (!src.includes("StackTrace::Print() const { __display_backtrace(STDERR_FILENO); }")) throw new Error("缺 Print");
if (!src.includes("UNREACHABLE();")) throw new Error("缺 OutputToStream 不可达");

// —— 真解析: 注册的信号清单 ——
const signals = [...src.matchAll(/sigaction\((SIG\w+), &action, nullptr\) == 0/g)].map((m) => m[1]);
if (signals.join(",") !== "SIGILL,SIGABRT,SIGFPE,SIGBUS,SIGSEGV,SIGSYS,SIGINT,SIGTERM") {
  throw new Error(`信号清单 ${signals}`);
}
console.log(`stack_trace_zos: 信号 ${signals.length}, 线程栈最小 4MB, Print=zoslib 直出, OutputToStream 不可达`);

let rust = banner(
  "src/base/debug/stack_trace_zos.cc.js",
  "src/base/debug/stack_trace_zos.cc",
  "z/OS 后端 (zoslib 回溯 + 专职信号线程)",
);

rust += `
//! v8: src/base/debug/stack_trace_zos.cc — 仅 z/OS 构建, 依赖 zoslib

// v8: 专职线程 StackDumpingSignalThread — 主线程 sigfillset 屏蔽全部信号后,
//     信号线程注册 ${signals.join(" ")} (SA_RESETHAND|SA_SIGINFO|SA_ONSTACK),
//     SIGPIPE 置 SIG_IGN; 线程栈最小 ${"4MB"} (kDefaultStackSize)
// v8: StackDumpSignalHandler — SIGABRT 直接 abort (zoslib 先 __display_traceback);
//     其余信号 IsDumpStackInSignalHandler 时 __display_backtrace(STDERR_FILENO) 后 raise

/// v8: zos EnableInProcessStackDumping — 起信号线程 + 主线程屏蔽全信号
pub fn enable_in_process_stack_dumping() -> bool;
/// v8: zos DisableSignalStackDump — 关开关 + __set_backtrace_on_abort(false)
pub fn disable_signal_stack_dump();

impl StackTrace {
    /// v8: zos StackTrace() — 空构造
    #[cfg(target_os = "zos")]
    pub fn capture() -> Self;
    /// v8: zos Print — __display_backtrace(STDERR_FILENO) 直出
    #[cfg(target_os = "zos")]
    pub fn print(&self);
    /// v8: zos OutputToStream — UNREACHABLE (zoslib 无 ostream 版, TODO 登记)
    #[cfg(target_os = "zos")]
    pub fn output_to_stream(&self, _os: &mut dyn std::io::Write) -> !;
}
`;

writeSrc("v8-base", "stack_trace_zos.rs.skel", rust);
console.log("written v8-base/src/stack_trace_zos.rs.skel");
