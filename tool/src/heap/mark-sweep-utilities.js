#!/usr/bin/env bun
// tool/src/heap/mark-sweep-utilities.js
// 机械提取 v8/src/heap/mark-sweep-utilities{.h,.cc} 族:
// MarkingVerifierBase (#ifdef VERIFY_HEAP) / 清理 visitor 骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/mark-sweep-utilities.h");
const ccSrc = readV8("src/heap/mark-sweep-utilities.cc");

const anchors = [
  [hSrc, "#ifdef VERIFY_HEAP"],
  [hSrc, "class MarkingVerifierBase : public ObjectVisitorWithCageBases,"],
  [hSrc, "virtual void VerifyMap(Tagged<Map> map) = 0;"],
  [hSrc, "void VerifyRoots();"],
  [hSrc, "void VerifyMarkingOnPage(const NormalPage* page, Address start, Address end);"],
  [hSrc, "void VerifyMarking(NewSpace* new_space);"],
  [hSrc, "void VerifyMarking(PagedSpaceBase* paged_space);"],
  [hSrc, "void VerifyMarking(LargeObjectSpace* lo_space);"],
  [hSrc, "class ExternalStringTableCleanerVisitor final : public RootVisitor {"],
  [hSrc, "class StringForwardingTableCleanerBase {"],
  [ccSrc, "void MarkingVerifierBase::VerifyRoots() {"],
  [ccSrc, "ExternalStringTableCleanerVisitor::VisitRootPointers"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// cc 函数对数
const ccFns = ["MarkingVerifierBase::MarkingVerifierBase", "MarkingVerifierBase::VisitMapPointer", "MarkingVerifierBase::VerifyRoots", "MarkingVerifierBase::VerifyMarkingOnPage"];
for (const f of ccFns) if (!ccSrc.includes(f)) throw new Error(`cc 缺 ${f}`);
// h 类计数
const hClasses = hSrc.match(/^class \w+/gm) || [];
if (hClasses.length < 4) throw new Error(`h 类计数 ${hClasses.length} < 4`);
console.log(`mark-sweep-utilities: h 类 ${hClasses.length}, cc 关键函数 ${ccFns.length}`);

let rust = banner(
  "src/heap/mark-sweep-utilities.js",
  "src/heap/mark-sweep-utilities.h, src/heap/mark-sweep-utilities.cc",
  "标记校验器与字符串表清理公共件",
);

rust += `#![cfg(verify_heap)] // 原文 #ifdef VERIFY_HEAP

/// v8: src/heap/mark-sweep-utilities.h MarkingVerifierBase (仅 VERIFY_HEAP 构建)
/// 虚方法面: Run / bitmap / VerifyMap / VerifyPointers(ObjectSlot|MaybeObjectSlot) /
///           VerifyCodePointer / VerifyRootPointers / IsMarked
/// VisitPointers → VerifyPointers 转发 (h 内联), VisitMapPointer 见 cc。
pub trait MarkingVerifierBase {
  fn run(&mut self);
  fn bitmap(&self, chunk: &MutablePage) -> &MarkingBitmap;
  fn verify_map(&mut self, map: TaggedMap);
  fn verify_pointers(&mut self, start: ObjectSlot, end: ObjectSlot);
  fn verify_code_pointer(&mut self, slot: InstructionStreamSlot);
  fn verify_root_pointers(&mut self, start: FullObjectSlot, end: FullObjectSlot);
  fn is_marked(&self, object: TaggedHeapObject) -> bool;

  /// v8: MarkingVerifierBase::VerifyRoots
  /// v8: MarkingVerifierBase::VerifyMarkingOnPage — 页区间逐对象校验
  /// v8: MarkingVerifierBase::VerifyMarking(NewSpace|PagedSpaceBase|LargeObjectSpace)
}

/// v8: src/heap/mark-sweep-utilities.h ExternalStringTableCleanerVisitor
/// 根 visitor: 外部字符串表清年龄/清表; VisitRootPointers 实现在 cc。
pub struct ExternalStringTableCleanerVisitor<'a> {
  /// v8: heap_
  pub heap: &'a mut Heap,
}

/// v8: src/heap/mark-sweep-utilities.h StringForwardingTableCleanerBase
/// 及其 StringForwardingTableCleaner / SharedStringForwardingTableCleaner (cc)
/// v8: src/heap/mark-sweep-utilities.cc 各 cleaner 的 VisitRootPointers 语义体
`;

const p = writeSrc("v8-heap-core", "mark_sweep_utilities.rs.skel", rust);
console.log(`written ${p}`);
