#!/usr/bin/env bun
// tool/src/codegen/arm64/cpu-arm64.cc.js
// 机械提取 v8/src/codegen/arm64/cpu-arm64.cc: CacheLineSizes + FlushICache 平台分支
// → v8-codegen-arm64/src/cpu.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/codegen/arm64/cpu-arm64.cc");

// 对数: CacheLineSizes + FlushICache 两个锚点
if (!src.includes("class CacheLineSizes {")) throw new Error("缺 CacheLineSizes");
if (!src.includes("void CpuFeatures::FlushICache(")) throw new Error("缺 FlushICache");
// ctr_el0 读取 (Linux GCC 分支)
if (!src.includes("ctr_el0")) throw new Error("缺 ctr_el0 锚点");

let rust = banner(
  "src/codegen/arm64/cpu-arm64.cc.js",
  "src/codegen/arm64/cpu-arm64.cc",
  "arm64 CacheLineSizes + FlushICache 平台分支登记",
);

rust += `
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
    Self { cache_type_register_: 0 }
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
  #[cfg(target_arch = "aarch64")]
  {
    #[cfg(target_os = "macos")]
    unsafe {
      unsafe extern "C" {
        fn sys_icache_invalidate(start: *mut std::ffi::c_void, len: usize);
      }
      sys_icache_invalidate(address as *mut std::ffi::c_void, length);
    }
    #[cfg(target_os = "windows")]
    unsafe {
      unsafe extern "system" {
        fn FlushInstructionCache(
          hProcess: *mut std::ffi::c_void,
          lpBaseAddress: *const std::ffi::c_void,
          dwSize: usize,
        ) -> i32;
        fn GetCurrentProcess() -> *mut std::ffi::c_void;
      }
      FlushInstructionCache(GetCurrentProcess(), address as *const std::ffi::c_void, length);
    }
    #[cfg(target_os = "linux")]
    unsafe {
      unsafe extern "C" {
        fn __clear_cache(start: *mut std::ffi::c_void, end: *mut std::ffi::c_void);
      }
      __clear_cache(
        address as *mut std::ffi::c_void,
        address.add(length) as *mut std::ffi::c_void,
      );
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
      unsafe { std::arch::aarch64::__isb(std::arch::aarch64::SY) };
    }
  }
  #[cfg(not(target_arch = "aarch64"))]
  {
    let _ = (address, length);
  }
}
`;

const p = writeSrc("v8-codegen-arm64", "cpu.rs", rust);
console.log(`cpu-arm64.cc: CacheLineSizes 1 / FlushICache 分支 4 → ${p}`);
