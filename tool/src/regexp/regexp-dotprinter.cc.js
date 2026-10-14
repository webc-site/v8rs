#!/usr/bin/env bun
// tool/src/regexp/regexp-dotprinter.cc.js — v8/src/regexp/regexp-dotprinter.{h,cc}
//   → v8-regexp-syntax/src/dot_printer_generated.rs
// 机械提取: FOR_EACH_NODE_TYPE 展开的 DotPrinterImpl Visit 分发表 (9) +
//   dot 节点形状表 (shape/边样式) + ActionNode 10 动作 label 模板 +
//   AssertionNode 5 态 label + 属性位名 (NI/WI/SI/@) + 截断限 (40/5);
//   C 字符串字面量解码为运行期值后登记;
// 对数断言: Visit 9 (与 regexp-nodes.h 宏交叉) / Action case 10 (与 ActionType 枚举交叉) /
//   断言 case 5 / 标签值人工锚 5。
import { banner, readV8, writeSrc } from "../../lib.js";

const h = readV8("src/regexp/regexp-dotprinter.h");
const cc = readV8("src/regexp/regexp-dotprinter.cc");
const nodesH = readV8("src/regexp/regexp-nodes.h");

function must(re, what, src = cc) {
  const m = src.match(re);
  if (!m) throw new Error(`regexp-dotprinter: 未找到 ${what}`);
  return m;
}

// C 字符串字面量内容 → 运行期值 (\" → ", \\ → \, \n → LF, \t → TAB)
function cUnescape(s) {
  return s.replace(/\\(.)/g, (_, c) => (c === "n" ? "\n" : c === "t" ? "\t" : c));
}
// 运行期值 → Rust 字符串字面量内容
function rustEscape(s) {
  return s.replace(/\\/g, "\\\\").replace(/"/g, '\\"').replace(/\n/g, "\\n");
}

// ── FOR_EACH_NODE_TYPE 9 (交叉 regexp-nodes.h:34-42) ──────────────────────
const nodeTypes = [...must(/#define FOR_EACH_NODE_TYPE\(VISIT\)([\s\S]*?)\n\n/, "FOR_EACH_NODE_TYPE", nodesH)[1]
  .matchAll(/VISIT\((\w+)\)/g)].map((r) => r[1]);
if (nodeTypes.length !== 9) throw new Error(`节点类型应 9, 实数 ${nodeTypes.length}`);
if (!h.includes("static void DotPrint(const char* label, Node* node);")) throw new Error("DotPrint 入口漂移");

// cc: Visit 实现 == 宏展开
const impls = [...cc.matchAll(/void DotPrinterImpl::Visit(\w+)\(\s*(\w+)Node\* that\)/g)].map((r) => r[1]);
if (impls.length !== 9) throw new Error(`cc Visit 实现应 9, 实数 ${impls.length}`);
// cc 定义序与宏序不同 (End/Action 在后), 按集合比较
if (new Set(impls).size !== 9 || nodeTypes.some((n) => !impls.includes(n))) {
  throw new Error(`Visit 实现集漂移: ${impls.join(",")}`);
}

// ── ActionNode 10 动作 label 模板 (cc:220-271) ────────────────────────────
// 按 case 分段取首个 os_ 字面量 (容忍带大括号的 case 体)
const actionFn = must(/void DotPrinterImpl::VisitAction\(ActionNode\* that\) \{([\s\S]*?)\n\}/, "VisitAction");
const actionCases = [];
{
  const segs = actionFn[1].split(/(?=case ActionNode::\w+:)/);
  for (const seg of segs) {
    const head = seg.match(/^case ActionNode::(\w+):/);
    if (!head) continue;
    const lit = seg.match(/os_ << "((?:[^"\\]|\\.)*)"/);
    if (!lit) throw new Error(`Action case ${head[1]} 无 label 字面量`);
    actionCases.push({ action: head[1], label: cUnescape(lit[1]) });
  }
}
if (actionCases.length !== 10) throw new Error(`Action case 应 10, 实数 ${actionCases.length}`);
// 交叉: regexp-nodes.h Trace::ActionType 枚举
const actionEnum = must(/enum ActionType \{([\s\S]*?)\n  \};/, "ActionType 枚举", nodesH)[1];
const actionNames = [...actionEnum.matchAll(/^\s*(\w+),?\s*(?:\/\/.*)?$/gm)].map((r) => r[1]).filter(Boolean);
if (actionNames.length !== 10) throw new Error(`ActionType 枚举应 10, 实数 ${actionNames.length}`);
const caseSet = new Set(actionCases.map((a) => a.action));
if (actionNames.some((n) => !caseSet.has(n))) throw new Error("Action case 与枚举名集不合");

// ── AssertionNode 5 态 label (cc:194-218) ─────────────────────────────────
const asrtFn = must(/void DotPrinterImpl::VisitAssertion\(AssertionNode\* that\) \{([\s\S]*?)\n\}/, "VisitAssertion");
const asrtCases = [...asrtFn[1].matchAll(/case AssertionNode::(\w+):\s*\n\s*os_ << "((?:[^"\\]|\\.)*)";/g)].map((r) => ({
  type: r[1], label: cUnescape(r[2]),
}));
if (asrtCases.length !== 5) throw new Error(`断言 case 应 5, 实数 ${asrtCases.length}`);
// 运行期值人工锚
const EXPECT_ASRT = {
  AT_END: 'label="$", shape=septagon',
  AT_START: 'label="^", shape=septagon',
  AT_BOUNDARY: 'label="\\b", shape=septagon',
  AT_NON_BOUNDARY: 'label="\\B", shape=septagon',
  AFTER_NEWLINE: 'label="(?<=\\n)", shape=septagon',
};
for (const c of asrtCases) {
  if (EXPECT_ASRT[c.type] !== c.label) throw new Error(`断言 ${c.type} label 漂移: ${JSON.stringify(c.label)}`);
}

// ── 节点形状/样式锚 ────────────────────────────────────────────────────────
const shapeAnchors = [
  ["Choice label 头", '[shape=Mrecord, label=\\"?'],
  ["Text 外观", '", shape=box, peripheries=2];'],
  ["BackReference 外观", '[label=\\"$'],
  ["End 外观", "[style=bold, shape=point];"],
  ["UnanchoredAdvance label", "unanchored advance"],
];
for (const [kind, anchor] of shapeAnchors) {
  if (!cc.includes(anchor)) throw new Error(`形状锚缺失: ${kind}`);
}
if (!cc.includes('"..$"')) throw new Error("回引 $..$ 寄存器区间段缺失");
// 边样式
if (!cc.includes(" [style=dotted];")) throw new Error("on_failure 虚点边漂移");
if (!cc.includes("style=dashed, color=grey, arrowhead=none")) throw new Error("属性虚线边漂移");
// 图头尾 (解码后: digraph G {\n  graph [label=" ... "];\n ... })
if (!cc.includes('os_ << "digraph G {\\n  graph [label=\\"";')) throw new Error("digraph 头漂移");
if (!cc.includes('os_ << "\\"];\\n";')) throw new Error("graph label 收口漂移");
if (!cc.includes('os_ << "}" << std::endl;')) throw new Error("图收口漂移");
// 属性节点外观
if (!cc.includes("[shape=Mrecord, color=grey, fontcolor=grey, ")) throw new Error("属性节点外观漂移");
if (!cc.includes('margin=0.1, fontsize=10, label=\\"{')) throw new Error("属性 label 前缀漂移");
// label 转义规则 (PrintNode): 反斜杠双写 / 引号原样
if (!cc.includes("case '\\\\':")) throw new Error("label 反斜杠转义缺失");
if (!cc.includes('case \'"\'')) throw new Error("label 引号 case 缺失");

// ── 属性位名 + 截断限 ──────────────────────────────────────────────────────
for (const bit of ['"NI"', '"WI"', '"SI"', '"@"']) {
  if (!cc.includes(bit)) throw new Error(`属性位名缺失: ${bit}`);
}
if (!cc.includes("if (i > 40)")) throw new Error("label 截断限 40 缺失");
if (!cc.includes("if (j > 5)")) throw new Error("ranges 截断限 5 缺失");

// AttributePrinter: 分隔符 |, 位仅真值打印, 正数打印
if (!cc.includes('os_ << "|";')) throw new Error("属性分隔符漂移");
if (!cc.includes("if (!value) return;")) throw new Error("假位跳过规则漂移");
if (!cc.includes("if (value < 0) return;")) throw new Error("负数跳过规则漂移");
// VisitText: 类区间 [^...] 展示 + AsUC32
if (!cc.includes('os_ << "[";')) throw new Error("类区间展示漂移");
if (!cc.includes('if (node->is_negated()) os_ << "^";')) throw new Error("取反展示漂移");

console.log(
  `regexp-dotprinter: Visit ${impls.length}/9, Action label ${actionCases.length}, 断言 label ${asrtCases.length}, ` +
    `形状锚 ${shapeAnchors.length}, 截断限 40/5 ✓`,
);

// ── 生成 Rust ──────────────────────────────────────────────────────────────
const body = `${banner(
  "src/regexp/regexp-dotprinter.cc.js",
  "src/regexp/regexp-dotprinter.{h,cc}",
  "Graphviz dot 输出格式表: 节点形状/label 模板/属性位/截断限; 遍历调度面待 Developer 就地修订",
)}
/// dot 图 Visit 分发全集 (FOR_EACH_NODE_TYPE 展开, 与编译器节点图同源)
pub const DOT_VISIT_NODE_TYPES: [&str; ${nodeTypes.length}] = [
${nodeTypes.map((n) => `  "${n}",`).join("\n")}
];

/// ActionNode 动作 → label 模板 (cc:220-271; $N/register 为运行期插值位)
pub const DOT_ACTION_LABELS: [(&str, &str); ${actionCases.length}] = [
${actionCases.map((a) => `  ("${a.action}", "${rustEscape(a.label)}"),`).join("\n")}
];

/// AssertionNode 类型 → dot 属性串 (cc:194-218, 全部 shape=septagon)
pub const DOT_ASSERTION_ATTRS: [(&str, &str); ${asrtCases.length}] = [
${asrtCases.map((a) => `  ("${a.type}", "${rustEscape(a.label)}"),`).join("\n")}
];

// ── dot 格式常量 (运行期值) ──
/// 图头 (cc:35)
pub const K_DOT_GRAPH_HEADER: &str = "digraph G {\\n  graph [label=\\"";
/// graph label 收口 (cc:53)
pub const K_DOT_GRAPH_LABEL_END: &str = "\\"];\\n";
/// 图收口 (cc:55)
pub const K_DOT_GRAPH_FOOTER: &str = "}";
/// 属性节点外观 (cc:100-101)
pub const K_DOT_ATTR_NODE_ATTRS: &str = " [shape=Mrecord, color=grey, fontcolor=grey, margin=0.1, fontsize=10, label=\\"{";
/// 属性节点连边 (cc:110-111)
pub const K_DOT_ATTR_EDGE: &str = "style=dashed, color=grey, arrowhead=none";
/// on_failure 边 (cc:65)
pub const K_DOT_ON_FAILURE_EDGE: &str = " [style=dotted];\\n";
/// 属性分隔符 (cc:76)
pub const K_DOT_ATTR_SEPARATOR: &str = "|";
/// 属性位名 (cc:104-108: NI=换行兴趣 WI=词兴趣 SI=起始兴趣 @=绑定位)
pub const K_DOT_ATTR_NEWLINE_INTEREST: &str = "NI";
pub const K_DOT_ATTR_WORD_INTEREST: &str = "WI";
pub const K_DOT_ATTR_START_INTEREST: &str = "SI";
pub const K_DOT_ATTR_LABEL_BOUND: &str = "@";
/// 文本节点外观 (cc:175)
pub const K_DOT_TEXT_NODE_ATTRS: &str = "\\", shape=box, peripheries=2];\\n";
/// 回引节点 label 头/尾 (cc:182-183)
pub const K_DOT_BACKREF_LABEL_HEAD: &str = " [label=\\"$";
pub const K_DOT_BACKREF_LABEL_TAIL: &str = "..$\\", shape=doubleoctagon];\\n";
/// 末节点外观 (cc:190)
pub const K_DOT_END_NODE_ATTRS: &str = " [style=bold, shape=point];\\n";
/// 未锚定推进节点 (cc:274)
pub const K_DOT_UNANCHORED_LABEL: &str = " [label=\\"unanchored advance\\", shape=septagon];\\n";
/// Choice 节点 (cc:115-116: Mrecord + "?" / 负环视补 " neg")
pub const K_DOT_CHOICE_ATTRS: &str = " [shape=Mrecord, label=\\"?";
pub const K_DOT_NEG_LOOKAROUND_MARK: &str = " neg";
/// Choice→分支直连边 (cc:121: 无样式属性)
pub const K_DOT_CHOICE_EDGE_HEAD: &str = "  n";

// ── 截断限 ──
/// graph label / Text 元素数截断限 (cc:48/170: i > 40 即补 "...")
pub const K_DOT_LABEL_TRUNCATE: i32 = 40;
/// 类区间显示上限 (cc:159: j > 5 即补 "...")
pub const K_DOT_RANGES_TRUNCATE: i32 = 5;
/// 截断省略号
pub const K_DOT_ELLIPSIS: &str = "...";

const _: () = assert!(DOT_VISIT_NODE_TYPES.len() == 9);
const _: () = assert!(DOT_ACTION_LABELS.len() == 10);
const _: () = assert!(DOT_ASSERTION_ATTRS.len() == 5);
const _: () = assert!(K_DOT_LABEL_TRUNCATE == 40 && K_DOT_RANGES_TRUNCATE == 5);
`;

writeSrc("v8-regexp-syntax", "dot_printer_generated.rs", body);
console.log("written: v8-regexp-syntax/src/dot_printer_generated.rs");
