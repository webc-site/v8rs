use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Number/BigInt operation error: {0}")]
  NumberError(String),

  #[error("Date/Time or Intl format error: {0}")]
  DateTimeError(String),

  #[error("System builtin dispatch error: {0}")]
  SystemError(String),

  #[error("WebAssembly runtime error: {0}")]
  WasmError(String),
}

pub type Result<T> = result::Result<T, Error>;
