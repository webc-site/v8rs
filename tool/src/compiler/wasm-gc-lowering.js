#!/usr/bin/env bun
// tool/src/compiler/wasm-gc-lowering.js
// v8/src/compiler/wasm-gc-lowering.h + wasm-gc-lowering.cc
// → v8-compiler-passes/src/wasm_gc_lowering.rs.skel
// wasm-gc 降低: 解析 19 个 Reduce* 面与 NullCheckStrategy 策略选择。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/wasm-gc-lowering.h");
const cc = readV8("src/compiler/wasm-gc-lowering.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`wasm-gc-lowering: ${msg}`);
};

for (const [s, name] of [
  ["class WasmGCLowering final : public AdvancedReducer {", "类定义"],
  ['return "WasmGCLowering";', "reducer_name"],
  ["Reduction WasmGCLowering::Reduce(Node* node) {", "cc Reduce"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— Reduce* 面 (真解析) ——
const reducers = [...h.matchAll(/^\s{2}Reduction Reduce(\w+)\(Node\* node\);/gm)].map((m) => m[1]);
need(reducers.length >= 18, `Reduce* ${reducers.length}`);
for (const r of ["WasmTypeCheck", "WasmTypeCheckAbstract", "WasmTypeCast", "WasmTypeCastAbstract", "AssertNotNull", "Null", "IsNull", "IsNotNull", "RttCanon", "WasmStructGet", "WasmStructSet", "WasmArrayGet", "WasmArraySet", "WasmArrayLength", "WasmArrayInitializeLength", "StringAsWtf16", "StringPrepareForGetCodeunit"]) {
  need(reducers.includes(r), `Reduce${r} 缺席`);
}

// —— NullCheckStrategy 选择 (真解析 cc ctor) ——
const strategy = cc.match(/null_check_strategy_\(([^;]+)&&/);
need(strategy, "策略初始化未找到");

// —— cc 定义对数 (前缀匹配) ——
const ccDefs = reducers.filter((r) => cc.includes(`WasmGCLowering::Reduce${r}(Node* node)`));
need(ccDefs.length >= 17, `cc 定义 ${ccDefs.length}/${reducers.length}`);

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const body = `${banner("src/compiler/wasm-gc-lowering.js", "src/compiler/wasm-gc-lowering.h + wasm-gc-lowering.cc", "wasm-gc 算子 → 机器算子")}
//! wasm-gc-lowering — 类型检查/转换/空值/结构/数组算子的机器级展开
//! (${reducers.length} 个 Reduce* 面)。
//!
//! 空检查策略: ${strategy ? strategy[1].replace(/\s+/g, " ").trim() : "见 cc"}。

/// 原版 NullCheckStrategy (wasm-compiler-definitions.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NullCheckStrategy {
  /// kExplicit — 显式分支
  Explicit,
  /// kTrapHandler — 陷井处理
  TrapHandler,
}

/// 原版 WasmGCLowering — AdvancedReducer
pub struct WasmGCLowering<'z> {
  /// null_check_strategy_
  pub null_check_strategy: NullCheckStrategy,
  /// gasm_
  pub gasm: WasmGraphAssembler<'z>,
  /// module_
  pub module: *const wasm::WasmModule,
  /// dead_
  pub dead: *mut Node,
  /// mcgraph_
  pub mcgraph: *const MachineGraph<'z>,
  /// source_position_table_
  pub source_position_table: *mut SourcePositionTable,
}

impl WasmGCLowering<'_> {
  /// 原版 reducer_name
  pub const REDUCER_NAME: &'static str = "WasmGCLowering";

  /// 原版 Reduce — opcode 分派
  /// v8: src/compiler/wasm-gc-lowering.cc WasmGCLowering::Reduce
  pub fn reduce(&mut self, node: &Node) -> Reduction {
    todo!("v8: src/compiler/wasm-gc-lowering.cc WasmGCLowering::Reduce")
  }

${reducers.map((r) => `  /// 原版 Reduce${r}\n  /// v8: src/compiler/wasm-gc-lowering.cc WasmGCLowering::Reduce${r}\n  fn ${snake("reduce_" + r)}(&mut self, node: &Node) -> Reduction {\n    todo!("v8: src/compiler/wasm-gc-lowering.cc WasmGCLowering::Reduce${r}")\n  }`).join("\n\n")}

  /// 原版 Null — 类型特定的 null 常量 (wasm null/JS null)
  /// v8: src/compiler/wasm-gc-lowering.cc WasmGCLowering::Null
  fn null(&mut self, type_: wasm::ValueType) -> *mut Node {
    todo!("v8: src/compiler/wasm-gc-lowering.cc WasmGCLowering::Null")
  }

  /// 原版 BuildLoadExternalPointerFromObject
  /// v8: src/compiler/wasm-gc-lowering.cc WasmGCLowering::BuildLoadExternalPointerFromObject
  fn build_load_external_pointer_from_object(&mut self, object: &Node, offset: i32, tag: ExternalPointerTag) -> *mut Node {
    todo!("v8: src/compiler/wasm-gc-lowering.cc WasmGCLowering::BuildLoadExternalPointerFromObject")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "wasm_gc_lowering.rs.skel", body);
console.log(`written ${p}\nreduce_face=${reducers.length} cc_defs=${ccDefs.length} strategy=${strategy ? strategy[1].trim().slice(0, 60) : "?"}`);
