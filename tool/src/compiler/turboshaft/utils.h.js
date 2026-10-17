#!/usr/bin/env bun
// tool/src/compiler/turboshaft/utils.h.js
// 机械提取 v8/src/compiler/turboshaft/utils.{h,cc}
// → v8-turboshaft/src/utils.rs.skel
//
// 提取面: any_of/all_of 匹配工具; MultiSwitch 编码公式; bisect 工具。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/utils.h");
const cc = readV8("src/compiler/turboshaft/utils.cc");

// 1. 哨兵
for (const [re, name] of [
  [/struct any_of : std::tuple<const Ts&\.\.\.>/, "any_of"],
  [/struct all_of : std::tuple<const Ts&\.\.\.>/, "all_of"],
  [/template <class T, class\.\.\. Ts>\s*\nbool operator==\(const T& value, any_of<Ts\.\.\.>/, "any_of 相等"],
  [/bool operator==\(all_of<Ts\.\.\.> values, const T& target\)/, "all_of 相等"],
  [/struct MultiSwitch;/, "MultiSwitch 前置"],
  [/struct MultiSwitchIntegral \{/, "MultiSwitchIntegral"],
  [/#define DEFINE_MULTI_SWITCH_INTEGRAL\(name, max_value\)/, "整型特化宏"],
  [/constexpr uint64_t multi\(const Ts&\.\.\. values\)/, "multi 入口"],
  [/V8_EXPORT_PRIVATE bool ShouldSkipOptimizationStep\(\);/, "bisect 声明"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc bisect 实现
for (const [re, name] of [
  [/static std::atomic<uint64_t> counter\{0\};/, "原子计数器"],
  [/turboshaft_opt_bisect_break/, "断点 flag"],
  [/turboshaft_opt_bisect_limit/, "上限 flag"],
  [/base::OS::DebugBreak\(\);/, "DebugBreak"],
  [/#ifdef DEBUG/, "仅 DEBUG"],
]) {
  if (!re.test(cc)) throw new Error(`cc 哨兵缺失: ${name}`);
}

let rust = banner(
  "src/compiler/turboshaft/utils.h.js",
  "src/compiler/turboshaft/utils.{h,cc}",
  "工具: any_of/all_of, multi-switch 编码, bisect",
);

rust += `
/// 原版 any_of<Ts...> — value == 任一候选 (tuple 引用包)
/// v8: utils.h any_of
pub fn is_any_of<T: PartialEq>(value: &T, options: &[T]) -> bool {
  options.iter().any(|o| o == value)
}

/// 原版 all_of<Ts...> — 全部候选 == target
/// v8: utils.h all_of
pub fn is_all_of<T: PartialEq>(values: &[T], target: &T) -> bool {
  values.iter().all(|v| v == target)
}

/// 原版 multi(v1..vn) — 多维 switch 编码 (小端式逐维混合):
///   multi(v1,v2,v3) =
///     t1 = enc(v3)
///     t2 = t1*max(T2) + enc(v2)
///     ret = t2*max(T1) + enc(v1)
/// 每类型须特化 MultiSwitch<T> { max_value, encode };
/// 整型经 DEFINE_MULTI_SWITCH_INTEGRAL(name, max_value) 生成,
/// bool 特化 max=2 已在文件尾给出。
/// v8: utils.h MultiSwitch / multi
pub fn multi(values: &[u64], max_values: &[u64]) -> u64 { todo!() }

/// 原版 ShouldSkipOptimizationStep (仅 DEBUG 编译; release 内联 false)
/// cc 实现: 原子自增计数, == turboshaft_opt_bisect_break 时
/// DebugBreak; >= turboshaft_opt_bisect_limit 返回 true。
/// v8: utils.{h,cc} ShouldSkipOptimizationStep
#[cfg(debug_assertions)]
pub fn should_skip_optimization_step() -> bool { todo!() }
#[cfg(not(debug_assertions))]
pub fn should_skip_optimization_step() -> bool { false }
`;

const p = writeSrc("v8-turboshaft", "utils.rs.skel", rust);
console.log(`written ${p}`);
console.log(`工具=4 (any_of/all_of/multi/bisect)`);
