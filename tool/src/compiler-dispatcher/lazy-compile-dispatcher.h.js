#!/usr/bin/env bun
// tool/src/compiler-dispatcher/lazy-compile-dispatcher.h.js
// 机械提取 v8/src/compiler-dispatcher/lazy-compile-dispatcher.{h,cc}
// 产物: v8-compiler-dispatcher/src/lazy_compile_dispatcher.rs — Job 状态机枚举 + FRIEND_TEST 台账 + 签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/compiler-dispatcher/lazy-compile-dispatcher.h");
const ccSrc = readV8("src/compiler-dispatcher/lazy-compile-dispatcher.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. JobId 类型别名
const jobId = hSrc.match(/using JobId = uintptr_t;/);
must(jobId, "using JobId = uintptr_t");

// 2. Job::State 状态机 9 态 (h 枚举体逐态提取)
const stateBody = hSrc.match(/enum class State \{([\s\S]*?)\n    \};/);
must(stateBody, "Job::State 枚举体");
const states = [...stateBody[1].matchAll(/^\s+(k\w+),?(?:\s*\/\/\s*(.*))?$/gm)]
  .filter((m) => !m[1] || !m[1].includes("---"))
  .map((m) => ({ name: m[1] ? m[1] : "", id: m[0] ? m[0] : "", raw: m[0] && m[0].trim() ? m[0].trim() : "" }));
// 上面三列无用, 重新精取: 只取行首枚举项标识符 (k 开头, 允许行尾注释)
const stateNames = [...stateBody[1].matchAll(/^\s*(k\w+),/gm)].map((m) => m[1]);
if (stateNames.length !== 9) fail("Job::State 状态数", stateNames.length, 9);

// 3. FRIEND_TEST 台账 8 条
const friends = [...hSrc.matchAll(/FRIEND_TEST\(LazyCompileDispatcherTest, (\w+)\);/g)].map((m) => m[1]);
if (friends.length !== 8) fail("FRIEND_TEST 数", friends.length, 8);

// 4. 公开 API (h)
const pubApi = ["LazyCompileDispatcher(", "~LazyCompileDispatcher", "Enqueue(", "IsEnqueued(", "FinishNow(", "AbortJob(", "AbortAll("];
for (const p of pubApi) {
  if (!hSrc.includes(p)) fail(`公开 API ${p}`, false, true);
}

// 5. 私有方法 (h)
const privApi = [
  "WaitForJobIfRunningOnBackground", "GetJobFor", "PopSingleFinalizeJob",
  "ScheduleIdleTaskFromAnyThread", "FinalizeSingleJob", "DoBackgroundWork", "DoIdleWork",
  "DeleteJob", "NotifyAddedBackgroundJob", "NotifyRemovedBackgroundJob", "VerifyBackgroundTaskCount",
];
for (const p of privApi) {
  if (!hSrc.includes(p)) fail(`私有方法 ${p}`, false, true);
}

// 6. 成员台账 (h 关键成员)
const members = [
  "isolate_", "worker_thread_runtime_call_stats_", "background_compile_timer_",
  "taskrunner_", "platform_", "max_stack_size_", "job_handle_",
  "trace_compiler_dispatcher_", "idle_task_manager_", "mutex_",
  "idle_task_scheduled_", "pending_background_jobs_", "finalizable_jobs_",
  "num_jobs_for_background_", "all_jobs_", "jobs_to_dispose_",
  "main_thread_blocking_on_job_", "main_thread_blocking_signal_",
  "block_for_testing_", "semaphore_for_testing_",
];
for (const m of members) {
  if (!hSrc.includes(m)) fail(`成员 ${m}`, false, true);
}

// 7. cc 常量
const finMs = ccSrc.match(/static constexpr int kMaxOpportunisticFinalizeTimeMs = (\d+);/);
must(finMs, "kMaxOpportunisticFinalizeTimeMs");
if (finMs[1] !== "1") fail("机会式 finalize 上限", finMs[1], "1");

// 8. cc 定义 1:1 — 21 个定义行; 其中 GetJobFor/PopSingleFinalizeJob 两行带
//    LazyCompileDispatcher::Job* 返回类型, 共 23 次出现
const ccDefLines = (ccSrc.match(/^.*LazyCompileDispatcher::.*$/gm) || []).length;
if (ccDefLines !== 21) fail("cc LazyCompileDispatcher:: 定义行", ccDefLines, 21);
const ccDefs = (ccSrc.match(/LazyCompileDispatcher::/g) || []).length;
if (ccDefs !== 23) fail("cc LazyCompileDispatcher:: 出现", ccDefs, 23);

// 9. is_running_on_background 谓词语义
must(hSrc.includes("s == State::kRunning || s == State::kAbortRequested"), "is_running_on_background 谓词");

let n = 47;
const rust = `${banner("src/compiler-dispatcher/lazy-compile-dispatcher.h.js", "src/compiler-dispatcher/lazy-compile-dispatcher.{h,cc}", "LazyCompileDispatcher: Job 状态机 + FRIEND_TEST 台账 + 签名骨架")}
//! 惰性编译调度器 — 原版 namespace v8::internal::LazyCompileDispatcher
//!
//! 语义: 空闲任务 + 后台任务组合推进惰性解析函数的 parse/compile;
//! 三队列协作: jobs_ 全集 / pending_background_jobs_ 可后台 / finalizable_jobs_ 待终结。

/// Job 句柄类型 (原版 using JobId = uintptr_t)
pub type JobId = u64;

/// 单个机会式 finalize 的时长上限, 毫秒 (原版 kMaxOpportunisticFinalizeTimeMs, cc:35)
pub const MAX_OPPORTUNISTIC_FINALIZE_TIME_MS: i32 = ${finMs[1]};

/// Job 状态机 (原版 LazyCompileDispatcher::Job::State, 9 态 1:1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum JobState {
${stateNames
  .map((s) => {
    const note =
      {
        kPending: "在后台待命队列",
        kRunning: "后台执行中",
        kAbortRequested: "请求中止 (丢弃结果)",
        kReadyToFinalize: "在终结队列",
        kAborted: "已中止",
        kPendingToRunOnForeground: "已弹出待命队列, 主线程待跑",
        kFinalizingNow: "已弹出终结队列, 正终结",
        kAbortingNow: "终结中且需中止",
        kFinalized: "终结完毕, 待删除",
      }[s] ?? "";
    return `  /// 原版 ${s}${note ? ` — ${note}` : ""}\n  ${s.replace(/^k/, "")},`;
  })
  .join("\n")}
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

/// 惰性编译调度器
#[derive(Debug, Default)]
pub struct LazyCompileDispatcher {
  max_stack_size: usize,
  next_job_id: parking_lot::Mutex<u64>,
  jobs: parking_lot::Mutex<Vec<LazyJob>>,
  pending_background_jobs: parking_lot::Mutex<Vec<JobId>>,
  finalizable_jobs: parking_lot::Mutex<Vec<JobId>>,
}

impl LazyCompileDispatcher {
  #[inline]
  #[must_use]
  pub fn new(max_stack_size: usize) -> Self {
    Self {
      max_stack_size,
      next_job_id: parking_lot::Mutex::new(1),
      jobs: parking_lot::Mutex::new(Vec::new()),
      pending_background_jobs: parking_lot::Mutex::new(Vec::new()),
      finalizable_jobs: parking_lot::Mutex::new(Vec::new()),
    }
  }

  #[inline]
  #[must_use]
  pub const fn max_stack_size(&self) -> usize {
    self.max_stack_size
  }

  /// 投递惰性解析任务
  pub fn enqueue(&self, function_id: u32, character_stream_len: usize) -> JobId {
    let mut next_id = self.next_job_id.lock();
    let id = *next_id;
    *next_id = next_id.saturating_add(1);

    let mut jobs = self.jobs.lock();
    jobs.push(LazyJob::new(id, function_id, character_stream_len));

    let mut pending = self.pending_background_jobs.lock();
    pending.push(id);

    id
  }

  /// 检查某函数是否在队列中
  pub fn is_enqueued(&self, function_id: u32) -> bool {
    let jobs = self.jobs.lock();
    jobs.iter().any(|j| j.function_id == function_id && !j.state.is_aborted() && j.state != JobState::Finalized)
  }

  /// 阻塞并立即在主线程完成编译
  pub fn finish_now(&self, function_id: u32) -> bool {
    let mut jobs = self.jobs.lock();
    if let Some(job) = jobs.iter_mut().find(|j| j.function_id == function_id && !j.state.is_aborted() && j.state != JobState::Finalized) {
      job.state = JobState::Finalized;
      // 从待定队列中移除
      let mut pending = self.pending_background_jobs.lock();
      pending.retain(|&id| id != job.id);
      let mut finalizable = self.finalizable_jobs.lock();
      finalizable.retain(|&id| id != job.id);
      true
    } else {
      false
    }
  }

  /// 中止某函数的编译任务
  pub fn abort_job(&self, function_id: u32) -> bool {
    let mut jobs = self.jobs.lock();
    if let Some(job) = jobs.iter_mut().find(|j| j.function_id == function_id && !j.state.is_aborted() && j.state != JobState::Finalized) {
      job.state = JobState::Aborted;
      let mut pending = self.pending_background_jobs.lock();
      pending.retain(|&id| id != job.id);
      let mut finalizable = self.finalizable_jobs.lock();
      finalizable.retain(|&id| id != job.id);
      true
    } else {
      false
    }
  }

  /// 中止全部任务
  pub fn abort_all(&self) {
    let mut jobs = self.jobs.lock();
    for job in jobs.iter_mut() {
      if !job.state.is_aborted() && job.state != JobState::Finalized {
        job.state = JobState::Aborted;
      }
    }
    self.pending_background_jobs.lock().clear();
    self.finalizable_jobs.lock().clear();
  }

  /// 模拟后台工作线程推进任务
  pub fn do_background_work(&self) -> usize {
    let mut pending = self.pending_background_jobs.lock();
    let mut processed = 0;
    while let Some(id) = pending.pop() {
      let mut jobs = self.jobs.lock();
      if let Some(job) = jobs.iter_mut().find(|j| j.id == id && j.state == JobState::Pending) {
        job.state = JobState::ReadyToFinalize;
        self.finalizable_jobs.lock().push(id);
        processed += 1;
      }
    }
    processed
  }

  /// 空闲时进行机会式终结
  pub fn do_idle_work(&self, _deadline_in_seconds: f64) -> usize {
    let mut finalizable = self.finalizable_jobs.lock();
    let mut finished = 0;
    while let Some(id) = finalizable.pop() {
      let mut jobs = self.jobs.lock();
      if let Some(job) = jobs.iter_mut().find(|j| j.id == id && j.state == JobState::ReadyToFinalize) {
        job.state = JobState::Finalized;
        finished += 1;
      }
    }
    finished
  }

  #[inline]
  #[must_use]
  pub fn pending_count(&self) -> usize {
    self.pending_background_jobs.lock().len()
  }

  #[inline]
  #[must_use]
  pub fn finalizable_count(&self) -> usize {
    self.finalizable_jobs.lock().len()
  }
}

/// 测试友元台账 (原版 FRIEND_TEST ×8 — Rust 侧以 #[cfg(test)] 模块承接)
pub const FRIEND_TESTS: [&str; 8] = [
${friends.map((f) => `  ${JSON.stringify(f)},`).join("\n")}
];
`;

const p = writeSrc("v8-compiler-dispatcher", "lazy_compile_dispatcher.rs", rust);
console.log(`OK lazy-compile-dispatcher.h.js: 产物 ${p}`);
