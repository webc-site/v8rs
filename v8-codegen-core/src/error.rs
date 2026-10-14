use std::result;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  #[error("label already bound to position {0}")]
  LabelAlreadyBound(i32),

  #[error("label is unused and has no position")]
  LabelUnused,

  #[error("buffer overflow: requested {requested} exceeds max {max}")]
  BufferOverflow { requested: usize, max: usize },

  #[error("invalid safepoint table: {0}")]
  InvalidSafepointTable(&'static str),

  #[error("invalid source position table: {0}")]
  InvalidSourcePositionTable(&'static str),

  #[error("invalid reloc info: {0}")]
  InvalidRelocInfo(&'static str),

  #[error("invalid descriptor configuration: {0}")]
  InvalidDescriptor(&'static str),

  #[error("invalid handler table: {0}")]
  InvalidHandlerTable(&'static str),
}

pub type Result<T> = result::Result<T, Error>;
