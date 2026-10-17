#!/usr/bin/env bun
// tool/src/parsing/parser.cc.js
// 机械提取 v8/src/parsing/parser.{h,cc} + expression-scope.h: 解析器面 (>2k 行只取骨架)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/parsing/parser.h");
const ccSrc = readV8("src/parsing/parser.cc");
const esH = readV8("src/parsing/expression-scope.h");

function collectDefs(src, prefix) {
  const out = [];
  const ls = src.split("\n");
  for (let i = 0; i < ls.length; i++) {
    const l = ls[i];
    if (!new RegExp(`^[\\w:<>,*&\\s]*\\b${prefix}::~?\\w+\\(`).test(l) || /^\s*\/\//.test(l)) continue;
    let sig = l;
    let j = i;
    while (!/\{\}?\s*$|;\s*$/.test(sig) && j + 1 < ls.length) sig += " " + ls[++j].trim();
    sig = sig.replace(/\{\s*$/, "").replace(/\{\}\s*$/, "").trim();
    if (sig.endsWith(";")) continue;
    out.push(sig);
  }
  return out;
}

const ccDefs = collectDefs(ccSrc, "Parser");
if (ccDefs.length < 40) throw new Error(`parser.cc Parser 定义头应 ≥40, 实得 ${ccDefs.length}`);
const hFns = [...hSrc.matchAll(/\n  (?:static )?(?:void|bool|int|DirectHandle|Expression|Statement)[\w:<>,*& ]*?(\w+)\(/g)].map((m) => m[1]);
if (hFns.length < 30) throw new Error(`parser.h 方法应 ≥30, 实得 ${hFns.length}`);

// expression-scope: 变量初始化追踪协议
if (!esH.includes("VariableTrackingParticipants") && !/class \w+ExpressionScope|ExpressionScope/.test(esH)) {
  throw new Error("expression-scope.h 面漂移");
}
const esClasses = [...esH.matchAll(/\nclass (\w+) : public \w*ExpressionScope/g)].map((m) => m[1]);

let rust = banner(
  "src/parsing/parser.cc.js",
  "src/parsing/parser.{h,cc} + expression-scope.h",
  "解析器骨架",
);

rust += `
/// Parser 方法面 (原版 parser.h, ${hFns.length} 项, 顺序登记)
pub const PARSER_METHODS: [&str; ${hFns.length}] = [${hFns.map((f) => `"${f}"`).join(", ")}];

/// expression-scope 表达式作用域族 (原版 expression-scope.h, ${esClasses.length + 1} 类:
/// 基座 ExpressionScope + 特化; 负责 let/箭头函数二义与 await 判定)
pub const EXPRESSION_SCOPE_CLASSES: [&str; ${Math.max(esClasses.length, 1)}] = [${esClasses.map((c) => `"${c}"`).join(", ") || '"ExpressionScope"'}];

// parser.cc Parser 定义头清单 (对数基准, ${ccDefs.length} 项):
${ccDefs.map((s) => "// " + s).join("\n")}
`;

const p = writeSrc("v8-parsing", "parser_generated.rs", rust);
console.log(`written ${p}; 头方法=${hFns.length}, cc定义=${ccDefs.length}, 表达式作用域=${esClasses.length}`);
