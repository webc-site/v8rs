use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Promise already resolved or rejected")]
  AlreadyResolvedOrRejected,

  #[error("Invalid promise capability: {0}")]
  InvalidCapability(String),

  #[error("Promise reaction failed: {0}")]
  ReactionFailed(String),

  #[error("Iterator closed unexpectedly")]
  IteratorClosed,
}

pub type Result<T> = result::Result<T, Error>;
