#!/usr/bin/env bun
// tool/src/base/page-allocator.h.js — 覆盖 v8/src/base/page-allocator.{h,cc}
// 解析 PageAllocator 14 个 override 方法面、Permission 枚举静态对账与 OS 转发
// → v8-base/src/page_allocator.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/page-allocator.h");
const cc = readV8("src/base/page-allocator.cc");

for (const anchor of [
  [h, "V8_BASE_PAGE_ALLOCATOR_H_"],
  [h, "class V8_BASE_EXPORT PageAllocator"],
  [h, "size_t AllocatePageSize() override { return allocate_page_size_; }"],
  [h, "void* AllocatePages(void* hint, size_t size, size_t alignment,"],
  [h, "bool SealPages(void* address, size_t size) override;"],
  [cc, "STATIC_ASSERT_ENUM(PageAllocator::kNoAccess,"],
  [cc, "return base::OS::Allocate(hint, size, alignment,"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: override 方法面
const overrides = [...h.matchAll(/^  (?:[\w:<>*& ]+?) (\w+)\([^;]*\) override;/gm)].map((m) => m[1]);
// 真解析: Permission 枚举静态对账对
const permAsserts = [...cc.matchAll(/STATIC_ASSERT_ENUM\(PageAllocator::(\w+),/g)].map((m) => m[1]);

const body = `${banner("src/base/page-allocator.h.js", "src/base/page-allocator.h + .cc", "页分配器: v8-platform 接口的 base 实现 (OS 转发)")}
//! v8 侧: override 方法 ${overrides.length}; Permission 对账 ${permAsserts.length} 项; 私有 RemapShared 1
//! Rust 策略: trait PageAllocator (对齐 v8::PageAllocator 接口); OS 层 mmap/mprotect 由 platform 席位承接

/// v8: Permission 枚举 — 与 base::OS::MemoryPermission 静态对账 (cc ${permAsserts.length} 条):
/// ${permAsserts.join(", ")}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Permission {
    /// v8: kNoAccess
    NoAccess,
    /// v8: kReadWrite
    ReadWrite,
    /// v8: kReadWriteExecute
    ReadWriteExecute,
    /// v8: kReadExecute
    ReadExecute,
    /// v8: kNoAccessWillJitLater
    NoAccessWillJitLater,
}

/// v8: class PageAllocator : public v8::PageAllocator
/// 构造: allocate_page_size_ = OS::AllocatePageSize(); commit_page_size_ = OS::CommitPageSize()
pub struct PageAllocator {
    /// v8: allocate_page_size_
    allocate_page_size: usize,
    /// v8: commit_page_size_
    commit_page_size: usize,
}

impl PageAllocator {
    /// v8: AllocatePageSize() / CommitPageSize()
    pub fn allocate_page_size(&self) -> usize { self.allocate_page_size }
    pub fn commit_page_size(&self) -> usize { self.commit_page_size }

    /// v8: SetRandomMmapSeed(seed) — 转发 OS::SetRandomMmapSeed
    pub fn set_random_mmap_seed(&self, seed: i64) { todo!() }
    /// v8: GetRandomMmapAddr() — 转发 OS
    pub fn get_random_mmap_addr(&self) -> *mut u8 { todo!() }
    /// v8: AllocatePages(hint, size, alignment, access)
    /// 语义: kNoAccessWillJitLater → kNoAccess 转换后 OS::Allocate
    pub fn allocate_pages(&self, hint: *mut u8, size: usize, alignment: usize, access: Permission) -> *mut u8 { todo!() }
    /// v8: CanAllocateSharedPages() / AllocateSharedPages(size, original_address)
    /// v8: FreePages(address, size) / ReleasePages(address, size, new_size)
    /// v8: SetPermissions / RecommitPages / DiscardSystemPages / DecommitPages / SealPages
    pub fn free_pages(&self, address: *mut u8, size: usize) -> bool { todo!() }
    pub fn set_permissions(&self, address: *mut u8, size: usize, access: Permission) -> bool { todo!() }
    pub fn discard_system_pages(&self, address: *mut u8, size: usize) -> bool { todo!() }
    pub fn decommit_pages(&self, address: *mut u8, size: usize) -> bool { todo!() }
    pub fn seal_pages(&self, address: *mut u8, size: usize) -> bool { todo!() }
    /// v8 (private): RemapShared(old, new, size) — SharedMemory 搬迁
    fn remap_shared(&self, old_address: *mut u8, new_address: *mut u8, size: usize) { todo!() }
}

// ---- v8 override 对账 (${overrides.length} 项) ----
// ${overrides.join(", ")}
// 另: cc 内部类 SharedMemoryMapping / SharedMemory (friend) — 语义面 Developer 补
`;

writeSrc("v8-base", "page_allocator.rs.skel", body);
console.log(`page-allocator.h.js: overrides=${overrides.length} permAsserts=${permAsserts.length}`);
