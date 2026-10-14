//! 对应 `v8/test/unittests/libplatform/task-queue-unittest.cc` 1:1 迁移。

use std::sync::{
  Arc, Mutex,
  atomic::{AtomicUsize, Ordering},
};

use aok::{OK, Void};
use log::info;
use v8_libplatform::{BoxTask, TaskQueue};
use v8_tasks::Task;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 对应原版 gmock `MockTask`：Run/Die 观测点改为计数 + 执行序记录。
struct ProbeTask {
  id: usize,
  order: Arc<Mutex<Vec<usize>>>,
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

fn probe(id: usize, order: &Arc<Mutex<Vec<usize>>>, die: &Arc<AtomicUsize>) -> BoxTask {
  Box::new(ProbeTask {
    id,
    order: order.clone(),
    die: die.clone(),
  })
}

/// 对应原 `TEST(TaskQueueTest, Basic)`。
#[test]
fn task_queue_basic() -> Void {
  info!("> task_queue_basic");
  let queue = TaskQueue::new();
  let order = Arc::new(Mutex::new(Vec::new()));
  let die = Arc::new(AtomicUsize::new(0));

  // 原版以指针身份断言取回同一任务；Rust 侧以执行序 + 析构计数验证身份。
  queue.append(probe(1, &order, &die));
  let mut task1 = queue.get_next().expect("append 后应取到任务");
  task1.run();
  queue.append(probe(2, &order, &die));
  let mut task2 = queue.get_next().expect("append 后应取到任务");
  task2.run();
  assert_eq!(*order.lock().expect("order 锁不应中毒"), vec![1, 2]);
  assert_eq!(die.load(Ordering::Relaxed), 0, "任务仍在持有中，不应析构");
  drop(task1);
  drop(task2);
  assert_eq!(die.load(Ordering::Relaxed), 2, "取回任务销毁即 Die");

  queue.terminate();
  assert!(
    queue.get_next().is_none(),
    "终止后应返回 None（原版 IsNull）"
  );
  OK
}

/// 对应原 `TEST(TaskQueueTest, TerminateMultipleReaders)`：
/// 两个 base::Thread 读者等待终止，各自收到 None（哨兵经信号量接力传播）。
#[test]
fn task_queue_terminate_multiple_readers() -> Void {
  info!("> task_queue_terminate_multiple_readers");
  use v8_base_platform::{Options, Thread};

  let queue = Arc::new(TaskQueue::new());
  let nulls = Arc::new(AtomicUsize::new(0));

  let mut readers: Vec<Thread> = (0..2)
    .map(|_| {
      let queue = queue.clone();
      let nulls = nulls.clone();
      Thread::new(Options::new("libplatform TaskQueueThread"), move || {
        if queue.get_next().is_none() {
          nulls.fetch_add(1, Ordering::Relaxed);
        }
      })
    })
    .collect();
  for reader in &mut readers {
    reader.start()?;
  }
  queue.terminate();
  for reader in &mut readers {
    reader.join()?;
  }
  assert_eq!(nulls.load(Ordering::Relaxed), 2, "两个读者都应收终止哨兵");
  OK
}
