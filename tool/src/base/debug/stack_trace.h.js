#!/usr/bin/env bun
// tool/src/base/debug/stack_trace.h.js — 族: src/base/debug/stack_trace.h + .cc
// 覆盖: EnableInProcessStackDumping/DisableSignalStackDump + StackTrace 类
//        (kMaxTraces=62, 构造/Addresses/Print/OutputToStream/ToString)
// → v8-base/src/stack_trace.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/debug/stack_trace.h");
const ccSrc = readV8("src/base/debug/stack_trace.cc");

// —— 锚点断言 ——
if (!hSrc.includes("V8_BASE_EXPORT bool EnableInProcessStackDumping();")) throw new Error("缺 Enable 声明");
if (!hSrc.includes("V8_BASE_EXPORT void DisableSignalStackDump();")) throw new Error("缺 Disable 声明");
if (!hSrc.includes("class V8_BASE_EXPORT StackTrace {")) throw new Error("缺 StackTrace 类");
if (!hSrc.includes("static const int kMaxTraces = 62;")) throw new Error("缺 kMaxTraces");
if (!hSrc.includes("void* trace_[kMaxTraces];")) throw new Error("缺 trace_ 数组");
if (!hSrc.includes("size_t count_ = 0u;")) throw new Error("缺 count_");
if (!ccSrc.includes("count = std::min(count, arraysize(trace_));")) throw new Error("缺截断语义");
if (!ccSrc.includes("std::string StackTrace::ToString() const {")) throw new Error("缺 ToString 定义");

// —— 真解析: 方法面计数 ——
const hMethods = ["StackTrace()", "StackTrace(const void* const* trace, size_t count)",
  "Addresses(size_t* count) const", "Print() const", "OutputToStream(std::ostream* os) const", "ToString() const"];
for (const m of hMethods) if (!hSrc.includes(m)) throw new Error(`缺方法 ${m}`);
const winOnly = (hSrc.match(/#if V8_OS_WIN[\s\S]*?#endif/g) || []).length;
console.log(`stack_trace: 全局 2, 类 1, 方法 ${hMethods.length}, win 专属构造 2 + InitTrace 1 (块 ${winOnly}), kMaxTraces 62`);

let rust = banner(
  "src/base/debug/stack_trace.h.js",
  "src/base/debug/stack_trace.h + src/base/debug/stack_trace.cc",
  "栈回溯捕获与符号化输出 (Chromium 移植)",
);

rust += `
//! v8: src/base/debug/stack_trace.h/.cc
//! 平台后端: posix/android/fuchsia/win/zos 各自 .cc (本席另脚本)

/// v8: stack_trace.h EnableInProcessStackDumping — 异常/信号时转储栈并立即退出 (仅测试用)
pub fn enable_in_process_stack_dumping() -> bool;
/// v8: stack_trace.h DisableSignalStackDump
pub fn disable_signal_stack_dump();

/// v8: stack_trace.h StackTrace
/// trace_: void*[62]; count_: 有效帧数
pub struct StackTrace {
    trace: [usize; MAX_TRACES],
    count: usize,
}

/// v8: stack_trace.h kMaxTraces = 62 (MSDN: FramesToSkip+FramesToCapture < 63)
pub const MAX_TRACES: usize = 62;

impl StackTrace {
    /// v8: stack_trace.h StackTrace() — 从当前位置捕获 (平台后端实现)
    pub fn capture() -> Self;
    /// v8: stack_trace.cc StackTrace(trace, count) — 外部指针数组, count 截到 kMaxTraces
    pub fn from_addresses(trace: &[usize]) -> Self;
    /// v8: stack_trace.cc Addresses — 返回有效帧切片 (count_==0 返回空)
    pub fn addresses(&self) -> &[usize];
    /// v8: stack_trace.h Print — 打到 stderr
    pub fn print(&self);
    /// v8: stack_trace.h OutputToStream — 符号化写流 (posix: backtrace_symbols)
    pub fn output_to_stream(&self, os: &mut dyn std::io::Write);
    /// v8: stack_trace.cc ToString — OutputToStream 到 stringstream
    pub fn to_string(&self) -> String;
}
`;

writeSrc("v8-base", "stack_trace.rs.skel", rust);
console.log("written v8-base/src/stack_trace.rs.skel");
