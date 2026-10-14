#!/usr/bin/env bun
// tool/src/regexp/regexp-node-printer.cc.js — v8/src/regexp/regexp-node-printer.{h,cc}
//   → v8-regexp-syntax/src/node_printer_generated.rs
// 机械提取: FOR_EACH_NODE_TYPE 展开的 NodePrinter<Node> Visit 分发表 (9) +
//   逐节点 PrintNodeLabel 名 + EndNode 3 动作输出 + ActionNode 10 动作文本 +
//   AssertionNode 5 态文本 + Guard 2 比较符 + 标记符 (⇝/↩/!^) + 着色规则;
// 对数断言: Visit 9 / End 3 / Action 10 (与 ActionType 枚举交叉) / 断言 5 /
//   Guard 2。
import { banner, readV8, writeSrc } from "../../lib.js";

const h = readV8("src/regexp/regexp-node-printer.h");
const cc = readV8("src/regexp/regexp-node-printer.cc");
const nodesH = readV8("src/regexp/regexp-nodes.h");

function must(re, what, src = cc) {
  const m = src.match(re);
  if (!m) throw new Error(`regexp-node-printer: 未找到 ${what}`);
  return m;
}
// C 字符串字面量 → 运行期值
const cUnescape = (s) => s.replace(/\\(.)/g, (_, c) => (c === "n" ? "\n" : c === "t" ? "\t" : c));

// ── FOR_EACH_NODE_TYPE 9 ──────────────────────────────────────────────────
const nodeTypes = [...must(/#define FOR_EACH_NODE_TYPE\(VISIT\)([\s\S]*?)\n\n/, "FOR_EACH_NODE_TYPE", nodesH)[1]
  .matchAll(/VISIT\((\w+)\)/g)].map((r) => r[1]);
if (nodeTypes.length !== 9) throw new Error(`节点类型应 9, 实数 ${nodeTypes.length}`);
if (!h.includes("FOR_EACH_NODE_TYPE(DECLARE_VISIT)")) throw new Error("h 缺宏声明展开");
if (!h.includes("using RegExpGraphNodePrinter = NodePrinter<Node>;")) throw new Error("别名漂移");

// cc: Visit 实现 9 (容忍签名换行, 按集合比较)
const impls = [...cc.matchAll(/void NodePrinter<Node>::Visit(\w+)\(\s*(\w+)Node\* (?:node|that)\)/g)].map((r) => r[1]);
if (impls.length !== 9) throw new Error(`cc Visit 实现应 9, 实数 ${impls.length}`);
if (new Set(impls).size !== 9 || nodeTypes.some((n) => !impls.includes(n))) {
  throw new Error(`Visit 实现集漂移: ${impls.join(",")}`);
}

// ── 逐节点 PrintNodeLabel 名 (cc 各 Visit 首调用) ────────────────────────
const LABEL_NAMES = { End: "End", Action: "Action", UnanchoredAdvance: "UnanchoredAdvance", Choice: "Choice", LoopChoice: "LoopChoice", NegativeLookaroundChoice: "NegativeLookaroundChoice", BackReference: "BackReference", Assertion: "Assertion", Text: "Text" };
for (const [kind, name] of Object.entries(LABEL_NAMES)) {
  const re = new RegExp(`Visit${kind}\\(\\s*\\w+Node\\* \\w+\\) \\{\\s*\\n\\s*PrintNodeLabel\\(\\w+, "(\\w+)"\\);`);
  const m = cc.match(re);
  if (!m) throw new Error(`Visit${kind} 缺 PrintNodeLabel 名`);
  if (m[1] !== name) throw new Error(`Visit${kind} 标签名漂移: ${m[1]}`);
}

// ── EndNode 3 动作输出 (cc:29-51) ────────────────────────────────────────
const endFn = must(/void NodePrinter<Node>::VisitEnd\(EndNode\* node\) \{([\s\S]*?)\n\}/, "VisitEnd");
const endCases = [...endFn[1].matchAll(/case EndNode::(\w+):\s*(?:\{)?[\s\S]*?os\(\) << "((?:[^"\\]|\\.)*)";/g)].map((r) => ({
  action: r[1], text: cUnescape(r[2]),
}));
if (endCases.length !== 3) throw new Error(`End 动作应 3, 实数 ${endCases.length}`);
// 交叉: EndNode::Action 枚举
const endEnum = must(/enum Action \{ ([\w, ]+?) \};/, "EndNode::Action", nodesH)[1].split(",").map((s) => s.trim());
if (endEnum.length !== 3) throw new Error(`EndNode::Action 枚举应 3, 实数 ${endEnum.length}`);
const endSet = new Set(endCases.map((c) => c.action));
if (endEnum.some((n) => !endSet.has(n))) throw new Error("End case 与枚举名集不合");

// ── ActionNode 10 动作文本 (cc:53-111) ───────────────────────────────────
const actFn = must(/void NodePrinter<Node>::VisitAction\(ActionNode\* node\) \{([\s\S]*?)\n\}/, "VisitAction");
const actCases = [];
{
  const segs = actFn[1].split(/(?=case ActionNode::\w+:)/);
  for (const seg of segs) {
    const head = seg.match(/^case ActionNode::(\w+):/);
    if (!head) continue;
    const lit = seg.match(/os\(\) << "((?:[^"\\]|\\.)*)"/);
    if (!lit) throw new Error(`Action case ${head[1]} 无文本字面量`);
    actCases.push({ action: head[1], text: cUnescape(lit[1]) });
  }
}
if (actCases.length !== 10) throw new Error(`Action 动作文本应 10, 实数 ${actCases.length}`);
const actionEnum = must(/enum ActionType \{([\s\S]*?)\n  \};/, "ActionType 枚举", nodesH)[1];
const actionNames = [...actionEnum.matchAll(/^\s*(\w+),?\s*(?:\/\/.*)?$/gm)].map((r) => r[1]).filter(Boolean);
if (actionNames.length !== 10) throw new Error(`ActionType 枚举应 10, 实数 ${actionNames.length}`);
const actSet = new Set(actCases.map((a) => a.action));
if (actionNames.some((n) => !actSet.has(n))) throw new Error("Action case 与枚举名集不合");

// ── AssertionNode 5 态文本 (cc:196-216) ──────────────────────────────────
const asrtCases = [...must(/void NodePrinter<Node>::VisitAssertion\(AssertionNode\* node\) \{([\s\S]*?)\n\}/, "VisitAssertion")[1]
  .matchAll(/case AssertionNode::(\w+):\s*\n\s*os\(\) << "((?:[^"\\]|\\.)*)";/g)].map((r) => ({ type: r[1], text: cUnescape(r[2]) }));
if (asrtCases.length !== 5) throw new Error(`断言态应 5, 实数 ${asrtCases.length}`);
const EXPECT_ASRT = { AT_END: "at end", AT_START: "at start", AT_BOUNDARY: "at boundary", AT_NON_BOUNDARY: "at non-boundary", AFTER_NEWLINE: "after newline" };
for (const c of asrtCases) {
  if (EXPECT_ASRT[c.type] !== c.text) throw new Error(`断言 ${c.type} 文本漂移: ${c.text}`);
}

// ── Guard 2 比较符 (cc:119-130) ──────────────────────────────────────────
const guardFn = must(/void NodePrinter<Node>::PrintGuard\(const Guard\* guard\) \{([\s\S]*?)\n\}/, "PrintGuard");
const guardOps = [...guardFn[1].matchAll(/case Guard::(\w+):\s*\n\s*os\(\) << "(.*?)";/g)].map((r) => ({ op: r[1], sym: r[2] }));
if (guardOps.length !== 2) throw new Error(`Guard 比较符应 2, 实数 ${guardOps.length}`);
if (guardOps[0].op !== "LT" || guardOps[0].sym !== "<") throw new Error("Guard LT 漂移");
if (guardOps[1].op !== "GEQ" || guardOps[1].sym !== ">=") throw new Error("Guard GEQ 漂移");

// ── 标记符与着色 ──────────────────────────────────────────────────────────
if (!cc.includes('os() << " ⇝ ";')) throw new Error("成功后继标记 ⇝ 漂移");
if (!cc.includes('os() << "↩ ";')) throw new Error("反向读标记 ↩ 漂移");
if (!cc.includes('os() << "!^ ";')) throw new Error("非起始标记 !^ 漂移");
if (!cc.includes('os() << " && ";')) throw new Error("守卫连接符 && 漂移");
if (!cc.includes('os() << "; Eats: " << node->EatsAtLeast(false) << "/";') && !cc.includes('os() << "; Eats: " <<')) {
  throw new Error("Eats 字段漂移");
}
if (!cc.includes("set_color(PrinterBase::Color::kBlue);")) throw new Error("标签蓝色漂移");
if (!cc.includes("set_color(PrinterBase::Color::kGreen);")) throw new Error("后继绿色漂移");
if (!cc.includes('os() << ": " << name << " ";')) throw new Error("标签名格式漂移");
// LoopChoice 专有字段
if (!cc.includes('os() << "greedy: "')) throw new Error("greedy 字段漂移");
if (!cc.includes('"; body can be empty: "')) throw new Error("body-can-be-empty 字段漂移");
if (!cc.includes('os() << " } [";')) throw new Error("循环体括号格式漂移");
// End 负子匹配成功 [stack: N; pos: M; clear captures a -- b]
if (!cc.includes('os() << "[stack: "')) throw new Error("负子匹配 stack 字段漂移");
if (!cc.includes("; clear captures ")) throw new Error("clear captures 字段漂移");

console.log(
  `regexp-node-printer: Visit ${impls.length}/9, End ${endCases.length}, Action ${actCases.length}, ` +
    `断言 ${asrtCases.length}, Guard ${guardOps.length}, 标记 4 ✓`,
);

// ── 生成 Rust ──────────────────────────────────────────────────────────────
const body = `${banner(
  "src/regexp/regexp-node-printer.cc.js",
  "src/regexp/regexp-node-printer.{h,cc}",
  "节点图文本打印机规则面: 标签名/动作文本/标记符/着色; 流式实现待 Developer 就地修订",
)}
/// Visit 分发全集 (FOR_EACH_NODE_TYPE 展开)
pub const NODE_PRINTER_TYPES: [&str; ${nodeTypes.length}] = [
${nodeTypes.map((n) => `  "${n}",`).join("\n")}
];

/// 逐节点标签名 (PrintNodeLabel 第二参, cc 各 Visit 首)
pub const NODE_LABEL_NAMES: [(&str, &str); ${Object.keys(LABEL_NAMES).length}] = [
${Object.entries(LABEL_NAMES).map(([k, v]) => `  ("${k}", "${v}"),`).join("\n")}
];

/// EndNode 动作 → 输出 (cc:29-51; NEGATIVE_SUBMATCH_SUCCESS 另附寄存器字段)
pub const END_ACTION_TEXTS: [(&str, &str); ${endCases.length}] = [
${endCases.map((c) => `  ("${c.action}", "${c.text}"),`).join("\n")}
];

/// ActionNode 动作 → 文本 (cc:53-111)
pub const ACTION_TEXTS: [(&str, &str); ${actCases.length}] = [
${actCases.map((a) => `  ("${a.action}", "${a.text}"),`).join("\n")}
];

/// AssertionNode 类型 → 文本 (cc:196-216)
pub const ASSERTION_TEXTS: [(&str, &str); ${asrtCases.length}] = [
${asrtCases.map((a) => `  ("${a.type}", "${a.text}"),`).join("\n")}
];

/// Guard 比较符 (cc:119-130)
pub const GUARD_LT_SYMBOL: &str = "<";
pub const GUARD_GEQ_SYMBOL: &str = ">=";

// ── 标记符 ──
/// 成功后继引导 (cc:23/163, U+21DD)
pub const K_SUCCESS_MARKER: &str = " ⇝ ";
/// 反向读标记 (cc:153/190, U+21A9)
pub const K_READ_BACKWARD_MARKER: &str = "↩ ";
/// 非起始标记 (cc:135/171)
pub const K_NOT_AT_START_MARKER: &str = "!^ ";
/// 守卫连接符 (cc:143/185)
pub const K_GUARD_CONJUNCTION: &str = " && ";
/// 标签名后缀格式 (cc:19: ": " + name + " ")
pub const K_LABEL_NAME_SUFFIX: &str = ": ";
/// Text 节点 Eats 字段 (cc:229: "; Eats: min/max")
pub const K_EATS_FIELD: &str = "; Eats: ";
/// Eats 两值分隔 (cc:230)
pub const K_EATS_SEPARATOR: &str = "/";
/// 循环体括号 (cc:156-157)
pub const K_LOOP_BODY_OPEN: &str = " } [";
pub const K_LOOP_BODY_CLOSE: &str = "]";
/// LoopChoice 专有字段 (cc:158-160)
pub const K_LOOP_GREEDY_FIELD: &str = "greedy: ";
pub const K_LOOP_BODY_EMPTY_FIELD: &str = "; body can be empty: ";
/// 负子匹配成功寄存器字段 (cc:40-47)
pub const K_NEG_SUBMATCH_STACK: &str = "[stack: ";
pub const K_NEG_SUBMATCH_POS: &str = "; pos: ";
pub const K_NEG_SUBMATCH_CLEAR: &str = "; clear captures ";
pub const K_FIELD_RANGE_JOIN: &str = " -- ";
/// 子匹配动作寄存器字段 (cc:69-105)
pub const K_SUBMATCH_STACK: &str = "begin positive submatch [stack: ";
pub const K_POSITIVE_SUBMATCH_SUCCESS: &str = "positive submatch success [stack: ";

// ── 着色 (PrinterBase::Color 色码) ──
/// 节点标签蓝 (cc:17)
pub const K_COLOR_LABEL: i32 = 34;
/// 成功后继绿 (cc:23/162)
pub const K_COLOR_SUCCESS: i32 = 32;
/// 复位 (cc:26/165)
pub const K_COLOR_DEFAULT: i32 = 0;

const _: () = assert!(NODE_PRINTER_TYPES.len() == 9);
const _: () = assert!(END_ACTION_TEXTS.len() == 3);
const _: () = assert!(ACTION_TEXTS.len() == 10);
const _: () = assert!(ASSERTION_TEXTS.len() == 5);
const _: () = assert!(GUARD_LT_SYMBOL == "<" && GUARD_GEQ_SYMBOL == ">=");
`;

writeSrc("v8-regexp-syntax", "node_printer_generated.rs", body);
console.log("written: v8-regexp-syntax/src/node_printer_generated.rs");
