#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-revec-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-revec-phase.{h,cc}
// → v8-turboshaft/src/wasm_revec_phase.rs.skel
//
// 提取面: WasmRevecVerifier; phase 声明; analyzer 门控 + 测试观察者流程。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-revec-phase.h");
const cc = readV8("src/compiler/turboshaft/wasm-revec-phase.cc");

// 1. 哨兵
for (const [re, name] of [
  [/class WasmRevecVerifier \{/, "verifier 类"],
  [/std::function<void\(const Graph&\)> handler_ = nullptr;/, "handler 字段"],
  [/void Verify\(const Graph& graph\)/, "Verify"],
  [/struct WasmRevecPhase \{/, "phase struct"],
  [/DECL_TURBOSHAFT_PHASE_CONSTANTS\(WasmRevec\)/, "phase 常量宏"],
  [/#error This header should only be included if WebAssembly is enabled\./, "wasm 门"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc 流程
for (const [re, name] of [
  [/WasmRevecAnalyzer analyzer\(data, temp_zone, data->graph\(\)\);/, "analyzer 构造"],
  [/analyzer\.ShouldReduce\(\)/, "门控"],
  [/data->set_wasm_revec_percent\(analyzer\.revectorized_percent\(\)\);/, "百分比回写"],
  [/CopyingPhase<WasmRevecReducer>::Run/, "reducer 复制"],
  [/Isolate::TryGetCurrent\(\)/, "isolate 探测"],
  [/wasm_revec_verifier_for_test\(\)/, "测试观察者"],
  [/revec_observer_for_test->Verify\(data->graph\(\)\);/, "观察者验证"],
  [/data->clear_wasm_revec_analyzer\(\);/, "卸 analyzer"],
]) {
  if (!re.test(cc)) throw new Error(`cc 哨兵缺失: ${name}`);
}

let rust = banner(
  "src/compiler/turboshaft/wasm-revec-phase.h.js",
  "src/compiler/turboshaft/wasm-revec-phase.{h,cc}",
  "wasm 重向量化: 128->256 位条件转换期",
);

rust += `
/// 原版 WasmRevecVerifier — 测试注入的图校验回调
/// v8: wasm-revec-phase.h WasmRevecVerifier
pub struct WasmRevecVerifier {
  /// handler_ 为空则 Verify 无操作
  pub handler: Option<Box<dyn Fn(&Graph)>>,
}

impl WasmRevecVerifier {
  /// v8: Verify(graph) — handler_ 存在则调用
  pub fn verify(&self, graph: &Graph) { /* ... */ }
}

/// 原版 WasmRevecPhase — 流程:
/// 1. WasmRevecAnalyzer(data, temp_zone, graph)
/// 2. ShouldReduce() 为真才:
///    - set_wasm_revec_percent(revectorized_percent())
///    - set_wasm_revec_analyzer
///    - UnparkedScope + CopyingPhase<WasmRevecReducer>::Run
///    - Isolate::TryGetCurrent() 取 wasm_revec_verifier_for_test
///      (测试注入) -> Verify(graph)
///    - clear_wasm_revec_analyzer
/// v8: src/compiler/turboshaft/wasm-revec-phase.{h,cc}
pub struct WasmRevecPhase;
`;

const p = writeSrc("v8-turboshaft", "wasm_revec_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`流程锚=8 verifier=1`);
