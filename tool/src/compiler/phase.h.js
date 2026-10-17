#!/usr/bin/env bun
// tool/src/compiler/phase.h.js
// v8/src/compiler/phase.h
// → v8-compiler-passes/src/phase.rs.skel
// 相位基础设施: 解析 5 个 zone 名常量 / PhaseKind 枚举 / 3 个相位宏 /
// Turbolizer JSON 辅助结构体字段面。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/phase.h");

const need = (cond, msg) => {
  if (!cond) throw new Error(`phase: ${msg}`);
};

for (const [s, name] of [
  ["inline constexpr char kCodegenZoneName[] = \"codegen-zone\";", "codegen-zone"],
  ["inline constexpr char kGraphZoneName[] = \"graph-zone\";", "graph-zone"],
  ["inline constexpr char kInstructionZoneName[] = \"instruction-zone\";", "instruction-zone"],
  ["inline constexpr char kRegisterAllocationZoneName[] =", "register-allocation-zone"],
  ["inline constexpr char kRegisterAllocatorVerifierZoneName[] =", "register-allocator-verifier-zone"],
  ["#define DECL_PIPELINE_PHASE_CONSTANTS_HELPER(Name, Kind, Mode,", "宏 HELPER"],
  ["#define DECL_PIPELINE_PHASE_CONSTANTS(Name)", "宏 常规"],
  ["#define DECL_MAIN_THREAD_PIPELINE_PHASE_CONSTANTS(Name)", "宏 主线程"],
  ["enum class PhaseKind {", "PhaseKind"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// —— zone 名常量 (真解析 5 个) ——
const zoneNames = [...h.matchAll(/constexpr char k(\w+ZoneName)\[\] =\s*"([^"]+)"/g)].map((m) => [m[1], m[2]]);
need(zoneNames.length === 5, `zone 名 ${zoneNames.length}`);
for (const [, v] of zoneNames) need(v.endsWith("-zone"), `zone 名异常 ${v}`);

// —— PhaseKind 成员 ——
const kinds = [...h.slice(h.indexOf("enum class PhaseKind {")).matchAll(/k(\w+),/g)].map((m) => m[1]);
need(kinds.length === 3 && kinds.join(",") === "Turbofan,Turbolev,Turboshaft", `PhaseKind ${kinds}`);

// —— 宏体常量面 (真解析 HELPER 展开产物) ——
const helper = h.slice(h.indexOf("#define DECL_PIPELINE_PHASE_CONSTANTS_HELPER"), h.indexOf("namespace v8::internal"));
const hasRcs = helper.includes("kRuntimeCallCounterId") && helper.includes("kCounterMode");
const phaseNameLit = helper.includes('return "V8.TF" #Name;');
const syncPoint = helper.includes("synchronization_point_name");
need(hasRcs && phaseNameLit && syncPoint, "宏体常量面不全");

// —— JSON 辅助结构体 (真解析; SourcePositionAsJSON/NodeOriginAsJSON 在 turbofan-graph-visualizer.h) ——
const jsonStructs = [...h.matchAll(/^struct (\w+AsJSON) \{/gm)].map((m) => m[1]);
need(jsonStructs.length === 3, `JSON 结构 ${jsonStructs}`);
// TurbolizerCodeOffsetsInfo 字段面
const offsetsBody = h.slice(h.indexOf("struct TurbolizerCodeOffsetsInfoAsJSON"));
const offsetFields = [...offsetsBody.slice(0, 900).matchAll(/offsets_info->(\w+)/g)].map((m) => m[1]);
need(offsetFields.length >= 7, `offsets 字段 ${offsetFields.length}`);

const body = `${banner("src/compiler/phase.h.js", "src/compiler/phase.h", "管线相位公共件: 相位名/zone 名/相位种类 + Turbolizer JSON 输出辅助")}
//! phase — 相位元数据与 JSON 可视化辅助
//!
//! DECL_PIPELINE_PHASE_CONSTANTS(Name) 展开:
//!   phase_name() = "V8.TF"#Name; kKind = PhaseKind::kTurbofan;
//!   kCounterMode = kThreadSpecific; synchronization_point_name() = "Turbofan"#Name;
//!   (RCS 开时再加 kRuntimeCallCounterId = kOptimize##Name)。

use std::sync::LazyLock;

/// 原版 PhaseKind — 相位所属管线
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseKind {
  /// kTurbofan
  Turbofan,
  /// kTurbolev
  Turbolev,
  /// kTurboshaft
  Turboshaft,
}

// —— zone 名常量 (5 个, 字面量逐字一致) ——
${zoneNames.map(([k, v]) => `/// k${k}\npub const ${k.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()}: &str = "${v}";`).join("\n\n")}

/// 原版 DECL_PIPELINE_PHASE_CONSTANTS 展开 — 单相位常量包
pub struct PhaseConstants {
  /// "V8.TF"#Name
  pub phase_name: &'static str,
  /// PhaseKind (kTurbofan / kTurbolev / kTurboshaft)
  pub kind: PhaseKind,
  /// "Turbofan"#Name
  pub synchronization_point_name: &'static str,
  /// RuntimeCallCounterId::kOptimize##Name (RCS 开)
  pub runtime_call_counter_id: Option<RuntimeCallCounterId>,
}

/// 原版 TurbolizerInstructionStartInfo — 单指令起点三元组
pub struct TurbolizerInstructionStartInfo {
  /// gap_pc_offset
  pub gap_pc_offset: i32,
  /// arch_instr_pc_offset
  pub arch_instr_pc_offset: i32,
  /// condition_pc_offset
  pub condition_pc_offset: i32,
}

// —— Turbolizer JSON 辅助 (3 个 AsJSON 包装: ${jsonStructs.join(", ")}; SourcePosition/NodeOrigin 两 AsJSON 在 turbofan-graph-visualizer.h) ——

/// 原版 InstructionStartsAsJSON — instructionOffsetToPCOffset 段
/// v8: src/compiler/phase.h InstructionStartsAsJSON
pub fn instruction_starts_as_json(out: &mut String, instr_starts: &[TurbolizerInstructionStartInfo]) {
  todo!("v8: src/compiler/phase.h operator<<(InstructionStartsAsJSON)")
}

/// 原版 TurbolizerCodeOffsetsInfoAsJSON — codeOffsetsInfo 段 (字段 ${offsetFields.join("/")}）
/// v8: src/compiler/phase.h operator<<(TurbolizerCodeOffsetsInfoAsJSON)
pub fn code_offsets_info_as_json(out: &mut String, info: &TurbolizerCodeOffsetsInfo) {
  todo!("v8: src/compiler/phase.h operator<<(TurbolizerCodeOffsetsInfoAsJSON)")
}

/// 原版 BlockStartsAsJSON — blockIdToOffset 段
/// v8: src/compiler/phase.h operator<<(BlockStartsAsJSON)
pub fn block_starts_as_json(out: &mut String, block_starts: &[i32]) {
  todo!("v8: src/compiler/phase.h operator<<(BlockStartsAsJSON)")
}

/// 原版 PrintCode / TraceSchedule 自由函数
/// v8: src/compiler/phase.h PrintCode
pub fn print_code(isolate: &Isolate, code: &Code, info: &OptimizedCompilationInfo) {
  todo!("v8: src/compiler/phase.h PrintCode")
}

/// v8: src/compiler/phase.h TraceSchedule
pub fn trace_schedule(info: &OptimizedCompilationInfo, data: &TFPipelineData<'_, '_>, schedule: &Schedule<'_>, phase_name: &str) {
  todo!("v8: src/compiler/phase.h TraceSchedule")
}
`;

const p = writeSrc("v8-compiler-passes", "phase.rs.skel", body);
console.log(`written ${p}\nzone_names=${zoneNames.length} phase_kinds=${kinds.length} json_structs=${jsonStructs.length} offset_fields=${offsetFields.length} rcs_face=${hasRcs}`);
