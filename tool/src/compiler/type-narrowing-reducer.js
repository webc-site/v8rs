#!/usr/bin/env bun
// tool/src/compiler/type-narrowing-reducer.js
// v8/src/compiler/type-narrowing-reducer.h + type-narrowing-reducer.cc
// → v8-compiler-passes/src/type_narrowing_reducer.rs.skel
// 类型收窄 reducer: 解析 Reduce 的 case 面 (显式 case + 宏列表展开自 opcodes.h)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/type-narrowing-reducer.h");
const cc = readV8("src/compiler/type-narrowing-reducer.cc");
const opcodesH = readV8("src/compiler/opcodes.h");

const need = (cond, msg) => {
  if (!cond) throw new Error(`type-narrowing-reducer: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE TypeNarrowingReducer final", "类定义"],
  [": public NON_EXPORTED_BASE(AdvancedReducer) {", "AdvancedReducer 基类"],
  ['return "TypeNarrowingReducer";', "reducer_name"],
  ["Reduction TypeNarrowingReducer::Reduce(Node* node) {", "cc Reduce"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— Reduce 的 case 面 (真解析: 显式 case + DECLARE_CASE + 宏列表) ——
const reduceBody = cc.slice(cc.indexOf("Reduction TypeNarrowingReducer::Reduce"), cc.indexOf("TFGraph* TypeNarrowingReducer::graph"));
const explicitCases = [...reduceBody.matchAll(/case IrOpcode::k(\w+):/g)].map((m) => m[1]);
const declareCases = [...reduceBody.matchAll(/DECLARE_CASE\((\w+)\)/g)].map((m) => m[1]);
// 宏列表展开 (opcodes.h 的 V(x) 表)
function expandList(name) {
  const m = opcodesH.match(new RegExp(`#define ${name}\\(V\\) \\\\\\n([\\\\\\s\\S]*?)\\n\\n`));
  if (!m) throw new Error(`${name} 未找到`);
  const body = m[1].replace(/\\\n/g, " ");
  const used = [...body.matchAll(/\b([A-Z0-9_]+_LIST)\(V\)/g)].map((x) => x[1]);
  const own = [...body.matchAll(/\bV\((\w+)\)/g)].map((x) => x[1]);
  return [...own, ...used.flatMap(expandList)];
}
const binops = expandList("SIMPLIFIED_NUMBER_BINOP_LIST");
const unops = expandList("SIMPLIFIED_NUMBER_UNOP_LIST");
need(binops.length >= 10 && unops.length >= 5, `宏展开 binop=${binops.length} unop=${unops.length}`);
// 展开引用核对: 宏名确实出现在 Reduce 体
need(reduceBody.includes("SIMPLIFIED_NUMBER_BINOP_LIST(DECLARE_CASE)"), "binop 宏引用缺失");
need(reduceBody.includes("SIMPLIFIED_NUMBER_UNOP_LIST(DECLARE_CASE)"), "unop 宏引用缺失");
const binopArity2 = /op_typer_\.Name\(NodeProperties::GetType\(node->InputAt\(0\)\),/.test(reduceBody);
const totalCases = explicitCases.length + declareCases.length + binops.length + unops.length;

const body = `${banner("src/compiler/type-narrowing-reducer.js", "src/compiler/type-narrowing-reducer.h + type-narrowing-reducer.cc", "依 OperationTyper 重算并收窄节点静态类型")}
//! type-narrowing-reducer — 用 OperationTyper 结果更新 NodeProperties 类型
//!
//! case 面 ${totalCases} 类:
//! - 显式: ${explicitCases.join(", ")}
//! - DECLARE_CASE: ${declareCases.join(", ")}
//! - SIMPLIFIED_NUMBER_BINOP_LIST 展开 ${binops.length}: ${binops.join(", ")}
//! - SIMPLIFIED_NUMBER_UNOP_LIST 展开 ${unops.length}: ${unops.join(", ")}

/// 原版 TypeNarrowingReducer — AdvancedReducer
pub struct TypeNarrowingReducer<'z, 'g> {
  /// jsgraph_
  pub jsgraph: &'g JSGraph<'z>,
  /// op_typer_
  pub op_typer: OperationTyper<'z>,
}

impl TypeNarrowingReducer<'_, '_> {
  /// 原版 reducer_name
  pub const REDUCER_NAME: &'static str = "TypeNarrowingReducer";

  /// 原版 Reduce — new_type = op_typer(...); 与原类型取交, 收紧则 Changed
  /// v8: src/compiler/type-narrowing-reducer.cc TypeNarrowingReducer::Reduce
  pub fn reduce(&mut self, node: &Node) -> Reduction {
    todo!("v8: src/compiler/type-narrowing-reducer.cc TypeNarrowingReducer::Reduce (${totalCases} case)")
  }

  /// 原版 NumberLessThan 专属: PlainNumber 区间比较 → singleton_true/false
  /// v8: src/compiler/type-narrowing-reducer.cc TypeNarrowingReducer::Reduce (kNumberLessThan 分支)
  fn narrow_number_less_than(&mut self, node: &Node) -> Type {
    todo!("v8: src/compiler/type-narrowing-reducer.cc TypeNarrowingReducer::Reduce kNumberLessThan")
  }

  /// 原版 TypeGuard 专属: TypeTypeGuard(op, input_type)
  /// v8: src/compiler/type-narrowing-reducer.cc TypeNarrowingReducer::Reduce (kTypeGuard 分支)
  fn narrow_type_guard(&mut self, node: &Node) -> Type {
    todo!("v8: src/compiler/type-narrowing-reducer.cc TypeNarrowingReducer::Reduce kTypeGuard")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "type_narrowing_reducer.rs.skel", body);
console.log(
  [
    `written ${p}`,
    `explicit=${explicitCases.length} (${explicitCases.join(",")})`,
    `declare=${declareCases.length} (${declareCases.join(",")})`,
    `binop展开=${binops.length} unop展开=${unops.length} 总case=${totalCases}`,
    `binop二元形态=${binopArity2}`,
  ].join("\n"),
);
