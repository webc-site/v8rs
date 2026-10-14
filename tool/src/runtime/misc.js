#!/usr/bin/env bun
// tool/src/runtime/misc.js — 杂项 intrinsic 组 (作用域/模块/调试/编译器/内部/测试/wasm 等)
// 覆盖: src/runtime/runtime-scopes.cc + runtime-module.cc + runtime-debug.cc +
//   runtime-compiler.cc + runtime-internal.cc + runtime-test.cc + runtime-test-wasm.cc +
//   runtime-wasm.cc + runtime-function.cc + runtime-generator.cc + runtime-promise.cc +
//   runtime-regexp.cc + runtime-symbol.cc + runtime-literals.cc + runtime-forin.cc +
//   runtime-futex.cc + runtime-intl.cc + runtime-shadow-realm.cc + runtime-trace.cc +
//   runtime-weak-refs.cc (+ runtime.h 组宏)
// 产出: v8-runtime/src/misc.rs.skel
// 规则: 同 object.js; runtime-test*.cc 仅测试桩 (158+39 项), 修订阶段隔离为 #[cfg(test)]。
// 复跑幂等: 纯文本扫描 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/runtime/misc.js";
const FILES = [
  "src/runtime/runtime-scopes.cc",
  "src/runtime/runtime-module.cc",
  "src/runtime/runtime-debug.cc",
  "src/runtime/runtime-compiler.cc",
  "src/runtime/runtime-internal.cc",
  "src/runtime/runtime-test.cc",
  "src/runtime/runtime-test-wasm.cc",
  "src/runtime/runtime-wasm.cc",
  "src/runtime/runtime-function.cc",
  "src/runtime/runtime-generator.cc",
  "src/runtime/runtime-promise.cc",
  "src/runtime/runtime-regexp.cc",
  "src/runtime/runtime-symbol.cc",
  "src/runtime/runtime-literals.cc",
  "src/runtime/runtime-forin.cc",
  "src/runtime/runtime-futex.cc",
  "src/runtime/runtime-intl.cc",
  "src/runtime/runtime-shadow-realm.cc",
  "src/runtime/runtime-trace.cc",
  "src/runtime/runtime-weak-refs.cc",
];
const h = readV8("src/runtime/runtime.h");
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[misc.js] ${m}`); };

const EXPECT_CC = {
  "src/runtime/runtime-scopes.cc": 30,
  "src/runtime/runtime-module.cc": 4,
  "src/runtime/runtime-debug.cc": 25,
  "src/runtime/runtime-compiler.cc": 25,
  "src/runtime/runtime-internal.cc": 65,
  "src/runtime/runtime-function.cc": 6,
  "src/runtime/runtime-generator.cc": 13,
  "src/runtime/runtime-promise.cc": 13,
  "src/runtime/runtime-regexp.cc": 12,
  "src/runtime/runtime-symbol.cc": 5,
  "src/runtime/runtime-literals.cc": 4,
  "src/runtime/runtime-forin.cc": 2,
  "src/runtime/runtime-futex.cc": 3,
  "src/runtime/runtime-intl.cc": 5,
  "src/runtime/runtime-shadow-realm.cc": 3,
  "src/runtime/runtime-trace.cc": 4,
  "src/runtime/runtime-weak-refs.cc": 2,
  "src/runtime/runtime-wasm.cc": 73,
};
const count = (s, re) => (s.match(re) ?? []).length;
for (const [f, want] of Object.entries(EXPECT_CC)) {
  // RUNTIME_FUNCTION_RETURN_PAIR 另计 (如 scopes 的 LoadLookupSlotForCall): 总出现 == 期望
  const got = count(txt[f], /RUNTIME_FUNCTION(?:_RETURN_PAIR)?\(/g);
  if (got !== want) die(`${f} RUNTIME_FUNCTION(含PAIR) 数=${got}, 期望 ${want}`);
}
// 测试桩两文件: 直接 RUNTIME_FUNCTION( 出现 (行首或宏内定义行均含; 调用形 ELEMENTS_KIND_CHECK_* 另计)
// runtime-test.cc = 155 直接 + 1 宏定义行内 + 8 展开调用 = grep -c 158 (基数已实测)
const testHits = count(txt["src/runtime/runtime-test.cc"], /RUNTIME_FUNCTION\(/g);
if (testHits !== 155) die(`runtime-test.cc 直接出现数=${testHits}, 期望 155 (另 +1 宏定义 +8 宏展开调用)`);
const testWasmHits = count(txt["src/runtime/runtime-test-wasm.cc"], /RUNTIME_FUNCTION\(/g);
if (testWasmHits !== 39) die(`runtime-test-wasm.cc 出现数=${testWasmHits}, 期望 39`);

console.log(`OK misc.js 针核验通过: ${FILES.length} 个原版文件计数核验全过`);

