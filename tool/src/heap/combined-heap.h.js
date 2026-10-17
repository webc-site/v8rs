#!/usr/bin/env bun
// tool/src/heap/combined-heap.h.js
// 机械提取 v8/src/heap/combined-heap.{h,cc}: CombinedHeapObjectIterator 骨架 +
// IsValidHeapObject/IsValidCodeObject 谓词台账。
// 规则: 类签名/内联谓词逐锚断言; cc 侧 Next() 两阶段迭代 (heap 迭代尽转只读堆)。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/combined-heap.h");
const cc = readV8("src/heap/combined-heap.cc");

for (const a of [
  "class V8_EXPORT_PRIVATE CombinedHeapObjectIterator final {",
  "CombinedHeapObjectIterator(",
  "HeapObjectIterator::HeapObjectsFiltering filtering =",
  "HeapObjectIterator::HeapObjectsFiltering::kNoFiltering);",
  "Tagged<HeapObject> Next();",
  "HeapObjectIterator heap_iterator_;",
  "ReadOnlyHeapObjectIterator ro_heap_iterator_;",
  "inline bool IsValidHeapObject(Heap* heap,",
  "return ReadOnlyHeap::Contains(object) || heap->Contains(object) ||",
  "heap->SharedHeapContains(object);",
  "inline bool IsValidCodeObject(Heap* heap,",
  "if (V8_EXTERNAL_CODE_SPACE_BOOL) {",
  "return ReadOnlyHeap::Contains(object) || heap->ContainsCode(object);",
]) {
  if (!h.includes(a)) throw new Error(`combined-heap.h 锚点缺失: ${a}`);
}
// cc: Next 先只读堆后可写堆
if (!cc.includes("Tagged<HeapObject> CombinedHeapObjectIterator::Next() {")) {
  throw new Error("cc Next 定义缺失");
}
if (!cc.includes("Tagged<HeapObject> object = ro_heap_iterator_.Next();")) {
  throw new Error("cc Next 只读堆优先锚缺失");
}
if (!cc.includes("return heap_iterator_.Next();")) throw new Error("cc Next 回退可写堆锚缺失");
console.log("CombinedHeapObjectIterator 1 类 + 谓词 2");

let rust = banner(
  "src/heap/combined-heap.h.js",
  "src/heap/combined-heap.{h,cc}",
  "全堆对象迭代器 (可写堆 + 只读堆两阶段)",
);

rust += `
/// 全堆对象迭代器 (对应 class CombinedHeapObjectIterator final)。
/// 先耗尽 ReadOnlyHeapObjectIterator (只读堆), 再走 HeapObjectIterator (可写堆)。
/// v8: src/heap/combined-heap.h CombinedHeapObjectIterator
pub struct CombinedHeapObjectIteratorSkeleton {
  /// 对应 ReadOnlyHeapObjectIterator ro_heap_iterator_;
  pub ro_heap_exhausted: bool,
  /// 对应 HeapObjectIterator heap_iterator_;
  pub heap_exhausted: bool,
}

impl CombinedHeapObjectIteratorSkeleton {
  /// 对应 Next(): ro_heap_iterator_.Next() 非空即返回; 否则 heap_iterator_.Next()
  /// v8: src/heap/combined-heap.cc CombinedHeapObjectIterator::Next
  pub fn next(&mut self) -> Option<()> {
    if !self.ro_heap_exhausted {
      // 骨架: 只读堆对象先出; 迭代尽后置位转入可写堆
      self.ro_heap_exhausted = true;
      unimplemented!("骨架: ReadOnlyHeapObjectIterator 待只读堆就位");
    }
    if !self.heap_exhausted {
      self.heap_exhausted = true;
      unimplemented!("骨架: HeapObjectIterator 待对象迭代族就位");
    }
    None
  }
}

/// 对应 inline bool IsValidHeapObject: 只读堆 ∨ 本堆 ∨ 共享堆 含 object
/// v8: src/heap/combined-heap.h IsValidHeapObject
#[must_use]
pub fn is_valid_heap_object(
  read_only_contains: bool,
  heap_contains: bool,
  shared_heap_contains: bool,
) -> bool {
  read_only_contains || heap_contains || shared_heap_contains
}

/// 对应 inline bool IsValidCodeObject: 外部代码空间开 → 只看 ContainsCode;
/// 否则 只读堆 ∨ ContainsCode
/// v8: src/heap/combined-heap.h IsValidCodeObject
#[must_use]
pub fn is_valid_code_object(
  external_code_space: bool,
  read_only_contains: bool,
  heap_contains_code: bool,
) -> bool {
  if external_code_space {
    heap_contains_code
  } else {
    read_only_contains || heap_contains_code
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(is_valid_heap_object(false, true, false));
  assert!(!is_valid_heap_object(false, false, false));
  assert!(is_valid_code_object(true, false, false) == false);
  assert!(is_valid_code_object(true, true, false) == true);
  assert!(is_valid_code_object(false, true, false));
};
`;

const p = writeSrc("v8-heap-core", "combined_heap.rs.skel", rust);
console.log(`written ${p}`);
