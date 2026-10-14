//! 错误集中定义 (全仓规范: src/error.rs)

use std::result;

use thiserror::Error;

/// Maglev 后端错误枚举
#[derive(Error, Debug)]
pub enum Error {
  #[error("Maglev 后端错误: {0}")]
  MaglevBackend(String),
}

pub type Result<T, E = Error> = result::Result<T, E>;
