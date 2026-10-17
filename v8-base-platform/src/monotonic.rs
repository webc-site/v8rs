//! 单调时钟抽象：darwin 走 `mach_absolute_time`；linux 走 `clock_gettime(CLOCK_MONOTONIC)`；
//! 其他平台以 `std::time::Instant` 兜底。
//!
//! `thread_ticks_micros` / `thread_ticks_supported` 对应原 `ThreadTicks::Now / IsSupported`：
//! darwin 走 mach `thread_info(THREAD_BASIC_INFO)`（原 time.cc:53-81, 779-790 对 darwin
//! 返回 true）；linux 走 `clock_gettime(CLOCK_THREAD_CPUTIME_ID)`；其他平台返回 false。

#[cfg(any(target_os = "macos", all(not(target_os = "linux"), not(windows))))]
use std::sync::OnceLock;
#[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
use std::time::Instant;

#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Clone, Copy)]
struct MachTimebaseInfo {
  numer: u32,
  denom: u32,
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
  fn mach_absolute_time() -> u64;
  fn mach_timebase_info(info: *mut MachTimebaseInfo) -> i32;
}

#[cfg(target_os = "macos")]
static MACH_INFO: OnceLock<MachTimebaseInfo> = OnceLock::new();

/// 单调递增的微秒计数；`TimeTicks::now` 再 `+1` 以避免 null。
pub fn monotonic_micros() -> i64 {
  #[cfg(target_os = "macos")]
  {
    let info = *MACH_INFO.get_or_init(|| {
      let mut raw = MachTimebaseInfo { numer: 0, denom: 0 };
      // SAFETY: `mach_timebase_info` 是纯读取；写入栈局部。
      unsafe {
        mach_timebase_info(&mut raw);
      }
      raw
    });
    // SAFETY: `mach_absolute_time` 只读，无副作用。
    let ticks = unsafe { mach_absolute_time() };
    if info.denom == 0 {
      return ticks.min(i64::MAX as u64) as i64;
    }
    if info.numer == 1 && info.denom == 1 {
      (ticks / 1_000).min(i64::MAX as u64) as i64
    } else {
      let ns = (ticks as i128) * (info.numer as i128) / (info.denom as i128);
      let us = ns / 1_000;
      us.clamp(0, i64::MAX as i128) as i64
    }
  }
  #[cfg(target_os = "linux")]
  {
    #[repr(C)]
    struct Timespec {
      tv_sec: i64,
      tv_nsec: i64,
    }
    unsafe extern "C" {
      fn clock_gettime(clk_id: i32, tp: *mut Timespec) -> i32;
    }
    const CLOCK_MONOTONIC: i32 = 1;
    let mut ts = Timespec {
      tv_sec: 0,
      tv_nsec: 0,
    };
    // SAFETY: POSIX 定义线程安全调用；栈局部 timespec。
    unsafe {
      clock_gettime(CLOCK_MONOTONIC, &mut ts);
    }
    ts.tv_sec
      .saturating_mul(1_000_000)
      .saturating_add(ts.tv_nsec / 1_000)
  }
  #[cfg(windows)]
  {
    use std::sync::atomic::{AtomicI64, Ordering};
    static FREQ: AtomicI64 = AtomicI64::new(0);
    let mut freq = FREQ.load(Ordering::Relaxed);
    if freq <= 0 {
      let mut qpf = 0i64;
      // SAFETY: qpf 为栈局部变量有效指针，满足 QueryPerformanceFrequency 写入契约。
      unsafe {
        win_qpc::QueryPerformanceFrequency(&mut qpf);
      }
      freq = if qpf <= 0 { 10_000_000 } else { qpf };
      FREQ.store(freq, Ordering::Relaxed);
    }

    let mut qpc = 0i64;
    // SAFETY: qpc 为栈局部变量有效指针，满足 QueryPerformanceCounter 写入契约。
    unsafe {
      win_qpc::QueryPerformanceCounter(&mut qpc);
    }
    let us = (qpc as i128 * 1_000_000) / (freq as i128);
    us.clamp(0, i64::MAX as i128) as i64
  }
  #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
  {
    static BASE: OnceLock<Instant> = OnceLock::new();
    let base = *BASE.get_or_init(Instant::now);
    // 注入 100 秒基底偏置，防止刚启动时时间戳小于业务超时与 slack 门限
    const INITIAL_BIAS_US: i64 = 100_000_000;
    let elapsed = base.elapsed().as_micros().min(i64::MAX as u128) as i64;
    elapsed.saturating_add(INITIAL_BIAS_US)
  }
}

/// 高分辨率时钟；对应 `TimeTicks::IsHighResolution`。
pub fn is_high_resolution() -> bool {
  true
}

/// 是否支持 `ThreadTicks`；对应原 `ThreadTicks::IsSupported`（time.cc:779-790，
/// darwin/linux/android 等返回 true）。
pub fn thread_ticks_supported() -> bool {
  cfg!(any(target_os = "macos", target_os = "linux"))
}

/// darwin mach `thread_info(THREAD_BASIC_INFO)`；对应原 `ComputeThreadTicks`
/// （time.cc:53-81）。拿不到数据时返回 `None`（原 `CHECK_EQ` 会终止进程，
/// 本 crate 禁 panic，改由调用方落到 `ThreadTicks()` 空值语义）。
#[cfg(target_os = "macos")]
fn mach_thread_cpu_micros() -> Option<i64> {
  use crate::time::TimeConstants;

  /// mach/thread_info.h:89-103 `struct thread_basic_info`：10 个 4 字节字段。
  #[repr(C)]
  struct ThreadBasicInfo {
    user_seconds: i32,
    user_microseconds: i32,
    system_seconds: i32,
    system_microseconds: i32,
    cpu_usage: i32,
    policy: i32,
    run_state: i32,
    flags: i32,
    suspend_count: i32,
    sleep_time: i32,
  }

  unsafe extern "C" {
    /// mach/mach.h：`thread_act_t pthread_mach_thread_np(pthread_t)`，
    /// `thread_act_t` = `natural_t` = c_uint。
    fn pthread_mach_thread_np(thread: libc::pthread_t) -> u32;
    /// `kern_return_t thread_info(thread_act_t, thread_flavor_t, thread_info_t,
    /// mach_msg_type_number_t*)`。
    fn thread_info(thread: u32, flavor: u32, info: *mut i32, count: *mut u32) -> i32;
  }

  const THREAD_BASIC_INFO: u32 = 3;
  const THREAD_BASIC_INFO_COUNT: u32 = 10; // sizeof(thread_basic_info)/sizeof(natural_t)
  const KERN_SUCCESS: i32 = 0;

  let mut info = ThreadBasicInfo {
    user_seconds: 0,
    user_microseconds: 0,
    system_seconds: 0,
    system_microseconds: 0,
    cpu_usage: 0,
    policy: 0,
    run_state: 0,
    flags: 0,
    suspend_count: 0,
    sleep_time: 0,
  };
  let mut count = THREAD_BASIC_INFO_COUNT;
  // SAFETY: `pthread_self` 恒返回当前线程句柄；`thread_info` 仅向栈局部的
  // `ThreadBasicInfo`/`count` 写入，flavor 与 count 匹配（THREAD_BASIC_INFO_COUNT=10）。
  let (act, kr) = unsafe {
    let act = pthread_mach_thread_np(libc::pthread_self());
    let kr = thread_info(
      act,
      THREAD_BASIC_INFO,
      &mut info as *mut ThreadBasicInfo as *mut i32,
      &mut count,
    );
    (act, kr)
  };
  if act == 0 || kr != KERN_SUCCESS {
    return None;
  }
  // 原版 CHECK 保证不溢出；饱和运算等价（µs 结果远超 i64 时取边界值）。
  let seconds = i64::from(info.user_seconds).saturating_add(i64::from(info.system_seconds));
  let micros = seconds
    .saturating_mul(TimeConstants::MICROSECONDS_PER_SECOND)
    .saturating_add(i64::from(info.user_microseconds))
    .saturating_add(i64::from(info.system_microseconds));
  Some(micros)
}

/// 每线程 CPU 时间微秒；仅在 `thread_ticks_supported()` 为 true 时有意义。
pub fn thread_ticks_micros() -> i64 {
  #[cfg(target_os = "macos")]
  {
    mach_thread_cpu_micros().unwrap_or(0)
  }
  #[cfg(target_os = "linux")]
  {
    #[repr(C)]
    struct Timespec {
      tv_sec: i64,
      tv_nsec: i64,
    }
    unsafe extern "C" {
      fn clock_gettime(clk_id: i32, tp: *mut Timespec) -> i32;
    }
    const CLOCK_THREAD_CPUTIME_ID: i32 = 3;
    let mut ts = Timespec {
      tv_sec: 0,
      tv_nsec: 0,
    };
    // SAFETY: POSIX 线程安全调用；栈局部 timespec。
    unsafe {
      clock_gettime(CLOCK_THREAD_CPUTIME_ID, &mut ts);
    }
    ts.tv_sec
      .saturating_mul(1_000_000)
      .saturating_add(ts.tv_nsec / 1_000)
  }
  #[cfg(not(any(target_os = "macos", target_os = "linux")))]
  {
    0
  }
}

#[cfg(windows)]
mod win_qpc {
  #[link(name = "kernel32")]
  unsafe extern "system" {
    pub fn QueryPerformanceCounter(lpPerformanceCount: *mut i64) -> i32;
    pub fn QueryPerformanceFrequency(lpFrequency: *mut i64) -> i32;
  }
}
