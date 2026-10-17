// 由 tool/src/trap-handler/handler-outside-posix.cc.js 从 v8/src/trap-handler/handler-outside-posix.cc 生成; 修订就地改, 复跑覆盖

//! posix 信号安装 (sigaction); 不运行在信号上下文
//! 规则区原文行区间: ["globals","register","remove"]
use std::{
  ffi::c_void,
  mem::zeroed,
  ptr::null_mut,
  sync::atomic::{AtomicBool, Ordering},
};

use crate::{
  inside_posix::{K_OOB_SIGNAL, handle_signal},
  internal::{Locked, th_check},
};

/// handler-outside-posix.cc:36 g_old_handler (安装时保存, fallback 恢复用)
static G_OLD_HANDLER: Locked<Option<libc::sigaction>> = Locked::new(None);
/// :40 g_is_default_signal_handler_registered
static G_REGISTERED: AtomicBool = AtomicBool::new(false);

/// G_OLD_HANDLER 的持锁窗口包裹: 屏蔽 kOobSignal → f → 恢复屏蔽字。
/// 原版 g_old_handler 无锁裸访问 (无死锁但存在数据竞争); Rust 侧上锁后,
/// remove_trap_handler 也会在信号处理器内被调 (handle_signal 接管失败路径,
/// 无 TrapHandlerGuard 保护) 并取同一把锁——若本线程在持锁窗口内被
/// kOobSignal 打断, 处理器内再次取锁将自旋死锁。窗口内屏蔽该信号彻底消除。
/// pthread_sigmask/sigemptyset/sigaddset 均为 async-signal-safe 调用。
fn with_old_handler<R>(f: impl FnOnce(&mut Option<libc::sigaction>) -> R) -> R {
  // SAFETY: sig 族调用均 async-signal-safe; zeroed 为 sigset_t POD 全零初值
  unsafe {
    let mut sigs: libc::sigset_t = zeroed();
    libc::sigemptyset(&mut sigs);
    libc::sigaddset(&mut sigs, K_OOB_SIGNAL);
    let mut old_mask: libc::sigset_t = zeroed();
    libc::pthread_sigmask(libc::SIG_BLOCK, &sigs, &mut old_mask);
    let r = f(&mut G_OLD_HANDLER.lock_outside());
    // 恢复进入窗口前的屏蔽字 (处理器内调用时即处理器自动屏蔽后的掩码)
    libc::pthread_sigmask(libc::SIG_SETMASK, &old_mask, null_mut());
    r
  }
}

/// handler-outside-posix.cc:44 RegisterDefaultTrapHandler
/// 安装 HandleSignal 为 kOobSignal 处理器; 失败向调用方报 false
pub fn register_default_trap_handler() -> bool {
  th_check!(!G_REGISTERED.load(Ordering::Relaxed));

  // SAFETY: zeroed 产生全零 sigaction POD 初值, 随后逐字段赋值;
  // sigaction/sigemptyset 为配置 API, 仅在此处 (非信号上下文) 调用
  unsafe {
    let mut action: libc::sigaction = zeroed();
    // fn item 先落 fn 指针再转整数 (避开 fn item 直转整数的截断歧义)
    let handler: extern "C" fn(i32, *mut libc::siginfo_t, *mut c_void) = handle_signal;
    action.sa_sigaction = handler as libc::sighandler_t;
    // :50 SA_ONSTACK: 若注册了 sigaltstack 则在备用栈上跑,
    // 栈指针损坏/栈溢出时保护处理器自身
    action.sa_flags = libc::SA_SIGINFO | libc::SA_ONSTACK;
    libc::sigemptyset(&mut action.sa_mask);

    let mut old: libc::sigaction = zeroed();
    // :61 安装失败 (非 0) → 报 false
    if libc::sigaction(K_OOB_SIGNAL, &action, &mut old) != 0 {
      return false;
    }
    // g_old_handler 访问经 with_old_handler: 屏蔽信号后再持锁 (见其 SAFETY 说明)
    with_old_handler(|h| *h = Some(old));
  }
  G_REGISTERED.store(true, Ordering::Relaxed);
  true
}

/// handler-outside-posix.cc:89 RemoveTrapHandler
/// 恢复安装前的原处理器 (供接管失败时 re-raise 回退)
pub fn remove_trap_handler() {
  if G_REGISTERED.load(Ordering::Relaxed) {
    // 取原处理器快照 (持锁窗口已屏蔽 kOobSignal, 处理器内重入取锁安全);
    // sigaction 为 Copy: 直接解引拷贝
    let old = with_old_handler(|h| *h);
    // SAFETY: old 为安装时保存的原处理器配置副本
    unsafe {
      if let Some(old) = old
        && libc::sigaction(K_OOB_SIGNAL, &old, null_mut()) == 0
      {
        G_REGISTERED.store(false, Ordering::Relaxed);
      }
    }
  }
}
