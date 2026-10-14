// 由 tool/src/codegen/arm64/cpu-arm64.cc.js 从 v8/src/codegen/arm64/cpu-arm64.cc 生成; 修订就地改, 复跑覆盖
// arm64 CacheLineSizes + FlushICache 平台分支登记

#[cfg(target_arch = "aarch64")]
use core::ffi::c_void;

/// 原版 CacheLineSizes: 读 ctr_el0 (ARM64 主机) 或置 0 (其他主机/Win/macOS)。
/// ExtractCacheLineSize: 4 << ((ctr >> shift) & 0xF) — 行大小以字为单位 2 的幂。
#[derive(Debug, Clone, Copy, Default)]
pub struct CacheLineSizes {
  cache_type_register_: u32,
}

/// icache 行大小字段位移 (原版 icache_line_size 调用 shift = 0)
pub const ICACHE_LINE_SIZE_SHIFT: i32 = 0;
/// dcache 行大小字段位移 (原版 shift = 16)
pub const DCACHE_LINE_SIZE_SHIFT: i32 = 16;

impl CacheLineSizes {
  /// 原版构造: V8_HOST_ARCH_ARM64 且非 Win/macOS → mrs ctr_el0; 否则 0。
  /// 本开发/目标环境按非 ARM64 主机处理 (登记: Linux ARM64 主机读 ctr_el0)。
  #[must_use]
  pub const fn new() -> Self {
    Self {
      cache_type_register_: 0,
    }
  }

  /// 原版 ExtractCacheLineSize(shift): 4 << ((ctr >> shift) & 0xF)
  #[inline]
  #[must_use]
  pub const fn extract_cache_line_size(&self, shift: i32) -> u32 {
    4 << ((self.cache_type_register_ >> shift) & 0xF)
  }

  /// 原版 icache_line_size()
  #[inline]
  #[must_use]
  pub const fn icache_line_size(&self) -> u32 {
    self.extract_cache_line_size(ICACHE_LINE_SIZE_SHIFT)
  }

  /// 原版 dcache_line_size()
  #[inline]
  #[must_use]
  pub const fn dcache_line_size(&self) -> u32 {
    self.extract_cache_line_size(DCACHE_LINE_SIZE_SHIFT)
  }
}

/// 原版 CpuFeatures::FlushICache(address, length) 平台分支登记:
/// - V8_OS_WIN    → ::FlushInstructionCache(GetCurrentProcess(), ...)
/// - V8_OS_DARWIN → sys_icache_invalidate(...)
/// - V8_OS_LINUX  → __builtin___clear_cache(begin, begin+length)
/// - 其他         → 用户态 cache 操作序列 (syscall 分支, 登记: 本目标取 darwin/linux)
/// - 非 ARM64 主机: 整体空操作。
pub fn flush_i_cache(address: *mut u8, length: usize) {
  if address.is_null() || length == 0 {
    return;
  }
  #[cfg(target_arch = "aarch64")]
  {
    #[cfg(target_os = "macos")]
    // SAFETY: sys_icache_invalidate 是 Darwin/AArch64 原生内核接口，使修改的代码在 I-Cache 中失效。
    // address 与 length 已进行非空与非零校验，由调用方确保为当前进程地址空间内有效的可执行代码段。
    unsafe {
      unsafe extern "C" {
        fn sys_icache_invalidate(start: *mut c_void, len: usize);
      }
      sys_icache_invalidate(address as *mut c_void, length);
    }
    #[cfg(target_os = "windows")]
    // SAFETY: FlushInstructionCache 是 Win32 API，
    // address 与 length 已进行非空与非零校验，指向已分配代码内存。
    unsafe {
      unsafe extern "system" {
        fn FlushInstructionCache(
          hProcess: *mut c_void,
          lpBaseAddress: *const c_void,
          dwSize: usize,
        ) -> i32;
        fn GetCurrentProcess() -> *mut c_void;
      }
      FlushInstructionCache(GetCurrentProcess(), address as *const c_void, length);
    }
    #[cfg(target_os = "linux")]
    // SAFETY: __clear_cache 是 GCC/Clang 内建缓存刷新函数，
    // address 非空且 length 非零，通过 address.add(length) 计算安全有效终点边界。
    unsafe {
      unsafe extern "C" {
        fn __clear_cache(start: *mut c_void, end: *mut c_void);
      }
      __clear_cache(address as *mut c_void, address.add(length) as *mut c_void);
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    // SAFETY: 执行 AArch64 ISB 同步屏障，丢弃流水线中旧预取指令。
    unsafe {
      use std::arch::aarch64::{__isb, SY};
      __isb(SY)
    };
  }
  #[cfg(not(target_arch = "aarch64"))]
  {
    let _ = (address, length);
  }
}
