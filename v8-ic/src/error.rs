use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Invalid IC state transition: {0}")]
  InvalidStateTransition(String),

  #[error("Handler bitfield encoding overflow: {0}")]
  HandlerBitfieldOverflow(String),

  #[error("StubCache lookup failure: map 0x{0:x}, name {1}")]
  StubCacheMiss(u64, String),

  #[error("Unsupported IC handler kind: {0}")]
  UnsupportedHandlerKind(String),
}

pub type Result<T> = result::Result<T, Error>;
