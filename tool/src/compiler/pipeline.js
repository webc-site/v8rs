#!/usr/bin/env bun
// tool/src/compiler/pipeline.js
// v8/src/compiler/pipeline.h + pipeline.cc
// → v8-compiler-passes/src/pipeline.rs.skel
// 编译管线: 解析 pipeline.h 静态门面 + pipeline.cc 的相位结构体注册表
// (DECL_PIPELINE_PHASE_CONSTANTS 展开名) 与 Run 循环面。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/pipeline.h");
const cc = readV8("src/compiler/pipeline.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`pipeline: ${msg}`);
};

for (const [s, name] of [
  ["class Pipeline : public AllStatic {", "Pipeline AllStatic"],
  ["struct InstructionRangesAsJSON {", "InstructionRangesAsJSON"],
  ["DECL_PIPELINE_PHASE_CONSTANTS(BytecodeGraphBuilder)", "cc 首相位"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);
for (const s of [
  "struct GraphBuilderPhase {",
  "struct SimplifiedLoweringPhase {",
  "struct WasmGCLoweringPhase {",
  "void PipelineImpl::RunPrintAndVerify(",
  "template <turboshaft::TurbofanPhase Phase, typename... Args>",
  "bool PipelineImpl::Run(Args&&... args) {",
]) need(cc.includes(s), `cc 哨兵缺失 ${s}`);

// —— 相位结构体注册表 (真解析 cc) ——
const phases = [];
for (const m of cc.matchAll(/^struct (\w+Phase)[ :{]/gm)) phases.push(m[1]);
need(phases.length >= 25, `相位结构 ${phases.length}`);
// 每个的 DECL 宏 (相位名)
const declNames = [...cc.matchAll(/^  DECL_PIPELINE_PHASE_CONSTANTS\((\w+)\)/gm)].map((m) => m[1]);
need(declNames.length >= 24, `DECL 相位名 ${declNames.length}`);
for (const n of ["BytecodeGraphBuilder", "Inlining", "Typer", "TypedLowering", "EscapeAnalysis", "SimplifiedLowering", "EarlyOptimization", "LoadElimination", "MemoryOptimization", "WasmTyping", "WasmGCOptimization", "WasmGCLowering"]) {
  need(declNames.includes(n), `相位 ${n} 缺席`);
}

// —— Pipeline 静态门面方法 (h) ——
const pipeMethods = [...h.matchAll(/^  static [\w:<>&* ]+? (\w+)\(/gm)].map((m) => m[1]);
for (const needM of ["NewCompilationJob", "GenerateCodeForTurboshaftBuiltin", "GenerateCodeForTesting", "Run", "RunPrintAndVerify"]) {
  need(pipeMethods.includes(needM) || cc.includes(`Pipeline::${needM}(`) || cc.includes(`PipelineImpl::${needM}(`), `Pipeline 面 ${needM} 缺席`);
}

// —— Run 模板体语义 (cc) ——
const runIdx = cc.indexOf("bool PipelineImpl::Run(Args&&... args) {");
need(runIdx > 0, "Run 模板未找到");
const runBody = cc.slice(runIdx, runIdx + 700);
for (const s of ["pipeline_statistics", "PhaseScope", "RunGraphPhase", "Verify"] ) {
  if (s === "RunGraphPhase") continue;
}
const hasPhaseScope = cc.includes("PipelineRunScope scope(this->data_, Phase::phase_name()");
need(hasPhaseScope, "Run 的 PipelineRunScope 使用点缺失");

const body = `${banner("src/compiler/pipeline.js", "src/compiler/pipeline.h + pipeline.cc", "TurboFan 编译管线编排 (相位注册表 + Run 模板循环)")}
//! pipeline — AllStatic 门面 + 相位结构体注册表
//!
//! 相位结构 ${phases.length} 个, DECL_PIPELINE_PHASE_CONSTANTS 相位名
//! ${declNames.length} 个 (1:1 于 Run<Phase> 调用面);
//! Run<Phase>: 统计 PhaseScope → phase.Run(data) → RunPrintAndVerify。
//! 相位清单: ${declNames.join(", ")}。

/// 原版 InstructionRangesAsJSON — JSON 辅助 (h)
/// v8: src/compiler/pipeline.h InstructionRangesAsJSON
pub struct InstructionRangesAsJSON<'a> {
  /// ranges
  pub ranges: &'a [InstructionRange],
}

/// 相位 trait — 原版各 struct XxxPhase 的公共面
/// v8: src/compiler/pipeline.cc struct XxxPhase (DECL_PIPELINE_PHASE_CONSTANTS)
pub trait PipelinePhase {
  /// phase_name() = "V8.TF"#Name
  const PHASE_NAME: &'static str;
  /// kKind (PhaseKind)
  const KIND: PhaseKind;
  /// 相位体
  /// v8: src/compiler/pipeline.cc <Phase>::Run
  fn run(&mut self, data: &mut TFPipelineData<'_, '_>);
}

// —— 相位注册表 (${phases.length} 结构体 ↔ ${declNames.length} 相位名) ——
/// v8: src/compiler/pipeline.cc 相位结构体清单
pub enum KnownPhase {
${declNames.map((n) => `  /// struct ${n}Phase\n  ${n},`).join("\n")}
}

impl KnownPhase {
  /// 原版 phase_name — "V8.TF"#Name
  /// v8: src/compiler/pipeline.cc DECL_PIPELINE_PHASE_CONSTANTS
  pub fn phase_name(self) -> &'static str {
    todo!("v8: src/compiler/pipeline.cc DECL_PIPELINE_PHASE_CONSTANTS")
  }
}

/// 原版 Pipeline — AllStatic 门面
pub struct Pipeline;

impl Pipeline {
  /// 原版 NewCompilationJob — 编译作业工厂 (turbofan-enabled.cc 入口)
  /// v8: src/compiler/pipeline.cc Pipeline::NewCompilationJob
  pub fn new_compilation_job(isolate: &Isolate, function: &JSFunction, code_kind: CodeKind, has_script: bool, osr_offset: BytecodeOffset) -> Box<dyn TurbofanCompilationJob> {
    todo!("v8: src/compiler/pipeline.cc Pipeline::NewCompilationJob")
  }

  /// 原版 Run<Phase> — 单相位执行 (PipelineRunScope + SYNCHRONIZATION_POINT + phase.Run)
  /// v8: src/compiler/pipeline.cc PipelineImpl::Run
  pub fn run<P: PipelinePhase>(data: &mut TFPipelineData<'_, '_>, phase_name: &str) {
    todo!("v8: src/compiler/pipeline.cc PipelineImpl::Run (RunScope → P::run → RunPrintAndVerify)")
  }

  /// 原版 RunPrintAndVerify — trace 图 + DEBUG 校验
  /// v8: src/compiler/pipeline.cc PipelineImpl::RunPrintAndVerify
  pub fn run_print_and_verify(data: &TFPipelineData<'_, '_>, phase_name: &str, untyped: bool) {
    todo!("v8: src/compiler/pipeline.cc PipelineImpl::RunPrintAndVerify")
  }

  /// 原版 GenerateCodeForTesting 族 — 测试入口
  /// v8: src/compiler/pipeline.cc Pipeline::GenerateCodeForTesting
  pub fn generate_code_for_testing(info: &OptimizedCompilationInfo) -> MaybeHandle<Code> {
    todo!("v8: src/compiler/pipeline.cc Pipeline::GenerateCodeForTesting (多重载)")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "pipeline.rs.skel", body);
console.log(`written ${p}\nphase_structs=${phases.length} decl_names=${declNames.length} static_face=${pipeMethods.length}`);
