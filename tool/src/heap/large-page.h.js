#!/usr/bin/env bun
// tool/src/heap/large-page.h.js
// 机械提取 v8/src/heap/large-page.{h,cc,-inl.h}:
// kMaxCodePageSize 常量 + ctor 断言 (可执行页大小上限) + ClearOutOfLiveRangeSlots
// 桶对齐清槽规则 + CastTraits (is_large 判定)。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/large-page.h");
const cc = readV8("src/heap/large-page.cc");
const inl = readV8("src/heap/large-page-inl.h");

for (const a of [
  "class LargePage final : public MutablePage {",
  "static constexpr int kMaxCodePageSize = 512 * MB;",
  "V8_INLINE static LargePage* FromHeapObject(Isolate* i, Tagged<HeapObject> o);",
  "Tagged<HeapObject> GetObject() const {",
  "LargePage* next_page() { return SbxCast<LargePage>(list_node_.next()); }",
  "void ClearOutOfLiveRangeSlots(Address free_start);",
  "friend class MemoryAllocator;",
  "static inline bool AllowFrom(const BasePage& page) { return page.is_large(); }",
]) {
  if (!h.includes(a)) throw new Error(`large-page.h 锚点缺失: ${a}`);
}
for (const a of [
  "LargePage::LargePage(Heap* heap, BaseSpace* space, size_t chunk_size,",
  "static_assert(LargePage::kMaxCodePageSize <= TypedSlotSet::kMaxOffset);",
  'if (executable && chunk_size > LargePage::kMaxCodePageSize) {\n    FATAL("Code page is too large.");',
  "list_node().Initialize();",
  "set_is_large();",
  "MutablePage::ComputeInitialFlags(executable) | MemoryChunk::LARGE_PAGE;",
  "void LargePage::ClearOutOfLiveRangeSlots(Address free_start) {",
  "Address aligned_area_end =",
  "ChunkAddress() + SlotSet::OffsetForBucket(BucketsInSlotSet());",
]) {
  if (!cc.includes(a)) throw new Error(`large-page.cc 锚点缺失: ${a}`);
}
if (!inl.includes("return SbxCast<LargePage>(BasePage::FromHeapObject(i, o));")) {
  throw new Error("large-page-inl.h FromHeapObject 实现漂移");
}
console.log("LargePage: 常量 1 + ctor 断言 2 + 方法 4");

let rust = banner(
  "src/heap/large-page.h.js",
  "src/heap/large-page.{h,cc,-inl.h}",
  "大对象页: 代码页大小上限与越活区清槽",
);

rust += `
/// 代码页大小上限 (对应 kMaxCodePageSize = 512 * MB)。
/// 动机: 保证 old-to-old 记忆集 typed slot offset 不溢出
/// (static_assert(kMaxCodePageSize <= TypedSlotSet::kMaxOffset))。
pub const K_MAX_CODE_PAGE_SIZE: usize = 512 * 1024 * 1024;

/// 大对象页骨架 (对应 class LargePage final : public MutablePage)。
/// v8: src/heap/large-page.h LargePage
pub struct LargePageSkeleton;

impl LargePageSkeleton {
  /// 对应 ctor 的可执行页大小检查: executable && chunk_size > kMaxCodePageSize
  /// → FATAL("Code page is too large.")
  /// v8: src/heap/large-page.cc LargePage::LargePage
  #[must_use]
  pub fn check_code_page_size(executable: bool, chunk_size: usize) -> Result<(), &'static str> {
    if executable && chunk_size > K_MAX_CODE_PAGE_SIZE {
      Err("Code page is too large.")
    } else {
      Ok(())
    }
  }

  /// 对应 ClearOutOfLiveRangeSlots: 清 free_start 起的越活区槽。
  /// area_end 不一定桶对齐 → 先对齐到桶边界使 RemoveRange 整桶丢弃。
  /// v8: src/heap/large-page.cc LargePage::ClearOutOfLiveRangeSlots
  #[must_use]
  pub fn aligned_area_end_for_bucket_clear(chunk_address: u64, area_end: u64, bucket_size: u64) -> u64 {
    let buckets = area_end.saturating_sub(chunk_address).div_ceil(bucket_size);
    chunk_address + buckets * bucket_size
  }

  /// 对应 CastTraits<LargePage>::AllowFrom: page.is_large()
  /// v8: src/heap/large-page.h CastTraits<LargePage>
  #[must_use]
  pub fn allow_from(is_large: bool) -> bool {
    is_large
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(K_MAX_CODE_PAGE_SIZE, 512 * 1024 * 1024);
  assert!(LargePageSkeleton::check_code_page_size(true, K_MAX_CODE_PAGE_SIZE + 1).is_err());
  assert!(LargePageSkeleton::check_code_page_size(true, K_MAX_CODE_PAGE_SIZE).is_ok());
  assert!(LargePageSkeleton::check_code_page_size(false, K_MAX_CODE_PAGE_SIZE * 2).is_ok());
  // 桶对齐: area_end 已对齐则不变, 否则向上取整
  assert_eq!(LargePageSkeleton::aligned_area_end_for_bucket_clear(0, 130, 64), 128 + 64);
  assert_eq!(LargePageSkeleton::aligned_area_end_for_bucket_clear(0, 128, 64), 128);
  assert!(LargePageSkeleton::allow_from(true));
};
`;

const p = writeSrc("v8-heap-core", "large_page.rs.skel", rust);
console.log(`written ${p}`);
