#!/usr/bin/env bun
// tool/src/diagnostics/arm64/unwinder-arm64.cc.js
// 机械提取 v8/src/diagnostics/arm64/unwinder-arm64.cc (空实现桩) → v8-codegen-arm64/src/unwinder.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/diagnostics/arm64/unwinder-arm64.cc");

// 对数: 唯一桩函数
if (!src.includes("void GetCalleeSavedRegistersFromEntryFrame(void* fp,"))
  throw new Error("缺 GetCalleeSavedRegistersFromEntryFrame");
if (!src.includes("struct RegisterState;")) throw new Error("缺 RegisterState 前置声明");

let rust = banner(
  "src/diagnostics/arm64/unwinder-arm64.cc.js",
  "src/diagnostics/arm64/unwinder-arm64.cc",
  "arm64 unwinder 桩 (EntryFrame 被调方保存寄存器)",
);

rust += `
/// 原版 GetCalleeSavedRegistersFromEntryFrame(fp, register_state):
/// arm64 平台为空实现桩。
/// RegisterState 定义见 v8-profiler 侧 (前置声明)。
pub fn get_callee_saved_registers_from_entry_frame(
  _fp: *mut (),
  _register_state: *mut (),
) {
  // arm64: 空实现 (原版语义)
}
`;

const p = writeSrc("v8-codegen-arm64", "unwinder.rs", rust);
console.log(`unwinder-arm64.cc: 桩 1 → ${p}`);
