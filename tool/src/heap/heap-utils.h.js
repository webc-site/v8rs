#!/usr/bin/env bun
// tool/src/heap/heap-utils.h.js
// 机械提取 v8/src/heap/heap-utils.{h,-inl.h}: HeapUtils::GetOwnerHeap 单函数台账。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/heap-utils.h");
const inl = readV8("src/heap/heap-utils-inl.h");

if (!h.includes("class HeapUtils final : public AllStatic {")) throw new Error("HeapUtils 类签名漂移");
if (!h.includes("static V8_INLINE Heap* GetOwnerHeap(Tagged<HeapObject> object);")) {
  throw new Error("GetOwnerHeap 声明漂移");
}
if (!inl.includes("Heap* HeapUtils::GetOwnerHeap(Tagged<HeapObject> object) {")) {
  throw new Error("GetOwnerHeap 定义缺失");
}
// 实现体 = MemoryChunk::FromHeapObject(object)->Metadata()->heap()
if (!inl.includes("return MemoryChunk::FromHeapObject(object)->Metadata()->heap();")) {
  throw new Error("GetOwnerHeap 实现体漂移");
}
console.log("HeapUtils: 方法 1");

let rust = banner(
  "src/heap/heap-utils.h.js",
  "src/heap/heap-utils.{h,-inl.h}",
  "堆工具: 对象→所属 Heap 归属查询",
);

rust += `
/// 堆工具集 (对应 class HeapUtils final : public AllStatic)。
pub struct HeapUtils;

impl HeapUtils {
  /// 对应 GetOwnerHeap: 对象所在页 (MemoryChunk) 的元数据 (Metadata) 里的 heap。
  /// v8: src/heap/heap-utils-inl.h HeapUtils::GetOwnerHeap
  pub fn get_owner_heap(_object: u64) -> *mut () {
    unimplemented!("骨架: MemoryChunk::FromHeapObject + Metadata 待 memory-chunk 族就位");
  }
}
`;

const p = writeSrc("v8-heap-core", "heap_utils.rs.skel", rust);
console.log(`written ${p}`);
