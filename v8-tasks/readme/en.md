# v8-tasks (en)

Rust transpilation of `v8/src/tasks` (cancelable-task, operations-barrier,
task-utils, ~564 original LOC) plus the Task/IdleTask/TaskPriority interfaces
from `v8/include/v8-platform.h`. See `zh.md` for the full symbol mapping,
deviations (deferred Isolate ctors, CHECK downgrade, no TaskSupplier in this
V8 checkout) and the test coverage table (`tests/SPEC.md`).

Dependencies actually adopted: `v8-base-platform` (Mutex/ConditionVariable/Thread),
`gxhash` (manager task map under lock), `strum` (derive). `crossbeam-deque` is
left to the `v8-libplatform` crate (TaskQueue/Worker live in `src/libplatform`).
