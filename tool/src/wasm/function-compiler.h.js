#!/usr/bin/env bun
// tool/src/wasm/function-compiler.h.js
// 机械提取 v8/src/wasm/function-compiler.h 的编译结果 Kind 枚举与 Validation 常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/function-compiler.h");

// 1. Validation 常量
const valMatch = hSrc.match(/constexpr Validation kAlreadyValidated\{false\};[\s\S]*?kMustValidate\{true\};/);
if (!valMatch) throw new Error("未找到 Validation 常量");

// 2. WasmCompilationResult::Kind (V8_ENABLE_DRUMBRAKE 分支展开: 开=4 项, 关=3 项)
const kindMatch = hSrc.match(/enum Kind : int8_t \{([\s\S]*?)\n  \};/);
if (!kindMatch) throw new Error("未找到 WasmCompilationResult::Kind");
const raw = kindMatch[1]
  .split("\n")
  .filter((l) => !/^\s*#(if|endif|else)/.test(l)) // 剥 #if/#endif 行
  .join("\n")
  .replace(/\/\/.*$/gm, "");
const kinds = raw.split(",").map((s) => s.trim()).filter(Boolean);
if (kinds.join(",") !== "kFunction,kWasmToJsWrapper,kStackEntryWrapper,kInterpreterEntry") {
  throw new Error(`Kind 内容不符: ${kinds}`);
}
// 关闭 DRUMBRAKE 时 kInterpreter 不存在, 关=3 项; 开启时 kInterpreter=3
const KIND_DOC = {
  kFunction: "普通 wasm 函数",
  kWasmToJsWrapper: "wasm→JS 包装器",
  kStackEntryWrapper: "栈入口包装器",
  kInterpreterEntry: "解释器入口 (仅 V8_ENABLE_DRUMBRAKE)",
};

// 3. ExecutionTierParm? 检查 for_debugging / tier 常量
const hasResult = /struct WasmCompilationResult/.test(hSrc);
if (!hasResult) throw new Error("未找到 WasmCompilationResult");

console.log(`function-compiler.h: Kind=${kinds.length} (含 DRUMBRAKE 条目)`);

let rust = banner(
  "src/wasm/function-compiler.h.js",
  "src/wasm/function-compiler.h",
  "Wasm 函数编译结果类别与校验标记",
);

rust += `
/// 校验标记 (原版 base::StrongAlias<ValidationFlagTag, bool>)
pub type Validation = bool;

/// 输入已通过校验 (原版 kAlreadyValidated)
pub const ALREADY_VALIDATED: Validation = false;

/// 输入需要校验 (原版 kMustValidate)
pub const MUST_VALIDATE: Validation = true;

/// 编译结果类别 (对应原版 WasmCompilationResult::Kind)
/// V8_ENABLE_DRUMBRAKE 关闭时为 3 项; 开启时 kInterpreter 亦有效
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum CompilationResultKind {
`;

for (const k of kinds) {
  const drumbakeOnly = k === "kInterpreterEntry";
  const line = `  /// ${k} — ${KIND_DOC[k]}\n`;
  if (drumbakeOnly) {
    rust += line + `  // InterpreterEntry = 3, // 仅 V8_ENABLE_DRUMBRAKE\n`;
  } else {
    rust += line + `  ${k.replace(/^k/, "")} = ${kinds.indexOf(k)},\n`;
  }
}

rust += `}

impl CompilationResultKind {
  /// DRUMBRAKE 开启时 kInterpreter 的取值 (原版枚举顺序位置)
  pub const INTERPRETER_ENTRY_REPR: i8 = 3;
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(MUST_VALIDATE);
  assert!(!ALREADY_VALIDATED);
  assert!(CompilationResultKind::Function as i8 == 0);
  assert!(CompilationResultKind::StackEntryWrapper as i8 == 2);
};
`;

const p = writeSrc("v8-wasm-core", "function_compiler_consts.rs", rust);
console.log(`written ${p}`);
