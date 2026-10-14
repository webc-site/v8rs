[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-libplatform

Rust transcription of `v8/src/libplatform` (~3k C++ lines). Scheduling: three seats —
LP-A platform/task-queue family, LP-B job/worker family, LP-C `tracing/` family.

- [LP-A implemented surface](#lp-a-implemented-surface)
- [Deviations & deferred registrations (LP-A)](#deviations-deferred-registrations-lp-a)

## LP-A implemented surface

Implemented 1:1 against the current upstream sources in this repo:

| Original | Rust module | Notes |
| --- | --- | --- |
| `task-queue.{h,cc}` | `src/task_queue.rs` | FIFO + `base::Semaphore`; terminate sentinel re-signals to broadcast |
| `delayed-task-queue.{h,cc}` | `src/delayed_task_queue.rs` | immediate FIFO + deadline-ordered multimap semantics (min-heap with post-seq tie-break); not thread-safe by design |
| `default-foreground-task-runner.{h,cc}` | `src/foreground_task_runner.rs` | best-effort order, `RunTaskScope` nesting depth, NonNestable pop rules, expired-delayed move, condvar wait |
| `default-platform.{h,cc}` | `src/platform.rs` + `src/api.rs` | lifecycle, `pump_message_loop`, `run_idle_tasks`, foreground runner map, time-function injection, notify-isolate-shutdown |
| `include/v8-platform.h` (consumer slice) | `src/api.rs` | `TaskRunner`/`Platform`/`TracingController` traits, enums, `SourceLocation` |

Tests migrated from `v8/test/unittests/libplatform/`: `task-queue-unittest.cc` →
`tests/task_queue_unittest.rs`; `default-platform-unittest.cc` →
`tests/default_platform_unittest.rs` (foreground suite; background-thread case `#[ignore]`).
`DelayedTaskQueue`/`DefaultForegroundTaskRunner` have no dedicated upstream unittests; they are
covered directly per source semantics (`tests/delayed_task_queue.rs`,
`tests/foreground_task_runner.rs`).

## Deviations & deferred registrations (LP-A)

- Seat ticket described an old upstream `TaskQueue` (Priority 1-4 / generation / CancelAll).
  The repo's current source is a plain FIFO + semaphore queue; cancelled-task management lives
  in the frozen `v8-tasks::CancelableTaskManager`. Transcribed the repo source, noted in
  `src/default_platform_generated.rs`.
- `DelayedTaskQueue` upstream is multimap keyed by deadline seconds (double), not a
  `deadline_ticks` heap; ported accordingly with stable post-order tie-break.
- DCHECK/CHECK branches: release-side downgrade to `debug_assert!`/log + task drop (no panic,
  crate red line); `Isolate*` map keys → `IsolateKey = usize` opaque handle.
- Worker thread pool spawn (LP-B): wired through the narrow `WorkerTaskRunner` trait +
  `WorkerRunnerFactory`; without LP-B, `post_task_on_worker_thread` returns
  `Error::WorkerRunnerUnavailable` (replaces `DCHECK_NOT_NULL`); upstream unittest
  `CustomDefaultPlatformTest.RunBackgroundTask` registered `#[ignore]`.
- `CreateJobImpl` / JobTask / JobHandle: LP-B default-job family — trait surface not defined yet.
- `GetThreadIsolatedAllocator`: LP-B thread-isolated-allocator family — omitted.
- Default `tracing::TracingController` construction: LP-C tracing/ family — kept `None` + log.
- `GetStackTracePrinter` / `InProcessStackDumping::Enabled`: base::debug family — log-only.
- `GetZeroSegmentSize`: non-`V8_ENABLE_PARTITION_ALLOC` branch returns 0 — platform-trimmed, not modeled.
- Free functions `PumpMessageLoop/RunIdleTasks/NotifyIsolateShutdown(platform, ...)` collapse to
  `DefaultPlatform` methods (Rust has no downcast-erasure need); `NewDefaultPlatform` parameters
  aggregate into `PlatformOptions`.
- `NotifyIsolateShutdown` missing-key path: upstream dereferences an empty `shared_ptr` (UB);
  Rust no-ops instead (`.cc:314` ↔ `src/platform.rs`), a deliberate safety deviation.
- `std::unique_ptr<Task>` → `BoxTask` (`Box<dyn Task + Send>`): tasks move across threads in Rust,
  so the Send bound is added on top of upstream ownership semantics.
- `SourceLocation::Current()` function name: no `Location::caller()` counterpart for the function
  string, left empty (`src/api.rs`).
- Default no-op virtuals of the original `Platform` not overridden by `DefaultPlatform`
  (`OnCriticalMemoryPressure`, `CreateBoostablePriorityScope`, `CreateBlockingScope`,
  `CurrentClockTimeMilliseconds`/`CurrentClockTimeMillisecondsHighResolution`,
  `DumpWithoutCrashing`, `GetHighAllocationThroughputObserver`, static `SystemClockTimeMillis`,
  `v8-platform.h:1304-1566`): not part of the LP-A narrow surface.


---

<a name="zh"></a>

# v8-libplatform（中文）

`v8/src/libplatform` 的 Rust 转写。三席并发：LP-A 平台/任务队列族、
LP-B job/worker 族、LP-C tracing/ 族。

- [LP-A 落地面（以本仓库现存上游源码为准，1:1）](#lp-a-落地面以本仓库现存上游源码为准11)
- [偏差与推迟登记（LP-A）](#偏差与推迟登记lp-a)

## LP-A 落地面（以本仓库现存上游源码为准，1:1）

- `src/task_queue.rs` ← `task-queue.{h,cc}`：FIFO + 信号量；终止哨兵接力广播。
- `src/delayed_task_queue.rs` ← `delayed-task-queue.{h,cc}`：即时 FIFO + 到期时刻排序
  （multimap 语义以最小堆 + 投递序 tie-break 承接）；类型本身非线程安全（同原版，由调用方加锁）。
- `src/foreground_task_runner.rs` ← `default-foreground-task-runner.{h,cc}`：best-effort 顺序、
  `RunTaskScope` 嵌套深度、NonNestable 弹出规则、到期搬运、条件变量限时等待。
- `src/platform.rs` + `src/api.rs` ← `default-platform.{h,cc}` 与 `include/v8-platform.h` 消费面：
  平台生命周期、消息泵、空闲任务预算泵、前台运行器表、时间函数注入、isolate 关停通知。

测试对照迁移：`task-queue-unittest.cc` → `tests/task_queue_unittest.rs`；
`default-platform-unittest.cc` → `tests/default_platform_unittest.rs`（前台族全量；
依赖 worker 线程池的 `RunBackgroundTask` 登记 `#[ignore]` 不硬跑）。
`DelayedTaskQueue`/`DefaultForegroundTaskRunner` 上游无独立 unittest，按源码语义直测
（`tests/delayed_task_queue.rs`、`tests/foreground_task_runner.rs`）。

## 偏差与推迟登记（LP-A）

- 席位工单描述的是上游旧版 `TaskQueue`（Priority 1-4/generation/CancelAll）；本仓库现存源码为
  FIFO+信号量版，取消管理面在已冻结的 `v8-tasks::CancelableTaskManager`，按现存源码转写，已在
  `src/default_platform_generated.rs` 注记更正。
- DCHECK/CHECK 分支 release 侧降级为 `debug_assert!`/日志 + 任务交回销毁（禁 panic 红线）；
  `Isolate*` 裸指针键 → `IsolateKey`（usize 不透明句柄）。
- worker 线程 spawn（LP-B）：经窄 trait `WorkerTaskRunner` + `WorkerRunnerFactory` 衔接；
  缺位时 worker 投递返回 `Error::WorkerRunnerUnavailable`（承接原 `DCHECK_NOT_NULL`）。
- `CreateJobImpl`/Job 接口面、`GetThreadIsolatedAllocator`：LP-B 族推迟。
- 缺省 tracing controller 构造：LP-C tracing/ 族推迟（暂持 None）。
- `GetStackTracePrinter`/进程内栈打印：base::debug 族推迟（日志登记）。
- `NotifyIsolateShutdown` 缺失键：原版空 `shared_ptr` 解引用（UB），Rust 侧 no-op 承接
  （`.cc:314` ⇔ `src/platform.rs`），属有意的安全性偏差。
- `std::unique_ptr<Task>` → `BoxTask`（`Box<dyn Task + Send>`）：Rust 任务跨线程移交，
  在上版所有权语义上追加 Send 约束。
- `SourceLocation::Current()` 函数名：`Location::caller()` 无函数名字符串对应，置空（`src/api.rs`）。
- `GetZeroSegmentSize`：非 partition_alloc 分支恒 0，平台裁剪不建模。
- 原版自由函数 `PumpMessageLoop` 等收敛为 `DefaultPlatform` 方法；`NewDefaultPlatform`
  入参聚合为 `PlatformOptions`（参数最小化）。
- 原版 `Platform` 中 `DefaultPlatform` 未覆写的默认空操作虚函数
  （`OnCriticalMemoryPressure`、`CreateBoostablePriorityScope`、`CreateBlockingScope`、
  `CurrentClockTimeMilliseconds`/`CurrentClockTimeMillisecondsHighResolution`、
  `DumpWithoutCrashing`、`GetHighAllocationThroughputObserver`、静态 `SystemClockTimeMillis`，
  `v8-platform.h:1304-1566` 段）不属 LP-A 窄面。

依赖：`v8-base-platform`、`v8-tasks`（均冻结，`cargo add --path` 接线）、`v8-base`
（`page_allocator`/`sys_info`，同冻结 crate，仅消费不改其体）。

