#!/usr/bin/env bun
// tool/src/base/win32-headers.h.js — 覆盖 v8/src/base/win32-headers.h
// 解析 NO* 精简宏族、Win 基础 typedef (BOOL/DWORD/LPVOID/HANDLE/ULONG_PTR...) 与 WINAPI
// → v8-base/src/win32_headers.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/win32-headers.h");

for (const anchor of [
  "V8_BASE_WIN32_HEADERS_H_",
  "#define WIN32_LEAN_AND_MEAN",
  "typedef int BOOL;",
  "typedef unsigned long DWORD;",
  "typedef void* HANDLE;",
  "#define WINAPI __stdcall",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: NO* 精简宏
const leanMacros = [...h.matchAll(/^#define (NO\w+)$/gm)].map((m) => m[1]);
// 真解析: typedef 表
const typedefs = [...h.matchAll(/^typedef ([\w ]+?\*?) (\w+)(?:, \*(\w+))?;/gm)].map((m) => ({
  type: m[1].trim(),
  name: m[2],
  ptrAlias: m[3],
}));

const body = `${banner("src/base/win32-headers.h.js", "src/base/win32-headers.h", "Win32 免 windows.h 的最小类型面")}
//! v8 侧: 精简宏 ${leanMacros.length + 1} (含 WIN32_LEAN_AND_MEAN); typedef ${typedefs.length} 条
//! Rust: 仅当对接 winapi 层时需要; 类型镜像登记, 语义面就地修订

/// v8: WIN32_LEAN_AND_MEAN + ${leanMacros.join(" ")}
/// 作用: 令后包含的 windows.h 裁剪 (Rust 无预处理器, 登记对账)
pub const WIN32_LEAN_AND_MEAN_MACROS: &[&str] = &["WIN32_LEAN_AND_MEAN", ${leanMacros.map((m) => `"${m}"`).join(", ")}];

// ---- Win 基础类型镜像 (v8 typedef ${typedefs.length} 条) ----
${typedefs.map((t) => {
  const rust = { "int": "i32", "unsigned long": "u32", "long": "i32", "void*": "*mut core::ffi::c_void", "unsigned __int64": "u64", "__w64 unsigned long": "u32", "long long": "i64", "unsigned long long": "u64", "char*": "*mut core::ffi::c_char" }[t.type] ?? "todo!() /* v8: typedef " + t.type + " */";
  return `/// v8: typedef ${t.type} ${t.name};
pub type ${t.name} = ${rust};`;
}).join("\n")}

/// v8: #define WINAPI __stdcall
/// Rust: extern "system" 调用约定即 stdcall (x64 下统一)
`;

writeSrc("v8-base", "win32_headers.rs.skel", body);
console.log(`win32-headers.h.js: leanMacros=${leanMacros.length + 1} typedefs=${typedefs.length}`);
