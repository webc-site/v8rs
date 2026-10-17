// 由 tool/src/compiler-dispatcher/optimizing-compile-dispatcher.h.js 从 v8/src/compiler-dispatcher/optimizing-compile-dispatcher.{h,cc} 生成; 修订就地改, 复跑覆盖
// 优化编译调度: 任务状态结构 + 输入/输出队列 + 执行器 + 调度器签名骨架

//! 优化编译调度器 — 原版 namespace v8::internal::OptimizingCompileDispatcher 一族
//!
//! 语义: 线程池执行 Turbofan 任务; 输入队列为环形容量队列 (多隔离共享),
//! 输出队列为隔离本地终结队列; 任务状态向量按缓存行对齐防伪共享。

use core::mem::align_of;
use std::sync::{
  Arc,
  atomic::{AtomicBool, AtomicU64, Ordering},
};

use parking_lot::Mutex;

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
      queue: Mutex::new(Vec::new()),
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
    if let Some(pos) = q
      .iter()
      .position(|j| j.isolate_id == isolate_id && j.function_id == function_id)
    {
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
      queue: Mutex::new(Vec::new()),
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
    q.iter()
      .any(|j| j.function_id == function_id && j.osr_offset == osr_offset)
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
  pub input_queue: Arc<OptimizingCompileInputQueue>,
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
      input_queue: Arc::new(OptimizingCompileInputQueue::new(capacity)),
      task_states: Mutex::new(states),
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
  next_job_id: AtomicU64,
  executor: Arc<OptimizingCompileTaskExecutor>,
  output_queue: OptimizingCompileOutputQueue,
  finalize: AtomicBool,
}

impl OptimizingCompileDispatcher {
  #[inline]
  #[must_use]
  pub fn new(isolate_id: u32, executor: Arc<OptimizingCompileTaskExecutor>) -> Self {
    Self {
      isolate_id,
      next_job_id: AtomicU64::new(1),
      executor,
      output_queue: OptimizingCompileOutputQueue::new(),
      finalize: AtomicBool::new(true),
    }
  }

  #[inline]
  #[must_use]
  pub const fn isolate_id(&self) -> u32 {
    self.isolate_id
  }

  pub fn try_queue_for_optimization(&self, function_id: u32, osr_offset: i32) -> bool {
    let job_id = self.next_job_id.fetch_add(1, Ordering::Relaxed);
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
    self
      .executor
      .has_compilation_jobs_for_isolate(self.isolate_id)
      || !self.output_queue.is_empty()
  }

  #[inline]
  #[must_use]
  pub fn finalize(&self) -> bool {
    self.finalize.load(Ordering::Acquire)
  }

  #[inline]
  pub fn set_finalize(&self, v: bool) {
    self.finalize.store(v, Ordering::Release);
  }

  pub fn flush(&self, _mode: ModeFlag) {
    self
      .executor
      .input_queue
      .flush_jobs_for_isolate(self.isolate_id);
    let mut q = self.output_queue.queue.lock();
    q.retain(|j| j.isolate_id != self.isolate_id);
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(align_of::<OptimizingCompileTaskState>() == 64);
  assert!(ModeFlag::Compile as u8 == 0);
  assert!(ModeFlag::Flush as u8 == 1);
};
