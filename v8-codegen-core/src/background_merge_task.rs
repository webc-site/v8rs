// 由 tool/src/codegen/background-merge-task.h.js 从 v8/src/codegen/background-merge-task.h 生成; 修订就地改, 复跑覆盖
// 后台合并任务骨架

/// 任务状态机 (原 enum State)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
  /// kNotStarted
  NotStarted = 0,
  /// kPendingBackgroundWork
  PendingBackgroundWork = 1,
  /// kPendingForegroundWork
  PendingForegroundWork = 2,
  /// kDone
  Done = 3,
}

/// 原 class BackgroundMergeTask — 后台合并 cached SFI 编译数据
/// 数据队列: new_compiled_data_for_cached_sfis_ (NewCompiledDataForCachedSfi:
///   cached_sfi / new_sfi 双句柄) + sfis_without_scope_info_ 集合
pub struct BackgroundMergeTask;

// TODO(dev): 补状态迁移与合并实现。

// —— 提取不变式断言 ——
const _: () = {
  assert!(State::NotStarted as u8 == 0);
};
