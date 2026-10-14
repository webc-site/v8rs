// 由 tool/src/heap/mark-compact.h.js 从 v8/src/heap/mark-compact.h 生成; 修订就地改, 复跑覆盖
// MarkCompact 内部枚举 (CollectorState 原版仅 DEBUG)

/// 压实启动方式 (对应 enum class StartCompactionMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StartCompactionMode {
  /// kIncremental
  Incremental = 0,
  /// kAtomic
  Atomic = 1,
}

/// 标记工作表处理方式 (对应 MarkingWorklistProcessingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MarkingWorklistProcessingMode {
  /// kDefault
  Default = 0,
  /// kProcessRememberedEphemerons
  ProcessRememberedEphemerons = 1,
}

/// 调用来源 (对应 CallOrigin)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CallOrigin {
  /// kIncrementalMarkingStep
  IncrementalMarkingStep = 0,
  /// kAtomicGC
  AtomicGc = 1,
}

/// Ephemeron 处理结果 (对应 EphemeronResult)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EphemeronResult {
  /// kUnresolved
  Unresolved = 0,
  /// kMarkedValue
  MarkedValue = 1,
  /// kResolved
  Resolved = 2,
}

/// 收集器阶段 (对应 enum CollectorState, 原版 #ifdef DEBUG)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CollectorState {
  /// IDLE
  Idle = 0,
  /// PrepareGc
  PrepareGc = 1,
  /// MarkLiveObjects
  MarkLiveObjects = 2,
  /// SweepSpaces
  SweepSpaces = 3,
  /// EncodeForwardingAddresses
  EncodeForwardingAddresses = 4,
  /// UpdatePointers
  UpdatePointers = 5,
  /// RelocateObjects
  RelocateObjects = 6,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(StartCompactionMode::Incremental as u8 == 0);
  assert!(StartCompactionMode::Atomic as u8 == 1);
  assert!(MarkingWorklistProcessingMode::Default as u8 == 0);
  assert!(MarkingWorklistProcessingMode::ProcessRememberedEphemerons as u8 == 1);
  assert!(CallOrigin::AtomicGc as u8 == 1);
  assert!(EphemeronResult::Unresolved as u8 == 0);
  assert!(EphemeronResult::Resolved as u8 == 2);
  assert!(CollectorState::Idle as u8 == 0);
  assert!(CollectorState::RelocateObjects as u8 == 6);
};
