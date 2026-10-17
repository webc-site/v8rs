// 英文语言字符串 (export const 大写变量名, 不用字典)
export const TITLE = "v8-common benchmark report";
export const INTRO =
  "v8-common (L0 foundation utilities) core hot-spot operations measured with criterion; implementations: crate API (v8-common), std-equivalent code (std), gxhash container comparison (gxhash).";
export const OP_TAG_CLASSIFY = "pointer tag classify (Smi / HeapObject / Weak)";
export const OP_COMPRESS_ROUNDTRIP = "pointer compress → decompress roundtrip";
export const OP_ALIGN_OPS = "alignment round-up + padding arithmetic";
export const OP_HASH_PROBE = "object_hash build-table + probe (container)";
export const COL_OP = "Operation";
export const COL_IMPL = "Impl";
export const COL_MEDIAN = "Median / iter";
export const COL_MEAN = "Mean / iter";
export const COL_CI = "95% CI (median)";
export const COL_SPEEDUP = "Speedup vs std";
export const TABLE_ABS = "Absolute time";
export const TABLE_REL = "Relative speedup (>1 = faster than std baseline)";
export const NOTE_BASELINE =
  "Speedup is median-time ratio against the std-equivalent baseline; higher is better.";
export const NOTE_METHOD =
  "criterion 0.8, tikv-jemallocator global allocator, 20 samples × 500 ms per benchmark, fixed-seed fastrand inputs.";
export const GEN_AT = "Generated at";
export const SVG_TITLE = "v8-common benchmark — median ns per iteration";
export const SVG_YLABEL = "ns / iteration";
export const LEGEND = "implementation";
