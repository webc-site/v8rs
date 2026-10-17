#!/usr/bin/env bun
// tool/src/regexp/regexp-nodes.h.js
// 机械提取 v8/src/regexp/regexp-nodes.h 的节点枚举与预算常量
// (枚举嵌于各节点类内, 按所属类归组登记; 语义实现面跳过)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/regexp/regexp-nodes.h");

function enumBody(re, what) {
  const m = hSrc.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  let names = [...m[1].matchAll(/^\s*(k?\w+),?\s*(?:\/\/.*)?$/gm)].map((x) => x[1]).filter(Boolean);
  if (names.length === 0) {
    names = m[1].split(",").map((x) => x.trim()).filter(Boolean);
  }
  if (names.length === 0) throw new Error(`${what} 为空`);
  return names;
}

const groups = [
  { mod: "drain_mode", name: "DrainMode", re: /enum class DrainMode : uint8_t \{([\s\S]*?)\n\};/, doc: "定长循环排空策略 (DrainMode)" },
  { mod: "atomic_loop_kind", name: "AtomicLoopKind", re: /enum class AtomicLoopKind : uint8_t \{([\s\S]*?)\n\};/, doc: "原子循环续体类别 (AtomicLoopKind)" },
  { mod: "parked_grant", name: "ParkedGrant", re: /enum class ParkedGrant : uint8_t \{([\s\S]*?)\n\};/, doc: "循环出口位置停留许可 (ParkedGrant)" },
  { mod: "limit_result", name: "LimitResult", re: /enum LimitResult \{ ([\w, ]+?) \};/, doc: "版本限制结果 (RegExpNode::LimitResult)" },
  { mod: "action_type", name: "ActionType", re: /enum ActionType \{([\s\S]*?)\n  \};/, doc: "延迟动作类别 (Trace::ActionType)" },
  { mod: "text_emit_pass_type", name: "TextEmitPassType", re: /enum TextEmitPassType \{([\s\S]*?)\n  \};/, doc: "文本发射趟次 (TextNode::TextEmitPassType)" },
  { mod: "assertion_type", name: "AssertionType", re: /enum AssertionType \{([\s\S]*?)\n  \};/, doc: "断言类别 (AssertionNode::AssertionType)" },
  { mod: "if_previous", name: "IfPrevious", re: /enum IfPrevious \{ ([\w, ]+?) \};/, doc: "回溯前提 (AssertionNode::IfPrevious)" },
  { mod: "end_action", name: "Action", re: /enum Action \{ ([\w, ]+?) \};/, doc: "终止节点动作 (EndNode::Action)" },
  { mod: "relation", name: "Relation", re: /enum Relation \{ ([\w, ]+?) \};/, doc: "守卫比较关系 (Guard::Relation)" },
];

const parsed = {};
for (const g of groups) {
  parsed[g.name] = enumBody(g.re, g.name);
}

// 常量
function constInt(re, what) {
  const m = hSrc.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m[1].trim();
}
const largeEats = parseInt(constInt(/static constexpr uint32_t kLargeEatsAtLeastValue = (\d+);/, "kLargeEatsAtLeastValue"), 10);
const tooComplex = constInt(/static const int kNodeIsTooComplexForFixedLengthLoops = (kMinInt);/, "kNodeIsTooComplexForFixedLengthLoops");
const recursionBudget = parseInt(constInt(/static const int kRecursionBudget = (\d+);/, "kRecursionBudget"), 10);
const maxCopies = parseInt(constInt(/static const int kMaxCopiesCodeGenerated = (\d+);/, "kMaxCopiesCodeGenerated"), 10);
const firstCharBudget = parseInt(constInt(/static const int kFirstCharBudget = (\d+);/, "kFirstCharBudget"), 10);
const lookaroundIndex = parseInt(constInt(/static constexpr int kLookaroundIndex = (\d+);/, "kLookaroundIndex"), 10);
const continueIndex = parseInt(constInt(/static constexpr int kContinueIndex = (\d+);/, "kContinueIndex"), 10);

// 人工对数锚 (与源一致)
if (largeEats !== 255) throw new Error("kLargeEatsAtLeastValue 不符");
if (recursionBudget !== 200) throw new Error("kRecursionBudget 不符");
if (maxCopies !== 10) throw new Error("kMaxCopiesCodeGenerated 不符");
if (firstCharBudget !== 10) throw new Error("kFirstCharBudget 不符");

const total = Object.values(parsed).reduce((a, b) => a + b.length, 0);
console.log(`regexp-nodes.h: enums=${groups.length} entries=${total}`);

let rust = banner(
  "src/regexp/regexp-nodes.h.js",
  "src/regexp/regexp-nodes.h",
  "正则节点图枚举与预算常量 (枚举 1:1; 语义实现面跳过)",
);

for (const g of groups) {
  const names = parsed[g.name];
  rust += `
/// ${g.doc} (共 ${names.length} 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ${g.name} {
`;
  for (const [i, n] of names.entries()) {
    rust += `  /// ${n}\n  ${n.replace(/^k/, "")} = ${i},\n`;
  }
  rust += `}
`;
}

rust += `
/// "吃入数"哨兵 (原版 RegExpNode::kLargeEatsAtLeastValue; 超此值走保守路径)
pub const LARGE_EATS_AT_LEAST_VALUE: u32 = ${largeEats};

/// 定长循环复杂度哨兵 (原版 kNodeIsTooComplexForFixedLengthLoops = kMinInt)
pub const NODE_IS_TOO_COMPLEX_FOR_FIXED_LENGTH_LOOPS: i32 = i32::MIN;

/// 递归预算 (原版 kRecursionBudget)
pub const RECURSION_BUDGET: i32 = ${recursionBudget};

/// 单节点代码生成拷贝上限 (原版 kMaxCopiesCodeGenerated; 达到即禁内联)
pub const MAX_COPIES_CODE_GENERATED: i32 = ${maxCopies};

/// 首字符预算 (原版 Trace::kFirstCharBudget)
pub const FIRST_CHAR_BUDGET: i32 = ${firstCharBudget};

/// 前瞻环视槽位索引 (原版 NegativeLookaroundChoiceNode::kLookaroundIndex)
pub const LOOKAROUND_INDEX: i32 = ${lookaroundIndex};

/// 续体槽位索引 (原版 kContinueIndex)
pub const CONTINUE_INDEX: i32 = ${continueIndex};

// —— 提取不变式断言 ——
const _: () = {
  assert!(LARGE_EATS_AT_LEAST_VALUE == 255);
  assert!(RECURSION_BUDGET == 200);
  assert!(MAX_COPIES_CODE_GENERATED == 10);
  assert!(FIRST_CHAR_BUDGET == 10);
  assert!(LOOKAROUND_INDEX == 0 && CONTINUE_INDEX == 1);
  assert!(DrainMode::Full as u8 == 0);
  assert!(EndAction::Accept as u8 == 0);
};
`;

const p = writeSrc("v8-regexp-compiler", "nodes_enums_generated.rs", rust);
console.log(`written ${p}`);
