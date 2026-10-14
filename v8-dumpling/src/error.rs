use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("IO error: {0}")]
  Io(String),
  #[error("Invalid position record format: {0}")]
  InvalidFormat(String),
  #[error("Dumping is disabled")]
  DumpingDisabled,
}

pub type Result<T> = result::Result<T, Error>;
