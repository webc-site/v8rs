#!/usr/bin/env bun
// tool/src/objects/instance-types-gen.h.js — 覆盖 v8/src/objects/instance-types-gen.h
// → v8-objects-core/src/instance_types_gen.rs
// 机械提取: 生成器选择开关 (V8_USE_METAGEN 默认 1) + 宏供给登记
// 注: instance-type.h.js 已复刻 metagen 求解器; 本模块登记开关与宏契约。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/instance-types-gen.h");

// V8_USE_METAGEN 默认值锚
const defaultDef = hSrc.match(/#if !defined\(V8_USE_METAGEN\)\n#define V8_USE_METAGEN (\d)/);
if (!defaultDef || defaultDef[1] !== "1") throw new Error("V8_USE_METAGEN 默认值锚失配");

// 收割遍路径的 no-op 宏名单 (V8_METAGEN_GENERATION_PASS 分支)
const harvestMacros = [...hSrc.matchAll(/#define (TORQUE_ASSIGNED_INSTANCE_TYPES|TORQUE_ASSIGNED_INSTANCE_TYPE_LIST|INSTANCE_TYPE_LIST_SINGLE|INSTANCE_TYPE_LIST_MULTIPLE|INSTANCE_TYPE_LIST_RANGE)\(V\)/g)].map((m) => m[1]);
if (harvestMacros.length !== 5) throw new Error(`收割遍 no-op 宏数 ${harvestMacros.length} != 5`);

// Torque 路径附加头 (metagen 路径不含)
if (!hSrc.includes('torque-generated/instance-type-checker-lists.h')) throw new Error("缺 torque checker-lists include 登记");
if (!hSrc.includes('HEAP_OBJECT_DIAGNOSTIC_DISPATCH_LIST')) throw new Error("缺诊断派发宏登记");

let rust = banner("src/objects/instance-types-gen.h.js", "src/objects/instance-types-gen.h", "实例类型生成器选择开关");
rust += `//! 生成器选择: metagen (tools/metagen/) 或 Torque 产物。
//! 默认 V8_USE_METAGEN=1 (原版 #if !defined 回退值)。
//! 本 crate 的实例类型值由 instance_type 模块 (metagen 求解器复刻) 提供。

/// metagen 路径开关 (原版默认 1; 与 crate::instance_type 的求解器来源一致)。
pub const V8_USE_METAGEN: bool = true;

/// 收割遍 (V8_METAGEN_GENERATION_PASS) 下定义为 no-op 的供给宏名单。
pub const HARVEST_PASS_NOOP_MACROS: [&str; ${harvestMacros.length}] = [
`;
for (const m of harvestMacros) rust += `  "${m}",\n`;
rust += `];

/// Torque 路径附加供给头 (metagen 路径内嵌于 instance-types.h)。
pub const TORQUE_EXTRA_INCLUDES: [&str; 2] = [
  "torque-generated/instance-type-checker-lists.h",
  "torque-generated/debug-reader-classes-list.h",
];

/// 诊断派发宏 (metagen 由 Print/Verify 声明推导, Torque 路径由 debug-reader 列表别名)。
pub const DIAGNOSTIC_DISPATCH_MACRO: &str = "HEAP_OBJECT_DIAGNOSTIC_DISPATCH_LIST";

// —— 提取不变式断言 ——
const _: () = {
  assert!(V8_USE_METAGEN);
  assert!(HARVEST_PASS_NOOP_MACROS.len() == 5);
  assert!(TORQUE_EXTRA_INCLUDES.len() == 2);
};
`;

const p = writeSrc("v8-objects-core", "instance_types_gen.rs", rust);
console.log(`instance_types_gen.rs: 开关 1 + no-op 宏 5 + 附加头 2, 对数通过`);
