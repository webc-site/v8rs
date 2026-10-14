#!/usr/bin/env bun
// tool/src/compiler/turboshaft/type-inference-analysis.h.js
// 机械提取 v8/src/compiler/turboshaft/type-inference-analysis.h
// → v8-turboshaft/src/type_inference_analysis.rs.skel
//
// 提取面: TypeInferenceAnalysis 方法面 (fixpoint 迭代 + Process* 分派)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/type-inference-analysis.h");

// 1. 哨兵
for (const [re, name] of [
  [/class TypeInferenceAnalysis \{/, "分析类"],
  [/GrowingOpIndexSidetable<Type> Run\(/, "Run 返回侧表"],
  [/void ProcessBlock\(const Block& block, uint32_t\* unprocessed_index\)/, "ProcessBlock"],
  [/void RefineTypesAfterBranch\(const BranchOp\* branch/, "分支精化"],
  [/void SetType\(OpIndex index, Type result_type, bool allow_narrowing/, "SetType"],
  [/using table_t = SnapshotTable<Type>;/, "SnapshotTable 复用"],
  [/loop_needs_revisit/, "循环重扫"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. Process* 分派面
const processMethods = [...h.matchAll(/^\s{2}void (Process\w+)\(/gm)].map((m) => m[1]);
if (processMethods.length !== 9) throw new Error(`Process* ${processMethods.length} != 9`);
for (const must of ["ProcessCheckTurboshaftTypeOf", "ProcessComparison", "ProcessConstant", "ProcessPhi", "ProcessWordBinop"]) {
  if (!processMethods.includes(must)) throw new Error(`缺 ${must}`);
}

let rust = banner(
  "src/compiler/turboshaft/type-inference-analysis.h.js",
  "src/compiler/turboshaft/type-inference-analysis.h",
  "类型推断分析: 输入图类型 fixpoint",
);

rust += `
/// 原版 TypeInferenceAnalysis — 输入图上的类型不动点分析,
/// 值表复用 SnapshotTable<Type>; 循环以 loop_needs_revisit 二扫。
/// v8: src/compiler/turboshaft/type-inference-analysis.h
pub struct TypeInferenceAnalysis<'g> {
  pub graph: &'g Graph,
  pub table: SnapshotTable<Type>,
}

impl<'g> TypeInferenceAnalysis<'g> {
  /// v8: Run — 返回 GrowingOpIndexSidetable<Type> (类型结果)
  pub fn run(&mut self) -> Vec<Type> { todo!() }
  /// v8: ProcessBlock — 块内 Process* 分派; uint32_t* unprocessed_index
  /// 驱动工作队列
  pub fn process_block(&mut self, block: &Block, unprocessed: &mut u32) { todo!() }

  /// Process* 分派面 (${processMethods.length} 个):
${processMethods.map((m) => `  /// v8: ${m}`).join("\n")}
  ///
  /// 另有:
  /// - RefineTypesAfterBranch: 按 Branch 条件精化 then/else 两侧类型
  ///   (Typer::BranchRefinements 承担具体规则)
  /// - RefineOperationType / SetType(allow_narrowing): 写回与收窄
  /// - ProcessLoopPhi 返回 bool (是否需重扫)
  pub fn refine_types_after_branch(&mut self, branch: &BranchOp, then_branch: bool) { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "type_inference_analysis.rs.skel", rust);
console.log(`written ${p}`);
console.log(`Process*=${processMethods.length}`);
