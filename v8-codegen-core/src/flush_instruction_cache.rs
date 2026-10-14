// 由 tool/src/codegen/cpu-features.h.js 从 v8/src/codegen/flush-instruction-cache.{h,cc} 生成; 修订就地改, 复跑覆盖
// FlushInstructionCache 自由函数签名骨架

/// 原版 FlushInstructionCache(void* start, size_t size):
/// 刷新指令缓存区间：
/// - 指针为空或大小为 0 时安全退出
/// - x86_64: 执行内存与指令同步屏障
/// - aarch64 (macOS): 调用 libkern sys_icache_invalidate 并执行 ISB
/// - aarch64 (Linux): 调用 __clear_cache 并执行 ISB
/// - 其他平台: 执行原子屏障保证内存可见性
pub fn flush_instruction_cache(start: *mut u8, size: usize) {
  if start.is_null() || size == 0 {
    return;
  }

  #[cfg(target_arch = "x86_64")]
  {
    use core::sync::atomic::{Ordering, fence};
    fence(Ordering::SeqCst);
  }

  #[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
  {
    use core::{arch::asm, ffi::c_void};
    unsafe extern "C" {
      fn sys_icache_invalidate(start: *mut c_void, len: usize);
    }
    unsafe {
      sys_icache_invalidate(start.cast(), size);
      asm!("isb", options(nostack, preserves_flags));
    }
  }

  #[cfg(all(target_arch = "aarch64", not(target_vendor = "apple")))]
  {
    use core::{arch::asm, ffi::c_void};
    unsafe extern "C" {
      fn __clear_cache(begin: *mut c_void, end: *mut c_void);
    }
    unsafe {
      let end = start.wrapping_add(size);
      __clear_cache(start.cast(), end.cast());
      asm!("isb", options(nostack, preserves_flags));
    }
  }

  #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
  {
    use core::sync::atomic::{Ordering, fence};
    fence(Ordering::SeqCst);
  }
}
