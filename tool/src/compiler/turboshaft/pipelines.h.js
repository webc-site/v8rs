#!/usr/bin/env bun
// tool/src/compiler/turboshaft/pipelines.h.js
// 机械提取 v8/src/compiler/turboshaft/pipelines.{h,cc}
// → v8-turboshaft/src/pipelines.rs.skel
//
// 提取面: kTempZoneName; SimplificationAndNormalizationPhase;
// detail::NoLinkage; RUN_MAYBE_ABORT 宏; OptimizeTurboshaftGraph 等
// Run<Phase> 执行序列; Pipeline/BuiltinPipeline 入口面。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/pipelines.h");
const cc = readV8("src/compiler/turboshaft/pipelines.cc");

// 1. 哨兵
for (const [re, name] of [
  [/inline constexpr char kTempZoneName\[\] = "temp-zone";/, "kTempZoneName"],
  [/#define RUN_MAYBE_ABORT\(phase, \.\.\.\)/, "RUN_MAYBE_ABORT"],
  [/struct SimplificationAndNormalizationPhase/, "SimplificationAndNormalizationPhase"],
  [/enum class NoLinkage \{ kNoLinkage \};/, "detail::NoLinkage"],
  [/class V8_EXPORT_PRIVATE Pipeline/, "Pipeline"],
  [/class BuiltinPipeline : public Pipeline/, "BuiltinPipeline"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. OptimizeTurboshaftGraph 的 Run<...> 序列 (含 wasm 分支; 区间到 SelectInstructions)
const seqM = h.match(/bool OptimizeTurboshaftGraph\(Linkage\* linkage\) \{([\s\S]*?)\n  \[\[nodiscard\]\] bool SelectInstructions/);
if (!seqM) throw new Error("未找到 OptimizeTurboshaftGraph");
const runSeq = [...seqM[1].matchAll(/RUN_MAYBE_ABORT\((?:turboshaft::)?(\w+)(?:, (\w+))?\)/g)].map((m) =>
  m[2] ? `${m[1]}(${m[2]})` : m[1],
);
if (runSeq.length < 12) throw new Error(`优化 phase 序列 ${runSeq.length} < 12`);
for (const must of ["WasmInJSInliningPhase", "MachineLoweringPhase", "CodeEliminationAndSimplificationPhase", "SimplificationAndNormalizationPhase", "DecompressionOptimizationPhase"]) {
  if (!runSeq.some((s) => s.startsWith(must))) throw new Error(`序列缺 ${must}`);
}

// 3. 后段序列 (SelectInstructions / AssembleCode)
const selM = h.match(/bool SelectInstructions\(Linkage\* linkage\) \{([\s\S]*?)\n  \}/);
const selSeq = selM ? [...selM[1].matchAll(/Run<(\w+Phase)>\(/g)].map((m) => m[1]) : [];

// 4. cc: BuiltinPipeline 内置优化序列
const biM = cc.match(/void BuiltinPipeline::OptimizeBuiltin\(\) \{([\s\S]*?)\n\}/);
if (!biM) throw new Error("未找到 OptimizeBuiltin");
const builtinSeq = [...biM[1].matchAll(/Run<(\w+Phase)>\(/g)].map((m) => m[1]);
if (builtinSeq.length !== 8) throw new Error(`builtin 序列 ${builtinSeq.length} != 8`);

let rust = banner(
  "src/compiler/turboshaft/pipelines.h.js",
  "src/compiler/turboshaft/pipelines.{h,cc}",
  "流水线编排: phase 执行序列与入口",
);

rust += `
/// 原版 kTempZoneName — phase 临时 zone 名
/// v8: src/compiler/turboshaft/pipelines.h kTempZoneName
pub const TEMP_ZONE_NAME: &str = "temp-zone";

/// 原版 RUN_MAYBE_ABORT(phase, ...) — phase 返回 false 即中止整条流水线
/// v8: src/compiler/turboshaft/pipelines.h RUN_MAYBE_ABORT
macro_rules! run_maybe_abort {
  ($phase:expr) => { if !$phase.run() { return None; } };
}

/// 原版 SimplificationAndNormalizationPhase (pipelines.cc 定义 Run)
/// v8: src/compiler/turboshaft/pipelines.{h,cc} SimplificationAndNormalizationPhase
pub struct SimplificationAndNormalizationPhase;

/// 原版 detail::NoLinkage — 无 linkage 构造 Pipeline 的哨兵重载
/// v8: src/compiler/turboshaft/pipelines.h detail::NoLinkage
pub enum NoLinkage { NoLinkage }

/// 原版 Pipeline — Turboshaft 主流水线 (data_ + linkage)
/// v8: src/compiler/turboshaft/pipelines.h Pipeline
pub struct Pipeline<'d> {
  pub data: &'d mut PipelineData,
  pub linkage: Option<&'d Linkage>,
}

impl<'d> Pipeline<'d> {
  /// OptimizeTurboshaftGraph 的 phase 执行序 (在位顺序):
${runSeq.map((s) => `  /// - ${s}`).join("\n")}
  ///
  /// v8: src/compiler/turboshaft/pipelines.h Pipeline::OptimizeTurboshaftGraph
  pub fn optimize_turboshaft_graph(&mut self) -> bool { todo!() }

  /// SelectInstructions 段:
${selSeq.map((s) => `  /// - ${s}`).join("\n")}
  /// v8: src/compiler/turboshaft/pipelines.h Pipeline::SelectInstructions
  pub fn select_instructions(&mut self) -> bool { todo!() }

  /// v8: pipelines.h Pipeline::CreateGraphWithMaglev — 跑 TurbolevFrontendPipeline
  /// 后接 TurbolevGraphBuildingPhase (可选 bailout)
  pub fn create_graph_with_maglev(&mut self, linkage: &Linkage) -> bool { todo!() }
  /// v8: pipelines.h Pipeline::GenerateCode / AllocateRegisters / AssembleCode
  pub fn generate_code(&mut self) -> Option<Code> { todo!() }
  pub fn allocate_registers(&mut self, config: &RegisterConfiguration) -> bool { todo!() }
  pub fn assemble_code(&mut self, linkage: &Linkage) -> bool { todo!() }
}

/// 原版 BuiltinPipeline::OptimizeBuiltin 序列 (CSA 内置, ${builtinSeq.length} phase):
${builtinSeq.map((s) => `/// - ${s}`).join("\n")}
/// v8: src/compiler/turboshaft/pipelines.cc BuiltinPipeline::OptimizeBuiltin
pub struct BuiltinPipeline<'d>(pub Pipeline<'d>);
`;

const p = writeSrc("v8-turboshaft", "pipelines.rs.skel", rust);
console.log(`written ${p}`);
console.log(`优化序列=${runSeq.length} 选指序列=${selSeq.length} builtin序列=${builtinSeq.length}`);
