#!/usr/bin/env bun
// tool/src/builtins/builtins-string.js — v8-builtins-string 基线生成
// 来源: builtins-string.cc/gen + builtins-string.tq + string-*.tq(17) + string-tsa
// → v8-builtins-string/src/{string, string_tsa}.rs
// (重建版: 原文件被同名族脚本误覆盖, 依审计 files/anchors 清单与产物 mod 结构还原)
import { groupRun } from "./builtins-lib.js";
groupRun("builtins-string.js", "v8-builtins-string", [
  { mod: "string", files: { cc: ["builtins-string.cc", "builtins-string-gen.cc"], h: ["builtins-string-gen.h"], tq: ["builtins-string.tq", "string-at.tq", "string-endswith.tq", "string-html.tq", "string-includes.tq", "string-indexof.tq", "string-iswellformed.tq", "string-iterator.tq", "string-match-search.tq", "string-pad.tq", "string-repeat.tq", "string-replaceall.tq", "string-slice.tq", "string-startswith.tq", "string-substr.tq", "string-substring.tq", "string-towellformed.tq", "string-trim.tq"] }, anchors: [["BUILTIN(StringFromCodePoint)", "builtins-string.cc"]] },
  { mod: "string_tsa", files: { cc: ["builtins-string-tsa.cc"], h: ["builtins-string-tsa-inl.h"], tq: [] }, anchors: [["String", "builtins-string-tsa.cc"]] },
]);
