#!/usr/bin/env bun
// tool/src/compiler/wasm-gc-operator-reducer.js
// v8/src/compiler/wasm-gc-operator-reducer.h + wasm-gc-operator-reducer.cc
// → v8-compiler-passes/src/wasm_gc_operator_reducer.rs.skel
// wasm-gc 优化: 解析 NodeWithType / 控制路径状态面 / Reduce* 族。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/wasm-gc-operator-reducer.h");
const cc = readV8("src/compiler/wasm-gc-operator-reducer.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`wasm-gc-operator-reducer: ${msg}`);
};

for (const [s, name] of [
  ["struct NodeWithType {", "NodeWithType"],
  ["class WasmGCOperatorReducer final", "类定义"],
  [": public AdvancedReducerWithControlPathState<NodeWithType,", "控制路径基类"],
  ['return "WasmGCOperatorReducer";', "reducer_name"],
  ["Reduction WasmGCOperatorReducer::Reduce(Node* node) {", "cc Reduce"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— Reduce* 面 (真解析) ——
const reducers = [...h.matchAll(/^\s{2}Reduction Reduce(\w+)\(Node\* node(?:, bool condition)?\);/gm)].map((m) => m[1]);
need(reducers.length >= 12, `Reduce* ${reducers.length}`);
for (const r of ["WasmStructOperation", "WasmArrayLength", "AssertNotNull", "CheckNull", "WasmTypeCheck", "WasmTypeCast", "TypeGuard", "WasmAnyConvertExtern", "Merge", "If", "Start"]) {
  need(reducers.includes(r), `Reduce${r} 缺席`);
}

// —— 辅助面 ——
const hNorm2 = h.replace(/\(\s*\n\s*/g, "(");
const helpers = [...hNorm2.matchAll(/^\s{2}(?:Node\*|void|wasm::TypeInModule|Reduction) (\w+)\(/gm)].map((m) => m[1]).filter((n) => !n.startsWith("Reduce"));
for (const hh of ["SetType", "UpdateSourcePosition", "ObjectTypeFromContext", "UpdateNodeAndAliasesTypes"]) {
  need(helpers.includes(hh), `辅助 ${hh} 缺席`);
}

// —— cc 定义对数 ——
const ccDefs = reducers.filter((r) => cc.includes(`WasmGCOperatorReducer::Reduce${r}(Node*`));
need(ccDefs.length >= 11, `cc 定义 ${ccDefs.length}/${reducers.length}`);

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const body = `${banner("src/compiler/wasm-gc-operator-reducer.js", "src/compiler/wasm-gc-operator-reducer.h + wasm-gc-operator-reducer.cc", "wasm-gc 类型检查/转换的消除与收窄")}
//! wasm-gc-operator-reducer — 双信息源: 节点标记类型 + 控制路径类型
//! (类型检查作分支条件时推断路径依赖类型)。

/// 原版 NodeWithType — (node, wasm::TypeInModule) 状态单元
#[derive(Clone, Copy)]
pub struct NodeWithType {
  /// node
  pub node: *mut Node,
  /// type
  pub type_: wasm::TypeInModule,
}

impl NodeWithType {
  /// 原版 IsSet
  /// v8: src/compiler/wasm-gc-operator-reducer.h NodeWithType::IsSet
  pub fn is_set(&self) -> bool {
    !self.node.is_null()
  }
}

/// 原版 WasmGCOperatorReducer — 控制路径状态 reducer (kMultipleInstances)
pub struct WasmGCOperatorReducer<'z> {
  /// mcgraph_
  pub mcgraph: *mut MachineGraph<'z>,
  /// gasm_
  pub gasm: WasmGraphAssembler<'z>,
  /// module_
  pub module: *const wasm::WasmModule,
  /// source_position_table_
  pub source_position_table: *mut SourcePositionTable,
}

impl WasmGCOperatorReducer<'_> {
  /// 原版 reducer_name
  pub const REDUCER_NAME: &'static str = "WasmGCOperatorReducer";

  /// 原版 Reduce — 分派 (${reducers.length} 面)
  /// v8: src/compiler/wasm-gc-operator-reducer.cc WasmGCOperatorReducer::Reduce
  pub fn reduce(&mut self, node: &Node) -> Reduction {
    todo!("v8: src/compiler/wasm-gc-operator-reducer.cc WasmGCOperatorReducer::Reduce")
  }

${reducers.map((r) => `  /// 原版 Reduce${r}\n  /// v8: src/compiler/wasm-gc-operator-reducer.cc WasmGCOperatorReducer::Reduce${r}\n  fn ${snake("reduce_" + r)}(&mut self, node: &Node /*, condition? */) -> Reduction {\n    todo!("v8: src/compiler/wasm-gc-operator-reducer.cc WasmGCOperatorReducer::Reduce${r}")\n  }`).join("\n\n")}

  /// 原版 SetType — 节点标记类型 (不可收窄则复制新节点)
  /// v8: src/compiler/wasm-gc-operator-reducer.cc WasmGCOperatorReducer::SetType
  fn set_type(&mut self, node: &Node, type_: wasm::ValueType) -> *mut Node {
    todo!("v8: src/compiler/wasm-gc-operator-reducer.cc WasmGCOperatorReducer::SetType")
  }

  /// 原版 ObjectTypeFromContext — 节点类型 ∩ 控制路径类型
  /// v8: src/compiler/wasm-gc-operator-reducer.cc WasmGCOperatorReducer::ObjectTypeFromContext
  fn object_type_from_context(&mut self, object: &Node, control: &Node, allow_non_wasm: bool) -> wasm::TypeInModule {
    todo!("v8: src/compiler/wasm-gc-operator-reducer.cc WasmGCOperatorReducer::ObjectTypeFromContext")
  }

  /// 原版 UpdateNodeAndAliasesTypes — 沿别名链更新类型
  /// v8: src/compiler/wasm-gc-operator-reducer.cc WasmGCOperatorReducer::UpdateNodeAndAliasesTypes
  fn update_node_and_aliases_types(&mut self, state_owner: &Node, parent_state: ControlPathTypes, node: &Node, type_: wasm::TypeInModule, in_new_block: bool) -> Reduction {
    todo!("v8: src/compiler/wasm-gc-operator-reducer.cc WasmGCOperatorReducer::UpdateNodeAndAliasesTypes")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "wasm_gc_operator_reducer.rs.skel", body);
console.log(`written ${p}\nreduce_face=${reducers.length} cc_defs=${ccDefs.length} helpers=${helpers.length}`);
