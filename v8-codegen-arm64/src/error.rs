use std::result;

use thiserror::Error;

/// ARM64 汇编与代码生成错误定义
#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  #[error("label already bound to position {0}")]
  LabelAlreadyBound(i32),

  #[error("label is unused and has no position")]
  LabelUnused,

  #[error("branch displacement out of range: {0} bytes")]
  BranchOffsetOutOfRange(i32),

  #[error("branch displacement {0} bytes is not 4-byte aligned")]
  MisalignedBranchOffset(i32),

  #[error("label position {0} is not 4-byte aligned")]
  MisalignedLabel(i32),

  #[error("immediate value {0} out of encodable range")]
  ImmediateOutOfRange(i64),

  #[error("invalid register size or combination: {0}")]
  InvalidRegister(&'static str),

  #[error("buffer overflow: requested {requested} exceeds max {max}")]
  BufferOverflow { requested: usize, max: usize },

  #[error(transparent)]
  Core(#[from] v8_codegen_core::Error),
}

pub type Result<T> = result::Result<T, Error>;
