// 由 tool/src/profiler/cpu.js 从 v8/src/profiler/cpu-profiler.h, src/profiler/cpu-profiler.cc, src/profiler/cpu-profiler-inl.h, src/profiler/tick-sample.h, src/profiler/tick-sample.cc, src/profiler/profiler-listener.h, src/profiler/profiler-listener.cc, src/profiler/profiler-stats.h, src/profiler/profiler-stats.cc, src/profiler/symbolizer.h, src/profiler/symbolizer.cc, src/profiler/circular-queue.h, src/profiler/circular-queue-inl.h, src/profiler/tracing-cpu-profiler.h, src/profiler/tracing-cpu-profiler.cc 生成; 修订就地改, 复跑覆盖
// CPU 采样面; 事件名表由 CODE_EVENTS_TYPE_LIST X-macro 展开, 对数取 clang-AST

//! CPU 采样分析器 — 原版 namespace v8::internal (cpu-profiler)
//!
//! 语义: 处理代码事件、符号化 Tick 样本并聚合统计分析。

use strum::{Display, EnumString};

/// 代码事件类型 (原版 CodeEventRecord::Type, 源码序展开)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
pub enum CodeEventType {
  /// 无事件
  NoEvent = 0,
  /// CodeCreation
  CodeCreation,
  /// CodeMove
  CodeMove,
  /// CodeDisableOpt
  CodeDisableOpt,
  /// CodeDeopt
  CodeDeopt,
  /// ReportBuiltin
  ReportBuiltin,
  /// CodeDelete
  CodeDelete,
  /// NativeContextMove
  NativeContextMove,
}

impl CodeEventType {
  /// 谓词: 是否为代码创建类事件
  #[inline]
  pub const fn is_creation(&self) -> bool {
    matches!(self, Self::CodeCreation | Self::ReportBuiltin)
  }

  /// 谓词: 是否为代码析构/移动类事件
  #[inline]
  pub const fn is_relocation_or_deletion(&self) -> bool {
    matches!(
      self,
      Self::CodeMove | Self::CodeDelete | Self::NativeContextMove
    )
  }
}

/// 是否记录 CEntryFrame (原版 enum RecordCEntryFrame 1:1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
pub enum RecordCEntryFrame {
  /// 记录 CEntryFrame
  IncludeCEntryFrame,
  /// 跳过 CEntryFrame
  SkipCEntryFrame,
}

/// 采样失败原因 (原版 enum ProfilerStats::Reason 1:1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
pub enum ProfilerStatsReason {
  /// TickBufferFull
  TickBufferFull,
  /// IsolateNotLocked
  IsolateNotLocked,
  /// SimulatorFillRegistersFailed
  SimulatorFillRegistersFailed,
  /// NoFrameRegion
  NoFrameRegion,
  /// InCallOrApply
  InCallOrApply,
  /// NoSymbolizedFrames
  NoSymbolizedFrames,
  /// NullPC
  NullPC,
}

/// 采样 Tick 样本数据 (原版 struct TickSample 1:1)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TickSample {
  /// 虚拟机当前状态 (解释/编译/GC/外部)
  pub state: u8,
  /// 程序计数器
  pub pc: usize,
  /// 栈指针
  pub sp: usize,
  /// 帧指针
  pub fp: usize,
  /// 顶部栈槽
  pub tos: usize,
  /// 栈帧地址列表 (深度最多 255)
  pub frames: Vec<usize>,
  /// 采样时间戳 (微秒)
  pub timestamp_us: u64,
  /// 是否有外部回调
  pub has_external_callback: bool,
  /// 外部回调地址
  pub external_callback_entry: usize,
}

impl TickSample {
  /// 创建新的 Tick 样本
  pub fn new(pc: usize, sp: usize, fp: usize, timestamp_us: u64) -> Self {
    Self {
      state: 0,
      pc,
      sp,
      fp,
      tos: 0,
      frames: Vec::new(),
      timestamp_us,
      has_external_callback: false,
      external_callback_entry: 0,
    }
  }

  /// 向调用栈样本追加帧
  pub fn add_frame(&mut self, frame_pc: usize) {
    if self.frames.len() < 255 {
      self.frames.push(frame_pc);
    }
  }

  /// 帧数量
  #[inline]
  pub fn frame_count(&self) -> usize {
    self.frames.len()
  }
}

/// 代码事件记录项 (原版 class CodeEventRecord 1:1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeEventRecord {
  /// 事件类型
  pub event_type: CodeEventType,
  /// 指令起始地址
  pub code_start: usize,
  /// 代码尺寸
  pub code_size: usize,
  /// 符号或函数名
  pub name: String,
}

impl CodeEventRecord {
  /// 构造新的代码事件记录
  pub fn new(
    event_type: CodeEventType,
    code_start: usize,
    code_size: usize,
    name: impl Into<String>,
  ) -> Self {
    Self {
      event_type,
      code_start,
      code_size,
      name: name.into(),
    }
  }
}

/// Profiler 采样统计分析器 (原版 class ProfilerStats 1:1)
#[derive(Debug, Clone, Default)]
pub struct ProfilerStats {
  counts: Vec<(ProfilerStatsReason, u64)>,
}

impl ProfilerStats {
  /// 创建新的统计器
  pub fn new() -> Self {
    Self { counts: Vec::new() }
  }

  /// 递增某一原因的失败计数
  pub fn record_failure(&mut self, reason: ProfilerStatsReason) {
    if let Some((_, count)) = self.counts.iter_mut().find(|(r, _)| *r == reason) {
      *count += 1;
    } else {
      self.counts.push((reason, 1));
    }
  }

  /// 获取指定原因的计数
  pub fn get_count(&self, reason: ProfilerStatsReason) -> u64 {
    self
      .counts
      .iter()
      .find(|(r, _)| *r == reason)
      .map(|(_, c)| *c)
      .unwrap_or(0)
  }
}
