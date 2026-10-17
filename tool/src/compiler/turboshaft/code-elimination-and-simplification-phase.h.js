#!/usr/bin/env bun
// tool/src/compiler/turboshaft/code-elimination-and-simplification-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/code-elimination-and-simplification-phase.{h,cc}
// → v8-turboshaft/src/code_elimination_and_simplification_phase.rs.skel
//
// 提取面: CodeEliminationAndSimplificationPhase (收尾清理 phase) 的
// CopyingPhase reducer 链 — 无条件 8 项 + 条件编译 2 项 (#ifdef DEBUG
// StoreLoadVerificationReducer / #if V8_ENABLE_WEBASSEMBLY WasmJSLoweringReducer)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/code-elimination-and-simplification-phase.h");
const cc = readV8("src/compiler/turboshaft/code-elimination-and-simplification-phase.cc");

// 1. phase 结构
const structM = h.match(/struct (\w+Phase)\s*\{([\s\S]*?)\n\};/);
if (!structM) throw new Error("未找到 phase struct");
if (structM[1] !== "CodeEliminationAndSimplificationPhase") {
  throw new Error(`struct 名 ${structM[1]} != CodeEliminationAndSimplificationPhase`);
}
const declM = structM[2].match(/DECL_TURBOSHAFT_PHASE_CONSTANTS(?:_WITH_LEGACY_NAME)?\((\w+)\)/);
if (!declM) throw new Error("未找到 DECL_TURBOSHAFT_PHASE_CONSTANTS");
if (declM[1] !== "CodeEliminationAndSimplification") throw new Error(`DECL 名 ${declM[1]} != CodeEliminationAndSimplification`);
const phaseName = `V8.TFTurboshaft${declM[1]}`;
if (!/void Run\(PipelineData\* data, Zone\* temp_zone\);/.test(structM[2])) {
  throw new Error("Run 签名缺失");
}

// 2. reducer 链: #if 行扫描分离无条件/条件成员 (嵌套条件 & 连接)
const chainM = cc.match(/CopyingPhase<([\s\S]*?)>::Run/);
if (!chainM) throw new Error("未找到 CopyingPhase<...>::Run");
const orderedAll = [...new Set([...chainM[1].replace(/\/\/[^\n]*/g, "").matchAll(/\b([A-Z]\w*Reducer)\b/g)].map((m) => m[1]))];
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
const unconditional = orderedAll.filter((r) => ![...condMembers].some((c) => c.split("@")[0] === r));
const expected = [
  "MaglevAssertTypesReducer", "DeadCodeEliminationReducer", "StackCheckLoweringReducer",
  "StoreStoreEliminationReducer", "LoadStoreSimplificationReducer",
  "DuplicationOptimizationReducer", "InstructionSelectionNormalizationReducer",
  "ValueNumberingReducer",
];
if (JSON.stringify(unconditional) !== JSON.stringify(expected)) {
  throw new Error(`无条件链不匹配: ${JSON.stringify(unconditional)}`);
}
const condArr = [...condMembers].sort();
if (condArr.length !== 2 || !condArr.some((c) => c === "StoreLoadVerificationReducer@DEBUG") ||
    !condArr.some((c) => c === "WasmJSLoweringReducer@V8_ENABLE_WEBASSEMBLY")) {
  throw new Error(`条件成员不匹配: ${JSON.stringify(condArr)}`);
}

// 3. 哨兵: cc 条件编译 include 与关键注释锚
for (const [frag, label] of [
  ['#if V8_ENABLE_WEBASSEMBLY\n#include "src/compiler/turboshaft/wasm-js-lowering-reducer.h"', "wasm include 条件区"],
  [/\/\/ StoreStoreElimination needs the broken unparked/, "StoreStoreElimination 注释锚"],
  [/UnparkedScopeIfNeeded scope\(data->broker\(\)\);/, "UnparkedScopeIfNeeded"],
  ['#include "src/compiler/turboshaft/phase.h"', "include phase.h"],
]) {
  const ok = typeof frag === "string" ? cc.includes(frag) : frag.test(cc) || frag.test(h);
  if (!ok) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/code-elimination-and-simplification-phase.h.js",
  "src/compiler/turboshaft/code-elimination-and-simplification-phase.{h,cc}",
  "收尾清理 phase: 死代码/存储消除/规范化 reducer 链编排骨架",
);
rust += `
/// v8: src/compiler/turboshaft/code-elimination-and-simplification-phase.h CodeEliminationAndSimplificationPhase
pub struct CodeEliminationAndSimplificationPhase;

impl CodeEliminationAndSimplificationPhase {
  /// 原版 kPhaseName = "V8.TF" "Turboshaft" "CodeEliminationAndSimplification"
  pub const PHASE_NAME: &'static str = "${phaseName}";

  /// CopyingPhase 无条件 reducer 链 (保序; 原版 8 项)
  pub const REDUCERS: &'static [&'static str] = &[
${expected.map((r) => `    "${r}",`).join("\n")}
  ];

  /// 条件编译成员 (原版 #ifdef DEBUG): StoreLoadVerificationReducer
  /// 条件编译成员 (原版 #if V8_ENABLE_WEBASSEMBLY): WasmJSLoweringReducer

  /// 原版 void Run(PipelineData* data, Zone* temp_zone)
  /// v8: src/compiler/turboshaft/code-elimination-and-simplification-phase.cc CodeEliminationAndSimplificationPhase::Run
  ///
  /// 原版序约束注释 (保留语义):
  /// - DuplicationOptimizationReducer 必须在 LoadStoreSimplificationReducer 之后
  ///   (优化其产出的 Load/Store)
  /// - StoreStoreElimination 需要非驻留 scope 以合并只读对象的连续存储
  pub fn run(data: &mut crate::phase::PipelineData, temp_zone: &mut crate::phase::Zone) {
    unimplemented!("Developer 修订: 编排 CopyingPhase 链 (含 DEBUG/WASM 条件成员)")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(CodeEliminationAndSimplificationPhase::PHASE_NAME == "V8.TFTurboshaftCodeEliminationAndSimplification");
  assert!(CodeEliminationAndSimplificationPhase::REDUCERS.len() == 8);
  assert!(CodeEliminationAndSimplificationPhase::REDUCERS[0] == "MaglevAssertTypesReducer");
  assert!(CodeEliminationAndSimplificationPhase::REDUCERS[7] == "ValueNumberingReducer");
};
`;

const p = writeSrc("v8-turboshaft", "code_elimination_and_simplification_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`phase=1 decl=${declM[1]} 无条件链=${unconditional.length} 条件成员=${condArr.length} (${condArr.join("; ")})`);
