#!/usr/bin/env bun
// tool/src/compiler/turboshaft/stack-check-lowering-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/stack-check-lowering-reducer.h
// → v8-turboshaft/src/stack_check_lowering_reducer.rs.skel
//
// 提取面: REDUCE(JSStackCheck)/(WasmStackCheck); 栈极限比较与 builtin 调用锚。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/stack-check-lowering-reducer.h");

// 1. REDUCE 面
const reduces = [...h.matchAll(/\bREDUCE\((\w+)\)\(/g)].map((m) => m[1]);
if (reduces.length !== 2 || !reduces.includes("JSStackCheck") || !reduces.includes("WasmStackCheck")) {
  throw new Error(`REDUCE 面 ${reduces.join(",")} != JSStackCheck/WasmStackCheck`);
}

// 2. 哨兵
for (const [re, name] of [
  [/class StackCheckLoweringReducer : public Next/, "reducer 类"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(StackCheckLowering\)/, "boilerplate"],
  [/StackCheckKind/, "StackCheckKind"],
  [/address_of_jslimit/, "jslimit 栈极限"],
  [/runtime::StackGuard/, "StackGuard runtime 调用"],
  [/Builtin::kWasmStackGuard/, "WasmStackGuard builtin"],
  [/V<Context> context,/, "JS 上下文参数"],
  [/WasmStackCheckOp::Kind kind/, "wasm kind"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 3. wasm kind 分支
const kinds = [...h.matchAll(/WasmStackCheckOp::Kind::k(\w+)/g)].map((m) => m[1]);
const uniqueKinds = [...new Set(kinds)];

let rust = banner(
  "src/compiler/turboshaft/stack-check-lowering-reducer.h.js",
  "src/compiler/turboshaft/stack-check-lowering-reducer.h",
  "栈检查降低: 栈溢出比较 + StackGuard 调用",
);

rust += `
/// 原版 StackCheckLoweringReducer — 把 JS/Wasm 栈检查 op 降为
/// 栈指针与 jslimit/wasm limit 的比较 + 条件调 StackGuard builtin。
/// v8: src/compiler/turboshaft/stack-check-lowering-reducer.h
pub struct StackCheckLoweringReducer;

impl StackCheckLoweringReducer {
  /// v8: REDUCE(JSStackCheck)(context, frame_state)
  /// - 读 JS 栈限 ExternalReference::address_of_jslimit(isolate)
  /// - 触发时 CallRuntime<runtime::StackGuard>(context, {})
  ///   (带 gap 变体: runtime::StackGuardWithGap)
  pub fn reduce_js_stack_check(&mut self, context: V<Context>, fs: V<FrameState>) -> V<None> { todo!() }

  /// v8: REDUCE(WasmStackCheck)(kind)
  /// kind 面: ${uniqueKinds.join(" / ")}
  /// builtin 面: kWasmStackGuard / kWasmStackGuardLoop (growable stacks flag)
  pub fn reduce_wasm_stack_check(&mut self, kind: WasmStackCheckKind) -> V<None> { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "stack_check_lowering_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`REDUCE=${reduces.length} wasmKind=${uniqueKinds.length}`);
