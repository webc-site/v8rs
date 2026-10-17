#!/usr/bin/env bun
// tool/src/wasm/baseline/liftoff-varstate.h.js
// 机械提取 v8/src/wasm/baseline/liftoff-varstate.h 的 VarState 位置枚举

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/wasm/baseline/liftoff-varstate.h");

const locMatch = hSrc.match(/enum Location : uint8_t \{([^}]*)\};/);
if (!locMatch) throw new Error("未找到 Location");
const locs = locMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (locs.join(",") !== "kStack,kRegister,kIntConst") {
  throw new Error(`Location 内容不符: ${locs}`);
}

console.log(`liftoff-varstate.h: Location=${locs.length}`);

let rust = banner(
  "src/wasm/baseline/liftoff-varstate.h.js",
  "src/wasm/baseline/liftoff-varstate.h",
  "Liftoff 值栈槽位状态的位置类别",
);

rust += `
/// 值的位置 (对应原版 LiftoffVarState::Location : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VarStateLocation {
  /// kStack — 已溢出到栈槽
  Stack = 0,
  /// kRegister — 驻留寄存器
  Register = 1,
  /// kIntConst — 常量折叠
  IntConst = 2,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(VarStateLocation::IntConst as u8 == 2);
};
`;

const p = writeSrc("v8-wasm-liftoff", "liftoff_varstate.rs", rust);
console.log(`written ${p}`);
