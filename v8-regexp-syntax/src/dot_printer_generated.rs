// 由 tool/src/regexp/regexp-dotprinter.cc.js 从 v8/src/regexp/regexp-dotprinter.{h,cc} 生成; 修订就地改, 复跑覆盖
// Graphviz dot 输出格式表: 节点形状/label 模板/属性位/截断限; 遍历调度面待 Developer 就地修订

/// dot 图 Visit 分发全集 (FOR_EACH_NODE_TYPE 展开, 与编译器节点图同源)
pub const DOT_VISIT_NODE_TYPES: [&str; 9] = [
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

/// ActionNode 动作 → label 模板 (cc:220-271; $N/register 为运行期插值位)
pub const DOT_ACTION_LABELS: [(&str, &str); 10] = [
  ("SET_REGISTER_FOR_LOOP", "label=\"$"),
  ("INCREMENT_REGISTER", "label=\"$"),
  ("STORE_POSITION", "label=\"$"),
  ("RESTORE_POSITION", "label=\"$"),
  ("BEGIN_POSITIVE_SUBMATCH", "label=\"$"),
  ("BEGIN_NEGATIVE_SUBMATCH", "label=\"$"),
  ("POSITIVE_SUBMATCH_SUCCESS", "label=\"escape\", shape=septagon"),
  ("EMPTY_MATCH_CHECK", "label=\"$"),
  ("CLEAR_CAPTURES", "label=\"clear $"),
  ("EATS_AT_LEAST", "label=\"eats at least $"),
];

/// AssertionNode 类型 → dot 属性串 (cc:194-218, 全部 shape=septagon)
pub const DOT_ASSERTION_ATTRS: [(&str, &str); 5] = [
  ("AT_END", "label=\"$\", shape=septagon"),
  ("AT_START", "label=\"^\", shape=septagon"),
  ("AT_BOUNDARY", "label=\"\\b\", shape=septagon"),
  ("AT_NON_BOUNDARY", "label=\"\\B\", shape=septagon"),
  ("AFTER_NEWLINE", "label=\"(?<=\\n)\", shape=septagon"),
];

// ── dot 格式常量 (运行期值) ──
/// 图头 (cc:35)
pub const K_DOT_GRAPH_HEADER: &str = "digraph G {\n  graph [label=\"";
/// graph label 收口 (cc:53)
pub const K_DOT_GRAPH_LABEL_END: &str = "\"];\n";
/// 图收口 (cc:55)
pub const K_DOT_GRAPH_FOOTER: &str = "}";
/// 属性节点外观 (cc:100-101)
pub const K_DOT_ATTR_NODE_ATTRS: &str = " [shape=Mrecord, color=grey, fontcolor=grey, margin=0.1, fontsize=10, label=\"{";
/// 属性节点连边 (cc:110-111)
pub const K_DOT_ATTR_EDGE: &str = "style=dashed, color=grey, arrowhead=none";
/// on_failure 边 (cc:65)
pub const K_DOT_ON_FAILURE_EDGE: &str = " [style=dotted];\n";
/// 属性分隔符 (cc:76)
pub const K_DOT_ATTR_SEPARATOR: &str = "|";
/// 属性位名 (cc:104-108: NI=换行兴趣 WI=词兴趣 SI=起始兴趣 @=绑定位)
pub const K_DOT_ATTR_NEWLINE_INTEREST: &str = "NI";
pub const K_DOT_ATTR_WORD_INTEREST: &str = "WI";
pub const K_DOT_ATTR_START_INTEREST: &str = "SI";
pub const K_DOT_ATTR_LABEL_BOUND: &str = "@";
/// 文本节点外观 (cc:175)
pub const K_DOT_TEXT_NODE_ATTRS: &str = "\", shape=box, peripheries=2];\n";
/// 回引节点 label 头/尾 (cc:182-183)
pub const K_DOT_BACKREF_LABEL_HEAD: &str = " [label=\"$";
pub const K_DOT_BACKREF_LABEL_TAIL: &str = "..$\", shape=doubleoctagon];\n";
/// 末节点外观 (cc:190)
pub const K_DOT_END_NODE_ATTRS: &str = " [style=bold, shape=point];\n";
/// 未锚定推进节点 (cc:274)
pub const K_DOT_UNANCHORED_LABEL: &str = " [label=\"unanchored advance\", shape=septagon];\n";
/// Choice 节点 (cc:115-116: Mrecord + "?" / 负环视补 " neg")
pub const K_DOT_CHOICE_ATTRS: &str = " [shape=Mrecord, label=\"?";
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
