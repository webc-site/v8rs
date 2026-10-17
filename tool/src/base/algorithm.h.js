#!/usr/bin/env bun
// tool/src/base/algorithm.h.js — 族: src/base/algorithm.h
// 覆盖: internal::can_use_memcpy_v 变量模板 + Copy 双重载 (memcpy 快路径 / std::copy 慢路径)
// → v8-base/src/algorithm.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/algorithm.h");

// —— 锚点断言 ——
if (!src.includes("constexpr bool can_use_memcpy_v =")) throw new Error("缺 can_use_memcpy_v");
const copyDefs = [...src.matchAll(/^(?:constexpr )?OutputIt Copy\(/gm)];
if (copyDefs.length !== 2) throw new Error(`Copy 重载数 ${copyDefs.length} != 2`);
if (!src.includes("requires(internal::can_use_memcpy_v<InputIt, OutputIt>)")) throw new Error("缺快路径 requires");
if (!src.includes("requires(!internal::can_use_memcpy_v<InputIt, OutputIt>)")) throw new Error("缺慢路径 requires");
if (!src.includes("base::MemCopy(dest_ptr, src_ptr, count * sizeof(T));")) throw new Error("缺 MemCopy 委托");

// —— 真解析: can_use_memcpy_v 的 4 个合取条件 ——
const conjuncts = [
  "std::contiguous_iterator<InputIt>",
  "std::contiguous_iterator<OutputIt>",
  "std::is_same_v", // value_type 相同
  "std::is_trivially_copyable_v",
];
for (const c of conjuncts) if (!src.includes(c)) throw new Error(`can_use_memcpy_v 缺条件 ${c}`);
console.log("algorithm: Copy 重载 2, can_use_memcpy_v 合取条件 4");

let rust = banner(
  "src/base/algorithm.h.js",
  "src/base/algorithm.h",
  "std::copy 的 MemCopy 快路径分派",
);

rust += `
//! v8: src/base/algorithm.h

/// v8: src/base/algorithm.h internal::can_use_memcpy_v
/// 原版合取条件:
/// 1. InputIt/OutputIt 均 contiguous_iterator
/// 2. 两者 value_type 相同
/// 3. value_type trivially_copyable
/// Rust 侧: 切片 &[T] → &mut [T] 天然满足, 由 trait 约束 T: Copy 表达;
/// 布尔变量模板无直接对应, 登记为文档语义
///
/// v8: src/base/algorithm.h Copy (快路径, requires can_use_memcpy_v)
/// 原版: MemCopy(dest, src, count * sizeof(T)); 返回 d_first + count
/// Rust 侧快路径即 copy_from_slice; 注意原版语义禁重叠 (par_unseq 假设)
pub fn copy<'a, T: Copy>(src: &'a [T], dst: &'a mut [T]) -> usize;

/// v8: src/base/algorithm.h Copy (慢路径, requires !can_use_memcpy_v)
/// 原版: 委托 std::copy (逐元素); Rust 侧由迭代器版 clone 语义表达
pub fn copy_with<I: Iterator, O: Extend<I::Item>>(first: I, last_hint: usize, d_first: O) -> usize;
`;

writeSrc("v8-base", "algorithm.rs.skel", rust);
console.log("written v8-base/src/algorithm.rs.skel");
