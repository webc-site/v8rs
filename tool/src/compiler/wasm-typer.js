#!/usr/bin/env bun
// tool/src/compiler/wasm-typer.js
// v8/src/compiler/wasm-typer.h + wasm-typer.cc
// → v8-compiler-passes/src/wasm_typer.rs.skel
// wasm-gc 类型重算: 解析 reducer 面与 cc 的 retyped opcode 集。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/wasm-typer.h");
const cc = readV8("src/compiler/wasm-typer.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`wasm-typer: ${msg}`);
};

for (const [s, name] of [
  ["class WasmTyper final : public AdvancedReducer {", "类定义"],
  ['return "WasmTyper";', "reducer_name"],
  ["WasmTyper(Editor* editor, MachineGraph* mcgraph, uint32_t function_index);", "ctor 签名"],
  ["Reduction WasmTyper::Reduce(Node* node) {", "cc Reduce"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// cc: retyped opcode 面 (真解析)
const reduceBody = cc.slice(cc.indexOf("Reduction WasmTyper::Reduce"));
const cases = [...reduceBody.slice(0, reduceBody.indexOf("\n}", 20)).matchAll(/case IrOpcode::k(\w+):/g)].map((m) => m[1]);
need(cases.length >= 5, `retyped case ${cases.length}`);
for (const c of ["WasmStructGet", "WasmArrayGet", "Phi", "TypeGuard", "WasmTypeCast"]) {
  need(cases.includes(c), `case ${c} 缺席`);
}
// 不动点迭代 (loop)
const hasLoop = cc.includes("loop") || cc.includes("Loop");

const body = `${banner("src/compiler/wasm-typer.js", "src/compiler/wasm-typer.h + wasm-typer.cc", "wasm-gc 类型沿图收窄到不动点")}
//! wasm-typer — 结构字段/数组元素/phi/类型转换/TypeGuard 重定型
//! (环上迭代至不动点)。
//!
//! retyped 面 (${cases.length}): ${cases.join(", ")}。

/// 原版 WasmTyper — AdvancedReducer
pub struct WasmTyper<'z> {
  /// function_index_
  pub function_index: u32,
  /// graph_zone_
  pub graph_zone: &'z Zone<'z>,
}

impl WasmTyper<'_> {
  /// 原版 reducer_name
  pub const REDUCER_NAME: &'static str = "WasmTyper";

  /// 原版 Reduce — 逐节点重定型, 类型收窄即 Changed
  /// v8: src/compiler/wasm-typer.cc WasmTyper::Reduce (${cases.length} case)
  pub fn reduce(&mut self, node: &Node) -> Reduction {
    todo!("v8: src/compiler/wasm-typer.cc WasmTyper::Reduce (${cases.join("/")})")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "wasm_typer.rs.skel", body);
console.log(`written ${p}\nretyped_cases=${cases.length} (${cases.join(",")}) loop_fp=${hasLoop}`);
