#!/usr/bin/env bun
// tool/src/codegen/unoptimized-compilation-info.h.js
// 机械提取 v8/src/codegen/unoptimized-compilation-info.{h,cc} — 骨架。
// 产物: v8-codegen-core/src/unoptimized_compilation_info.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/codegen/unoptimized-compilation-info.h";
const CC_REL = "src/codegen/unoptimized-compilation-info.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

const classM = h.match(/class V8_EXPORT_PRIVATE UnoptimizedCompilationInfo final\s*\{([\s\S]*?)\n\};/);
if (!classM) throw new Error("未找到 UnoptimizedCompilationInfo 类");
const methods = [...classM[1].matchAll(/^\s*(?:inline\s+)?[\w:<>,&*\s]+?\b(\w+)\(/gm)].map((m) => m[1]);
if (methods.length < 8) throw new Error(`方法面不足: ${methods.length}`);
for (const anchor of ["set_literal(FunctionLiteral* literal)", "has_source_range_map()"]) {
  if (!classM[1].includes(anchor)) throw new Error(`缺锚 ${anchor}`);
}

let rust = banner("src/codegen/unoptimized-compilation-info.h.js", "src/codegen/unoptimized-compilation-info.{h,cc}", "非优化编译信息骨架");
rust += `
/// 原 class UnoptimizedCompilationInfo (字节码编译期信息: literal/zone/
/// source_range_map/coverageinfo 槽位) 方法面 (${[...new Set(methods)].length} 条):
${[...new Set(methods)].map((s) => `// - ${s}`).join("\n")}

pub struct UnoptimizedCompilationInfo;

// TODO(dev): 补字段与 set/clear 生命周期。

// —— 提取不变式断言 ——
const _: () = {};
`;

const p = writeSrc("v8-codegen-core", "unoptimized_compilation_info.rs", rust);
console.log(`written ${p} (方法 ${[...new Set(methods)].length})`);
