#!/usr/bin/env bun
// tool/src/heap/heap-verifier.h.js
// 机械提取 v8/src/heap/heap-verifier.{h,cc}:
// SpaceVerificationVisitor trait (3 钩子) + HeapVerifier 静态台账
// (VERIFY_HEAP 实臂 6 + 空臂 5 + VerifyHeapIfEnabled 规则) + cc 访问者清单。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/heap-verifier.h");
const cc = readV8("src/heap/heap-verifier.cc");

for (const a of [
  "class SpaceVerificationVisitor {",
  "virtual void VerifyObject(Tagged<HeapObject> object) = 0;",
  "virtual void VerifyPage(const BasePage* chunk) = 0;",
  "virtual void VerifyPageDone(const BasePage* chunk) = 0;",
  "class HeapVerifier final {",
  "#ifdef VERIFY_HEAP",
  "V8_EXPORT_PRIVATE static void VerifyHeap(Heap* heap);",
  "V8_EXPORT_PRIVATE static void VerifyReadOnlyHeap(Heap* heap);",
  "V8_EXPORT_PRIVATE static void VerifySafeMapTransition(",
  "V8_EXPORT_PRIVATE static void VerifyObjectLayoutChange(",
  "static void VerifyObjectLayoutChangeIsAllowed(Heap* heap,",
  "static void SetPendingLayoutChangeObject(Heap* heap,",
  "static void VerifyRememberedSetFor(Heap* heap, Tagged<HeapObject> object) {}",
  "V8_INLINE static void VerifyHeapIfEnabled(Heap* heap) {",
  "if (v8_flags.verify_heap) {",
  "VerifyHeap(heap);",
]) {
  if (!h.includes(a)) throw new Error(`heap-verifier.h 锚缺失: ${a}`);
}
// VERIFY_HEAP 空臂计数 (非 VERIFY 构建兜底: 7 个方法全空体)
const emptyArms = (h.match(/static void Verify\w+\(Heap\* heap[^{]*\{\}/g) || []).length;
if (emptyArms !== 7) throw new Error(`空臂数 ${emptyArms}, 期望 7`);
// cc 访问者
for (const a of [
  "class VerifySmisVisitor final : public RootVisitor {",
  "class VerifyPointersVisitor : public HeapVisitor<VerifyPointersVisitor>,",
  "void VerifyPointersVisitor::VisitPointers(Tagged<HeapObject> host,",
  "void VerifyPointersVisitor::VisitRootPointers(Root root,",
  "void VerifyPointersVisitor::VisitMapPointer(Tagged<HeapObject> host) {",
  "void HeapVerifier::VerifyHeap(Heap* heap) {",
  "void HeapVerifier::VerifyReadOnlyHeap(Heap* heap) {",
]) {
  if (!cc.includes(a)) throw new Error(`heap-verifier.cc 锚缺失: ${a}`);
}
console.log("HeapVerifier: trait 3 钩子 + 静态方法 7 + 空臂 7");

let rust = banner(
  "src/heap/heap-verifier.h.js",
  "src/heap/heap-verifier.{h,cc}",
  "堆校验 (VERIFY_HEAP): 空间校验访问者 trait 与开关门面",
);

rust += `
/// 空间校验访问者 (对应 class SpaceVerificationVisitor)。
/// v8: src/heap/heap-verifier.h SpaceVerificationVisitor
pub trait SpaceVerificationVisitor {
  /// 对应 VerifyObject: 空间内每个对象调用一次
  fn verify_object(&mut self, object: u64);
  /// 对应 VerifyPage: 每页首对象前调用
  fn verify_page(&mut self, chunk: u64);
  /// 对应 VerifyPageDone: 页内全部对象校验完调用
  fn verify_page_done(&mut self, chunk: u64);
}

/// 堆校验器 (对应 class HeapVerifier final, AllStatic)。
/// 非 VERIFY_HEAP 构建下全部方法为空 (对应 #else 空臂)。
/// v8: src/heap/heap-verifier.h HeapVerifier
pub struct HeapVerifier;

impl HeapVerifier {
  /// 对应 VerifyHeapIfEnabled: v8_flags.verify_heap 时才真校验
  /// v8: src/heap/heap-verifier.h HeapVerifier::VerifyHeapIfEnabled
  pub fn verify_heap_if_enabled(verify_heap_flag: bool) -> Option<()> {
    if verify_heap_flag {
      Some(Self::verify_heap())
    } else {
      None
    }
  }

  /// 对应 VerifyHeap: 全堆扫描 (VERIFY_HEAP 构建)
  /// v8: src/heap/heap-verifier.cc HeapVerifier::VerifyHeap
  pub fn verify_heap() {
    unimplemented!("骨架: VerifySmis/VerifyPointers 双访问者待对象族就位");
  }

  /// 对应 VerifyReadOnlyHeap: 只读堆对象建完后校验
  /// v8: src/heap/heap-verifier.cc HeapVerifier::VerifyReadOnlyHeap
  pub fn verify_read_only_heap() {
    unimplemented!("骨架: 只读堆迭代待 read-only 族就位");
  }

  /// 对应 VerifyObjectLayoutChangeIsAllowed:
  /// 共享空间对象只允许 string 改布局
  /// v8: src/heap/heap-verifier.cc HeapVerifier::VerifyObjectLayoutChangeIsAllowed
  #[must_use]
  pub fn layout_change_allowed(in_shared_space: bool, is_string: bool) -> bool {
    !in_shared_space || is_string
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(HeapVerifier::layout_change_allowed(false, false));
  assert!(HeapVerifier::layout_change_allowed(true, true));
  assert!(!HeapVerifier::layout_change_allowed(true, false));
  assert_eq!(HeapVerifier::verify_heap_if_enabled(false), None);
};
`;

const p = writeSrc("v8-heap-core", "heap_verifier.rs.skel", rust);
console.log(`written ${p}`);
