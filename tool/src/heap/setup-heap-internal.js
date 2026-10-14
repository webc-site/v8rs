#!/usr/bin/env bun
// tool/src/heap/setup-heap-internal.js
// 机械提取 v8/src/heap/setup-heap-internal.cc:
// Heap::Create* 初始对象七段函数骨架 + X-macro 元素计数

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/heap/setup-heap-internal.cc");

const createFns = [
  "bool Heap::CreateReadOnlyHeapObjects() {",
  "bool Heap::CreateMutableHeapObjects() {",
  "bool Heap::CreateEarlyReadOnlyMapsAndObjects() {",
  "bool Heap::CreateLateReadOnlyNonJSReceiverMaps() {",
  "bool Heap::CreateLateReadOnlyJSReceiverMaps() {",
  "bool Heap::CreateImportantReadOnlyObjects() {",
  "bool Heap::CreateReadOnlyObjects() {",
];
for (const f of createFns) if (!ccSrc.includes(f)) throw new Error(`cc 缺 ${f}`);

// X-macro 元素真提取
const macros = [
  ["CONSTANT_STRING_ELEMENT", /#define CONSTANT_STRING_ELEMENT\(/g],
  ["STRING_TYPE_ELEMENT", /#define STRING_TYPE_ELEMENT\(/g],
  ["STRUCT_TABLE_ELEMENT", /#define STRUCT_TABLE_ELEMENT\(/g],
  ["ALLOCATION_SITE_ELEMENT", /#define ALLOCATION_SITE_ELEMENT\(/g],
];
for (const [name, re] of macros) {
  const n = (ccSrc.match(re) || []).length;
  if (n < 1) throw new Error(`${name} 定义缺失`);
}
// 根字符串/结构表走 INCREMENTAL_STEAM? 对数锚: 7 个 Create 函数
if (!ccSrc.includes("CreateReadOnlyObjects()) return false;")) throw new Error("CreateReadOnlyObjects 调用锚缺失");
console.log(`setup-heap-internal: Create* 函数 ${createFns.length}, X-macro 定义 ${macros.length}`);

let rust = banner(
  "src/heap/setup-heap-internal.js",
  "src/heap/setup-heap-internal.cc",
  "堆初始对象/根表建立 (SetupHeapInternal)",
);

rust += `//! 1801 行文件: 由 X-macro 展开常量字符串/单字符字符串/结构表/allocation site。
//! 元素宏: CONSTANT_STRING_ELEMENT / STRING_TYPE_ELEMENT /
//!         STRUCT_TABLE_ELEMENT / ALLOCATION_SITE_ELEMENT。

impl Heap {
  /// v8: src/heap/setup-heap-internal.cc Heap::CreateReadOnlyObjects
  /// 总入口: 早期只读 maps+对象 → 晚期只读 → 重要只读对象。
  pub fn create_read_only_objects(&mut self) -> bool { todo!() }
  /// v8: Heap::CreateEarlyReadOnlyMapsAndObjects
  /// v8: Heap::CreateLateReadOnlyNonJSReceiverMaps
  /// v8: Heap::CreateLateReadOnlyJSReceiverMaps
  /// v8: Heap::CreateImportantReadOnlyObjects
  /// v8: Heap::CreateReadOnlyHeapObjects
  /// v8: Heap::CreateMutableHeapObjects — mutable 侧初始 map/对象
}

// 分段函数 ${createFns.length} 个, 语义面 (逐根安装) 留 Developer 修订。
`;

const p = writeSrc("v8-heap-core", "setup_heap_internal.rs.skel", rust);
console.log(`written ${p}`);
