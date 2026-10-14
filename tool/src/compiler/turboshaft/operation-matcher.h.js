#!/usr/bin/env bun
// tool/src/compiler/turboshaft/operation-matcher.h.js
// 机械提取 v8/src/compiler/turboshaft/operation-matcher.h
// → v8-turboshaft/src/operation_matcher.rs.skel
//
// 提取面: detail::ValueMatch/OptionMatch 匹配原语; OperationMatcher 全部
// Match* 方法签名 (通配/绑定/常量三态); MatchPowerOfTwo 系工具。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/operation-matcher.h");

// 1. 类骨架哨兵
for (const [re, name] of [
  [/class OperationMatcher \{/, "OperationMatcher"],
  [/using VMatch = detail::ValueMatch<T, const_or_v_exists_v<T>>;/, "VMatch 别名"],
  [/using OMatch = detail::OptionMatch<T>;/, "OMatch 别名"],
  [/struct ValueMatch<T, false>/, "无常量特化"],
  [/struct OptionMatch \{/, "OptionMatch"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. Match* 方法面 (去重名, 统计重载)
const matchMethods = [...h.matchAll(/^\s{2}(?:bool|std::optional<[^>]*>|V<\w+>)\s+(Match\w+)\(/gm)].map((m) => m[1]);
const unique = [...new Set(matchMethods)];
if (unique.length < 30) throw new Error(`Match* 方法 ${unique.length} < 30`);
for (const must of [
  "MatchZero", "MatchSmiZero", "MatchFloat64Constant", "MatchHeapConstant",
  "MatchIntegralWordConstant", "MatchWordBinop", "MatchBitwiseAndWithConstant",
  "MatchEqual", "MatchConstantShift", "MatchPhi", "MatchPowerOfTwoWord32Constant",
]) {
  if (!unique.includes(must)) throw new Error(`Match* 缺 ${must}`);
}

// 3. 常量折叠相关工具
const matchT = [...h.matchAll(/template <typename T = intptr_t>/g)].length;

let rust = banner(
  "src/compiler/turboshaft/operation-matcher.h.js",
  "src/compiler/turboshaft/operation-matcher.h",
  "操作匹配器: 模式匹配 Match* 工具面",
);

rust += `
/// 原版 detail::ValueMatch — 值匹配三态: 通配 / OpIndex 绑定 / 常量
/// v8: src/compiler/turboshaft/operation-matcher.h detail::ValueMatch
pub enum ValueMatch<T> {
  /// Wildcard: 任意值均可
  Wildcard,
  /// 指定 OpIndex: 须全等
  Index(OpIndex),
  /// OpIndex* 输出绑定: 匹配成功时写出
  Bind(*mut OpIndex),
  /// constexpr 常量: 比对常量值
  Constant(T),
}

/// 原版 detail::OptionMatch — 选项匹配 (通配/值/绑定)
/// v8: src/compiler/turboshaft/operation-matcher.h detail::OptionMatch
pub struct OptionMatch<T>(pub OptionE<T>);

/// 原版 OperationMatcher — 只读图上的模式匹配器
/// v8: src/compiler/turboshaft/operation-matcher.h OperationMatcher
pub struct OperationMatcher<'g> {
  pub graph: &'g Graph,
}

impl<'g> OperationMatcher<'g> {
  /// v8: operation-matcher.h Is / TryCast / Cast / Get / Index — 薄委托 graph
  pub fn is<Op>(&self, idx: V) -> bool { todo!() }
  pub fn try_cast<Op>(&self, idx: V) -> Option<&'g Op> { todo!() }
  pub fn get(&self, idx: V) -> &'g Operation { todo!() }

  // —— Match* 方法面 (${unique.length} 个名, ${matchMethods.length} 个重载) ——
${unique.map((m) => `  /// v8: operation-matcher.h ${m}\n  pub fn ${m}(&self, /* ... */) -> bool { todo!() }`).join("\n")}
}

/// MatchIntegralWordPtrConstant 的 ${matchT} 处 intptr_t 默认模板参数
/// (Is64() 时走 Word64 分支) — Rust 侧按指针宽度静态分派。
`;

const p = writeSrc("v8-turboshaft", "operation_matcher.rs.skel", rust);
console.log(`written ${p}`);
console.log(`Match* 唯一名=${unique.length} 重载总数=${matchMethods.length} intptr默认=${matchT}`);
