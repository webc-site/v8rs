// 由 tool/src/compiler-dispatcher/lazy-compile-dispatcher.h.js 从 v8/src/compiler-dispatcher/lazy-compile-dispatcher.{h,cc} 生成; 修订就地改, 复跑覆盖
// LazyCompileDispatcher: Job 状态机 + FRIEND_TEST 台账 + 签名骨架

//! 惰性编译调度器 — 原版 namespace v8::internal::LazyCompileDispatcher
//!
//! 语义: 空闲任务 + 后台任务组合推进惰性解析函数的 parse/compile;
//! 三队列协作: jobs_ 全集 / pending_background_jobs_ 可后台 / finalizable_jobs_ 待终结。

use core::sync::atomic::{AtomicU64, Ordering};

use parking_lot::Mutex;

/// Job 句柄类型 (原版 using JobId = uintptr_t)
pub type JobId = u64;

/// 单个机会式 finalize 的时长上限, 毫秒 (原版 kMaxOpportunisticFinalizeTimeMs, cc:35)
pub const MAX_OPPORTUNISTIC_FINALIZE_TIME_MS: i32 = 1;

/// Job 状态机 (原版 LazyCompileDispatcher::Job::State, 9 态 1:1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum JobState {
  /// 原版 kPending — 在后台待命队列
  Pending,
  /// 原版 kRunning — 后台执行中
  Running,
  /// 原版 kAbortRequested — 请求中止 (丢弃结果)
  AbortRequested,
  /// 原版 kReadyToFinalize — 在终结队列
  ReadyToFinalize,
  /// 原版 kAborted — 已中止
  Aborted,
  /// 原版 kPendingToRunOnForeground — 已弹出待命队列, 主线程待跑
  PendingToRunOnForeground,
  /// 原版 kFinalizingNow — 已弹出终结队列, 正终结
  FinalizingNow,
  /// 原版 kAbortingNow — 终结中且需中止
  AbortingNow,
  /// 原版 kFinalized — 终结完毕, 待删除
  Finalized,
}

impl JobState {
  /// 原版 Job::is_running_on_background() — kRunning 或 kAbortRequested 视为后台在跑
  #[inline]
  #[must_use]
  pub const fn is_running_on_background(self) -> bool {
    matches!(self, Self::Running | Self::AbortRequested)
  }

  /// 是否待终结
  #[inline]
  #[must_use]
  pub const fn is_finalizable(self) -> bool {
    matches!(self, Self::ReadyToFinalize | Self::FinalizingNow)
  }

  /// 是否已中止
  #[inline]
  #[must_use]
  pub const fn is_aborted(self) -> bool {
    matches!(self, Self::Aborted | Self::AbortingNow)
  }

  /// 是否在等待队列
  #[inline]
  #[must_use]
  pub const fn is_pending(self) -> bool {
    matches!(self, Self::Pending | Self::PendingToRunOnForeground)
  }
}

/// 惰性编译作业结构体
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyJob {
  pub id: JobId,
  pub function_id: u32,
  pub state: JobState,
  pub character_stream_len: usize,
}

impl LazyJob {
  #[inline]
  #[must_use]
  pub const fn new(id: JobId, function_id: u32, character_stream_len: usize) -> Self {
    Self {
      id,
      function_id,
      state: JobState::Pending,
      character_stream_len,
    }
  }
}

#[derive(Debug, Default)]
struct DispatcherState {
  jobs: Vec<LazyJob>,
  pending_background_jobs: Vec<JobId>,
  finalizable_jobs: Vec<JobId>,
}

/// 惰性编译调度器
#[derive(Debug, Default)]
pub struct LazyCompileDispatcher {
  max_stack_size: usize,
  next_job_id: AtomicU64,
  state: Mutex<DispatcherState>,
}

impl LazyCompileDispatcher {
  #[inline]
  #[must_use]
  pub fn new(max_stack_size: usize) -> Self {
    Self {
      max_stack_size,
      next_job_id: AtomicU64::new(1),
      state: Mutex::new(DispatcherState::default()),
    }
  }

  #[inline]
  #[must_use]
  pub const fn max_stack_size(&self) -> usize {
    self.max_stack_size
  }

  /// 投递惰性解析任务
  pub fn enqueue(&self, function_id: u32, character_stream_len: usize) -> JobId {
    let id = self.next_job_id.fetch_add(1, Ordering::Relaxed);

    let mut state = self.state.lock();
    state
      .jobs
      .push(LazyJob::new(id, function_id, character_stream_len));
    state.pending_background_jobs.push(id);

    id
  }

  /// 检查某函数是否在队列中
  pub fn is_enqueued(&self, function_id: u32) -> bool {
    let state = self.state.lock();
    state.jobs.iter().any(|j| {
      j.function_id == function_id && !j.state.is_aborted() && j.state != JobState::Finalized
    })
  }

  /// 阻塞并立即在主线程完成编译
  pub fn finish_now(&self, function_id: u32) -> bool {
    let mut state = self.state.lock();
    if let Some(job) = state.jobs.iter_mut().find(|j| {
      j.function_id == function_id && !j.state.is_aborted() && j.state != JobState::Finalized
    }) {
      let id = job.id;
      job.state = JobState::Finalized;
      state.pending_background_jobs.retain(|&i| i != id);
      state.finalizable_jobs.retain(|&i| i != id);
      true
    } else {
      false
    }
  }

  /// 中止某函数的编译任务
  pub fn abort_job(&self, function_id: u32) -> bool {
    let mut state = self.state.lock();
    if let Some(job) = state.jobs.iter_mut().find(|j| {
      j.function_id == function_id && !j.state.is_aborted() && j.state != JobState::Finalized
    }) {
      let id = job.id;
      job.state = JobState::Aborted;
      state.pending_background_jobs.retain(|&i| i != id);
      state.finalizable_jobs.retain(|&i| i != id);
      true
    } else {
      false
    }
  }

  /// 中止全部任务
  pub fn abort_all(&self) {
    let mut state = self.state.lock();
    for job in state.jobs.iter_mut() {
      if !job.state.is_aborted() && job.state != JobState::Finalized {
        job.state = JobState::Aborted;
      }
    }
    state.pending_background_jobs.clear();
    state.finalizable_jobs.clear();
  }

  /// 模拟后台工作线程推进任务
  pub fn do_background_work(&self) -> usize {
    let mut state = self.state.lock();
    let mut processed = 0;
    while let Some(id) = state.pending_background_jobs.pop() {
      if let Some(job) = state
        .jobs
        .iter_mut()
        .find(|j| j.id == id && j.state == JobState::Pending)
      {
        job.state = JobState::ReadyToFinalize;
        state.finalizable_jobs.push(id);
        processed += 1;
      }
    }
    processed
  }

  /// 空闲时进行机会式终结
  pub fn do_idle_work(&self, _deadline_in_seconds: f64) -> usize {
    let mut state = self.state.lock();
    let mut finished = 0;
    while let Some(id) = state.finalizable_jobs.pop() {
      if let Some(job) = state
        .jobs
        .iter_mut()
        .find(|j| j.id == id && j.state == JobState::ReadyToFinalize)
      {
        job.state = JobState::Finalized;
        finished += 1;
      }
    }
    finished
  }

  #[inline]
  #[must_use]
  pub fn pending_count(&self) -> usize {
    self.state.lock().pending_background_jobs.len()
  }

  #[inline]
  #[must_use]
  pub fn finalizable_count(&self) -> usize {
    self.state.lock().finalizable_jobs.len()
  }
}

/// 测试友元台账 (原版 FRIEND_TEST ×8 — Rust 侧以 #[cfg(test)] 模块承接)
pub const FRIEND_TESTS: [&str; 8] = [
  "IdleTaskNoIdleTime",
  "IdleTaskSmallIdleTime",
  "FinishNowWithWorkerTask",
  "AbortJobNotStarted",
  "AbortJobAlreadyStarted",
  "AsyncAbortAllPendingWorkerTask",
  "AsyncAbortAllRunningWorkerTask",
  "CompileMultipleOnBackgroundThread",
];
