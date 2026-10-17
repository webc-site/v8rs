//! yield_processor：对应 `src/base/platform/yield-processor.h`。
//!
//! 偏差：原实现在 TSan 下走 1ms 睡眠；此处只保留真实平台的暂停指令与通用回退。

#[cfg(target_arch = "aarch64")]
use std::arch::asm;
#[cfg(target_arch = "x86")]
use std::arch::x86::_mm_pause;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_mm_pause;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
use std::hint::spin_loop;

/// V8 `YieldProcessor::Yield()` 的等价函数。
#[inline(always)]
pub fn yield_processor() {
  #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
  {
    _mm_pause();
  }
  #[cfg(target_arch = "aarch64")]
  {
    // SAFETY: `isb` 仅同步本地指令流，不改变内存语义；空副作用。
    unsafe {
      asm!("isb sy", options(nomem, nostack, preserves_flags));
    }
  }
  #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
  {
    spin_loop();
  }
}

/// 结构体形式；与 V8 类 API 对齐。
#[derive(Debug, Clone, Copy, Default)]
pub struct YieldProcessor;

impl YieldProcessor {
  #[inline(always)]
  pub fn yield_cpu() {
    yield_processor();
  }
}
