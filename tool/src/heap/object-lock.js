#!/usr/bin/env bun
// tool/src/heap/object-lock.js
// 机械提取 v8/src/heap/object-lock{.h,-inl.h} 族:
// ObjectLock / ObjectLockGuard — 对象级页锁

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/object-lock.h");
const inlSrc = readV8("src/heap/object-lock-inl.h");

const anchors = [
  [hSrc, "class ObjectLock final {"],
  [hSrc, "V8_INLINE static void Lock(Isolate* isolate, Tagged<HeapObject> heap_object);"],
  [hSrc, "class ObjectLockGuard final {"],
  [inlSrc, "void ObjectLock::Lock(Isolate* isolate, Tagged<HeapObject> heap_object) {"],
  [inlSrc, "void ObjectLock::Unlock(Isolate* isolate, Tagged<HeapObject> heap_object) {"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// inl 语义: 锁在 MutablePage 上
if (!inlSrc.includes("MutablePage::FromHeapObject") && !inlSrc.includes("mutex()->Lock")) {
  // 宽松校验: 至少存在页级锁痕迹
  if (!/Lock\(\)|->Lock/.test(inlSrc)) throw new Error("inl 缺页锁调用");
}
console.log(`object-lock: 类 2, 静态方法 2, RAII 守卫 1`);

let rust = banner(
  "src/heap/object-lock.js",
  "src/heap/object-lock.h, src/heap/object-lock-inl.h",
  "对象级锁: 转成所在 MutablePage 的互斥锁",
);

rust += `/// v8: src/heap/object-lock.h ObjectLock
pub struct ObjectLock;

impl ObjectLock {
  /// v8: src/heap/object-lock-inl.h ObjectLock::Lock
  /// 语义: MutablePage::FromHeapObject(heap_object)->mutex()->Lock()
  pub fn lock(isolate: &Isolate, heap_object: TaggedHeapObject) { todo!() }
  /// v8: src/heap/object-lock-inl.h ObjectLock::Unlock
  pub fn unlock(isolate: &Isolate, heap_object: TaggedHeapObject) { todo!() }
}

/// v8: src/heap/object-lock.h ObjectLockGuard — RAII: 构造 Lock, 析构 Unlock
pub struct ObjectLockGuard<'a> {
  /// v8: isolate_
  pub isolate: &'a Isolate,
  /// v8: raw_object_
  pub raw_object: TaggedHeapObject,
}

impl Drop for ObjectLockGuard<'_> {
  /// v8: ~ObjectLockGuard → ObjectLock::Unlock
  fn drop(&mut self) { todo!() }
}
`;

const p = writeSrc("v8-heap-core", "object_lock.rs.skel", rust);
console.log(`written ${p}`);
