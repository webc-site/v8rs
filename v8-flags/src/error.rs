use std::result;

use thiserror::Error;

use crate::value::FlagType;

/// flag 处理错误 (对应原版 FlagError 终止路径 / 解析错误, 统一 Result 化)
#[derive(Error, Debug)]
pub enum Error {
  /// 命令行出现未识别的 flag (remove=false 时中断解析)
  #[error("未识别的 flag: {arg}")]
  Unrecognized { arg: String },

  /// 非布尔 flag 缺值 (命令行末尾)
  #[error("flag {arg} 缺少值 (类型 {ty})")]
  MissingValue { arg: String, ty: FlagType },

  /// 值非法 (bool 带值 / 非布尔取反 / 数值尾部残余)
  #[error("flag {arg} 值非法 (类型 {ty})")]
  IllegalValue { arg: String, ty: FlagType },

  /// 无符号值越界 (原版 TryParseUnsigned 拒负值/超上限)
  #[error("flag {arg} 无符号值越界")]
  OutOfBounds { arg: String },

  /// 带 '=' 的参数 flag 名超 1KB 解析缓冲 (原版 SplitArgument 的 FlagError 路径)
  #[error("flag 名过长: {arg}")]
  NameTooLong { arg: String },

  /// 只读 flag 被显式改值 (矛盾检查开启时)
  #[error("只读 flag {name} 的值被显式变更")]
  ReadonlyConflict { name: &'static str },

  /// 只读 flag 被蕴含改值
  #[error("只读 flag {name} 被 {by} 蕴含冲突")]
  ReadonlyImplied {
    name: &'static str,
    by: &'static str,
  },

  /// 两个弱蕴含互相冲突
  #[error("flag {name} 的弱蕴含冲突: {a:?} 与 {b:?}")]
  WeakImplications {
    name: &'static str,
    a: Option<&'static str>,
    b: Option<&'static str>,
  },

  /// 两个强蕴含互相冲突
  #[error("flag {name} 的蕴含冲突: {a:?} 与 {b:?}")]
  Implications {
    name: &'static str,
    a: Option<&'static str>,
    b: Option<&'static str>,
  },

  /// bool flag 值与显式设置冲突
  #[error("flag {name} 被 {by} 蕴含, 与显式设置冲突")]
  ImpliedVsExplicit {
    name: &'static str,
    by: &'static str,
  },

  /// 非 bool flag 同时被蕴含与显式设置
  #[error("flag {name} 被 {by} 蕴含, 但也被显式设置")]
  ImpliedButExplicit {
    name: &'static str,
    by: &'static str,
  },

  /// bool flag 命令行同时给 true 和 false
  #[error("命令行 flag {name} 同时指定了 true 与 false")]
  CmdBothTrueFalse { name: &'static str },

  /// 非 bool flag 命令行重复指定
  #[error("命令行 flag {name} 被重复指定")]
  CmdMultiple { name: &'static str },

  /// DEFINE_REQUIREMENT 不变式被违反 (原版直接终止进程)
  #[error("flag 不变式违反: {stmt}")]
  Requirement { stmt: &'static str },

  /// 蕴含形成环 (原版 CheckForCycle 哈希命中)
  #[error("flag 蕴含存在环")]
  ImplicationCycle,

  /// flag 已冻结, 拒绝赋值 (原版 CHECK 失败点)
  #[error("flag 已冻结")]
  Frozen,

  /// --flag-processing-mode 值非法
  #[error("处理模式值非法: {0}")]
  InvalidProcessingMode(String),
}

pub type Result<T> = result::Result<T, Error>;
