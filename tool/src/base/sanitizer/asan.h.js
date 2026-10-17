#!/usr/bin/env bun
// tool/src/base/sanitizer/asan.h.js — 覆盖 v8/src/base/sanitizer/asan.h
// 解析 ASan 宏对 (启用/空转两分支)、DISABLE_ASAN/HWASAN 开关与 poison 三原语
// → v8-base/src/sanitizer/asan.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/sanitizer/asan.h");

for (const anchor of [
  "V8_BASE_SANITIZER_ASAN_H_",
  "#define DISABLE_ASAN __attribute__((no_sanitize_address))",
  "#define ASAN_POISON_MEMORY_REGION(start, size)",
  "#define ASAN_UNPOISON_MEMORY_REGION(start, size)",
  "#define ASAN_CHECK_WHOLE_MEMORY_REGION_IS_POISONED(start, size)",
  "#define DISABLE_HWASAN __attribute__((no_sanitize(\"hwaddress\")))",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 宏定义清单 (启用分支)
const macros = [...new Set([...h.matchAll(/^#define (\w+)\(/gm)].map((m) => m[1]))];
// 空转分支计数
const noopMacros = [...new Set([...h.matchAll(/^#define (\w+)\(start, size\)\s*\\\n\s*do \{/gm)].map((m) => m[1]))];

const body = `${banner("src/base/sanitizer/asan.h.js", "src/base/sanitizer/asan.h", "AddressSanitizer 宏面")}
//! v8 侧: 参数化宏 ${macros.length} (ASan 开 + 空转两分支); DISABLE_ASAN/HWASAN 2
//! Rust: ASAN_POISON/UNPOISON ↔ miri/夜 nightly API 或 extern "C" __asan_poison_memory_region; 默认空转

/// v8: DISABLE_ASAN — no_sanitize_address (Rust: cfg 不适用, 恒空)
/// v8: DISABLE_HWASAN — no_sanitize("hwaddress")
/// v8: ASAN_POISON_MEMORY_REGION(start, size) — 标红 (不可访问)
/// v8: ASAN_UNPOISON_MEMORY_REGION(start, size) — 标绿
/// v8: ASAN_CHECK_WHOLE_MEMORY_REGION_IS_POISONED(start, size) — 断言全红
#[cfg(feature = "asan")]
pub fn poison_memory_region(start: *mut u8, size: usize) {
    extern "C" { fn __asan_poison_memory_region(addr: *const core::ffi::c_void, size: usize); }
    unsafe { __asan_poison_memory_region(start as _, size) }
}

#[cfg(not(feature = "asan"))]
/// v8: 空转分支 (do {} while(0) + static_cast void)
pub fn poison_memory_region(_start: *mut u8, _size: usize) {}

// 宏对账 (v8 解析 ${macros.length} 项): ${macros.join(", ")}
`;

writeSrc("v8-base", "sanitizer/asan.rs.skel", body);
console.log(`sanitizer/asan.h.js: macros=${macros.length} noopBranch=${noopMacros.length}`);
