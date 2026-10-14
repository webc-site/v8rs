//! 原版 v8/src/base/sanitizer/asan.h — Rust 移植默认不启用 ASan, 全部退化为 no-op
//! (对应 !V8_USE_ADDRESS_SANITIZER 分支); 若未来接入 sanitizer 构建, 在此集中加 cfg。

use std::{hint::black_box, ptr::read_volatile};

/// ASAN_POISON_MEMORY_REGION — 非 ASan 构建为 no-op
#[inline]
pub fn asan_poison_memory_region(_addr: *const u8, _size: usize) {}

/// ASAN_UNPOISON_MEMORY_REGION — 非 ASan 构建为 no-op
#[inline]
pub fn asan_unpoison_memory_region(_addr: *const u8, _size: usize) {}

/// ASAN_CHECK_WHOLE_MEMORY_REGION_IS_POISONED — 非 ASan 构建为 no-op
#[inline]
pub fn asan_check_whole_memory_region_is_poisoned(_addr: *const u8, _size: usize) {}

/// AsanUnpoisonScope — 非 ASan 构建为空作用域
#[derive(Debug)]
pub struct AsanUnpoisonScope;

impl AsanUnpoisonScope {
  #[must_use]
  pub fn new(_addr: *const u8, _size: usize) -> Self {
    Self
  }
}

/// NoSanitizeProbeMemory — 以 volatile 读探测地址 (等价原版)
///
/// # Safety
/// 地址必须可访问, 否则进程收到 SIGSEGV (这正是原版的用途)。
#[inline]
pub unsafe fn no_sanitize_probe_memory(address: usize) {
  // SAFETY: 契约要求地址可读; 与原版 `*(volatile uint8_t*)address` 完全等价
  let v = unsafe { read_volatile(address as *const u8) };
  black_box(v);
}

// DISABLE_ASAN / DISABLE_HWASAN — C++ 属性注解, Rust 无对应, 登记即可。
