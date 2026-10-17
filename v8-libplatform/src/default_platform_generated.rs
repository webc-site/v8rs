// 由 tool/src/libplatform/default-platform.cc.js 从 v8/src/libplatform/default-platform.{h,cc} + worker-thread.{h,cc} + task-queue.{h,cc} + default-thread-isolated-allocator.{h,cc} + etw/etw-provider-win.h 生成; 修订就地改, 复跑覆盖
// 默认平台族骨架
// R5L(LP-A) 就地修订: 本族语义面已落地 src/api.rs + src/platform.rs (DefaultPlatform/泵/前台管理)
//   + src/task_queue.rs + src/delayed_task_queue.rs + src/foreground_task_runner.rs;
//   本文件保留为脚本对照面 (未挂 lib.rs, 不参与编译)。
//   原版实测更正: 当前 task-queue.{h,cc} 为 FIFO+信号量 (base::Semaphore), 非优先级/generation 队列
//   (席位工单描述对应上游旧版; CancelAll/CancelDelayed 面在 v8-tasks CancelableTaskManager)。

/// DefaultPlatform 方法面 (原版 default-platform.h, 13 项, 顺序登记)
pub const PLATFORM_METHODS: [&str; 13] = ["EnsureBackgroundTaskRunnerInitialized", "RunIdleTasks", "SetTracingController", "SetTimeFunctionForTesting", "NumberOfWorkerThreads", "PostTaskOnWorkerThreadImpl", "PostDelayedTaskOnWorkerThreadImpl", "CreateJobImpl", "MonotonicallyIncreasingTime", "CurrentClockTimeMillis", "NotifyIsolateShutdown", "priority_to_index", "num_worker_runners"];

/// 支撑组件锚点
pub const PLATFORM_SUPPORTS: [&str; 4] = ["PageAllocator", "MonotonicallyIncreasingTime", "TracingController", "WorkerThreadsTaskRunner"];

pub mod platform_skel {
  /// 工作线程池上限 (原版 kMaxThreadPoolSize)
  pub const MAX_THREAD_POOL_SIZE: usize = 16;

  /// 工作线程池: 每线程一个 WorkerThread 消费 TaskQueue (LP-B 族, 仍推迟)
  pub fn worker_pool_skel(_threads: usize) {
    unimplemented!("骨架")
  }
  /// 任务队列终止协议 (原版 TaskQueue: 终止哨兵 + 信号量接力; 已实现于 src/task_queue.rs)
  pub fn task_queue_terminate_skel() {
    unimplemented!("已迁移: task_queue::TaskQueue::terminate/get_next")
  }
  /// 线程隔离分配器 (原版 DefaultThreadIsolatedAllocator; PKEY 隔离内存)
  pub fn thread_isolated_allocator_skel() {
    unimplemented!("骨架")
  }
}

/// 平台分支登记: ETW 提供者 (原版 etw/etw-provider-win.h) — 仅 Windows, 当前目标跳过
pub const ETW_PROVIDER: &str = "windows-only; X_SDK ETW TraceLogging, 本目标跳过";
