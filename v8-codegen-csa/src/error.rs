use core::result;

use thiserror::Error;

/// CodeStubAssembler 专用错误枚举
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
  #[error("label already bound: {0}")]
  LabelAlreadyBound(&'static str),

  #[error("invalid node type tag: {0}")]
  InvalidNodeType(&'static str),

  #[error("invalid node ID: {0}")]
  InvalidNodeId(u32),

  #[error("compilation error: {0}")]
  CompilationError(&'static str),
}

/// CSA 统一 Result 别名
pub type Result<T> = result::Result<T, Error>;
