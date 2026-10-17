use std::result;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  #[error("near jump displacement out of 8-bit range: {0}")]
  NearJumpOutOfRange(i32),

  #[error(transparent)]
  Core(#[from] v8_codegen_core::Error),
}

pub type Result<T> = result::Result<T, Error>;
