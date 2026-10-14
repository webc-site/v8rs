//! 对应 `src/tasks/task-utils.{h,cc}`，规约见 tests/SPEC.md 16-19。

use std::sync::{
  Arc,
  atomic::{AtomicU64, Ordering},
};

use aok::{OK, Void};
use log::info;
use v8_tasks::{
  CancelableTaskManager, INVALID_TASK_ID, IdleTask, Task, TaskId, make_cancelable_idle_task,
  make_cancelable_task,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn make_cancelable_task_runs() -> Void {
  let manager = CancelableTaskManager::new();
  let hits = Arc::new(AtomicU64::new(0));
  let mut task = make_cancelable_task(&manager, {
    let hits = hits.clone();
    move || {
      hits.fetch_add(1, Ordering::Relaxed);
    }
  })?;
  task.run();
  assert_eq!(1u64, hits.load(Ordering::Relaxed));
  drop(task);
  manager.cancel_and_wait();
  OK
}

#[test]
fn make_cancelable_task_canceled() -> Void {
  let manager = CancelableTaskManager::new();
  let hits = Arc::new(AtomicU64::new(0));
  let mut task = make_cancelable_task(&manager, {
    let hits = hits.clone();
    move || {
      hits.fetch_add(1, Ordering::Relaxed);
    }
  })?;
  manager.cancel_and_wait();
  task.run();
  assert_eq!(0u64, hits.load(Ordering::Relaxed), "取消后不得执行闭包");
  drop(task);
  info!("make_cancelable_task_canceled ok");
  OK
}

#[test]
fn make_cancelable_idle_task_runs() -> Void {
  let manager = CancelableTaskManager::new();
  let last = Arc::new(AtomicU64::new(0));
  let mut task = make_cancelable_idle_task(&manager, {
    let last = last.clone();
    move |deadline: f64| {
      last.store(deadline.to_bits(), Ordering::Relaxed);
    }
  })?;
  task.run(7.25);
  assert_eq!(7.25f64.to_bits(), last.load(Ordering::Relaxed));
  drop(task);
  manager.cancel_and_wait();
  OK
}

#[test]
fn make_cancelable_task_id() -> Void {
  let manager = CancelableTaskManager::new();
  let noop = || {};
  let t1: TaskId;
  let t2: TaskId;
  {
    let a = make_cancelable_task(&manager, noop)?;
    let b = make_cancelable_task(&manager, noop)?;
    t1 = a.id();
    t2 = b.id();
    assert_ne!(INVALID_TASK_ID, t1);
  }
  manager.cancel_and_wait();
  assert_eq!(1u64, t1);
  assert_eq!(2u64, t2);
  OK
}
