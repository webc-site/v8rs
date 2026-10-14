//! 原版 v8/src/base/build_config.h 的机械展开 (当前平台集: macOS/aarch64 + Linux/x86_64)

/// V8_HAS_PKU_SUPPORT: 仅 Linux x64
pub const HAS_PKU_SUPPORT: bool = cfg!(all(target_os = "linux", target_arch = "x86_64"));

/// V8_HAS_PKU_JIT_WRITE_PROTECT
pub const HAS_PKU_JIT_WRITE_PROTECT: bool = HAS_PKU_SUPPORT;

/// V8_HAS_PTHREAD_JIT_WRITE_PROTECT: 仅 arm64 macOS
pub const HAS_PTHREAD_JIT_WRITE_PROTECT: bool =
  cfg!(all(target_arch = "aarch64", target_os = "macos"));

/// V8_HAS_BECORE_JIT_WRITE_PROTECT: iOS 17.4+ — 本移植不支持 iOS
pub const HAS_BECORE_JIT_WRITE_PROTECT: bool = false;

/// V8_TARGET_ARCH_STORES_RETURN_ADDRESS_ON_STACK: x64/ia32 为 true
pub const TARGET_ARCH_STORES_RETURN_ADDRESS_ON_STACK: bool =
  cfg!(any(target_arch = "x86_64", target_arch = "x86"));

/// kReturnAddressStackSlotCount
pub const RETURN_ADDRESS_STACK_SLOT_COUNT: usize = if TARGET_ARCH_STORES_RETURN_ADDRESS_ON_STACK {
  1
} else {
  0
};

/// kPageSizeBits — V8 分页空间页数位宽 (非 PPC64 大页平台统一 18)
pub const PAGE_SIZE_BITS: u32 = 18;

/// kRegularPageSize
pub const REGULAR_PAGE_SIZE: usize = 1 << PAGE_SIZE_BITS;

/// kMinimumOSPageSize — 操作系统最小可保护页
pub const MINIMUM_OS_PAGE_SIZE: usize = minimum_os_page_size();

const fn minimum_os_page_size() -> usize {
  if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
    // macOS arm64 使用 16kB 页
    16 * 1024
  } else if cfg!(all(
    target_os = "linux",
    any(target_arch = "aarch64", target_arch = "powerpc64")
  )) {
    // Linux arm64/ppc64 可配置到 64kB
    64 * 1024
  } else {
    4 * 1024
  }
}

/// 目标平台为 little-endian (x64/arm64 均是)
pub const TARGET_LITTLE_ENDIAN: bool = true;
