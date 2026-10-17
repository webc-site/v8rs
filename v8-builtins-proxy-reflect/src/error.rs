use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Cannot create proxy with a non-object target or handler")]
  TargetOrHandlerNotObject,

  #[error("Proxy revoked: cannot perform operation")]
  ProxyRevoked,

  #[error("Trap returned false or invalid result: {0}")]
  TrapInvariantViolated(String),
}

pub type Result<T> = result::Result<T, Error>;
