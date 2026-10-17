#!/usr/bin/env bun
// tool/src/heap/traced-handles-marking-visitor.js
// 机械提取 v8/src/heap/traced-handles-marking-visitor{.h,.cc} 族:
// ConservativeTracedHandlesMarkingVisitor

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/traced-handles-marking-visitor.h");
const ccSrc = readV8("src/heap/traced-handles-marking-visitor.cc");

const anchors = [
  [hSrc, "class ConservativeTracedHandlesMarkingVisitor final"],
  [hSrc, ": public ::heap::base::StackVisitor {"],
  [hSrc, "ConservativeTracedHandlesMarkingVisitor(Heap&, MarkingWorklists::Local&,"],
  [hSrc, "cppgc::internal::CollectionType);"],
  [hSrc, "void VisitPointer(const void*) override;"],
  [hSrc, "const ConservativeTracedHandlesNodeScanner scanner_;"],
  [hSrc, "const TracedHandles::MarkMode mark_mode_;"],
  [ccSrc, "void ConservativeTracedHandlesMarkingVisitor::VisitPointer("],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

console.log(`traced-handles-marking-visitor: 类 1, 方法 2 (构造/VisitPointer), 字段 4`);

let rust = banner(
  "src/heap/traced-handles-marking-visitor.js",
  "src/heap/traced-handles-marking-visitor.h, src/heap/traced-handles-marking-visitor.cc",
  "保守栈扫描下的 traced handles 标记",
);

rust += `/// v8: src/heap/traced-handles-marking-visitor.h
/// ConservativeTracedHandlesMarkingVisitor (::heap::base::StackVisitor)
/// 假设栈/堆上的保守指针可能指进 traced handle 节点 → 保活。
pub struct ConservativeTracedHandlesMarkingVisitor<'a> {
  /// v8: heap_ (引用)
  pub heap: &'a mut Heap,
  /// v8: marking_state_
  pub marking_state: &'a mut MarkingState,
  /// v8: local_marking_worklist_
  pub local_marking_worklist: &'a mut MarkingWorklistsLocal,
  /// v8: scanner_ — ConservativeTracedHandlesNodeScanner
  /// v8: mark_mode_ — TracedHandles::MarkMode (major/minor 由 CollectionType 决定)
  pub mark_mode: TracedHandlesMarkMode,
}

impl ConservativeTracedHandlesMarkingVisitor<'_> {
  /// v8: VisitPointer(const void*) — 扫描指入 handle 节点的保守指针,
  ///     命中则标节点并把对象推标记队列 (cc 实现面)。
  pub fn visit_pointer(&mut self, pointer: *const ()) { todo!() }
}
`;

const p = writeSrc("v8-heap-core", "traced_handles_marking_visitor.rs.skel", rust);
console.log(`written ${p}`);
