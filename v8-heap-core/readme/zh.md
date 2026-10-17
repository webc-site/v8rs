# v8-heap-core 详细台账

对应原版 `v8/src/heap` 内存分配器文件族（本仓新版实测勘定: 无 space.h/space-type, 文件族 memory-chunk/spaces/main-allocator/memory-allocator/memory-pool/free-list）。看板行只留状态; 波次台账与对数记录见 ../../log/v8-heap-core.md, 模块总览见 ../README.md。

## spaces 族 todo! 台账

判定基准: 真实可执行 `todo!()` 调用形（注释/doc 文面提及不计）。现存 12 处（spaces.rs 7 / large_spaces.rs 5, grep 实测）。分组 A-E。

**A 组 · 深水 — Heap/GC 整合票解锁（4 处, 全在 large_spaces.rs）**

| # | 落点 | 原版锚 | 前置码 |
|---|------|--------|--------|
| 8 | `LargeObjectSpace::allocate_large_page` | large-spaces.h:105-106 + .cc:143-164 | MemoryAllocator::AllocateLargePage 填体（memory_allocator.rs 推迟台账 1, HC-2b 系）+ `Heap::heap_expansion_mutex`/`IsOldGenerationExpansionAllowed`（Heap 整合票）; cc:156 DCHECK_GE 随体 |
| 11 | `OldLargeObjectSpace::allocate_raw_executability` | large-spaces.h:136-139 + .cc:104-141 | `Heap::ShouldExpandOldGenerationOnSlowAllocation`/`CanExpandOldGeneration`/`StartIncrementalMarkingIfAllocationLimitIsReached`/`GCFlagsForIncrementalMarking`/`marking_state()->TryMarkAndAccountLiveBytes`/`incremental_marking()->black_allocation`/`NotifyOldGenerationExpansion` + `v8_flags.sticky_mark_bits`（Heap 整合票 + marking 状态机）; cc:108-109 ALIGN_TO_ALLOCATION_ALIGNMENT 与 cc:134 InitializationMemoryFence（面已落 memory_chunk.rs:303）随体 |
| 12 | `OldLargeObjectSpace::promote_new_large_object` | large-spaces.h:132 + .cc:173-186 | 页 `owner()` 反解 `LargeObjectSpace*`（页实体票）+ `SetOldGenerationPageFlags(incremental_marking()->marking_mode())`（GC 阶段状态机）; cc:174-180 DEBUG 断言区随体 |
| 13 | `NewLargeObjectSpace::allocate_raw` | large-spaces.h:163-164 + .cc:333-369 | 同 11 族（`CanExpandOldGeneration`/`ShouldExpandYoungGenerationOnSlowAllocation`）+ #8（AllocateLargePage）; TO_PAGE 旗标/ClearLiveness 已可经 LargePageOps 桥, 随体接入 |

**B 组 · 深水 — Heap::space + 页实体解锁（4 处, 全在 spaces.rs; 对应 spaces.rs 文末台账 1/2）**

| # | 落点 | 原版锚 | 前置码 |
|---|------|--------|--------|
| 1 | `SpaceIterator::has_next` | spaces.h:211 + .cc:48-57 | `Heap::space(int)`（src/heap/heap.h 他票; 本 crate Heap 为 memory_allocator.rs:61 空 enum 承接位） |
| 2 | `SpaceIterator::next_space` | spaces.h:212 + .cc:59-64（DCHECK_LE 实测 cc:60, F7 校正） | 同 #1 + `Space*` 异构返回收敛形（Space trait 含关联类型不可 dyn, 待 enum_dispatch/具体空间单态化） |
| 3 | `MemoryChunkIterator::has_next` | spaces.h:224 + -inl.h:92-102 | #1/#2 |
| 4 | `MemoryChunkIterator::next_chunk` | spaces.h:225 + -inl.h:104-108 | #1/#2 + `MutablePage::list_node().next()`（mutable-page.h 实体他票） |

**C 组 · 即填清零（2 处已灭 — 留档对位映射）**

| # | 落点 | 原版锚 | 处置 |
|---|------|--------|------|
| 10 | `OldLargeObjectSpace::allocate_raw` | large-spaces.h:129-130 + .cc:98-102（cc:101 单行转发） | 已改直调 `self.allocate_raw_executability(…, Executability::NotExecutable, …)`, 域参数与原版逐字对齐; todo! 灭 |
| 14 | `CodeLargeObjectSpace::allocate_raw` | large-spaces.h:183-184 + .cc:417-422（cc:420-421 单行转发） | 已改直调 `old.allocate_raw_executability(…, Executability::Executable, …)`; todo! 灭（运行时终抵 #11 todo 体, 行为不变） |

**D 组 · NoFreeList 占位型（3 处, spaces.rs; 对应 spaces.rs 文末台账与 cc:53-54 nullptr 语义）**

| # | 落点 | 原版锚 | 前置码 |
|---|------|--------|--------|
| 5 | `NoFreeList::allocate` | free-list.h:159-161 | 无解锁计划: 原版 LargeObjectSpace 构造传 nullptr（large-spaces.cc:53-54）, 链面从不消费; 若下波空间需真链, 换具体 FreeList 型而非填此占位 |
| 6 | `NoFreeList::get_page_for_size` | free-list.h:163-165 | 同 #5 |
| 7 | `NoFreeList::select_free_list_category_type` | free-list.h:262-265 | 同 #5 |

**E 组 · DEBUG-only print（1 处）**

| # | 落点 | 原版锚 | 前置码 |
|---|------|--------|--------|
| 9 | `LargeObjectSpace::print`（已归位 impl Space trait 槽） | spaces.h:113（`#ifdef DEBUG` 纯虚）+ large-spaces.h:91-93 + .cc:313-321 | utils/ostreams `i::Print(HeapObject, ostream&)`（他票）; 目标配置 DEBUG 关 → 展开侧剥除, 仅 debug build 可达 |

**登记待定项**: `size_`/`objects_size_`/`capacity_` 字段尾下划线系原版成员名逐字对位（large-spaces.h:109-111/176）, 与 Rust 命名风格冲突但**不强改**（改则破坏 1:1 对位检索）; 定夺归主控裁决。push_back 前置 DCHECK、双存储同步约束已补记 spaces.rs 文末台账 13/6。

## spaces/large_spaces 测试映射

- `tests/spaces_unittest.rs`（11 例）与 `tests/large_spaces_unittest.rs`（19 例）复刻原版断言面; 原版参照 spaces-unittest.cc（1628 行, TEST_F 实测全依赖真 Isolate/Heap → 不可直迁, 申报同 HC-23R 测试口径）, 逐断言注释指原版 file:line 算例:
- spaces 侧: PageList 序语义（list.h:86-90/130-131/30-39/51-57/80 + 未命中 DCHECK should_panic）、PageIteratorImpl 前后置推进快照序（spaces-inl.h:23-34）、for_all 覆盖与零值界（spaces.h:57-62）、Account{,Un}committed 累计 + 回绕/下溢双 should_panic（base-space.h:54-62, checked 形观测力）、first_page_address env 注入 + 空表违例（h:107-110）、Space 基类默认槽组（h:77-105 + cc:53-54 nullptr 语义）。
- large 侧: add/remove_page 三账对偶（cc:188-203, owner 恒等 + objects_size_ 不减惰性面）、update_accounting 舍入差算例（cc:242-254, cps=4096 旗标支 + cc:244 违例 should_panic）、shrink_page_to_object_size 三分支+尾 DCHECK（cc:205-240: A 无缩零调用 / B PartialFreeMemory 双账回退 / C set_area_end 仅收区）、contains/contains_slow（cc:256-272, FromAddress 1<<18 掩码归位）、free_dead_objects 存活重算+摘页安全序+池旗标双支（cc:386-408 + cc:388 should_panic）、flip 调用序（cc:378-384, FROM_PAGE/TO_PAGE）、available 饱和/SetCapacity max（cc:371-376/410-412）、对象迭代器跳 Filler（cc:36-48）、tear_down 清空循环与对账差异金断言（cc:62-72 不减三账）、advance_and_invoke step-128 节奏 0/1/2/3/19/20/1（1:1 复刻 spaces-unittest.cc:160-197 期望数, 宿主换 cc:74-88; 含 cc:76 早退不推进门）、Code::RemovePage 顺序覆写（cc:428-431）、六支 wrapper id（cc:324-440）、转发终抵 todo 体锚定（104-141 vs 333-369 文案区分）。
- 实测勘定: 本仓 `large_page_pool` 缺省 **true**（v8-flags defs.rs:4614 `FlagValue::Bool(true)`）→ free_dead_objects 池旗标双支断言按实态对调定位（缺省支 = kDelayThenPool, cc:391; 显式 false 支 = kImmediately, cc:392）, 原版 cc:390 语境无缺省值叙事, 不构成 src 病灶。

## 转换脚本

`tool/src/heap/`（路径镜像 v8）: memory-chunk.h/.cc/-inl/-layout/-constants、linear-allocation-area.h、allocation-result.h、allocation-stats.h、allocation-observer.h/.cc、memory-pool.h/.cc、free-list.h/.cc 共 14 个。对数基准 clang -E 预处理输出, .skel 产物幂等可 cmp, 断言数只增不减。

## 门禁口径

`.cargo/config.toml` 的 term quiet=true 吞诊断 → 一切门禁命令带 --config 'term.quiet=false'; clippy 一律 --all-targets -- -D warnings; 同 crate 并发席瞬红按面归因豁免, 禁改对席文件。

## .skel 收编台账 (W10HCb 接力, 2026-10-13)

实测盘点基线 (W10HCb 时点原记, raw 口径): src 18 文件 8979 行; 可执行 `todo!` 156 (非-skel 72: main_allocator 50 / spaces 7 / large_spaces 5 系深水, 已在 A-E 台账与本文件尾 HC-2c 台账逐行登记, 禁重填; .skel 84)。`.skel` 草案 78。cargo check 0 error。
  - **W10HCS 实测勘正 (复审 O1/M1 点名, 原记混用 raw/可执行两口径致四数失真)**: 真可执行 `todo!` 现值 = 挂载面 63 (main_allocator **49** (原 50, 访问器已落 1 处) / spaces 7 / large_spaces 5 / memory_balancer 2, 本波挂载) + skel 面 72 = **135**; 原「非-skel 72」系含注释文面 raw 口径且与分项 50/7/5=62 自相矛盾, 作废; 注释/doc 文面提及现测 11 处不计; `.skel` 草案 78 → 77 (slot_set_cc 收编) → **76 件** (memory_balancer 收编); src 现测 19 个 `.rs` 9530 行。逐行登记措辞与 skel 面文案纪律缺口归下波 (见 log W10HCR 复审 O4/M5)。

**第一批收编 (依赖面已就位, 纯逻辑自含) — 挂载 slot_set 族**:

- `slot_set.rs` (原孤儿, 生成器 slot-set.h.js 面) + `slot_set_cc.skel` (生成器 slot-set.cc.js 面) 合并收编入编译面 (lib.rs `pub mod slot_set`), 导入校正 `crate::common::*` → `v8_common::*` + `v8_heap_base::SlotCallbackResult` (cargo add 本 crate 依赖, 归 todo.md L3 DAG heap-core→heap-base 边)。
- 落地面: SlotType/IterationMode 枚举 (slot-set.h:234-266/310)、位域布局常量、机械规则面 (typed_slot_encode/decode_type/decode_offset/cleared_typed_slot/next_capacity/slot_in_ranges)、**TypedSlots/TypedSlotSet 数据结构体 1:1 复刻** slot-set.{h,cc} (insert/ensure_chunk/new_chunk/merge/iterate/clear_invalid_slots/assert_no_invalid_slots/iterate_slots_in_ranges)。原版裸指针分块单链 + std::vector 以 `VecDeque<Chunk>` 头→尾序安全建模, **全程零 unsafe** (miri 触及用例 slot_set 域 **24/24 绿**对账, W10HCS 实测勘正, 原记 4/4 系收编初值)。
- 测试: `tests/slot_set_unittest.rs` 复刻 slot-set-unittest.cc 三支 TypedSlotSet 纯逻辑用例 (Iterate :62 / ClearInvalidSlots :99 / Merge :129) + 机械规则面对偶 1 例, 期望数逐例注释原版算例; slot_set 域现行共 **24 例** (W10HCS 实测勘正, 原记 4 例: 本文件 7 = 收编 4 + 复审席补 3, 另 `slot_set_typed_slots_unittest.rs` 17 例系并发席新增)。
- **slot_set 断言降级口径 (W10HCS 补登, 复审 D2/M2 点名)**: ① `typed_slot_encode` 偏移越界原版 `CHECK` 硬失败 (bit-field.h:100, slot-set.cc:25 消费)、类型越界原版 `DCHECK` (:91) ↔ src 双 `debug_assert` (slot_set.rs 编码面) — **release 构建剥除掩蔽, 与原版致命语义差**, 违例封锁仅在 debug 域; ② `assert_no_invalid_slots` 原版 `CHECK_WITH_MSG` 恒致命 (slot-set.cc:72) ↔ src `debug_assert` 同上降级; 两面违例 should_panic 用例已在测试面 (typed_slots 双 target), 本行为语义差唯一台账位。
- **slot_set 内部推迟 (强挂约束)**: ① `SlotSet` 页位图类 (slot-set.h:35-122 PossiblyEmptyBuckets + :127-232 SlotSet, WordsForBuckets/BucketsForSize; W10HCS 勘正 M4-⑤, 原记 h:60-231 失实, src 头推迟注已对位) → 依赖页 bitmap/MemoryChunk 实体, 其位图基座已由 v8-heap-base basic_slot_set.rs 落地覆盖, 本 crate slot_set 不挂该类 (原版 slot-set-unittest.cc:18/:40/:48 三例归该面); ② TypedSlotSet 并发原子迭代 (LoadHead/StoreNext/LoadNext relaxed atomic, GC 线程并发读) → 本模块取单线程语义, 并发面待页实体票接入。

**不强挂 (页实体/运行态依赖未落, 分组登记; W10HCS 勘正: `Isolate`/`Tagged<HeapObject>`/`TimeDelta`/`TimeTicks` 类型面均已落地, 下表前置按实态重核; memory_balancer 已随本勘正挂载出组)**:

| 组 | 落点 .skel | 符号 | 前置 | 解锁 crate |
|---|-----------|------|------|-----------|
| skel-F | mutable_page_cc.skel (34 todo!) | MutablePage 释放序方法体 / 页旗标计算 (Old/YoungGenerationPageFlags) | MutablePage/RawPage 实体 + RememberedSet 实体 (SlotSet/TypedSlotSet 页侧) | heap-core 页实体票 (HC-4 二波) |
| skel-F | object_lock.rs.skel (3) | ObjectLock::Lock/Unlock, ObjectLockGuard::Drop | 真实前置 = `MutablePage::FromHeapObject` 实体 (heap/object-lock-inl.h:19/:25 消费位, mutable-page.h 未挂) + 页 `object_mutex`; `Isolate`/`Tagged<HeapObject>` **已落地** (W10HCS 勘正 O1, 原「实体未落」行文失实: v8-execution-isolate/src/isolate.rs:16 / v8-objects-core/src/tagged.rs:33 / heap_object.rs:8) | heap-core 页实体票 (HC-4 二波) + execution-isolate 依赖边 |
| skel-F | pending_allocations.rs.skel (7) | YoungPendingAllocations::{contains_synchronized,update_lab,remove_lab,update_large_object,remove_large_object,update_snapshot,reset_version} (Snapshot::contains 纯面可随挂) | `Heap` 实体 mutex + 版本失效运行态 | heap-core Heap 实体票 |
| ~~skel-F~~ 已挂载 | memory_balancer.rs (**W10HCS 收编出组**, 原 skel 4 项) | update_allocation_rate (cc:74-84) / update_gc_speed (cc:63-72) **已 1:1 填体** (纯 EWMA, 方法体零 heap_ 引用); heartbeat_update (cc:86-100) / recompute_limits (cc:17-25) 留 todo! | `TimeDelta`/`TimeTicks` **已落地** (W10HCS 勘正 D1, 原「依赖未落」失实: v8-base-platform time.rs:51/:607 + lib.rs:54); 剩余两件真前置 = Heap 运行态 `OldGenerationSizeOfObjects`/`limits()`/`GetForegroundTaskRunner` (cc:21/:44/:105) + `CancelableTask` 任务面 (h:87-97/cc:109-112) | heap-gc / Heap 实体票 |
| skel-F | read_only_heap.rs.skel (3) | ReadOnlyHeap 空间装载/迭代 | `ReadOnlySpace` 实体 + Isolate snapshot 装载态 | heap-core read-only-spaces (HC-4 二波) |
| skel-F | memory_measurement.rs.skel (4) | MemoryMeasurement::{enqueue_request,start_processing,finish_processing,default_delegate} | `Isolate`/`v8::Local<Context>`/`Promise::Resolver`/task_runner (measureMemory API 面) | v8-api / execution-isolate |

**脚本核查口径 (task 4)**: slot-set.h.js / slot-set.cc.js 属本族基线生成器, 输出为 `slot_set.rs` 枚举面 / `slot_set_cc.skel` 机械骨架。收编后 `slot_set.rs` 系 Developer 就地修订语义的挂载态 (数据结构体不可规则提取), 依 todo.md 十一「复跑仅限重生成基线, 修订就地改」——**复跑生成器以 src 现态为准, 不覆写挂载 module**; 生成器输出含 `/// v8: src/heap/slot-set.*` 锚 (幂等确定性)。本席未复跑 slot-set 生成器 (避覆写挂载面), 其余未触。

## 接力席位 W10HCb 收笼状态 (2026-10-13)

- 挂载 slot_set 族 (1 .skel 批) + lib.rs 注册 + cargo add v8-heap-base 依赖边; 深水 .skel (mutable_page/object_lock/pending_allocations/memory_balancer/read_only_heap/memory_measurement) 依上表分组不强挂。
- 门禁实测: `cargo clippy -p v8-heap-core --all-targets -- -D warnings` 0 警告; `cargo nextest run -p v8-heap-core` 收编初值 98/98 → **118/118 绿** (W10HCS 实测勘正, 复审席 +3 例与并发席 typed_slots +17 例后 slot_set 域 24 例, 全 crate 118; W10HCS 挂 memory_balancer 后 126/126, 见下文勘正节); `cargo +nightly fmt -p v8-heap-core -- --check` 净; `cargo +nightly miri nextest run -p v8-heap-core --test slot_set_unittest` 初值 4/4 → slot_set 双 target **24/24 绿** (W10HCS 实测勘正; 挂载零 unsafe, 对账)。
- todo! 净变化: 可执行面 (rs+skel 广计) **156 → 148** (收编灭 slot_set_cc.skel 8 处草稿骨架 todo!, 其内容并入挂载 `slot_set.rs`); 非-skel 深水 72 (含 main_allocator 49, W10HCS 实测勘正原记 50, 访问器已落地 1 处 / spaces+large 注释+体) 无动, 台账齐全。`.skel` 草案 78 → 77 → 76 (W10HCS 收编 memory_balancer.rs.skel 出编译面)。

## W10HCS round1 整改勘正 (2026-10-13)

- **memory_balancer 收编 (R3/D1 处置)**: `memory_balancer.rs.skel` → `memory_balancer.rs` 挂入 lib.rs。前置 `TimeDelta`/`TimeTicks` 经核已落 (v8-base-platform time.rs:51/:607, lib.rs:54 再导出, in_milliseconds_f time.rs:175), 原「依赖未落」判定失实已勘正。填体三符号 1:1: `new` (cc:14-15 纯存储 ctor, 测试构造面)、`update_allocation_rate` (cc:74-84)、`update_gc_speed` (cc:63-72) — 方法体零 `heap_` 引用纯 EWMA, 双 optional 字段 (h:77/:78) 与缺省初值 (h:63/:74/:82/:84) 补齐对位。推迟两件真前置 = Heap 运行态: `heartbeat_update` (cc:86-100, OldGenerationSizeOfObjects/TimeTicks::Now/RefreshLimit/PostHeartbeatTask) / `recompute_limits` (cc:17-25, OldGenerationSizeOfObjects + limits() cc:44/:46/:59); RefreshLimit (cc:27-61) / PostHeartbeatTask (cc:102-107) / HeartbeatTask (h:87-97 + cc:109-112) 不入签名面, 归 heap-gc/Heap 实体票。测试 `tests/memory_balancer_unittest.rs` 8 例 (原版无 memory-balancer 专名 unittest, v8/test/unittests 全树 grep 零命中 → 等价覆盖口径同 HC-23R/HC-2b 申报, 逐断言注 cc/h 算例)。收编后挂载面可执行 `todo!` 实测 63 (main_allocator 49 / spaces 7 / large_spaces 5 / memory_balancer 2), `.skel` 面 72。
- **object_lock 勘正 (O1/R3 处置, 未挂载)**: Lock/Unlock/Drop 三 todo! 体全部经 `MutablePage::FromHeapObject(isolate, heap_object)->object_mutex()` (heap/object-lock-inl.h:19/:25), MutablePage 实体未挂 → 推迟结论成立, 但原台账「`Isolate`/`Tagged<HeapObject>` 实体未落」行文失实已按实态勘正 (isolate.rs:16 / tagged.rs:33 / heap_object.rs:8 已落), 解锁面据实归 HC-4 页实体票 + execution-isolate 依赖边。
- **SpaceWithLinearArea 一名一主冲突登记 (复审 O5/M4 点名, 本波只登记不裁决)**: `main_allocator.rs:63 pub enum SpaceWithLinearArea {}` (h:27 前置声明 opaque 承接) 与 `spaces.rs:594 pub trait SpaceWithLinearArea: Space` (spaces.h 实体面) 双主同名 → HC-2c 填 ctor 前归收敛票定夺 (跨票面二义, 本票禁碰对席文件未并一)。
- **MainAllocator `allocation_info_` 自指裸指针约束登记 (R4/M3 处置)**: 原版 cc:48-49 三目 `allocation_info_ = allocation_info ? allocation_info : &owned_allocation_info_` (h:344 指针 + h:345-346 自持位) 直译入 `main_allocator.rs` 后, `allocation_info_` 可指向**同结构体内** `owned_allocation_info_` → **构造后禁 move/禁按值转移 MainAllocator** (Rust move = 陈旧指针, 后续 7 处 LAB 解引用位 start/top/limit/allocation_top_address/allocation_limit_address/extend_lab 面即悬垂读)。约束承接: ① ctor 填体票 (`new` :516 / `new_in_gc` :527 现 todo!, 不可达期) 必须落「owned 支取 &mut owned_allocation_info_ 后置指针」并以本约束为 SAFETY 前提, 或改 Box/间接持有 owned 位使约束显式化 (填体票裁决, 最小侵入); ② 实现面实测无 `Vec<MainAllocator>`/按值存储移动路径 (全仓 grep 零命中), 暂不触达 debug_assert 补救面; ③ HC-2c 填体票开工前以本节为深水约束, 违则引入真 UB 面 (miri 现覆盖不到)。
- **W10HCS 门禁 (CARGO_TARGET_DIR=target/agent_W10HCS)**: 见 log/v8-heap-core.md「W10HCS round1 整改」节实录。

