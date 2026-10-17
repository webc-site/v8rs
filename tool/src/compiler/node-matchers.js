#!/usr/bin/env bun
// tool/src/compiler/node-matchers.js
// v8/src/compiler/node-matchers.h + node-matchers.cc
// → v8-compiler-passes/src/node_matchers.rs.skel
// 节点模式匹配器族: 解析 matcher 结构清单 / using 别名表 /
// DisplacementMode 枚举 / AddressOption 位标志 / ScaleMatcher 的 scale 表。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/node-matchers.h");
const cc = readV8("src/compiler/node-matchers.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`node-matchers: ${msg}`);
};

// —— 哨兵 ——
for (const [s, name] of [
  ["struct NodeMatcher {", "NodeMatcher 定义"],
  ["struct ScaleMatcher {", "ScaleMatcher 定义"],
  ["struct BaseWithIndexAndDisplacementMatcher {", "BWID matcher 定义"],
  ["enum DisplacementMode { kPositiveDisplacement, kNegativeDisplacement };", "DisplacementMode"],
  ["enum class AddressOption : uint8_t {", "AddressOption"],
  ["bool NodeMatcher::IsComparison() const {", "cc IsComparison"],
  ["BranchMatcher::BranchMatcher(Node* branch)", "cc BranchMatcher"],
  ["DiamondMatcher::DiamondMatcher(Node* merge)", "cc DiamondMatcher"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— 1. ScaleMatcher scale 表 (真解析 Initialize 分支) ——
const scaleFn = h.slice(h.indexOf("struct ScaleMatcher {"));
const scaleTable = [];
{
  // mul/shift 主表: if (value == N) { scale_ = S; }
  for (const m of scaleFn.matchAll(/if \(value == (\d+)\) \{\s*scale_ = (\d+);\s*\}/g)) {
    scaleTable.push({ value: Number(m[1]), scale: Number(m[2]), p2p1: false });
  }
  // p2p1 附加表 (需 allow_power_of_two_plus_one)
  for (const m of scaleFn.matchAll(/if \(value == (\d+)\) \{\s*scale_ = (\d+);\s*power_of_two_plus_one_ = true;/g)) {
    scaleTable.push({ value: Number(m[1]), scale: Number(m[2]), p2p1: true });
  }
  // shift 分支: value ∈ [0,3] → scale = value
  need(/value >= 0 && value <= 3/.test(scaleFn), "shift 分支哨兵");
  for (const [v, s] of [[1, 0], [2, 1], [4, 2], [8, 3]]) {
    need(scaleTable.some((e) => e.value === v && e.scale === s && !e.p2p1), `mul 主表缺 ${v}->${s}`);
  }
  for (const [v, s] of [[3, 1], [5, 2], [9, 3]]) {
    need(scaleTable.some((e) => e.value === v && e.scale === s && e.p2p1), `p2p1 附加表缺 ${v}->${s}`);
  }
}

// —— 2. matcher 结构清单 (跨行继承子句容错) ——
const structs = [...h.matchAll(/\bstruct (?:V8_EXPORT_PRIVATE )?(\w+)(?:\s+final)?(?:[^;{\n]*|\s*:[^{]*)\{/g)].map((m) => m[1]);
for (const s of ["NodeMatcher", "ValueMatcher", "IntMatcher", "FloatMatcher", "BoxedFloatMatcher", "HeapObjectMatcherImpl", "ExternalReferenceMatcher", "LoadMatcher", "BinopMatcher", "ScaleMatcher", "AddMatcher", "BaseWithIndexAndDisplacementMatcher", "BranchMatcher", "DiamondMatcher", "LoadTransformMatcher"]) {
  need(structs.includes(s), `struct ${s} 缺席 (实得 ${structs.join(",")})`);
}

// —— 3. using 别名表 (含 64 位指针条件分支解析) ——
const aliases = [];
for (const m of h.matchAll(/^using (\w+) =\s*([\s\S]*?);/gm)) {
  const rhs = m[2].replace(/\s+/g, " ").trim();
  if (/^BinopMatcher</.test(rhs)) {
    // BinopMatcher<LeftMatcher, RightMatcher, kOpA, kOpB, allow_swap>
    const inner = rhs.slice("BinopMatcher<".length, -1);
    const parts = inner.split(",").map((p) => p.trim());
    aliases.push({ name: m[1], kind: "binop", lhs: parts[0], rhs: parts[1] });
  } else if (/^[A-Za-z_]\w*$/.test(rhs)) {
    // 裸标识符 (如 IntPtrMatcher = Int64Matcher / NumberMatcher = Float64Matcher)
    aliases.push({ name: m[1], kind: "simple", of: rhs });
  } else {
    // 模板形 (IntMatcher<T, kOp> / ScaleMatcher<...> / AddMatcher<...> / BWID<...>)
    const base = rhs.match(/^(\w+)</);
    if (!base) continue;
    const kind = base[1] === "BinopMatcher" ? "binop" : ["ScaleMatcher", "AddMatcher", "BaseWithIndexAndDisplacementMatcher"].includes(base[1]) ? "composite" : "value";
    aliases.push({ name: m[1], kind, of: base[1] });
  }
}
const aliasNames = aliases.map((a) => a.name);
for (const a of ["Int32Matcher", "Uint32Matcher", "Int64Matcher", "Uint64Matcher", "IntPtrMatcher", "UintPtrMatcher", "Float32Matcher", "Float64Matcher", "NumberMatcher", "HeapObjectMatcher", "Int32BinopMatcher", "Float64BinopMatcher", "Int32ScaleMatcher", "Int64ScaleMatcher", "Int32AddMatcher", "Int64AddMatcher", "BaseWithIndexAndDisplacement32Matcher", "BaseWithIndexAndDisplacement64Matcher"]) {
  need(aliasNames.includes(a), `别名 ${a} 缺席`);
}

// —— 4. AddressOption 位标志求值 ——
const optBody = h.slice(h.indexOf("enum class AddressOption : uint8_t {"), h.indexOf("};", h.indexOf("enum class AddressOption")));
const optBits = {};
for (const m of optBody.matchAll(/k(\w+)\s*=\s*([^,\n]+)/g)) {
  const rhs = m[2].replace(/\s+/g, "");
  if (rhs === "0u" || rhs === "0") optBits[m[1]] = 0;
  else if (/^1u?<<\d$/.test(rhs)) optBits[m[1]] = 1 << Number(rhs.replace(/^1u?<</, ""));
  else {
    // kAllowAll = kAllowInputSwap | kAllowScale
    const val = rhs.split("|").reduce((acc, t) => acc | optBits[t.replace(/^k/, "")], 0);
    optBits[m[1]] = val;
  }
}
need(optBits.AllowInputSwap === 1 && optBits.AllowScale === 2 && optBits.AllowAll === 3, `AddressOption 位值 ${JSON.stringify(optBits)}`);

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
const cap = (s) => s.toUpperCase();

const body = `${banner("src/compiler/node-matchers.js", "src/compiler/node-matchers.h + node-matchers.cc", "节点模式匹配器族: 常量匹配/二元匹配/地址分解匹配")}
//! node-matchers — 节点模式匹配器 (只读查询, 不改图)
//!
//! 层次: NodeMatcher (opcode 面) → ValueMatcher<T> (常量参数面, 跳过 value
//! identity) → Int/Float/BoxedFloat/HeapObject 特化 → BinopMatcher (左右操作
//! 数 + 可交换换位) → Scale/Add/BaseWithIndexAndDisplacement (寻址分解)。
//! \`Is##Opcode()\` 全 opcode 谓词由 IrOpcode 判等直接覆盖 (见 v8-compiler-graph
//! opcodes), 不在此重表。

/// 原版 DisplacementMode — 位移符号
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplacementMode {
  /// kPositiveDisplacement
  Positive,
  /// kNegativeDisplacement
  Negative,
}

/// 原版 AddressOption 位标志 (base::Flags<AddressOption, uint8_t>)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AddressOptions(u8);

impl AddressOptions {
  /// kAllowNone
  pub const NONE: Self = Self(0);
  /// kAllowInputSwap = 1 << 0
  pub const ALLOW_INPUT_SWAP: Self = Self(${optBits.AllowInputSwap});
  /// kAllowScale = 1 << 1
  pub const ALLOW_SCALE: Self = Self(${optBits.AllowScale});
  /// kAllowAll = kAllowInputSwap | kAllowScale
  pub const ALLOW_ALL: Self = Self(${optBits.AllowAll});

  /// 位域
  pub const fn bits(self) -> u8 {
    self.0
  }
}

impl core::ops::BitOr for AddressOptions {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self {
    Self(self.0 | rhs.0)
  }
}

/// 原版 ScaleMatcher — 乘/移位寻址缩放识别表
///
/// shift 分支: value ∈ [0,3] → scale = value;
/// mul 分支主表 (2 的幂) 与 p2p1 附加表 (需 allow_power_of_two_plus_one):
pub const SCALE_TABLE: [(i64, i8, bool); ${scaleTable.length}] = [
${scaleTable.map((e) => `  (${e.value}, ${e.scale}, ${e.p2p1 ? "true" : "false"}),`).join("\n")}
];

/// matcher 结构清单 (Rust 侧以泛型/组合重建, 各自签名见锚)
${structs.map((s) => `/// v8: src/compiler/node-matchers.h struct ${s}`).join("\n")}

/// using 别名 → Rust 类型别名 (64 位宿主: IntPtr = Int64)
pub type Int32Matcher = ValueMatcherI32;
pub type Uint32Matcher = ValueMatcherU32;
pub type Int64Matcher = ValueMatcherI64;
pub type Uint64Matcher = ValueMatcherU64;
pub type IntPtrMatcher = Int64Matcher;
pub type UintPtrMatcher = Uint64Matcher;
${aliases.filter((a) => a.kind === "simple" && !["Int32Matcher", "Uint32Matcher", "Int64Matcher", "Uint64Matcher", "IntPtrMatcher", "UintPtrMatcher"].includes(a.name)).map((a) => `pub type ${a.name} = ${a.of};`).join("\n")}
${aliases.filter((a) => a.kind === "value").map((a) => `/// v8: ${a.name} = ${a.of}<...>\npub type ${a.name} = ${a.of}<T, K>;`).join("\n")}
${aliases.filter((a) => a.kind === "binop").map((a) => `/// v8: ${a.name} = BinopMatcher<${a.lhs}, ${a.rhs}, ...>\npub type ${a.name} = BinopMatcher<${a.lhs}, ${a.rhs}>;`).join("\n")}
${aliases.filter((a) => a.kind === "composite").map((a) => `/// v8: ${a.name} = ${a.of}<...>\npub type ${a.name} = ${a.of}<Int32AddMatcher>;`).join("\n")}

/// 原版 NodeMatcher — opcode 查询面
/// v8: src/compiler/node-matchers.cc NodeMatcher::IsComparison
pub fn is_comparison(opcode: IrOpcode) -> bool {
  todo!("v8: src/compiler/node-matchers.cc NodeMatcher::IsComparison")
}

/// 原版 BranchMatcher — Branch + IfTrue/IfFalse 投影匹配
/// v8: src/compiler/node-matchers.cc BranchMatcher::BranchMatcher
pub struct BranchMatcher<'n> {
  /// if_true_
  pub if_true: Option<&'n Node>,
  /// if_false_
  pub if_false: Option<&'n Node>,
}

/// 原版 DiamondMatcher — 分支-合并菱形匹配
/// v8: src/compiler/node-matchers.cc DiamondMatcher::DiamondMatcher
pub struct DiamondMatcher<'n> {
  /// branch_
  pub branch: Option<&'n Node>,
  /// if_true_
  pub if_true: &'n Node,
  /// if_false_
  pub if_false: &'n Node,
}

impl DiamondMatcher<'_> {
  /// 原版 IfProjectionsAreOwned — 两投影均被 merge 独占
  /// v8: src/compiler/node-matchers.h DiamondMatcher::IfProjectionsAreOwned
  pub fn if_projections_are_owned(&self) -> bool {
    todo!("v8: src/compiler/node-matchers.h DiamondMatcher::IfProjectionsAreOwned")
  }

  /// 原版 TrueInputOf — phi 的 true 侧输入
  /// v8: src/compiler/node-matchers.h DiamondMatcher::TrueInputOf
  pub fn true_input_of(&self, phi: &Node) -> Option<&Node> {
    todo!("v8: src/compiler/node-matchers.h DiamondMatcher::TrueInputOf")
  }

  /// 原版 FalseInputOf — phi 的 false 侧输入
  /// v8: src/compiler/node-matchers.h DiamondMatcher::FalseInputOf
  pub fn false_input_of(&self, phi: &Node) -> Option<&Node> {
    todo!("v8: src/compiler/node-matchers.h DiamondMatcher::FalseInputOf")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "node_matchers.rs.skel", body);
console.log(
  [
    `written ${p}`,
    `structs=${structs.length} aliases=${aliases.length} (binop=${aliases.filter((a) => a.kind === "binop").length})`,
    `scale_table=${scaleTable.length} 项 (p2p1=${scaleTable.filter((e) => e.p2p1).length})`,
    `address_option bits=${JSON.stringify(optBits)}`,
  ].join("\n"),
);
