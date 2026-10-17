//! v8-tasks：V8 `src/tasks` 的 Rust 转写（cancelable-task / operations-barrier /
//! task-utils），接口语义来源为 `v8/include/v8-platform.h` 的 Task/IdleTask。
//!
//! 范围偏差与推迟项（详见 readme/zh.md）：
//! - `Isolate*` 便捷构造依赖执行层，推迟；
//! - `TaskQueue`/`Worker` 属 `v8/src/libplatform`（v8-libplatform crate）；
//! - 本版 V8 已无 `TaskSupplier` work-stealing 协议，不适用。

#![cfg_attr(docsrs, feature(doc_cfg))]

mod cancelable;
mod error;
mod operations_barrier;
mod task;
mod task_utils;

pub use cancelable::{
  Cancelable, CancelableIdleTask, CancelableTask, CancelableTaskManager, INVALID_TASK_ID,
  IdleTaskBody, Status, TaskBody, TaskId, TryAbortResult,
};
pub use error::{Error, Result};
pub use operations_barrier::{BarrierToken, OperationsBarrier};
pub use task::{IdleTask, MAX_PRIORITY, Task, TaskPriority};
pub use task_utils::{make_cancelable_idle_task, make_cancelable_task};
