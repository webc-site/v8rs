#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-js-lowering-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-js-lowering-reducer.h
// → v8-turboshaft/src/wasm_js_lowering_reducer.rs.skel
//
// 提取面: REDUCE(WasmTrap)/(TrapIf); 延迟块 trap 调用链锚。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-js-lowering-reducer.h");

// 1. REDUCE 面
const reduces = [...h.matchAll(/\bREDUCE\((\w+)\)\(/g)].map((m) => m[1]);
if (reduces.length !== 2 || !reduces.includes("WasmTrap") || !reduces.includes("TrapIf")) {
  throw new Error(`REDUCE 面 ${reduces.join(",")} != WasmTrap/TrapIf`);
}

// 2. 哨兵
for (const [re, name] of [
  [/class WasmJSLoweringReducer : public Next/, "reducer 类"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(WasmJSLowering\)/, "boilerplate"],
  [/void LowerWasmTrap\(OptionalV<EagerFrameState> frame_state, TrapId trap_id\)/, "LowerWasmTrap"],
  [/GetBuiltinCallDescriptor\(\s*\n?\s*trap, Asm\(\)\.graph_zone\(\), StubCallMode::kCallBuiltinPointer,/, "builtin 描述符"],
  [/CreateFrameStateForStackTrace\(/, "栈迹 FrameState"],
  [/__ Call\(call_target, new_frame_state, \{\}, ts_descriptor\);/, "调用序列"],
  [/__ Unreachable\(\);.*The trap builtin never returns/, "Unreachable 收尾"],
  [/negated \? __ Word32Equal\(condition, 0\) : condition/, "取反规则"],
  [/This reducer is part of the JavaScript pipeline/, "JS 管线注释"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

let rust = banner(
  "src/compiler/turboshaft/wasm-js-lowering-reducer.h.js",
  "src/compiler/turboshaft/wasm-js-lowering-reducer.h",
  "wasm JS 降低: trap 节点到延迟块调用",
);

rust += `
/// 原版 WasmJSLoweringReducer — JS 管线侧 (内联 wasm 后),
/// 把所有 TrapIf 换成条件跳转到含 trap builtin 调用的延迟代码。
/// v8: src/compiler/turboshaft/wasm-js-lowering-reducer.h
pub struct WasmJSLoweringReducer;

impl WasmJSLoweringReducer {
  /// v8: REDUCE(WasmTrap)(frame_state, trap_id)
  /// (frame_state 必须有效: JS 中 wasm trap 全带 FrameState)
  pub fn reduce_wasm_trap(&mut self, frame_state: OptionalV<EagerFrameState>, trap_id: TrapId) -> V<None> { todo!() }

  /// v8: REDUCE(TrapIf)(condition, frame_state, negated, trap_id)
  /// - negated 时取反: Word32Equal(condition, 0)
  /// - IF (UNLIKELY(should_trap)) { LowerWasmTrap(...) }
  pub fn reduce_trap_if(&mut self, condition: V<Word32>, frame_state: OptionalV<EagerFrameState>, negated: bool, trap_id: TrapId) -> V<None> { todo!() }

  /// v8: LowerWasmTrap —
  /// 1. GetBuiltinCallDescriptor(trap, kCallBuiltinPointer, needs_frame_state=true)
  /// 2. CreateFrameStateForStackTrace: 原状态换 trap 位 ScriptOffset 的 LazyFrameState
  /// 3. Call(NumberConstant(trap), new_frame_state, {}, ts_descriptor)
  /// 4. Unreachable (trap builtin 不返回)
  fn lower_wasm_trap(&mut self, frame_state: OptionalV<EagerFrameState>, trap_id: TrapId) { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "wasm_js_lowering_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`REDUCE=${reduces.length} 私有helper=1`);
