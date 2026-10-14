// 中文语言字符串 (export const 大写变量名, 不用字典)
export const TITLE = "v8-common 评测报告";
export const INTRO =
  "v8-common (L0 地基层工具) 核心热点操作, 基于 criterion 评测; 实现方: crate API (v8-common)、std 等价写法 (std)、gxhash 容器对照 (gxhash)。";
export const OP_TAG_CLASSIFY = "指针标签判定 (Smi / HeapObject / Weak)";
export const OP_COMPRESS_ROUNDTRIP = "指针压缩 → 解压往返";
export const OP_ALIGN_OPS = "对齐向上取整 + padding 算术";
export const OP_HASH_PROBE = "object_hash 建表 + 探针 (容器)";
export const COL_OP = "操作";
export const COL_IMPL = "实现";
export const COL_MEDIAN = "中位数 / 次";
export const COL_MEAN = "均值 / 次";
export const COL_CI = "95% 置信区间 (中位数)";
export const COL_SPEEDUP = "相对 std 加速比";
export const TABLE_ABS = "绝对耗时";
export const TABLE_REL = "相对加速比 (>1 = 快于 std 基线)";
export const NOTE_BASELINE = "加速比为相对 std 等价写法基线的中位数耗时比值, 越大越好。";
export const NOTE_METHOD =
  "criterion 0.8, tikv-jemallocator 全局分配器, 每项 20 样本 × 500ms, fastrand 固定种子输入。";
export const GEN_AT = "生成时间";
export const SVG_TITLE = "v8-common 评测 — 每次迭代中位数耗时 (ns)";
export const SVG_YLABEL = "ns / 次迭代";
export const LEGEND = "实现方";
