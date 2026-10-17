# v8-objects-js-async 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## Wave 席位 W6O4 状态格原文

| W6O4 | objects backing-store/序列化/managed/intl/simd | v8-objects-js-async | 70 | backing_store 18 + 序列化族填体 | [x] (中断·提供方故障; 差集: todo! 70→48, 新产 hash_seed_wrapper/heap_number/oddball 三件, 撞线红态 E0401/const PartialEq/RoundingMode::kHalfEven) → 接力 W6O4b (开发中, 先修红禁重做) |

- [轮7 W7O4] 开工基线盘点: grep `todo!` = 48 (backing_store_cc 18 + intl_objects 7 + value_serializer_skel 5 + option_utils 4 + managed 4 + cpp_heap_object_wrapper 4 + allocation_site_scopes 4 + regexp_match_info 2); 挂载模块 (microtask / promise / cpp_heap_external_object / allocation_site_tq / oddball / heap_number / hash_seed_wrapper / waiter_queue_node / bigint_tq / simd / js_promise / js_generator) 无重复实现且 0 todo!; cargo check 绿, 门禁基线 6/6。
- 本轮填体 (backing_store 优先): `src/backing_store_cc.rs` 引入值语义面 `BackingStore` (buffer_start Option<NonNull> + atomic byte_length + flags AtomicU16 + id 计数 + deleter/deleter_data TypeSpecificData 承接), 补齐 `WasmMemoryFlag` 枚举、`Flag` 位标枚举、`DeleterCallback` fn 指针型、`K_WASM_PAGE_SIZE / K_MAX_WASM_MEMORY64_PAGES / K_MAX_MEMORY64_SIZE` 常量、`NEXT_BACKING_STORE_ID` 全局原子; 填 5 面:
  - `BackingStore::new` ctor (含原版 8 条 DCHECK/CHECK 位/长度约束 + flags 落位 + id fetch_add)
  - `BackingStore::get_wasm_reservation_size` (wasm32 全 guard / wasm64+arm64 附加页 / 非 guard 直返 三分支)
  - `BackingStore::wrap_allocation` (`Option<DeleterCallback>` 承接 v8::BackingStore::EmptyDeleter 语义位, 绕 Rust 侧 fn 指针比较不稳定)
  - `BackingStore::empty_backing_store`
  - `BackingStore::make_wasm_memory_resizable_by_js`
  - 附 backing-store.h inline 访问器面 (is_shared / is_resizable_by_js / is_immutable / is_wasm_memory / is_wasm_memory64 / has_guard_regions / wasm_memory_flag / byte_length / max_byte_length / byte_capacity / id / is_empty / per_isolate_accounting_length / raw_buffer_start) + `#[allow(clippy::too_many_arguments)]` ctor。
- 剩余 43 todo! 全归推迟台账 (readme/zh.md「推迟台账」节, 依 backing-store.cc 13 / value-serializer 5 / managed 4 / cpp-heap-object-wrapper 4 / option-utils 4 / intl-objects 7 / allocation-site-scopes 4 / regexp-match-info 2 分档, 每条含原函数名 · 前置 · 解锁 crate)。
- 新增测试 `test_backing_store_reservation_and_value_semantics` 覆盖 reservation 三分支 / EmptyBackingStore / WrapAllocation 双 deleter / 位标翻转 / id 单调 / AllocationStatus 与 Flag 常量对数; 门禁 7/7 全绿。
- 收笼计数: todo! 48 → 43 (填体 5); 期间 v8-objects-core `compressed_slots.rs` 曾被并行席位改到红态 (pointer→int const eval 拒), 未影响本 crate 门禁。
- 红线遵守: 仅动 v8-objects-js-async 与 readme/zh.md + tests/main.rs + log; 无 git 提交; 无全仓门禁; 无 panic!; 锚点 `/// v8: src/objects/<file>.{h,cc} <Class>::<method>` 齐。

## Round 1 Review 审查与优化结论 (2026-10-12)

- 规范符合性盘点:
  - 12 项 Rust Review 规范全量核对完毕。
  - 0 panic!、0 unwrap()、0 allow 绕过。
  - BackingStoreInit 解耦入参结构体彻底消除 `clippy::too_many_arguments`。
  - 内存布局与原版 V8 1:1 对齐（JSPromise 5 words、JSGeneratorObject 11 words、JSAsyncGeneratorObject 13 words、JSAsyncFunctionObject 14 words、AsyncGeneratorRequest 5 words）。
- 源码逐行交叉比对:
  - `js-promise.{h,-inl.h,tq}` vs `js_promise.rs`: PromiseState (Pending/Fulfilled/Rejected) 状态机、Reaction 链表与 Result 互斥槽位、Flags 状态位域完全一致。
  - `js-generator.{h,-inl.h,tq}` vs `js_generator.rs`: ResumeMode (Next/Return/Throw/Rethrow)、Continuation (-2 executing, -1 closed, >=0 suspended) 状态转换、AsyncGeneratorRequest 队列操作、闭包挂起机制完全一致。
  - `backing-store.{h,cc}` vs `backing_store_cc.rs`: BackingStoreInit 解耦结构体、BackingStore 值语义面构造与 DCHECK 约束、get_wasm_reservation_size (wasm32 8GB guard / wasm64+arm64 额外分配页 / 非 guard 直返)、WrapAllocation (Option DeleterCallback 处理 EmptyDeleter) 与 EmptyBackingStore 完全一致。
- 门禁实测:
  - `CARGO_TARGET_DIR=target/agent_w9oasr1 cargo nextest run -p v8-objects-js-async`: 7/7 tests passed (0 skipped)。
  - `CARGO_TARGET_DIR=target/agent_w9oasr1 cargo clippy -p v8-objects-js-async --all-targets -- -D warnings`: 0 warnings。
  - `rg "\[allow"`: 0 命中。
- 结论: clean (达成 clean 1/3 目标)。

## Round 2 Review 审查与优化结论 (2026-10-12)

- 规范符合性深度复核与优化:
  - 12 项 Rust Review 规范全量深度复核通过。
  - 彻底优化消除 `src/value_serializer_skel.rs` 中 2 处 `expect` 调用，全 crate 达成真正 0 panic!、0 unwrap()、0 expect()。
  - `rg "\[allow"` 全 crate 0 命中，无任何 clippy 警告抑制。
  - 增强 `tests/main.rs` 单元测试，将原消极引用的 Flag 数组重构为对 BackingStore 的 `has_guard_regions` 与 `is_immutable` 真实行为断言，消除未使用导入并提升覆盖质量。
- 源码逐行交叉比对复核:
  - `js-promise.{h,-inl.h,tq}` vs `src/js_promise.rs`:
    - PromiseState 状态机及字符串映射与 V8 1:1 对齐。
    - Reactions 链表与 Result 结算结果槽位重叠（互斥读取、单向状态转移、防二次结算）。
    - Flags 位域位宽与掩码（Status 2b, IsNativeResolverInvoked 1b, HasHandler 1b, IsSilent 1b, AsyncTaskId 26b）及 GetNextAsyncTaskId 回绕跳 0 逻辑完全一致。
    - 内存布局（5 words 头部、对象内属性容量上限 250）编译期静态断言守死。
  - `js-generator.{h,-inl.h,tq}` vs `src/js_generator.rs`:
    - ResumeMode 严格映射 (Next/Return/Throw/Rethrow)。
    - Continuation 状态转换 (-2 executing, -1 closed, >=0 suspended) 单向流转闭环。
    - AsyncGeneratorRequest 单向链表队列及 JSAsyncGeneratorObject 的 is_awaiting 标志操作完全一致。
    - JSAsyncFunctionObject 闭包安全与内存布局（11/13/14 words 头部）编译期静态断言守死。
  - `backing-store.{h,cc}` vs `src/backing_store_cc.rs`:
    - BackingStoreInit 结构体解耦彻底规避 `too_many_arguments`。
    - 8 项 DCHECK/CHECK 约束前置校验完全等价承接。
    - get_wasm_reservation_size 三分支算法（wasm32 8GB guard / wasm64+arm64 附加页 / 非 guard 直返）严格匹配。
    - WrapAllocation / EmptyBackingStore / EmptyDeleter / per_isolate_accounting_length 行为逻辑完全对齐。
- 门禁实测:
  - `CARGO_TARGET_DIR=target/agent_w9oasr2 cargo nextest run -p v8-objects-js-async`: 7/7 tests passed (0 skipped)。
  - `CARGO_TARGET_DIR=target/agent_w9oasr2 cargo clippy -p v8-objects-js-async --all-targets -- -D warnings`: 0 warnings。
  - `rg "\[allow"`: 0 命中。
  - `rg "panic!|unwrap\(|expect\("`: 0 命中。
- 结论: clean (达成 clean 2/3 目标)。

## Round 3 Review 终验审查与冻结结论 (2026-10-12)

- 规范符合性终实验收 (12 项原则全量合规):
  - 0 panic!、0 unwrap()、0 expect()、0 allow 绕过。
  - BackingStoreInit 解耦入参结构体，彻底杜绝 `clippy::too_many_arguments`。
  - 静态编译期断言全量覆盖核心对象内存布局 (JSPromise 5 words、JSGeneratorObject 11 words、JSAsyncGeneratorObject 13 words、JSAsyncFunctionObject 14 words、AsyncGeneratorRequest 5 words)。
  - 零拷贝借用、状态机单向流转、无死锁、无循环依赖。
- 逐行源码交叉比对终验:
  - `v8/src/objects/js-promise.{h,-inl.h,tq}` vs `src/js_promise.rs`:
    - PromiseState (Pending/Fulfilled/Rejected) 状态机及单向流转闭环。
    - Reactions 链表与 Result 结算互斥读写及防二次结算机制 1:1 对齐。
    - Flags 位域位宽（Status 2b, IsNativeResolverInvoked 1b, HasHandler 1b, IsSilent 1b, AsyncTaskId 26b）共 31 位严格与 SmiTagged 吻合。
    - get_next_async_task_id 回绕跳 0 行为严格对齐。
  - `v8/src/objects/js-generator.{h,-inl.h,tq}` vs `src/js_generator.rs`:
    - ResumeMode 严格对齐 (Next/Return/Throw/Rethrow)。
    - Continuation 哨兵状态机 (-2 executing, -1 closed, >=0 suspended) 单向转换严谨。
    - AsyncGeneratorRequest 单向链表队列与 JSAsyncGeneratorObject is_awaiting 标志操作完全一致。
    - JSAsyncFunctionObject 闭包字段安全绑定。
  - `v8/src/objects/backing-store.{h,cc}` vs `src/backing_store_cc.rs`:
    - BackingStore 值语义面解耦结构体与原子属性（byte_length, flags, id）完全对应。
    - 8 项 DCHECK/CHECK 前置约束（64位内存与wasm条件、guard_regions条件、resizable与custom_deleter互斥、byte_capacity非零等）严密等价。
    - get_wasm_reservation_size 三分支公式（wasm32 8GB guard / wasm64+arm64 额外页 / 非 guard 直返）1:1 严格对齐。
    - WrapAllocation (Option<DeleterCallback> 承接 EmptyDeleter) 与 EmptyBackingStore 行为一致。
    - PerIsolateAccountingLength 内存计费规则严密对齐。
- 门禁实测:
  - `CARGO_TARGET_DIR=target/agent_w9oasr3 cargo nextest run -p v8-objects-js-async`: 7/7 tests passed (0 skipped)。
  - `CARGO_TARGET_DIR=target/agent_w9oasr3 cargo clippy -p v8-objects-js-async --all-targets -- -D warnings`: 0 warnings。
  - `rg "\[allow"`: 0 命中。
  - `rg "panic!|unwrap\(|expect\("`: 0 命中。
- 结论: clean (达成 clean 3/3 终检验收冻结，已就绪)。


## W11JA 语义面补波 (2026-10-13)

- 开工三件套盘点: `wc -l src` 5710 行 / `cargo check -p` 绿 / `todo!` 复点 43 (backing_store_cc 13 + intl_objects 7 + value_serializer_skel 5 + option_utils 4 + managed 4 + cpp_heap_object_wrapper 4 + allocation_site_scopes 4 + regexp_match_info 2); `.skel` 2 (js_generator_tq / js_promise, 布局常量参考骨架, 无 todo!, 语义已落地 js_*.rs); `[allow]`/`panic!`/`unwrap`/`expect` 0。
- 依赖面实测: 本 crate Cargo.toml 仅链 v8-objects-arrays/core/js-core/js-func/maps; 兄弟 objects crate 均不链 handles/isolate/sandbox。43 todo! 全需 Isolate / Handle / v8-sandbox cpp 堆指针表 / v8-heap-cppgc / ICU / PageAllocator / Wasm 引擎 → 深水不可在本层实现 (实现差集 = 0, 冻结不动; 加这些依赖会违反分层 + 触碰到并行在飞/复审环禁改 crate)。
- 填体件数: 0 (实现); 合规补波件数: 43 todo! 消息全部规范化为 `原函数名: 前置; 解锁 crate` 带参格式 + 深水 stub 补 `/// v8:` 锚 (原 intl 7 面缺函数名、多面缺「解锁 crate」)。
- 测试 1:1: 深水用例依赖 Isolate (value-serializer-unittest / managed-unittest / intl-unittest / backing-store cctest), 无原用例名可迁 → 归台账; 新增 3 例覆盖已实现但零覆盖的纯语义核: test_managed_external_memory_accounting / test_option_utils_pure_cores / test_value_serializer_buffer_primitives_roundtrip。
- 台账增减: readme/zh.md 推迟台账条目不变 (43 全在); 新增 W11JA 补波说明节 + 收笼计数行 (43→43)。
- 脚本核查: 本族 tool/src/objects/*.js 为生成种子, 落地文件经就地修订已超集 (WriteHeader/ExpandBuffer/Release 已填, 复跑回退), 依红线未重跑, 保持落地权威。
- 收官门禁: `CARGO_TARGET_DIR=target/agent_W11JA cargo clippy -p v8-objects-js-async --all-targets -- -D warnings` 0 警告; `cargo nextest run -p v8-objects-js-async` 10/10 全绿 (7 基线 + 3 新增, 0 skipped)。
- 红线遵守: 仅动 v8-objects-js-async (src 8 件 + tests/main.rs + readme/zh.md) 与 log; 未碰 v8-heap-cppgc / v8-snapshot / 其他在飞 crate; 无 git 提交; 无全仓门禁; 无 panic/unwrap/expect; 无 [allow]; todo! 基线 43 → 现值 43。


