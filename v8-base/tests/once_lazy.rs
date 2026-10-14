//! once + lazy_instance 线程语义对抗测试 (round3 审查任务 2d)
//!
//! 原版 once.h/once.cc 无专属 unittest; 本文件按 7.3 规约提取断言面:
//! - CallOnce 语义契约 (once.h:10-13): init_func 恰执行一次, 且任何调用
//!   在首次 init 完成前不返回 (无半构造暴露)。
//! - LazyInstance 契约 (lazy-instance.h:8-18): 构造仅一次, Get()/Pointer()
//!   返回同一完全初始化实例。
//!
//! 并发面: 多线程竞争初始化 (固定线程数与迭代数, 断言均为不变量, 不依赖交错序)。

use std::{
  sync::{
    Barrier,
    atomic::{AtomicUsize, Ordering},
  },
  thread,
};

use aok::{OK, Void};
use v8_base::{
  lazy_instance::{LazyInstance, SingleThreadLazyInstance},
  once::{ONCE_STATE_DONE, ONCE_STATE_UNINITIALIZED, OnceFlag, call_once, declare_once},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const THREADS: usize = 8;
const ITERATIONS: usize = 500;

/// call_once: 恰一次执行 + happens-before 可见性 (全部线程观察到同一实例)
#[test]
fn once_call_once_exactly_once() -> Void {
  let once = declare_once();
  let constructions = AtomicUsize::new(0);
  let payload = AtomicUsize::new(0);
  let barrier = Barrier::new(THREADS);
  let (once_ref, constructions_ref, payload_ref, barrier_ref) =
    (&once, &constructions, &payload, &barrier);

  thread::scope(|scope| {
    for _ in 0..THREADS {
      scope.spawn(move || {
        barrier_ref.wait();
        for _ in 0..ITERATIONS {
          call_once(once_ref, || {
            constructions_ref.fetch_add(1, Ordering::SeqCst);
            payload_ref.store(0xDEAD_BEEF, Ordering::Release);
          });
          // 快/慢路径返回后必观察到初始化值 (Release/Acquire 同步)
          assert_eq!(payload_ref.load(Ordering::Acquire), 0xDEAD_BEEF);
        }
        assert_eq!(once_ref.load(Ordering::Acquire), ONCE_STATE_DONE);
      });
    }
  });
  assert_eq!(constructions.load(Ordering::SeqCst), 1);
  OK
}

/// OnceFlag 封装: 同语义 + 状态可观察
#[test]
fn once_flag_wrapper() -> Void {
  let flag = OnceFlag::new();
  assert_eq!(flag.state(), ONCE_STATE_UNINITIALIZED);
  let mut seen = 0;
  for _ in 0..4 {
    flag.call_once(|| seen += 1);
  }
  assert_eq!(seen, 1);
  assert_eq!(flag.state(), ONCE_STATE_DONE);
  OK
}

/// LazyInstance: 多线程竞争 get_with, 构造恰一次, 全员同值同址
#[test]
fn lazy_instance_threadsafe_init_once() -> Void {
  static INSTANCE: LazyInstance<Heavy> = LazyInstance::new();
  let barrier = Barrier::new(THREADS);

  struct Heavy {
    magic: u64,
  }
  impl Default for Heavy {
    fn default() -> Self {
      Self {
        magic: 0x1234_5678_9ABC_DEF0,
      }
    }
  }
  let constructions = AtomicUsize::new(0);

  // get_with 自定义构造: 首个抢到的线程执行
  let barrier_ref = &barrier;
  let constructions_ref = &constructions;
  thread::scope(|scope| {
    for _ in 0..THREADS {
      scope.spawn(move || {
        barrier_ref.wait();
        for i in 0..ITERATIONS {
          let v = if i % 2 == 0 {
            INSTANCE.get_with(|| {
              constructions_ref.fetch_add(1, Ordering::SeqCst);
              Heavy { magic: 42 }
            })
          } else {
            INSTANCE.get()
          };
          // 两种入口必须收敛到同一实例 (同值同址)
          assert_eq!(v.magic, 42);
          assert_eq!(v as *const Heavy, INSTANCE.get() as *const Heavy);
        }
      });
    }
  });
  assert_eq!(constructions.load(Ordering::SeqCst), 1);
  assert!(INSTANCE.is_initialized());
  OK
}

/// SingleThreadLazyInstance: 单线程语义 (懒构造一次 + 复用)
#[test]
fn single_thread_lazy_instance() -> Void {
  let instance = SingleThreadLazyInstance::<u64>::new();
  let a = instance.get_with(|| 7);
  assert_eq!(*a, 7);
  // 再次 get_with: 已初始化, 构造闭包不再执行 (原值保留)
  let b = instance.get_with(|| panic!("SingleThreadLazyInstance 二次构造违反契约"));
  assert_eq!(*b, 7);
  assert_eq!(*instance.get(), 7);
  OK
}
