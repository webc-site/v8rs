#!/usr/bin/env bun
// tool/src/objects/heap-number.h.js — 覆盖 v8/src/objects/heap-number.{h,tq}
// → v8-objects-rest/src/heap_number.rs
// 机械提取: tq 布局字段表 + kSize 锚 (core 已有手写 heap_number.rs, 落 rest 避冲突)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/heap-number.h");
const tqSrc = readV8("src/objects/heap-number.tq");

// tq 布局: HeapNumber 1 字段 (value: float64); UninitializedHeapNumber 同布局
function tqClass(name) {
  const m = tqSrc.match(new RegExp(`extern class ${name} extends (\\w+)\\s*\\{([^}]*)\\}`));
  if (!m) throw new Error(`未找到 tq class ${name}`);
  const fields = [...m[2].matchAll(/^\s{2}(\w+):\s*([^;]+);/gm)].map((x) => ({ name: x[1], type: x[2].trim() }));
  return { base: m[1], fields };
}

const hn = tqClass("HeapNumber");
if (hn.base !== "PrimitiveHeapObject") throw new Error(`HeapNumber 基类失配: ${hn.base}`);
if (hn.fields.length !== 1 || hn.fields[0].name !== "value" || hn.fields[0].type !== "float64") {
  throw new Error(`HeapNumber 布局失配: ${JSON.stringify(hn.fields)}`);
}
const un = tqClass("UninitializedHeapNumber");
if (un.base !== "HeapObject") throw new Error(`UninitializedHeapNumber 基类失配: ${un.base}`);
if (un.fields.length !== 1 || un.fields[0].type !== "float64") throw new Error("UninitializedHeapNumber 布局失配");

// h 侧: kSize = sizeof(HeapNumber) (UninitializedHeapNumber 复用)
const kSize = hSrc.match(/static constexpr int kSize = sizeof\(HeapNumber\);/);
if (!kSize) throw new Error("未找到 UninitializedHeapNumber::kSize 表达式");

// double 值大小 (机械: float64)
const DOUBLE_SIZE = 8;

let rust = banner("src/objects/heap-number.h.js", "src/objects/heap-number.{h,tq}", "HeapNumber/UninitializedHeapNumber 布局表");
rust += `//! HeapNumber 布局 (生成基线; 语义实现归 crate 已有 heap_number 模块)。

/// float64 载荷大小 (原版 value 字段)。
pub const VALUE_SIZE: usize = ${DOUBLE_SIZE};

/// HeapNumber Torque 布局字段表 (heap-number.tq, 顺序 1:1)。
pub const HEAP_NUMBER_FIELDS: [(&str, &str); ${hn.fields.length}] = [
  ("value", "float64"),
];

/// UninitializedHeapNumber 与 HeapNumber 布局等宽 (原版 kSize = sizeof(HeapNumber);
/// 用于可变 double 字段的未初始化占位, 便于就地转换)。
pub const UNINITIALIZED_HEAP_NUMBER_FIELDS: [(&str, &str); ${un.fields.length}] = [
  ("value", "float64"),
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(VALUE_SIZE == 8);
  assert!(HEAP_NUMBER_FIELDS.len() == 1);
  assert!(UNINITIALIZED_HEAP_NUMBER_FIELDS.len() == 1);
};
`;

const p = writeSrc("v8-objects-rest", "heap_number.rs", rust);
console.log(`heap_number.rs: 双类布局表 (各 1 字段), 对数通过`);
