// 由 tool/src/diagnostics/objects-printer.js 从 v8/src/diagnostics/objects-printer.cc;src/common/globals.h;src/numbers/conversions-inl.h 生成; 修订就地改, 复跑覆盖

//! 对象打印零耦合字符串工具面 (基线生成; 行为面见 objects_print.rs)。

// ── PrintDouble 边界/字面 ──
pub const MAX_SAFE_INTEGER_BITS: u32 = 53;
pub const MAX_SAFE_INTEGER_F64: f64 = ((1u64 << MAX_SAFE_INTEGER_BITS) - 1) as f64;
pub const MIN_SAFE_INTEGER_F64: f64 = -MAX_SAFE_INTEGER_F64;
pub const MINUS_ZERO_STR: &str = "-0.0";
pub const NAN_SUFFIX: &str = " (0x";

// ── JSIteratorHelperState → 名 (4 项) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsIteratorHelperState {
  SuspendedStart,
  SuspendedYield,
  Executing,
  Completed,
}
pub const ITER_HELPER_STATE_NAMES: [&str; 4] = [
  "SUSPENDED_START",
  "SUSPENDED_YIELD",
  "EXECUTING",
  "COMPLETED",
];

// ── JSIteratorZipHelperMode → 名 (3 项) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsIteratorZipHelperMode {
  Shortest,
  Longest,
  Strict,
}
pub const ITER_ZIP_HELPER_MODE_NAMES: [&str; 3] = ["SHORTEST", "LONGEST", "STRICT"];
