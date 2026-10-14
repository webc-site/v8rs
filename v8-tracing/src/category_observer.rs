// 由 tool/src/tracing/tracing-category-observer.h.js 从 v8/src/tracing/tracing-category-observer.h;src/tracing/tracing-category-observer.cc 生成; 修订就地改, 复跑覆盖
// Mode 位+启停映射表, 挂接语义就地修订

//! TracingCategoryObserver (tracing-category-observer.h/.cc, 47+104 行)。
//!
//! 原版 no-perfetto 分支: 实现 `TracingController::TraceStateObserver`
//! (OnTraceEnabled/OnTraceDisabled), SetUp 时挂到平台 controller, 依据
//! disabled-by-default 类别组是否启用, 对 `i::TracingFlags` 原子位 fetch_or/fetch_and。
//! Rust 侧 TracingFlags 静态在 v8-logging (tracing_flags.rs) — 直接复用 (其
//! ENABLED_BY_* 常量标【待回收】, 与本模块 Mode 位数值一致)。
//! perfetto 分支 (TrackEventSessionObserver) 【推迟登记: 依赖 perfetto SDK】。

use std::sync::{Arc, Mutex, atomic::Ordering};

use v8_logging::{GC, GC_STATS, IC_STATS, RUNTIME_STATS, ZONE_STATS};

use crate::{
  Error,
  platform::TraceStateObserver,
  trace_event::{add_trace_state_observer, category_enabled, remove_trace_state_observer},
};

// ---- Mode (tracing-category-observer.h:21-25) ----
/// ENABLED_BY_NATIVE (native 采样器置位)。
pub const ENABLED_BY_NATIVE: u32 = 1 << 0;
/// ENABLED_BY_TRACING (tracing 启用置位)。
pub const ENABLED_BY_TRACING: u32 = 1 << 1;
/// ENABLED_BY_SAMPLING (采样模式置位)。
pub const ENABLED_BY_SAMPLING: u32 = 1 << 2;

// ---- OnTraceEnabled 6 映射的类别组 (tracing-category-observer.cc:46-82) ----
/// TRACE_DISABLED_BY_DEFAULT("v8.runtime_stats")。
const CAT_RUNTIME_STATS: &str = "disabled-by-default-v8.runtime_stats";
/// TRACE_DISABLED_BY_DEFAULT("v8.runtime_stats_sampling")。
const CAT_RUNTIME_STATS_SAMPLING: &str = "disabled-by-default-v8.runtime_stats_sampling";
/// TRACE_DISABLED_BY_DEFAULT("v8.gc")。
const CAT_GC: &str = "disabled-by-default-v8.gc";
/// TRACE_DISABLED_BY_DEFAULT("v8.gc_stats")。
const CAT_GC_STATS: &str = "disabled-by-default-v8.gc_stats";
/// TRACE_DISABLED_BY_DEFAULT("v8.ic_stats")。
const CAT_IC_STATS: &str = "disabled-by-default-v8.ic_stats";
/// TRACE_DISABLED_BY_DEFAULT("v8.zone_stats")。
const CAT_ZONE_STATS: &str = "disabled-by-default-v8.zone_stats";

/// 观察者实现体: 原版 `TracingCategoryObserver` 单例 (instance_, .cc:17) 的行为面。
struct Observer;

impl TraceStateObserver for Observer {
  /// `OnTraceEnabled` (tracing-category-observer.cc:43-82):
  /// 逐类别组查询启用 → 对应 TracingFlags 位 fetch_or。
  fn on_trace_enabled(&self) {
    if category_enabled(CAT_RUNTIME_STATS) {
      RUNTIME_STATS.fetch_or(ENABLED_BY_TRACING, Ordering::Relaxed);
    }
    if category_enabled(CAT_RUNTIME_STATS_SAMPLING) {
      RUNTIME_STATS.fetch_or(ENABLED_BY_SAMPLING, Ordering::Relaxed);
    }
    if category_enabled(CAT_GC) {
      GC.fetch_or(ENABLED_BY_TRACING, Ordering::Relaxed);
    }
    if category_enabled(CAT_GC_STATS) {
      GC_STATS.fetch_or(ENABLED_BY_TRACING, Ordering::Relaxed);
    }
    if category_enabled(CAT_IC_STATS) {
      IC_STATS.fetch_or(ENABLED_BY_TRACING, Ordering::Relaxed);
    }
    if category_enabled(CAT_ZONE_STATS) {
      ZONE_STATS.fetch_or(ENABLED_BY_TRACING, Ordering::Relaxed);
    }
  }

  /// `OnTraceDisabled` (tracing-category-observer.cc:91-100):
  /// 4 项位清除; zone_stats 不清除 (原版行为)。
  fn on_trace_disabled(&self) {
    RUNTIME_STATS.fetch_and(
      !(ENABLED_BY_TRACING | ENABLED_BY_SAMPLING),
      Ordering::Relaxed,
    );
    GC.fetch_and(!ENABLED_BY_TRACING, Ordering::Relaxed);
    GC_STATS.fetch_and(!ENABLED_BY_TRACING, Ordering::Relaxed);
    IC_STATS.fetch_and(!ENABLED_BY_TRACING, Ordering::Relaxed);
  }
}

/// `v8::tracing::TracingCategoryObserver` (tracing-category-observer.h:19-25):
/// 静态门面, 行为全在 `Observer`; 单例状态存 `INSTANCE` (instance_, .cc:17)。
pub struct TracingCategoryObserver;

/// 单例持有: controller 侧 observer 判同 (`Arc::ptr_eq`) 依赖同一 Arc。
static INSTANCE: Mutex<Option<Arc<Observer>>> = Mutex::new(None);

impl TracingCategoryObserver {
  /// `SetUp` (tracing-category-observer.cc:19-29): 挂到全局 controller;
  /// 重复 SetUp 报错。
  pub fn set_up() -> crate::Result<()> {
    let mut g = INSTANCE.lock().map_err(|_| Error::ObserverPoisoned)?;
    if g.is_some() {
      return Err(Error::ObserverInstalled);
    }
    let obs = Arc::new(Observer);
    add_trace_state_observer(Arc::clone(&obs) as Arc<dyn TraceStateObserver>);
    *g = Some(obs);
    Ok(())
  }

  /// `TearDown` (tracing-category-observer.cc:31-39): 从 controller 摘除并销毁;
  /// 锁中毒时无单例可摘, 静默返回 (本模块禁 panic → 锁不可中毒, 纯防御)。
  pub fn tear_down() {
    if let Ok(mut g) = INSTANCE.lock()
      && let Some(obs) = g.take()
    {
      let dyn_obs: Arc<dyn TraceStateObserver> = obs;
      remove_trace_state_observer(&dyn_obs);
    }
  }
}
