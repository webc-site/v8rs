#!/usr/bin/env bun
// tool/src/compiler/turboshaft/loop-peeling-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/loop-peeling-phase.{h,cc}
// → v8-turboshaft/src/loop_peeling_phase.rs.skel
//
// 提取面: LoopPeelingPhase 结构 (DECL_TURBOSHAFT_PHASE_CONSTANTS 展开的
// kPhaseName 命名约定) + Run 的 CopyingPhase reducer 链 (保序)。
// 语义实现禁编造, Run 仅登记签名骨架。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/loop-peeling-phase.h");
const cc = readV8("src/compiler/turboshaft/loop-peeling-phase.cc");

// ---------------------------------------------------------------------------
// 1. phase 结构: struct 名 + DECL 宏 + Run 签名
// ---------------------------------------------------------------------------
const structM = h.match(/struct (\w+Phase)\s*\{([\s\S]*?)\n\};/);
if (!structM) throw new Error("未找到 LoopPeelingPhase struct");
const phaseStruct = structM[1];
if (phaseStruct !== "LoopPeelingPhase") throw new Error(`struct 名 ${phaseStruct} != LoopPeelingPhase`);

const declM = structM[2].match(/DECL_TURBOSHAFT_PHASE_CONSTANTS(?:_WITH_LEGACY_NAME)?\((\w+)\)/);
if (!declM) throw new Error("未找到 DECL_TURBOSHAFT_PHASE_CONSTANTS");
// DECL_TURBOSHAFT_PHASE_CONSTANTS(Name) → CallStatsName = Turboshaft##Name
// kPhaseName = "V8.TF" "Turboshaft" #Name
const declName = declM[1];
if (declName !== "LoopPeeling") throw new Error(`DECL 名 ${declName} != LoopPeeling`);
const phaseName = `V8.TFTurboshaft${declName}`;

const runM = structM[2].match(/void Run\(PipelineData\* data, Zone\* temp_zone\);/);
if (!runM) throw new Error("Run 签名缺失 (void Run(PipelineData*, Zone*))");

// ---------------------------------------------------------------------------
// 2. reducer 链 (copying-phase.cc): CopyingPhase<A, B, ...>::Run 保序提取
// ---------------------------------------------------------------------------
const chainM = cc.match(/CopyingPhase<([\s\S]*?)>::Run/);
if (!chainM) throw new Error("未找到 CopyingPhase<...>::Run");
const chainBody = chainM[1].replace(/\/\/[^\n]*/g, "");
const reducers = [...chainBody.matchAll(/\b([A-Z]\w*Reducer)\b/g)].map((m) => m[1]);
const ordered = [...new Set(reducers)];

// 关键锚点: 链首/链尾 + 总数 (对数基准来自 cc 原文)
if (ordered.length !== 3) throw new Error(`reducer 链 ${ordered.length} != 3`);
if (ordered[0] !== "LoopPeelingReducer") throw new Error(`链首 ${ordered[0]} != LoopPeelingReducer`);
if (ordered[ordered.length - 1] !== "ValueNumberingReducer") throw new Error(`链尾 != ValueNumberingReducer`);

// 哨兵: cc include 该 reducer 头 + phase.h 在 h
for (const [frag, label] of [
  ['#include "src/compiler/turboshaft/loop-peeling-reducer.h"', "include loop-peeling-reducer"],
  ['#include "src/compiler/turboshaft/copying-phase.h"', "include copying-phase"],
  ['#include "src/compiler/turboshaft/phase.h"', "include phase.h (h)"],
]) {
  const src = label.includes("(h)") ? h : cc;
  if (!src.includes(frag)) throw new Error(`哨兵缺失: ${label}`);
}

// ---------------------------------------------------------------------------
// 3. 生成 Rust
// ---------------------------------------------------------------------------
let rust = banner(
  "src/compiler/turboshaft/loop-peeling-phase.h.js",
  "src/compiler/turboshaft/loop-peeling-phase.{h,cc}",
  "循环剥离 phase: reducer 链编排骨架",
);
rust += `
/// v8: src/compiler/turboshaft/loop-peeling-phase.h LoopPeelingPhase
pub struct ${phaseStruct};

impl ${phaseStruct} {
  /// 原版 kPhaseName = "V8.TF" "Turboshaft" "LoopPeeling" (拼接展开)
  pub const PHASE_NAME: &'static str = "${phaseName}";

  /// CopyingPhase reducer 链 (保序, 原版模板参数序)
  pub const REDUCERS: &'static [&'static str] = &[
`;
for (const r of ordered) {
  rust += `    "${r}",\n`;
}
rust += `  ];

  /// 原版 void Run(PipelineData* data, Zone* temp_zone)
  /// v8: src/compiler/turboshaft/loop-peeling-phase.cc LoopPeelingPhase::Run
  pub fn run(data: &mut crate::phase::PipelineData, temp_zone: &mut crate::phase::Zone) {
    // 原版: CopyingPhase<${ordered.join(", ")}>::Run(data, temp_zone);
    // wasm-gc 依赖 MachineOptimizationReducer 先于其它 phase 执行 (原版注释保留)
    unimplemented!("Developer 修订: 编排 CopyingPhase 链")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(${phaseStruct}::PHASE_NAME == "V8.TFTurboshaftLoopPeeling");
  assert!(${phaseStruct}::REDUCERS.len() == ${ordered.length});
  assert!(${phaseStruct}::REDUCERS[0] == "LoopPeelingReducer");
};
`;

const p = writeSrc("v8-turboshaft", "loop_peeling_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`phase=1 decl=${declName} reducer链=${ordered.length} (${ordered.join(" -> ")})`);
