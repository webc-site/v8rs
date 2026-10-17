#!/usr/bin/env bun
// tool/src/compiler/turboshaft/late-escape-analysis-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/late-escape-analysis-reducer.{h,cc}
// → v8-turboshaft/src/late_escape_analysis_reducer.rs.skel
//
// 提取面: LateEscapeAnalysisAnalyzer 7 方法 (h 声明 + cc 定义双侧核对) +
// LateEscapeAnalysisReducer (Analyze 钩子) + allocs_ 收集策略。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/late-escape-analysis-reducer.h");
const cc = readV8("src/compiler/turboshaft/late-escape-analysis-reducer.cc");

// 1. analyzer 方法面 (h 声明, 保序)
const analyzerM = h.match(/class LateEscapeAnalysisAnalyzer\s*\{([\s\S]*?)\n\};/);
if (!analyzerM) throw new Error("未找到 LateEscapeAnalysisAnalyzer");
const declMethods = [...analyzerM[1].matchAll(/^\s*(?:void|bool) (\w+)\(/gm)].map((m) => m[1]);
const expected = ["Run", "RecordAllocateUse", "CollectUsesAndAllocations", "FindRemovableAllocations", "AllocationIsEscaping", "EscapesThroughUse", "MarkToRemove"];
if (JSON.stringify(declMethods) !== JSON.stringify(expected)) {
  throw new Error(`h 方法面 ${JSON.stringify(declMethods)} != 预期 7 项`);
}

// 2. cc 定义面: 每个方法必须有 LateEscapeAnalysisAnalyzer::X 定义
for (const name of expected) {
  const re = new RegExp(`(?:void|bool) LateEscapeAnalysisAnalyzer::${name}\\(`);
  if (!re.test(cc)) throw new Error(`cc 缺定义: LateEscapeAnalysisAnalyzer::${name}`);
}

// 3. reducer 类 + 字段面
for (const [re, label] of [
  [/class LateEscapeAnalysisReducer : public Next/, "LateEscapeAnalysisReducer 基类链"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(LateEscapeAnalysis\)/, "BOILERPLATE(LateEscapeAnalysis)"],
  [/void Analyze\(\)\s*\{\s*analyzer_\.Run\(\);/, "Analyze → analyzer_.Run()"],
  [/LateEscapeAnalysisAnalyzer analyzer_\{Asm\(\)\.modifiable_input_graph\(\),/, "analyzer_ 成员初始化"],
  [/ZoneVector<OpIndex> allocs_;/, "allocs_ 字段"],
  [/\/\/ \{allocs_\} is filled with all of the AllocateOp of the graph/, "allocs_ 收集注释锚"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 4. cc 语义锚: 可移除判定的关键调用
for (const [re, label] of [
  [/Is<AllocateOp>/, "AllocateOp 识别"],
  [/StoreOp|Is<StoreOp>/, "Store 使用检查"],
]) {
  if (!re.test(cc)) throw new Error(`cc 语义锚缺失: ${label}`);
}

// 5. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/late-escape-analysis-reducer.h.js",
  "src/compiler/turboshaft/late-escape-analysis-reducer.{h,cc}",
  "晚期逃逸分析: 无逃逸 AllocateOp 删除骨架",
);
rust += `
/// v8: src/compiler/turboshaft/late-escape-analysis-reducer.h LateEscapeAnalysisAnalyzer
///
/// 策略: allocs_ 收集全部 AllocateOp → 收集使用 → 找可移除分配
/// (仅 Store 自身且无其它逃逸路径) → 标记移除。
pub struct LateEscapeAnalysisAnalyzer {
  /// allocs_: 全图 AllocateOp 列表
  pub allocs: Vec<crate::index::OpIndex>,
}

impl LateEscapeAnalysisAnalyzer {
`;
const sigs = {
  Run: "() — 两遍: 收集使用+分配 → 找可移除",
  RecordAllocateUse: "(OpIndex alloc, OpIndex use) — 记录分配的使用",
  CollectUsesAndAllocations: "() — 遍历全图填 allocs_ 与使用",
  FindRemovableAllocations: "() — 逐分配判定可移除",
  AllocationIsEscaping: "(OpIndex alloc) -> bool — 逃逸判定",
  EscapesThroughUse: "(OpIndex alloc, OpIndex use) -> bool — 单使用逃逸",
  MarkToRemove: "(OpIndex alloc) — 标记 (替换为过 diminution)",
};
for (const name of expected) {
  const snakeName = name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
  rust += `  /// 原版 LateEscapeAnalysisAnalyzer::${name}${sigs[name]}
  /// v8: src/compiler/turboshaft/late-escape-analysis-reducer.cc LateEscapeAnalysisAnalyzer::${name}
  pub fn ${snakeName}(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
`;
}
rust += `}

/// v8: src/compiler/turboshaft/late-escape-analysis-reducer.h LateEscapeAnalysisReducer
pub struct LateEscapeAnalysisReducer;

impl LateEscapeAnalysisReducer {
  /// 原版 void Analyze() — analyzer_.Run() 后 Next::Analyze()
  pub fn analyze(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
}
`;

const p = writeSrc("v8-turboshaft", "late_escape_analysis_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`analyzer方法=${declMethods.length} (h=cc 双侧核对) reducer=1 cc锚=4`);
