#!/usr/bin/env bun
// tool/src/base/template-meta-programming/list.h.js — 覆盖 v8/src/base/template-meta-programming/list.h
// 解析 list/list1 双轨类型表、14 组 _v/_t 导出别名与 impl 结构全集
// → v8-base/src/template_meta_programming/list.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/template-meta-programming/list.h");

for (const anchor of [
  "V8_BASE_TEMPLATE_META_PROGRAMMING_LIST_H_",
  "struct list {};",
  "struct list1 {};",
  "struct length_impl;",
  "struct fold_right_impl;",
  "using insert_at1_t = insert_at1<List1, I, Ts...>::type;",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 容器
const containers = [...h.matchAll(/^struct (list1?) \{\};$/gm)].map((m) => m[1]);
// 真解析: impl 结构
const impls = [...new Set([...h.matchAll(/^struct (\w+_impl)/gm)].map((m) => m[1]))];
// 真解析: 导出别名 (_v / _t)
const aliases = [...h.matchAll(/^(?:using|constexpr) (\w+) = /gm)].map((m) => m[1]);

const body = `${banner("src/base/template-meta-programming/list.h.js", "src/base/template-meta-programming/list.h", "类型级 list 库 (list/list1 双轨)")}
//! v8 侧: 容器 ${containers.length} (${containers.join("/")}); _impl 结构 ${impls.length}; 导出别名 ${aliases.length}
//! Rust 策略: 类型级链表用 trait + 虚元组; 高阶操作 (map/fold) 对应宏展开, 语义面 Developer 落地

/// v8: src/base/template-meta-programming/list.h struct list / struct list1
/// list = 类型包; list1 = 一元模板包 (TYPENAME1)
pub struct Nil;
/// v8: list<Ts...> 递归 spine
pub struct Cons<Head, Tail>(core::marker::PhantomData<(Head, Tail)>);

// ---- impl 结构对账 (v8 侧 ${impls.length} 个) ----
// ${impls.join(", ")}
// 语义分组:
//   长度/取值: length, element
//   变换: map, fold_right
//   查找: index_of, contains
//   判定: all_of, all_equal
//   拼接/插入: append, insert_at
//   转换: from_tuple (list ↔ std::tuple)

// ---- 导出别名对账 (v8 侧 ${aliases.length} 条) ----
// ${aliases.join(", ")}

/// v8: length_v = length<List>::value
pub trait ListLength { const VALUE: usize; }
/// v8: element_t<List, Index>
pub trait ListElement<Index> { type Type; }
/// v8: contains_v<List, T>
pub trait ListContains<T> { const VALUE: bool; }
/// v8: append_t<List, Ts...>
pub trait ListAppend<Ts> { type Type; }
/// v8: fold_right_t<F, List, T>
pub trait ListFoldRight<F, Init> { type Type; }
`;

writeSrc("v8-base", "template_meta_programming/list.rs.skel", body);
console.log(`tmp/list.h.js: containers=${containers.length} impls=${impls.length} aliases=${aliases.length}`);
