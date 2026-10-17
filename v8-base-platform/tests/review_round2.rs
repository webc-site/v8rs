//! 对抗审查 round2 修复回归用例（原版无 1:1 gtest 对应，守护本轮修复项）。
use std::{ptr::null_mut, sync::Arc, thread, time::Duration};

use aok::{OK, Void};
use log::info;
use v8_base_platform::{
  ConditionVariable, MemoryPermission, MemoryProtectionKey, Mutex, OS, Options, Permission,
  Semaphore, Thread, TimeDelta,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// `Semaphore::wait_for(TimeDelta::max())` 不得 panic（round2 修复：
/// `Instant::now() + dur` 溢出会 panic）；对应原版 sem_timedwait 远未来
/// timespec = 无限期等待（semaphore.cc:82-106），信号到达即 true。
#[test]
fn semaphore_wait_for_max_no_panic() -> Void {
  let sem = Arc::new(Semaphore::new(0));
  let s2 = sem.clone();
  thread::spawn(move || {
    thread::sleep(Duration::from_millis(20));
    s2.signal();
  });
  // 修复前：deadline 加法溢出立即 panic；修复后：等到 20ms 后的信号。
  assert!(
    sem.wait_for(TimeDelta::max()),
    "max 超时应等到信号返回 true"
  );
  OK
}

/// 正向快路径：计数 > 0 时 max 超时立即返回 true（fast-path 不经过 deadline）。
#[test]
fn semaphore_wait_for_max_fast_path() -> Void {
  let sem = Semaphore::new(1);
  assert!(sem.wait_for(TimeDelta::max()), "计数>0 应立即获取");
  assert_eq!(0, sem.count(), "获取后计数归零");
  OK
}

/// `ConditionVariable::wait_for(mutex, TimeDelta::max())` 不得 panic；
/// 通知到达返回 true（原版 absl WaitWithTimeout 远未来语义）。
#[test]
fn condition_variable_wait_for_max_no_panic() -> Void {
  let m = Arc::new(Mutex::new());
  let cv = Arc::new(ConditionVariable::new());
  let (m2, cv2) = (m.clone(), cv.clone());
  let t = thread::spawn(move || {
    let _g = m2.lock_guard();
    cv2.wait_for(&m2, TimeDelta::max())
  });
  thread::sleep(Duration::from_millis(20));
  {
    let _g = m.lock_guard();
    cv.notify_all();
  }
  assert!(t.join().unwrap(), "max 超时 + 通知 → true");
  OK
}

/// Thread 栈钳制（platform-posix.cc:1232-1233）：8KB（< PTHREAD_STACK_MIN=16KB）
/// 传入 start / start_synchronously 均应成功启动（构造时钳到下限，两路共享）；
/// darwin stack_size=0 默认 1MB（platform-posix.cc:1325-1329）同验。
#[test]
fn thread_stack_clamp_both_start_paths() -> Void {
  let mut t1 = Thread::new(Options::new("v8:rs2-a").with_stack(8 * 1024), || {});
  assert!(t1.start().is_ok(), "start 应钳到 PTHREAD_STACK_MIN 后成功");
  assert!(t1.join().is_ok());

  let mut t2 = Thread::new(Options::new("v8:rs2-b").with_stack(8 * 1024), || {});
  assert!(
    t2.start_synchronously().is_ok(),
    "start_synchronously 同样受钳（round2 修复前该路径漏钳）"
  );
  assert!(t2.join().is_ok());

  // darwin 默认栈路径（stack_size=0 → 1MB）。
  let mut t3 = Thread::new(Options::new("v8:rs2-c"), || {});
  assert!(t3.start().is_ok());
  assert!(t3.join().is_ok());
  OK
}

/// `ComputeRegisterMaskForPermissionSwitch(key, permission)`
/// （memory-protection-key.cc:142-153）：公式 `permission << (key * 2)`。
#[test]
fn mpk_register_mask_formula() -> Void {
  // key=0 DisableAccess(1) → 1。
  assert_eq!(
    1,
    MemoryProtectionKey::compute_register_mask_for_permission_switch(0, Permission::DisableAccess)
  );
  // key=3 DisableWrite(2) → 2 << 6 = 128。
  assert_eq!(
    128,
    MemoryProtectionKey::compute_register_mask_for_permission_switch(3, Permission::DisableWrite)
  );
  // 本机（非 linux x64）无 API、读键恒 NoRestrictions。
  if !cfg!(all(target_os = "linux", target_arch = "x86_64")) {
    assert!(!MemoryProtectionKey::has_memory_protection_key_apis());
    assert_eq!(
      Permission::NoRestrictions,
      MemoryProtectionKey::get_key_permission(0)
    );
  }
  OK
}

/// darwin RecommitPages 恒 Ok（platform-posix.cc:621-630 一律 return true）。
#[test]
fn os_recommit_pages_always_ok() -> Void {
  let page = OS::commit_page_size();
  // SAFETY: 页对齐分配并在用例内释放。
  let p =
    unsafe { OS::allocate(null_mut(), page, page, MemoryPermission::ReadWrite) }.expect("allocate");
  // SAFETY: 已分配的 RW 页，范围合法。
  assert!(unsafe { OS::recommit_pages(p, page, MemoryPermission::ReadWrite) }.is_ok());
  // SAFETY: 释放。
  unsafe { OS::free(p, page) }.expect("free");
  info!("recommit ok");
  OK
}
