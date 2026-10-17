#![cfg_attr(docsrs, feature(doc_cfg))]

mod api;
mod error;
pub use error::{Error, Result};

// 模块面按 AGENTS.md 收敛为私有（W3-6b 裁定=收紧，无任何模块对外暴露豁免项）：
// tests/ 与仓外消费者均走 crate 根扁平导出形；job/worker_thread/worker_threads_task_runner/
// tracing 为 LP-B/LP-C 占位（原版对应类在 src/libplatform 的 v8::platform 命名空间，
// 非 include/v8-platform.h 公共面），暂无可导出实体，落地时再按本形追加 pub use。
mod delayed_task_queue;
mod foreground_task_runner;
mod job;
mod platform;
mod task_queue;
mod tracing;
mod worker_thread;
mod worker_threads_task_runner;

// LP-A 族导出（api 面 + 各模块主体）。
pub use api::{
  BoxIdleTask, BoxTask, IdleTaskSupport, InProcessStackDumping, IsolateKey, MessageLoopBehavior,
  Platform, PriorityMode, SourceLocation, TaskRunner, TimeFunction, TracingController,
  WorkerRunnerFactory, WorkerTaskRunner,
};
pub use delayed_task_queue::{DelayedTaskQueue, MaybeNextTask};
pub use foreground_task_runner::{DefaultForegroundTaskRunner, RunTaskScope};
pub use platform::{
  DefaultPlatform, K_MAX_THREAD_POOL_SIZE, PlatformOptions, new_default_platform,
  new_single_threaded_default_platform,
};
pub use task_queue::TaskQueue;
