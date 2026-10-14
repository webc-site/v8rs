#!/usr/bin/env bun
// tool/src/wasm/stacks.h.js
// 机械提取 v8/src/wasm/stacks.h 的栈状态枚举与安全偏移

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/stacks.h");

// 1. StackState
const stateMatch = hSrc.match(/enum StackState : int32_t \{([\s\S]*?)\n  \};/);
if (!stateMatch) throw new Error("未找到 StackState");
const states = [];
let doc = [];
for (const rawLine of stateMatch[1].split("\n")) {
  let line = rawLine.trim();
  if (!line) continue;
  // 同行尾注并入文档
  let tail = "";
  const tailIdx = line.indexOf("//");
  if (tailIdx >= 0) {
    tail = line.slice(tailIdx + 2).trim();
    line = line.slice(0, tailIdx).trim();
  }
  if (!line) {
    // 整行注释: 记为文档
    doc.push(tail);
    continue;
  }
  const m = line.match(/^(\w+),?$/);
  if (!m) throw new Error(`无法解析 StackState 行: "${rawLine}"`);
  doc.push(tail);
  states.push({ name: m[1], doc: doc.filter(Boolean).join(" ") });
  doc = [];
}
if (states.map((s) => s.name).join(",") !== "Active,Suspended,Inactive,Retired") {
  throw new Error(`StackState 内容不符: ${states.map((s) => s.name)}`);
}

// 2. 栈基安全偏移 (USE_SIMULATOR 分支: 20 * kSystemPointerSize; 其余 0)
const simMatch = hSrc.match(/constexpr int kStackBaseSafetyOffset = (\d+) \* kSystemPointerSize;/);
const plainMatch = hSrc.match(/#else\n\s*constexpr int kStackBaseSafetyOffset = (\d+);/);
if (!simMatch || !plainMatch) throw new Error("未找到 kStackBaseSafetyOffset 分支");
const K_SYSTEM_POINTER_SIZE = 8;
const simOffset = parseInt(simMatch[1], 10) * K_SYSTEM_POINTER_SIZE;
const plainOffset = parseInt(plainMatch[1], 10);

console.log(`stacks.h: StackState=${states.length} simOffset=${simOffset} plainOffset=${plainOffset}`);

let rust = banner(
  "src/wasm/stacks.h.js",
  "src/wasm/stacks.h",
  "Wasm 栈内存状态机与栈基安全偏移",
);

rust += `
/// 栈生命周期状态 (对应原版 StackState : int32_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum StackState {
`;
for (const [i, s] of states.entries()) {
  rust += `  /// ${s.name}${s.doc ? ` — ${s.doc}` : ""}\n  ${s.name} = ${i},\n`;
}
rust += `}

/// 栈基安全偏移 (USE_SIMULATOR 分支: 20 * kSystemPointerSize;
/// 模拟器为运行时调用预留映射区, 防栈起点附近未映射崩溃)
pub const STACK_BASE_SAFETY_OFFSET_SIMULATOR: i32 = ${simOffset};

/// 栈基安全偏移 (非模拟器: 0)
pub const STACK_BASE_SAFETY_OFFSET: i32 = ${plainOffset};

/// 指针宽 (x64/arm64, 原版 kSystemPointerSize)
pub const SYSTEM_POINTER_SIZE: i32 = ${K_SYSTEM_POINTER_SIZE};

// —— 提取不变式断言 ——
const _: () = {
  assert!(StackState::Retired as i32 == 3);
  assert!(STACK_BASE_SAFETY_OFFSET_SIMULATOR == 160);
  assert!(STACK_BASE_SAFETY_OFFSET == 0);
};
`;

const p = writeSrc("v8-wasm-core", "stacks_consts.rs", rust);
console.log(`written ${p}`);
