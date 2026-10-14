//! `DelayedTaskQueue` 语义用例（原版无独立 unittest，延迟语义经
//! `default-worker-threads-task-runner-unittest.cc` 间接覆盖；本文件按源码语义直测）。

use std::{
  cell::Cell,
  sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
  },
};

use aok::{OK, Void};
use log::info;
use v8_base_platform::TimeConstants;
use v8_libplatform::{BoxTask, DelayedTaskQueue, MaybeNextTask};
use v8_tasks::Task;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

thread_local! {
  /// 假时钟（秒），对应原版 FakeClock 口径。
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

/// 记录执行序与析构（对应 gmock Run/Die）的任务。
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

struct Fixture {
  order: Arc<Mutex<Vec<u32>>>,
  die: Arc<AtomicUsize>,
}

impl Fixture {
  fn new() -> Self {
    Self {
      order: Arc::new(Mutex::new(Vec::new())),
      die: Arc::new(AtomicUsize::new(0)),
    }
  }

  fn probe(&self, id: u32) -> BoxTask {
    Box::new(ProbeTask {
      id,
      order: self.order.clone(),
      die: self.die.clone(),
    })
  }

  fn order(&self) -> Vec<u32> {
    self.order.lock().expect("order 锁不应中毒").clone()
  }
}

fn next_task(queue: &mut DelayedTaskQueue) -> Option<BoxTask> {
  match queue.try_get_next() {
    MaybeNextTask::Task(task) => Some(task),
    _ => None,
  }
}

/// 取出并执行，校验执行序。
fn run_next(queue: &mut DelayedTaskQueue, fx: &Fixture, expect_id: u32) {
  let mut task = next_task(queue).expect("应取到到期/即时任务");
  task.run();
  drop(task);
  assert_eq!(
    fx.order().last().copied(),
    Some(expect_id),
    "执行序应命中期望任务"
  );
}

/// 即时任务按投递序执行；空载回报 WaitIndefinite，终止后回报 Terminated。
#[test]
fn immediate_tasks_run_in_post_order() -> Void {
  info!("> immediate_tasks_run_in_post_order");
  reset_clock();
  let fx = Fixture::new();
  let mut queue = DelayedTaskQueue::new(clock);
  assert!(matches!(
    queue.try_get_next(),
    MaybeNextTask::WaitIndefinite
  ));

  queue.append(fx.probe(1));
  queue.append(fx.probe(2));
  queue.append(fx.probe(3));
  run_next(&mut queue, &fx, 1);
  run_next(&mut queue, &fx, 2);
  run_next(&mut queue, &fx, 3);
  assert!(matches!(
    queue.try_get_next(),
    MaybeNextTask::WaitIndefinite
  ));
  assert_eq!(fx.order(), vec![1, 2, 3]);

  queue.terminate();
  assert!(matches!(queue.try_get_next(), MaybeNextTask::Terminated));
  OK
}

/// 延迟任务按到期时刻升序出队；未到期回报 WaitDelayed。
#[test]
fn delayed_tasks_ordered_by_deadline() -> Void {
  info!("> delayed_tasks_ordered_by_deadline");
  reset_clock();
  let fx = Fixture::new();
  let mut queue = DelayedTaskQueue::new(clock);

  queue.append_delayed(fx.probe(2), 100.0);
  queue.append_delayed(fx.probe(1), 10.0);

  // 最早到期在 10s：回报等待时长（对应原 kWaitDelayed）。
  let wait = match queue.try_get_next() {
    MaybeNextTask::WaitDelayed(d) => d,
    _ => panic!("期望 WaitDelayed 分支"),
  };
  assert_eq!(
    wait.in_microseconds(),
    TimeConstants::MICROSECONDS_PER_SECOND * 10
  );

  increase_clock(11.0);
  run_next(&mut queue, &fx, 1);
  assert!(matches!(
    queue.try_get_next(),
    MaybeNextTask::WaitDelayed(_)
  ));

  increase_clock(90.0);
  run_next(&mut queue, &fx, 2);
  assert!(matches!(
    queue.try_get_next(),
    MaybeNextTask::WaitIndefinite
  ));
  assert_eq!(fx.order(), vec![1, 2]);

  queue.terminate();
  OK
}

/// 到期延迟任务搬入即时队列尾部，与新即时任务按序竞争
/// （对应原注释：延迟任务与即时任务间无相对顺序保证）。
#[test]
fn expired_delayed_mixed_with_immediate() -> Void {
  info!("> expired_delayed_mixed_with_immediate");
  reset_clock();
  let fx = Fixture::new();
  let mut queue = DelayedTaskQueue::new(clock);

  queue.append(fx.probe(1));
  queue.append_delayed(fx.probe(2), 10.0);
  increase_clock(11.0);
  queue.append(fx.probe(3));

  // try_get_next 时才把到期任务搬到即时队列尾部：序为 1、3、2。
  run_next(&mut queue, &fx, 1);
  run_next(&mut queue, &fx, 3);
  run_next(&mut queue, &fx, 2);

  queue.terminate();
  OK
}

/// 同 deadline 并列时按投递序（multimap 等键序承接）。
#[test]
fn same_deadline_keeps_post_order() -> Void {
  info!("> same_deadline_keeps_post_order");
  reset_clock();
  let fx = Fixture::new();
  let mut queue = DelayedTaskQueue::new(clock);

  queue.append_delayed(fx.probe(1), 5.0);
  queue.append_delayed(fx.probe(2), 5.0);
  increase_clock(5.0);

  run_next(&mut queue, &fx, 1);
  run_next(&mut queue, &fx, 2);
  queue.terminate();
  OK
}

/// 终止后延迟任务随队列析构销毁（Die 语义）。
#[test]
fn terminated_destroys_pending_delayed_tasks() -> Void {
  info!("> terminated_destroys_pending_delayed_tasks");
  reset_clock();
  let fx = Fixture::new();
  {
    let mut queue = DelayedTaskQueue::new(clock);
    queue.append_delayed(fx.probe(1), 10.0);
    assert_eq!(fx.die.load(Ordering::Relaxed), 0);
    queue.terminate();
    assert!(matches!(queue.try_get_next(), MaybeNextTask::Terminated));
  }
  assert_eq!(
    fx.die.load(Ordering::Relaxed),
    1,
    "队列析构应销毁滞留延迟任务"
  );
  OK
}
