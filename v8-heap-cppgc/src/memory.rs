// 由 tool/src/heap/cppgc-internal/memory.js 从 v8/src/heap/cppgc-internal/memory.h + src/heap/cppgc-internal/memory.cc 生成; 修订就地改, 复跑覆盖
// 分配/释放内存访问模型; 目标配置非 sanitizer 非 DEBUG

use core::ptr::{write_bytes, write_volatile};

/// v8: src/heap/cppgc-internal/memory.h kZappedValue
/// 最低位为 0 → 被 zap 的对象永不算完整构造
pub const K_ZAPPED_VALUE: u8 = 0xdc;

/// v8: src/heap/cppgc-internal/memory.cc NoSanitizeMemset — volatile 逐字节写, 绕过 sanitizer 截获
///
/// # Safety
/// `address` 起 `bytes` 字节须为可写有效内存。
pub unsafe fn no_sanitize_memset(address: *mut u8, c: u8, bytes: usize) {
  // SAFETY: 调用契约保证 [address, address+bytes) 可写。
  for i in 0..bytes {
    unsafe { write_volatile(address.add(i), c) };
  }
}

/// v8: src/heap/cppgc-internal/memory.h ZapMemory
///
/// # Safety
/// `address` 起 `size` 字节须为可写有效内存。
pub unsafe fn zap_memory(address: *mut u8, size: usize) {
  // SAFETY: 调用契约保证 [address, address+size) 可写。
  unsafe { write_bytes(address, K_ZAPPED_VALUE, size) };
}

/// v8: src/heap/cppgc-internal/memory.h CheckMemoryIsZapped
/// 原版 DEBUG-only CHECK; Rust 侧以 debug_assert 承接 (release 静默, 无 panic 出口)。
///
/// # Safety
/// 调用方须确保当 `address` 非空时，`[address, address + size)` 为有效可读内存。
pub unsafe fn check_memory_is_zapped(address: *const u8, size: usize) {
  if address.is_null() {
    return;
  }
  // SAFETY: 调用契约保证 address 非空时 [address, address + size) 为有效可读内存。
  debug_assert!((0..size).all(|i| unsafe { *address.add(i) } == K_ZAPPED_VALUE));
}

/// v8: src/heap/cppgc-internal/memory.h CheckMemoryIsZero
///
/// # Safety
/// 调用方须确保当 `address` 非空时，`[address, address + size)` 为有效可读内存。
pub unsafe fn check_memory_is_zero(address: *const u8, size: usize) {
  if address.is_null() {
    return;
  }
  // SAFETY: 调用契约保证 address 非空时 [address, address + size) 为有效可读内存。
  debug_assert!((0..size).all(|i| unsafe { *address.add(i) } == 0));
}

/// v8: src/heap/cppgc-internal/memory.h CheckMemoryIsInaccessible
/// 目标配置 (release) 为空操作; debug 下可校验 zapped。
///
/// # Safety
/// 当 `address` 非空且 debug_assertions 时，`[address, address + size)` 须为有效可读内存。
pub unsafe fn check_memory_is_inaccessible(address: *const u8, size: usize) {
  #[cfg(debug_assertions)]
  unsafe {
    check_memory_is_zapped(address, size);
  };
  #[cfg(not(debug_assertions))]
  let _ = (address, size);
}

/// v8: src/heap/cppgc-internal/memory.h CPPGC_SHOULD_ZAP_MEMORY
/// MSAN || ASAN || DEBUG → 1; 目标配置 (release, 无 sanitizer) → 0。
/// 本席不声明 sanitizer feature, sanitizer 臂恒不启用, 以 debug_assertions 代表 DEBUG 门。
pub const CPPGC_SHOULD_ZAP_MEMORY: bool = cfg!(debug_assertions);

/// v8: src/heap/cppgc-internal/memory.h SetMemoryAccessible
/// 目标配置非 sanitizer 非 DEBUG → 空操作 (sanitizer 臂属裁剪面)。
pub fn set_memory_accessible(_address: *mut u8, _size: usize) {}

/// v8: src/heap/cppgc-internal/memory.h SetMemoryInaccessible
/// release 分支: 清零 (原版 memset 0 + ASAN POISON; POISON 臂属 sanitizer 裁剪面)。
///
/// # Safety
/// `address` 起 `size` 字节须为可写有效内存。
pub unsafe fn set_memory_inaccessible(address: *mut u8, size: usize) {
  // SAFETY: 调用契约保证 [address, address+size) 可写。
  unsafe { write_bytes(address, 0, size) };
}

/// v8: src/heap/cppgc-internal/memory.h CheckMemoryIsInaccessibleIsNoop
/// 目标配置 (无 MSAN/ASAN, release) → true (sanitizer/DEBUG 侧为 false, 属裁剪面)。
pub const CHECK_MEMORY_IS_INACCESSIBLE_IS_NOOP: bool = true;
