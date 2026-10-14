#!/usr/bin/env bun
// tool/src/base/intrusive-set.h.js — 族: src/base/intrusive-set.h
// 覆盖: IntrusiveSetIndex + IntrusiveSet (O(1) 增删查, swap-pop + index 回写)
// → v8-base/src/intrusive_set.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/intrusive-set.h");

// —— 锚点断言 ——
if (!src.includes("class IntrusiveSetIndex {")) throw new Error("缺 IntrusiveSetIndex");
if (!src.includes("static constexpr size_t kNotInSet = std::numeric_limits<size_t>::max();")) throw new Error("缺 kNotInSet");
if (!src.includes("class IntrusiveSet {")) throw new Error("缺 IntrusiveSet");
if (!src.includes("static_assert(std::is_trivially_copyable_v<T>);")) throw new Error("缺 trivially copyable 约束");

// —— 真解析: Remove swap-pop 语义锚 ——
if (!src.includes("Index(elements_.back()) = index;")) throw new Error("缺尾元素索引回写");
if (!src.includes("elements_[index] = elements_.back();")) throw new Error("缺 swap-pop");
if (!src.includes("elements_.pop_back();")) throw new Error("缺 pop_back");
if (!src.includes("bool Contains(T x) const { return Index(x) != IntrusiveSetIndex::kNotInSet; }")) throw new Error("缺 Contains");
// 迭代器: 记 last_index_location_ 检测当前元素被删
if (!src.includes("const size_t* last_index_location_ = nullptr;")) throw new Error("缺迭代器删除检测");
const methods = ["Contains", "Add", "Remove", "begin", "end"];
for (const m of methods) if (!src.includes(m)) throw new Error(`缺方法 ${m}`);
console.log(`intrusive-set: 类 2 (Index + Set), 方法 ${methods.length}, 复杂度 Add/Contains/Remove 均 O(1)`);

let rust = banner(
  "src/base/intrusive-set.h.js",
  "src/base/intrusive-set.h",
  "侵入式指针集 (元素自带位置索引, swap-pop 删除)",
);

rust += `
//! v8: src/base/intrusive-set.h

/// v8: src/base/intrusive-set.h IntrusiveSetIndex
/// value = kNotInSet (SIZE_MAX) 表示不在集合内
pub struct IntrusiveSetIndex { value: usize }

impl IntrusiveSetIndex {
    /// v8: kNotInSet = numeric_limits<size_t>::max()
    pub const NOT_IN_SET: usize = usize::MAX;
}

/// v8: src/base/intrusive-set.h IntrusiveSet<T, GetIntrusiveSetIndex, Container>
/// T 须 trivially copyable (指针类轻值); index_functor 产 IntrusiveSetIndex&
/// Add: Index(x) = size, push_back — 均摊 O(1), 迭代中可加 (会被遍历到)
/// Remove: 尾元素换位 + 索引回写 + pop — O(1); 迭代中仅可删当前元素
pub struct IntrusiveSet<T> { elements: Vec<T> }

impl<T> IntrusiveSet<T> {
    /// v8: Contains — Index(x) != kNotInSet
    pub fn contains(&self, x: &T) -> bool;
    /// v8: Add — DCHECK !Contains
    pub fn add(&mut self, x: T);
    /// v8: Remove — DCHECK Contains; DCHECK_EQ(x, elements_[index]); swap-pop
    pub fn remove(&mut self, x: &T);
    /// v8: iterator — operator* 先于 operator++ (记录 last_index_location_
    ///     检测当前元素被 swap 换走来跳过重复访问); Rust 侧 Developer 按此语义实现
    pub fn iter(&self) -> impl Iterator<Item = &T>;
}
`;

writeSrc("v8-base", "intrusive_set.rs.skel", rust);
console.log("written v8-base/src/intrusive_set.rs.skel");
