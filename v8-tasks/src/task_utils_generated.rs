// 由 tool/src/tasks/task-utils.h.js 从 v8/src/tasks/task-utils.{h,cc} 生成; 修订就地改, 复跑覆盖
// 可取消任务工厂: 签名骨架

/// 任务工厂签名骨架 (原版 MakeCancelableTask / MakeCancelableIdleTask ×2 重载)。
/// 语义: 包装 Isolate 或裸管理器 → 构造对应 Cancelable 任务并装入 unique_ptr。
pub mod task_utils_skel {
  use crate::cancelable_generated::{TaskId, TryAbortResult};

  /// 原版: MakeCancelableTask(Isolate*, std::function<void()>)
  pub fn make_cancelable_task_from_isolate(isolate_key: usize, run: impl FnOnce()) -> CancelableTaskBox {
    let _ = (isolate_key, run);
    unimplemented!("骨架")
  }

  /// 原版: MakeCancelableTask(CancelableTaskManager*, std::function<void()>)
  pub fn make_cancelable_task_from_manager(manager_key: usize, run: impl FnOnce()) -> CancelableTaskBox {
    let _ = (manager_key, run);
    unimplemented!("骨架")
  }

  /// 原版: MakeCancelableIdleTask(Isolate*, std::function<void(double)>)
  pub fn make_cancelable_idle_task_from_isolate(isolate_key: usize, run: impl FnOnce(f64)) -> CancelableIdleTaskBox {
    let _ = (isolate_key, run);
    unimplemented!("骨架")
  }

  /// 原版: MakeCancelableIdleTask(CancelableTaskManager*, std::function<void(double)>)
  pub fn make_cancelable_idle_task_from_manager(manager_key: usize, run: impl FnOnce(f64)) -> CancelableIdleTaskBox {
    let _ = (manager_key, run);
    unimplemented!("骨架")
  }

  /// 原版 std::unique_ptr<CancelableTask> 的占位别名
  pub struct CancelableTaskBox {
    pub manager_key: usize,
    pub task_id: TaskId,
  }

  /// 原版 std::unique_ptr<CancelableIdleTask> 的占位别名
  pub struct CancelableIdleTaskBox {
    pub manager_key: usize,
    pub task_id: TaskId,
    /// 空闲任务终结结果占位 (对齐 TryAbortResult 协议)
    pub last_abort: Option<TryAbortResult>,
  }
}
