// 由 tool/src/regexp/regexp-graph-printer.cc.js 从 v8/src/regexp/regexp-graph-printer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 节点图打印机规则面: 制表符表/调度分类/Id 宽度/Trace 格式/配色; 流水线实现待 Developer 就地修订

use std::f64::consts::LN_10;

/// 连接方向位 (原版 ConnectionLocation, cc:211-218)
pub const K_LOC_TOP: u8 = 1 << 0;
pub const K_LOC_LEFT: u8 = 1 << 1;
pub const K_LOC_RIGHT: u8 = 1 << 2;
pub const K_LOC_BOTTOM: u8 = 1 << 3;
/// 组合位
pub const K_LOC_HORIZONTAL: u8 = K_LOC_LEFT | K_LOC_RIGHT;
pub const K_LOC_VERTICAL: u8 = K_LOC_TOP | K_LOC_BOTTOM;
pub const K_LOC_ALL: u8 = K_LOC_TOP | K_LOC_LEFT | K_LOC_RIGHT | K_LOC_BOTTOM;

/// 连接形态 → 制表符 (Connection::ToString 全 16 态, cc:228-264)
pub const CONNECTION_GLYPHS: [(u8, &str); 16] = [
  (0, " "),                                                   // 0
  (K_LOC_TOP, "╵"),                                           // kTop
  (K_LOC_LEFT, "╴"),                                          // kLeft
  (K_LOC_RIGHT, "╶"),                                         // kRight
  (K_LOC_BOTTOM, "╷"),                                        // kBottom
  (K_LOC_TOP | K_LOC_LEFT, "╯"),                              // kTop|kLeft
  (K_LOC_TOP | K_LOC_RIGHT, "╰"),                             // kTop|kRight
  (K_LOC_BOTTOM | K_LOC_LEFT, "╮"),                           // kBottom|kLeft
  (K_LOC_BOTTOM | K_LOC_RIGHT, "╭"),                          // kBottom|kRight
  (K_LOC_TOP | K_LOC_BOTTOM, "│"),                            // kTop|kBottom
  (K_LOC_LEFT | K_LOC_RIGHT, "─"),                            // kLeft|kRight
  (K_LOC_TOP | K_LOC_BOTTOM | K_LOC_LEFT, "┤"),               // kTop|kBottom|kLeft
  (K_LOC_TOP | K_LOC_BOTTOM | K_LOC_RIGHT, "├"),              // kTop|kBottom|kRight
  (K_LOC_LEFT | K_LOC_RIGHT | K_LOC_TOP, "┴"),                // kLeft|kRight|kTop
  (K_LOC_LEFT | K_LOC_RIGHT | K_LOC_BOTTOM, "┬"),             // kLeft|kRight|kBottom
  (K_LOC_TOP | K_LOC_LEFT | K_LOC_RIGHT | K_LOC_BOTTOM, "┼"), // kTop|kLeft|kRight|kBottom
];

/// Scheduler 对 9 类节点的调度策略 (cc:117-175)
pub const SCHEDULER_STRATEGIES: [(&str, &str); 9] = [
  ("End", "VisitOther"),
  ("Action", "VisitSeqNode"),
  ("UnanchoredAdvance", "VisitSeqNode"),
  ("Choice", "VisitChoice"),
  ("LoopChoice", "LoopChoice(双后继)"),
  ("NegativeLookaroundChoice", "VisitChoice"),
  ("BackReference", "VisitSeqNode"),
  ("Assertion", "VisitSeqNode"),
  ("Text", "VisitSeqNode"),
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
  (f64::ceil(f64::ln(f64::from(val + 1)) / LN_10)) as i32
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
pub const K_ANSI_COLOR_PREFIX: &str = "\u{1b}[0;3";
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
const _: () =
  assert!(int_width(-1) == 2 && int_width(9) == 1 && int_width(99) == 2 && int_width(100) == 3);
