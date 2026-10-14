// 由 tool/src/heap/base/memory-tagging.js 从 v8/src/heap/base/memory-tagging.h + memory-tagging.cc 生成; 修订就地改, 复跑覆盖
// Arm MTE 标签检查挂起域: 仅 linux+arm64 生效, 其余 no-op

//! Memory Tagging Extension 辅助: 扫栈期间挂起标签检查。
//! 生效条件 (机械提取): V8_OS_LINUX && V8_HOST_ARCH_ARM64。
//! 偏差: 原版读 TCO 校验的 CHECK 裁剪 (禁 panic); CPU::has_mte() →
//! is_aarch64_feature_detected!("mte")。

/// 原 SuspendTagCheckingScope (memory-tagging.h:15-20):
/// 生存期内经 PSTATE.TCO 挂起 MTE 标签检查。
pub struct SuspendTagCheckingScope {
  /// 是否真正动过 TCO (非 MTE 平台/false 时析构不动寄存器);
  /// 仅 linux+arm64 存在真实读写, 其余目标裁剪。
  #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
  suspended: bool,
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
impl SuspendTagCheckingScope {
  /// 原构造 (memory-tagging.cc:18-33): 检出 MTE 才置 TCO=1。
  pub fn new() -> Self {
    let has_mte = std::arch::is_aarch64_feature_detected!("mte");
    if has_mte {
      // SAFETY: msr tco 为进程内 PSTATE 位写, 无内存依赖。
      unsafe {
        std::arch::asm!(
          ".arch_extension memtag",
          "msr tco, #1",
          options(nostack, preserves_flags)
        );
      }
    }
    Self { suspended: has_mte }
  }
}

#[cfg(not(all(target_os = "linux", target_arch = "aarch64")))]
impl SuspendTagCheckingScope {
  /// 非 linux+arm64: no-op (对应原版 V8_HAS_MTE_SUPPORT 未定义分支)。
  pub fn new() -> Self {
    Self {}
  }
}

impl Default for SuspendTagCheckingScope {
  fn default() -> Self {
    Self::new()
  }
}

impl Drop for SuspendTagCheckingScope {
  /// 原析构 (memory-tagging.cc:35-44): msr tco,#0 复位。
  fn drop(&mut self) {
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    if self.suspended {
      // SAFETY: 见构造 — PSTATE.TCO 位写, 无内存依赖。
      unsafe {
        std::arch::asm!(
          ".arch_extension memtag",
          "msr tco, #0",
          options(nostack, preserves_flags)
        );
      }
    }
  }
}
