#!/usr/bin/env bun
// tool/src/base/virtual-address-space-page-allocator.h.js — 覆盖 v8/src/base/virtual-address-space-page-allocator.{h,cc}
// 解析桥接适配器 (新 VAS API → 旧 PageAllocator API) 的 override 转发表
// → v8-base/src/virtual_address_space_page_allocator.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/virtual-address-space-page-allocator.h");
const cc = readV8("src/base/virtual-address-space-page-allocator.cc");

for (const anchor of [
  [h, "V8_BASE_VIRTUAL_ADDRESS_SPACE_PAGE_ALLOCATOR_H_"],
  [h, "class V8_BASE_EXPORT VirtualAddressSpacePageAllocator"],
  [h, "explicit VirtualAddressSpacePageAllocator(v8::VirtualAddressSpace* vas);"],
  [h, "size_t AllocatePageSize() override { return vas_->allocation_granularity(); }"],
  [h, "size_t CommitPageSize() override { return vas_->page_size(); }"],
  [cc, "VirtualAddressSpacePageAllocator::VirtualAddressSpacePageAllocator("],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: override 清单
const overrides = [...h.matchAll(/^  (?:[\w:<>*& ]+?) (\w+)\([^;]*\) override(?:;| \{)/gm)].map((m) => m[1]);
// 真解析: 一行转发体
const inlineForwards = [...h.matchAll(/^  (?:size_t|void\*|void) (\w+)\([^)]*\) override \{([^}]+)\}/gm)].map((m) => ({ name: m[1], body: m[2].trim() }));

const body = `${banner("src/base/virtual-address-space.h.js", "src/base/virtual-address-space-page-allocator.h + .cc", "VAS→PageAllocator 桥接适配器")}
//! v8 侧: override ${overrides.length}; 其中一行转发 ${inlineForwards.length}
//! Rust 策略: 适配器模式包 &dyn VirtualAddressSpace 实现 PageAllocator trait

/// v8: class VirtualAddressSpacePageAllocator : public v8::PageAllocator
/// 桥接: 新 API (VirtualAddressSpace) → 旧 API (PageAllocator)
pub struct VirtualAddressSpacePageAllocator<'a> {
    /// v8: vas_
    vas: &'a dyn VirtualAddressSpace,
}

impl<'a> VirtualAddressSpacePageAllocator<'a> {
    /// v8: VirtualAddressSpacePageAllocator(vas)
    pub fn new(vas: &'a dyn VirtualAddressSpace) -> Self { Self { vas } }
}

// ---- 转发表 (v8 原文) ----
${inlineForwards.map((f) => `/// v8: ${f.name} — ${f.body.replace(/\s+/g, " ")}
pub fn ${f.name.replace(/([A-Z])/g, "_$1").toLowerCase()}(&self) -> usize { todo!() }`).join("\n")}
/// v8: AllocatePages / FreePages / ReleasePages / SetPermissions / RecommitPages / DiscardSystemPages / DecommitSystemPages
/// 语义: Permission → PagePermissions 位映射 (kNoAccess→0 等), 地址换算后委托 vas_
// override 对账 (${overrides.length}): ${overrides.join(", ")}

// 依赖 trait 声明 (本体在 virtual_address_space.rs.skel)
pub trait VirtualAddressSpace {
    fn allocation_granularity(&self) -> usize;
    fn page_size(&self) -> usize;
}
`;

writeSrc("v8-base", "virtual_address_space_page_allocator.rs.skel", body);
console.log(`virtual-address-space-page-allocator.h.js: overrides=${overrides.length} inlineForwards=${inlineForwards.length}`);
