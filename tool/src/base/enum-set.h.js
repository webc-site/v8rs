#!/usr/bin/env bun
// tool/src/base/enum-set.h.js — 族: src/base/enum-set.h
// 覆盖: EnumSet<E,T> 位集 (17 方法 + 12 运算符 + Mask 位掩码) + operator<<
// → v8-base/src/enum_set.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/enum-set.h");

// —— 锚点断言 ——
if (!src.includes("template <class E, class T = int>\nclass EnumSet {")) throw new Error("缺 EnumSet 模板");
if (!src.includes("using StorageType = T;")) throw new Error("缺 StorageType");
if (!src.includes("static_assert(std::is_enum_v<E>")) throw new Error("缺 enum 约束");

// —— 真解析: 方法与运算符对数 ——
const methods = ["empty", "contains(E", "contains_all", "contains_any", "contains_only", "is_subset_of",
  "Add(E", "Add(EnumSet", "Remove(E", "Remove(EnumSet", "RemoveAll", "Intersect", "ToIntegral", "FromIntegral", "Mask"];
for (const m of methods) if (!src.includes(m)) throw new Error(`缺 ${m}`);
const binOps = (src.match(/^  constexpr EnumSet operator[|&-]\(/gm) || []).length; // |,&,- 各 2 (set/element)
if (binOps !== 6) throw new Error(`constexpr 运算符 ${binOps} != 6`);
const assignOps = (src.match(/^  EnumSet& operator\|=/gm) || []).length;
const assignTotal = (src.match(/^  EnumSet& operator/gm) || []).length;
if (assignTotal !== 6) throw new Error(`赋值运算符 ${assignTotal} != 6`);
if (!src.includes("constexpr EnumSet operator~() const")) throw new Error("缺取反");
if (!src.includes("constexpr auto operator<=>(const EnumSet&) const = default;")) throw new Error("缺三路比较");
if (!src.includes("operator<<(std::ostream& os, EnumSet<E, T> set)")) throw new Error("缺流输出");
if (!src.includes("bits::CountTrailingZerosNonZero(bits)")) throw new Error("缺 CTZ 逐元素打印");
console.log(`enum-set: 方法 ${methods.length}, 运算符 13 (二元 6 + 赋值 6 + ~), 比较 1, 流输出 1`);

let rust = banner(
  "src/base/enum-set.h.js",
  "src/base/enum-set.h",
  "枚举位集 (bitset of enum, 无显式值枚举按序数占位)",
);

rust += `
//! v8: src/base/enum-set.h

/// v8: src/base/enum-set.h EnumSet<E, T = int>
/// Mask(e) = T{1} << e (DCHECK e < sizeof(T)*8); bits_ 默认 0
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EnumSet<E, T = i32> {
    bits: T,
    _marker: std::marker::PhantomData<E>,
}

impl<E: Into<usize>, T: Bits> EnumSet<E, T> {
    /// v8: EnumSet(std::initializer_list<E>) — 逐个 |= Mask(e)
    pub const fn from_iter(init: impl Iterator<Item = E>) -> Self;
    /// v8: empty — bits_ == 0
    pub const fn is_empty(&self) -> bool;
    /// v8: contains(E)
    pub fn contains(&self, element: E) -> bool;
    /// v8: contains_all — (bits & set.bits) == set.bits
    pub fn contains_all(&self, set: Self) -> bool;
    /// v8: contains_any — (bits & set.bits) != 0
    pub fn contains_any(&self, set: Self) -> bool;
    /// v8: contains_only — bits == Mask(element)
    pub fn contains_only(&self, element: E) -> bool;
    /// v8: is_subset_of — (bits & set.bits) == bits
    pub fn is_subset_of(&self, set: Self) -> bool;
    /// v8: Add(E) / Add(EnumSet)
    pub fn add(&mut self, element: E);
    pub fn add_set(&mut self, set: Self);
    /// v8: Remove(E) / Remove(EnumSet) / RemoveAll
    pub fn remove(&mut self, element: E);
    pub fn remove_set(&mut self, set: Self);
    pub fn remove_all(&mut self);
    /// v8: Intersect — bits &= set.bits
    pub fn intersect(&mut self, set: Self);
    /// v8: ToIntegral
    pub const fn to_integral(self) -> T;
    /// v8: FromIntegral
    pub const fn from_integral(bits: T) -> Self;
}
// v8: 运算符面: | & - 与 ~ 及对应赋值 (EnumSet 与 E 两套操作数), Rust 侧由 BitOr/BitAnd/Not trait 实现, 登记跳过
// v8: operator<< — "{e1, e2}" 逐元素 (CTZNonZero 逐位摘除), Rust 侧 Debug/Display 补
`;

writeSrc("v8-base", "enum_set.rs.skel", rust);
console.log("written v8-base/src/enum_set.rs.skel");
