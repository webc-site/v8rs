#!/usr/bin/env bun
// tool/src/regexp/regexp-parser.cc.js — v8/src/regexp/regexp-parser.cc → v8-regexp-syntax/src/parser_generated.rs
// 机械提取三组语法字符集 (IsSyntaxCharacterOrSlash / IsClassSetSyntaxCharacter /
// IsClassSetReservedPunctuator) 与双字符操作符集 (IsClassSetReservedDoublePunctuator),
// 对数断言; 状态机语义在 parser.rs
import { banner, readV8, writeSrc } from "../../lib.js";

const src = readV8("src/regexp/regexp-parser.cc");

// 提取指定函数体中 switch 的 case 'x': 字符 (排除 default)
function caseChars(fn) {
  const re = new RegExp(`bool ParserImpl<CharT, mode>::${fn}\\(base::uc32 c\\) \\{([\\s\\S]*?)\\n\\}`);
  const m = src.match(re);
  if (!m) throw new Error(`regexp-parser.cc: ${fn} 结构漂移`);
  return [...m[1].matchAll(/case '(.)':/g)].map((r) => r[1]);
}

function doubleChars() {
  const m = src.match(/IsClassSetReservedDoublePunctuator\(base::uc32 c\) \{([\s\S]*?)\n\}/);
  if (!m) throw new Error("regexp-parser.cc: IsClassSetReservedDoublePunctuator 结构漂移");
  return [...m[1].matchAll(/DOUBLE_PUNCTUATOR_CASE\('(.)'\)/g)].map((r) => r[1]);
}

const syntax = caseChars("IsSyntaxCharacterOrSlash");
const classSyntax = caseChars("IsClassSetSyntaxCharacter");
const reserved = caseChars("IsClassSetReservedPunctuator");
const doubles = doubleChars();

const uniq = (a) => [...new Set(a)].sort();
const [u1, u2, u3, u4] = [syntax, classSyntax, reserved, doubles].map(uniq);
if (u1.length !== syntax.length || u2.length !== classSyntax.length || u3.length !== reserved.length || u4.length !== doubles.length) {
  throw new Error("语法字符集存在重复 case");
}
console.log(`regexp-parser: 语法字符 ${syntax.length}, v 模式语法字符 ${classSyntax.length}, 保留标点 ${reserved.length}, 双字符操作符 ${doubles.length} ✓`);

const arr = (name, chars) =>
  `/// ${name} (${chars.length} 个, 升序)\npub static ${name.toUpperCase()}: [char; ${chars.length}] = [${chars.map((c) => `'${c}'`).join(", ")}];`;

const body = `${banner("src/regexp/regexp-parser.cc.js", "src/regexp/regexp-parser.cc", "语法字符集提取; 解析状态机在 parser.rs")}

${arr("SYNTAX_CHARS", u1)}

${arr("CLASS_SET_SYNTAX_CHARS", u2)}

${arr("CLASS_SET_RESERVED_PUNCTUATORS", u3)}

${arr("CLASS_SET_DOUBLE_PUNCTUATORS", u4)}

// ── 对数断言: 集合实数 == 原版 case 数 ──
const _: () = assert!(SYNTAX_CHARS.len() == ${u1.length});
const _: () = assert!(CLASS_SET_SYNTAX_CHARS.len() == ${u2.length});
const _: () = assert!(CLASS_SET_RESERVED_PUNCTUATORS.len() == ${u3.length});
const _: () = assert!(CLASS_SET_DOUBLE_PUNCTUATORS.len() == ${u4.length});
`;

writeSrc("v8-regexp-syntax", "parser_generated.rs", body);
console.log("written: parser_generated.rs");
