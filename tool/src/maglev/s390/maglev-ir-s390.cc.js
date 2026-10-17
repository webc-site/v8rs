#!/usr/bin/env bun
// tool/src/maglev/s390/maglev-ir-s390.cc.js
// S390 非本轮目标架构 (平台分支按 x64+arm64 展开, 规则九) → 跳过语义转换;
// 本脚本提取文件内唯一规则代码 (ReduceInterruptBudgetType 枚举) + 结构对数登记。

import { readV8, writeSrc, banner } from "../../../lib.js";

const GEN = "src/maglev/s390/maglev-ir-s390.cc.js";
const die = (m) => {
  throw new Error(`[${GEN}] ${m}`);
};

const IR_REL = "src/maglev/s390/maglev-ir-s390.cc";
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
if (anchors !== 99) die(`IR 方法锚 ${anchors} != 99`);
if (ir.split("\n").length !== 1092) die(`行数 ${ir.split("\n").length} != 1092`);

let rust = banner(GEN, IR_REL, "S390 maglev-ir 跳过登记 + 预算枚举 (非目标架构)");
rust += `
//! 规则九: 平台分支按 x64+arm64 展开, S390 跳过语义转换。
//! 本文件语义为各 IR 节点 GenerateCode/SetValueLocationConstraints (99 锚),
//! 不展开; 仅登记文件面与预算类型枚举。

/// 原版 enum class ReduceInterruptBudgetType (中断预算计费点)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ReduceInterruptBudgetType {
${variants.map((v, i) => `  /// ${v}\n  ${v.replace(/^k/, "")} = ${i},`).join("\n")}
}

pub static REGISTERED_FILES: &[(&str, usize, usize)] = &[
  ("src/maglev/s390/maglev-ir-s390.cc", 1092, 99),
];

const _: () = {
  assert!(REGISTERED_FILES.len() == 1);
  assert!(REGISTERED_FILES[0].1 == 1092);
  assert!(REGISTERED_FILES[0].2 == 99);
};
`;

const p = writeSrc("v8-maglev-backend", "s390/maglev_ir_s390.rs", rust);
console.log(`OK ${GEN}: enum=${variants.length} anchors=${anchors} → ${p}`);
