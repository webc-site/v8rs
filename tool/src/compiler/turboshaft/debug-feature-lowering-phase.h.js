#!/usr/bin/env bun
// tool/src/compiler/turboshaft/debug-feature-lowering-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/debug-feature-lowering-phase.{h,cc}
// → v8-turboshaft/src/debug_feature_lowering_phase.rs.skel
//
// 提取面: DebugFeatureLoweringPhase — 整条 CopyingPhase 链包在
// #ifdef V8_ENABLE_DEBUG_CODE 内 (release 构建空跑)。对数基准来自 cc 原文。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/debug-feature-lowering-phase.h");
const cc = readV8("src/compiler/turboshaft/debug-feature-lowering-phase.cc");

// 1. phase 结构
const structM = h.match(/struct (\w+Phase)\s*\{([\s\S]*?)\n\};/);
if (!structM) throw new Error("未找到 phase struct");
if (structM[1] !== "DebugFeatureLoweringPhase") throw new Error(`struct 名 ${structM[1]} != DebugFeatureLoweringPhase`);
const declM = structM[2].match(/DECL_TURBOSHAFT_PHASE_CONSTANTS(?:_WITH_LEGACY_NAME)?\((\w+)\)/);
if (!declM) throw new Error("未找到 DECL_TURBOSHAFT_PHASE_CONSTANTS");
if (declM[1] !== "DebugFeatureLowering") throw new Error(`DECL 名 ${declM[1]} != DebugFeatureLowering`);
const phaseName = `V8.TFTurboshaft${declM[1]}`;

// 2. reducer 链: CopyingPhase<...>::Run — 链整体在 #ifdef V8_ENABLE_DEBUG_CODE 内
const chainM = cc.match(/CopyingPhase<([\s\S]*?)>::Run/);
if (!chainM) throw new Error("未找到 CopyingPhase<...>::Run");
const ordered = [...new Set([...chainM[1].replace(/\/\/[^\n]*/g, "").matchAll(/\b([A-Z]\w*Reducer)\b/g)].map((m) => m[1]))];
if (JSON.stringify(ordered) !== JSON.stringify(["DebugFeatureLoweringReducer", "InstructionSelectionNormalizationReducer"])) {
  throw new Error(`reducer 链不匹配: ${JSON.stringify(ordered)}`);
}
// 链包裹条件断言: Run 体内 #ifdef 在 CopyingPhase 之前出现
const runBody = cc.match(/void DebugFeatureLoweringPhase::Run\(([\s\S]*?)\n\}/);
if (!runBody) throw new Error("未找到 Run 函数体");
const ifdefPos = runBody[1].indexOf("#ifdef V8_ENABLE_DEBUG_CODE");
const chainPos = runBody[1].indexOf("CopyingPhase");
if (ifdefPos < 0 || chainPos < 0 || ifdefPos > chainPos) {
  throw new Error("CopyingPhase 链未被 #ifdef V8_ENABLE_DEBUG_CODE 包裹");
}

// 3. 哨兵
for (const [frag, label] of [
  ['#include "src/compiler/turboshaft/debug-feature-lowering-reducer.h"', "include debug-feature-lowering-reducer"],
  ['#include "src/compiler/turboshaft/phase.h"', "include phase.h"],
]) {
  if (!(h.includes(frag) || cc.includes(frag))) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/debug-feature-lowering-phase.h.js",
  "src/compiler/turboshaft/debug-feature-lowering-phase.{h,cc}",
  "调试特性降级 phase: DEBUG_CODE 条件链编排骨架",
);
rust += `
/// v8: src/compiler/turboshaft/debug-feature-lowering-phase.h DebugFeatureLoweringPhase
pub struct DebugFeatureLoweringPhase;

impl DebugFeatureLoweringPhase {
  /// 原版 kPhaseName = "V8.TF" "Turboshaft" "DebugFeatureLowering"
  pub const PHASE_NAME: &'static str = "${phaseName}";

  /// CopyingPhase reducer 链 (保序) — 原版整体包在 #ifdef V8_ENABLE_DEBUG_CODE 内,
  /// release 构建下 Run 为空操作
  pub const REDUCERS: &'static [&'static str] = &[
    "DebugFeatureLoweringReducer",
    "InstructionSelectionNormalizationReducer",
  ];

  /// 原版 void Run(PipelineData* data, Zone* temp_zone)
  /// v8: src/compiler/turboshaft/debug-feature-lowering-phase.cc DebugFeatureLoweringPhase::Run
  pub fn run(data: &mut crate::phase::PipelineData, temp_zone: &mut crate::phase::Zone) {
    // 原版: #ifdef V8_ENABLE_DEBUG_CODE CopyingPhase<...>::Run(data, temp_zone); #endif
    unimplemented!("Developer 修订: DEBUG_CODE 条件下编排链")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(DebugFeatureLoweringPhase::PHASE_NAME == "V8.TFTurboshaftDebugFeatureLowering");
  assert!(DebugFeatureLoweringPhase::REDUCERS.len() == ${ordered.length});
};
`;

const p = writeSrc("v8-turboshaft", "debug_feature_lowering_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`phase=1 decl=${declM[1]} 链=${ordered.length} 包裹=#ifdef_V8_ENABLE_DEBUG_CODE`);
