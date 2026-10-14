//! 错误集中定义 (全仓规范: src/error.rs)

use std::result;

use thiserror::Error;

/// 编译器后端错误枚举
#[derive(Error, Debug)]
pub enum Error {}

pub type Result<T> = result::Result<T, Error>;
