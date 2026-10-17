//! v8/src/base/once.h + once.cc 的 1:1 转写 — pthread_once 的可移植模拟
//! 状态机与内存序逐条对齐原版:
//! - 状态常量 ONCE_STATE_* (uint8_t): kUninitialized=0 / kExecutingFunction=1 / kDone=2
//! - CallOnce 快路径: Acquire load != DONE 才进慢路径
//! - CallOnceImpl: Acquire load 复检 → CAS(Uninitialized→Executing, AcqRel)
//!   → 执行 init_func → Release store(DONE); 竞争者自旋 yield 直到非 EXECUTING
//! - 原版禁 panic 语境: init_func 若 panic 将破坏状态机, 约定调用方内部不用 panic 路径

use std::{
  ffi::c_void,
  sync::atomic::{AtomicU8, Ordering},
  thread,
};

/// 原版 OnceType = std::atomic<uint8_t>
pub type OnceType = AtomicU8;

/// 原版 enum : uint8_t { ONCE_STATE_UNINITIALIZED, ... }
pub const ONCE_STATE_UNINITIALIZED: u8 = 0;
/// 函数正在被某个线程执行
pub const ONCE_STATE_EXECUTING_FUNCTION: u8 = 1;
/// 函数已执行完成
pub const ONCE_STATE_DONE: u8 = 2;

/// V8_DECLARE_ONCE(NAME) / V8_ONCE_INIT: 静态初始化表达式
/// 原版保证 POD 无静态初始化器 → Rust const 构造
pub const fn declare_once() -> OnceType {
  AtomicU8::new(ONCE_STATE_UNINITIALIZED)
}

/// 原版 PointerArgFunction = void (*)(void* arg)
pub type PointerArgFunction = unsafe extern "C" fn(arg: *mut c_void);

/// once.cc: CallOnceImpl — 慢路径状态机, 内存序逐条对齐
/// 禁 panic: 返回 () 即保证 DONE 状态达成 (init_func 不得 panic, 由调用方约束)
pub fn call_once_impl(once: &OnceType, init_func: impl FnOnce()) {
  // Fast path. The provided function was already executed.
  if once.load(Ordering::Acquire) == ONCE_STATE_DONE {
    return;
  }

  // UNINITIALIZED: 首个调用线程; EXECUTING_FUNCTION: 另一线程在执行中
  // 先尝试 Uninitialized → ExecutingFunction 原子迁移
  let expected = ONCE_STATE_UNINITIALIZED;
  if once
    .compare_exchange(
      expected,
      ONCE_STATE_EXECUTING_FUNCTION,
      Ordering::AcqRel,
      Ordering::Acquire,
    )
    .is_ok()
  {
    // 本线程负责执行初始化函数
    init_func();
    once.store(ONCE_STATE_DONE, Ordering::Release);
  } else {
    // 另一线程已开始执行函数, 自旋让出直到初始化完成
    while once.load(Ordering::Acquire) == ONCE_STATE_EXECUTING_FUNCTION {
      // 原版 sched_yield()/Sleep(0)
      thread::yield_now();
    }
  }
}

/// CallOnce(OnceType*, init_func): 快路径 + 慢路径, 与原版 inline 包装一致
#[inline]
pub fn call_once(once: &OnceType, init_func: impl FnOnce()) {
  if once.load(Ordering::Acquire) != ONCE_STATE_DONE {
    call_once_impl(once, init_func);
  }
}

/// CallOnce 指针参数形态 (原版 PointerArgFunction + FunctionWithArgs 路径):
/// init_func 与用户 arg 均为裸指针, 跨 FFI 边界使用
///
/// # Safety
/// - `func` 必须是非空合法函数指针, 且可安全接收 `arg`
///   (与原版 C++ 对 void* 手工转型的同款约束)
/// - `once` 必须是有效 OnceType
#[inline]
pub unsafe fn call_once_with_arg(once: &OnceType, func: PointerArgFunction, arg: *mut c_void) {
  call_once(once, move || {
    // SAFETY: 调用方契约保证 func/arg 匹配且恰执行一次
    unsafe { func(arg) };
  });
}

/// 原版 `CallOnce(OnceType*, void (*init_func)())` 指针形态入口
/// (once.h:24 注释声明的用法; 实现体 once.h:88 收 std::function<void()>)
/// Rust 边界: `extern "C" fn()` 是函数指针, 须经闭包适配为 `FnOnce` 后进入
/// CallOnceImpl 状态机 —— 与原版把它包进 std::function 完全同构
#[inline]
pub fn call_once_legacy(once: &OnceType, init_func: extern "C" fn()) {
  call_once(once, move || init_func());
}

impl Default for OnceFlag {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

/// 便于 static 落地的新类型封装 (等价于直接持有 OnceType)
#[repr(transparent)]
pub struct OnceFlag(OnceType);

impl OnceFlag {
  /// V8_ONCE_INIT: const 构造, 无静态初始化器
  #[inline]
  pub const fn new() -> Self {
    Self(declare_once())
  }

  /// 暴露底层状态供上层 DCHECK/测试观察
  #[inline]
  pub fn state(&self) -> u8 {
    self.0.load(Ordering::Acquire)
  }

  /// 委托 CallOnce
  #[inline]
  pub fn call_once(&self, init_func: impl FnOnce()) {
    call_once(&self.0, init_func);
  }
}
