#!/usr/bin/env bun
// tool/src/compiler/turboshaft/type-assertions-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/type-assertions-phase.{h,cc}
// → v8-turboshaft/src/type_assertions_phase.rs.skel
//
// 提取面: phase 声明; CopyingPhase reducer 栈; typing args 配置。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/type-assertions-phase.h");
const cc = readV8("src/compiler/turboshaft/type-assertions-phase.cc");

// 1. 哨兵
for (const [re, name] of [
  [/struct TypeAssertionsPhase \{/, "phase struct"],
  [/DECL_TURBOSHAFT_PHASE_CONSTANTS\(TypeAssertions\)/, "phase 常量宏"],
  [/void Run\(PipelineData\* data, Zone\* temp_zone\);/, "Run 声明"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc: reducer 栈 + typing 配置
const stackM = cc.match(/CopyingPhase<([\s\S]*?)>::Run/);
if (!stackM) throw new Error("cc 未找到 CopyingPhase");
const stack = stackM[1].replace(/\s+/g, "").split(",").filter((s) => s && !s.startsWith("//"));
if (stack.length !== 3) throw new Error(`reducer 栈 ${stack.length} != 3`);
const typing = [...cc.matchAll(/TypeInferenceReducerArgs::(\w+)::\s*\n?\s*k(\w+)/g)].map((m) => `${m[1]}=k${m[2]}`);
if (typing.length !== 2) throw new Error(`typing 配置 ${typing.length} != 2`);

let rust = banner(
  "src/compiler/turboshaft/type-assertions-phase.h.js",
  "src/compiler/turboshaft/type-assertions-phase.{h,cc}",
  "类型断言: 插入 CheckTurboshaftTypeOf 校验推断",
);

rust += `
/// 原版 TypeAssertionsPhase — DEBUG 用: 为每个操作插入类型断言。
/// v8: src/compiler/turboshaft/type-assertions-phase.{h,cc}
pub struct TypeAssertionsPhase;

impl TypeAssertionsPhase {
  /// v8: cc Run —
  /// - typing args: ${typing.join(", ")}
  /// - UnparkedScopeIfNeeded(broker)
  /// - CopyingPhase<${stack.join(", ")}>::Run
  /// (AssertTypesReducer 借 TypeInference 推断结果插 CheckTurboshaftTypeOf)
  pub fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "type_assertions_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`reducer栈=${stack.length} typing=${typing.length}`);
