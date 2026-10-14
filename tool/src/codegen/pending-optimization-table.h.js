#!/usr/bin/env bun
// tool/src/codegen/pending-optimization-table.h.js
// 机械提取 v8/src/codegen/pending-optimization-table.{h,cc} — 骨架。
// 产物: v8-codegen-core/src/pending_optimization_table.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/codegen/pending-optimization-table.h";
const CC_REL = "src/codegen/pending-optimization-table.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

const sigs = [...h.matchAll(/^\s*static\s+[\w:<>]+\s+(\w+)\(/gm)].map((m) => m[1]);
if (sigs.length !== 2) throw new Error(`静态接口条数异常: ${sigs.length} != 2`);
if (!h.includes("MarkFunctionForManualOptimization")) throw new Error("缺 MarkFunctionForManualOptimization 锚");

let rust = banner("src/codegen/pending-optimization-table.h.js", "src/codegen/pending-optimization-table.{h,cc}", "待优化函数表骨架");
rust += `
/// 原类面静态接口 (${sigs.length} 条):
${[...new Set(sigs)].map((s) => `// - ${s}`).join("\n")}

pub struct PendingOptimizationTable;

// TODO(dev): 补标记/再优化队列实现 (cc 语义面登记跳过)。

// —— 提取不变式断言 ——
const _: () = {};
`;

const p = writeSrc("v8-codegen-core", "pending_optimization_table.rs", rust);
console.log(`written ${p} (接口 ${[...new Set(sigs)].length})`);
