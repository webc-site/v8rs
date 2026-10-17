use core::result;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum Error {
  #[error("栈溢出")]
  StackOverflow,

  #[error("执行已被外部终止 (TerminateExecution)")]
  Terminated,

  #[error("微任务嵌套执行深度超出上限")]
  MicrotaskDepthExceeded,

  #[error("无效的 C 函数调用签名")]
  InvalidSignature,

  #[error("挂起异常: {0:#x}")]
  PendingException(usize),
}

pub type Result<T> = result::Result<T, Error>;
