#!/usr/bin/env bun
// tool/src/compiler/turboshaft/build-graph-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/build-graph-phase.{h,cc}
// → v8-turboshaft/src/build_graph_phase.rs.skel
//
// 提取面: BuildGraphPhase (唯一带 Schedule→Graph 构建入口的 phase,
// Run 返回 std::optional<BailoutReason>, 参数表 4 项) + 调用约定
// BuildGraph(data, schedule, temp_zone, linkage)。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/build-graph-phase.h");
const cc = readV8("src/compiler/turboshaft/build-graph-phase.cc");

// 1. phase 结构: Run 签名与常规 void Run 不同 — optional<BailoutReason> + 4 参数
const structM = h.match(/struct (\w+Phase)\s*\{([\s\S]*?)\n\};/);
if (!structM) throw new Error("未找到 BuildGraphPhase struct");
const phaseStruct = structM[1];
if (phaseStruct !== "BuildGraphPhase") throw new Error(`struct 名 ${phaseStruct} != BuildGraphPhase`);

const declM = structM[2].match(/DECL_TURBOSHAFT_PHASE_CONSTANTS(?:_WITH_LEGACY_NAME)?\((\w+)\)/);
if (!declM) throw new Error("未找到 DECL_TURBOSHAFT_PHASE_CONSTANTS");
if (declM[1] !== "BuildGraph") throw new Error(`DECL 名 ${declM[1]} != BuildGraph`);
const phaseName = `V8.TFTurboshaft${declM[1]}`;

// Run 签名: 4 参数 (data, temp_zone, schedule, linkage)
const runM = structM[2].match(
  /std::optional<BailoutReason> Run\(PipelineData\* data,\s*Zone\* temp_zone,\s*Schedule\* schedule,\s*Linkage\* linkage\);/,
);
if (!runM) throw new Error("Run 签名缺失 (optional<BailoutReason> Run(PipelineData*, Zone*, Schedule*, Linkage*))");

// 2. cc 侧: Run 定义 + BuildGraph 调用 (无 CopyingPhase)
const defM = cc.match(/std::optional<BailoutReason> BuildGraphPhase::Run\(/);
if (!defM) throw new Error("cc 未找到 BuildGraphPhase::Run 定义");
const callM = cc.match(/turboshaft::BuildGraph\(data, schedule, temp_zone, linkage\)/);
if (!callM) throw new Error("cc 未找到 BuildGraph(data, schedule, temp_zone, linkage) 调用");

// 哨兵: UnparkedScopeIfNeeded + DCHECK_NOT_NULL(schedule)
const sentinels = [
  [/DCHECK_NOT_NULL\(schedule\);/, "DCHECK_NOT_NULL(schedule)"],
  [/UnparkedScopeIfNeeded scope\(data->broker\(\)\);/, "UnparkedScopeIfNeeded"],
  ['#include "src/compiler/turboshaft/phase.h"', "include phase.h"],
];
for (const [re, label] of sentinels) {
  const src = typeof re === "string" ? (h.includes(re) || cc.includes(re)) : re.test(cc);
  if (!src) throw new Error(`哨兵缺失: ${label}`);
}

// 3. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/build-graph-phase.h.js",
  "src/compiler/turboshaft/build-graph-phase.{h,cc}",
  "Turbofan Schedule → Turboshaft Graph 构建 phase 骨架",
);
rust += `
/// v8: src/compiler/turboshaft/build-graph-phase.h BuildGraphPhase
pub struct BuildGraphPhase;

impl BuildGraphPhase {
  /// 原版 kPhaseName = "V8.TF" "Turboshaft" "BuildGraph" (宏拼接展开)
  pub const PHASE_NAME: &'static str = "${phaseName}";

  /// 原版 std::optional<BailoutReason> Run(PipelineData*, Zone*, Schedule*, Linkage*)
  /// v8: src/compiler/turboshaft/build-graph-phase.cc BuildGraphPhase::Run
  ///
  /// 流程 (原版): UnparkedScopeIfNeeded → 调 BuildGraph(data, schedule,
  /// temp_zone, linkage), 返回 bailout 即提前退出, 否则 {}。
  pub fn run(
    data: &mut crate::phase::PipelineData,
    temp_zone: &mut crate::phase::Zone,
    schedule: &mut crate::phase::Schedule,
    linkage: &mut crate::phase::Linkage,
  ) -> Option<crate::phase::BailoutReason> {
    // 原版: if (auto bailout = turboshaft::BuildGraph(data, schedule, temp_zone, linkage)) return bailout;
    unimplemented!("Developer 修订: 委托 graph-builder 的 BuildGraph")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BuildGraphPhase::PHASE_NAME == "V8.TFTurboshaftBuildGraph");
};
`;

const p = writeSrc("v8-turboshaft", "build_graph_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`phase=1 decl=${declM[1]} run参数=4 调用=BuildGraph(data,schedule,temp_zone,linkage)`);
