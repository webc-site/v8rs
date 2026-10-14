#!/usr/bin/env bun
// tool/src/maglev/passes.js — 优化遍: optimizer/截断/phi选择/CSE/区间/逃逸/统计/tracer
// 覆盖: src/maglev/maglev-graph-optimizer.h/.cc + maglev-truncation.h/.cc +
//       src/maglev/maglev-phi-representation-selector.h/.cc +
//       src/maglev/maglev-cse.h + maglev-graph-processor.h +
//       src/maglev/maglev-range.h + maglev-range-analysis.h +
//       src/maglev/maglev-range-verification.h/.cc +
//       src/maglev/maglev-post-hoc-optimizations-processors.h +
//       src/maglev/maglev-pre-regalloc-codegen-processors.h +
//       src/maglev/maglev-pipeline-statistics.h/.cc + maglev-tracer.h +
//       src/maglev/turbolev-escape-analysis.h/.cc + src/maglev/hamt.h
// 产出: v8-maglev-frontend/src/passes.rs.skel
// 复跑幂等: 纯文本扫描 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/maglev/passes.js";
const FILES = [
  "src/maglev/maglev-graph-optimizer.h", "src/maglev/maglev-graph-optimizer.cc",
  "src/maglev/maglev-truncation.h", "src/maglev/maglev-truncation.cc",
  "src/maglev/maglev-phi-representation-selector.h", "src/maglev/maglev-phi-representation-selector.cc",
  "src/maglev/maglev-cse.h", "src/maglev/maglev-graph-processor.h",
  "src/maglev/maglev-range.h", "src/maglev/maglev-range-analysis.h",
  "src/maglev/maglev-range-verification.h", "src/maglev/maglev-range-verification.cc",
  "src/maglev/maglev-post-hoc-optimizations-processors.h",
  "src/maglev/maglev-pre-regalloc-codegen-processors.h",
  "src/maglev/maglev-pipeline-statistics.h", "src/maglev/maglev-pipeline-statistics.cc",
  "src/maglev/maglev-tracer.h",
  "src/maglev/turbolev-escape-analysis.h", "src/maglev/turbolev-escape-analysis.cc",
  "src/maglev/hamt.h",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[passes.js] ${m}`); };

const go = txt["src/maglev/maglev-graph-optimizer.h"];
if (!go.includes("class V8_EXPORT_PRIVATE MaglevGraphOptimizer {")) die("缺 MaglevGraphOptimizer");
const tr = txt["src/maglev/maglev-truncation.h"];
for (const needle of ["class PropagateTruncationProcessor {", "class TruncationProcessor {"]) {
  if (!tr.includes(needle)) die(`truncation 缺针 ${needle}`);
}
const ps = txt["src/maglev/maglev-phi-representation-selector.h"];
for (const needle of ["class MaglevPhiRepresentationSelector {",
  "enum class UntaggingKind : uint8_t {", "enum class RetaggingKind {"]) {
  if (!ps.includes(needle)) die(`phi-selector 缺针 ${needle}`);
}
const ra = txt["src/maglev/maglev-range-analysis.h"];
for (const needle of ["class LessEqualConstraint {", "class NodeRanges {", "class RangeProcessor {"]) {
  if (!ra.includes(needle)) die(`range 缺针 ${needle}`);
}
const rh = txt["src/maglev/maglev-range.h"];
if (!rh.includes("class Range {")) die("缺 Range");
const tr2 = txt["src/maglev/maglev-tracer.h"];
for (const needle of ["enum class TraceColor : uint8_t {", "class Tracer {", "class TraceLogger {"]) {
  if (!tr2.includes(needle)) die(`tracer 缺针 ${needle}`);
}
const ea = txt["src/maglev/turbolev-escape-analysis.h"];
for (const needle of ["enum class CandidateStatus { kCanMaybeElide, kCannotElide };",
  "class EscapeAnalysis {"]) {
  if (!ea.includes(needle)) die(`escape 缺针 ${needle}`);
}
const hamt = txt["src/maglev/hamt.h"];
for (const needle of ["class HAMT {", "class HAMTTest;"]) {
  if (!hamt.includes(needle)) die(`hamt 缺针 ${needle}`);
}
const stat = txt["src/maglev/maglev-pipeline-statistics.h"];
if (!stat.includes("PipelineStatistics") && !stat.includes("pipeline_statistics")) die("缺 PipelineStatistics 面");

const out = [];
out.push(banner(GEN, FILES.join(", "), "优化遍全族 (Phase kPostOptimizer→kRangeAnalysis)"));
out.push(`//! 遍序 (禁重排, 与 MaglevPhase 对齐): PostOptimizer → LoopOptimization(LICM) →`);
out.push(`//!   PrePhiUntagging → PhiUntagging → EscapeAnalysis → RangeAnalysis → AnyUseMarking → DeadNodeSweeping。`);
out.push(``);
out.push(`// ── MaglevGraphOptimizer (总装: kna_processor_ + saved_kna_ 回滚; LoopLabel 内类) ──`);
out.push(``);
out.push(`// ── 截断双遍 (TruncationPropagation kTruncationPropagation → Truncation kTruncation) ──`);
out.push(`//   PropagateTruncationProcessor; TruncationProcessor`);
out.push(``);
out.push(`// ── Phi 表示选择 (kPrePhiUntagging/kPhiUntagging; UntaggingKind/RetaggingKind) ──`);
out.push(`//   MaglevPhiRepresentationSelector; 用 UseRepresentation hint (见 repr.js)`);
out.push(``);
out.push(`// ── CSE + 图处理器底 (maglev-cse.h; graph-processor.h 遍历框架) ──`);
out.push(``);
out.push(`// ── 区间分析 (Range/LessEqualConstraint/NodeRanges/RangeProcessor + verification) ──`);
out.push(``);
out.push(`// ── 逃逸分析 Turbolev (CandidateStatus 二态; AssumeMap/AssumeType 消费面) ──`);
out.push(`//   EscapeAnalysis; AssertEscapeAnalysisElided 断言臂`);
out.push(``);
out.push(`// ── 管线统计 + 追踪 (PipelineStatistics; TraceColor/Tracer/TraceLogger 调试面) ──`);
out.push(``);
out.push(`// ── HAMT (持久化哈希阵列映射 trie; KNA/逃逸的不可变表底) ──`);
out.push(``);
out.push(`// ── regalloc 前/后处理器 (pre-regalloc-codegen / post-hoc-optimizations 两头) ──`);
out.push(``);
out.push(`// 对数: optimizer 1 + 截断 2 + phi 3 + range 4 + tracer 3 + 逃逸 2 + hamt 2 + 统计 1 全过`);

writeSrc("v8-maglev-frontend", "passes.rs.skel", out.join("\n") + "\n");
console.log(`OK passes.js → v8-maglev-frontend/src/passes.rs.skel`);
