#!/usr/bin/env bun
// tool/src/compiler-dispatcher/optimizing-compile-dispatcher.h.js
// 机械提取 v8/src/compiler-dispatcher/optimizing-compile-dispatcher.{h,cc}
// 产物: v8-compiler-dispatcher/src/optimizing_compile_dispatcher.rs — 四类骨架 + ModeFlag 枚举

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/compiler-dispatcher/optimizing-compile-dispatcher.h");
const ccSrc = readV8("src/compiler-dispatcher/optimizing-compile-dispatcher.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. alignas 缓存行对齐的任务状态结构 (2 字段)
const taskState = hSrc.match(/struct alignas\(PROCESSOR_CACHE_LINE_SIZE\) OptimizingCompileTaskState \{([\s\S]*?)\};/);
must(taskState, "OptimizingCompileTaskState 结构");
const tsFields = [...taskState[1].matchAll(/^\s*(\w[\w:]*)\*?\s+(\w+);$/gm)].map((m) => m[2]);
if (tsFields.length !== 2 || tsFields[0] !== "isolate" || tsFields[1] !== "job") {
  fail("TaskState 字段", tsFields.join(","), "isolate,job");
}

// 2. OptimizingCompileInputQueue 公开方法 9 (含 ctor)
const queueApi = [
  "OptimizingCompileInputQueue(", "IsAvailable(", "Length(", "Dequeue(",
  "DequeueIfIsolateMatches(", "Enqueue(", "FlushJobsForIsolate(", "HasJobForIsolate(", "Prioritize(",
];
for (const q of queueApi) {
  if (!hSrc.includes(q)) fail(`InputQueue 方法 ${q}`, false, true);
}

// 3. OptimizingCompileTaskExecutor 公开方法 14 + 2 个优先级常量
const executorApi = [
  "OptimizingCompileTaskExecutor(", "~OptimizingCompileTaskExecutor", "EnsureStarted(",
  "Stop(", "RunCompilationJob(", "NextInput(", "NextInputIfIsolateMatches(",
  "IsTaskRunningForIsolate(", "ClearTaskState(", "ResetJob(", "TryQueueForOptimization(",
  "WaitUntilCompilationJobsDoneForIsolate(", "CancelCompilationJobsForIsolate(",
  "HasCompilationJobsForIsolate(",
];
for (const e of executorApi) {
  if (!hSrc.includes(e)) fail(`TaskExecutor 方法 ${e}`, false, true);
}
must(hSrc.includes("static constexpr TaskPriority kTaskPriority = TaskPriority::kUserVisible;"), "kTaskPriority");
must(hSrc.includes("kEfficiencyTaskPriority =\n      TaskPriority::kBestEffort;"), "kEfficiencyTaskPriority");

// 4. OptimizingCompileOutputQueue 方法 6
const outputApi = ["Enqueue(", "Dequeue(", "IsJobReady(", "InstallGeneratedBuiltins(", "size() const", "empty() const"];
for (const o of outputApi) {
  if (!hSrc.includes(o)) fail(`OutputQueue 方法 ${o}`, false, true);
}

// 5. OptimizingCompileDispatcher 公开方法 17 (含 ctor/dtor + ModeFlag)
const dispatcherApi = [
  "OptimizingCompileDispatcher(", "~OptimizingCompileDispatcher", "Flush(", "FlushQueues(",
  "FlushInputQueue(", "FlushOutputQueue(", "TryQueueForOptimization(",
  "WaitUntilCompilationJobsDone(", "InstallOptimizedFunctions(", "InstallOptimizedFunctionsIfReady(",
  "InstallGeneratedBuiltins(", "IsQueueAvailable()", "Enabled()", "HasJobs()",
  "finalize() const", "set_finalize(", "Prioritize(", "StartTearDown(", "FinishTearDown(",
  "QueueFinishedJob(",
];
for (const d of dispatcherApi) {
  if (!hSrc.includes(d)) fail(`Dispatcher 方法 ${d}`, false, true);
}

// 6. ModeFlag 枚举 (COMPILE, FLUSH) — 显式展开
const modeFlag = hSrc.match(/enum ModeFlag \{ COMPILE, FLUSH \};/);
must(modeFlag, "enum ModeFlag");

// 7. finalize_ 默认 true
must(hSrc.includes("bool finalize_ = true;"), "finalize_ 默认");

// 8. cc 定义 1:1 — 按类分组核对定义行数 (实测: TaskExecutor 15 / Dispatcher 16 /
//    InputQueue 6 / OutputQueue 6)
const defLines = (cls, want) => {
  const re = new RegExp(`^.*${cls}::.*$`, "gm");
  const got = (ccSrc.match(re) || []).length;
  if (got !== want) fail(`cc ${cls} 定义行`, got, want);
};
defLines("OptimizingCompileTaskExecutor", 15);
defLines("OptimizingCompileDispatcher", 16);
defLines("OptimizingCompileInputQueue", 6);
defLines("OptimizingCompileOutputQueue", 6);

// 9. Enabled 语义 (v8_flags.concurrent_recompilation)
must(hSrc.includes("v8_flags.concurrent_recompilation"), "Enabled 读 concurrent_recompilation");

let n = 56;
const rust = `${banner("src/compiler-dispatcher/optimizing-compile-dispatcher.h.js", "src/compiler-dispatcher/optimizing-compile-dispatcher.{h,cc}", "优化编译调度: 任务状态结构 + 输入/输出队列 + 执行器 + 调度器签名骨架")}
//! 优化编译调度器 — 原版 namespace v8::internal::OptimizingCompileDispatcher 一族
//!
//! 语义: 线程池执行 Turbofan 任务; 输入队列为环形容量队列 (多隔离共享),
//! 输出队列为隔离本地终结队列; 任务状态向量按缓存行对齐防伪共享。

/// 任务状态 (原版 struct alignas(PROCESSOR_CACHE_LINE_SIZE) OptimizingCompileTaskState)
/// 对齐语义: Rust 侧用 #[repr(align(64))] 承接 (x64/arm64 缓存行 64B)
#[repr(align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OptimizingCompileTaskState {
  pub isolate_id: u32,
  pub job_id: u64,
}

impl OptimizingCompileTaskState {
  #[inline]
  #[must_use]
  pub const fn new(isolate_id: u32, job_id: u64) -> Self {
    Self { isolate_id, job_id }
  }

  #[inline]
  #[must_use]
  pub const fn is_idle(&self) -> bool {
    self.isolate_id == 0 && self.job_id == 0
  }
}

/// 输入队列调度模式 (原版 enum ModeFlag, 显式展开: COMPILE=0, FLUSH=1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum ModeFlag {
  /// 原版 COMPILE — 正常编译
  Compile = 0,
  /// 原版 FLUSH — 冲刷丢弃
  Flush = 1,
}

impl ModeFlag {
  #[inline]
  #[must_use]
  pub const fn is_compile(self) -> bool {
    matches!(self, Self::Compile)
  }

  #[inline]
  #[must_use]
  pub const fn is_flush(self) -> bool {
    matches!(self, Self::Flush)
  }
}

/// 执行器任务优先级 (原版 kTaskPriority / kEfficiencyTaskPriority)
pub const TASK_PRIORITY: &str = "kUserVisible";
pub const EFFICIENCY_TASK_PRIORITY: &str = "kBestEffort";

/// 优化编译作业数据结构
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizingJob {
  pub job_id: u64,
  pub isolate_id: u32,
  pub function_id: u32,
  pub osr_offset: i32,
}

impl OptimizingJob {
  #[inline]
  #[must_use]
  pub const fn new(job_id: u64, isolate_id: u32, function_id: u32, osr_offset: i32) -> Self {
    Self {
      job_id,
      isolate_id,
      function_id,
      osr_offset,
    }
  }
}

/// 环形容量线程安全输入队列
#[derive(Debug)]
pub struct OptimizingCompileInputQueue {
  capacity: usize,
  queue: parking_lot::Mutex<Vec<OptimizingJob>>,
}

impl OptimizingCompileInputQueue {
  #[inline]
  #[must_use]
  pub const fn new(capacity: usize) -> Self {
    Self {
      capacity,
      queue: parking_lot::Mutex::new(Vec::new()),
    }
  }

  #[inline]
  #[must_use]
  pub const fn capacity(&self) -> usize {
    self.capacity
  }

  #[inline]
  #[must_use]
  pub fn is_available(&self) -> bool {
    self.queue.lock().len() < self.capacity
  }

  #[inline]
  #[must_use]
  pub fn length(&self) -> usize {
    self.queue.lock().len()
  }

  pub fn enqueue(&self, job: OptimizingJob) -> bool {
    let mut q = self.queue.lock();
    if q.len() >= self.capacity {
      return false;
    }
    q.push(job);
    true
  }

  pub fn dequeue(&self) -> Option<OptimizingJob> {
    let mut q = self.queue.lock();
    if q.is_empty() {
      None
    } else {
      Some(q.remove(0))
    }
  }

  pub fn flush_jobs_for_isolate(&self, isolate_id: u32) {
    let mut q = self.queue.lock();
    q.retain(|j| j.isolate_id != isolate_id);
  }

  pub fn has_job_for_isolate(&self, isolate_id: u32) -> bool {
    let q = self.queue.lock();
    q.iter().any(|j| j.isolate_id == isolate_id)
  }

  pub fn prioritize(&self, isolate_id: u32, function_id: u32) {
    let mut q = self.queue.lock();
    if let Some(pos) = q.iter().position(|j| j.isolate_id == isolate_id && j.function_id == function_id) {
      let job = q.remove(pos);
      q.insert(0, job);
    }
  }
}

/// 优化编译输出终结队列
#[derive(Debug, Default)]
pub struct OptimizingCompileOutputQueue {
  queue: parking_lot::Mutex<Vec<OptimizingJob>>,
}

impl OptimizingCompileOutputQueue {
  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self {
      queue: parking_lot::Mutex::new(Vec::new()),
    }
  }

  pub fn enqueue(&self, job: OptimizingJob) {
    self.queue.lock().push(job);
  }

  pub fn dequeue(&self) -> Option<OptimizingJob> {
    let mut q = self.queue.lock();
    if q.is_empty() {
      None
    } else {
      Some(q.remove(0))
    }
  }

  pub fn is_job_ready(&self, function_id: u32, osr_offset: i32) -> bool {
    let q = self.queue.lock();
    q.iter().any(|j| j.function_id == function_id && j.osr_offset == osr_offset)
  }

  pub fn install_generated_builtins(&self, installed_count: i32) -> i32 {
    let mut q = self.queue.lock();
    let to_remove = (installed_count as usize).min(q.len());
    q.drain(0..to_remove);
    to_remove as i32
  }

  #[inline]
  #[must_use]
  pub fn size(&self) -> usize {
    self.queue.lock().len()
  }

  #[inline]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.queue.lock().is_empty()
  }
}

/// 任务执行器
#[derive(Debug)]
pub struct OptimizingCompileTaskExecutor {
  pub input_queue: std::sync::Arc<OptimizingCompileInputQueue>,
  task_states: parking_lot::Mutex<Vec<OptimizingCompileTaskState>>,
}

impl OptimizingCompileTaskExecutor {
  #[inline]
  #[must_use]
  pub fn new(capacity: usize, worker_threads: usize) -> Self {
    let mut states = Vec::with_capacity(worker_threads);
    for _ in 0..worker_threads {
      states.push(OptimizingCompileTaskState::default());
    }
    Self {
      input_queue: std::sync::Arc::new(OptimizingCompileInputQueue::new(capacity)),
      task_states: parking_lot::Mutex::new(states),
    }
  }

  pub fn try_queue_for_optimization(&self, job: OptimizingJob) -> bool {
    self.input_queue.enqueue(job)
  }

  pub fn run_compilation_job(&self, task_idx: usize) -> Option<OptimizingJob> {
    if let Some(job) = self.input_queue.dequeue() {
      let mut states = self.task_states.lock();
      if let Some(state) = states.get_mut(task_idx) {
        state.isolate_id = job.isolate_id;
        state.job_id = job.job_id;
      }
      Some(job)
    } else {
      None
    }
  }

  pub fn is_task_running_for_isolate(&self, isolate_id: u32) -> bool {
    let states = self.task_states.lock();
    states.iter().any(|s| s.isolate_id == isolate_id)
  }

  pub fn has_compilation_jobs_for_isolate(&self, isolate_id: u32) -> bool {
    self.input_queue.has_job_for_isolate(isolate_id) || self.is_task_running_for_isolate(isolate_id)
  }
}

/// 顶层优化编译调度器
#[derive(Debug)]
pub struct OptimizingCompileDispatcher {
  isolate_id: u32,
  next_job_id: parking_lot::Mutex<u64>,
  executor: std::sync::Arc<OptimizingCompileTaskExecutor>,
  output_queue: OptimizingCompileOutputQueue,
  finalize: parking_lot::Mutex<bool>,
}

impl OptimizingCompileDispatcher {
  #[inline]
  #[must_use]
  pub fn new(isolate_id: u32, executor: std::sync::Arc<OptimizingCompileTaskExecutor>) -> Self {
    Self {
      isolate_id,
      next_job_id: parking_lot::Mutex::new(1),
      executor,
      output_queue: OptimizingCompileOutputQueue::new(),
      finalize: parking_lot::Mutex::new(true),
    }
  }

  #[inline]
  #[must_use]
  pub const fn isolate_id(&self) -> u32 {
    self.isolate_id
  }

  pub fn try_queue_for_optimization(&self, function_id: u32, osr_offset: i32) -> bool {
    let mut next_id = self.next_job_id.lock();
    let job_id = *next_id;
    *next_id = next_id.saturating_add(1);
    let job = OptimizingJob::new(job_id, self.isolate_id, function_id, osr_offset);
    self.executor.try_queue_for_optimization(job)
  }

  pub fn install_optimized_functions(&self) -> usize {
    let mut count = 0;
    while self.output_queue.dequeue().is_some() {
      count += 1;
    }
    count
  }

  pub fn install_optimized_functions_if_ready(&self, function_id: u32, osr_offset: i32) -> bool {
    self.output_queue.is_job_ready(function_id, osr_offset)
  }

  pub fn install_generated_builtins(&self, count: i32) -> i32 {
    self.output_queue.install_generated_builtins(count)
  }

  #[inline]
  #[must_use]
  pub fn is_queue_available(&self) -> bool {
    self.executor.input_queue.is_available()
  }

  #[inline]
  #[must_use]
  pub fn has_jobs(&self) -> bool {
    self.executor.has_compilation_jobs_for_isolate(self.isolate_id) || !self.output_queue.is_empty()
  }

  #[inline]
  #[must_use]
  pub fn finalize(&self) -> bool {
    *self.finalize.lock()
  }

  #[inline]
  pub fn set_finalize(&self, v: bool) {
    *self.finalize.lock() = v;
  }

  pub fn flush(&self, _mode: ModeFlag) {
    self.executor.input_queue.flush_jobs_for_isolate(self.isolate_id);
    let mut q = self.output_queue.queue.lock();
    q.retain(|j| j.isolate_id != self.isolate_id);
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(core::mem::align_of::<OptimizingCompileTaskState>() == 64);
  assert!(ModeFlag::Compile as u8 == 0);
  assert!(ModeFlag::Flush as u8 == 1);
};
`;

const p = writeSrc("v8-compiler-dispatcher", "optimizing_compile_dispatcher.rs", rust);
console.log(`OK optimizing-compile-dispatcher.h.js: 产物 ${p}`);
