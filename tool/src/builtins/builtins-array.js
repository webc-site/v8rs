#!/usr/bin/env bun
// tool/src/builtins/builtins-array.js — v8-builtins-array 基线生成
// 来源: builtins-array.cc/gen/h + array*.tq(31) + growable-fixed-array.tq/gen.cc/h
// → v8-builtins-array/src/{array, growable}.rs
// (重建版: 原文件被同名族脚本误覆盖, 依审计 files/anchors 清单与产物 mod 结构还原)
import { groupRun } from "./builtins-lib.js";
groupRun("builtins-array.js", "v8-builtins-array", [
  { mod: "array", files: { cc: ["builtins-array.cc", "builtins-array-gen.cc"], h: ["builtins-array-gen.h"], tq: ["array.tq", "array-at.tq", "array-concat.tq", "array-copywithin.tq", "array-every.tq", "array-filter.tq", "array-find.tq", "array-findindex.tq", "array-findlast.tq", "array-findlastindex.tq", "array-flat.tq", "array-foreach.tq", "array-from.tq", "array-isarray.tq", "array-join.tq", "array-lastindexof.tq", "array-map.tq", "array-of.tq", "array-reduce.tq", "array-reduce-right.tq", "array-reverse.tq", "array-shift.tq", "array-slice.tq", "array-some.tq", "array-splice.tq", "array-to-reversed.tq", "array-to-sorted.tq", "array-to-spliced.tq", "array-unshift.tq", "array-with.tq"] }, anchors: [["BUILTIN(ArrayPrototypeFill)", "builtins-array.cc"], ["TF_BUILTIN(ArrayPrototypePop, CodeStubAssembler)", "builtins-array-gen.cc"]] },
  { mod: "growable", files: { cc: ["growable-fixed-array-gen.cc"], h: ["growable-fixed-array-gen.h"], tq: ["growable-fixed-array.tq"] }, anchors: [["GrowableFixedArray", "growable-fixed-array-gen.cc"]] },
]);
