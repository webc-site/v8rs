#!/usr/bin/env bun
// tool/src/builtins/builtins-collections.js — v8-builtins-collections 基线生成
// 来源: collections.tq + set-*.tq(7) + groupby/fromentries + builtins-collections*
// → v8-builtins-collections/src/{collections, setops}.rs
// (重建版: 原文件被同名族脚本误覆盖, 依审计 files/anchors 清单与产物 mod 结构还原)
import { groupRun } from "./builtins-lib.js";
groupRun("builtins-collections.js", "v8-builtins-collections", [
  { mod: "collections", files: { cc: ["builtins-collections.cc", "builtins-collections-gen.cc"], h: ["builtins-collections-gen.h"], tq: ["collections.tq", "map-groupby.tq", "object-groupby.tq", "object-fromentries.tq"] }, anchors: [["BUILTIN(MapPrototypeClear)", "builtins-collections.cc"]] },
  { mod: "setops", files: { cc: [], h: [], tq: ["set-difference.tq", "set-intersection.tq", "set-is-disjoint-from.tq", "set-is-subset-of.tq", "set-is-superset-of.tq", "set-symmetric-difference.tq", "set-union.tq"] }, anchors: [["Set", "set-union.tq"]] },
]);
