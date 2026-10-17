#!/usr/bin/env bun
// tool/src/compiler/simplified-operator-reducer.js
// v8/src/compiler/simplified-operator-reducer.h + simplified-operator-reducer.cc
// → v8-compiler-passes/src/simplified_operator_reducer.rs.skel
// 简化算子常量折叠: 解析 Replace* 工厂族与 cc Reduce 分派面。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/simplified-operator-reducer.h");
const cc = readV8("src/compiler/simplified-operator-reducer.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`simplified-operator-reducer: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE SimplifiedOperatorReducer final", "类定义"],
  ['return "SimplifiedOperatorReducer";', "reducer_name"],
  ["BranchSemantics branch_semantics_;", "branch_semantics_ 字段"],
  ["Reduction SimplifiedOperatorReducer::Reduce(Node* node) {", "cc Reduce"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— Replace* 工厂清单 (头文件真解析) ——
const replaces = [...h.matchAll(/^\s{2}Reduction Replace(\w+)\(([^)]*)\)/gm)].map((m) => m[1]);
for (const r of ["Boolean", "Float64", "Int32", "Uint32", "Number"]) {
  need(replaces.includes(r), `Replace${r} 缺席`);
}
// 重载去重 (Float64/Number 有两个重载)
const uniqueReplaces = [...new Set(replaces)];

// —— cc Reduce 的 case 面 ——
const reduceBody = cc.slice(cc.indexOf("Reduction SimplifiedOperatorReducer::Reduce"));
const cases = [...reduceBody.slice(0, reduceBody.indexOf("\n}", 50)).matchAll(/case IrOpcode::k(\w+):/g)].map((m) => m[1]);
need(cases.length >= 10, `case 数 ${cases.length}`);

const body = `${banner("src/compiler/simplified-operator-reducer.js", "src/compiler/simplified-operator-reducer.h + simplified-operator-reducer.cc", "简化算子的常量折叠与布尔化简")}
//! simplified-operator-reducer — 常量输入的直接折叠
//!
//! BranchSemantics 决定 Boolean 化简语义 (kJS vs kMachine);
//! Replace* 工厂把标量包回图常量。

/// 原版 SimplifiedOperatorReducer — AdvancedReducer
pub struct SimplifiedOperatorReducer<'z, 'g> {
  /// jsgraph_
  pub jsgraph: &'g JSGraph<'z>,
  /// broker_
  pub broker: *mut JSHeapBroker,
  /// branch_semantics_
  pub branch_semantics: BranchSemantics,
}

impl SimplifiedOperatorReducer<'_, '_> {
  /// 原版 reducer_name
  pub const REDUCER_NAME: &'static str = "SimplifiedOperatorReducer";

  /// 原版 Reduce — ${cases.length} 个 opcode 分派 (${cases.join(", ")})
  /// v8: src/compiler/simplified-operator-reducer.cc SimplifiedOperatorReducer::Reduce
  pub fn reduce(&mut self, node: &Node) -> Reduction {
    todo!("v8: src/compiler/simplified-operator-reducer.cc SimplifiedOperatorReducer::Reduce")
  }

  /// 原版 Change — 换 operator 保持输入
  /// v8: src/compiler/simplified-operator-reducer.cc SimplifiedOperatorReducer::Change
  fn change(&mut self, node: &Node, op: &Operator, a: &Node) -> Reduction {
    todo!("v8: src/compiler/simplified-operator-reducer.cc SimplifiedOperatorReducer::Change")
  }

${uniqueReplaces.map((r) => `  /// 原版 Replace${r} — 折叠为 ${r} 常量节点\n  /// v8: src/compiler/simplified-operator-reducer.cc SimplifiedOperatorReducer::Replace${r}\n  fn replace_${r.toLowerCase()}(&mut self, /* value */) -> Reduction {\n    todo!("v8: src/compiler/simplified-operator-reducer.cc SimplifiedOperatorReducer::Replace${r}")\n  }`).join("\n\n")}
}
`;

const p = writeSrc("v8-compiler-passes", "simplified_operator_reducer.rs.skel", body);
console.log(`written ${p}\nreplace_factories=${uniqueReplaces.length} (重载展开 ${replaces.length}) reduce_cases=${cases.length}`);
