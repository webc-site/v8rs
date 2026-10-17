#!/usr/bin/env bun
// tool/src/base/template-meta-programming/string-literal.h.js — 覆盖 v8/src/base/template-meta-programming/string-literal.h
// 解析 StringLiteral<N> 结构类型 (编译期字符串作非类型模板参数) 与 to_array 兼容层
// → v8-base/src/template_meta_programming/string_literal.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/template-meta-programming/string-literal.h");

for (const anchor of [
  "V8_BASE_TEMPLATE_META_PROGRAMMING_STRING_LITERAL_H_",
  "class StringLiteral {",
  "constexpr StringLiteral(const char (&s)[N])",
  "size_t size() const",
  "const char* c_str() const",
  "StringLiteral(const char (&)[N]) -> StringLiteral<N>;",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 方法签名
const methods = [...h.matchAll(/^  (?:size_t|const char\*) (\w+)\(\) const/gm)].map((m) => m[1]);
// 条件编译开关
const hasClassTypes = h.includes("HAS_CPP_CLASS_TYPES_AS_TEMPLATE_ARGS");

const body = `${banner("src/base/template-meta-programming/string-literal.h.js", "src/base/template-meta-programming/string-literal.h", "编译期字符串字面量 (结构类型传模板参数)")}
//! v8 侧: 方法 ${methods.length}; 条件开关 HAS_CPP_CLASS_TYPES_AS_TEMPLATE_ARGS=${hasClassTypes}
//! Rust 策略: &'static str 常量即结构化; const 泛型数组 [u8; N] 等价 StringLiteral

/// v8: src/base/template-meta-programming/string-literal.h StringLiteral<N>
/// data_ 为 public 成员 (结构类型要求); 以 '\\0' 结尾
#[derive(Clone, Copy)]
pub struct StringLiteral<const N: usize> {
    /// v8: data_ (std::array<char, N>, 不含结尾 '\\0' 的 size = N-1)
    pub data: [u8; N],
}

impl<const N: usize> StringLiteral<N> {
    /// v8: StringLiteral(const char (&s)[N]) — DCHECK 尾部 '\\0'
    pub const fn new(s: &[u8; N]) -> Self { Self { data: *s } }
    /// v8: size() — 不含 '\\0'
    pub const fn size(&self) -> usize { N - 1 }
    /// v8: c_str()
    pub const fn c_str(&self) -> &[u8] { &self.data }
}
`;

writeSrc("v8-base", "template_meta_programming/string_literal.rs.skel", body);
console.log(`tmp/string-literal.h.js: methods=${methods.length} classTypesGate=${hasClassTypes}`);
