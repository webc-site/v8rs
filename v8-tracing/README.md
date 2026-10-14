[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-tracing (English)

Rust port of `v8/src/tracing` (18 files, 3654 lines, plus the tracing surface of include/v8-platform.h). Logging goes through the `log` crate.

- [File → module map](#file-module-map)
- [X-macro expansion registry highlights](#x-macro-expansion-registry-highlights)
- [Deferred (perfetto SDK / L3 object layer)](#deferred-perfetto-sdk-l3-object-layer)
- [Tests (34, nextest green)](#tests-34-nextest-green)
- [Conversion scripts (ch. 11 registry)](#conversion-scripts-ch-11-registry)

## File → module map

| Rust module | Upstream | Lines | Scope |
|---|---|---|---|
| `src/trace_event_macros.rs` | `trace-event-no-perfetto.h` | 982 | X-macro expansion: 55 constants (54 upstream: phase 23 / flag 16 / value_type 9 / scope 6, plus 1 Rust-side prefix) + 123 macro functions (118 full + 5 typed no-ops) + 2 registered-not-expanded |
| `src/trace_event.rs` | `trace-event.h` + `trace-event.cc` | 672+63 | CategoryGroupEnabledFlags (3 bits) / kGlobalScope/kNoId / TraceId (10 ctor overloads → From) / TraceStringWithCopy / TraceValue (SetTraceValue typing) / emit×4 + scoped×2 core / ScopedTracer (Drop updates duration) / global controller install point (TraceEventHelper equivalent) |
| `src/platform.rs` | `include/v8-platform.h` (389-468) | ~80 | Embedder surface: `TracingController` trait (default no-op = upstream inline stubs) / `TraceStateObserver` / `ConvertableToTraceFormat` / `NoopController` |
| `src/traced_value.rs` | `traced-value.h` + `.cc` | 89+231 | Full `TracedValue` JSON builder: 7 escapes + `\uXXXX` uppercase hex / ECMA-262 9.8.1 shortest double (v8-base dtoa SHORTEST + conversions.cc:1178-1207 segmentation) / nesting stack |
| `src/trace_categories.rs` | `trace-categories.h` + `.cc` | 85+33 | PERFETTO_DEFINE_CATEGORIES X-macro expansion: 32 categories (27 single + 5 groups, disabled-by-default prefixes expanded in place) + 3 test prefixes; clock id (GetTraceClockId) is platform-dependent【deferred】, no constant |
| `src/category_observer.rs` | `tracing-category-observer.h` + `.cc` | 47+104 | `TracingCategoryObserver`: Mode 3 bits + 6 set / 4 clear mappings (zone_stats not cleared, upstream behavior), wired to v8-logging `TracingFlags` atomics |
| `src/trace_id.rs` | `trace-id.h` | 19 | `TraceId()` relaxed fetch_add monotonic sequence (complete) |
| `src/perfetto_utils.rs` | `perfetto-utils.h` + `.cc` | 72+33 | `PerfettoV8String` data plane: is_one_byte/size/buffer/eq (derive PartialEq)/hash (v8-base hash_range+combine)/3 encodings (Latin1/LE/BE); `Tagged<String>` readout deferred |
| (no code) | `perfetto-sdk.h` | 42 | Pure perfetto SDK include shell — registered only |
| (no code) | `code-trace-context.h` | 106 | Depends on perfetto protos — deferred |
| (no code) | `code-data-source.h` + `.cc` | 163+302 | Depends on perfetto DataSource + L3 (Isolate/Script/SharedFunctionInfo); contains `GetJsScriptType` (5 mappings) / `GetJsFunctionKind` (28 mappings) — tables land with the object layer |
| (no code) | `perfetto-logger.h` + `.cc` | 86+525 | Depends on code-events `LogEventListener` full interface + L3 — deferred |

## X-macro expansion registry highlights

- 118 expanded + 5 typed no-ops = 123 macro functions; `tool/src/tracing/trace-event-no-perfetto.h.js` is the expander, each fn doc cites the upstream macro and line
- Family folding: `ADD`→`emit` / `ADD_WITH_ID`→`emit_id` (auto `HAS_ID`) / `ADD_WITH_TIMESTAMP`→`emit_ts` / `ADD_WITH_ID_TID_AND_TIMESTAMP`→`emit_id_ts` (thread_id explicitly dropped upstream, trace-event.h:274-280) / `ADD_SCOPED`(+FLOW)→`scoped_add*` returning a `ScopedTracer` guard
- Fixed argument names preserved: counter `"value"` / async step `"step"` / clock sync `"__metadata"`, `"clock_sync"`, `"sync_id"`, `"issue_ts"` / object snapshot `"snapshot"`
- Counter values are `static_cast<int>` → Rust `i32` (upstream truncation)
- `TRACE_EVENT_COPY_NESTABLE_ASYNC_*_WITH_TTS2` keeps the literal `ASYNC_TTS|COPY` combo
- Registered-not-expanded (2): `TRACE_EVENT_METADATA1` (its `INTERNAL_TRACE_EVENT_METADATA_ADD` is undefined tree-wide) / `TRACE_EVENT_IS_NEW_TRACE` (`TRACE_EVENT_API_GET_NUM_TRACES_RECORDED` = `UNIMPLEMENTED()`, trace-event.h:87)
- 9 perfetto stub types are all empty-bodied; not generated (zero dead code), registered in the module footer
- Counting baseline: 180 `#define`s = 125 user macros (123 expanded + 2 registered) + 54 constants + 1 INTERNAL
- Macro argument evaluation timing【registered deviation】: upstream argument expressions sit inside the `if (category enabled)` block (trace-event.h:181-190) and are skipped when disabled; Rust parameters are evaluated exactly once at the call site (no multi-evaluation) — laziness would require closing over all 123 signatures, revisit with the L4 call surface
- `PerfettoV8String::hash`【registered deviation】: upstream `hash.Combine(is_one_byte_)` is a static member called through an instance with its result discarded (base/hashing.h:151), so only AddRange takes effect; this port keeps Combine per the code's intent — process-local bucketing only, no golden data bound
- `TracedValue::Default` mirrors the private upstream ctor: root container is always a dict (traced-value.cc:78-80)

## Deferred (perfetto SDK / L3 object layer)

- `CallStatsScopedTracer` (trace-event.cc:25-57) and the perfetto `TRACE_EVENT_CALL_STATS_SCOPED` branch (trace-event.h:631-669)
- perfetto four-piece set (perfetto-sdk / code-trace-context / code-data-source / perfetto-logger): protos + DataSource + Isolate/Script chain
- `TracingCategoryObserver` perfetto branch (TrackEventSessionObserver, tracing-category-observer.cc:20-27/40/85)
- `GetTraceClockId` clock id (trace-categories.cc:27): platform-dependent (linux/android/apple = kClockMonotonic, windows = kClockBootTime); no constant landed
- headless VirtualTime override point of `TRACE_TIME_NOW` (mentioned upstream, no definition body)

## Tests (34, nextest green)

Source of truth `v8/test/unittests/tracing/traced-value-unittest.cc` — all 7 cases migrated 1:1 with original gtest names, plus 17-value double-formatting golden set (conversions.cc segmentation) and self-authored coverage for the macro pipeline (8), observer lifecycle, category registry counts, perfetto-utils data plane, TracedValue default root dict. Tests rely on nextest process isolation (global controller install point).

## Conversion scripts (ch. 11 registry)

7 scripts under `tool/src/tracing/` mirroring upstream paths; skeleton-producing scripts overwrite in-place revisions on rerun — rerun only to regenerate baselines (`tool/lib.js` convention). `trace_event_macros.rs` needs `cargo fmt -p v8-tracing` after a rerun.


---

<a name="zh"></a>

# v8-tracing 中文说明

`v8/src/tracing` (18 文件 3654 行, 另 include/v8-platform.h 追踪面) 的 Rust 移植。日志门面统一走 `log` crate。

- [文件 → 模块映射](#文件-模块映射)
- [X-macro 展开登记要点](#x-macro-展开登记要点)
- [与原版的结构性等价承接](#与原版的结构性等价承接)
- [推迟登记 (依赖 perfetto SDK / L3 对象层)](#推迟登记-依赖-perfetto-sdk-l3-对象层)
- [测试覆盖映射 (7.3)](#测试覆盖映射-73)
- [转换脚本 (十一章登记)](#转换脚本-十一章登记)

## 文件 → 模块映射

| Rust 模块 | 原版 | 行数 | 范围 |
|---|---|---|---|
| `src/trace_event_macros.rs` | `trace-event-no-perfetto.h` | 982 | X-macro 展开: 55 常量 (上游 54: phase 23/flag 16/value_type 9/scope 6, 另 1 Rust 前缀常量) + 123 宏函数 (118 完整展开 + 5 typed no-op) + 2 项登记不展开 |
| `src/trace_event.rs` | `trace-event.h` + `trace-event.cc` | 672+63 | CategoryGroupEnabledFlags 3 位 / kGlobalScope/kNoId / TraceId (10 构造重载→From) / TraceStringWithCopy / TraceValue (SetTraceValue 类型打标) / emit×4+scoped×2 核 / ScopedTracer (Drop 补时长) / 全局 controller 安装点 (TraceEventHelper 等价) |
| `src/platform.rs` | `include/v8-platform.h` (389-468) | ~80 | embedder 面: `TracingController` trait (默认 no-op = 原版 inline 空实现) / `TraceStateObserver` / `ConvertableToTraceFormat` / `NoopController` |
| `src/traced_value.rs` | `traced-value.h` + `traced-value.cc` | 89+231 | `TracedValue` JSON 构造器全量: 7 转义 + `\uXXXX` 大写十六进制 / ECMA-262 9.8.1 双精度最短表示 (v8-base dtoa SHORTEST + conversions.cc:1178-1207 分段) / nesting 配对栈 |
| `src/trace_categories.rs` | `trace-categories.h` + `trace-categories.cc` | 85+33 | PERFETTO_DEFINE_CATEGORIES X-macro 展开: 32 类别 (27 单 + 5 组, disabled-by-default 前缀就地展开) + 3 测试前缀; 时钟 id (GetTraceClockId) 平台相关【推迟登记】, 不落常量 |
| `src/category_observer.rs` | `tracing-category-observer.h` + `.cc` | 47+104 | `TracingCategoryObserver`: Mode 3 位 + 6 条置位/4 条清除映射 (zone_stats 不清除, 原版行为), 位操作直挂 v8-logging `TracingFlags` 原子静态 |
| `src/trace_id.rs` | `trace-id.h` | 19 | `TraceId()` relaxed fetch_add 单调序号 (全量) |
| `src/perfetto_utils.rs` | `perfetto-utils.h` + `.cc` | 72+33 | `PerfettoV8String` 数据面: is_one_byte/size/buffer/eq(derive PartialEq)/hash(v8-base hash_range+combine)/编码 3 态 (Latin1/LE/BE); `Tagged<String>` 读出推迟 |
| (无代码) | `perfetto-sdk.h` | 42 | 纯 perfetto SDK include 壳, 无可转写语义 — 登记 |
| (无代码) | `code-trace-context.h` | 106 | 依赖 perfetto protos (TracePacketHandle/InternedData) — 推迟 |
| (无代码) | `code-data-source.h` + `.cc` | 163+302 | 依赖 perfetto DataSource + L3 (Isolate/Script/SharedFunctionInfo); 内含 `GetJsScriptType` 5 映射 / `GetJsFunctionKind` 28 映射 (FunctionKind→proto 枚举) — 表随对象层回收 |
| (无代码) | `perfetto-logger.h` + `.cc` | 86+525 | 依赖 code-events `LogEventListener` 全接口 + L3 (AbstractCode/SharedFunctionInfo) — 推迟 |

## X-macro 展开登记要点

- 展开 118 + typed no-op 5 = 123 宏函数, 脚本 `tool/src/tracing/trace-event-no-perfetto.h.js` 即宏展开器; 每函数 doc 注明原宏名与行号
- 族归并: `ADD`→`emit` / `ADD_WITH_ID`→`emit_id`(自动 `HAS_ID`) / `ADD_WITH_TIMESTAMP`→`emit_ts` / `ADD_WITH_ID_TID_AND_TIMESTAMP`→`emit_id_ts` (原版重定向显式丢弃 thread_id, trace-event.h:274-280) / `ADD_SCOPED`(+FLOW)→`scoped_add*` 返回 `ScopedTracer` 守卫
- 固定实参保留: counter 的 `"value"` / async step 的 `"step"` / clock sync 的 `"__metadata"`,`"clock_sync"`,`"sync_id"`,`"issue_ts"` / object snapshot 的 `"snapshot"`
- 计数器 value 一律 `static_cast<int>` → Rust `i32` (原版截断行为)
- WITH_TTS 双标志组合 `ASYNC_TTS|COPY` 逐字保留
- `TRACE_DISABLED_BY_DEFAULT(name)` → `trace_disabled_by_default()` 前缀拼接
- 登记不展开 2 项: `TRACE_EVENT_METADATA1` (其 `INTERNAL_TRACE_EVENT_METADATA_ADD` 全 checkout 无定义) / `TRACE_EVENT_IS_NEW_TRACE` (`TRACE_EVENT_API_GET_NUM_TRACES_RECORDED` = `UNIMPLEMENTED()`, trace-event.h:87)
- perfetto stub 9 类型 (EventContext/StaticString/DynamicString/Track/ThreadTrack/NamedTrack/CounterTrack/Flow/TerminatingFlow) 全空方法体, 不生成 (零死代码), 登记于宏模块尾注
- 对数基线: 原文件 180 个 `#define` = 125 用户宏 (123 展开 + 2 登记) + 54 常量 + 1 INTERNAL

## 与原版的结构性等价承接

- `TraceEventHelper::GetTracingController` 的 `V8::GetCurrentPlatform()` 链 (trace-event.cc:19) → 本 crate 进程级 `set_tracing_controller(Arc<dyn TracingController>)` 安装点 (OnceLock, 重复安装报 `Error::ControllerInstalled`, 未安装 no-op)
- `INTERNAL_TRACE_EVENT_GET_CATEGORY_INFO` 的静态原子缓存 (指针稳定前提) → 每次调用重查 controller 启用字节 (语义等价, 无 unsafe 指针)
- `SetTraceValue` (u8 标签, u64 位模式) 平面数组 → `TraceValue` 枚举 + `type_tag()` (标签 1:1); `AddTraceEventImpl` 的 convertable 指针解包在类型化承载下坍缩
- `TracedValue::Create()` 返回 `Box`, `AppendAsTraceFormat` 输出首尾 `{}`; 无参 `BeginArray/BeginDictionary` 重载 → `begin_nested_array/begin_nested_dictionary` (Rust 无重载); `Default` 与原版私有构造器同态 (根容器恒为字典, traced-value.cc:78-80)
- 宏实参求值时机【偏差登记】: 原版实参表达式位于 `if (category enabled)` 块内, 类别未启用时不求值 (trace-event.h:181-190); Rust 函数形参在调用点恒求值恰一次 (无多重求值), 未启用时多出形参求值与 `Vec<TraceEventArg>`/`TraceValue` 构造开销 — 惰性化需闭包化全部 123 签名, 待 L4 调用面实测后再评估
- v8-logging `tracing_flags.rs` 的 `ENABLED_BY_*` 标注【待回收】, 与本 crate `category_observer::ENABLED_BY_*` 数值一致, 由编排者统一回收
- `PerfettoV8String::hash`【偏差登记】: 上游 `hash.Combine(is_one_byte_)` 为 static 成员经实例调用且返回值被丢弃 (base/hashing.h:151), 实际仅 AddRange; 本实现按代码意图保留 Combine — 哈希仅进程内分桶用, 无黄金数据绑定

## 推迟登记 (依赖 perfetto SDK / L3 对象层)

- `CallStatsScopedTracer` (trace-event.cc:25-57): `Isolate::counters()->runtime_call_stats()` (v8-execution)
- `TRACE_EVENT_CALL_STATS_SCOPED` perfetto 分支 (trace-event.h:631-669): RuntimeCallStats Dump + EventContext
- perfetto 四件套 (perfetto-sdk/code-trace-context/code-data-source/perfetto-logger): protos + DataSource + Isolate/Script 全链
- `TracingCategoryObserver` perfetto 分支 (TrackEventSessionObserver, tracing-category-observer.cc:20-27/40/85)
- `GetTraceClockId` 时钟 id (trace-categories.cc:27): perfetto TrackEvent 平台相关时钟 (linux/android/apple = kClockMonotonic, windows = kClockBootTime), 上游无单值常量, 常量不落地
- `TRACE_TIME_NOW` 的 headless VirtualTime 覆盖点 (原版注释提及, 上游无定义体)

## 测试覆盖映射 (7.3)

规约源 `v8/test/unittests/tracing/traced-value-unittest.cc` (唯一现存测试文件; 无 trace-event/categories 专项单测)。

| 原版用例 | Rust 用例 | 文件 |
|---|---|---|
| `TEST_F(TracedValueTest, FlatDictionary)` | `traced_value_test_flat_dictionary` | tests/traced_value.rs |
| `TEST_F(TracedValueTest, NoDotPathExpansion)` | `traced_value_test_no_dot_path_expansion` | tests/traced_value.rs |
| `TEST_F(TracedValueTest, Hierarchy)` | `traced_value_test_hierarchy` | tests/traced_value.rs |
| `TEST_F(TracedValueTest, Nesting)` | `traced_value_test_nesting` | tests/traced_value.rs |
| `TEST_F(TracedValueTest, LongStrings)` | `traced_value_test_long_strings` | tests/traced_value.rs |
| `TEST_F(TracedValueTest, Escaping)` | `traced_value_test_escaping` | tests/traced_value.rs |
| `TEST_F(TracedValueTest, Utf8)` | `traced_value_test_utf8` | tests/traced_value.rs |
| conversions.cc:1147-1207 (DoubleToStringView 分段) | `traced_value_double_formatting_boundaries` (17 组黄金) | tests/traced_value.rs |

新面测试 (原版无 gtest, 按 X-macro 展开语义自拟, tests/trace_event.rs + tests/categories.rs):

- 宏展开管道 8 例: counter "value" 固定名 / instant scope 并 flags / WITH_ID 置位 / step 固定名 / clock_sync 固定名 / TTS 组合 / object+timestamp / scoped 守卫 Drop 补时长
- disabled 类别早退 ×2 / with_flow bind_id / convertable 装载 / TraceValue 8 标签 / TraceId 符号扩展 / category_enabled 查询 / NOOP 默认 / observer 生命周期+类别门控 / ticks 单调
- 类别表对数 (32=27+5, slow×21, desc, 组拼接) / 测试前缀 / perfetto_utils 数据面 / trace_id 单调 / Default 根字典

共 34 用例, nextest 全绿; 测试依赖 nextest 进程级隔离 (全局 controller 安装点每测试独占)。

## 转换脚本 (十一章登记)

| 脚本 | 产物 | 规则代码 |
|---|---|---|
| `tool/src/tracing/trace-event-no-perfetto.h.js` | `src/trace_event_macros.rs` | X-macro 展开 (123 函数 + 55 常量), 复跑幂等 |
| `tool/src/tracing/trace-event.h.js` | `src/trace_event.rs` | 3 位标志 + 常量 + 签名骨架 (语义体就地修订) |
| `tool/src/tracing/traced-value.h.js` | `src/traced_value.rs` | 7 转义表 + 方法清单骨架 |
| `tool/src/tracing/trace-categories.h.js` | `src/trace_categories.rs` | 32 类别 X-macro 展开 (含链式 SetDescription/SetTags) |
| `tool/src/tracing/tracing-category-observer.h.js` | `src/category_observer.rs` | Mode 3 位 + 6+4 启停映射表 |
| `tool/src/tracing/trace-id.h.js` | `src/trace_id.rs` | 全量 (19 行) |
| `tool/src/tracing/perfetto-utils.h.js` | `src/perfetto_utils.rs` | 数据面 + 编码 3 态 |

骨架类脚本复跑会覆盖就地修订 — 复跑仅限重生成基线 (tool/lib.js 约定); `trace_event_macros.rs` 复跑后需 `cargo fmt -p v8-tracing` 对齐格式。

