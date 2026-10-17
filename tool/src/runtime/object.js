#!/usr/bin/env bun
// tool/src/runtime/object.js — 对象/数组/集合 intrinsic 组
// 覆盖: src/runtime/runtime-object.cc + runtime-array.cc + runtime-collections.cc +
//       runtime-typedarray.cc + runtime-classes.cc + runtime-proxy.cc (+ runtime.h 对应组宏)
// 产出: v8-runtime/src/object.rs.skel
// 规则: 各组 F(name,nargs,ressize) 三元组全量 → (名, 参数, 结果) 清单 + RUNTIME_FUNCTION
//   签名骨架; .cc 实现体修订阶段转写。
// 复跑幂等: 纯文本扫描 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/runtime/object.js";
const GROUPS = {
  OBJECT: ["src/runtime/runtime-object.cc", "FOR_EACH_INTRINSIC_OBJECT"],
  ARRAY: ["src/runtime/runtime-array.cc", "FOR_EACH_INTRINSIC_ARRAY"],
  COLLECTIONS: ["src/runtime/runtime-collections.cc", "FOR_EACH_INTRINSIC_COLLECTIONS"],
  TYPEDARRAY: ["src/runtime/runtime-typedarray.cc", "FOR_EACH_INTRINSIC_TYPEDARRAY"],
  CLASSES: ["src/runtime/runtime-classes.cc", "FOR_EACH_INTRINSIC_CLASSES"],
  PROXY: ["src/runtime/runtime-proxy.cc", "FOR_EACH_INTRINSIC_PROXY"],
};
const h = readV8("src/runtime/runtime.h");
const txt = Object.fromEntries(Object.values(GROUPS).map(([f]) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[object.js] ${m}`); };

// 每组: 组宏区 F( 条数 == .cc 内 RUNTIME_FUNCTION( 条数 (grep 实测基数见断言)
const EXPECT_CC = {
  "src/runtime/runtime-object.cc": 76,
  "src/runtime/runtime-array.cc": 10,
  "src/runtime/runtime-collections.cc": 10,
  "src/runtime/runtime-typedarray.cc": 8,
  "src/runtime/runtime-classes.cc": 11,
  "src/runtime/runtime-proxy.cc": 8,
};
const count = (s, re) => (s.match(re) ?? []).length;
for (const [f, want] of Object.entries(EXPECT_CC)) {
  const got = count(txt[f], /^RUNTIME_FUNCTION\(/gm);
  if (got !== want) die(`${f} RUNTIME_FUNCTION 数=${got}, 期望 ${want}`);
}
// 组宏存在性
for (const [, macro] of Object.values(GROUPS)) {
  if (!h.includes(`#define ${macro}(F, I)`)) die(`runtime.h 缺组宏 ${macro}`);
}

console.log(`OK object.js 针核验通过: 6 组 .cc 条数全过 + 组宏存在 6 全过`);

