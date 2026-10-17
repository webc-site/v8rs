# v8-heap-core 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## Wave 席位 R5H 状态格原文

| R5H | `heap/memory-allocator.h/.cc` (实态 420+832) | v8-heap-core | HC-2b 接力 (盘点静默在飞面→填体→测试), 富余开 HC-2c 骨架 | [x] (HC-2b 收笼 64/64 双零; HC-2c 骨架 998 行 83 todo! 交下波填体) |

## Wave 席位 R5H3 状态格原文

| R5H3 | `heap/{spaces.h,-inl.h,.cc}` + `heap/{large-spaces.h,.cc}` (实测 1062 行; 原票 base/basic-blocks 两件实测 vendored 树不存在, 已删) | v8-heap-core | HC-4 空间实体族一波: spaces.rs + large_spaces.rs 填体, main_allocator 83 todo! 不在本票 | [x] (150 轮熔断无交票, 以工作树现场抢救: spaces.rs 803 + large_spaces.rs 1106 = 1909 行填体落盘且 lib.rs 已挂 module, todo! 残 23 为深水推迟; 伙伴 11:36 rolling init 顺带入账; 主控代跑门禁 nextest 64/64 + clippy -D RC=0 + fmt 净 (target/rescue_r5h3); 审查环兜底 r1 HCR1 已收 (非干净票 2 must-fix+8 观察, F6 两处纯转发漏填/F1 测试零覆盖 → 处置席 R5H4 已收笼采纳: F1-F10 全闭环——F6 allocate_raw 双转发即填 (域参数逐字对位 cc:98-102/417-422)/F2 checked_add/sub 带回绕观测力+双 should_panic/F3 print 槽归位/F4 spaces_enums 孤儿删除+lib.rs 零引用/F5 todo! 台账五组落 readme/zh.md (可执行形 14→12 主控亲验逐处锚注)/F1 测试复刻 +30 例 (spaces 11+large 19, 1390 行) 全绿; 实态勘定申报 v8-flags large_page_pool 缺省 true (defs.rs:4614) 测试按实测落位; 主控独立复跑 nextest 94/94 + clippy -D RC=0 (Checking 行实证) + fmt 净, 改动面止于授权面自证) → **审查环 r2 HCR2 在飞** (主打 30 例新测试逐例鉴别力审计+F2/F6 修点核+F4 删除零丢失对账) |

## 看板行日志 (自 todo.md 收敛)

波次: HC-1 chunk/allocation/observer/LAA + 二波 pool(1019)/free-list(1925) 已落地; **R5H 接力 HC-2b 收笼 (2026-10-12 主控独立复验)**: memory_allocator.rs 875 行填体 29 函数/推迟台账 22/todo! 0, 实态勘定 h420/cc832 (旧 536/716 叙事作废), nextest 64/64 + clippy RC=0 + fmt 净; HC-2c main_allocator.rs 骨架 998 行 83 todo! 已开待填体席 (实态 h367/cc983); **HC-4 一波 R5H3 抢救收笼 (2026-10-12 主控独立复验)**: spaces.{h,-inl.h,cc}+large-spaces.{h,cc} 实测 1062 行族填体落盘 spaces.rs 803 + large_spaces.rs 1106 (150 轮熔断无交票以工作树为准, 伙伴 rolling init 顺带入账; todo! 残 23 系深水推迟, 明细待审查环核), 主控代跑 nextest 64/64 + clippy -D RC=0 + fmt 净; 审查环兜底 HCR1 round1 已派; **审查环 round1 (HCR1, 全新只读席, 主控处置): 非干净票 2 must-fix + 8 观察**——门三绿复现属实但 64/64 对本族零触达; 对位语义高质量 (三账/取整/迭代安全序无静默数值缺陷)。F1 (must, 测试复刻零覆盖—席熔断未及), F6 (must, large_spaces.rs:702/909 两处纯转发 allocate_raw 漏填, 原版 cc:98-102/cc:417-422 单行转发零依赖即填), F2 (低, account_committed saturating_add 断言恒真→checked_add), F3 (低, LO 未覆写 trait print 槽且固有 print 签名 &self≠&mut self), F4 (低, spaces_enums.rs 孤儿未挂 mod 台账「已落地」不实+重复型风险), F5 (台账, todo! 真实可执行 14 处非 23, A-E 五组台账判定制已给), F7-F10 (观察: 锚点漂移簇/page_list push_back 前置 DCHECK 未注/尾下划线命名混用/Vec 与侵入链双存储同步约束需申报)。→ 处置开发席 R5H4 已派 (F1 测试复刻两文件 + F2/F3/F6 修 + F4 裁决 + F7 锚校)。**注: 主控前记「todo! 残 23」系 grep 文字命中口径, 以 HCR1 F5 实数 14 为准**; 队列 HC-2c 填体 + HC-4 二波 (local/new/paged/read-only-spaces, 全族实测 5815 行含一波后余 ~4.7k); 详账 v8-heap-core/readme/zh.md — **处置轮 R5H4 已收笼 (本长账补记)**: F1-F10 全闭环, +30 例新测试 1390 行 (期望值逐例注释原版算例), todo! 可执行形 14→12 且逐处台账锚注 readme/zh.md 五组登记 (主控行级亲验 7+5=12), F2 回绕/F3 print 槽/F4 孤儿删除+F6 转发域参数逐字对位, 实态申报 large_page_pool 缺省 true; 主控独立复跑 94/94 + clippy -D RC=0 + fmt 净 + porcelain 面自证。**R5H4 整改计数不重置审查环: clean 维持 0 (HCR1 两 must-fix 系 r1 票面 finding, 须 HCR2 全新只读席确认闭合方可计 1/3)**; 审查环 r2 HCR2 在飞 (测试鉴别力审计主打); 队列 HC-2c 填体 + HC-4 二波不变

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

en.md 仅 1 字节, 无英文日志可迁。以下为 readme/zh.md 迁出的波次台账原文 (readme 留置: spaces 族 todo! 台账 A-E 现行台账、spaces/large_spaces 测试映射、转换脚本、门禁口径)。

### 波次与状态

- **HC-1 memory-chunk+allocation 族（已落地终验）**: memory_chunk.rs 573 行 / 测试 474 行 13 例; allocation.rs 328 + allocation_observer.rs 483 / 测试 326 行 6 例 6/6 绿; linear_allocation_area.rs 190。tool/src/heap 10 脚本全绿, 对数合计 379→rs 锚增补后 chunk 族 146/39/41/43/41, result/stats/observer 30/31/44/27。全包门禁双零 (nextest 20/20 + clippy --all-targets RC=0)。
  - 新版勘定 (伙伴独立复核同结论): 无 MetadataMemoryChunk / 无 free_list 槽位字段 / 无 GetStart-End-Contains-ChunkOf; MemoryChunk 双字段 untrusted_main_thread_flags_ + metadata_index_; AllocationSpace 已在 v8-common 直接类型化。
  - defer 清单 7 项: h:519/523/528 + cc:233/264/1171 + layout:21。
  - 观察项 (已处置 20:55): HC-1R2b 面 4 脚本 rs 锚未增补 → 首代 HC-1R 僵尸席临终续跑完成 allocation 族锚增补 (result 30→66 / stats 31→59 / observer.h 44→110 / observer.cc 27→85), 主控 6 脚本复跑全绿, 只增不减纪律成立, 采纳入账; 首代席 20:48 后无动静, 灭失终谳。
- **HC-23R 两族终收（已收票闭环, 10-12 08:0x 主控独立复跑采纳）**: 夜袭翻案——首代双席从未灭失（额度限流静默），与二代同面并发整晚致 #38 放大形；额度团灭六席后 HC-3 系 ~01:00 自愈续跑至 03:21。终态: free_list.rs 1925（推迟台账 14 项）/ memory_pool.rs 1019（推迟 11 项）/ free_list_unittest 510 行 9 例 / memory_pool_unittest 643 行 16 例; 四脚本 280/141/191/176 全绿（free-list.h.js 病灶=static_assert 期望形缺 `as i32` 窄化, 追实态净 +3 锚未删断言）; **主控亲验 nextest 45/45 + clippy --all-targets RC=0 警告 0**。测试口径: 原版无 pool/free-list 专名 unittest, spaces-unittest.cc 两用例 1:1 + CHECK/DCHECK 语义等价覆盖（已申报）。**二波 pool+free-list 落地达成**。
- **HC-2b memory-allocator（R5H 接力席收笼, 2026-10-12 主控独立复验采纳）**: 票面实态勘定 h 420 / .cc 832（旧叙事 536/716 与本仓源实态不符；h:535 friend MainAllocator 与 h:117 AllocateMemoryRange 实态不存在，friend 实为两个 Test*Scope，台账 16 承接）。终态 memory_allocator.rs 875 行: 前任已填体 29 函数（含 3 cfg(debug_assertions) DEBUG 面）+ 类型/字段/静态全落, R5H 补台账 21（DeleteMemoryChunk h:81+cc:766-779）/22（RegisterReadOnlyMemory h:327 悬空声明, 全树无定义无调用）, 推迟台账 22 项; todo! 0 / 空体 0 / 签名缺失 0。测试 memory_allocator_unittest.rs 996 行 19 例等价覆盖（原版 heap-allocator-unittest.cc 实测 53 行仅 TryResizeLargeObject 1 例, 主测 MainHeapAllocator 归 HC-2c）, R5H 修正前任 3 处误引（台账 2→21 ×2、cc:765→766、形参 MemoryChunk*→MutablePage*）。主控实测 nextest 64/64 + clippy(-D warnings) RC=0 + fmt 净。
- **L1 普查交叉波（伙伴主控方所有, 10-11 夜落地）**: heap 小族 7 骨架模块入 src（zapping/spaces_enums/slot_set/mutable_page_enums/index_generator/heap_enums/gc_callbacks, 未注册 lib.rs）+ 11 脚本入 tool/src/heap（zapping/spaces-enums/slot-set/mutable-page/index-generator/heap/gc-callbacks/gc-tracer/mark-compact/memory-reducer/sweeper; memory-pool 双脚本经翻案归我首代席产物）。普查目标形≠缺陷形终验, 归属 L1 台账, 本方席位全程禁碰（互斥清单已嵌各接力票）。
  - **HCR1-F4 勘误（R5H4, 本波）**: 上行「入 src」中 **spaces_enums.rs 系未挂 mod 的孤儿文件, 从未进编译面 → 实质「未落地」**; 全仓 grep 实证零 Rust 消费者（仅自身 + spaces.rs 台账文面提及 + tool 生成脚本/文档叙事）, 且其 MemoryFreeMode 与 memory_allocator.rs:105 FreeMode 构成重复型 → 已删除（「已落地」表述按此更正为「未落地/已删」）。PagedSpaceIteratorState 等枚举面待 paged-spaces 下波在其宿主模块自声明（spaces.rs 台账 3 同记）; 生成脚本 tool/src/heap/spaces-enums.js 复跑与否归主控裁决。
- **队列**: HC-2c main-allocator 签名骨架已开（R5H 富余面, 2026-10-12）: main_allocator.rs 998 行 83 todo! 位（72 pub fn + trait 面, 不填体）, 实态勘定 h 367 / .cc 983（票面 h552/cc1698 与 cc:1191-1359 行号叙事陈旧, 填体席按实态重勘; 模块头已申报）, 文件尾 HC-2c 推迟台账 1-5（-inl.h 实现面/空间实体族/运行态族/gc-tracer include/TimeDelta）→ 下波填体席。HC-4 spaces 族（L1 骨架 spaces_enums/mutable_page_enums 素材衔接待审查环裁决; HCR1 已裁 spaces_enums → 删, 见上节勘误; mutable_page_enums 衔接待裁）。factory/heap.cc 与 GC 入口归 heap-gc 波次另议。EXP-4 experimental 入口面让位伙伴（其 experimental/interpreter.rs 10-12 07:57 仍在写）。

### spaces 族测试复刻 (HCR1-F1, R5H4 波 2026-10-13 落地)

新增 `tests/spaces_unittest.rs`（11 例）与 `tests/large_spaces_unittest.rs`（19 例）, 复刻 HCR1 判「现行 mock 即可落」用例面; 原版参照 spaces-unittest.cc（1628 行, TEST_F 实测全依赖真 Isolate/Heap → 不可直迁, 申报同 HC-23R 测试口径）, 逐断言注释指原版 file:line 算例:

- spaces 侧: PageList 序语义（list.h:86-90/130-131/30-39/51-57/80 + 未命中 DCHECK should_panic）、PageIteratorImpl 前后置推进快照序（spaces-inl.h:23-34）、for_all 覆盖与零值界（spaces.h:57-62）、Account{,Un}committed 累计 + 回绕/下溢双 should_panic（base-space.h:54-62, 承接 F2 checked 形观测力）、first_page_address env 注入 + 空表违例（h:107-110）、Space 基类默认槽组（h:77-105 + cc:53-54 nullptr 语义）。
- large 侧: add/remove_page 三账对偶（cc:188-203, owner 恒等 + objects_size_ 不减惰性面）、update_accounting 舍入差算例（cc:242-254, cps=4096 旗标支定谳 + cc:244 违例 should_panic）、shrink_page_to_object_size 三分支+尾 DCHECK（cc:205-240: A 无缩零调用 / B PartialFreeMemory 双账回退 / C set_area_end 仅收区）、contains/contains_slow（cc:256-272, FromAddress 1<<18 掩码归位实测）、free_dead_objects 存活重算+摘页安全序+池旗标双支（cc:386-408 + cc:388 should_panic）、flip 调用序（cc:378-384, FROM_PAGE/TO_PAGE）、available 饱和/SetCapacity max（cc:371-376/410-412）、对象迭代器跳 Filler（cc:36-48）、tear_down 清空循环与对账差异金断言（cc:62-72 不减三账）、advance_and_invoke step-128 节奏 0/1/2/3/19/20/1（1:1 复刻 spaces-unittest.cc:160-197 期望数, 宿主换 cc:74-88; 含 cc:76 早退不推进门）、Code::RemovePage 顺序覆写（cc:428-431）、六支 wrapper id（cc:324-440）、F6 转发终抵 todo 体锚定（104-141 vs 333-369 文案区分）。
- 复刻期实测勘定: 本仓 `large_page_pool` 缺省 **true**（v8-flags defs.rs:4614 `FlagValue::Bool(true)`）→ free_dead_objects 池旗标双支断言按实态对调定位（缺省支 = kDelayThenPool, cc:391; 显式 false 支 = kImmediately, cc:392）, 原版 cc:390 语境无缺省值叙事, 不构成 src 病灶。
- 门禁实测（本波收笼复跑）: nextest **94/94 绿**（基线 64 + 新增 30）、clippy --all-targets -D warnings RC=0、`cargo +nightly fmt -p v8-heap-core -- --check` RC=0; todo! 净减 **14→12**（spaces 7 / large 5, F6 灭台账 C 组 2 处）。

## 三、收敛与终验复审日志- **Round 1 (全新只读 Reviewer 深度审查席, 目标 clean 1/3)**:
  - 规范与审查：严格依 `.agents/skills/rust_review/SKILL.md` 12 原则审查；
  - 发现并优化：清理 `large_spaces.rs` (5 处) 与 `spaces.rs` (7 处) 残留的 `#[allow(clippy::todo)]` 绕过注解，实现全包 `rg "\[allow" v8-heap-core` 零命中；
  - 语义与 SAFETY：逐行核对 `main-allocator.{h,cc}`、`spaces.{h,cc}`、`large-spaces.{h,cc}`、`memory-allocator.{h,cc}` 原版源码，内存分配器状态机、PageList 双向链表管理、FreeList 分类算法、原子记账与 SAFETY 注释严密完备；
  - 门禁实测：
    - 隔离编译目录 `target/agent_w7hcr` 下 `cargo nextest run -p v8-heap-core`：94/94 测试全通过；
    - `cargo clippy -p v8-heap-core --all-targets -- -D warnings`：0 警告；
    - `cargo +nightly fmt -p v8-heap-core -- --check`：格式完全干净；
  - 结论：0 语义缺陷 / 0 遗漏 / 0 allow 绕过 / 测试全部通过（**达成 clean 1/3**）。
- **Round 2 (全新独立 Reviewer, 目标 clean 2/3)**:
  - 规范与审查：严格依 `.agents/skills/rust_review/SKILL.md` 12 项原则（极致性能、零拷贝、单次遍历、现代 Rust、状态机、无 panic、无 allow 绕过等）严格审查；
  - 逐行源码比对：逐行比对 `v8/src/heap/main-allocator.{h,cc}`、`spaces.{h,cc}`、`large-spaces.{h,cc}`、`memory-allocator.{h,cc}` 原版源码与 Rust 实现，页链表管理 (`PageList` / `PageIteratorImpl`)、分配器三账核算 (`size_` / `objects_size_` / `committed_`)、FreeList 分类算法及 unsafe 契约边界严密等价；
  - 内存安全与 SAFETY：unsafe 边界严格限制在底层双链/池释放/虚表派发内部，100% 具备明确 `// SAFETY:` 注释，唯一公有 unsafe 函数 `next_ref` 包含严密 `# Safety` 文档；
  - 警告与规避检查：`rg "\[allow" v8-heap-core` 0 命中，无任何 allow 绕过，零死代码，零裸 panic / unwrap；
  - 门禁实测：
    - 隔离编译目录 `target/agent_w7hcr2` 下 `cargo nextest run -p v8-heap-core`：94/94 测试全通过；
    - `cargo clippy -p v8-heap-core --all-targets -- -D warnings`：0 警告；
    - `cargo +nightly fmt -p v8-heap-core -- --check`：格式完全干净；
  - 测试鉴别力审计：94 个测试用例覆盖 8 个测试套件，全面覆盖正反向边界断言、异常 panic (`#[should_panic]`)、Observer 步长节奏 (1:1 原版算例 0/1/2/3/19/20/1) 与状态转移，无恒真废话测试；
  - 结论：0 语义偏差 / 0 规范缺陷 / 0 遗漏 / 测试全过（**达成 clean 2/3**）。
- **Round 3 (全新独立终验 Reviewer, 目标 clean 3/3 终检验收冻结)**:
  - 规范与审查：严格按 `.agents/skills/rust_review/SKILL.md` 12 项原则（极致性能、零拷贝、单次遍历、现代 Rust、状态机、无 panic、无 allow 绕过等）进行终验独立审查；
  - 独立全量比对：逐行重核 `main-allocator.{h,cc}`、`spaces.{h,-inl.h,cc}`、`large-spaces.{h,cc}`、`memory-allocator.{h,cc}` 原版 C++ 与 `v8-heap-core/src/`：
    - 页链表与空间管理：`PageList` 尾插/摘除/空哨兵语义 1:1 对位 `list.h`；`PageIteratorImpl` 快照序与前向推进 1:1 对位 `spaces-inl.h:23-34`；`SpaceBase` 与 `Space` trait 默认槽、三账核算 (`committed_`/`size_`/`objects_size_`)、溢出/下溢检查 (`checked_add`/`checked_sub`) 严密闭环；
    - 大对象空间管理：`LargeObjectSpace` 与各特化空间 (`Old`/`New`/`Code`/`Shared`/`Trusted`/`SharedTrusted`) 1:1 对位 `large-spaces.{h,cc}`，含三分支 `shrink_page_to_object_size`、`contains`/`contains_slow` 快速/慢速归属判据、`free_dead_objects` 存活重算与摘页安全序、`update_accounting_after_resizing_object` 舍入对偶核算、Code 空间 `remove_code_memory_chunk` 顺序覆写；
    - 内存分配器：`MemoryAllocator` 静态提交页对齐、容量饱和算例、原子快速边界 `UpdateAllocatedSpaceLimits` (CAS 循环)、保守栈扫描 `normal_pages` 集合与 `large_pages` BTreeSet upper_bound 查找 1:1 对位 `memory-allocator.{h,cc}`；
    - 自由链表：`FreeList` 分类搜索、节点摘取与回池算法严格等价；
  - 内存安全与 SAFETY：全包 unsafe 块 100% 具备详尽 `// SAFETY:` 注释，唯一公有 unsafe 函数 `next_ref` 具有标准 `# Safety` 契约文档，指针边界与解引用均符合 V8 内存模型；
  - 警告与绕过检查：全包 `rg "\[allow" v8-heap-core` 0 命中，无任何 allow 绕过，零死代码，零裸 panic / unwrap；
  - 门禁实测：
    - 隔离编译目录 `target/agent_w7hcr3` 下 `cargo nextest run -p v8-heap-core`：94/94 测试全通过；
    - `cargo clippy -p v8-heap-core --all-targets --no-deps -- -D warnings`：0 警告；
    - `cargo +nightly fmt -p v8-heap-core -- --check`：格式完全干净；
  - 测试鉴别力审计：94 个单元/集成测试全面覆盖，包含正常路径、正反边界、溢出回绕 panic (`#[should_panic]`)、Observer 步长节奏 (0/1/2/3/19/20/1 原版算例)，无任何废话测试；
  - 结论：0 语义偏差 / 0 规范缺陷 / 0 遗漏 / 测试全过 / 0 警告 / 0 allow 绕过（**达成 clean 3/3 终检验收，正式冻结**）。

## 四、中断接力席位 W10HCb (2026-10-13)

前任 W10HC 提供方中断，本席按 7.5「实测盘点 → 只补差集」接力，禁重做已落地内容。

- **实测盘点 (复跑为准)**: src 18 文件 8979 行 / 可执行 `todo!` 156 (非-skel 72: main_allocator 50 + spaces 11 + large_spaces 11, 其中可执行体 spaces 7 / large 5 系深水, 与 readme A-E 台账及 main_allocator 文末 HC-2c 台账逐行对齐, 无缺登, 禁重填; .skel 84) / `.skel` 草案 78 / cargo check 0 error。
- **差集处置**:
  1. 挂载面 (spaces/large_spaces/main_allocator): 复跑核对深水面全部已登台账, **零重填** (遵「禁重做已落地」+「台账内不重做」)。
  2. .skel 收编第一批: 挂载 **slot_set 族** (唯一依赖面已就位、纯逻辑自含件) — slot_set.rs (枚举/常量) + slot_set_cc.skel (机械规则) 合并入编译面, 导入校正 `crate::common`→`v8_common`/`v8_heap_base::SlotCallbackResult`; **1:1 复刻 TypedSlots/TypedSlotSet 数据结构体** (insert/ensure_chunk/merge/iterate/clear_invalid_slots/assert_no_invalid_slots/iterate_slots_in_ranges), 裸指针单链以 `VecDeque<Chunk>` 安全建模, **零 unsafe**; 收编后删 absorbed 草稿 slot_set_cc.skel。新增依赖 v8-heap-base (todo.md L3 DAG heap-core→heap-base 边, cargo add)。
  3. 不强挂深水面登组: mutable_page_cc / object_lock / pending_allocations / memory_balancer / read_only_heap / memory_measurement (依赖 MutablePage/RawPage/Isolate/Heap/v8-API/Promise/TimeDelta 未落) → readme「.skel 收编台账」skel-F 组登记 (组|符号|前置|解锁 crate)。
  4. 脚本核查: slot-set.h.js/.cc.js 本族基线生成器, 输出含 `/// v8:` 锚, 确定性幂等; 收编后挂载 slot_set.rs 以 src 现态为准, **未复跑生成器避免覆写** (十一「复跑仅限重生成基线」)。
  5. 测试 1:1: 新增 `tests/slot_set_unittest.rs` 复刻 slot-set-unittest.cc 三支 TypedSlotSet 用例 (Iterate/ClearInvalidSlots/Merge) + 机械规则面对偶, 共 4 例, 原用例名, ctor log_init, Void/OK 尾, 无临时文件名。
- **门禁实测 (CARGO_TARGET_DIR=target/agent_W10HCb)**: clippy `--all-targets -- -D warnings` **0 警告**; nextest **98/98 绿** (基线 94 + 新增 4); nightly fmt `--check` 净; **A 档 miri** `cargo +nightly miri nextest run --test slot_set_unittest` **4/4 绿** (挂载零 unsafe, 对账)。todo! 可执行面 156 → 148。
- **台账增减**: readme 新增「.skel 收编台账 (W10HCb)」节 (slot_set 收编记录 + skel-F 六件推迟分组 + 脚本口径); A-E 台账无动 (核实在位)。稳定态: 可编译 + 三门禁绿 + 台账完整, 无红态残留。





## 五、Wave 10 round1 审查 (W10HCR, 2026-10-13)

独立 Reviewer 全新席位, clean 计数 1/3。重点面 = W10HCb 收编 slot_set 族 (src/slot_set.rs + tests/slot_set_unittest.rs) 逐行对抗审查; 兼顾 main_allocator/spaces/large_spaces 挂载面与 readme 台账一致性。只读审查 + 补 tests/ (本席新增 3 例入既有 slot_set_unittest.rs, 无临时文件名, 12.8 合规); 未改任何 src 语义体。

**并发席说明**: 审查期间 slot_set.rs / 测试面被开发席在飞更新 (本席读后已现新版: 常量面改引 v8_common::K_PAGE_SIZE_BITS + v8_heap_base::K_CELLS_PER_BUCKET/K_BITS_PER_CELL; typed_slot_encode 增 debug_assert 前置; 新目标 tests/slot_set_typed_slots_unittest.rs 16 例)。中途一次瞬红按 readme 门禁口径「并发席瞬红按面归因」豁免, 终态以下述复跑为准。

### 核验通过面 (无 finding)

1. **语义 1:1** (原版 slot-set.h/.cc ↔ src/slot_set.rs 逐行): Insert cc:24-29 / EnsureChunk cc:46-54 (空则初块 100, 头满头插 NextCapacity 扩容) / NewChunk cc:56-62 (reserve↔with_capacity+cap 逻辑容量) / Merge cc:31-44 (他空早退:32-34, 己空接管:35-37, 否则己尾接他头, 己序全先; VecDeque::append 等价) / Iterate h:325-362 (head→尾序 h:327/:359; cleared 跳过 h:334; KEEP 计数+置 cleared 语义; FREE_EMPTY_CHUNKS 摘块 h:346-355 等价重建 kept 保序; 返回 int↔i32) / ClearInvalidSlots↔IterateSlotsInRanges cc:77-100 (空区间早退 cc:80; cleared 跳 cc:86; std::map::upper_bound 回退判定 cc:88-94 ≡ slot_in_ranges partition_point, 终点排他, 双测+const 断言对账)。**块链头尾序/invalid slot 判定/迭代序全等**。
2. **零 unsafe 属实**: src/slot_set.rs 全文无 unsafe 块 (仅文面注述 :11); 棘轮无增长来自本模块。VecDeque<Chunk> 与裸指针分块链的容量阶梯 (100→倍增→16384 饱和, h:293-297) / 增长边界等价; Merge O(1) 拼链→O(n) 元素移动系性能差非语义差 (登记观察, 不报缺)。
3. **单线程语义化登记属实**: 原版并发面 = LoadNext/StoreNext/LoadHead/StoreHead relaxed atomic (h:377-386, cc:82/:98 消费) + Iterate 写侧 StoreHead/StoreNext (h:350-352); src 头 :12-15 与 readme:74② 均已登记推迟, 引用准确。SlotType/IterationMode 判别值逐行对 h:234-266/:310 (kCleared=6=kLast)。
4. **main_allocator 50 todo! ↔ HC-2c 台账 1-5**: 可执行 todo! 实测 50 (文面提及 1 处不计), 逐条锚 cc/-inl 归组: -inl 族 4 (677/702/717/751)→台账 1; cc:332-344 (892)→台账 5; 其余空间实体/运行态族→台账 2/3 (CreateAllocatorPolicy cc:51、cc:446、cc:589-636、cc:159-204、cc:801-834 均台账点名位)。**无缺登、无虚登深水** (本轮未抓到本可填却谎登项)。
5. **spaces 7 / large_spaces 5 可执行 todo!** 实测与 readme A-E 组数全等 (C 组 2 处已灭核实为直调转发)。
6. **skel-F 六件中五件依赖属实**: MutablePage/RawPage/RememberedSet 实体未挂 (mutable_page_enums.rs 系孤儿未入 lib 树); ReadOnlySpace/EarlyReadOnlyRoots 全仓无挂载面; Promise::Resolver 无落地; Heap=memeory_allocator.rs:61 空 enum opaque (pending_allocations 7 项封锁属实, Snapshot::contains 纯面已在 skel 内实现=诚实登记); main_allocator 三策略 ctor 的 Self{allocator, space} 字段对位 h:56-58/74-75/97 + h:106 委托件面复核通过。
7. **原 gtest 复刻忠实**: slot-set-unittest.cc 三支纯逻辑用例逐断言对位 (:62/:99/:129, kDelta/entries/区间集/期望数全等), 余三支 PossiblyEmptyBuckets/SlotSet 推迟属实 (所依赖 SlotSet 页位图类确未挂, 位图基座在 v8-heap-base/basic_slot_set.rs)。

### 缺陷 (4) 与遗漏 (3)

- **D1 [must-fix M1] skel-F memory_balancer 行半虚** — 前置登记 "TimeDelta/TimeTicks + Heap/Isolate 运行态", 但 TimeDelta/TimeTicks **已落** (v8-base-platform/src/lib.rs:54 pub use; time.rs:51/:607, in_milliseconds_f :175), 且 update_allocation_rate/update_gc_speed 方法体零 heap_ 引用系纯函数 (v8/src/heap/memory-balancer.cc:74-84 / :63-71) ↔ v8-heap-core/src/memory_balancer.rs.skel:44/:46。**两件属「本可填」**, 仅 heartbeat_update (cc:86+ heap_->OldGenerationSizeOfObjects) / recompute_limits (cc:44-59 heap_->limits()) 真依赖运行态。
- **D2 [must-fix M2] CHECK→debug_assert 降级未上台账** — typed_slot_encode 偏移越界原版 CHECK 硬失败 (v8/src/base/bit-field.h:100, slot-set.cc:25 消费) ↔ slot_set.rs:97-100 debug_assert; AssertNoInvalidSlots 原版 CHECK_WITH_MSG 恒致命 (slot-set.cc:72) ↔ slot_set.rs:355 debug_assert (release 剥除)。src 注释/测试头已申报 (slot_set.rs:88-89/:343), readme:71-74 台账无一行, 违 12.6/12.1 登记纪律。
- **D3 [must-fix M3] MainAllocator 自引用裸指针隐患 (本波新增 unsafe 面)** — 原版 cc:48-49 `allocation_info_(allocation_info ? allocation_info : &owned_allocation_info_)` 直译成同结构体内 *mut 指向自身字段 (main_allocator.rs:478-483), Rust move 即悬垂; start/top/limit/extend_lab/allocation_top_address/allocation_limit_address/is_lab_valid 共 7 处 unsafe 解引用 (532-581/:908/:965) 的 SAFETY 注释「ctor 恒置有效」暂不成立 (new/new_in_gc=todo! :516/:527, 不可达但注释超前)。填 ctor 票必须登记「构造后禁 move」约束或改 Box/间接持有, 否则引入真 UB 面 (miri 现覆盖不到)。
- **D4 [must-fix M4] 锚点失实 5 处** — ① slot_set.rs:60/:397 "slot-set.h:120/122 kCellsPerBucket/kBitsPerCell": 实位 basic-slot-set.h:277/:281 (slot-set.h:120=bitmap_ 字段, :122=FRIEND_TEST); ② slot_set.rs:267 "h:385 page_start_": 实位 slot-set.h:392 (:385=StoreHead 体); ③ slot_set.rs:247 "cc:13-22 迭代基座": cc:13-22 系析构链, 迭代基座=h:327-359/cc:82-99; ④ main_allocator.rs:479/:539 "cc:45-46 三目": 实位 cc:48-49; ⑤ readme zh.md:74 "slot-set.h:60-231": 实位 h:35-122 (PossiblyEmptyBuckets) + h:127-232 (SlotSet), src 头 :12-13 已改对而 readme 未跟。12.1 对账以锚为准, 逐处必改。
- **O1 [归 M1] object_lock 台账行文陈旧**: "`Isolate`/`Tagged<HeapObject>` 实体"未落不实 — Isolate struct 已落 v8-execution-isolate/src/isolate.rs:16, Tagged/HeapObject 已落 v8-objects-core/src/tagged.rs:33 与 heap_object.rs:8 (后者系本 crate 依赖)。推迟结论仍成立 (MutablePage::FromHeapObject 缺 + 无 execution-isolate 依赖边), 措辞需勘正以免误导解锁面。
- **O2 [归 M2] readme 门禁计数漂移**: readme:72 "miri 4/4" / :73 "共 4 例" / :92 "98/98 绿" — 现实测 slot_set 域 = slot_set_unittest.rs 7 例 (4+本席 3) + slot_set_typed_slots_unittest.rs 16 例 = 23 例, 全量 117/117, miri 7/7。
- **O3 [已补] 测试盲区**: 收编 4 例全走 KEEP_EMPTY_CHUNKS + 少量槽, FREE_EMPTY_CHUNKS 摘块 / iterate 返回计数 / 跨 chunk 头插序 / merge 空方三支 / assert 违例面零覆盖 → 本席补 typed_slot_set_chunk_growth_order_gold / typed_slot_set_merge_edge_gold / typed_slot_set_clear_assert_edge_gold 三例入既有文件 (逐断言注 h:327-362/cc:31-54 原版锚); 并发席另补 16 例新目标, 两面互补不删。

### 机器门禁复跑 (CARGO_TARGET_DIR=target/agent_W10HCR)

- `cargo nextest run -p v8-heap-core`: **117/117 绿** (含 slot_set 域 23/23)。
- `cargo clippy -p v8-heap-core --all-targets -- -D warnings`: **0 警告**。
- `cargo +nightly fmt -p v8-heap-core -- --check`: 净 (本席补例后经 fmt 一次)。
- A 档 `cargo +nightly miri nextest run -p v8-heap-core --test slot_set_unittest`: **7/7 绿** (含本席新增 3 例; 零 unsafe 面对账)。

### must-fix 清单 (交整改席)

1. M1: readme skel-F memory_balancer 行改准 (两件纯函数面可随挂载票填, 两件运行态封锁); object_lock 行勘正 Isolate/Tagged 已落实态。
2. M2: readme slot_set 组补 CHECK→debug_assert 降级口径一行; 门禁/例数计数刷新实测 (4→23 例域, 98→117, miri 4/4→7/7)。
3. M3: main_allocator 文末台账登记 allocation_info_ 自引用指针禁移约束 (ctor 填体票承接, 或改 owned 间接化)。
4. M4: 五处锚点勘正 (basic-slot-set.h:277/281; slot-set.h:392; cc:82-99; main-allocator.cc:48-49; slot-set.h:35-122/127-232)。

**三数**: 缺陷 4 / 遗漏 3 / 测试 117/117 绿 (slot_set 域 23/23, clippy 0 警, A 档 miri 7/7)。clean 计数不成立 (本轮抓到台账失实与锚点缺陷), round1 未过, 待整改后复审。

### 复核补记 (W10HCR, 同轮终态复跑)

- 并发开发席在审查期间多次 amend init 头 (cdb9e3e→ba15292→b5f50e7) 并 live 修订 slot_set.rs/main_allocator.rs/tests; 本席全部结论已按**终态工作树**复核。
- 已吸收整改 (对席自修, 本席复验): ① typed_slot_encode 补双 debug_assert (bit-field.h:91/:100 双引用齐, release 掩蔽口径注释自述); ② 锚点 basic-slot-set.h:277/:281 与迭代基座 h:327-359 已改对; ③ LAB 访问器全部带 SAFETY 注释; ④ 新增 slot_set_typed_slots_unittest.rs 17 例 + 本席 slot_set_unittest.rs 3 例边界。
- 残留 must-fix (终态仍未改):
  - R1: slot_set.rs:280 锚 "h:385 page_start_" 应为 slot-set.h:392 (原版 h:385 非该声明)。
  - R2: main_allocator.rs:479/:507/:539 三目锚 "cc:45-46" 应为 main-allocator.cc:48-49。
  - R3: readme 台账失实四处 — skel-F memory_balancer 前置仍列 "TimeDelta/TimeTicks" (v8-base-platform lib.rs:54 已落地, 4 项中 2 项纯 EWMA 本可填) 与 object_lock 前置列 Isolate/Tagged<HeapObject> (v8-objects-core tagged.rs:33 / heap_object.rs:8 / v8-execution-isolate isolate.rs:16 已落地); :72 "miri 4/4"、:73 "共 4 例"、:92 "98/98" 均过期 (实 24/24、slot_set 域 24 例、全 crate 118/118); :67/:93 "main_allocator 50" 现值 49 (访问器已落地 1 处)。
  - R4: main_allocator allocation_info_ 自指裸指针 (cc:48-49 三目落 owned_allocation_info_) 的 no-move/Pin 约束未在台账登记, ctor 填充前须挂 HC-2c 深水约束。
- 终态门禁: nextest 118/118 绿; clippy --all-targets -D warnings 0 (exit 0); nightly fmt 净; A 档 miri slot_set 双 target 24/24 绿。零 unsafe 声明 (slot_set.rs) 终态核验属实 (仅 :12 文档文字)。
- **三数 (终态)**: 缺陷 4 / 遗漏 4 / 测试 118/118 绿 (slot_set 域 24/24, clippy 0 警, A 档 miri 24/24, fmt 净)。clean 计数不成立, round1 未过; R1–R4 交整改席。


## W10HCR round1 复审 (2026-10-13)

同名审查席第二轮 (对抗复扫, 不采信对席结论, 全部锚点回原始 C++ 一手核对)。范围: `src/slot_set.rs` (440 行) + `tests/slot_set_unittest.rs` (390 行) 逐行, `main_allocator.rs`/`spaces.rs`/`large_spaces.rs` 挂载面与 `readme/zh.md` 台账对账, `.skel` 77 件面盘点。**本席改动了 src** (仅明确缺陷与锚点, 零重构零跨 crate 零 lib.rs 变更, 未 commit)。

### 一、语义 1:1 复核 (原版 ↔ src, 逐函数一手核对)

| 面 | 原版锚 | src | 结论 |
|---|---|---|---|
| Insert | slot-set.cc:24-29 | `TypedSlots::insert` | 全等 (尾块满则 EnsureChunk 后 push) |
| EnsureChunk | cc:46-54 | `ensure_chunk` | 全等: 空链初块 `kInitialBufferSize=100` (h:289), 否则 `NextCapacity` **头前插** — 本 revision 无 `tail_`, 头即最新块; 若按旧版尾插会误判「迭代序反」, 故此处经 h:327 `chunk = head_` 反向坐实 |
| NewChunk | cc:56-62 | `new_chunk` | `reserve`↔`with_capacity`+`extend` 逻辑容量等价, h:60 `DCHECK_EQ(capacity)` 落 const/debug_assert |
| Capacity 阶梯 | h:293-297 `std::min({kMaxBufferSize, capacity*2})` | `next_capacity` | 全等: 100→200→400→…→16384 饱和 (用例 B/C 实测 3100 槽阶梯 [1500,3100)/[700,1500)/[300,700)/[100,300)/[0,100) 与 41885 槽头块起点 25500 对账) |
| Merge | cc:31-44 | `merge` | 三支全等: 他空早退 :32-34 / 己空整链接管 :35-37 / 否则己尾接他头 (己序全先); VecDeque `append` 系 O(n) 元素移动, 原版 O(1) 拼链 — **性能差非语义差**, 登记观察不报缺 |
| Iterate | h:325-362 | `iterate` | 头→尾序 :327/:359 全等; cleared 跳过 :334; `KeepSlot` 计数+否则置 cleared; `FreeEmptyChunks` 空块摘除 :346-355 ≡ 重建 kept 保序; 返回 `int`↔`i32` 不转 usize |
| ClearInvalidSlots | cc:64-67 | `clear_invalid_slots` | 全等 (转发 `IterateSlotsInRanges` 置 cleared) |
| IterateSlotsInRanges | cc:77-100 | `iterate_slots_in_ranges` | 空区间早退 cc:80; cleared 跳 cc:86; **`upper_bound` 回退判定 cc:88-96** (`== begin()` 跳, 否则 `--`, `second > offset` 命中) ≡ `partition_point` + 终点排他; 上轮「乱序 slice 静默漏清」已由 `debug_assert_sorted` 双挂点封锁 (原版 std::map 天然有序) |
| AssertNoInvalidSlots | cc:69-75 | `assert_no_invalid_slots` | 命中即 `debug_assert` (原版 CHECK_WITH_MSG :72 恒致命) — 降级须上台账, 见 M2 |
| BitField 布局 | h:268-274 / bit-field.h:111 | `typed_slot_encode/decode_*` | 29 位偏移 + 3 位类型 (`kTypeShift=29`), decode `(v & mask) >> shift` 全等; kSlotTypeCleared=6=kLast (h:234-266/:310) |

**零 unsafe 声称核验: 属实** — `src/slot_set.rs` 全文 `unsafe` 仅 :12 文档文面一处, 无 unsafe 块/项; A 档 miri 24/24 绿作对账 (非因 unsafe 必需, 系门禁纪律)。

### 二、本席缺陷处置 (5, 均已落树)

- **F1 [P3, 已修] 区间序前置条件未校验** — `slot_in_ranges` 的 `partition_point` 只在起点升序时等价 `std::map::upper_bound` 回退; 原版容器有序故不可能违例, Rust `&[(u32,u32)]` 可乱序 → 新增 `debug_assert_sorted` (锚 cc:88) 挂 `iterate_slots_in_ranges` 早退后 + `assert_no_invalid_slots` 首行, 补违例用例。
- **F2 [P3, 已修] 常量二次导出 (AGENTS 红线)** — 本地 `K_PAGE_SIZE_BITS/CELLS_PER_BUCKET/BITS_PER_CELL` 与上游重复 → 改直承 `v8_common::K_PAGE_SIZE_BITS` + `v8_heap_base::{K_CELLS_PER_BUCKET,K_BITS_PER_CELL}`, const block 加 `assert!(==32)` 交叉核对 (一手确认 basic-slot-set.h:277/:281 = 32/32; slot-set.h:132 仅消费)。
- **F3 [P3, 已修] `typed_slot_encode` 越界静默掩蔽** — 原版偏移越界 `CHECK` 硬失败 (bit-field.h:100), 类型越界 `DCHECK` (:91) → 补双 `debug_assert` 并双锚; release 掩蔽口径入注释 (台账仍缺, M2)。
- **F4 [P4, 已修] 锚点失实 3 处** — ① `slot_set.rs:280` "h:385 page_start_" → **h:392** (实测 `Address page_start_;` 在 h:392, :385 系 `StoreHead` 体); ② `slot_set.rs:379` "h:80/h:86" → **cc:80/cc:86** (ranges.empty 早退与 cleared 跳过均在 slot-set.cc); ③ `main_allocator.rs:479/:507/:539` "cc:45-46 三目" → **cc:48-49** (实测 `allocation_info_` 三目在 main-allocator.cc:48-49)。上席 R1/R2 列而未修, 本席清。
- **F5 [P4, 已修] 既有测试断言语义反向** — `slot_set_unittest.rs` 原断 `typed_slot_decode_offset(1<<29) == 0` 把「越界静默截断」当规格 (原版是 CHECK 硬失败) → 改写为合法域 `[0, kMaxOffset)` 表 + 违例口径注记, 指向新目标 `typed_slot_encode_out_of_range_offset_panics`。

### 三、遗漏 (6, 台账/测试缺口, 交整改席)

- **O1 [P2] `readme/zh.md:67` 台账口径不实 (四数同时失真)** — 「可执行 `todo!` 156」: 现 raw grep 148 / **真可执行体 137** (非-skel 61 = main 49 + spaces 7 + large 5, + skel 76; 余 11 处系注释/doc 文面: main_allocator.rs:4、spaces.rs:39/705/769/774、large_spaces.rs:36/705/1072/1082/1087/1096); 「非-skel 72」与其分项「50/7/5」= 62 自相矛盾 (72 系含注释的 raw 值); 「.skel 84」实 **76**; 「.skel 草案 78」实 **77**; 「src 18 文件 8979 行」现 **9363 行**。票面「slot_set.rs 1235 行零 unsafe」→ 挂载 440 行 + 测试 390/533 行, 无 1235 这个量。
- **O2 [P2] CHECK/DCHECK→debug_assert 降级未上台账** — `typed_slot_encode` 与 `assert_no_invalid_slots` 的 release 语义差 (致命 vs 剥除) 仅见 src 注释, `readme/zh.md:71-74` 台账零行, 违 12.6/12.1。
- **O3 [P3] skel-F memory_balancer 行半虚** — 前置登记 "TimeDelta/TimeTicks" 已在 `v8-base-platform/src/lib.rs:54` 落地, 且 `update_allocation_rate`/`update_gc_speed` 体零 `heap_` 引用 (memory-balancer.cc:63-71/:74-84) 系**本可填**; 仅 heartbeat_update/recompute_limits 真封运行态。(对席 O1 另指 object_lock 行 Isolate/Tagged 已落, 同批勘正。)
- **O4 [P3] `.skel` deferred 文案纪律 0/76** — 76 处 skel `todo!` 全无「前置; 解锁 crate」文面 (`memory_allocator.rs.skel:25/:27/:29` 裸 `todo!()`、`mutable_page_cc.skel:40-65` 仅「就地修订」); `main_allocator.rs` 另有 **10 处锚-only** `todo!("就地修订: cc:xxx")` 无原因 (760/766/808/822/828/892/982/1018/1057/1081), 与 :67「逐行登记」措辞不符。
- **O5 [P3] 一名一主冲突** — `main_allocator.rs:58-61` `pub enum SpaceWithLinearArea {}` (opaque 占位) 与 `spaces.rs:594` `pub trait SpaceWithLinearArea: Space` 双主, 收敛票未登记 (跨票面二义, HC-2c 填 ctor 前必收)。
- **O6 [P4] 推迟面锚与文件行数失实** — `readme/zh.md:74` "slot-set.h:60-231" 应作 h:35-122 (PossiblyEmptyBuckets) + h:127-232 (SlotSet 页位图类) + h:377-386 (并发原子面); 「slot-set.h 388 行」实 **398 行** (src 头已改对, readme 未跟)。

### 四、新增测试 (17)

`v8-heap-core/tests/slot_set_typed_slots_unittest.rs` (533 行, 17 例, 无临时/round 文件名, `#[ctor::ctor(unsafe)]` log_init, 返 `aok::Void` 以 `OK` 收尾, 显式导入)。原版逐断言/算例对位:

A/B/C 增长面 (`typed_slots_growth_prepends_head_chunk` 105 槽→[100..105)++[0..100); `..._capacity_doubling_ladder` 3100 槽五段阶梯; `..._capacity_saturates_at_max_buffer` 41885 槽 16384 饱和); D/E 迭代模式 (`..._free_empty_chunks_deletes_chunk` / `..._keep_empty_chunks_retains_cleared_capacity` — cleared 项仍占容量, 扩容边界在第 101 次插入); F 返回计数; G page_start 地址基 (K_PAGE_START=0x7f00_1000, 区间按 offset 而非 addr 键); H 空链双模式; I 空区间 no-op (cc:80); J **upper_bound 回退怪癖实测** (探 5/10/15/19/20/24/25/26 × 区间 [(10,30),(20,25)] → 存活 [5,25,26], 逐点注 cc:88-96); K1/K2 assert 净/违例; L1/L2/L3 encode 越界偏移/越界类型/乱序区间 `#[should_panic]`; M/N merge 空方整链接管 + 跨增长 merge 保序。与对席 `slot_set_unittest.rs` 3 例互补, 两面并存不删。

### 五、must-fix 清单 (按严重度)

1. **M1 [P2]** `readme/zh.md:67/:93` 四数勘正 (156→137 真体/148 raw 并标口径; 分项 49/7/5+skel 76=137; .skel 77 件 76 处 todo; 9363 行) + 票面「1235 行/可执行 148」口径统一。
2. **M2 [P2]** readme slot_set 组补 CHECK/DCHECK→debug_assert 降级口径一行 (src 注释 + 违例用例已在树, 台账缺行)。
3. **M3 [P3]** skel-F memory_balancer 行改准 (两件纯 EWMA 随挂载票可填, 两件运行态封锁); object_lock 行勘正 Isolate/Tagged 落实态。
4. **M4 [P3]** `SpaceWithLinearArea` 一名一主收敛登记 (opaque enum ↔ trait)。
5. **M5 [P3]** skel 76 处 + main_allocator 10 处锚-only todo! 补「前置; 解锁 crate」文面 (与 M1 同批改 readme:67「逐行登记」措辞)。
6. **M6 [P4]** readme:74 推迟面锚 (h:35-122/127-232/377-386) 与 slot-set.h 398 行勘正。
7. **M7 [P4] 观察项 (不计本 crate 遗漏)** — `lib.rs` 13 处 `pub mod` 而非 `pub use` 系基线债 (AGENTS 二次导出红线), 归主控清; 另 `merge` O(n) 元素移动 / `slot_type_from_u8` 对域外值 (7) 归 `Cleared` 与原版 `static_cast` 未校验同形, 均登记不报缺。

### 六、机器门禁实测 (CARGO_TARGET_DIR=target/agent_W10HCR, 单包 -p v8-heap-core, 未跑全仓脚本)

- `cargo nextest run -p v8-heap-core`: **118/118 绿**, RC=0 (slot_set 域 24/24 = slot_set_unittest 7 + slot_set_typed_slots_unittest 17)。
- `cargo clippy -p v8-heap-core --all-targets -- -D warnings`: **0 警告**, RC=0。
- `cargo +nightly fmt -p v8-heap-core -- --check`: 净, RC=0。
- A 档 `cargo +nightly miri nextest run -p v8-heap-core --test slot_set_unittest --test slot_set_typed_slots_unittest`: **24/24 绿**, RC=0 (nightly miri 工具链在位, 无缺失豁免)。
- 一切命令带 `--config 'term.quiet=false'` (本 crate `.cargo/config.toml` quiet 吞诊断)。

**三数 (本席复审)**: 缺陷 **5** / 遗漏 **6** / 新增测试 **17** (全 crate 118/118 绿)。**已改 src**: `slot_set.rs` (F1 debug_assert_sorted / F2 常量直承 / F3 双 debug_assert / F4-①②锚点) + `main_allocator.rs` (F4-③锚点 3 处) + `tests/slot_set_unittest.rs` (F5 断言语义) + 新目标 `tests/slot_set_typed_slots_unittest.rs`。clean 计数不成立: 缺陷面本席已清零且三门禁 + miri 全绿, 但台账失实项 (O1–O6) 属 readme 面, 须整改席落笔后判过。

## 六、W10HCS round1 整改 (2026-10-13)

整改席 W10HCS, 处理 W10HCR round1 残留 must-fix R1–R4。铁律执行: 只修点名项, 零重做已绿内容。

### R1–R4 逐项处置

- **R1 [闭合, 让位外部席]** `slot_set.rs:280` "h:385 page_start_" → **h:392**: 本席回原始 v8/src/heap/slot-set.h 一手核实 (`Address page_start_;` 实位 :392, :385 系 StoreHead 体), 核毕发现对席已于 01:07:30/01:09:10 自修入树 (复审补记 F4-①「上席列而未修, 本席清」), 本席让位不回写, 仅磁盘现态验证通过 ("h:385" 全文件零命中)。**归外部席**。
- **R2 [闭合, 让位外部席]** `main_allocator.rs` 三目锚 `cc:45-46` → **cc:48-49** (现位移 :481/:509/:541, 对席 01:23:55 推进后行号+2): 本席一手核实 main-allocator.cc:48-49 三目属实; 对席 F4-③ 已自修, 本席让位不回写, 现态验证 "cc:45-46" 零命中。**归外部席**。
- **R3 [本席落笔, readme/zh.md 台账修正 9 处 + 填体 1 件]**:
  - **memory_balancer 填体**: 核实 TimeDelta/TimeTicks 已落 (v8-base-platform time.rs:51/:607, lib.rs:54, in_milliseconds_f :175), update_allocation_rate (cc:74-84)/update_gc_speed (cc:63-72) 方法体零 heap_ 引用 → `memory_balancer.rs.skel` 收编挂载 lib.rs (本席唯一 src 面新增文件, 无对席冲突面), **填体 3 符号** (两件纯函数 + cc:14-15 纯存储 ctor 作测试构造面), heartbeat_update (cc:86-100)/recompute_limits (cc:17-25) 真前置 Heap 运行态 (OldGenerationSizeOfObjects/limits()/GetForegroundTaskRunner cc:21/:44/:105 + CancelableTask h:87/cc:109-112) 留 todo! 逐处锚注; 字段面补齐 h:77/:78 双 optional + h:63/:74/:82/:84 缺省; absorbed 草稿 skel 删除 (slot_set_cc 先例)。**新测 tests/memory_balancer_unittest.rs 8 例** (原版无专名 unittest, v8/test/unittests grep 零命中, 等价覆盖口径同 HC-23R 申报, 逐断言注 cc/h 算例)。
  - **object_lock [核实后不改登「可填」]**: Lock/Unlock/Drop 全体经 `MutablePage::FromHeapObject` (object-lock-inl.h:19/:25), MutablePage 实体未挂 → 推迟结论成立, 但「Isolate/Tagged<HeapObject> 实体未落」失实行已按落实态勘正 (v8-execution-isolate/src/isolate.rs:16 / v8-objects-core/src/tagged.rs:33 / heap_object.rs:8 本席逐处亲验)。
  - **四处过期计数改实测**: :72 miri 4/4→**24/24** (本席实跑 slot_set 双 target miri 24/24 RC=0 佐证), :73 共 4 例→**24 例** (7+17 逐文件计数), :92 98/98→**118/118** (+本波挂 balancer 后 126/126 注记), :67/:93 main_allocator 50→**49** (本席 grep 复核); 另随复审席点名口径把 :67 盘点行整体勘正 (真可执行 todo! 135 = 挂载 63 + skel 72, 草案 76 件, src 19 个 .rs 9530 行, raw/可执行混用口径作废)。
  - **同组补登**: :74 锚 "slot-set.h:60-231" → h:35-122/127-232 (M4-⑤/M6, src 头已对 readme 跟改); CHECK/DCHECK→debug_assert 降级口径台账行 (D2/M2/O2, src :94/:98/:356 与测试 6 should_panic 本席亲验后落笔); SpaceWithLinearArea 一名一主冲突登记 (O5/M4, main_allocator.rs:63 enum ↔ spaces.rs:594 trait, 只登记归收敛票, 未碰两文件)。
- **R4 [闭合, 台账登记路径]**: `allocation_info_` (h:344) 三目落 `&owned_allocation_info_` (h:345-346/cc:48-49) 的**构造后禁 move/Pin 约束**已登记 readme/zh.md「W10HCS 勘正」节, 约束承接三点 (ctor 填体票 SAFETY 前提 / owned 支改 Box-间接持有裁决位 / 违则真 UB 面 miri 覆盖不到申报)。实现面全仓 grep 无 `Vec<MainAllocator>`/按值存储移动路径 → 按「最小侵入」口径**不动 main_allocator.rs** (该文件系对席热面 00:30→01:07→01:23 三次推进, 拉扯禁入)。

### 门禁实测 (CARGO_TARGET_DIR=target/agent_W10HCS, 单包 -p v8-heap-core, 全带 --config 'term.quiet=false')

- `cargo nextest run -p v8-heap-core`: **126/126 绿** (基线 118 + balancer 8), RC=0 (终态复跑见下)。
- `cargo clippy -p v8-heap-core --all-targets -- -D warnings`: **0 警告**, RC=0 (Checking v8-heap-core 行实证, 含对席 01:23:55 推进后的 main_allocator.rs 现态)。
- `cargo +nightly fmt -p v8-heap-core -- --check`: 净, RC=0 (仅本席两文件回格式, 对席热文件 mtime 未动实证)。
- A 档 miri: 本席新测 `--test memory_balancer_unittest` **8/8 绿** RC=0; 对账 `--test slot_set_unittest --test slot_set_typed_slots_unittest` **24/24 绿** RC=0。

### 台账修正行数与让位登记

readme/zh.md 改动 **9 处落点** (:67 盘点行勘正 / :72 miri / :73 例数 / :74 锚 / :76 不强挂抬头勘正 / :81 object_lock 行 / :83 memory_balancer 行出组 / :92 门禁行 / :93 计数行) + 新增「W10HCS round1 整改勘正」节 4 条 (含 R4 约束登记)。**让位文件**: slot_set.rs / main_allocator.rs (对席在飞自修 R1/R2, 本席验证不回写); 零对席测试文件触碰。瞬红观察: 本席全程无瞬态红, 终态门禁以本节复跑为准。
