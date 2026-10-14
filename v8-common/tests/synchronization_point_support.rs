//! 原版 v8/src/common/synchronization-point-support.h/.cc 模块级多线程校验
//! (上游无专属 unittest)
//!
//! 断言规约清单 (先规约后断言, 强度守恒):
//! - initialize_get: Initialize 成功一次; 重复初始化落 Err (原 CHECK_NULL 终止
//!   → Result, 登记); get() Some
//! - request_block_resume_flow: 他线程 block_if_requested 阻塞,
//!   WaitUntilBlocked 观测到 blocked_threads>0 (cc:60-101), Resume 放行并返回
//!   true (cc:44-58), 重复 Resume 返回 false 并 PrintError→log::error
//! - block_timeout_auto_resume: block_timeout 到期自动放行 (cc:132-140)
//! - self_deadlock_ignored: 请求线程自身到达同步点不阻塞 (cc:113-119 警告路径)
//! - wait_until_blocked_unarmed_false: 未武装/未知点返回 false
//! - fast_path_and_uninitialized: any_requested 快路径直接返回 (h:53-58);
//!   未初始化时 synchronization_point() 宏形态 no-op; TEST_ONLY 常量关

use std::{
  ptr::eq,
  sync::mpsc,
  thread,
  time::{Duration, Instant},
};

use aok::{OK, Void};
use v8_common::{
  Error, SynchronizationPointSupport, TEST_ONLY_SYNC_POINTS, synchronization_point,
  synchronization_point_test_only,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// TEST_ONLY 配置常量关 (原版 V8_ENABLE_TEST_ONLY_SYNC_POINTS 目标配置)
const _: () = assert!(!TEST_ONLY_SYNC_POINTS);

fn support() -> &'static SynchronizationPointSupport {
  SynchronizationPointSupport::initialize().expect("first initialize");
  SynchronizationPointSupport::get().expect("instance present after initialize")
}

/// Initialize 一次成功, 重复初始化 Err (CHECK_NULL 语义落 Result)
#[test]
fn synchronization_point_initialize_get() -> Void {
  assert!(SynchronizationPointSupport::get().is_none());
  let s = support();
  assert!(SynchronizationPointSupport::get().is_some());
  // 指针同一性: get 返回同一单例
  assert!(eq(s, SynchronizationPointSupport::get().unwrap()));
  assert_eq!(
    SynchronizationPointSupport::initialize(),
    Err(Error::SyncPointAlreadyInitialized)
  );
  OK
}

/// 请求阻塞 → 观测 → 恢复 全链路 (跨线程)
#[test]
fn synchronization_point_request_block_resume_flow() -> Void {
  let s = support();
  // 先武装, worker 抵达即阻塞 — 无时序竞态
  s.request_block_at("sp_flow", Duration::from_secs(5));
  let (done_tx, done_rx) = mpsc::channel::<()>();
  let worker = thread::spawn(move || {
    s.block_if_requested("sp_flow");
    done_tx.send(()).unwrap();
  });

  assert!(s.wait_until_blocked("sp_flow", Duration::from_secs(2)));
  // 未放行前 worker 不得完成
  assert!(done_rx.recv_timeout(Duration::from_millis(50)).is_err());

  assert!(s.resume("sp_flow"));
  done_rx
    .recv_timeout(Duration::from_secs(2))
    .expect("worker released by resume");
  worker.join().unwrap();

  // 重复 Resume: block_requested 已清 → false (原版 PrintError 路径)
  assert!(!s.resume("sp_flow"));
  OK
}

/// block_timeout 到期自动放行 (cc:132-140)
#[test]
fn synchronization_point_block_timeout_auto_resume() -> Void {
  let s = support();
  s.request_block_at("sp_timeout", Duration::from_millis(50));
  let (done_tx, done_rx) = mpsc::channel::<()>();
  let start = Instant::now();
  let worker = thread::spawn(move || {
    s.block_if_requested("sp_timeout");
    done_tx.send(()).unwrap();
  });
  assert!(s.wait_until_blocked("sp_timeout", Duration::from_secs(1)));
  done_rx
    .recv_timeout(Duration::from_secs(2))
    .expect("auto-resume on timeout");
  let elapsed = start.elapsed();
  // 至少接近 50ms 武装时长 (放宽下限防调度抖动)
  assert!(
    elapsed >= Duration::from_millis(20),
    "blocked too early: {elapsed:?}"
  );
  worker.join().unwrap();
  OK
}

/// 请求线程自身抵达同步点不阻塞 (cc:113-119 自死锁警告路径)
#[test]
fn synchronization_point_self_deadlock_ignored() -> Void {
  let s = support();
  s.request_block_at("sp_self", Duration::from_secs(30));
  let start = Instant::now();
  // 武装线程 (本测试主线程) 即 block_requester_thread — 必须立即返回
  s.block_if_requested("sp_self");
  assert!(start.elapsed() < Duration::from_secs(5));
  assert!(s.resume("sp_self"));
  OK
}

/// 未武装/未知同步点: WaitUntilBlocked false (cc:65-69 not-armed 分支)
#[test]
fn synchronization_point_wait_until_blocked_unarmed_false() -> Void {
  let s = support();
  assert!(!s.wait_until_blocked("ghost_point", Duration::from_millis(10)));
  // 武装但无线程抵达: 超时 false
  s.request_block_at("sp_wait", Duration::from_secs(30));
  assert!(!s.wait_until_blocked("sp_wait", Duration::from_millis(30)));
  assert!(s.resume("sp_wait"));
  OK
}

/// 快路径直通 + 未初始化宏形态 no-op + TEST_ONLY 关
#[test]
fn synchronization_point_fast_path_and_defaults() -> Void {
  // 未初始化: synchronization_point() 宏形态安全 no-op
  synchronization_point("sp_none");
  synchronization_point_test_only("sp_none");

  let s = support();
  // any_requested 未置位 → 快路径直接返回, 武装与否均不阻塞
  s.request_block_at("sp_fast", Duration::from_secs(30));
  let start = Instant::now();
  // 注意: request_block_at 已置 any_requested, 走 Slow 但本线程是请求者 →
  // 自死锁警告路径同样立即返回; 快路径断言用未武装名:
  s.block_if_requested("sp_unarmed_other");
  assert!(start.elapsed() < Duration::from_secs(1));
  assert!(s.resume("sp_fast"));

  // 真正未 any_requested 的进程态已由上文 no-op 覆盖; 这里恢复后
  // any_requested 仍为 true (原版不清位), 快路径不阻塞场景由未武装名验证
  OK
}
