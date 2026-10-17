// 由 tool/src/wasm/wasm-code-coverage.h.js 从 v8/src/wasm/wasm-code-coverage.h 生成; 修订就地改, 复跑覆盖
// Wasm 代码覆盖区间结构

/// 代码字节偏移区间 [start, end], 双端含 (对应原版 WasmCodeRange)
/// 偏移基于函数 wire bytes 起点而非模块起点
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WasmCodeRange {
  /// 区间起点 (无代码时为 NO_CODE_POSITION)
  pub start: i32,
  /// 区间终点 (无代码时为 NO_CODE_POSITION)
  pub end: i32,
}

/// 无代码位置哨兵 (原版 kNoCodePosition)
pub const NO_CODE_POSITION: i32 = -1;

impl WasmCodeRange {
  /// 原版 constexpr 构造 WasmCodeRange(start, end)
  #[inline]
  #[must_use]
  pub const fn new(start: i32, end: i32) -> Self {
    Self { start, end }
  }
}

/// 函数级覆盖数据 (原版 WasmFunctionCoverageData, 语义面待 Developer 修订)
pub struct WasmFunctionCoverageData;

// —— 提取不变式断言 ——
const _: () = {
  assert!(NO_CODE_POSITION == -1);
};
