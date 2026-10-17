#!/usr/bin/env bun
// tool/src/base/numerics/integral_constant_like.h.js — 覆盖 v8/src/base/numerics/integral_constant_like.h
// 解析 IntegralConstantLike concept 的 6 条谓词约束
// → v8-base/src/numerics/integral_constant_like.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/numerics/integral_constant_like.h");

for (const anchor of [
  "V8_BASE_NUMERICS_INTEGRAL_CONSTANT_LIKE_H_",
  "concept IntegralConstantLike",
  "std::is_integral_v<decltype(T::value)>",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: concept 体的谓词行 (&& 链)
const conceptBody = h.match(/concept IntegralConstantLike =([\s\S]+?);/);
if (!conceptBody) throw new Error("concept body not found");
const preds = conceptBody[1]
  .split("&&")
  .map((s) => s.trim())
  .filter((s) => s.length > 0);
if (preds.length !== 6) throw new Error(`expect 6 predicates, got ${preds.length}`);

const body = `${banner("src/base/numerics/integral_constant_like.h.js", "src/base/numerics/integral_constant_like.h", "[span.syn] 展示性 concept, Rust 映射为 trait 约束")}
//! v8 侧 concept 1 个, 谓词 ${preds.length} 条 (&& 链)
//! Rust 无 concept; 对应 trait: IntegralConstantLike (由 Developer 定型, 常量求值约束交 const 泛型/关联常量)

/// v8: src/base/numerics/integral_constant_like.h concept IntegralConstantLike
/// 谓词原文 (逐条):
${preds.map((p, i) => `///   (${i + 1}) ${p}`).join("\n")}
pub trait IntegralConstantLike {
    /// v8 谓词 (1): decltype(T::value) 为整型且非 bool
    type Value: Copy + Eq + core::fmt::Debug;
    /// v8 谓词 (1)(2): T::value 是整型常量 (非 bool)
    const VALUE: Self::Value;
    /// v8 谓词 (3)(4): T 可转换/可比较到 Value
    /// v8 谓词 (5)(6): T() == T::value (可默认构造且等于 value) — 常量求值, 就地修订
}
`;

writeSrc("v8-base", "numerics/integral_constant_like.rs.skel", body);
console.log(`integral_constant_like.h.js: concepts=1 predicates=${preds.length}`);
