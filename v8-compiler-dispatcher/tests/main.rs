use core::mem::align_of;
use std::{sync::Arc, thread::spawn};

use aok::{OK, Void};
use v8_compiler_dispatcher::{
  Error, FRIEND_TESTS, JobState, LazyCompileDispatcher, LazyJob,
  MAX_OPPORTUNISTIC_FINALIZE_TIME_MS, ModeFlag, OptimizingCompileDispatcher,
  OptimizingCompileInputQueue, OptimizingCompileOutputQueue, OptimizingCompileTaskExecutor,
  OptimizingCompileTaskState, OptimizingJob,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_lazy_compile_dispatcher_job_state() -> Void {
  let states = [
    JobState::Pending,
    JobState::Running,
    JobState::AbortRequested,
    JobState::ReadyToFinalize,
    JobState::Aborted,
    JobState::PendingToRunOnForeground,
    JobState::FinalizingNow,
    JobState::AbortingNow,
    JobState::Finalized,
  ];

  assert_eq!(states.len(), 9);
  assert!(JobState::Running.is_running_on_background());
  assert!(JobState::AbortRequested.is_running_on_background());
  assert!(!JobState::Pending.is_running_on_background());

  assert!(JobState::ReadyToFinalize.is_finalizable());
  assert!(JobState::FinalizingNow.is_finalizable());
  assert!(!JobState::Aborted.is_finalizable());

  assert!(JobState::Aborted.is_aborted());
  assert!(JobState::AbortingNow.is_aborted());
  assert!(!JobState::Running.is_aborted());

  assert!(JobState::Pending.is_pending());
  assert!(JobState::PendingToRunOnForeground.is_pending());
  assert!(!JobState::Finalized.is_pending());

  // strum Display / FromStr
  let s = JobState::ReadyToFinalize.to_string();
  assert_eq!(s, "ReadyToFinalize");
  let parsed: JobState = s.parse().map_err(|e| Error::Message(format!("{e}")))?;
  assert_eq!(parsed, JobState::ReadyToFinalize);

  assert_eq!(FRIEND_TESTS.len(), 8);
  assert_eq!(MAX_OPPORTUNISTIC_FINALIZE_TIME_MS, 1);
  OK
}

#[test]
fn test_lazy_compile_dispatcher_flow() -> Void {
  let dispatcher = LazyCompileDispatcher::new(1024 * 1024);
  assert_eq!(dispatcher.max_stack_size(), 1024 * 1024);

  let job_id1 = dispatcher.enqueue(101, 500);
  let job_id2 = dispatcher.enqueue(102, 1200);

  assert_eq!(job_id1, 1);
  assert_eq!(job_id2, 2);
  assert!(dispatcher.is_enqueued(101));
  assert!(dispatcher.is_enqueued(102));
  assert!(!dispatcher.is_enqueued(999));
  assert_eq!(dispatcher.pending_count(), 2);

  // 后台模拟推进
  let processed = dispatcher.do_background_work();
  assert_eq!(processed, 2);
  assert_eq!(dispatcher.pending_count(), 0);
  assert_eq!(dispatcher.finalizable_count(), 2);

  // 空闲终结
  let finalized = dispatcher.do_idle_work(0.01);
  assert_eq!(finalized, 2);
  assert_eq!(dispatcher.finalizable_count(), 0);
  assert!(!dispatcher.is_enqueued(101));

  // 立即完成 finish_now
  let _ = dispatcher.enqueue(103, 300);
  assert!(dispatcher.is_enqueued(103));
  assert!(dispatcher.finish_now(103));
  assert!(!dispatcher.finish_now(103));

  // 中止任务 abort_job
  let _ = dispatcher.enqueue(104, 800);
  assert!(dispatcher.abort_job(104));
  assert!(!dispatcher.abort_job(104));
  assert!(!dispatcher.is_enqueued(104));

  // 中止全部 abort_all
  let _ = dispatcher.enqueue(105, 100);
  let _ = dispatcher.enqueue(106, 200);
  dispatcher.abort_all();
  assert!(!dispatcher.is_enqueued(105));
  assert!(!dispatcher.is_enqueued(106));

  let job = LazyJob::new(99, 42, 100);
  assert_eq!(job.id, 99);
  assert_eq!(job.function_id, 42);
  assert_eq!(job.state, JobState::Pending);
  assert_eq!(job.character_stream_len, 100);

  OK
}

#[test]
fn test_optimizing_compile_task_state_and_mode() -> Void {
  let state = OptimizingCompileTaskState::default();
  assert!(state.is_idle());
  assert_eq!(align_of::<OptimizingCompileTaskState>(), 64);

  let active = OptimizingCompileTaskState::new(1, 10);
  assert!(!active.is_idle());
  assert_eq!(active.isolate_id, 1);
  assert_eq!(active.job_id, 10);

  assert_eq!(ModeFlag::Compile as u8, 0);
  assert_eq!(ModeFlag::Flush as u8, 1);
  assert!(ModeFlag::Compile.is_compile());
  assert!(!ModeFlag::Compile.is_flush());
  assert!(ModeFlag::Flush.is_flush());
  assert!(!ModeFlag::Flush.is_compile());

  let parsed: ModeFlag = "Compile"
    .parse()
    .map_err(|e| Error::Message(format!("{e}")))?;
  assert_eq!(parsed, ModeFlag::Compile);

  OK
}

#[test]
fn test_optimizing_compile_queues() -> Void {
  let in_queue = OptimizingCompileInputQueue::new(2);
  assert_eq!(in_queue.capacity(), 2);
  assert!(in_queue.is_available());
  assert_eq!(in_queue.length(), 0);

  let j1 = OptimizingJob::new(1, 10, 1001, -1);
  let j2 = OptimizingJob::new(2, 10, 1002, 32);
  let j3 = OptimizingJob::new(3, 20, 2001, -1);

  assert!(in_queue.enqueue(j1));
  assert!(in_queue.enqueue(j2));
  assert!(!in_queue.enqueue(j3.clone())); // 队列已满
  assert_eq!(in_queue.length(), 2);
  assert!(!in_queue.is_available());
  assert!(in_queue.has_job_for_isolate(10));
  assert!(!in_queue.has_job_for_isolate(20));

  // 优先级调整
  in_queue.prioritize(10, 1002);
  let popped = in_queue.dequeue().ok_or(Error::JobNotFound { job_id: 2 })?;
  assert_eq!(popped.function_id, 1002);

  // 冲刷 isolate 队列
  in_queue.flush_jobs_for_isolate(10);
  assert_eq!(in_queue.length(), 0);

  // 输出队列
  let out_queue = OptimizingCompileOutputQueue::new();
  assert!(out_queue.is_empty());
  assert_eq!(out_queue.size(), 0);
  out_queue.enqueue(j3);
  assert_eq!(out_queue.size(), 1);
  assert!(out_queue.is_job_ready(2001, -1));
  assert!(!out_queue.is_job_ready(2001, 0));

  let installed = out_queue.install_generated_builtins(1);
  assert_eq!(installed, 1);
  assert!(out_queue.is_empty());

  OK
}

#[test]
fn test_optimizing_compile_executor_and_dispatcher() -> Void {
  let executor = Arc::new(OptimizingCompileTaskExecutor::new(8, 2));
  assert!(!executor.is_task_running_for_isolate(1));
  assert!(!executor.has_compilation_jobs_for_isolate(1));

  let dispatcher = OptimizingCompileDispatcher::new(1, Arc::clone(&executor));
  assert_eq!(dispatcher.isolate_id(), 1);
  assert!(dispatcher.is_queue_available());
  assert!(!dispatcher.has_jobs());
  assert!(dispatcher.finalize());

  dispatcher.set_finalize(false);
  assert!(!dispatcher.finalize());
  dispatcher.set_finalize(true);

  // 投递任务
  assert!(dispatcher.try_queue_for_optimization(501, -1));
  assert!(dispatcher.has_jobs());
  assert!(executor.has_compilation_jobs_for_isolate(1));

  // 执行器工作线程模拟提取执行
  let job = executor
    .run_compilation_job(0)
    .ok_or(Error::JobNotFound { job_id: 1 })?;
  assert_eq!(job.function_id, 501);
  assert!(executor.is_task_running_for_isolate(1));

  // 冲刷
  dispatcher.flush(ModeFlag::Flush);
  assert_eq!(dispatcher.install_optimized_functions(), 0);
  assert_eq!(dispatcher.install_generated_builtins(5), 0);
  assert!(!dispatcher.install_optimized_functions_if_ready(501, -1));

  OK
}

#[test]
fn test_error_display() -> Void {
  let err = Error::QueueFull { capacity: 16 };
  assert_eq!(format!("{err}"), "Input queue is full (capacity: 16)");
  let err = Error::JobNotFound { job_id: 42 };
  assert_eq!(format!("{err}"), "Job not found: 42");
  let err = Error::InvalidTaskIndex { index: 3 };
  assert_eq!(format!("{err}"), "Invalid task index: 3");
  let err = Error::JobAlreadyFinalized { job_id: 7 };
  assert_eq!(format!("{err}"), "Job already finalized: 7");
  let err = Error::JobAborted { job_id: 9 };
  assert_eq!(format!("{err}"), "Job aborted: 9");
  let err = Error::Message("custom error".into());
  assert_eq!(format!("{err}"), "Dispatcher error: custom error");
  OK
}

#[test]
fn test_lazy_compile_dispatcher_concurrency() -> Void {
  let dispatcher = Arc::new(LazyCompileDispatcher::new(1024 * 1024));
  let mut handles = Vec::new();

  // 生产者线程
  for t in 0..4 {
    let disp = Arc::clone(&dispatcher);
    handles.push(spawn(move || {
      for i in 0..100 {
        let fn_id = (t * 1000 + i) as u32;
        disp.enqueue(fn_id, 100);
      }
    }));
  }

  // 工作线程 (并发 do_background_work, finish_now, abort_job)
  for _ in 0..4 {
    let disp = Arc::clone(&dispatcher);
    handles.push(spawn(move || {
      for _ in 0..50 {
        disp.do_background_work();
        disp.do_idle_work(0.001);
      }
    }));
  }

  for h in handles {
    h.join().unwrap();
  }

  // 清空剩余任务
  dispatcher.do_background_work();
  dispatcher.do_idle_work(0.01);
  assert_eq!(dispatcher.pending_count(), 0);
  assert_eq!(dispatcher.finalizable_count(), 0);

  OK
}
