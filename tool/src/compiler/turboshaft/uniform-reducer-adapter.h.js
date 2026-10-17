#!/usr/bin/env bun
// tool/src/compiler/turboshaft/uniform-reducer-adapter.h.js
// 机械提取 v8/src/compiler/turboshaft/uniform-reducer-adapter.h
// → v8-turboshaft/src/uniform_reducer_adapter.rs.skel
//
// 提取面: UniformReducerAdapter 机制 (REDUCE 全展开 + Continuation 转发)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/uniform-reducer-adapter.h");

// 1. 哨兵
for (const [re, name] of [
  [/class UniformReducerAdapter : public Next \{/, "adapter 类"],
  [/auto ReduceOperation\(Args\.\.\. args\) \{/, "ReduceOperation"],
  [/auto ReduceInputGraphOperation\(OpIndex ig_index, const Op& operation\) \{/, "ReduceInputGraphOperation"],
  [/TURBOSHAFT_OPERATION_LIST\(REDUCE\)/, "全操作 X-macro 展开"],
  [/struct Reduce##op##Continuation final/, "Continuation 生成"],
  [/ReduceInputGraphOperation<op##Op,/, "RIG 统一入口"],
  [/ReduceOperation<Opcode::k##op, Reduce##op##Continuation>/, "R 统一入口"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. REDUCE 宏块结构
const macroBody = h.slice(h.indexOf("#define REDUCE(op)"), h.indexOf("TURBOSHAFT_OPERATION_LIST(REDUCE)"));
const genMethods = [...macroBody.matchAll(/auto (Reduce[\w#]+)\(/g)].map((m) => m[1]);
if (genMethods.length !== 3) throw new Error(`宏生成方法 ${genMethods.length} != 3`);

let rust = banner(
  "src/compiler/turboshaft/uniform-reducer-adapter.h.js",
  "src/compiler/turboshaft/uniform-reducer-adapter.h",
  "统一适配器: 逐操作 REDUCE 汇到单一入口",
);

rust += `
/// 原版 UniformReducerAdapter<Reducer, Next> — 把每个操作各自的
/// ReduceXyz / ReduceInputGraphXyz 转发到统一的:
///   ReduceOperation<opcode, Continuation>(args...)
///   ReduceInputGraphOperation<Op, Continuation>(ig_index, op)
/// 每操作生成 Reduce{op}Continuation{this_}, 提供:
///   ReduceInputGraph(idx, op) -> this_->ReduceInputGraph{op}
///   Reduce(args...)           -> this_->Reduce{op}
/// 转发选择 (类注释 NOTICE):
/// - Next::ReduceXyz  : 跳过本 reducer 的统一处理
/// - Adapter::ReduceXyz: 也走统一 ReduceOperation
/// v8: src/compiler/turboshaft/uniform-reducer-adapter.h UniformReducerAdapter
pub trait UniformReducerAdapter {
  /// v8: REDUCE 宏对 TURBOSHAFT_OPERATION_LIST 全展开 — 生成方法:
${genMethods.map((m) => `  /// - ${m === "Reduce" ? "Reduce (Continuation 内, 转发到 Reduce##op)" : `${m} (每操作一份)`}`).join("\n")}
  fn reduce_operation(&mut self, opcode: Opcode, args: &[OpIndex]) -> OpIndex;
  fn reduce_input_graph_operation(&mut self, idx: OpIndex, op: &Operation) -> OpIndex;
}
`;

const p = writeSrc("v8-turboshaft", "uniform_reducer_adapter.rs.skel", rust);
console.log(`written ${p}`);
console.log(`宏生成方法=${genMethods.length}`);
