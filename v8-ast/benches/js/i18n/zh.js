// v8-ast genbench 报告中文语言文件 (i18n, 不用字典, export const 大写变量名)

export const REPORT_TITLE = "v8-ast 性能评测报告";
export const REPORT_INTRO =
  "v8-ast crate 的 criterion 评测: Zone 节点构造分配、enum_dispatch 派发遍历、节点尺寸/布局检查, 与等价手写 Box<dyn> 虚派发路径对比。";
export const OP_ALLOC_NODES = "alloc_nodes — 代表性 AST 节点分配 (每轮 1000 个)";
export const OP_BUILD_TREE = "build_tree — 合成表达式树构造 (深度×分支)";
export const OP_VISIT_TREE = "visit_tree — AstVisitor 全树遍历";
export const OP_LAYOUT_CHECK = "layout_check — 节点类型/布局检查扫描";
export const COL_OPERATION = "评测操作";
export const COL_IMPL = "实现";
export const COL_MEAN = "平均值";
export const COL_MEDIAN = "中位数";
export const COL_STD_DEV = "标准差";
export const COL_CI95 = "95% 置信区间";
export const COL_VS_FASTEST = "相对最快";
export const GAIN_TITLE = "enum_dispatch 相对 Box<dyn> 的收益";
export const GAIN_SUMMARY = "各对比组平均而言, v8-ast 路径比 Box<dyn> 对比实现快";
export const GAIN_SUMMARY_SUFFIX = "";
export const FASTEST = "最快";
export const UNIT_NOTE = "数值越低越好, 时间取 criterion 均值 (纳秒制, 已人性化单位)。";
export const GENERATED_AT = "生成时间";
export const FOOTER = "genbench 产出链: bench.js → bench.json → table.js / md.js / svg.js";
export const CHART_TITLE = "v8-ast criterion 评测 (越低越好)";
export const TABLE_CAPTION = "各评测操作下参与实现的对比表";
