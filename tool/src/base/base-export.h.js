#!/usr/bin/env bun
// tool/src/base/base-export.h.js — 族: src/base/base-export.h
// 覆盖: V8_BASE_EXPORT 宏的 4 分支展开 (win dllexport/dllimport, posix visibility, 空)
// → v8-base/src/base_export.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/base-export.h");

// —— 锚点断言 ——
const branches = [
  "#define V8_BASE_EXPORT __declspec(dllexport)",
  "#define V8_BASE_EXPORT __declspec(dllimport)",
  "#define V8_BASE_EXPORT __attribute__((visibility(\"default\")))",
  "#define V8_BASE_EXPORT",
];
for (const b of branches) if (!src.includes(b)) throw new Error(`base-export.h 缺分支: ${b}`);
if (!src.includes("#if V8_OS_WIN")) throw new Error("缺 V8_OS_WIN 守卫");
if (!src.includes("#include \"include/v8config.h\"")) throw new Error("缺 v8config include");

// —— 真解析: 宏定义计数 (5 处 #define, 4 种展开: 空展开出现 2 次) ——
const defCount = (src.match(/^#define V8_BASE_EXPORT\b/gm) || []).length;
const distinctExpansions = new Set((src.match(/^#define V8_BASE_EXPORT[ \t]*(.*)$/gm) || []).map((l) => l.replace(/^#define V8_BASE_EXPORT[ \t]*/, ""))).size;
if (defCount !== 5) throw new Error(`V8_BASE_EXPORT 定义数 ${defCount} != 5`);
if (distinctExpansions !== 4) throw new Error(`展开种数 ${distinctExpansions} != 4`);
console.log("base-export: V8_BASE_EXPORT 宏 4 分支, 无枚举/常量/类型");

let rust = banner(
  "src/base/base-export.h.js",
  "src/base/base-export.h",
  "DLL 导出宏, Rust 无对应物",
);

rust += `
//! v8: src/base/base-export.h
//! 纯链接器可见性宏, Rust crate 系统天然按 pub 控制可见性, 无需转换。
//! 登记以下 4 个展开分支供审计对数:
//!
//! | 条件 | 展开 |
//! | --- | --- |
//! | V8_OS_WIN && BUILDING_V8_BASE_SHARED | __declspec(dllexport) |
//! | V8_OS_WIN && USING_V8_BASE_SHARED | __declspec(dllimport) |
//! | !V8_OS_WIN && (BUILDING_V8_BASE_SHARED \|\| USING_V8_BASE_SHARED) | __attribute__((visibility("default"))) |
//! | 其余 | 空 |
`;

writeSrc("v8-base", "base_export.rs.skel", rust);
console.log("written v8-base/src/base_export.rs.skel");
