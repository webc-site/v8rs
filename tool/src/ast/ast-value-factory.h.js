#!/usr/bin/env bun
// tool/src/ast/ast-value-factory.h.js
// 机械提取 v8/src/ast/ast-value-factory.h 的 AST_STRING_CONSTANTS X-macro
// (单字符表来自 v8/src/init/heap-symbols.h 的生成器宏) 展开为 Rust 常量枚举

import { readV8, writeSrc, banner } from "../../lib.js";

const valueFactoryH = readV8("src/ast/ast-value-factory.h");
const heapSymbolsH = readV8("src/init/heap-symbols.h");

// 1. 提取单字符 ASCII 表 (SINGLE_CHARACTER_ASCII_INTERNALIZED_STRING_LIST_GENERATOR)
const genMatch = heapSymbolsH.match(
  /#define SINGLE_CHARACTER_ASCII_INTERNALIZED_STRING_LIST_GENERATOR\(V_, _\) \\\n([\s\S]*?)(?=\n#define |\n\n)/,
);
if (!genMatch) throw new Error("未找到 SINGLE_CHARACTER_ASCII_INTERNALIZED_STRING_LIST_GENERATOR");
const oneChars = [...genMatch[1].matchAll(/V_\(_, (\w+), "((?:[^"\\]|\\.)*)"\)/g)].map((m) => ({
  name: m[1],
  raw: m[2],
}));

// 2. 提取命名常量表 (AST_STRING_CONSTANTS 中的 F(name, "contents") 行, 跳过单字符宏引用)
const listMatch = valueFactoryH.match(/#define AST_STRING_CONSTANTS\(F\)([\s\S]*?)class AstStringConstants/);
if (!listMatch) throw new Error("未找到 AST_STRING_CONSTANTS");
const named = [...listMatch[1].matchAll(/F\((\w+), "((?:[^"\\]|\\.)*)"\)/g)].map((m) => ({
  name: m[1],
  raw: m[2],
}));

// 3. C 字符串字面量转码点序列
function decodeCLiteral(raw) {
  const chars = [];
  let i = 0;
  while (i < raw.length) {
    if (raw[i] === "\\") {
      const next = raw[i + 1];
      if (next === "x") {
        const hex = raw.slice(i + 2, i + 4);
        chars.push(parseInt(hex, 16));
        i += 4;
      } else {
        const map = { "0": 0, a: 7, b: 8, t: 9, n: 10, v: 11, f: 12, r: 13, "\\": 92, '"': 34, "'": 39 };
        if (!(next in map)) throw new Error(`未知转义 \\${next}`);
        chars.push(map[next]);
        i += 2;
      }
    } else {
      chars.push(raw.charCodeAt(i));
      i += 1;
    }
  }
  return chars;
}

// 4. Rust 字符串字面量编码 (码点序列 → 转义)
function encodeRustLiteral(chars) {
  let out = "";
  for (const c of chars) {
    if (c === 0x22) out += '\\"';
    else if (c === 0x5c) out += "\\\\";
    else if (c >= 0x20 && c < 0x7f) out += String.fromCharCode(c);
    else out += `\\u{${c.toString(16)}}`;
  }
  return `"${out}"`;
}

// 5. 变体命名: 去掉 _string 后缀, snake_case → PascalCase; 单字母区分大小写 (A→UpperA, a→LowerA)
function variantName(name) {
  const stem = name.replace(/_string$/, "");
  if (/^[a-z]$/.test(stem)) return `Lower${stem.toUpperCase()}`;
  if (/^[A-Z]$/.test(stem)) return `Upper${stem}`;
  return stem
    .split("_")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join("");
}

const oneCharEntries = oneChars.map((e) => ({
  variant: variantName(e.name),
  chars: decodeCLiteral(e.raw),
}));
const namedEntries = named.map((e) => ({
  variant: variantName(e.name),
  chars: decodeCLiteral(e.raw),
}));

// 完整性校验: 单字符表必须每项恰 1 码点且 0..127, 与 kMaxOneCharStringValue 推导一致
if (oneCharEntries.length !== 128) {
  throw new Error(`单字符表应 128 项, 实得 ${oneCharEntries.length}`);
}
for (const [i, e] of oneCharEntries.entries()) {
  if (e.chars.length !== 1 || e.chars[0] !== i) {
    throw new Error(`单字符表第 ${i} 项 ${e.variant} 内容异常: ${e.chars}`);
  }
}
const variants = new Set([...oneCharEntries, ...namedEntries].map((e) => e.variant));
if (variants.size !== oneCharEntries.length + namedEntries.length) {
  throw new Error("存在重名变体");
}

console.log(`提取字符串常量: 单字符=${oneCharEntries.length}, 命名=${namedEntries.length}, 合计=${oneCharEntries.length + namedEntries.length}`);

// 6. 生成 Rust 代码: value_generated.rs
const all = [...oneCharEntries, ...namedEntries];
const out = [
  banner("src/ast/ast-value-factory.h.js", "src/ast/ast-value-factory.h + src/init/heap-symbols.h", "AST 单例字符串常量表 (AST_STRING_CONSTANTS X-macro 展开)"),
  `use strum::{Display, FromRepr};`,
  ``,
  `/// AST 单例字符串常量 (原版 AST_STRING_CONSTANTS X-macro 展开)`,
  `///`,
  `/// 前 ${oneCharEntries.length} 项为单字符 ASCII 常量 (判别值 = 字节值), 后接 ${namedEntries.length} 项命名常量。`,
  `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]`,
  `#[repr(u8)]`,
  `pub enum AstStringConstant {`,
  ...all.map((e, idx) => `  ${e.variant} = ${idx},`),
  `}`,
  ``,
  `impl AstStringConstant {`,
  `  /// 常量总数`,
  `  pub const COUNT: usize = ${all.length};`,
  `  /// 单字符常量数 (原版 kMaxOneCharStringValue)`,
  `  pub const ONE_CHAR_COUNT: usize = ${oneCharEntries.length};`,
  ``,
  `  /// 常量内容`,
  `  pub const fn contents(&self) -> &'static str {`,
  `    match self {`,
  ...all.map((e) => `      Self::${e.variant} => ${encodeRustLiteral(e.chars)},`),
  `    }`,
  `  }`,
  ``,
  `  /// 按字节取单字符常量 (原版 AstStringConstants::one_character_string, 仅 0..ONE_CHAR_COUNT)`,
  `  pub fn one_char(c: u8) -> Option<Self> {`,
  `    if (c as usize) < Self::ONE_CHAR_COUNT {`,
  `      Self::from_repr(c)`,
  `    } else {`,
  `      None`,
  `    }`,
  `  }`,
  `}`,
  ``,
  `// ── 对数不变式断言 ──`,
  `const _: () = {`,
  `  assert!(AstStringConstant::COUNT == ${all.length});`,
  `  assert!(AstStringConstant::ONE_CHAR_COUNT == ${oneCharEntries.length});`,
  ...oneCharEntries.slice(0, 4).map((e, i) => `  assert!(AstStringConstant::${e.variant} as u8 == ${i});`),
  `  assert!(AstStringConstant::${oneCharEntries.at(-1).variant} as u8 == ${oneCharEntries.length - 1});`,
  `  assert!(AstStringConstant::${namedEntries[0].variant} as u8 == ${oneCharEntries.length});`,
  `  assert!(AstStringConstant::${namedEntries.at(-1).variant} as u8 == ${all.length - 1});`,
  `};`,
  ``,
];

const writtenPath = writeSrc("v8-ast", "value_generated.rs", out.join("\n"));
console.log(`生成成功: ${writtenPath}`);
