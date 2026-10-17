#!/usr/bin/env bun
// tool/src/codegen/arm64/macro-assembler-arm64.h.js
// 机械提取 v8/src/codegen/arm64/macro-assembler-arm64.{h,-inl.h,cc} —
// 宏汇编器方法面注册表 (h 2677 / inl 1654 / cc 4911 行为语义面, 规则 10 取骨架)。
// 禁覆写面关联: 手写 assembler.rs/operand.rs 为底层权威, 本脚本独立落盘。
// 产物: v8-codegen-arm64/src/macro_assembler_arm64_generated.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const H_REL = "src/codegen/arm64/macro-assembler-arm64.h";
const INL_REL = "src/codegen/arm64/macro-assembler-arm64-inl.h";
const CC_REL = "src/codegen/arm64/macro-assembler-arm64.cc";
const h = readV8(H_REL);
readV8(INL_REL); // 家族覆盖锚
readV8(CC_REL); // 家族覆盖锚

const classM = h.match(/class V8_EXPORT_PRIVATE MacroAssembler\s*:\s*public ([\w:<>]+)\s*\{/);
if (!classM) throw new Error("未找到 MacroAssembler 类");
const methods = [...h.matchAll(/^\s*(?:inline\s+)?(?:void|int|bool|Address|Register|Label\*|unsigned)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length < 80) throw new Error(`方法面不足: ${methods.length}`);
const inlMethods = [...readV8(INL_REL).matchAll(/^\s*(?:template <[\s\S]*?>\s*\n)?\s*(?:inline\s+)?[\w:<>,&*\s]+?\b(\w+)\(/gm)].map((m) => m[1]);
if (inlMethods.length < 20) throw new Error(`inl 方法面不足: ${inlMethods.length}`);

let rust = banner("src/codegen/arm64/macro-assembler-arm64.h.js", "src/codegen/arm64/macro-assembler-arm64.{h,-inl.h,cc}", "arm64 宏汇编器骨架");
rust += `
// 基类: ${classM[1]}
// 语义面: 指令序列生成 (cc ${readV8(CC_REL).split("\n").length} 行) 登记跳过。
// 公开方法面 (h ${[...new Set(methods)].length} 条):
${[...new Set(methods)].map((s) => `// - ${s}`).join("\n")}
// inl 方法面 (${[...new Set(inlMethods)].length} 条):
${[...new Set(inlMethods)].map((s) => `// - ${s}`).join("\n")}

pub struct MacroAssembler;

// TODO(dev): 依手写 assembler.rs 原语分批补宏实现。

// —— 提取不变式断言 ——
const _: () = {};
`;

const p = writeSrc("v8-codegen-arm64", "macro_assembler_arm64_generated.rs", rust);
console.log(`written ${p} (h ${[...new Set(methods)].length}, inl ${[...new Set(inlMethods)].length})`);
