// en.js — 报告英文文案 (export const 大写变量名, 不用字典)

export const TITLE = "v8-date Benchmark Report";
export const SUBTITLE =
  "criterion-based evaluation of the v8rs v8-date crate against reference libraries";
export const GENERATED_AT = "Generated at";
export const BENCH_ARGS = "Criterion args";
export const COL_OP = "Operation";
export const COL_IMPL = "Implementation";
export const COL_MEAN = "Mean";
export const COL_MEDIAN = "Median";
export const COL_RATIO = "vs v8-date";
export const COL_ERR = "StdErr";
export const TABLE_HINT =
  "Times are per iteration over the full fixed sample set; lower is better. Ratio > 1x means slower than v8-date.";
export const RESULTS = "Results";
export const SECTION_OPS = "Operations";
export const OP_PARSE =
  "parse_date_time: date-time string -> epoch ms (ISO / RFC 2822 / legacy / out-of-range / invalid samples)";
export const OP_YMD =
  "epoch_ms_to_ymd: epoch ms -> (year, month, day), DateCache cached lookup path";
export const OP_TOD =
  "epoch_ms_to_time_of_day: epoch ms -> in-day (hour, minute, second, ms), pure LocalTimePhase-style split functions";
export const IMPL_NOTE =
  "std-systemtime has no string parsing capability, so the parse operation is not registered for it (control group only).";
export const AXIS_LABEL = "time per iteration (lower is better)";
export const SVG_TITLE = "v8-date benchmark: mean time per operation";
