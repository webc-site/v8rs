#!/usr/bin/env bun
// tool/src/base/sanitizer/lsan.h.js — 覆盖 v8/src/base/sanitizer/lsan.h
// 解析 LSAN_IGNORE_OBJECT 双分支 (ASan+非Win 启用 / 空转+类型断言)
// → v8-base/src/sanitizer/lsan.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/sanitizer/lsan.h");

for (const anchor of [
  "V8_BASE_SANITIZER_LSAN_H_",
  "#define LSAN_IGNORE_OBJECT(ptr) __lsan_ignore_object(ptr)",
  "LSAN_IGNORE_OBJECT can only be used with pointer types",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 条件宏分支
const enabled = h.includes("__lsan_ignore_object(ptr)");
const gateCond = h.match(/#if defined\((\w+)\) && !defined\((\w+)\)/);

const body = `${banner("src/base/sanitizer/lsan.h.js", "src/base/sanitizer/lsan.h", "LeakSanitizer 宏面")}
//! v8 侧: 宏 1 个, 双分支; 启用门 = ${gateCond ? `${gateCond[1]} && !${gateCond[2]}` : "?"}
//! (LSan 无独立编译开关, 随 ASan detect_leaks=1; Windows 无 LSan)

/// v8: LSAN_IGNORE_OBJECT(ptr)
/// 启用: __lsan_ignore_object(ptr); 空转: static_assert 可转 const void*
#[cfg(feature = "asan")]
pub fn lsan_ignore_object<T>(ptr: &T) {
    extern "C" { fn __lsan_ignore_object(a: *const core::ffi::c_void); }
    unsafe { __lsan_ignore_object(ptr as *const T as _) }
}

#[cfg(not(feature = "asan"))]
pub fn lsan_ignore_object<T>(ptr: &T) { let _ = ptr as *const T as *const core::ffi::c_void; }
`;

writeSrc("v8-base", "sanitizer/lsan.rs.skel", body);
console.log(`sanitizer/lsan.h.js: macros=1 enabled=${enabled} gate=${gateCond ? "ASan&&!WIN" : "?"}`);
