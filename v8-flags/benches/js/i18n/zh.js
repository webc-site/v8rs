// benches/js/i18n/zh.js — 中文报告字符串 (export const 大写变量名, 不用字典)

export const REPORT_TITLE = "v8-flags 评测报告";

export const REPORT_INTRO = `本评测用 criterion 对 v8-flags 的核心操作做基准测量:
按名查找 (phf 静态表 vs std HashMap 动态表)、flag 字符串解析、批量设置与默认值重置。
flag 语义为自研实现 (todo 10.2 红线, 不引入 clap); 解析/批量/重置类操作无对比库,
仅由 v8rs-phf 单实现测量, 报告中对比实现以 — 标记。
相对值以 v8rs-phf 为基线, 小于 1 表示对比实现更快。`;

export const COL_OPERATION = "操作";
export const COL_RELATIVE = "相对基线";
export const GENERATED_AT = "生成时间";
export const ARTIFACT_LINKS = "数据表: [bench-table.csv](bench-table.csv) · 原始数据: [bench-result.json](bench-result.json)";
export const BASELINE_NOTE = "基线实现: v8rs-phf (phf 编译期完美散列静态表)";

export const LOOKUP_HIT_LABEL = "按名查找命中 (每迭代 64 次)";
export const LOOKUP_MISS_LABEL = "按名查找未命中 (每迭代 64 次)";
export const PARSE_BOOL_LABEL = "解析 --flag (布尔置位)";
export const PARSE_NEG_LABEL = "解析 --no-flag (布尔取反)";
export const PARSE_KV_LABEL = "解析 --flag=值 (整数)";
export const PARSE_STR_KV_LABEL = "解析 --flag=值 (字符串)";
export const PARSE_MULTI_LABEL = "解析多 flag 串 (空格分隔)";
export const PARSE_UNKNOWN_LABEL = "解析未知 flag (错误路径)";
export const BATCH_SET_LABEL = "批量 set_from_string (64 个 flag)";
export const RESET_DEFAULT_LABEL = "默认值重置";
