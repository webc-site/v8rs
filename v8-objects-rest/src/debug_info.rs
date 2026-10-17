// 由 tool/src/objects/debug-objects.h.js 从 v8/src/objects/debug-objects.{h,-inl.h,cc,tq} 生成; 修订就地改, 复跑覆盖
// DebugInfo 调试标志位链/执行模式/副作用状态
//! DebugInfo: 函数调试信息 (trusted); 标志位链 + 执行模式 + 副作用三态。
//! 语义实现面 (DebugInfo.cc 执行/补丁逻辑) 登记跳过, 由 Developer 就地实现。

/// 原版 HasBreakInfoBit (bit 0)。
pub const HAS_BREAK_INFO_SHIFT: u32 = 0;
pub const HAS_BREAK_INFO_MASK: u32 = 0x1;
/// 原版 PreparedForDebugExecutionBit (bit 1)。
pub const PREPARED_FOR_DEBUG_EXECUTION_SHIFT: u32 = 1;
pub const PREPARED_FOR_DEBUG_EXECUTION_MASK: u32 = 0x2;
/// 原版 HasCoverageInfoBit (bit 2)。
pub const HAS_COVERAGE_INFO_SHIFT: u32 = 2;
pub const HAS_COVERAGE_INFO_MASK: u32 = 0x4;
/// 原版 BreakAtEntryBit (bit 3)。
pub const BREAK_AT_ENTRY_SHIFT: u32 = 3;
pub const BREAK_AT_ENTRY_MASK: u32 = 0x8;
/// 原版 CanBreakAtEntryBit (bit 4)。
pub const CAN_BREAK_AT_ENTRY_SHIFT: u32 = 4;
pub const CAN_BREAK_AT_ENTRY_MASK: u32 = 0x10;
/// 原版 DebugExecutionModeBit (bit 5)。
pub const DEBUG_EXECUTION_MODE_SHIFT: u32 = 5;
pub const DEBUG_EXECUTION_MODE_MASK: u32 = 0x20;

/// 标志位总数 (原版 kFlagCount)。
pub const FLAG_COUNT: u32 = 6;

/// 原版 enum Flag (各 = 对应位掩码; kNone = 0)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Flag {
  /// kNone
  None = 0,
  /// kHasBreakInfo
  Hasbreakinfo = 0x1,
  /// kPreparedForDebugExecution
  Preparedfordebugexecution = 0x2,
  /// kHasCoverageInfo
  Hascoverageinfo = 0x4,
  /// kBreakAtEntry
  Breakatentry = 0x8,
  /// kCanBreakAtEntry
  Canbreakatentry = 0x10,
  /// kDebugExecutionMode
  Debugexecutionmode = 0x20,
}

/// 调试执行模式 (原版 enum ExecutionMode; kSideEffects 复用 kDebugExecutionMode 位)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExecutionMode {
  /// kBreakpoints
  Breakpoints = 0,
  /// kSideEffects (= DebugExecutionModeBit 位值)
  SideEffects = 32,
}

/// 副作用三态 (原版 enum SideEffectState)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SideEffectState {
  /// kNotComputed
  Notcomputed = 0,
  /// kHasSideEffects
  Hassideeffects = 1,
  /// kRequiresRuntimeChecks
  Requiresruntimechecks = 2,
  /// kHasNoSideEffect
  Hasnosideeffect = 3,
}

/// 原版 debugger_hints 位段 SideEffectStateBits<i32, 0, 2>。
pub const SIDE_EFFECT_STATE_BITS_SHIFT: u32 = 0;
pub const SIDE_EFFECT_STATE_BITS_MASK: u32 = 0x3;
/// 原版 debugger_hints 位段 DebugIsBlackboxedBit<bool, 2, 1>。
pub const DEBUG_IS_BLACKBOXED_SHIFT: u32 = 2;
pub const DEBUG_IS_BLACKBOXED_MASK: u32 = 0x4;
/// 原版 debugger_hints 位段 ComputedDebugIsBlackboxedBit<bool, 3, 1>。
pub const COMPUTED_DEBUG_IS_BLACKBOXED_SHIFT: u32 = 3;
pub const COMPUTED_DEBUG_IS_BLACKBOXED_MASK: u32 = 0x8;
/// 原版 debugger_hints 位段 DebuggingIdBits<i32, 4, 20>。
pub const DEBUGGING_ID_BITS_SHIFT: u32 = 4;
pub const DEBUGGING_ID_BITS_MASK: u32 = 0xfffff0;

/// 无调试 ID (原版 kNoDebuggingId)。
pub const NO_DEBUGGING_ID: i32 = 0;
/// 预估断点数上限 (原版 kEstimatedNofBreakPointsInFunction)。
pub const ESTIMATED_NOF_BREAK_POINTS_IN_FUNCTION: i32 = 4;

// —— 提取不变式断言 ——
const _: () = {
  assert!(FLAG_COUNT == 6);
  assert!(HAS_BREAK_INFO_BIT_MASK == 0x1);
  assert!(DEBUG_EXECUTION_MODE_BIT_MASK == 0x20);
  assert!(ExecutionMode::SideEffects as u8 == 0x20);
  assert!(SideEffectState::HasNoSideEffect as i32 == 3);
  assert!(DEBUGGING_ID_BITS_MASK == 0xFFFFF0);
  assert!(ESTIMATED_NOF_BREAK_POINTS_IN_FUNCTION == 4);
};
