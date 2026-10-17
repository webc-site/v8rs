#!/usr/bin/env bun
// tool/src/compiler/pipeline-statistics.js
// v8/src/compiler/pipeline-statistics.h + pipeline-statistics.cc
// → v8-compiler-passes/src/pipeline_statistics.rs.skel
// 管线统计: 解析三层类 (Base/CommonStats/统计类) 与 PhaseScope RAII 面。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/pipeline-statistics.h");
const cc = readV8("src/compiler/pipeline-statistics.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`pipeline-statistics: ${msg}`);
};

for (const [s, name] of [
  ["class PipelineStatisticsBase {", "Base"],
  ["class CommonStats {", "CommonStats"],
  ["class TurbofanPipelineStatistics : public PipelineStatisticsBase,", "统计类"],
  ["static constexpr char kTraceCategory[] =", "kTraceCategory"],
  ["class V8_NODISCARD PhaseScope {", "PhaseScope"],
  ["class V8_NODISCARD PhaseScopeKind {", "PhaseScopeKind"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// kTraceCategory 真解析 (两个 trace 类别)
const traceCats = [...h.matchAll(/TRACE_DISABLED_BY_DEFAULT\("([^"]+)"\)/g)].map((m) => m[1]);
need(traceCats.length === 2 && traceCats.includes("v8.turbofan") && traceCats.includes("v8.wasm.turbofan"), `trace 类别 ${traceCats}`);

// CommonStats 字段 (真解析)
const cs = h.slice(h.indexOf("class CommonStats {"));
const csFields = [...cs.slice(0, cs.indexOf("};")).matchAll(/^\s{4}(?:std::unique_ptr<[\w:]+>|base::ElapsedTimer|size_t)(?: [\w:]+)? (\w+)(?: = \d+)?;/gm)].map((m) => m[1]);
need(csFields.length === 5, `CommonStats 字段 ${csFields.length} (${csFields.join(",")})`);

// cc 定义对数
const ccDefs = [
  "PipelineStatisticsBase::PipelineStatisticsBase(",
  "PipelineStatisticsBase::~PipelineStatisticsBase()",
  "void PipelineStatisticsBase::BeginPhaseKind(const char* phase_kind_name)",
  "void PipelineStatisticsBase::EndPhaseKind(",
  "void PipelineStatisticsBase::BeginPhase(const char* phase_name)",
  "void PipelineStatisticsBase::EndPhase(",
  "void PipelineStatisticsBase::CommonStats::Begin(",
  "void PipelineStatisticsBase::CommonStats::End(",
  "TurbofanPipelineStatistics::TurbofanPipelineStatistics(",
  "TurbofanPipelineStatistics::~TurbofanPipelineStatistics()",
  "void TurbofanPipelineStatistics::BeginPhaseKind(const char* name)",
  "void TurbofanPipelineStatistics::EndPhaseKind()",
  "void TurbofanPipelineStatistics::BeginPhase(const char* name)",
  "void TurbofanPipelineStatistics::EndPhase()",
];
for (const d of ccDefs) need(cc.includes(d), `cc 缺 ${d}`);

const body = `${banner("src/compiler/pipeline-statistics.js", "src/compiler/pipeline-statistics.h + pipeline-statistics.cc", "编译相位计时/内存统计与 trace 输出")}
//! pipeline-statistics — CompilationStatistics 的相位/相位类两层数据源
//!
//! trace 类别: ${traceCats.join(", ")}; CommonStats 五字段:
//! ${csFields.join(" / ")}; PhaseScope/PhaseScopeKind 为 RAII 配对 Begin/End。

/// 原版 PipelineStatisticsBase::CommonStats — 单区间统计
pub struct CommonStats {
  /// scope_ — ZoneStats::StatsScope
  pub scope: Option<StatsScope<'static>>,
  /// timer_
  pub timer: ElapsedTimer,
  /// outer_zone_initial_size_
  pub outer_zone_initial_size: usize,
  /// allocated_bytes_at_start_
  pub allocated_bytes_at_start: usize,
  /// graph_size_at_start_
  pub graph_size_at_start: usize,
}

/// 原版 PipelineStatisticsBase — 统计基类 (总/相位类/相位三层 CommonStats)
pub struct PipelineStatisticsBase {
  /// outer_zone_
  pub outer_zone: *mut Zone<'static>,
  /// zone_stats_
  pub zone_stats: *mut ZoneStats,
  /// compilation_stats_
  pub compilation_stats: Option<std::sync::Arc<CompilationStatistics>>,
  /// code_kind_
  pub code_kind: CodeKind,
  /// function_name_
  pub function_name: String,
  /// total_stats_
  pub total_stats: CommonStats,
  /// phase_kind_name_ / phase_kind_stats_
  pub phase_kind_name: Option<&'static str>,
  pub phase_kind_stats: CommonStats,
  /// phase_name_ / phase_stats_
  pub phase_name: Option<&'static str>,
  pub phase_stats: CommonStats,
}

impl PipelineStatisticsBase {
  /// 原版 BeginPhaseKind / EndPhaseKind
  /// v8: src/compiler/pipeline-statistics.cc PipelineStatisticsBase::BeginPhaseKind
  pub fn begin_phase_kind(&mut self, phase_kind_name: &'static str) {
    todo!("v8: src/compiler/pipeline-statistics.cc PipelineStatisticsBase::BeginPhaseKind")
  }

  /// 原版 BeginPhase / EndPhase — 计时 + zone 增量入 CompilationStatistics
  /// v8: src/compiler/pipeline-statistics.cc PipelineStatisticsBase::BeginPhase
  pub fn begin_phase(&mut self, name: &'static str) {
    todo!("v8: src/compiler/pipeline-statistics.cc PipelineStatisticsBase::BeginPhase")
  }

  /// 原版 CommonStats::Begin / End
  /// v8: src/compiler/pipeline-statistics.cc CommonStats::Begin
  pub fn common_stats_begin(stats: &mut CommonStats, pipeline_stats: &mut PipelineStatisticsBase) {
    todo!("v8: src/compiler/pipeline-statistics.cc CommonStats::Begin")
  }
}

/// 原版 TurbofanPipelineStatistics — TF 专用统计 (Malloced)
pub struct TurbofanPipelineStatistics {
  /// base
  pub base: PipelineStatisticsBase,
}

impl TurbofanPipelineStatistics {
  /// 原版 kTraceCategory — "${traceCats.join(",")}"
  pub const TRACE_CATEGORY: &'static str = "v8.turbofan,v8.wasm.turbofan";

  /// 原版 BeginPhaseKind — 记录 + TRACE_EVENT
  /// v8: src/compiler/pipeline-statistics.cc TurbofanPipelineStatistics::BeginPhaseKind
  pub fn begin_phase_kind(&mut self, name: &'static str) {
    todo!("v8: src/compiler/pipeline-statistics.cc TurbofanPipelineStatistics::BeginPhaseKind")
  }

  /// 原版 BeginPhase — 同上 (相位层)
  /// v8: src/compiler/pipeline-statistics.cc TurbofanPipelineStatistics::BeginPhase
  pub fn begin_phase(&mut self, name: &'static str) {
    todo!("v8: src/compiler/pipeline-statistics.cc TurbofanPipelineStatistics::BeginPhase")
  }
}

/// 原版 PhaseScope — RAII 相位区间
pub struct StatsPhaseScope<'s> {
  /// pipeline_stats_
  pub pipeline_stats: Option<&'s mut TurbofanPipelineStatistics>,
}

/// 原版 PhaseScopeKind — RAII 相位类区间
pub struct StatsPhaseScopeKind<'s> {
  /// pipeline_stats_
  pub pipeline_stats: Option<&'s mut TurbofanPipelineStatistics>,
}
`;

const p = writeSrc("v8-compiler-passes", "pipeline_statistics.rs.skel", body);
console.log(`written ${p}\ntrace_cats=${traceCats.length} common_stats_fields=${csFields.length} cc_defs=${ccDefs.length}`);
