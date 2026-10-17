use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Invalid regular expression flags: {0}")]
  InvalidFlags(String),

  #[error("Regular expression syntax error: {0}")]
  SyntaxError(String),

  #[error("RegExp stack overflow")]
  StackOverflow,

  #[error("RegExp execution failed: {0}")]
  ExecutionFailed(String),
}

pub type Result<T> = result::Result<T, Error>;
