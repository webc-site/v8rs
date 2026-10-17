[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-tasks (en)

Rust transpilation of `v8/src/tasks` (cancelable-task, operations-barrier,
task-utils, ~564 original LOC) plus the Task/IdleTask/TaskPriority interfaces
from `v8/include/v8-platform.h`. See `zh.md` for the full symbol mapping,
deviations (deferred Isolate ctors, CHECK downgrade, no TaskSupplier in this
V8 checkout) and the test coverage table (`tests/SPEC.md`).

Dependencies actually adopted: `v8-base-platform` (Mutex/ConditionVariable/Thread),
`gxhash` (manager task map under lock), `strum` (derive). `crossbeam-deque` is
left to the `v8-libplatform` crate (TaskQueue/Worker live in `src/libplatform`).


---

<a name="zh"></a>

# v8-tasks 转写说明

范围：`v8/src/tasks`（cancelable-task / operations-barrier / task-utils，原版 6 个
h/cc 文件共 564 行），语义来源 `v8/include/v8-platform.h` 的 Task / IdleTask /
TaskPriority。签名先行对照与测试规约见 `tests/SPEC.md`。

- [原版文件 → Rust 模块映射](#原版文件-rust-模块映射)
- [符号对照（原版 40 项可数符号 → Rust 全量落地）](#符号对照原版-40-项可数符号-rust-全量落地)
- [偏差与推迟项（待回收登记）](#偏差与推迟项待回收登记)
- [测试覆盖映射（7.3）](#测试覆盖映射73)
- [依赖（todo 10.2 登记建议）](#依赖todo-102-登记建议)

## 原版文件 → Rust 模块映射

| 原版 | Rust | 备注 |
|---|---|---|
| `src/tasks/cancelable-task.h/.cc` | `src/cancelable.rs` | TryAbortResult / CancelableTaskManager / Cancelable / CancelableTask / CancelableIdleTask |
| `src/tasks/operations-barrier.h/.cc` | `src/operations_barrier.rs` | OperationsBarrier / Token→`BarrierToken` |
| `src/tasks/task-utils.h/.cc` | `src/task_utils.rs` | make_cancelable_task / make_cancelable_idle_task + 闭包包装（对应 CancelableFuncTask / CancelableIdleFuncTask）|
| `include/v8-platform.h`（Task/IdleTask/TaskPriority）| `src/task.rs` | 纯接口，tasks 的语义来源 |
| — | `src/error.rs` | InvalidTaskId / TaskIdOverflow（原版 CHECK 路径 Result 化）|

## 符号对照（原版 40 项可数符号 → Rust 全量落地）

- `enum class TryAbortResult{kTaskRemoved,kTaskRunning,kTaskAborted}` → `TryAbortResult{TaskRemoved,TaskRunning,TaskAborted}`
- `CancelableTaskManager::{Id, kInvalidTaskId, ctor, dtor, Register, TryAbort, TryAbortAll, CancelAndWait, canceled, RemoveFinishedTask}` → `TaskId, INVALID_TASK_ID, new, Drop(debug_assert), Cancelable::new 折叠(内部 register), try_abort, try_abort_all, cancel_and_wait, canceled, remove_finished_task`
- `Cancelable::{ctor, dtor, id, Status{kWaiting,kCanceled,kRunning}, TryRun(含 previous 重载), Cancel, CompareExchangeStatus}` → `Cancelable::{new, Drop, id, Status, try_run/try_run_prev, compare_exchange_status}`；`Cancel` 落在私有 `CancelInner::cancel`（模块私有，原版 private+friend 语义）（内存序 acq_rel/acquire 逐条对齐）
- `CancelableTask / CancelableIdleTask`（Run final + RunInternal 纯虚）→ 同名 struct + `TaskBody / IdleTaskBody` trait + `impl Task / IdleTask`
- `OperationsBarrier::{TryLock, CancelAndWait, cancelled, Release}` 与 `Token::{move ctor, operator=, operator bool}` → 同名方法 + `BarrierToken::{ok, Drop}`（Rust 移动语义承接 move-assign 的旧引用释放）
- `MakeCancelableTask / MakeCancelableIdleTask` + `CancelableFuncTask / CancelableIdleFuncTask` → `make_cancelable_task / make_cancelable_idle_task` + 私有关闭包装

## 偏差与推迟项（待回收登记）

1. **Isolate 便捷构造未转写**（4 个重载：`CancelableTask(Isolate*)`、
   `CancelableIdleTask(Isolate*)`、`MakeCancelableTask(Isolate*,…)`、
   `MakeCancelableIdleTask(Isolate*,…)`）。依赖 `execution/isolate.h`（L4 执行层），
   待 v8-objects/execution 落地后由上层 crate 以 `isolate.cancelable_task_manager()`
   入口回收。
2. **CHECK 降级**：`~CancelableTaskManager` 的 `CHECK(canceled_)`、`TryAbort` 的
   `CHECK_NE(kInvalidTaskId, id)`、`Register` 的 id 回绕 CHECK，按"产品路径禁 panic"
   规范分别降级为 `debug_assert!` 与 `Err(Error::{InvalidTaskId, TaskIdOverflow})`。
   id 溢出防御为粘性错误：持锁分配前检测若已达 u64::MAX 则钉死报错，
   小于 u64::MAX 才加一，彻底杜绝 debug 模式算术溢出 panic，同时永不复用 id（防 ABA）。
3. **所有权模型**：原版管理器存 `Cancelable*` 裸指针；此处 `Arc<CancelInner>` +
   管理器侧 `Weak<CancelInner>`，`~Cancelable` 的 CAS 探测 + RemoveFinishedTask 协议
   逐条保留（含"取消态任务不触发摘除"的分支）。`Register(task)` 折叠进
   `Cancelable::new`。
4. **无 TaskSupplier**：全仓 grep `src/`、`include/` 无 `TaskSupplier/RemoteTaskLock`
   文件（属早期 V8 libplatform 设计，本 checkout 已移除）。work-stealing 协议本轮
   不适用；**crossbeam-deque 本轮不引入**（避免死依赖），由 v8-libplatform 轮按需接入。
5. **TaskQueue / Worker / StackGuard**：位于 `v8/src/libplatform`
   （task-queue.{h,cc}、worker-thread.{h,cc}），todo 三章 L1 归 **v8-libplatform**
   crate，不在 v8-tasks 范围；StackGuard 待 execution 层。
6. 管理器内部 `UnsafeCell<HashMap>`（锁内独占）替代 clippy `mut_from_ref` 形态的
   helper；`canceled_` 非同步读改为 `AtomicBool`（消除原版数据 race）。

## 测试覆盖映射（7.3）

| 原版测试 | Rust | 状态 |
|---|---|---|
| `test/unittests/tasks/cancelable-tasks-unittest.cc`（TEST_F CancelableTaskManagerTest，11 用例 cc:123-263）| `tests/cancelable_tasks.rs`（同名 11 用例 + 6 个头注释/CHECK/CAS/回绕防御契约用例，共 17 用例）| 1:1 迁移；忙等保留为原子自旋（事件同步，非墙钟）|
| `test/unittests/tasks/background-compile-task-unittest.cc`（7 用例）| — | **整文件跳过**：依赖 Isolate/Compiler/flags/zone 全家桶，属 L3+ 引擎层，非 tasks 语义 |
| `operations-barrier`（原版无单测）| `tests/operations_barrier.rs`（5 用例）| 从 .h 契约注释提取规约（SPEC.md 20–25）；阻塞性用计数不变量 + channel 事件同步证明，无墙钟断言 |
| `task-utils`（原版无直接单测）| `tests/task_utils.rs`（4 用例）| 闭包包装与 id 登记语义 |
| `include/v8-platform.h` Task/IdleTask | `tests/cancelable_tasks.rs` 内 run 守卫/ deadline 用例 | 接口经 CancelableTask 承接验证 |

## 依赖（todo 10.2 登记建议）

- 实际接入：`v8-base-platform`（Mutex/ConditionVariable/Thread，路径依赖）、
  `gxhash`（管理器任务表，锁内使用）、`strum`（derive，Status FromRepr）。
- 10.2 v8-tasks 行的 `crossbeam-deque` 未启用（见偏差 4），建议 Reviewer 在
  todo 10.2 该行为 v8-tasks 补注："实际采用 v8-base-platform + gxhash + strum；
  crossbeam-deque 转记到 v8-libplatform"。

