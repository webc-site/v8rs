use std::result;

use thiserror::Error;

/// 错误集中定义 (信号路径禁 panic; 可恢复使用错误统一走此枚举,
/// 契约破坏类错误保留原版 IMMEDIATE_CRASH 语义 → abort)
#[derive(Error, Debug)]
pub enum Error {
  /// EnableTrapHandler 重复调用, 或晚于 is_trap_handler_enabled 调用。
  /// 原版 TH_CHECK 即崩溃; 此处转 Err 供上层降级
  #[error("trap handler 已启用或已被查询, 禁止重复启用")]
  EnableTwice,
  /// 代码对象表达到 int 索引上限 (原版 kInvalidIndex 路径)
  #[error("代码对象表已满")]
  CodeObjectTableFull,
}

pub type Result<T> = result::Result<T, Error>;
