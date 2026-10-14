#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/memory.js — v8/src/heap/cppgc-internal/memory.h + .cc → v8-heap-cppgc/src/memory.rs.skel
// 内存访问模型: kZappedValue=0xdc + Zap/Check + sanitizer 分支 (目标配置: 不 zap)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/memory.h";
const REL_CC = "src/heap/cppgc-internal/memory.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

need(src, "V8_NOINLINE DISABLE_ASAN void NoSanitizeMemset(void* address, char c,", "NoSanitizeMemset 声明");
need(src, "static constexpr uint8_t kZappedValue = 0xdc;", "kZappedValue 0xdc");
need(src, "V8_INLINE void ZapMemory(void* address, size_t size) {", "ZapMemory");
need(src, "memset(address, kZappedValue, size);", "ZapMemset 委托");
need(src, "V8_INLINE void CheckMemoryIsZapped(const void* address, size_t size) {", "CheckMemoryIsZapped");
need(src, "V8_INLINE void CheckMemoryIsZero(const void* address, size_t size) {", "CheckMemoryIsZero");
need(src, "#define CPPGC_SHOULD_ZAP_MEMORY 1", "zap 开 1 分支");
need(src, "#define CPPGC_SHOULD_ZAP_MEMORY 0", "zap 关 0 分支");
need(src, "constexpr bool CheckMemoryIsInaccessibleIsNoop() {", "IsNoop 声明");
need(src, "void SetMemoryAccessible(void* address, size_t size);", "SetMemoryAccessible 声明");
need(src, "void SetMemoryInaccessible(void* address, size_t size);", "SetMemoryInaccessible 声明");
need(src, "void CheckMemoryIsInaccessible(const void* address, size_t size);", "CheckMemoryIsInaccessible 声明");
need(srcCc, "volatile uint8_t* const base = static_cast<uint8_t*>(address);", "volatile 逐字节写");
need(srcCc, "MSAN_MEMORY_IS_INITIALIZED(address, size);", "MSAN accessible");
need(srcCc, "ASAN_UNPOISON_MEMORY_REGION(address, size);", "ASAN accessible");
need(srcCc, "ASAN_POISON_MEMORY_REGION(address, size);", "ASAN inaccessible");
need(srcCc, "::cppgc::internal::ZapMemory(address, size);", "DEBUG 分支 zap");

const body = `${banner("src/heap/cppgc-internal/memory.js", `${REL} + ${REL_CC}`, "分配/释放内存访问模型; 目标配置非 sanitizer 非 DEBUG")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/memory.h kZappedValue
/// 最低位为 0 → 被 zap 的对象永不算完整构造
pub const K_ZAPPED_VALUE: u8 = 0xdc;

/// v8: src/heap/cppgc-internal/memory.cc NoSanitizeMemset — volatile 逐字节写, 绕过 sanitizer 截获
pub fn no_sanitize_memset(address: *mut u8, c: u8, bytes: usize) {
    unsafe {
        for i in 0..bytes { *address.add(i).cast_volatile() = c; }
    }
}

/// v8: src/heap/cppgc-internal/memory.h ZapMemory
pub fn zap_memory(address: *mut u8, size: usize) {
    unsafe { core::ptr::write_bytes(address, K_ZAPPED_VALUE, size); }
}

/// v8: src/heap/cppgc-internal/memory.h CheckMemoryIsZapped
pub fn check_memory_is_zapped(address: *const u8, size: usize) {
    for i in 0..size { assert_eq!(unsafe { *address.add(i) }, K_ZAPPED_VALUE); }
}

/// v8: src/heap/cppgc-internal/memory.h CheckMemoryIsZero
pub fn check_memory_is_zero(address: *const u8, size: usize) {
    for i in 0..size { assert_eq!(unsafe { *address.add(i) }, 0); }
}

/// v8: src/heap/cppgc-internal/memory.h CPPGC_SHOULD_ZAP_MEMORY
/// MSAN || ASAN || DEBUG → 1; 目标配置 (release, 无 sanitizer) → 0
#[cfg(not(any(feature = "msan", feature = "asan", debug_assertions)))]
pub const CPPGC_SHOULD_ZAP_MEMORY: bool = false;

/// v8: src/heap/cppgc-internal/memory.h SetMemoryAccessible (release 分支: 空操作)
#[cfg(not(any(feature = "msan", feature = "asan", debug_assertions)))]
pub fn set_memory_accessible(_address: *mut u8, _size: usize) {}

/// v8: src/heap/cppgc-internal/memory.h SetMemoryInaccessible (release 分支: 清零)
#[cfg(not(any(feature = "msan", feature = "asan", debug_assertions)))]
pub fn set_memory_inaccessible(address: *mut u8, size: usize) {
    unsafe { core::ptr::write_bytes(address, 0, size); }
}

/// v8: src/heap/cppgc-internal/memory.h CheckMemoryIsInaccessibleIsNoop
#[cfg(not(any(feature = "msan", feature = "asan", debug_assertions)))]
pub const CHECK_MEMORY_IS_INACCESSIBLE_IS_NOOP: bool = true;

// ── sanitizer/DEBUG 分支语义锚 (目标配置不展开) ─────────────────────────
// v8: src/heap/cppgc-internal/memory.cc SetMemoryAccessible
//   MSAN: MSAN_MEMORY_IS_INITIALIZED; ASAN: UNPOISON; DEBUG: memset 0
// v8: src/heap/cppgc-internal/memory.cc SetMemoryInaccessible
//   MSAN: memset 0 + MSAN_ALLOCATED_UNINITIALIZED; ASAN: NoSanitizeMemset 0 + POISON; DEBUG: ZapMemory
// v8: src/heap/cppgc-internal/memory.cc CheckMemoryIsInaccessible
//   MSAN: no-op (静态断言 IsNoop 为 true); ASAN: CHECK_WHOLE_POISONED + UNPOISON + CheckMemoryIsZero + REPOISON;
//   DEBUG: CheckMemoryIsZapped
`;

writeSrc("v8-heap-cppgc", "memory.rs.skel", body);
console.log(`memory.js: 对数 ${n} 项 (.h 11 + .cc 5) → v8-heap-cppgc/src/memory.rs.skel`);
