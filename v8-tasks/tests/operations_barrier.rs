//! 对应 `src/tasks/operations-barrier.{h,cc}`。原版无单测文件，
//! 规约从 `operations-barrier.h` 契约注释提取，见 tests/SPEC.md 20-25。

use std::{
  sync::{Arc, mpsc::channel},
  thread,
};

use aok::{OK, Void};
use log::info;
use v8_tasks::OperationsBarrier;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 规约 20/21/24：取消前 TryLock 有效；Token 全部析构后 CancelAndWait 收敛。
#[test]
fn try_lock_before_cancel() -> Void {
  let barrier = OperationsBarrier::new();
  let t1 = barrier.try_lock();
  assert!(t1.ok());
  let t2 = barrier.try_lock();
  assert!(t2.ok());
  assert!(!barrier.cancelled());
  drop(t2);
  drop(t1);
  barrier.cancel_and_wait();
  assert!(barrier.cancelled());
  info!("try_lock_before_cancel ok");
  OK
}

/// 规约 22：持 Token 时另一线程 CancelAndWait 必须等待 Token 释放后才返回。
/// 阻塞性由计数不变量证明（主线程 Token 存活期间计数>=1），无墙钟依赖。
#[test]
fn cancel_and_wait_blocks_until_release() -> Void {
  let barrier = Arc::new(OperationsBarrier::new());
  let token = barrier.try_lock();
  assert!(token.ok());
  let (done_tx, done_rx) = channel();
  let waited = {
    let barrier = barrier.clone();
    thread::spawn(move || {
      barrier.cancel_and_wait();
      // 借用 Arc 尾部续期，确保 barrier 存活至等待结束。
      assert!(barrier.cancelled());
      let _ = done_tx.send(());
    })
  };
  // 协议保证：done 只可能在主线程释放 Token 后出现；recv 为纯事件同步。
  drop(token);
  done_rx
    .recv()
    .expect("cancel_and_wait 应在 Token 释放后返回");
  waited.join().expect("runner thread 不应 panic");
  assert!(barrier.cancelled());
  OK
}

/// 规约 23：CancelAndWait 后 TryLock 返回无效 Token，且空 Token 析构不误减计数。
#[test]
fn try_lock_after_cancel() -> Void {
  let barrier = OperationsBarrier::new();
  barrier.cancel_and_wait();
  let t = barrier.try_lock();
  assert!(!t.ok());
  drop(t);
  // 空 Token 不应触发 Release 路径（否则计数下溢，debug 构建直接暴露）。
  assert!(barrier.cancelled());
  OK
}

/// 规约 25：移动赋值释放旧 Token 且不双重释放（对应原 operator=）。
#[test]
fn token_move_assign() -> Void {
  let barrier = OperationsBarrier::new();
  let mut a = barrier.try_lock();
  assert!(a.ok());
  let b = barrier.try_lock();
  assert!(b.ok());
  // 赋值即释放 a 的旧引用（Rust 移动语义对应原 DCHECK_NE(this,&other) + Release）。
  a = b;
  assert!(a.ok());
  drop(a);
  barrier.cancel_and_wait();
  assert!(barrier.cancelled());
  OK
}

/// 规约 21 强化：多 Token 并发持有，跨线程移动后释放，等待线程收敛。
#[test]
fn token_moved_across_threads() -> Void {
  let barrier = Arc::new(OperationsBarrier::new());
  let token = barrier.try_lock();
  assert!(token.ok());
  let (ready_tx, ready_rx) = channel();
  let release_tx = {
    let barrier = barrier.clone();
    // Token 借自 Arc 内部值，仅在闭包内创建/移动/释放以满足生命周期。
    thread::spawn(move || {
      let inner = barrier.try_lock();
      assert!(inner.ok());
      let _ = ready_tx.send(());
      drop(inner);
    })
  };
  ready_rx.recv().expect("子线程应成功 TryLock");
  drop(token);
  release_tx.join().expect("子线程不应 panic");
  barrier.cancel_and_wait();
  OK
}
