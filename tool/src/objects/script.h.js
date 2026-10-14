#!/usr/bin/env bun
// tool/src/objects/script.h.js — 覆盖 v8/src/objects/script.{h,tq}
// → v8-objects-rest/src/script.rs
// 机械提取: Script::Type / CompilationState / CompilationKind / OffsetFlag 枚举,
//   kTemporaryScriptId, PositionInfo 结构骨架; 平台分支按 WASM 开展开。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/script.h");

// 目标配置 (与 v8rs 一致): V8_ENABLE_WEBASSEMBLY 开 → #if 段保留
function stripConfigComments(src) {
  return src
    .replace(/\/\*[\s\S]*?\*\//g, "")
    .replace(/^[ \t]*\/\/.*$/gm, "")
    // 机械展开: #if V8_ENABLE_WEBASSEMBLY 内容保留, 其余预处理行删除
    .replace(/^[ \t]*#(else|elif|endif)[^\n]*$/gm, "")
    .replace(/^[ \t]*#(if|ifdef|ifndef)[^\n]*$/gm, "");
}

const src = stripConfigComments(hSrc);

function parseEnum(re, label) {
  const m = src.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  return m[1]
    .split(",")
    .map((s) => s.trim())
    .filter((s) => s && !s.startsWith("#"));
}

// Type 枚举: kWasm 段被 #if V8_ENABLE_WEBASSEMBLY 包裹 (目标配置: 保留)
const typeEntries = parseEnum(/enum class Type\s*\{([^}]+)\}/, "Script::Type");
if (typeEntries.length !== 5) throw new Error(`Script::Type 条数 ${typeEntries.length} != 5 (kNative..kInspector, WASM 开)`);

const compState = parseEnum(/enum class CompilationState\s*\{([^}]+)\}/, "CompilationState");
if (compState.length !== 2) throw new Error(`CompilationState 条数 ${compState.length} != 2`);

const compKind = parseEnum(/enum class CompilationKind\s*\{([^}]+)\}/, "CompilationKind");
if (compKind.length !== 5) throw new Error(`CompilationKind 条数 ${compKind.length} != 5`);

const offsetFlag = parseEnum(/enum class OffsetFlag\s*\{([^}]+)\}/, "OffsetFlag");
if (offsetFlag.length !== 2) throw new Error(`OffsetFlag 条数 ${offsetFlag.length} != 2`);

const tempId = src.match(/static constexpr int kTemporaryScriptId = (-?\d+);/);
if (!tempId) throw new Error("未找到 kTemporaryScriptId");
const kTemporaryScriptId = parseInt(tempId[1], 10);

// 显式值锚: kNative=0, kExtension=1, kNormal=2, kWasm=3, kInspector=4 (WASM 开)
const anchors = ["kNative = 0", "kExtension = 1", "kNormal = 2", "kWasm = 3", "kInspector = 4"];
for (const a of anchors) {
  if (!src.includes(a)) throw new Error(`Script::Type 锚缺失: ${a}`);
}

const pascal = (s) => s.replace(/^k/, "").toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => c.toUpperCase());

function emitEnum(name, entries, explicit) {
  let s = `/// 对应原版 Script::${name}, 共 ${entries.length} 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ${name} {
`;
  entries.forEach((e, i) => {
    s += `  /// ${e}\n  ${pascal(e)} = ${explicit ? explicit[i] : i},\n`;
  });
  s += `}

impl ${name} {
  /// 变体总数
  pub const COUNT: usize = ${entries.length};

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
`;
  entries.forEach((e, i) => {
    s += `      ${explicit ? explicit[i] : i} => Some(Self::${pascal(e)}),\n`;
  });
  s += `      _ => None,
    }
  }
}

`;
  return s;
}

let rust = banner("src/objects/script.h.js", "src/objects/script.{h,tq}", "Script 类型/编译状态枚举与位置信息骨架");
rust += `//! Script: 脚本元数据 (源/位置/编译状态)。

/// 临时脚本 ID (不加入 script list)。
pub const TEMPORARY_SCRIPT_ID: i32 = ${kTemporaryScriptId};

`;
rust += emitEnum("ScriptType", typeEntries, [0, 1, 2, 3, 4]);
rust += emitEnum("CompilationState", compState);
rust += emitEnum("CompilationKind", compKind);
rust += emitEnum("OffsetFlag", offsetFlag);

rust += `/// 源位置信息 (原版 Script::PositionInfo 骨架; 语义实现由 Developer 补)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositionInfo {
  /// 零基行号。
  pub line: i32,
  /// 零基列号。
  pub column: i32,
  /// 行首字符位置。
  pub line_start: i32,
  /// 行尾换行位置。
  pub line_end: i32,
}

impl Default for PositionInfo {
  fn default() -> Self {
    Self { line: -1, column: -1, line_start: -1, line_end: -1 }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(ScriptType::COUNT == 5);
  assert!(ScriptType::Wasm as u8 == 3);
  assert!(ScriptType::Inspector as u8 == 4);
  assert!(CompilationState::COUNT == 2);
  assert!(CompilationKind::COUNT == 5);
  assert!(OffsetFlag::COUNT == 2);
  assert!(TEMPORARY_SCRIPT_ID == -2);
};
`;

const p = writeSrc("v8-objects-rest", "script.rs", rust);
console.log(`script.rs: 4 枚举 5+2+5+2 变体 + kTemporaryScriptId + PositionInfo 骨架, 对数通过`);
