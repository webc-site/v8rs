//! mutex-unittest.cc 1:1 迁移。规约见 tests/SPEC.md。
use aok::{OK, Void};
use log::info;
use v8_base_platform::{
  LazyMutex, LazyRecursiveMutex, LockGuard, Mutex, MutexGuard, RecursiveMutex,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// TEST(Mutex, LockGuardMutex) cc:20-24
#[test]
fn mutex_lock_guard_mutex() -> Void {
  info!("> mutex_lock_guard_mutex");
  let mutex = Mutex::new();
  {
    let _lock_guard = MutexGuard::new(&mutex);
  }
  // 强化: guard drop 后锁已释放
  assert!(mutex.try_lock());
  unsafe { mutex.unlock() };
  {
    let _lock_guard = MutexGuard::new(&mutex);
  }
  assert!(mutex.try_lock());
  unsafe { mutex.unlock() };
  OK
}

// TEST(Mutex, LockGuardRecursiveMutex) cc:27-34
#[test]
fn mutex_lock_guard_recursive_mutex() -> Void {
  let recursive_mutex = RecursiveMutex::new();
  {
    let _lock_guard = LockGuard::<RecursiveMutex>::new(&recursive_mutex);
  }
  assert!(recursive_mutex.try_lock());
  unsafe { recursive_mutex.unlock() };
  {
    let _lock_guard1 = LockGuard::<RecursiveMutex>::new(&recursive_mutex);
    let _lock_guard2 = LockGuard::<RecursiveMutex>::new(&recursive_mutex);
  }
  assert!(recursive_mutex.try_lock());
  unsafe { recursive_mutex.unlock() };
  OK
}

// TEST(Mutex, LockGuardLazyMutex) cc:37-41
// 原版: LazyMutex lazy_mutex = LAZY_MUTEX_INITIALIZER; 两次 MutexGuard 锁放。
#[test]
fn mutex_lock_guard_lazy_mutex() -> Void {
  info!("> mutex_lock_guard_lazy_mutex");
  let lazy_mutex: LazyMutex = LazyMutex::new();
  {
    let _lock_guard = MutexGuard::new(lazy_mutex.pointer());
  }
  {
    let _lock_guard = MutexGuard::new(lazy_mutex.pointer());
  }
  // 强化: guard drop 后锁已释放（对齐非 lazy 用例的强化断言）
  let mutex = lazy_mutex.pointer();
  assert!(mutex.try_lock());
  unsafe { mutex.unlock() };
  OK
}

// TEST(Mutex, LockGuardLazyRecursiveMutex) cc:44-51
#[test]
fn mutex_lock_guard_lazy_recursive_mutex() -> Void {
  let lazy_recursive_mutex: LazyRecursiveMutex = LazyRecursiveMutex::new();
  {
    let _lock_guard = LockGuard::<RecursiveMutex>::new(lazy_recursive_mutex.pointer());
  }
  {
    let _lock_guard1 = LockGuard::<RecursiveMutex>::new(lazy_recursive_mutex.pointer());
    let _lock_guard2 = LockGuard::<RecursiveMutex>::new(lazy_recursive_mutex.pointer());
  }
  // 强化: guard drop 后锁已释放
  let recursive_mutex = lazy_recursive_mutex.pointer();
  assert!(recursive_mutex.try_lock());
  unsafe { recursive_mutex.unlock() };
  OK
}

// TEST(Mutex, MultipleMutexes) cc:54-72
#[test]
fn mutex_multiple_mutexes() -> Void {
  let mutex1 = Mutex::new();
  let mutex2 = Mutex::new();
  let mutex3 = Mutex::new();
  // Order 1
  mutex1.lock();
  mutex2.lock();
  mutex3.lock();
  unsafe {
    mutex1.unlock();
    mutex2.unlock();
    mutex3.unlock();
  }
  // Order 2
  mutex1.lock();
  mutex2.lock();
  mutex3.lock();
  unsafe {
    mutex3.unlock();
    mutex2.unlock();
    mutex1.unlock();
  }
  // 强化: 两组序列后全部已释放
  for m in [&mutex1, &mutex2, &mutex3] {
    assert!(m.try_lock());
    unsafe { m.unlock() };
  }
  OK
}

// TEST(Mutex, MultipleRecursiveMutexes) cc:75-96
#[test]
fn mutex_multiple_recursive_mutexes() -> Void {
  let recursive_mutex1 = RecursiveMutex::new();
  let recursive_mutex2 = RecursiveMutex::new();
  // Order 1
  recursive_mutex1.lock();
  recursive_mutex2.lock();
  assert!(recursive_mutex1.try_lock()); // cc:81 EXPECT_TRUE
  assert!(recursive_mutex2.try_lock()); // cc:82 EXPECT_TRUE
  unsafe {
    recursive_mutex1.unlock();
    recursive_mutex1.unlock();
    recursive_mutex2.unlock();
    recursive_mutex2.unlock();
  }
  // Order 2
  recursive_mutex1.lock();
  assert!(recursive_mutex1.try_lock()); // cc:89 EXPECT_TRUE
  recursive_mutex2.lock();
  assert!(recursive_mutex2.try_lock()); // cc:91 EXPECT_TRUE
  unsafe {
    recursive_mutex2.unlock();
    recursive_mutex1.unlock();
    recursive_mutex2.unlock();
    recursive_mutex1.unlock();
  }
  // 强化: 收尾全部已释放
  assert!(recursive_mutex1.try_lock());
  unsafe { recursive_mutex1.unlock() };
  assert!(recursive_mutex2.try_lock());
  unsafe { recursive_mutex2.unlock() };
  OK
}
