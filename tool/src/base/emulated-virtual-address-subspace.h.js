#!/usr/bin/env bun
// tool/src/base/emulated-virtual-address-subspace.h.js — 族: src/base/emulated-virtual-address-subspace.h + .cc
// 覆盖: EmulatedVirtualAddressSubspace (VirtualAddressSpace 实现, mapped+unmapped 双区)
// → v8-base/src/emulated_virtual_address_subspace.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/base/emulated-virtual-address-subspace.h");
const ccSrc = readV8("src/base/emulated-virtual-address-subspace.cc");

// —— 锚点断言 ——
if (!hSrc.includes("class V8_BASE_EXPORT EmulatedVirtualAddressSubspace final")) throw new Error("缺类");
if (!hSrc.includes("EmulatedVirtualAddressSubspace(v8::VirtualAddressSpace* parent_space,")) throw new Error("缺构造器");
if (!hSrc.includes("return std::nullopt;")) throw new Error("缺 ActiveMemoryProtectionKey 恒空");
if (!hSrc.includes("bool IsUsableSizeForUnmappedRegion(size_t size) const {")) throw new Error("缺未映射区限额谓词");
if (!hSrc.includes("return size <= (unmapped_size() / 2);")) throw new Error("缺 /2 限额语义");
if (!hSrc.includes("RegionAllocator region_allocator_;")) throw new Error("缺 region_allocator_ 字段");
if (!hSrc.includes("RandomNumberGenerator rng_;")) throw new Error("缺 rng_ 字段");

// —— 真解析: cc 方法定义对数 ——
const defs = new Set([...ccSrc.matchAll(/EmulatedVirtualAddressSubspace::~?(\w+)\(/g)].map((m) => m[1]));
const expected = [
  "EmulatedVirtualAddressSubspace", "SetRandomSeed",
  "RandomPageAddress", "AllocatePages", "FreePages", "AllocateSharedPages", "FreeSharedPages",
  "SetPagePermissions", "AllocateGuardRegion", "FreeGuardRegion", "CanAllocateSubspaces",
  "AllocateSubspace", "RecommitPages", "DiscardSystemPages", "DecommitPages",
];
for (const d of expected) if (!defs.has(d)) throw new Error(`cc 缺定义 ${d}`);
if (!ccSrc.includes("~EmulatedVirtualAddressSubspace()")) throw new Error("cc 缺析构定义");
// h 私有 Contains 双重载 + 私有访问器
const priv = ["mapped_size()", "unmapped_size()", "mapped_base()", "unmapped_base()", "MappedRegionContains", "UnmappedRegionContains"];
for (const p of priv) if (!hSrc.includes(p)) throw new Error(`h 缺私有 ${p}`);
console.log(`emulated-virtual-address-subspace: cc 定义 ${expected.length}, h 私有辅助 ${priv.length}`);

let rust = banner(
  "src/base/emulated-virtual-address-subspace.h.js",
  "src/base/emulated-virtual-address-subspace.h + src/base/emulated-virtual-address-subspace.cc",
  "仿真虚拟地址子空间 (mapped 背书 + unmapped 提示分配)",
);

rust += `
//! v8: src/base/emulated-virtual-address-subspace.h/.cc
//! 警示 (原版): 破坏"父空间分配不会落进子空间"不变量, 安全保证弱于真子空间

/// v8: src/base/emulated-virtual-address-subspace.h EmulatedVirtualAddressSubspace
/// 布局: [mapped_size_ 映射区 | (total_size - mapped_size_) 未映射区]
/// 字段: mapped_size_(const), parent_space_*, mutex_, region_allocator_, rng_
/// 分配策略: 先背书映射区, 不够再在未映射区用 hint 向父空间申请新映射
pub struct EmulatedVirtualAddressSubspace {
    mapped_size: usize,
    // Developer: parent_space 引用 + Mutex<RegionAllocator> + RandomNumberGenerator
}

impl EmulatedVirtualAddressSubspace {
    /// v8: 构造 — 接管 parent_space 的背书页分配, 析构时释放
    pub fn new(parent_space: &mut dyn VirtualAddressSpace, base: Address, mapped_size: usize, total_size: usize) -> Self;
    /// v8: cc SetRandomSeed
    pub fn set_random_seed(&mut self, seed: i64);
    /// v8: cc RandomPageAddress — rng 域内随机地址
    pub fn random_page_address(&mut self) -> Address;
    /// v8: cc AllocatePages — mapped 区优先, 溢出走 unmapped 区 hint 分配
    pub fn allocate_pages(&mut self, hint: Address, size: usize, alignment: usize, permissions: PagePermissions) -> Address;
    /// v8: cc FreePages — 按所在区归还 (mapped: region_allocator; unmapped: parent)
    pub fn free_pages(&mut self, address: Address, size: usize);
    /// v8: cc AllocateSharedPages — 仅 mapped 区
    pub fn allocate_shared_pages(&mut self, hint: Address, size: usize, permissions: PagePermissions, handle: SharedMemoryHandle, offset: u64) -> Address;
    /// v8: cc FreeSharedPages
    pub fn free_shared_pages(&mut self, address: Address, size: usize);
    /// v8: cc SetPagePermissions
    pub fn set_page_permissions(&mut self, address: Address, size: usize, permissions: PagePermissions) -> bool;
    /// v8: cc AllocateGuardRegion — 仅 unmapped 区登记 guard
    pub fn allocate_guard_region(&mut self, address: Address, size: usize) -> bool;
    /// v8: cc FreeGuardRegion
    pub fn free_guard_region(&mut self, address: Address, size: usize);
    /// v8: h ActiveMemoryProtectionKey — 恒 None
    pub fn active_memory_protection_key(&self) -> Option<MemoryProtectionKeyId>;
    /// v8: cc CanAllocateSubspaces
    pub fn can_allocate_subspaces(&self) -> bool;
    /// v8: cc AllocateSubspace
    pub fn allocate_subspace(&mut self, hint: Address, size: usize, alignment: usize, max_page_permissions: PagePermissions, key: Option<MemoryProtectionKeyId>, handle: Option<SharedMemoryHandle>) -> Option<Box<dyn VirtualAddressSpace>>;
    /// v8: cc RecommitPages
    pub fn recommit_pages(&mut self, address: Address, size: usize, permissions: PagePermissions) -> bool;
    /// v8: cc DiscardSystemPages
    pub fn discard_system_pages(&mut self, address: Address, size: usize) -> bool;
    /// v8: cc DecommitPages
    pub fn decommit_pages(&mut self, address: Address, size: usize) -> bool;
}

// v8: h 私有辅助: mapped_size/unmapped_size/mapped_base/unmapped_base、
//     Contains(外,内) 闭区间包含、Mapped/UnmappedRegionContains、
//     IsUsableSizeForUnmappedRegion = size <= unmapped_size/2 (限制循环上界估计)
`;

writeSrc("v8-base", "emulated_virtual_address_subspace.rs.skel", rust);
console.log("written v8-base/src/emulated_virtual_address_subspace.rs.skel");
