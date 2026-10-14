#!/usr/bin/env bun
// tool/src/regexp/regexp-printer.cc.js — v8/src/regexp/regexp-printer.{h,cc}
//   → v8-regexp-syntax/src/printer_generated.rs
// 机械提取: PrinterBase::Color ANSI 色码表 (9) + set_color/set_bold 转义格式
//   + GraphLabeller 编号起点 (next_node_label_ = 1) + NodePrinter 前缀字符
//   ('t'/'n', 从 ast-printer/node-printer 头交叉) + Diagnostics 持有面签名骨架;
// 对数断言: 色码 9 项 / 前缀 2 态 / 未命中等值面。
import { banner, readV8, writeSrc } from "../../lib.js";

const h = readV8("src/regexp/regexp-printer.h");
const cc = readV8("src/regexp/regexp-printer.cc");
const astH = readV8("src/regexp/regexp-ast-printer.h");
const nodeH = readV8("src/regexp/regexp-node-printer.h");

function must(re, what, src = h) {
  const m = src.match(re);
  if (!m) throw new Error(`regexp-printer: 未找到 ${what}`);
  return m;
}

// ── Color 色码表 (h:65-74) ──────────────────────────────────────────────────
const colorBlock = must(/enum class Color : int \{([\s\S]*?)\n  \};/, "Color 枚举")[1];
const colors = [...colorBlock.matchAll(/k(\w+) = (\d+),?/g)].map((r) => ({ name: r[1], code: Number(r[2]) }));
if (colors.length !== 8) throw new Error(`Color 应 8 项, 实数 ${colors.length}`);
for (const c of colors) {
  if (!(c.code === 0 || (31 <= c.code && c.code <= 37))) {
    throw new Error(`Color ${c.name} 色码 ${c.code} 越界 (0 或 31..37)`);
  }
}
if (colors.filter((c) => c.code === 0).length !== 1 || colors[0].name !== "Default") {
  throw new Error("Color kDefault 必须为 0 号");
}

// ── ANSI 转义格式 (h:82/86) ────────────────────────────────────────────────
if (!h.includes('os() << "\\033[" << color_ << "m";')) throw new Error("set_color 转义格式漂移");
if (!h.includes('os() << "\\033[" << (bold ? "1" : "0") << ";" << color() << "m";')) {
  throw new Error("set_bold 转义格式漂移");
}
// log_colour 门控两处
if ((h.match(/if \(!v8_flags\.log_colour\) return;/g) || []).length !== 2) {
  throw new Error("log_colour 门控应 2 处");
}

// ── GraphLabeller 编号规则 (h:33-60) ───────────────────────────────────────
if (!h.includes("int next_node_label_ = 1;")) throw new Error("编号起点应为 1");
if (!h.includes("return -1;")) throw new Error("未命中查询应返回 -1");

// ── 前缀字符交叉: ast 't' / node 'n' ──────────────────────────────────────
if (!astH.includes("NodePrinterBase(os, labeller, zone, 't')")) throw new Error("ast-printer 前缀 't' 漂移");
if (!nodeH.includes("NodePrinterBase(os, labeller, zone, 'n')")) throw new Error("node-printer 前缀 'n' 漂移");

// ── Diagnostics 持有面 (h:136-167) ────────────────────────────────────────
const holders = [...h.matchAll(/void set_(\w+)\(std::unique_ptr<(\w+)<(\w+)>> (\w+)\);/g)].map((r) => ({
  setter: r[1], kind: r[2], param: r[3],
}));
if (holders.length !== 3) throw new Error(`unique_ptr 持有 setter 应 3, 实数 ${holders.length}`);
if (!h.includes("void set_trace_tree_scope(TraceTreeScope* scope)")) throw new Error("裸指针 setter 缺失");

// cc 侧: 4 个 set_* 实现与 h 声明对数 (cc 内 AstNodePrinter 别名走 set_ast_printer)
const ccImpls = [...cc.matchAll(/void Diagnostics::set_(\w+)\(/g)].map((r) => r[1]);
if (ccImpls.length !== 4) throw new Error(`cc set_* 实现应 4, 实数 ${ccImpls.length}`);
if (new Set(ccImpls).size !== 4) throw new Error("cc set_* 重名");
// 整文件受 V8_ENABLE_REGEXP_DIAGNOSTICS 门控 (h 与 cc 各自)
if (!h.includes("#ifdef V8_ENABLE_REGEXP_DIAGNOSTICS") || !cc.includes("#ifdef V8_ENABLE_REGEXP_DIAGNOSTICS")) {
  throw new Error("诊断门控宏缺失");
}

console.log(`regexp-printer: Color 色码 ${colors.length}, ANSI 格式 2, 持有 setter ${holders.length}+1, 前缀 t/n ✓`);

// ── 生成 Rust ──────────────────────────────────────────────────────────────
const body = `${banner(
  "src/regexp/regexp-printer.cc.js",
  "src/regexp/regexp-printer.{h,cc}",
  "打印机公共基座规则面: ANSI 色码表 + 转义格式 + 编号规则; 流式输出/Zone 持有面待 Developer 就地修订",
)}
/// ANSI 前景色码 (原版 PrinterBase::Color, h:65-74)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Color {
${colors.map((c) => `  /// ANSI ${c.code === 0 ? "复位" : `前景 ${c.code}`}\n  ${c.name} = ${c.code},`).join("\n")}
}

pub const NUM_COLORS: usize = ${colors.length};
const _: () = assert!(Color::${colors[colors.length - 1].name} as i32 == ${colors[colors.length - 1].code});

/// 原色转义: ESC [ {code} m (h:82, 仅 log_colour 开)
pub const K_COLOR_ESCAPE: &str = "\\u{1b}[";
/// 加粗形态: ESC [ {0|1};{color} m (h:86)
pub const K_BOLD_SUFFIX: &str = "m";

/// 构造 set_color 输出 (语义面; Rust 侧重写为 String 拼接)
#[must_use]
pub fn color_sequence(color: Color) -> String {
  format!("{K_COLOR_ESCAPE}{}m", color as i32)
}

/// 构造 set_bold 输出 (h:86)
#[must_use]
pub fn bold_sequence(bold: bool, color: Color) -> String {
  format!("{K_COLOR_ESCAPE}{};{}m", if bold { 1 } else { 0 }, color as i32)
}

/// 节点标签起始编号 (原版 GraphLabeller::next_node_label_ 初值 1, h:60)
pub const K_FIRST_NODE_LABEL: i32 = 1;
/// 未登记节点的查询结果 (原版 const NodeId 返回 -1, h:53)
pub const K_UNKNOWN_NODE_ID: i32 = -1;

/// AST 树打印机标签前缀 (原版 NodePrinterBase prefix_, ast-printer.h:26)
pub const K_AST_PRINTER_PREFIX: char = 't';
/// 编译节点图打印机标签前缀 (node-printer.h:23)
pub const K_NODE_PRINTER_PREFIX: char = 'n';

/// Diagnostics 的 unique_ptr 持有面 (h:136-162, 3 个 set_* + 裸指针 scope)
pub const DIAGNOSTICS_HOLDER_SETTERS: [&str; 4] = [
${holders.map((x) => `  "set_${x.setter}",`).join("\n")}
  "set_trace_tree_scope",
];
const _: () = assert!(DIAGNOSTICS_HOLDER_SETTERS.len() == 4);
`;

writeSrc("v8-regexp-syntax", "printer_generated.rs", body);
console.log("written: v8-regexp-syntax/src/printer_generated.rs");
