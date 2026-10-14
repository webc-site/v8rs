// v8-compiler-broker/src/error.rs
//! 堆对象代理系统错误类型定义

use std::result;

use thiserror::Error;

use crate::broker::BrokerMode;

/// 堆对象代理与编译期依赖错误枚举
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  /// 非法模式迁移
  #[error("非法 BrokerMode 状态迁移: 从 {from:?} 到 {to:?}")]
  InvalidModeTransition {
    /// 原状态
    from: BrokerMode,
    /// 目标状态
    to: BrokerMode,
  },

  /// 当前模式不允许序列化
  #[error("当前 BrokerMode ({0:?}) 禁止执行序列化操作")]
  SerializationNotAllowed(BrokerMode),

  /// 找不到指定地址的对象快照
  #[error("Broker 对象缓存表中未找到地址 {address:#x} 的快照")]
  ObjectNotFound {
    /// 对象地址
    address: u64,
  },

  /// 对象类型不匹配
  #[error("对象类型不匹配: 期望 {expected}, 实际为 {actual}")]
  TypeMismatch {
    /// 期望的类型描述
    expected: &'static str,
    /// 实际的类型描述
    actual: &'static str,
  },

  /// JSFunction 缺少初始 Map
  #[error("JSFunction 实例缺少 initial_map 属性")]
  MissingInitialMap,

  /// JSFunction 缺少原型对象
  #[error("JSFunction 实例缺少 instance_prototype")]
  MissingPrototype,

  /// JSFunction 缺少 SharedFunctionInfo
  #[error("JSFunction 实例缺少 SharedFunctionInfo 元数据")]
  MissingSharedFunctionInfo,

  /// 编译期假设依赖校验失败
  #[error("编译期依赖校验失效: {reason}")]
  DependencyValidationFailed {
    /// 失效原因
    reason: String,
  },

  /// Map 已废弃
  #[error("Map 处于废弃 (deprecated) 状态: 地址 {address:#x}")]
  DeprecatedMap {
    /// 废弃 Map 地址
    address: u64,
  },

  /// Zone 内存分配失败
  #[error("Zone 分配错误: {0}")]
  ZoneAllocation(String),

  /// 编译期假设依赖收集器已提交固化，禁止继续修改
  #[error("CompilationDependencies 处于已提交状态，禁止继续写入")]
  DependenciesAlreadyCommitted,
}

/// 统一结果类型
pub type Result<T> = result::Result<T, Error>;
