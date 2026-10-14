// benches/js/i18n/en.js — English report strings (export const UPPER names, no dictionary)

export const REPORT_TITLE = "v8-flags Benchmark Report";

export const REPORT_INTRO = `criterion-based benchmarks for the core operations of v8-flags:
name lookup (phf static table vs std HashMap dynamic table), flag string parsing
in its variants, batch set_from_string and reset-to-default.
Flag semantics are home-grown (todo 10.2 red line: no clap); parsing, batch and reset
have no comparison library and are measured by v8rs-phf only, marked — for the
compared impl in the report.
Relative values use v8rs-phf as baseline; below 1 means the compared impl is faster.`;

export const COL_OPERATION = "Operation";
export const COL_RELATIVE = "Vs Baseline";
export const GENERATED_AT = "Generated at";
export const ARTIFACT_LINKS = "Table: [bench-table.csv](bench-table.csv) · Raw: [bench-result.json](bench-result.json)";
export const BASELINE_NOTE = "Baseline: v8rs-phf (compile-time perfect-hash static table)";

export const LOOKUP_HIT_LABEL = "lookup hit (64 per iteration)";
export const LOOKUP_MISS_LABEL = "lookup miss (64 per iteration)";
export const PARSE_BOOL_LABEL = "parse --flag (bool set)";
export const PARSE_NEG_LABEL = "parse --no-flag (bool negated)";
export const PARSE_KV_LABEL = "parse --flag=value (int)";
export const PARSE_STR_KV_LABEL = "parse --flag=value (string)";
export const PARSE_MULTI_LABEL = "parse multi-flag string (space separated)";
export const PARSE_UNKNOWN_LABEL = "parse unknown flag (error path)";
export const BATCH_SET_LABEL = "batch set_from_string (64 flags)";
export const RESET_DEFAULT_LABEL = "reset to defaults";
