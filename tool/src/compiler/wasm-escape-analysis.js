#!/usr/bin/env bun
// tool/src/compiler/wasm-escape-analysis.js
// v8/src/compiler/wasm-escape-analysis.h + wasm-escape-analysis.cc
// → v8-compiler-passes/src/wasm_escape_analysis.rs.skel
// wasm 逃逸分析: 解析单方法 reducer 面与 cc 的 AllocateRaw 处理。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/wasm-escape-analysis.h");
const cc = readV8("src/compiler/wasm-escape-analysis.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`wasm-escape-analysis: ${msg}`);
};

for (const [s, name] of [
  ["class WasmEscapeAnalysis final : public AdvancedReducer {", "类定义"],
  ['return "WasmEscapeAnalysis";', "reducer_name"],
  ["Reduction Reduce(Node* node) final;", "Reduce 声明"],
  ["Reduction ReduceAllocateRaw(Node* call);", "ReduceAllocateRaw 声明"],
  ["Reduction WasmEscapeAnalysis::Reduce(Node* node) {", "cc Reduce"],
  ["Reduction WasmEscapeAnalysis::ReduceAllocateRaw(Node* node) {", "cc ReduceAllocateRaw"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// cc: 判据 (真解析 — 仅 store 使用才保活)
const body1 = cc.slice(cc.indexOf("Reduction WasmEscapeAnalysis::ReduceAllocateRaw"));
const storeOnly = /IrOpcode::kStore|IsStore/.test(body1);
need(cc.includes("IrOpcode::kAllocateRaw"), "cc AllocateRaw 分派缺失");

const body = `${banner("src/compiler/wasm-escape-analysis.js", "src/compiler/wasm-escape-analysis.h + wasm-escape-analysis.cc", "消除仅作 store 目标的分配")}
//! wasm-escape-analysis — AllocateRaw 无逃逸 (非 store 使用) 即删
//! (phi/重命名暂不豁免 — 原版注释的 future work)。

/// 原版 WasmEscapeAnalysis — AdvancedReducer
pub struct WasmEscapeAnalysis<'z> {
  /// mcgraph_
  pub mcgraph: &'z MachineGraph<'z>,
}

impl WasmEscapeAnalysis<'_> {
  /// 原版 reducer_name
  pub const REDUCER_NAME: &'static str = "WasmEscapeAnalysis";

  /// 原版 Reduce — 仅 AllocateRaw 进入
  /// v8: src/compiler/wasm-escape-analysis.cc WasmEscapeAnalysis::Reduce
  pub fn reduce(&mut self, node: &Node) -> Reduction {
    todo!("v8: src/compiler/wasm-escape-analysis.cc WasmEscapeAnalysis::Reduce")
  }

  /// 原版 ReduceAllocateRaw — 使用全为 store → 替换为 dead (store_only 判定: ${storeOnly ? "Store 谓词核对在案" : "见 cc"})
  /// v8: src/compiler/wasm-escape-analysis.cc WasmEscapeAnalysis::ReduceAllocateRaw
  fn reduce_allocate_raw(&mut self, call: &Node) -> Reduction {
    todo!("v8: src/compiler/wasm-escape-analysis.cc WasmEscapeAnalysis::ReduceAllocateRaw")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "wasm_escape_analysis.rs.skel", body);
console.log(`written ${p}\nstore_only_judge=${storeOnly}`);
