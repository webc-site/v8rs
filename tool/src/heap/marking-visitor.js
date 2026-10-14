#!/usr/bin/env bun
// tool/src/heap/marking-visitor.js
// 机械提取 v8/src/heap/marking-visitor{.h,-inl.h} 族:
// MarkingVisitorBase / FullMarkingVisitorBase 访问器签名骨架 (模板静态开关)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/marking-visitor.h");
const inlSrc = readV8("src/heap/marking-visitor-inl.h");

const anchors = [
  [hSrc, "class MarkingVisitorBase : public ConcurrentHeapVisitor<ConcreteVisitor> {"],
  [hSrc, "class FullMarkingVisitorBase : public MarkingVisitorBase<ConcreteVisitor> {"],
  [hSrc, "V8_INLINE size_t VisitDescriptorArray(Tagged<Map> map,"],
  [hSrc, "V8_INLINE size_t VisitEphemeronHashTable(Tagged<Map> map,"],
  [hSrc, "V8_INLINE size_t VisitWeakCell(Tagged<Map> map, Tagged<WeakCell> object,"],
  [hSrc, "constexpr bool CanUpdateValuesInHeap() { return true; }"],
  [hSrc, "V8_INLINE static constexpr bool ShouldVisitReadOnlyMapPointer() {"],
  [hSrc, "V8_INLINE static constexpr bool CanEncounterFillerOrFreeSpace() {"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// Visit 重载对数 (h 内 V8_INLINE size_t Visit* 计数)
const visitMethods = hSrc.match(/V8_INLINE size_t Visit(\w+)\(/g) || [];
const expected = ["DescriptorArray", "EphemeronHashTable", "FixedArray", "JSFunction", "JSGlobalProxy", "JSWeakRef", "Map", "SharedFunctionInfo", "TransitionArray", "WeakCell"];
if (visitMethods.length !== expected.length) throw new Error(`Visit 计数 ${visitMethods.length} != ${expected.length}`);
for (const e of expected) if (!hSrc.includes(`Visit${e}(`)) throw new Error(`缺 Visit${e}`);
console.log(`marking-visitor: Visit 特化 ${expected.length}, 类 2 (MarkingVisitorBase/FullMarkingVisitorBase)`);

let rust = banner(
  "src/heap/marking-visitor.js",
  "src/heap/marking-visitor.h, src/heap/marking-visitor-inl.h",
  "主标记 visitor: 按 map 类型分派的访问器骨架",
);

rust += `//! 原文 CRTP (ConcreteVisitor 模板参数); Rust 侧以 trait 承载分派。
//! 构造参数: local_marking_worklists / local_weak_objects / heap /
//!   mark_compact_epoch / code_flush_mode(EnumSet<CodeFlushMode>) /
//!   should_keep_ages_unchanged / code_flushing_increase(u16)。

/// v8: src/heap/marking-visitor.h MarkingVisitorBase
pub trait MarkingVisitorBase {
  /// v8: VisitDescriptorArray — descriptor array 只标 weak 字段并记录于 weak 处理
  fn visit_descriptor_array(&mut self, map: TaggedMap, object: TaggedDescriptorArray) -> usize;
  /// v8: VisitEphemeronHashTable — ephemeron 表分两阶段 (key 存活才标 value)
  fn visit_ephemeron_hash_table(&mut self, map: TaggedMap, object: TaggedEphemeronHashTable) -> usize;
  /// v8: VisitFixedArray
  fn visit_fixed_array(&mut self, map: TaggedMap, object: TaggedFixedArray) -> usize;
  /// v8: VisitJSFunction — 记录 flushed/js-attached 关闭候选
  fn visit_js_function(&mut self, map: TaggedMap, object: TaggedJsFunction) -> usize;
  /// v8: VisitJSGlobalProxy
  fn visit_js_global_proxy(&mut self, map: TaggedMap, object: TaggedJsGlobalProxy) -> usize;
  /// v8: VisitJSWeakRef — 讓 KeepDuringJob 列表接管
  fn visit_js_weak_ref(&mut self, map: TaggedMap, object: TaggedJsWeakRef) -> usize;
  /// v8: VisitMap — 转移数组/后向指针弱处理
  fn visit_map(&mut self, map: TaggedMap, object: TaggedMap) -> usize;
  /// v8: VisitSharedFunctionInfo — 字节码/flush candidate 年龄控制
  fn visit_shared_function_info(&mut self, map: TaggedMap, object: TaggedSharedFunctionInfo) -> usize;
  /// v8: VisitTransitionArray
  fn visit_transition_array(&mut self, map: TaggedMap, object: TaggedTransitionArray) -> usize;
  /// v8: VisitWeakCell — unregister target 记账
  fn visit_weak_cell(&mut self, map: TaggedMap, object: TaggedWeakCell) -> usize;

  /// v8: ShouldVisitReadOnlyMapPointer — 只读堆 map 跳过标记 (静态开关)
  /// v8: CanEncounterFillerOrFreeSpace — 并发/主线程 visitor 语义开关
  /// v8: IsTrivialWeakReferenceValue — 弱引用值是否无需处理
  /// v8: VisitMapPointer (final override)
}

/// v8: src/heap/marking-visitor.h FullMarkingVisitorBase
/// 追加 marking_state_ (heap->marking_state()) 与 CanUpdateValuesInHeap() = true
pub trait FullMarkingVisitorBase: MarkingVisitorBase {
  /// v8: AddStrongReferenceForReferenceSummarizer (空实现, 供 summarizer 覆写)
  /// v8: AddWeakReferenceForReferenceSummarizer
  /// v8: MarkPointerTableEntry(HeapObject, IndirectPointerSlot)
  /// v8: CanUpdateValuesInHeap — full visitor 允许写堆
}
`;

const p = writeSrc("v8-heap-gc", "marking_visitor.rs.skel", rust);
console.log(`written ${p}`);
