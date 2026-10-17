#!/usr/bin/env bun
// tool/src/compiler/turboshaft/copying-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/copying-phase.{h,cc}
// → v8-turboshaft/src/copying_phase.rs.skel
//
// 提取面: 通用复制 phase 引擎 — CanHavePhis/ForCloning 两枚举 + GraphVisitor
// 访问器方法签名面 (VisitGraph/VisitAllBlocks/VisitBlock/VisitBlockBody/
// VisitBlockTerminator/CloneBlockAndGoto/CloneAndInlineBlock/MapToNewGraph)
// + CopyingPhase::Run 两入口签名 + trace 输出面 (PaddingSpace)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/copying-phase.h");
const cc = readV8("src/compiler/turboshaft/copying-phase.cc");

// 1. 枚举
function enumVariants(re, label, expect) {
  const m = h.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  const names = [...m[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((x) => x[1]);
  if (names.length !== expect) throw new Error(`${label} ${names.length} != ${expect}`);
  return names;
}
const canHavePhis = enumVariants(/enum class CanHavePhis\s*\{([^}]+)\}/, "CanHavePhis", 2);
const forCloning = enumVariants(/enum class ForCloning\s*\{([^}]+)\}/, "ForCloning", 2);

// 2. 顶层声明哨兵
for (const [re, label] of [
  [/using MaybeVariable = std::optional<Variable>;/, "MaybeVariable"],
  [/V8_EXPORT_PRIVATE int CountDecimalDigits\(uint32_t value\);/, "CountDecimalDigits"],
  [/struct PaddingSpace\s*\{/, "PaddingSpace"],
  [/class OutputGraphAssembler/, "OutputGraphAssembler"],
  [/class GraphVisitor : public OutputGraphAssembler<GraphVisitor<AfterNext>,/, "GraphVisitor 基类链"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(CopyingPhase\)/, "REDUCER_BOILERPLATE(CopyingPhase)"],
  [/template <typename Next>\s*class ReducerBaseForwarder;/, "ReducerBaseForwarder 前向声明"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 3. GraphVisitor 方法签名面 (登记清单; 逐个在 h 中断言存在)
const visitorMethods = [
  ["visit_graph", "template <bool trace_reduction> void VisitGraph()", "遍历入口: Asm().Analyze() → VisitAllBlocks"],
  ["visit_all_blocks", "void VisitAllBlocks()", "RPO 序遍历块"],
  ["visit_block", "void VisitBlock(const Block* input_block)", "单块: 前驱汇齐才 VisitBlockBody"],
  ["visit_block_body", "void VisitBlockBody(const Block*, CanHavePhis, const Block* successor)", "块体: phi 插入 (can_have_phis) + 逐操作"],
  ["visit_block_terminator", "void VisitBlockTerminator(const Operation& terminator, const Block* input_block, Block* current_block)", "终结操作分发"],
  ["clone_block_and_goto", "void CloneBlockAndGoto(const Block* input_block)", "克隆块 + Goto (LoopUnrolling/LoopPeeling)"],
  ["clone_and_inline_block", "void CloneAndInlineBlock(const Block* input_block)", "克隆并内联到当前块"],
  ["map_to_new_graph", "OpIndex MapToNewGraph(OpIndex old_index, int predecessor_index = -1)", "旧图→新图索引映射"],
  ["map_optional", "OptionalOpIndex MapToNewGraph(OptionalOpIndex old_index, ...)", "可选索引映射"],
  ["visit_graph_tpl", "static void Run(PipelineData*, Graph& input_graph, Zone* phase_zone, ...) [带 input_graph 版]", "CopyingPhase::Run 入口 1"],
  ["run", "static void Run(PipelineData* data, Zone* phase_zone) [默认入口]", "CopyingPhase::Run 入口 2: 构造输入/输出图对"],
];
for (const [, sig] of visitorMethods) {
  // 取签名中第一个标识符片段做存在性断言
  const key = sig.match(/(\w+)\(/)[1] === "template" ? sig.match(/void (\w+)\(/)[1] : sig.match(/(\w+)\(/)[1];
  if (!h.includes(key)) throw new Error(`GraphVisitor 方法缺失: ${key}`);
}
if (!/static void Run\(PipelineData\* data, Zone\* phase_zone\)/.test(h)) {
  throw new Error("CopyingPhase::Run 默认入口缺失");
}

// 4. cc 侧: trace 工具
if (!/std::ostream& operator<<\(std::ostream& os, PaddingSpace padding\)/.test(cc)) {
  throw new Error("cc PaddingSpace operator<< 缺失");
}
if (!/int CountDecimalDigits\(uint32_t value\)/.test(cc)) {
  throw new Error("cc CountDecimalDigits 定义缺失");
}

// 5. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/copying-phase.h.js",
  "src/compiler/turboshaft/copying-phase.{h,cc}",
  "通用复制 phase 引擎: GraphVisitor 方法面与两 Run 入口签名骨架",
);
rust += `
/// v8: src/compiler/turboshaft/copying-phase.h CanHavePhis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanHavePhis {
${canHavePhis.map((n, i) => `  /// k${n}\n  ${n} = ${i},`).join("\n")}
}

/// v8: src/compiler/turboshaft/copying-phase.h ForCloning
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForCloning {
${forCloning.map((n, i) => `  /// k${n}\n  ${n} = ${i},`).join("\n")}
}

/// 原版 MaybeVariable = std::optional<Variable>
pub type MaybeVariable = Option<crate::variable::Variable>;

/// v8: src/compiler/turboshaft/copying-phase.h GraphVisitor
///
/// 复制引擎核心 (模板 <AfterNext>, 混入 OutputGraphAssembler +
/// VariableReducer)。字段面 (原版私有域):
/// - input_graph_: &Graph (modifiable_input_graph)
/// - current_input_block_: 当前访问块
/// - op_mapping_ / block_mapping_: 输入→输出映射 sidetable
/// - blocks_needing_variables_: 块位图
/// - old_opindex_to_variables_: 旧索引→变量表
/// - blocks_to_clone_: 待克隆块集合
pub struct GraphVisitor {
  pub placeholder: (),
}

impl GraphVisitor {
`;
for (const [rustName, sig, comment] of visitorMethods) {
  rust += `
  /// 原版 ${sig}
  /// ${comment}
  pub fn ${rustName}(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
`;
}
rust += `}
`;

const p = writeSrc("v8-turboshaft", "copying_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`枚举=2 (${canHavePhis.length}+${forCloning.length} 变体) 方法面=${visitorMethods.length} Run入口=2`);
