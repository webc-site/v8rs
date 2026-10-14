#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-dead-code-elimination-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-dead-code-elimination-phase.{h,cc}
// → v8-turboshaft/src/wasm_dead_code_elimination_phase.rs.skel
//
// 提取面: phase 声明; CopyingPhase 7-reducer 栈; GVN 注释锚。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-dead-code-elimination-phase.h");
const cc = readV8("src/compiler/turboshaft/wasm-dead-code-elimination-phase.cc");

// 1. phase 哨兵
for (const [re, name] of [
  [/struct WasmDeadCodeEliminationPhase \{/, "phase struct"],
  [/DECL_TURBOSHAFT_PHASE_CONSTANTS\(WasmDeadCodeElimination\)/, "phase 常量宏"],
  [/void Run\(PipelineData\* data, Zone\* temp_zone\);/, "Run 声明"],
  [/#error This header should only be included if WebAssembly is enabled\./, "wasm 门"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc reducer 栈
const stackM = cc.match(/CopyingPhase<([\s\S]*?)>::Run/);
if (!stackM) throw new Error("cc 未找到 CopyingPhase");
const raw = stackM[1]
  .replace(/\/\/[^\n]*/g, "")
  .replace(/\s+/g, "")
  .split(",").filter(Boolean);
if (raw.length !== 7) throw new Error(`reducer 栈 ${raw.length} != 7`);
for (const must of ["DeadCodeEliminationReducer", "StackCheckLoweringReducer", "GrowableStacksReducer", "LoadStoreSimplificationReducer", "DuplicationOptimizationReducer", "InstructionSelectionNormalizationReducer", "ValueNumberingReducer"]) {
  if (!raw.includes(must)) throw new Error(`栈缺 ${must}`);
}

let rust = banner(
  "src/compiler/turboshaft/wasm-dead-code-elimination-phase.h.js",
  "src/compiler/turboshaft/wasm-dead-code-elimination-phase.{h,cc}",
  "wasm 死代码消除: 7-reducer 收尾清理",
);

rust += `
/// 原版 WasmDeadCodeEliminationPhase — wasm 管线收尾清理期。
/// v8: src/compiler/turboshaft/wasm-dead-code-elimination-phase.{h,cc}
pub struct WasmDeadCodeEliminationPhase;

impl WasmDeadCodeEliminationPhase {
  /// v8: cc Run — UnparkedScopeIfNeeded(broker, DEBUG_BOOL) 后
  /// CopyingPhase<${raw.join(", ")}>::Run
  /// 顺序语义 (cc 注释):
  /// - ValueNumbering 在复杂 load 间共享计算
  /// - DuplicationOptimization 必须在 LoadStoreSimplification 之后
  ///   (后者直接 Next::ReduceLoad/Store, 不走 Assembler helper)
  pub fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "wasm_dead_code_elimination_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`reducer栈=${raw.length}`);
