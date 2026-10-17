#!/usr/bin/env bun
// tool/src/base/compiler-specific.h.js — 族: src/base/compiler-specific.h
// 覆盖: 11 个编译器注记宏 (ALLOW_UNUSED/PRINTF_FORMAT/NOEXCEPT/ALIGNAS/...)
// → v8-base/src/compiler_specific.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/compiler-specific.h");

// —— 真解析: 宏名清单 (去重, 多分支重复与 include 守卫排除), 对数 ——
const macros = [...new Set([...src.matchAll(/^#define ([A-Z0-9_]+)[(\s]/gm)].map((m) => m[1]))].filter((n) => !/_H_$/.test(n));
const expected = [
  "V8_ALLOW_UNUSED", "PRINTF_FORMAT", "STATIC_CONST_MEMBER_DEFINITION",
  "MSVC_SUPPRESS_WARNING", "NON_EXPORTED_BASE", "V8_NOEXCEPT",
  "ALIGNAS", "V8_DEBUGGING_EXPORT", "HAS_CPP_CLASS_TYPES_AS_TEMPLATE_ARGS",
];
for (const e of expected) if (!macros.includes(e)) throw new Error(`缺宏 ${e}`);
if (macros.length !== 9) throw new Error(`带参宏数 ${macros.length} != 9`);

// —— 锚点断言 (关键展开分支) ——
if (!src.includes("#define V8_ALLOW_UNUSED __attribute__((unused))")) throw new Error("缺 UNUSED 展开");
if (!src.includes("__attribute__((format(printf, format_param, dots_param))")) throw new Error("缺 printf format 展开");
if (!src.includes("#define V8_NOEXCEPT noexcept")) throw new Error("缺 noexcept 分支");
if (!src.includes("#define ALIGNAS(byte_alignment) __attribute__((aligned(byte_alignment)))")) throw new Error("缺 aligned 展开");
if (!src.includes("__attribute__((used, retain, optnone, visibility(\"default\")))")) throw new Error("缺 DEBUGGING_EXPORT 展开");
if (!src.includes("#if __cplusplus >= 202002L")) throw new Error("缺 C++20 门");
console.log(`compiler-specific: 带参宏 ${macros.length}, MSVC 专属块 1, C++20 门 1`);

let rust = banner(
  "src/base/compiler-specific.h.js",
  "src/base/compiler-specific.h",
  "编译器注记宏, Rust 无对应物逐项登记",
);

rust += `
//! v8: src/base/compiler-specific.h — 编译器差异抹平宏, Rust 侧全部语义等价或不需要

// | v8 宏 | Rust 对应 | 备注 |
// | --- | --- | --- |
// | v8: V8_ALLOW_UNUSED | #[allow(dead_code)] | attribute__((unused)) |
// | v8: PRINTF_FORMAT(f, d) | 无 (Rust 格式串编译期检查) | GNUC format(printf,...) |
// | v8: STATIC_CONST_MEMBER_DEFINITION | 无 | MSVC declspec(selectany) |
// | v8: MSVC_SUPPRESS_WARNING(n) | 无 | 仅 MSVC |
// | v8: NON_EXPORTED_BASE(code) | 无 | MSVC C4275 抑制 |
// | v8: V8_NOEXCEPT | 无 (Rust fn 天然不抛) | 旧编译器 noexcept 兼容层 |
// | v8: ALIGNAS(n) | #[repr(align(n))] | attribute__((aligned(n))) |
// | v8: V8_DEBUGGING_EXPORT | 无 | used+retain+optnone+visibility, GDB 调试用 |
// | v8: HAS_CPP_CLASS_TYPES_AS_TEMPLATE_ARGS | 无 | C++20 门 (202002L), Rust 恒真等价 |
`;

writeSrc("v8-base", "compiler_specific.rs.skel", rust);
console.log("written v8-base/src/compiler_specific.rs.skel");
