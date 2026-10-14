#!/usr/bin/env bun
// report english texts (export const upper-case names, no dictionary)

export const TITLE = "v8-parsing Lexical Scanner Benchmark";
export const SUMMARY =
  "v8rs Scanner (full ECMAScript lexical semantics) vs a single-pass hand-written naive tokenizer (minimal equivalent path: whitespace/comments/punctuation only). Semantic cost = v8-scanner mean / naive-tokenizer mean, the overhead ratio of full lexical semantics; the lookahead row is reversed, showing the gain of the 4-slot ring cache hits over uncached re-scanning.";
export const GENERATED = "Generated at";
export const COL_OP = "Operation";
export const COL_SAMPLE = "Sample";
export const COL_SIZE = "Size";
export const COL_IMPL = "Implementation";
export const COL_MEAN = "Mean Time";
export const COL_MEDIAN = "Median";
export const COL_STD = "Std Dev";
export const COL_THRPT = "Throughput";
export const COL_RATIO = "Semantic Cost";
export const DESC_SCAN_TOKENS =
  "Full token scan: identifiers/keywords/numeric literals/strings/templates/comments/regex-start ambiguity";
export const DESC_SCAN_REGEXP =
  "Regex ambiguity re-scan: '/' routed through scan_regexp_pattern + scan_regexp_flags public API";
export const DESC_LOOKAHEAD =
  "4-slot ring lookahead hit path: peek_ahead / peek_ahead_ahead after cache is filled";
export const DESC_LITERAL_FILL =
  "LiteralBuffer writes: start reset + add_char one-byte (Latin-1) path";
export const CHART_TITLE = "v8-parsing criterion benchmark · mean time (bars normalized per group)";
export const LEGEND_V8 = "v8-scanner";
export const LEGEND_NAIVE = "naive-tokenizer";
