//! 对抗审查 round1 修复回归用例（原版无 1:1 对应，为修复项守护断言）。
use std::{
  env::temp_dir,
  ffi::c_void,
  fs::{create_dir_all, write},
  ptr::null_mut,
  thread::sleep,
  time::{Duration, Instant},
};

use aok::{OK, Void};
use log::info;
use v8_base_platform::{
  AddressSpaceReservation, FileMode, LazyConditionVariable, MemoryMappedFile, MemoryPermission,
  Mutex, OS, Time, TimeDelta,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// reservation::create 语义（platform-posix.cc:710-729）：预留块保持
/// PROT_NONE（max_permission 只是上限），alignment 生效；allocate 放开权限后可写；
/// free 回到 PROT_NONE（DecommitPages）。
#[test]
fn reservation_create_keeps_prot_none_and_honors_alignment() -> Void {
  let page = OS::commit_page_size();
  let alignment = 64 * 1024;
  // SAFETY: 分配的 reservation 在用例末尾释放。
  let r = unsafe {
    AddressSpaceReservation::create(
      null_mut(),
      alignment * 4,
      alignment,
      MemoryPermission::ReadWriteExecute,
    )
  }
  .expect("create reservation");
  assert_eq!(0, r.base() as usize % alignment, "base 应按 alignment 对齐");
  let base = r.base();
  // 放开第一页为 ReadWrite 并写入；未放开的第二页不触碰（PROT_NONE 写会 SIGSEGV）。
  // SAFETY: base..base+page 在 reservation 内。
  assert!(unsafe { r.allocate(base, page, MemoryPermission::ReadWrite).unwrap() });
  // SAFETY: 已放开为 ReadWrite。
  unsafe { (base as *mut u64).write_volatile(0xC0FFEE) };
  assert_eq!(0xC0FFEE, unsafe { (base as *const u64).read_volatile() });
  // SAFETY: free 回 PROT_NONE（decommit）。
  assert!(unsafe { r.free(base, page).unwrap() });
  // SAFETY: NoAccess 分支直接成功（platform-posix.cc:1154-1159）。
  assert!(unsafe { r.allocate(base, page, MemoryPermission::NoAccess).unwrap() });
  // SAFETY: 释放整块。
  unsafe { r.free_reservation() }.expect("free reservation");
  OK
}

/// MemoryMappedFile 空文件语义（platform-posix.cc:869/894）：
/// open 空文件 / create size=0 均返回有效 handle（size=0、memory=null）。
#[test]
fn memory_mapped_file_empty_file_valid_handle() -> Void {
  let dir = temp_dir().join("v8bp_r1_mmf");
  create_dir_all(&dir).unwrap();
  let empty = dir.join("empty.dat");
  write(&empty, b"").unwrap();
  let h =
    MemoryMappedFile::open(&empty, FileMode::ReadOnly).expect("空文件 open 应返回有效 handle");
  assert_eq!(0, h.size());
  assert!(h.memory().is_null());
  let h2 = MemoryMappedFile::create(dir.join("zero.dat"), 0, b"").unwrap();
  assert_eq!(0, h2.size());
  assert!(h2.memory().is_null());
  OK
}

/// TimeDelta::from_seconds_d / from_milliseconds_d（time.h:97-103）：
/// 有限值换算、NaN→0、±inf 饱和（saturated_cast 语义）。
#[test]
fn time_delta_from_double_saturated() -> Void {
  assert_eq!(TimeDelta::from_seconds_d(2.0), TimeDelta::from_seconds(2));
  assert_eq!(
    TimeDelta::from_milliseconds_d(2000.0),
    TimeDelta::from_milliseconds(2000)
  );
  assert!(TimeDelta::from_seconds_d(f64::NAN).is_zero());
  assert!(TimeDelta::from_seconds_d(f64::INFINITY).is_max());
  assert!(TimeDelta::from_seconds_d(f64::NEG_INFINITY).is_min());
  OK
}

/// Time::from_timespec 哨兵（time.cc:425-437）：仅 (i64::MAX, NS-1) 映射 max；
/// (i32::MAX, NS-1) 是普通时间（2038 秒值）。
#[test]
fn time_from_timespec_sentinel_only_i64_max() -> Void {
  let t = Time::from_timespec(i32::MAX as i64, 999_999_999);
  assert!(!t.is_max());
  assert!(!t.is_null());
  assert!(Time::from_timespec(i64::MAX, 999_999_999).is_max());
  OK
}

/// OS::get_user_time（platform-posix.cc:952-960）：来自 getrusage 的用户 CPU
/// 时间——睡眠 100ms 期间用户 CPU 几乎不增长（增量远小于墙钟）。
#[test]
fn os_get_user_time_is_cpu_not_wall() -> Void {
  let (s0, u0) = OS::get_user_time();
  let start = Instant::now();
  let wall_us = loop {
    // 自旋消耗少量用户 CPU，保证两个计数器都在动。
    if start.elapsed().as_micros() > 5_000 {
      break start.elapsed().as_micros() as i64;
    }
  };
  sleep(Duration::from_millis(100));
  let (s1, u1) = OS::get_user_time();
  let cpu_us = (i64::from(s1) - i64::from(s0)) * 1_000_000 + i64::from(u1) - i64::from(u0);
  let wall_total = wall_us + 100_000;
  assert!(cpu_us >= 0, "用户 CPU 时间不应倒退");
  assert!(
    cpu_us < wall_total,
    "睡眠期间 CPU 远小于墙钟: cpu={cpu_us}us"
  );
  OK
}

/// LazyConditionVariable（condition-variable.h:73-88）首次 pointer() 初始化冒烟。
#[test]
fn lazy_condition_variable_initializer_smoke() -> Void {
  info!("lazy cv smoke");
  static LAZY_CV: v8_base_platform::LazyConditionVariable = LazyConditionVariable::new();
  let m = Mutex::new();
  let cv = LAZY_CV.pointer();
  let _g = m.lock_guard();
  cv.notify_all();
  assert!(!cv.wait_for(&m, TimeDelta::from_microseconds(0)));
  OK
}

/// darwin 分配带 VM_MAKE_TAG(255) fd（platform-posix.cc:170-177）：
/// 分配-写-释放路径正常（tag 不影响可用性）。
#[test]
fn os_allocate_tagged_fd_roundtrip() -> Void {
  let page = OS::commit_page_size();
  // SAFETY: 页对齐分配并在用例内释放。
  let p = unsafe {
    OS::allocate(
      null_mut::<c_void>(),
      page,
      page,
      MemoryPermission::ReadWrite,
    )
  }
  .expect("allocate");
  // SAFETY: 已放开 ReadWrite。
  unsafe { (p as *mut u8).write_volatile(7) };
  assert_eq!(7, unsafe { *(p as *const u8) });
  // SAFETY: 释放。
  unsafe { OS::free(p, page) }.expect("free");
  OK
}
