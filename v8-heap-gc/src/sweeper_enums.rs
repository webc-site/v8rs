// 由 tool/src/heap/sweeper.h.js 从 v8/src/heap/sweeper.h + incremental-marking.h + safepoint.h 生成; 修订就地改, 复跑覆盖
// 清扫/增量标记/安全点枚举

/// 空闲块处理方式 (对应 enum class FreeSpaceTreatmentMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FreeSpaceTreatmentMode {
  /// kIgnoreFreeSpace
  IgnoreFreeSpace = 0,
  /// kZapFreeSpace
  ZapFreeSpace = 1,
}

/// 清扫时机 (对应 Sweeper::SweepingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SweepingMode {
  /// kEagerDuringGC
  EagerDuringGc = 0,
  /// kLazyOrConcurrent
  LazyOrConcurrent = 1,
}

/// 清扫范围 (对应 Sweeper::SweepingScope)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SweepingScope {
  /// kMinor
  Minor = 0,
  /// kMajor
  Major = 1,
}

/// 是否含主线程 (对应 SafepointScope::IncludeMainThread)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IncludeMainThread {
  /// kYes
  Yes = 0,
  /// kNo
  No = 1,
}

/// 安全点种类 (对应 enum class SafepointKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SafepointKind {
  /// kIsolate
  Isolate = 0,
  /// kGlobal
  Global = 1,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FreeSpaceTreatmentMode::ZapFreeSpace as u8 == 1);
  assert!(SweepingMode::LazyOrConcurrent as u8 == 1);
  assert!(SweepingScope::Major as u8 == 1);
  assert!(IncludeMainThread::No as u8 == 1);
  assert!(SafepointKind::Global as u8 == 1);
};
