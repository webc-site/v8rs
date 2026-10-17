#!/usr/bin/env bun
// tool/src/tracing/code-data-source.h.js
// 机械提取 v8/src/tracing/ 缺口族: code-data-source.{h,cc} + code-trace-context.h +
// perfetto-logger.{h,cc} + perfetto-sdk.h → 数据源/日志监听签名骨架与 SDK 分支矩阵

import { readV8, writeSrc, banner } from "../../lib.js";

const cdsH = readV8("src/tracing/code-data-source.h");
const cdsCc = readV8("src/tracing/code-data-source.cc");
const ctxH = readV8("src/tracing/code-trace-context.h");
const logH = readV8("src/tracing/perfetto-logger.h");
const logCc = readV8("src/tracing/perfetto-logger.cc");
const sdkH = readV8("src/tracing/perfetto-sdk.h");

// 1. perfetto-sdk.h 分支矩阵 (#if V8_USE_PERFETTO / V8_USE_PERFETTO_SDK)
if (!/PERFETTO_ENABLE_LEGACY_TRACE_EVENTS 1/.test(sdkH)) {
  throw new Error("perfetto-sdk.h 未找到 LEGACY_TRACE_EVENTS 开关");
}
const sdkPerfettoHeaders = [
  "protozero/scattered_heap_buffer.h",
  "tracing/data_source.h",
  "tracing/track_event.h",
  "tracing/track_event_legacy.h",
  "builtin_clock.pbzero.h",
  "data_source_descriptor.gen.h",
  "v8_config.gen.h",
  "v8.pbzero.h",
  "interned_data.pbzero.h",
  "trace_packet.pbzero.h",
  "trace_packet_defaults.pbzero.h",
  "debug_annotation.pbzero.h",
];
for (const hd of sdkPerfettoHeaders) {
  if (!sdkH.includes(hd)) throw new Error(`perfetto-sdk.h 未引入 ${hd}`);
}

// 2. CodeTraceContext Intern 方法集 (code-trace-context.h)
const internFns = [...ctxH.matchAll(/uint64_t Intern(\w+)\(/g)].map((m) => m[1]);
if (internFns.join() !== "Isolate,JsScript,JsFunction,WasmScript") {
  throw new Error(`Intern 方法漂移: ${internFns.join()}`);
}
const setFns = [...ctxH.matchAll(/pbzero::V8(\w+)\*\s+set_v8_\w+\(\)/g)].map((m) => m[1]);
if (setFns.join() !== "JsCode,InternalCode,WasmCode,RegExpCode") {
  throw new Error(`set_v8_* 访问漂移: ${setFns.join()}`);
}
if (!ctxH.includes("set_v8_code_move()")) throw new Error("未找到 set_v8_code_move");

// 3. CodeDataSource 生命周期 (code-data-source.h)
for (const k of ["OnSetup", "OnStart", "OnStop", "Register()"]) {
  if (!cdsH.includes(k)) throw new Error(`code-data-source.h 未找到 ${k}`);
}

// 4. IncrementalState 判定位
for (const k of ["is_initialized", "log_script_sources", "log_instructions"]) {
  if (!cdsH.includes(k)) throw new Error(`IncrementalState 未找到 ${k}`);
}

// 5. PerfettoLogger 事件面 (perfetto-logger.h override 集, 机械提取)
const eventFns = [...logH.matchAll(/\n  (?:void|bool) (\w+)\(/g)].map((m) => m[1]);
const overrides = [
  "CallbackEvent",
  "GetterCallbackEvent",
  "SetterCallbackEvent",
  "SharedFunctionInfoMoveEvent",
  "NativeContextMoveEvent",
  "CodeMovingGCEvent",
  "WeakCodeClearEvent",
];
for (const o of overrides) {
  if (!eventFns.includes(o)) throw new Error(`perfetto-logger.h 未覆写 ${o}`);
}
const codeCreateCount = (logH.match(/void CodeCreateEvent\(/g) || []).length;
if (codeCreateCount !== 5) throw new Error(`CodeCreateEvent 重载应 5, 实得 ${codeCreateCount}`);

// 6. cc 侧存在性对数
for (const k of ["PerfettoLogger::CodeCreateEvent", "PerfettoLogger::CodeMoveEvent", "PerfettoLogger::CodeDeoptEvent"]) {
  if (!logCc.includes(k)) throw new Error(`perfetto-logger.cc 未实现 ${k}`);
}
if (!cdsCc.includes("CodeDataSource::Register")) throw new Error("code-data-source.cc 未实现 Register");

// —— 产物 1: perfetto SDK 分支矩阵 ——
let sdkRust = banner("src/tracing/code-data-source.h.js", "src/tracing/perfetto-sdk.h", "Perfetto SDK 接入分支矩阵");
sdkRust += `
/// Perfetto SDK 接入矩阵 (原版 perfetto-sdk.h 的 #if 展开)
pub mod perfetto_sdk {
  /// V8_USE_PERFETTO: 全量启用 (x64/arm64 默认构建)
  pub const USE_PERFETTO: bool = true;
  /// 前置宏: 沿用 legacy trace events 语义 (原版 PERFETTO_ENABLE_LEGACY_TRACE_EVENTS=1)
  pub const ENABLE_LEGACY_TRACE_EVENTS: u8 = 1;
  /// V8_USE_PERFETTO_SDK: 用整包 perfetto.h; 否则按头清单散引
  pub const USE_PERFETTO_SDK: bool = true;

  /// 散引模式头清单 (12 个, 对数用)
  pub const SPLIT_HEADERS: [&str; ${sdkPerfettoHeaders.length}] = [
${sdkPerfettoHeaders.map((h) => `    "perfetto/${h}",`).join("\n")}
  ];
}
`;
const pSdk = writeSrc("v8-tracing", "perfetto_sdk_generated.rs", sdkRust);

// —— 产物 2: 数据源 + 上下文骨架 ——
let cdsRust = banner(
  "src/tracing/code-data-source.h.js",
  "src/tracing/code-data-source.{h,cc} + code-trace-context.h",
  "代码追踪数据源/增量状态/上下文签名骨架",
);
cdsRust += `
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
  /// Intern 转发方法集 (对数: ${internFns.length})
  pub const INTERN_METHODS: [&str; ${internFns.length}] = [${internFns.map((f) => `"Intern${f}"`).join(", ")}];

  /// 取包内 V8 代码事件子消息 (对数: 4 代码 + 1 move)
  pub const SETTERS: [&str; 5] = ["v8_js_code", "v8_internal_code", "v8_wasm_code", "v8_reg_exp_code", "v8_code_move"];

  /// 析构语义: 有缓冲 interned 数据则冲刷 (原版 ~CodeTraceContext)
  pub fn flush_on_drop() {
    unimplemented!("骨架")
  }
}
`;
const pCds = writeSrc("v8-tracing", "code_data_source_generated.rs", cdsRust);

// —— 产物 3: PerfettoLogger 事件面骨架 ——
let logRust = banner("src/tracing/code-data-source.h.js", "src/tracing/perfetto-logger.{h,cc}", "PerfettoLogger 事件面签名骨架");
logRust += `
/// PerfettoLogger — LogEventListener 到 perfetto 包的转写层 (签名骨架)
pub trait PerfettoEventListenerSkel {
  /// 代码创建事件, ${codeCreateCount} 个重载 (AbstractCode ×4 + WasmCode ×1)
  fn code_create_event(&mut self, _tag: u8, _code: usize, _name: usize);
  /// 回调登记
  fn callback_event(&mut self, _name: usize, _entry_point: usize);
  fn getter_callback_event(&mut self, _name: usize, _entry_point: usize);
  fn setter_callback_event(&mut self, _name: usize, _entry_point: usize);
  /// 正则代码创建
  fn reg_exp_code_create_event(&mut self, _code: usize, _name: usize);
  /// 代码移动
  fn code_move_event(&mut self, _from: usize, _to: usize);
  fn bytecode_move_event(&mut self, _from: usize, _to: usize);
  fn shared_function_info_move_event(&mut self, _from: usize, _to: usize);
  fn native_context_move_event(&mut self, _from: usize, _to: usize);
  /// GC 触发的整体移动
  fn code_moving_gc_event(&mut self);
  /// 关优化
  fn code_disable_opt_event(&mut self, _code: usize, _sfi: usize);
  /// 反优化 (kind + pc + 原栈参数)
  fn code_deopt_event(&mut self, _code: usize, _kind: u8, _pc: usize, _fp_to_sp: usize);
  /// 依赖变更
  fn code_dependency_change_event(&mut self, _code: usize);
  /// 弱代码清理
  fn weak_code_clear_event(&mut self);
}

/// 非覆写辅助 (原版 perfetto-logger.h 顶部两个公有方法)
pub mod perfetto_logger_skel {
  /// 登记既有代码补发 (原版 LogExistingCode)
  pub fn log_existing_code() {
    unimplemented!("骨架")
  }
  /// 触发存量代码日志 (原版 TriggerExistingCodeLogging)
  pub fn trigger_existing_code_logging() {
    unimplemented!("骨架")
  }

  /// 事件方法总数对数守卫 (含重载)
  pub const EVENT_METHOD_COUNT: usize = ${eventFns.length + codeCreateCount - 1};
}
`;
const pLog = writeSrc("v8-tracing", "perfetto_logger_generated.rs", logRust);

console.log(
  `written ${pSdk}, ${pCds}, ${pLog}; SDK头=${sdkPerfettoHeaders.length}, Intern=${internFns.length}, 事件=${eventFns.length}, 重载=${codeCreateCount}`,
);
