#!/usr/bin/env bun
// tool/src/codegen/compilation-cache.h.js
// 机械提取 v8/src/codegen/compilation-cache.{h,cc} — 编译缓存骨架。
// 产物: v8-codegen-core/src/compilation_cache.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/codegen/compilation-cache.h";
const CC_REL = "src/codegen/compilation-cache.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

const genM = h.match(/static const int kGenerations = (\d+);/);
if (!genM) throw new Error("未找到 kGenerations");
if (!h.includes("DirectHandle<CompilationCacheTable> GetTable(int generation);")) throw new Error("缺 GetTable 锚");
// 缓存类别锚 (eval/regexp/script)
for (const anchor of ["CompilationCacheEval", "CompilationCacheRegExp", "CompilationCacheScript"]) {
  if (!h.includes(`class ${anchor}`)) throw new Error(`缺类锚 ${anchor}`);
}

let rust = banner("src/codegen/compilation-cache.h.js", "src/codegen/compilation-cache.{h,cc}", "编译缓存骨架");
rust += `
/// RegExp 子缓存代数 (原 kGenerations)
pub const GENERATIONS: i32 = ${genM[1]};

/// 原类面 (分缓存类别):
/// - class CompilationCacheEval   — eval/函数字面量缓存
/// - class CompilationCacheRegExp — RegExp 源串+标志缓存
/// - class CompilationCacheScript — 脚本源缓存
/// - 原签名: DirectHandle<CompilationCacheTable> GetTable(int generation)
/// TODO(dev): 补代龄淘汰与表存取实现。

pub struct CompilationCacheEval;
pub struct CompilationCacheRegExp;
pub struct CompilationCacheScript;

// —— 提取不变式断言 ——
const _: () = {
  assert!(GENERATIONS == 2);
};
`;

const p = writeSrc("v8-codegen-core", "compilation_cache.rs", rust);
console.log(`written ${p}`);
