// 由 tool/src/inspector/agents.js 从 v8/src/inspector/v8-heap-profiler-agent-impl.h, src/inspector/v8-heap-profiler-agent-impl.cc, src/inspector/v8-profiler-agent-impl.h, src/inspector/v8-profiler-agent-impl.cc, src/inspector/v8-schema-agent-impl.h, src/inspector/v8-schema-agent-impl.cc, src/inspector/custom-preview.h, src/inspector/custom-preview.cc 生成; 修订就地改, 复跑覆盖
// Heap/Profiler/Schema 三 Agent 后端状态机与自定义预览限制

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// 自定义预览最大递归嵌套深度 (v8/src/inspector/custom-preview.h:15)
pub const K_MAX_CUSTOM_PREVIEW_DEPTH: i32 = 20;

/// 自定义对象预览信息 (v8/src/inspector/custom-preview.h:17)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomPreview {
  pub header: String,
  pub has_body: bool,
  pub body_getter_id: Option<String>,
  pub depth: i32,
}

impl CustomPreview {
  /// 构造自定义预览 (检查深度是否越界)
  pub fn new(
    header: impl Into<String>,
    has_body: bool,
    body_getter_id: Option<String>,
    depth: i32,
  ) -> Result<Self> {
    if depth > K_MAX_CUSTOM_PREVIEW_DEPTH {
      return Err(Error::MaxDepthExceeded(depth));
    }
    Ok(Self {
      header: header.into(),
      has_body,
      body_getter_id,
      depth,
    })
  }
}

/// 堆采样分析代理后端 (v8/src/inspector/v8-heap-profiler-agent-impl.h:24)
#[derive(Debug, Default)]
pub struct V8HeapProfilerAgentImpl {
  enabled: bool,
  tracking: bool,
  sampling_interval_bytes: usize,
}

impl V8HeapProfilerAgentImpl {
  /// 创建新堆分析代理
  pub fn new() -> Self {
    Self {
      enabled: false,
      tracking: false,
      sampling_interval_bytes: 32768,
    }
  }

  /// 启用堆分析
  #[inline]
  pub fn enable(&mut self) {
    self.enabled = true;
  }

  /// 停用堆分析
  #[inline]
  pub fn disable(&mut self) {
    self.enabled = false;
    self.tracking = false;
  }

  /// 是否已启用
  #[inline]
  pub const fn is_enabled(&self) -> bool {
    self.enabled
  }

  /// 开始对象追踪
  #[inline]
  pub fn start_tracking_heap_objects(&mut self) {
    self.tracking = true;
  }

  /// 停止对象追踪
  #[inline]
  pub fn stop_tracking_heap_objects(&mut self) {
    self.tracking = false;
  }

  /// 获取采样间隔
  #[inline]
  pub const fn sampling_interval_bytes(&self) -> usize {
    self.sampling_interval_bytes
  }
}

/// CPU 采样分析代理后端 (v8/src/inspector/v8-profiler-agent-impl.h:26)
#[derive(Debug, Default)]
pub struct V8ProfilerAgentImpl {
  enabled: bool,
  started: bool,
  sampling_interval_us: i32,
}

impl V8ProfilerAgentImpl {
  /// 创建新 CPU 分析代理
  pub fn new() -> Self {
    Self {
      enabled: false,
      started: false,
      sampling_interval_us: 1000,
    }
  }

  /// 启用代理
  #[inline]
  pub fn enable(&mut self) {
    self.enabled = true;
  }

  /// 停用代理
  #[inline]
  pub fn disable(&mut self) {
    self.enabled = false;
    self.started = false;
  }

  /// 开始分析
  #[inline]
  pub fn start(&mut self) {
    self.started = true;
  }

  /// 停止分析
  #[inline]
  pub fn stop(&mut self) {
    self.started = false;
  }
  /// 是否已启用
  #[inline]
  pub const fn is_enabled(&self) -> bool {
    self.enabled
  }

  /// 获取当前分析状态
  #[inline]
  pub const fn is_started(&self) -> bool {
    self.started
  }

  /// 获取采样间隔 (微秒)
  #[inline]
  pub const fn sampling_interval_us(&self) -> i32 {
    self.sampling_interval_us
  }

  /// 设置采样间隔 (微秒)
  #[inline]
  pub fn set_sampling_interval(&mut self, interval_us: i32) {
    self.sampling_interval_us = interval_us;
  }
}

/// CDP 协议结构代理后端 (v8/src/inspector/v8-schema-agent-impl.h:20)
#[derive(Debug, Clone)]
pub struct V8SchemaAgentImpl {
  domains: Vec<String>,
}

impl Default for V8SchemaAgentImpl {
  fn default() -> Self {
    Self::new()
  }
}

impl V8SchemaAgentImpl {
  /// 创建新协议代理
  pub fn new() -> Self {
    Self {
      domains: vec![
        "Schema".to_string(),
        "Runtime".to_string(),
        "Debugger".to_string(),
        "Console".to_string(),
        "Profiler".to_string(),
        "HeapProfiler".to_string(),
      ],
    }
  }

  /// 支持的域列表
  #[inline]
  pub fn get_domains(&self) -> &[String] {
    &self.domains
  }
}
