#!/usr/bin/env bun
// 报告中文文案 (export const 大写变量名, 不用字典)

export const TITLE = "v8-parsing 词法扫描评测";
export const SUMMARY =
  "v8rs Scanner (完整 ECMAScript 词法语义) 对比 单遍手写简易 tokenizer (只扫空白/注释/标点的最小等价路径)。语义成本 = v8-scanner 平均耗时 / naive-tokenizer 平均耗时, 表示完整词法语义的额外开销倍率; lookahead 一行反过来, 表示 4 槽环形缓存命中相对无缓存重扫的收益。";
export const GENERATED = "生成时间";
export const COL_OP = "操作";
export const COL_SAMPLE = "样本";
export const COL_SIZE = "规模";
export const COL_IMPL = "实现";
export const COL_MEAN = "平均耗时";
export const COL_MEDIAN = "中位数";
export const COL_STD = "标准差";
export const COL_THRPT = "吞吐";
export const COL_RATIO = "语义成本";
export const DESC_SCAN_TOKENS =
  "全量词法扫描: 标识符/关键字/数字字面量/字符串/模板/注释/正则起始歧义";
export const DESC_SCAN_REGEXP =
  "正则起始歧义重扫: '/' 处走 scan_regexp_pattern + scan_regexp_flags 公开 API";
export const DESC_LOOKAHEAD =
  "4 槽环形 lookahead 命中路径: 缓存填满后连续 peek_ahead / peek_ahead_ahead";
export const DESC_LITERAL_FILL = "LiteralBuffer 写入: start 复位 + add_char 单字节 (Latin-1) 路径";
export const CHART_TITLE = "v8-parsing criterion 评测 · 平均耗时 (每组内条形按组内最大值归一)";
export const LEGEND_V8 = "v8-scanner";
export const LEGEND_NAIVE = "naive-tokenizer";
