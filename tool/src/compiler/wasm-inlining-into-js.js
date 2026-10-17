#!/usr/bin/env bun
// tool/src/compiler/wasm-inlining-into-js.js
// v8/src/compiler/wasm-inlining-into-js.h + wasm-inlining-into-js.cc
// → v8-compiler-passes/src/wasm_inlining_into_js.rs.skel
// wasm→JS 内联: 解析 TryInlining 唯一入口与 cc 的调用约定面。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/wasm-inlining-into-js.h");
const cc = readV8("src/compiler/wasm-inlining-into-js.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`wasm-inlining-into-js: ${msg}`);
};

for (const [s, name] of [
  ["class WasmIntoJSInliner {", "类定义"],
  ["static bool TryInlining(Zone* zone, const wasm::WasmModule* module,", "TryInlining 签名"],
  ["bool WasmIntoJSInliner::TryInlining(", "cc TryInlining"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// cc: 参数面计数 (真解析 — wasm→js 包装的内联成本)
const body1 = cc; // 参数门控在辅助类 ctor 内, 不按 TryInlining 切片
const paramCheck = body1.match(/function->sig->parameter_count\(\) (>|\?\?|=) (\d+)/);
need(body1.includes("parameter_count"), "cc 参数计数判定缺失");
const usesDecoder = cc.includes("wasm::FunctionBody") || cc.includes("DecodeWasmModule");

const body = `${banner("src/compiler/wasm-inlining-into-js.js", "src/compiler/wasm-inlining-into-js.h + wasm-inlining-into-js.cc", "小 wasm 函数体直接内联进 JS 图")}
//! wasm-inlining-into-js — wrapper 内联: JS→wasm 调用点直接建 wasm 体
//! (参数计数受限; 走 wasm 解码器取函数体)。

/// 原版 WasmIntoJSInliner — AllStatic
pub struct WasmIntoJSInliner;

impl WasmIntoJSInliner {
  /// 原版 TryInlining — 唯一入口 (参数计数门控: ${paramCheck ? paramCheck[0].replace(/\s+/g, " ") : "见 cc"})
  /// v8: src/compiler/wasm-inlining-into-js.cc WasmIntoJSInliner::TryInlining
  pub fn try_inlining(zone: &Zone<'_>, module: &wasm::WasmModule, mcgraph: &mut MachineGraph<'_>, body: &wasm::FunctionBody, bytes: &[u8], source_position_table: &mut SourcePositionTable, inlining_id: i32) -> bool {
    todo!("v8: src/compiler/wasm-inlining-into-js.cc WasmIntoJSInliner::TryInlining (解码器面: ${usesDecoder ? "wasm FunctionBody 解码" : "见 cc"})")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "wasm_inlining_into_js.rs.skel", body);
console.log(`written ${p}\nparam_gate=${paramCheck ? paramCheck[0].replace(/\s+/g, " ") : "其他"} decoder=${usesDecoder}`);
