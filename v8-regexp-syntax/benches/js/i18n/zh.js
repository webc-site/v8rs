#!/usr/bin/env bun
// 中文语言包

export const TITLE = "v8-regexp-syntax 语法层性能评测";
export const SUBTITLE = "criterion 评测汇总（基线 {baseline}）";

export const OP_PARSE = "解析模式集";
export const OP_CHAR_RANGES = "字符区间集合代数";
export const OP_FORMAT_AST = "format_ast 输出";

export const DESC_PARSE =
  "RegExpParser 解析代表性模式集 × Flags 组合：字面量、分支、量词嵌套、捕获组、命名组与引用、字符类、范围合并规范化、Unicode property 路径、断言。";
export const DESC_CHAR_RANGES =
  "CharacterRange 集合代数批量操作：规范化合并（并集）、交集、差集、取反，外加 Interval::union 区间合并对照。固定确定性 128+128 码点区间。";
export const DESC_FORMAT_AST = "RegExpTree format_ast 序列化输出：6 棵代表性语法树逐棵重渲染。";

export const COL_TARGET = "实现";
export const COL_MEAN = "均值";
export const COL_MEDIAN = "中位数";
export const COL_SLOPE = "斜率/迭代";
export const COL_CI = "置信区间 (95%)";
export const COL_REL = "相对最快";

export const NOTE_ORACLE =
  "regex-syntax 仅为解析对比 oracle：RegExp 回溯语义与 regex/RE2 类库不兼容（红线 10.3），不进产品路径，不能作执行替换；其无 V8 Flags 概念，按标志组合次数重复解析以对齐操作数，字符类集合用其 hir::ClassUnicode 区间代数对照。";
export const NOTE_SAMPLES =
  "样本集固定确定性（xorshift64 定种子），全部实现共用同一数据；v8rs 为被测产品实现。";
export const GENERATED_AT = "生成时间";
