#!/usr/bin/env bun
// tool/src/compiler/turboshaft/typed-optimizations-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/typed-optimizations-phase.{h,cc}
// → v8-turboshaft/src/typed_optimizations_phase.rs.skel
//
// 提取面: phase 声明; CopyingPhase reducer 栈; typing 配置。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/typed-optimizations-phase.h");
const cc = readV8("src/compiler/turboshaft/typed-optimizations-phase.cc");

// 1. 哨兵
for (const [re, name] of [
  [/struct TypedOptimizationsPhase \{/, "phase struct"],
  [/DECL_TURBOSHAFT_PHASE_CONSTANTS\(TypedOptimizations\)/, "phase 常量宏"],
  [/void Run\(PipelineData\* data, Zone\* temp_zone\);/, "Run 声明"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc reducer 栈 + typing
const stackM = cc.match(/CopyingPhase<([\s\S]*?)>::Run/);
if (!stackM) throw new Error("cc 未找到 CopyingPhase");
const stack = stackM[1].replace(/\s+/g, "").split(",").filter(Boolean);
if (stack.length !== 2) throw new Error(`reducer 栈 ${stack.length} != 2`);
const typing = [...cc.matchAll(/TypeInferenceReducerArgs::(\w+)::k(\w+)/g)].map((m) => `${m[1]}=k${m[2]}`);
if (typing.length !== 2) throw new Error(`typing ${typing.length} != 2`);
const debugOnly = /#ifdef DEBUG[\s\S]*turboshaft_trace_typing/.test(cc);

let rust = banner(
  "src/compiler/turboshaft/typed-optimizations-phase.h.js",
  "src/compiler/turboshaft/typed-optimizations-phase.{h,cc}",
  "类型优化: 借类型做常量折叠",
);

rust += `
/// 原版 TypedOptimizationsPhase — 输入图精确类型 + TypedOptimizationsReducer
/// v8: src/compiler/turboshaft/typed-optimizations-phase.{h,cc}
pub struct TypedOptimizationsPhase;

impl TypedOptimizationsPhase {
  /// v8: cc Run —
  /// - typing args: ${typing.join(", ")}
  /// - ${debugOnly ? "UnparkedScope 仅 DEBUG (trace_typing)" : "UnparkedScope"}
  /// - CopyingPhase<${stack.join(", ")}>::Run
  pub fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "typed_optimizations_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`reducer栈=${stack.length} typing=${typing.length}`);
