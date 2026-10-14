#!/usr/bin/env bun
// tool/src/compiler/turboshaft/loop-optimization-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/loop-optimization-phase.{h,cc}
// → v8-turboshaft/src/loop_optimization_phase.rs.skel
//
// 提取面: LoopOptimizationPhase (DECL_TURBOSHAFT_PHASE_CONSTANTS 展开的 kPhaseName)
// + Run 的 CopyingPhase reducer 链 (保序, #if 条件成员单列)。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/loop-optimization-phase.h");
const cc = readV8("src/compiler/turboshaft/loop-optimization-phase.cc");

// 1. phase 结构
const structM = h.match(/struct (\w+Phase)\s*\{([\s\S]*?)\n\};/);
if (!structM) throw new Error("未找到 phase struct");
const phaseStruct = structM[1];
if (phaseStruct !== "LoopOptimizationPhase") throw new Error(`struct 名 ${phaseStruct} != LoopOptimizationPhase`);

const declM = structM[2].match(/DECL_TURBOSHAFT_PHASE_CONSTANTS(?:_WITH_LEGACY_NAME)?\((\w+)\)/);
if (!declM) throw new Error("未找到 DECL_TURBOSHAFT_PHASE_CONSTANTS");
const declName = declM[1];
if (declName !== "LoopOptimization") throw new Error(`DECL 名 ${declName} != LoopOptimization`);
const phaseName = `V8.TFTurboshaft${declName}`;

if (!/void Run\(PipelineData\* data, Zone\* temp_zone\);/.test(structM[2])) {
  throw new Error("Run 签名缺失 (void Run(PipelineData*, Zone*))");
}

// 2. reducer 链: CopyingPhase<...>::Run 保序提取; #if 分支成员单列
const chainM = cc.match(/CopyingPhase<([\s\S]*?)>::Run/);
if (!chainM) throw new Error("未找到 CopyingPhase<...>::Run");
const body = chainM[1].replace(/\/\/[^\n]*/g, "");
const ordered = [...new Set([...body.matchAll(/\b([A-Z]\w*Reducer)\b/g)].map((m) => m[1]))];
// 条件编译成员: #if/#endif 行扫描, 记录所在条件
const condMembers = new Set();
let depthCond = [];
for (const line of chainM[1].split("\n")) {
  const ifM = line.match(/^\s*#(?:ifdef|if)\s+(\w+)/);
  if (ifM) depthCond.push(ifM[1]);
  else if (/^\s*#endif/.test(line)) depthCond.pop();
  else if (depthCond.length > 0) {
    for (const m of line.matchAll(/\b([A-Z]\w*Reducer)\b/g)) condMembers.add(m[1] + "@" + depthCond.join("&"));
  }
}
const unconditional = ordered.filter((r) => ![...condMembers].some((c) => c.split("@")[0] === r));
const expected = ["LoopOptimizationReducer","ValueNumberingReducer"];
if (JSON.stringify(unconditional) !== JSON.stringify(expected)) {
  throw new Error(`reducer 链不匹配: ${JSON.stringify(unconditional)} 预期 ${JSON.stringify(expected)}`);
}

// 3. 哨兵
for (const [frag, label] of [
  ['#include "src/compiler/turboshaft/loop-optimization-reducer.h"', "include loop-optimization-reducer.h"],
  ['#include "src/compiler/turboshaft/phase.h"', "include phase.h"],
]) {
  if (!(h.includes(frag) || cc.includes(frag))) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/loop-optimization-phase.h.js",
  "src/compiler/turboshaft/loop-optimization-phase.{h,cc}",
  "LoopOptimizationPhase reducer 链编排骨架",
);
rust += `
/// v8: src/compiler/turboshaft/loop-optimization-phase.h LoopOptimizationPhase
pub struct LoopOptimizationPhase;

impl LoopOptimizationPhase {
  /// 原版 kPhaseName = "V8.TF" "Turboshaft" "LoopOptimization" (宏拼接展开)
  pub const PHASE_NAME: &'static str = "${phaseName}";

  /// CopyingPhase 无条件 reducer 链 (保序, 原版模板参数序)
  pub const REDUCERS: &'static [&'static str] = &[
    "LoopOptimizationReducer",\n    "ValueNumberingReducer",
  ];
`;
for (const c of [...condMembers]) {
  const [name, cond2] = c.split("@");
  rust += `  /// 条件编译成员 (原版 #if ${cond2}): ${name}\n`;
}
rust += `

  /// 原版 void Run(PipelineData* data, Zone* temp_zone)
  /// v8: src/compiler/turboshaft/loop-optimization-phase.cc LoopOptimizationPhase::Run
  pub fn run(data: &mut crate::phase::PipelineData, temp_zone: &mut crate::phase::Zone) {
    // 原版: CopyingPhase<LoopOptimizationReducer, ValueNumberingReducer>::Run(data, temp_zone);
    unimplemented!("Developer 修订: 编排 CopyingPhase 链")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(LoopOptimizationPhase::PHASE_NAME == "V8.TFTurboshaftLoopOptimization");
  assert!(LoopOptimizationPhase::REDUCERS.len() == 2);
  assert!(LoopOptimizationPhase::REDUCERS[0] == "LoopOptimizationReducer");
};
`

const p = writeSrc("v8-turboshaft", "loop_optimization_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`phase=1 decl=${declName} 无条件链=${unconditional.length} 条件成员=${condMembers.size} (${unconditional.join(" -> ")})`);
