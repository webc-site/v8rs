use std::{io, result};

use thiserror::Error;

/// crate 统一错误类型；字符串错误禁止，全部用此枚举承载。
#[derive(Error, Debug)]
pub enum Error {
  /// 线程启动失败（std::thread::Builder::spawn 返回 Err）。
  #[error("thread start failed")]
  ThreadStartFailed,

  /// 线程 join 失败（worker panic 或句柄无效）。
  #[error("thread join failed")]
  JoinFailed,

  /// TLS key 创建失败（pthread_key_create 返回非零）。
  #[error("tls key create failed")]
  TlsKeyCreateFailed,

  /// mmap 失败。
  #[error("mmap failed")]
  MmapFailed,

  /// mprotect 失败。
  #[error("mprotect failed")]
  MProtectFailed,

  /// madvise 失败。
  #[error("madvise failed")]
  MadviseFailed,

  /// munmap 失败。
  #[error("munmap failed")]
  MunmapFailed,

  /// RemapPages 平台不支持。
  #[error("remap pages unsupported on this platform")]
  RemapUnsupported,

  /// 平台不支持的操作。
  #[error("operation unsupported on this platform")]
  Unsupported,

  /// 无效参数。
  #[error("invalid argument")]
  InvalidArgument,

  /// 文件相关错误透传。
  #[error(transparent)]
  Io(#[from] io::Error),

  /// 原始 OS errno。
  #[error("os error: {0}")]
  Os(i32),
}

pub type Result<T> = result::Result<T, Error>;
