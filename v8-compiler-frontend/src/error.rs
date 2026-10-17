use std::result;

use thiserror::Error;

use crate::fast_api_calls::CTypeInfoType;

#[derive(Error, Debug)]
pub enum Error {
  #[error(transparent)]
  CompilerGraph(#[from] v8_compiler_graph::Error),

  #[error(transparent)]
  CompilerOps(#[from] v8_compiler_ops::Error),

  #[error("unreachable CTypeInfo: {0:?}")]
  UnreachableCTypeInfo(CTypeInfoType),

  #[error("invalid argument: {0}")]
  InvalidArgument(&'static str),
}

pub type Result<T> = result::Result<T, Error>;
