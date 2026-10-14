use std::result;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  /// 参数/状态非法 (对应原版 CHECK 失败路径的 Rust 化承接)
  #[error("非法参数: {0}")]
  InvalidArg(&'static str),
}

pub type Result<T> = result::Result<T, Error>;
