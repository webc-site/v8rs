// 目的: 英文报告字符串 (export const 大写变量名, 不用字典)
export const TITLE = "v8-bigint benchmark report";
export const SUB = "Self-contained criterion bench; deterministic fixed-seed fastrand samples.";
export const COL_OP = "Operation";
export const COL_RADIX = "Radix";
export const COL_BITS = "Bits";
export const COL_RATIO = "peer / v8rs";
export const SEC_TABLE = "Summary";
export const SEC_SVG = "Chart";
export const SEC_NOTES = "Notes";
export const NOTE_MALACHITE =
  "Comparison lib is num-bigint only: malachite-nz is too heavy to compile for this workspace and was not introduced.";
export const NOTE_BARRETT =
  "Barrett division switches at >=13000 digits (~832k bits), beyond the 64k-bit sample ceiling, so it is not benchmarked; the Burnikel-Ziegler boundary (57 digits) is covered by div/rem.";
export const NOTE_ALGO =
  "Multiplication widths cross the schoolbook / Karatsuba (34) / Toom-Cook (210) / FFT (720) digit thresholds; string ops cover radix 10 (generic fast path) and radix 16 (power-of-two shortcut).";
export const NOTE_SAMPLES =
  "Samples: normalized (msd non-zero) random digits; div/rem divisor is half the dividend width (64-bit level skipped).";
export const NOTE_TIME =
  "Numbers are criterion mean point estimates under a short window (warm-up 1s / measure 2s); override with env BENCH_ARGS for higher precision.";
