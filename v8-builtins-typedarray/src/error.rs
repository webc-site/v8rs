use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Invalid typed array length: {0}")]
  InvalidLength(usize),

  #[error("Typed array detached or out of bounds")]
  DetachedOrOutOfBounds,

  #[error("Invalid byte offset {offset} for element size {element_size}")]
  InvalidByteOffset { offset: usize, element_size: usize },

  #[error("Invalid data view operation: {0}")]
  InvalidDataViewOp(String),
}

pub type Result<T> = result::Result<T, Error>;
