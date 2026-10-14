[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-zone

Rust port of V8 `src/zone` (arena allocator), L0 foundation layer. `Zone` bump allocation with segment chaining, `AccountingAllocator` tracing, and zone containers (`ZoneVector`, `ZoneList`, `ZoneDeque`/`ZoneQueue`/`ZoneStack`, `ZoneForwardList`/`ZoneLinkedList`, `ZoneAllocator`/`RecyclingZoneAllocator`). Lifetimes replace raw `Zone*`: allocations and containers borrow the zone via `'z`.

Containers also include `ZoneCompactSet` (zone-compact-set.h port: packed pointer+2-bit-tag empty/singleton encoding, copy-on-write sorted zone list). Under the `precise-zone-stats` feature, `TypeStats` (type-stats.h port) records per-TypeTag allocation ledgers, merged and dumped by the tracing `AccountingAllocator` (accounting-allocator.cc:162-175).

See [中文文档](./zh.md) for the full module/test mapping tables, the bumpalo kernel evaluation (not adopted), the semantic deviation log, the deferred list, and the symbol audit confirming that `AccountingScope`/`AllocationRecord`/three-mode allocation/`ZoneUnique`/`MakeZoned` do not exist in this V8 checkout.

Development log (adversarial review round fix registers): see ../../log/v8-zone.md.


---

<a name="zh"></a>

# v8-zone

V8 `src/zone` (arena 分配器) 的 Rust 转写, L0 地基层。原版约 3.5k 行 C++ (zone.h/cc、zone-segment、accounting-allocator、zone-allocator、zone-containers、zone-list、zone-utils、type-stats、zone-compact-set); Rust 侧 src 约 2.9k 行 + tests 约 1.5k 行。

生命周期模型: `Zone<'a>` 借用 `AccountingAllocator`; 容器与分配结果经 `&'z Zone` 参数化拿到 `'z` 借用, 取代原版裸 `Zone*`/placement-new。非线程安全语义保留 (原版注释), 计数用原子量仅为 tracing 读取口径。

- [模块映射](#模块映射)
- [测试映射 (7.3 规约先行, 断言强度守恒)](#测试映射-73-规约先行-断言强度守恒)
- [内核评估 (bumpalo, todo.md 十.3)](#内核评估-bumpalo-todomd-十3)
- [语义偏差登记](#语义偏差登记)
- [待回收 (下一轮替换为本仓库对应 crate)](#待回收-下一轮替换为本仓库对应-crate)
- [推迟清单 (本 checkout 存在、当前未转写)](#推迟清单-本-checkout-存在当前未转写)
- [符号盘点结论 (任务清单核对)](#符号盘点结论-任务清单核对)

## 模块映射

| 原版 (v8/src/zone) | Rust 模块 | 说明 |
|---|---|---|
| `zone.h` / `zone.cc` | `src/zone/core.rs` | Zone/ZoneSnapshot/ZoneScope/ZoneAllocationPolicy; position_/limit_ 封装进 `kernel/chunk.rs` (Chunk::try_bump = AllocateUnchecked) |
| `zone-segment.h` / `.cc` | `src/zone/segment.rs` | LIFO 段链 + zap 调试填充 (kZapDeadByte=0xcd) |
| `accounting-allocator.h` / `.cc` | `src/zone/accounting.rs` | AccountingAllocator + TracingAccountingAllocator (合并为 `new_tracing` 形态); TracingFlags 用 static AtomicBool 替身 |
| `zone-allocator.h` | `src/containers/allocator.rs` | ZoneAllocator/RecyclingZoneAllocator (头部 O(1) 空闲链) + `ZoneBoolAllocator`/`ZoneIntAllocator` 别名 |
| `zone-containers.h` ZoneVector | `src/containers/vector.rs` | [data,end) 活 / [end,cap) 未初始化不变量逐行对照 |
| `zone-containers.h` ZoneDeque/ZoneQueue/ZoneStack | `src/containers/deque.rs` | 双栈式等价实现 (偏差见下) |
| `zone-containers.h` ZoneForwardList/ZoneLinkedList | `src/containers/linked.rs` | V8 实际使用子集 (push/pop/前插前删/迭代/clear) |
| `zone-list.h` / `zone-list-inl.h` | `src/containers/list.rs` | ZoneList 全语义 (Add/AddBlock/InsertAt/Remove/Rewind/Sort/StableSort/Iterate/DropAndClear) |
| `zone-utils.h` CloneVector | `Zone::clone_vector` (core.rs) | 本 checkout zone-utils.h 仅此一函数 |
| `zone-type-traits.h` ZonePtrList | `containers::ZonePtrList` 别名 | ZoneTypeTraits/FullZonePtr 为 C++ 指针间接机制, Rust 无对应需求, 不转写 |
| `type-stats.h` / `.cc` | `src/zone/type_stats.rs` (feature `precise-zone-stats`) | TypeStats/StatsEntry: type_name 静态串替身 type_index, BTreeMap 确定序; Zone 挂账于 `allocate_tagged`/`delete`, AccountingAllocator 侧 MergeWith/Dump 对应 accounting-allocator.cc:162-175 |
| `zone-compact-set.h` | `src/containers/compact_set.rs` | ZoneCompactSet/ZoneCompactSetTraits: usize 指针+2bit tag 打包替身 PointerWithPayload, 0/1 值紧凑编码 + zone 内有序列表写时复制 |

依赖: `thiserror` (error.rs 集中错误); dev-deps: aok / ctor / log / log_init (测试规范 7.3)。

## 测试映射 (7.3 规约先行, 断言强度守恒)

| 原测试文件 | Rust 测试文件 | 用例 | 说明 / 跳过理由 |
|---|---|---|---|
| `zone/zone-unittest.cc` | `tests/zone_unittest.rs` | 8 | `8ByteAlignment` 1:1 (`zone_test_8_byte_alignment`); 其余 7 例为 zone.h/zone.cc 语义对照补充 (allocation_size 口径、Expand 高水位、Reset 保留末段、Snapshot/Restore、ZoneScope 回滚、Drop 归还、错 zone Err) |
| `zone/zone-vector-unittest.cc` | `tests/zone_vector_unittest.rs` | 4 | Basic/Assign/Insert/Erase × Trivial/CopyAssignable/MoveAssignable/NotAssignable 全 16 组合; LiveSet (地址集合) → serial 登记 (任何时刻存活集合==向量元素集合, 不变量等价); C++ 临时量语义以内层 `{}` 作用域 + 显式 `drop` 复刻, 保证 `CheckContainsAll` size 断言成立 (如 v4 必须先于空 v3 断言析构, 对应 cc:199-203 内层花括号) |
| `zone/zone-allocator-unittest.cc` | `tests/zone_allocator_unittest.rs` | 7 | 7 个 TEST_F 全转写, 指针相等断言保留; `TestWithStdContainers` 的 std::set 分量跳过 — Rust 无 allocator 注入等价物, vector/list 分量以 ZoneVector/ZoneLinkedList 同源验证 sum=5050 |
| `zone/zone-compact-set-unittest.cc` | `tests/zone_compact_set_unittest.rs` | 12+1 | 12 个 TEST_F 全转写 (用例名保留, 含原版 DoesNotContainsNonSubset 拼写); 另补 clone/union/iter/range 构造 1 例 |
| — (自建) | `tests/main.rs` | 4 | New/NewVector/CloneVector/Seal/Delete tracing 记账 (`zone_test_make_and_vector_forms` 等) + 别名面 (`zone_test_type_aliases`) |
| — (自建) | `tests/zone_type_stats_unittest.rs` | 4 | AddAllocated/AddDeallocated 逐标签口径、MergeWith 并账、TraceZoneDestruction 收尾 Dump、开关默认关闭 (仅 `precise-zone-stats` 生效) |
| — (自建) | `tests/zone_containers_unittest.rs` | 6 | ZoneList/ZoneDeque/ZoneQueue/ZoneStack/ZoneForwardList/ZoneLinkedList/AccountingAllocator/ZoneAllocationPolicy 语义 (原版无官方单测, 按头文件自拟) |

用例名保留原版 gtest 名 (snake_case, `fixture_name` 前缀)。门禁: build 0 警告 + clippy `--all-targets --all-features -D warnings` 0 条 + nextest 79/79 + `cargo +nightly miri test` 全过。

## 内核评估 (bumpalo, todo.md 十.3)

结论: **不采用 bumpalo, 维持自研 chunk 内核** (复测记录同步见 `src/kernel/mod.rs`)。

- 性能 (macOS aarch64, release, 1,000,000 次分配): 固定 32B 自研 0.0022s vs bumpalo 0.0039s (bumpalo 侧含 fill_default 清零开销); 变长 1..=64B 自研 0.0020s vs bumpalo 0.0107s; 两者 8 字节对齐违例均为 0。
- 语义缺口 (决定性): `bumpalo::Bump` 无 Segment 级链表 (Snapshot/Restore 需按段归还)、无 TraceAllocateSegment/allocation_size 记账钩子、reset 不保留末段、无 per-zone `segment_bytes_allocated` 口径 — 无法满足 zone.h 外部行为 1:1 红线 (五章 "zone 语义 API 自研")。
- bumpalo 依赖与评估代码已移除, 无遗留双轨; 若下一轮需要重测, 依本节流程重开。

## 语义偏差登记

- `Zone::make`/数组接口以 `Result` 返回 (原版 placement-new + CHECK); 分配失败映射 `Error::OutOfMemory`, 禁 panic。
- arena 借出: `&'z self -> &'z mut T` 为 Zone 核心设计 (对应原版 `Allocate` 返回裸内存), impl 级 `#[allow(clippy::mut_from_ref)]` 注明理由; 生命周期 `'z` 即原版 Zone 内存有效期的类型化。
- **D4 API 收紧 (soundness)**: 回滚类入口与借出引用互斥 ——
  - `Zone::reset(&self)` → `Zone::reset(&mut self)`; 同时恢复原版回卷语义 (position 回到 `RoundUp(keep->start())`, zone.cc:89; 旧版为规避别名保留高水位的偏差已撤销)
  - `ZoneSnapshot::restore(&self, &Zone)` → `restore(&self, &mut Zone)`
  - `ZoneScope::new(&'z Zone)` → `new(&'z mut Zone)`; `zone(&self)` → `zone(&mut self) -> &mut Zone` (作用域内分配借用被约束在作用域内, 回滚后同地址再借出在类型层面不可表达)
  - 调用方需以块/drop 先行结束借出引用再回滚; 旧式 "持有 zone 借出的同时建 scope" 现为编译错误
- `ZoneVector::clone` 无法传播分配错误: 失败保持空向量 (原版拷贝赋值无失败面, 偏差登记于此)。
- C++ UB 调用形态 (空 pop/front/back、Restore 传错 zone) 收紧为 `None`/`Error::WrongZone`, 不弱化对应用例断言。
- ZoneDeque: 原版为 `std::deque` 包装, 保证元素引用稳定; Rust 双栈实现不保证引用稳定 (以索引/值访问代替), 中段 insert/erase 原版无使用者未转写。
- ZoneList 不实现 Drop (与 `~ZoneList() = default` 一致); Rust 侧为维持 "[0,length) 活" 不变量在 shrink/clear 处补 `drop_in_place` (C++ 不调析构, 对 POD 等价)。
- ZST 分配不占段空间, 返回 dangling 对齐指针 (原版 sizeof 语义下仍走 RoundUp, Rust 空类型无字段可读, 等价)。
- 测试作用域: C++ 局部变量逆序析构与 Rust drop 顺序一致, 但临时量 (C++ full-expression 末尾即析构) 在 Rust 需显式 `{}` + `drop` 限定, `zone_vector_unittest.rs` 全部按此复刻。

## 待回收 (下一轮替换为本仓库对应 crate)

- `src/internal/bits.rs`: ALIGNMENT/KB/MB/align_up/align_down/is_aligned 私有替身 — 评估结论: **不接线 v8-base**。v8-base `src/bits.rs` 是原版 `src/base/bits.h` 的位计数/旋转/溢出工具 (count_leading_zeros/rotate_right/signed_mul_high 等), 不含 RoundUp/RoundDown/kKB/kMB — 原版这三者位于 `src/utils/allocation.h` 与 `src/base/macros.h`, v8-base 尚未转写该工具族; API 不匹配, 强接即错位映射。待 v8-base 落地 allocation 工具后再回收本模块 (本轮未注册依赖)。
- `TracingFlags::is_zone_stats_enabled` 替身 (static AtomicBool) → v8-tracing; `zone_stats_tolerance`/`trace_zone_stats` 两个 flag → v8-flags。
- `managed_zone_memory` 路径 (BoundedPageAllocator/VirtualMemory/MemoryPool) → L3 heap-base 后回收 (accounting-allocator.h:47-52)。

## 推迟清单 (本 checkout 存在、当前未转写)

- `ZoneObject` (zone.h:324): C++ placement-new 标记基类, Rust `Zone::make` 已覆盖其语义, 无需转写。
- `AsanNew`/ASan redzone (zone.cc:42-67): `V8_USE_ADDRESS_SANITIZER` 构建路径, 动态检查由 7.4 Miri/ASan harness 承担。
- STL/absl 包装容器: `ZoneSet`/`ZoneMultiset`/`ZoneMap`/`ZoneMultimap`/`ZoneUnorderedMap`/`ZoneUnorderedSet`/`ZoneMapInit`/`ZonePriorityQueue`/`ZoneAbslFlatHashMap`/`ZoneAbslFlatHashSet`/`ZoneAbslBTreeMap` (zone-containers.h:698-860) — V8 下游使用点集中在 L2+ (ast/parsing/compiler), 按需回补。
- 依赖 v8-base 的容器: `SmallZoneVector`/`SmallZoneMap` (base::SmallVector/SmallMap)、`ZoneIntrusiveSet` (base::IntrusiveSet)、`ZoneHashMap`/`CustomMatcherZoneHashMap` (base::PointerTemplateHashMapImpl)。
- `ZoneHandleSet<Handle<T>>` 别名 — 本体 `ZoneCompactSet` 已转写, 别名依赖 Handle (L3), 按需补一行 `pub type`。

## 符号盘点结论 (任务清单核对)

经全仓 grep 验证, **本 checkout (当前 v8/src/zone) 不存在** 以下历史符号, 无对应原版行为可转写, 不实现、不预留: `AccountingScope`、`AllocationRecord`、三态分配模式 (`kAllocateAll`/`kAllocateForCalculationOnly`/`kCalcOnlyNoAlloc`)、`ZoneUnique`/`UniquePtr`/`MakeZoned`/`ToZoneUnique`/`ToSharedZoned`、`ZoneAllocationList`、`accounting-scope.h/cc`。它们属旧版 V8 (上游已删)。当前 checkout 中对应职能的位置: tracing 记账并入 `Zone` 构造/析构/`Expand` 钩子 (zone.h/cc), 类型标注仅剩 `Allocate<TypeTag>` 模板参 (仅 V8_ENABLE_PRECISE_ZONE_STATS 下用于 type_stats)。清单其余符号 (`Zone`/`ZoneSnapshot`/`ZoneScope`/`ZoneAllocationPolicy`/`ZoneVector`/`ZoneDeque`/`ZoneList`/`ZoneAllocator`/`RecyclingZoneAllocator`/`IntVector`) 均已实现并覆盖测试。

---

开发日志 (审查环 round1-round5 修复登记) 见 ../../log/v8-zone.md。

