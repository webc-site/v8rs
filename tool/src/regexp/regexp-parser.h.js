#!/usr/bin/env bun
// tool/src/regexp/regexp-parser.h.js
// 机械提取 v8/src/regexp/regexp-parser.h 的 Parser 接口骨架
// (AllStatic 类, 两入口签名; 常量在 regexp-parser.cc 侧已有生成物)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/regexp/regexp-parser.h");

if (!/class V8_EXPORT_PRIVATE Parser : public AllStatic/.test(hSrc)) {
  throw new Error("未找到 Parser : AllStatic");
}
const entries = [...hSrc.matchAll(/static (?:bool) (ParseRegExpFromHeapString|VerifyRegExpSyntax)\(/g)].map((m) => m[1]);
if (entries.join(",") !== "ParseRegExpFromHeapString,VerifyRegExpSyntax") {
  throw new Error(`Parser 入口异常: ${entries}`);
}

console.log("regexp-parser.h: Parser skeleton (2 entries)");

let rust = banner(
  "src/regexp/regexp-parser.h.js",
  "src/regexp/regexp-parser.h",
  "正则解析器入口骨架 (签名登记; 语义实现见 regexp-parser.cc 侧生成物)",
);

rust += `
/// 正则解析器 (对应原版 regexp::Parser, AllStatic — 无实例状态)
pub struct Parser;

impl Parser {
  /// 原版 Parser::ParseRegExpFromHeapString(Isolate*, Zone*, String, Flags, CompileData*)
  /// 解析堆上字符串为 CompileData; 失败返回 false
  pub fn parse_regexp_from_heap_string(
    _input: &str,
    _flags: crate::flags_generated::Flags,
    _result: &mut crate::parser_generated::CompileData,
  ) -> bool {
    unimplemented!("登记骨架: 实现见 v8/src/regexp/regexp-parser.cc")
  }

  /// 原版 Parser::VerifyRegExpSyntax<CharT>(Zone*, stack_limit, input, len, Flags, CompileData*, no_gc)
  /// 仅校验语法, 不产出数据
  pub fn verify_regexp_syntax(
    _stack_limit: usize,
    _input: &str,
    _flags: crate::flags_generated::Flags,
    _result: &mut crate::parser_generated::CompileData,
  ) -> bool {
    unimplemented!("登记骨架: 实现见 v8/src/regexp/regexp-parser.cc")
  }
}
`;

const p = writeSrc("v8-regexp-syntax", "parser_h_skeleton.rs", rust);
console.log(`written ${p}`);
