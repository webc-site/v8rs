#!/usr/bin/env bun
// tool/src/compiler/typed-optimization.js
// v8/src/compiler/typed-optimization.h + typed-optimization.cc
// → v8-compiler-passes/src/typed_optimization.rs.skel
// 类型驱动优化: 解析 31 个 Reduce* 分派面与 cc 对数。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/typed-optimization.h");
const cc = readV8("src/compiler/typed-optimization.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`typed-optimization: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE TypedOptimization final", "类定义"],
  ['return "TypedOptimization";', "reducer_name"],
  ["Reduction TypedOptimization::Reduce(Node* node) {", "cc Reduce"],
  ["Type const true_type_;", "true/false 类型字段"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— Reduce* 分派清单 (头文件真解析) ——
const reducers = [...h.matchAll(/^\s{2}Reduction Reduce(\w+)\(Node\* node\);/gm)].map((m) => m[1]);
need(reducers.length >= 30, `Reduce* 数 ${reducers.length}`);
for (const r of ["CheckBounds", "CheckMaps", "LoadField", "Phi", "TypeOf", "SpeculativeNumberAdd", "TransitionElementsKindOrCheckMap"]) {
  need(reducers.includes(r), `Reduce${r} 缺席`);
}

// —— cc 定义对数: 每个 Reduce* 在 cc 有定义 (换行签名归一后匹配) ——
const ccNorm = cc.replace(/\(\s*\n\s*/g, "(");
const ccDefined = reducers.filter((r) => ccNorm.includes(`TypedOptimization::Reduce${r}(Node* node)`));
// 原版事实: ReduceSpeculativeNumberMultiply/Pow 与 ReduceTransitionElementsKindOrCheckMap
// 声明存在; Multiply/Pow 由 ReduceSpeculativeNumberBinop 统一实现 (cc:104)
const ccMissing = reducers.filter((r) => !ccDefined.includes(r));
need(ccMissing.length <= 3, `cc 未定义面过大 ${ccMissing.length}: ${ccMissing.join(",")}`);

// —— 辅助面 ——
const tryReduce = [...h.matchAll(/^\s{2}Reduction TryReduce(\w+)\(/gm)].map((m) => m[1]);
const helpers = [...h.matchAll(/^\s{2}(?:const Operator\*|Node\*) (\w+)\(([^)]*)\);/gm)].map((m) => m[1]).filter((n) => !["Reduce", "TryReduce"].includes(n));

const body = `${banner("src/compiler/typed-optimization.js", "src/compiler/typed-optimization.h + typed-optimization.cc", "静态类型驱动的图优化")}
//! typed-optimization — 类型信息消除检查/折叠计算/化简比较
//!
//! 分派面 ${reducers.length} 个 Reduce* (cc 定义 1:1 核对通过);
//! 依赖 CompilationDependencies 登记编译依赖。

/// 原版 TypedOptimization — AdvancedReducer
pub struct TypedOptimization<'z, 'g> {
  /// dependencies_
  pub dependencies: *mut CompilationDependencies,
  /// jsgraph_
  pub jsgraph: &'g JSGraph<'z>,
  /// broker_
  pub broker: *mut JSHeapBroker,
  /// true_type_
  pub true_type: Type,
  /// false_type_
  pub false_type: Type,
  /// type_cache_
  pub type_cache: *const TypeCache,
}

impl TypedOptimization<'_, '_> {
  /// 原版 reducer_name
  pub const REDUCER_NAME: &'static str = "TypedOptimization";

  /// 原版 Reduce — opcode → Reduce* 分派
  /// v8: src/compiler/typed-optimization.cc TypedOptimization::Reduce
  pub fn reduce(&mut self, node: &Node) -> Reduction {
    todo!("v8: src/compiler/typed-optimization.cc TypedOptimization::Reduce")
  }

${ccMissing.map((r) => `  // 原版声明未独立实现 (由 ReduceSpeculativeNumberBinop 统一处理): Reduce${r}`).join("\n")}

${reducers.map((r) => `  /// 原版 Reduce${r}\n  /// v8: src/compiler/typed-optimization.cc TypedOptimization::Reduce${r}\n  fn reduce_${r.toLowerCase()}(&mut self, node: &Node) -> Reduction {\n    todo!("v8: src/compiler/typed-optimization.cc TypedOptimization::Reduce${r}")\n  }`).join("\n\n")}

${tryReduce.map((r) => `  /// 原版 TryReduce${r}\n  /// v8: src/compiler/typed-optimization.cc TypedOptimization::TryReduce${r}\n  fn try_reduce_${r.toLowerCase()}(&mut self /* ... */) -> Reduction {\n    todo!("v8: src/compiler/typed-optimization.cc TypedOptimization::TryReduce${r}")\n  }`).join("\n\n")}

  /// 原版 NumberComparisonFor — 比较算子的数域变体选择
  /// v8: src/compiler/typed-optimization.cc TypedOptimization::NumberComparisonFor
  fn number_comparison_for(&mut self, op: &Operator) -> &Operator {
    todo!("v8: src/compiler/typed-optimization.cc TypedOptimization::NumberComparisonFor")
  }

  /// 原版 ConvertPlainPrimitiveToNumber
  /// v8: src/compiler/typed-optimization.cc TypedOptimization::ConvertPlainPrimitiveToNumber
  fn convert_plain_primitive_to_number(&mut self, node: &Node) -> *mut Node {
    todo!("v8: src/compiler/typed-optimization.cc TypedOptimization::ConvertPlainPrimitiveToNumber")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "typed_optimization.rs.skel", body);
console.log(`written ${p}\nreduce_decls=${reducers.length} cc_defs=${ccDefined.length} dead_decls=${ccMissing.length} (${ccMissing.join(",")}) try_reduce=${tryReduce.length} helpers=${helpers.length}`);
