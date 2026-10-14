use std::result;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  #[error("Array capacity overflow: requested {requested}, max is {max}")]
  CapacityOverflow { requested: usize, max: usize },
  #[error("Index out of bounds: index {index}, length {length}")]
  IndexOutOfBounds { index: usize, length: usize },
  #[error("Builtin not found in global table: {0}")]
  BuiltinNotFound(String),
}

pub type Result<T> = result::Result<T, Error>;
