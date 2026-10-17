#!/usr/bin/env bun
// tool/src/runtime/numbers.js — 数值/字符串/日期/bigint/atomics intrinsic 组
// 覆盖: src/runtime/runtime-numbers.cc + runtime-strings.cc + runtime-bigint.cc +
//       runtime-atomics.cc + runtime-date.cc + runtime-operators.cc (+ runtime.h 组宏)
// 产出: v8-runtime/src/numbers.rs.skel
// 规则: 同 object.js — F 三元组清单 + 签名骨架; 数值语义 (ECMAScript 精确) 修订阶段对
//   unittests/numbers, 禁 zmij/itoa 替换格式层 (见 todo.md 10.3)。
// 复跑幂等: 纯文本扫描 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/runtime/numbers.js";
const GROUPS = {
  NUMBERS: ["src/runtime/runtime-numbers.cc", "FOR_EACH_INTRINSIC_NUMBERS"],
  STRINGS: ["src/runtime/runtime-strings.cc", "FOR_EACH_INTRINSIC_STRINGS"],
  BIGINT: ["src/runtime/runtime-bigint.cc", "FOR_EACH_INTRINSIC_BIGINT"],
  ATOMICS: ["src/runtime/runtime-atomics.cc", "FOR_EACH_INTRINSIC_ATOMICS"],
  DATE: ["src/runtime/runtime-date.cc", "FOR_EACH_INTRINSIC_DATE"],
  OPERATORS: ["src/runtime/runtime-operators.cc", "FOR_EACH_INTRINSIC_OPERATORS"],
};
const h = readV8("src/runtime/runtime.h");
const txt = Object.fromEntries(Object.values(GROUPS).map(([f]) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[numbers.js] ${m}`); };

const EXPECT_CC = {
  "src/runtime/runtime-numbers.cc": 7,
  "src/runtime/runtime-strings.cc": 23,
  "src/runtime/runtime-bigint.cc": 10,
  "src/runtime/runtime-atomics.cc": 22,
  "src/runtime/runtime-date.cc": 1,
  "src/runtime/runtime-operators.cc": 10,
};
const count = (s, re) => (s.match(re) ?? []).length;
for (const [f, want] of Object.entries(EXPECT_CC)) {
  const got = count(txt[f], /^RUNTIME_FUNCTION\(/gm);
  if (got !== want) die(`${f} RUNTIME_FUNCTION 数=${got}, 期望 ${want}`);
}
for (const [, macro] of Object.values(GROUPS)) {
  if (!h.includes(`#define ${macro}(F, I)`)) die(`runtime.h 缺组宏 ${macro}`);
}

console.log(`OK numbers.js 针核验通过: 6 组 .cc 条数全过 + 组宏存在 6 全过`);

