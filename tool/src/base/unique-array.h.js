#!/usr/bin/env bun
// tool/src/base/unique-array.h.js — 覆盖 v8/src/base/unique-array.h
// 解析 UniqueArray<T> 构造族 (New/New(init)/NewForOverwrite/CopiedFrom) 与视图方法
// → v8-base/src/unique_array.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/unique-array.h");

for (const anchor of [
  "V8_BASE_UNIQUE_ARRAY_H_",
  "class UniqueArray final {",
  "static UniqueArray<T> New(size_t size) {",
  "static UniqueArray<T> NewForOverwrite(size_t size) {",
  "static UniqueArray<T> CopiedFrom(",
  "UniqueArray(UniqueArray<U>&& other) V8_NOEXCEPT {",
  "constexpr size_t size() const { return length_; }",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 静态构造族
const ctors = [...h.matchAll(/^  static UniqueArray<T> (\w+)\(/gm)].map((m) => m[1]);
// 真解析: 视图/容量方法
const views = [...h.matchAll(/^  (?:constexpr|V8_INLINE) (?:[\w:<>*&]+ )(\w+)\(\)/gm)].map((m) => m[1]);

const body = `${banner("src/base/unique-array.h.js", "src/base/unique-array.h", "带长度独占数组 (unique_ptr<T[]> + size)")}
//! v8 侧: 静态构造 ${ctors.length}; 视图方法 ${new Set(views).size}
//! Rust 策略: Box<[T]> 原生等价 (ptr + len 胖指针); 保留 API 面做迁移对账

/// v8: src/base/unique-array.h class UniqueArray<T> final
/// v8 动机: unique_ptr<T[]> 不记 size; Rust Box<[T]> 胖指针自带
pub struct UniqueArray<T> {
    /// v8: data_ + length_
    data: Box<[T]>,
}

impl<T: Clone> UniqueArray<T> {
    /// v8: New(size) — 值初始化 (Rust: vec!宏填 T::default 后转 Box)
    pub fn new(size: usize) -> Self where T: Default { todo!() }
    /// v8: New(size, init) — fill_n 赋 init
    pub fn new_with(size: usize, init: T) -> Self { todo!() }
    /// v8: NewForOverwrite(size) — 默认初始化不填零 (Rust: MaybeUninit 语义, 安全侧退化为 new)
    pub fn new_for_overwrite(size: usize) -> Self where T: Default { todo!() }
    /// v8: CopiedFrom(data, size) / CopiedFrom(span)
    pub fn copied_from(data: &[T]) -> Self { todo!() }
}

impl<T> UniqueArray<T> {
    /// v8: size() / empty() / begin() / end() / operator[] / as_span()
    /// 对账 (解析 ${new Set(views).size} 项): ${[...new Set(views)].join(", ")}
    pub fn size(&self) -> usize { self.data.len() }
    pub fn empty(&self) -> bool { self.data.is_empty() }
    pub fn as_slice(&self) -> &[T] { &self.data }
    pub fn as_mut_slice(&mut self) -> &mut [T] { &mut self.data }
}
// v8: UniqueArray<U>&& 跨型移动 (T→const T 转换) — Rust 生命周期/可变性天然覆盖
`;

writeSrc("v8-base", "unique_array.rs.skel", body);
console.log(`unique-array.h.js: ctors=${ctors.length} views=${new Set(views).size}`);
