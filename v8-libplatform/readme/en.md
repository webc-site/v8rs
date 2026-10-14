# v8-libplatform

Rust transcription of `v8/src/libplatform` (~3k C++ lines). Scheduling: three seats —
LP-A platform/task-queue family, LP-B job/worker family, LP-C `tracing/` family.

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
