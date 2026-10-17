#!/usr/bin/env bun
// tool/src/ic/stub-cache.js — StubCache / ICStats / CallOptimization 面校验
// 覆盖: src/ic/stub-cache.h + stub-cache.cc + ic-stats.h + ic-stats.cc +
//       call-optimization.h + call-optimization.cc
import { readV8 } from "../../lib.js";

const FILES = [
  "src/ic/stub-cache.h",
  "src/ic/stub-cache.cc",
  "src/ic/ic-stats.h",
  "src/ic/ic-stats.cc",
  "src/ic/call-optimization.h",
  "src/ic/call-optimization.cc",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => {
  throw new Error(`[stub-cache.js] ${m}`);
};

for (const [f, needle] of [
  ["src/ic/stub-cache.h", "class V8_EXPORT_PRIVATE StubCache {"],
  ["src/ic/stub-cache.h", "class SCTableReference {"],
  ["src/ic/ic-stats.h", "struct ICInfo {"],
  ["src/ic/ic-stats.h", "class ICStats {"],
  ["src/ic/call-optimization.h", "class CallOptimization {"],
]) {
  if (!txt[f].includes(needle)) die(`${f} 缺针 ${needle}`);
}

console.log(`OK stub-cache.js 针核验通过 (类面针 5 全过)`);
