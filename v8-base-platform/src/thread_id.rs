//! 稳定的当前线程 id；对应 V8 `OS::GetCurrentThreadId`。
//!
//! 偏差：Rust 无内建稳定的 thread-id 公开 API；unix 上以 `pthread_self()` 为基础；
//! 若返回 0 则回退到进程内递增的原子计数器。id 仅用于同一进程内的线程区分，
//! 不做跨启动可移植性保证；语义等价 V8 `int` 版本但用 `u64` 避免符号问题。

use std::{
  cell::Cell,
  sync::atomic::{AtomicU64, Ordering},
};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);
thread_local! {
  static CACHED_ID: Cell<u64> = const { Cell::new(0) };
}

#[inline]
fn allocate_fallback_id() -> u64 {
  CACHED_ID.with(|c| {
    let v = c.get();
    if v != 0 {
      return v;
    }
    let fresh = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    c.set(fresh);
    fresh
  })
}

/// 当前线程 id。同一线程返回值稳定；不同线程返回值不保证连续但互不相同（非零）。
pub fn current_thread_id() -> u64 {
  #[cfg(any(target_os = "macos", target_os = "linux", target_os = "android"))]
  {
    // SAFETY: `pthread_self` 是 POSIX 定义无副作用调用，在该线程生命周期内稳定。
    let raw = unsafe { libc::pthread_self() };
    // 在 glibc / darwin 上 pthread_t 是指针或整型，均可安全 cast 到 u64。
    let id = raw as u64;
    if id == 0 { allocate_fallback_id() } else { id }
  }
  #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "android")))]
  {
    allocate_fallback_id()
  }
}
