#!/usr/bin/env bun
// tool/src/compiler/turboshaft/decompression-optimization-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/decompression-optimization-phase.{h,cc}
// → v8-turboshaft/src/decompression_optimization_phase.rs.skel
//
// 提取面: DecompressionOptimizationPhase — 非 CopyingPhase, Run 直接调
// RunDecompressionOptimization。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/decompression-optimization-phase.h");
const cc = readV8("src/compiler/turboshaft/decompression-optimization-phase.cc");

// 1. phase 结构
const structM = h.match(/struct (\w+Phase)\s*\{([\s\S]*?)\n\};/);
if (!structM) throw new Error("未找到 phase struct");
if (structM[1] !== "DecompressionOptimizationPhase") {
  throw new Error(`struct 名 ${structM[1]} != DecompressionOptimizationPhase`);
}
const declM = structM[2].match(/DECL_TURBOSHAFT_PHASE_CONSTANTS(?:_WITH_LEGACY_NAME)?\((\w+)\)/);
if (!declM) throw new Error("未找到 DECL_TURBOSHAFT_PHASE_CONSTANTS");
if (declM[1] !== "DecompressionOptimization") throw new Error(`DECL 名 ${declM[1]} != DecompressionOptimization`);
const phaseName = `V8.TFTurboshaft${declM[1]}`;

// 2. cc: Run 定义 + 直接调用 (无 CopyingPhase)
if (!/void DecompressionOptimizationPhase::Run\(PipelineData\* data, Zone\* temp_zone\)/.test(cc)) {
  throw new Error("cc 未找到 Run 定义");
}
const callM = cc.match(/RunDecompressionOptimization\(data->graph\(\), temp_zone\)/);
if (!callM) throw new Error("cc 未找到 RunDecompressionOptimization(data->graph(), temp_zone) 调用");
if (/CopyingPhase/.test(cc)) throw new Error("cc 不应出现 CopyingPhase");

// 3. 哨兵
for (const [frag, label] of [
  ['#include "src/compiler/turboshaft/decompression-optimization.h"', "include decompression-optimization"],
  ['#include "src/compiler/turboshaft/phase.h"', "include phase.h"],
]) {
  if (!(h.includes(frag) || cc.includes(frag))) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/decompression-optimization-phase.h.js",
  "src/compiler/turboshaft/decompression-optimization-phase.{h,cc}",
  "解压优化 phase: 直调 RunDecompressionOptimization 骨架",
);
rust += `
/// v8: src/compiler/turboshaft/decompression-optimization-phase.h DecompressionOptimizationPhase
pub struct DecompressionOptimizationPhase;

impl DecompressionOptimizationPhase {
  /// 原版 kPhaseName = "V8.TF" "Turboshaft" "DecompressionOptimization"
  pub const PHASE_NAME: &'static str = "${phaseName}";

  /// 原版 void Run(PipelineData* data, Zone* temp_zone)
  /// v8: src/compiler/turboshaft/decompression-optimization-phase.cc DecompressionOptimizationPhase::Run
  ///
  /// 非 CopyingPhase — 直接: RunDecompressionOptimization(data->graph(), temp_zone);
  /// (实参为 Zone* 指针)
  pub fn run(data: &mut crate::phase::PipelineData, temp_zone: &mut crate::phase::Zone) {
    unimplemented!("Developer 修订: 委托 run_decompression_optimization")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(DecompressionOptimizationPhase::PHASE_NAME == "V8.TFTurboshaftDecompressionOptimization");
};
`;

const p = writeSrc("v8-turboshaft", "decompression_optimization_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`phase=1 decl=${declM[1]} 调用=RunDecompressionOptimization(graph,temp_zone) CopyingPhase=0`);
