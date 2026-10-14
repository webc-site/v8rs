//! 对应 `v8/src/libplatform/default-platform.{h,cc}`：默认 Platform 实现（LP-A 面）。
//!
//! 覆盖：平台生命周期（构造/析构/后台运行器惰性初始化）、消息泵
//! `pump_message_loop`、`run_idle_tasks`、前台 TaskRunner 管理
//! （`get_foreground_task_runner` ⇔ 原版 `CallOnForegroundThread` 的投递入口）、
//! `notify_isolate_shutdown`、时间函数注入与 worker 投递转发。
//!
//! 推迟登记（见 README.md 双段）：
//! - worker 线程 spawn：LP-B `worker_threads_task_runner.rs` 未就位，经
//!   `WorkerRunnerFactory` 窄 trait 衔接；缺位时 worker 投递返回
//!   `Error::WorkerRunnerUnavailable`（对应原版 `DCHECK_NOT_NULL`）。
//! - `CreateJobImpl`：LP-B default-job 族（JobTask/JobHandle 接口本波未定义）。
//! - `GetThreadIsolatedAllocator`：LP-B thread-isolated-allocator 族。
//! - `GetStackTracePrinter` / `InProcessStackDumping::Enabled`：base::debug 族。
//! - `GetZeroSegmentSize`：原版 `V8_ENABLE_PARTITION_ALLOC` 未定义分支恒返 0，
//!   平台裁剪登记，不建模。
//! - 默认 tracing controller 构造：LP-C tracing/ 族未就位，缺省为 None。
//!
//! 并发纪律沿用原版：`state` 只在持有 `lock` 时访问。

use std::{cell::UnsafeCell, collections::HashMap, sync::Arc};

use v8_base::{
  page_allocator::{DefaultPageAllocator, PageAllocator},
  sys_info::number_of_processors,
};
use v8_base_platform::{Mutex, OS, Priority, TimeConstants, TimeTicks};
use v8_tasks::{MAX_PRIORITY, TaskPriority};

use crate::{
  api::{
    BoxTask, IdleTaskSupport, InProcessStackDumping, IsolateKey, MessageLoopBehavior, Platform,
    PriorityMode, SourceLocation, TaskRunner, TimeFunction, TracingController, WorkerRunnerFactory,
    WorkerTaskRunner,
  },
  error::{Error, Result},
  foreground_task_runner::{DefaultForegroundTaskRunner, RunTaskScope},
};

/// 对应原 `kMaxThreadPoolSize`。
pub const K_MAX_THREAD_POOL_SIZE: i32 = 16;

/// 对应原版 `NewDefaultPlatform` 的入参聚合（Rust 无默认参数，收敛为选项结构体，
/// 参数最小化原则）。
#[derive(Default)]
pub struct PlatformOptions {
  /// <=0 时按在线处理器数自动（仅 `new_default_platform` 口径；直接 `new` 同原版存原值）。
  pub thread_pool_size: i32,
  pub idle_task_support: IdleTaskSupport,
  pub priority_mode: PriorityMode,
  pub in_process_stack_dumping: InProcessStackDumping,
  /// 缺省 None 时原版会构造 tracing::TracingController（LP-C 未就位，推迟登记）。
  pub tracing_controller: Option<Arc<dyn TracingController>>,
  /// LP-B 衔接点：缺位时后台运行器槽保持空，worker 投递返回 Err。
  pub worker_runner_factory: Option<WorkerRunnerFactory>,
}

/// 对应原 `DefaultPlatform` 私有状态（lock 保护域）。
struct PlatformState {
  /// 每优先级一个后台运行器；对应原 `worker_threads_task_runners_[prio]`。
  worker_threads_task_runners: Vec<Option<Arc<dyn WorkerTaskRunner>>>,
  foreground_task_runner_map: HashMap<IsolateKey, Arc<DefaultForegroundTaskRunner>>,
  tracing_controller: Option<Arc<dyn TracingController>>,
  time_function_for_testing: Option<TimeFunction>,
}

/// 对应原 `class v8::platform::DefaultPlatform`。
pub struct DefaultPlatform {
  lock: Mutex,
  state: UnsafeCell<PlatformState>,
  thread_pool_size: i32,
  idle_task_support: IdleTaskSupport,
  priority_mode: PriorityMode,
  page_allocator: DefaultPageAllocator,
  worker_runner_factory: Option<WorkerRunnerFactory>,
}

// SAFETY: `state` 只在持有 `lock` 时读写；其余字段构造后不可变且为 Copy。
unsafe impl Send for DefaultPlatform {}
// SAFETY: 同上。
unsafe impl Sync for DefaultPlatform {}

/// 对应原匿名命名空间 `DefaultTimeFunction`：TimeTicks 内部微秒值折算为秒。
fn default_time_function() -> f64 {
  TimeTicks::now().to_internal_value() as f64 / TimeConstants::MICROSECONDS_PER_SECOND as f64
}

impl DefaultPlatform {
  /// 对应原构造函数（thread_pool_size 取原值，不做自动折算，同原版直构语义）。
  pub fn new(options: PlatformOptions) -> Arc<Self> {
    debug_assert!(options.thread_pool_size >= 0);
    let num_runners = Self::worker_runner_count(options.priority_mode);
    let platform = Arc::new(Self {
      lock: Mutex::new(),
      state: UnsafeCell::new(PlatformState {
        worker_threads_task_runners: vec![None; num_runners],
        foreground_task_runner_map: HashMap::new(),
        tracing_controller: options.tracing_controller,
        time_function_for_testing: None,
      }),
      thread_pool_size: options.thread_pool_size,
      idle_task_support: options.idle_task_support,
      priority_mode: options.priority_mode,
      page_allocator: DefaultPageAllocator,
      worker_runner_factory: options.worker_runner_factory,
    });
    // 对应原构造尾：缺省 tracing 时构造 tracing::TracingController（LP-C 推迟，留 None）。
    if platform.state_ref_init().tracing_controller.is_none() {
      log::debug!("tracing_controller 缺省构造推迟：LP-C tracing/ 族未就位");
    }
    if platform.thread_pool_size > 0 {
      if platform.worker_runner_factory.is_none() {
        log::warn!(
          "LP-B worker runner 工厂未接入：后台任务投递将返回 WorkerRunnerUnavailable（推迟登记）"
        );
      }
      platform.ensure_background_task_runner_initialized();
    }
    platform
  }

  /// 构造期只读快照（尚未发布给其他线程，直接取引用）。
  fn state_ref_init(&self) -> &PlatformState {
    // SAFETY: `new` 阶段 Arc 未共享，无并发访问。
    unsafe { &*self.state.get() }
  }

  /// 对应原 `EnsureBackgroundTaskRunnerInitialized`。
  fn ensure_background_task_runner_initialized(&self) {
    {
      let _guard = self.lock();
      // SAFETY: 已持有 `lock`。
      let state = unsafe { &mut *self.state.get() };
      // 对应原 DCHECK_NULL(worker_threads_task_runners_[0])。
      debug_assert!(
        state
          .worker_threads_task_runners
          .iter()
          .all(Option::is_none)
      );
      if let Some(factory) = self.worker_runner_factory {
        let time_function = state
          .time_function_for_testing
          .unwrap_or(default_time_function);
        for (i, slot) in state.worker_threads_task_runners.iter_mut().enumerate() {
          let priority = self.priority_from_index(i as i32);
          *slot = Some(factory(self.thread_pool_size, time_function, priority));
        }
      }
      // 对应原 DCHECK_NOT_NULL(worker_threads_task_runners_[0])（工厂缺位时跳过）。
      debug_assert!(
        self.worker_runner_factory.is_none()
          || state
            .worker_threads_task_runners
            .first()
            .is_some_and(Option::is_some)
      );
    }
  }

  /// 对应原 `SetTimeFunctionForTesting`：须在平台构造后立即设置。
  pub fn set_time_function_for_testing(&self, time_function: TimeFunction) {
    let _guard = self.lock();
    // SAFETY: 已持有 `lock`。
    let state = unsafe { &mut *self.state.get() };
    state.time_function_for_testing = Some(time_function);
    debug_assert!(state.foreground_task_runner_map.is_empty());
  }

  /// 对应原 `PumpMessageLoop`：非嵌套时任意任务可跑；嵌套泵内只跑 nestable 任务。
  pub fn pump_message_loop(&self, isolate: IsolateKey, behavior: MessageLoopBehavior) -> bool {
    let failed_result = behavior == MessageLoopBehavior::WaitForWork;
    let task_runner = {
      let _guard = self.lock();
      // SAFETY: 已持有 `lock`。
      match unsafe { &*self.state.get() }
        .foreground_task_runner_map
        .get(&isolate)
      {
        Some(runner) => runner.clone(),
        None => return failed_result,
      }
    };

    let Some(mut task) = task_runner.pop_task_from_queue(behavior) else {
      return failed_result;
    };
    let _scope = RunTaskScope::new(task_runner);
    task.run();
    true
  }

  /// 对应原 `RunIdleTasks`：在 `idle_time_in_seconds` 预算内连续跑空闲任务。
  pub fn run_idle_tasks(&self, isolate: IsolateKey, idle_time_in_seconds: f64) {
    debug_assert!(self.idle_task_support == IdleTaskSupport::Enabled);
    let task_runner = {
      let _guard = self.lock();
      // SAFETY: 已持有 `lock`。
      match unsafe { &*self.state.get() }
        .foreground_task_runner_map
        .get(&isolate)
      {
        Some(runner) => runner.clone(),
        None => return,
      }
    };
    let deadline_in_seconds = self.monotonically_increasing_time() + idle_time_in_seconds;

    while deadline_in_seconds > self.monotonically_increasing_time() {
      let Some(mut task) = task_runner.pop_task_from_idle_queue() else {
        return;
      };
      let _scope = RunTaskScope::new(task_runner.clone());
      task.run(deadline_in_seconds);
    }
  }

  /// 对应原 `SetTracingController`。
  pub fn set_tracing_controller(&self, tracing_controller: Arc<dyn TracingController>) {
    let _guard = self.lock();
    // SAFETY: 已持有 `lock`。
    unsafe { &mut *self.state.get() }.tracing_controller = Some(tracing_controller);
  }

  /// 对应原 `GetForegroundTaskRunner`：isolate 首次访问时惰性建前台运行器。
  fn foreground_task_runner(&self, isolate: IsolateKey) -> Arc<DefaultForegroundTaskRunner> {
    let _guard = self.lock();
    // SAFETY: 已持有 `lock`。
    let state = unsafe { &mut *self.state.get() };
    if let Some(runner) = state.foreground_task_runner_map.get(&isolate) {
      return runner.clone();
    }
    let time_function = state
      .time_function_for_testing
      .unwrap_or(default_time_function);
    let runner = Arc::new(DefaultForegroundTaskRunner::new(
      self.idle_task_support,
      time_function,
    ));
    state
      .foreground_task_runner_map
      .insert(isolate, runner.clone());
    runner
  }

  /// 对应原 `PostTaskOnWorkerThreadImpl`。
  fn post_task_on_worker_thread_impl(&self, priority: TaskPriority, task: BoxTask) -> Result<()> {
    let runner = self.worker_runner(priority)?;
    runner.post_task(task);
    Ok(())
  }

  /// 对应原 `PostDelayedTaskOnWorkerThreadImpl`。
  fn post_delayed_task_on_worker_thread_impl(
    &self,
    priority: TaskPriority,
    task: BoxTask,
    delay_in_seconds: f64,
  ) -> Result<()> {
    let runner = self.worker_runner(priority)?;
    runner.post_delayed_task(task, delay_in_seconds);
    Ok(())
  }

  fn worker_runner(&self, priority: TaskPriority) -> Result<Arc<dyn WorkerTaskRunner>> {
    let index = self.priority_to_index(priority);
    let _guard = self.lock();
    // SAFETY: 已持有 `lock`。
    let runner = unsafe { &*self.state.get() }.worker_threads_task_runners[index].clone();
    runner.ok_or(Error::WorkerRunnerUnavailable { priority })
  }

  /// 对应原 `NotifyIsolateShutdown`：摘除并在锁外终止前台运行器。
  /// 原版对缺失 key 会空引用解引用（UB）；此处按 no-op 承接（偏差登记 README.md）。
  pub fn notify_isolate_shutdown(&self, isolate: IsolateKey) {
    let task_runner = {
      let _guard = self.lock();
      // SAFETY: 已持有 `lock`。
      unsafe { &mut *self.state.get() }
        .foreground_task_runner_map
        .remove(&isolate)
    };
    if let Some(runner) = task_runner {
      runner.terminate();
    }
  }

  /// 对应原 `MonotonicallyIncreasingTime`。
  pub fn monotonically_increasing_time(&self) -> f64 {
    // 原版无锁读 time_function_for_testing_；Rust 下统一持锁消除数据竞争。
    let _guard = self.lock();
    // SAFETY: 已持有 `lock`。
    let time_function = unsafe { &*self.state.get() }
      .time_function_for_testing
      .unwrap_or(default_time_function);
    time_function()
  }

  /// 对应原 `priority_from_index`。
  fn priority_from_index(&self, i: i32) -> Priority {
    if self.priority_mode == PriorityMode::DontApply {
      return Priority::Default;
    }
    match i {
      0 => Priority::BestEffort,
      1 => Priority::UserVisible,
      2 => Priority::UserBlocking,
      // 对应原 UNREACHABLE()：越界索引按 Default 兜底。
      _ => Priority::Default,
    }
  }

  /// 对应原 `priority_to_index`。
  fn priority_to_index(&self, priority: TaskPriority) -> usize {
    if self.priority_mode == PriorityMode::DontApply {
      return 0;
    }
    priority as u8 as usize
  }

  /// 对应原 `num_worker_runners`。
  fn worker_runner_count(priority_mode: PriorityMode) -> usize {
    if priority_mode == PriorityMode::DontApply {
      1
    } else {
      MAX_PRIORITY as u8 as usize + 1
    }
  }

  fn num_worker_runners(&self) -> usize {
    Self::worker_runner_count(self.priority_mode)
  }

  fn lock(&self) -> v8_base_platform::MutexGuard<'_> {
    self.lock.lock_guard()
  }
}

impl Platform for DefaultPlatform {
  /// 对应原 `NumberOfWorkerThreads`。
  fn number_of_worker_threads(&self) -> i32 {
    self.thread_pool_size
  }

  fn get_foreground_task_runner(
    &self,
    isolate: IsolateKey,
    _priority: TaskPriority,
  ) -> Arc<dyn TaskRunner> {
    self.foreground_task_runner(isolate)
  }

  /// 对应原 `IdleTasksEnabled`（原版忽略 isolate 参数）。
  fn idle_tasks_enabled(&self, _isolate: IsolateKey) -> bool {
    self.idle_task_support == IdleTaskSupport::Enabled
  }

  fn post_task_on_worker_thread(
    &self,
    priority: TaskPriority,
    task: BoxTask,
    _location: SourceLocation,
  ) -> Result<()> {
    self.post_task_on_worker_thread_impl(priority, task)
  }

  fn post_delayed_task_on_worker_thread(
    &self,
    priority: TaskPriority,
    task: BoxTask,
    delay_in_seconds: f64,
    _location: SourceLocation,
  ) -> Result<()> {
    self.post_delayed_task_on_worker_thread_impl(priority, task, delay_in_seconds)
  }

  fn monotonically_increasing_time(&self) -> f64 {
    DefaultPlatform::monotonically_increasing_time(self)
  }

  /// 对应原 `CurrentClockTimeMillis`。
  fn current_clock_time_millis(&self) -> f64 {
    OS::time_current_millis()
  }

  fn get_tracing_controller(&self) -> Option<Arc<dyn TracingController>> {
    let _guard = self.lock();
    // SAFETY: 已持有 `lock`。
    unsafe { &*self.state.get() }.tracing_controller.clone()
  }

  /// 对应原 `GetPageAllocator`。
  fn get_page_allocator(&self) -> &(dyn PageAllocator + Send + Sync) {
    &self.page_allocator
  }
}

impl Drop for DefaultPlatform {
  /// 对应原析构：终止全部后台与前台运行器。
  fn drop(&mut self) {
    let _guard = self.lock.lock_guard();
    // SAFETY: 已持有 `lock`；`Drop` 独占 `&mut self`。
    let state = unsafe { &mut *self.state.get() };
    if state
      .worker_threads_task_runners
      .first()
      .is_some_and(Option::is_some)
    {
      for runner in state.worker_threads_task_runners[..self.num_worker_runners()]
        .iter()
        .flatten()
      {
        runner.terminate();
      }
    }
    for runner in state.foreground_task_runner_map.values() {
      runner.terminate();
    }
  }
}

/// 对应原匿名命名空间 `GetActualThreadPoolSize`。
fn get_actual_thread_pool_size(thread_pool_size: i32) -> i32 {
  debug_assert!(thread_pool_size >= 0);
  let size = if thread_pool_size < 1 {
    number_of_processors() - 1
  } else {
    thread_pool_size
  };
  size.clamp(1, K_MAX_THREAD_POOL_SIZE)
}

/// 对应原 `NewDefaultPlatform`（`InProcessStackDumping::Enabled` 的 base::debug
/// 栈打印未就位：降级为日志登记）。
#[must_use]
pub fn new_default_platform(options: PlatformOptions) -> Arc<DefaultPlatform> {
  if options.in_process_stack_dumping == InProcessStackDumping::Enabled {
    log::warn!("InProcessStackDumping::Enabled：base::debug 栈打印族未接入（推迟登记）");
  }
  let mut options = options;
  options.thread_pool_size = get_actual_thread_pool_size(options.thread_pool_size);
  DefaultPlatform::new(options)
}

/// 对应原 `NewSingleThreadedDefaultPlatform`：线程池固定 0（禁用 worker 池）。
#[must_use]
pub fn new_single_threaded_default_platform(options: PlatformOptions) -> Arc<DefaultPlatform> {
  let mut options = options;
  options.thread_pool_size = 0;
  DefaultPlatform::new(options)
}
