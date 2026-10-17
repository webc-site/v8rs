# v8-heap-gc 开发日志

> 2026-10-12 启动: V8 分代垃圾回收 (GC) 核心算法纯语义与规则面全线收敛

## 一、架构设计与模块组织

- `v8-heap-gc/src/lib.rs`:
  - 顶层声明 `#![forbid(unsafe_code)]`，全库实现 100% Safe Rust；
  - 扁平导出所有 GC 核心子模块的纯算法与规则接口，杜绝二次导出。
- `heap_layout.rs`:
  - 定义页大小（256KB，`K_PAGE_SIZE_BITS = 18`）、页内掩码与常规堆对象上限（128KB）；
  - 定义对齐掩码、Tagged 宽度与位图单元全 1 常量。
- `marking.rs`:
  - `MarkingBitmap`：页标记位图（65536 位，1024 个 64 位单元，总计 8192 字节）；
  - `MarkBit`：单标记位（单元索引 + 位掩码）；
  - 支持单位读写、范围标记 (`set_range`)、范围清空 (`clear_range`)、范围全 1/全 0 判别及位图清洁度检查 (`is_clean`)；
  - 导出 `WorklistTarget`、`LivenessMode` 及地址/索引映射算术纯函数。
- `marking_progress_tracker.rs`:
  - 大对象分块进度追踪器（分块粒度 128KB）；
  - 线程安全认领下一分块 (`get_next_chunk_to_mark`)、总分块查询与启用控制。
- `gc_tracer.rs` & `gc_tracer_cc.rs`:
  - `ScopeId`：166 项追踪作用域全枚举；
  - `EventType`、`EventState`、`Mode`、`MarkingType`、`Priority` 标准 UpperCamelCase 枚举与名称双向表映射；
  - 提供平均速度与吞吐量有界性函数（`bounded_average_speed`, `bounded_throughput`）。
- `incremental_marking.rs` & `incremental_marking_job.rs`:
  - `StepOrigin`（`V8` / `Task`）及最大步进时长算术；
  - 激活阈值判别纯函数 (`is_below_activation_thresholds`) 与前台调度任务骨架。
- `mark_compact_enums.rs` & `mark_compact.rs`:
  - 压缩回收状态机枚举、疏散模式计算与常量规范。
- `memory_reducer_enums.rs` & `memory_reducer.rs`:
  - 内存缩减器状态机 (`Uninit` / `Done` / `Wait` / `Run`) 纯函数 `step` 状态转移；
  - 看门狗超时 (`watchdog_gc`)、增长因子（1.1）与增量门限判定。
- `minor_mark_sweep.rs`: 次清扫并行任务上限与槽位类型枚举。
- `sweeper_enums.rs` & `sweeper.rs`:
  - 清扫空间常量（`K_NEW_SPACE` 到 `K_SHARED_TRUSTED_SPACE`）；
  - 填充字常量（`K_ZAP_TAGGED = 0xbeadbeef`）、并发度上限计算 (`get_max_concurrency`) 与死对象处置模式选择 (`should_zap_dead_objects_on_page`)。
- `scavenger.rs`: 新生代清除器任务数计算与对象年龄枚举。
- `tests/heap_gc.rs`: 全覆盖单元测试与集成测试（26/26 passed）。

## 二、复审环日志

- **Round 1 (全新独立 Reviewer, 目标 clean 1/3)**:
  - 逐行比对原版源码 `v8/src/heap/` (mark-compact.{h,cc}, scavenger.{h,cc}, sweeper.{h,cc}) 与 Rust 实现 `v8-heap-gc/src/`：
    - 疏散候选页选择算法 (`select_evacuation_candidates`) 与防 compact-expand 零净减震荡逻辑与 V8 1:1 语义等价；
    - 年龄线判定 (`is_address_below_age_mark`) 判定序与页界判定与 upstream 完全一致；
    - 清除状态机 (`SweepingStateMachine`) 状态流转完备闭环，补齐 `stop_concurrent_sweeping`（对齐 cc:303）；
    - 健壮性优化：`compute_free_bytes_threshold` 负值门限安全截断；`is_address_below_age_mark` 与 `get_max_concurrency` 算术使用 `saturating_add` 规避边界溢出；`MarkingBitmap` 范围操作自动截断至 `K_LENGTH` 防越界 panic；
    - 门禁验证：`CARGO_TARGET_DIR=target/agent_w8hgcr cargo nextest run -p v8-heap-gc` 39/39 全通；`cargo clippy --all-targets -- -D warnings` 0 警告；`rg "\[allow"` 0 绕过；
  - 达成 clean 1/3。
- **Round 2 (全新独立 Reviewer, 目标 clean 2/3)**:
  - 严格对照 `.agents/skills/rust_review/SKILL.md` 12 项原则全面复审：
    - 极致性能与零拷贝：`select_evacuation_candidates` 单次遍历过滤 + 配额超限提前 break，切片零拷贝切分；`MarkingBitmap` 栈数组零堆分配，范围操作按 cell 单次批量更新；
    - 语义等价核对：逐行重审 `v8/src/heap/` 对应 C++ 源码：
      - `mark-compact.cc:649-789`: 空闲门限筛选、升序贪心、防 compact->expand 零释放震荡等价闭环，且在累计活字节时强化 `saturating_add` 规避 `usize` 溢出；
      - `scavenger.cc:1579-1601` & `new-spaces-inl.h:80`: 任务上限计算、`is_address_below_age_mark` 边界判别序与页内 limit 判定 1:1 等价；
      - `sweeper.cc:281-393`: `SweepingStateMachine` 并发清扫全生命周期、偶奇 trace ID 派生与 `can_allocate_on_page` 准入判定完备闭环；
      - `marking.cc`: 位图边界安全截断至 `K_LENGTH`，彻底杜绝越界 panic；
    - 规范与门禁验证：
      - `CARGO_TARGET_DIR=target/agent_w8hgcr2 cargo nextest run -p v8-heap-gc` → 39/39 100% 通过；
      - `CARGO_TARGET_DIR=target/agent_w8hgcr2 cargo clippy -p v8-heap-gc --all-targets -- -D warnings` → 0 警告；
      - `rg "\[allow" v8-heap-gc/src/ v8-heap-gc/tests/` → 0 绕过；
      - 全库 `#![forbid(unsafe_code)]`，0 panic，0 unsafe；
  - 结论：0 语义偏差、0 缺陷、0 遗漏，达成 clean 2/3。
- **Round 3 (全新独立终验 Reviewer, 目标 clean 3/3 终检验收冻结)**:
  - 严格对照 `.agents/skills/rust_review/SKILL.md` 12 项原则逐项终验：
    - 极致性能与零拷贝：
      - `select_evacuation_candidates` 单次遍历过滤 + `sort_unstable_by_key` 排序 + 配额超限提前 break 截断，切片切分零额外拷贝；
      - `MarkingBitmap` 栈固定数组 `[MarkBitCell; K_CELLS_COUNT]` 零堆分配，范围操作按 cell 切片单次批量操作；
      - 纯标量传递与常量计算，0 不必要 clone，0 Arc，无锁状态机无阻塞；
    - 源码逐行交叉比对：
      - `mark-compact.cc:649-789`: 空闲门限计算、升序贪心纳选、`saturating_add` 防累加溢出、防 compact-expand 零净减震荡中止策略与 upstream 1:1 严格一致；
      - `scavenger.cc:1579-1601` & `new-spaces-inl.h:80`: 任务上限计算、`is_address_below_age_mark` 边界判别序与页内 limit 判定完全等价；
      - `sweeper.cc:281-393`: `SweepingStateMachine` 封装 Idle -> Initialized -> Sweeping (Active/Paused) -> Joined -> Finished 完整生命周期，`compute_sweeper_trace_ids` 偶奇派生与 `can_allocate_on_page` 准入判定严格闭环；
      - `marking.cc:17-68` & `marking-inl.h:88-140`: 位图 `set_range`/`clear_range`/`all_bits_set_in_range`/`all_bits_clear_in_range` 边界安全钳位至 `K_LENGTH`，彻底杜绝越界 panic；
    - 门禁与规范验证：
      - `CARGO_TARGET_DIR=target/agent_w8hgcr3 cargo nextest run -p v8-heap-gc` → 39/39 100% 通过（耗时 0.042s）；
      - `CARGO_TARGET_DIR=target/agent_w8hgcr3 cargo clippy -p v8-heap-gc --all-targets -- -D warnings` → 0 警告；
      - `rg "\[allow" v8-heap-gc/` → 0 绕过；
      - 全库 `#![forbid(unsafe_code)]`，0 panic，0 unsafe；
  - 终验结论：0 语义偏差、0 规范缺陷、0 遗漏，测试全部通过，达成 clean 3/3 终检验收冻结。

- `[轮7 W7HGC]` 开工基线: mounted .rs 0 todo!, 208 todo! 全在 .skel 草案, cargo check 绿。
  - **skel 收编对账**: 14 个 .skel 全部核对 → 重复(已挂纯面)跳过 10 个
    (mark_compact_cc/sweeper_cc/memory_reducer_cc/incremental_marking_cc/marking/
    gc_tracer_cc/minor_mark_sweep/scavenger/marking_progress_tracker/incremental_marking_job);
    净新深水面引用未定义对象模型类型 (Heap/Isolate/Tagged/NormalPage/Space/Visitor),
    挂载即破编译 → 登记推迟台账 4 个 (marking_barrier/marking_state/marking_worklist/marking_visitor)。
  - **填体启动 (1:1)**: 在 mark_compact.rs 新增 `compute_evacuation_heuristics`
    (cc:596 纯规则核, 标量入参替 Heap*, 保留 reduce>optimize>常规判定序 + trace 估速),
    含编译期不变式与 3 个单测; flag 值以常量占位登记台账。
  - 推迟台账: 落 `readme/zh.md` (基线 208 todo 全表 + 按 cc 锚点方法面清单)。
  - 门禁: `cargo nextest run -p v8-heap-gc` → 29/29 passed。
  - todo! 前后: 挂载面 0→0 (未强挂), 草案登记 208 (推迟); 新增纯函数体 1。

- `[轮8 W8HGC]` 核心算法填体与规范收敛:
  - **Clippy 告警修复**: 修复 `mark_compact.rs` 中的 `clippy::if_same_then_else`（合并 `reduce_memory || optimize_memory`）。
  - **Tool 脚本对齐**: 规范化 `tool/src/heap/` 转换脚本（`scavenger.js`, `sweeper.cc.js`, `mark-compact.cc.js`），枚举全员对齐 UpperCamelCase 规范，消除 skel 冗余 allow。
  - **mark_compact 疏散候选策略纯逻辑填体**:
    - 引入 `compute_free_bytes_threshold`、`estimated_new_pages`、`EvacuationCandidatePage` 与 `EvacuationSelectionResult`；
    - 1:1 转写 `MarkCompactCollector::CollectEvacuationCandidates`（cc:649-789）：空闲空间门限筛选、升序贪心纳选、防 compact->expand 零释放震荡截断。
  - **scavenger 拷贝与晋升算法纯逻辑填体**:
    - 引入 `is_address_below_age_mark`（1:1 对齐 `SemiSpaceNewSpace::ShouldBePromoted` / `IsAddressBelowAgeMark`）；
    - 引入 `ScavengePreference`、`determine_scavenge_preference`、`MigrationOutcome`、`ScavengerTaskStats` 与分段就地排空门限。
  - **sweeper 并发清除状态机纯逻辑填体**:
    - 引入 `compute_sweeper_trace_ids`（1:1 对齐 cc:317-320 奇偶追踪 ID 派生）；
    - 引入 `PageSweepingState` 页清扫状态与分配准入判定；
    - 引入 `SweepingStateMachine`，完备建模 `Idle -> Initialized -> Sweeping (Active/Paused) -> Joined -> Finished` 闭环转移。
  - **错误处理与无 Allow 审查**:
    - `error.rs` 定义结构化 `InvalidSweepingState` 错误，全库 0 panic，0 unsafe，0 `#[allow]` 绕过；
  - **门禁验证**:
    - `CARGO_TARGET_DIR=target/agent_w8hgc cargo nextest run -p v8-heap-gc` → 39/39 100% 通过；
    - `CARGO_TARGET_DIR=target/agent_w8hgc cargo clippy -p v8-heap-gc --all-targets -- -D warnings` → 0 警告；
    - `rg "\[allow" v8-heap-gc/src` → 0 匹配。
  - **推迟台账更新**: 更新 `readme/zh.md` 记录本轮收编与深水面推迟项。

- `[轮10 W10HG]` 接力: 差集补测 + 台账分组 + 脚本核查 (不强挂对象模型深水, 保持 crate 恒可编译):
  - **实测盘点 (三件套)**: `wc -l src` = 3062 行 (16 挂载 .rs + 14 .skel); 挂载 `.rs` 0 `todo!`; 208 `todo!` 全在 `.skel` 草案。票面 per-module 计数陈旧 (实测 mark_compact_cc 97 / sweeper_cc 70 / memory_reducer_cc 13 / marking_barrier 14 / minor_mark_sweep 4 / marking_state 5 / marking_worklist 3 / scavenger 2)。`cargo check` 基线绿。
  - **依赖面判定**: 上游 `v8-heap-core`/`v8-objects-core` 的 `Heap`/`Tagged`/`NormalPage`/`MutablePage`/`SlotSet`/`Space`/`ObjectVisitor` 仍仅存于各自 `.skel` (未冻结); 本 crate `#![forbid(unsafe_code)]` 无这些类型。逐一核对 8 个含 todo! 的 skel → 208 体全为对象模型/平台耦合方法体 (引用未定义类型), 挂载即破编译 → 维持不强挂, 与前任告一致; 纯逻辑核已在轮 7/8 全数挂载, 无遗漏差集可填, 未强造。
  - **台账分组**: `readme/zh.md` 新增「推迟台账分组」表 (组|符号|前置|解锁 crate|todo!), 11 组共 208, 解锁条件写明 v8-heap-core/v8-objects-core/v8-base-platform/v8-roots 具体面。
  - **测试 1:1 迁移**: 从 `unittests/heap/{marking-bitmap,marking-progress-tracker,memory-reducer}-unittest.cc` 转写 25 例并入 `tests/{marking_bitmap,marking_progress_tracker,memory_reducer}.rs`, 保留原 gtest 用例名, `#[ctor::ctor(unsafe)]` log init, 返回 `aok::Void` 以 `OK` 结尾 (memory-reducer flag 默认值参数化 long=8000/start=30000/max_gcs=2, 含 factor 临界 f64 用例)。
  - **脚本核查**: gc 族 14 个 `.skel` 写手复跑幂等 13/14; `marking.js` 重生成 stale allow 行按「src 现态为准」恢复草案未覆写; 4 个 `.h.js` 直写已挂枚举 `.rs` 依规则不执行 (防退 UpperCamelCase); 未触碰他席 crate 脚本与 `tool/lib.js`。
  - **门禁**: `CARGO_TARGET_DIR=target/agent_W10HG cargo nextest run -p v8-heap-gc` → 39→64 全绿; `cargo clippy -p v8-heap-gc --all-targets -- -D warnings` → 0 警告; `rg "\[allow" src tests` → 0 绕过; `#![forbid(unsafe_code)]` 保持, 本轮 0 unsafe 改动 → Miri 非触及 (A 档要求为「有 unsafe」, 本 crate 全安全, 无需跑, 未新增 unsafe 亦不改档)。
  - **收尾数**: todo! 基线 208 → 现 208 (全对象模型推迟, 挂载面恒 0); skel 收编 0 强挂 (纯面已尽, 差集=测/台账/脚本); 挂载 .rs 16 文件不变; nextest 64 通过; clippy 0; 台账分组 11 组。


---

## W11HGR 复审 (2026-10-13, round1, 席 W11HGR)

- **票面复核**: nextest 64/64 绿、clippy `-D warnings` RC=0 复跑确认 (一次瞬时 "workspace.edition not defined" 为他席并发改根 Cargo.toml, 复跑即过); skel 实测每文件 97/70/14/13/5/4/3/2 + 其余 0 = 208 ✓, 与轮9 收编对账表逐行一致; 挂载 .rs 0 todo ✓。
- **1:1 复审**: marking/marking_bitmap/progress-tracker/memory-reducer/scavenger/sweeper/gc_tracer (ScopeId X-macro 展开 166 条逐字节对)/incremental-marking 纯面逐项对 vendored v8 复核一致; K_ZAP_TAGGED = static_cast<Tagged_t>(kZapValue) 截断正确。
- **缺陷 3 (全修)**:
  - D1 `memory_reducer.rs::step()` DONE/UNINIT×MarkCompact 阈值: 原码未复现原版 `static_cast<size_t>` 截断 + 整数域 `std::max` (memory-reducer.cc:181-188), 已改截断后整数比较。
  - D2 `sweeper.rs` AllocationSpace 判别值错位: 曾臆造 CODE_MAP_SPACE=6 且 SHARED/TRUSTED/LO 缺失; 按 globals.h:1488-1513 修正 (NEW=1…SHARED_TRUSTED=6, LO=8), 删除臆造常数, 加编译期邻接断言; lib.rs 再导出同步。
  - D3 `readme/zh.md` 推迟台账分组表 todo 列和 196 ≠ 自称合计 208: 按 skel 锚点实测拆分 mark_compact_cc 97 = collector 82 + 内联 visitor 11 (RootMarkingVisitor 6 + Evacuator 3 + LiveObjectVisitor 2) + recordslot 4, 校正 60→82、21→11 并留复审注; 现列和 = 208。
- **遗漏 2 (登记不阻断)**:
  - `get_max_duration_ms` 未建模原版 `v8_flags.predictable → TimeDelta::Max()` 首分支 (incremental-marking.cc:70-72, flag 运行值域无哨兵); 已补 doc 注释声明。
  - `compute_evacuation_heuristics` reduce/optimize 合并分支用占位常数 (0% / 1024MB), 原版 flag 默认 20% 与 12/6/4MB 分模; 注释已自述「占位 (原值取 flag)」, 属参数化占位面。
- **25 测试审计 (W10HG 轮9)**: 无弱化断言; factor 临界例与阈值式同式自洽, 整数域截断语义相符。
- **新增测试 5**: `tests/memory_reducer_committed_memory.rs` (平局 100MB 阈值=last+delta 恰相等; 截断卡进 last=1000MB+1 → 阈值 1153433601, 整数域 vs 实数域可判别) + `tests/sweeper_space_ids.rs` (判别值/count/kZapTagged 截断 3 例, D2 防回归)。aok::Void/OK + `#[ctor::ctor(unsafe)]` log_init, 无 round*/tmp/zz_ 文件名。
- **unsafe 审计**: `#![forbid(unsafe_code)]` 保持; src/tests 0 实 unsafe (unwrap 仅 src #[cfg(test)] 块); Miri 无适用面, 未跑 (与 W10HG 先例一致)。
- **门禁**: `CARGO_TARGET_DIR=target/agent_W11HGR cargo nextest run -p v8-heap-gc` → **69/69 全绿 RC=0** (基线 64+5); `cargo clippy -p v8-heap-gc --all-targets -- -D warnings` → **RC=0**; readme 修订后 110 行。
- **改 src**: 是 — memory_reducer.rs (阈值语义)、sweeper.rs (判别值+断言)、lib.rs (再导出同步)、incremental_marking.rs (doc 注释)、readme 台账; 均最小修, 无无关重构, 未碰他 crate。

---

## W11HGR2 round2 复审 (2026-10-13, 席 W11HGR2)

- **指纹判定**: 开局 `cat v8-heap-gc/src/*.rs | shasum -a 256` = **1592a924c73b422e**f8e387d734bc41a4… 与锚一致 → **无漂移, 入账复审**。审毕现态 = 3adfef602412f5c6… (差异全部为本轮有意修复, 见「改 src」)。
- **三数 (复核 + 现态)**: skel `todo!` 实测 97/70/14/13/5/4/3/2 + 其余 0 = **208 不变**; 挂载 .rs todo **0**; readme 分组表列和实再算 (11 行) = **208** ✓。
- **D1/D2/D3 回归复核 (无回归)**:
  - D1: `step()` DONE/UNINIT×MarkCompact 阈值逐运算序对 memory-reducer.cc:180-188 — f64 乘→`as usize` 向零截断 / delta 饱和加 / 整数域 max / `<` 保留方向, 全符; watchdog/短延迟常量对 cc:41-42 ✓。
  - D2: 对 globals.h:1489-1513 **全枚举**逐值 — RO=0/NEW=1/OLD=2/CODE=3/SHARED=4/TRUSTED=5/SHARED_TRUSTED=6/(NEW_LO=7 跳槽)/LO=8 ✓; `kZapValue` 64 位支 0xdeadbeedbeadbeef (globals.h:1153) 截 Tagged_t 低 32 = 0xbeadbeef ✓; `kNumberOfSweepingSpaces = LAST-FIRST+1` (sweeper.h:207) = 6 ✓; trace_id 奇偶式对 cc:317-320 ✓。
  - D3: 台账列和实再算 = 208 (round1 校正值维持)。
- **挂载件逐行 (round1 未深攻面)**:
  - `marking.rs` 对 marking.h:96-141 / marking-inl.h:18-140 / marking.cc:13-137 逐式 1:1 — 位图 word 边界三式 `!(start_mask-1)` / `end_mask|(end_mask-1)` / 同 cell `end_mask|(end_mask-start_mask)` 与原版逐字节同; IndexToCell/IndexInCellMask/CellAlignIndex/AddressToIndex/LimitAddressToIndex 及布局 (kPageSizeBits=18 build_config.h:80 默认支 / kTaggedSizeLog2=2 globals.h:581 / kAlignmentMask memory-chunk.h:306-307) 全对。**夹逼偏离**: 原版 SetRange/ClearRange/AllBits* 无 min 夹逼 (调用方保证域), 本面 `min(kLength)` OOB 安全化, 域内行为一致 → 已在模块 doc 补「夹逼偏离注」。
  - `heap_layout.rs`: 6 常量 + 断言逐值对原版 ✓。
  - `gc_tracer.rs`: 背景段尾 149-165 与 heap-symbols.h:1157-1178 逐条对 (含 MC_CLEAR_STRING_TABLE=158 混排位) ✓; Event::Type 0-5 / State 0-3 (gc-tracer.h:189 NOT_RUNNING 首位) / Mode 0-2 / MarkingType 0-1 / YoungGenerationSpeedMode 0-1 (gc-tracer.h:24) 判别值全符。
  - `gc_tracer_cc.rs`: 类型双名表/优先级三名表对 gc-tracer.cc:110-125/85-95 ✓; 速度界 clamp[1, GB] / min(GB) 对 cc:58-69, `GB`=v8-internal.h:43 (1024³) ✓。
- **缺陷 2 (本轮新发现, 均就地修, 非 round1 三缺陷回归)**:
  - **W2D1** `gc_tracer_cc.rs::Priority` 判别值与原版**倒置**: 曾 UserBlocking=0/BestEffort=2, 原版 v8-isolate.h:698-708 为 kBestEffort=0 < kUserVisible=1 < kUserBlocking=2 → 按原序改 + 编译期断言 3 条。
  - **W2D2** `event_state_to_string(NotRunning)` 臆造名 "Not running": 原版 gc-tracer.cc:80-81 NOT_RUNNING 返 **nullptr** (无名) → 返回类型改 `Option<&'static str>` (NotRunning→None), 同改 src 单测与 `tests/heap_gc.rs` 断言; lib.rs 再导出按名透传未动。
  - 防回归: 新增 `tests/gc_tracer_names.rs` 3 例 (判别值升序 / 状态名含 None / 双名表全枚举), 注入值口径无时钟 → 无 flaky。
- **金标准对数补迁**: `tests/memory_reducer.rs::from_wait_to_wait` 对 memory-reducer-unittest.cc:138-193 逐步核出 **2 处迁移缺口** (round1 25 例审计漏): 缺第 6 步 `MarkCompactEventNoGarbageLeft(2000,0)` 全断言组、缺末步 next_gc_start_ms/started_gcs 断言 → 已补; marking-progress-tracker 7 例逐步核无缺 (DeathTest 2 例维持记跳)。
- **skel 台账换面抽 4 组** (避开 round1 collector/visitor 面): marking-barrier(14) / memory-reducer-heap(13) / minor-mark-sweep(4) / marking-worklist(3) 现场对上游现态 grep (只读):
  - **无谎登组** (四组 skel 体引用的硬前置确仍未落, 挂载即破编译结论不变, 208 维持)。
  - **3 组前置措辞过期** (保守向失实, 已校正): objects-core 已挂 `tagged.rs`(Tagged/Object)/`heap_object.rs`/`map.rs`/`visitors.rs`(ObjectVisitor/RootVisitor); heap-core 已挂 19 .rs 含 `memory_balancer.rs`/`memory_chunk.rs`/`spaces.rs`/`slot_set.rs`(TypedSlots/TypedSlotSet)/`free_list.rs`; `base::Worklist` 已挂 `v8-heap-base/src/worklist.rs` (0 todo)。真阻塞收敛为: `Heap`/`LocalHeap` (皆 .skel)、`MutablePage`/`NormalPage` (仅 opaque 空桩)、位图页类 `SlotSet`、`JobHandle`/延时任务。W10HG「同名类型仅存 .skel」段与 marking-barrier/minor-mark-sweep/marking-worklist 三行已在 readme 就地校正 + 引用块留痕。
- **队列面**: vendored 原版树无 `scavenger-queue.h` (并行 scavenge 走 heap/base worklist); 本 crate 无队列语义挂载 (scavenger.rs 无 wrap-around/overflow 面) → 判 **N/A**, 推迟登记维持 marking-worklist 组。
- **round1 遗漏追踪**: `get_max_duration_ms` predictable 补注已落实 (incremental_marking.rs:32-33) ✓; `compute_evacuation_heuristics` 占位常数维持登记。
- **失败敏感性 (3 例闭环)**: M1 set_range 起始掩码 `-1→+1` → `marking_bitmap::set_and_clear_range` **FAIL**; M2 阈值 `<→<=` → from_done_to_wait + 阈值 tie/truncation **3 FAIL**; M3 GetMaxDuration 双臂互换 → const 断言 E0080 **编译红** (fail-closed)。三例均恢复并经 sha256 与变异前快照比对 **RESTORED/ALL_RESTORED**。⚠ 门禁期间观测到有外部写手进程并发改写 marking.rs/memory_reducer.rs/incremental_marking.rs (早于恢复步抢回原文), 终态已逐文件哈希核验, 本轮有意修改 (gc_tracer_cc.rs 等) 均存活。
- **门禁**: `CARGO_TARGET_DIR=target/agent_W11HGR2 cargo nextest run -p v8-heap-gc` → **72/72 全绿 RC=0** (基线 69+3 新增); `cargo clippy -p v8-heap-gc --all-targets -- -D warnings` → **RC=0** 0 警; `#![forbid(unsafe_code)]` 保持, 挂载面 0 unsafe / 0 `[allow`; readme 分组列和 208。
- **改 src**: 是 — gc_tracer_cc.rs (W2D1/W2D2 修 + 断言 + 单测随改)、marking.rs (夹逼偏离 doc 注); 改 tests — heap_gc.rs (None 断言)、memory_reducer.rs (补迁 2 处)、新增 gc_tracer_names.rs; 改 readme/zh.md (现态校正)。未动 lib.rs / 他 crate / tool/lib.js; 未 git 操作。
- **结论**: 基线三数与 D1/D2/D3 修复复核**无回归**; 新发现缺陷 2 (全修, 无遗留 must-fix) + 迁移缺口 1 (已补) + 台账失实措辞 3 组 (已校正) → **非 0 发现轮, 不判 round2 clean**; 门禁全绿。

---

## W11HGR3 round3 复审 (2026-10-13, 席 W11HGR3)

- **锚判定 (停审)**: 开局 `cat v8-heap-gc/src/*.rs | shasum -a 256` (zsh 字典序, 17 个 .rs 文件) = **5838c6fa51f5089527…** ≠ 基准锚 **a9f792251cf33eeb** → **漂移**。bash/zsh 复算一致 (非口径差)。**归因 = 新语义改动, 非等价归一**:
  - lib.rs 新增公共桥接 `gc_prepare_linear_allocation_area` (lib.rs:112-119, 真实行为: `limit().saturating_sub(top())` 计浪费字节 + `reset(0,0)`), 并引入跨 crate 依赖边 `use v8_heap_core::linear_allocation_area::LinearAllocationArea`; 现态 grep 确证 heap-core 该类型具 `limit()/top()/reset(&mut,_,_)` 签名 → 系**完整可编译的新公共 API 语义面**, 非撕扯半态。
  - round1 明载 lib.rs 仅「D2 再导出同步」、round2 明载「未动 lib.rs」, 两轮均未记录此桥接 → 基线锚 (round2 出账时主控以标准式重算冻结的 a9f792) 之后新接入; 合 todo.md「外部会话 w10ocb objects-core / heap-core r2 在飞」与「heap-core/objects-core 快速落地」及 waves.md:215 重锚口径。近期改动文件 lib.rs(04:32)/incremental_marking.rs(03:57)/memory_reducer.rs(03:49)/marking.rs(03:21) 时间戳均晚于 round2 出账 (~02:4x), 正合 round2 预警的「外部写手并发改写 marking/memory_reducer/incremental_marking」危害。
  - **判**: 新语义改动 → **树漂移, 本轮不计 clean, 停审**; 连续 clean **0/3** (不推进)。
- **三数 (静态快量, 仅入账记录, 不作 clean 依据; 三数本身未随漂移变动)**: skel `todo!` 实测 97/70/14/13/5/4/3/2 + 其余 0 = **208 不变**; 挂载 .rs `todo!` **0**; 挂载面 `allow(` **0**; unsafe **0** (仅 `#![forbid(unsafe_code)]` 属性 lib.rs:12 + marking.rs:6 doc 文字, 无实 unsafe); readme 分组表列和 208。
- **锚邻域快读非回归抽证 (非深攻)**: D2 sweeper 判别值 NEW=1/OLD=2/…/SHARED_TRUSTED=6/LO=8 + K_ZAP_TAGGED=0xbeadbeef 仍在 (sweeper.rs:43-55); round2 W2D1 Priority 升序 BestEffort=0<UserVisible=1<UserBlocking=2 + 编译期断言 3 条 (gc_tracer_cc.rs:11-18/125-130)、W2D2 `event_state_to_string→Option<&'static str>` 且 NotRunning→None (gc_tracer_cc.rs:74-81) 均存活 → round1/2 修复无回归; 但基线已移, 存活亦不构成 clean。
- **must-fix**: 本轮停审, **不对移动靶出定案 must-fix**; 唯一 action = 报主控基线漂移, 请以现态标准式**重锚 (re-freeze round3 anchor)**后再派复审。附带提示: 新桥接使 lib.rs 对 v8-heap-core 产生新硬依赖边, 复审环须确认 heap-core 冻结序, 防依赖再漂移。
- **门禁**: **停审未跑** (基线失效 + 外部写手活动窗未定, 对移动靶跑认证门禁无 clean 意义且易撕扯); 未跑 nextest/clippy; 未新增 tests; **未改 src** (lib.rs 禁动遵守); 全程只读, 复核 git status v8-heap-gc clean、锚读期间 5838c6fa 稳定; 未 git 操作、未碰 tool/lib.js、未触他 crate。
- **结论**: 锚不一致 + 新语义漂移 (lib.rs LinearAllocationArea 桥接) → **round3 停审, 不计 clean**; 连续 clean 维持 **0/3**, 续战待重锚。
