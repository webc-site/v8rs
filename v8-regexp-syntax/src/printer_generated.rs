// 由 tool/src/regexp/regexp-printer.cc.js 从 v8/src/regexp/regexp-printer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 打印机公共基座规则面: ANSI 色码表 + 转义格式 + 编号规则; 流式输出/Zone 持有面待 Developer 就地修订

/// ANSI 前景色码 (原版 PrinterBase::Color, h:65-74)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Color {
  /// ANSI 复位
  Default = 0,
  /// ANSI 前景 31
  Red = 31,
  /// ANSI 前景 32
  Green = 32,
  /// ANSI 前景 33
  Yellow = 33,
  /// ANSI 前景 34
  Blue = 34,
  /// ANSI 前景 35
  Magenta = 35,
  /// ANSI 前景 36
  Cyan = 36,
  /// ANSI 前景 37
  White = 37,
}

pub const NUM_COLORS: usize = 8;
const _: () = assert!(Color::White as i32 == 37);

/// 原色转义: ESC [ {code} m (h:82, 仅 log_colour 开)
pub const K_COLOR_ESCAPE: &str = "\u{1b}[";
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
  "set_graph_labeller",
  "set_tree_labeller",
  "set_ast_printer",
  "set_trace_tree_scope",
];
const _: () = assert!(DIAGNOSTICS_HOLDER_SETTERS.len() == 4);
