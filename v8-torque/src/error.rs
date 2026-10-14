//! 错误集中定义: 对应原版 ReportError/MessageBuilder 异常流 → Result。

use std::{io, result};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  /// 骨架阶段未填函数体, 后续波次逐块替换
  #[error("unimplemented")]
  Unimplemented,
  /// 编译期报错 (原版 Error(...).Throw()), 携带消息
  #[error("torque error: {0}")]
  Message(String),
  /// 消息已收集到上下文, 直接中止 (对应 TorqueAbortCompilation)
  #[error("compilation aborted, see collected messages")]
  Abort,
  /// 打开文件失败 (原版 "Cannot open file path/uri")
  #[error("cannot open file path/uri: {0}")]
  OpenFile(String),
  /// 文件 IO 错误
  #[error(transparent)]
  Io(#[from] io::Error),
}

pub type Result<T> = result::Result<T, Error>;
