// 由 tool/src/regexp/regexp-ast-printer.cc.js 从 v8/src/regexp/regexp-ast-printer.{h,cc} 生成; 修订就地改, 复跑覆盖
// AST Dump 分发表与格式常量 (X-macro 展开); 流式输出实现面在 ast.rs, 就地修订

/// AST 节点种类 (FOR_EACH_REG_EXP_TREE_TYPE 展开, 与 ast_generated::TreeKind 同源)
pub const TREE_KINDS: [&str; 14] = [
  "Disjunction",
  "Alternative",
  "Assertion",
  "ClassRanges",
  "ClassSetOperand",
  "ClassSetExpression",
  "Atom",
  "Quantifier",
  "Capture",
  "Group",
  "Lookaround",
  "BackReference",
  "Empty",
  "Text",
];

/// 逐节点 Dump 前缀 (原版各 Visit 首字面量; ClassRanges 的 ^ 视 is_negated 前置)
pub const TREE_KIND_PREFIXES: [(&str, &str); 14] = [
  ("Disjunction", "(|"),
  ("Alternative", "(:"),
  ("Assertion", "@^i"),
  ("ClassRanges", "^"),
  ("ClassSetOperand", "!["),
  ("ClassSetExpression", "++"),
  ("Atom", "'"),
  ("Quantifier", "(# "),
  ("Capture", "(^ "),
  ("Group", "(?"),
  ("Lookaround", "("),
  ("BackReference", "(<- "),
  ("Empty", "%"),
  ("Text", "(!"),
];

/// 断言类型 → Dump 标签 (cc:108-130, 与 ast.h Assertion::Type 7 项一一对应)
pub const ASSERTION_TAGS: [(&str, &str); 7] = [
  ("START_OF_INPUT", "@^i"),
  ("END_OF_INPUT", "@$i"),
  ("END_OF_BUFFER", "@$Z"),
  ("START_OF_LINE", "@^l"),
  ("END_OF_LINE", "@$l"),
  ("BOUNDARY", "@b"),
  ("NON_BOUNDARY", "@B"),
];

/// ClassSetExpression 操作 → 记号 (cc:84-94)
pub const CLASS_SET_OP_TOKENS: [(&str, &str); 3] = [
  ("Union", "++"),
  ("Intersection", "&&"),
  ("Subtraction", "--"),
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

const _: () = assert!(TREE_KINDS.len() == 14);
const _: () = assert!(ASSERTION_TAGS.len() == 7);
const _: () = assert!(CLASS_SET_OP_TOKENS.len() == 3);
