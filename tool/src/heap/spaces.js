#!/usr/bin/env bun
// tool/src/heap/spaces.js
// 机械提取 v8/src/heap/spaces{.h,.cc,-inl.h} 族:
// Space 基类 / SpaceIterator / PageIterator / MemoryChunkIterator 骨架
// (AllocationSpace 在 globals.h, spaces-enums 面 he 席, 引用不重复定义)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/spaces.h");
const ccSrc = readV8("src/heap/spaces.cc");
const inlSrc = readV8("src/heap/spaces-inl.h");

const anchors = [
  [hSrc, "class V8_EXPORT_PRIVATE Space : public BaseSpace {"],
  [hSrc, "class V8_EXPORT_PRIVATE ObjectIterator : public Malloced {"],
  [hSrc, "class PageIteratorImpl"],
  [hSrc, "class PageRange {"],
  [hSrc, "class ConstPageRange {"],
  [hSrc, "class V8_EXPORT_PRIVATE SpaceWithLinearArea : public Space {"],
  [hSrc, "class V8_EXPORT_PRIVATE SpaceIterator : public Malloced {"],
  [hSrc, "int current_space_;  // from enum AllocationSpace."],
  [hSrc, "class MemoryChunkIterator {"],
  [ccSrc, "SpaceWithLinearArea::SpaceWithLinearArea(Heap* heap, AllocationSpace id,"],
  [ccSrc, "SpaceIterator::SpaceIterator(Heap* heap)"],
  [ccSrc, "bool SpaceIterator::HasNext() {"],
  [ccSrc, "while (current_space_ <= LAST_MUTABLE_SPACE) {"],
  [inlSrc, "PageIteratorImpl<PageType>& PageIteratorImpl<PageType>::operator++() {"],
  [inlSrc, "OldGenerationMemoryChunkIterator::next() {"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

const hClasses = ["Space", "ObjectIterator", "PageIteratorImpl", "PageRange", "ConstPageRange", "SpaceWithLinearArea", "SpaceIterator", "MemoryChunkIterator"];
for (const c of hClasses) if (!new RegExp(`class (?:V8_EXPORT_PRIVATE )?${c}\\b`).test(hSrc)) throw new Error(`h 缺类 ${c}`);
console.log(`spaces: 类 ${hClasses.length}, cc 4 方法 (SpaceWithLinearArea 构造/SpaceIterator×3), inl 迭代器面`);

let rust = banner(
  "src/heap/spaces.js",
  "src/heap/spaces.h, src/heap/spaces.cc, src/heap/spaces-inl.h",
  "空间公共件: Space 基类与各类迭代器",
);

rust += `//! AllocationSpace (RO_SPACE..LAST_MUTABLE_SPACE) 定义于 globals.h,
//! FIRST_MUTABLE_SPACE/LAST_MUTABLE_SPACE 圈定迭代范围。

/// v8: src/heap/spaces.h Space (BaseSpace 派生)
/// 字段面: memory_chunk_list_ / free_list_(unique_ptr<FreeList>)/
///         external_backing_store_bytes_ / allocated_bytes_ 等
/// v8: Space::{CommittedPhysicalMemory, Available, Size, SizeOfObjects,
///     Capacity, Waste, RemovePage, AddPage, ReleasePage, Free, IsInlineable}

/// v8: src/heap/spaces.h ObjectIterator — GetObjectIterator 接口
pub trait ObjectIterator {
  /// v8: ObjectIterator::Next
  fn next(&mut self) -> Option<TaggedHeapObject>;
}

/// v8: src/heap/spaces.h PageIteratorImpl<PageType>
/// ++: page_ = page_->next_page(); operator== 比较 page_
/// v8: src/heap/spaces-inl.h PageIteratorImpl operator++/operator==

/// v8: src/heap/spaces.h PageRange / ConstPageRange — [begin,end) 页区间
/// v8: PageRange(NormalPage* page) 单页便捷构造 (inl)

/// v8: src/heap/spaces.h SpaceWithLinearArea — 线性区空间基类
/// 构造: Space(heap, id, std::move(free_list)) (cc)
/// v8: supports_allocation_observer / allocation_info 等 LAB 面

/// v8: src/heap/spaces.h SpaceIterator — FIRST_MUTABLE_SPACE..LAST_MUTABLE_SPACE
/// HasNext: 跳过空 space; Next: 取当前并 ++ (cc)
pub struct SpaceIterator<'a> {
  pub heap: &'a mut Heap,
  /// v8: current_space_ (AllocationSpace)
  pub current_space: AllocationSpace,
}

/// v8: src/heap/spaces.h MemoryChunkIterator — 跨空间 chunk 迭代 (经 SpaceIterator)
/// v8: OldGenerationMemoryChunkIterator (inl): variant<PageIterator, LargePageIterator>
///     按 OLD_SPACE/NEW_SPACE? 否 — 旧代 chunk 专迭代, 见 inl 43-90 行

// —— 提取不变式断言 ——
const _: () = {
  // SpaceIterator 从 FIRST_MUTABLE_SPACE 起扫 (cc 构造)
};
`;

const p = writeSrc("v8-heap-core", "spaces.rs.skel", rust);
console.log(`written ${p}`);
