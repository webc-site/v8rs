#!/usr/bin/env bun
// tool/src/base/debug/stack_trace_posix.cc.js — 族: src/base/debug/stack_trace_posix.cc
// 覆盖: backtrace 捕获 + DemangleSymbols + ProcessBacktrace + 信号处理器 (4 信号 si_code 表)
//        + itoa_r (async-signal-safe 整数转 ASCII) + WarmUpBacktrace
// → v8-base/src/stack_trace_posix.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/base/debug/stack_trace_posix.cc");

// —— 锚点断言 ——
if (!src.includes("#define HAVE_EXECINFO_H 1")) throw new Error("缺 execinfo 门");
if (!src.includes('const char kMangledSymbolPrefix[] = "_Z";')) throw new Error("缺 mangle 前缀");
if (!src.includes("void DemangleSymbols(std::string* text) {")) throw new Error("缺 demangle");
if (!src.includes("abi::__cxa_demangle")) throw new Error("缺 cxa_demangle 调用");
if (!src.includes("class BacktraceOutputHandler {")) throw new Error("缺输出 handler 抽象");
if (!src.includes("void ProcessBacktrace(void* const* trace, size_t size,")) throw new Error("缺 ProcessBacktrace");
if (!src.includes("volatile sig_atomic_t in_signal_handler = 0;")) throw new Error("缺信号内标志");
if (!src.includes("void WarmUpBacktrace() {")) throw new Error("缺预热");
if (!src.includes("char* itoa_r(intptr_t i, char* buf, size_t sz, int base, size_t padding) {")) throw new Error("缺 itoa_r");
if (!src.includes('"0123456789abcdef"[j % base]')) throw new Error("缺数字表");
if (!src.includes("count_ = static_cast<size_t>(backtrace(trace_, arraysize(trace_)));")) throw new Error("缺 backtrace 捕获");
if (!src.includes("MemoryProtectionKey::SetDefaultPermissionsForAllKeysInSignalHandler();")) throw new Error("缺 PKU 分支");

// —— 真解析: 信号注册与 si_code 展开表, 对数 ——
const signals = [...src.matchAll(/success &= \(sigaction\((SIG\w+), &action, nullptr\) == 0\);/g)].map((m) => m[1]);
if (signals.join(",") !== "SIGILL,SIGABRT,SIGFPE,SIGBUS,SIGSEGV,SIGSYS") throw new Error(`信号清单 ${signals}`);
const siCodes = [...src.matchAll(/info->si_code == (\w+)\) \{\s*\n\s*PrintToStderr\(" (\w+) "\);/g)].map((m) => `${m[1]}=${m[2]}`);
if (siCodes.length !== 20) throw new Error(`si_code 分支 ${siCodes.length} != 20`);
const handlers = ["PrintBacktraceOutputHandler", "StreamBacktraceOutputHandler"];
for (const h of handlers) if (!src.includes(`class ${h} : public BacktraceOutputHandler`)) throw new Error(`缺 ${h}`);
console.log(`stack_trace_posix: 信号 ${signals.length}, si_code 分支 ${siCodes.length}, 输出 handler 2, itoa_r 1, demangle 1`);

let rust = banner(
  "src/base/debug/stack_trace_posix.cc.js",
  "src/base/debug/stack_trace_posix.cc",
  "POSIX 栈回溯后端 (backtrace + 信号转储, async-signal-safe 约束)",
);

rust += `
//! v8: src/base/debug/stack_trace_posix.cc
//! 硬约束: 信号处理器路径禁 malloc/stdio — itoa_r 自实现, write 直写 stderr

// —— v8: internal::itoa_r(i, buf, sz, base, padding) ——
/// async-signal-safe 整数转 ASCII; base ∈ [2,16]; 负数仅 base 10;
/// padding 前导零补齐; 溢出/非法 base 返回 None (原版 nullptr)
pub fn itoa_r(i: isize, buf: &mut [u8], base: u32, padding: usize) -> Option<&str>;

// —— v8: 符号 demangle ——
// kMangledSymbolPrefix = "_Z" (Itanium ABI);
// kSymbolCharacters = [a-zA-Z0-9_];
// DemangleSymbols: 扫 "_Z"+词字符, __cxa_demangle 成功则替换; 失败从 _Z 后 2 字节重试
fn demangle_symbols(text: &mut String);

// —— v8: 输出抽象 ——
// trait BacktraceOutputHandler { handle_output(&str); output_file_descriptor() -> RawFd }
// ProcessBacktrace(trace, size, handler):
//   非 handler 态: backtrace_symbols + demangle 逐帧
//   handler 态且有 fd: backtrace_symbols_fd 直写
//   兜底: " [0x...]" itoa_r(16, padding 12) 逐帧
// v8: PrintBacktraceOutputHandler (stderr fd) / StreamBacktraceOutputHandler (ostream)

/// v8: posix EnableInProcessStackDumping — SIGPIPE 忽略 + WarmUpBacktrace +
///     注册 ${signals.join(" ")} (SA_RESETHAND|SA_SIGINFO|SA_ONSTACK)
pub fn enable_in_process_stack_dumping() -> bool;
/// v8: posix DisableSignalStackDump — dump_stack_in_signal_handler = false
pub fn disable_signal_stack_dump();

// v8: StackDumpSignalHandler — in_signal_handler=1; PKU (V8_HAS_PKU_SUPPORT) 时
//     SetDefaultPermissionsForAllKeysInSignalHandler; 打印信号号与 si_code 名;
//     SIGBUS/FPE/ILL/SEGV 附 si_addr; 末尾恢复 SIG_DFL 后 _exit(1)
// si_code 表 (20 项): ${siCodes.map(([a, b]) => `${b}`).join(" ")}

impl StackTrace {
    /// v8: posix StackTrace() — backtrace(trace_, 62); 无 execinfo 时 count_=0
    #[cfg(unix)]
    pub fn capture() -> Self;
    /// v8: posix Print — PrintBacktraceOutputHandler + ProcessBacktrace
    #[cfg(unix)]
    pub fn print(&self);
    /// v8: posix OutputToStream — StreamBacktraceOutputHandler + ProcessBacktrace
    #[cfg(unix)]
    pub fn output_to_stream(&self, os: &mut dyn std::io::Write);
}

// v8: WarmUpBacktrace — 预跑一次 StackTrace, 防 glibc pthread_once 在信号态 malloc 死锁
`;

writeSrc("v8-base", "stack_trace_posix.rs.skel", rust);
console.log("written v8-base/src/stack_trace_posix.rs.skel");
