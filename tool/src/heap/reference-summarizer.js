#!/usr/bin/env bun
// tool/src/heap/reference-summarizer.js
// 机械提取 v8/src/heap/reference-summarizer{.h,.cc} 族:
// ReferenceSummary — 强/弱引用集合汇总 (校验用)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/reference-summarizer.h");
const ccSrc = readV8("src/heap/reference-summarizer.cc");

const anchors = [
  [hSrc, "class ReferenceSummary {"],
  [hSrc, "static ReferenceSummary SummarizeReferencesFrom(Heap* heap,"],
  [hSrc, "using UnorderedHeapObjectSet ="],
  [hSrc, "UnorderedHeapObjectSet& strong_references() { return strong_references_; }"],
  [hSrc, "UnorderedHeapObjectSet& weak_references() { return weak_references_; }"],
  [hSrc, "void Clear() {"],
  [ccSrc, "ReferenceSummary ReferenceSummary::SummarizeReferencesFrom"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

if (!ccSrc.includes("SummarizeReferencesFrom")) throw new Error("cc 缺 SummarizeReferencesFrom");
console.log(`reference-summarizer: 类 1 (ReferenceSummary), 静态入口 1, 集合 2 (strong/weak)`);

let rust = banner(
  "src/heap/reference-summarizer.js",
  "src/heap/reference-summarizer.h, src/heap/reference-summarizer.cc",
  "对象引用汇总 (仅供堆校验)",
);

rust += `/// v8: src/heap/reference-summarizer.h ReferenceSummary
/// strong/weak 两个 UnorderedHeapObjectSet (unordered_set<HeapObject, Object::Hasher>)
pub struct ReferenceSummary {
  /// v8: strong_references_ — 对象的全部强指针目标
  pub strong_references: UnorderedHeapObjectSet,
  /// v8: weak_references_ — 弱指针目标; ephemeron 表的 value 也归此
  pub weak_references: UnorderedHeapObjectSet,
}

impl ReferenceSummary {
  /// v8: ReferenceSummary::SummarizeReferencesFrom (static)
  /// 用真实标记 visitor 遍历, 结果贴近真实 GC 行为; 仅校验用。
  pub fn summarize_references_from(heap: &mut Heap, obj: TaggedHeapObject) -> Self { todo!() }
  /// v8: Clear
  pub fn clear(&mut self) {
    self.strong_references.clear();
    self.weak_references.clear();
  }
}

/// v8: src/heap/reference-summarizer.cc — SummarizeReferencesFrom 实现
/// (基于 SummarizerVisitor / MarkingVisitorBase 子类收集引用)
/// v8: ReferenceSummarizer visitor 面
`;

const p = writeSrc("v8-heap-core", "reference_summarizer.rs.skel", rust);
console.log(`written ${p}`);
