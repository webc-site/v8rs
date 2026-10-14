//! `DefaultForegroundTaskRunner` 语义用例：best-effort 顺序 + NonNestable 嵌套语义
//! （对应 `default-platform-unittest.cc` 前台行为在运行器层的直测；
//! 原版该组件无独立 unittest，经 DefaultPlatform::PumpMessageLoop 覆盖）。

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
  BoxIdleTask, BoxTask, DefaultForegroundTaskRunner, IdleTaskSupport, MessageLoopBehavior,
  RunTaskScope, SourceLocation, TaskRunner,
};
use v8_tasks::{IdleTask, Task};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

thread_local! {
  static CLOCK: Cell<f64> = const { Cell::new(0.0) };
}

fn clock() -> f64 {
  CLOCK.with(Cell::get)
}

fn reset_clock() {
  CLOCK.with(|c| c.set(0.0));
}

fn increase_clock(delta: f64) {
  CLOCK.with(|c| c.set(c.get() + delta));
}

/// 记录执行序 / Die 析构的任务。
struct ProbeTask {
  id: u32,
  order: Arc<Mutex<Vec<u32>>>,
  die: Arc<AtomicUsize>,
}

impl Task for ProbeTask {
  fn run(&mut self) {
    if let Ok(mut v) = self.order.lock() {
      v.push(self.id);
    }
  }
}

impl Drop for ProbeTask {
  fn drop(&mut self) {
    self.die.fetch_add(1, Ordering::Relaxed);
  }
}

struct ProbeIdleTask {
  /// 投递序号: run 时并入 deadline 记录, 使弹出序可被独立鉴别 (复审 W2 强化)
  seq: u32,
  deadline_sink: Arc<Mutex<Vec<f64>>>,
  die: Arc<AtomicUsize>,
}

impl IdleTask for ProbeIdleTask {
  fn run(&mut self, deadline_in_seconds: f64) {
    if let Ok(mut v) = self.deadline_sink.lock() {
      v.push(self.seq as f64 * 1000.0 + deadline_in_seconds);
    }
  }
}

impl Drop for ProbeIdleTask {
  fn drop(&mut self) {
    self.die.fetch_add(1, Ordering::Relaxed);
  }
}

struct Fixture {
  order: Arc<Mutex<Vec<u32>>>,
  die: Arc<AtomicUsize>,
  deadlines: Arc<Mutex<Vec<f64>>>,
  idle_die: Arc<AtomicUsize>,
  /// 空闲 probe 投递计数器 (W2: 鉴别弹出序而非仅记录 run 调用序)
  idle_seq: Arc<AtomicUsize>,
}

impl Fixture {
  fn new() -> Self {
    Self {
      order: Arc::new(Mutex::new(Vec::new())),
      die: Arc::new(AtomicUsize::new(0)),
      deadlines: Arc::new(Mutex::new(Vec::new())),
      idle_die: Arc::new(AtomicUsize::new(0)),
      idle_seq: Arc::new(AtomicUsize::new(0)),
    }
  }

  fn probe(&self, id: u32) -> BoxTask {
    Box::new(ProbeTask {
      id,
      order: self.order.clone(),
      die: self.die.clone(),
    })
  }

  fn idle_probe(&self) -> BoxIdleTask {
    Box::new(ProbeIdleTask {
      seq: self.idle_seq.fetch_add(1, Ordering::Relaxed) as u32 + 1,
      deadline_sink: self.deadlines.clone(),
      die: self.idle_die.clone(),
    })
  }

  fn order(&self) -> Vec<u32> {
    self.order.lock().expect("order 锁不应中毒").clone()
  }

  fn runner(&self, idle: IdleTaskSupport) -> Arc<DefaultForegroundTaskRunner> {
    Arc::new(DefaultForegroundTaskRunner::new(idle, clock))
  }
}

fn pop(runner: &Arc<DefaultForegroundTaskRunner>) -> Option<BoxTask> {
  runner.pop_task_from_queue(MessageLoopBehavior::DoNotWait)
}

fn drain(runner: &Arc<DefaultForegroundTaskRunner>) {
  while let Some(mut task) = pop(runner) {
    task.run();
    drop(task);
  }
}

/// 即时任务 FIFO；空队列 DoNotWait 直接返回 None。
#[test]
fn fifo_order() -> Void {
  info!("> fifo_order");
  reset_clock();
  let fx = Fixture::new();
  let runner = fx.runner(IdleTaskSupport::Disabled);
  assert!(pop(&runner).is_none());

  runner.post_task(fx.probe(1), SourceLocation::current());
  runner.post_task(fx.probe(2), SourceLocation::current());
  drain(&runner);
  assert_eq!(fx.order(), vec![1, 2]);
  OK
}

/// NonNestable 语义：嵌套（RunTaskScope 持有时）只弹 nestable 任务；
/// 非嵌套态任意任务可弹；非 nestable 只可能更晚，不会更早。
#[test]
fn non_nestable_task_semantics() -> Void {
  info!("> non_nestable_task_semantics");
  reset_clock();
  let fx = Fixture::new();
  let runner = fx.runner(IdleTaskSupport::Disabled);

  runner.post_task(fx.probe(1), SourceLocation::current());
  runner.post_non_nestable_task(fx.probe(2), SourceLocation::current());
  runner.post_task(fx.probe(3), SourceLocation::current());
  runner.post_non_nestable_task(fx.probe(4), SourceLocation::current());

  // 嵌套深度 1：只允许 nestable（1、3 依序）。
  {
    let _scope = RunTaskScope::new(runner.clone());
    assert!(
      runner
        .pop_task_from_queue(MessageLoopBehavior::DoNotWait)
        .is_some_and(|mut t| {
          t.run();
          drop(t);
          true
        })
    );
    assert!(
      runner
        .pop_task_from_queue(MessageLoopBehavior::DoNotWait)
        .is_some_and(|mut t| {
          t.run();
          drop(t);
          true
        })
    );
    // 队列仅剩 non-nestable：嵌套态不可弹。
    assert!(pop(&runner).is_none());
  }

  // 退出嵌套：non-nestable 依序可弹。
  drain(&runner);
  assert_eq!(
    fx.order(),
    vec![1, 3, 2, 4],
    "对应原版 PumpMessageLoopNested 期望序"
  );
  assert!(runner.non_nestable_tasks_enabled());
  assert!(runner.non_nestable_delayed_tasks_enabled());
  OK
}

/// 延迟任务到期前不可弹；到期后可弹（前台无饥饿保护：先即时后到期）。
#[test]
fn delayed_task_expiry() -> Void {
  info!("> delayed_task_expiry");
  reset_clock();
  let fx = Fixture::new();
  let runner = fx.runner(IdleTaskSupport::Disabled);

  runner.post_delayed_task(fx.probe(9), 100.0, SourceLocation::current());
  runner.post_delayed_task(fx.probe(1), 10.0, SourceLocation::current());
  assert!(pop(&runner).is_none());

  increase_clock(11.0);
  drain(&runner);
  assert_eq!(fx.order(), vec![1]);

  increase_clock(90.0);
  drain(&runner);
  assert_eq!(fx.order(), vec![1, 9]);
  OK
}

/// Terminate：滞留任务锁外销毁；终止后投递即弃（对应原 PostTaskLocked 交回语义）。
#[test]
fn terminate_and_post_after_termination() -> Void {
  info!("> terminate_and_post_after_termination");
  reset_clock();
  let fx = Fixture::new();
  let runner = fx.runner(IdleTaskSupport::Enabled);

  runner.post_task(fx.probe(1), SourceLocation::current());
  runner.post_delayed_task(fx.probe(2), 10.0, SourceLocation::current());
  runner.post_idle_task(fx.idle_probe(), SourceLocation::current());
  assert_eq!(fx.die.load(Ordering::Relaxed), 0);

  runner.terminate();
  assert_eq!(
    fx.die.load(Ordering::Relaxed),
    2,
    "即时+延迟滞留任务应随 Terminate 销毁"
  );
  assert_eq!(
    fx.idle_die.load(Ordering::Relaxed),
    1,
    "空闲滞留任务应随 Terminate 销毁"
  );

  // 终止后仍可投递（对应原 PostForegroundTaskAfterPlatformTermination）：任务即弃不执行。
  runner.post_task(fx.probe(3), SourceLocation::current());
  runner.post_delayed_task(fx.probe(4), 10.0, SourceLocation::current());
  runner.post_idle_task(fx.idle_probe(), SourceLocation::current());
  assert_eq!(
    fx.die.load(Ordering::Relaxed),
    4,
    "终止后即时+延迟投递均即弃"
  );
  assert_eq!(fx.idle_die.load(Ordering::Relaxed), 2);
  assert_eq!(fx.order(), Vec::<u32>::new(), "终止后任务不得执行");
  assert!(pop(&runner).is_none());
  OK
}

/// 空闲任务 FIFO 与使能开关。
#[test]
fn idle_tasks_fifo_and_support() -> Void {
  info!("> idle_tasks_fifo_and_support");
  reset_clock();
  let fx = Fixture::new();
  let runner = fx.runner(IdleTaskSupport::Enabled);
  assert!(runner.idle_tasks_enabled());

  runner.post_idle_task(fx.idle_probe(), SourceLocation::current());
  runner.post_idle_task(fx.idle_probe(), SourceLocation::current());
  let mut t1 = runner.pop_task_from_idle_queue().expect("应有空闲任务");
  let mut t2 = runner.pop_task_from_idle_queue().expect("应有空闲任务");
  t1.run(65.0);
  t2.run(66.0);
  // 记录值 = seq*1000 + deadline: 1065 在前坐实 FIFO 弹出序 (seq1 任务先被弹), 非仅 run 调用序
  assert_eq!(
    *fx.deadlines.lock().expect("deadlines 锁不应中毒"),
    vec![1065.0, 2066.0]
  );

  drop(t1);
  drop(t2);
  // 未使能空闲任务时投递：对应原版 CHECK_EQ 降级为日志并即弃任务。
  let disabled = fx.runner(IdleTaskSupport::Disabled);
  assert!(!disabled.idle_tasks_enabled());
  let idle_die_before = fx.idle_die.load(Ordering::Relaxed);
  disabled.post_idle_task(fx.idle_probe(), SourceLocation::current());
  assert_eq!(
    fx.idle_die.load(Ordering::Relaxed),
    idle_die_before + 1,
    "未使能时空闲投递应即弃"
  );
  disabled.terminate();
  runner.terminate();
  OK
}
