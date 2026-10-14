#!/usr/bin/env bun
// tool/src/codegen/x64/assembler-x64.cc.js
// 机械提取 v8/src/codegen/x64/assembler-x64.{cc,-inl.h} —
// 汇编器规则面: CpuFeaturesFromCompiler 基线特征 + 内联访问器注册表。
// 5546 行指令发射器为语义面, 登记跳过 (规则 10)。
// 禁覆写面: 手写 assembler.rs/operand.rs 为权威, 本脚本落 _generated 旁路。
// 产物: v8-codegen-x64/src/assembler_x64_generated.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const CC_REL = "src/codegen/x64/assembler-x64.cc";
const INL_REL = "src/codegen/x64/assembler-x64-inl.h";
const cc = readV8(CC_REL);
const inl = readV8(INL_REL);

// CpuFeaturesFromCompiler: 编译器基线特征
const fcm = cc.match(/static constexpr CpuFeatureSet CpuFeaturesFromCompiler\(\)\s*\{([\s\S]*?)\n\}/);
if (!fcm) throw new Error("未找到 CpuFeaturesFromCompiler");
const feats = [...fcm[1].matchAll(/features\.Add\((\w+)\);/g)].map((m) => m[1]);
if (feats.length < 8) throw new Error(`基线特征不足: ${feats.length}`);
if (!fcm[1].includes("SSE2 and CMOV are baseline")) throw new Error("缺 SSE2 基线注释锚");

// inl 内联方法面
const inlMethods = [...inl.matchAll(/^\s*(?:template <[\s\S]*?>\s*\n)?\s*(?:inline\s+)?[\w:<>,&*\s]+?\b(\w+)\(/gm)].map((m) => m[1]);
if (inlMethods.length < 20) throw new Error(`inl 方法面不足: ${inlMethods.length}`);
const poolM = cc.match(/static constexpr int kConstantPoolSize = (\d+);/);
if (!poolM || poolM[1] !== "0") throw new Error("kConstantPoolSize 锚不符 (x64 无常量池)");

let rust = banner("src/codegen/x64/assembler-x64.cc.js", "src/codegen/x64/assembler-x64.{cc,-inl.h}", "x64 汇编器骨架");
rust += `
// 禁覆写面登记: 手写 assembler.rs / operand.rs 保持权威, 本文件仅登记规则面。
// 语义面: 指令发射器 (cc ${cc.split("\n").length} 行) 登记跳过。

/// 编译器基线 CPU 特征 (原 CpuFeaturesFromCompiler):
${feats.map((f) => `/// - __${f.startsWith("__") ? f : f}__ 编译宏触发`).join("\n")}
pub const BASELINE_CPU_FEATURES_FROM_COMPILER: &[&str] = &[${feats.map((f) => `"${f}"`).join(", ")}];

/// x64 无常量池 (原 kConstantPoolSize)
pub const CONSTANT_POOL_SIZE: i32 = 0;

/// inl 内联方法面 (${[...new Set(inlMethods)].length} 条):
${[...new Set(inlMethods)].map((s) => `// - ${s}`).join("\n")}

pub struct AssemblerX64Inl;

// TODO(dev): 依手写 assembler.rs 分批补发射器。

// —— 提取不变式断言 ——
const _: () = {
  assert!(CONSTANT_POOL_SIZE == 0);
};
`;

const p = writeSrc("v8-codegen-x64", "assembler_x64_generated.rs", rust);
console.log(`written ${p} (特征 ${feats.length}, inl ${[...new Set(inlMethods)].length})`);
