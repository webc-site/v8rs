#!/usr/bin/env bun
// tool/src/heap/visit-object.js
// 机械提取 v8/src/heap/visit-object{.h,.cc} 族:
// VisitObject / VisitObjectBody 自由函数族

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/visit-object.h");
const ccSrc = readV8("src/heap/visit-object.cc");

const anchors = [
  [hSrc, "void VisitObject(Isolate* isolate, Tagged<HeapObject> object,"],
  [hSrc, "void VisitObject(LocalIsolate* isolate, Tagged<HeapObject> object,"],
  [hSrc, "void VisitObjectBody(Isolate* isolate, Tagged<HeapObject> object,"],
  [hSrc, "void VisitObjectBody(Isolate* isolate, Tagged<Map> map,"],
  [hSrc, "void VisitObjectBody(LocalIsolate* isolate, Tagged<HeapObject> object,"],
  [ccSrc, "void VisitObject(Isolate* isolate, Tagged<HeapObject> object,"],
  [ccSrc, "void VisitObjectBody(Isolate* isolate, Tagged<Map> map,"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

const overloads = (hSrc.match(/void VisitObject(Body)?\(/g) || []).length;
if (overloads !== 5) throw new Error(`重载数 ${overloads} != 5`);
console.log(`visit-object: 自由函数重载 ${overloads} (VisitObject 2 + VisitObjectBody 3)`);

let rust = banner(
  "src/heap/visit-object.js",
  "src/heap/visit-object.h, src/heap/visit-object.cc",
  "单对象 visit 入口 (调试/校验)",
);

rust += `/// v8: src/heap/visit-object.h VisitObject(Isolate*, HeapObject, ObjectVisitor*)
pub fn visit_object(isolate: &Isolate, object: TaggedHeapObject, visitor: &mut dyn ObjectVisitor) { todo!() }
/// v8: VisitObject(LocalIsolate*, ...) — 后台线程变体
pub fn visit_object_local(isolate: &LocalIsolate, object: TaggedHeapObject, visitor: &mut dyn ObjectVisitor) { todo!() }
/// v8: VisitObjectBody(Isolate*, object, visitor) — 先取 map 再转调下者
/// v8: VisitObjectBody(Isolate*, map, object, visitor) — 按 map 分派 body
/// v8: VisitObjectBody(LocalIsolate*, object, visitor) — 后台变体
/// v8: src/heap/visit-object.cc 各实现 (VisitObjectWithoutVisitingPhantom etc. 见锚)
`;

const p = writeSrc("v8-heap-core", "visit_object.rs.skel", rust);
console.log(`written ${p}`);
