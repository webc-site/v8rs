#!/usr/bin/env bun
// tool/src/regexp/regexp-ast.h.js — v8/src/regexp/regexp-ast.h → v8-regexp-syntax/src/ast_generated.rs
// 机械提取 FOR_EACH_REG_EXP_TREE_TYPE X-macro (节点种类) + StandardCharacterSet 字符表
// + ClassRanges Flag 位 + Assertion::Type 序数, 对数断言; 结构体语义在 ast.rs
import { banner, readV8, writeSrc } from "../../lib.js";

const src = readV8("src/regexp/regexp-ast.h");

// 1. RegExpTree 节点种类
const mk = src.match(/#define FOR_EACH_REG_EXP_TREE_TYPE\(VISIT\)([\s\S]*?)\n\n/);
if (!mk) throw new Error("regexp-ast.h: FOR_EACH_REG_EXP_TREE_TYPE 结构漂移");
const kinds = [...mk[1].matchAll(/VISIT\((\w+)\)/g)].map((r) => r[1]);
if (kinds.length === 0) throw new Error("FOR_EACH_REG_EXP_TREE_TYPE 零条目");

// 2. StandardCharacterSet (char 值表)
const scs = src.match(/enum class StandardCharacterSet : char \{([\s\S]*?)\};/);
if (!scs) throw new Error("StandardCharacterSet 结构漂移");
const scsRows = [...scs[1].matchAll(/k(\w+) = '(.)',/g)].map((r) => ({ name: r[1], ch: r[2] }));

// 3. ClassRanges::Flag 位
const crf = src.match(/enum Flag \{([\s\S]*?)\};/);
if (!crf) throw new Error("ClassRanges Flag 结构漂移");
const crfRows = [...crf[1].matchAll(/(\w+) = 1 << (\d+)/g)].map((r) => ({ name: r[1], bit: Number(r[2]) }));

// 4. Assertion::Type 序数 (SCREAMING → PascalCase 承接, 变体名本仓惯例)
const pascal = (s) => s.split("_").map((w) => w[0] + w.slice(1).toLowerCase()).join("");
const at = src.match(/enum class Type \{([\s\S]*?)\s*LAST_ASSERTION_TYPE/);
if (!at) throw new Error("Assertion::Type 结构漂移");
const atRows = [...at[1].matchAll(/(\w+) = (\d+)/g)].map((r) => ({ name: pascal(r[1]), val: Number(r[2]) }));

console.log(
  `regexp-ast: 节点种类 ${kinds.length}, 标准字符集 ${scsRows.length}, ClassRanges 位 ${crfRows.length}, 断言类型 ${atRows.length} ✓`,
);

const lastKind = kinds[kinds.length - 1];
const lastScs = scsRows[scsRows.length - 1].name;
const lastAt = atRows[atRows.length - 1].name;

const body = `${banner("src/regexp/regexp-ast.h.js", "src/regexp/regexp-ast.h", "X-macro/枚举展开; 节点结构体与语义在 ast.rs")}
/// RegExpTree 节点种类全集 (FOR_EACH_REG_EXP_TREE_TYPE 展开)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TreeKind {
${kinds.map((k, i) => `  ${k} = ${i},`).join("\n")}
}

pub const NUM_TREE_KINDS: usize = ${kinds.length};

// ── 对数断言: 枚举实数 == 宏实数 ──
const _: () = assert!(TreeKind::${lastKind} as usize + 1 == NUM_TREE_KINDS);

/// 命名标准字符集 (原版 enum class StandardCharacterSet : char)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StandardCharacterSet {
${scsRows.map((r, i) => `  ${r.name} = ${i},`).join("\n")}
}

pub const NUM_STANDARD_CHARACTER_SETS: usize = ${scsRows.length};
const _: () = assert!(StandardCharacterSet::${lastScs} as usize + 1 == NUM_STANDARD_CHARACTER_SETS);

impl StandardCharacterSet {
  /// 原 char 值 (转义字母或简写符号)
  pub const CHAR: [char; NUM_STANDARD_CHARACTER_SETS] = [
${scsRows.map((r) => `    '${r.ch}',`).join("\n")}
  ];

  /// 按转义字符检索 ('\\u{5c}' 转义的目标字符)
  #[must_use]
  pub const fn from_escape(c: char) -> Option<Self> {
    match c {
${scsRows.map((r) => `      '${r.ch}' => Some(Self::${r.name}),`).join("\n")}
      _ => None,
    }
  }
}

/// ClassRanges 附加标志 (原版 ClassRanges::Flag)
pub type ClassRangesFlags = u8;
${crfRows.map((r) => `pub const ${r.name}: ClassRangesFlags = 1 << ${r.bit};`).join("\n")}

pub const NUM_CLASS_RANGES_FLAGS: usize = ${crfRows.length};

/// 断言类型 (原版 Assertion::Type, 序数 1:1; SCREAMING → PascalCase 承接)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AssertionType {
${atRows.map((r) => `  ${r.name} = ${r.val},`).join("\n")}
}

pub const NUM_ASSERTION_TYPES: usize = ${atRows.length};
const _: () = assert!(AssertionType::${lastAt} as usize + 1 == NUM_ASSERTION_TYPES);
`;

writeSrc("v8-regexp-syntax", "ast_generated.rs", body);
console.log("written: ast_generated.rs");
