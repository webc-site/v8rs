#!/usr/bin/env bun
// tool/src/compiler/turboshaft/select-lowering-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/select-lowering-reducer.h
// → v8-turboshaft/src/select_lowering_reducer.rs.skel
//
// 提取面: REDUCE(Select) 的 CMove/分支决策表 (Implementation × rep)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/select-lowering-reducer.h");

// 1. 哨兵
for (const [re, name] of [
  [/class SelectLoweringReducer : public Next/, "reducer 类"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(SelectLowering\)/, "boilerplate"],
  [/REDUCE\(Select\)\(V<Word32> cond, V<Any> vtrue, V<Any> vfalse,/, "REDUCE(Select) 签名"],
  [/SelectOp::Implementation::kForceBranch/, "kForceBranch"],
  [/SelectOp::Implementation::kForceCMove/, "kForceCMove"],
  [/SelectOp::Implementation::kAny/, "kAny"],
  [/NewLoopInvariantVariable\(rep\)/, "loop 不变量变量"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. 决策表: rep -> SupportedOperations 查询 (4 个探测) + 恒 false 组
const repCases = [...h.matchAll(/case RegisterRepresentation::Enum::k(\w+):\s*\n\s*use_cmove = SupportedOperations::(\w+)\(\);/g)].map((m) => ({
  rep: m[1],
  probe: m[2],
}));
if (repCases.length !== 4) throw new Error(`rep 探测 ${repCases.length} != 4`);
const falseReps = [...h.matchAll(/case RegisterRepresentation::Enum::k(\w+):(?=\s*\n\s*case|\s*\n\s*\/\/)/g)].map((m) => m[1]);
if (falseReps.length < 2) throw new Error(`恒 false rep ${falseReps.length} < 2`);

let rust = banner(
  "src/compiler/turboshaft/select-lowering-reducer.h.js",
  "src/compiler/turboshaft/select-lowering-reducer.h",
  "Select 降低: 三目选择到菱形控制流",
);

rust += `
/// 原版 SelectLoweringReducer — 把 Select(cond, vtrue, vfalse) 降为
/// if/else 菱形 (Loop 不变量 Variable 承载结果), 除非目标机支持 CMove。
/// v8: src/compiler/turboshaft/select-lowering-reducer.h
pub struct SelectLoweringReducer;

impl SelectLoweringReducer {
  /// v8: select-lowering-reducer.h REDUCE(Select)
  pub fn reduce_select(
    &mut self, cond: V<Word32>, vtrue: V<Any>, vfalse: V<Any>,
    rep: RegisterRepresentation, hint: BranchHint,
    implem: SelectImplementation,
  ) -> V<Any> {
    // 决策表 (原版 switch):
    // - kForceBranch -> 菱形
    // - kForceCMove  -> 透传 Next::ReduceSelect(kForceCMove)
    // - kAny         -> 按表示查 SupportedOperations:
${repCases.map((c) => `    //   ${c.rep}: ${c.probe}()`).join("\n")}
    //   ${falseReps.join(" / ")}: false (不用 CMove)
    // 若走菱形: result = NewLoopInvariantVariable(rep);
    //   IF(cond) SetVariable(result, vtrue) ELSE SetVariable(result, vfalse)
    //   return GetVariable(result)
    todo!()
  }
}
`;

const p = writeSrc("v8-turboshaft", "select_lowering_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`rep探测=${repCases.length} 恒false rep=${falseReps.length}`);
