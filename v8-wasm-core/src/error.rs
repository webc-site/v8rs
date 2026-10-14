//! v8-wasm-core 错误集中定义
//! WasmError 对应 v8/src/wasm/wasm-result.h 的 WasmError (offset + message);
//! Result<T> 对应 wasm-result.h 的 Result<T> (T | WasmError)。
//! 登记差异: wasm-result.h 全量转写推迟 (Result 模板族由 std::result 覆盖)。
// 由 tool/src/wasm/decoder.h.js 生成 (wasm-result.h 的最小子集); 修订就地改, 复跑覆盖
use std::result;

use thiserror::Error;

/// wasm 解码错误 (原版 WasmError: offset + message)
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{message} (at offset {offset})")]
pub struct WasmError {
  /// 二进制内字节偏移
  pub offset: u32,
  /// 错误消息
  pub message: String,
}

impl WasmError {
  /// kNoErrorOffset = kMaxUInt32
  pub const NO_ERROR_OFFSET: u32 = u32::MAX;

  /// WasmError(offset, message)
  pub fn new(offset: u32, message: impl Into<String>) -> Self {
    debug_assert!(offset != Self::NO_ERROR_OFFSET);
    let message = message.into();
    debug_assert!(!message.is_empty());
    Self { offset, message }
  }

  /// has_error 语义由 Option 形态承载; offset()
  #[inline]
  pub const fn offset(&self) -> u32 {
    self.offset
  }

  /// message()
  #[inline]
  pub fn message(&self) -> &str {
    &self.message
  }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  /// 解码错误 (含偏移与消息)
  #[error(transparent)]
  Wasm(#[from] WasmError),
}

pub type Result<T> = result::Result<T, Error>;
