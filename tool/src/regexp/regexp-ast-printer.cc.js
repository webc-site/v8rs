#!/usr/bin/env bun
// tool/src/regexp/regexp-ast-printer.cc.js — v8/src/regexp/regexp-ast-printer.{h,cc}
//   → v8-regexp-syntax/src/ast_printer_generated.rs
// 机械提取: FOR_EACH_REG_EXP_TREE_TYPE 展开的 Visit 分发表 (14) + 逐节点
//   Dump 前缀/格式串 (从 cc 各 Visit 体提取) + Assertion::Type 7 态标签
//   + ClassSetExpression 操作 3 态记号 + Quantifier 贪婪三态 + TraceTreeScope 深度规则;
// 对数断言: 树种类 14 (与 regexp-ast.h 宏交叉) / 断言标签 7 (与 ast.h 枚举交叉) /
//   集合运算 3 / cc 实现的 Visit 数 == 宏展开数。
import { banner, readV8, writeSrc } from "../../lib.js";

const h = readV8("src/regexp/regexp-ast-printer.h");
const cc = readV8("src/regexp/regexp-ast-printer.cc");
const astH = readV8("src/regexp/regexp-ast.h");

function must(re, what, src = cc) {
  const m = src.match(re);
  if (!m) throw new Error(`regexp-ast-printer: 未找到 ${what}`);
  return m;
}

// ── 树种类宏展开 (regexp-ast.h:22-34) ──────────────────────────────────────
const mk = must(/#define FOR_EACH_REG_EXP_TREE_TYPE\(VISIT\)([\s\S]*?)\n\n/, "FOR_EACH_REG_EXP_TREE_TYPE", astH);
const kinds = [...mk[1].matchAll(/VISIT\((\w+)\)/g)].map((r) => r[1]);
if (kinds.length !== 14) throw new Error(`树种类应 14, 实数 ${kinds.length}`);

// h: 声明的 Visit 集合 == 宏展开
if (!h.includes("FOR_EACH_REG_EXP_TREE_TYPE(DECLARE_VISIT)")) throw new Error("h 缺宏声明展开");
if (!h.includes("using AstNodePrinter = NodePrinter<Tree>;")) throw new Error("AstNodePrinter 别名漂移");

// cc: 逐 Visit 实现抽取
const visits = new Map();
for (const kind of kinds) {
  const re = new RegExp(`void\\* AstNodePrinter::Visit${kind}\\(${kind}\\* that,\\s*void\\* data\\) \\{([\\s\\S]*?)\\n\\}`);
  const m = cc.match(re);
  if (!m) throw new Error(`cc 缺 Visit${kind} 实现`);
  visits.set(kind, m[1]);
}
if (visits.size !== kinds.length) throw new Error(`cc Visit 实现应 ${kinds.length}`);
// VisitCharacterRange 单独 (非宏成员, CharacterRange 是值类型)
if (!cc.includes("void AstNodePrinter::VisitCharacterRange(CharacterRange that) {")) {
  throw new Error("cc 缺 VisitCharacterRange");
}

// ── 逐节点前缀串 (cc 内首个 os() 字面量) ─────────────────────────────────
function firstLiteral(kind) {
  const m = visits.get(kind).match(/os\(\) << "((?:[^"\\]|\\.)*)"|os\(\) << '((?:[^'\\]|\\.)*)'/);
  if (!m) throw new Error(`Visit${kind} 无首字面量`);
  return m[1] ?? m[2];
}
// 人工锚 (与源一致; 分支首 case / 首条件)
const EXPECT_PREFIX = {
  Disjunction: "(|",
  Alternative: "(:",
  ClassRanges: "^", // is_negated 分支在前, 否则 "["
  ClassSetOperand: "![",
  ClassSetExpression: "++", // switch 首 case kUnion
  Assertion: "@^i", // switch 首 case START_OF_INPUT
  Atom: "'",
  Text: "(!", // 多元素分支
  Quantifier: "(# ",
  Capture: "(^ ",
  Group: "(?",
  Lookaround: "(",
  BackReference: "(<- ",
  Empty: "%",
};
for (const [kind, want] of Object.entries(EXPECT_PREFIX)) {
  const got = firstLiteral(kind);
  if (got !== want) throw new Error(`Visit${kind} 前缀漂移: 期望 "${want}" 实得 "${got}"`);
}

// ── Assertion 7 态标签 (cc VisitAssertion switch) ─────────────────────────
const asrtCases = [...visits.get("Assertion").matchAll(
  /case Assertion::Type::(\w+):\s*\n\s*os\(\) << "((?:[^"\\]|\\.)*)";/g,
)].map((r) => ({ type: r[1], tag: r[2] }));
if (asrtCases.length !== 7) throw new Error(`断言标签应 7, 实数 ${asrtCases.length}`);
// 交叉: ast.h Assertion::Type 枚举 7 项, 名字与 switch case 集合全等
const atEnum = must(/enum class Type \{([\s\S]*?)\s*LAST_ASSERTION_TYPE/, "Assertion::Type", astH)[1];
const atNames = [...atEnum.matchAll(/(\w+) = (\d+)/g)].map((r) => r[1]);
if (atNames.length !== 7) throw new Error(`ast.h 断言枚举应 7, 实数 ${atNames.length}`);
const caseSet = new Set(asrtCases.map((c) => c.type));
if (caseSet.size !== atNames.length || atNames.some((n) => !caseSet.has(n))) {
  throw new Error("断言 case 与枚举名集不合");
}
// 标签值人工锚
const EXPECT_TAG = {
  START_OF_INPUT: "@^i",
  END_OF_INPUT: "@$i",
  END_OF_BUFFER: "@$Z",
  START_OF_LINE: "@^l",
  END_OF_LINE: "@$l",
  BOUNDARY: "@b",
  NON_BOUNDARY: "@B",
};
for (const c of asrtCases) {
  if (EXPECT_TAG[c.type] !== c.tag) throw new Error(`断言 ${c.type} 标签漂移: ${c.tag}`);
}

// ── ClassSetExpression 操作 3 态记号 ──────────────────────────────────────
const ops = [...visits.get("ClassSetExpression").matchAll(
  /case ClassSetExpression::OperationType::k(\w+):\s*\n\s*os\(\) << "((?:[^"\\]|\\.)*)";/g,
)].map((r) => ({ op: r[1], token: r[2] }));
if (ops.length !== 3) throw new Error(`集合运算应 3, 实数 ${ops.length}`);
const EXPECT_OP = { Union: "++", Intersection: "&&", Subtraction: "--" };
for (const o of ops) {
  if (EXPECT_OP[o.op] !== o.token) throw new Error(`运算 ${o.op} 记号漂移: ${o.token}`);
}

// ── Quantifier 贪婪三态 (cc VisitQuantifier) ──────────────────────────────
const q = visits.get("Quantifier");
if (!q.includes('os() << (that->is_greedy() ? "g " : that->is_possessive() ? "p " : "n ");')) {
  throw new Error("贪婪三态格式漂移");
}
if (!q.includes("that->max() == Tree::kInfinity")) throw new Error("kInfinity 上限分支缺失");

// ── Lookaround 方向二态 ────────────────────────────────────────────────────
const lk = visits.get("Lookaround");
if (!lk.includes('LOOKAHEAD ? "->" : "<-"')) throw new Error("环视方向箭头漂移");
if (!lk.includes('is_positive() ? " + " : " - "') && !lk.includes('that->is_positive() ? " + " : " - "')) {
  throw new Error("环视正负号漂移");
}

// ── VisitCharacterRange 规则 ──────────────────────────────────────────────
if (!cc.includes("if (!that.IsSingleton())")) throw new Error("单点区间规则缺失");
if (!cc.includes('os() << "-" << AsUC32(that.to())')) throw new Error("区间上界输出缺失");

// ── TraceTreeScope 深度规则 (cc:226-255) ─────────────────────────────────
if (!cc.includes("depth_(parent_ != nullptr ? parent_->depth_ + 1 : -1)")) {
  throw new Error("嵌套深度起点 -1 规则漂移");
}
if (!cc.includes('for (int i = 0; i < depth_; i++) os << "  ";')) {
  throw new Error("每层 2 空格缩进规则漂移");
}
if (!cc.includes("printer->set_color(PrinterBase::Color::kRed)")) throw new Error("树打印红色规则缺失");

console.log(
  `regexp-ast-printer: 树种类 ${kinds.length}, 断言标签 ${asrtCases.length}, 集合运算 ${ops.length}, ` +
    `前缀锚 ${Object.keys(EXPECT_PREFIX).length}, 三态/二态 2 ✓`,
);

// ── 生成 Rust ──────────────────────────────────────────────────────────────
const body = `${banner(
  "src/regexp/regexp-ast-printer.cc.js",
  "src/regexp/regexp-ast-printer.{h,cc}",
  "AST Dump 分发表与格式常量 (X-macro 展开); 流式输出实现面在 ast.rs, 就地修订",
)}
/// AST 节点种类 (FOR_EACH_REG_EXP_TREE_TYPE 展开, 与 ast_generated::TreeKind 同源)
pub const TREE_KINDS: [&str; ${kinds.length}] = [
${kinds.map((k) => `  "${k}",`).join("\n")}
];

/// 逐节点 Dump 前缀 (原版各 Visit 首字面量; ClassRanges 的 ^ 视 is_negated 前置)
pub const TREE_KIND_PREFIXES: [(&str, &str); ${kinds.length}] = [
${kinds.map((k) => `  ("${k}", "${EXPECT_PREFIX[k].replace(/"/g, '\\"')}"),`).join("\n")}
];

/// 断言类型 → Dump 标签 (cc:108-130, 与 ast.h Assertion::Type 7 项一一对应)
pub const ASSERTION_TAGS: [(&str, &str); ${asrtCases.length}] = [
${asrtCases.map((c) => `  ("${c.type}", "${c.tag}"),`).join("\n")}
];

/// ClassSetExpression 操作 → 记号 (cc:84-94)
pub const CLASS_SET_OP_TOKENS: [(&str, &str); ${ops.length}] = [
${ops.map((o) => `  ("${o.op}", "${o.token}"),`).join("\n")}
];

/// 量词贪婪三态输出 (cc:171: greedy g / possessive p / 其余 n)
pub const K_QUANT_GREEDY: &str = "g ";
pub const K_QUANT_POSSESSIVE: &str = "p ";
pub const K_QUANT_NON_GREEDY: &str = "n ";

/// 环视方向箭头 (cc:199: LOOKAHEAD -> / 其余 <-)
pub const K_LOOKAHEAD_ARROW: &str = "->";
pub const K_LOOKBEHIND_ARROW: &str = "<-";
/// 环视正负号 (cc:200)
pub const K_POSITIVE_SEP: &str = " + ";
pub const K_NEGATIVE_SEP: &str = " - ";

/// 区间输出: 单点仅 from, 否则 "from-to" (cc:17-22)
pub const K_RANGE_SEPARATOR: &str = "-";
/// 字符串集元素引号 (cc:71-73)
pub const K_STRING_QUOTE: &str = "'";

/// 嵌套树打印: 每层 2 空格缩进, 顶层 depth 为 -1 (cc:228/253)
pub const K_TRACE_INDENT: &str = "  ";
pub const K_TRACE_ROOT_DEPTH: i32 = -1;
/// 树打印着色 (cc:244: kRed, 完后复位 kDefault)
pub const K_TREE_PRINT_COLOR: i32 = 31;

const _: () = assert!(TREE_KINDS.len() == ${kinds.length});
const _: () = assert!(ASSERTION_TAGS.len() == 7);
const _: () = assert!(CLASS_SET_OP_TOKENS.len() == 3);
`;

writeSrc("v8-regexp-syntax", "ast_printer_generated.rs", body);
console.log("written: v8-regexp-syntax/src/ast_printer_generated.rs");
