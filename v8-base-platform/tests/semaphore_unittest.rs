//! semaphore-unittest.cc 1:1 迁移。规约见 tests/SPEC.md。
use std::{
  cell::UnsafeCell,
  sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
  },
};

use aok::{OK, Void};
use log::info;
use v8_base_platform::{Options, Semaphore, Thread, TimeDelta};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// cc:16-19
const K_ALPHABET: &[u8] = b"XKOAD";
const K_ALPHABET_SIZE: usize = K_ALPHABET.len(); // sizeof(kAlphabet) - 1 == 5
const K_BUFFER_SIZE: usize = 987; // GCD(buffer size, alphabet size) = 1
const K_DATA_SIZE: usize = K_BUFFER_SIZE * K_ALPHABET_SIZE * 10;

/// 跨线程共享缓冲；访问序由 free_space/used_space 信号量协议全序化，
/// 与原版 `char* buffer_` 裸指针共享等价。
struct Shared<T>(UnsafeCell<T>);
// SAFETY: 任意时刻仅一个线程通过信号量协议进入临界区访问。
unsafe impl<T> Sync for Shared<T> {}

// TEST(Semaphore, ProducerConsumer) cc:90-101
#[test]
fn semaphore_producer_consumer() -> Void {
  info!("> semaphore_producer_consumer");
  let buffer = Arc::new(Shared(UnsafeCell::new([0u8; K_BUFFER_SIZE])));
  let free_space = Arc::new(Semaphore::new(K_BUFFER_SIZE as i32));
  let used_space = Arc::new(Semaphore::new(0));
  // cc:92 memset 清零 == [0u8; K_BUFFER_SIZE] 初始化（类型保证, 无断言）

  let mismatches = Arc::new(AtomicUsize::new(0));

  // 原版 EXPECT_EQ 在消费者线程内 (cc:58)；Rust 侧记录违规由主线程汇总断言，
  // 避免 worker panic 使另一 worker 阻塞导致挂起，断言强度不变。
  let producer_buffer = Arc::clone(&buffer);
  let p_free = Arc::clone(&free_space);
  let p_used = Arc::clone(&used_space);
  let mut producer_thread = Thread::new(Options::new("ProducerThread"), move || {
    for n in 0..K_DATA_SIZE {
      p_free.wait();
      // SAFETY: free_space 授予独占写权。
      unsafe { (*producer_buffer.0.get())[n % K_BUFFER_SIZE] = K_ALPHABET[n % K_ALPHABET_SIZE] };
      p_used.signal();
    }
  });

  let consumer_buffer = Arc::clone(&buffer);
  let c_free = Arc::clone(&free_space);
  let c_used = Arc::clone(&used_space);
  let c_mismatch = Arc::clone(&mismatches);
  let mut consumer_thread = Thread::new(Options::new("ConsumerThread"), move || {
    for n in 0..K_DATA_SIZE {
      c_used.wait();
      // cc:58 EXPECT_EQ(kAlphabet[n % 5], buffer[n % 987])
      let got = unsafe { (*consumer_buffer.0.get())[n % K_BUFFER_SIZE] };
      if got != K_ALPHABET[n % K_ALPHABET_SIZE] {
        c_mismatch.fetch_add(1, Ordering::Relaxed);
      }
      c_free.signal();
    }
  });

  assert!(producer_thread.start().is_ok()); // cc:97 CHECK
  assert!(consumer_thread.start().is_ok()); // cc:98 CHECK
  assert!(producer_thread.join().is_ok());
  assert!(consumer_thread.join().is_ok());
  assert_eq!(
    mismatches.load(Ordering::Relaxed),
    0,
    "消费者 EXPECT_EQ 全部应成立"
  );
  OK
}

// TEST(Semaphore, WaitAndSignal) cc:104-121
#[test]
fn semaphore_wait_and_signal() -> Void {
  let semaphore = Arc::new(Semaphore::new(0));
  let violations = Arc::new(AtomicUsize::new(0));

  // worker: cc:75-80 ASSERT_FALSE(WaitFor(1µs))
  let spawn_waiter = |v: &Arc<AtomicUsize>| {
    let sem = Arc::clone(&semaphore);
    let viol = Arc::clone(v);
    Thread::new(Options::new("WaitAndSignalThread"), move || {
      for _ in 0..100 {
        sem.wait();
        if sem.wait_for(TimeDelta::from_microseconds(1)) {
          viol.fetch_add(1, Ordering::Relaxed);
        }
        sem.signal();
      }
    })
  };
  let mut t1 = spawn_waiter(&violations);
  let mut t2 = spawn_waiter(&violations);

  assert!(t1.start().is_ok()); // cc:109 CHECK
  assert!(t2.start().is_ok()); // cc:110 CHECK

  // cc:113 Make something available.
  semaphore.signal();

  assert!(t1.join().is_ok()); // cc:115
  assert!(t2.join().is_ok()); // cc:116

  semaphore.wait(); // cc:118

  assert!(!semaphore.wait_for(TimeDelta::from_microseconds(1))); // cc:120 EXPECT_FALSE
  assert_eq!(
    violations.load(Ordering::Relaxed),
    0,
    "worker ASSERT_FALSE WaitFor(1µs) 应全部成立"
  );
  OK
}

// TEST(Semaphore, WaitFor) cc:124-139
#[test]
fn semaphore_wait_for() -> Void {
  let semaphore = Semaphore::new(0);

  // Semaphore not signalled - timeout.
  assert!(!semaphore.wait_for(TimeDelta::from_microseconds(0))); // cc:128
  assert!(!semaphore.wait_for(TimeDelta::from_microseconds(100))); // cc:129
  assert!(!semaphore.wait_for(TimeDelta::from_microseconds(1000))); // cc:130

  // Semaphore signalled - no timeout.
  semaphore.signal(); // cc:133
  assert!(semaphore.wait_for(TimeDelta::from_microseconds(0))); // cc:134
  semaphore.signal(); // cc:135
  assert!(semaphore.wait_for(TimeDelta::from_microseconds(100))); // cc:136
  semaphore.signal(); // cc:137
  assert!(semaphore.wait_for(TimeDelta::from_microseconds(1000))); // cc:138
  OK
}
