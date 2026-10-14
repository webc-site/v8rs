#!/usr/bin/env bun
// tool/src/compiler/turboshaft/assembler.h.js
// 机械提取 v8/src/compiler/turboshaft/assembler.{h,cc}
// → v8-turboshaft/src/assembler.rs.skel
//
// 提取面: Range/IndexRange/Sequence/Zip 迭代工具族 + ConditionWithHint +
// LabelBase::Likelyness 枚举 + Label/LoopLabel + AssemblerOpInterface 的
// REDUCE 接口面 + __ 宏 (AssembleMacro) 约定。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/assembler.h");
const cc = readV8("src/compiler/turboshaft/assembler.cc");

// 1. 迭代工具族类面 (基类链核对)
for (const [re, label] of [
  [/^class Range\s*\{/m, "Range"],
  [/class IndexRange : public Range<T>/, "IndexRange : Range"],
  [/class Sequence : private Range<T>/, "Sequence (私有继承 Range)"],
  [/class Zip/, "Zip"],
  [/class ConditionWithHint final/, "ConditionWithHint final"],
  [/^class LabelBase \{/m, "LabelBase (带默认模板参)"],
  [/class Label : public LabelBase<false, Ts\.\.\.>/, "Label : LabelBase<false>"],
  [/class LoopLabel : public LabelBase<true, Ts\.\.\.>/, "LoopLabel : LabelBase<true>"],
  [/struct BlockData\s*\{/, "LabelBase::BlockData"],
  [/class AssemblerOpInterface/, "AssemblerOpInterface"],
  [/class CatchScopeImpl/, "CatchScopeImpl"],
  [/class FrameStateForCall\s*\{/, "FrameStateForCall"],
  [/class Uninitialized\s*\{/, "Uninitialized"],
  [/#define TURBOSHAFT_REDUCER_BOILERPLATE\(Name\)/, "TURBOSHAFT_REDUCER_BOILERPLATE 宏定义"],
  [/#define EMIT_OP\(Name\)/, "EMIT_OP 宏定义"],
  [/#define LABEL_BLOCK\(label\)/, "LABEL_BLOCK 宏定义"],
]) {
  if (!re.test(h)) throw new Error(`h 哨兵缺失: ${label}`);
}

// 2. LabelBase::Likelyness 枚举
const likelyM = h.match(/enum Likelyness\s*\{([^}]+)\}/);
if (!likelyM) throw new Error("未找到 LabelBase::Likelyness");
const likely = [...likelyM[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((m) => m[1]);
if (JSON.stringify(likely) !== JSON.stringify(["Unknown", "Likely", "Unlikely"])) {
  throw new Error(`Likelyness ${JSON.stringify(likely)} != 3 项`);
}

// 3. REDUCE 接口面 (AssemblerOpInterface 声明, 数量对数)
const reduceNames = [...new Set([...h.matchAll(/\bREDUCE\((\w+)\)\s*\(/g)].map((m) => m[1]))];
if (reduceNames.length !== 9) throw new Error(`REDUCE 接口 ${reduceNames.length} != 9`);

// 4. Range 协议 (Begin/IsEnd/Next/IsEmpty) + cc 侧
for (const [re, label] of [
  [/iterator_type Begin\(A& assembler\) const/, "Range::Begin"],
  [/OptionalV<Word32> IsEnd\(A& assembler, iterator_type current_iterator\) const/, "Range::IsEnd (Word32/Word64 分派)"],
  [/stride_/, "Range::stride_"],
  [/CheckLabelWasBound/, "Label 析构校验"],
]) {
  if (!re.test(h)) throw new Error(`协议缺失: ${label}`);
}
if (!/^namespace v8::internal::compiler::turboshaft \{/m.test(cc) && !cc.includes("turboshaft")) {
  throw new Error("cc 命名空间锚缺失");
}

// 5. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/assembler.h.js",
  "src/compiler/turboshaft/assembler.{h,cc}",
  "汇编器接口: 迭代工具族/Label 族/REDUCE 接口面骨架",
);
rust += `
/// v8: src/compiler/turboshaft/assembler.h LabelBase::Likelyness
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Likelyness {
  /// kUnknown
  unknown,
  /// kLikely
  likely,
  /// kUnlikely
  unlikely,
}

/// v8: src/compiler/turboshaft/assembler.h Range
///
/// Word32/Word64 循环区间 (begin/end/stride), 协议:
/// Begin(assembler) / IsEnd(assembler, it) (Word32/Word64 LessThanOrEqual 分派) /
/// Next / IsEmpty。IndexRange: OpIndex 版; Sequence: 私有继承 Range (展开列表);
/// Zip: 多 Range 并行。
pub struct Range<T> {
  pub begin: T,
  pub end: T,
  pub stride: T,
}

/// v8: src/compiler/turboshaft/assembler.h ConditionWithHint
/// V<Word32> 条件 + BranchHint (kNone 默认); OpIndex 隐式转换重载
pub struct ConditionWithHint {
  pub condition: crate::index::OpIndex,
  pub hint: crate::operations::BranchHint,
}

/// v8: src/compiler/turboshaft/assembler.h LabelBase<loop, Ts...>
///
/// 值多元组: values_t = tuple<V<Ts>...>; LoopLabel(loop=true) 允许重复绑定;
/// 析构 CheckLabelWasBound (未绑定即 bug)。BlockData: block + phi 缓冲。
pub struct LabelBase {
  pub is_loop: bool,
  pub likelyness: Likelyness,
}

/// v8: src/compiler/turboshaft/assembler.h Label / LoopLabel
pub type Label = LabelBase;
pub type LoopLabel = LabelBase;

/// v8: src/compiler/turboshaft/assembler.h AssemblerOpInterface
///
/// REDUCE 接口面 (${reduceNames.length} 声明): ${reduceNames.join(" / ")}
/// 宏族: TURBOSHAFT_REDUCER_BOILERPLATE / EMIT_OP / LABEL_BLOCK (生成器语法糖)。
pub struct AssemblerOpInterface;
`;

const p = writeSrc("v8-turboshaft", "assembler.rs.skel", rust);
console.log(`written ${p}`);
console.log(`类面=13 Likelyness=${likely.length} REDUCE接口=${reduceNames.length} (${reduceNames.join("/")})`);
