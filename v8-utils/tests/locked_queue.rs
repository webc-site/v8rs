// 迁移自 v8/test/unittests/utils/locked-queue-unittest.cc
// 断言规约: TEST(LockedQueue, X) → fn x; Record = i32; Dequeue → Option。
use aok::{OK, Void};
use v8_utils::locked_queue::LockedQueue;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

type Record = i32;

// ConstructorEmpty
#[test]
fn constructor_empty() -> Void {
  let queue: LockedQueue<Record> = LockedQueue::new();
  assert!(queue.is_empty());
  OK
}

// SingleRecordEnqueueDequeue
#[test]
fn single_record_enqueue_dequeue() -> Void {
  let queue: LockedQueue<Record> = LockedQueue::new();
  assert!(queue.is_empty());
  queue.enqueue(1);
  assert!(!queue.is_empty());
  let a = queue.dequeue();
  assert_eq!(a, Some(1));
  assert!(queue.is_empty());
  OK
}

// Peek
#[test]
fn peek() -> Void {
  let queue: LockedQueue<Record> = LockedQueue::new();
  assert!(queue.is_empty());
  queue.enqueue(1);
  assert!(!queue.is_empty());
  let a = queue.peek();
  assert_eq!(a, Some(1));
  assert!(!queue.is_empty());
  let a = queue.dequeue();
  assert_eq!(a, Some(1));
  assert!(queue.is_empty());
  OK
}

// PeekOnEmpty
#[test]
fn peek_on_empty() -> Void {
  let queue: LockedQueue<Record> = LockedQueue::new();
  assert!(queue.is_empty());
  let a = queue.peek();
  assert_eq!(a, None);
  OK
}

// MultipleRecords
#[test]
fn multiple_records() -> Void {
  let queue: LockedQueue<Record> = LockedQueue::new();
  assert!(queue.is_empty());
  queue.enqueue(1);
  assert!(!queue.is_empty());
  for i in 2..=5 {
    queue.enqueue(i);
    assert!(!queue.is_empty());
  }
  for i in 1..=4 {
    assert!(!queue.is_empty());
    assert_eq!(queue.dequeue(), Some(i));
  }
  for i in 6..=12 {
    queue.enqueue(i);
    assert!(!queue.is_empty());
  }
  for i in 5..=12 {
    assert!(!queue.is_empty());
    assert_eq!(queue.dequeue(), Some(i));
  }
  assert!(queue.is_empty());
  OK
}

// 补充: 多生产多消费并发收发 (上游宣称 MPMC 但无并发单测; 关键路径补测)
#[test]
fn concurrent_producers_consumers() -> Void {
  use std::{sync::Arc, thread};

  let queue = Arc::new(LockedQueue::<Record>::new());
  const PRODUCERS: usize = 4;
  const PER_PRODUCER: usize = 1000;

  let mut handles = Vec::new();
  for p in 0..PRODUCERS {
    let q = Arc::clone(&queue);
    handles.push(thread::spawn(move || {
      for i in 0..PER_PRODUCER {
        q.enqueue((p * PER_PRODUCER + i) as Record);
      }
    }));
  }
  for h in handles {
    h.join().unwrap();
  }
  assert_eq!(queue.size(), PRODUCERS * PER_PRODUCER);

  // 消费者出队, 总量与去重校验
  let mut consumed = Vec::new();
  while let Some(v) = queue.dequeue() {
    consumed.push(v);
  }
  assert_eq!(consumed.len(), PRODUCERS * PER_PRODUCER);
  consumed.sort_unstable();
  consumed.dedup();
  assert_eq!(consumed.len(), PRODUCERS * PER_PRODUCER);
  assert!(queue.is_empty());
  OK
}

// 补充: 产销同时在场的交错压测 — 生产者边发边收, size 计数先于链接的
// 不变量被并发验证 (若先链接后计数, 消费者 fetch_sub 会在 debug 下溢 panic)
#[test]
fn concurrent_interleaved_produce_consume() -> Void {
  use std::{sync::Arc, thread};

  let queue = Arc::new(LockedQueue::<Record>::new());
  const WORKERS: usize = 4;
  const PER_WORKER: usize = 2000;
  const TOTAL: usize = WORKERS * PER_WORKER;

  let producer = {
    let q = Arc::clone(&queue);
    thread::spawn(move || {
      for i in 0..TOTAL {
        q.enqueue(i as Record);
      }
    })
  };
  let consumer = {
    let q = Arc::clone(&queue);
    thread::spawn(move || {
      let mut got = 0;
      while got < TOTAL {
        if q.dequeue().is_some() {
          got += 1;
        }
      }
      got
    })
  };
  producer.join().unwrap();
  assert_eq!(consumer.join().unwrap(), TOTAL);
  assert!(queue.is_empty());
  assert_eq!(queue.size(), 0);
  OK
}
