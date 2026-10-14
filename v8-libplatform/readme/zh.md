# v8-libplatform（中文）

`v8/src/libplatform` 的 Rust 转写。三席并发：LP-A 平台/任务队列族、
LP-B job/worker 族、LP-C tracing/ 族。

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
