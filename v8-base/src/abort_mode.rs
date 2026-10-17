//! 原版 v8/src/base/abort-mode.h/cc — abort 行为模式全局量

use std::sync::atomic::{AtomicU8, Ordering};

/// AbortMode 枚举, 与原版变体 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AbortMode {
  /// 受控崩溃视为无害: DCHECK 变 no-op, CHECK 变 exit(0)
  ExitWithSuccessAndIgnoreDcheckFailures = 0,
  /// 同上但 exit 非零
  ExitWithFailureAndIgnoreDcheckFailures = 1,
  /// 无安全影响的致命错误直接退出
  ExitIfNoSecurityImpact = 2,
  /// DCHECK/CHECK 使用 IMMEDIATE_CRASH
  ImmediateCrash = 3,
  /// 默认: abort()
  Default = 4,
}

impl AbortMode {
  const fn to_u8(self) -> u8 {
    self as u8
  }

  fn from_u8(v: u8) -> Self {
    match v {
      0 => Self::ExitWithSuccessAndIgnoreDcheckFailures,
      1 => Self::ExitWithFailureAndIgnoreDcheckFailures,
      2 => Self::ExitIfNoSecurityImpact,
      3 => Self::ImmediateCrash,
      _ => Self::Default,
    }
  }
}

/// `AbortMode g_abort_mode` — 全局量, 原子存储保证线程可见
static G_ABORT_MODE: AtomicU8 = AtomicU8::new(AbortMode::Default.to_u8());

/// 读当前 abort 模式
#[must_use]
pub fn get_abort_mode() -> AbortMode {
  AbortMode::from_u8(G_ABORT_MODE.load(Ordering::Relaxed))
}

/// 写 abort 模式 (等价 g_abort_mode = ...)
pub fn set_abort_mode(mode: AbortMode) {
  G_ABORT_MODE.store(mode.to_u8(), Ordering::Relaxed);
}

/// ControlledCrashesAreHarmless()
#[must_use]
pub fn controlled_crashes_are_harmless() -> bool {
  matches!(
    get_abort_mode(),
    AbortMode::ExitWithSuccessAndIgnoreDcheckFailures
      | AbortMode::ExitWithFailureAndIgnoreDcheckFailures
  )
}

/// DcheckFailuresAreIgnored()
#[must_use]
pub fn dcheck_failures_are_ignored() -> bool {
  controlled_crashes_are_harmless()
}

/// FatalErrorsWithNoSecurityImpactShouldExit()
#[must_use]
pub fn fatal_errors_with_no_security_impact_should_exit() -> bool {
  matches!(get_abort_mode(), AbortMode::ExitIfNoSecurityImpact)
}
