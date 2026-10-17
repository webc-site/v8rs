// 由 tool/src/codegen/source-position-table.h.js 从 v8/src/codegen/source-position-table.{h,cc} 生成; 修订就地改, 复跑覆盖
// 源位置表枚举与骨架

/// 位置记录模式 (原 enum RecordingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RecordingMode {
  /// 永不生成 (空表)
  OmitSourcePositions = 0,
  /// 暂不需要, 可后补
  LazySourcePositions = 1,
  /// 立即生成
  RecordSourcePositions = 2,
}

/// 迭代过滤器 (原 enum IterationFilter)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IterationFilter {
  JavaScriptOnly = 0,
  ExternalOnly = 1,
  All = 2,
}

/// 首条目过滤器 (原 enum FunctionEntryFilter)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FunctionEntryFilter {
  SkipFunctionEntry = 0,
  DontSkipFunctionEntry = 1,
}

/// 迭代终止哨兵 (原 kDone)
pub const DONE: i32 = -1;

/// 原 SourcePositionTableBuilder / SourcePositionTableIterator 骨架
/// - 原签名: SourcePositionTableBuilder(Zone*, RecordingMode = RECORD_SOURCE_POSITIONS)
/// - 原签名: void AddPosition(code_offset, source_position, is_statement, is_breakable = true)
/// - 原签名: bool Omit() const / bool Lazy() const
/// - 原签名: IndexAndPositionState (迭代器保存/恢复)
pub struct SourcePositionTableBuilder;

// TODO(dev): 补 delta 编码字节流实现 (cc 语义面登记跳过)。

// —— 提取不变式断言 ——
const _: () = {
  assert!(DONE == -1);
  assert!(IterationFilter::All as u8 == 2);
};
