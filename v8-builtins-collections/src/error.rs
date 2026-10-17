use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Collection target is not an object")]
  NotAnObject,

  #[error("Key is not an object (WeakCollection requires object keys)")]
  InvalidWeakKey,

  #[error("OrderedHashTable capacity overflow")]
  CapacityOverflow,

  #[error("Set operation failed: {0}")]
  SetOperationFailed(String),
}

pub type Result<T> = result::Result<T, Error>;
