use std::result;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  #[error("Invalid sweeping state transition: expected {expected}, got {actual}")]
  InvalidSweepingState {
    expected: &'static str,
    actual: &'static str,
  },
}

pub type Result<T> = result::Result<T, Error>;
