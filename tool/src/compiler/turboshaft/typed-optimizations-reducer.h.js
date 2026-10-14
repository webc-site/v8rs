#!/usr/bin/env bun
// tool/src/compiler/turboshaft/typed-optimizations-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/typed-optimizations-reducer.h
// → v8-turboshaft/src/typed_optimizations_reducer.rs.skel
//
// 提取面: UniformReducerAdapter 复用; 类型驱动的化简决策锚。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/typed-optimizations-reducer.h");

// 1. 哨兵
for (const [re, name] of [
  [/class TypedOptimizationsReducer\s*\n\s*: public UniformReducerAdapter<TypedOptimizationsReducer, Next> \{/, "reducer 类 (Adapter 基)"],
  [/static_assert\(next_contains_reducer<Next, TypeInferenceReducer>::value\);/, "须栈含 TypeInference 断言"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(TypedOptimizations\)/, "boilerplate"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. ReduceOperation 决策面 (模板方法体)
const ops = [...h.matchAll(/ReduceInputGraph(\w+)\(OpIndex ig_index, const \w+& \w+\)/g)].map((m) => m[1]);
if (ops.length < 2) throw new Error(`决策面 ${ops.length} < 2`);

// 3. 类型查询锚
const queries = [...new Set([...h.matchAll(/As(\w+)\(\)|type\.(\w+)\(\)|GetType\(/g)].map((m) => m[1] || m[2]))];

let rust = banner(
  "src/compiler/turboshaft/typed-optimizations-reducer.h.js",
  "src/compiler/turboshaft/typed-optimizations-reducer.h",
  "类型化优化: 输入图类型驱动的化简",
);

rust += `
/// 原版 TypedOptimizationsReducer — 经 UniformReducerAdapter 统一入口,
/// 从 TypeInference 侧表读类型做化简决策。
/// v8: src/compiler/turboshaft/typed-optimizations-reducer.h
pub struct TypedOptimizationsReducer;

impl TypedOptimizationsReducer {
  /// ReduceInputGraph 决策面 (${ops.length} 个特化):
${ops.map((o) => `  /// v8: ReduceInputGraph${o}`).join("\n")}
  ///
  /// 决策依据: 输入图类型侧表 (${queries.slice(0, 8).join(", ")})
  /// 命中窄类型 (如 Word32 单值集) 时用常量替代; 否则透传 Next。
  pub fn reduce_input_graph_operation(&mut self, idx: OpIndex, op: &Operation) -> OpIndex { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "typed_optimizations_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`决策面=${ops.length} 查询=${queries.length}`);
