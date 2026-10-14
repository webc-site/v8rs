#!/usr/bin/env bun
// tool/src/builtins/builtins-regexp.js — v8-builtins-regexp 基线生成
// 来源: builtins-regexp.cc/gen/h + regexp*.tq(9)
// → v8-builtins-regexp/src/regexp.rs
// (重建版: 原文件被同名族脚本误覆盖, 依审计 files/anchors 清单与产物 mod 结构还原)
import { groupRun } from "./builtins-lib.js";
groupRun("builtins-regexp.js", "v8-builtins-regexp", [
  { mod: "regexp", files: { cc: ["builtins-regexp.cc", "builtins-regexp-gen.cc"], h: ["builtins-regexp-gen.h"], tq: ["regexp.tq", "regexp-exec.tq", "regexp-match.tq", "regexp-match-all.tq", "regexp-replace.tq", "regexp-search.tq", "regexp-source.tq", "regexp-split.tq", "regexp-test.tq"] }, anchors: [["RegExp", "builtins-regexp.cc"]] },
]);
