#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-gc-optimize-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-gc-optimize-phase.{h,cc}
// → v8-turboshaft/src/wasm_gc_optimize_phase.rs.skel
//
// 提取面: phase 声明; 2-reducer 栈 (load 消除 + GC 类型)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-gc-optimize-phase.h");
const cc = readV8("src/compiler/turboshaft/wasm-gc-optimize-phase.cc");

// 1. phase 哨兵
for (const [re, name] of [
  [/struct WasmGCOptimizePhase \{/, "phase struct"],
  [/DECL_TURBOSHAFT_PHASE_CONSTANTS\(WasmGCOptimize\)/, "phase 常量宏"],
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
if (raw[0] !== "WasmLoadEliminationReducer" || raw[1] !== "WasmGCTypedOptimizationReducer") {
  throw new Error(`栈序 ${raw.join(",")} 不符`);
}

let rust = banner(
  "src/compiler/turboshaft/wasm-gc-optimize-phase.h.js",
  "src/compiler/turboshaft/wasm-gc-optimize-phase.{h,cc}",
  "wasm GC 优化: 载入消除 + 类型优化",
);

rust += `
/// 原版 WasmGCOptimizePhase — GC 语言扩展的优化期。
/// v8: src/compiler/turboshaft/wasm-gc-optimize-phase.{h,cc}
pub struct WasmGCOptimizePhase;

impl WasmGCOptimizePhase {
  /// v8: cc Run — UnparkedScopeIfNeeded(broker, trace_reduction) 后
  /// CopyingPhase<WasmLoadEliminationReducer, WasmGCTypedOptimizationReducer>::Run
  pub fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "wasm_gc_optimize_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`reducer栈=${raw.length}`);
