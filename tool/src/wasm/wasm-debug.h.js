#!/usr/bin/env bun
// tool/src/wasm/wasm-debug.h.js
// 机械提取 v8/src/wasm/wasm-debug.h 的 DebugSideTable::Entry::Storage 枚举

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-debug.h");

const stMatch = hSrc.match(/enum Storage : int8_t \{([^}]*)\};/);
if (!stMatch) throw new Error("未找到 DebugSideTable::Entry::Storage");
const storages = stMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (storages.join(",") !== "kConstant,kRegister,kStack") {
  throw new Error(`Storage 内容不符: ${storages}`);
}

// DebugSideTable 存在性
if (!/class DebugSideTable/.test(hSrc)) throw new Error("未找到 DebugSideTable");

console.log(`wasm-debug.h: Storage=${storages.length}`);

let rust = banner(
  "src/wasm/wasm-debug.h.js",
  "src/wasm/wasm-debug.h",
  "调试侧表条目值的存储类别",
);

rust += `
/// DebugSideTable 条目值的存放类别 (对应原版 DebugSideTable::Entry::Storage : int8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum DebugSideTableStorage {
  /// kConstant — 常量值
  Constant = 0,
  /// kRegister — 寄存器 (reg_code)
  Register = 1,
  /// kStack — 栈槽 (stack_offset)
  Stack = 2,
}

impl DebugSideTableStorage {
  /// 总数
  pub const COUNT: usize = ${storages.length};
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(DebugSideTableStorage::Stack as i8 == 2);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_debug_consts.rs", rust);
console.log(`written ${p}`);
