#!/usr/bin/env bun
// tool/src/base/bits-iterator.h.js — 族: src/base/bits-iterator.h
// 覆盖: BitsIterator 前向迭代器 (LSB/MSB 两序) + IterateBits/IterateBitsBackwards
// → v8-base/src/bits_iterator.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/bits-iterator.h");

// —— 锚点断言 ——
if (!src.includes("class BitsIterator : public iterator<std::forward_iterator_tag, int>")) {
  throw new Error("缺 BitsIterator 类");
}
if (!src.includes("kMSBFirst ? 8 * sizeof(T) - 1 - CountLeadingZeros(bits_)")) throw new Error("缺 MSB 解引用");
if (!src.includes(": CountTrailingZeros(bits_)")) throw new Error("缺 LSB 解引用");
if (!src.includes("bits_ &= ~(T{1} << **this);")) throw new Error("缺自增清除位语义");
if (!src.includes("auto IterateBits(T bits)")) throw new Error("缺 IterateBits");
if (!src.includes("auto IterateBitsBackwards(T bits)")) throw new Error("缺 IterateBitsBackwards");

// —— 真解析: 模板参数与哨兵 ——
const tmpl = src.match(/template <typename T, bool kMSBFirst = false>/);
if (!tmpl) throw new Error("缺模板默认参 kMSBFirst=false");
console.log("bits-iterator: 类 1 (双序迭代器), 工厂 2, 运算符 3 (*/++/==)");

let rust = banner(
  "src/base/bits-iterator.h.js",
  "src/base/bits-iterator.h",
  "置位位迭代器 (LSB→MSB / MSB→LSB)",
);

rust += `
//! v8: src/base/bits-iterator.h

/// v8: src/base/bits-iterator.h BitsIterator<T, kMSBFirst>
/// operator* : kMSBFirst ? sizeof(T)*8-1-CLZ(bits) : CTZ(bits)
/// operator++: bits &= ~(1 << *self)  (清除当前位, 0 为终哨兵)
/// Rust 侧语义等价物 (供 Developer 采纳):
///   (0..bits.count_zeros() 无关; 正确为) 遍历 bits.trailing_zeros() 循环清除
pub struct BitsIterator<T> {
    bits: T,
    msb_first: bool,
}

impl<T: PrimInt> Iterator for BitsIterator<T> {
    type Item = u32;
    /// v8: operator* — MSB 序取 8*sizeof(T)-1-CountLeadingZeros, LSB 序取 CountTrailingZeros
    fn next(&mut self) -> Option<u32>;
}

/// v8: src/base/bits-iterator.h IterateBits — LSB → MSB 的置位位序
pub fn iterate_bits<T: PrimInt>(bits: T) -> impl Iterator<Item = u32>;

/// v8: src/base/bits-iterator.h IterateBitsBackwards — MSB → LSB 的置位位序
pub fn iterate_bits_backwards<T: PrimInt>(bits: T) -> impl Iterator<Item = u32>;
`;

writeSrc("v8-base", "bits_iterator.rs.skel", rust);
console.log("written v8-base/src/bits_iterator.rs.skel");
