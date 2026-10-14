//! 对应 `src/tasks/task-utils.{h,cc}`：闭包包装成可取消任务。
//! 原版 `Isolate*` 重载依赖执行层（未转写），推迟项登记 readme。

use crate::{
  cancelable::{CancelableIdleTask, CancelableTask, CancelableTaskManager, IdleTaskBody, TaskBody},
  error::Result,
};

struct FuncTask(Box<dyn FnMut() + Send>);

impl TaskBody for FuncTask {
  fn run_internal(&mut self) {
    (self.0)();
  }
}

struct IdleFuncTask(Box<dyn FnMut(f64) + Send>);

impl IdleTaskBody for IdleFuncTask {
  fn run_internal(&mut self, deadline_in_seconds: f64) {
    (self.0)(deadline_in_seconds);
  }
}

/// 对应原 `MakeCancelableTask(CancelableTaskManager*, std::function<void()>)`。
pub fn make_cancelable_task(
  manager: &CancelableTaskManager,
  func: impl FnMut() + Send + 'static,
) -> Result<CancelableTask> {
  CancelableTask::new(manager, FuncTask(Box::new(func)))
}

/// 对应原 `MakeCancelableIdleTask(CancelableTaskManager*, std::function<void(double)>)`。
pub fn make_cancelable_idle_task(
  manager: &CancelableTaskManager,
  func: impl FnMut(f64) + Send + 'static,
) -> Result<CancelableIdleTask> {
  CancelableIdleTask::new(manager, IdleFuncTask(Box::new(func)))
}
