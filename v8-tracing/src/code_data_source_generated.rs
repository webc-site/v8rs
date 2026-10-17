// 由 tool/src/tracing/code-data-source.h.js 从 v8/src/tracing/code-data-source.{h,cc} + code-trace-context.h 生成; 修订就地改, 复跑覆盖
// 代码追踪数据源/增量状态/上下文签名骨架

/// CodeDataSource — perfetto 数据源 (签名骨架, 语义实现由 Developer 修订)
pub mod code_data_source_skel {
  /// 生命周期: 注册 (原版 Register)
  pub fn register() {
    unimplemented!("骨架")
  }
  /// 生命周期回调 (原版 OnSetup/OnStart/OnStop)
  pub fn on_setup(_args: usize) {
    unimplemented!("骨架")
  }
  pub fn on_start(_args: usize) {
    unimplemented!("骨架")
  }
  pub fn on_stop(_args: usize) {
    unimplemented!("骨架")
  }
}

/// 增量状态判定位 (原版 CodeDataSourceIncrementalState)
pub mod incremental_state_skel {
  /// 缓冲 interned 数据是否非空 (原版 has_buffered_interned_data)
  pub fn has_buffered_interned_data(_state: usize) -> bool {
    unimplemented!("骨架")
  }
  /// 冲刷缓冲 interned 数据进包 (原版 FlushInternedData)
  pub fn flush_interned_data(_state: usize, _packet: usize) {
    unimplemented!("骨架")
  }
  /// Intern 表: isolate / js script / js function / wasm script (原版 Intern* 四项)
  pub fn intern_isolate(_isolate_key: usize) -> u64 {
    unimplemented!("骨架")
  }
  pub fn intern_js_script(_isolate_key: usize, _script: usize) -> u64 {
    unimplemented!("骨架")
  }
  pub fn intern_js_function(_isolate_key: usize, _sfi: usize, _script_iid: u64, _line: i32, _col: i32) -> u64 {
    unimplemented!("骨架")
  }
  pub fn intern_wasm_script(_isolate_key: usize, _script_id: i32, _url: &str, _native_module: usize) -> u64 {
    unimplemented!("骨架")
  }
  /// 判定位三件套 (原版 is_initialized / log_script_sources / log_instructions)
  pub fn is_initialized(_state: usize) -> bool {
    unimplemented!("骨架")
  }
  pub fn log_script_sources(_state: usize) -> bool {
    unimplemented!("骨架")
  }
  pub fn log_instructions(_state: usize) -> bool {
    unimplemented!("骨架")
  }
}

/// JsFunction 去重键 (原版 IncrementalState::Function, 三元组等值 + Hasher::Combine)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionKey {
  pub v8_js_script_iid: u64,
  pub is_toplevel: bool,
  pub start_position: i32,
}

/// Script 去重键 (原版 IncrementalState::ScriptUniqueId)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScriptUniqueId {
  pub isolate_id: i32,
  pub script_id: i32,
}

/// CodeTraceContext — 包级辅助 (原版 code-trace-context.h)
pub mod code_trace_context_skel {
  /// Intern 转发方法集 (对数: 4)
  pub const INTERN_METHODS: [&str; 4] = ["InternIsolate", "InternJsScript", "InternJsFunction", "InternWasmScript"];

  /// 取包内 V8 代码事件子消息 (对数: 4 代码 + 1 move)
  pub const SETTERS: [&str; 5] = ["v8_js_code", "v8_internal_code", "v8_wasm_code", "v8_reg_exp_code", "v8_code_move"];

  /// 析构语义: 有缓冲 interned 数据则冲刷 (原版 ~CodeTraceContext)
  pub fn flush_on_drop() {
    unimplemented!("骨架")
  }
}
