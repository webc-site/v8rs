#!/usr/bin/env bun
// tool/src/base/sanitizer/msan.h.js — 覆盖 v8/src/base/sanitizer/msan.h
// 解析 MSan 宏对 (未初始化标记/初始化查询/检查) 与 DISABLE_MSAN
// → v8-base/src/sanitizer/msan.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/sanitizer/msan.h");

for (const anchor of [
  "V8_BASE_SANITIZER_MSAN_H_",
  "#define MSAN_ALLOCATED_UNINITIALIZED_MEMORY(start, size)",
  "#define MSAN_MEMORY_IS_INITIALIZED(start, size)",
  "#define DISABLE_MSAN __attribute__((no_sanitize(\"memory\")))",
  "#define MSAN_CHECK_MEM_IS_INITIALIZED(start, byte_size)",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 宏清单
const macros = [...new Set([...h.matchAll(/^#define (\w+)\(/gm)].map((m) => m[1]))];

const body = `${banner("src/base/sanitizer/msan.h.js", "src/base/sanitizer/msan.h", "MemorySanitizer 宏面")}
//! v8 侧: 参数化宏 ${macros.length} (双分支); DISABLE_MSAN 1
//! Rust: extern "C" __msan_* 或空转; 语义面就地修订

/// v8: MSAN_ALLOCATED_UNINITIALIZED_MEMORY(start, size)
/// 语义: 标记新分配内存为未初始化 (shadow 置脏)
pub fn msan_allocated_uninitialized_memory(start: *mut u8, size: usize) { let _ = (start, size); /* v8: __msan_allocated_memory */ }

/// v8: MSAN_MEMORY_IS_INITIALIZED(start, size) — bool
pub fn msan_memory_is_initialized(start: *const u8, size: usize) -> bool { let _ = (start, size); true }

/// v8: MSAN_CHECK_MEM_IS_INITIALIZED(start, byte_size) — CHECK 失败即崩
pub fn msan_check_mem_is_initialized(start: *const u8, size: usize) { let _ = (start, size); }

/// v8: DISABLE_MSAN — no_sanitize("memory") (Rust 恒空)
// 宏对账 (v8 解析 ${macros.length} 项): ${macros.join(", ")}
`;

writeSrc("v8-base", "sanitizer/msan.rs.skel", body);
console.log(`sanitizer/msan.h.js: macros=${macros.length}`);
