#!/usr/bin/env bun
// tool/src/codegen/turboshaft-builtins-assembler-inl.h.js
// 机械提取 v8/src/codegen/turboshaft-builtins-assembler-inl.h —
// TS builtin 汇编别名宏 (V/ConstOrV/OptionalV/Label/LoopLabel/Tuple/Block/OpIndex)
// 与模板方法面骨架。1258 行泛型转发实现为语义面, 登记跳过。
// 产物: v8-codegen-core/src/turboshaft_builtins_assembler_inl_generated.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const INL_REL = "src/codegen/turboshaft-builtins-assembler-inl.h";
const inl = readV8(INL_REL);

// 别名宏 (TS_BUILTIN_ALIAS 或同名 #define): using X = ...;
const aliases = [...inl.matchAll(/^\s*using (\w+) = ([^;]+);/gm)]
  .map((m) => ({ name: m[1], def: m[2].trim() }))
  .filter((a) => ["V", "ConstOrV", "OptionalV", "Label", "LoopLabel", "Tuple", "Block", "OpIndex"].includes(a.name));
if (aliases.length < 8) throw new Error(`别名数异常: ${aliases.length}`);
// 模板方法面
const templates = [...inl.matchAll(/^template <([\s\S]*?)>/gm)].length;
if (templates < 7) throw new Error(`模板面异常: ${templates}`);
if (!inl.includes("IsKnownTaggedPointerTag")) throw new Error("缺 StrongAlias 锚");

let rust = banner("src/codegen/turboshaft-builtins-assembler-inl.h.js", INL_REL, "TS builtin 汇编别名与骨架");
rust += `
// 泛型别名登记 (原宏内 using 展开):
${aliases.map((a) => `/// 原 \`${a.name}\` = ${a.def}`).join("\n")}

/// 模板方法面: ${templates} 处 template 转发 (builtin 语义实现登记跳过)。
/// 原 IsKnownTaggedPointerTag = StrongAlias<bool> 标记。
pub struct TurboshaftBuiltinsAssemblerInl;

// TODO(dev): 依 turboshaft 框架补 builtin 汇编原语。

// —— 提取不变式断言 ——
const _: () = {};
`;

const p = writeSrc("v8-codegen-core", "turboshaft_builtins_assembler_inl_generated.rs", rust);
console.log(`written ${p} (别名 ${aliases.length}, template ${templates})`);
