[English](#en) | [中文](#zh)

---

<a name="en"></a>

#

---

<a name="zh"></a>

# v8-logging 中文说明

`v8/src/logging` 的 Rust 移植。日志门面统一走 `log` crate(红线: 禁自造第二套日志门面)。

- [核心门面(log.h / log.cc / log-inl.h / local-logger.h/.cc / metrics.h/.cc)](#核心门面logh-logcc-log-inlh-local-loggerhcc-metricshcc)
  - [与 `log` crate 的映射(禁自造门面)](#与-log-crate-的映射禁自造门面)
  - [flags_lite flag 登记](#flags_lite-flag-登记)
  - [推迟登记](#推迟登记)
  - [测试覆盖映射(7.3)](#测试覆盖映射73)
- [counters 组](#counters-组)

## 核心门面(log.h / log.cc / log-inl.h / local-logger.h/.cc / metrics.h/.cc)

对应关系:

| Rust 模块 | 原版 | 范围 |
|---|---|---|
| `src/log.rs` | `log.cc`(函数/脚本事件簇)+ `globals.h:ScriptEventType` | `ScriptEvent`/`FunctionEvent`/`CompilationCacheEvent`/`CodeNameEvent`/`MoveEventInternal`/`AppendFunctionMessage`, 纯格式化、解耦 Isolate/Handle |
| `src/log_inl.rs` | `log-inl.h` | `ToNativeByScript`(`Tagged<Script>` 解耦为 `is_native_script: bool`) |
| `src/local_logger.rs` | `local-logger.h/.cc` | `LocalLogger` 主线程谓词快照 + `ScriptEvent` 转发 |
| `src/metrics.rs` | `metrics.h/.cc` | `Recorder` 签名骨架(见推迟) |
| `src/lib.rs` | — | 导出面摊平; 导入名对齐各模块实际符号: `code_events::Event`→实名 `CodeEvent`, `log_file::{close,new_message_builder,write_log_header}`→`LogFile` 关联方法不摊平, `events::string_event_unchecked`→实名 `unchecked_string_event` |

### 与 `log` crate 的映射(禁自造门面)

- 现源码树中 `log.h` 的 `LOG(isolate, Call)` 宏(机械展开为 `if (v8_flags.log) isolate->v8_file_logger()->Call`, log.h:78-81)是 **V8FileLogger 文件日志事件**派发, 不是 severity 日志; severity 家族(`LOG(WARNING)`/`LOG_ERROR`/`DFATAL`)在 `base/logging.h`(v8-base 范围)。
- Rust 侧 severity 诊断一律使用 `log` crate: V8 `LOG(ERROR)`/`LOG(FATAL)` 语义 → `log::error!`; `LOG(WARNING)` → `log::warn!`; `LOG_LOCAL`/`OSLOG` → `log::debug!`; 引擎侧信息性输出 → `log::info!/debug!/trace!`。
- 老版 `logutils` 的 `LogMessageType`/`LogMessage::Delegate(Print/Format/Ignore)`/`WMS`/`V8_WARN`/`V8_PUSH_WARN`/`V8_POP_WARN`/`StandardDelegate`/`DelegateForTest`/`IsLoggingDebugging`/`IsLoggingMisc`/`IsLoggingNativeCode` 在现存源码树**均不存在**(全树 grep 仅 riscv 模拟器无关命中), 系 2010 年代 `src/logutils.h` 已删除设施——**不虚构转写**, 消息前缀 `code-`/`bignum-` 同理(现存前缀即 `code-creation`/`code-move`/`script`/`function` 等, 已由 events.rs/log.rs 逐条对齐)。
- V8FileLogger 的 stderr/console 输出经 `log_file.rs` `OutputHandle::Stdout`(对应 `logfile = "-"`), 不经 `log` crate, 保持 v8.log 字节格式。

### flags_lite flag 登记

flag 类型/默认值 1:1 取自 `v8/src/flags/flag-definitions.h`; 消费点接线读取, 不再以 bool 参数下传门控。

| flag | 默认值 | 定义行 | 消费点 |
|---|---|---|---|
| `log_function_events` | false | L4056 | `script_event`/`function_event`/`compilation_cache_event` 门控 |
| `log_code` | false | L4048 | `move_event_internal` 门控 |
| `predictable` | false | L4246 | `append_function_message` 的 `0.1` 分支 |
| `log_source_code` | false | L4046 | `EnsureLogScriptSource`(推迟未转写), 备查 |
| `logfile_per_isolate` | **true** | L4036 | 日志文件名按 isolate 拆分(V8FileLogger, 推迟), 备查 |
| `log_maps` | false | L3355 | 自查补入: v8/src/logging 引用 5 次(log.cc), MapCreate/MapDetails 推迟, 备查 |
| `log_source_position` | false | L4047 | 自查补入: log.cc:1415 引用, 源码位置记录推迟, 备查 |

另 7 个 flag (按 `v8/src/logging` 全部 `v8_flags.*` 引用点核验; 消费均属推迟组件, 备查):

| flag | 默认值 | 定义行 | 原版引用点(消费均属推迟组件, 备查) |
|---|---|---|---|
| `log_ic` | false | L3308 | log.cc:2041 `LOG_IC`(IC 状态转移, V8FileLogger 推迟) |
| `log_maps_details` | **true** | L3356 | log.cc:2147 `MapDetails`(推迟) |
| `log_feedback_vector` | false | L4052 | log.cc:1566 `LogFeedbackVector`(Isolate 绑定, 推迟) |
| `log_code_disassemble` | false | L4053 | log.cc:1490 `LogCodeDisassemble`(AbstractCode, 推迟) |
| `prof_include_idle` | false | L4063 | log.cc:1150/2012(sampler/profiler, 推迟) |
| `interpreted_frames_native_stack` | false | L4177 | log.cc:135 `ComputeMarker`(Tagged 堆对象, 推迟) |
| `logfile`(字符串) | "v8.log" | L4033 | log.cc:2396 `V8FileLogger::Initialize`(推迟); Rust 以 `OnceLock<RwLock<Box<str>>>` 持位 |

implication 机制不在本层复刻(同 `logfile_per_isolate` 决策, 已在 flags_lite
模块头登记): `log_ic→log_code`(L3309)/`log_maps→log_code`(L3357)/
`log_code_disassemble→log_code`(L4055)/`log_code→log_source_code`、
`log_code→log_source_position`(L4054-4056, WEAK)。

**与 v8-flags 的两 crate 关系**: 工程内 `v8-flags` crate 已落地
完整 flag 面(defs.rs X-macro 生成 + implications/contradictions 执行器),
flags_lite 是 v8-logging 内的**最小私有过渡层**(模块头【待回收】标记):上述
flag 已与 `v8-flags/src/defs.rs` 逐一比对, 名称/注释/默认值一致
(如 `log_maps_details` 默认 true, defs.rs:5576-5580)。**决策**: 暂不新增
v8-logging→v8-flags 依赖接线(flags_lite 已满足本 crate 门控读取; 跨 crate
依赖方向属总编排决策), 回收时机 = 编排允许 v8-logging 消费 v8-flags 时整体
删除本模块、门控改读 `v8_flags`。

- **`log_api` 不存在**: 现 `v8/src/flags/flag-definitions.h` 与 v8/src 全树均无此 flag
  (老 V8 `--log-api` 遗留名); 现 `LOG(isolate, Call)` 宏族门控为 `v8_flags.log`
  (log.h:78-81), flags_lite 已有 `log` 覆盖, 故不设。
- 平台特定 flag(`perf_*`/`ll_prof`/`enable_etw_*`/`gdbjit`/`logfile` 之外的
  路径类)属 JitLogger/ETW/perf 集成, 随该部分推迟, 不入 flags_lite。

### 推迟登记

- `metrics.rs`: 现存原版为 embedder 指标 `Recorder`(metrics.h), **无** `MetricsReader`(os_log/tracepoint, 早已移除); 泛型 `AddMainThreadEvent<T>` 事件族依赖 `include/v8-metrics.h`(L7), task runner/`V8::GetCurrentPlatform` 依赖 v8-init——骨架仅 `set/has/notify/disposal/delay/drain` 语义。
- `LocalLogger::ScriptDetails`/`CodeLinePosInfoRecordEvent`/`MapCreate`/`MapDetails`、`V8FileLogger` 全类(Isolate 绑定)、`log-inl.h` 的 `TimerEventScope`、`CodeEventLogger`/`ExternalLogEventListener`/`ExistingCodeLogger`/`CodeEvent` struct: L3+ 回收。
- `FunctionEvent` 的 `Tagged<String>` 重载与 `VMStateIfMainThread<LOGGING>`: 依赖对象层。

### 测试覆盖映射(7.3)

规约源 `v8/test/unittests/logging/log-unittest.cc`(原版不存在 log-test.cc/local-logger-test.cc/test-logging.cc; 亦无 local-logger 专项用例, `tests/local_logger.rs` 断言直接取自 local-logger.h/.cc 语义)。

| 原版用例 | Rust 用例 | 文件 |
|---|---|---|
| `TEST_F(LogTest, LogVersion)` | `log_test_log_version` | tests/log.rs |
| `TEST_F(LogFunctionEventsTest, LogFunctionEvents)` | `log_function_events_test_log_function_events`(行格式子集) | tests/log.rs |
| log.cc:1919-1940 switch | `log_test_script_event_names` | tests/log.rs |
| log.cc:1915 门控负例 | `log_test_script_event_gated_off` | tests/log.rs |
| log.cc:1835-1842 | `log_test_code_name_event` | tests/log.rs |
| log.cc:1844-1851 | `log_test_move_event_internal` | tests/log.rs |
| log.cc:1897-1912 | `log_test_compilation_cache_event` | tests/log.rs |
| `msg << double`(ostream `%g`) | `log_test_format_g_matches_ostream`(15 组黄金) | tests/log.rs |
| local-logger.cc:14-18 | `local_logger_snapshots_predicates` | tests/local_logger.rs |
| local-logger.cc:23-25 | `local_logger_script_event_forwards` | tests/local_logger.rs |
| log-file.cc 空 builder 快路径 | `local_logger_not_logging_skips` | tests/local_logger.rs |
| `TEST_F(AggregatedMemoryHistogramTest, One/Two/ManySamples*)` 全 14 例 | `aggregated_memory_histogram_test_*` | tests/counters.rs |
| `TEST_F(RuntimeCallStatsTest, RuntimeCallTimer)` | `runtime_call_stats_test_runtime_call_timer` | tests/runtime_call_stats.rs |
| `...RuntimeCallTimerSubTimer` | `runtime_call_stats_test_runtime_call_timer_sub_timer` | tests/runtime_call_stats.rs |
| `...RuntimeCallTimerRecursive` | `runtime_call_stats_test_runtime_call_timer_recursive` | tests/runtime_call_stats.rs |
| `...RuntimeCallTimerScope` | `runtime_call_stats_test_runtime_call_timer_scope` | tests/runtime_call_stats.rs |
| `...RuntimeCallTimerScopeRecursive` | `runtime_call_stats_test_runtime_call_timer_scope_recursive` | tests/runtime_call_stats.rs |
| `...RenameTimer` | `runtime_call_stats_test_rename_timer` | tests/runtime_call_stats.rs |
| `...BasicPrintAndSnapshot` | `runtime_call_stats_test_basic_print_and_snapshot` | tests/runtime_call_stats.rs |
| `...PrintAndSnapshot` | `runtime_call_stats_test_print_and_snapshot` | tests/runtime_call_stats.rs |
| `...NestedScopes` | `runtime_call_stats_test_nested_scopes` | tests/runtime_call_stats.rs |

跳过负登记: `Issue23768`/`LogCallbacks`/`LogAccessorCallbacks`/`LogAll`/`LogMaps*`/`ConsoleTimeEvents`/`BuiltinsNotLoggedAsLazyCompile`/`ExternalLogEventListener*`/`Issue539892`/序列化族 —— 均需真实 Isolate/Heap/JIT(`TestWithIsolate`+`RunJS`), 待引擎闭环(M4+)回收; 其中行格式断言已按上表落入纯门面用例。
补遗跳过负登记: `SnapshotNativeCounterTest.WriteBarrier`(counters-unittest.cc:214, 需 NativeContext+`V8_SNAPSHOT_NATIVE_CODE_COUNTERS`); `RuntimeCallStatsTest.{BasicJavaScript,FunctionLengthGetter,CallbackFunction,ApiGetter,GarbageCollection}`(runtime-call-stats-unittest.cc:403-643, 需 RunJS/API hook/Heap)。

---

开发日志 (flags_lite 补齐时间线、删除登记、round1/round2 对抗审查 finding 台账) 见 ../../log/v8-logging.md。

## counters 组

(由并发代理补充, 本代理不改)

