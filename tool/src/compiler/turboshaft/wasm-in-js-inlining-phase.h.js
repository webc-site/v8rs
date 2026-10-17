#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-in-js-inlining-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-in-js-inlining-phase.{h,cc}
// → v8-turboshaft/src/wasm_in_js_inlining_phase.rs.skel
//
// 提取面: phase 声明 (JS 管线侧); 3-reducer 栈; GVN 动机注释。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-in-js-inlining-phase.h");
const cc = readV8("src/compiler/turboshaft/wasm-in-js-inlining-phase.cc");

// 1. phase 哨兵
for (const [re, name] of [
  [/struct WasmInJSInliningPhase \{/, "phase struct"],
  [/DECL_TURBOSHAFT_PHASE_CONSTANTS\(WasmInJSInlining\)/, "phase 常量宏"],
  [/void Run\(PipelineData\* data, Zone\* temp_zone\);/, "Run 声明"],
  [/This reducer is part of the JavaScript pipeline/, "JS 管线注释"],
  [/#error This header should only be included if WebAssembly is enabled\./, "wasm 门"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc reducer 栈
const stackM = cc.match(/CopyingPhase<([\s\S]*?)>::Run/);
if (!stackM) throw new Error("cc 未找到 CopyingPhase");
const raw = stackM[1].replace(/\s+/g, "").split(",").filter(Boolean);
if (raw.length !== 3) throw new Error(`reducer 栈 ${raw.length} != 3`);
for (const must of ["WasmInJSInliningReducer", "SimplifiedOptimizationReducer", "ValueNumberingReducer"]) {
  if (!raw.includes(must)) throw new Error(`栈缺 ${must}`);
}

// 3. GVN 动机注释
if (!/GVN \(ValueNumberingReducer\) is run here to deduplicate Wasm operations/.test(cc)) {
  throw new Error("缺 GVN 动机注释");
}

let rust = banner(
  "src/compiler/turboshaft/wasm-in-js-inlining-phase.h.js",
  "src/compiler/turboshaft/wasm-in-js-inlining-phase.{h,cc}",
  "wasm-in-JS 内联期: 小/热 wasm 函数进 JS 管线",
);

rust += `
/// 原版 WasmInJSInliningPhase — 属 JS 管线 (非 wasm 管线),
/// 把足够小/热的 wasm 函数体内联进调用方 JS 函数。
/// v8: src/compiler/turboshaft/wasm-in-js-inlining-phase.{h,cc}
pub struct WasmInJSInliningPhase;

impl WasmInJSInliningPhase {
  /// v8: cc Run — UnparkedScopeIfNeeded(broker, DEBUG_BOOL) 后
  /// CopyingPhase<${raw.join(", ")}>::Run
  /// GVN 目的 (cc 注释): 跨多个被内联函数去重 wasm 操作,
  /// 亦利于后续 WasmGC 类型优化 / wasm 载入消除。
  pub fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone) { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "wasm_in_js_inlining_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`reducer栈=${raw.length}`);
