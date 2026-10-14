#!/usr/bin/env bun
// tool/src/base/sanitizer/lsan-virtual-address-space.h.js — 覆盖 v8/src/base/sanitizer/lsan-virtual-address-space.{h,cc}
// 解析 LsanVirtualAddressSpace 装饰器 override 面与 AllocatePages/FreePages 的根登记特化
// → v8-base/src/sanitizer/lsan_virtual_address_space.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/sanitizer/lsan-virtual-address-space.h");
const cc = readV8("src/base/sanitizer/lsan-virtual-address-space.cc");

for (const anchor of [
  [h, "V8_BASE_SANITIZER_LSAN_VIRTUAL_ADDRESS_SPACE_H_"],
  [h, "class V8_BASE_EXPORT LsanVirtualAddressSpace final"],
  [cc, "Address LsanVirtualAddressSpace::AllocatePages(Address hint, size_t size,"],
  [cc, "__lsan_register_root_region"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: override 面
const overrides = [...h.matchAll(/^  (?:[\w:<>*& <>,]+?) (\w+)\([^;]*\) override(?:;| \{)/gm)].map((m) => m[1]);
// 真解析: 一行转发
const inlineFwd = [...h.matchAll(/^  (?:[\w:<>*& ]+?) (\w+)\([^)]*\) override \{([^}]+)\}/gm)].map((m) => ({ name: m[1], body: m[2].trim() }));

const body = `${banner("src/base/sanitizer/lsan-virtual-address-space.h.js", "src/base/sanitizer/lsan-virtual-address-space.h + .cc", "VAS 的 LSan 根区域登记装饰器")}
//! v8 侧: override ${overrides.length}; 一行转发 ${inlineFwd.length}
//! 语义: AllocatePages 登记根 (__lsan_register_root_region), FreePages 注销; 其余转发 vas_

/// v8: class LsanVirtualAddressSpace final — 装饰 v8::VirtualAddressSpace
pub struct LsanVirtualAddressSpace<'a> {
    /// v8: vas_
    vas: &'a mut dyn VirtualAddressSpace,
}

impl<'a> LsanVirtualAddressSpace<'a> {
    pub fn new(vas: &'a mut dyn VirtualAddressSpace) -> Self { Self { vas } }

    /// v8: AllocatePages — 分配成功后登记 LSan 根
    pub fn allocate_pages(&mut self, hint: usize, size: usize, alignment: usize, permissions: PagePermissions) -> usize { todo!() }
    /// v8: FreePages — 注销根
    pub fn free_pages(&mut self, address: usize, size: usize) { todo!() }
    /// v8: AllocateSubspace — 递归包 LsanVirtualAddressSpace
    pub fn allocate_subspace(&mut self, hint: usize, size: usize, alignment: usize, max: PagePermissions) { todo!() }
}

// ---- 一行转发面 (v8 原文) ----
${inlineFwd.map((f) => `/// v8: ${f.name} — ${f.body.replace(/\s+/g, " ").replace(/return |;$/g, "")}`).join("\n")}

// ---- override 对账 (${overrides.length} 项) ----
// ${overrides.join(", ")}
// 依赖 trait: VirtualAddressSpace (virtual_address_space.rs.skel)
`;

writeSrc("v8-base", "sanitizer/lsan_virtual_address_space.rs.skel", body);
console.log(`sanitizer/lsan-virtual-address-space.h.js: overrides=${overrides.length} inlineFwd=${inlineFwd.length}`);
