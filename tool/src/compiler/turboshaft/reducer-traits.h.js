#!/usr/bin/env bun
// tool/src/compiler/turboshaft/reducer-traits.h.js
// 机械提取 v8/src/compiler/turboshaft/reducer-traits.h
// → v8-turboshaft/src/reducer_traits.rs.skel
//
// 提取面: reducer_list 系列 type-level 操作 (length/contains/starts_with/
// index_of/insert_at/append/to_stack) 与 next_reducer_is/next_contains_reducer。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/reducer-traits.h");

// 1. 别名/结构清单
const traits = [
  "reducer_list", "reducer_list_length", "reducer_list_contains",
  "reducer_list_starts_with", "reducer_list_index_of", "reducer_list_insert_at",
  "reducer_list_append", "reducer_list_to_stack", "next_reducer_is",
  "next_contains_reducer", "next_is_bottom_of_assembler_stack",
];
for (const t of traits) {
  if (!new RegExp(`(?:using|struct) ${t}\\b`).test(h)) throw new Error(`缺 ${t}`);
}

// 2. 哨兵
for (const [re, name] of [
  [/template <template <typename> typename\.\.\. Ts>/, "模板模板参"],
  [/base::tmp::fold_right1<base::tmp::instantiate, RL, Bottom>/, "to_stack 折叠"],
  [/class EmitProjectionReducer;/, "栈底前置声明"],
  [/next_reducer_is<Next, EmitProjectionReducer, AssemblerOpInterface,\s*ReducerBase, GraphEmitter>/, "栈底判定"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

let rust = banner(
  "src/compiler/turboshaft/reducer-traits.h.js",
  "src/compiler/turboshaft/reducer-traits.h",
  "reducer 栈类型操作面",
);

rust += `
/// 原版 reducer_list 与配套 type-level 操作 — Rust 侧以编译期
/// 类型队列 (tuple + trait) 或直接用嵌套泛型承载:
///
${traits.map((t) => `/// - ${t}`).join("\n")}
///
/// 语义对照:
/// - reducer_list_length_v<RL>        => 队列长度
/// - reducer_list_contains_v<RL, R>   => 队列含 R
/// - reducer_list_starts_with_v<RL,R> => index_of == 0
/// - reducer_list_index_of_v<RL, R, Otherwise> => 位置或 SIZE_MAX
/// - reducer_list_insert_at_t / append_t => 插入/追加
/// - reducer_list_to_stack_t<RL, Bottom> => fold_right instantiate 嵌套栈
/// - next_reducer_is_v<Next, Rs...>   => Next 是 Rs 之一的实例化
/// - next_contains_reducer_v<Next, R> => 栈中含 R (递归剥壳)
/// - next_is_bottom_of_assembler_stack => Next ∈ {EmitProjectionReducer,
///   AssemblerOpInterface, ReducerBase, GraphEmitter}
///
/// v8: src/compiler/turboshaft/reducer-traits.h (全部定义)
pub trait ReducerList {
  /// v8: reducer-traits.h reducer_list_length_v
  const LEN: usize;
  /// v8: reducer-traits.h reducer_list_contains_v
  fn contains<R: Reducer>() -> bool;
  /// v8: reducer-traits.h reducer_list_index_of_v — Otherwise = SIZE_MAX
  fn index_of<R: Reducer>() -> usize;
}

/// v8: reducer-traits.h next_is_bottom_of_assembler_stack
/// 栈底四件套: EmitProjectionReducer / AssemblerOpInterface /
/// ReducerBase / GraphEmitter (BuildReducerStack 注入)
pub trait IsBottomOfAssemblerStack {}
`;

const p = writeSrc("v8-turboshaft", "reducer_traits.rs.skel", rust);
console.log(`written ${p}`);
console.log(`trait登记=${traits.length}`);
