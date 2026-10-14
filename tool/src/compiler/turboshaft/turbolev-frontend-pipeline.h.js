#!/usr/bin/env bun
// tool/src/compiler/turboshaft/turbolev-frontend-pipeline.h.js
// 机械提取 v8/src/compiler/turboshaft/turbolev-frontend-pipeline.{h,cc}
// → v8-turboshaft/src/turbolev_frontend_pipeline.rs.skel
//
// 提取面: TurbolevFrontendPipeline 方法面; maglev phase 过滤; 打印面。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/turbolev-frontend-pipeline.h");
const cc = readV8("src/compiler/turboshaft/turbolev-frontend-pipeline.cc");

// 1. 哨兵
for (const [re, name] of [
  [/class TurbolevFrontendPipeline \{/, "类"],
  [/std::optional<maglev::Graph\*> Run\(\);/, "Run 返回 maglev 图"],
  [/bool TurbolevPhaseMatchesFilter\(maglev::MaglevPhase phase\);/, "phase 过滤"],
  [/bool ShouldPrintMaglevGraph\(/, "打印判定"],
  [/void PrintBytecode\(\);/, "PrintBytecode"],
  [/void PrintMaglevGraph\(maglev::MaglevPhase phase\);/, "PrintMaglevGraph"],
  [/void PrintInliningTreeDebugInfo\(\);/, "PrintInliningTreeDebugInfo"],
  [/std::unique_ptr<maglev::MaglevCompilationInfo> compilation_info_;/, "compilation_info_"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc Run 的 maglev phase 序列 (RUN_MAYBE_ABORT(XxxPhase) 形式)
const runM = cc.match(/std::optional<maglev::Graph\*> TurbolevFrontendPipeline::Run\(\) \{([\s\S]*?)\n\}/);
if (!runM) throw new Error("cc 未找到 Run");
const maglevRuns = [...runM[1].matchAll(/RUN_MAYBE_ABORT\((\w+)(?:,\s*([^)]+))?\)/g)].map((m) => m[1]);
const maglevPhases = [...new Set(maglevRuns)];
if (maglevPhases.length < 8) throw new Error(`maglev phase ${maglevPhases.length} < 8`);

// 3. flag 面
const flags = [...new Set([...cc.matchAll(/v8_flags\.(\w+)/g)].map((m) => m[1]))];

let rust = banner(
  "src/compiler/turboshaft/turbolev-frontend-pipeline.h.js",
  "src/compiler/turboshaft/turbolev-frontend-pipeline.{h,cc}",
  "Turbolev 前端: bytecode -> maglev 图",
);

rust += `
/// 原版 TurbolevFrontendPipeline — 跑 maglev 前端产出 maglev::Graph,
/// 供 TurbolevGraphBuildingPhase 转 Turboshaft。
/// v8: src/compiler/turboshaft/turbolev-frontend-pipeline.{h,cc}
pub struct TurbolevFrontendPipeline<'d> {
  pub data: &'d mut PipelineData,
  pub compilation_info: MaglevCompilationInfo,
  pub graph: Option<*mut MaglevGraph>,
}

impl<'d> TurbolevFrontendPipeline<'d> {
  /// v8: cc Run — RUN_MAYBE_ABORT 驱动的 maglev phase 序列
  /// (${maglevPhases.length} 个, flag 条件各异):
${maglevPhases.map((p) => `  /// - ${p}`).join("\n")}
  /// 返回 Some(maglev::Graph*) 或 None (bailout)
  pub fn run(&mut self) -> Option<&'d mut MaglevGraph> { todo!() }

  /// v8: TurbolevPhaseMatchesFilter — print_filter 匹配
  fn turbolev_phase_matches_filter(&self, phase: MaglevPhase) -> bool { todo!() }
  /// v8: ShouldPrintMaglevGraph — trace_turbo_graph 或
  /// print_turbolev_frontend, 且过 phase 过滤
  fn should_print_maglev_graph(&self, phase: Option<MaglevPhase>) -> bool { todo!() }
  /// v8: PrintBytecode / PrintMaglevGraph / PrintInliningTreeDebugInfo
  fn print_bytecode(&self) { /* ... */ }
  fn print_maglev_graph(&self, phase: MaglevPhase) { /* ... */ }
  fn print_inlining_tree_debug_info(&self) { /* ... */ }
}

/// cc 涉及 flag: ${flags.join(", ")}
`;

const p = writeSrc("v8-turboshaft", "turbolev_frontend_pipeline.rs.skel", rust);
console.log(`written ${p}`);
console.log(`maglev phase=${maglevPhases.length} flags=${flags.length}`);
