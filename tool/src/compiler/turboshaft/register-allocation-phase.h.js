#!/usr/bin/env bun
// tool/src/compiler/turboshaft/register-allocation-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/register-allocation-phase.h
// → v8-turboshaft/src/register_allocation_phase.rs.skel
//
// 提取面: 全部寄存器分配相关 phase struct (legacy 名 + 后端 builder 调用)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/register-allocation-phase.h");

// 1. phase struct 清单 + 各自 builder 调用
const phases = [...h.matchAll(
  /struct (\w+Phase) \{\s*\n\s*DECL_TURBOSHAFT_(?:MAIN_THREAD_PIPELINE_)?PHASE_CONSTANTS_WITH_LEGACY_NAME\(\s*(\w+)\s*\)([\s\S]*?)\n\};/g,
)].map((m) => {
  const body = m[3];
  const runM = body.match(/void Run\(PipelineData\* data, Zone\* temp_zone[^)]*\) \{([\s\S]*?)\n  \}/);
  if (!runM) throw new Error(`${m[1]} 缺 Run`);
  // builder 调用面: XxxBuilder(...).Method() / Xxx(...).Method()
  const calls = [...runM[1].matchAll(/(\w+) (\w+)\((?:[^;]*)\)\.(\w+)\(\)/g)].map((c) => `${c[1]} ${c[2]}`).filter((s) => s !== "ZoneVector result");
  const stmts = [...runM[1].matchAll(/^\s{4}(\w+ \w+\(|\w+::\w+\(|\w+\.\w+\()/gm)].map((s) => s[1].trim());
  return { name: m[1], legacy: m[2], actions: stmts };
});
if (phases.length !== 19) throw new Error(`phase struct ${phases.length} != 19`);
for (const must of ["MeetRegisterConstraintsPhase", "ResolvePhisPhase", "BuildLiveRangesPhase", "AllocateGeneralRegistersPhase", "OptimizeMovesPhase", "AssembleCodePhase", "FinalizeCodePhase"]) {
  if (!phases.some((p) => p.name === must)) throw new Error(`缺 ${must}`);
}

// 2. 全部 kOutputIsTraceableGraph = false
const traceableFalse = [...h.matchAll(/static constexpr bool kOutputIsTraceableGraph = false;/g)].length;
if (traceableFalse !== phases.length) throw new Error(`kOutputIsTraceableGraph=false ${traceableFalse} != ${phases.length}`);

// 3. 分配三类模板 phase
const tmpl = [...h.matchAll(/template <typename RegAllocator>\s*\nstruct (Allocate\w+Phase)/g)].map((m) => m[1]);
if (tmpl.length !== 3) throw new Error(`模板分配 phase ${tmpl.length} != 3`);

let rust = banner(
  "src/compiler/turboshaft/register-allocation-phase.h.js",
  "src/compiler/turboshaft/register-allocation-phase.h",
  "寄存器分配: 后端 phase 包装层",
);

rust += `
/// 原版寄存器分配 phase 族 — 每个 phase 都是后端 builder 的薄包装
/// (kOutputIsTraceableGraph = false, 全部带 legacy 名)。
/// v8: src/compiler/turboshaft/register-allocation-phase.h
pub trait RegAllocPhase {
  /// 原版 kPhaseName ("V8.TF"+CallStatsName) / kLegacyPhaseName
  const PHASE_NAME: &'static str;
  const LEGACY_PHASE_NAME: &'static str;
  /// v8: register-allocation-phase.h 各 Run(PipelineData*, Zone*)
  fn run(data: &mut PipelineData, temp_zone: &mut Zone);
}

/// phase → 后端动作登记 (${phases.length} 个):
${phases.map((p) => `/// - ${p.name} (legacy ${p.legacy}): ${p.actions.join("; ") || "直接调用"}`).join("\n")}
///
/// 模板三联: ${tmpl.join(" / ")} — 按 RegisterKind 泛化
/// (kGeneral / kDouble / kSimd128)。
macro_rules! reg_alloc_phases {
  ($($name:ident => $action:expr),* $(,)?) => { /* ... */ };
}

pub mod phases {
  pub struct MeetRegisterConstraintsPhase;
  pub struct ResolvePhisPhase;
  pub struct BuildLiveRangesPhase;
  pub struct BuildLiveRangeBundlesPhase;
  pub struct AllocateGeneralRegistersPhase<R>(pub R);
  pub struct AllocateFPRegistersPhase<R>(pub R);
  pub struct AllocateSimd128RegistersPhase<R>(pub R);
  pub struct DecideSpillingModePhase;
  pub struct AssignSpillSlotsPhase;
  pub struct CommitAssignmentPhase;
  pub struct PopulateReferenceMapsPhase;
  pub struct ConnectRangesPhase;
  pub struct ResolveControlFlowPhase;
  pub struct OptimizeMovesPhase;
  /// BUILTIN_BLOCK_POSITION 条件编译
  pub struct BlockPositioningPhase;
  pub struct FrameElisionPhase;
  pub struct JumpThreadingPhase;
  pub struct AssembleCodePhase;
  /// 主线程限定 (DECL_TURBOSHAFT_MAIN_THREAD_PIPELINE_...)
  pub struct FinalizeCodePhase;
}
`;

const p = writeSrc("v8-turboshaft", "register_allocation_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`phase=${phases.length} 模板分配=${tmpl.length} traceable=false:${traceableFalse}`);
