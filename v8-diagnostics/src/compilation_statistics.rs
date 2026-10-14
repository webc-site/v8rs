// 由 tool/src/diagnostics/compilation-statistics.js 从 v8/src/diagnostics/compilation-statistics.{h,cc} 生成; 修订就地改, 复跑覆盖
// 编译统计: BasicStats 字段/类层级/签名骨架

use std::{
  collections::BTreeMap,
  fmt::{Display, Formatter, Result as FmtResult, Write as _},
};

/// 原版 class BasicStats: 阶段耗时 + 内存 + 图规模 (字段机械对应)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BasicStats {
  /// 原版 delta_ (TimeDelta; 以微秒承载)
  pub delta_us: i64,
  /// 原版 total_allocated_bytes_ (默认 0)
  pub total_allocated_bytes_: usize,
  /// 原版 max_allocated_bytes_
  pub max_allocated_bytes_: usize,
  /// 原版 absolute_max_allocated_bytes_
  pub absolute_max_allocated_bytes_: usize,
  /// 原版 input_graph_size_
  pub input_graph_size_: usize,
  /// 原版 output_graph_size_
  pub output_graph_size_: usize,
  /// 原版 function_name_
  pub function_name_: String,
}

impl BasicStats {
  /// 原版 Accumulate(stats): 累加
  pub fn accumulate(&mut self, stats: &BasicStats) {
    self.delta_us = self.delta_us.saturating_add(stats.delta_us);
    self.total_allocated_bytes_ = self
      .total_allocated_bytes_
      .saturating_add(stats.total_allocated_bytes_);
    if stats.absolute_max_allocated_bytes_ > self.absolute_max_allocated_bytes_ {
      self.absolute_max_allocated_bytes_ = stats.absolute_max_allocated_bytes_;
      self.max_allocated_bytes_ = stats.max_allocated_bytes_;
      self.function_name_.clone_from(&stats.function_name_);
    }
    self.input_graph_size_ = self
      .input_graph_size_
      .saturating_add(stats.input_graph_size_);
    self.output_graph_size_ = self
      .output_graph_size_
      .saturating_add(stats.output_graph_size_);
  }

  /// 原版 AsJSON()
  #[must_use]
  pub fn as_json(&self) -> String {
    debug_assert!(!self.function_name_.contains('"'));
    format!(
      r#"{{"function_name":"{}","total_allocated_bytes":{},"max_allocated_bytes":{},"absolute_max_allocated_bytes":{}}}"#,
      self.function_name_,
      self.total_allocated_bytes_,
      self.max_allocated_bytes_,
      self.absolute_max_allocated_bytes_
    )
  }
}

/// 原版 class TotalStats : BasicStats (+ source_size_, count_)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TotalStats {
  pub base: BasicStats,
  /// 原版 source_size_
  pub source_size_: u64,
  /// 原版 count_
  pub count_: usize,
}

/// 原版 class OrderedStats : BasicStats (+ insert_order_)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OrderedStats {
  pub base: BasicStats,
  /// 原版 insert_order_
  pub insert_order_: usize,
}

/// 原版 class PhaseStats : OrderedStats (+ phase_kind_name_)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PhaseStats {
  pub base: BasicStats,
  pub insert_order_: usize,
  /// 原版 phase_kind_name_
  pub phase_kind_name_: String,
}

/// 原版 using PhaseKindStats = OrderedStats; PhaseKindMap/PhaseMap = 按名排序 map
pub type PhaseKindMap = BTreeMap<String, OrderedStats>;
pub type PhaseMap = BTreeMap<String, PhaseStats>;

/// 原版 struct AsPrintableStatistics
pub struct AsPrintableStatistics<'a> {
  /// 原版 compiler
  pub compiler: &'a str,
  /// 原版 s
  pub s: &'a CompilationStatistics,
  /// 原版 machine_output
  pub machine_output: bool,
}

/// 原版 class CompilationStatistics (单例语义见 cc Get)
#[derive(Default, Debug, Clone)]
pub struct CompilationStatistics {
  /// 原版 total_stats_
  total_stats_: TotalStats,
  /// 原版 phase_kind_map_
  phase_kind_map_: PhaseKindMap,
  /// 原版 phase_map_
  phase_map_: PhaseMap,
}

impl CompilationStatistics {
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// 原版 RecordPhaseStats(phase_kind_name, phase_name, stats)
  pub fn record_phase_stats(
    &mut self,
    phase_kind_name: &str,
    phase_name: &str,
    stats: &BasicStats,
  ) {
    let next_order = self.phase_map_.len();
    let entry = self
      .phase_map_
      .entry(phase_name.to_string())
      .or_insert_with(|| PhaseStats {
        base: BasicStats::default(),
        insert_order_: next_order,
        phase_kind_name_: phase_kind_name.to_string(),
      });
    entry.base.accumulate(stats);
  }

  /// 原版 RecordPhaseKindStats(phase_kind_name, stats)
  pub fn record_phase_kind_stats(&mut self, phase_kind_name: &str, stats: &BasicStats) {
    let next_order = self.phase_kind_map_.len();
    let entry = self
      .phase_kind_map_
      .entry(phase_kind_name.to_string())
      .or_insert_with(|| OrderedStats {
        base: BasicStats::default(),
        insert_order_: next_order,
      });
    entry.base.accumulate(stats);
  }

  /// 原版 RecordTotalStats(stats)
  pub fn record_total_stats(&mut self, stats: &BasicStats) {
    self.total_stats_.base.accumulate(stats);
    self.total_stats_.count_ = self.total_stats_.count_.saturating_add(1);
  }

  #[inline]
  #[must_use]
  pub const fn total_stats(&self) -> &TotalStats {
    &self.total_stats_
  }

  #[inline]
  #[must_use]
  pub const fn phase_kind_map(&self) -> &PhaseKindMap {
    &self.phase_kind_map_
  }

  #[inline]
  #[must_use]
  pub const fn phase_map(&self) -> &PhaseMap {
    &self.phase_map_
  }
}

const DASH_LINE: &str = "----------------------------------------------------------------------------------------------------------------------\n";
const PHASE_KIND_BREAK: &str = "                                   -----------------------------------------------------------------------------------\n";

fn write_line(
  os: &mut String,
  machine_format: bool,
  name: &str,
  compiler: &str,
  stats: &BasicStats,
  total_stats: &BasicStats,
) {
  let ms = stats.delta_us as f64 / 1000.0;
  let percent = if total_stats.delta_us == 0 {
    0.0
  } else {
    (stats.delta_us as f64 / total_stats.delta_us as f64) * 100.0
  };
  let size_percent = if total_stats.total_allocated_bytes_ == 0 {
    0.0
  } else {
    (stats.total_allocated_bytes_ as f64 * 100.0) / total_stats.total_allocated_bytes_ as f64
  };
  let growth = if stats.input_graph_size_ == 0 {
    0.0
  } else {
    stats.output_graph_size_ as f64 / stats.input_graph_size_ as f64
  };
  let mops_per_s = if ms <= 0.0 {
    0.0
  } else {
    (stats.output_graph_size_ as f64 / 1_000_000.0) / (ms / 1000.0)
  };

  if machine_format {
    let _ = write!(
      os,
      "\"{compiler}_{name}_time\"={ms:.3}\n\"{compiler}_{name}_space\"={}",
      stats.total_allocated_bytes_
    );
  } else {
    if stats.output_graph_size_ != 0 {
      let _ = write!(
        os,
        "{name:>34} {ms:>10.3} ({percent:>4.1}%)  {:>10} ({size_percent:>4.1}%) {:>10} {:>10}   {growth:>5.3} {mops_per_s:>6.2}",
        stats.total_allocated_bytes_,
        stats.max_allocated_bytes_,
        stats.absolute_max_allocated_bytes_,
      );
    } else {
      let _ = write!(
        os,
        "{name:>34} {ms:>10.3} ({percent:>4.1}%)  {:>10} ({size_percent:>4.1}%) {:>10} {:>10}               ",
        stats.total_allocated_bytes_,
        stats.max_allocated_bytes_,
        stats.absolute_max_allocated_bytes_,
      );
    }
    if !stats.function_name_.is_empty() {
      os.push_str("  ");
      os.push_str(&stats.function_name_);
    }
    os.push('\n');
  }
}

fn write_header(os: &mut String, compiler: &str) {
  os.push_str(DASH_LINE);
  let _ = write!(
    os,
    "{compiler:>24} phase            Time (ms)                      Space (bytes)            Growth MOps/s Function\n                                                                Total         Max.     Abs. max.\n"
  );
  os.push_str(DASH_LINE);
}

/// 原版 operator<<(os, AsPrintableStatistics)
#[must_use]
pub fn printable_statistics(ps: &AsPrintableStatistics<'_>) -> String {
  let s = ps.s;
  let mut os = String::new();

  let mut sorted_phase_kinds: Vec<(&String, &OrderedStats)> = s.phase_kind_map_.iter().collect();
  sorted_phase_kinds.sort_by_key(|(_, stats)| stats.insert_order_);

  let mut sorted_phases: Vec<(&String, &PhaseStats)> = s.phase_map_.iter().collect();
  sorted_phases.sort_by_key(|(_, stats)| stats.insert_order_);

  if !ps.machine_output {
    write_header(&mut os, ps.compiler);
  }

  for (phase_kind_name, phase_kind_stats) in sorted_phase_kinds {
    if !ps.machine_output {
      for (phase_name, phase_stats) in &sorted_phases {
        if &phase_stats.phase_kind_name_ != phase_kind_name {
          continue;
        }
        write_line(
          &mut os,
          ps.machine_output,
          phase_name,
          ps.compiler,
          &phase_stats.base,
          &s.total_stats_.base,
        );
      }
      os.push_str(PHASE_KIND_BREAK);
    }
    write_line(
      &mut os,
      ps.machine_output,
      phase_kind_name,
      ps.compiler,
      &phase_kind_stats.base,
      &s.total_stats_.base,
    );
    os.push('\n');
  }

  if !ps.machine_output {
    os.push_str(DASH_LINE);
  }
  write_line(
    &mut os,
    ps.machine_output,
    "totals",
    ps.compiler,
    &s.total_stats_.base,
    &s.total_stats_.base,
  );

  if ps.machine_output {
    os.push('\n');
    let _ = write!(
      os,
      "\"{}_totals_count\"={}",
      ps.compiler, s.total_stats_.count_
    );
  }

  os
}

impl Display for AsPrintableStatistics<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_str(&printable_statistics(self))
  }
}
