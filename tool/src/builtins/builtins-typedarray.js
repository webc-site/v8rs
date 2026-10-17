#!/usr/bin/env bun
// tool/src/builtins/builtins-typedarray.js — v8-builtins-typedarray 基线生成
// 来源: typed-array*.tq(22) + arraybuffer/dataview/shared 系列 .cc/.h/.tq
// → v8-builtins-typedarray/src/{typed_array, buffer}.rs
import { groupRun } from "./builtins-lib.js";
groupRun("builtins-typedarray.js", "v8-builtins-typedarray", [
  { mod: "typed_array", files: { cc: ["builtins-typed-array.cc", "builtins-typed-array-gen.cc"], h: ["builtins-typed-array-gen.h"], tq: ["typed-array.tq", "typed-array-at.tq", "typed-array-createtypedarray.tq", "typed-array-entries.tq", "typed-array-every.tq", "typed-array-filter.tq", "typed-array-find.tq", "typed-array-findindex.tq", "typed-array-findlast.tq", "typed-array-findlastindex.tq", "typed-array-foreach.tq", "typed-array-from.tq", "typed-array-keys.tq", "typed-array-of.tq", "typed-array-reduce.tq", "typed-array-reduceright.tq", "typed-array-set.tq", "typed-array-slice.tq", "typed-array-some.tq", "typed-array-sort.tq", "typed-array-subarray.tq", "typed-array-to-reversed.tq", "typed-array-to-sorted.tq", "typed-array-values.tq", "typed-array-with.tq"] }, anchors: [["TypedArray", "builtins-typed-array.cc"]] },
  { mod: "buffer", files: { cc: ["builtins-arraybuffer.cc", "builtins-dataview.cc", "builtins-sharedarraybuffer.cc", "builtins-sharedarraybuffer-gen.cc", "builtins-shared-array.cc"], h: ["builtins-data-view-gen.h", "data-view-ops.h"], tq: ["arraybuffer.tq", "data-view.tq"] }, anchors: [["ArrayBuffer", "builtins-arraybuffer.cc"]] },
]);
