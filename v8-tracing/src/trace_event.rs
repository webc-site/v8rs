// 由 tool/src/tracing/trace-event.h.js 从 v8/src/tracing/trace-event.h;src/tracing/trace-event.cc 生成; 修订就地改, 复跑覆盖
// 签名骨架 + 常量, 语义体就地修订

//! trace-event.h/.cc 核心管道 (672+63 行)。
//!
//! 原版链路: `TRACE_EVENT*` 宏 (→ trace_event_macros) →
//! `AddTraceEvent[WithTimestamp]` 模板重载 (6 个, 0/1/2 参, trace-event.h:476-559) →
//! `AddTraceEventImpl` (extract convertable, trace-event.h:383) →
//! `TracingController::AddTraceEvent` (embedder 虚函数, include/v8-platform.h:419)。
//! Rust 收敛: 重载塌缩为 `emit`/`emit_ts`/`emit_id`/`emit_id_ts` 4 核 + `scoped_add`(2);
//! `SetTraceValue` 类型打标 (trace-event.h:431-468) → `TraceValue` 类型化 enum。
//!
//! `CategoryGroupEnabledFlags` (trace-event.h:35-42): GetCategoryGroupEnabled 返回字节的位含义。
//!
//! 【推迟登记】(trace-event.cc:25-57): `CallStatsScopedTracer` 依赖
//! `Isolate::counters()->runtime_call_stats()` (L3 对象层), 待 v8-execution 回收;
//! `TraceEventHelper::GetTracingController` 的 `V8::GetCurrentPlatform` 全局链
//! (trace-event.cc:19-21) 以本模块全局 controller 安装点等价承接。

use std::{
  fmt,
  sync::{Arc, OnceLock},
};

use coarsetime::{Clock, Instant};

use crate::{
  Error,
  platform::{ConvertableToTraceFormat, NoopController, TraceStateObserver, TracingController},
  trace_event_macros::{
    TRACE_EVENT_FLAG_HAS_ID, TRACE_EVENT_FLAG_NONE, TRACE_EVENT_PHASE_COMPLETE,
    TRACE_VALUE_TYPE_BOOL, TRACE_VALUE_TYPE_CONVERTABLE, TRACE_VALUE_TYPE_COPY_STRING,
    TRACE_VALUE_TYPE_DOUBLE, TRACE_VALUE_TYPE_INT, TRACE_VALUE_TYPE_POINTER,
    TRACE_VALUE_TYPE_STRING, TRACE_VALUE_TYPE_UINT,
  },
};

// ---- CategoryGroupEnabledFlags (trace-event.h:35-42) ----
/// kEnabledForRecording_CategoryGroupEnabledFlags (trace-event.h:37)。
pub const ENABLED_FOR_RECORDING: u8 = 1 << 0;
/// kEnabledForEventCallback_CategoryGroupEnabledFlags (trace-event.h:39)。
pub const ENABLED_FOR_EVENT_CALLBACK: u8 = 1 << 2;
/// kEnabledForETWExport_CategoryGroupEnabledFlags (trace-event.h:41)。
pub const ENABLED_FOR_ETW_EXPORT: u8 = 1 << 3;

/// recording 模式启用位掩码 (trace-event.h:57-60, RECORDING|CALLBACK)。
const RECORDING_MODE_MASK: u8 = ENABLED_FOR_RECORDING | ENABLED_FOR_EVENT_CALLBACK;

/// kZeroNumArgs (trace-event.h:308)。
pub const K_ZERO_NUM_ARGS: i32 = 0;
/// kNoId (trace-event.h:310)。
pub const K_NO_ID: u64 = 0;
/// kGlobalScope = nullptr scope (trace-event.h:309)。
pub const K_GLOBAL_SCOPE: Option<&'static str> = None;

// ---- 全局 controller 安装点 (TraceEventHelper::GetTracingController, trace-event.cc:19-21) ----

static CONTROLLER: OnceLock<Arc<dyn TracingController>> = OnceLock::new();

/// no-op controller 单例 (NoopController 无状态, 共享实例免热路径分配)
static NOOP: OnceLock<Arc<dyn TracingController>> = OnceLock::new();

/// 当前 controller; 未安装时回落 no-op (v8-platform.h inline 空实现语义)。
fn controller() -> Arc<dyn TracingController> {
  CONTROLLER
    .get()
    .cloned()
    .unwrap_or_else(|| NOOP.get_or_init(|| Arc::new(NoopController)).clone())
}

/// 安装全局 TracingController; 二次安装报错 (embedder 一次性安装约定)。
pub fn set_tracing_controller(ctrl: Arc<dyn TracingController>) -> crate::Result<()> {
  CONTROLLER.set(ctrl).map_err(|_| Error::ControllerInstalled)
}

/// 取当前全局 controller (未安装为 no-op 默认实现)。
pub fn tracing_controller() -> Arc<dyn TracingController> {
  controller()
}

/// `AddTraceStateObserver` 便捷转发 (v8-platform.h:461)。
pub fn add_trace_state_observer(observer: Arc<dyn TraceStateObserver>) {
  controller().add_trace_state_observer(observer);
}

/// `RemoveTraceStateObserver` 便捷转发 (v8-platform.h:467)。
pub fn remove_trace_state_observer(observer: &Arc<dyn TraceStateObserver>) {
  controller().remove_trace_state_observer(observer);
}

/// `TRACE_EVENT_API_LOAD_CATEGORY_GROUP_ENABLED` + recording 模式位检查
/// (trace-event.h:57-60): 宏层唯一的类别启用早退判定。
pub fn category_enabled(category_group: &str) -> bool {
  controller().get_category_group_enabled(category_group) & RECORDING_MODE_MASK != 0
}

/// `TRACE_TIME_TICKS_NOW()` (trace-event-no-perfetto.h:898): 单调时钟纳秒
/// (`base::TimeTicks::Now()`, coarsetime 低开销时钟)。
pub fn ticks_now_ns() -> i64 {
  Instant::now().as_ticks() as i64
}

/// `TRACE_TIME_NOW()` (trace-event-no-perfetto.h:902): 墙钟纳秒
/// (`base::Time::Now()`, epoch 起)。
pub fn time_now_ns() -> i64 {
  Clock::now_since_epoch().as_u64() as i64
}

// ---- SetTraceValue 类型打标 (trace-event.h:431-468) ----

/// 追踪事件实参值: 原版 (type, uint64 位模式) 二元组的类型化等价物。
/// 变体 ↔ `TRACE_VALUE_TYPE_*` 一一对应 (trace-event.h:916-927)。
pub enum TraceValue {
  /// bool → TRACE_VALUE_TYPE_BOOL。
  Bool(bool),
  /// 无符号整型 → TRACE_VALUE_TYPE_UINT。
  Uint(u64),
  /// 有符号整型/枚举 → TRACE_VALUE_TYPE_INT。
  Int(i64),
  /// double 位模式 → TRACE_VALUE_TYPE_DOUBLE。
  Double(f64),
  /// const void* → TRACE_VALUE_TYPE_POINTER。
  Pointer(u64),
  /// 长生命周期 const char* → TRACE_VALUE_TYPE_STRING (不拷贝)。
  Str(&'static str),
  /// `TraceStringWithCopy` → TRACE_VALUE_TYPE_COPY_STRING。
  CopyStr(String),
  /// `ConvertableToTraceFormat*` → TRACE_VALUE_TYPE_CONVERTABLE。
  Convertable(Box<dyn ConvertableToTraceFormat>),
}

impl fmt::Debug for TraceValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Convertable 无 Debug 面 → 打类型标签占位
    match self {
      Self::Bool(v) => write!(f, "Bool({v})"),
      Self::Uint(v) => write!(f, "Uint({v})"),
      Self::Int(v) => write!(f, "Int({v})"),
      Self::Double(v) => write!(f, "Double({v})"),
      Self::Pointer(v) => write!(f, "Pointer({v:#x})"),
      Self::Str(v) => write!(f, "Str({v})"),
      Self::CopyStr(v) => write!(f, "CopyStr({v})"),
      Self::Convertable(_) => write!(f, "Convertable(<dyn>)"),
    }
  }
}

impl TraceValue {
  /// 原版平面数组 arg_types[i] 的类型标签 (SetTraceValue 写入的 type)。
  pub const fn type_tag(&self) -> u8 {
    match self {
      Self::Bool(_) => TRACE_VALUE_TYPE_BOOL,
      Self::Uint(_) => TRACE_VALUE_TYPE_UINT,
      Self::Int(_) => TRACE_VALUE_TYPE_INT,
      Self::Double(_) => TRACE_VALUE_TYPE_DOUBLE,
      Self::Pointer(_) => TRACE_VALUE_TYPE_POINTER,
      Self::Str(_) => TRACE_VALUE_TYPE_STRING,
      Self::CopyStr(_) => TRACE_VALUE_TYPE_COPY_STRING,
      Self::Convertable(_) => TRACE_VALUE_TYPE_CONVERTABLE,
    }
  }

  /// `SetTraceValue(const void*)` (trace-event.h:448): 指针实参。
  pub const fn pointer(p: usize) -> Self {
    Self::Pointer(p as u64)
  }
}

macro_rules! impl_trace_value_from {
  ($($t:ty => $variant:ident),* $(,)?) => {
    $(
      impl From<$t> for TraceValue {
        fn from(v: $t) -> Self {
          Self::$variant(v as _)
        }
      }
    )*
  };
}

// SetTraceValue 整型基模板 (trace-event.h:431-439): 无符号 → UINT, 有符号 → INT
impl_trace_value_from!(
  u8 => Uint, u16 => Uint, u32 => Uint, u64 => Uint, usize => Uint,
  i8 => Int, i16 => Int, i32 => Int, i64 => Int, isize => Int,
  bool => Bool, f64 => Double,
);

impl From<&'static str> for TraceValue {
  fn from(v: &'static str) -> Self {
    Self::Str(v)
  }
}

impl From<String> for TraceValue {
  fn from(v: String) -> Self {
    Self::CopyStr(v)
  }
}

impl From<Box<dyn ConvertableToTraceFormat>> for TraceValue {
  fn from(v: Box<dyn ConvertableToTraceFormat>) -> Self {
    Self::Convertable(v)
  }
}

/// `TraceStringWithCopy` (trace-event.h:45-50): 标记 const char* 需要拷贝
/// (`TRACE_STR_COPY`, trace-event.h:50)。
pub struct TraceStringWithCopy(pub &'static str);

impl From<TraceStringWithCopy> for TraceValue {
  fn from(v: TraceStringWithCopy) -> Self {
    Self::CopyStr(String::from(v.0))
  }
}

// ---- TraceID (trace-event.h:320-377) ----

/// `v8::internal::tracing::TraceID` (trace-event.h:322): 事件 id 的 64 位归一 +
/// 可选 scope 字符串。10 个 C++ 构造重载塌缩为 `From` 整型族 (有符号补码扩展)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceId {
  /// scope_ (trace-event.h:375): nullptr = kGlobalScope。
  pub scope: Option<&'static str>,
  /// raw_id() (trace-event.h:371)。
  pub raw_id: u64,
}

impl TraceId {
  /// `TRACE_ID_WITH_SCOPE(scope, id)` (trace-event.h:54):
  /// `TraceID::WithScope` 构造 (trace-event.h:326-335)。
  pub fn with_scope(scope: &'static str, id: impl Into<Self>) -> Self {
    Self {
      scope: Some(scope),
      raw_id: id.into().raw_id,
    }
  }
}

macro_rules! impl_trace_id_from {
  ($($t:ty),* $(,)?) => {
    $(
      impl From<$t> for TraceId {
        fn from(v: $t) -> Self {
          // static_cast<uint64_t>: 有符号补码扩展, 无符号零扩展 (trace-event.h:336-362)
          Self { scope: None, raw_id: v as u64 }
        }
      }
    )*
  };
}

impl_trace_id_from!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

// ---- 事件与实参容器 ----

/// 原版平面数组 (arg_names[i], arg_types[i], arg_values[i]) 的类型化等价物。
#[derive(Debug)]
pub struct TraceEventArg {
  /// arg_names[i]。
  pub name: &'static str,
  /// 类型打标后的值。
  pub value: TraceValue,
}

impl TraceEventArg {
  /// (name, SetTraceValue(val)) 对。
  pub fn new(name: &'static str, value: impl Into<TraceValue>) -> Self {
    Self {
      name,
      value: value.into(),
    }
  }
}

/// `TracingController::AddTraceEvent` 的平面参数包 (v8-platform.h:402-418)
/// 收敛为单一事件结构; 字段与原版形参一一对应。
#[derive(Debug)]
pub struct TraceEvent {
  /// char phase。
  pub phase: u8,
  /// category_group (长生命周期字面量)。
  pub category_group: &'static str,
  /// name。
  pub name: &'static str,
  /// scope (kGlobalScope = None)。
  pub scope: Option<&'static str>,
  /// id (kNoId = 0)。
  pub id: u64,
  /// bind_id (kNoId = 0)。
  pub bind_id: u64,
  /// flags。
  pub flags: u32,
  /// (arg_names, arg_types, arg_values) 平面数组的类型化打包。
  pub args: Vec<TraceEventArg>,
}

/// `AddTraceEventImpl` (trace-event.h:383-407): convertable 指针解包后
/// 转发 controller (Rust 侧 TraceValue 已类型化, 无需解包步)。
fn add_trace_event_impl(ev: TraceEvent) -> u64 {
  controller().add_trace_event(&ev)
}

/// `AddTraceEventWithTimestampImpl` (trace-event.h:409-429)。
fn add_trace_event_with_timestamp_impl(ev: TraceEvent, timestamp: i64) -> u64 {
  controller().add_trace_event_with_timestamp(&ev, timestamp)
}

// ---- AddTraceEvent 重载族 → emit 核 4 (trace-event.h:476-559) ----

/// `INTERNAL_TRACE_EVENT_ADD` (trace-event.h:184-196):
/// 类别 recording 检查 → `AddTraceEvent` 0/1/2 参重载塌缩。
pub fn emit(
  phase: u8,
  category_group: &'static str,
  name: &'static str,
  flags: u32,
  args: Vec<TraceEventArg>,
) {
  if !category_enabled(category_group) {
    return; // 早退: recording 模式未启用 (trace-event.h:187)
  }
  add_trace_event_impl(TraceEvent {
    phase,
    category_group,
    name,
    scope: K_GLOBAL_SCOPE,
    id: K_NO_ID,
    bind_id: K_NO_ID,
    flags,
    args,
  });
}

/// `INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP` (trace-event.h:248-261)。
pub fn emit_ts(
  phase: u8,
  category_group: &'static str,
  name: &'static str,
  timestamp: i64,
  flags: u32,
  args: Vec<TraceEventArg>,
) {
  if !category_enabled(category_group) {
    return;
  }
  add_trace_event_with_timestamp_impl(
    TraceEvent {
      phase,
      category_group,
      name,
      scope: K_GLOBAL_SCOPE,
      id: K_NO_ID,
      bind_id: K_NO_ID,
      flags,
      args,
    },
    timestamp,
  );
}

/// `INTERNAL_TRACE_EVENT_ADD_WITH_ID` (trace-event.h:213-231):
/// `flags |= TRACE_EVENT_FLAG_HAS_ID` + TraceID 归一。
pub fn emit_id(
  phase: u8,
  category_group: &'static str,
  name: &'static str,
  id: TraceId,
  flags: u32,
  args: Vec<TraceEventArg>,
) {
  if !category_enabled(category_group) {
    return;
  }
  add_trace_event_impl(TraceEvent {
    phase,
    category_group,
    name,
    scope: id.scope,
    id: id.raw_id,
    bind_id: K_NO_ID,
    flags: flags | TRACE_EVENT_FLAG_HAS_ID,
    args,
  });
}

/// `INTERNAL_TRACE_EVENT_ADD_WITH_ID_AND_TIMESTAMP` (trace-event.h:234-246);
/// `ADD_WITH_ID_TID_AND_TIMESTAMP` (trace-event.h:275) 重定向至此 (tid 丢弃)。
pub fn emit_id_ts(
  phase: u8,
  category_group: &'static str,
  name: &'static str,
  id: TraceId,
  timestamp: i64,
  flags: u32,
  args: Vec<TraceEventArg>,
) {
  if !category_enabled(category_group) {
    return;
  }
  add_trace_event_with_timestamp_impl(
    TraceEvent {
      phase,
      category_group,
      name,
      scope: id.scope,
      id: id.raw_id,
      bind_id: K_NO_ID,
      flags: flags | TRACE_EVENT_FLAG_HAS_ID,
      args,
    },
    timestamp,
  );
}

// ---- ScopedTracer (trace-event.h:561-596) ----

/// `INTERNAL_TRACE_EVENT_ADD_SCOPED` 的 COMPLETE 事件守卫:
/// Drop 时若启用字节仍非 0, 补 `UpdateTraceEventDuration` (trace-event.h:564-569)。
#[derive(Debug, Default)]
pub struct ScopedTracer {
  /// p_data_ == nullptr 表示未初始化 (类别未启用, trace-event.h:563)。
  data: Option<ScopedData>,
}

#[derive(Debug)]
struct ScopedData {
  category_group: &'static str,
  name: &'static str,
  event_handle: u64,
}

impl ScopedTracer {
  /// `ScopedTracer::Initialize` (trace-event.h:570-576)。
  fn initialize(&mut self, category_group: &'static str, name: &'static str, event_handle: u64) {
    self.data = Some(ScopedData {
      category_group,
      name,
      event_handle,
    });
  }
}

impl Drop for ScopedTracer {
  fn drop(&mut self) {
    let Some(d) = self.data.as_ref() else { return };
    let ctrl = controller();
    // Relaxed_Load 重读启用字节: 字节非 0 即补记 (trace-event.h:566-568)
    let flag = ctrl.get_category_group_enabled(d.category_group);
    if flag != 0 {
      ctrl.update_trace_event_duration(flag, d.name, d.event_handle);
    }
  }
}

/// `INTERNAL_TRACE_EVENT_ADD_SCOPED` (trace-event.h:198-211):
/// COMPLETE 相位 + FLAG_NONE + 守卫句柄。
pub fn scoped_add(
  category_group: &'static str,
  name: &'static str,
  args: Vec<TraceEventArg>,
) -> ScopedTracer {
  let mut tracer = ScopedTracer::default();
  if category_enabled(category_group) {
    let handle = add_trace_event_impl(TraceEvent {
      phase: TRACE_EVENT_PHASE_COMPLETE,
      category_group,
      name,
      scope: K_GLOBAL_SCOPE,
      id: K_NO_ID,
      bind_id: K_NO_ID,
      flags: TRACE_EVENT_FLAG_NONE,
      args,
    });
    tracer.initialize(category_group, name, handle);
  }
  tracer
}

/// `INTERNAL_TRACE_EVENT_ADD_SCOPED_WITH_FLOW` (trace-event.h:214-231):
/// bind_id 归一传递 + flow flags 原样。
pub fn scoped_add_with_flow(
  category_group: &'static str,
  name: &'static str,
  bind_id: TraceId,
  flow_flags: u32,
  args: Vec<TraceEventArg>,
) -> ScopedTracer {
  let mut tracer = ScopedTracer::default();
  if category_enabled(category_group) {
    let handle = add_trace_event_impl(TraceEvent {
      phase: TRACE_EVENT_PHASE_COMPLETE,
      category_group,
      name,
      scope: K_GLOBAL_SCOPE,
      id: K_NO_ID,
      bind_id: bind_id.raw_id,
      flags: flow_flags,
      args,
    });
    tracer.initialize(category_group, name, handle);
  }
  tracer
}
