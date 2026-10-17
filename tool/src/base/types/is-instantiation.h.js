#!/usr/bin/env bun
// tool/src/base/types/is-instantiation.h.js — 覆盖 v8/src/base/types/is-instantiation.h
// 解析 is_instantiation_v 变量模板主模板/特化与 is_instantiation concept
// → v8-base/src/types/is_instantiation.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/types/is-instantiation.h");

for (const anchor of [
  "V8_BASE_TYPES_IS_INSTANTIATION_H_",
  "inline constexpr bool is_instantiation_v = false;",
  "is_instantiation_v<C<Ts...>, C> = true;",
  "concept is_instantiation =",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 变量模板定义数
const varDefs = (h.match(/inline constexpr bool is_instantiation_v/g) || []).length;

const body = `${banner("src/base/types/is-instantiation.h.js", "src/base/types/is-instantiation.h", "模板实例判定 (类型级谓词)")}
//! v8 侧: 变量模板定义 ${varDefs} (主 false + 特化 true); concept 1
//! Rust 策略: 用宏生成 impl 块, T 为具体泛型包装时 impl IsInstantiation<C>

/// v8: src/base/types/is-instantiation.h is_instantiation<T, C>
/// 语义: T 是否为 C<...> 的实例 (不考虑引用/cv 限定, 需先 decay)
pub trait IsInstantiation<C> {
    /// v8: is_instantiation_v
    const VALUE: bool;
}

/// v8: internal::is_instantiation_v = false (主模板)
/// 语义面就地修订: 为每个关心的 (T, C) 对用宏批量 impl
///   macro_rules! is_instantiation_of { ... }
/// 注意 v8 特化侧: is_instantiation_v<C<Ts...>, C> = true
`;

writeSrc("v8-base", "types/is_instantiation.rs.skel", body);
console.log(`is-instantiation.h.js: varDefs=${varDefs} concepts=1`);
