//! v8-libplatform 错误集中定义（对应原版各 DCHECK/CHECK 中会中断流程的分支）。

use std::result;

use thiserror::Error;
use v8_tasks::TaskPriority;

#[derive(Error, Debug)]
pub enum Error {
  /// 工作线程任务运行器不可用：LP-B `worker-threads-task-runner` 族尚未接入
  /// （原版 `DCHECK_NOT_NULL(worker_threads_task_runners_[index])`，Rust 侧以错误承接）。
  #[error("worker task runner unavailable for priority {priority:?} (LP-B 族未接入)")]
  WorkerRunnerUnavailable { priority: TaskPriority },
}

pub type Result<T> = result::Result<T, Error>;
