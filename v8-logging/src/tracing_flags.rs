//! 1:1 转写自 `v8/src/logging/tracing-flags.h/.cc`。
//!
//! C++ 中为 `std::atomic_uint TracingFlags::{runtime_stats,gc,gc_stats,ic_stats,zone_stats}`
//! 全局静态,默认 0;`is_*_enabled()` 为 `load(relaxed) != 0`。

use std::sync::atomic::{AtomicU32, Ordering};

/// 对应 `v8::tracing::TracingCategoryObserver::ENABLED_BY_NATIVE`。
/// 【待回收】v8-tracing 转写完成后改用其常量。
pub const ENABLED_BY_NATIVE: u32 = 1 << 0;
/// 对应 `TracingCategoryObserver::ENABLED_BY_TRACING`。
pub const ENABLED_BY_TRACING: u32 = 1 << 1;
/// 对应 `TracingCategoryObserver::ENABLED_BY_SAMPLING`。
pub const ENABLED_BY_SAMPLING: u32 = 1 << 2;

/// 对应 `std::atomic_uint TracingFlags::runtime_stats`。
pub static RUNTIME_STATS: AtomicU32 = AtomicU32::new(0);
/// 对应 `std::atomic_uint TracingFlags::gc`。
pub static GC: AtomicU32 = AtomicU32::new(0);
/// 对应 `std::atomic_uint TracingFlags::gc_stats`。
pub static GC_STATS: AtomicU32 = AtomicU32::new(0);
/// 对应 `std::atomic_uint TracingFlags::ic_stats`。
pub static IC_STATS: AtomicU32 = AtomicU32::new(0);
/// 对应 `std::atomic_uint TracingFlags::zone_stats`。
pub static ZONE_STATS: AtomicU32 = AtomicU32::new(0);

pub struct TracingFlags;

impl TracingFlags {
  #[inline]
  pub fn is_runtime_stats_enabled() -> bool {
    RUNTIME_STATS.load(Ordering::Relaxed) != 0
  }

  #[inline]
  pub fn is_gc_enabled() -> bool {
    GC.load(Ordering::Relaxed) != 0
  }

  #[inline]
  pub fn is_gc_stats_enabled() -> bool {
    GC_STATS.load(Ordering::Relaxed) != 0
  }

  #[inline]
  pub fn is_ic_stats_enabled() -> bool {
    IC_STATS.load(Ordering::Relaxed) != 0
  }

  #[inline]
  pub fn is_zone_stats_enabled() -> bool {
    ZONE_STATS.load(Ordering::Relaxed) != 0
  }
}
