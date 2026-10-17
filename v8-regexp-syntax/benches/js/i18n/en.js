#!/usr/bin/env bun
// English language pack

export const TITLE = "v8-regexp-syntax Syntax Benchmark";
export const SUBTITLE = "criterion benchmark summary (baseline {baseline})";

export const OP_PARSE = "Pattern parsing";
export const OP_CHAR_RANGES = "Character range algebra";
export const OP_FORMAT_AST = "format_ast rendering";

export const DESC_PARSE =
  "RegExpParser on a representative pattern set x flag combinations: literals, alternation, nested quantifiers, captures, named groups and backreferences, character classes, range-merge canonicalization, Unicode property paths, assertions.";
export const DESC_CHAR_RANGES =
  "Batched CharacterRange set algebra: canonicalize (union), intersect, subtract, negate, plus Interval::union merging. Fixed deterministic 128+128 code point ranges.";
export const DESC_FORMAT_AST =
  "RegExpTree format_ast serialization: re-render 6 representative syntax trees.";

export const COL_TARGET = "Implementation";
export const COL_MEAN = "Mean";
export const COL_MEDIAN = "Median";
export const COL_SLOPE = "Slope/iter";
export const COL_CI = "CI (95%)";
export const COL_REL = "Rel. to fastest";

export const NOTE_ORACLE =
  "regex-syntax is a parse-comparison oracle only: V8 RegExp backtracking semantics are incompatible with regex/RE2 libraries (redline 10.3), it never enters the product path and cannot replace execution; it has no V8 Flags concept (patterns parsed once per flag combination to align operand counts) and uses hir::ClassUnicode interval algebra for range operations.";
export const NOTE_SAMPLES =
  "Sample sets are fixed and deterministic (xorshift64 with constant seeds), shared by all implementations; v8rs is the implementation under test.";
export const GENERATED_AT = "Generated at";
