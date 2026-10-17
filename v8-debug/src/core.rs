// 由 tool/src/debug/core.js 从 v8/src/debug/debug.h, src/debug/debug.cc, src/debug/debug-interface.h, src/debug/debug-interface.cc, src/debug/interface-types.h 生成
// 调试核心组 (断点/单步/异常中断面, 调试接口优先)

use strum::{Display, EnumIter, EnumString};

/// 单步执行动作 (来源: debug.h:39, i8 底, StepNone=-1 哨兵)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
#[repr(i8)]
pub enum StepAction {
  #[strum(serialize = "StepNone", serialize = "None")]
  None = -1,
  #[strum(serialize = "StepOut", serialize = "Out")]
  Out = 0,
  #[strum(serialize = "StepOver", serialize = "Over")]
  Over = 1,
  #[strum(serialize = "StepInto", serialize = "Into")]
  Into = 2,
}

impl StepAction {
  pub const fn is_step_into(&self) -> bool {
    matches!(self, Self::Into)
  }

  pub const fn is_step_over(&self) -> bool {
    matches!(self, Self::Over)
  }

  pub const fn is_step_out(&self) -> bool {
    matches!(self, Self::Out)
  }

  pub const fn is_none(&self) -> bool {
    matches!(self, Self::None)
  }
}

/// 异常中断类型 (来源: debug.h:49)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum ExceptionBreakType {
  #[strum(serialize = "BreakCaughtException", serialize = "Caught")]
  Caught = 0,
  #[strum(serialize = "BreakUncaughtException", serialize = "Uncaught")]
  Uncaught = 1,
}

impl ExceptionBreakType {
  pub const fn is_caught(&self) -> bool {
    matches!(self, Self::Caught)
  }

  pub const fn is_uncaught(&self) -> bool {
    matches!(self, Self::Uncaught)
  }
}

/// 调试断点类型 (来源: debug.h:56, 7 变体, 顺序即谓词序)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum DebugBreakType {
  #[strum(serialize = "NOT_DEBUG_BREAK", serialize = "NotDebugBreak")]
  NotDebugBreak,
  #[strum(serialize = "DEBUG_BREAK_AT_ENTRY", serialize = "DebugBreakAtEntry")]
  DebugBreakAtEntry,
  #[strum(serialize = "DEBUGGER_STATEMENT", serialize = "DebuggerStatement")]
  DebuggerStatement,
  #[strum(serialize = "DEBUG_BREAK_SLOT", serialize = "DebugBreakSlot")]
  DebugBreakSlot,
  #[strum(
    serialize = "DEBUG_BREAK_SLOT_AT_CALL",
    serialize = "DebugBreakSlotAtCall"
  )]
  DebugBreakSlotAtCall,
  #[strum(
    serialize = "DEBUG_BREAK_SLOT_AT_RETURN",
    serialize = "DebugBreakSlotAtReturn"
  )]
  DebugBreakSlotAtReturn,
  #[strum(
    serialize = "DEBUG_BREAK_SLOT_AT_SUSPEND",
    serialize = "DebugBreakSlotAtSuspend"
  )]
  DebugBreakSlotAtSuspend,
}

impl DebugBreakType {
  pub const fn is_debugger_statement(&self) -> bool {
    matches!(self, Self::DebuggerStatement)
  }

  pub const fn is_break_slot(&self) -> bool {
    matches!(
      self,
      Self::DebugBreakSlot
        | Self::DebugBreakSlotAtCall
        | Self::DebugBreakSlotAtReturn
        | Self::DebugBreakSlotAtSuspend
    )
  }
}

/// 中断原因枚举 (来源: debug-interface.h:135, u8 底, 9 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
#[repr(u8)]
pub enum BreakReason {
  #[strum(serialize = "kAlreadyPaused", serialize = "AlreadyPaused")]
  AlreadyPaused,
  #[strum(serialize = "kStep", serialize = "Step")]
  Step,
  #[strum(serialize = "kAsyncStep", serialize = "AsyncStep")]
  AsyncStep,
  #[strum(serialize = "kException", serialize = "Exception")]
  Exception,
  #[strum(serialize = "kAssert", serialize = "Assert")]
  Assert,
  #[strum(serialize = "kDebuggerStatement", serialize = "DebuggerStatement")]
  DebuggerStatement,
  #[strum(serialize = "kOOM", serialize = "OOM")]
  OOM,
  #[strum(serialize = "kScheduled", serialize = "Scheduled")]
  Scheduled,
  #[strum(serialize = "kAgent", serialize = "Agent")]
  Agent,
}

impl BreakReason {
  pub const fn is_step(&self) -> bool {
    matches!(self, Self::Step | Self::AsyncStep)
  }

  pub const fn is_exception(&self) -> bool {
    matches!(self, Self::Exception)
  }

  pub const fn is_debugger_statement(&self) -> bool {
    matches!(self, Self::DebuggerStatement)
  }
}

/// 覆盖率收集模式 (来源: interface-types.h:66)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum CoverageMode {
  #[strum(serialize = "kBestEffort", serialize = "BestEffort")]
  BestEffort,
  #[strum(serialize = "kPreciseCount", serialize = "PreciseCount")]
  PreciseCount,
  #[strum(serialize = "kPreciseBinary", serialize = "PreciseBinary")]
  PreciseBinary,
  #[strum(serialize = "kBlockPrecise", serialize = "BlockPrecise")]
  BlockPrecise,
}

/// 异常中断状态机模式 (来源: debug-interface.h:125)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
#[repr(u8)]
pub enum ExceptionBreakState {
  #[strum(serialize = "NoBreak")]
  NoBreak = 0,
  #[strum(serialize = "BreakOnCaughtException")]
  BreakOnCaughtException = 1,
  #[strum(serialize = "BreakOnUncaughtException")]
  BreakOnUncaughtException = 2,
  #[strum(serialize = "BreakOnAnyException")]
  BreakOnAnyException = 3,
}

impl ExceptionBreakState {
  pub const fn is_break_on_caught(&self) -> bool {
    matches!(
      self,
      Self::BreakOnCaughtException | Self::BreakOnAnyException
    )
  }

  pub const fn is_break_on_uncaught(&self) -> bool {
    matches!(
      self,
      Self::BreakOnUncaughtException | Self::BreakOnAnyException
    )
  }

  pub const fn is_break_on_any(&self) -> bool {
    matches!(self, Self::BreakOnAnyException)
  }
}

/// 私有成员过滤器位标志 (来源: debug-interface.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
#[repr(u8)]
pub enum PrivateMemberFilter {
  #[strum(serialize = "kPrivateMethods")]
  PrivateMethods = 1,
  #[strum(serialize = "kPrivateFields")]
  PrivateFields = 2,
  #[strum(serialize = "kPrivateAccessors")]
  PrivateAccessors = 4,
}

/// 原生访问器类型标志 (来源: debug-interface.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
#[repr(u8)]
pub enum NativeAccessorType {
  None = 0,
  HasGetter = 1,
  HasSetter = 2,
  IsValueUnavailable = 4,
}

/// 全局求值模式 (来源: debug-interface.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum EvaluateGlobalMode {
  #[strum(serialize = "kDefault")]
  Default,
  #[strum(serialize = "kDisableBreaks")]
  DisableBreaks,
  #[strum(serialize = "kDisableBreaksAndThrowOnSideEffect")]
  DisableBreaksAndThrowOnSideEffect,
}

/// 源码偏移检索模式 (来源: debug-interface.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum GetSourceOffsetMode {
  #[strum(serialize = "kStrict")]
  Strict,
  #[strum(serialize = "kClamp")]
  Clamp,
}

/// 断点位置元数据结构体 (来源: debug.h:71)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BreakLocation {
  pub position: i32,
  pub break_type: DebugBreakType,
}

impl BreakLocation {
  pub const fn new(position: i32, break_type: DebugBreakType) -> Self {
    Self {
      position,
      break_type,
    }
  }

  pub const fn invalid() -> Self {
    Self {
      position: -1,
      break_type: DebugBreakType::NotDebugBreak,
    }
  }

  pub const fn is_valid(&self) -> bool {
    self.position >= 0 && !matches!(self.break_type, DebugBreakType::NotDebugBreak)
  }

  /// v8: src/debug/debug.h:82 `bool IsSuspend() const`
  pub const fn is_suspend(&self) -> bool {
    matches!(self.break_type, DebugBreakType::DebugBreakSlotAtSuspend)
  }

  /// v8: src/debug/debug.h:83 `bool IsReturn() const`
  pub const fn is_return(&self) -> bool {
    matches!(self.break_type, DebugBreakType::DebugBreakSlotAtReturn)
  }

  /// v8: src/debug/debug.h:84 `bool IsReturnOrSuspend() const`
  pub const fn is_return_or_suspend(&self) -> bool {
    matches!(
      self.break_type,
      DebugBreakType::DebugBreakSlotAtReturn | DebugBreakType::DebugBreakSlotAtSuspend
    )
  }

  /// v8: src/debug/debug.h:85 `bool IsCall() const`
  pub const fn is_call(&self) -> bool {
    matches!(self.break_type, DebugBreakType::DebugBreakSlotAtCall)
  }

  /// v8: src/debug/debug.h:86 `bool IsDebugBreakSlot() const`
  pub const fn is_debug_break_slot(&self) -> bool {
    self.break_type.is_break_slot()
  }

  /// v8: src/debug/debug.h:87 `bool IsDebuggerStatement() const`
  pub const fn is_debugger_statement(&self) -> bool {
    self.break_type.is_debugger_statement()
  }

  /// v8: src/debug/debug.h:88 `bool IsDebugBreakAtEntry() const`
  pub const fn is_debug_break_at_entry(&self) -> bool {
    matches!(self.break_type, DebugBreakType::DebugBreakAtEntry)
  }
}

/// 禁用断点守卫抽象 (来源: debug.h:487)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DisableBreak {
  pub previous_state: bool,
}

impl DisableBreak {
  pub const fn new(previous_state: bool) -> Self {
    Self { previous_state }
  }
}

/// 调试范围生命周期守卫 (来源: debug.h:512)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DebugScope {
  pub active: bool,
}

impl DebugScope {
  pub const fn new() -> Self {
    Self { active: true }
  }

  pub fn close(&mut self) {
    self.active = false;
  }
}

impl Default for DebugScope {
  fn default() -> Self {
    Self::new()
  }
}
