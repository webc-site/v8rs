#!/usr/bin/env bun
// tool/src/base/template-meta-programming/functional.h.js — 覆盖 v8/src/base/template-meta-programming/functional.h
// 解析 lazy_true/lazy_false 与 call_parameters 函数指针/成员函数指针特化
// → v8-base/src/template_meta_programming/functional.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/template-meta-programming/functional.h");

for (const anchor of [
  "V8_BASE_TEMPLATE_META_PROGRAMMING_FUNCTIONAL_H_",
  "struct lazy_false : std::false_type {};",
  "struct lazy_true : std::true_type {};",
  "struct call_parameters<R (*)(Args...)> {",
  "struct call_parameters<R (O::*)(Args...)> {",
  "using call_parameters_t =",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: struct 定义
const structs = [...h.matchAll(/^struct (\w+)(?: : std::\w+)? \{|^struct (\w+);$/gm)].map((m) => m[1] || m[2]);
// 真解析: call_parameters 特化数
const callSpecs = (h.match(/struct call_parameters</g) || []).length;

const body = `${banner("src/base/template-meta-programming/functional.h.js", "src/base/template-meta-programming/functional.h", "惰性 bool 与函数签名分解")}
//! v8 侧: 惰性元函数 ${h.includes("lazy_true") ? 2 : 0}; call_parameters 特化 ${callSpecs}
//! Rust 策略: fn 指针分解用 trait FnSignature; Rust 泛型无成员函数指针特化需求, 面向堆调用场景落地

/// v8: src/base/template-meta-programming/functional.h lazy_false/lazy_true
/// 用途: 延迟实例化的编译期 bool (static_assert 分支防误触发)
pub struct LazyFalse<T>(core::marker::PhantomData<T>);
pub struct LazyTrue<T>(core::marker::PhantomData<T>);

/// v8: call_parameters<R(*)(Args...)>::type = list<Args...>
/// v8: call_parameters<R(O::*)(Args...)>::type = list<Args...> (成员版)
/// Rust: 自由函数指针 → 元组trait 不可类型级分解; 运行期由 call 侧闭包适配, 语义面就地修订
pub trait CallParameters {
    /// v8: call_parameters_t
    type Args;
}
`;

writeSrc("v8-base", "template_meta_programming/functional.rs.skel", body);
console.log(`tmp/functional.h.js: lazy=${2} callSpecs=${callSpecs}`);
