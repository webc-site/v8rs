#!/usr/bin/env bun
// tool/src/base/export-template.h.js — 族: src/base/export-template.h
// 覆盖: EXPORT_TEMPLATE_DECLARE/DEFINE 宏链 (GCC/MSVC 显式实例化导出差异)
// → v8-base/src/export_template.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/export-template.h");

// —— 真解析: 宏清单, 对数 ——
const macros = [...src.matchAll(/^#define (EXPORT_TEMPLATE_\w+)/gm)].map((m) => m[1]);
const keep = (s) => !["EXPORT_TEMPLATE_TEST", "EXPORT_TEMPLATE_TEST_DEFAULT_DEFAULT", "EXPORT_TEMPLATE_TEST_MSVC_HACK_MSVC_HACK"].includes(s);
const core = macros.filter(keep);
const expectedCore = [
  "EXPORT_TEMPLATE_DECLARE", "EXPORT_TEMPLATE_DEFINE",
  "EXPORT_TEMPLATE_INVOKE", "EXPORT_TEMPLATE_INVOKE_2",
  "EXPORT_TEMPLATE_DECLARE_DEFAULT", "EXPORT_TEMPLATE_DEFINE_DEFAULT",
  "EXPORT_TEMPLATE_DECLARE_MSVC_HACK", "EXPORT_TEMPLATE_DEFINE_MSVC_HACK",
  "EXPORT_TEMPLATE_STYLE", "EXPORT_TEMPLATE_STYLE_2", "EXPORT_TEMPLATE_STYLE_3",
  "EXPORT_TEMPLATE_STYLE_MATCH_foj3FJo5StF0OvIzl7oMxA",
  "EXPORT_TEMPLATE_STYLE_MATCH_foj3FJo5StF0OvIzl7oMxA__attribute__",
  "EXPORT_TEMPLATE_STYLE_MATCH_foj3FJo5StF0OvIzl7oMxA__declspec",
  "EXPORT_TEMPLATE_STYLE_MATCH_DECLSPEC_dllexport", "EXPORT_TEMPLATE_STYLE_MATCH_DECLSPEC_dllimport",
];
for (const e of expectedCore) if (!core.includes(e)) throw new Error(`缺宏 ${e}`);
if (core.length !== expectedCore.length) throw new Error(`核心宏数 ${core.length} != ${expectedCore.length}`);

// —— 锚点断言 ——
if (!src.includes("EXPORT_TEMPLATE_TEST(MSVC_HACK, __declspec(dllexport));")) throw new Error("缺 dllexport 自检");
const testCount = (src.match(/^EXPORT_TEMPLATE_TEST\(/gm) || []).length;
if (testCount !== 4) throw new Error(`自检数 ${testCount} != 4`);
console.log(`export-template: 核心宏 ${core.length}, 自检 4, 无枚举/常量/类型`);

let rust = banner(
  "src/base/export-template.h.js",
  "src/base/export-template.h",
  "模板显式实例化的导出宏链, Rust 无对应物",
);

rust += `
//! v8: src/base/export-template.h
//! 作用: GCC 要求 FOO_EXPORT 放显式实例化声明处, MSVC 要求 dllexport 放定义处;
//!       宏链按 export 展开值分发 DEFAULT / MSVC_HACK 两风格。
//! Rust: 无模板显式实例化与 DLL 导出问题, 整文件登记跳过。
//!
//! 分发表 (STYLE 判别):
//! | export 展开 | 风格 | DECLARE | DEFINE |
//! | --- | --- | --- | --- |
//! | 空 / __attribute__(...) / __declspec(dllimport) | DEFAULT | export | (空) |
//! | __declspec(dllexport) | MSVC_HACK | (空) | export |
`;

writeSrc("v8-base", "export_template.rs.skel", rust);
console.log("written v8-base/src/export_template.rs.skel");
