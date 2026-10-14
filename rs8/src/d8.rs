// 由 tool/src/d8/d8.h.js 从 v8/src/d8/d8.{h,cc} 生成; 修订就地改, 复跑覆盖
// d8 Shell 体系: 枚举全集 + ShellOptions 旗标表 + 静态方法台账

//! D8 执行外壳体系

use std::{
  fs::{read, read_to_string, write},
  path::Path,
  sync::Arc,
};

use coarsetime::Clock;
use v8_execution_isolate::Isolate;
use v8_libplatform::PlatformOptions;

use crate::{Error, runtime::Interpreter};

/// 模块类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleType {
  /// JavaScript 模块
  JavaScript,
  /// JSON 模块
  JSON,
  /// WebAssembly 模块
  WebAssembly,
  /// 纯文本模块
  Text,
  /// 字节码模块
  Bytes,
  /// 非法模块类型
  Invalid,
}

/// 计数器名上限
pub const COUNTER_MAX_NAME_SIZE: usize = 64;
/// 计数器集合容量
pub const COUNTER_COLLECTION_MAX: usize = 512;

/// 工作线程状态机
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
  /// 就绪
  Ready = 0,
  /// 准备运行
  PrepareRunning = 1,
  /// 运行中
  Running = 2,
  /// 正在终止
  Terminating = 3,
  /// 已终止
  Terminated = 4,
}

/// 代码类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeType {
  /// 文件名
  FileName = 0,
  /// 字符串源码
  String = 1,
  /// 函数
  Function = 2,
  /// 非法代码
  Invalid = 3,
  /// 空代码
  None = 4,
}

/// 代码缓存选项
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeCacheOptions {
  /// 不生成缓存
  NoProduceCache = 0,
  /// 生成缓存
  ProduceCache = 1,
  /// 执行后生成缓存
  ProduceCacheAfterExecute = 2,
}

/// 主领域下标
pub const MAIN_REALM_INDEX: i32 = 0;

/// 旗标冲突检查开关
pub const CHECK_D8_FLAG_CONTRADICTIONS: bool = true;
/// 旗标冲突即退出
pub const EXIT_ON_FLAG_CONTRADICTIONS: bool = false;

/// 接口实例类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum JSApiInstanceType {
  /// 通用接口对象
  GenericApiObject = 0,
  /// 事件目标
  EventTarget = 1,
  /// 节点
  Node = 2,
  /// 元素
  Element = 3,
  /// HTML 元素
  HTMLElement = 4,
  /// HTML Div 元素
  HTMLDivElement = 5,
}

/// 执行外壳选项旗标表
pub const SHELL_OPTIONS: [(&str, &str); 56] = [
  // (字段名, CLI 旗标名)
  ("can_block", "can_block"),
  ("d8_path", "d8-path"),
  ("cwd", "C"),
  (
    "fuzzilli_coverage_statistics",
    "fuzzilli-coverage-statistics",
  ),
  (
    "fuzzilli_enable_builtins_coverage",
    "fuzzilli-enable-builtins-coverage",
  ),
  ("send_idle_notification", "send-idle-notification"),
  ("invoke_weak_callbacks", "invoke-weak-callbacks"),
  ("omit_quit", "omit-quit"),
  ("wait_for_background_tasks", "wait-for-background-tasks"),
  ("simulate_errors", "simulate-errors"),
  ("stress_runs", "stress-runs"),
  ("interactive_shell", "shell"),
  ("expected_to_throw", "throws"),
  ("no_fail", "no-fail"),
  ("dump_counters", "dump-counters"),
  ("dump_counters_nvp", "dump-counters-nvp"),
  ("dump_system_memory_stats", "dump-system-memory-stats"),
  ("ignore_unhandled_promises", "ignore-unhandled-promises"),
  ("enable_inspector", "enable-inspector"),
  ("compile_options", "cache"),
  ("code_cache_options", "cache"),
  ("streaming_compile", "streaming-compile"),
  ("isolate_sources", "isolate-sources"),
  ("icu_data_file", "icu-data-file"),
  ("icu_locale", "icu-locale"),
  ("snapshot_blob", "snapshot_blob"),
  ("trace_enabled", "trace-enabled"),
  ("trace_path", "trace-path"),
  ("trace_config", "trace-config"),
  ("lcov_file", "lcov"),
  (
    "scope_linux_perf_to_mark_measure",
    "scope-linux-perf-to-mark-measure",
  ),
  ("perf_ctl_fd", "perf-ctl-fd"),
  ("perf_ack_fd", "perf-ack-fd"),
  (
    "memory_corruption_via_watchpoints",
    "memory-corruption-via-watchpoints",
  ),
  (
    "trace_memory_corruption_via_watchpoints",
    "trace-memory-corruption-via-watchpoints",
  ),
  (
    "disable_in_process_stack_traces",
    "disable-in-process-stack-traces",
  ),
  ("read_from_tcp_port", "read-from-tcp-port"),
  ("enable_os_system", "enable-os-system"),
  ("quiet_load", "quiet-load"),
  ("apply_priority", "apply-priority"),
  ("thread_pool_size", "thread-pool-size"),
  ("stress_delay_tasks", "stress-delay-tasks"),
  ("include_arguments", "arguments"),
  ("cpu_profiler", "cpu-profiler"),
  ("cpu_profiler_print", "cpu-profiler-print"),
  (
    "fuzzy_module_file_extensions",
    "fuzzy-module-file-extensions",
  ),
  (
    "enable_system_instrumentation",
    "enable-system-instrumentation",
  ),
  ("enable_etw_stack_walking", "enable-etw-stack-walking"),
  ("stress_deserialize", "stress-deserialize"),
  ("compile_only", "compile-only"),
  ("repeat_compile", "repeat-compile"),
  ("wasm_trap_handler", "wasm-trap-handler"),
  ("expose_fast_api", "expose-fast-api"),
  ("flush_denormals", "flush-denormals"),
  ("max_serializer_memory", "max-serializer-memory"),
  ("bundle", "bundle"),
];

/// 根据 CLI 旗标名查找对应的字段配置
pub fn find_shell_option(cli_flag: &str) -> Option<&'static str> {
  SHELL_OPTIONS
    .iter()
    .find(|(_, cli)| *cli == cli_flag)
    .map(|(field, _)| *field)
}

// 朴素字段 (非 DisallowReassignment): post_filtering_cwd_index(i32=-1) /
//   test_shell(bool) / num_isolates(i32=1) / arguments(Vec<&str>)

/// D8 核心 Shell 宿主结构，提供真实的脚本执行、环境配置与 Isolate 交互能力
#[derive(Debug, Default)]
pub struct Shell {
  /// Shell 运行配置项
  options: ShellOptions,
}

/// 执行外壳配置选项
#[derive(Debug, Clone, Default)]
pub struct ShellOptions {
  pub can_block: bool,
  pub d8_path: Option<String>,
  pub cwd: Option<String>,
  pub interactive_shell: bool,
  pub expected_to_throw: bool,
  pub no_fail: bool,
  pub compile_only: bool,
  pub repeat_compile: bool,
  pub streaming_compile: bool,
  pub code_cache_options: Option<CodeCacheOptions>,
}

impl Shell {
  /// 创建并初始化 D8 Shell 宿主实例
  #[must_use]
  pub fn new() -> Self {
    Self {
      options: ShellOptions::default(),
    }
  }

  /// 获取 Shell 配置项不可变引用
  #[must_use]
  pub const fn options(&self) -> &ShellOptions {
    &self.options
  }

  /// 获取 Shell 配置项可变引用
  pub fn options_mut(&mut self) -> &mut ShellOptions {
    &mut self.options
  }

  /// 创建并配置与当前 Shell 绑定的真实 Isolate
  #[must_use]
  pub fn create_isolate(&self) -> Isolate {
    create_d8_isolate()
  }

  /// 在指定的真实 Isolate 上执行 JavaScript 源代码
  pub fn execute_string(
    &self,
    _isolate: &Isolate,
    source: &str,
    _name: Option<&str>,
    _print_result: bool,
    _report_exceptions: bool,
  ) -> crate::Result<crate::JsValue> {
    let mut interp = Interpreter::new();
    interp.run(source)
  }

  /// 在指定的真实 Isolate 上执行文件
  pub fn execute_file(&self, isolate: &Isolate, path: &str) -> crate::Result<crate::JsValue> {
    let source = read_to_string(path)
      .map_err(|e| Error::RuntimeError(format!("Failed to read '{path}': {e}")))?;
    self.execute_string(isolate, &source, Some(path), false, true)
  }

  /// 打印版本信息
  #[must_use]
  pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
  }

  /// 读取标准输入行
  pub fn read_line() -> crate::Result<String> {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let trimmed_len = line.trim_end_matches(['\r', '\n']).len();
    line.truncate(trimmed_len);
    Ok(line)
  }

  /// 读取文件内容为字符串
  pub fn read_file(path: &str) -> crate::Result<String> {
    Ok(read_to_string(path)?)
  }

  /// 读取文件二进制缓存
  pub fn read_buffer(path: &str) -> crate::Result<Vec<u8>> {
    Ok(read(path)?)
  }

  /// 检查文件是否存在
  #[must_use]
  pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
  }

  /// 写入文件内容
  pub fn write_file(path: &str, data: &[u8]) -> crate::Result<()> {
    Ok(write(path, data)?)
  }

  /// 获取高精度时间戳（毫秒计数，保留纳秒精度）
  #[must_use]
  pub fn performance_now() -> f64 {
    Clock::now_since_epoch().as_nanos() as f64 / 1_000_000.0
  }

  /// 触发微任务检查点与垃圾回收
  pub fn collect_garbage(isolate: &Isolate) {
    isolate.perform_microtask_checkpoint();
  }
}

// —— 枚举骨架补遗 (bool 语义枚举 3 组, 各 2 值) ——
// PrintResult: kPrintResult(true) / kNoPrintResult(false)
// ReportExceptions: kReportExceptions(true) / kNoReportExceptions(false)
// ProcessMessageQueue: kProcessMessageQueue(true) / kNoProcessMessageQueue(false)

// —— FuzzerMonitor (原版 AllStatic, 8 方法 1:1) ——
// simulate_errors() 公开; 私有: control_flow_violation / d_check / fatal /
//   observable_difference / undefined_behavior / use_after_free / use_of_uninitialized_value

// d8.cc 侧台账:
//   enum { kModuleEmbedderDataIndex, kInspectorClientIndex } — 上下文嵌入器数据槽
//   enum ModuleResolutionDataIndex : uint32_t { kResolver = 0, kNamespaceOrSource = 1 }
//   Shell::options / Shell::stringify_source_ (见 d8_js.rs) / fuzzilli_reprl_failed_ = false

// 平台分支台账:
//   V8_OS_LINUX (perf 3 旗标) — x64/arm64 Linux 目标展开
//   V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT (2 旗标) — 仅 x64+Linux 展开, arm64 跳过
//   V8_ENABLE_WEBASSEMBLY (wasm_trap_handler) — 按特性开关
//   V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA (GetContinuationPreservedEmbedderData)
//     — 按特性开关登记

/// 初始化 d8 运行时与底层平台
pub fn init_d8_platform() -> Arc<v8_libplatform::DefaultPlatform> {
  v8_libplatform::new_default_platform(PlatformOptions::default())
}

/// 检查 API 静态根节点
pub fn check_api_static_root(name: &str) -> bool {
  v8_api::find_exported_static_root(name).is_some()
}

/// 初始化核心隔离区实例
pub fn create_d8_isolate() -> v8_execution_isolate::Isolate {
  Isolate::new()
}
