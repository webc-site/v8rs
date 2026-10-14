#!/usr/bin/env bun
// tool/src/base/debug/stack_trace_android.cc.js — 族: src/base/debug/stack_trace_android.cc
// 覆盖: _Unwind_Backtrace 捕获 + StackCrawlState + SIGPIPE 忽略
// → v8-base/src/stack_trace_android.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/base/debug/stack_trace_android.cc");

// —— 锚点断言 ——
if (!src.includes("struct StackCrawlState {")) throw new Error("缺 StackCrawlState");
if (!src.includes("_Unwind_Reason_Code TraceStackFrame(_Unwind_Context* context, void* arg)")) throw new Error("缺回帧回调");
if (!src.includes("if (ip != 0 && !state->have_skipped_self) {")) throw new Error("缺跳过自身帧");
if (!src.includes("_Unwind_Backtrace(&TraceStackFrame, &state);")) throw new Error("缺 backtrace 调用");
if (!src.includes("action.sa_handler = SIG_IGN;")) throw new Error("缺 SIGPIPE 忽略");
if (!src.includes("return (sigaction(SIGPIPE, &action, nullptr) == 0);")) throw new Error("缺 sigaction 判定");

// —— 真解析: 字段与语义计数 ——
const stateFields = ["uintptr_t* frames", "size_t frame_count", "size_t max_depth", "bool have_skipped_self"];
for (const f of stateFields) if (!src.includes(`  ${f};`)) throw new Error(`缺字段 ${f}`);
if (!src.includes("return _URC_END_OF_STACK;")) throw new Error("缺达深终止");
console.log("stack_trace_android: 回帧回调 1, 状态字段 4, Enable=SIGPIPE 忽略, 输出 #NN+地址");

let rust = banner(
  "src/base/debug/stack_trace_android.cc.js",
  "src/base/debug/stack_trace_android.cc",
  "Android 栈回溯 (_Unwind_Backtrace)",
);

rust += `
//! v8: src/base/debug/stack_trace_android.cc — 仅 Android 构建

// v8: struct StackCrawlState { frames, frame_count, max_depth, have_skipped_self }
// v8: TraceStackFrame — _Unwind_Backtrace 回调: 首个非零 ip 是回调自身, 跳过一帧;
//     帧数达 max_depth 返回 _URC_END_OF_STACK

/// v8: android EnableInProcessStackDumping — 仅把 SIGPIPE 设为 SIG_IGN (返回 sigaction==0)
pub fn enable_in_process_stack_dumping() -> bool;
/// v8: android DisableSignalStackDump — 空实现
pub fn disable_signal_stack_dump();

impl StackTrace {
    /// v8: android StackTrace() — _Unwind_Backtrace 填 trace_[..kMaxTraces]
    #[cfg(target_os = "android")]
    pub fn capture() -> Self;
    /// v8: android OutputToStream — "#<setw2 i><ip>" 逐帧
    #[cfg(target_os = "android")]
    pub fn output_to_stream(&self, os: &mut dyn std::io::Write);
}
`;

writeSrc("v8-base", "stack_trace_android.rs.skel", rust);
console.log("written v8-base/src/stack_trace_android.rs.skel");
