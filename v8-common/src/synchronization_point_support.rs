// 由 tool/src/common.js 从 v8/src/common/synchronization-point-support.h/.cc 生成;
// 修订就地改, 复跑覆盖
//
// 修订 (Developer): 全语义落位 — RequestBlockAt / Resume / WaitUntilBlocked /
// BlockIfRequested(Slow)。base::TimeDelta/TimeTicks → Duration/Instant;
// base::OS::PrintError → log::error!; ThreadId::Invalid → Option<ThreadId>;
// states 表条目改 Arc<(Mutex, Condvar)> 独立锁对 (登记: 原 C++ 单 mutex +
// 每 cv, Rust Condvar 必须绑自身锁); LocalHeap::ExecuteWhileParked 分支依赖
// heap 层 → L3 接入点, 本 crate 走直等路径。
//
// 修订 (审查环 round5): F9 — 等待剩余时长改 `timeout.checked_sub(elapsed)`
// 承载 (rs:141 / rs:185 两处)。原 `(start + timeout)` 以 Instant + Duration
// 求截止: timeout 为合法 pub API 入参 (Duration 全域), 极大值在 debug 构建
// 直接 panic, 违反 crate 禁 panic 红线 (同 register_jit_allocations 溢出
// round1-D1 纪律); 原版 TimeTicks + TimeDelta 为整数微秒算术无 panic
// (synchronization-point-support.cc:83-85, 127-129)。减法形式数学等价
// (remaining = timeout - (now - start)), Duration::checked_sub 溢出即过期,
// 与原版 remaining ≤ 0 分支语义一致。

use core::{
  sync::atomic::{AtomicBool, Ordering},
  time::Duration,
};
use std::{
  collections::BTreeMap,
  sync::{Arc, OnceLock},
  thread::{self, ThreadId},
  time::Instant,
};

use parking_lot::{Condvar, Mutex};

use crate::error::{Error, Result};

/// SYNCHRONIZATION_POINT_TEST_ONLY 开关 (synchronization-point-support.h:93-98)
/// V8_ENABLE_TEST_ONLY_SYNC_POINTS 目标配置关
pub const TEST_ONLY_SYNC_POINTS: bool = false;

/// 单例 (原版 SynchronizationPointSupport* instance_)
static INSTANCE: OnceLock<SynchronizationPointSupport> = OnceLock::new();

/// 原版 SynchronizationPointSupport (synchronization-point-support.h:32-82) —
/// 测试/fuzzer 用的同步点阻塞
pub struct SynchronizationPointSupport {
  any_requested: AtomicBool,
  states: Mutex<BTreeMap<Box<str>, Arc<SyncPointState>>>,
}

struct SyncPointState {
  cv: Condvar,
  inner: Mutex<SyncPointInner>,
}

struct SyncPointInner {
  /// 请求到达此点的线程阻塞
  block_requested: bool,
  /// 已到达且当前阻塞的线程数
  blocked_threads: i32,
  /// 设置 block_requested 的线程
  block_requester_thread: Option<ThreadId>,
  /// 无恢复时的阻塞时长
  block_timeout: Duration,
}

impl SyncPointState {
  const fn new() -> Self {
    Self {
      cv: Condvar::new(),
      inner: Mutex::new(SyncPointInner {
        block_requested: false,
        blocked_threads: 0,
        block_requester_thread: None,
        block_timeout: Duration::ZERO,
      }),
    }
  }
}

impl SynchronizationPointSupport {
  /// 原版 Initialize (synchronization-point-support.cc:27-30) —
  /// CHECK_NULL(instance_) → 重复初始化落 Err (登记)
  pub fn initialize() -> Result<()> {
    INSTANCE
      .set(Self {
        any_requested: AtomicBool::new(false),
        states: Mutex::new(BTreeMap::new()),
      })
      .map_err(|_| Error::SyncPointAlreadyInitialized)
  }

  /// 原版 Get() (synchronization-point-support.h:35) — 未初始化为 None
  pub fn get() -> Option<&'static SynchronizationPointSupport> {
    INSTANCE.get()
  }

  /// 原版 RequestBlockAt (synchronization-point-support.cc:32-42)
  pub fn request_block_at(&self, synchronization_point: impl Into<Box<str>>, timeout: Duration) {
    self.any_requested.store(true, Ordering::Relaxed);
    let state = {
      let mut states = self.states.lock();
      states
        .entry(synchronization_point.into())
        .or_insert_with(|| Arc::new(SyncPointState::new()))
        .clone()
    };
    let mut inner = state.inner.lock();
    inner.block_requested = true;
    inner.block_requester_thread = Some(thread::current().id());
    inner.block_timeout = timeout;
  }

  /// 原版 Resume (synchronization-point-support.cc:44-58)
  pub fn resume(&self, synchronization_point: &str) -> bool {
    let state = {
      let states = self.states.lock();
      states.get(synchronization_point).cloned()
    };
    if let Some(state) = state {
      let mut inner = state.inner.lock();
      if inner.block_requested {
        inner.block_requested = false;
        state.cv.notify_all();
        return true;
      }
    }
    log::error!("Synchronization point not armed");
    false
  }

  /// 原版 WaitUntilBlocked (synchronization-point-support.cc:60-101) —
  /// 等到某线程阻塞在指定同步点; 未武装或超时返回 false
  pub fn wait_until_blocked(&self, synchronization_point: &str, timeout: Duration) -> bool {
    // 原版 LocalHeap::ExecuteWhileParked 分支 — L3 接入点, 此处直等
    let state = {
      let states = self.states.lock();
      states.get(synchronization_point).cloned()
    };
    let Some(state) = state else {
      log::error!("Synchronization point not armed");
      return false;
    };

    let start = Instant::now();
    let mut inner = state.inner.lock();
    while inner.blocked_threads == 0 {
      if !inner.block_requested {
        log::error!("Synchronization point not armed");
        return false;
      }
      // 剩余 = timeout - (now - start): 禁 Instant + Duration 溢出 panic
      // (round5-F9; 原版 TimeTicks + TimeDelta 整数微秒回绕无 panic, cc:83-85)
      let Some(remaining) = timeout.checked_sub(start.elapsed()) else {
        log::error!("Synchronization point wait timed out");
        return false;
      };
      state.cv.wait_for(&mut inner, remaining);
    }
    true
  }

  /// 原版 BlockIfRequested (synchronization-point-support.h:53-58) —
  /// 快路径: 无任何阻塞请求即返回
  #[inline]
  pub fn block_if_requested(&self, synchronization_point: &str) {
    if !self.any_requested.load(Ordering::Relaxed) {
      return;
    }
    self.block_if_requested_slow(synchronization_point);
  }

  /// 原版 BlockIfRequestedSlow (synchronization-point-support.cc:103-141)
  fn block_if_requested_slow(&self, synchronization_point: &str) {
    let current = thread::current().id();
    let state = {
      let states = self.states.lock();
      states.get(synchronization_point).cloned()
    };
    let Some(state) = state else {
      return;
    };

    let start = Instant::now();
    let mut inner = state.inner.lock();
    if !inner.block_requested {
      return;
    }
    if inner.block_requester_thread == Some(current) {
      log::error!(
        "Warning: ignoring self-deadlock at synchronization point '{synchronization_point}'"
      );
      return;
    }
    inner.blocked_threads += 1;
    state.cv.notify_all();
    while inner.block_requested {
      // 剩余 = block_timeout - (now - start): 同 round5-F9, 禁溢出 panic
      // (原版 TimeTicks + TimeDelta 回绕无 panic, cc:127-129)
      let Some(remaining) = inner.block_timeout.checked_sub(start.elapsed()) else {
        break;
      };
      state.cv.wait_for(&mut inner, remaining);
    }
    inner.blocked_threads -= 1;

    if inner.block_requested {
      log::error!(
        "Warning: Synchronization point '{synchronization_point}' timed out after {} ms. Resuming automatically.",
        inner.block_timeout.as_millis(),
      );
    }
  }
}

/// 原版 SYNCHRONIZATION_POINT 宏 (synchronization-point-support.h:86-88)
#[inline]
pub fn synchronization_point(sync_point_name: &str) {
  if let Some(s) = SynchronizationPointSupport::get() {
    s.block_if_requested(sync_point_name);
  }
}

/// 原版 SYNCHRONIZATION_POINT_TEST_ONLY (synchronization-point-support.h:93-98) —
/// TEST_ONLY_SYNC_POINTS 关 → no-op
#[inline]
pub fn synchronization_point_test_only(_sync_point_name: &str) {}
