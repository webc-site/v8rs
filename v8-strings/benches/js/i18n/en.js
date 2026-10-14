// en 语言字符串 (export const 大写变量名, 不用字典)。

export const TITLE = "v8-strings core operation benchmarks";
export const CORPUS_NOTE =
  "Corpus: deterministic samples (ASCII / Latin / CJK / surrogate-pair mixed), lengths 1k and 32 code points; values are criterion mean ns per iteration over the whole sample.";
export const METHOD_NOTE =
  "Method: criterion, short-run profiling (sample-size 10, warm-up 1s, measurement 1s), jemalloc global allocator.";
export const RATIO_NOTE = "Ratio: ns vs std baseline (lower is faster).";
export const COL_IMPL = "impl";
export const COL_RESULT = "ns / sample";
export const COL_VS_STD = "vs std";
export const OP_TRANSCODE_ENCODE = "UTF-16 to UTF-8 transcode";
export const OP_TRANSCODE_DECODE = "UTF-8 to UTF-16 transcode";
export const OP_COPY_CHARS = "CopyChars (well-formed copy)";
export const OP_SCAN_IDENTIFIER = "Identifier start/continue scan";
export const OP_CASE_LOWER = "ASCII case lower mapping";
export const OP_FIND_BYTE = "Single byte search";
export const OP_FIND_SUBSTRING = "Substring search";
export const OP_NORMALIZE = "NFC normalization";
