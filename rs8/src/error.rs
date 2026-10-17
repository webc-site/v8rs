use std::{io, result};

use thiserror::Error;

use crate::runtime::JsValue;

#[derive(Error, Debug)]
pub enum Error {
  #[error("JS 语法解析错误: {0}")]
  ParseError(String),
  #[error("JS 运行时错误: {0}")]
  RuntimeError(String),
  #[error("ReferenceError: {0}")]
  ReferenceError(String),
  #[error("TypeError: {0}")]
  TypeError(String),
  #[error("RangeError: {0}")]
  RangeError(String),
  #[error("JS 抛出异常: {0}")]
  Thrown(JsValue),
  #[error("字节码链路不支持的构造: {0}")]
  Unsupported(String),
  #[error(transparent)]
  IoError(#[from] io::Error),
}

pub type Result<T> = result::Result<T, Error>;
