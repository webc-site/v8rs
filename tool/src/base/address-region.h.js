#!/usr/bin/env bun
// tool/src/base/address-region.h.js — 族: src/base/address-region.h
// 覆盖: AddressRegion 类 (地址区间) 10 方法 + AddressRegionOf 2 模板
// → v8-base/src/address_region.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/address-region.h");

// —— 锚点断言 ——
const anchors = [
  "class AddressRegion {",
  "using Address = uintptr_t;",
  "constexpr Address begin() const",
  "constexpr Address end() const",
  "constexpr size_t size() const",
  "constexpr bool is_empty() const",
  "constexpr bool contains(Address address) const",
  "constexpr base::AddressRegion GetOverlap(AddressRegion region) const",
  "ASSERT_TRIVIALLY_COPYABLE(AddressRegion);",
];
for (const a of anchors) if (!src.includes(a)) throw new Error(`address-region.h 缺锚点: ${a}`);

// —— 真解析: 收集全部 constexpr 方法签名, 对数 ——
const methods = [...src.matchAll(/constexpr (\w[\w:<>* ]*?) (\w+)\(([^)]*)\) const/g)].map(
  (m) => `${m[1].trim()} ${m[2]}(${m[3].replace(/\s+/g, " ").trim()})`,
);
if (methods.length !== 8) throw new Error(`constexpr 方法数 ${methods.length} != 8`);
const hasSetSize = src.includes("void set_size(size_t size)");
const hasRegionOf = (src.match(/AddressRegionOf\(/g) || []).length >= 3; // 2 定义 + 1 decltype 引用
if (!hasSetSize || !hasRegionOf) throw new Error("缺 set_size 或 AddressRegionOf");
console.log(`address-region: constexpr 方法 ${methods.length}, set_size 1, AddressRegionOf 2`);

let rust = banner(
  "src/base/address-region.h.js",
  "src/base/address-region.h",
  "地址区间 [address, address+size) 表示与包含/交叠判定",
);

rust += `
//! v8: src/base/address-region.h

/// v8: src/base/address-region.h AddressRegion::Address
pub type Address = usize;

/// v8: src/base/address-region.h AddressRegion
/// 原版字段 address_: Address = 0, size_: size_t = 0; trivially copyable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AddressRegion {
    address: Address,
    size: usize,
}

impl AddressRegion {
    /// v8: src/base/address-region.h AddressRegion(Address, size_t)
    pub const fn new(address: Address, size: usize) -> Self;

    /// v8: src/base/address-region.h begin
    pub const fn begin(&self) -> Address;

    /// v8: src/base/address-region.h end — address_ + size_ (可能回绕, 原版即如此)
    pub const fn end(&self) -> Address;

    /// v8: src/base/address-region.h size
    pub const fn size(&self) -> usize;

    /// v8: src/base/address-region.h set_size
    pub fn set_size(&mut self, size: usize);

    /// v8: src/base/address-region.h is_empty — size_ == 0
    pub const fn is_empty(&self) -> bool;

    /// v8: src/base/address-region.h contains(Address)
    /// 原版用无符号回绕技巧: (address - begin()) < size()
    pub const fn contains(&self, address: Address) -> bool;

    /// v8: src/base/address-region.h contains(Address, size_t)
    pub const fn contains_range(&self, address: Address, size: usize) -> bool;

    /// v8: src/base/address-region.h contains(AddressRegion)
    pub const fn contains_region(&self, region: AddressRegion) -> bool;

    /// v8: src/base/address-region.h GetOverlap
    pub const fn get_overlap(&self, region: AddressRegion) -> AddressRegion;
}

/// v8: src/base/address-region.h AddressRegionOf(T*, size_t)
/// 指针版本: reinterpret_cast 到 Address, 字节数 = sizeof(T) * size
pub fn address_region_of<T>(ptr: *const T, size: usize) -> AddressRegion;

/// v8: src/base/address-region.h AddressRegionOf(Container&&)
/// 容器版本: 取 data()/size()
pub fn address_region_of_container<C>(c: &C) -> AddressRegion;
`;

writeSrc("v8-base", "address_region.rs.skel", rust);
console.log("written v8-base/src/address_region.rs.skel");
