// zh.js — 报告中文文案 (export const 大写变量名, 不用字典)

export const TITLE = "v8-date 性能评测报告";
export const SUBTITLE = "基于 criterion 的 v8rs v8-date crate 与对比库评测";
export const GENERATED_AT = "生成时间";
export const BENCH_ARGS = "criterion 参数";
export const COL_OP = "评测操作";
export const COL_IMPL = "实现";
export const COL_MEAN = "平均耗时";
export const COL_MEDIAN = "中位数";
export const COL_RATIO = "相对 v8-date";
export const COL_ERR = "标准误";
export const TABLE_HINT =
  "耗时为一次遍历完整固定样本集的总时间，越小越好；倍率大于 1x 表示慢于 v8-date。";
export const RESULTS = "评测结果";
export const SECTION_OPS = "评测操作说明";
export const OP_PARSE =
  "parse_date_time: 日期时间字符串 -> Epoch 毫秒（ISO / RFC 2822 / legacy / 超界年份 / 非法样本）";
export const OP_YMD = "epoch_ms_to_ymd: Epoch 毫秒 -> (年, 月, 日)，DateCache 缓存命中查询路径";
export const OP_TOD =
  "epoch_ms_to_time_of_day: Epoch 毫秒 -> 日内 (时, 分, 秒, 毫秒)，LocalTimePhase 风格纯函数拆分";
export const IMPL_NOTE =
  "std-systemtime 无字符串解析能力，未注册 parse 操作，仅作时间戳处理对照组。";
export const AXIS_LABEL = "单次迭代耗时（越小越好）";
export const SVG_TITLE = "v8-date 评测：各操作平均耗时对比";
