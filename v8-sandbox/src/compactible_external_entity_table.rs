// 由 tool/src/sandbox/compactible-external-entity-table.h.js 从 v8/src/sandbox/compactible-external-entity-table.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// CompactibleExternalEntityTable 枚举与骨架

/// 迁移时的标记位处理 (原 enum class EvacuateMarkMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EvacuateMarkMode {
  /// kTransferMark
  TransferMark = 0,
  /// kLeaveUnmarked
  LeaveUnmarked = 1,
  /// kClearMark
  ClearMark = 2,
}

/// 表压缩结果 (供 ExternalPointerTableCompactionOutcome 直方图)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ExternalEntityTableCompactionOutcome {
  /// 压缩成功
  Success = 0,
  /// 中止 (freelist 过短); 值 1 部分成功已弃用
  Aborted = 2,
}

/// 原 template <typename Entry, size_t size> class CompactibleExternalEntityTable
/// : public ExternalEntityTable (基类由 tool/src/sandbox/external-entity-table.js 覆盖)
pub const SUPPORTS_COMPACTION: bool = true;

/// 原 CompactionResult
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactionResult {
  pub start_of_evacuation_area: u32,
  pub success: bool,
}

/// 原 Space (表空间 + 压缩状态) 方法面 (11 条):
// - Space
// - StartCompactingIfNeeded
// - IsCompacting
// - CompactingWasAborted
// - FieldWasInvalidated
// - StartCompacting
// - StopCompacting
// - AbortCompacting
// - ClearInvalidatedFields
// - AddInvalidatedField
// - max

/// 模板类方法面 (17 条):
// - CompactibleExternalEntityTable
// - Space
// - StartCompactingIfNeeded
// - IsCompacting
// - CompactingWasAborted
// - FieldWasInvalidated
// - StartCompacting
// - StopCompacting
// - AbortCompacting
// - ClearInvalidatedFields
// - AddInvalidatedField
// - max
// - AllocateEntry
// - FinishCompaction
// - MaybeCreateEvacuationEntry
// - ResolveEvacuationEntryDuringSweeping
// - SweepAndCompactSegment

pub struct CompactibleExternalEntityTable;

// TODO(dev): 依 ExternalEntityTable 基类补 Mark/Sweep/Compact 实现
// (inl 侧 334 行语义面登记跳过)。

// —— 提取不变式断言 ——
const _: () = {
  assert!(ExternalEntityTableCompactionOutcome::Success as u32 == 0);
  assert!(ExternalEntityTableCompactionOutcome::Aborted as u32 == 2);
};
