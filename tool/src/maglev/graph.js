#!/usr/bin/env bun
// tool/src/maglev/graph.js — Graph/BasicBlock/Phase + 打印·标注·序列化·校验
// 覆盖: src/maglev/maglev-graph.h + maglev-graph.cc +
//       src/maglev/maglev-basic-block.h + src/maglev/maglev-phase.h +
//       src/maglev/maglev-graph-labeller.h/.cc + maglev-graph-printer.h/.cc +
//       src/maglev/maglev-graph-serializer.h/.cc + maglev-graph-verifier.h
// 产出: v8-maglev-ir/src/graph.rs.skel
// 复跑幂等: 纯文本扫描 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/maglev/graph.js";
const FILES = [
  "src/maglev/maglev-graph.h", "src/maglev/maglev-graph.cc",
  "src/maglev/maglev-basic-block.h", "src/maglev/maglev-phase.h",
  "src/maglev/maglev-graph-labeller.h", "src/maglev/maglev-graph-labeller.cc",
  "src/maglev/maglev-graph-printer.h", "src/maglev/maglev-graph-printer.cc",
  "src/maglev/maglev-graph-serializer.h", "src/maglev/maglev-graph-serializer.cc",
  "src/maglev/maglev-graph-verifier.h",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[graph.js] ${m}`); };

// —— Graph 锚 ————
const gh = txt["src/maglev/maglev-graph.h"];
for (const needle of [
  "class Graph final : public ZoneObject {",
  "ZoneVector<BasicBlock*> blocks_;",
  "ZoneVector<InitialValue*>& osr_values()",
  "ZoneVector<ValueNode*>& parameters()",
  "bool is_osr() const {",
  "struct MaglevLoopPeelInfo {",
  "struct InliningTreeDebugInfo : public ZoneObject {",
]) {
  if (!gh.includes(needle)) die(`graph.h 缺针 ${needle}`);
}

// —— BasicBlock 锚 ————
const bb = txt["src/maglev/maglev-basic-block.h"];
for (const needle of [
  "class BasicBlock {",
  "void Print() const;",
  "bool is_merge_block() const { return type_ == kMerge; }",
  "base::SmallVector<BasicBlock*, 2> successors() const;",
  "bool has_state() const { return type_ == kMerge && state_ != nullptr; }",
  "void set_id(Id id) {",
  "kMerge, kEdgeSplit, kOther",
]) {
  if (!bb.includes(needle)) die(`basic-block 缺针 ${needle}`);
}

// —— Phase 锚: 17 相, 顺序即管线序 (打印/校验依赖此序) ————
const ph = txt["src/maglev/maglev-phase.h"];
const PHASES = ["kNone", "kMaglevGraphBuilding", "kInlining", "kLoopPeeling",
  "kTruncationPropagation", "kTruncation", "kPostOptimizer", "kLoopOptimization",
  "kPrePhiUntagging", "kPhiUntagging", "kEscapeAnalysis", "kRangeAnalysis",
  "kAnyUseMarking", "kDeadNodeSweeping", "kDuringRegAlloc", "kRegAlloc", "kCodeGen"];
for (const p of PHASES) {
  if (!ph.includes(`  ${p},`) && !ph.includes(`  ${p}\n`)) die(`缺 Phase ${p}`);
}
if (PHASES.length !== 17) die(`Phase 数=${PHASES.length}, 期望 17`);
if (!ph.includes("inline const char* PhaseName(MaglevPhase phase)")) die("缺 PhaseName");

// —— 打印·标注·序列化·校验锚 ————
const lab = txt["src/maglev/maglev-graph-labeller.h"];
for (const needle of ["class MaglevGraphLabeller {", "void PrintNodeLabel(std::ostream& os, const NodeBase* node,",
  "void PrintInput(std::ostream& os, ConstInput input, bool has_regalloc_data) {"]) {
  if (!lab.includes(needle)) die(`labeller 缺针 ${needle}`);
}
const pr = txt["src/maglev/maglev-graph-printer.h"];
for (const needle of ["void PrintGraph(std::ostream& os, Graph* const graph, MaglevPhase phase);",
  "void PrintGraphToFile(Graph* const graph, MaglevPhase phase);"]) {
  if (!pr.includes(needle)) die(`printer 缺针 ${needle}`);
}
const se = txt["src/maglev/maglev-graph-serializer.h"];
if (!se.includes("void PrintMaglevGraphAsJSON(MaglevCompilationInfo* info, Graph* graph,")) die("缺 PrintMaglevGraphAsJSON");
const ve = txt["src/maglev/maglev-graph-verifier.h"];
if (!ve.includes("class Graph;")) die("verifier 缺 Graph 前向声明");

const out = [];
out.push(banner(GEN, FILES.join(", "), "Graph/Block/Phase 17 + 打印标注序列化校验"));
out.push(`//! 管线序即 Phase 枚举序 (kNone→kCodeGen); 打印/校验的不变量依赖此序, 修订禁重排。`);
out.push(`//! 破环登记: Graph 只认 Block/Node ID 与布局; broker/isolate 回调走窄 trait (禁 dyn):`);
out.push(`//!   pub trait MaglevGraphEnv { /* JSHeapBroker 只读面 */ }`);
out.push(``);
out.push(`// ── MaglevPhase (17, 顺序即管线序) ──`);
for (const p of PHASES) out.push(`//   PHASE ${p}`);
out.push(``);
out.push(`// ── Graph (ZoneObject; blocks/osr_values/parameters 三表; OSR 经 compilation_unit) ──`);
out.push(`//   Graph::blocks/osr_values/parameters; is_osr(); Add/AddBlocksAt(重排 block id)`);
out.push(``);
out.push(`// ── BasicBlock (kMerge/kEdgeSplit/kOther; merge 块才有 state/KNA) ──`);
out.push(`//   is_merge_block/has_state/successors(≤2)/first_id+control.id 区间断言`);
out.push(``);
out.push(`// ── 打印·标注·序列化·校验 (修订阶段转 Rust 调试面, 非语义) ──`);
out.push(`//   MaglevGraphLabeller::PrintNodeLabel/PrintInput(has_regalloc_data 门)`);
out.push(`//   PrintGraph(os,graph,phase)/PrintGraphToFile; PrintMaglevGraphAsJSON; verifier`);
out.push(``);
out.push(`// 对数: graph 7 + block 7 + phase 17 + 打印 6 全过`);

writeSrc("v8-maglev-ir", "graph.rs.skel", out.join("\n") + "\n");
console.log(`OK graph.js: phase 17 → v8-maglev-ir/src/graph.rs.skel`);
