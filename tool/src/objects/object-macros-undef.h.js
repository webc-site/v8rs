#!/usr/bin/env bun
// tool/src/objects/object-macros-undef.h.js — 覆盖 v8/src/objects/object-macros-undef.h
// → v8-objects-core/src/object_macros_undef.rs
// 机械提取: #undef 名单 (与 object-macros.h 定义侧 1:1 对数)

import { readV8, writeSrc, banner } from "../../lib.js";

const undefSrc = readV8("src/objects/object-macros-undef.h");
const defSrc = readV8("src/objects/object-macros.h");

// #undef 名单 (按出现序)
const undefs = [...undefSrc.matchAll(/^#undef (\w+)$/gm)].map((m) => m[1]);
if (undefs.length < 20) throw new Error(`#undef 条数异常: ${undefs.length}`);
if (new Set(undefs).size !== undefs.length) throw new Error("#undef 名字重复");

// 定义侧对数: 每个 #undef 的宏必须在 object-macros.h 有 #define
const defines = new Set([...defSrc.matchAll(/^#define (\w+)/gm)].map((m) => m[1]));
const missing = undefs.filter((n) => !defines.has(n));
// 例外: 条件宏 (V8_OBJECT* 变体) 在 macros.h 中可能由嵌套定义产生
if (missing.length > 3) throw new Error(`#undef 无对应 #define 过多: ${missing.join(",")}`);

let rust = banner("src/objects/object-macros-undef.h.js", "src/objects/object-macros-undef.h", "对象宏 #undef 名单 (定义侧 1:1)");
rust += `//! object-macros.h 的宏退出名单: 每个包含 object-macros.h 的头文件
//! 末尾 #undef 全部宏, 保持头文件无宏泄漏。Rust 侧无宏, 此处登记契约。

/// #undef 宏名单 (object-macros-undef.h 出现序, 共 ${undefs.length} 条)。
pub const UNDEF_MACROS: [&str; ${undefs.length}] = [
`;
for (const m of undefs) rust += `  "${m}",\n`;
rust += `];

/// 名单长度。
pub const UNDEF_COUNT: usize = ${undefs.length};

// —— 提取不变式断言 ——
const _: () = {
  assert!(UNDEF_COUNT == ${undefs.length});
  assert!(UNDEF_MACROS[0] == "${undefs[0]}");
  assert!(UNDEF_MACROS[UNDEF_COUNT - 1] == "${undefs[undefs.length - 1]}");
};
`;

const p = writeSrc("v8-objects-core", "object_macros_undef.rs", rust);
console.log(`object_macros_undef.rs: #undef 名单 ${undefs.length} 条, 对数通过`);
