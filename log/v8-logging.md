# v8-logging 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### flags_lite 补齐时间线

- 首轮补入 7 个 flag (类型/默认值 1:1 取自 `v8/src/flags/flag-definitions.h`), 并接线消费 (不再以 bool 参数下传门控); flag 清单保留于 readme「flags_lite flag 登记」第一表。
- round1 全集核验 (grep `v8/src/logging` 全部 `v8_flags.*` 引用点) 二次补入 7 个; flag 清单保留于 readme「flags_lite flag 登记」第二表。
- lib.rs 导出面曾修复 3 处 E0432 (导入名对齐各模块实际符号: `code_events::Event`→无此符号 (实名 `CodeEvent`), `log_file::{close,new_message_builder,write_log_header}`→`LogFile` 关联方法不摊平, `events::string_event_unchecked`→实名 `unchecked_string_event`); 修复叙事自 readme 模块映射表 lib.rs 行迁出, 表行只留对齐结果。

### 删除登记 (clippy 零死代码处置, 附原版对照)

| 删除项 | 原版对照 | 理由 |
|---|---|---|
| `TimedHistogram::stop(&mut ElapsedTimer)` | counters.h:253(protected)/counters.cc:59-63 `TimedHistogram::Stop(base::ElapsedTimer*)` | 原版在现源码树**无任何调用方**(全树 grep 无命中; `BaseTimedHistogramScope::StopInternal` 已内联同等逻辑), 系 V8 自身死代码, 不予转写 |
| `rcs_names.rs` 的 `K_SECTION_OFFSET/LEN_{GC,MANUAL,RUNTIME,BUILTIN,API}` 与 `K_SECTION_LEN_HANDLER`(11 个常量) | C++ `RuntimeCallCounterId` enum 段序(仅编译期符号布局) | 原版段边界无运行时消费者(C++ 仅 `kFirstThreadVariantCounter`/`kNumberOfCounters` 有使用); 段名表已由 `SECTION_ORDER` 数组承载, offset/len 属生成期冗余元数据; 保留有消费者的 `K_SECTION_OFFSET/LEN_THREAD` 与 `K_SECTION_OFFSET_HANDLER` |
| `time_base.rs` 的 `install_test_clock`/`uninstall_test_clock`/`now_ticks`/`TEST_TICK_BASE`(round2) | 原版无对应物(自造测试设施) | crate 内与 tests/ 均零消费的 pub 孤儿(rust_review 零死代码红线); 时间注入已有 `runtime_call_stats::set_now` 承载 |

接线(原 never-used → 有消费, 未删): `TimedHistogram::reset`/
`NestedTimedHistogram::reset`/`AggregatableHistogramTimer::reset` 由
`Counters::reset_create_histogram_function` 走各类型自身包装(对应 C++
`HistogramResetter` 经继承分派); `OptionalTimedHistogramScope` 补显式
`Drop`(对应 counters-scopes.h:103-107 析构 `Stop()`); `LogFile` 行缓冲
`write_raw_bytes` 统一经 `flush_line_buffer` 落盘; `WorkerThreadRuntimeCallStats::isolate_thread_id`
按原版注释语义(h:743-745 "防主线程误建 worker 表")接入 `new_table` 检查。

依赖清理: `phf` 移除 —— crate 内无静态查找表(rcs_names 为顺序数组,
C++ 原版亦无按名查 counter 的入口), 无真实消费点。

### round1 对抗审查 finding 台账 (全新会话逐行比对 v8/src/logging 全文件)

| finding | 原版引用 | Rust 引用 | 处置 |
|---|---|---|---|
| flags_lite 缺 7 个 flag(`log_ic`/`log_maps_details`/`log_feedback_vector`/`log_code_disassemble`/`prof_include_idle`/`interpreted_frames_native_stack`/`logfile`) | flag-definitions.h:3308/3356/4052/4053/4063/4177/4033; 引用点 log.cc:2041/2147/1566/1490/1150+2012/135/2396 | src/flags_lite.rs | 已补定义+默认值+`reset_defaults`+lib.rs 导出; 消费点均属推迟组件, 备查持位 |
| `get_code_event_type_for_tag` doc 声称 "kLength 返回 Unknown 并记录 debug 日志", 实现与原版一致映射 `kBuiltinType`, 文档失真 | log.cc:94-125(`kLength`+`kBuiltin` 同映射) | src/code_events.rs | 已修 doc, 实现不动(本就 1:1) |
| `write_raw_bytes` 逐字符 `write_all`+`flush`(一次日志行数百次 syscall), 缺 C++ `OFStream`(stdio 缓冲)语义 | log-file.cc:245-251(`os_ <<` 缓冲写, `std::endl` 行尾 flush) | src/log_file.rs | 加行缓冲 `line_buffer`: 行尾 `WriteToLogFile` 一次性落盘, `Close`/`Drop` 对应 `fflush` 兜底; 超长行(≥8×kMessageBufferSize)分块防无界内存 |
| tests 未覆盖 `counters-unittest.cc`/`runtime-call-stats-unittest.cc` | counters-unittest.cc:77-213; runtime-call-stats-unittest.cc:141-402 | tests/counters.rs + tests/runtime_call_stats.rs | 已迁 14+9 用例(见覆盖映射表), 跳过项负登记 |
| crate 根杂散空文件 `v8-logging/lib.rs`(0 字节, 历史遗留) | — | v8-logging/lib.rs | 已删除 |
| `RuntimeCallStats::dump_entries` 自行前置 `Snapshot()`(round2) | runtime-call-stats.cc:284-289 `Dump` **无** Snapshot(提交运行中计时是 `Print` 的行为, cc:255-264) | src/runtime_call_stats.rs | 已删该前置, 对齐原版(注释同步标注) |

### round2 对抗审查 finding 台账 (全新会话, X-macro 程序化全量比对 + 语义逐行)

| finding | 原版引用 | Rust 引用 | 处置 |
|---|---|---|---|
| 日志文件打开失败未降级 stdout(Err 传播), 原版 FOpen/OpenTemporaryFile 失败 → `output_handle_=nullptr` → `os_=stdout` + 不写 header, 消息继续(stdout) | log-file.cc:32-43/59/61; log.cc:2421(`v8_flags.log` → `UpdateIsLogging(true)`) | src/log_file.rs(`create_output_handle`/`flush_line_buffer`) | 已修: 打开失败 → `Ok(None)` 同态降级; 行缓冲刷出时 `Some(Stdout)` 与 `None` 均落 stdout; header 条件不变(`is_some`); 回归用例 `log_test_open_failure_degrades_to_stdout` |
| tests/counters.rs 头注"15 用例"笔误(实迁 14 例) | counters-unittest.cc:77-213(One 4 + Two 8 + Many 2 = 14) | tests/counters.rs | 已修 doc |

X-macro 程序化全量比对结果(round2, 脚本逐项 diff): RCS 五清单 GC 166/166、MANUAL 101/101、API 187/187、HANDLER 57/57、THREAD 228/228(ADD_THREAD_SPECIFIC_COUNTER 114 对含跨行续体)逐项一致; counters 九清单 HR 49/49、HR_SLOW 4/4、HT 24/24、HT_SLOW 1/1、TIMED 67/67、AGG 1/1、HP 1/1、HM 2/2、SC 46/46 逐项一致; 段常量 `K_SECTION_OFFSET_HANDLER=454`/`K_SECTION_OFFSET_THREAD=511`/`K_TEST_COUNTER1..3=261..263`/`K_JS_EXECUTION=226` 全过; 桌面默认常量 `KGarbageCollectionReasonMaxValue=30`(v8-internal.h:1598)/`kMaxExternalPointers=512MB/8`/`kMaxTrustedPointers=64MB/8`(v8-internal.h:934)/`kMaxCppHeapPointers=512MB/8`/`kMaxJSDispatchEntries=16MB`(globals.h:640)全对齐。flags_lite 24 个 flag 默认值逐个对照定义行全过(`prof_sampling_interval` 非安卓=1000 L4077-4079; `log_deopt`=DEFINE_DEVELOPER_FLAG→false L359-361/3162; `histogram_interval`=600000 L3275, C++ int/Rust f64 值与浮点消费域一致)。测试断言强度: RCS 9 用例逐断言守恒(24+24+16+4+6+12+12+30+6), AggregatedMemory 14 用例逐值守恒。

### round2 对抗审查 finding 台账 (第三独立会话, 即 todo 看板 round2)

| finding | 原版引用 | Rust 引用 | 处置 |
|---|---|---|---|
| 上一条"打开失败降级 stdout"修复不完整: `write_raw_bytes` 仍以 `output_handle.is_some()` 门控, 句柄缺失(log=true+打开失败)时消息被**静默丢弃**, `flush_line_buffer` 的 `None → stdout` 分支不可达, 与 C++ `os_` 绑定 stdout 后消息照写矛盾 | log-file.cc:55-62(构造 `os_(nullptr ? stdout : handle)`) + log-file.h:77-80 `operator<<` 直写 `os_` | src/log_file.rs `write_raw_bytes`(is_some 门控) | 已修: 去掉门控, 消息无条件入行缓冲, 输出分派全部由 `flush_line_buffer` 承担(File/TempFile→文件, Stdout/None→stdout); 回归用例增强(`string_event` 走降级路径不丢不 panic) |
| `open_temporary_file` 具名文件 `v8_log_test_env_var_{i}` 残留磁盘; 原版 `base::OS::OpenTemporaryFile()`(posix 即 `tmpfile()`)为匿名临时文件, 关闭自动删除(log-file.h:53-55 注释) | log-file.cc:38-39 → base/platform/platform-posix.cc `OpenTemporaryFile`/`tmpfile` | src/log_file.rs `open_temporary_file` | 已修: 创建成功后立即 `fs::remove_file`(POSIX fd 保持有效), 复刻 tmpfile 匿名语义 |
| `AggregatedMemoryHistogram::add_sample` 自造 `interval <= 0 → 钳 10` 分支; 原版不检查 interval 合法性, 病态配置由 `kMaxSamples=1000` 兜底 | counters.h:451-452(`double sample_interval_ms = v8_flags.histogram_interval;` 无钳制) | src/counters.rs | 已修: 删钳制, 1:1 对齐(kMaxSamples 循环保护已在) |
| `StatsCounter::get()` 内存序 Relaxed; C++ `GetPtr()->load()` 无序参, 默认 seq_cst | counters.h:105 | src/counters.rs | 已修: 改 `SeqCst` |
| `Counters::new` doc 声称"顺序登记: stats-counters → histograms, 同 CountersVisitor::Start", 实际按字段声明序(histogram 先)且互不依赖, 注释失真 | counters.cc:190-193(Start 顺序, 无副作用依赖) | src/counters.rs | 已修 doc |

本轮独立复核: 15 模块逐文件比对 + flags_lite 24 flag 与 flag-definitions.h 逐行核对 + `format_g` 15 组黄金 + `ElapsedTimer` pause/resume 与 elapsed-timer.h 逐语句等价推导 + `NestedTimedHistogram` Mutex 栈与 C++ 裸指针链进出栈语义等价推导, 其余未重报。

审查结论(逐文件): log.cc/h、log-inl.h、log-file.cc/h、local-logger.h/.cc、code-events.h、tracing-flags.h/.cc、metrics.h/.cc、counters.h/.cc/definitions/scopes(五张 X-macro 清单含 GC 段 166 项递归展开逐项比对)、runtime-call-stats.h/.cc + rcs-scope 与五段名称表(GC 166/MANUAL 101/API 187/HANDLER 57/THREAD 228, `TestCounter1..3`=261..263/`JS_Execution`=226 核验)全部语义等价; 推迟项与既有校正(LogMessageType/WMS/V8_WARN/StandardDelegate 不存在、LocalLogger 谓词快照、metrics=Recorder)未重报。

### 附注

- readme/en.md 为空文件 (1 字节), 无英文日志可迁。
- readme 测试覆盖映射「跳过负登记」补遗原标注 (round1), 轮次标记随收敛去掉, 内容 (结构性延期用例台账) 留在 readme。
