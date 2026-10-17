#!/usr/bin/env bun
// tool/src/heap/normal-page.js
// 机械提取 v8/src/heap/normal-page{.h,.cc,-inl.h} 族:
// NormalPage 常量/方法骨架 (kPageSize = kRegularPageSize)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/normal-page.h");
const ccSrc = readV8("src/heap/normal-page.cc");
const inlSrc = readV8("src/heap/normal-page-inl.h");

const anchors = [
  [hSrc, "class NormalPage final : public MutablePage {"],
  [hSrc, "static constexpr int kPageSize = kRegularPageSize;"],
  [hSrc, "V8_INLINE static NormalPage* FromAddress(Address addr);"],
  [hSrc, "V8_INLINE static NormalPage* FromAllocationAreaAddress(Address address);"],
  [hSrc, "static bool OnSamePage(Address address1, Address address2) {"],
  [hSrc, "static bool IsAlignedToPageSize(Address addr) {"],
  [hSrc, "static NormalPage* ConvertNewToOld(NormalPage* old_page, FreeMode free_mode);"],
  [hSrc, "return area_size() - wasted_memory() - allocated_bytes();"],
  [hSrc, "FreeListCategory* free_list_category(FreeListCategoryType type) {"],
  [hSrc, "V8_EXPORT_PRIVATE void CreateBlackArea(Address start, Address end);"],
  [hSrc, "void InitializeFreeListCategories();"],
  [hSrc, "V8_INLINE static NormalPage* FromHeapObject(Tagged<HeapObject> o);"],
  [ccSrc, "NormalPage::NormalPage("],
  [ccSrc, "NormalPage::ConvertNewToOld("],
  [inlSrc, "NormalPage::FromAddress"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// kRegularPageSize = 1 << kPageSizeBits (build_config.h:83), kPageSizeBits 默认 18
if (!hSrc.includes("kPageSize = kRegularPageSize")) throw new Error("kPageSize 表达式漂移");
const ccFns = ["NormalPage::NormalPage", "NormalPage::ConvertNewToOld", "NormalPage::AvailableInFreeList", "NormalPage::AllocateFreeListCategories", "NormalPage::ReleaseFreeListCategories", "NormalPage::CreateBlackArea", "NormalPage::DestroyBlackArea", "NormalPage::ClearBlackAllocation"];
for (const f of ccFns) if (!ccSrc.includes(f)) throw new Error(`cc 缺 ${f}`);
console.log(`normal-page: kPageSize=kRegularPageSize(=1<<kPageSizeBits, 默认 18 → 256KB), cc 方法 ${ccFns.length}`);

let rust = banner(
  "src/heap/normal-page.js",
  "src/heap/normal-page.h, src/heap/normal-page.cc, src/heap/normal-page-inl.h",
  "常规页 (256KB): 页头/自由链/黑区记账",
);

rust += `/// v8: src/heap/normal-page.h NormalPage
pub struct NormalPage {
  /// v8: MutablePage 基类面 (见 mutable_page.rs, 他席)
  pub base: MutablePage,
  /// v8: categories_[FreeListCategoryType] — 自由链分类数组
  pub categories: [FreeListCategory; FREE_LIST_CATEGORY_COUNT],
  /// v8: active_system_pages_ (unique_ptr)
  pub active_system_pages: Option<Box<ActiveSystemPages>>,
}

impl NormalPage {
  /// v8: NormalPage::kPageSize = kRegularPageSize = 1 << kPageSizeBits
  /// kPageSizeBits 默认 18 (v8/src/base/build_config.h:80) → 262144
  pub const K_PAGE_SIZE: usize = K_REGULAR_PAGE_SIZE;

  /// v8: NormalPage::FromAddress(addr) — 地址 → 页 (-inl.h)
  /// v8: NormalPage::FromHeapObject(o) (-inl.h)
  /// v8: NormalPage::FromAllocationAreaAddress — 允许 [area_start, +kTaggedSize] 越尾
  /// v8: OnSamePage — MemoryChunk::FromAddress 等值
  /// v8: IsAlignedToPageSize — MemoryChunk::IsAligned
  /// v8: ConvertNewToOld(page, free_mode) — 新生代页转老生代 (cc)
  /// v8: next_page/prev_page — list_node_ 链
  /// v8: ForAllFreeListCategories(callback)
  /// v8: AvailableInFreeList / AvailableInFreeListFromAllocatedBytes =
  ///      area_size() - wasted_memory() - allocated_bytes()
  /// v8: free_list_category(type) — &categories_[type]
  /// v8: CreateBlackArea/DestroyBlackArea/ClearBlackAllocation
  /// v8: InitializeFreeListCategories/AllocateFreeListCategories/ReleaseFreeListCategories
  /// v8: ClearTypedSlotsInFreeMemory / AssertNoTypedSlotsInFreeMemory (模板 remembered_set)
  /// v8: MarkNeverAllocateForTesting / MarkEvacuationCandidate / ClearEvacuationCandidate / AbortEvacuation
}

// —— 提取不变式断言 (kPageSizeBits=18 构建) ——
const _: () = {
  assert!(NormalPage::K_PAGE_SIZE == 262_144);
};
`;

const p = writeSrc("v8-heap-core", "normal_page.rs.skel", rust);
console.log(`written ${p}`);
