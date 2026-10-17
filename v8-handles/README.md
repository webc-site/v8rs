[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-handles (English)

Rust transpilation of the `v8/src/handles` core family (11 files, ~2.5k lines):
Handle/DirectHandle/HandleScope/SealHandleScope, LocalHandles/LocalHandleScope,
MaybeHandle/MaybeDirectHandle/MaybeObjectHandle, PersistentHandles (+List/Scope),
SharedObjectConveyorHandles.

No real GC heap: storage is a testable chunked-address-array — a thread_local `HandleArena`
(blocks + spare + HandleScopeData) mirroring the original isolate->handle_scope_data() and
HandleScopeImplementer blocks/spare layout. next/limit slot-pointer bumping, Enter/Leave
chained scopes and DeleteExtensions block recycling (with spare reuse) are 1:1; a block is
`[Address; KB-2]`.

- [Group → file mapping](#group-file-mapping)
- [Verification](#verification)

## Group → file mapping

| v8 sources (lines) | Rust module | script |
|---|---|---|
| handles.h + handles.cc + handles-inl.h (1772) | src/handles.rs | tool/src/handles/handles.js |
| local-handles.h + local-handles-inl.h + local-handles.cc (337) | src/local_handles.rs | tool/src/handles/local-handles.js |
| maybe-handles.h + maybe-handles-inl.h (671) | src/maybe_handles.rs | tool/src/handles/maybe-handles.js |
| persistent-handles.h + persistent-handles.cc (334) | src/persistent_handles.rs | tool/src/handles/persistent-handles.js |
| shared-object-conveyor-handles.h + .cc (83) | src/shared_object_conveyor_handles.rs | tool/src/handles/shared-object-conveyor-handles.js |

Deferred (not transpiled): global-handles.* (Isolate/Heap bound), traced-handles.* (cppgc
bound), handle-scope-implementer.* (thread archive + context stacks bound; its chunking face
is inlined into HandleArena and will be merged once that family lands).

## Verification

Constant/enum/API-surface assertions live in the generation scripts (audit ledger in
log/v8-handles.md); the two zap constants are compile-time asserted against v8_common
(single source of truth). Rust-idiom deviations (bool+out → Option, CHECK → Result,
RAII Enter/Leave, phantom-typed Tagged) are documented in the Chinese readme. 54 semantic
tests cover the listed guarantees.


---

<a name="zh"></a>

# v8-handles (中文)

`v8/src/handles` 核心族 (11 文件, ~2.5k 行) 的 Rust 转写: Handle/DirectHandle/HandleScope/
SealHandleScope, LocalHandles/LocalHandleScope, MaybeHandle/MaybeDirectHandle/MaybeObjectHandle,
PersistentHandles (+List/Scope), SharedObjectConveyorHandles。

不引入真实 GC 堆: 底层存储为可测试的分块地址数组 —— thread_local `HandleArena`
(blocks + spare + HandleScopeData) 模拟原版 isolate->handle_scope_data() 与
HandleScopeImplementer 的 blocks/spare 数据布局, next/limit 槽位指针推进、Enter/Leave
链式作用域、DeleteExtensions 块回收 (含 spare 复用) 语义 1:1; 块 = `[Address; KB-2]`。

- [组 → 文件映射](#组-文件映射)
- [惯用法映射 (Rust 化偏离)](#惯用法映射-rust-化偏离)
- [依赖](#依赖)
- [测试映射 (原版无独立 gtest, 按 API 语义规约编写)](#测试映射-原版无独立-gtest-按-api-语义规约编写)

## 组 → 文件映射

| v8 源文件 (行数) | Rust 模块 | 转换脚本 |
|---|---|---|
| handles.h + handles.cc + handles-inl.h (1772) | src/handles.rs | tool/src/handles/handles.js |
| local-handles.h + local-handles-inl.h + local-handles.cc (337) | src/local_handles.rs | tool/src/handles/local-handles.js |
| maybe-handles.h + maybe-handles-inl.h (671) | src/maybe_handles.rs | tool/src/handles/maybe-handles.js |
| persistent-handles.h + persistent-handles.cc (334) | src/persistent_handles.rs | tool/src/handles/persistent-handles.js |
| shared-object-conveyor-handles.h + shared-object-conveyor-handles.cc (83) | src/shared_object_conveyor_handles.rs | tool/src/handles/shared-object-conveyor-handles.js |

复跑幂等; 复跑会覆盖就地修订, 仅限重生成基线。

推迟登记 (不转写, 理由):

- global-handles.* (1397) — GlobalHandles/EternalHandles 强绑 Isolate/Heap/WeakCallback,
  依赖 crate (execution/heap) 未建
- traced-handles.* (1435) — TracedHandles 依赖 cppgc (GC 子系统), 对应 crate 未建
- handle-scope-implementer.* (419) — ArchiveThread/RestoreThread 与 entered/saved contexts
  强绑 Isolate/Context (v8-objects-core 并行开发中); 其分块面 (blocks/spare/
  DeleteExtensions/GetSpareOrNewBlock/DetachPersistent) 已按原版数据布局内聚进
  HandleArena (pub(crate)), 待该族正式转写时迁移归并

(对数核对台账见 ../../log/v8-handles.md。)

## 惯用法映射 (Rust 化偏离)

| 原版 | Rust | 理由 |
|---|---|---|
| `Isolate*` 构造参数 | `&HandleScope` / thread_local arena / 显式 `&PersistentHandlesList` | Isolate 推迟; 参数保留证明作用域存活 |
| HandleScope 构造/析构 | RAII `new()`/`Drop` | Enter/Leave 1:1 (move 后不二次关闭, 编译器保证) |
| `bool ToHandle(Handle<S>* out)` | `Option<Handle<T>>` | bool+out 惯用法 |
| `ToHandleChecked()` CHECK 致命 | `Result<_, Error::NullHandle>` | 错误集中 src/error.rs, 禁 panic |
| `GetPersisted` DCHECK | `Result<_, Error::InvalidObjectId>` | 同上 |
| `operator*`/`operator->` | `value()`/`deref()` | Rust 无重载 |
| `RootVisitor::VisitRootPointers` | `iterate(impl FnMut(start, end))` 槽区间回调 | RootVisitor 归 heap 层 |
| `LocalHeap`/`is_main_thread` | `LocalHeapRef` (MAIN / Background(*mut LocalHandles)) | LocalHeap 推迟; 裸指针后端保同宿主嵌套作用域拓扑 |
| C++ 隐式上转 `Handle<S>→Handle<T>` | `unchecked_cast::<T>()` / `as_object()` | Rust 无隐式子类型 |
| direct-handle 配置宏分支 | 仅 `!V8_ENABLE_DIRECT_HANDLE` 分支 (DirectHandle=Handle 包装) | 保守栈扫描关闭的上游默认配置 |
| `NullMaybeHandleType` 哨兵 | `null()` 构造 | Option 语义内化 |

Tagged<T>/Object/HeapObject/Smi 为本 crate 内置幻类型最小面 (v8-objects-core 并行开发未
冻结, 禁依赖), 冻结后归并。

## 依赖

v8-common (Address/KB/tag 常量), v8-base (SmallVector), log,
parking_lot (PersistentHandlesList 互斥)。

## 测试映射 (原版无独立 gtest, 按 API 语义规约编写)

| 规约 | 测试 |
|---|---|
| HandleScope 嵌套 Enter/Leave 与作用域隔离/槽位复用 | tests/handle_scope.rs::nested_scopes_isolated |
| 计数语义 ((块数-1)*块大小+末块已用) | tests/handle_scope.rs::number_of_handles_counts |
| 块扩展 (超 1022 跨块) | tests/handle_scope.rs::block_extension_beyond_one_block |
| CloseAndEscape 逃逸存活 | tests/handle_scope.rs::close_and_escape_survives_scope |
| CloseAndEscape 空句柄安全收窄 (原版 *handle 解引用 null 属 UB → 返回 null, 不在父作用域分配) | tests/handle_scope.rs (2 条) / tests/local_handles.rs (2 条) |
| SealHandleScope 封印/解封 | tests/handle_scope.rs::seal_handle_scope_restriction |
| 关闭即失效 (槽复用覆盖) | tests/handle_scope.rs::scope_close_invalidates_handles |
| Handle 间接访问/解引用 | tests/handle_semantics.rs::indirect_dereference |
| PatchValue 同 location 可见性 | tests/handle_semantics.rs::patch_value_shared_location |
| equals (location) vs is_identical_to (对象身份) | tests/handle_semantics.rs::equals_vs_is_identical_to |
| 空句柄语义 | tests/handle_semantics.rs::null_handle_semantics |
| UncheckedCast 换型复用 location | tests/handle_semantics.rs::unchecked_cast_reuses_location |
| Smi 31 位编解码 | tests/handle_semantics.rs::tagged_smi_encoding |
| DirectHandle 包装/FromSlot/FromAddress | tests/handle_semantics.rs (3 条) |
| DirectHandleVector/SmallVector 容器面 | tests/handle_semantics.rs (2 条) |
| MaybeHandle 空/转换 (Option/Err) | tests/maybe_handle.rs::maybe_handle_null_and_conversion |
| MaybeObjectHandle 弱/强 (make_weak tag) | tests/maybe_handle.rs::maybe_object_handle_weak_strong |
| MaybeObjectHandle/Direct identical | tests/maybe_handle.rs (2 条) |
| PersistentHandles 跨 scope 存活/重分配/块扩展/iterate | tests/persistent_handles.rs (4 条) |
| 链表 Add/Remove/iterate | tests/persistent_handles.rs::persistent_list_registration |
| move 语义 (Box 移交后句柄/链表有效) | tests/persistent_handles.rs::persistent_move_semantics |
| Attach/Detach owner | tests/persistent_handles.rs::persistent_attach_detach_owner |
| PersistentHandlesScope detach 移交/计数 | tests/persistent_handles.rs (2 条) |
| LocalHandles 分配/嵌套回退/块扩展/iterate | tests/local_handles.rs (3 条) |
| Main 后端转发全局 arena | tests/local_handles.rs::local_scope_main_thread_forwarding |
| LocalHandleScope CloseAndEscape | tests/local_handles.rs::local_scope_close_and_escape |
| conveyor persist 递增 id/越界 Err/跨 scope 保活 | tests/shared_conveyor.rs (2 条) |
| thread_local 线程隔离 / PersistentHandles 跨线程 Send | tests/threads.rs (2 条) |
| zap 常量 1:1 (kPersistentHandleZapValue 不得与句柄 zap 混用) / 回收块 zap / 分界块边界 / Seal break 分支 | tests/zap_semantics.rs (7 条) |

