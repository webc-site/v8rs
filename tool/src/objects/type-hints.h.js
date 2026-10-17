#!/usr/bin/env bun
// tool/src/objects/type-hints.h.js — 覆盖 v8/src/objects/type-hints.h + type-hints.cc
// → v8-objects-rest/src/type_hints.rs
// 机械提取: BinaryOperationHint / CompareOperationHint / ForInHint / StringAddFlags
//   四枚举 + .cc 的 ostream 显示名表 (1:1 对数)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/type-hints.h");
const ccSrc = readV8("src/objects/type-hints.cc");

// 去注释 (保护字符串/字符字面量)
function stripComments(src) {
  let out = "", i = 0, inStr = false, inChr = false;
  while (i < src.length) {
    const c = src[i], n = src[i + 1];
    if (inStr || inChr) {
      out += c;
      if (c === "\\") { out += n ?? ""; i += 2; continue; }
      if (inStr && c === '"') inStr = false;
      if (inChr && c === "'") inChr = false;
      i++; continue;
    }
    if (c === '"') { inStr = true; out += c; i++; continue; }
    if (c === "'") { inChr = true; out += c; i++; continue; }
    if (c === "/" && n === "/") { while (i < src.length && src[i] !== "\n") i++; continue; }
    if (c === "/" && n === "*") { i += 2; while (i < src.length && !(src[i] === "*" && src[i + 1] === "/")) i++; i += 2; continue; }
    out += c; i++;
  }
  return out;
}

// 枚举体条目切分 (括号深度 0 处按逗号切)
function splitEntries(body) {
  const parts = [];
  let depth = 0, cur = "";
  for (const ch of body) {
    if (ch === "(" || ch === "<") depth++;
    else if (ch === ")" || ch === ">") depth--;
    if (ch === "," && depth === 0) { parts.push(cur); cur = ""; continue; }
    cur += ch;
  }
  if (cur.trim()) parts.push(cur);
  return parts.map((p) => p.trim()).filter(Boolean);
}

// 解析 enum class X : T { ... } / enum X { ... }
function parseEnum(src, re, label) {
  const m = src.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  return splitEntries(m[1]).map((e) => e.replace(/^k/, ""));
}

// 从 cc 的 operator<< switch 提取显示名表
function parseDisplayNames(cc, hintName, label) {
  const re = new RegExp(
    `operator<<\\(std::ostream&[^)]*${hintName}[^)]*\\)\\s*\\{([\\s\\S]*?)UNREACHABLE`,
  );
  const m = cc.match(re);
  if (!m) throw new Error(`未找到 ${label} 显示名 switch`);
  const out = new Map();
  const caseRe = /case\s+\w+::(\w+):\s*(?:\n\s*)?return os << "([^"]+)";/g;
  let cm;
  while ((cm = caseRe.exec(m[1])) !== null) out.set(cm[1], cm[2]);
  return out;
}

const binHints = parseEnum(hSrc, /enum class BinaryOperationHint : uint8_t\s*\{([^}]+)\}/, "BinaryOperationHint");
const cmpHints = parseEnum(hSrc, /enum class CompareOperationHint : uint8_t\s*\{([^}]+)\}/, "CompareOperationHint");
const forInHints = parseEnum(hSrc, /enum class ForInHint : uint8_t\s*\{([^}]+)\}/, "ForInHint");
const stringAddFlags = parseEnum(
  stripComments(hSrc),
  /enum StringAddFlags\s*\{([^}]+)\}/,
  "StringAddFlags",
);

// 对数: 原版枚举条目数 (clang -E 后枚举体 1:1)
if (binHints.length !== 11) throw new Error(`BinaryOperationHint 条数 ${binHints.length} != 11`);
if (cmpHints.length !== 14) throw new Error(`CompareOperationHint 条数 ${cmpHints.length} != 14`);
if (forInHints.length !== 4) throw new Error(`ForInHint 条数 ${forInHints.length} != 4`);
if (stringAddFlags.length !== 3) throw new Error(`StringAddFlags 条数 ${stringAddFlags.length} != 3`);

const binNames = parseDisplayNames(ccSrc, "BinaryOperationHint", "BinaryOperationHint");
const cmpNames = parseDisplayNames(ccSrc, "CompareOperationHint", "CompareOperationHint");
const forInNames = parseDisplayNames(ccSrc, "ForInHint", "ForInHint");
const flagNames = (() => {
  const m = ccSrc.match(/operator<<\(std::ostream& os, const StringAddFlags& flags\)\s*\{([\s\S]*?)UNREACHABLE/);
  if (!m) throw new Error("未找到 StringAddFlags 显示名 switch");
  const out = new Map();
  const caseRe = /case\s+(\w+):\s*(?:\n\s*)?return os << "([^"]+)";/g;
  let cm;
  while ((cm = caseRe.exec(m[1])) !== null) out.set(cm[1], cm[2]);
  return out;
})();

if (binNames.size !== 11) throw new Error(`BinaryOperationHint 显示名 ${binNames.size} != 11`);
if (cmpNames.size !== 14) throw new Error(`CompareOperationHint 显示名 ${cmpNames.size} != 14`);
if (forInNames.size !== 4) throw new Error(`ForInHint 显示名 ${forInNames.size} != 4`);
if (flagNames.size !== 3) throw new Error(`StringAddFlags 显示名 ${flagNames.size} != 3`);

// PascalCase 变体名
const pascal = (s) =>
  s.toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => c.toUpperCase());

function emitEnum(rustName, reprType, entries, names, rawPrefixK) {
  let s = `/// 对应原版 enum ${rawPrefixK ? "" : "class "}${rustName} : ${reprType}, 共 ${entries.length} 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(${reprType})]
pub enum ${rustName} {
`;
  entries.forEach((e, i) => {
    const disp = names.get(rawPrefixK ? `k${e}` : e);
    s += `  /// ${rawPrefixK ? "k" : "k"}${e} → "${disp}"
  ${pascal(e)} = ${i},
`;
  });
  s += `}

impl ${rustName} {
  /// 变体总数
  pub const COUNT: usize = ${entries.length};

  /// 原版 operator<< 显示名
  #[inline]
  #[must_use]
  pub const fn display_name(self) -> &'static str {
    match self {
`;
  entries.forEach((e, i) => {
    s += `      Self::${pascal(e)} => "${names.get(rawPrefixK ? `k${e}` : e)}",
`;
  });
  s += `    }
  }

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: ${reprType}) -> Option<Self> {
    match v {
`;
  entries.forEach((e, i) => {
    s += `      ${i} => Some(Self::${pascal(e)}),
`;
  });
  s += `      _ => None,
    }
  }
}

`;
  return s;
}

let rust = banner("src/objects/type-hints.h.js", "src/objects/type-hints.{h,cc}", "类型反馈提示枚举与显示名表");
rust += `//! 类型提示 (Type Hints): 二元/比较/for-in 操作的 IC 反馈类别。

`;

rust += emitEnum("BinaryOperationHint", "u8", binHints, binNames, true);
rust += emitEnum("CompareOperationHint", "u8", cmpHints, cmpNames, true);
rust += emitEnum("ForInHint", "u8", forInHints, forInNames, true);

// StringAddFlags: 原版非 class 枚举 (TODO(ishell) 注明), 值 0..2
rust += `/// 字符串拼接参数检查标志 (原版 enum StringAddFlags, 非 class 枚举)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum StringAddFlags {
`;
stringAddFlags.forEach((f, i) => {
  rust += `  /// ${f} → "${flagNames.get(f)}"
  ${pascal(f)} = ${i},
`;
});
rust += `}

impl StringAddFlags {
  /// 变体总数
  pub const COUNT: usize = ${stringAddFlags.length};
}

`;

// 提取不变式断言
rust += `// —— 提取不变式断言 ——
const _: () = {
  assert!(BinaryOperationHint::COUNT == 11);
  assert!(CompareOperationHint::COUNT == 14);
  assert!(ForInHint::COUNT == 4);
  assert!(StringAddFlags::COUNT == 3);
  assert!(BinaryOperationHint::None as u8 == 0);
  assert!(CompareOperationHint::Any as u8 == 13);
  assert!(ForInHint::Any as u8 == 3);
  assert!(StringAddFlags::StringAddConvertRight as u8 == 2);
};
`;

const p = writeSrc("v8-objects-rest", "type_hints.rs", rust);
console.log(`type_hints.rs: 4 枚举 11+14+4+3=32 变体, 显示名表 32 条, 全部对数通过`);
