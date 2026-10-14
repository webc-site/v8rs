#!/usr/bin/env bun
// tool/src/objects/function-syntax-kind.h.js — 覆盖 v8/src/objects/function-syntax-kind.h
// FunctionSyntaxKind 语法种类枚举与字符串表
// → v8-objects-js-func/src/function_syntax_kind.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/function-syntax-kind.h");

const enumMatch = hSrc.match(/enum class FunctionSyntaxKind : uint8_t \{([\s\S]*?)\n\};/);
if (!enumMatch) throw new Error("未找到 FunctionSyntaxKind 枚举");

const entries = [];
for (const line of enumMatch[1].split("\n")) {
  const t = line.trim();
  const cm = t.match(/^(k\w+),$/);
  if (cm) entries.push(cm[1]);
}
const last = enumMatch[1].match(/kLastFunctionSyntaxKind = (k\w+)/);
if (!last) throw new Error("未找到 kLastFunctionSyntaxKind 哨兵");
if (entries.length !== 5) throw new Error(`FunctionSyntaxKind 条数不符: 期望 5, 得 ${entries.length}`);
if (entries[entries.length - 1] !== last[1]) throw new Error("哨兵与末项不符");

const strBody = hSrc.match(/FunctionSyntaxKind2String\(FunctionSyntaxKind kind\) \{([\s\S]*?)\n  \}/);
if (!strBody) throw new Error("未找到 FunctionSyntaxKind2String");
const strMap = new Map();
for (const m of strBody[1].matchAll(/case FunctionSyntaxKind::(k\w+):\s*return "([^"]*)";/g)) strMap.set(m[1], m[2]);
if (strMap.size !== 5) throw new Error(`字符串表条数不符: 期望 5, 得 ${strMap.size}`);
for (const e of entries) if (!strMap.has(e)) throw new Error(`2String 缺 ${e}`);

const pascal = (k) => k.replace(/^k/, "").replace(/^./, (c) => c.toLowerCase());

console.log(`FunctionSyntaxKind ${entries.length} 项`);

let rust = banner("src/objects/function-syntax-kind.h.js", "src/objects/function-syntax-kind.h", "FunctionSyntaxKind 语法种类");

rust += `
/// 函数语法种类 (原版 enum class FunctionSyntaxKind : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FunctionSyntaxKind {
`;

entries.forEach((e, i) => {
  rust += `  /// ${e}\n  ${e.replace(/^k/, "")} = ${i},\n`;
});

rust += `}

impl FunctionSyntaxKind {
  /// 原版 kLastFunctionSyntaxKind
  pub const LAST: Self = Self::${last[1].replace(/^k/, "")};

  /// 原版 FunctionSyntaxKind2String
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
`;
for (const e of entries) rust += `      Self::${e.replace(/^k/, "")} => "${strMap.get(e)}",\n`;
rust += `    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FunctionSyntaxKind::LAST as u8 == 4);
  assert!(FunctionSyntaxKind::Declaration.to_str() == "Declaration");
};
`;

const p = writeSrc("v8-objects-js-func", "function_syntax_kind.rs", rust);
console.log(`written ${p}`);
