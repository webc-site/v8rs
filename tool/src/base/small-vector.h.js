#!/usr/bin/env bun
// tool/src/base/small-vector.h.js — 覆盖 v8/src/base/small-vector.h
// 解析 SmallVector<T, kSize> 三指针布局 (begin_/end_/end_of_storage_)、inline 存储联合
// 与 grow 策略、变更方法族 (push/pop/insert/erase/resize/reserve)
// → v8-base/src/small_vector.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/small-vector.h");

for (const anchor of [
  "V8_BASE_SMALL_VECTOR_H_",
  "class SmallVector {",
  "static constexpr size_t kInlineSize = kSize;",
  "static constexpr bool kHasTrivialElement =",
  "~SmallVector() { FreeStorage(); }",
  "void push_back(T x) {",
  "void pop_back(size_t count = 1) {",
  "V8_NOINLINE V8_PRESERVE_MOST void Grow() { Grow(0); }",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 构造器
const ctors = (h.match(/^  (?:explicit )?(?:V8_INLINE )?SmallVector\(/gm) || []).length;
// 真解析: 变更/查询方法
const mutators = [...new Set([...h.matchAll(/^  (?:V8_INLINE |V8_NOINLINE |void |T\* |size_t |bool |auto )(?:size_t |const T& )?(\w+)\(/gm)].map((m) => m[1]))];
// 真解析: 布局成员
const layoutMembers = [...h.matchAll(/^  (begin_|end_|end_of_storage_|allocator_);/gm)].map((m) => m[1]);
// 真解析: grow 相关
const growFns = [...new Set([...h.matchAll(/^  V8_(?:NO)?INLINE(?: V8_PRESERVE_MOST)? (?:void|T\*) (grow\w+|\w+_unchecked)\(/gm)].map((m) => m[1]))];

const body = `${banner("src/base/small-vector.h.js", "src/base/small-vector.h", "内联优先小向量 (inline 存储 + 溢出转堆)")}
//! v8 侧: 构造 ${ctors}; 方法 ${mutators.length}; 布局成员 ${layoutMembers.length}; grow 族 ${growFns.length}
//! Rust 策略: enum { Inline(MaybeUninit<[T; kSize]>, len), Heap(Box<RawVec>)} 或 ArrayVec crate 对齐

/// v8: src/base/small-vector.h class SmallVector<T, kSize, Allocator>
/// v8 布局: begin_/end_/end_of_storage_ 三指针; 小容量时指向 inline_[] 数组
/// v8: kHasTrivialElement = trivially_copyable && trivially_destructible (拷贝路径分派)
pub struct SmallVector<T, const K_SIZE: usize> {
    /// v8: begin_..end_of_storage_ (当前 len/cap 由指针差导出)
    inline_storage: [core::mem::MaybeUninit<T>; K_SIZE],
    /// v8: 溢出后 begin_ 指向堆
    heap: Option<Box<[core::mem::MaybeUninit<T>]>>,
    len: usize,
    /// v8: inline 容量 kInlineSize = kSize
    pub inline_capacity: usize,
}

impl<T, const K_SIZE: usize> SmallVector<T, K_SIZE> {
    /// v8: SmallVector(size) — 默认初始化 (注意非值初始化)
    /// v8: SmallVector(size, initial_value) — 值初始化版
    /// v8: SmallVector(initializer_list / Vector<const T>)
    /// 对账: 构造 ${ctors} 个 (含拷贝/移动 ×2)
    pub fn new() -> Self { todo!() }

    /// v8: push_back(x) — 满 then grow_and_push_back
    pub fn push_back(&mut self, x: T) { todo!() }
    /// v8: emplace_back(args...) — 就地构造
    pub fn emplace_back(&mut self, x: T) { todo!() }
    /// v8: pop_back(count=1)
    pub fn pop_back(&mut self, count: usize) { todo!() }
    /// v8: insert(pos, value|count,value|range|init_list|container) — 4 重载
    pub fn insert(&mut self, pos: usize, x: T) { todo!() }
    /// v8: erase(first, last) / erase(pos) — 返回删除后 pos 迭代器
    pub fn erase(&mut self, start: usize, end: usize) { todo!() }
    /// v8: resize(new_size) / resize(new_size, init) / resize_no_init
    pub fn resize(&mut self, new_size: usize) { todo!() }
    /// v8: reserve(new_capacity) — 仅扩容
    pub fn reserve(&mut self, new_capacity: usize) { todo!() }
    /// v8: clear() — 析构元素, 容量保留
    pub fn clear(&mut self) { todo!() }

    pub fn size(&self) -> usize { self.len }
    pub fn empty(&self) -> bool { self.len == 0 }
    /// v8: capacity() = end_of_storage_ - begin_
    pub fn capacity(&self) -> usize { todo!() }
}

// ---- v8 方法对账 (解析 ${mutators.length} 项) ----
// ${mutators.join(", ")}

// ---- grow 族 (v8: 容量翻倍, V8_PRESERVE_MOST 冷路径) ----
// ${growFns.join(", ")}
// 语义: Grow(0) 容量翻倍; grow_and_emplace_back/grow_and_push_back — Developer 补
`;

writeSrc("v8-base", "small_vector.rs.skel", body);
console.log(`small-vector.h.js: ctors=${ctors} methods=${mutators.length} layout=${layoutMembers.length} grow=${growFns.length}`);
