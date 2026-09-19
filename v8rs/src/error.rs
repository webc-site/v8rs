use std::result;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  // #[error(transparent)]
}

pub type Result<T> = result::Result<T, Error>;
