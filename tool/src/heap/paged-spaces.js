#!/usr/bin/env bun
// tool/src/heap/paged-spaces.js
// 机械提取 v8/src/heap/paged-spaces{.cc,-inl.h} 族 (族 .h 归他席, 读取仅作上下文):
// PagedSpaceBase 常量/方法 + CompactionSpace::DestinationHeap

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/paged-spaces.h"); // 上下文 (本席表外)
const ccSrc = readV8("src/heap/paged-spaces.cc");
const inlSrc = readV8("src/heap/paged-spaces-inl.h");

const anchors = [
  [hSrc, "class HeapObjectRange final {"],
  [hSrc, "class V8_EXPORT_PRIVATE PagedSpaceObjectIterator : public ObjectIterator {"],
  [hSrc, "class V8_EXPORT_PRIVATE PagedSpaceBase"],
  [hSrc, "static const size_t kCompactionMemoryWanted = 500 * KB;"],
  [hSrc, "class V8_EXPORT_PRIVATE PagedSpace : public PagedSpaceBase {"],
  [hSrc, "class V8_EXPORT_PRIVATE CompactionSpace final : public PagedSpace {"],
  [hSrc, "enum class DestinationHeap {"],
  [hSrc, "kSharedSpaceHeap"],
  [hSrc, "class CompactionSpaceCollection : public Malloced {"],
  [ccSrc, "PagedSpaceBase::PagedSpaceBase(Heap* heap, AllocationSpace space,"],
  [ccSrc, "void PagedSpaceBase::TearDown() {"],
  [ccSrc, "void PagedSpaceBase::MergeCompactionSpace(CompactionSpace* other) {"],
  [hSrc, "RawAllocateBackground(LocalHeap* local_heap, size_t min_size_in_bytes,"],
  [inlSrc, "PagedSpaceBase::Free("],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// 常量真提取
const wanted = hSrc.match(/kCompactionMemoryWanted = (\d+) \* KB;/);
if (!wanted || wanted[1] !== "500") throw new Error("kCompactionMemoryWanted 漂移");
const dest = hSrc.match(/enum class DestinationHeap \{([^}]*)\};/)[1].replace(/\/\/[^\n]*/g, "").split(",").map((s) => s.trim()).filter(Boolean);
if (dest.join() !== "kSameHeap,kSharedSpaceHeap") throw new Error(`DestinationHeap 漂移: ${dest}`);
// 注: kMaxPagesToSweep = 1 位于 main-allocator.cc (ContributeToSweeping), 不属本族
console.log(`paged-spaces: kCompactionMemoryWanted=500KB, DestinationHeap ${dest.length}`);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
let rust = banner(
  "src/heap/paged-spaces.js",
  "src/heap/paged-spaces.cc, src/heap/paged-spaces-inl.h (族 .h: src/heap/paged-spaces.h)",
  "分页空间: 迭代器/记账/压缩空间",
);

rust += `/// v8: src/heap/paged-spaces.h CompactionSpace::DestinationHeap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DestinationHeap {
`;
dest.forEach((k, i) => (rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`));
rust += `}

/// v8: src/heap/paged-spaces.h PagedSpaceBase 常量
pub mod paged_space_base_consts {
  /// v8: kCompactionMemoryWanted = 500 * KB — 压缩目标回收量
  pub const K_COMPACTION_MEMORY_WANTED: usize = ${wanted[1]} * 1024;
}

/// v8: src/heap/paged-spaces.h HeapObjectRange — 单页对象区间迭代器
/// 迭代器字段: cage_base_/cur_addr_/cur_size_(u32)/cur_end_
/// v8: HeapObjectRange::iterator::AdvanceToNextObject (-inl.h)
pub struct HeapObjectRange<'a> { pub page: &'a NormalPage }

/// v8: src/heap/paged-spaces.h PagedSpaceObjectIterator — 空间级对象迭代
/// Next() 跳 filler 与垃圾段; AdvanceToNextPage 慢路径 (cc)
/// v8: PagedSpaceObjectIterator::AdvanceToNextPage (cc)
pub struct PagedSpaceObjectIterator<'a> { pub space: &'a PagedSpaceBase }

/// v8: src/heap/paged-spaces.h PagedSpaceBase (SpaceWithLinearArea 派生)
/// 字段面: accounting_stats_ / free_list_ / executable_ /
///         compaction_space_kind_ / sweeper(Sweeper*) /
///         allocated_pages_(AtomicPtr<NormalPage>) 等
/// v8: Contains/ContainsSlow, Capacity, CommittedPhysicalMemory,
///     Increment/DecrementCommittedPhysicalMemory, ClearAllocatorState,
///     Available, Size, Waste, RawAllocateBackground,
///     Free/FreeDuringSweeping (-inl.h), ResetFreeList,
///     Decrease/IncreaseAllocatedBytes, Decrease/IncreaseCapacity,
///     TearDown, MergeCompactionSpace, InitializePage
/// v8: src/heap/paged-spaces.cc PagedSpaceBase 各实现 (方法面见锚)
/// v8: src/heap/paged-spaces.h PagedSpace — 指定 AllocationSpace 的分页空间
/// v8: src/heap/paged-spaces.h CompactionSpace — 后台合并目标 (DestinationHeap)
/// v8: src/heap/paged-spaces.h CompactionSpaceCollection — 多空间压缩集合

// —— 提取不变式断言 ——
const _: () = {
  assert!(DestinationHeap::SHARED_SPACE_HEAP == 1);
  assert!(paged_space_base_consts::K_COMPACTION_MEMORY_WANTED == 512_000);
};
`;

const p = writeSrc("v8-heap-core", "paged_spaces.rs.skel", rust);
console.log(`written ${p}`);
