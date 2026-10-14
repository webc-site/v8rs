#!/usr/bin/env bun
// tool/src/base/virtual-address-space.h.js — 覆盖 v8/src/base/virtual-address-space.{h,cc}
// 解析 VirtualAddressSpaceBase/VirtualAddressSpace/VirtualAddressSubspace 三层类、
// 33 个 override 方法面与 subspace 的 RegionAllocator/保护键布局
// → v8-base/src/virtual_address_space.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/virtual-address-space.h");
const cc = readV8("src/base/virtual-address-space.cc");

for (const anchor of [
  [h, "V8_BASE_VIRTUAL_ADDRESS_SPACE_H_"],
  [h, "class VirtualAddressSpaceBase"],
  [h, "class V8_BASE_EXPORT VirtualAddressSpace : public VirtualAddressSpaceBase {"],
  [h, "class V8_BASE_EXPORT VirtualAddressSubspace : public VirtualAddressSpaceBase {"],
  [h, "Address AllocatePages(Address hint, size_t size, size_t alignment,"],
  [h, "bool AllocateGuardRegion(Address address, size_t size) override;"],
  [h, "RegionAllocator to manage the virtual address reservation and divide it"],
  [cc, "VirtualAddressSpace::VirtualAddressSpace()"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: override 总数与按类分布
const overrideTotal = (h.match(/override;/g) || []).length;
const vasSection = h.split("class V8_BASE_EXPORT VirtualAddressSpace :")[1]?.split("class V8_BASE_EXPORT VirtualAddressSubspace")[0] ?? "";
const subSection = h.split("class V8_BASE_EXPORT VirtualAddressSubspace")[1] ?? "";
const vasCount = (vasSection.match(/override;/g) || []).length;
const subCount = (subSection.match(/override;/g) || []).length;
// 真解析: subspace 私有布局
const hasProtKey = h.includes("Memory protection key for this virtual address subspace");

const body = `${banner("src/base/virtual-address-space.h.js", "src/base/virtual-address-space.h + .cc", "进程虚拟地址空间与子空间")}
//! v8 侧: override 总 ${overrideTotal} (根空间 ${vasCount} + 子空间 ${subCount} + Base 纯虚); 三层类
//! Rust 策略: trait VirtualAddressSpace (v8-platform 接口); 根空间直调 OS; 子空间包 RegionAllocator

/// v8: class VirtualAddressSpaceBase — 公共父类, 负责子空间析构回调 (FreeSubspace 纯虚)
pub trait VirtualAddressSpaceBase {
    /// v8: FreeSubspace(subspace) — 子空间析构时由父释放预留
    fn free_subspace(&mut self, subspace: &mut VirtualAddressSubspace);
}

/// v8: class VirtualAddressSpace : public VirtualAddressSpaceBase
/// 当前进程的整个虚拟地址空间; 单例语义
pub struct VirtualAddressSpace {
    /// v8: root 语义, 无成员 (cc 构造仅断言页大小)
}

impl VirtualAddressSpace {
    pub fn new() -> Self { Self {} }

    /// v8: SetRandomSeed(seed) / RandomPageAddress()
    pub fn set_random_seed(&self, seed: i64) { todo!() }
    pub fn random_page_address(&self) -> usize { todo!() }
    /// v8: AllocatePages(hint, size, alignment, access) / FreePages
    /// v8: SetPagePermissions / AllocateGuardRegion / FreeGuardRegion
    /// v8: AllocateSharedPages(hint, size, perms, handle, offset) / FreeSharedPages
    /// v8: ActiveMemoryProtectionKey() / CanAllocateSubspaces() / AllocateSubspace(...)
    /// v8: RecommitPages / DiscardSystemPages / DecommitPages
    /// v8: SetName(name) — 根空间恒 false
    pub fn allocate_pages(&self, hint: usize, size: usize, alignment: usize, access: PagePermissions) -> usize { todo!() }
    pub fn free_pages(&self, address: usize, size: usize) { todo!() }
    pub fn allocate_subspace(&self, hint: usize, size: usize, alignment: usize, max: PagePermissions) -> Box<VirtualAddressSubspace> { todo!() }
}

/// v8: class VirtualAddressSubspace — 父空间内连续预留区
/// 布局: RegionAllocator (地址切分) + 地址换算 (base/scale) + 保护键 + name
pub struct VirtualAddressSubspace {
    /// v8: region_allocator_ — 预留区切分管理
    region_allocator: super::region_allocator::RegionAllocator,
    /// v8: Memory protection key (pkey) — ${hasProtKey ? "有" : "无"}
    protection_key: Option<u32>,
    /// v8: name_ — SetName 命名
    name: Option<String>,
}

impl VirtualAddressSubspace {
    /// v8: 同名 override ${subCount} 个; 地址经 base/scale 换算到父空间后委托
    /// v8: CanAllocateSubspaces() 恒 true
    pub fn allocate_pages(&self, hint: usize, size: usize, alignment: usize, access: PagePermissions) -> usize { todo!() }
    pub fn free_pages(&self, address: usize, size: usize) { todo!() }
    pub fn set_name(&mut self, name: &str) -> bool { todo!() }
}

/// v8: PagePermissions — 位标志 (来自 v8-platform, platform 席位承接枚举本体)
pub type PagePermissions = u32;
`;

writeSrc("v8-base", "virtual_address_space.rs.skel", body);
console.log(`virtual-address-space.h.js: overrides=${overrideTotal} (vas=${vasCount} sub=${subCount}) protKey=${hasProtKey}`);
