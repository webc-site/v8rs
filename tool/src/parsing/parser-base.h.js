#!/usr/bin/env bun
// tool/src/parsing/parser-base.h.js
// 机械提取 v8/src/parsing/parser-base.h — 顶层解析枚举 + 模板类面骨架。
// 7196 行模板方法为语义面, 登记跳过 (规则 10)。
// 注: parser_generated.rs (parser.cc.js 产物) 已存在, 本脚本独立落盘。
// 产物: v8-parsing/src/parser_base_generated.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/parsing/parser-base.h";
const h = readV8(H_REL);

// 顶层枚举
function parseAuto(body) {
  const clean = body.replace(/\/\/[^\n]*/g, "");
  const out = [];
  let next = 0;
  for (const raw of clean.split(",")) {
    const m = raw.trim().match(/^(\w+)\s*(?:=\s*([^,\n]+))?$/);
    if (!m) continue;
    let value;
    if (m[2] === undefined) value = next;
    else if (/^0[xX][0-9a-fA-F]+$|^\d+$/.test(m[2].trim())) value = Number(m[2].trim());
    else if (/^\d+\s*<<\s*\d+$/.test(m[2].trim())) {
      const p2 = m[2].split("<<");
      value = Number(p2[0].trim()) << Number(p2[1].trim());
    } else throw new Error(`不支持的表达式: ${raw}`);
    out.push({ name: m[1], value });
    next = value + 1;
  }
  return out;
}
function needEnum(name, re, expect) {
  const m = h.match(re);
  if (!m) throw new Error(`未找到 ${name}`);
  const items = parseAuto(m[1]);
  if (items.length !== expect) throw new Error(`${name} 条数不符: ${items.length} != ${expect}`);
  return items;
}

const fnv = needEnum("FunctionNameValidity", /enum FunctionNameValidity\s*\{([^}]+)\};/, 3);
const alf = needEnum("AllowLabelledFunctionStatement", /enum AllowLabelledFunctionStatement\s*\{([^}]+)\};/, 2);
const arrow = needEnum("ParsingArrowHeadFlag", /enum ParsingArrowHeadFlag\s*\{([^}]+)\};/, 2);
const pff = needEnum("ParseFunctionFlag", /enum class ParseFunctionFlag : uint8_t\s*\{([^}]+)\};/, 3);
if (pff[1].value !== 1 || pff[2].value !== 2) throw new Error("ParseFunctionFlag 位锚不符");
const ppk = needEnum("ParsePropertyKind", /enum class ParsePropertyKind : uint8_t\s*\{([^}]+)\};/, 11);
const vdc = needEnum("VariableDeclarationContext", /enum VariableDeclarationContext\s*\{([^}]+)\};/, 3);
const tt = needEnum("TargetType", /enum TargetType\s*\{([^}]+)\};/, 2);

// 模板类面方法数
const methods = [...h.matchAll(/^\s+(?:V8_WARN_UNUSED_RESULT\s+)?(?:inline\s+)?[\w:<>,&*\s]+?\b(Parse\w+)\(/gm)].map((m) => m[1]);
if (methods.length < 20) throw new Error(`Parse 方法面异常: ${methods.length}`);

let rust = banner("src/parsing/parser-base.h.js", H_REL, "解析器基类枚举与骨架");
rust += `
// 语义面: ParserBase<Impl> 模板方法 (${methods.length}+ 处 Parse*) 登记跳过,
// 由 parser_generated.rs (parser.cc 侧) 与 Developer 修订补齐。

/// 函数名合法性检查模式 (原 enum FunctionNameValidity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FunctionNameValidity {
${fnv.map((e, i) => `  /// ${e.name}\n  ${e.name.replace(/^k/, "")} = ${i},`).join("\n")}
}

/// 带标号函数语句许可 (原 enum AllowLabelledFunctionStatement)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AllowLabelledFunctionStatement {
${alf.map((e, i) => `  /// ${e.name}\n  ${e.name.replace(/^k/, "")} = ${i},`).join("\n")}
}

/// 箭头函数头判定 (原 enum ParsingArrowHeadFlag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ParsingArrowHeadFlag {
${arrow.map((e, i) => `  /// ${e.name}\n  ${e.name.replace(/^k/, "")} = ${i},`).join("\n")}
}

/// 解析函数标志位集 (原 enum class ParseFunctionFlag : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ParseFunctionFlag {
  IsNormal = 0,
  IsGenerator = 1 << 0,
  IsAsync = 1 << 1,
}

/// 位集别名 (原 base::Flags<ParseFunctionFlag>)
pub type ParseFunctionFlags = u8;

/// 属性解析类别 (原 enum class ParsePropertyKind : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ParsePropertyKind {
${ppk.map((e, i) => `  /// ${e.name}\n  ${e.name.replace(/^k/, "")} = ${i},`).join("\n")}
}

/// 变量声明上下文 (原 enum VariableDeclarationContext)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VariableDeclarationContext {
${vdc.map((e, i) => `  /// ${e.name}\n  ${e.name.replace(/^k/, "")} = ${i},`).join("\n")}
}

/// 循环目标类型 (原 enum TargetType)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TargetType {
${tt.map((e, i) => `  /// ${e.name}\n  ${e.name} = ${i},`).join("\n")}
}

/// 原 template <typename Impl> class ParserBase 骨架
pub struct ParserBase;

// —— 提取不变式断言 ——
const _: () = {
  assert!((ParseFunctionFlag::IsGenerator as u8) | (ParseFunctionFlag::IsAsync as u8) == 3);
};
`;

const p = writeSrc("v8-parsing", "parser_base_generated.rs", rust);
console.log(`written ${p} (枚举 7, Parse 方法 ${methods.length})`);
