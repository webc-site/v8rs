// 由 tool/src/inspector/debugger.js 从 v8/src/inspector/v8-debugger.h, src/inspector/v8-debugger.cc, src/inspector/v8-debugger-agent-impl.h, src/inspector/v8-debugger-agent-impl.cc, src/inspector/v8-debugger-script.h, src/inspector/v8-debugger-script.cc, src/inspector/v8-deep-serializer.h, src/inspector/v8-deep-serializer.cc, src/inspector/v8-serialization-duplicate-tracker.h, src/inspector/v8-serialization-duplicate-tracker.cc 生成; 修订就地改, 复跑覆盖
// Debugger 域: BreakpointType/WrapMode/Language 及 V8DebuggerScript/去重跟踪器

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::error::{Error, Result};

/// 对象序列化包装模式 (v8/src/inspector/v8-debugger.h:35)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
pub enum WrapMode {
  /// JSON 直接表达
  Json,
  /// 仅保留对象标识
  IdOnly,
  /// 生成轻量预览
  Preview,
  /// 深度完全序列化
  Deep,
}

impl WrapMode {
  /// 是否为深拷贝序列化
  #[inline]
  pub const fn is_deep(&self) -> bool {
    matches!(self, Self::Deep)
  }

  /// 是否为预览模式
  #[inline]
  pub const fn is_preview(&self) -> bool {
    matches!(self, Self::Preview)
  }
}

/// 断点类型枚举 (v8/src/inspector/v8-debugger-agent-impl.h:34)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
#[repr(u8)]
pub enum BreakpointType {
  /// ByUrl = 1
  ByUrl = 1,
  /// ByUrlRegex = 2
  ByUrlRegex = 2,
  /// ByScriptHash = 3
  ByScriptHash = 3,
  /// ByScriptId = 4
  ByScriptId = 4,
  /// DebugCommand = 5
  DebugCommand = 5,
  /// MonitorCommand = 6
  MonitorCommand = 6,
  /// BreakpointAtEntry = 7
  BreakpointAtEntry = 7,
  /// InstrumentationBreakpoint = 8
  InstrumentationBreakpoint = 8,
}

impl BreakpointType {
  /// 从原始数值解析
  pub const fn from_u8(tag: u8) -> Result<Self> {
    match tag {
      1 => Ok(Self::ByUrl),
      2 => Ok(Self::ByUrlRegex),
      3 => Ok(Self::ByScriptHash),
      4 => Ok(Self::ByScriptId),
      5 => Ok(Self::DebugCommand),
      6 => Ok(Self::MonitorCommand),
      7 => Ok(Self::BreakpointAtEntry),
      8 => Ok(Self::InstrumentationBreakpoint),
      _ => Err(Error::InvalidBreakpointType(tag)),
    }
  }

  /// 是否为通过 URL 设定的断点
  #[inline]
  pub const fn is_by_url(&self) -> bool {
    matches!(self, Self::ByUrl | Self::ByUrlRegex)
  }

  /// 是否为通过脚本哈希或 ID 设定的断点
  #[inline]
  pub const fn is_by_script(&self) -> bool {
    matches!(self, Self::ByScriptHash | Self::ByScriptId)
  }

  /// 是否为控制台命令触发断点
  #[inline]
  pub const fn is_command(&self) -> bool {
    matches!(self, Self::DebugCommand | Self::MonitorCommand)
  }
}

/// 调试器脚本语言类型 (v8/src/inspector/v8-debugger-script.h:56)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
pub enum DebuggerScriptLanguage {
  /// JavaScript
  JavaScript,
  /// WebAssembly
  WebAssembly,
}

impl DebuggerScriptLanguage {
  /// 是否为 JavaScript
  #[inline]
  pub const fn is_javascript(&self) -> bool {
    matches!(self, Self::JavaScript)
  }

  /// 是否为 WebAssembly
  #[inline]
  pub const fn is_wasm(&self) -> bool {
    matches!(self, Self::WebAssembly)
  }
}

/// 脚本行列位置区间 (v8/src/inspector/v8-debugger-script.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ScriptPositionSpan {
  pub start_line: i32,
  pub start_column: i32,
  pub end_line: i32,
  pub end_column: i32,
}

impl ScriptPositionSpan {
  /// 构造区间
  pub const fn new(start_line: i32, start_column: i32, end_line: i32, end_column: i32) -> Self {
    Self {
      start_line,
      start_column,
      end_line,
      end_column,
    }
  }
}

/// 调试脚本元数据 (v8/src/inspector/v8-debugger-script.h:58)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V8DebuggerScript {
  script_id: i32,
  url: String,
  source_mapping_url: Option<String>,
  hash: String,
  span: ScriptPositionSpan,
  language: DebuggerScriptLanguage,
}

impl V8DebuggerScript {
  /// 构造调试脚本模型
  pub fn new(
    script_id: i32,
    url: impl Into<String>,
    source_mapping_url: Option<String>,
    hash: impl Into<String>,
    span: ScriptPositionSpan,
    language: DebuggerScriptLanguage,
  ) -> Self {
    Self {
      script_id,
      url: url.into(),
      source_mapping_url,
      hash: hash.into(),
      span,
      language,
    }
  }

  /// 起始行号
  #[inline]
  pub const fn start_line(&self) -> i32 {
    self.span.start_line
  }

  /// 起始列号
  #[inline]
  pub const fn start_column(&self) -> i32 {
    self.span.start_column
  }

  /// 结束行号
  #[inline]
  pub const fn end_line(&self) -> i32 {
    self.span.end_line
  }

  /// 结束列号
  #[inline]
  pub const fn end_column(&self) -> i32 {
    self.span.end_column
  }

  /// 脚本 ID
  #[inline]
  pub const fn script_id(&self) -> i32 {
    self.script_id
  }

  /// 源 URL
  #[inline]
  pub fn url(&self) -> &str {
    &self.url
  }

  /// 源码映射 URL
  #[inline]
  pub fn source_mapping_url(&self) -> Option<&str> {
    self.source_mapping_url.as_deref()
  }

  /// 脚本哈希
  #[inline]
  pub fn hash(&self) -> &str {
    &self.hash
  }

  /// 行列区间
  #[inline]
  pub const fn span(&self) -> ScriptPositionSpan {
    self.span
  }

  /// 语言类型
  #[inline]
  pub const fn language(&self) -> DebuggerScriptLanguage {
    self.language
  }
}

/// 断点记录结构
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V8Breakpoint {
  id: i32,
  breakpoint_type: BreakpointType,
  condition: Option<String>,
  hit_count: u32,
}

impl V8Breakpoint {
  /// 创建新断点
  pub fn new(id: i32, breakpoint_type: BreakpointType, condition: Option<String>) -> Self {
    Self {
      id,
      breakpoint_type,
      condition,
      hit_count: 0,
    }
  }

  /// 断点 ID
  #[inline]
  pub const fn id(&self) -> i32 {
    self.id
  }

  /// 断点类型
  #[inline]
  pub const fn breakpoint_type(&self) -> BreakpointType {
    self.breakpoint_type
  }

  /// 断点触发条件表达式
  #[inline]
  pub fn condition(&self) -> Option<&str> {
    self.condition.as_deref()
  }

  /// 命中计数累加
  #[inline]
  pub fn record_hit(&mut self) {
    self.hit_count = self.hit_count.saturating_add(1);
  }

  /// 命中计数
  #[inline]
  pub const fn hit_count(&self) -> u32 {
    self.hit_count
  }
}

/// 序列化去重跟踪器 (v8/src/inspector/v8-serialization-duplicate-tracker.h:20)
#[derive(Debug, Default)]
pub struct V8SerializationDuplicateTracker {
  known_objects: HashSet<u64>,
}

impl V8SerializationDuplicateTracker {
  /// 创建新去重跟踪器
  pub fn new() -> Self {
    Self {
      known_objects: HashSet::new(),
    }
  }

  /// 查询是否已访问过此对象地址/ID
  #[inline]
  pub fn find_known_object(&self, object_id: u64) -> bool {
    self.known_objects.contains(&object_id)
  }

  /// 记录并标记此对象已访问
  #[inline]
  pub fn remember_object(&mut self, object_id: u64) -> bool {
    self.known_objects.insert(object_id)
  }

  /// 清除所有跟踪
  #[inline]
  pub fn clear(&mut self) {
    self.known_objects.clear();
  }
}

/// V8 调试器模型管理器 (v8/src/inspector/v8-debugger.h:51)
#[derive(Debug, Default)]
pub struct V8Debugger {
  breakpoints_active: bool,
  next_breakpoint_id: i32,
  breakpoints: HashMap<i32, V8Breakpoint>,
}

impl V8Debugger {
  /// 创建新调试器模型
  pub fn new() -> Self {
    Self {
      breakpoints_active: true,
      next_breakpoint_id: 1,
      breakpoints: HashMap::new(),
    }
  }

  /// 设置断点总开关
  #[inline]
  pub fn set_breakpoints_active(&mut self, active: bool) {
    self.breakpoints_active = active;
  }

  /// 获取断点总开关状态
  #[inline]
  pub const fn breakpoints_active(&self) -> bool {
    self.breakpoints_active
  }

  /// 设置新断点
  pub fn set_breakpoint(&mut self, bp_type: BreakpointType, condition: Option<String>) -> i32 {
    let id = self.next_breakpoint_id;
    self.next_breakpoint_id += 1;
    let bp = V8Breakpoint::new(id, bp_type, condition);
    self.breakpoints.insert(id, bp);
    id
  }

  /// 移除断点
  pub fn remove_breakpoint(&mut self, id: i32) -> bool {
    self.breakpoints.remove(&id).is_some()
  }

  /// 获取断点引用
  pub fn get_breakpoint(&self, id: i32) -> Option<&V8Breakpoint> {
    self.breakpoints.get(&id)
  }
}
