#!/usr/bin/env bun
// tool/src/heap/evacuation-verifier.h.js
// 机械提取 v8/src/heap/evacuation-verifier.{h,cc} (整体 VERIFY_HEAP 条件编译):
// EvacuationVerifier 访问者台账 (8 个 Visit 覆写 + Run 驱动序)。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/evacuation-verifier.h");
const cc = readV8("src/heap/evacuation-verifier.cc");

for (const a of [
  "class EvacuationVerifier final : public ObjectVisitorWithCageBases,",
  "explicit EvacuationVerifier(Heap* heap);",
  "void Run();",
  "void VisitPointers(Tagged<HeapObject> host, ObjectSlot start,",
  "void VisitPointers(Tagged<HeapObject> host, MaybeObjectSlot start,",
  "void VisitInstructionStreamPointer(Tagged<Code> host,",
  "void VisitRootPointers(Root root, const char* description,",
  "void VisitMapPointer(Tagged<HeapObject> object) final;",
  "void VisitCodeTarget(Tagged<InstructionStream> host, RelocInfo* rinfo) final;",
  "void VisitEmbeddedPointer(Tagged<InstructionStream> host,",
  "void VerifyHeapObjectImpl(Tagged<HeapObject> heap_object);",
  "bool ShouldVerifyObject(Tagged<HeapObject> heap_object);",
  "void VerifyRoots();",
  "void VerifyEvacuationOnPage(Address start, Address end);",
  "void VerifyEvacuation(NewSpace* new_space);",
  "void VerifyEvacuation(PagedSpaceBase* paged_space);",
]) {
  if (!h.includes(a)) throw new Error(`evacuation-verifier.h 锚缺失: ${a}`);
}
if (!h.includes("#endif  // VERIFY_HEAP")) throw new Error("VERIFY_HEAP 条件臂缺失");
// cc: Run 驱动序锚
for (const a of [
  "void EvacuationVerifier::Run() {",
  "VerifyRoots();",
  "VerifyEvacuation(heap_->new_space());",
  "VerifyEvacuation(heap_->old_space());",
  "VerifyEvacuation(heap_->code_space());",
  "if (heap_->shared_space()) VerifyEvacuation(heap_->shared_space());",
  "void EvacuationVerifier::VisitMapPointer(Tagged<HeapObject> object) {",
  "VerifyHeapObjectImpl(object->map());",
  "void EvacuationVerifier::VisitPointers(Tagged<HeapObject> host,",
  "VerifyPointersImpl(start, end);",
]) {
  if (!cc.includes(a)) throw new Error(`evacuation-verifier.cc 锚缺失: ${a}`);
}
console.log("EvacuationVerifier: Visit 覆写 8 + Verify 私有 5 + Run 驱动 5");

let rust = banner(
  "src/heap/evacuation-verifier.h.js",
  "src/heap/evacuation-verifier.{h,cc}",
  "压空校验 (仅 VERIFY_HEAP): 全部转发地址有效性的访问者",
);

rust += `
/// 压空校验器骨架 (对应 class EvacuationVerifier final, VERIFY_HEAP 专用)。
/// Run 驱动序: VerifyRoots → new/old/code (+shared 若有) 逐空间 VerifyEvacuation。
/// v8: src/heap/evacuation-verifier.cc EvacuationVerifier::Run
pub struct EvacuationVerifierSkeleton;

impl EvacuationVerifierSkeleton {
  /// 对应 Run(): 根 + 四空间扫描
  pub fn run(&mut self) {
    self.verify_roots();
    // VerifyEvacuation(heap_->new_space()); old_space(); code_space();
    // if (heap_->shared_space()) VerifyEvacuation(...)
    unimplemented!("骨架: 空间迭代待空间族就位");
  }

  /// 对应 VerifyRoots: 扫全部根
  pub fn verify_roots(&mut self) {
    unimplemented!("骨架: isolate 根表待 roots 族就位");
  }

  /// 对应 VisitMapPointer → VerifyHeapObjectImpl(object->map())
  pub fn visit_map_pointer(&mut self, _object: u64) {
    unimplemented!("骨架: map 对象校验待对象族就位");
  }

  /// 对应 VisitPointers (ObjectSlot/MaybeObjectSlot) → VerifyPointersImpl
  pub fn visit_pointers(&mut self, _start: u64, _end: u64) {
    unimplemented!("骨架: 槽迭代待 slot 族就位");
  }
}
`;

const p = writeSrc("v8-heap-core", "evacuation_verifier.rs.skel", rust);
console.log(`written ${p}`);
