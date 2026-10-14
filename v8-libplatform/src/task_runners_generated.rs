// 由 tool/src/libplatform/default-worker-threads-task-runner.cc.js 从 v8/src/libplatform/default-worker-threads-task-runner.{h,cc} + delayed-task-queue.{h,cc} + default-foreground-task-runner.{h,cc} + default-job.{h,cc} 生成; 修订就地改, 复跑覆盖
// 任务运行器族骨架
// R5L(LP-A) 就地修订: delayed-task-queue / default-foreground-task-runner 两族语义面已落地
//   src/delayed_task_queue.rs / src/foreground_task_runner.rs; DWTR 与 default-job 仍属 LP-B 推迟。
//   原版实测更正: DelayedTaskQueue 当前实现为 multimap<deadline秒(double)> 而非 deadline_ticks 堆。

/// Worker 线程运行器方法面 (原版 default-worker-threads-task-runner.h)
pub const WORKER_RUNNER_METHODS: [&str; 7] = ["Terminate", "MonotonicallyIncreasingTime", "IdleTasksEnabled", "PostTaskImpl", "PostDelayedTaskImpl", "PostIdleTaskImpl", "GetNext"];

/// 前台运行器嵌套控制标志 (原版 default-foreground-task-runner.h)
pub const FOREGROUND_NESTING_FLAGS: [&str; 2] = ["nesting_depth_", "task_queue_"];

pub mod runner_skel {
  /// 延迟任务队列 (原版 DelayedTaskQueue: multimap 按到期时刻排序, 自身无锁、由调用方保护)
  pub fn delayed_task_queue_skel() {
    unimplemented!("已迁移: delayed_task_queue::DelayedTaskQueue")
  }
  /// 任务协作分派 (原版 DefaultJobState/DefaultJobHandle: 加入/退出/等待)
  pub fn job_state_skel() {
    unimplemented!("骨架")
  }
  /// 消息泵嵌套协议 (原版 foreground runner: PostTask 时 NotifyOne 唤醒已嵌套泵)
  pub fn message_pump_nesting_skel() {
    unimplemented!("已迁移: foreground_task_runner::DefaultForegroundTaskRunner + RunTaskScope")
  }
}
