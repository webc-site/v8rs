#!/usr/bin/env bun
// tool/src/base/doubly-threaded-list.h.js — 族: src/base/doubly-threaded-list.h
// 覆盖: DoublyThreadedListTraits + DoublyThreadedList (prev 存前驱 next 域地址的侵入式双链)
// → v8-base/src/doubly_threaded_list.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/doubly-threaded-list.h");

// —— 锚点断言 ——
if (!src.includes("struct DoublyThreadedListTraits {")) throw new Error("缺 Traits");
if (!src.includes("class DoublyThreadedList {")) throw new Error("缺链表类");
if (!src.includes("class end_iterator {};")) throw new Error("缺哨兵 end_iterator");
if (!src.includes("static void Remove(T x) {")) throw new Error("缺静态 Remove");
if (!src.includes("*DTLTraits::prev(x) = nullptr;")) throw new Error("缺 Remove 收尾语义");

// —— 真解析: 类方法面, 对数 ——
const expected = [
  "Remove", "PushFront", "Front", "PopFront", "empty", "begin", "end",
  "RemoveAt", "Contains", "ContainsSlow",
];
for (const m of expected) {
  const re = new RegExp(`(?:static )?(?:void|T|bool|iterator|end_iterator) ${m}\\(`);
  if (!re.test(src)) throw new Error(`缺方法 ${m}`);
}
// Traits 3 钩子
for (const t of ["static T** prev(T t)", "static T* next(T t)", "static bool non_empty(T t)"]) {
  if (!src.includes(t)) throw new Error(`缺 Traits 钩子 ${t}`);
}
console.log(`doubly-threaded-list: Traits 钩子 3, 类方法 ${expected.length}, 迭代器 1 (前向)`);

let rust = banner(
  "src/base/doubly-threaded-list.h.js",
  "src/base/doubly-threaded-list.h",
  "侵入式双链 (prev 指向前驱的 next 域, 删头无需特判)",
);

rust += `
//! v8: src/base/doubly-threaded-list.h

/// v8: src/base/doubly-threaded-list.h DoublyThreadedListTraits<T>
/// prev(t) 返回 T** (前驱 next 域地址), next(t) 返回 T*, non_empty(t) = t != nullptr
pub trait DoublyThreadedListTraits {
    type Node;
    /// v8: DoublyThreadedListTraits::prev — t->prev()
    fn prev(t: &Self::Node) -> &mut Option<Self::Node>;
    /// v8: DoublyThreadedListTraits::next — t->next()
    fn next(t: &Self::Node) -> &mut Option<Self::Node>;
    /// v8: DoublyThreadedListTraits::non_empty
    fn non_empty(t: &Self::Node) -> bool;
}

/// v8: src/base/doubly-threaded-list.h DoublyThreadedList<T, DTLTraits>
/// 不变量: 节点 prev = 前驱的 next 域地址 → Remove 不需表头; 已删节点 prev = nullptr
/// Rust 侧建议链域用 Option<NonNull<T>>, Developer 定别名/借用形态
pub struct DoublyThreadedList<T> { head: Option<T> }

impl<T> DoublyThreadedList<T> {
    /// v8: DoublyThreadedList::Remove(x) — 静态; 已摘除 (*prev(x)==nullptr) 直接返回
    pub fn remove(x: &mut T);
    /// v8: DoublyThreadedList::PushFront — 头插, 不影响既有迭代器
    pub fn push_front(&mut self, x: T);
    /// v8: DoublyThreadedList::Front — 空表 DCHECK
    pub fn front(&self) -> &T;
    /// v8: DoublyThreadedList::PopFront — Remove(Front())
    pub fn pop_front(&mut self);
    /// v8: DoublyThreadedList::empty
    pub fn is_empty(&self) -> bool;
    /// v8: DoublyThreadedList::RemoveAt(it) — 摘除并返回下一迭代器
    pub fn remove_at(&mut self, it: Iterator) -> Iterator;
    /// v8: DoublyThreadedList::Contains — DTLTraits::in_use 快查 (原版依赖外部 in_use 钩子)
    pub fn contains(&self, needle: &T) -> bool;
    /// v8: DoublyThreadedList::ContainsSlow — 全表线性扫
    pub fn contains_slow(&self, needle: &T) -> bool;
    /// v8: DoublyThreadedList::begin/end — 前向迭代, end 为哨兵类型 (Rust: iterator 终态 None)
    pub fn iter(&self) -> impl Iterator<Item = &T>;
}

// v8: 移动构造: 搬 head_ 并把 next(head_) 的 prev 改指新表头 — Rust move 语义天然正确, 登记跳过
`;

writeSrc("v8-base", "doubly_threaded_list.rs.skel", rust);
console.log("written v8-base/src/doubly_threaded_list.rs.skel");
