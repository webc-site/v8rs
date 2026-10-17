#!/usr/bin/env bun
// tool/src/base/debug/stack_trace_fuchsia.cc.js — 族: src/base/debug/stack_trace_fuchsia.cc
// 覆盖: Fuchsia 后端 — Enable 恒 false (系统 crashlogger 负责回溯)
// → v8-base/src/stack_trace_fuchsia.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/base/debug/stack_trace_fuchsia.cc");

// —— 锚点断言 ——
if (!src.includes("bool EnableInProcessStackDumping() {\n  // The system crashlogger")) throw new Error("缺 crashlogger 注释锚");
if (!src.includes("  return false;\n}")) throw new Error("缺恒 false 返回");
if (!src.includes("void DisableSignalStackDump() {}")) throw new Error("缺空 Disable");
if (!src.includes("StackTrace::StackTrace() {}")) throw new Error("缺空捕获构造");
if (!src.includes('*os << "#" << std::setw(2) << i << trace_[i] << "\\n";')) throw new Error("缺帧输出格式");
console.log("stack_trace_fuchsia: Enable 恒 false, 构造/Disable 空, 输出 #NN+地址");

let rust = banner(
  "src/base/debug/stack_trace_fuchsia.cc.js",
  "src/base/debug/stack_trace_fuchsia.cc",
  "Fuchsia 后端 (系统 crashlogger 代劳, 进程内不捕获)",
);

rust += `
//! v8: src/base/debug/stack_trace_fuchsia.cc — 仅 Fuchsia 构建

/// v8: fuchsia EnableInProcessStackDumping — 恒 false: 系统崩溃日志器抓回溯,
///     host 侧 addr2line 符号化; 设备上无符号, 进程内捕获无意义
pub fn enable_in_process_stack_dumping() -> bool { false }

/// v8: fuchsia DisableSignalStackDump — 空
pub fn disable_signal_stack_dump() {}

impl StackTrace {
    /// v8: fuchsia StackTrace() — 空构造 (trace 全零)
    #[cfg(target_os = "fuchsia")]
    pub fn capture() -> Self { Self { trace: [0; MAX_TRACES], count: 0 } }
    /// v8: fuchsia OutputToStream — "#<setw2 i><ip>" (沿用通用格式)
    #[cfg(target_os = "fuchsia")]
    pub fn output_to_stream(&self, os: &mut dyn std::io::Write);
}
`;

writeSrc("v8-base", "stack_trace_fuchsia.rs.skel", rust);
console.log("written v8-base/src/stack_trace_fuchsia.rs.skel");
