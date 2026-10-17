#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-optimize-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-optimize-phase.{h,cc}
// → v8-turboshaft/src/wasm_optimize_phase.rs.skel
//
// 提取面: phase 声明; 7-reducer 优化栈。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-optimize-phase.h");
const cc = readV8("src/compiler/turboshaft/wasm-optimize-phase.cc");

// 1. phase 哨兵
for (const [re, name] of [
  [/struct WasmOptimizePhase \{/, "phase struct"],
  [/DECL_TURBOSHAFT_PHASE_CONSTANTS\(WasmOptimize\)/, "phase 常量宏"],
  [/void Run\(PipelineData\* data, Zone\* temp_zone\);/, "Run 声明"],
  [/#error This header should only be included if WebAssembly is enabled\./, "wasm 门"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc reducer 栈
const stackM = cc.match(/CopyingPhase<([\s\S]*?)>::Run/);
if (!stackM) throw new Error("cc 未找到 CopyingPhase");
const raw = stackM[1].replace(/\s+/g, "").split(",").filter(Boolean);
if (raw.length !== 7) throw new Error(`reducer 栈 ${raw.length} != 7`);
for (const must of ["LateEscapeAnalysisReducer", "MachineOptimizationReducer", "MemoryOptimizationReducer", "BranchEliminationReducer", "LateLoadEliminationReducer", "WasmCodeCoverageReducer", "ValueNumberingReducer"]) {
  if (!raw.includes(must)) throw new Error(`栈缺 ${must}`);
}

let rust = banner(
  "src/compiler/turboshaft/wasm-optimize-phase.h.js",
  "src/compiler/turboshaft/wasm-optimize-phase.{h,cc}",
  "wasm 优化期: 逃逸/机器/内存/分支/载入/GVN",
);

rust += `
/// 原版 WasmOptimizePhase — wasm 主优化期。
/// v8: src/compiler/turboshaft/wasm-optimize-phase.{h,cc}
pub struct WasmOptimizePhase;

impl WasmOptimizePhase {
  /// v8: cc Run — UnparkedScopeIfNeeded(broker, trace_reduction) 后
  /// CopyingPhase<${raw.join(",\n  ///                ")}>::Run
  pub fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "wasm_optimize_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`reducer栈=${raw.length}`);
