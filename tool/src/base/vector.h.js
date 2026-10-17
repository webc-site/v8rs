#!/usr/bin/env bun
// tool/src/base/vector.h.js — 覆盖 v8/src/base/vector.h
// 解析 Vector<T> (span 包装) 构造族/视图方法/比较与 hash_value 自由函数
// → v8-base/src/vector.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/vector.h");

for (const anchor of [
  "V8_BASE_VECTOR_H_",
  "class Vector final {",
  "constexpr Vector(T* data V8_LIFETIME_BOUND, size_t length)",
  "constexpr Vector(std::span<U, n> span) : span_(span) {}",
  "constexpr Vector<T> SubVector(size_t from, size_t to) const {",
  "constexpr size_t size() const { return span_.size(); }",
  "bool operator==(const Vector<T>& other) const {",
  "V8_INLINE size_t hash_value(base::Vector<T> v) {",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 构造器
const ctors = [...h.matchAll(/^  (?:template <[\s\S]*?>\n\s*)?constexpr Vector\(([^)]*)/gm)].map((m) => m[1].split(",")[0].trim()).filter(Boolean);
// 真解析: 方法面
const methods = [...new Set([...h.matchAll(/^  (?:constexpr|V8_INLINE|bool|int|void|template) (?:[\w:<>*& ]*?)?(\w+)\(/gm)].map((m) => m[1]))]
  .filter((n) => !["Vector"].includes(n));
// 真解析: ranges 特化
const hasRanges = h.includes("std::ranges::enable_view");

const body = `${banner("src/base/vector.h.js", "src/base/vector.h", "非拥有视图 Vector (span 包装, ranges view/borrowed_range)")}
//! v8 侧: 构造 ${ctors.length}; 方法/算子 ${methods.length}; ranges 特化 ${hasRanges ? 2 : 0}
//! Rust 策略: &[T] 切片原生等价; 保留薄 newtype 对齐 v8 调用面 (at/front/back/first/last/SubVector)

/// v8: src/base/vector.h class Vector<T> final (内包 std::span)
/// v8: enable_view + enable_borrowed_range 特化 → Rust 切片天然是视图+借用
pub struct Vector<'a, T> {
    /// v8: span_
    span: &'a mut [T],
}

impl<'a, T> Vector<'a, T> {
    /// v8: Vector(data, length) — DCHECK(length==0 || data != nullptr)
    pub fn new(data: &'a mut [T]) -> Self { Self { span: data } }

    /// v8: SubVector(from, to) / SubVectorFrom(from) — [from, to) 子视图
    pub fn sub_vector(&mut self, from: usize, to: usize) -> Vector<'a, T> {
        debug_assert!(from <= to);
        todo!()
    }
    /// v8: SubVectorFrom(from)
    pub fn sub_vector_from(&mut self, from: usize) -> Vector<'a, T> { todo!() }

    /// v8: first(count) / last(count)
    pub fn first(&self, count: usize) -> &[T] { &self.span[..count] }
    pub fn last(&self, count: usize) -> &[T] { &self.span[self.span.len() - count..] }

    /// v8: length() — int 返回 (CHECK <= INT_MAX); size() — size_t
    pub fn length(&self) -> i32 { self.span.len() as i32 }
    pub fn size(&self) -> usize { self.span.len() }
    pub fn empty(&self) -> bool { self.span.is_empty() }

    /// v8: operator[] (DCHECK 界检) / at / front / back / begin / end / data
    pub fn at(&self, i: usize) -> &T { &self.span[i] }
    pub fn front(&self) -> &T { &self.span[0] }
    pub fn back(&self) -> &T { &self.span[self.span.len() - 1] }

    /// v8: OverwriteWith(other) — DCHECK 等长后 Copy
    pub fn overwrite_with(&mut self, other: &[T]) where T: Copy {
        debug_assert_eq!(self.size(), other.len());
        self.span.copy_from_slice(other);
    }

    /// v8: Truncate(length) / DropFront(count) / DropBack() — span_ 就地收缩
    pub fn truncate(&mut self, length: usize) { self.span = &mut core::mem::take(&mut self.span)[..length]; }
}

// ---- v8 方法面对账 (解析 ${methods.length} 项) ----
// ${methods.join(", ")}
// 语义: begin/end/cbegin/cend/rbegin/rend → 切片迭代器; operator Vector<const U> → 重新借用;
// operator== → 切片逐元相等; hash_value → 切片 hash (v8 用 base::hash_range)

/// v8: hash_value(Vector<T>) — 自由函数
pub fn hash_value<T: Hash>(v: &[T], state: &mut impl core::hash::Hasher) {
    use core::hash::Hash;
    v.len().hash(state);
    for e in v { e.hash(state); }
}
`;

writeSrc("v8-base", "vector.rs.skel", body);
console.log(`vector.h.js: ctors=${ctors.length} methods=${methods.length} ranges=${hasRanges ? 2 : 0}`);
