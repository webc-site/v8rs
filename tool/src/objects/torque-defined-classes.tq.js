#!/usr/bin/env bun
// tool/src/objects/torque-defined-classes.tq.js — Torque 定义类归档登记
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/torque-defined-classes.tq → v8-objects-rest/src/torque_defined_classes.rs
// 规则面: 该文件为纯注释归档说明 (0 类定义); 断言空并登记归属规则
import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/torque-defined-classes.tq");

// —— 空文件断言: 除注释外无任何类/宏/常量 ——
const codeLines = tq.split("\n").map((l) => l.trim()).filter((l) => l && !l.startsWith("//"));
if (codeLines.length !== 0) throw new Error(`torque-defined-classes.tq 应为纯注释归档 (0 代码行), 实得 ${codeLines.length}: ${codeLines}`);
// 归档语义锚
if (!tq.includes("Classes defined in Torque that are not exported are attributed to this file"))
  throw new Error("归档说明锚缺失");

const body = `${banner("src/objects/torque-defined-classes.tq.js", "src/objects/torque-defined-classes.tq", "空归档登记: 0 类定义")}
//! Torque 定义类归档 (原版 torque-defined-classes.tq)
//!
//! 原版该文件是纯注释占位: 非导出 Torque 类在生成器输出中统一归档到
//! "torque-defined-classes" 名下, 因此源文件本身 0 定义.
//! Rust 侧无需对应物, 本模块仅为函数级对账 (12.1) 计"已处理/空".

/// 登记标记: 0 类 / 0 常量 (对账时按空文件计)
pub const TORQUE_DEFINED_CLASSES_COUNT: usize = 0;
`;

const p = writeSrc("v8-objects-rest", "torque_defined_classes.rs", body);
console.log(`torque-defined-classes.tq.js: 已生成 ${p} (代码行 0, 空归档断言通过)`);
