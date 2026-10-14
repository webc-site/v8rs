// 由 tool/src/inspector/runtime.js 从 v8/src/inspector/v8-runtime-agent-impl.h, src/inspector/v8-runtime-agent-impl.cc, src/inspector/injected-script.h, src/inspector/injected-script.cc, src/inspector/inspected-context.h, src/inspector/inspected-context.cc, src/inspector/remote-object-id.h, src/inspector/remote-object-id.cc, src/inspector/value-mirror.h, src/inspector/value-mirror.cc 生成; 修订就地改, 复跑覆盖
// Runtime 域: V8InternalValueType/DiscardReason 枚举与 InspectedContext/RemoteObject

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use strum::{Display, EnumString};

/// V8 内部值类型 (v8/src/inspector/inspected-context.h:31)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
pub enum V8InternalValueType {
  /// None
  None,
  /// Entry
  Entry,
  /// Scope
  Scope,
  /// ScopeList
  ScopeList,
  /// PrivateMethodList
  PrivateMethodList,
  /// PrivateMethod
  PrivateMethod,
}

impl V8InternalValueType {
  /// 是否为无内部值
  #[inline]
  pub const fn is_none(&self) -> bool {
    matches!(self, Self::None)
  }

  /// 是否为作用域类型
  #[inline]
  pub const fn is_scope(&self) -> bool {
    matches!(self, Self::Scope | Self::ScopeList)
  }

  /// 是否为私有方法
  #[inline]
  pub const fn is_private_method(&self) -> bool {
    matches!(self, Self::PrivateMethod | Self::PrivateMethodList)
  }
}

/// 注入脚本丢弃原因 (v8/src/inspector/injected-script.h:299)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
pub enum DiscardReason {
  /// Fulfilled
  Fulfilled,
  /// PromiseCollected
  PromiseCollected,
  /// TearDown
  TearDown,
}

impl DiscardReason {
  /// 是否因已敲定而丢弃
  #[inline]
  pub const fn is_fulfilled(&self) -> bool {
    matches!(self, Self::Fulfilled)
  }

  /// 是否因销毁而丢弃
  #[inline]
  pub const fn is_tear_down(&self) -> bool {
    matches!(self, Self::TearDown)
  }
}

/// 受检上下文元数据 (v8/src/inspector/inspected-context.h:38)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectedContext {
  context_id: i32,
  context_group_id: i32,
  origin: String,
  human_readable_name: String,
}

impl InspectedContext {
  /// 构造受检上下文
  pub fn new(
    context_id: i32,
    context_group_id: i32,
    origin: impl Into<String>,
    human_readable_name: impl Into<String>,
  ) -> Self {
    Self {
      context_id,
      context_group_id,
      origin: origin.into(),
      human_readable_name: human_readable_name.into(),
    }
  }

  /// 上下文 ID
  #[inline]
  pub const fn context_id(&self) -> i32 {
    self.context_id
  }

  /// 上下文组 ID
  #[inline]
  pub const fn context_group_id(&self) -> i32 {
    self.context_group_id
  }

  /// 来源源名
  #[inline]
  pub fn origin(&self) -> &str {
    &self.origin
  }

  /// 可读名称
  #[inline]
  pub fn human_readable_name(&self) -> &str {
    &self.human_readable_name
  }
}

/// 注入脚本模型 (v8/src/inspector/injected-script.h:78)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InjectedScript {
  injected_script_id: i32,
  context_id: i32,
}

impl InjectedScript {
  /// 构造注入脚本实例
  pub const fn new(injected_script_id: i32, context_id: i32) -> Self {
    Self {
      injected_script_id,
      context_id,
    }
  }

  /// 注入脚本标识
  #[inline]
  pub const fn injected_script_id(&self) -> i32 {
    self.injected_script_id
  }

  /// 上下文标识
  #[inline]
  pub const fn context_id(&self) -> i32 {
    self.context_id
  }
}

/// CDP 远程对象表达 (Chrome DevTools Protocol RemoteObject)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteObject {
  #[serde(rename = "type")]
  pub type_: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subtype: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<serde_json::Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub object_id: Option<String>,
}

impl RemoteObject {
  /// 构造基础字符串类型远程对象
  pub fn string(val: impl Into<String>) -> Self {
    let s = val.into();
    Self {
      type_: "string".to_string(),
      subtype: None,
      description: Some(s.clone()),
      value: Some(Value::String(s)),
      object_id: None,
    }
  }

  /// 构造基础数字类型远程对象
  pub fn number(val: f64) -> Self {
    Self {
      type_: "number".to_string(),
      subtype: None,
      description: Some(val.to_string()),
      value: Number::from_f64(val).map(Value::Number),
      object_id: None,
    }
  }

  /// 构造布尔类型远程对象
  pub fn boolean(val: bool) -> Self {
    Self {
      type_: "boolean".to_string(),
      subtype: None,
      description: Some(val.to_string()),
      value: Some(Value::Bool(val)),
      object_id: None,
    }
  }

  /// 构造 undefined
  pub fn undefined() -> Self {
    Self {
      type_: "undefined".to_string(),
      subtype: None,
      description: Some("undefined".to_string()),
      value: None,
      object_id: None,
    }
  }

  /// 构造 null
  pub fn null() -> Self {
    Self {
      type_: "object".to_string(),
      subtype: Some("null".to_string()),
      description: Some("null".to_string()),
      value: Some(Value::Null),
      object_id: None,
    }
  }
}

/// 值镜像结构 (v8/src/inspector/value-mirror.h:50)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueMirror {
  type_: String,
  subtype: Option<String>,
  description: String,
}

impl ValueMirror {
  /// 构造镜像
  pub fn new(
    type_: impl Into<String>,
    subtype: Option<String>,
    description: impl Into<String>,
  ) -> Self {
    Self {
      type_: type_.into(),
      subtype,
      description: description.into(),
    }
  }

  /// 主类型
  #[inline]
  pub fn type_name(&self) -> &str {
    &self.type_
  }

  /// 描述信息
  #[inline]
  pub fn description(&self) -> &str {
    &self.description
  }

  /// 子类型
  #[inline]
  pub fn subtype(&self) -> Option<&str> {
    self.subtype.as_deref()
  }
}
