//! 错误集中面 (全仓规范: src/error.rs)

use std::result;

use thiserror::Error;

/// crate 错误枚举 — 迁移期仅收拢既有依赖错误, 后续族模块就地追加变体
#[derive(Error, Debug)]
pub enum Error {
  /// zone 分配失败 (v8/src/zone 面; HAMT 等 zone 分配器共用)
  #[error(transparent)]
  Zone(#[from] v8_zone::Error),
}

pub type Result<T> = result::Result<T, Error>;
