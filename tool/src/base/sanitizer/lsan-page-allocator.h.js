#!/usr/bin/env bun
// tool/src/base/sanitizer/lsan-page-allocator.h.js — 覆盖 v8/src/base/sanitizer/lsan-page-allocator.{h,cc}
// 解析 LsanPageAllocator 装饰器 override 面、LSan 根注册豁免逻辑 (AllocatePages/FreePages 特化)
// 与 not_registered_regions_ 布局
// → v8-base/src/sanitizer/lsan_page_allocator.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/sanitizer/lsan-page-allocator.h");
const cc = readV8("src/base/sanitizer/lsan-page-allocator.cc");

for (const anchor of [
  [h, "V8_BASE_SANITIZER_LSAN_PAGE_ALLOCATOR_H_"],
  [h, "class V8_BASE_EXPORT LsanPageAllocator : public v8::PageAllocator {"],
  [h, "explicit LsanPageAllocator(v8::PageAllocator* page_allocator);"],
  [h, "not_registered_regions_"],
  [cc, "void* LsanPageAllocator::AllocatePages("],
  [cc, "__lsan_register_root_region"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: override 面
const overrides = [...h.matchAll(/^  (?:[\w:<>*& ]+?) (\w+)\([^;]*\) override(?:;| \{)/gm)].map((m) => m[1]);
// 真解析: 特化方法 (声明在 .h 定义在 .cc, 即非一行体)
const deferred = [...h.matchAll(/^  (?:[\w:<>*& <>,]+) (\w+)\([^;]*\) override;/gm)].map((m) => m[1]);

const body = `${banner("src/base/sanitizer/lsan-page-allocator.h.js", "src/base/sanitizer/lsan-page-allocator.h + .cc", "LSan 根区域登记装饰器")}
//! v8 侧: override ${overrides.length}; LSan 特化 ${deferred.length}; 条件布局 LEAK_SANITIZER
//! 语义: 通用 v8 堆外分配不进 LSan 根集合 → 本装饰器把页登记为根 (__lsan_register_root_region)

/// v8: class LsanPageAllocator : public v8::PageAllocator
/// 布局: page_allocator_ (被装饰者) + allocate/commit_page_size_ +
///       (LEAK_SANITIZER) not_registered_regions_mutex_ + not_registered_regions_
pub struct LsanPageAllocator<'a> {
    /// v8: page_allocator_
    inner: &'a mut dyn PageAllocator,
    /// v8: not_registered_regions_ — AllocatePages 时 LSan 不可用则记延迟登记
    not_registered_regions: std::collections::BTreeSet<usize>,
}

impl<'a> LsanPageAllocator<'a> {
    /// v8: LsanPageAllocator(page_allocator) — 透传页大小
    pub fn new(inner: &'a mut dyn PageAllocator) -> Self { todo!() }

    /// v8: AllocatePages — 分配后 __lsan_register_root_region; 失败则入 not_registered_regions_
    /// (下次 AllocatePages 前补登记)
    pub fn allocate_pages(&mut self, address: usize, size: usize, alignment: usize, access: Permission) -> usize { todo!() }
    /// v8: AllocateSharedPages / CanAllocateSharedPages — 同根登记语义
    /// v8: FreePages — __lsan_unregister_root_region + 从延迟集移除
    pub fn free_pages(&mut self, address: usize, size: usize) -> bool { todo!() }
    /// v8: ReleasePages — 缩小后重新登记
    pub fn release_pages(&mut self, address: usize, size: usize, new_size: usize) -> bool { todo!() }

    // 纯转发面 (v8 一行体):
    /// v8: SetRandomMmapSeed / GetRandomMmapAddr / SetPermissions / RecommitPages /
    ///     DiscardSystemPages / DecomitPages / AllocatePageSize / CommitPageSize
    pub fn set_permissions(&mut self, address: usize, size: usize, access: Permission) -> bool { self.inner.set_permissions(address, size, access) }
}

// ---- override 对账 (${overrides.length} 项) ----
// ${overrides.join(", ")}
// 依赖 trait: PageAllocator (page_allocator.rs.skel) + Permission
`;

writeSrc("v8-base", "sanitizer/lsan_page_allocator.rs.skel", body);
console.log(`sanitizer/lsan-page-allocator.h.js: overrides=${overrides.length} deferred=${deferred.length}`);
