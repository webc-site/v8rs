#!/usr/bin/env bun
// tool/src/base/debug/stack_trace_win.cc.js — 族: src/base/debug/stack_trace_win.cc
// 覆盖: 异常过滤器 + InitializeSymbols (dbghelp) + OutputTraceToStream + InitTrace (StackWalk64)
// → v8-base/src/stack_trace_win.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/base/debug/stack_trace_win.cc");

// —— 锚点断言 ——
if (!src.includes("long WINAPI StackDumpExceptionFilter(EXCEPTION_POINTERS* info)")) throw new Error("缺异常过滤器");
if (!src.includes("bool InitializeSymbols() {")) throw new Error("缺符号初始化");
if (!src.includes("SymSetOptions(SYMOPT_DEFERRED_LOADS | SYMOPT_UNDNAME | SYMOPT_LOAD_LINES);")) throw new Error("缺 Sym 选项");
if (!src.includes("SymInitialize(GetCurrentProcess(), nullptr, TRUE)")) throw new Error("缺 SymInitialize");
if (!src.includes('::LoadLibrary(L"msdia140.dll");')) throw new Error("缺 msdia 预载");
if (!src.includes("void OutputTraceToStream(const void* const* trace, size_t count,")) throw new Error("缺符号化输出");
if (!src.includes("const int kMaxNameLength = 256;")) throw new Error("缺名字缓冲");
if (!src.includes("void StackTrace::InitTrace(const CONTEXT* context_record) {")) throw new Error("缺 InitTrace");
if (!src.includes("while (StackWalk64(machine_type, GetCurrentProcess(), GetCurrentThread(),")) throw new Error("缺 StackWalk64 循环");
if (!src.includes("g_previous_filter = SetUnhandledExceptionFilter(&StackDumpExceptionFilter);")) throw new Error("缺过滤器安装");

// —— 真解析: 构造器与全局计数 ——
const ctors = [...src.matchAll(/^StackTrace::StackTrace\(([^)]*)\) \{/gm)].map((m) => m[1].trim());
if (ctors.length !== 3) throw new Error(`构造器 ${ctors.length} != 3`);
if (!ctors.includes("")) throw new Error("缺默认构造");
if (!ctors.some((c) => c.includes("EXCEPTION_POINTERS*"))) throw new Error("缺异常构造");
if (!ctors.some((c) => c.includes("const CONTEXT*"))) throw new Error("缺 context 构造");
const globals = ["g_previous_filter", "g_dump_stack_in_signal_handler", "g_initialized_symbols", "g_init_error"];
for (const g of globals) if (!src.includes(g)) throw new Error(`缺全局 ${g}`);
console.log(`stack_trace_win: 构造器 3, 全局 4, dbghelp API 6+ (Sym*/StackWalk64), kMaxNameLength 256`);

let rust = banner(
  "src/base/debug/stack_trace_win.cc.js",
  "src/base/debug/stack_trace_win.cc",
  "Windows 后端 (dbghelp 符号化 + StackWalk64)",
);

rust += `
//! v8: src/base/debug/stack_trace_win.cc — 仅 Windows 构建

// v8: 全局: g_previous_filter / g_dump_stack_in_signal_handler / g_initialized_symbols / g_init_error

/// v8: win StackDumpExceptionFilter — 测试进程异常过滤器:
/// 开关开时 StackTrace(info).Print(), 链回 g_previous_filter, 否则 EXCEPTION_CONTINUE_SEARCH
fn stack_dump_exception_filter(info: *mut EXCEPTION_POINTERS) -> i32;

/// v8: win InitializeSymbols — 预载 msdia140.dll (SDK 10.0.28000+ 动态加载),
/// SymSetOptions(DEFERRED_LOADS|UNDNAME|LOAD_LINES), SymInitialize,
/// 搜索路径追加可执行目录 (kSymbolsArraySize=1024 wchar)
fn initialize_symbols() -> bool;

// v8: OutputTraceToStream — 每帧格式: <tab>SymbolName[0xAddr+Offset] (File:Line);
//     kMaxNameLength=256; SYMBOL_INFO 缓冲 ULONG64 对齐; 失败帧回退地址输出
// v8: StackTrace::InitTrace — 复制 CONTEXT (StackWalk64 就地改寄存器),
//     machine_type 按 CPU 架构选, 循环 StackWalk64(…, SymFunctionTableAccess64,
//     SymGetModuleBase64, …) 填 trace_ 直至 kMaxTraces

impl StackTrace {
    /// v8: win StackTrace() — 无异常捕获 (空)
    #[cfg(windows)]
    pub fn capture() -> Self;
    /// v8: win StackTrace(EXCEPTION_POINTERS*) / (const CONTEXT*) — InitTrace 走 StackWalk64
    #[cfg(windows)]
    pub fn from_exception(info: *mut EXCEPTION_POINTERS) -> Self;
    /// v8: win Print — OutputToStream(std::cerr)
    #[cfg(windows)]
    pub fn print(&self);
    /// v8: win OutputToStream — InitializeSymbols + OutputTraceToStream
    #[cfg(windows)]
    pub fn output_to_stream(&self, os: &mut dyn std::io::Write);
}

// v8: win EnableInProcessStackDumping — SetUnhandledExceptionFilter + 开开关;
//     win DisableSignalStackDump — 关开关
`;

writeSrc("v8-base", "stack_trace_win.rs.skel", rust);
console.log("written v8-base/src/stack_trace_win.rs.skel");
