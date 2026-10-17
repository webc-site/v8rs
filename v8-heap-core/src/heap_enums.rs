// 由 tool/src/heap/heap.h.js 从 v8/src/heap/heap.{h,cc} 生成; 修订就地改, 复跑覆盖
// Heap 顶层枚举与名称表 (Heap 类本体为语义实现, 不在本产物)

/// 根跳过集合 (对应 enum class SkipRoot)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SkipRoot {
  /// kExternalStringTable
  EXTERNAL_STRING_TABLE = 0,
  /// kGlobalHandles
  GLOBAL_HANDLES = 1,
  /// kTracedHandles
  TRACED_HANDLES = 2,
  /// kOldGeneration
  OLD_GENERATION = 3,
  /// kStack
  STACK = 4,
  /// kMainThreadHandles
  MAIN_THREAD_HANDLES = 5,
  /// kUnserializable
  UNSERIALIZABLE = 6,
  /// kWeak
  WEAK = 7,
  /// kConservativeStack
  CONSERVATIVE_STACK = 8,
  /// kReadOnlyBuiltins
  READ_ONLY_BUILTINS = 9,
}

/// GC 触发标志位 (对应 enum class GCFlag : uint8_t, 位标志)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum GCFlag {
  #[default]
  /// kNoFlags = 0
  NO_FLAGS = 0,
  /// kReduceMemoryFootprint = 1 << 0
  REDUCE_MEMORY_FOOTPRINT = 1 << 0,
  /// kForced = 1 << 1 (测试 --expose-gc 或 DevTools LowMemoryNotification 强制 GC)
  FORCED = 1 << 1,
  /// kLastResort = 1 << 2
  LAST_RESORT = 1 << 2,
}

/// GCFlag 组合位集 (对应 using GCFlags = base::Flags<GCFlag, uint8_t>)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GCFlags(pub u8);

impl GCFlags {
  /// 对应 base::Flags 的位或
  #[must_use]
  pub const fn contains(self, flag: GCFlag) -> bool {
    (self.0 & flag as u8) == flag as u8
  }

  /// 并入一个标志
  pub fn add(&mut self, flag: GCFlag) {
    self.0 |= flag as u8;
  }
}

/// 完成清扫的原因 (对应 enum class CompleteSweepingReason)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CompleteSweepingReason {
  /// kCollectCodeStatistics
  COLLECT_CODE_STATISTICS = 0,
  /// kHeapObjectIterator
  HEAP_OBJECT_ITERATOR = 1,
  /// kStartMinorMarking
  START_MINOR_MARKING = 2,
  /// kStartMajorMarking
  START_MAJOR_MARKING = 3,
  /// kMinorGC
  MINOR_GC = 4,
  /// kMajorGC
  MAJOR_GC = 5,
  /// kHeapSnapshot
  HEAP_SNAPSHOT = 6,
  /// kTearDown
  TEAR_DOWN = 7,
  /// kFreeze
  FREEZE = 8,
  /// kTesting
  TESTING = 9,
  /// kReadOnly
  READ_ONLY = 10,
}

impl CompleteSweepingReason {
  /// 名称表 (对应 ToString(CompleteSweepingReason))
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
      Self::COLLECT_CODE_STATISTICS => "collect code statistics",
      Self::HEAP_OBJECT_ITERATOR => "heap object iterator",
      Self::START_MAJOR_MARKING => "start major marking",
      Self::START_MINOR_MARKING => "start minor marking",
      Self::MINOR_GC => "minor gc",
      Self::MAJOR_GC => "major gc",
      Self::HEAP_SNAPSHOT => "heap snapshot",
      Self::TEAR_DOWN => "tear down",
      Self::FREEZE => "freeze",
      Self::TESTING => "testing",
      Self::READ_ONLY => "read only",
    }
  }
}

/// Heap 主状态 (对应 enum HeapState)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HeapState {
  /// NOT_IN_GC
  NOT_IN_GC = 0,
  /// SCAVENGE
  SCAVENGE = 1,
  /// MARK_COMPACT
  MARK_COMPACT = 2,
  /// MINOR_MARK_SWEEP
  MINOR_MARK_SWEEP = 3,
  /// TEAR_DOWN
  TEAR_DOWN = 4,
}

/// 槽清除方式 (对应 enum class SlotClearingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SlotClearingMode {
  /// kCheckSweeping
  CHECK_SWEEPING = 0,
  /// kUnconditional
  UNCONDITIONAL = 1,
}

/// Embedder 栈状态来源 (对应 enum class EmbedderStackStateOrigin)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EmbedderStackStateOrigin {
  /// kImplicitThroughTask
  IMPLICIT_THROUGH_TASK = 0,
  /// kExplicitInvocation
  EXPLICIT_INVOCATION = 1,
}

/// 离开堆时的状态 (对应 enum class LeaveHeapState)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LeaveHeapState {
  /// kNotify
  NOTIFY = 0,
  /// kReachedTimeout
  REACHED_TIMEOUT = 1,
}

/// 栈扫描方式 (对应 enum class StackScanMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StackScanMode {
  /// kNone
  NONE = 0,
  /// kFull
  FULL = 1,
  /// kSelective
  SELECTIVE = 2,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(SkipRoot::EXTERNAL_STRING_TABLE == 0);
  assert!(SkipRoot::READ_ONLY_BUILTINS == 9);
  assert!(GCFlag::REDUCE_MEMORY_FOOTPRINT as u8 == 1);
  assert!(GCFlag::FORCED as u8 == 2);
  assert!(GCFlag::LAST_RESORT as u8 == 4);
  assert!(CompleteSweepingReason::COLLECT_CODE_STATISTICS == 0);
  assert!(CompleteSweepingReason::READ_ONLY == 10);
  assert!(CompleteSweepingReason::MINOR_GC.to_str() == "minor gc");
  assert!(HeapState::NOT_IN_GC == 0);
  assert!(HeapState::TEAR_DOWN == 4);
  assert!(StackScanMode::SELECTIVE == 2);
};
