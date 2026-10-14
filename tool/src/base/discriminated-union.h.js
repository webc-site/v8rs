#!/usr/bin/env bun
// tool/src/base/discriminated-union.h.js — 族: src/base/discriminated-union.h
// 覆盖: DiscriminatedUnion<TagEnum, Ts...> — tag 枚举对齐的类型联合 (4 个 get 重载)
// → v8-base/src/discriminated_union.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/discriminated-union.h");

// —— 锚点断言 ——
if (!src.includes("class DiscriminatedUnion {")) throw new Error("缺 DiscriminatedUnion");
if (!src.includes("static_assert((std::is_trivially_destructible_v<Ts> && ...));")) {
  throw new Error("缺 trivially destructible 约束");
}
if (!src.includes("alignas(std::max({alignof(Ts)...})) char data_[std::max({sizeof(Ts)...})];")) {
  throw new Error("缺 max-align/max-size 存储");
}
if (!src.includes("uint8_t tag_;")) throw new Error("缺 uint8_t tag_");
if (!src.includes("static_assert(sizeof...(Ts) <= std::numeric_limits<uint8_t>::max());")) {
  throw new Error("缺类型数上限断言");
}

// —— 真解析: 构造器/get 重载对数 ——
const ctors = (src.match(/constexpr explicit DiscriminatedUnion\(/g) || []).length;
if (ctors !== 2) throw new Error(`构造器 ${ctors} != 2 (tag+data / 仅 data)`);
const gets = (src.match(/constexpr (?:const )?auto& get\(\)/g) || []).length;
if (gets !== 4) throw new Error(`get 重载 ${gets} != 4 (按 tag const/mutable + 按类型 const/mutable)`);
if (!src.includes("constexpr Tag tag() const")) throw new Error("缺 tag()");
console.log("discriminated-union: 构造器 2, get 重载 4, tag() 1, 静态断言 2");

let rust = banner(
  "src/base/discriminated-union.h.js",
  "src/base/discriminated-union.h",
  "枚举判别的类型联合 (Rust enum 天然形态)",
);

rust += `
//! v8: src/base/discriminated-union.h
//! 原版: TagEnum 成员数 = 类型数, 值序对应类型序; 全类型 trivially destructible;
//!       存储 max{sizeof}, 对齐 max{alignof}, tag 为 uint8 (类型数 <= 255)
//! Rust: enum 直接表达; 本骨架登记 API 面 + 不变量

/// v8: src/base/discriminated-union.h DiscriminatedUnion<TagEnum, Ts...>
/// Rust 对应: enum X { A(T0), B(T1), ... } — tag() = 判别值, get<T>() = 模式匹配
/// 以下为语义约定 (Developer 按具体 TagEnum 生成):
/// - tag() 返回 TagEnum, 值 = 类型在 Ts... 中的序数 (原版 uint8_t tag_)
/// - get<Tag>()/get<T>() 原版 DCHECK tag 匹配; Rust 侧用 match 穷尽, 失配不可达
/// - 原版禁重复类型 (TODO(leszeks) 登记); Rust enum 变体可同名域不受限
/// - 构造: DiscriminatedUnion(tag, t) 与 DiscriminatedUnion(t) 等价 (前者 DCHECK tag)
pub trait DiscriminatedUnion {
    type Tag;
    /// v8: tag() — static_cast<Tag>(tag_)
    fn tag(&self) -> Self::Tag;
}

// v8: index_of_type_v<T, Ts...> / nth_type_t<N, Ts...> 来自 template-utils.h (别席),
//     Rust 侧由 enum 变体序数天然表达, 登记跳过
`;

writeSrc("v8-base", "discriminated_union.rs.skel", rust);
console.log("written v8-base/src/discriminated_union.rs.skel");
