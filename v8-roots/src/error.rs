use std::result;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  #[error("无效根索引: {0}")]
  InvalidRootIndex(usize),
  #[error("只读根已锁定，禁止修改: {0}")]
  ReadOnlyRootMutation(&'static str),
  #[error("未初始化的根对象: {0}")]
  UninitializedRoot(&'static str),
}

pub type Result<T> = result::Result<T, Error>;
