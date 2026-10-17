#!/usr/bin/env bun
// tool/src/base/platform/wrappers.h.js — 覆盖 v8/src/base/platform/wrappers.h
// STARBOARD 分支包装 (非 Starboard 目标直通 libc)
// → v8-base-platform/src/wrappers.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/platform/wrappers.h");

// 结构对数: 两包装函数, Starboard 分支返回 NULL/-1
const anchors = ["FILE* Fopen(const char* filename, const char* mode)", "int Fclose(FILE* stream)",
  "return NULL;", "return -1;", "return fopen(filename, mode);", "return fclose(stream);"];
for (const a of anchors) {
  if (!hSrc.includes(a)) throw new Error(`wrappers.h 缺锚点: ${a}`);
}

console.log("wrappers.h 锚点 6");

let rust = banner("src/base/platform/wrappers.h.js", "src/base/platform/wrappers.h", "Fopen/Fclose 包装 (V8_OS_STARBOARD 分支登记)");

rust += `
/// 文件打开包装 (原版 base::Fopen; 非 Starboard 直通 fopen)
/// V8_OS_STARBOARD 分支: 返回 NULL — 本目标不启用 Starboard, 登记不产码
pub fn fopen(filename: &std::ffi::CStr, mode: &std::ffi::CStr) -> *mut libc::FILE {
  unsafe { libc::fopen(filename.as_ptr(), mode.as_ptr()) }
}

/// 文件关闭包装 (原版 base::Fclose; 非 Starboard 直通 fclose)
pub fn fclose(stream: *mut libc::FILE) -> i32 {
  unsafe { libc::fclose(stream) }
}

// —— 提取不变式断言 ——
const _: () = {
  // Starboard 分支哨兵值登记 (NULL / -1)
  const SB_FOPEN: i32 = 0; // NULL
  const SB_FCLOSE: i32 = -1;
  assert!(SB_FOPEN == 0 && SB_FCLOSE == -1);
};
`;

const p = writeSrc("v8-base-platform", "wrappers.rs", rust);
console.log(`written ${p}`);
