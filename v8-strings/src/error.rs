use std::result;

use thiserror::Error;

use crate::uri::UriError;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  #[error(transparent)]
  Uri(#[from] UriError),
}

pub type Result<T> = result::Result<T, Error>;
