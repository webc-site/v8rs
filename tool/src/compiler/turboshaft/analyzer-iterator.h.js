#!/usr/bin/env bun
// tool/src/compiler/turboshaft/analyzer-iterator.h.js
// 机械提取 v8/src/compiler/turboshaft/analyzer-iterator.{h,cc}
// → v8-turboshaft/src/analyzer_iterator.rs.skel
//
// 提取面: AnalyzerIterator 世代访问协议 — 两常量 (kNotVisitedGeneration=0,
// kGenerationForFirstVisit=1) + StackNode + 4 方法 (HasNext/Next/
// MarkLoopForRevisit/MarkLoopForRevisitSkipHeader) + cc Next 实现。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/analyzer-iterator.h");
const cc = readV8("src/compiler/turboshaft/analyzer-iterator.cc");

// 1. 常量 (值核对)
function numConst(re, label, expect) {
  const m = h.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  if (Number(m[1]) !== expect) throw new Error(`${label}=${m[1]} 预期 ${expect}`);
  return Number(m[1]);
}
const notVisited = numConst(/static constexpr uint64_t kNotVisitedGeneration = (\d+);/, "kNotVisitedGeneration", 0);
const firstVisit = numConst(/static constexpr uint64_t kGenerationForFirstVisit = (\d+);/, "kGenerationForFirstVisit", 1);

// 2. 方法面
for (const [re, label] of [
  [/class V8_EXPORT_PRIVATE AnalyzerIterator/, "AnalyzerIterator 类"],
  [/bool HasNext\(\) const/, "HasNext"],
  [/const Block\* Next\(\);/, "Next 声明"],
  [/void MarkLoopForRevisit\(\);/, "MarkLoopForRevisit"],
  [/void MarkLoopForRevisitSkipHeader\(\);/, "MarkLoopForRevisitSkipHeader"],
  [/struct StackNode\s*\{/, "StackNode"],
  [/void PopOutdated\(\);/, "PopOutdated (私有)"],
  [/bool IsOutdated\(StackNode node\) const/, "IsOutdated"],
  [/FixedBlockSidetable<uint64_t> visited_;/, "visited_ 侧表"],
  [/ZoneVector<StackNode> stack_;/, "stack_"],
  [/StackNode curr_ = \{nullptr, 0\};/, "curr_"],
  [/DCHECK_IMPLIES\(!stack_\.empty\(\), !IsOutdated\(stack_\.back\(\)\)\);/, "栈顶非过期不变式"],
]) {
  if (!re.test(h)) throw new Error(`h 哨兵缺失: ${label}`);
}

// 3. cc: Next/PopOutdated/MarkLoopForRevisit* 定义
for (const [re, label] of [
  [/const Block\* AnalyzerIterator::Next\(\)/, "Next 定义"],
  [/void AnalyzerIterator::PopOutdated\(\)/, "PopOutdated 定义"],
  [/void AnalyzerIterator::MarkLoopForRevisit\(\)/, "MarkLoopForRevisit 定义"],
  [/void AnalyzerIterator::MarkLoopForRevisitSkipHeader\(\)/, "MarkLoopForRevisitSkipHeader 定义"],
]) {
  if (!re.test(cc)) throw new Error(`cc 哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/analyzer-iterator.h.js",
  "src/compiler/turboshaft/analyzer-iterator.{h,cc}",
  "分析迭代器: 世代标记的循环重访访问协议骨架",
);
rust += `
/// 原版 AnalyzerIterator::kNotVisitedGeneration
pub const NOT_VISITED_GENERATION: u64 = ${notVisited};
/// 原版 AnalyzerIterator::kGenerationForFirstVisit
pub const GENERATION_FOR_FIRST_VISIT: u64 = ${firstVisit};

/// v8: src/compiler/turboshaft/analyzer-iterator.h StackNode
pub struct StackNode<'a> {
  pub block: Option<&'a crate::graph::Block>,
  pub generation: u64,
}

/// v8: src/compiler/turboshaft/analyzer-iterator.h AnalyzerIterator
///
/// 协议 (原版注释保留): visited_ 记块世代; 栈顶永非过期 (HasNext 即栈非空);
/// 弹出块世代 < 记录值则跳过; > 记录值说明途经环回头, 本块也需重访;
/// MarkLoopForRevisit 提升环内块的待访世代。
pub struct AnalyzerIterator<'a> {
  /// visited_: FixedBlockSidetable<u64>
  pub visited: Vec<u64>,
  /// stack_: 待访栈
  pub stack: Vec<StackNode<'a>>,
  /// current_generation_
  pub current_generation: u64,
}

impl<'a> AnalyzerIterator<'a> {
  /// 原版 bool HasNext() const — 栈非空 (栈顶非过期由构造维护)
  pub fn has_next(&self) -> bool {
    !self.stack.is_empty()
  }
  /// 原版 const Block* Next() — v8: src/compiler/turboshaft/analyzer-iterator.cc
  /// 弹出 (PopOutdated 后) 记录世代; 后继未访/可重访则压栈
  pub fn next(&mut self) -> Option<&'a crate::graph::Block> {
    unimplemented!("Developer 修订")
  }
  /// 原版 void MarkLoopForRevisit() — 以当前块 (回边) 为头的环整环重访
  pub fn mark_loop_for_revisit(&mut self) {
    unimplemented!("Developer 修订")
  }
  /// 原版 void MarkLoopForRevisitSkipHeader() — 同上但跳过环头
  pub fn mark_loop_for_revisit_skip_header(&mut self) {
    unimplemented!("Developer 修订")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(NOT_VISITED_GENERATION == 0);
  assert!(GENERATION_FOR_FIRST_VISIT == 1);
};
`;

const p = writeSrc("v8-turboshaft", "analyzer_iterator.rs.skel", rust);
console.log(`written ${p}`);
console.log(`常量=2 方法=4 h哨兵=12 cc定义=4`);
