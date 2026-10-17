#!/usr/bin/env bun
// tool/src/compiler/turboshaft/required-optimization-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/required-optimization-reducer.h
// → v8-turboshaft/src/required_optimization_reducer.rs.skel
//
// 提取面: REDUCE(Phi) 三条正确性必需规则 (同输入 / 同常量 / 同 RttCanon)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/required-optimization-reducer.h");

// 1. 哨兵
for (const [re, name] of [
  [/class RequiredOptimizationReducer : public Next/, "reducer 类"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(RequiredOptimization\)/, "boilerplate"],
  [/REDUCE\(Phi\)\(base::Vector<const OpIndex> inputs,/, "REDUCE(Phi) 签名"],
  [/bool same_inputs = true;/, "同输入规则"],
  [/first_constant->kind, first_constant->storage/, "常量重发"],
  [/maybe_rtt->rtts\(\) == first_rtt->rtts\(\)/, "RttCanon 规则"],
  [/#if V8_ENABLE_WEBASSEMBLY/, "wasm 条件块"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. 三条规则结构
const rules = [
  { name: "same_inputs", anchor: "if (same_inputs) {" },
  { name: "same_constant", anchor: "first_constant" },
  { name: "same_rtt_canon", anchor: "first_rtt" },
];
for (const r of rules) {
  if (!h.includes(r.anchor === "first_constant" ? "const ConstantOp* first_constant" : r.anchor === "first_rtt" ? "const RttCanonOp* first_rtt" : r.anchor)) {
    throw new Error(`规则锚缺失: ${r.name}`);
  }
}

let rust = banner(
  "src/compiler/turboshaft/required-optimization-reducer.h.js",
  "src/compiler/turboshaft/required-optimization-reducer.h",
  "必需优化: Phi 输入折叠三条规则",
);

rust += `
/// 原版 RequiredOptimizationReducer — 指令选择正确性依赖的折叠
/// (须在每次 VariableReducer 之后运行)。
/// v8: src/compiler/turboshaft/required-optimization-reducer.h
pub struct RequiredOptimizationReducer;

impl RequiredOptimizationReducer {
  /// v8: required-optimization-reducer.h REDUCE(Phi)
  pub fn reduce_phi(&mut self, inputs: &[OpIndex], rep: RegisterRepresentation) -> OpIndex {
    // 规则 1 (same_inputs): 所有输入全等 -> 直接返回第一个输入
    //   (VariableReducer 为 call target 引入的 phi 须消掉, 否则指令选择
    //    识别不出直接调用目标)
    // 规则 2 (same_constant): 全部输入是同一 ConstantOp (值相等) ->
    //   重发该常量 __ ReduceConstant(kind, storage)
    // 规则 3 (same_rtt_canon, 仅 V8_ENABLE_WEBASSEMBLY):
    //   全部输入是同 rtts() 同 type_index 的 RttCanonOp ->
    //   重发 __ ReduceRttCanon(rtts, type_index)
    //   (供 WasmGCTypedOptimizationReducer 解类型索引;
    //    loop peeling/unrolling 会引入此类 phi)
    todo!()
  }
}
`;

const p = writeSrc("v8-turboshaft", "required_optimization_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`规则=${rules.length}`);
