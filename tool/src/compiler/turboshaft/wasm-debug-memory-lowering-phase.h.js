#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-debug-memory-lowering-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-debug-memory-lowering-phase.{h,cc}
// → v8-turboshaft/src/wasm_debug_memory_lowering_phase.rs.skel
//
// 提取面: phase 声明; DEBUG-only 编译门; 单 reducer 栈。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-debug-memory-lowering-phase.h");
const cc = readV8("src/compiler/turboshaft/wasm-debug-memory-lowering-phase.cc");

// 1. phase 哨兵
for (const [re, name] of [
  [/struct WasmDebugMemoryLoweringPhase \{/, "phase struct"],
  [/DECL_TURBOSHAFT_PHASE_CONSTANTS\(WasmDebugMemoryLowering\)/, "phase 常量宏"],
  [/void Run\(PipelineData\* data, Zone\* temp_zone\);/, "Run 声明"],
  [/#error This header should only be included if WebAssembly is enabled\./, "wasm 门"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc DEBUG 门 + reducer 栈
if (!cc.startsWith("/*\n// Copyright 2025") && !/#ifdef DEBUG\s*\n#include "src\/compiler\/turboshaft\/wasm-debug-memory-lowering-phase.h"/.test(cc)) {
  throw new Error("cc 缺 #ifdef DEBUG 门");
}
const stackM = cc.match(/CopyingPhase<([\s\S]*?)>::Run/);
if (!stackM) throw new Error("cc 未找到 CopyingPhase");
const raw = stackM[1].replace(/\s+/g, "").split(",").filter(Boolean);
if (raw.length !== 1 || raw[0] !== "MemoryOptimizationReducer") {
  throw new Error(`reducer 栈 ${raw.join(",")} != MemoryOptimizationReducer`);
}

let rust = banner(
  "src/compiler/turboshaft/wasm-debug-memory-lowering-phase.h.js",
  "src/compiler/turboshaft/wasm-debug-memory-lowering-phase.{h,cc}",
  "wasm 调试内存降低: DEBUG-only 内存优化",
);

rust += `
/// 原版 WasmDebugMemoryLoweringPhase — 整个 .cc 包在 #ifdef DEBUG 内,
/// 仅调试构建存在 (release 为空翻译单元)。
/// v8: src/compiler/turboshaft/wasm-debug-memory-lowering-phase.{h,cc}
pub struct WasmDebugMemoryLoweringPhase;

impl WasmDebugMemoryLoweringPhase {
  /// v8: cc Run — UnparkedScopeIfNeeded(broker, trace_reduction) 后
  /// CopyingPhase<MemoryOptimizationReducer>::Run
  #[cfg(debug_assertions)]
  pub fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "wasm_debug_memory_lowering_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`reducer栈=1 DEBUG门=1`);
