#!/usr/bin/env bun
// tool/src/compiler/turboshaft/simplified-optimization-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/simplified-optimization-reducer.h
// → v8-turboshaft/src/simplified_optimization_reducer.rs.skel
//
// 提取面: 5 个 REDUCE 面 + flag 门控 (条件求值)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/simplified-optimization-reducer.h");

// 1. REDUCE 面
const reduces = [...h.matchAll(/\bREDUCE\((\w+)\)\(/g)].map((m) => m[1]);
if (reduces.length !== 5) throw new Error(`REDUCE ${reduces.length} != 5`);
for (const must of ["TruncateJSPrimitiveToUntagged", "TruncateJSPrimitiveToWord32OrDeopt", "ChangeOrDeopt", "ObjectIs", "WordBinopDeoptOnOverflow"]) {
  if (!reduces.includes(must)) throw new Error(`缺 REDUCE(${must})`);
}

// 2. 门控锚
for (const [re, name] of [
  [/class SimplifiedOptimizationReducer : public Next/, "reducer 类"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(SimplifiedOptimization\)/, "boilerplate"],
  [/ShouldSkipOptimizationStep\(\)/, "bisect 钩子"],
  [/LABEL_BLOCK\(no_change\)/, "no_change 标签"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}
const labelBlocks = [...h.matchAll(/LABEL_BLOCK\(no_change\)/g)].length;

let rust = banner(
  "src/compiler/turboshaft/simplified-optimization-reducer.h.js",
  "src/compiler/turboshaft/simplified-optimization-reducer.h",
  "simplified 层小优化: 截断/改写/溢出检查前移",
);

rust += `
/// 原版 SimplifiedOptimizationReducer — simplified 操作的小步优化
/// (在 wasm-in-js inlining 后运行)。
/// v8: src/compiler/turboshaft/simplified-optimization-reducer.h
pub struct SimplifiedOptimizationReducer;

impl SimplifiedOptimizationReducer {
  /// REDUCE 面 (${reduces.length} 个):
${reduces.map((r) => `  /// v8: simplified-optimization-reducer.h REDUCE(${r})`).join("\n")}
  ///
  /// 语义登记:
  /// - TruncateJSPrimitiveToUntagged: 按 From / UntaggedKind 决策截断序列
  ///   (kInt32 恒等回原输入, kBit 转 0/1)
  /// - TruncateJSPrimitiveToWord32OrDeopt: 不可安全截断时走 Deopt
  /// - ChangeOrDeopt: 表示变更失败转 DeoptimizeIf
  /// - ObjectIs: kind+input 双常量折叠 (ShouldSkipOptimizationStep 钩子)
  /// - WordBinopDeoptOnOverflow: 溢出后恢复原 op 语义
  /// 每个 REDUCE 以 LABEL_BLOCK(no_change) 收尾 (共 ${labelBlocks} 处)
  pub fn reduce(&mut self, op: &SimplifiedOp) -> OpIndex { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "simplified_optimization_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`REDUCE=${reduces.length} no_change标签=${labelBlocks}`);
