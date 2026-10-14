//! 原版 v8/src/base/fpu.h/cc — 非规格化数刷零 (FTZ/DAZ) 控制
//! 机械展开为双平台显式实现: x86_64 走 MXCSR, arm64 走 FPCR。

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
use core::arch::asm;
use core::marker::PhantomData;

/// Intel: FTZ(bit15) | DAZ(bit6) = 0x8040 (kFlushDenormToZeroBits)
#[cfg(target_arch = "x86_64")]
const FLUSH_DENORM_TO_ZERO_BITS_X86: u32 = 0x8040;
/// ARM64: FPCR bit24 FZ (kFlushDenormToZeroBit)
#[cfg(target_arch = "aarch64")]
const FLUSH_DENORM_TO_ZERO_BIT_ARM: u64 = 1 << 24;

/// FPU — 静态读写 FTZ 状态
pub struct Fpu;

impl Fpu {
  #[must_use]
  pub fn get_flush_denormals() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
      let mut csr: u32 = 0;
      // SAFETY: x86_64 基线支持 SSE; stmxcsr 读无副作用
      unsafe {
        asm!("stmxcsr [{}]", in(reg) &mut csr, options(nostack, preserves_flags));
      }
      csr & FLUSH_DENORM_TO_ZERO_BITS_X86 != 0
    }
    #[cfg(target_arch = "aarch64")]
    {
      let v: u64;
      // SAFETY: 读 FPCR 无副作用, 与原版 mrs 等价
      unsafe { asm!("mrs {}, fpcr", out(reg) v, options(nomem, preserves_flags)) };
      v & FLUSH_DENORM_TO_ZERO_BIT_ARM != 0
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
      false
    }
  }

  pub fn set_flush_denormals(value: bool) {
    #[cfg(target_arch = "x86_64")]
    {
      let mut old: u32 = 0;
      // SAFETY: 同 get
      unsafe {
        asm!("stmxcsr [{}]", in(reg) &mut old, options(nostack, preserves_flags));
      }
      let new = if value {
        old | FLUSH_DENORM_TO_ZERO_BITS_X86
      } else {
        old & !FLUSH_DENORM_TO_ZERO_BITS_X86
      };
      // SAFETY: ldmxcsr 写 MXCSR, 与原版一致
      unsafe {
        asm!("ldmxcsr [{}]", in(reg) &new, options(nostack, preserves_flags));
      }
    }
    #[cfg(target_arch = "aarch64")]
    {
      let old: u64;
      // SAFETY: mrs/msr FPCR 与原版逐指令等价
      unsafe {
        asm!("mrs {}, fpcr", out(reg) old, options(nomem, preserves_flags));
        let new = if value {
          old | FLUSH_DENORM_TO_ZERO_BIT_ARM
        } else {
          old & !FLUSH_DENORM_TO_ZERO_BIT_ARM
        };
        asm!("msr fpcr, {}", in(reg) new, options(nomem, preserves_flags));
      };
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
      let _ = value;
    }
  }
}

/// FlushDenormalsScope — RAII 作用域, Drop 时恢复旧状态
/// (原版 FlushDenormalsScope; 跨线程移动不安全, 用 !Send 标记由借用系统约束)
pub struct FlushDenormalsScope {
  old_flush_state: bool,
  _marker: PhantomData<*const ()>,
}

impl FlushDenormalsScope {
  #[must_use]
  pub fn new(value: bool) -> Self {
    let old_flush_state = Fpu::get_flush_denormals();
    Fpu::set_flush_denormals(value);
    Self {
      old_flush_state,
      _marker: PhantomData,
    }
  }
}

impl Drop for FlushDenormalsScope {
  fn drop(&mut self) {
    Fpu::set_flush_denormals(self.old_flush_state);
  }
}
