# v8-zone 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### 审查环 round1 修复登记 (对抗审查)

- D1 Expand 高水位上限失效: `new_size.max(min_new_size).max(MAX)` → `min_new_size.max(MAX)` (zone.cc:168-174), 20K 请求得 32768 段。
- D2 `ZoneList::insert_at` double drop: 右移链改 `ptr::copy` (memmove) + `ptr::write`, 零析构 (zone-list-inl.h:97-104)。
- D3 `ZoneSnapshot::restore` 错误路径非原子: 先只读确认 snapshot.segment_head 在段链可达, 不可达直接 Err 不归还段; 测试 `zone_test_restore_wrong_zone_error_must_not_release_segments` 转。
- D4 见上文 API 收紧 (收紧详情保留于 readme「语义偏差登记」D4 条目); round1 两红测改判 "类型层排除 + 回滚复用恢复原版语义"。
- D5 `ZoneAllocator::allocate` 绕过对齐检查: 转调 `Zone::allocate_array` (原版经 `zone->AllocateArray<T>` 的 static_assert 等价)。
- D6 `Allocate(0)` 返回段内 `position_` (zone.h:66-69), 仅空 zone 兜底 dangling。
- D7 `ZoneVector::erase(index==size)` release 越界读: `index >= size() → None` 防护。
- O1 `ZoneCompactSet` 转写落地 (417 行) + 12 用例; O2 `TypeStats` 转写落地 (96 行, feature 门控) + MergeWith/Dump。
- O3-O8 维持推迟 (原 readme 推迟清单): zone-hashmap、STL/absl 包装容器、SmallZoneVector/SmallZoneMap/ZoneIntrusiveSet、ZoneHandleSet 别名 (Handle, L3)、AsanNew redzone、managed_zone_memory。

### 审查环 round2 / round3 修复登记 (对抗审查)

- R2-1 `allocate_bytes` 超限请求直通 align_up 回绕: 入口 `size > kMaxNewSize` 拦截映射 `Err(OutOfMemory)` (原版 zone.cc:163-177 `new_size > INT_MAX` → FatalProcessOutOfMemory; core.rs:171)。
- R2-2 `Zone::make` ZST 路径实参提前析构 (UAF): `mem::forget` 转移所有权 (C++ 无此形态 — `sizeof(T)==0` 类型不可默认构造; core.rs:227)。
- R2-3 `AccountingAllocator::allocate_segment` 小于头部大小即放置构造越界: `bytes < size_of::<Segment>() → None` (原版 accounting-allocator.cc:135 `DCHECK_LE(sizeof(Segment), count)` 的 release 直通收紧为运行时防护; accounting.rs:144)。
- R2-4 `ZoneVector::assign_from` 自赋值 drop 后复读 (double drop/UAF, 原版 zone-containers.h:131 `DCHECK_NE(this, &other)` 契约): 收紧为 `ptr::eq` 短路 (vector.rs:504)。
- R2-5 `prepare_for_insertion` `old_size + count` 整数回绕 (原版 zone-containers.h:510 `CHECK(max - size() >= count)`): `checked_add → Err(SizeOverflow)` (vector.rs:344)。
- R2-6 `RecyclingZoneAllocator::deallocate` / `Zone::delete` 乘法溢出: `checked_mul` 防护 (allocator.rs:105; core.rs:298-302)。
- R3-1 线程安全边界显式化: `unsafe impl Send for Zone/ZoneSnapshot` (原版 zone.h "the Zone must be used by only one thread" 契约), 容器与分配器保持 !Send + !Sync, 正/负向界用例锁定 (core.rs:531,548)。
- R3-2 `Zone::reset` 多段复位一致性: position_base 与 segment_head 同步 (zone.cc:69-93; core.rs:325)。
- R3-3 `AccountingAllocator` 析构零泄漏守恒 + `ZoneAllocationPolicy` Debug/PartialEq/Eq 语义 (review_round3 用例锁定)。
- R3-4 (round3 第二批, 独立 Reviewer) `ZoneVector::assign_from` realloc 分支容量漂移: 旧实现走 `Grow` (高水位 `max(2*old_cap, min)`), 原版 zone-containers.h:166-170 按 `other.capacity()` 精确分配 → 改 `grow_exact` (vector.rs:533); 回归 `zone_vector_assign_from_realloc_exact_capacity`。
- R3-5 (round3 第二批) `ZoneVector::with_default` 容量漂移: 旧实现经 `resize_default`→`EnsureCapacity`→`Grow`, size=1 被 kMinCapacity=2 抬到 2; 原版 zone-containers.h:64-69 `AllocateArray(size)` 容量恰为 size → 改 `grow_exact` 直配 (vector.rs:551); 回归 `zone_vector_default_ctor_exact_capacity`。
- R3-6 (round3 第二批) 泄漏压力补强: 固定 seed (fastrand) 2000 轮 alloc/snapshot/restore/reset/副 zone 混合序列, 逐迭代核对 `allocation_size` 记账与 `current_memory_usage == segment_bytes_allocated` 守恒, 收尾断言 allocator 归零 (`zone_stress_random_alloc_restore_reset_no_leak`)。同批复核结论: D2 `insert_at` 的 `ptr::copy` 右移源区间被本操作整体覆盖 ([index+1,old) 收到移入拷贝, index 写入 elem), 无位复制残留, 双 drop 论证成立; `AccountingScope` 等历史符号经 grep 双侧确认不存在 (readme 符号盘点一致)。

### 审查环 round4 修复登记 (对抗审查)

- R4-1 `ZoneVector::from_slice` (initializer_list ctor) 容量漂移: 旧实现走 `reserve`→`Grow` (size=1 被 kMinCapacity=2 抬到 2), 原版 zone-containers.h:87-96 `AllocateArray(size)` 容量恰为 size → 改 `grow_exact` 直配 (vector.rs:481); 回归 `zone_vector_from_slice_exact_capacity`。
- R4-2 `ZoneVector::assign_from` 就地覆盖分支缺同 zone 前提: 原版 zone-containers.h:133 条件为 `capacity() >= other.size() && zone_ == other.zone_`, 旧实现仅查容量, 跨 zone 且容量足够时误走就地分支 (容量/记账口径漂移) → 补 zone 地址相等判定, 跨 zone 一律落 realloc 分支 (vector.rs:514); 回归 `zone_vector_assign_from_cross_zone_takes_realloc_branch`。
- R4-3 `ZoneVector::assign_from` realloc 分支旧存储未 `DeleteArray`: 原版 zone-containers.h:151-152 析构元素后归还旧块记账 (`freed_size_for_tracing` 漏计) → clear 后补 `zone.delete(old_data, old_cap)` (vector.rs:537); 回归 `zone_vector_assign_from_realloc_frees_old_storage`。
- R4-4 (Miri 抽查, 7.4) unsafe 面三项整改, 全套件 `cargo +nightly miri test` 通过:
  - `AccountingAllocator::allocate_segment` 暴露 raw 指针 provenance (accounting.rs); `Segment::zap_contents/zap_header` 改 raw 指针直算偏移 (不再经 `&Segment` 共享 retag + int 回转, segment.rs);
  - `Chunk` position/limit 改真实指针 (provenance 派生自段 raw 链), 分配写路径零 int→ptr 通配符; snapshot/restore 仍存整数 (对应原版 `Address position_` 绝对地址), Restore 按快照偏移自段 raw 指针重派生 (语义与 zone.cc:243 一致);
  - DEBUG zap (zone.h:90-93 memset / Segment::ZapContents/ZapHeader) 在 `cfg(miri)` 下跳过: Stacked Borrows 借用追踪无 C++ DEBUG memset 对应物, poison 写会使同段存活派生 tag 出栈; freed 记账与 release 行为不变;
  - round2 自赋值用例 `zone_vector_assign_self_safe` miri ignore (登记): 用例经裸指针自造 `&mut`+`&` 别名, 调用点本身即调用方侧 UB; R2-4 的 `ptr::eq` 短路守卫防的是函数体内二次解引用, 常规构建完整覆盖 — Rust 无 sound 的同对象双引用构造形态, 对应原版 DCHECK_NE 契约。
- 同批复核结论 (未构成缺陷): (a) 原版 `AccountingAllocator::AllocateSegment` 非 managed 路径经 `AllocAtLeastWithRetry`/`malloc_usable_size` 可能以大于请求的 count 构造 Segment (accounting-allocator.cc:121-136), Rust 精确分配满足契约 `total_size >= new_size` (zone.cc:183), 仅诊断计数有 ≤15B/段 平台性口径差, 属平台行为偏差非缺陷; (b) 原版 `TracingAccountingAllocator::nesting_depth_` (accounting-allocator.h:103) 仅增减无读取点, Rust 不落该字段零行为差; (c) `is_zone_stats_enabled` = `zone_stats != 0` (tracing-flags.h:44-45) 与 Rust 单 AtomicBool 替身语义一致; (d) mirror/accounting-scope 模式经 grep 确认本 checkout 无此符号 (与符号盘点一致), ZoneScope 快照回滚的四处计数恢复 (allocation/segment_bytes/tracing/freed) 逐项与 zone.cc:240-249 对齐; (e) `from_slice` 之外的容量口径已全量锁定 (with_value/with_default/clone/assign_from realloc 均 grow_exact 直配)。

### 审查环 round5 修复登记 (对抗审查)

- R5-1 `ZoneCompactSet` 缺 `hash_value` 对应面: 原版 zone-compact-set.h:264-266 `friend hash_value(ZoneCompactSet) = data_.raw()` (下游 maglev-cse.h:27-28 `gvn_hash_value` 经 `base::hash_value` 分发消费), Rust 侧无 `std::hash::Hash` 实现 → 补 `impl Hash` (对打包字散列, 与原版 data_.raw() 口径一致, compact_set.rs); 回归 `zone_compact_set_test_hash_value_matches_packed_word`。
- R5-2 `ZoneCompactSet` 指针回转未接 exposed-provenance (round4 R4-4 只覆盖 core/accounting/segment): `data` 打包字的 `(x) as *mut List/()` 无暴露 provenance, Miri SB 下属通配符回转 → 存入点 (`from_handle`/`insert`/`from_iter`/`new_list` 后打包) 补 `expose_provenance()`, 取回点 (`list()`/`singleton_ptr()`/`at()`) 改 `ptr::with_exposed_provenance_mut` (compact_set.rs); trait 签名与行为不变, 残余 miri 提示为 sanctioned 通知 (与 accounting.rs 同态)。
- 复核结论 (未构成缺陷): (a) `total_alloc_bytes_registers` 原版 grep 不存在 (本 checkout AccountingAllocator 仅 `current_memory_usage_`/`max_memory_usage_`), Rust 逐字段一致 (fetch_add + CAS max 更新 + fetch_sub, accounting-allocator.cc:127-134/139-152); (b) RecyclingZoneAllocator 空闲链指针在 Zone Reset/DeleteAll 后悬垂的契约与原版一致 (zone-allocator.h:80-94 不感知 zone 生命周期, 使用方约束), C++ 拷贝构造重置 free_list_ (zone-allocator.h:64-67) 在 Rust 无拷贝构造形态, 无对应偏差; (c) ZoneSnapshot 跨 zone/二次恢复矩阵: 跨 zone `Err(WrongZone)` (D3/R4 已收紧)、同 zone 二次恢复幂等 (与 zone.cc:219-250 一致)、快照后 Reset 使段不可达 → 预检 Err 不部分归还 (D3); ZoneScope Drop 内 restore 失败为 debug_assert + release 静默 (禁 panic 红线, 对应原版 CHECK 崩溃路径收紧); (d) ZoneList::sort_by=sort_unstable_by 对应 std::sort (introsort) 不稳定契约, stable_sort_by=slice sort_by 对应 std::stable_sort; 原版 StableSort DEBUG 断言区间 `i < l` 为上游笔误 (zone-list-inl.h:182), Rust 按正确区间 `start+1..start+len` 校验 (更严, 偏差登记); (e) miri ignore 唯一点 `zone_vector_assign_self_safe` 评估保留: 用例 UB 在调用点自造 `&mut`+&` 别名, 与 Chunk 真实指针改造正交, Rust 无 sound 等价形态。

### 门禁沿革附注 (自 readme 测试映射句迁出)

- 原 readme 测试映射「门禁」句附轮次引用 "(round4 起, unsafe 面抽查见 R4-4; round5 compact_set 接入 exposed-provenance 见 R5-2)", 随本台账迁入; readme 只保留当前门禁口径 (build 0 警告 + clippy 0 条 + nextest 79/79 + miri 全过)。
- readme/en.md 的 round1 修复英文摘要 (D1-D7 + D4 API change) 与 zh.md round1 内容重复, 无独有信息, 未搬英文原文, 英文原文弃置。
