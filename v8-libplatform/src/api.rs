//! libplatform 对外接口面转写：对应 `v8/include/v8-platform.h` 的 TaskRunner /
//! Platform / TracingController 接口与 `include/libplatform/libplatform.h` 的枚举。
//!
//! 与原版差异（登记于 README.md 双段）：
//! - `Isolate*` 裸指针键 → `IsolateKey`（usize 不透明句柄）；
//! - `std::unique_ptr<Task>` → `BoxTask`（`Box<dyn Task + Send>`，任务跨线程移交）；
//! - Job/`CreateJobImpl` 面属 LP-B default-job 族，本波不定义（推迟登记）；
//! - `SourceLocation::Current()` 的函数名在 Rust `Location::caller()` 无对应，置空串。

use std::{fmt, panic::Location as CallerLocation, sync::Arc};

use v8_base::page_allocator::PageAllocator;
use v8_base_platform::Priority;
use v8_tasks::{IdleTask, Task, TaskPriority};

use crate::error::Result;

/// 单调时钟函数指针，对应原版 `using TimeFunction = double (*)()`（秒）。
pub type TimeFunction = fn() -> f64;

/// `v8::Isolate*` 的消息循环键：Rust 侧以不透明 usize 承接（原版用裸指针作 map 键）。
pub type IsolateKey = usize;

/// 任务堆指针，对应 `std::unique_ptr<v8::Task>`；`Send` 约束源于任务跨线程移交。
pub type BoxTask = Box<dyn Task + Send>;

/// 空闲任务堆指针，对应 `std::unique_ptr<v8::IdleTask>`。
pub type BoxIdleTask = Box<dyn IdleTask + Send>;

/// 对应原版 `enum class IdleTaskSupport`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum IdleTaskSupport {
  #[default]
  Disabled,
  Enabled,
}

/// 对应原版 `enum class InProcessStackDumping`（进程内栈打印属 base::debug 族，推迟）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum InProcessStackDumping {
  #[default]
  Disabled,
  Enabled,
}

/// 对应原版 `enum class MessageLoopBehavior : bool`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum MessageLoopBehavior {
  #[default]
  DoNotWait,
  WaitForWork,
}

/// 对应原版 `enum class PriorityMode : bool`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum PriorityMode {
  #[default]
  DontApply,
  Apply,
}

/// 对应原版 `v8::SourceLocation`（`v8-source-location.h`，模仿 C++20 std::source_location）。
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct SourceLocation {
  function: &'static str,
  file: &'static str,
  line: usize,
}

impl SourceLocation {
  /// 对应 `SourceLocation::Current()`；Rust 无编译期函数名，`function` 置空串。
  #[must_use]
  #[track_caller]
  pub const fn current() -> Self {
    let loc = CallerLocation::caller();
    Self {
      function: "",
      file: loc.file(),
      line: loc.line() as usize,
    }
  }

  pub const fn function(&self) -> &'static str {
    self.function
  }

  pub const fn file_name(&self) -> &'static str {
    self.file
  }

  pub const fn line(&self) -> usize {
    self.line
  }

  /// 对应 `operator bool()`：line != 0 即已初始化。
  pub const fn is_valid(&self) -> bool {
    self.line != 0
  }
}

impl fmt::Display for SourceLocation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}:{} ({})", self.file, self.line, self.function)
  }
}

impl fmt::Debug for SourceLocation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(self, f)
  }
}

/// 对应原版 `class v8::TracingController` 的窄接柄：libplatform 仅做持有与传递，
/// 事件面方法属 LP-C tracing/ 族与 include API 层，本波不展开（推迟登记）。
pub trait TracingController: Send + Sync {}

/// 对应原版 `class v8::TaskRunner`：Post* 公开面与 Impl 面在 Rust 合并
/// （原版非虚公开方法仅转发 Impl，`location` 原版即未消费，保留形参以对齐签名）。
pub trait TaskRunner: Send + Sync {
  /// 对应 `PostTaskImpl`。
  fn post_task(&self, task: BoxTask, location: SourceLocation);

  /// 对应 `PostNonNestableTaskImpl`；要求 `non_nestable_tasks_enabled`。
  fn post_non_nestable_task(&self, task: BoxTask, location: SourceLocation);

  /// 对应 `PostDelayedTaskImpl`。
  fn post_delayed_task(&self, task: BoxTask, delay_in_seconds: f64, location: SourceLocation);

  /// 对应 `PostNonNestableDelayedTaskImpl`；要求 `non_nestable_delayed_tasks_enabled`。
  fn post_non_nestable_delayed_task(
    &self,
    task: BoxTask,
    delay_in_seconds: f64,
    location: SourceLocation,
  );

  /// 对应 `PostIdleTaskImpl`；要求 `idle_tasks_enabled`。
  fn post_idle_task(&self, task: BoxIdleTask, location: SourceLocation);

  /// 对应 `IdleTasksEnabled()`。
  fn idle_tasks_enabled(&self) -> bool;

  /// 对应 `NonNestableTasksEnabled()`，默认不支持（同原版默认实现）。
  fn non_nestable_tasks_enabled(&self) -> bool {
    false
  }

  /// 对应 `NonNestableDelayedTasksEnabled()`，默认不支持（同原版默认实现）。
  fn non_nestable_delayed_tasks_enabled(&self) -> bool {
    false
  }
}

/// 对应 `DefaultWorkerThreadsTaskRunner` 供 DefaultPlatform 消费的窄面：
/// LP-B `worker_threads_task_runner.rs` 落地后实现本 trait 并经 `WorkerRunnerFactory` 注入。
pub trait WorkerTaskRunner: Send + Sync {
  /// 对应 `PostTask`。
  fn post_task(&self, task: BoxTask);

  /// 对应 `PostDelayedTask`。
  fn post_delayed_task(&self, task: BoxTask, delay_in_seconds: f64);

  /// 对应 `Terminate`。
  fn terminate(&self);
}

/// 工作线程运行器工厂（LP-B 衔接点）：参数对齐原版构造入参
/// （线程池大小、时间函数、线程优先级）。
pub type WorkerRunnerFactory =
  fn(pool_size: i32, time_function: TimeFunction, priority: Priority) -> Arc<dyn WorkerTaskRunner>;

/// 对应原版 `class v8::Platform` 的 LP-A 消费面：Job / ThreadIsolatedAllocator /
/// StackTracePrinter / ZeroSegmentSize 分属 LP-B 与 base::debug、partition_alloc 平台分支，
/// 本波不定义（推迟登记）。worker 投递以 `Result` 承接原版 DCHECK_NOT_NULL。
pub trait Platform: Send + Sync {
  /// 对应 `NumberOfWorkerThreads()`。
  fn number_of_worker_threads(&self) -> i32;

  /// 对应 `GetForegroundTaskRunner(isolate, priority)`。
  fn get_foreground_task_runner(
    &self,
    isolate: IsolateKey,
    priority: TaskPriority,
  ) -> Arc<dyn TaskRunner>;

  /// 对应 `IdleTasksEnabled(isolate)`，默认关闭（同原版默认实现）。
  fn idle_tasks_enabled(&self, _isolate: IsolateKey) -> bool {
    false
  }

  /// 对应 `PostTaskOnWorkerThreadImpl`。
  fn post_task_on_worker_thread(
    &self,
    priority: TaskPriority,
    task: BoxTask,
    location: SourceLocation,
  ) -> Result<()>;

  /// 对应 `PostDelayedTaskOnWorkerThreadImpl`。
  fn post_delayed_task_on_worker_thread(
    &self,
    priority: TaskPriority,
    task: BoxTask,
    delay_in_seconds: f64,
    location: SourceLocation,
  ) -> Result<()>;

  /// 对应 `MonotonicallyIncreasingTime()`。
  fn monotonically_increasing_time(&self) -> f64;

  /// 对应 `CurrentClockTimeMillis()`。
  fn current_clock_time_millis(&self) -> f64;

  /// 对应 `GetTracingController()`；LP-C 缺位时可为 None（原版保证非空，偏差登记）。
  fn get_tracing_controller(&self) -> Option<Arc<dyn TracingController>>;

  /// 对应 `GetPageAllocator()`。
  fn get_page_allocator(&self) -> &(dyn PageAllocator + Send + Sync);
}
