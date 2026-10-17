#![cfg(unix)]

// 启用语义规约: trap-handler.h EnableTrapHandler / IsTrapHandlerEnabled 的
// 一次性与查询冻结 (nextest 每测试一进程, 进程级状态天然隔离)
use std::{mem::zeroed, ptr::null};

use aok::{OK, Void};
use log::info;
use v8_trap_handler::{Error, enable_trap_handler, is_trap_handler_enabled, remove_trap_handler};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 仅允许启用一次; 重复启用报 EnableTwice (原版 IMMEDIATE_CRASH 的 Err 化)
#[test]
fn enable_once() -> Void {
  assert!(matches!(enable_trap_handler(false), Ok(true)));
  assert!(is_trap_handler_enabled());
  assert!(matches!(
    enable_trap_handler(false),
    Err(Error::EnableTwice)
  ));
  // 未装信号 (use_v8_handler=false), remove 为幂等空操作
  remove_trap_handler();
  info!("enable_once OK");
  OK
}

/// 查询即冻结: is_trap_handler_enabled 调用后禁止再启用
#[test]
fn enable_query_freezes() -> Void {
  assert!(!is_trap_handler_enabled());
  assert!(matches!(
    enable_trap_handler(false),
    Err(Error::EnableTwice)
  ));
  OK
}

/// use_v8_handler=true: 安装 V8 默认信号处理器, 卸载恢复
#[test]
fn enable_with_v8_handler() -> Void {
  let sig = if cfg!(target_os = "macos") {
    libc::SIGBUS
  } else {
    libc::SIGSEGV
  };
  // SAFETY: sigaction 查询为配置 API, 非信号上下文
  unsafe {
    let mut before: libc::sigaction = zeroed();
    assert_eq!(libc::sigaction(sig, null(), &mut before), 0);

    assert!(matches!(enable_trap_handler(true), Ok(true)));
    assert!(is_trap_handler_enabled());

    let mut now: libc::sigaction = zeroed();
    assert_eq!(libc::sigaction(sig, null(), &mut now), 0);
    // 处理器已换装 (handle_signal)
    assert_ne!(now.sa_sigaction, before.sa_sigaction);

    remove_trap_handler();
    let mut after: libc::sigaction = zeroed();
    assert_eq!(libc::sigaction(sig, null(), &mut after), 0);
    assert_eq!(after.sa_sigaction, before.sa_sigaction);
  }
  OK
}
