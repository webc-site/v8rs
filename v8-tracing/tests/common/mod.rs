//! 测试公共件: 录制型 TracingController (事件/时长/观察者全捕获)。
//! nextest 进程级隔离保证全局安装点每测试独占。

use std::sync::{Arc, Mutex};

use v8_tracing::{
  ENABLED_FOR_EVENT_CALLBACK, ENABLED_FOR_RECORDING, TraceEvent, TraceEventArg, TraceStateObserver,
  TraceValue, TracingController, set_tracing_controller,
};

/// 录制 controller: 类别启用表 + 事件流 + 时长补记 + 观察者。
pub struct Recorder {
  /// 启用的类别组 (命中返回 RECORDING|CALLBACK 位)。
  pub enabled: Mutex<Vec<String>>,
  /// 已写入事件 (add_trace_event / with_timestamp 双路)。
  pub events: Mutex<Vec<TraceEvent>>,
  /// (name, category_enabled_flag, handle) 的 UpdateTraceEventDuration 调用序列。
  pub durations: Mutex<Vec<(String, u8, u64)>>,
  /// 已挂观察者。
  pub observers: Mutex<Vec<Arc<dyn TraceStateObserver>>>,
}

/// 深拷贝事件 (Convertable 不可克隆 → 序列化为 JSON 串占位)。
fn clone_event(ev: &TraceEvent) -> TraceEvent {
  let args = ev
    .args
    .iter()
    .map(|a| {
      let value = match &a.value {
        TraceValue::Bool(b) => TraceValue::Bool(*b),
        TraceValue::Uint(u) => TraceValue::Uint(*u),
        TraceValue::Int(i) => TraceValue::Int(*i),
        TraceValue::Double(d) => TraceValue::Double(*d),
        TraceValue::Pointer(p) => TraceValue::Pointer(*p),
        TraceValue::Str(s) => TraceValue::Str(s),
        TraceValue::CopyStr(s) => TraceValue::CopyStr(s.clone()),
        TraceValue::Convertable(c) => {
          let mut json = String::new();
          c.append_as_trace_format(&mut json);
          TraceValue::CopyStr(json)
        }
      };
      TraceEventArg {
        name: a.name,
        value,
      }
    })
    .collect();
  TraceEvent {
    phase: ev.phase,
    category_group: ev.category_group,
    name: ev.name,
    scope: ev.scope,
    id: ev.id,
    bind_id: ev.bind_id,
    flags: ev.flags,
    args,
  }
}

impl Recorder {
  /// 构造并安装为全局 controller (返回强类型句柄供断言)。
  pub fn install(enabled: &[&str]) -> Arc<Self> {
    let rec = Arc::new(Self {
      enabled: Mutex::new(enabled.iter().map(|s| s.to_string()).collect()),
      events: Mutex::new(Vec::new()),
      durations: Mutex::new(Vec::new()),
      observers: Mutex::new(Vec::new()),
    });
    set_tracing_controller(Arc::clone(&rec) as Arc<dyn TracingController>)
      .expect("全局 controller 安装");
    rec
  }
}

impl TracingController for Recorder {
  fn get_category_group_enabled(&self, name: &str) -> u8 {
    if self
      .enabled
      .lock()
      .map(|g| g.iter().any(|c| c == name))
      .unwrap_or(false)
    {
      ENABLED_FOR_RECORDING | ENABLED_FOR_EVENT_CALLBACK
    } else {
      0
    }
  }

  fn add_trace_event(&self, ev: &TraceEvent) -> u64 {
    let mut evs = self.events.lock().unwrap();
    let handle = evs.len() as u64 + 1;
    evs.push(clone_event(ev));
    handle
  }

  fn add_trace_event_with_timestamp(&self, ev: &TraceEvent, _ts: i64) -> u64 {
    self.add_trace_event(ev)
  }

  fn update_trace_event_duration(&self, flag: u8, name: &str, handle: u64) {
    self
      .durations
      .lock()
      .unwrap()
      .push((name.to_string(), flag, handle));
  }

  fn add_trace_state_observer(&self, observer: Arc<dyn TraceStateObserver>) {
    self.observers.lock().unwrap().push(observer);
  }

  fn remove_trace_state_observer(&self, observer: &Arc<dyn TraceStateObserver>) {
    self
      .observers
      .lock()
      .unwrap()
      .retain(|o| !Arc::ptr_eq(o, observer));
  }
}
