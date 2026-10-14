//! 对应 `v8/test/unittests/tasks/cancelable-tasks-unittest.cc`
//! (TEST_F CancelableTaskManagerTest)，规约见 tests/SPEC.md。

use std::{
  hint::spin_loop,
  sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
  },
};

use aok::{OK, Void};
use log::info;
use v8_base_platform::{Options, Thread};
use v8_tasks::{
  Cancelable, CancelableIdleTask, CancelableTask, CancelableTaskManager, Error, INVALID_TASK_ID,
  IdleTask, IdleTaskBody, Status, Task, TaskBody, TaskId, TryAbortResult,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

type ResultType = Arc<AtomicU64>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
  DoNothing,
  WaitTillCancelTriggered,
  CheckNotRun,
}

struct TestTask {
  cancelable: Cancelable,
  result: ResultType,
  ran: Arc<AtomicBool>,
  triggered: Arc<AtomicBool>,
  mode: Mode,
}

impl TestTask {
  fn new(fx: &Fixture, result: ResultType, ran: Arc<AtomicBool>, mode: Mode) -> Self {
    // 对应原构造：Cancelable(test->manager())，Register 折叠进 Cancelable::new。
    Self {
      cancelable: Cancelable::new(&fx.manager).expect("fresh manager register"),
      result,
      ran,
      triggered: fx.triggered.clone(),
      mode,
    }
  }

  fn id(&self) -> TaskId {
    self.cancelable.id()
  }

  fn run(&mut self) {
    if !self.cancelable.try_run() {
      return;
    }
    self.result.store(self.cancelable.id(), Ordering::Release);
    match self.mode {
      Mode::DoNothing => {}
      Mode::WaitTillCancelTriggered => {
        // 原版忙等：自旋于原子进度量（事件同步，非墙钟）。
        while !self.triggered.load(Ordering::Acquire) {
          spin_loop();
        }
      }
      Mode::CheckNotRun => {
        // 原 EXPECT_TRUE(false)：被执行即失败，以标志位承接（测试禁 panic 噪声）。
        self.ran.store(true, Ordering::Release);
      }
    }
  }
}

/// 对应原 SequentialRunner。
struct SequentialRunner {
  task: Option<TestTask>,
  task_id: TaskId,
}

impl SequentialRunner {
  fn new(task: TestTask) -> Self {
    let task_id = task.id();
    Self {
      task: Some(task),
      task_id,
    }
  }

  fn run(&mut self) {
    // 对应 task_->Run(); task_.reset()：take 后立即析构。
    if let Some(mut task) = self.task.take() {
      task.run();
    }
  }

  fn task_id(&self) -> TaskId {
    self.task_id
  }
}

/// 对应原 ThreadedRunner（继承 base::Thread，"runner thread"）。
struct ThreadedRunner {
  task_id: TaskId,
  thread: Option<Thread>,
}

impl ThreadedRunner {
  fn new(task: TestTask) -> Self {
    let task_id = task.id();
    let mut task = Some(task);
    let thread = Thread::new(Options::new("runner thread"), move || {
      if let Some(mut task) = task.take() {
        task.run();
      }
    });
    Self {
      task_id,
      thread: Some(thread),
    }
  }

  fn start(&mut self) {
    self
      .thread
      .as_mut()
      .expect("thread alive")
      .start()
      .expect("start runner thread");
  }

  fn join(&mut self) {
    if let Some(mut thread) = self.thread.take() {
      thread.join().expect("join runner thread");
    }
  }

  fn task_id(&self) -> TaskId {
    self.task_id
  }
}

/// 对应原 fixture CancelableTaskManagerTest。
struct Fixture {
  manager: Arc<CancelableTaskManager>,
  triggered: Arc<AtomicBool>,
}

impl Fixture {
  fn new() -> Self {
    Self {
      manager: Arc::new(CancelableTaskManager::new()),
      triggered: Arc::new(AtomicBool::new(false)),
    }
  }

  fn new_task(&self, result: ResultType, mode: Mode) -> (TestTask, Arc<AtomicBool>) {
    let ran = Arc::new(AtomicBool::new(false));
    (TestTask::new(self, result, ran.clone(), mode), ran)
  }

  fn cancel_and_wait(&self) {
    self.triggered.store(true, Ordering::Release);
    self.manager.cancel_and_wait();
  }

  fn try_abort_all(&self) -> TryAbortResult {
    self.triggered.store(true, Ordering::Release);
    self.manager.try_abort_all()
  }
}

fn wait_started(result: &ResultType) {
  // 对应原忙等：等两侧任务确认进入执行。
  while result.load(Ordering::Acquire) == 0 {
    spin_loop();
  }
}

#[test]
fn empty_cancelable_task_manager() -> Void {
  let fx = Fixture::new();
  fx.cancel_and_wait();
  OK
}

#[test]
fn sequential_cancel_and_wait() -> Void {
  let fx = Fixture::new();
  let result1: ResultType = Arc::new(AtomicU64::new(0));
  let (task, ran) = fx.new_task(result1.clone(), Mode::CheckNotRun);
  let mut runner1 = SequentialRunner::new(task);
  assert_eq!(0u64, result1.load(Ordering::Acquire));
  fx.cancel_and_wait();
  assert_eq!(0u64, result1.load(Ordering::Acquire));
  runner1.run();
  assert_eq!(0u64, result1.load(Ordering::Acquire));
  assert!(!ran.load(Ordering::Acquire), "kCheckNotRun 任务体不得执行");
  info!("sequential_cancel_and_wait ok");
  OK
}

#[test]
fn sequential_multiple_tasks() -> Void {
  let fx = Fixture::new();
  let result1: ResultType = Arc::new(AtomicU64::new(0));
  let result2: ResultType = Arc::new(AtomicU64::new(0));
  let (task1, _r1) = fx.new_task(result1.clone(), Mode::DoNothing);
  let (task2, _r2) = fx.new_task(result2.clone(), Mode::DoNothing);
  let mut runner1 = SequentialRunner::new(task1);
  let mut runner2 = SequentialRunner::new(task2);
  assert_eq!(1u64, runner1.task_id());
  assert_eq!(2u64, runner2.task_id());

  assert_eq!(0u64, result1.load(Ordering::Acquire));
  runner1.run();
  assert_eq!(1u64, result1.load(Ordering::Acquire));

  assert_eq!(0u64, result2.load(Ordering::Acquire));
  runner2.run();
  assert_eq!(2u64, result2.load(Ordering::Acquire));

  fx.cancel_and_wait();
  assert_eq!(TryAbortResult::TaskRemoved, fx.manager.try_abort(1)?);
  assert_eq!(TryAbortResult::TaskRemoved, fx.manager.try_abort(2)?);
  OK
}

#[test]
fn threaded_multiple_tasks_started() -> Void {
  let fx = Fixture::new();
  let result1: ResultType = Arc::new(AtomicU64::new(0));
  let result2: ResultType = Arc::new(AtomicU64::new(0));
  let (task1, _r1) = fx.new_task(result1.clone(), Mode::WaitTillCancelTriggered);
  let (task2, _r2) = fx.new_task(result2.clone(), Mode::WaitTillCancelTriggered);
  let mut runner1 = ThreadedRunner::new(task1);
  let mut runner2 = ThreadedRunner::new(task2);
  runner1.start();
  runner2.start();
  // 对齐原版：确认两任务均已进入执行再触发取消收尾。
  while result1.load(Ordering::Acquire) == 0 || result2.load(Ordering::Acquire) == 0 {
    spin_loop();
  }
  fx.cancel_and_wait();
  runner1.join();
  runner2.join();
  assert_eq!(1u64, result1.load(Ordering::Acquire));
  assert_eq!(2u64, result2.load(Ordering::Acquire));
  OK
}

#[test]
fn threaded_multiple_tasks_not_run() -> Void {
  let fx = Fixture::new();
  let result1: ResultType = Arc::new(AtomicU64::new(0));
  let result2: ResultType = Arc::new(AtomicU64::new(0));
  let (task1, ran1) = fx.new_task(result1.clone(), Mode::CheckNotRun);
  let (task2, ran2) = fx.new_task(result2.clone(), Mode::CheckNotRun);
  let mut runner1 = ThreadedRunner::new(task1);
  let mut runner2 = ThreadedRunner::new(task2);
  fx.cancel_and_wait();
  // 任务已被取消：runner 空转退出，不更新 result。
  runner1.start();
  runner2.start();
  runner1.join();
  runner2.join();
  assert_eq!(0u64, result1.load(Ordering::Acquire));
  assert_eq!(0u64, result2.load(Ordering::Acquire));
  assert!(!ran1.load(Ordering::Acquire));
  assert!(!ran2.load(Ordering::Acquire));
  OK
}

#[test]
fn remove_before_cancel_and_wait() -> Void {
  let fx = Fixture::new();
  let result1: ResultType = Arc::new(AtomicU64::new(0));
  let (task, ran) = fx.new_task(result1.clone(), Mode::CheckNotRun);
  let mut runner1 = ThreadedRunner::new(task);
  let id = runner1.task_id();
  assert_eq!(1u64, id);
  assert_eq!(TryAbortResult::TaskAborted, fx.manager.try_abort(id)?);
  runner1.start();
  runner1.join();
  fx.cancel_and_wait();
  assert_eq!(0u64, result1.load(Ordering::Acquire));
  assert!(!ran.load(Ordering::Acquire));
  OK
}

#[test]
fn remove_after_cancel_and_wait() -> Void {
  let fx = Fixture::new();
  let result1: ResultType = Arc::new(AtomicU64::new(0));
  let (task, _ran) = fx.new_task(result1.clone(), Mode::DoNothing);
  let mut runner1 = ThreadedRunner::new(task);
  let id = runner1.task_id();
  assert_eq!(1u64, id);
  runner1.start();
  runner1.join();
  fx.cancel_and_wait();
  assert_eq!(TryAbortResult::TaskRemoved, fx.manager.try_abort(id)?);
  assert_eq!(1u64, result1.load(Ordering::Acquire));
  OK
}

#[test]
fn remove_unmanaged_id() -> Void {
  let fx = Fixture::new();
  assert_eq!(TryAbortResult::TaskRemoved, fx.manager.try_abort(1)?);
  assert_eq!(TryAbortResult::TaskRemoved, fx.manager.try_abort(2)?);
  fx.cancel_and_wait();
  assert_eq!(TryAbortResult::TaskRemoved, fx.manager.try_abort(1)?);
  assert_eq!(TryAbortResult::TaskRemoved, fx.manager.try_abort(3)?);
  OK
}

#[test]
fn empty_try_abort_all() -> Void {
  let fx = Fixture::new();
  assert_eq!(TryAbortResult::TaskRemoved, fx.try_abort_all());
  fx.cancel_and_wait();
  OK
}

#[test]
fn threaded_multiple_tasks_not_run_try_abort_all() -> Void {
  let fx = Fixture::new();
  let result1: ResultType = Arc::new(AtomicU64::new(0));
  let result2: ResultType = Arc::new(AtomicU64::new(0));
  let (task1, ran1) = fx.new_task(result1.clone(), Mode::CheckNotRun);
  let (task2, ran2) = fx.new_task(result2.clone(), Mode::CheckNotRun);
  let mut runner1 = ThreadedRunner::new(task1);
  let mut runner2 = ThreadedRunner::new(task2);
  assert_eq!(TryAbortResult::TaskAborted, fx.try_abort_all());
  // 任务已被取消：runner 空转退出，不更新 result。
  runner1.start();
  runner2.start();
  runner1.join();
  runner2.join();
  assert_eq!(0u64, result1.load(Ordering::Acquire));
  assert_eq!(0u64, result2.load(Ordering::Acquire));
  assert!(!ran1.load(Ordering::Acquire));
  assert!(!ran2.load(Ordering::Acquire));
  fx.cancel_and_wait();
  OK
}

#[test]
fn threaded_multiple_tasks_started_try_abort_all() -> Void {
  let fx = Fixture::new();
  let result1: ResultType = Arc::new(AtomicU64::new(0));
  let result2: ResultType = Arc::new(AtomicU64::new(0));
  let (task1, _r1) = fx.new_task(result1.clone(), Mode::WaitTillCancelTriggered);
  let (task2, _r2) = fx.new_task(result2.clone(), Mode::WaitTillCancelTriggered);
  let mut runner1 = ThreadedRunner::new(task1);
  let mut runner2 = ThreadedRunner::new(task2);
  runner1.start();
  wait_started(&result1);
  // 对应原版 AnyOf：任务恰好抢先结束则 kTaskAborted，否则 kTaskRunning。
  let outcome = fx.try_abort_all();
  assert!(outcome == TryAbortResult::TaskAborted || outcome == TryAbortResult::TaskRunning);
  runner2.start();
  runner1.join();
  runner2.join();
  assert_eq!(1u64, result1.load(Ordering::Acquire));
  assert_eq!(0u64, result2.load(Ordering::Acquire));
  fx.cancel_and_wait();
  OK
}

/// 原版头注释 cancelable-task.h:41-42：CancelAndWait 后 Register 立即取消新任务。
#[test]
fn register_after_cancel_and_wait() -> Void {
  let fx = Fixture::new();
  fx.cancel_and_wait();
  let task = Cancelable::new(&fx.manager).expect("register after cancel is infallible");
  assert_eq!(INVALID_TASK_ID, task.id());
  assert!(!task.try_run());
  OK
}

/// 原版 `CHECK_NE(kInvalidTaskId, id)` 的 Result 化改写。
#[test]
fn try_abort_invalid_id() -> Void {
  let manager = CancelableTaskManager::new();
  assert!(matches!(
    manager.try_abort(INVALID_TASK_ID),
    Err(Error::InvalidTaskId)
  ));
  manager.cancel_and_wait();
  OK
}

/// 规约 13a：任务 ID 计数器达到 u64::MAX 时的溢出保护与防 ABA 钉死。
#[test]
fn task_id_counter_overflow() -> Void {
  let manager = CancelableTaskManager::new();
  manager.set_task_id_counter_for_testing(u64::MAX - 1);
  let t1 = Cancelable::new(&manager)?;
  assert_eq!(u64::MAX, t1.id());
  assert!(matches!(
    Cancelable::new(&manager),
    Err(Error::TaskIdOverflow)
  ));
  assert!(matches!(
    Cancelable::new(&manager),
    Err(Error::TaskIdOverflow)
  ));
  drop(t1);
  manager.cancel_and_wait();
  OK
}

/// 原 `CompareExchangeStatus` 的 `*previous` 语义（cancelable-task.h:128-136）：
/// 成功时 previous 保持交换前传入值，失败时取回实际观察值。
#[test]
fn compare_exchange_previous_semantics() -> Void {
  let manager = CancelableTaskManager::new();
  let c = Cancelable::new(&manager)?;
  let mut prev = Status::Waiting;
  assert!(c.compare_exchange_status(&mut prev, Status::Running));
  assert_eq!(Status::Waiting, prev, "成功时 previous 应为交换前原值");
  assert!(!c.compare_exchange_status(&mut prev, Status::Waiting));
  assert_eq!(Status::Running, prev, "失败时 previous 应取回实际值");
  drop(c);
  manager.cancel_and_wait();
  OK
}

struct CountingBody {
  hits: Arc<AtomicU64>,
  last: Arc<AtomicU64>,
}

impl TaskBody for CountingBody {
  fn run_internal(&mut self) {
    self.hits.fetch_add(1, Ordering::Relaxed);
  }
}

impl IdleTaskBody for CountingBody {
  fn run_internal(&mut self, deadline_in_seconds: f64) {
    self.hits.fetch_add(1, Ordering::Relaxed);
    self
      .last
      .store(deadline_in_seconds.to_bits(), Ordering::Relaxed);
  }
}

#[test]
fn cancelable_task_run_guard() -> Void {
  let manager = CancelableTaskManager::new();
  let hits = Arc::new(AtomicU64::new(0));
  {
    // 任务对象须在 cancel_and_wait 前析构（对应原版平台执行后即销毁）。
    let mut task = CancelableTask::new(
      &manager,
      CountingBody {
        hits: hits.clone(),
        last: hits.clone(),
      },
    )?;
    task.run();
    assert_eq!(1u64, hits.load(Ordering::Relaxed));
    // 已执行（Running 态）后二次 run 被 TryRun 拦住，对应 Run() final 语义。
    task.run();
    assert_eq!(1u64, hits.load(Ordering::Relaxed));
  }

  let hits2 = Arc::new(AtomicU64::new(0));
  let mut canceled = CancelableTask::new(
    &manager,
    CountingBody {
      hits: hits2.clone(),
      last: hits2.clone(),
    },
  )?;
  assert_eq!(
    TryAbortResult::TaskAborted,
    manager.try_abort(canceled.id())?
  );
  canceled.run();
  assert_eq!(0u64, hits2.load(Ordering::Relaxed));
  drop(canceled);
  manager.cancel_and_wait();
  OK
}

#[test]
fn cancelable_idle_task_deadline() -> Void {
  let manager = CancelableTaskManager::new();
  let hits = Arc::new(AtomicU64::new(0));
  let last = Arc::new(AtomicU64::new(0));
  {
    let mut task = CancelableIdleTask::new(
      &manager,
      CountingBody {
        hits: hits.clone(),
        last: last.clone(),
      },
    )?;
    let deadline = 12.5f64;
    task.run(deadline);
    assert_eq!(1u64, hits.load(Ordering::Relaxed));
    assert_eq!(deadline.to_bits(), last.load(Ordering::Relaxed));
  }
  manager.cancel_and_wait();
  OK
}
