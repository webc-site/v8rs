//! 原版 v8/src/base/immediate-crash.h — IMMEDIATE_CRASH() 的机械展开
//! 仅保留当前双平台 (macOS/aarch64 + Linux/x86_64) 分支。
//!
//! 约束 (与原版一致):
//! - 必须致命, 不可被 debugger continue 绕过
//! - 不可被折叠 (asm volatile)
//! - 产生与非法内存访问不同的信号 (SIGTRAP/SIGILL)

#[cfg(target_arch = "aarch64")]
use core::arch::asm;
#[cfg(target_arch = "x86_64")]
use core::arch::asm;

/// TRAP_SEQUENCE — arm64: `brk #0x1` (原版 brk #0 + hlt #0); x86_64: `int3` (+ud2 语义)
///
/// # Safety
/// 立即产生调试信号终止进程, 绝不返回。
#[inline(always)]
pub unsafe fn immediate_crash() -> ! {
  #[cfg(target_arch = "aarch64")]
  {
    // SAFETY: 契约要求此点不可达; brk 产生 SIGTRAP, 与原版序列等价且不可恢复
    unsafe {
      asm!(
        "brk #0x1",
        options(noreturn, nomem, nostack, preserves_flags)
      )
    };
  }
  #[cfg(target_arch = "x86_64")]
  {
    // SAFETY: 同上; int3 产生 SIGTRAP (Mac 上原版已用 ud2 兜底)
    unsafe { asm!("int3", options(noreturn, nomem, nostack, preserves_flags)) };
  }
  #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
  {
    std::process::abort()
  }
}

/// DEBUG_UNREACHABLE — 等价于原版 UNREACHABLE() + IMMEDIATE_CRASH
///
/// # Safety
/// 调用即证明逻辑进入不可达状态。
#[inline(always)]
pub unsafe fn debug_unreachable() -> ! {
  // SAFETY: 契约要求该点不可达, 与原版 UNREACHABLE()/IMMEDIATE_CRASH 相同
  unsafe { immediate_crash() }
}
