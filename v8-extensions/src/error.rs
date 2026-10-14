use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Invalid extension option: key={key}, value={value}")]
  InvalidOption { key: String, value: String },
  #[error("Extension execution failed: {0}")]
  ExecutionFailed(String),
  #[error("Unsupported extension operation: {0}")]
  UnsupportedOperation(String),
  #[error("Extension error: {0}")]
  Message(String),
}

pub type Result<T> = result::Result<T, Error>;
