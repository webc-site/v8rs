#!/usr/bin/env bun
// tool/src/compiler/turboshaft/type-inference-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/type-inference-reducer.h
// → v8-turboshaft/src/type_inference_reducer.rs.skel
//
// 提取面: TypeInferenceReducerArgs 两枚举; REDUCE 面; UniformReducerAdapter
// 复用面。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/type-inference-reducer.h");

// 1. 两枚举
const igM = h.match(/enum class InputGraphTyping \{([^}]+)\}/);
const ogM = h.match(/enum class OutputGraphTyping \{([^}]+)\}/);
if (!igM || !ogM) throw new Error("未找到 Input/OutputGraphTyping");
const ig = [...igM[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((m) => m[1]);
const og = [...ogM[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((m) => m[1]);
if (ig.length !== 2) throw new Error(`InputGraphTyping ${ig.length} != 2`);
if (og.length !== 3) throw new Error(`OutputGraphTyping ${og.length} != 3`);

// 2. 哨兵
for (const [re, name] of [
  [/struct TypeInferenceReducerArgs\s*\n\s*: base::ContextualClass<TypeInferenceReducerArgs>/, "Contextual args"],
  [/class TypeInferenceReducer\s*\n\s*: public UniformReducerAdapter<TypeInferenceReducer, Next> \{/, "reducer 类 (Adapter 基)"],
  [/static_assert\(next_is_bottom_of_assembler_stack<Next>::value\);/, "栈底断言"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(TypeInference\)/, "boilerplate"],
  [/CanBeTyped\(const Op& operation\)/, "CanBeTyped"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 3. REDUCE 面
const reduces = [...h.matchAll(/\bREDUCE\((\w+)\)\(/g)].map((m) => m[1]);

let rust = banner(
  "src/compiler/turboshaft/type-inference-reducer.h.js",
  "src/compiler/turboshaft/type-inference-reducer.h",
  "类型推断 reducer: 复制时逐操作打类型",
);

rust += `
/// 原版 TypeInferenceReducerArgs::InputGraphTyping
/// v8: type-inference-reducer.h TypeInferenceReducerArgs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InputGraphTyping {
  /// kNone: 不为输入图算类型
  None = 0,
  /// kPrecise: 输入图完整 fixpoint
  Precise = 1,
}

/// 原版 TypeInferenceReducerArgs::OutputGraphTyping
/// v8: type-inference-reducer.h TypeInferenceReducerArgs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputGraphTyping {
  /// kNone: 不为输出图算类型
  None = 0,
  /// kPreserveFromInputGraph: 尽量沿用输入图类型
  PreserveFromInputGraph = 1,
  /// kRefineFromInputGraph: 沿用并算新节点/更精确类型
  RefineFromInputGraph = 2,
}

/// 原版 TypeInferenceReducer — 必须是 reducer 栈最后一个 (类注释 NOTE,
/// static_assert 栈底); 基类即 UniformReducerAdapter<TypeInferenceReducer,Next>,
/// Contextual args (Scope) 提供上述配置, 未特化操作统一走 ReduceOperation。
/// REDUCE 特化面 (${reduces.length} 个):
${reduces.map((r) => `/// - REDUCE(${r})`).join("\n")}
/// - CanBeTyped(op): outputs_rep().size() > 0 才参与
/// v8: src/compiler/turboshaft/type-inference-reducer.h TypeInferenceReducer
pub struct TypeInferenceReducer;
`;

const p = writeSrc("v8-turboshaft", "type_inference_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`Input=${ig.length} Output=${og.length} REDUCE=${reduces.length}`);
