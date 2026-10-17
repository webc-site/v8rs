// 由 tool/src/heap/mutable-page.h.js 从 v8/src/heap/mutable-page.h 生成; 修订就地改, 复跑覆盖
// 页级枚举: 记忆集种类 / 标记模式 / 并发清扫状态

/// 记忆集种类 (对应 enum RememberedSetType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RememberedSetType {
  /// OLD_TO_NEW
  OLD_TO_NEW = 0,
  /// OLD_TO_NEW_BACKGROUND
  OLD_TO_NEW_BACKGROUND = 1,
  /// OLD_TO_OLD
  OLD_TO_OLD = 2,
  /// OLD_TO_SHARED
  OLD_TO_SHARED = 3,
  /// TRUSTED_TO_CODE
  TRUSTED_TO_CODE = 4,
  /// TRUSTED_TO_TRUSTED
  TRUSTED_TO_TRUSTED = 5,
  /// TRUSTED_TO_SHARED_TRUSTED
  TRUSTED_TO_SHARED_TRUSTED = 6,
  /// SURVIVOR_TO_EXTERNAL_POINTER
  SURVIVOR_TO_EXTERNAL_POINTER = 7,
}

/// 种类总数 (对应 NUMBER_OF_REMEMBERED_SET_TYPES)
pub const NUMBER_OF_REMEMBERED_SET_TYPES: u8 = 8;

/// 页标记模式 (对应 enum class MarkingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MarkingMode {
  /// kNoMarking
  NO_MARKING = 0,
  /// kMinorMarking
  MINOR_MARKING = 1,
  /// kMajorMarking
  MAJOR_MARKING = 2,
}

/// 并发清扫状态 (对应 enum class ConcurrentSweepingState : intptr_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i64)]
pub enum ConcurrentSweepingState {
  /// kDone
  DONE = 0,
  /// kPendingSweeping
  PENDING_SWEEPING = 1,
  /// kPendingIteration
  PENDING_ITERATION = 2,
  /// kInProgress
  IN_PROGRESS = 3,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(RememberedSetType::OLD_TO_NEW == 0);
  assert!(RememberedSetType::OLD_TO_NEW_BACKGROUND == 1);
  assert!(RememberedSetType::OLD_TO_OLD == 2);
  assert!(RememberedSetType::OLD_TO_SHARED == 3);
  assert!(RememberedSetType::SURVIVOR_TO_EXTERNAL_POINTER == 7);
  assert!(NUMBER_OF_REMEMBERED_SET_TYPES == 8);
  assert!(MarkingMode::NO_MARKING == 0);
  assert!(MarkingMode::MAJOR_MARKING == 2);
};
