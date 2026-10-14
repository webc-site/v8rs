#!/usr/bin/env bun
// tool/src/base/bounded-page-allocator.h.js — 族: src/base/bounded-page-allocator.h + .cc
// 覆盖: PageInitializationMode(3)/PageFreeingMode(2)/AllocationStatus(4) 枚举 + Stats
//        + BoundedPageAllocator 类 20 方法 (v8::PageAllocator 实现)
// → v8-base/src/bounded_page_allocator.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/base/bounded-page-allocator.h");
const ccSrc = readV8("src/base/bounded-page-allocator.cc");

// —— 枚举真解析, 对数 ——
function enumVariants(src, name) {
  const m = src.match(new RegExp(`enum class ${name} \\{([\\s\\S]*?)\\};`));
  if (!m) throw new Error(`缺枚举 ${name}`);
  return [...m[1].matchAll(/^\s{2,4}(k\w+),?$/gm)].map((x) => x[1]);
}
const initModes = enumVariants(hSrc, "PageInitializationMode");
const freeModes = enumVariants(hSrc, "PageFreeingMode");
const statuses = enumVariants(hSrc, "AllocationStatus");
if (initModes.length !== 3) throw new Error(`PageInitializationMode ${initModes.length} != 3`);
if (freeModes.length !== 2) throw new Error(`PageFreeingMode ${freeModes.length} != 2`);
if (statuses.length !== 4) throw new Error(`AllocationStatus ${statuses.length} != 4`);

// —— 锚点断言 ——
if (!hSrc.includes("class V8_BASE_EXPORT BoundedPageAllocator : public v8::PageAllocator")) {
  throw new Error("缺 BoundedPageAllocator 类");
}
if (!hSrc.includes("struct Stats {")) throw new Error("缺 Stats");
if (!hSrc.includes("constexpr const char* ToString(BoundedPageAllocator::AllocationStatus status)")) {
  throw new Error("缺自由 ToString");
}
// ToString 字符串表 (h 内 switch + cc 内 AllocationStatusToString 双份)
for (const s of ['"success"', '"failed to commit"', '"ran out of reservation"', '"hinted address taken or not found"']) {
  if (!hSrc.includes(s) && !ccSrc.includes(s)) throw new Error(`缺状态字符串 ${s}`);
}
// cc 方法定义对数: BoundedPageAllocator:: 成员定义
const ccDefs = new Set([...ccSrc.matchAll(/BoundedPageAllocator::(\w+)\(/g)].map((m) => m[1]));
const expectedDefs = [
  "BoundedPageAllocator", "begin", "size", "AllocatePages", "AllocatePagesAt", "ResizeAllocationAt",
  "ReserveForSharedMemoryMapping", "FreePages", "ReleasePages", "SetPermissions", "RecommitPages",
  "DiscardSystemPages", "DecommitPages", "SealPages", "AllocationStatusToString", "RecordStats",
];
for (const d of expectedDefs) if (!ccDefs.has(d)) throw new Error(`cc 缺定义 ${d}`);
console.log(`bounded-page-allocator: 枚举 3/2/4, cc 方法定义 ${ccDefs.size} 种, ToString 字符串 4`);

// —— 真解析: ToString 映射表 (供规则代码) ——
const toStringMap = [...hSrc.matchAll(/case BoundedPageAllocator::AllocationStatus::(k\w+):\s*\n\s*return ("[^"]*");/g)].map(
  (m) => [m[1], m[2]],
);
if (toStringMap.length !== 4) throw new Error(`ToString 分支 ${toStringMap.length} != 4`);

let rust = banner(
  "src/base/bounded-page-allocator.h.js",
  "src/base/bounded-page-allocator.h + src/base/bounded-page-allocator.cc",
  "预预留区域内受限页分配器 (线程安全)",
);

rust += `
//! v8: src/base/bounded-page-allocator.h/.cc

/// v8: src/base/bounded-page-allocator.h PageInitializationMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageInitializationMode {
    /// 已提交页在 Free/Release 时 decommit, 保证下次分配清零
    KAllocatedPagesMustBeZeroInitialized,
    /// 不清零, Free/Release 仅改不可访问, 略快
    KAllocatedPagesCanBeUninitialized,
    /// 假定页已 discarded 且权限正确; 须配 PageFreeingMode::KDiscard
    KRecommitOnly,
}

/// v8: src/base/bounded-page-allocator.h PageFreeingMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFreeingMode {
    /// 释放 = 权限改 kNoAccess (常规平台首选)
    KMakeInaccessible,
    /// 释放 = DiscardSystemPages (macOS ARM64 不许改 RWX 权限时用; 与 kAllocatedPagesMustBeZeroInitialized 不兼容)
    KDiscard,
}

/// v8: src/base/bounded-page-allocator.h BoundedPageAllocator::AllocationStatus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AllocationStatus {
    #[default]
    KSuccess,
    KFailedToCommit,
    KRanOutOfReservation,
    KHintedAddressTakenOrNotFound,
}

impl AllocationStatus {
    /// v8: src/base/bounded-page-allocator.cc AllocationStatusToString
    /// v8: src/base/bounded-page-allocator.h ToString — 字符串表 1:1 展开
    pub fn as_str(&self) -> &'static str {
        match self {
${toStringMap.map(([k, s]) => `            Self::${K(k)} => ${s},`).join("\n")}
        }
    }
}

/// v8: src/base/bounded-page-allocator.h BoundedPageAllocator::Stats
#[derive(Debug, Default)]
pub struct Stats {
    pub free_size: usize,
    pub largest_free_region: usize,
    pub allocation_status: AllocationStatus,
}

/// v8: src/base/bounded-page-allocator.h BoundedPageAllocator
/// 字段: mutex_, allocate_page_size_, commit_page_size_, page_allocator_,
///        region_allocator_, page_initialization_mode_, page_freeing_mode_, allocation_status_
/// 用途: 指针压缩 cage / 可执行页分配 (PC 相对 32 位位移)
/// v8: 构造器要求 allocate_page_size 对齐 commit_page_size (cc L24 DCHECK)
pub struct BoundedPageAllocator { /* Developer: RegionAllocator + Mutex 组合 */ }

impl BoundedPageAllocator {
    /// v8: bounded-page-allocator.cc begin
    pub fn begin(&self) -> Address;
    /// v8: bounded-page-allocator.cc size
    pub fn size(&self) -> usize;
    /// v8: bounded-page-allocator.h contains — 委托 region_allocator_.contains
    pub fn contains(&self, address: Address) -> bool;
    /// v8: bounded-page-allocator.cc AllocatePages(void* hint, ...) — 随机/提示地址分配
    pub fn allocate_pages(&mut self, hint: Option<Address>, size: usize, alignment: usize, access: Permission) -> Option<Address>;
    /// v8: bounded-page-allocator.cc AllocatePagesAt — 指定地址分配, 成功返回 true
    pub fn allocate_pages_at(&mut self, address: Address, size: usize, access: Permission) -> bool;
    /// v8: bounded-page-allocator.cc ResizeAllocationAt — old/new_size 均须 commit 页对齐
    pub fn resize_allocation_at(&mut self, address: Address, old_size: usize, new_size: usize, access: Permission) -> bool;
    /// v8: bounded-page-allocator.cc ReserveForSharedMemoryMapping
    pub fn reserve_for_shared_memory_mapping(&mut self, address: Address, size: usize) -> bool;
    /// v8: bounded-page-allocator.cc FreePages — 按初始化/释放模式 decommit 或改权限
    pub fn free_pages(&mut self, address: Address, size: usize) -> bool;
    /// v8: bounded-page-allocator.cc ReleasePages — 释放尾部 (size-new_size 须 commit 页对齐)
    pub fn release_pages(&mut self, address: Address, size: usize, new_size: usize) -> bool;
    /// v8: bounded-page-allocator.cc SetPermissions
    pub fn set_permissions(&mut self, address: Address, size: usize, access: Permission) -> bool;
    /// v8: bounded-page-allocator.cc RecommitPages
    pub fn recommit_pages(&mut self, address: Address, size: usize, access: Permission) -> bool;
    /// v8: bounded-page-allocator.cc DiscardSystemPages
    pub fn discard_system_pages(&mut self, address: Address, size: usize) -> bool;
    /// v8: bounded-page-allocator.cc DecommitPages
    pub fn decommit_pages(&mut self, address: Address, size: usize) -> bool;
    /// v8: bounded-page-allocator.cc SealPages
    pub fn seal_pages(&mut self, address: Address, size: usize) -> bool;
    /// v8: bounded-page-allocator.h get_last_allocation_status
    pub fn last_allocation_status(&self) -> AllocationStatus;
    /// v8: bounded-page-allocator.cc RecordStats
    pub fn record_stats(&self) -> Stats;
}
`;

function K(name) { return name.replace(/^k/, "K"); }
writeSrc("v8-base", "bounded_page_allocator.rs.skel", rust);
console.log("written v8-base/src/bounded_page_allocator.rs.skel");
