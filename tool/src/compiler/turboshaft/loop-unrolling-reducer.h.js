#!/usr/bin/env bun
// tool/src/compiler/turboshaft/loop-unrolling-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/loop-unrolling-reducer.{h,cc}
// → v8-turboshaft/src/loop_unrolling_reducer.rs.skel
//
// 提取面: IterationCount::Kind (3) + StaticCanonicalForLoopMatcher 的
// CmpOp (9) / BinOp (9) + UnrollingStatus (3) 四枚举 + LoopStackCheckElision/
// LoopUnrollingReducer/LoopUnrollingAnalyzer/StaticCanonicalForLoopMatcher
// 四类面 + cc 侧 matcher 方法定义表。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/loop-unrolling-reducer.h");
const cc = readV8("src/compiler/turboshaft/loop-unrolling-reducer.cc");

// 1. 枚举 (变体 1:1)
function variants(re, label, expect) {
  const m = h.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  const vs = [...m[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((x) => x[1]);
  if (JSON.stringify(vs) !== JSON.stringify(expect)) {
    throw new Error(`${label} ${JSON.stringify(vs)} != ${JSON.stringify(expect)}`);
  }
  return vs;
}
const kindVs = variants(/class IterationCount\s*\{[\s\S]*?enum class Kind\s*\{([^}]+)\}/, "IterationCount::Kind",
  ["Exact", "Approx", "Unknown"]);
const cmpVs = variants(/enum class CmpOp\s*\{([^}]+)\}/, "CmpOp",
  ["Equal", "SignedLessThan", "SignedLessThanOrEqual", "UnsignedLessThan", "UnsignedLessThanOrEqual",
   "SignedGreaterThan", "SignedGreaterThanOrEqual", "UnsignedGreaterThan", "UnsignedGreaterThanOrEqual"]);
const binVs = variants(/enum class BinOp\s*\{([^}]+)\}/, "BinOp",
  ["Add", "Mul", "Sub", "BitwiseAnd", "BitwiseOr", "BitwiseXor",
   "OverflowCheckedAdd", "OverflowCheckedMul", "OverflowCheckedSub"]);
const unrollVs = variants(/enum class UnrollingStatus\s*\{([\s\S]*?)\n  \};/m, "UnrollingStatus",
  ["NotUnrolling", "Unrolling", "RemoveLoop"]);

// 2. 四类面哨兵
for (const [re, label] of [
  [/class IterationCount\s*\{/, "IterationCount"],
  [/class V8_EXPORT_PRIVATE StaticCanonicalForLoopMatcher/, "StaticCanonicalForLoopMatcher (V8_EXPORT_PRIVATE)"],
  [/class LoopStackCheckElisionReducer : public Next/, "LoopStackCheckElisionReducer"],
  [/class LoopUnrollingReducer : public Next/, "LoopUnrollingReducer"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(LoopStackCheckElision\)/, "BOILERPLATE(LoopStackCheckElision)"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(LoopUnrolling\)/, "BOILERPLATE(LoopUnrolling)"],
  [/IterationCount GetIterCountIfStaticCanonicalForLoop\(/, "GetIterCountIfStaticCanonicalForLoop"],
  [/static constexpr CmpOp ComparisonKindToCmpOp\(ComparisonOp::Kind kind\);/, "ComparisonKindToCmpOp"],
  [/static constexpr CmpOp InvertComparisonOp\(CmpOp op\);/, "InvertComparisonOp"],
  [/static constexpr BinOp BinopFromWordBinopKind\(WordBinopOp::Kind kind\);/, "BinopFromWordBinopKind"],
  [/bool RemoveLoop\(const Block\* header\);/, "RemoveLoop"],
  [/bool FullyUnrollLoop\(const Block\* header\);/, "FullyUnrollLoop"],
  [/bool CanUnrollAtLeastOneLoop\(\) const \{ return can_unroll_at_least_one_loop_; \}/, "CanUnrollAtLeastOneLoop (h 内联)"],
]) {
  if (!re.test(h)) throw new Error(`h 哨兵缺失: ${label}`);
}

// 3. cc 侧: LoopUnrollingAnalyzer 与 matcher 方法定义表
for (const [re, label] of [
  [/void LoopUnrollingAnalyzer::DetectUnrollableLoops\(\)/, "DetectUnrollableLoops"],
  [/IterationCount LoopUnrollingAnalyzer::GetLoopIterationCount\(/, "GetLoopIterationCount"],
  [/bool StaticCanonicalForLoopMatcher::MatchPhiCompareCst\(/, "MatchPhiCompareCst"],
  [/bool StaticCanonicalForLoopMatcher::MatchCheckedOverflowBinop\(/, "MatchCheckedOverflowBinop"],
  [/bool StaticCanonicalForLoopMatcher::MatchWordBinop\(/, "MatchWordBinop"],
  [/using CmpOp = StaticCanonicalForLoopMatcher::CmpOp;/, "cc CmpOp 别名"],
  [/using BinOp = StaticCanonicalForLoopMatcher::BinOp;/, "cc BinOp 别名"],
  [/std::ostream& operator<<\(std::ostream& os, const IterationCount& count\)/, "IterationCount operator<<"],
]) {
  if (!re.test(cc)) throw new Error(`cc 哨兵缺失: ${label}`);
}

// 4. 生成 Rust
function rustEnum(name, vs, ctx) {
  return `/// v8: src/compiler/turboshaft/loop-unrolling-reducer.h ${name} — ${ctx}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ${name} {
${vs.map((v) => `  /// k${v}`).join("\n")}
}

impl ${name} {
  pub const ALL: &'static [${name}] = &[
${vs.map((v) => `    ${name}::${v.replace(/^./, (c) => c.toLowerCase())},`).join("\n")}
  ];
}
`;
}
let rust = banner(
  "src/compiler/turboshaft/loop-unrolling-reducer.h.js",
  "src/compiler/turboshaft/loop-unrolling-reducer.{h,cc}",
  "循环展开: 静态规范 for 循环匹配与展开状态机骨架",
);
rust += "\n";
rust += rustEnum("IterationCountKind", kindVs, "IterationCount::Kind (Exact 可全展开 / Approx 可去栈检查 / Unknown)");
rust += rustEnum("CmpOp", cmpVs, "StaticCanonicalForLoopMatcher::CmpOp (9 种比较)");
rust += rustEnum("BinOp", binVs, "StaticCanonicalForLoopMatcher::BinOp (含溢出检查变体)");
rust += rustEnum("UnrollingStatus", unrollVs, "LoopUnrollingReducer 状态机 (kRemoveLoop 双语义: 末次 header / 删环)");
rust += `
/// v8: src/compiler/turboshaft/loop-unrolling-reducer.h IterationCount
/// kind + count 载体; 构造: Exact(count)/Approx(count)/Unknown()
pub struct IterationCount {
  pub kind: IterationCountKind,
  pub count: usize,
}

/// v8: src/compiler/turboshaft/loop-unrolling-reducer.h StaticCanonicalForLoopMatcher
/// 静态规范 for 循环匹配器 (phi 比较 + 溢出检查二元组), cc 定义面:
/// MatchPhiCompareCst / MatchCheckedOverflowBinop / MatchWordBinop
pub struct StaticCanonicalForLoopMatcher;

/// v8: src/compiler/turboshaft/loop-unrolling-reducer.cc LoopUnrollingAnalyzer
/// DetectUnrollableLoops / GetLoopIterationCount / CanUnrollAtLeastOneLoop
pub struct LoopUnrollingAnalyzer;

/// v8: src/compiler/turboshaft/loop-unrolling-reducer.h LoopStackCheckElisionReducer
/// 小迭代数 (Approx) 循环的栈检查消除
pub struct LoopStackCheckElisionReducer;

/// v8: src/compiler/turboshaft/loop-unrolling-reducer.h LoopUnrollingReducer
pub struct LoopUnrollingReducer;

impl LoopUnrollingReducer {
  /// 原版 bool RemoveLoop(const Block* header)
  pub fn remove_loop(&mut self) -> bool {
    unimplemented!("签名骨架, Developer 修订")
  }
  /// 原版 bool FullyUnrollLoop(const Block* header)
  pub fn fully_unroll_loop(&mut self) -> bool {
    unimplemented!("签名骨架, Developer 修订")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(IterationCountKind::ALL.len() == ${kindVs.length});
  assert!(CmpOp::ALL.len() == ${cmpVs.length});
  assert!(BinOp::ALL.len() == ${binVs.length});
  assert!(UnrollingStatus::ALL.len() == ${unrollVs.length});
};
`;

const p = writeSrc("v8-turboshaft", "loop_unrolling_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`枚举=4 (${kindVs.length}/${cmpVs.length}/${binVs.length}/${unrollVs.length} 变体) 类面=4+IterationCount cc锚=9`);
