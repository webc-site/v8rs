use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Runtime function not found: {0}")]
  FunctionNotFound(String),

  #[error("Invalid argument count for {0}: expected {1}, got {2}")]
  InvalidArgumentCount(String, i8, i8),

  #[error("Runtime execution trapped: {0}")]
  ExecutionTrapped(String),
}

pub type Result<T> = result::Result<T, Error>;
