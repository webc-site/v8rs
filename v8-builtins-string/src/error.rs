use std::result;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  #[error("Invalid character code: {0}")]
  InvalidCharCode(u32),
  #[error("Index out of bounds: index {index}, length {length}")]
  IndexOutOfBounds { index: usize, length: usize },
  #[error("Builtin not found in global table: {0}")]
  BuiltinNotFound(String),
}

pub type Result<T> = result::Result<T, Error>;
