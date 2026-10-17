#!/usr/bin/env bun
// tool/src/ast/scopes.h.js
// 机械提取 v8/src/ast/scopes.h — DeserializationMode/RareVariable 枚举 +
// RareData 布局常量锚 + Scope 类面骨架。
// 注: scopes.rs (手写) 与 scopes_cc_generated.rs (.cc 侧) 已存在,
//     本脚本落 scopes_generated.rs (ast.h.js/ast.cc.js 同款命名), 禁覆写已有文件。
// 产物: v8-ast/src/scopes_generated.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/ast/scopes.h";
const h = readV8(H_REL);

const dmM = h.match(/enum class DeserializationMode\s*\{([^}]+)\};/);
if (!dmM) throw new Error("未找到 DeserializationMode 枚举");
const dm = dmM[1].replace(/\/\/[^\n]*/g, "").split(",").map((s) => s.trim()).filter(Boolean);
if (dm.length !== 2) throw new Error(`DeserializationMode 条数不符: ${dm.length} != 2`);

const rvM = h.match(/enum class RareVariable\s*\{([^}]+)\};/);
if (!rvM) throw new Error("未找到 RareVariable 枚举");
const rv = [...rvM[1].matchAll(/k(\w+)\s*=\s*offsetof\((\w+),\s*(\w+)\)/g)].map((m) => ({
  name: "k" + m[1], struct: m[2], field: m[3],
}));
if (rv.length !== 2 || rv[0].struct !== "RareData") throw new Error("RareVariable offsetof 锚不符");

const availM = h.match(/static constexpr int kAvailableBits = (\d+);/);
if (!availM || availM[1] !== "1") throw new Error("kAvailableBits 锚不符");
if (!h.includes("struct RareData")) throw new Error("缺 RareData 结构锚");

let rust = banner("src/ast/scopes.h.js", H_REL, "作用域枚举与骨架");
rust += `
// 语义面: Scope/DeclarationScope 变量解析 (~1700 行) 登记跳过,
// 由手写 scopes.rs 权威覆盖; 本文件登记枚举与布局登记面。

/// 反序列化模式 (原 enum class DeserializationMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DeserializationMode {
  /// 含变量
  IncludingVariables = 0,
  /// 仅作用域
  ScopesOnly = 1,
}

/// 稀疏变量槽 (原 enum class RareVariable = offsetof(RareData, 字段)):
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RareVariable {
${rv.map((r) => `  /// ${r.name} = offsetof(${r.struct}, ${r.field})\n  ${r.name.replace(/^k/, "")} = 0,`).join("\n")}
}
// 注: 槽位偏移依赖 RareData 内存布局 (this_function/generator_object 两 Variable* 槽),
// Rust 侧建议以显式槽位数组替代 offsetof 语义, 由 Developer 裁定。

/// 作用域可用位标记 (原 kAvailableBits)
pub const AVAILABLE_BITS: i32 = ${availM[1]};

/// 原 RareData — 两槽稀疏变量表
pub struct RareData;

/// 原 class Scope 骨架 (变量登记/查找/反序列化) — 手写 scopes.rs 权威。
pub struct Scope;

// —— 提取不变式断言 ——
const _: () = {
  assert!(AVAILABLE_BITS == 1);
};
`;

const p = writeSrc("v8-ast", "scopes_generated.rs", rust);
console.log(`written ${p} (DM ${dm.length}, RV ${rv.length})`);
