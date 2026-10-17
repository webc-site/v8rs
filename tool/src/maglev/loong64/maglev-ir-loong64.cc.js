#!/usr/bin/env bun
// tool/src/maglev/loong64/maglev-ir-loong64.cc.js
// LoongArch64 非本轮目标架构 (平台分支按 x64+arm64 展开, 规则九) → 跳过语义转换;
// 本脚本提取文件内唯一规则代码 (ReduceInterruptBudgetType 枚举) + 结构对数登记。

import { readV8, writeSrc, banner } from "../../../lib.js";

const GEN = "src/maglev/loong64/maglev-ir-loong64.cc.js";
const die = (m) => {
  throw new Error(`[${GEN}] ${m}`);
};

const IR_REL = "src/maglev/loong64/maglev-ir-loong64.cc";
const ir = readV8(IR_REL);

// —— 1. ReduceInterruptBudgetType 枚举 (文件内唯一规则代码) ————
const enumMatch = ir.match(/enum class ReduceInterruptBudgetType\s*\{([^}]*)\};/);
if (!enumMatch) die("未找到 ReduceInterruptBudgetType 枚举");
const variants = enumMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (variants.join(",") !== "kLoop,kReturn") die(`枚举值异常: ${variants.join(",")}`);

// —— 2. 结构对数: IR 节点方法定义锚 ————
const anchors = [...ir.matchAll(/\w+::(GenerateCode|SetValueLocationConstraints|MaxCallStackArgs)\(/g)].length;
if (anchors !== 101) die(`IR 方法锚 ${anchors} != 101`);
if (ir.split("\n").length !== 1320) die(`行数 ${ir.split("\n").length} != 1320`);

let rust = banner(GEN, IR_REL, "LoongArch64 maglev-ir 跳过登记 + 预算枚举 (非目标架构)");
rust += `
//! 规则九: 平台分支按 x64+arm64 展开, LoongArch64 跳过语义转换。
//! 本文件语义为各 IR 节点 GenerateCode/SetValueLocationConstraints (101 锚),
//! 不展开; 仅登记文件面与预算类型枚举。

/// 原版 enum class ReduceInterruptBudgetType (中断预算计费点)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ReduceInterruptBudgetType {
${variants.map((v, i) => `  /// ${v}\n  ${v.replace(/^k/, "")} = ${i},`).join("\n")}
}

pub static REGISTERED_FILES: &[(&str, usize, usize)] = &[
  ("src/maglev/loong64/maglev-ir-loong64.cc", 1320, 101),
];

const _: () = {
  assert!(REGISTERED_FILES.len() == 1);
  assert!(REGISTERED_FILES[0].1 == 1320);
  assert!(REGISTERED_FILES[0].2 == 101);
};
`;

const p = writeSrc("v8-maglev-backend", "loong64/maglev_ir_loong64.rs", rust);
console.log(`OK ${GEN}: enum=${variants.length} anchors=${anchors} → ${p}`);
