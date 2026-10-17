#!/usr/bin/env bun
// tool/src/base/threaded-list.h.js — 覆盖 v8/src/base/threaded-list.h
// 解析 ThreadedListBase<T> 头尾指针布局、TLTraits 适配面与 16 个链表操作方法
// → v8-base/src/threaded_list.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/threaded-list.h");

for (const anchor of [
  "V8_BASE_THREADED_LIST_H_",
  "class ThreadedListBase final : public BaseClass {",
  "ThreadedListBase() : head_(nullptr), tail_(&head_) {}",
  "void Add(T* v) {",
  "void DropHead() {",
  "bool Remove(T* v) {",
  "Iterator RemoveAt(Iterator it) {",
  "inline void EnsureValidTail() const {",
  "T* head_;\n  mutable T** tail_;",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 方法面 (void/bool/int + Iterator 系)
const ops = [...new Set([
  ...[...h.matchAll(/^  (?:V8_INLINE )?(?:void|bool|int|Iterator|ConstIterator) (\w+)\(/gm)].map((m) => m[1]),
  ...[...h.matchAll(/^  (?:Iterator|ConstIterator) (begin|end)\(\)/gm)].map((m) => m[1]),
])];
// 真解析: traits 与别名
const aliases = [...h.matchAll(/^using (\w+) = ThreadedListBase</gm)].map((m) => m[1]);
const hasUnsafe = h.includes("kUnsafeInsertion = true");

const body = `${banner("src/base/threaded-list.h.js", "src/base/threaded-list.h", "穿透式侵入链表 (节点自带 next 指针)")}
//! v8 侧: 方法 ${ops.length}; 别名 ${aliases.length}; kSupportsUnsafeInsertion=${hasUnsafe}
//! Rust 策略: 节点 trait ThreadedNode { fn next(&mut self) -> &mut Option<NonNull<T>> };
//! 链表持 head: Option<NonNull<T>> + tail 二级指针语义 (Rust 用 tail 节点引用或重扫)

/// v8: TLTraits — 默认要求节点有 T** next() (next 值即下一节点指针)
/// Rust: trait 暴露 next 链接单元
pub trait ThreadedListTraits {
    fn threaded_next(this: &mut Self) -> &mut Option<core::ptr::NonNull<Self>>;
}

/// v8: class ThreadedListBase<T, BaseClass, TLTraits, kSupportsUnsafeInsertion>
/// 布局: head_ (T*) + tail_ (T**, 指向最后节点 next 槽或 head_)
/// kSupportsUnsafeInsertion: 允许只改 next 不改 list 对象的插入 (tail 可能失效, EnsureValidTail 重扫)
pub struct ThreadedList<T> {
    /// v8: head_
    head: Option<core::ptr::NonNull<T>>,
    /// v8: mutable T** tail_ (const 迭代也要能修 tail)
    len: usize,
}

impl<T> ThreadedList<T> {
    /// v8: Add(v) — 尾插; AddFront(v) — 头插
    pub fn add(&mut self, v: T) { todo!() }
    pub fn add_front(&mut self, v: T) { todo!() }
    /// v8: DropHead() — 去头 (仅头有效)
    pub fn drop_head(&mut self) { todo!() }
    /// v8: Contains(v) — 线性扫
    pub fn contains(&self, v: &T) -> bool { todo!() }
    /// v8: Append(list&&) / Prepend(list&&) — 整表拼接
    pub fn append(&mut self, list: Self) { todo!() }
    pub fn prepend(&mut self, list: Self) { todo!() }
    /// v8: TruncateAt(rem, v) — v 处切断, 余段进 rem
    pub fn truncate_at(&mut self, rem: &mut Self, v: &T) { todo!() }
    /// v8: Clear() — 清空 (不清节点)
    pub fn clear(&mut self) { todo!() }
    /// v8: Remove(v) — 删节点返回是否删到
    pub fn remove(&mut self, v: &T) -> bool { todo!() }
    /// v8: RemoveAt(it) — 迭代器删, 返回下一个
    pub fn remove_at(&mut self, it: usize) -> usize { todo!() }
    /// v8: Rewind(reset_point) — 回卷到迭代点
    pub fn rewind(&mut self, reset_point: usize) { todo!() }
    /// v8: MoveTail(from_list, from_location) — 搬尾段
    pub fn move_tail(&mut self, from_list: &mut Self, from_location: usize) { todo!() }
    /// v8: is_empty() / begin()/end() 迭代 / LengthForTest / Verify
    pub fn is_empty(&self) -> bool { self.head.is_none() }
}

// ---- v8 方法对账 (解析 ${ops.length} 项) ----
// ${ops.join(", ")}
// 别名: ${aliases.map((a) => `${a} = ThreadedListBase<...>`).join("; ")} (unsafe 版带 kUnsafeInsertion)
`;

writeSrc("v8-base", "threaded_list.rs.skel", body);
console.log(`threaded-list.h.js: ops=${ops.length} aliases=${aliases.length} unsafe=${hasUnsafe}`);
