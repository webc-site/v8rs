use core::mem::discriminant;
use std::result;

use thiserror::Error;

/// crate 统一错误类型; 原版 CHECK/FatalProcessOutOfMemory/SBXCHECK 在此以错误变体承载 (禁 panic)
// 变体判等 (内部 Os 携带 io 错误不可全等比较, 按判别值即可)
impl PartialEq for Error {
  fn eq(&self, other: &Self) -> bool {
    discriminant(self) == discriminant(other)
  }
}
impl Eq for Error {}

#[derive(Error, Debug)]
pub enum Error {
  /// 沙箱虚拟地址空间预留失败 (原版 FatalProcessOutOfMemory "Failed to reserve ...")
  #[error("sandbox address space reservation failed")]
  SandboxReservationFailed,

  /// 指针不在沙箱内 (原版 CHECK(Sandbox::current()->Contains(pointer)))
  #[error("pointer outside sandbox")]
  SandboxViolation,

  /// 表未初始化即使用 (原版 CHECK(is_initialized()))
  #[error("table or sandbox not initialized")]
  NotInitialized,

  /// 重复初始化 (原版 CHECK(!initialized_))
  #[error("already initialized")]
  AlreadyInitialized,

  /// 表预留耗尽, 条目分配失败 (原版 FatalProcessOutOfMemory AllocateEntry 两次尝试后)
  #[error("table entry allocation failed (reservation exhausted)")]
  OutOfTableSpace,

  /// 只读段配额耗尽 (原版 CHECK_LT(read_only_segments_used_*kSegmentSize, ro_size))
  #[error("read-only segment quota exhausted")]
  ReadOnlySegmentExhausted,

  /// 无效参数 (非 2 的幂 / 未对齐等, 原版 CHECK(base::bits::IsPowerOfTwo(...)))
  #[error("invalid argument")]
  InvalidArgument,

  /// 平台不支持的操作 (原版 UNREACHABLE 的非 Linux pkey 路径等)
  #[error("operation unsupported on this platform")]
  Unsupported,

  /// 平台层错误透明转发
  #[error(transparent)]
  Os(#[from] v8_base_platform::Error),
}

pub type Result<T> = result::Result<T, Error>;
