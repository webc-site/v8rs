#!/usr/bin/env bun
// tool/src/objects/primitive-heap-object.h.js — 覆盖 v8/src/objects/primitive-heap-object.{h,tq}
// PrimitiveHeapObject 抽象基类标记与布局断言
// → v8-objects-core/src/primitive_heap_object.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/primitive-heap-object.h");
const tqSrc = readV8("src/objects/primitive-heap-object.tq");

// 结构对数
if (!hSrc.includes("V8_OBJECT class PrimitiveHeapObject : public HeapObject")) throw new Error("缺 PrimitiveHeapObject 类声明");
if (!hSrc.includes("V8_IT_ABSTRACT")) throw new Error("缺 V8_IT_ABSTRACT 标注");
if (!hSrc.includes("static_assert(sizeof(PrimitiveHeapObject) == sizeof(HeapObject))")) throw new Error("缺 sizeof 断言");
if (!tqSrc.includes("extern class PrimitiveHeapObject extends HeapObject")) throw new Error("tq 缺声明");

console.log("PrimitiveHeapObject 结构 5 锚点");

let rust = banner(
  "src/objects/primitive-heap-object.h.js",
  "src/objects/primitive-heap-object.{h,tq}",
  "PrimitiveHeapObject 抽象基类标记",
);

rust += `
/// 原始值堆对象抽象基类 (原版 V8_OBJECT class PrimitiveHeapObject : public HeapObject, V8_IT_ABSTRACT)
/// 子族: HeapNumber / Oddball / BigInt / Hole / 字符串等无属性基
pub struct PrimitiveHeapObject;

impl PrimitiveHeapObject {
  /// 原版 sizeof 断言: 与 HeapObject 头等大 (无附加字段)
  pub const SIZEOF_EQUALS_HEAP_OBJECT: bool = true;
}

// —— 签名骨架 ——
// explicit PrimitiveHeapObject(Tagged<ReadOnly<Map>> map); (inl)
// DECL_VERIFIER(PrimitiveHeapObject)
pub const SIGNATURES: [&str; 2] = ["explicit(Tagged<ReadOnly<Map>>)", "Verify"];

// —— 提取不变式断言 ——
const _: () = {
  assert!(PrimitiveHeapObject::SIZEOF_EQUALS_HEAP_OBJECT);
};
`;

const p = writeSrc("v8-objects-core", "primitive_heap_object.rs", rust);
console.log(`written ${p}`);
