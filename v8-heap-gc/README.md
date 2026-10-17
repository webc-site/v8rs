[English](#en) | [中文](#zh)

---

<a name="en"></a>

#

---

<a name="zh"></a>

# v8-heap-gc 推迟台账 (deferred ledger)

分工: 本 crate 承载 GC 算法纯语义面; 依赖 v8-heap-core 对象模型
(Heap / Isolate / Tagged / Map / NormalPage / MutablePage / Space /
SlotSet / RootVisitor / ObjectVisitor) 的深水面方法体在本表登记,
待 heap-core 冻结后逐函数挂载。禁引外部 GC 库, 挂载时 1:1 保留原版算法。

- [.skel 草案收编对账 (轮8 W8HGC)](#skel-草案收编对账-轮8-w8hgc)
- [推迟方法面 (按原版锚点)](#推迟方法面-按原版锚点)
  - [mark-compact (cc 8k 行大族, 锚点见 skel 内 /// v8: 行)](#mark-compact-cc-8k-行大族-锚点见-skel-内-v8-行)
  - [sweeper](#sweeper)
  - [marking-barrier (14)](#marking-barrier-14)
  - [marking-state (5)](#marking-state-5)
  - [marking-worklist (3)](#marking-worklist-3)
  - [marking-visitor (0 todo, 未挂)](#marking-visitor-0-todo-未挂)
  - [scavenger / minor-mark-sweep / memory-reducer](#scavenger-minor-mark-sweep-memory-reducer)
- [推迟台账分组 (轮10 W10HG, 组 | 符号 | 前置 | 解锁 crate)](#推迟台账分组-轮10-w10hg-组-符号-前置-解锁-crate)
- [工具脚本核查 (轮10 W10HG)](#工具脚本核查-轮10-w10hg)
- [测试迁移对账 (轮10 W10HG, 1:1 原用例名)](#测试迁移对账-轮10-w10hg-11-原用例名)

## .skel 草案收编对账 (轮8 W8HGC)

| skel | todo! | 处置 | 说明 |
| --- | --- | --- | --- |
| mark_compact_cc.rs.skel | 97 | 跳过(重复) | 纯面已挂 mark_compact.rs (启发式/疏散候选筛选/防震荡); 余为对象迁移方法骨架 |
| sweeper_cc.rs.skel | 70 | 跳过(重复) | 纯面已挂 sweeper.rs (并发清扫状态机 SweepingStateMachine/Zapping/TraceId); 余为平台任务句柄与页面列表 |
| marking_barrier.rs.skel | 14 | 登记推迟 | MarkingBarrier 写屏障面 (Heap/LocalHeap/MarkCompactCollector) |
| memory_reducer_cc.rs.skel | 13 | 跳过(重复) | 纯面已挂 memory_reducer.rs; 余为 State::Step 深分支 |
| marking_state.rs.skel | 5 | 登记推迟 | MarkingStateBase (Isolate/Tagged/bitmap 挂载点) |
| minor_mark_sweep.rs.skel | 4 | 跳过(重复) | 纯面已挂 minor_mark_sweep.rs |
| marking_worklist.rs.skel | 3 | 登记推迟 | MarkingWorklists (base::Worklist 对象模型) |
| scavenger.rs.skel | 2 | 跳过(重复) | 纯面已挂 scavenger.rs (AgeMark 晋升判别/任务计算/拷贝策略/竞争结果/统计); 余为 ScavengerCollector 入口 |
| marking.rs.skel | 0 | 跳过(重复) | 已挂 marking.rs (MarkBit/MarkingBitmap) |
| marking_visitor.rs.skel | 0 | 登记推迟 | MarkingVisitorBase CRTP trait 面 (Tagged* 参数) |
| marking_progress_tracker.rs.skel | 0 | 跳过(重复) | 已挂 marking_progress_tracker.rs |
| incremental_marking_job.rs.skel | 0 | 跳过(重复) | 已挂 incremental_marking_job.rs |
| incremental_marking_cc.rs.skel | 0 | 跳过(重复) | 已挂 incremental_marking.rs (阈值/步时长纯式) |
| gc_tracer_cc.rs.skel | 0 | 跳过(重复) | 已挂 gc_tracer_cc.rs (事件名表/速度界) |

基线 208 todo! 全部位于以上未挂载草案; 已挂 .rs 面 0 todo!。
草案均引用本 crate 未定义的对象模型类型, 挂载即破编译,
故按「重复登记 / 推迟台账」处置, 不强行挂载。

## 推迟方法面 (按原版锚点)

### mark-compact (cc 8k 行大族, 锚点见 skel 内 /// v8: 行)
- MarkCompactCollector: ctor/TearDown cc:300/314, StartCompaction cc:349,
  StartMarking cc:437, CollectGarbage cc:526,
  ComputeEvacuationHeuristics cc:596 (纯规则核已挂 mark_compact.rs::compute_evacuation_heuristics),
  CollectEvacuationCandidates cc:649 (纯规则核已挂 mark_compact.rs::select_evacuation_candidates，涵盖碎片空闲门限判定、升序排序、配额贪心限制与防 compact->expand 反复震荡截断),
  Prepare/Finish cc:800/873,
  MarkRoots cc:1923, MarkTransitiveClosure* cc:2141-2286,
  ProcessMarkingWorklist cc:2300, ProcessEphemeron cc:2374,
  ClearNonLiveReferences cc:3082, code flushing 面 cc:3525-3910,
  weak collections cc:4145-4245, reloc slots cc:4317-4373,
  Evacuate* cc:4641-5281, UpdatePointers* cc:5781-5946,
  aborted candidates cc:6010-6070, Sweep* cc:6135-6270,
  MarkObject/RecordSlot -inl.h:27-70
- RootMarkingVisitor cc:6339-6344, -inl.h:122-136
- Evacuator cc:4780/4807/4861; LiveObjectVisitor cc:4836/4850

### sweeper
- Sweeper::StartSweeping/RequestSweeping/SweepSomeOrAll/ConcurrentSweeping 面
- SweepingState 状态机: 纯逻辑已挂 sweeper.rs::SweepingStateMachine，实现 Idle -> Initialized -> Sweeping (Active/Paused) -> Joined -> Finished (Idle) 闭环状态转移与 trace_id 派生；具体多线程 JobHandle 调度依赖外部平台设施。
- ConcurrentSweeper::SweepSpace/CompleteSweeping; quarantine 面

### marking-barrier (14)
- Activate/Publish/Deactivate 全谱, Write/WriteWithoutHost,
  MarkValue*/MarkRange, RecordRelocSlot (-inl.h)

### marking-state (5)
- TryMark / TryMarkAndAccountLiveBytes / IsMarked / IsUnmarked 两实例化
  (ATOMIC/NON_ATOMIC)

### marking-worklist (3)
- CreateContextWorklists/ReleaseContextWorklists,
  Local::Push/Pop/Publish/ShareWork/MergeOnHold/PublishCppHeapObjects

### marking-visitor (0 todo, 未挂)
- MarkingVisitorBase / FullMarkingVisitorBase CRTP→trait 面

### scavenger / minor-mark-sweep / memory-reducer
- Scavenger: 纯逻辑已挂 scavenger.rs (NumberOfScavengeTasks 并发任务数纯式、is_address_below_age_mark 对象晋升判别式、determine_scavenge_preference 目标偏好、MigrationOutcome 复制竞争结局模型、ScavengerTaskStats 任务级进度统计与门限检测)；对象槽位迭代与物理页搬移推迟待接。
- MinorMS 依赖对象模型的方法; MemoryReducer 深分支

## 推迟台账分组 (轮10 W10HG, 组 | 符号 | 前置 | 解锁 crate)

实测确认: 208 处 `todo!` 全部位于 `.skel` 草案的**对象模型/平台耦合方法体** (引用 `Heap`/`Isolate`/`Tagged<HeapObject>`/`Map`/`NormalPage`/`MutablePage`/`Space`/`SlotSet`/`ObjectVisitor`/`RelocInfo`/`JobHandle` 等); 本 crate `#![forbid(unsafe_code)]` 且无这些上游类型定义, 挂载即破编译。上游 `v8-heap-core`/`v8-objects-core` 的同名类型现仅存在于各自 `.skel` 草案 (未冻结), 故全部按「不强挂、分组登记」处理。纯逻辑核 (疏散/位图/状态机/任务数/reducer step) 均已在轮 7/8 挂载, 本轮无遗漏可补差集。

> **W11HGR2 现态校正 (2026-10-13)**: 上段「同名类型仅存在于 .skel」已过时——现场 grep: v8-objects-core 已挂载 `tagged.rs`(Tagged/Object)/`heap_object.rs`/`map.rs`/`visitors.rs`(ObjectVisitor/RootVisitor); v8-heap-core 已挂载 `memory_balancer.rs`/`memory_chunk.rs`/`spaces.rs`(Space trait/PageList)/`slot_set.rs`(TypedSlots/TypedSlotSet)/`free_list.rs` 等 19 个 .rs; `base::Worklist` 已在 v8-heap-base/src/worklist.rs 挂载 (0 todo)。但各推迟组**硬前置仍未落**: `Heap`/`LocalHeap` 无挂载 (heap_cc/local_heap 皆 .skel), `MutablePage`/`NormalPage` 仅为 opaque 空枚举桩 (无成员), 位图页类 `SlotSet` 未挂, `JobHandle`/延时任务投递未挂。故 208 推迟结论与「挂载即破编译」判定不变; 各组的失实前置词在下列行内校正。

| 组 | 符号 (方法面) | 前置 (缺什么) | 解锁 crate/面 | todo! |
| --- | --- | --- | --- | --- |
| mark-compact-collector | ctor/TearDown/StartCompaction/StartMarking/CollectGarbage/Prepare/Finish/MarkRoots/MarkTransitiveClosure*/ProcessMarkingWorklist/ProcessEphemeron/MarkLiveObjects/ClearNonLiveReferences/FlushSFI/ClearWeak*/Evacuate*/UpdatePointers*/Sweep* 等 (cc 全谱) | `Heap`/`Isolate`/`PagedSpace`/`NormalPage`/`MutablePage`/`Tagged<HeapObject>`/`SlotSet`/`ObjectVisitor`/`RelocInfo` 对象模型 + RawPage 页遍历 | v8-heap-core (spaces/page/slot_set/visit_object 冻结) + v8-objects-core (Tagged/Map 冻结) | 82 |
| mark-compact 内联 visitor | RootMarkingVisitor (VisitRootPointer/Pointers/MarkObjectByPointer), Evacuator (EvacuatePage/RawEvacuatePage/Finalize), LiveObjectVisitor (VisitMarkedObjects*) | `Root`/`FullObjectSlot`/`MutablePage`/`Tagged<HeapObject>`/`MarkBit` 挂载点 | v8-roots + v8-heap-core page + v8-objects-core | 11 |
| mark-compact-recordslot | MarkObject/MarkRootObject/RecordSlot (inl.h) | `Tagged<HeapObject>`/`HeapObjectSlot`/`MemoryChunk` + 写屏障 | v8-heap-core (barrier/slot) | 4 |
| marking-barrier | Activate/Publish/Deactivate/Write/WriteWithoutHost/MarkValue*/MarkRange/RecordRelocSlot | `Heap`/`LocalHeap`/`MarkingBarrier` 写屏障状态 (W11HGR2: `Tagged` 已挂 objects-core/tagged.rs, 不再阻塞) | v8-heap-core (heap-write-barrier/local-heap 冻结) | 14 |
| marking-state | TryMark/TryMarkAndAccountLiveBytes/IsMarked/IsUnmarked (ATOMIC+NON_ATOMIC 两实例) | `Isolate`/`Tagged<HeapObject>`/`MutablePage::IncrementLiveBytes*` + 原子位图挂载 (bitmap 纯面已挂) | v8-heap-core (mutable_page) + v8-objects-core (tagged) | 5 |
| marking-worklist | CreateContextWorklists/ReleaseContextWorklists/Local::Push/Pop/Publish/ShareWork/MergeOnHold/PublishCppHeapObjects | (W11HGR2: `base::Worklist` 已挂 v8-heap-base/worklist.rs 0 todo; 余阻 = skel 的 `MarkingWorklist` 类型与 Local 字段形状未建模 + 本 crate 未引该依赖且需放宽 forbid(unsafe_code)) | v8-heap-core (weak_object_worklists) + v8-heap-base 依赖引入; 且需放宽 forbid(unsafe_code) | 3 |
| marking-visitor | MarkingVisitorBase/FullMarkingVisitorBase CRTP→trait 面 (0 todo, 未挂) | `Tagged<HeapObject>`/`ObjectVisitor`/`MarkingState` 参数 | v8-objects-core + v8-heap-core | 0 |
| sweeper-platform | SweepingState(Initialize/Start/StartConcurrent/Join/Finish/Pause/Resume/HasValid/HasActive/Stop) + LocalSweeper(ParallelSweepSpace/Page/IteratePromotedPages*) + Sweeper(ctor/TearDown/Initialize/Start*/GetSweptPage/FinishJobs/Ensure*Completed/CanSwitchTo*/SweepSomeOrAll/Wait*) | `base::JobHandle`/`ManualJob`/平台线程调度 + `Space`/`MutablePage` 页链表 (纯状态转移已挂 SweepingStateMachine) | v8-base-platform (并发任务句柄) + v8-heap-core (spaces/page) | 70 |
| memory-reducer-heap | GetCommitedSize/Base ctor/Create/TimerTask::RunInternal/NotifyTimer/NotifyMarkCompact/NotifyPossibleGarbage/ScheduleTimer + LimitBased*(纯 step 已挂) | `Heap::GetCommittedMemory` + 平台定时器投递 | v8-heap-core (heap) + v8-base-platform (task runner) | 13 |
| minor-mark-sweep | CollectGarbage/StartMarking/RequestGC/TearDown | `MutablePage`/`SlotSet`/`Heap`/记忆集 (W11HGR2: `TypedSlotSet` 已挂 heap-core/slot_set.rs、`ObjectVisitor` 已挂 objects-core/visitors.rs、Mutable/NormalPage 仅空桩不算落; 不再阻塞项已剔除) | v8-heap-core (slot_set 位图页/page 成员) + v8-objects-core | 4 |
| scavenger-collector | ScavengerCollector::CollectGarbage/CompleteSweepingQuarantinedPagesIfNeeded | `Heap`/`QuarantinedPageSweeper`/from-to-space 页对象槽位 (任务数/age-mark/拷贝偏好纯式已挂) | v8-heap-core (new-spaces/mutable-page) | 2 |

合计 208, 与基线一致; 挂载文件 (已挂 .rs) 0 todo 维持。
W11HGR 复审校正: mark-compact 三子组按 skel 锚点实测重分 (collector 60→82,
内联 visitor 21→11=RootMarkingVisitor 6+Evacuator 3+LiveObjectVisitor 2,
recordslot 4 不变), 三行和 = mark_compact_cc.rs.skel 实测 97; 原表列和 196≠208。

## 工具脚本核查 (轮10 W10HG)

- gc 族 14 个 `.cc.js`/主脚本复跑对 `.skel` 草案幂等: 13/14 完全一致。
- `marking.js` 复跑会重生成一行 `#[allow(clippy::upper_case_acronyms)]` (已提交草案刻意删除), 属生成器滞后; 按「src 现态为准不覆写」保留草案现态, 未覆盖。
- gc 族 `.h.js` (gc-tracer.h/mark-compact.h/memory-reducer.h/sweeper.h) 直写**已挂枚举 .rs** (非 .skel), 复跑会回退 UpperCamelCase 修订, 依规则**不执行**; 未触碰他席 crate 脚本与 tool/lib.js。
- 锚核对: mark_compact_cc.rs.skel 含 99 处 `/// v8:` 锚, marking.rs.skel 27 处, 齐备。

## 测试迁移对账 (轮10 W10HG, 1:1 原用例名)

- `unittests/heap/marking-bitmap-unittest.cc` → `tests/marking_bitmap.rs`: IsZeroInitialized/Cells/CellsCount/IsClean/Clear/ClearRange1/ClearRange2/SetAndClearRange/ClearMultipleRanges/TransitionMarkBit (Atomic 实例化与非原子语义等价, 合并验证)。
- `unittests/heap/marking-progress-tracker-unittest.cc` → `tests/marking_progress_tracker.rs`: DefaultDisabled/EnabledAfterExplicitEnable/ZerothChunkFirst/NumberOfChunks/GetNextChunkToMarkIncrements/ResetIfEnabledOnDisabled/ResetIfEnabledOnEnabled (DeathTest 2 例 DEBUG-only, 无安全断言面对应, 记跳过)。
- `unittests/heap/memory-reducer-unittest.cc` → `tests/memory_reducer.rs`: FromDoneToDone/FromDoneToWait/FromWaitToWait/FromWaitToRun/FromWaitToDone/FromRunToRun/FromRunToDone/FromRunToWait (flag 默认值参数化 long=8000/start=30000/max_gcs=2)。
- `mark-compact-unittest.cc`/`gc-tracer-unittest.cc`: 全部 TEST_F 依赖 `TestWithHeap`/`GCTracer` 实例 (对象模型), 属推迟面, 未迁。
- `marking-worklist-unittest.cc`: 依赖 `base::Worklist<Object>` 无锁队列 (unsafe), 属推迟面, 未迁。
- 门禁: nextest 39 → 64 全绿 (+25), clippy `--all-targets -D warnings` 0 警告。

