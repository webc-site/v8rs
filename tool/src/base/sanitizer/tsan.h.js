#!/usr/bin/env bun
// tool/src/base/sanitizer/tsan.h.js — 覆盖 v8/src/base/sanitizer/tsan.h
// 解析 TSan 宏对 (IGNORE_READS/ACQUIRE/RELEASE 注解) 与 DISABLE_TSAN
// → v8-base/src/sanitizer/tsan.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/sanitizer/tsan.h");

for (const anchor of [
  "V8_BASE_SANITIZER_TSAN_H_",
  "#define DISABLE_TSAN __attribute__((no_sanitize_thread))",
  "#define TSAN_IGNORE_READS_BEGIN AnnotateIgnoreReadsBegin(__FILE__, __LINE__)",
  "#define TSAN_ACQUIRE(addr) __tsan_acquire(reinterpret_cast<void*>(addr))",
  "#define TSAN_RELEASE(addr) __tsan_release(reinterpret_cast<void*>(addr))",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 宏清单 (启用分支)
const macros = [...new Set([...h.matchAll(/^#define (TSAN_\w+)\(/gm)].map((m) => m[1]))];

const body = `${banner("src/base/sanitizer/tsan.h.js", "src/base/sanitizer/tsan.h", "ThreadSanitizer 宏面")}
//! v8 侧: 参数化宏 ${macros.length} (双分支: 注解 vs (void)0); DISABLE_TSAN 1
//! Rust: TSAN_ACQUIRE/RELEASE ↔ 原子 Ordering::Acquire/Release 语义由编译器映射; 空转默认

/// v8: TSAN_IGNORE_READS_BEGIN / TSAN_IGNORE_READS_END — 读访问忽略区间
pub fn tsan_ignore_reads_begin() { /* v8: AnnotateIgnoreReadsBegin */ }
pub fn tsan_ignore_reads_end() { /* v8: AnnotateIgnoreReadsEnd */ }

/// v8: TSAN_ACQUIRE(addr) — happens-before 获取注解
pub fn tsan_acquire(addr: usize) { let _ = addr; /* v8: __tsan_acquire */ }

/// v8: TSAN_RELEASE(addr) — happens-before 释放注解
pub fn tsan_release(addr: usize) { let _ = addr; /* v8: __tsan_release */ }

/// v8: DISABLE_TSAN — no_sanitize_thread (Rust 恒空)
// 宏对账 (v8 解析 ${macros.length} 项): ${macros.join(", ")}
`;

writeSrc("v8-base", "sanitizer/tsan.rs.skel", body);
console.log(`sanitizer/tsan.h.js: macros=${macros.length}`);
