#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-lowering-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-lowering-phase.{h,cc}
// → v8-turboshaft/src/wasm_lowering_phase.rs.skel
//
// 提取面: phase 声明; 2-reducer 栈 (wasm 降低 + 机器优化)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-lowering-phase.h");
const cc = readV8("src/compiler/turboshaft/wasm-lowering-phase.cc");

// 1. phase 哨兵
for (const [re, name] of [
  [/struct WasmLoweringPhase \{/, "phase struct"],
  [/DECL_TURBOSHAFT_PHASE_CONSTANTS\(WasmLowering\)/, "phase 常量宏"],
  [/void Run\(PipelineData\* data, Zone\* temp_zone\);/, "Run 声明"],
  [/#error This header should only be included if WebAssembly is enabled\./, "wasm 门"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc reducer 栈
const stackM = cc.match(/CopyingPhase<([\s\S]*?)>::Run/);
if (!stackM) throw new Error("cc 未找到 CopyingPhase");
const raw = stackM[1].replace(/\s+/g, "").split(",").filter(Boolean);
if (raw.length !== 2) throw new Error(`reducer 栈 ${raw.length} != 2`);
if (raw[0] !== "WasmLoweringReducer" || raw[1] !== "MachineOptimizationReducer") {
  throw new Error(`栈序 ${raw.join(",")} 不符`);
}

// 3. 动机注释
if (!/help the late load\s*\n\s*\/\/ elimination that follows this phase/.test(cc)) {
  throw new Error("缺 late-load-elimination 动机注释");
}

let rust = banner(
  "src/compiler/turboshaft/wasm-lowering-phase.h.js",
  "src/compiler/turboshaft/wasm-lowering-phase.{h,cc}",
  "wasm 降低期: GC 操作到机器操作",
);

rust += `
/// 原版 WasmLoweringPhase — wasm GC 操作降为机器级。
/// v8: src/compiler/turboshaft/wasm-lowering-phase.{h,cc}
pub struct WasmLoweringPhase;

impl WasmLoweringPhase {
  /// v8: cc Run — UnparkedScopeIfNeeded(broker, trace_reduction) 后
  /// CopyingPhase<WasmLoweringReducer, MachineOptimizationReducer>::Run
  /// MachineOptimization 同跑 (cc 注释): 助后续 late load
  /// elimination 消更多载入。
  pub fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "wasm_lowering_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`reducer栈=${raw.length}`);
