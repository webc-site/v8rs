#!/usr/bin/env bun
// tool/src/base/container-utils.h.js — 族: src/base/container-utils.h
// 覆盖: detail::has_find 探测 + 13 个容器算法模板 (contains/index_of/erase_at/...)
// → v8-base/src/container_utils.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/container-utils.h");

// —— 真解析: 全部顶层模板函数, 对数 ——
const fns = [...src.matchAll(/^(?:template <[^>]+>\n)?(?:inline )?[\w:<>, &]+ (\w+)\(/gm)].map((m) => m[1]);
const counts = {};
for (const f of fns) counts[f] = (counts[f] || 0) + 1;
const expectedCounts = {
  contains: 1, index_of: 1, index_of_if: 1, erase_at: 1, erase_if: 1, count_if: 1,
  all_of: 2, any_of: 2, none_of: 1, sort: 2, all_equal: 2, vector_append: 1,
};
for (const [name, n] of Object.entries(expectedCounts)) {
  if (counts[name] !== n) throw new Error(`${name} 重载数 ${counts[name] ?? 0} != ${n}`);
}
const total = Object.values(expectedCounts).reduce((a, b) => a + b, 0);

// —— 锚点断言 ——
if (!src.includes("struct has_find : std::false_type {};")) throw new Error("缺 has_find 探测");
if (!src.includes("constexpr bool has_find_v =")) throw new Error("缺 has_find_v");
if (!src.includes("if (std::size(container) <= index) return 0;")) throw new Error("缺 erase_at 越界语义");
if (!src.includes("if (std::size(container) <= 1) return true;")) throw new Error("缺 all_equal 短路语义");
console.log(`container-utils: 模板函数 ${total} (含重载), has_find 探测 1`);

let rust = banner(
  "src/base/container-utils.h.js",
  "src/base/container-utils.h",
  "容器算法泛化薄层 (std 算法 + find 成员探测)",
);

rust += `
//! v8: src/base/container-utils.h
//! Rust 侧: 迭代器/切片原生覆盖大部分; 保留 v8 API 面登记

// v8: detail::has_find<T, U> / has_find_v — 探测 container.find(u)
// Rust: trait HasFind { fn find(&self, u: &U) -> Option<usize> }, 探测由 trait 实现表达

/// v8: container-utils.h contains — 有 find 用 find, 否则 std::find
pub fn contains<C: Find<T>, T: PartialEq>(container: &C, element: &T) -> bool;

/// v8: container-utils.h index_of — 首个匹配下标, 无则 None
pub fn index_of<C, T: PartialEq>(container: &C, element: &T) -> Option<usize>;

/// v8: container-utils.h index_of_if — 首个满足谓词下标
pub fn index_of_if<C, P: Fn(&C::Item) -> bool>(container: &C, predicate: P) -> Option<usize>;

/// v8: container-utils.h erase_at(container, index, count=1)
/// 语义: index 越界返回 0; count 截到尾部; 返回实际删除数
/// Rust: Vec::drain(index..index+count).count() 变体, Developer 补切片版
pub fn erase_at<T>(container: &mut Vec<T>, index: usize, count: usize) -> usize;

/// v8: container-utils.h erase_if — 删全部满足谓词元素, 返回删除数 (= std::vec::retain 双计数)
pub fn erase_if<T>(container: &mut Vec<T>, predicate: impl FnMut(&T) -> bool) -> usize;

/// v8: container-utils.h count_if
pub fn count_if<C, P: Fn(&C::Item) -> bool>(container: &C, predicate: P) -> usize;

/// v8: container-utils.h all_of (谓词版 / bool 转换版 2 重载)
pub fn all_of<C, P: Fn(&C::Item) -> bool>(container: &C, predicate: P) -> bool;
pub fn all_of_bools<C: IntoIterator<Item: Into<bool>>>(container: C) -> bool;

/// v8: container-utils.h any_of (2 重载)
pub fn any_of<C, P: Fn(&C::Item) -> bool>(container: &C, predicate: P) -> bool;
pub fn any_of_bools<C: IntoIterator<Item: Into<bool>>>(container: C) -> bool;

/// v8: container-utils.h none_of
pub fn none_of<C, P: Fn(&C::Item) -> bool>(container: &C, predicate: P) -> bool;

/// v8: container-utils.h sort (默认序 / 比较器 2 重载) — Rust sort_unstable_by_key 对应
pub fn sort<C: Sortable>(container: &mut C);
pub fn sort_by<C: Sortable, Comp: Fn(&C::Item, &C::Item) -> Ordering>(container: &mut C, comp: Comp);

/// v8: container-utils.h all_equal (无参: 全互相相等, size<=1 短路 true / 带值 2 重载)
pub fn all_equal<C: IntoIterator<Item: PartialEq>>(container: C) -> bool;
pub fn all_equal_to<C: IntoIterator, T: PartialEq>(container: C, value: &T) -> bool;

/// v8: container-utils.h vector_append — v.extend(container)
pub fn vector_append<V: Extend<C::Item>, C: IntoIterator>(v: &mut V, container: C);
`;

writeSrc("v8-base", "container_utils.rs.skel", rust);
console.log("written v8-base/src/container_utils.rs.skel");
