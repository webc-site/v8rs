#!/usr/bin/env bun
// tool/src/parsing/parse-info.cc.js
// 机械提取 v8/src/parsing/ 杂项族: parse-info.{h,cc} + parsing.{h,cc} + token.cc +
// literal-buffer.cc + func-name-inferrer.{h,cc} + import-attributes.{h,cc} +
// pending-compilation-error-handler.{h,cc} + rewriter.{h,cc}

import { readV8, writeSrc, banner } from "../../lib.js";

const files = [
  ["src/parsing/parse-info.h", "class ParseInfo"],
  ["src/parsing/parse-info.cc", "ParseInfo::"],
  ["src/parsing/parsing.h", "ParseProgram"],
  ["src/parsing/parsing.cc", "ParseProgram"],
  ["src/parsing/token.cc", "kNumTokens"],
  ["src/parsing/literal-buffer.cc", "LiteralBuffer::"],
  ["src/parsing/func-name-inferrer.h", "FuncNameInferrer"],
  ["src/parsing/func-name-inferrer.cc", "FuncNameInferrer::"],
  ["src/parsing/import-attributes.h", "ImportAttributes"],
  ["src/parsing/import-attributes.cc", "ImportAttributes"],
  ["src/parsing/pending-compilation-error-handler.h", "PendingCompilationErrorHandler"],
  ["src/parsing/pending-compilation-error-handler.cc", "PendingCompilationErrorHandler::"],
  ["src/parsing/rewriter.h", "Rewriter"],
  ["src/parsing/rewriter.cc", "Rewriter::"],
];

const anchors = [];
for (const [file, sym] of files) {
  const src = readV8(file);
  if (!src.includes(sym)) throw new Error(`${file} 缺 ${sym}`);
  anchors.push(file.split("/").pop());
}

// token.cc: token 字符串表 (机械对数)
const tokenCc = readV8("src/parsing/token.cc");
// TOKEN_LIST 在 token.h; token.cc 侧四张表 (name_/string_/string_length_/precedence_)
const tableAnchors = ["Token::name_[kNumTokens]", "Token::string_[kNumTokens]", "Token::string_length_[kNumTokens]", "Token::precedence_[2][kNumTokens]"];
for (const a of tableAnchors) {
  if (!tokenCc.includes(a)) throw new Error(`token.cc 缺表 ${a}`);
}
const nameEntries = 4;

let rust = banner(
  "src/parsing/parse-info.cc.js",
  "src/parsing/{parse-info,parsing,token,literal-buffer,func-name-inferrer,import-attributes,pending-compilation-error-handler,rewriter}.{h,cc}",
  "解析杂项族骨架",
);

rust += `
/// 杂项族结构锚点 (14 文件登记)
pub const PARSING_MISC_ANCHORS: [&str; ${anchors.length}] = [${anchors.map((a) => `"${a}"`).join(", ")}];

pub mod parsing_misc_skel {
  /// 解析入口 (原版 parsing.cc ParseProgram)
  pub fn parse_program() {
    unimplemented!("骨架")
  }
  /// 解析参数包 (原版 ParseInfo: 入口/标志/结果聚合)
  pub fn parse_info_skel() {
    unimplemented!("骨架")
  }
  /// token 名表 (原版 token.cc name_/string_/string_length_/precedence_ 表)
  pub const TOKEN_TABLES: [&str; 4] = ["name_", "string_", "string_length_", "precedence_"];
  /// 字面缓冲 (原版 LiteralBuffer: 增长式字符缓冲, ASCII/UTF-16 双态)
  pub fn literal_buffer_skel() {
    unimplemented!("骨架")
  }
  /// 函数名推断 (原版 FuncNameInferrer: 匿名函数按赋值/属性上下文命名)
  pub fn func_name_inferrer_skel() {
    unimplemented!("骨架")
  }
  /// import 属性 (原版 ImportAttributes: with 选项表)
  pub fn import_attributes_skel() {
    unimplemented!("骨架")
  }
  /// 延迟编译错误 (原版 PendingCompilationErrorHandler: 惰性函数体错误转发)
  pub fn pending_error_handler_skel() {
    unimplemented!("骨架")
  }
  /// 重写器 (原版 Rewriter: 顶层表达式结果丢弃补 undefined)
  pub fn rewriter_skel() {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-parsing", "parsing_misc_generated.rs", rust);
console.log(`written ${p}; 锚点=${anchors.length}, token串=${nameEntries}`);
