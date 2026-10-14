use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Unsupported baseline architecture: {0}")]
  UnsupportedArchitecture(String),
  #[error("Invalid bytecode offset: {offset}")]
  InvalidBytecodeOffset { offset: usize },
  #[error("Compilation failed: {0}")]
  CompilationFailed(String),
  #[error("Baseline error: {0}")]
  Message(String),
}

pub type Result<T> = result::Result<T, Error>;
