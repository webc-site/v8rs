// 由 tool/src/regexp/regexp-node-printer.cc.js 从 v8/src/regexp/regexp-node-printer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 节点图文本打印机规则面: 标签名/动作文本/标记符/着色; 流式实现待 Developer 就地修订

/// Visit 分发全集 (FOR_EACH_NODE_TYPE 展开)
pub const NODE_PRINTER_TYPES: [&str; 9] = [
  "End",
  "Action",
  "Choice",
  "LoopChoice",
  "NegativeLookaroundChoice",
  "BackReference",
  "Assertion",
  "Text",
  "UnanchoredAdvance",
];

/// 逐节点标签名 (PrintNodeLabel 第二参, cc 各 Visit 首)
pub const NODE_LABEL_NAMES: [(&str, &str); 9] = [
  ("End", "End"),
  ("Action", "Action"),
  ("UnanchoredAdvance", "UnanchoredAdvance"),
  ("Choice", "Choice"),
  ("LoopChoice", "LoopChoice"),
  ("NegativeLookaroundChoice", "NegativeLookaroundChoice"),
  ("BackReference", "BackReference"),
  ("Assertion", "Assertion"),
  ("Text", "Text"),
];

/// EndNode 动作 → 输出 (cc:29-51; NEGATIVE_SUBMATCH_SUCCESS 另附寄存器字段)
pub const END_ACTION_TEXTS: [(&str, &str); 3] = [
  ("ACCEPT", "Accept"),
  ("BACKTRACK", "Backtrack"),
  ("NEGATIVE_SUBMATCH_SUCCESS", "]"),
];

/// ActionNode 动作 → 文本 (cc:53-111)
pub const ACTION_TEXTS: [(&str, &str); 10] = [
  ("SET_REGISTER_FOR_LOOP", "set register for loop "),
  ("INCREMENT_REGISTER", "increment register "),
  ("STORE_POSITION", "store position "),
  ("RESTORE_POSITION", "restore position "),
  ("BEGIN_POSITIVE_SUBMATCH", "begin positive submatch [stack: "),
  ("BEGIN_NEGATIVE_SUBMATCH", "begin negative submatch [stack: "),
  ("POSITIVE_SUBMATCH_SUCCESS", "positive submatch success [stack: "),
  ("EMPTY_MATCH_CHECK", "empty match check [start reg: "),
  ("CLEAR_CAPTURES", "clear captures "),
  ("EATS_AT_LEAST", "eats at least "),
];

/// AssertionNode 类型 → 文本 (cc:196-216)
pub const ASSERTION_TEXTS: [(&str, &str); 5] = [
  ("AT_END", "at end"),
  ("AT_START", "at start"),
  ("AT_BOUNDARY", "at boundary"),
  ("AT_NON_BOUNDARY", "at non-boundary"),
  ("AFTER_NEWLINE", "after newline"),
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
