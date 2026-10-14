use std::result;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  #[error("Unknown builtin name: {0}")]
  UnknownBuiltin(String),
  #[error("Invalid builtin id: {0}")]
  InvalidBuiltinId(usize),
}

pub type Result<T> = result::Result<T, Error>;
