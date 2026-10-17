#!/usr/bin/env bun
// tool/src/base/template-utils.h.js — 覆盖 v8/src/base/template-utils.h
// 解析 make_array/pass_value_or_ref/tuple 算子族 (head/drop/for_each/map/map2/fold)
// 与类型包元函数 (nth_type_t/index_of_type_v/has_type_v)
// → v8-base/src/template_utils.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/template-utils.h");

for (const anchor of [
  "V8_BASE_TEMPLATE_UTILS_H_",
  "constexpr auto make_array(Function f) {",
  "struct pass_value_or_ref {",
  "concept has_output_operator =",
  "using append_tuple_type =",
  "constexpr auto tuple_head(Tuple&& tpl) {",
  "constexpr auto tuple_drop(Tuple&& tpl) {",
  "constexpr void tuple_for_each(Tuple&& tpl, Function&& function) {",
  "constexpr auto tuple_map(Tuple&& tpl, Function&& function) {",
  "constexpr auto tuple_fold(T&& initial, Tuple&& tpl, Function&& function) {",
  "using nth_type_t =",
  "constexpr size_t index_of_type_v =",
  "constexpr bool has_type_v =",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: detail impl 与公开导出
const details = [...new Set([...h.matchAll(/^constexpr (?:auto|bool|inline auto) (\w+_impl)\(/gm)].map((m) => m[1]))];
const tupleOps = [...h.matchAll(/^constexpr (?:auto|void) (tuple_\w+)\(/gm)].map((m) => m[1]);
const typeMeta = [...h.matchAll(/^(?:using|constexpr) (nth_type_t|index_of_type_v|has_type_v|append_tuple_type|prepend_tuple_type)/gm)].map((m) => m[1]);

const body = `${banner("src/base/template-utils.h.js", "src/base/template-utils.h", "模板工具: make_array/tuple 算子/类型包元函数")}
//! v8 侧: detail impl ${details.length}; tuple 算子 ${new Set(tupleOps).size}; 类型级导出 ${new Set(typeMeta).size}
//! Rust 策略: make_array → const fn 数组推导; tuple 算子 → 元组/数组迭代适配; 类型包 → 宏展开

/// v8: src/base/template-utils.h make_array<Size>(f)
/// f(0..Size-1) 逐项填数组; constexpr 传播
pub fn make_array<T, const N: usize>(f: impl Fn(usize) -> T) -> [T; N] {
    core::array::from_fn(|i| f(i))
}

/// v8: pass_value_or_ref<T> — 标量/数组按值, 其余按 const 引用
/// Rust: Copy 类型隐式按值; 非 Copy 显式 &T; 此元函数退化为文档锚

/// v8: concept has_output_operator<T> — stream << t 可行
/// Rust: core::fmt::Display bound 即判定

// ---- tuple 算子 (v8 ${new Set(tupleOps).size} 个) ----
// 对账: ${[...new Set(tupleOps)].join(", ")}
/// tuple_head<N> / tuple_drop<N>: 取前 N 项 / 去前 N 项
/// tuple_for_each / tuple_for_each_with_index: 逐项副作用 (fold 表达式展开)
/// tuple_map / tuple_map2: 逐项/双 zip 变换
/// tuple_fold: 右折叠 (I 从 tuple_size-1 递归到 0)
// Rust 落地: 运行期用 Vec/数组 + iterator; 编译期异构用宏逐项展开 — Developer 补

// ---- 类型包元函数 (v8 ${new Set(typeMeta).size} 个) ----
// 对账: ${[...new Set(typeMeta)].join(", ")}
/// nth_type_t<N, Ts...>: 第 N 类型 (v8 用 __type_pack_element 内建)
/// index_of_type_v<SearchT, Ts...>: 线性查型序号 (编译期 for)
/// has_type_v<SearchT, Ts...>: (is_same_v || ...) 折叠
/// append/prepend_tuple_type: 元组拼接方向
`;

writeSrc("v8-base", "template_utils.rs.skel", body);
console.log(`template-utils.h.js: details=${details.length} tupleOps=${new Set(tupleOps).size} typeMeta=${new Set(typeMeta).size}`);
