# v8-heap-cppgc 中文说明

## 挂载面（已落地）

扁平导出，全部经 `src/lib.rs` `pub use`。核心对象模型与元数据表已 1:1 对位 Oilpan 规范。

## 推迟台账

### 一、本轮挂载收编（8 草案）

W6GC2b 遗留、本波 [轮7 W7GC2] 挂载的 8 件，均按已落地 API（`AccessMode` 运行期参数、
`HeapObjectHeader` 面）就地改写适配，`.skel` 草稿已删除：

| 件 | 原版 | 状态 |
|---|---|---|
| `task_handle.rs` | task-handle.h SingleThreadedHandle | 填体（`shared_ptr<bool>`→`Arc<Cell<bool>>`；DCHECK 非空臂以 debug_assert 承接，无 panic 出口） |
| `memory.rs` | memory.h/cc | 填体（目标配置非 sanitizer；`assert_eq` CHECK 面 → `debug_assert`） |
| `liveness_broker.rs` | liveness-broker.h/cc | 填体（补 `LivenessBroker` 门面结构 + 默认构造） |
| `object_view.rs` | object-view.h ObjectView | 填体 + 深水占位（见二） |
| `object_size_trait.rs` | object-size-trait.cc | 填体 GC 臂 + 深水占位 mixin 臂 |
| `compaction_worklists.rs` | compaction-worklists.h/cc | 填体（`base::Worklist<_,256>` → `Vec` 暂承，见三） |
| `process_heap_statistics.rs` | process-heap-statistics.h/cc | 填体（补 `PROCESS_HEAP_STATS` static + `current()`；trait 绑定见三） |
| `metric_recorder.rs` | metric-recorder.h | 类型面全落 + 空 trait（perfetto 落地面留） |

挂载件内实测 `todo!()` 深水面 **3** 处（见下）。

### 二、挂载件深水面 `todo!`（3）

1. `object_view.rs::end` — 大对象臂 `LargePage::PayloadEnd`，依赖 `BasePage::FromPayload` 回溯，随 heap_page 批次。
2. `object_view.rs::size` — 大对象臂 `LargePage::ObjectSize`，同上。
3. `object_size_trait.rs::get_object_size_for_garbage_collected_mixin` — `BasePage::FromPayload` + `ObjectHeaderFromInnerAddress`，同上。

### 三、跨批次绑定（挂载件内固有面已落，trait/外部绑定待承接）

- `compaction_worklists`：`MovableReferencesWorklist` 暂用单线程 `Vec`，分段固定容量并发队列待 `heap::base::Worklist` 绑定面（他 crate，本席禁触）。
- `process_heap_statistics::AllocationObserverImpl`：原版 `: public StatsCollectorAllocationObserver`，trait 绑定待 `stats_collector` 批次；本面先落同签名固有方法。

### 四、未挂载草案登记（50 件 `.skel`，深水面推迟）

以下 cppgc 手写面草稿依赖尚未落地的堆骨架类型（`HeapBase`/`BasePage`/`NormalPage`/`LargePage`/
`RawHeap`/`HeapHandle`/`PageBackend`/`AllocationHandle`/`Worklist`/`TracingController`/`Platform` 等），
挂载即断门禁，故留 `.skel`，随 heap_page / heap_base / marker / sweeper / allocation / object_allocator
批次收编：

- 堆骨架与页：`heap` `heap_base` `raw_heap` `heap_page` `heap_space` `heap_growing` `heap_config`
  `heap_consistency` `page_memory` `virtual_memory` `caged_heap` `caged_heap_local_data` `cpp_heap` `platform`
- 分配面：`allocation` `object_allocator` `free_list` `object_poisoner`
- 标记面：`marker` `concurrent_marker` `marking_state` `cpp_marking_state` `unified_heap_marking_state`
  `marking_visitor` `unified_heap_marking_visitor` `marking_worklists` `unified_heap_marking_verifier`
  `marking_verifier` `remembered_set` `cross_heap_remembered_set` `write_barrier` `unmarker` `liveness`（已并 liveness_broker）
- 终结/存活面：`prefinalizer_handler` `explicit_management` `persistent_node` `pointer_policies` `member_storage`
- GC 调度与统计：`gc_invoker` `garbage_collector` `sweeper` `compactor` `compaction`（并入 compaction_worklists）
  `heap_state` `heap_statistics_collector` `stats_collector` `process_heap` `trace_event` `logging` `testing`

### 五、非目标配置裁剪面（不实现）

- sanitizer 臂（ASAN/MSAN poison、`ASAN_UNPOISON`）：`memory`/`heap_object_header` 中裁剪。
- `CPPGC_POINTER_COMPRESSION` 全部件（`member_storage` 整件 cfg 内）：目标配置 `!CPPGC_POINTER_COMPRESSION`。
- ARMv7 `ShouldForceNonAtomic` 臂：目标 x64/arm64 恒 false。
- `V8_USE_PERFETTO` trace 臂、`CAGE_BASE` 偏移换算：随各自批次或裁剪。
