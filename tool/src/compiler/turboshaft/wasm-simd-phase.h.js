#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-simd-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-simd-phase.{h,cc}
// → v8-turboshaft/src/wasm_simd_phase.rs.skel
//
// 提取面: phase 声明; analyzer 门控流程; 单 reducer。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-simd-phase.h");
const cc = readV8("src/compiler/turboshaft/wasm-simd-phase.cc");

// 1. phase 哨兵
for (const [re, name] of [
  [/struct WasmSimdPhase \{/, "phase struct"],
  [/DECL_TURBOSHAFT_PHASE_CONSTANTS\(WasmSimd\)/, "phase 常量宏"],
  [/void Run\(PipelineData\* data, Zone\* temp_zone\);/, "Run 声明"],
  [/#error This header should only be included if WebAssembly is enabled\./, "wasm 门"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc 流程
for (const [re, name] of [
  [/WasmShuffleAnalyzer analyzer\(temp_zone, data->graph\(\)\);/, "analyzer 构造"],
  [/analyzer\.Run\(\);/, "analyzer 运行"],
  [/analyzer\.ShouldReduce\(\)/, "门控判定"],
  [/data->set_wasm_shuffle_analyzer\(&analyzer\);/, "挂 analyzer 到 data"],
  [/data->clear_wasm_shuffle_analyzer\(\);/, "卸 analyzer"],
  [/CopyingPhase<WasmShuffleReducer>::Run/, "单 reducer 复制"],
]) {
  if (!re.test(cc)) throw new Error(`cc 哨兵缺失: ${name}`);
}

let rust = banner(
  "src/compiler/turboshaft/wasm-simd-phase.h.js",
  "src/compiler/turboshaft/wasm-simd-phase.{h,cc}",
  "wasm SIMD: shuffle 降低 (256 位重向量化前置)",
);

rust += `
/// 原版 WasmSimdPhase — 先分析后条件执行。
/// v8: src/compiler/turboshaft/wasm-simd-phase.{h,cc}
pub struct WasmSimdPhase;

impl WasmSimdPhase {
  /// v8: cc Run —
  /// 1. WasmShuffleAnalyzer(temp_zone, graph).run()
  /// 2. should_reduce() 为真才:
  ///    set_wasm_shuffle_analyzer(&analyzer)
  ///    CopyingPhase<WasmShuffleReducer>::Run
  ///    clear_wasm_shuffle_analyzer()
  pub fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "wasm_simd_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`流程步=6 reducer=1`);
