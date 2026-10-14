//! 对应 `v8/test/unittests/libplatform/default-platform-unittest.cc` 1:1 迁移
//! （前台消息泵族；`CustomDefaultPlatformTest.RunBackgroundTask` 依赖 LP-B
//! worker 线程池，登记推迟 `#[ignore]`）。

use std::{
  cell::Cell,
  sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
  },
};

use aok::{OK, Void};
use log::info;
use v8_libplatform::{
  BoxIdleTask, BoxTask, DefaultPlatform, Error, IdleTaskSupport, IsolateKey, MessageLoopBehavior,
  Platform, PlatformOptions, SourceLocation, TaskRunner,
};
use v8_tasks::{IdleTask, Task, TaskPriority};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

thread_local! {
  /// 对应原版 `DefaultPlatformWithMockTime::mock_time_`。
  static MOCK_TIME: Cell<f64> = const { Cell::new(0.0) };
}

fn mock_time() -> f64 {
  MOCK_TIME.with(Cell::get)
}

fn reset_time() {
  MOCK_TIME.with(|c| c.set(0.0));
}

/// 对应原版 `IncreaseTime`。
fn increase_time(seconds: f64) {
  MOCK_TIME.with(|c| c.set(c.get() + seconds));
}

/// 对应原版 `MockTask`（Run 记录执行序并可触发嵌套泵；析构即 Die）。
struct MockTask {
  id: u32,
  order: Arc<Mutex<Vec<u32>>>,
  die: Arc<AtomicUsize>,
  hook: Option<Box<dyn FnOnce() + Send>>,
}

impl Task for MockTask {
  fn run(&mut self) {
    if let Ok(mut v) = self.order.lock() {
      v.push(self.id);
    }
    if let Some(hook) = self.hook.take() {
      hook();
    }
  }
}

impl Drop for MockTask {
  fn drop(&mut self) {
    self.die.fetch_add(1, Ordering::Relaxed);
  }
}

/// 对应原版 `MockIdleTask`（Run 记录 deadline；析构即 Die）。
struct MockIdleTask {
  deadlines: Arc<Mutex<Vec<f64>>>,
  die: Arc<AtomicUsize>,
}

impl IdleTask for MockIdleTask {
  fn run(&mut self, deadline_in_seconds: f64) {
    if let Ok(mut v) = self.deadlines.lock() {
      v.push(deadline_in_seconds);
    }
  }
}

impl Drop for MockIdleTask {
  fn drop(&mut self) {
    self.die.fetch_add(1, Ordering::Relaxed);
  }
}

/// 对应原版 fixture `PlatformTest`：mock 时钟 + idle 使能的默认平台。
struct Fixture {
  order: Arc<Mutex<Vec<u32>>>,
  die: Arc<AtomicUsize>,
  deadlines: Arc<Mutex<Vec<f64>>>,
  idle_die: Arc<AtomicUsize>,
  isolate: IsolateKey,
}

impl Fixture {
  fn new(isolate: IsolateKey) -> Self {
    reset_time();
    Self {
      order: Arc::new(Mutex::new(Vec::new())),
      die: Arc::new(AtomicUsize::new(0)),
      deadlines: Arc::new(Mutex::new(Vec::new())),
      idle_die: Arc::new(AtomicUsize::new(0)),
      isolate,
    }
  }

  /// 对应原版 `DefaultPlatformWithMockTime(thread_pool_size)`。
  fn platform(&self, thread_pool_size: i32) -> Arc<DefaultPlatform> {
    let platform = DefaultPlatform::new(PlatformOptions {
      thread_pool_size,
      idle_task_support: IdleTaskSupport::Enabled,
      ..Default::default()
    });
    platform.set_time_function_for_testing(mock_time);
    platform
  }

  fn task(&self, id: u32) -> BoxTask {
    Box::new(MockTask {
      id,
      order: self.order.clone(),
      die: self.die.clone(),
      hook: None,
    })
  }

  fn idle_task(&self) -> BoxIdleTask {
    Box::new(MockIdleTask {
      deadlines: self.deadlines.clone(),
      die: self.idle_die.clone(),
    })
  }

  fn order(&self) -> Vec<u32> {
    self.order.lock().expect("order 锁不应中毒").clone()
  }

  fn die_count(&self) -> usize {
    self.die.load(Ordering::Relaxed)
  }

  /// 对应原版 `task_runner()` 懒建。
  fn runner(&self, platform: &Arc<DefaultPlatform>) -> Arc<dyn TaskRunner> {
    platform.get_foreground_task_runner(self.isolate, TaskPriority::UserBlocking)
  }

  /// 对应原版 `PumpMessageLoop()`（默认 kDoNotWait）。
  fn pump(&self, platform: &DefaultPlatform) -> bool {
    platform.pump_message_loop(self.isolate, MessageLoopBehavior::DoNotWait)
  }
}

/// 对应原 `TEST_F(DefaultPlatformTest, PumpMessageLoop)`。
#[test]
fn pump_message_loop() -> Void {
  info!("> pump_message_loop");
  let fx = Fixture::new(0x1001);
  let platform = fx.platform(0);
  assert!(!fx.pump(&platform), "运行器未建时泵应 false");

  let runner = fx.runner(&platform);
  runner.post_task(fx.task(1), SourceLocation::current());
  assert!(fx.pump(&platform));
  assert_eq!(fx.order(), vec![1]);
  assert!(!fx.pump(&platform));
  assert_eq!(fx.die_count(), 1, "任务跑完即 Die");
  OK
}

/// 对应原 `TEST_F(DefaultPlatformTest, PumpMessageLoopWithTaskRunner)`。
#[test]
fn pump_message_loop_with_task_runner() -> Void {
  info!("> pump_message_loop_with_task_runner");
  let fx = Fixture::new(0x1002);
  let platform = fx.platform(0);
  let runner = fx.runner(&platform);
  assert!(!fx.pump(&platform));

  runner.post_task(fx.task(1), SourceLocation::current());
  assert!(fx.pump(&platform));
  assert!(!fx.pump(&platform));
  assert_eq!(fx.order(), vec![1]);
  OK
}

/// 对应原 `TEST_F(DefaultPlatformTest, PumpMessageLoopNested)`：
/// nestable FIFO、non-nestable FIFO；嵌套泵内只跑 nestable（期望序 1,3,2,4）。
#[test]
fn pump_message_loop_nested() -> Void {
  info!("> pump_message_loop_nested");
  let fx = Fixture::new(0x1003);
  let platform = fx.platform(0);
  let runner = fx.runner(&platform);
  assert!(!fx.pump(&platform));

  // 对应原版 WillOnce([this]{ EXPECT_TRUE(PumpMessageLoop()); })。
  let inner_platform = platform.clone();
  let inner_isolate = fx.isolate;
  let nestable_task1 = Box::new(MockTask {
    id: 1,
    order: fx.order.clone(),
    die: fx.die.clone(),
    hook: Some(Box::new(move || {
      let pumped = inner_platform.pump_message_loop(inner_isolate, MessageLoopBehavior::DoNotWait);
      assert!(pumped, "嵌套泵应取到 nestable_task3");
    })),
  }) as BoxTask;
  runner.post_task(nestable_task1, SourceLocation::current());
  runner.post_non_nestable_task(fx.task(2), SourceLocation::current());
  runner.post_task(fx.task(3), SourceLocation::current());
  runner.post_non_nestable_task(fx.task(4), SourceLocation::current());

  assert!(fx.pump(&platform)); // 跑 1，嵌套跑 3
  assert_eq!(fx.order(), vec![1, 3]);
  assert!(fx.pump(&platform)); // 跑 2
  assert!(fx.pump(&platform)); // 跑 4
  assert_eq!(fx.order(), vec![1, 3, 2, 4]);
  assert!(!fx.pump(&platform));
  assert_eq!(fx.die_count(), 4);
  OK
}

/// 对应原 `TEST_F(DefaultPlatformTestWithMockTime, PumpMessageLoopDelayed)`。
#[test]
fn pump_message_loop_delayed() -> Void {
  info!("> pump_message_loop_delayed");
  let fx = Fixture::new(0x1004);
  let platform = fx.platform(0);
  let runner = fx.runner(&platform);
  assert!(!fx.pump(&platform));

  runner.post_delayed_task(fx.task(2), 100.0, SourceLocation::current());
  runner.post_delayed_task(fx.task(1), 10.0, SourceLocation::current());
  assert!(!fx.pump(&platform));

  increase_time(11.0);
  assert!(fx.pump(&platform));
  assert_eq!(fx.order(), vec![1]);
  assert!(!fx.pump(&platform));

  increase_time(90.0);
  assert!(fx.pump(&platform));
  assert_eq!(fx.order(), vec![1, 2]);
  OK
}

/// 对应原 `TEST_F(DefaultPlatformTestWithMockTime, PumpMessageLoopNoStarvation)`。
#[test]
fn pump_message_loop_no_starvation() -> Void {
  info!("> pump_message_loop_no_starvation");
  let fx = Fixture::new(0x1005);
  let platform = fx.platform(0);
  let runner = fx.runner(&platform);
  assert!(!fx.pump(&platform));

  runner.post_task(fx.task(1), SourceLocation::current());
  runner.post_delayed_task(fx.task(2), 10.0, SourceLocation::current());
  increase_time(11.0);

  assert!(fx.pump(&platform));
  runner.post_task(fx.task(3), SourceLocation::current());
  assert!(fx.pump(&platform));
  assert!(fx.pump(&platform));
  assert_eq!(fx.order(), vec![1, 2, 3]);
  OK
}

/// 对应原 `PendingDelayedTasksAreDestroyedOnShutdown`。
#[test]
fn pending_delayed_tasks_destroyed_on_shutdown() -> Void {
  info!("> pending_delayed_tasks_destroyed_on_shutdown");
  let fx = Fixture::new(0x1006);
  {
    let platform = fx.platform(0);
    let runner = fx.runner(&platform);
    runner.post_delayed_task(fx.task(1), 10.0, SourceLocation::current());
    assert_eq!(fx.die_count(), 0);
  }
  assert_eq!(fx.die_count(), 1, "平台析构应终止运行器并销毁滞留延迟任务");
  OK
}

/// 对应原 `TEST_F(DefaultPlatformTestWithMockTime, RunIdleTasks)`：
/// deadline = 当前 mock 时间 + 空闲预算。
#[test]
fn run_idle_tasks() -> Void {
  info!("> run_idle_tasks");
  let fx = Fixture::new(0x1007);
  let platform = fx.platform(0);
  let runner = fx.runner(&platform);
  runner.post_idle_task(fx.idle_task(), SourceLocation::current());

  increase_time(23.0);
  platform.run_idle_tasks(fx.isolate, 42.0);
  assert_eq!(
    *fx.deadlines.lock().expect("deadlines 锁不应中毒"),
    vec![42.0 + 23.0]
  );
  assert_eq!(fx.idle_die.load(Ordering::Relaxed), 1);
  OK
}

/// 对应原 `PendingIdleTasksAreDestroyedOnShutdown`。
#[test]
fn pending_idle_tasks_destroyed_on_shutdown() -> Void {
  info!("> pending_idle_tasks_destroyed_on_shutdown");
  let fx = Fixture::new(0x1008);
  {
    let platform = fx.platform(0);
    let runner = fx.runner(&platform);
    runner.post_idle_task(fx.idle_task(), SourceLocation::current());
    assert_eq!(fx.idle_die.load(Ordering::Relaxed), 0);
  }
  assert_eq!(fx.idle_die.load(Ordering::Relaxed), 1);
  OK
}

/// 对应原 `TEST(CustomDefaultPlatformTest, PostForegroundTaskAfterPlatformTermination)`：
/// 平台销毁后仍可向前台运行器投递，任务即弃不执行。
#[test]
fn post_foreground_task_after_platform_termination() -> Void {
  info!("> post_foreground_task_after_platform_termination");
  let fx = Fixture::new(0x1009);
  let runner = {
    let platform = fx.platform(1);
    fx.runner(&platform)
  };

  runner.post_task(fx.task(1), SourceLocation::current());
  runner.post_delayed_task(fx.task(2), 10.0, SourceLocation::current());
  runner.post_idle_task(fx.idle_task(), SourceLocation::current());
  assert_eq!(fx.order(), Vec::<u32>::new(), "终止后任务不得执行");
  assert_eq!(fx.die_count(), 2, "投递即 Die");
  assert_eq!(fx.idle_die.load(Ordering::Relaxed), 1);
  OK
}

/// 对应原 `TEST(CustomDefaultPlatformTest, RunBackgroundTask)`：
/// 依赖 LP-B worker 线程池（DefaultWorkerThreadsTaskRunner/WorkerThread），
/// 本波推迟登记，不硬跑。
#[test]
#[ignore = "LP-B worker_threads_task_runner 族未就位（窄 trait WorkerRunnerFactory 衔接后启用）"]
fn run_background_task() -> Void {
  let fx = Fixture::new(0x100a);
  let _platform = fx.platform(1);
  // TODO(LP-B): 注入 worker_runner_factory 后经 Platform::post_task_on_worker_thread
  // 投递、以 Semaphore 等待执行的语义比对原版用例。
  OK
}

/// LP-A 衔接面补充：LP-B 工厂缺位时 worker 投递返回窄错误（对应原 DCHECK_NOT_NULL）。
#[test]
fn worker_post_without_lp_b_returns_err() -> Void {
  info!("> worker_post_without_lp_b_returns_err");
  let fx = Fixture::new(0x100b);
  let platform = fx.platform(0);
  let err = platform
    .post_task_on_worker_thread(
      TaskPriority::UserVisible,
      fx.task(1),
      SourceLocation::current(),
    )
    .expect_err("工厂缺位应返回 Err");
  assert!(
    matches!(
      err,
      Error::WorkerRunnerUnavailable {
        priority: TaskPriority::UserVisible
      }
    ),
    "错误变体应为 WorkerRunnerUnavailable"
  );
  assert_eq!(fx.die_count(), 1, "Err 路径任务应被销毁");
  assert_eq!(platform.number_of_worker_threads(), 0);
  assert!(platform.idle_tasks_enabled(fx.isolate));
  // 前台运行器随平台使能 idle。
  assert!(
    platform
      .get_foreground_task_runner(fx.isolate, TaskPriority::BestEffort)
      .idle_tasks_enabled()
  );
  assert!(platform.current_clock_time_millis() > 0.0);
  assert!(platform.monotonically_increasing_time() >= 0.0);
  OK
}
