// 由 tool/src/inspector/core.js 从 v8/src/inspector/v8-inspector-impl.h, src/inspector/v8-inspector-impl.cc, src/inspector/v8-inspector-session-impl.h, src/inspector/v8-inspector-session-impl.cc, src/inspector/test-interface.h, src/inspector/test-interface.cc, src/inspector/v8-debugger-barrier.h, src/inspector/v8-debugger-barrier.cc, src/inspector/v8-debugger-id.h, src/inspector/v8-debugger-id.cc, src/inspector/remote-object-id.h, src/inspector/remote-object-id.cc 生成; 修订就地改, 复跑覆盖
// 中枢连接/会话路由 + V8DebuggerId + RemoteObjectId 强类型实现

use std::collections::HashMap;

use crate::error::{Error, Result};

/// V8 调试器会话标识 (v8/src/inspector/v8-debugger-id.h:19)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct V8DebuggerId {
  first: i64,
  second: i64,
}

impl V8DebuggerId {
  /// 从二元组构造
  #[inline]
  pub const fn new(first: i64, second: i64) -> Self {
    Self { first, second }
  }

  /// 获取数值对
  #[inline]
  pub const fn pair(&self) -> (i64, i64) {
    (self.first, self.second)
  }

  /// 是否有效 (first || second 非零)
  #[inline]
  pub const fn is_valid(&self) -> bool {
    self.first != 0 || self.second != 0
  }

  /// 序列化为 "{first}.{second}" 格式
  pub fn to_string_repr(&self) -> String {
    format!("{}.{}", self.first, self.second)
  }

  /// 解析字符串格式
  pub fn parse(s: &str) -> Result<Self> {
    let dot_pos = s
      .find('.')
      .ok_or_else(|| Error::InvalidDebuggerId(s.to_string()))?;
    let first = s[..dot_pos]
      .parse::<i64>()
      .map_err(|_| Error::InvalidDebuggerId(s.to_string()))?;
    let second = s[dot_pos + 1..]
      .parse::<i64>()
      .map_err(|_| Error::InvalidDebuggerId(s.to_string()))?;
    Ok(Self { first, second })
  }
}

/// 远程对象 ID 基础结构 (v8/src/inspector/remote-object-id.h:16)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RemoteObjectIdBase {
  isolate_id: u64,
  injected_script_id: i32,
  id: i32,
}

impl RemoteObjectIdBase {
  /// 内部解析三段点分式 "{isolateId}.{injectedScriptId}.{id}"
  fn parse_id(s: &str) -> Result<(u64, i32, i32)> {
    let mut parts = s.split('.');
    let p1 = parts
      .next()
      .ok_or_else(|| Error::InvalidRemoteObjectId(s.to_string()))?;
    let p2 = parts
      .next()
      .ok_or_else(|| Error::InvalidRemoteObjectId(s.to_string()))?;
    let p3 = parts
      .next()
      .ok_or_else(|| Error::InvalidRemoteObjectId(s.to_string()))?;
    if parts.next().is_some() {
      return Err(Error::InvalidRemoteObjectId(s.to_string()));
    }
    let isolate_id = p1
      .parse::<u64>()
      .map_err(|_| Error::InvalidRemoteObjectId(s.to_string()))?;
    let injected_script_id = p2
      .parse::<i32>()
      .map_err(|_| Error::InvalidRemoteObjectId(s.to_string()))?;
    let id = p3
      .parse::<i32>()
      .map_err(|_| Error::InvalidRemoteObjectId(s.to_string()))?;
    Ok((isolate_id, injected_script_id, id))
  }

  /// 序列化为三段点分式
  fn serialize_id(isolate_id: u64, injected_script_id: i32, id: i32) -> String {
    format!("{isolate_id}.{injected_script_id}.{id}")
  }

  /// 隔离区标识
  #[inline]
  pub const fn isolate_id(&self) -> u64 {
    self.isolate_id
  }

  /// 上下文 / 注入脚本标识
  #[inline]
  pub const fn context_id(&self) -> i32 {
    self.injected_script_id
  }
}

/// 远程对象 ID (v8/src/inspector/remote-object-id.h:32)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RemoteObjectId {
  base: RemoteObjectIdBase,
}

impl RemoteObjectId {
  /// 从三段式解析
  pub fn parse(s: &str) -> Result<Self> {
    let (isolate_id, injected_script_id, id) = RemoteObjectIdBase::parse_id(s)?;
    Ok(Self {
      base: RemoteObjectIdBase {
        isolate_id,
        injected_script_id,
        id,
      },
    })
  }

  /// 序列化
  #[inline]
  pub fn serialize(isolate_id: u64, injected_script_id: i32, id: i32) -> String {
    RemoteObjectIdBase::serialize_id(isolate_id, injected_script_id, id)
  }

  /// 对象局部 ID
  #[inline]
  pub const fn id(&self) -> i32 {
    self.base.id
  }

  /// 隔离区 ID
  #[inline]
  pub const fn isolate_id(&self) -> u64 {
    self.base.isolate_id
  }

  /// 上下文 ID
  #[inline]
  pub const fn context_id(&self) -> i32 {
    self.base.injected_script_id
  }
}

/// 远程调用帧 ID (v8/src/inspector/remote-object-id.h:41)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RemoteCallFrameId {
  base: RemoteObjectIdBase,
}

impl RemoteCallFrameId {
  /// 从三段式解析
  pub fn parse(s: &str) -> Result<Self> {
    let (isolate_id, injected_script_id, id) =
      RemoteObjectIdBase::parse_id(s).map_err(|_| Error::InvalidCallFrameId(s.to_string()))?;
    Ok(Self {
      base: RemoteObjectIdBase {
        isolate_id,
        injected_script_id,
        id,
      },
    })
  }

  /// 序列化
  #[inline]
  pub fn serialize(isolate_id: u64, injected_script_id: i32, frame_ordinal: i32) -> String {
    RemoteObjectIdBase::serialize_id(isolate_id, injected_script_id, frame_ordinal)
  }

  /// 帧序号
  #[inline]
  pub const fn frame_ordinal(&self) -> i32 {
    self.base.id
  }
}

/// 调试屏障管理器 (v8/src/inspector/v8-debugger-barrier.h:16)
#[derive(Debug, Default)]
pub struct V8DebuggerBarrier {
  context_group_id: i32,
  paused: bool,
}

impl V8DebuggerBarrier {
  /// 构造调试屏障
  pub fn new(context_group_id: i32) -> Self {
    Self {
      context_group_id,
      paused: false,
    }
  }

  /// 上下文组 ID
  #[inline]
  pub const fn context_group_id(&self) -> i32 {
    self.context_group_id
  }

  /// 屏障是否已进入暂停
  #[inline]
  pub const fn is_paused(&self) -> bool {
    self.paused
  }

  /// 设置屏障暂停状态
  #[inline]
  pub fn set_paused(&mut self, paused: bool) {
    self.paused = paused;
  }
}

/// 会话状态模型 (v8/src/inspector/v8-inspector-session-impl.h:35)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V8InspectorSessionImpl {
  context_group_id: i32,
  session_id: i32,
  is_instrumentation: bool,
}

impl V8InspectorSessionImpl {
  /// 创建会话
  pub const fn new(context_group_id: i32, session_id: i32) -> Self {
    Self {
      context_group_id,
      session_id,
      is_instrumentation: false,
    }
  }

  /// 上下文组标识
  #[inline]
  pub const fn context_group_id(&self) -> i32 {
    self.context_group_id
  }

  /// 会话标识
  #[inline]
  pub const fn session_id(&self) -> i32 {
    self.session_id
  }

  /// 是否为 Instrumentation 会话
  #[inline]
  pub const fn is_instrumentation(&self) -> bool {
    self.is_instrumentation
  }

  /// 设置 Instrumentation 标识
  #[inline]
  pub fn set_instrumentation(&mut self, is_instrumentation: bool) {
    self.is_instrumentation = is_instrumentation;
  }
}

/// Inspector 中枢管理器 (v8/src/inspector/v8-inspector-impl.h:59)
#[derive(Debug, Default)]
pub struct V8InspectorImpl {
  next_session_id: i32,
  next_unique_id: i64,
  sessions: HashMap<(i32, i32), V8InspectorSessionImpl>,
  max_async_task_stacks_limit: i32,
}

impl V8InspectorImpl {
  /// 初始化管理器
  pub fn new() -> Self {
    Self {
      next_session_id: 1,
      next_unique_id: 1,
      sessions: HashMap::new(),
      max_async_task_stacks_limit: 128,
    }
  }

  /// 连接新会话
  pub fn connect(&mut self, context_group_id: i32) -> i32 {
    let sid = self.next_session_id;
    self.next_session_id += 1;
    let session = V8InspectorSessionImpl::new(context_group_id, sid);
    self.sessions.insert((context_group_id, sid), session);
    sid
  }

  /// 断开指定会话
  pub fn disconnect(&mut self, context_group_id: i32, session_id: i32) -> bool {
    self
      .sessions
      .remove(&(context_group_id, session_id))
      .is_some()
  }

  /// 按 (contextGroupId, sessionId) 双键查找会话
  pub fn session_by_id(
    &self,
    context_group_id: i32,
    session_id: i32,
  ) -> Option<&V8InspectorSessionImpl> {
    self.sessions.get(&(context_group_id, session_id))
  }

  /// 生成单调递增唯一标识
  pub fn generate_unique_id(&mut self) -> i64 {
    let id = self.next_unique_id;
    self.next_unique_id += 1;
    id
  }

  /// 生成调试器 ID 结构
  pub fn generate_debugger_id(&mut self) -> V8DebuggerId {
    let first = self.generate_unique_id();
    let second = self.generate_unique_id();
    V8DebuggerId::new(first, second)
  }

  /// 设置异步任务栈最大采样深度 (test-interface.h:11)
  pub fn set_max_async_task_stacks_for_test(&mut self, limit: i32) {
    self.max_async_task_stacks_limit = limit;
  }

  /// 获取测试用异步栈深度限制
  pub const fn max_async_task_stacks_limit(&self) -> i32 {
    self.max_async_task_stacks_limit
  }
}
