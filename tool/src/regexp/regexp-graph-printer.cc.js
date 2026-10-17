#!/usr/bin/env bun
// tool/src/regexp/regexp-graph-printer.cc.js — v8/src/regexp/regexp-graph-printer.{h,cc}
//   → v8-regexp-syntax/src/graph_printer_generated.rs
// 机械提取: ConnectionLocation 位标志 (4 基+2 组合) + Connection::ToString
//   16 态制表符表 + Scheduler 的 FOR_EACH_NODE_TYPE 9 类调度分类 + Id 宽度
//   计算规则 + PrintTrace 格式键 + 连线配色规则 (i%6)+1;
// 对数断言: 位标志 6 / 制表符 16 / 调度 9 / Trace 键 6 / 箭头锚。
import { banner, readV8, writeSrc } from "../../lib.js";

const h = readV8("src/regexp/regexp-graph-printer.h");
const cc = readV8("src/regexp/regexp-graph-printer.cc");
const nodesH = readV8("src/regexp/regexp-nodes.h");

function must(re, what, src = cc) {
  const m = src.match(re);
  if (!m) throw new Error(`regexp-graph-printer: 未找到 ${what}`);
  return m;
}

// ── ConnectionLocation 位标志 (cc:211-218) ────────────────────────────────
const locBlock = must(/enum ConnectionLocation \{([\s\S]*?)\};/, "ConnectionLocation")[1];
const locBits = [...locBlock.matchAll(/(k\w+) = 1 << (\d+)/g)].map((r) => ({ name: r[1], bit: Number(r[2]) }));
const locCombos = [...locBlock.matchAll(/(k\w+) = (k\w+) \| (k\w+)/g)].map((r) => ({ name: r[1], a: r[2], b: r[3] }));
if (locBits.length !== 4) throw new Error(`方向位应 4, 实数 ${locBits.length}`);
if (locCombos.length !== 2) throw new Error(`组合位应 2, 实数 ${locCombos.length}`);
for (let i = 0; i < 4; i++) {
  if (locBits[i].bit !== i) throw new Error(`方向位 ${locBits[i].name} 应为 1<<${i}`);
}
if (locCombos[0].name !== "kHorizontal" || locCombos[1].name !== "kVertical") {
  throw new Error("组合位命名漂移");
}

// ── Connection::ToString 16 态制表符表 (cc:228-264) ───────────────────────
const tsFn = must(/const char\* ToString\(\) const \{([\s\S]*?)\n  \}/, "Connection::ToString");
const tsCases = [...tsFn[1].matchAll(/((?:0|k\w+(?: \| k\w+)*)):\s*\n\s*return "(.*?)";/g)].map((r) => ({
  key: r[1].replace(/\s*\|\s*/g, "|"), glyph: r[2],
}));
if (tsCases.length !== 16) throw new Error(`制表符应 16 态, 实数 ${tsCases.length}`);
// 键覆盖: 空(0) + 4 单方向 + 11 组合
const tsKeys = new Set(tsCases.map((c) => c.key));
if (!tsKeys.has("0") || !tsKeys.has("kTop") || !tsKeys.has("kTop | kLeft | kRight | kBottom".replace(/\s*\|\s*/g, "|"))) {
  throw new Error("制表符键覆盖缺失 (0/单方向/全连)");
}
// 人工锚: 6 个关键字形
const EXPECT_GLYPH = {
  "0": " ",
  "kTop|kBottom": "│",
  "kLeft|kRight": "─",
  "kTop|kLeft|kRight|kBottom": "┼",
  "kTop|kLeft": "╯",
  "kBottom|kRight": "╭",
};
for (const c of tsCases) {
  if (c.key in EXPECT_GLYPH && EXPECT_GLYPH[c.key] !== c.glyph) {
    throw new Error(`制表符 ${c.key} 漂移: ${c.glyph}`);
  }
}
// 完备性: 4 位全部 16 组合各出现一次
if (tsKeys.size !== 16) throw new Error(`制表符键应 16 种, 实得 ${tsKeys.size}`);

// ── Scheduler 9 类调度分类 (cc:117-175) ───────────────────────────────────
const nodeTypes = [...must(/#define FOR_EACH_NODE_TYPE\(VISIT\)([\s\S]*?)\n\n/, "FOR_EACH_NODE_TYPE", nodesH)[1]
  .matchAll(/VISIT\((\w+)\)/g)].map((r) => r[1]);
if (nodeTypes.length !== 9) throw new Error(`节点类型应 9, 实数 ${nodeTypes.length}`);
// 方法头定位 (容忍签名换行; 排除私有助手 VisitSeqNode/VisitOther), 体取到下一方法头
const allHeads = [...cc.matchAll(/void GraphPrinter::Scheduler::Visit(\w+)\(/g)];
const schedHeads = allHeads.filter((m) => m[1] !== "SeqNode" && m[1] !== "Other");
if (schedHeads.length !== 9) throw new Error(`Scheduler Visit 应 9, 实数 ${schedHeads.length}`);
if (new Set(schedHeads.map((m) => m[1])).size !== 9) throw new Error("Scheduler Visit 重名");
const schedDef = schedHeads.map((m, i) => {
  const end = i + 1 < schedHeads.length ? schedHeads[i + 1].index : m.index + 400;
  return { type: m[1], body: cc.slice(m.index, end) };
});
// 按体分类: VisitSeqNode (串行) / VisitChoice (分支) / VisitOther (叶) / LoopChoice (双后继)
for (const s of schedDef) {
  if (!nodeTypes.includes(s.type)) throw new Error(`Scheduler 越类型: ${s.type}`);
  if (s.body.includes("VisitSeqNode(")) s.strategy = "VisitSeqNode";
  else if (s.body.includes("VisitChoice(node);") || s.body.includes("alternatives")) s.strategy = "VisitChoice";
  else if (s.body.includes("VisitOther(node);")) s.strategy = "VisitOther";
  else if (s.type === "LoopChoice") s.strategy = "LoopChoice(双后继)";
  else throw new Error(`Scheduler 策略未知: ${s.type}`);
}
const stratCount = schedDef.filter((s) => s.strategy === "VisitSeqNode").length;
if (stratCount !== 5) throw new Error(`串行策略应 5 (Action/UnanchoredAdvance/BackReference/Assertion/Text), 实数 ${stratCount}`);
// 人工锚: LoopChoice 双后继 (loop_node + continue_node)
const loopBody = must(/void GraphPrinter::Scheduler::VisitLoopChoice\(LoopChoiceNode\* node\) \{([\s\S]*?)\n\}/, "VisitLoopChoice");
if (!loopBody[1].includes("node->loop_node()") || !loopBody[1].includes("node->continue_node()")) {
  throw new Error("LoopChoice 双后继漂移");
}

// ── Id 宽度规则 (cc:273-306) ──────────────────────────────────────────────
if (!cc.includes("if (val == -1) return 2;")) throw new Error("IntWidth -1 特例漂移");
if (!cc.includes("return std::ceil(std::log10(val + 1));")) throw new Error("IntWidth 公式漂移");
if (!cc.includes("int max_width = IntWidth(max_node_id) + 2;")) throw new Error("MaxIdWidth +2 漂移");
if (!cc.includes("int padding_width = std::max(0, max_width - id_width);")) throw new Error("填充宽度下限 0 漂移");

// ── PrintTrace 格式键 (cc:589-627) ───────────────────────────────────────
const traceFn = must(/void GraphPrinter::PrintTrace\(Trace\* trace\) \{([\s\S]*?)\n\}/, "PrintTrace");
const TRACE_KEYS = ['"Trace: "', '" -> "', '"{"', '"cp:"', '", at_start:"', '", preloaded:"', '", backtrack:label["', '", action:"', '", special_loop"', '"}"'];
for (const k of TRACE_KEYS) {
  if (!cc.includes(k.replace(/^"|"$/g, ""))) throw new Error(`Trace 格式键缺失: ${k}`);
}
// at_start 三态 T/F
if (!cc.includes('(trace_part->at_start() == Trace::TRUE_VALUE ? "T" : "F")')) throw new Error("at_start T/F 漂移");
// trivial 跳过
if (!cc.includes("if (trace->is_trivial()) return;")) throw new Error("trivial Trace 跳过漂移");

// ── 连线配色 + 箭头 (cc:405-515) ─────────────────────────────────────────
const colorUses = [...cc.matchAll(/\(i % 6\) \+ 1/g)];
if (colorUses.length < 4) throw new Error(`配色 (i%6)+1 应 ≥4 处, 实数 ${colorUses.length}`);
if (!cc.includes('"\\033[0;3" << desired_color << "m"')) throw new Error("ANSI 配色转义漂移");
if (!cc.includes('"\\033[0m"')) throw new Error("ANSI 复位漂移");
if (!cc.includes('os() << "◄─";')) throw new Error("回边箭头 ◄─ 漂移");
if (!cc.includes('os() << (saw_target ? "► " : (!IsTarget(current) ? "  " : "─ "));')) {
  throw new Error("目标箭头 ► 三态漂移");
}
if (!cc.includes('os() << "▼";')) throw new Error("下落箭头 ▼ 漂移");
if (!cc.includes('os() << "↓";')) throw new Error("直落箭头 ↓ 漂移");
// fallback 颜色扫描: 从尾找起始边
if (!cc.includes("for (size_t e = edges_.size() - 1; e >= 0; --e)")) throw new Error("fallback 配色扫描漂移");

// ── BoyerMoore 打印 (cc:629-650) ─────────────────────────────────────────
if (!cc.includes('os() << "[";')) throw new Error("BM 位置信息 [ 头漂移");
if (!cc.includes('os() << ",";')) throw new Error("BM 位置分隔漂移");
if (!cc.includes('"BM Info:"')) throw new Error("BM Info 头漂移");
if (!cc.includes('os() << "     " << i << ": ";')) throw new Error("BM 行格式漂移");
if (!cc.includes("BoyerMoorePositionInfo::kMapSize")) throw new Error("kMapSize 遍历漂移");

// ── AddTarget/AddEdge 槽位规则 (cc:184-197/315-328) ──────────────────────
if (!cc.includes("if (targets.size() == 0 || (targets.back() != nullptr))")) throw new Error("AddTarget 尾槽规则漂移");
if (!cc.includes("while (i > 0) {\n    if (targets[i - 1] != nullptr) break;")) throw new Error("AddTarget 回填扫描漂移");
if (!cc.includes("if (next == target) return false;")) throw new Error("fallthrough 判定漂移");

console.log(
  `regexp-graph-printer: 方向位 ${locBits.length}+${locCombos.length}, 制表符 ${tsCases.length}, ` +
    `Scheduler 调度 ${schedDef.length}, Trace 键 ${TRACE_KEYS.length}, 配色 ${colorUses.length} 处 ✓`,
);

// ── 生成 Rust ──────────────────────────────────────────────────────────────
const body = `${banner(
  "src/regexp/regexp-graph-printer.cc.js",
  "src/regexp/regexp-graph-printer.{h,cc}",
  "节点图打印机规则面: 制表符表/调度分类/Id 宽度/Trace 格式/配色; 流水线实现待 Developer 就地修订",
)}
/// 连接方向位 (原版 ConnectionLocation, cc:211-218)
pub const K_LOC_TOP: u8 = 1 << ${locBits[0].bit};
pub const K_LOC_LEFT: u8 = 1 << ${locBits[1].bit};
pub const K_LOC_RIGHT: u8 = 1 << ${locBits[2].bit};
pub const K_LOC_BOTTOM: u8 = 1 << ${locBits[3].bit};
/// 组合位
pub const K_LOC_HORIZONTAL: u8 = K_LOC_LEFT | K_LOC_RIGHT;
pub const K_LOC_VERTICAL: u8 = K_LOC_TOP | K_LOC_BOTTOM;
pub const K_LOC_ALL: u8 = K_LOC_TOP | K_LOC_LEFT | K_LOC_RIGHT | K_LOC_BOTTOM;

/// 连接形态 → 制表符 (Connection::ToString 全 16 态, cc:228-264)
pub const CONNECTION_GLYPHS: [(u8, &str); ${tsCases.length}] = [
${tsCases.map((c) => {
    const val = c.key === "0" ? "0" : c.key.split("|").map((k) => `K_LOC_${k.slice(1).toUpperCase()}`).join(" | ");
    return `  (${val}, "${c.glyph}"), // ${c.key}`;
  }).join("\n")}
];

/// Scheduler 对 9 类节点的调度策略 (cc:117-175)
pub const SCHEDULER_STRATEGIES: [(&str, &str); ${schedDef.length}] = [
${schedDef.map((s) => `  ("${s.type}", "${s.strategy}"),`).join("\n")}
];

// ── Id 宽度规则 (cc:273-306) ──
/// IntWidth(-1) 固定 2; 其余 ceil(log10(val+1))
pub const K_NEG_ONE_WIDTH: i32 = 2;
/// MaxIdWidth = IntWidth(max_id) + 2 (+ padding_adjustment)
pub const K_ID_WIDTH_PAD: i32 = 2;

#[must_use]
pub fn int_width(val: i32) -> i32 {
  if val == -1 {
    return K_NEG_ONE_WIDTH;
  }
  (f64::ceil(f64::ln(f64::from(val + 1)) / std::f64::consts::LN_10)) as i32
}

// ── PrintTrace 格式键 (cc:589-627) ──
pub const K_TRACE_HEADER: &str = "Trace: ";
pub const K_TRACE_PART_SEPARATOR: &str = " -> ";
pub const K_TRACE_OPEN: &str = "{";
pub const K_TRACE_CLOSE: &str = "}";
pub const K_TRACE_CP: &str = "cp:";
pub const K_TRACE_AT_START: &str = ", at_start:";
pub const K_TRACE_PRELOADED: &str = ", preloaded:";
pub const K_TRACE_BACKTRACK: &str = ", backtrack:label[";
pub const K_TRACE_ACTION: &str = ", action:";
pub const K_TRACE_SPECIAL_LOOP: &str = ", special_loop";
/// at_start 三态输出 (TRUE_VALUE→"T", FALSE_VALUE→"F", UNKNOWN→省略)
pub const K_AT_START_TRUE: &str = "T";
pub const K_AT_START_FALSE: &str = "F";
/// backtrack 标签按指针十六进制 8 位输出 (cc:611-615)
pub const K_BACKTRACK_HEX_WIDTH: usize = 8;

// ── 连线配色与箭头 ──
/// 活动线色 = (边序 % 6) + 1, ANSI 0;3N (cc:424/434/443 等)
pub const K_LINE_COLOR_MODULUS: usize = 6;
pub const K_LINE_COLOR_BASE: usize = 1;
pub const K_ANSI_COLOR_PREFIX: &str = "\\u{1b}[0;3";
/// 回边箭头 (cc:454)
pub const K_ARROW_LOOP_BACK: &str = "◄─";
/// 目标箭头三态 (cc:461: 目标 ► / 空两格 / 直连 ─ )
pub const K_ARROW_TARGET: &str = "► ";
pub const K_ARROW_NONE: &str = "  ";
pub const K_ARROW_FALLTHROUGH: &str = "─ ";
/// 下落箭头 (cc:567/572)
pub const K_ARROW_DROP_BRANCH: &str = "▼";
pub const K_ARROW_DROP: &str = "↓";

// ── BoyerMoore 打印 (cc:629-650) ──
pub const K_BM_INFO_HEADER: &str = "BM Info:";
/// BM 行前导 5 空格 + "i: "
pub const K_BM_ROW_PREFIX: &str = "     ";

const _: () = assert!(CONNECTION_GLYPHS.len() == 16);
const _: () = assert!(SCHEDULER_STRATEGIES.len() == 9);
const _: () = assert!(K_LOC_ALL == 0b1111);
const _: () = assert!(int_width(-1) == 2 && int_width(9) == 1 && int_width(99) == 2 && int_width(100) == 3);
`;

writeSrc("v8-regexp-syntax", "graph_printer_generated.rs", body);
console.log("written: v8-regexp-syntax/src/graph_printer_generated.rs");
