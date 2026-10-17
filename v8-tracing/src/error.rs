use std::result;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  /// 全局 TracingController 已安装 (set_tracing_controller 二次调用)。
  #[error("tracing controller already installed")]
  ControllerInstalled,
  /// TracingCategoryObserver::SetUp 重复调用。
  #[error("tracing category observer already installed")]
  ObserverInstalled,
  /// observer 单例锁中毒。
  #[error("observer state mutex poisoned")]
  ObserverPoisoned,
}

pub type Result<T> = result::Result<T, Error>;
