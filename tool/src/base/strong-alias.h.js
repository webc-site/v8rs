#!/usr/bin/env bun
// tool/src/base/strong-alias.h.js — 覆盖 v8/src/base/strong-alias.h
// 解析 StrongAlias<TagType, UnderlyingType> 方法面 (取值/比较/hash/ostream)
// → v8-base/src/strong_alias.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/strong-alias.h");

for (const anchor of [
  "V8_BASE_STRONG_ALIAS_H_",
  "template <typename TagType, typename UnderlyingType>",
  "class StrongAlias {",
  "using underlying_type = UnderlyingType;",
  "constexpr explicit StrongAlias(const UnderlyingType& v) : value_(v) {}",
  "constexpr UnderlyingType* operator->()",
  "constexpr UnderlyingType& operator*() &",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 方法/运算符面
const ops = [...h.matchAll(/^  (?:constexpr|friend|V8_INLINE)[^{]*(operator\S+|\bvalue\(\)|hash_value|emplace\w*)/gm)].map((m) => m[1]);
// 真解析: 友元自由函数
const friends = [...h.matchAll(/^friend (\w+) (\w+)\(/gm)].map((m) => m[2]);

const body = `${banner("src/base/strong-alias.h.js", "src/base/strong-alias.h", "类型安全别名 (phantom tag newtype)")}
//! v8 侧: 运算/方法面 ${new Set(ops).size}; 友元函数 ${new Set(friends).size}
//! Rust 策略: 语言级 newtype 即零成本; TagType → 零大小 PhantomData 单元结构

/// v8: src/base/strong-alias.h class StrongAlias<TagType, UnderlyingType>
/// v8 动机: C++ 无类型安全 typedef; Rust newtype 原生解决 (Apple(2) vs Orange(2) 不同型)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrongAlias<Tag, Underlying> {
    /// v8: value_
    pub value: Underlying,
    _tag: core::marker::PhantomData<Tag>,
}

impl<Tag, U: PartialEq> StrongAlias<Tag, U> {
    /// v8: operator==/!= — 仅同 Tag 实例可比 (Rust 泛型参数天然区分)
    pub fn eq(&self, o: &Self) -> bool { self.value == o.value }
}

impl<Tag, U: PartialOrd> StrongAlias<Tag, U> {
    /// v8: operator</<=/>/>= — 有序容器用
    pub fn lt(&self, o: &Self) -> bool { self.value < o.value }
}

// ---- v8 运算面对账 (解析 ${new Set(ops).size} 项) ----
// ${[...new Set(ops)].join(", ")}
// 语义: operator-> / operator* / value() → 直接访问 .value 字段;
// hash_value (友元) → derive Hash; operator<< (友元) → derive Debug/Display;
// is_null (值初始化判空) → Option<StrongAlias> 或显式判 — Developer 按用点补
`;

writeSrc("v8-base", "strong_alias.rs.skel", body);
console.log(`strong-alias.h.js: ops=${new Set(ops).size} friends=${new Set(friends).size}`);
