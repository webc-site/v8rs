#!/usr/bin/env bun
// tool/src/base/region-allocator.h.js — 覆盖 v8/src/base/region-allocator.{h,cc}
// 解析 RegionState/AllocationStrategy 枚举、Region 子结构、双有序集布局 (all_regions_/free_regions_)
// 与分配/释放/查询方法族
// → v8-base/src/region_allocator.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/region-allocator.h");
const cc = readV8("src/base/region-allocator.cc");

for (const anchor of [
  [h, "V8_BASE_REGION_ALLOCATOR_H_"],
  [h, "class V8_BASE_EXPORT RegionAllocator final {"],
  [h, "enum class RegionState {"],
  [h, "enum class AllocationStrategy {"],
  [h, "Address AllocateRegion(size_t size, AllocationStrategy allocation_strategy ="],
  [h, "bool AllocateRegionAt(Address requested_address, size_t size,"],
  [h, "size_t FreeRegion(Address address, AddressRegion* free_region = nullptr) {"],
  [h, "using AllRegionsSet = std::set<Region*, AddressEndOrder>;"],
  [cc, "RegionAllocator::RegionAllocator(Address memory_region_begin,"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: 两枚举
const enumRe = /enum class (\w+) \{\s*([\s\S]*?)\n  \};/;
const stateE = h.match(/enum class RegionState \{\s*([\s\S]*?)\n  \};/);
const stratE = h.match(/enum class AllocationStrategy \{\s*([\s\S]*?)\n  \};/);
const states = stateE ? stateE[1].split(",").map((s) => s.trim()).filter(Boolean) : [];
const strategies = stratE ? stratE[1].split(",").map((s) => s.trim()).filter(Boolean) : [];
// 真解析: 公共方法
const methods = [...new Set([...h.matchAll(/^  (?:Address|size_t|bool|void) (\w+)\(/gm)].map((m) => m[1]))];

const body = `${banner("src/base/region-allocator.h.js", "src/base/region-allocator.h + .cc", "地址区间分配器 (区段切分/合并, 不做真实内存操作)")}
//! v8 侧: 枚举 ${states.length}+${strategies.length}; 方法 ${methods.length}; 双集布局
//! Rust 策略: BTreeSet<Region> ×2 (AddressEndOrder / SizeAddressOrder) 或 Vec + 排序索引

/// v8: enum class RegionState { ${states.join(", ")} }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionState {
    /// v8: ${states[0]}
    Free,
    /// v8: ${states[1]}
    Excluded,
    /// v8: ${states[2]}
    Allocated,
}

/// v8: enum class AllocationStrategy { ${strategies.join(", ")} }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllocationStrategy {
    /// v8: ${strategies[0]}
    FirstFit,
    /// v8: ${strategies[1]}
    LargestFit,
}

/// v8: class Region : public AddressRegion (address + size + state_)
#[derive(Clone, Debug)]
pub struct Region {
    pub address: usize,
    pub size: usize,
    /// v8: state_
    pub state: RegionState,
}

/// v8: class RegionAllocator final
/// 布局: whole_region_ + all_regions_ (按地址尾排序) + free_regions_ (按大小/地址排序)
/// 回调: on_split_ / on_merge_ (SplitMergeCallback)
pub struct RegionAllocator {
    /// v8: whole_region_ (构造时 [address, address+size))
    whole_region: (usize, usize),
    /// v8: all_regions_ — AddressEndOrder
    all_regions: std::collections::BTreeSet<Region>,
    /// v8: free_regions_ — SizeAddressOrder
    free_size: usize,
    /// v8: page_size_
    page_size: usize,
}

impl RegionAllocator {
    /// v8: RegionAllocator(address, size, page_size) — 初始一个 kFree 整区
    pub fn new(address: usize, size: usize, page_size: usize) -> Self { todo!() }

    /// v8: AllocateRegion(size, strategy=kFirstFit) / AllocateRegion(rng, size) — 随机地址版
    /// v8: AllocateRegionAt(requested_address, size, state) — 定点
    /// v8: AllocateAlignedRegion(size, alignment) / AllocateRegion(hint, size, alignment)
    /// v8: FreeRegion(address) → 释放大小; TrimRegion(address, new_size) → 截小
    /// v8: TryGrowRegion(address, new_size) — 相邻空闲吞并
    pub fn allocate_region(&mut self, size: usize, strategy: AllocationStrategy) -> usize { todo!() }
    pub fn allocate_region_at(&mut self, requested_address: usize, size: usize, state: RegionState) -> bool { todo!() }
    pub fn free_region(&mut self, address: usize) -> usize { todo!() }
    pub fn trim_region(&mut self, address: usize, new_size: usize) -> usize { todo!() }
    pub fn try_grow_region(&mut self, address: usize, new_size: usize) -> bool { todo!() }

    /// v8: CheckRegion(address) → 大小; IsFree(address, size)
    /// v8: begin/end/size/contains ×2/free_size/GetLargestFreeRegionSize/page_size
    pub fn check_region(&self, address: usize) -> usize { todo!() }
    pub fn is_free(&self, address: usize, size: usize) -> bool { todo!() }
    pub fn free_size(&self) -> usize { self.free_size }
    pub fn get_largest_free_region_size(&self) -> usize { todo!() }
    /// v8: set_on_split_callback / set_on_merge_callback — 切分/合并通知
    /// v8: Print(os) — 调试输出
}

// ---- v8 方法对账 (${methods.length} 项) ----
// ${methods.join(", ")}
`;

writeSrc("v8-base", "region_allocator.rs.skel", body);
console.log(`region-allocator.h.js: states=${states.length} strategies=${strategies.length} methods=${methods.length}`);
