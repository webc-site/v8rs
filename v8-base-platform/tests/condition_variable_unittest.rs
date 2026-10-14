//! condition-variable-unittest.cc 1:1 迁移。规约见 tests/SPEC.md。
// 原版 C++ `for (int n = 0; ...)` 下标循环按 1:1 直译，静默对应风格 lint（不影响断言）。
use std::{
  cell::UnsafeCell,
  sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
  },
};

use aok::{OK, Void};
use log::info;
use v8_base_platform::{ConditionVariable, Mutex, MutexGuard, Options, Thread, TimeDelta};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 由配对 Mutex 保护的共享标志（对应原版裸 bool 成员 + mutex_ 协议）。
struct Shared<T>(UnsafeCell<T>);
// SAFETY: 仅在持有配对 Mutex 时读写。
unsafe impl<T> Sync for Shared<T> {}

// TEST(ConditionVariable, WaitForAfterNofityOnSameThread) cc:14-27（保留原版拼写）
#[test]
fn condition_variable_wait_for_after_nofity_on_same_thread() -> Void {
  info!("> condition_variable_wait_for_after_nofity_on_same_thread");
  for n in 0..10i64 {
    let mutex = Mutex::new();
    let cv = ConditionVariable::new();
    let _lock_guard = MutexGuard::new(&mutex);

    cv.notify_one();
    assert!(!cv.wait_for(&mutex, TimeDelta::from_microseconds(n))); // cc:22 EXPECT_FALSE

    cv.notify_all();
    assert!(!cv.wait_for(&mutex, TimeDelta::from_microseconds(n))); // cc:25 EXPECT_FALSE
  }
  OK
}

// ThreadWithMutexAndConditionVariable (cc:32-54)
struct PerThreadState {
  running: Shared<bool>,
  finished: Shared<bool>,
  cv: ConditionVariable,
  mutex: Mutex,
}

fn separate_worker(state: Arc<PerThreadState>) -> Thread {
  Thread::new(
    Options::new("ThreadWithMutexAndConditionVariable"),
    move || {
      let _lock_guard = MutexGuard::new(&state.mutex);
      // SAFETY: 持有 mutex。
      unsafe { *state.running.0.get() = true };
      state.cv.notify_one();
      while unsafe { *state.running.0.get() } {
        state.cv.wait(&state.mutex);
      }
      unsafe { *state.finished.0.get() = true };
      state.cv.notify_all();
    },
  )
}

// TEST(ConditionVariable, MultipleThreadsWithSeparateConditionVariables) cc:59-105
#[test]
fn condition_variable_multiple_threads_with_separate_condition_variables() -> Void {
  const K_THREAD_COUNT: usize = 128; // cc:60
  let states: Vec<Arc<PerThreadState>> = (0..K_THREAD_COUNT)
    .map(|_| {
      Arc::new(PerThreadState {
        running: Shared(UnsafeCell::new(false)),
        finished: Shared(UnsafeCell::new(false)),
        cv: ConditionVariable::new(),
        mutex: Mutex::new(),
      })
    })
    .collect();
  let mut threads: Vec<Thread> = states
    .iter()
    .map(|s| separate_worker(Arc::clone(s)))
    .collect();

  // 启动阶段: cc:63-72
  for n in 0..K_THREAD_COUNT {
    let s = &states[n];
    let _lock_guard = MutexGuard::new(&s.mutex);
    assert!(!unsafe { *s.running.0.get() }); // cc:65 EXPECT_FALSE running_
    assert!(!unsafe { *s.finished.0.get() }); // cc:66 EXPECT_FALSE finished_
    assert!(threads[n].start().is_ok()); // cc:67 CHECK(Start())
    // Wait for nth thread to start. cc:69-71
    while !unsafe { *s.running.0.get() } {
      s.cv.wait(&s.mutex);
    }
  }

  // cc:74-78 (逆序)
  for s in states.iter().rev() {
    let _lock_guard = MutexGuard::new(&s.mutex);
    assert!(unsafe { *s.running.0.get() }); // cc:76 EXPECT_TRUE running_
    assert!(!unsafe { *s.finished.0.get() }); // cc:77 EXPECT_FALSE finished_
  }

  // cc:80-87 通知退出
  for s in &states {
    let _lock_guard = MutexGuard::new(&s.mutex);
    assert!(unsafe { *s.running.0.get() }); // cc:82 EXPECT_TRUE running_
    assert!(!unsafe { *s.finished.0.get() }); // cc:83 EXPECT_FALSE finished_
    unsafe { *s.running.0.get() = false }; // cc:85
    s.cv.notify_one(); // cc:86
  }

  // cc:89-97 等待退出
  for n in (0..K_THREAD_COUNT).rev() {
    let s = &states[n];
    let _lock_guard = MutexGuard::new(&s.mutex);
    while !unsafe { *s.finished.0.get() } {
      s.cv.wait(&s.mutex);
    }
    assert!(!unsafe { *s.running.0.get() }); // cc:95
    assert!(unsafe { *s.finished.0.get() }); // cc:96
  }

  // cc:99-104 Join
  for n in 0..K_THREAD_COUNT {
    assert!(threads[n].join().is_ok());
    let s = &states[n];
    let _lock_guard = MutexGuard::new(&s.mutex);
    assert!(!unsafe { *s.running.0.get() }); // cc:102
    assert!(unsafe { *s.finished.0.get() }); // cc:103
  }
  OK
}

// ThreadWithSharedMutexAndConditionVariable (cc:110-134)
struct SharedState {
  running: Shared<Vec<bool>>,
  finished: Shared<Vec<bool>>,
  cv: ConditionVariable,
  mutex: Mutex,
}

fn shared_worker(state: Arc<SharedState>, index: usize) -> Thread {
  Thread::new(
    Options::new("ThreadWithSharedMutexAndConditionVariable"),
    move || {
      let _lock_guard = MutexGuard::new(&state.mutex);
      // SAFETY: 持有配对 mutex（对应原版共享 bool 数组 + mutex_ 协议）。
      unsafe { (&mut (*state.running.0.get()))[index] = true };
      state.cv.notify_all();
      while unsafe { (&(*state.running.0.get()))[index] } {
        state.cv.wait(&state.mutex);
      }
      unsafe { (&mut (*state.finished.0.get()))[index] = true };
      state.cv.notify_all();
    },
  )
}

// TEST(ConditionVariable, MultipleThreadsWithSharedSeparateConditionVariables) cc:139-214（保留原版名）
#[test]
fn condition_variable_multiple_threads_with_shared_separate_condition_variables() -> Void {
  const K_THREAD_COUNT: usize = 128; // cc:140
  let state = Arc::new(SharedState {
    running: Shared(UnsafeCell::new(vec![false; K_THREAD_COUNT])),
    finished: Shared(UnsafeCell::new(vec![false; K_THREAD_COUNT])),
    cv: ConditionVariable::new(),
    mutex: Mutex::new(),
  });
  let mut threads: Vec<Thread> = (0..K_THREAD_COUNT)
    .map(|n| shared_worker(Arc::clone(&state), n))
    .collect();

  // Start all threads. cc:151-158
  {
    let _lock_guard = MutexGuard::new(&state.mutex);
    for (n, thread) in threads.iter_mut().enumerate() {
      assert!(!unsafe { (&(*state.running.0.get()))[n] }); // cc:154
      assert!(!unsafe { (&(*state.finished.0.get()))[n] }); // cc:155
      assert!(thread.start().is_ok()); // cc:156 CHECK
    }
  }

  // Wait for all threads to start. cc:161-168
  {
    let _lock_guard = MutexGuard::new(&state.mutex);
    for n in (0..K_THREAD_COUNT).rev() {
      while !unsafe { (&(*state.running.0.get()))[n] } {
        state.cv.wait(&state.mutex);
      }
    }
  }

  // Make sure that all threads are running. cc:171-177
  {
    let _lock_guard = MutexGuard::new(&state.mutex);
    for n in 0..K_THREAD_COUNT {
      assert!(unsafe { (&(*state.running.0.get()))[n] }); // cc:174
      assert!(!unsafe { (&(*state.finished.0.get()))[n] }); // cc:175
    }
  }

  // Tell all threads to quit. cc:180-189
  {
    let _lock_guard = MutexGuard::new(&state.mutex);
    for n in (0..K_THREAD_COUNT).rev() {
      assert!(unsafe { (&(*state.running.0.get()))[n] }); // cc:183
      assert!(!unsafe { (&(*state.finished.0.get()))[n] }); // cc:184
      unsafe { (&mut (*state.running.0.get()))[n] = false }; // cc:186
    }
    state.cv.notify_all(); // cc:188
  }

  // Wait for all threads to quit. cc:192-199
  {
    let _lock_guard = MutexGuard::new(&state.mutex);
    for n in 0..K_THREAD_COUNT {
      while !unsafe { (&(*state.finished.0.get()))[n] } {
        state.cv.wait(&state.mutex);
      }
    }
  }

  // Make sure all threads are finished. cc:202-208
  {
    let _lock_guard = MutexGuard::new(&state.mutex);
    for n in (0..K_THREAD_COUNT).rev() {
      assert!(!unsafe { (&(*state.running.0.get()))[n] }); // cc:205
      assert!(unsafe { (&(*state.finished.0.get()))[n] }); // cc:206
    }
  }

  // Join all threads. cc:211-213
  for mut thread in threads {
    assert!(thread.join().is_ok());
  }
  OK
}

// LoopIncrementThread (cc:219-262)
struct LoopState {
  counter: Shared<i32>,
  cv: ConditionVariable,
  mutex: Mutex,
}

// TEST(ConditionVariable, LoopIncrement) cc:267-296
#[test]
fn condition_variable_loop_increment() -> Void {
  const K_MAX_THREAD_COUNT: i32 = 16; // cc:268
  let state = Arc::new(LoopState {
    counter: Shared(UnsafeCell::new(0)),
    cv: ConditionVariable::new(),
    mutex: Mutex::new(),
  });
  for thread_count in 1..K_MAX_THREAD_COUNT {
    let limit = thread_count * 10; // cc:272
    unsafe { *state.counter.0.get() = 0 }; // cc:273 int counter = 0
    let violations = Arc::new(AtomicUsize::new(0));

    // cc:277-280 Setup the threads（EXPECT_LT/EXPECT_EQ 构造断言在主线程）
    assert!(0 < thread_count, "cc:230 EXPECT_LT(rem, thread_count)");
    assert_eq!(
      0,
      limit % thread_count,
      "cc:231 EXPECT_EQ(0, limit % thread_count)"
    );
    let mut threads: Vec<Thread> = (0..thread_count)
      .map(|rem| {
        let st = Arc::clone(&state);
        let viol = Arc::clone(&violations);
        Thread::new(Options::new("LoopIncrementThread"), move || {
          let mut last_count: i32 = -1;
          loop {
            let _lock_guard = MutexGuard::new(&st.mutex);
            let mut count = unsafe { *st.counter.0.get() };
            while count % thread_count != rem && count < limit {
              st.cv.wait(&st.mutex);
              count = unsafe { *st.counter.0.get() };
            }
            if count >= limit {
              break;
            }
            // cc:244 EXPECT_EQ(*counter_, count)
            if unsafe { *st.counter.0.get() } != count {
              viol.fetch_add(1, Ordering::Relaxed);
            }
            // cc:246 EXPECT_EQ(last_count + (thread_count_ - 1), count)
            if last_count != -1 && last_count + (thread_count - 1) != count {
              viol.fetch_add(1, Ordering::Relaxed);
            }
            count += 1;
            unsafe { *st.counter.0.get() = count };
            last_count = count;
            st.cv.notify_all();
          }
        })
      })
      .collect();

    // Start all threads. cc:283-285 逆序
    for n in (0..thread_count as usize).rev() {
      assert!(threads[n].start().is_ok());
    }

    // Join and cleanup. cc:288-291
    for t in &mut threads {
      assert!(t.join().is_ok());
    }

    assert_eq!(0, violations.load(Ordering::Relaxed), "worker Run 内不变量");
    assert_eq!(
      limit,
      unsafe { *state.counter.0.get() },
      "cc:294 EXPECT_EQ(limit, counter)"
    );
  }
  OK
}
