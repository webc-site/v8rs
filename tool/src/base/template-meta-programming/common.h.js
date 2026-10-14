#!/usr/bin/env bun
// tool/src/base/template-meta-programming/common.h.js — 覆盖 v8/src/base/template-meta-programming/common.h
// 解析 equals/equals1/instantiate/is_instantiation_of 四组元函数及其特化
// → v8-base/src/template_meta_programming/common.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/template-meta-programming/common.h");

for (const anchor of [
  "V8_BASE_TEMPLATE_META_PROGRAMMING_COMMON_H_",
  "#define TYPENAME1",
  "struct equals : std::bool_constant<false> {};",
  "struct equals1 : std::bool_constant<false> {};",
  "struct instantiate {",
  "struct is_instantiation_of : std::bool_constant<false> {};",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: struct 元函数 (主模板) 清单
const metafns = [...h.matchAll(/^struct (\w+) : std::bool_constant<(?:false|true)> \{\};$/gm)].map((m) => m[1]);
// 真解析: 特化数
const specs = (h.match(/^struct \w+<[^>]*> : std::bool_constant<true> \{\};$/gm) || []).length;

const body = `${banner("src/base/template-meta-programming/common.h.js", "src/base/template-meta-programming/common.h", "TMP 公共元函数: 类型等值/实例化判定")}
//! v8 侧: 谓词元函数 ${metafns.length} 组 (主 false + 特化 true 各 ${specs}); instantiate 1
//! Rust: 类型级等值用 PhantomData + trait; 判定面在 Developer 落地

${metafns.map((f) => `/// v8: src/base/template-meta-programming/common.h struct ${f}
/// 主模板 false; 特化 (同型/实例匹配) true`).join("\n")}
/// v8: struct instantiate { using type = T<U>; } — 一元模板施用
pub mod common {
    /// v8: equals<T, U> / equals1<T, U> (TYPENAME1 版)
    pub trait Equals<U> { const VALUE: bool; }
    /// v8: is_instantiation_of<I, T> — I 为 T<...> 实例时 true
    pub trait IsInstantiationOf<T> { const VALUE: bool; }
}
`;

writeSrc("v8-base", "template_meta_programming/common.rs.skel", body);
console.log(`tmp/common.h.js: metafns=${metafns.length} specs=${specs}`);
