# Wave 派发台账 (waves)

> 2026-10-12 自 todo.md 收敛: 各轮 Wave 派发表 / 席位禁碰 / 门禁口径 / Wave 2+ 消账记录。
> 席位文件表在 `target/wave<n>/<席名>.txt` (编排者生成, 不入库); 当前基线与覆盖审计口径见 todo.md 看板头。

## Wave 1 (并发 9 席, 范围内缺口 1047 — 脚本面已收, 残 10 文件转 Wave3)

| 席 | 文件族 | 缺口 | 独占 crate | 状态 |
| --- | --- | --- | --- | --- |
| W1S1 | `base/` 除 platform | 140 | v8-base | [x] |
| W1S2 | `base/platform/` | 32 | v8-base-platform | [x] |
| W1S3 | `compiler/turboshaft/` | 174 | v8-turboshaft | [x] |
| W1S4 | `compiler/backend/` | 79 | v8-compiler-backend | [x] |
| W1S5 | `compiler/` 顶层 js-*/bytecode-*/feedback-source/fast-api-calls/graph-assembler | 33 | v8-compiler-frontend | [x] |
| W1S6 | `compiler/` 顶层其余 (reducer/typer/pipeline/...) | 174 | v8-compiler-passes | [x] (残 opcodes.cc/operator.cc 转 R3H) |
| W1S7 | `heap/` 除 cppgc | 189 | v8-heap-core, v8-heap-gc | [x] (残 8 文件转 R3H) |
| W1S8 | `heap/cppgc-internal/`, `heap/cppgc-js/` | 108 | v8-heap-cppgc | [x] (脚本面; .skel 手写面开发中) |
| W1S9 | `objects/js-*` | 118 | v8-objects-js-core, v8-objects-js-func, v8-objects-js-async | [x] |

## Wave 3 (轮3, 并发 16 席, 范围内缺口 274 → 0 清零; 表 `target/wave3/`)

| 席 | 文件族 | 数 | 独占 crate | 状态 |
| --- | --- | --- | --- | --- |
| R3O1 | objects tagged/slots/谓词底座 | 26 | v8-objects-core | [x] |
| R3O2 | objects 哈希/字典/lookup/keys | 20 | v8-objects-collections | [x] |
| R3O3 | objects code/bytecode/类型系统 | 23 | v8-objects-rest | [x] |
| R3O4 | objects map/prototype/property 元数据 | 16 | v8-objects-maps | [x] |
| R3O5 | objects elements/数组族 | 13 | v8-objects-arrays | [x] |
| R3O6 | objects 函数/脚本/反馈/模板 | 18 | v8-objects-js-func | [x] (席超时, 产物由并发写手补齐) |
| R3O7 | objects 模块/contexts/debug | 19 | v8-debug (已 scaffold 入 members) | [x] |
| R3O8 | objects managed/序列化/杂项 | 27 | v8-objects-js-async | [x] |
| R3RE1 | regexp printer 族 | 11 | v8-regexp-syntax | [x] (并发写手) |
| R3RE2 | regexp 编译/字节码/双架构 | 14 | v8-regexp-compiler/-exec/-x64/-arm64 | [x] (并发写手) |
| R3W1 | wasm/interpreter (DrumBrake) | 10 | v8-wasm-interp | [x] (并发写手) |
| R3W2 | wasm/baseline (Liftoff) | 8 | v8-wasm-liftoff | [x] (并发写手) |
| R3W3 | wasm 对象/代码管理 | 20 | v8-wasm-objects | [x] (并发写手) |
| R3W4 | wasm module 编译/实例化/绑定 | 22 | v8-wasm-core, v8-wasm-js | [x] (并发写手) |
| R3W5 | wasm graph 接口/wrapper 缓存 | 17 | v8-wasm-turbofan | [x] (并发写手) |
| R3H | heap 残 8 + compiler 残 2 | 10 | v8-heap-gc, v8-heap-core, v8-compiler-passes | [x] (并发写手) |


## Wave 5 (轮5 派发, 并发 5 席 + 接力席 R5H3; 语义面 Developer/Reviewer 流水线; 表 `target/wave5/`)

| 席 | 文件族 (原版) | 独占 crate | 目标 | 状态 |
| --- | --- | --- | --- | --- |
| R5T | `torque/declaration-visitor` + `declarations` + `declarable` (~1.9k 行) | v8-torque | declare 段 35 待填体, 解锁 ExpectSuccessfulCompilation 类 + ls-server-data 9 例 | [x] (熔断抢救收割; 复审环 r3 clean 1/3, r4 在飞 — [./log/v8-torque.md](./v8-torque.md)) |
| R5H | `heap/memory-allocator.h/.cc` (实态 420+832) | v8-heap-core | HC-2b 接力 (盘点静默在飞面→填体→测试), 富余开 HC-2c 骨架 | [x] (HC-2b 收笼 64/64 双零; HC-2c 骨架 998 行 83 todo! 交下波填体 — [./log/v8-heap-core.md](./v8-heap-core.md)) |
| R5W | `wasm/wasm-objects.h/-inl.h/.cc` (3.5k) | v8-wasm-objects | wasm_objects_cc.rs 77 todo! 填体 (WasmMemory/WasmInstance 优先), 深水推迟登记 | [x] (纯逻辑面 15 fn+异常编码填体; 复审环 r1-r6, F14 整改席 R5W2 在飞 — [./log/v8-wasm-objects.md](./v8-wasm-objects.md)) |
| R5L | `libplatform/{default-platform,task-queue,delayed-task-queue,default-foreground-task-runner}` | v8-libplatform | LP-A 接力 (src 近空壳→按既有脚本面实现) | [x] (LP-A 冻结 clean 3/3; LP-B/LP-C 推迟 — [./log/v8-libplatform.md](./v8-libplatform.md)) |
| R5H3 | `heap/{spaces.h,-inl.h,.cc}` + `heap/{large-spaces.h,.cc}` (实测 1062 行; 原票 base/basic-blocks 两件实测 vendored 树不存在, 已删) | v8-heap-core | HC-4 空间实体族一波: spaces.rs + large_spaces.rs 填体, main_allocator 83 todo! 不在本票 | [x] (熔断抢救收笼 1909 行填体; HCR1 兜底审查 2 must-fix → R5H4 处置在飞 — [./log/v8-heap-core.md](./v8-heap-core.md)) |
| R5M | `maglev/maglev-ir.h` 等只读比对 | (无写权) v8-maglev-ir | 复审环 round1-13 已收, r13 clean 2/3 | (审查环中, clean 2/3 — [./log/v8-maglev-ir.md](./v8-maglev-ir.md)) |

- Wave 5 全席禁碰: `v8-ic/`、`tool/src/ic/` (伙伴在飞); heap L1 普查骨架面 (zapping/spaces_enums/slot_set/mutable_page_enums/index_generator/heap_enums/gc_callbacks + tool 对应脚本, 伙伴所有); 各席禁改他席 crate 与 lib.rs; 禁跑生成脚本; 禁 git commit/push (主控收票入库)
- 门禁口径 (每席): `CARGO_TARGET_DIR=target/agent_<席名>` + `-p <独占crate>` 单包; 命令带 `--config 'term.quiet=false'` (.cargo/config.toml quiet 吞诊断); clippy `--all-targets`, 依赖面瞬红按面归因让位不代修

## Wave 6 (轮6 派发, 并发 12 席, 语义面填体流水线; 表 `target/wave6/`)

- 轮6 开工重锚 (编排者, 2026-10-12 10:4x): 基线实测见看板头 (范围内缺口 0 维持, 本轮纯语义面); 伙伴会话在飞 (10:43 仍在写): v8-torque/v8-wasm-objects/v8-libplatform/v8-runtime(+tool/src/runtime)/v8-heap-core/v8-maglev-ir/v8-ic — 以上 7 crate + 对应 tool 域本波全员禁碰
- 本波依据: 各 crate `todo!(` 实测计数普查 (非脚本缺口); turboshaft(123)/compiler-passes(863) 两面纯 .skel 大盘, 依赖 broker/frontend 语义就位, 留下一波

| 席 | 文件族 (原版) | 独占 crate | todo! 基线 | 目标 | 状态 |
| --- | --- | --- | --- | --- | --- |
| W6O1 | objects 底座 (slots/instance-type/tagged-impl/谓词inl) | v8-objects-core | 153 | 填体 + unittests/objects 1:1 | [x] (中断·排队故障; 差集: todo! 153→115 已填 38, instance_type_inl 23→1, 挂断中 8 文件待核验 tagged_value_inl/tagged_field/map_word/compressed_slots/tagged/instance_type_inl/maybe_object+tests/map_word_unittest, 依赖面 v8-base 瞬红归因 W6A1) → 接力 W6O1b (开发中, 先核验半写态禁重做) |
| W6O2 | objects map/transitions/map-updater/property-descriptor | v8-objects-maps | 100 | transitions 36 + map_updater 32 填体 | [x] (票面收笼·主控复验: todo! 104→81, 填体 19 处 field_index/megadom/property_descriptor_object 接线, 83 条全数推迟台账 7 组 TR/MU 注明 handlified 前置; nextest 19/19, clippy 包级净 (v8-base 伙伴 lint 瞬红归因让位), fmt 净, miri 6/6; 待开复审环) |
| W6O3 | objects code/bytecode-array/instruction-stream/抽象码 | v8-objects-rest | 79 | code 25 + bytecode 面填体 | [x] (中断·排队故障; 差集: todo! 79→57 已填 22, code.rs 25→3, 新产物 code_kind.rs 未挂载, 本票红态 15 错 E0432/E0425 CodeInit·raw + E0423 InstanceType 待归因) → 接力 W6O3b (开发中, 先修红禁重做) |
| W6O4 | objects backing-store/序列化/managed/intl/simd | v8-objects-js-async | 70 | backing_store 18 + 序列化族填体 | [x] (中断·提供方故障; 差集: todo! 70→48, 新产 hash_seed_wrapper/heap_number/oddball 三件, 撞线红态 E0401/const PartialEq/RoundingMode::kHalfEven) → 接力 W6O4b (中断·排队故障; 红态已修绿, 新挂载 microtask/promise/cpp_heap_external_object/allocation_site_tq, todo! 仍 48) → 三接力 W6O4c (开发中, 每组文件即门禁防丢现场) |
| W6O5 | objects 哈希/字典/lookup/keys | v8-objects-collections | 49 | ordered-hash-table/swiss 填体 | [x] (中断·排队故障 142 轮; 差集: todo! 49→38, hash_table/dictionary 两面已清, 红仅依赖面 v8-base 瞬红归因 W6A1) → 接力 W6O5b (开发中, 每组即门禁) |
| W6CG | codegen 平台无关 (assembler/code-factory/handler-table) | v8-codegen-core | 93 | code_factory 22 + handler_table 14 填体 | (开发中) |
| W6DBG | debug/module/source-position/liveedit | v8-debug | 116 | source_text_module/call_site_info 各 33 填体 | [x] (中断·排队故障; 差集: todo! 116→93, module.rs 21→0 全清, 挂断中 8 文件半写态, 本票红态 10 错 DebugExecutionMode/ResetState/Copy/Error 变体/is_the_hole) → 接力 W6DBGb (开发中, 先修红禁重做) |
| W6HGC | heap mark-compact/sweeper/marking/scavenger | v8-heap-gc | 208 | .skel 收编挂载 + mark_compact 97 填体启动 | (开发中) |
| W6BX | builtins/x64 entry 汇编内建 | v8-builtins-x64 | 73 | entry.rs 全填 | [x] (票面全落·主控复验 todo! 73→0; 待开复审环 — [./log/v8-builtins-x64.md](./v8-builtins-x64.md)) |
| W6BA | builtins/arm64 entry 汇编内建 | v8-builtins-arm64 | 71 | entry.rs 全填 | (开发中) |
| W6A1 | base numerics/threaded-list/small-vector/page-region-allocator | v8-base | 148 | skel 收编 + checked_math 20 | [x] (中断·排队故障 96 轮; 差集: todo! 仍 148 全在 skel, 实际推进=numerics 子模块树挂载 76 非 skel 文件 + checked_math/safe_conversions 半写, 挂断红 E0432 Checked/CheckedOps——L0 红阻塞他席门禁) → 接力 W6A1b (开发中, 第一优先修红解阻塞) |
| W6GC2 | heap/cppgc 本体 .skel 手写面 | v8-heap-cppgc | 114 | 盘点→收编→填体启动 | [x] (中断·提供方故障; 差集实测 8 草案未挂载, todo! 114→112) → 接力 W6GC2b (开发中, 禁重做已落地 — [./log/v8-heap-cppgc.md](./v8-heap-cppgc.md)) |

- Wave 6 全席禁碰: 伙伴在飞 7 crate (上列) + 他席独占 crate 与 lib.rs; 禁跑他族生成脚本 (`*_generated.rs`/生成件复跑仅限重生成基线); 禁 git commit/push; 门禁口径同 Wave 5 (`CARGO_TARGET_DIR=target/agent_<席名>` + `-p` 单包 + `--config 'term.quiet=false'` 置于 `--` 前)

## Wave 7 (轮7 派发, 并发 15 席: 12 Developer 填体/接力 + 3 Reviewer 复审环; 表 `target/wave7/`)

- 轮7 开工重锚 (编排者, 2026-10-12 15:2x): 复跑 `bun tool/tool-coverage.js` 实测范围内缺口 0 / 陈旧引用 0 维持 (脚本轮收敛, 本轮纯语义面); 上波 Wave6 各席提供方挂断静默 (最后写 14:57 v8-base checked_math); 伙伴 7 crate 上午后无写动
- 编排修复 (轮7 开工, 2 处): v8-codegen-core register_config.rs 补挂断未写的 K_REP_FLOAT32/64/SIMD128/256 常量 (machine-type.h 判别值 17/18/19/20) + codegen_semantic.rs 删除 `</content>` 挂断残留; lib 复绿; 该 crate 测试面残 31 错 (`_keep` 引用/模块私有/未导出类型, 属 W6CG 半写测试) 归 W7CG 席位自修, 不阻塞依赖链 (lib check 过)

| 席 | 文件族 (原版) | 独占 crate | todo! 基线 | 目标 | 状态 |
| --- | --- | --- | --- | --- | --- |
| W7A1 | `base/` 除 platform (接力 W6A1b) | v8-base | skel 148 | numerics 半写收尾 + skel 收编填体 + 首优先解 L0 阻塞 | [x] (主控复验: checked_math/safe_conversions E0432 根因消除, numerics 挂载面 0 todo!; nextest 351/351 绿; 67 skel 逐一对账归四类已折入/平台裁剪/无 Rust 对应物, base 族判定无遗漏可移植面) |
| W7O1 | objects 底座 (接力 W6O1b) | v8-objects-core | 117 | 8 文件半写态核验→填体, 禁重做 | [x] (主控复验: todo! 开工实测 105→97, heap_object_field_inl 6 面+set_map 填体, 4 模块挂载 (compressed_slots/maybe_object/tagged_value_inl/instance_type_inl), compressed_slots::from_value 去误标 const fn, 98→实测 97 条全数入 readme/zh.md 推迟台账 (11 文件×原函数名+前置+解锁 crate); nextest 62/62 绿) |
| W7O3 | objects code/bytecode (接力 W6O3b) | v8-objects-rest | 57 | code_kind.rs 挂载修红→填体 | [x] (撞轮次上限熔断; 主控盘点: todo! 57→47, check 绿, code_kind 已挂载, 停在 abstract_code.rs 尾部 stub 删除半态) → 接力 W7O3c [x] (主控复验: 半删态核验后填体/删净 36 处, todo! 47→11 全为深水台账项; deoptimization_data_inl 经 objects-arrays TaggedArrayExt 解锁 10 处, turboshaft_types 6→0, turbofan_types_inl 6→0; nextest 5/5 绿; readme/zh.md 建账 11 条含解锁 crate + 已回填区) |
| W7O4 | objects backing-store/序列化 (三接力 W6O4c) | v8-objects-js-async | 48 | 每组文件即门禁 | [x] (主控复验: todo! 48→43, backing_store 5 面填体+类型/访问器族落地, 门禁 7/7 绿; 43 条按 8 组登记 readme 推迟台账含解锁 crate; 余面留 round 波) |
| W7O5 | objects 哈希/字典 (接力 W6O5b) | v8-objects-collections | 38 | swiss/ordered-hash 填体 + 修复 unsafe 块 | [x] (主控复验: swiss unsafe 修复+SAFETY 补齐, ordered_name_dictionary 补全, 0 allow, nextest 19/19 绿, clippy 0 警告; 转复审环) |
| W7O5R | 复审环 objects collections (round1) | (只读+tests) v8-objects-collections | 0 | 深度审查 swiss/ordered_hash 内存安全、原地重哈希与探查语义 | [x] (round1 clean 1/3: 0 缺陷/0 遗漏/19 例绿; 修正 OrderedNameDictionary 前缀布局/哈希继承, Group16 零拷贝优化; 转 round2) |
| W7O5R2 | 复审环 objects collections (round2) | (只读+tests) v8-objects-collections | 0 | round2 全新 Reviewer 独立交叉比对 | [x] (round2 clean 2/3: 0 缺陷/0 遗漏/19 例绿; swap 零堆分配原地槽位交换, into_iter 消费语义; 转 round3 冲刺终验) |
| W7O5R3 | 复审环 objects collections (round3) | (只读+tests) v8-objects-collections | 0 | round3 全新 Reviewer 终验交叉审查 (冲刺 clean 3/3) | [x] (round3 clean 3/3 终验冻结达成: 0 缺陷/0 遗漏/19 例绿; 单次 probe 原地更新消二次探测, clippy 0 警告, 0 allow; 终验冻结) |
| W7CG | codegen 平台无关 | v8-codegen-core | 69 + 测试面 31 错 | 修自写测试红→code_factory 22 + handler_table 14 填体 | [x] (终票收笼主控复验: 测试面 31/31 修复, lib 面 todo! 69→45 (code_factory 22 全填含 CEntry debug_assert/NewFunctionContext 精确档/ArrayConstructor 双分支, handler_table emit 0xcc 填充修正), 45 深水面 README 台账按文件分组; nextest 22/22 绿, clippy RC=0; 转复审环) |
| W7CGR | 复审环 codegen-core (round1) | (只读+tests) v8-codegen-core | 0 | 深度比对 handler_table/code_factory 语义与单测强度 | [x] (round1 clean 1/3: 修复位域边界与对齐UB, 补齐枚举与分派, 21/21 绿, clippy 0 警告; 转 round2) |
| W7CGR2 | 复审环 codegen-core (round2) | (只读+tests) v8-codegen-core | 0 | round2 全新 Reviewer 独立交叉审查 | [x] (round2 clean 2/3: 0 缺陷/0 遗漏/22 例绿; 补充模式断言/ElementsKind枚举/默认实参, 消除allow; 转 round3 冲刺终验) |
| W7CGR3 | 复审环 codegen-core (round3) | (只读+tests) v8-codegen-core | 0 | round3 全新 Reviewer 终验交叉审查 (冲刺 clean 3/3) | [x] (round3 clean 3/3 终验冻结达成: 0 缺陷/0 遗漏/22 例绿; jump_table对接AssemblerBase, 脚本幂等, clippy 0 警告, 0 allow; 终验冻结) |
| W7DBG | debug/module/source-position (接力 W6DBGb) | v8-debug | 93 | 半写 8 文件先修红禁重做 | [x] (主控复验: 红 10 处全清, todo! 93→0, 填体实 816+732+409 行, 6/6 绿, 深水面 23 项入 readme 台账; 测试面仅 6 例偏薄, 待开 round1 复审环核验体质量) |
| W7HGC | heap mark-compact/sweeper/scavenger | v8-heap-gc | 208 | .skel 收编挂载 + mark_compact 97 | [x] (主控复验: 挂载面 todo! 0, nextest 29/29 绿, 14 skel 全入 readme/zh.md 推迟台账; marking_barrier/state/worklist/visitor 深水面引用未定义对象模型类型不强挂; mark_compact.rs 现 172 行 vs 原版 8.6k — 主算法面留下一波填体席, 待开复审环) |
| W7GC2 | heap/cppgc 本体 (接力 W6GC2b) | v8-heap-cppgc | 107 | 8 草案挂载收编→填体启动 | [x] (主控复验: 挂载 mods 9→17, skel 58→50, 挂载面 todo! 0→3 (object_view 大对象臂/object_size_trait mixin 臂登记 readme 台账), 门禁 12/12 绿; 未挂 50 件依赖 HeapBase/BasePage/RawHeap 等未落堆骨架留下一波) |
| W7HC | `heap/main-allocator.h/cc` 新席 | v8-heap-core | 105 (main 83 + spaces/large 22) | HC-2c/HC-4 填体 | [x] (撞线熔断; 接力 W7HCb) |
| W7HCb | heap-core 接力填体 (main_allocator 深面) | v8-heap-core | 83 (main 区) | 三组即门禁续填, 半写态核验禁重做 | [x] (主控复验: pages 12→0 / memory_pool 6→0 / space_memory_allocator 8→0 三组落地, 填体 26 处, main 区 todo! 83→57 全入 readme 63 行台账; v8-heap-core nextest 94/94 绿; 转复审环) |
| W7HCR | 复审环 heap-core (round1) | (只读+tests) v8-heap-core | 0 | 逐行比对 main_allocator/spaces/memory 语义与内存安全 | [x] (round1 clean 1/3: 0 缺陷/0 遗漏/94 例绿; 清除 12 处 allow 残留, 0 allow, 内存安全与三账核算严格对齐; 转 round2) |
| W7HCR2 | 复审环 heap-core (round2) | (只读+tests) v8-heap-core | 0 | round2 全新 Reviewer 独立交叉审查 | [x] (round2 clean 2/3: 0 缺陷/0 遗漏/94 例绿; 原版逐行比对无语义偏差, 0 allow, clippy 0 警告, fmt 净; 转 round3 终验) |
| W7HCR3 | 复审环 heap-core (round3) | (只读+tests) v8-heap-core | 0 | round3 全新 Reviewer 终验交叉比对 (冲刺 clean 3/3) | [x] (round3 clean 3/3 终验冻结达成: 0 缺陷/0 遗漏/94 例全绿; 彻底清除 allow 绕过, 100% 内存安全 SAFETY, 0 clippy 警告; 终验冻结) |
| W7WO | `wasm/wasm-objects.h/cc/inl` | v8-wasm-objects | 79 | wasm_objects_cc.rs 填体 + 深水推迟登记 | [x] (主控复验: unused 清理, Memory/TrustedInstance/Table/Module 纯逻辑填体, 结构体封装消 too_many_args, clippy 0 警告; 迟到终票收笼: 填体 55/76, 余 21 全为 New/工厂/堆写/isolate 面深水项入 readme B 节 21 条台账 (组\|符号\|前置\|解锁 crate), 调度表 entry_set/clear/maybegrowuses/adduse/grow 状态机 + wrapper map + exception for_tag 落地; nextest 22/22 绿; 转复审环) |
| W7WOR | 复审环 wasm-objects (round1) | (只读+tests) v8-wasm-objects | 0 | 逐行比对 wasm-objects 契约与单测强度 | [x] (round1 clean 1/3: 0 缺陷/0 遗漏/17 例绿; TableCopyOp 状态机闭环, 边界溢出防御; 转 round2) |
| W7WOR2 | 复审环 wasm-objects (round2) | (只读+tests) v8-wasm-objects | 0 | round2 全新 Reviewer 独立交叉比对 | [x] (round2 clean 2/3: 0 缺陷/0 遗漏/22 例绿; SetWrapperParams 清除 allow 绕过, fmt 净; 转 round3 冲刺终验) |
| W7WOR3 | 复审环 wasm-objects (round3) | (只读+tests) v8-wasm-objects | 0 | round3 全新 Reviewer 终验交叉审查 (冲刺 clean 3/3) | [x] (round3 clean 3/3 终验冻结达成: 0 缺陷/0 遗漏/22 例绿; 100% Safe Rust, 零 panic, 零 allow 绕过, nextest 22/22 绿, clippy 0 警告; 终验冻结) |
| W7LIT | 零头 (simulator 4/regexp-exec 3/wasm-interp 2/js-core 1) | v8-execution-simulator, v8-regexp-exec, v8-wasm-interp, v8-objects-js-core | 10 | 全清 | [x] (主控复验: 字面命中 10 中真 todo 宏仅 7; simulator 填 2 (fp16 往返 65536 全模式 0 失配) + 推迟 2; regexp-exec 0 真 todo (红线说明文字); wasm-interp 2 + js-core 1 全 .skel 深水面登记推迟; 门禁 211/211 绿) |
| W7O2R | 复审环 objects maps (W6O2 票面收笼后) | (只读+tests) v8-objects-maps | 81 推迟台账 | round1 全新 Reviewer 逐行比对 | [x] (round1 clean 1/3: 0 缺陷/0 遗漏/30 例绿; 27 项 must-fix 修复, 纠正 5 槽 Struct 布局, 补齐并发线性查找与 prototype 定义, 30/30 绿; 转 round2) |
| W7O2R2 | 复审环 objects maps (round2) | (只读+tests) v8-objects-maps | 0 | round2 全新 Reviewer 独立交叉审查与优化 | [x] (round2 clean 2/3: 0 缺陷/0 遗漏/31 例绿; 原型过渡遍历补齐, Acquire/Release 内存序发布, 31/31 绿; 转 round3 终验·会话 56b370ce) |
| W7O2R3 | 复审环 objects maps (round3) | (只读+tests) v8-objects-maps | 0 | round3 全新 Reviewer 终验交叉审查 (冲刺 clean 3/3) | [x] (round3 clean 3/3 终验冻结达成: 0 缺陷/0 遗漏/31 例全绿; 隐藏类过渡树/Acquire-Release内存序/5槽Struct布局/推迟台账100%闭合, 0 警告, 0 allow; 终验冻结) |
| W7BXR | 复审环 builtins-x64 (W6BX 73→0) | (只读+tests) v8-builtins-x64 | 0 | round1 全新 Reviewer | [x] (round1: 6 缺陷/2 遗漏/8 例绿, 主控复验一致; 定性同 arm64 席=名面骨架无汇编体; 名面 73 双射+与 arm64 71 对时一致 (差 {AdaptShadowStackForDeopt,DumpFrame}=x64_specific); nit 转整改席 W7BXM; clippy 外部红归因 W7CG; clean 0) |
| W7DBGR | 复审环 debug 新填 93 体 (round1) | (只读+tests) v8-debug | 0 | 逐行比对 dfs 环检测语义+测试强度缺口固化 | [x] (round1 clean 1/3: 0 缺陷/0 遗漏/32 例绿; 星导出环检测/纯import/合成模块状态机/ResolveContext重构消除allow, 32/32 绿; 转 round2) |
| W7DBGR2 | 复审环 debug (round2) | (只读+tests) v8-debug | 0 | round2 全新 Reviewer 独立交叉审查与优化 | [x] (round2 clean 2/3: 0 缺陷/0 遗漏/35 例绿; 消除堆分配, ECMAScript前导零数组索引, UTF-8切片安全, as_chunks重构; 转 round3 终验) |
| W7DBGR3 | 复审环 debug (round3) | (只读+tests) v8-debug | 0 | round3 全新 Reviewer 终验交叉比对 (冲刺 clean 3/3) | [x] (round3 clean 3/3 终验冻结达成: 0 缺陷/0 遗漏/35 例全绿; 7 项 BreakLocation 判定方法对齐, 100% Safe Rust, 0 警告, 0 allow; 终验冻结) |
| W7BXM | builtins-x64 整改 (round1 缺陷处置) | v8-builtins-x64 | round1 nit×5+遗漏×2 | error.rs 死面/谓词组名订正/expected_frame_type 精化或注记/readme 推迟台账; 整改后待 round2 | [x] (整改完成: error.rs 与 thiserror 彻底清理, 9/9 绿, clippy 0 警告, 0 allow, 单测零堆分配重构, 脚本 100% 幂等; 转 round2) |
| W7BXR2 | 复审环 builtins-x64 (round2) | (只读+tests) v8-builtins-x64 | 0 | round2 全新 Reviewer 独立交叉审查 | [x] (round2 clean 1/3: 0 缺陷/0 遗漏/9 例绿; 73 项双射无缝对齐, 100% Safe Rust, 脚本 100% 幂等; 转 round3) |
| W7BXR3 | 复审环 builtins-x64 (round3) | (只读+tests) v8-builtins-x64 | 0 | round3 全新 Reviewer 独立交叉比对 | [x] (round3 clean 2/3: 0 缺陷/0 遗漏/9 例绿; 73 项双射无缝对齐, 100% Safe Rust, 脚本 100% 幂等; 转 round4 冲刺终验) |
| W7BXR4 | 复审环 builtins-x64 (round4) | (只读+tests) v8-builtins-x64 | 0 | round4 全新 Reviewer 终验交叉审查 (冲刺 clean 3/3) | [x] (round4 clean 3/3 终验冻结达成: 73 项双射无偏差, 零堆分配, 100% Safe Rust, nextest 9/9 绿, clippy 0 警告, 0 allow; 终验冻结) |
| W7BAR | 复审环 builtins-arm64 (W6BA entry 已全落) | (只读+tests) v8-builtins-arm64 | 0 | round1 全新 Reviewer | [x] (round1: 5 缺陷/2 遗漏/7 例绿, 主控复验一致; 定性=entry 名面骨架无汇编体, 逐指令比对 N/A; 名面 71 Generate_ 双射全核; 缺陷1 clippy 红归因 v8-codegen-core code_comments.rs 由 W7CG 自修; nit2-5+遗漏AB 转整改席 W7BAM; clean 计数 0) |
| W7BAM | builtins-arm64 整改 (round1 缺陷处置) | v8-builtins-arm64 | round1 nit×4+遗漏×2 | error.rs 冗余/CcBuiltin 死面注记/谓词启发式表述/登记推迟面; 整改后待 round2 全新 Reviewer | [x] (主控复验: error.rs 删除+lib.rs 净, 7/7 绿, clippy --no-deps RC=0, readme 推迟台账六项完整; thiserror 冗余依赖已清理; 转 round2) |
| W7BAR2 | 复审环 builtins-arm64 (round2) | (只读+tests) v8-builtins-arm64 | 0 | round2 全新 Reviewer 深度审查与优化 | [x] (round2 clean 1/3: 0 缺陷/0 遗漏/7 例绿; tests 零分配重构, 脚本同步; 转 round3) |
| W7BAR3 | 复审环 builtins-arm64 (round3) | (只读+tests) v8-builtins-arm64 | 0 | round3 全新 Reviewer 独立交叉审查 | [x] (round3 clean 2/3: 0 缺陷/0 遗漏/7 例绿; 消除单测运行时堆分配, 100% safe rust; 转 round4 冲刺终验) |
| W7BAR4 | 复审环 builtins-arm64 (round4) | (只读+tests) v8-builtins-arm64 | 0 | round4 全新 Reviewer 终验交叉比对 (冲刺 clean 3/3) | [x] (round4 clean 3/3 终验冻结达成: 71 双射吻合, 零堆分配, 100% Safe Rust, nextest 7/7 绿, clippy 0 警告, 0 allow; 终验冻结) |

- Wave 7 全席禁碰: 他席独占 crate 与 lib.rs; 禁跑他族生成脚本; 禁 git commit/push (主控收票入库); 门禁口径加严: 每席 `CARGO_TARGET_DIR=target/agent_<席名>` + `-p <独占crate>` 单包; 严格执行「写完必须实跑 `cargo nextest run -p <独占crate>`」门禁, 严禁跨席跑全仓 `./test.sh` / `./sh/clippy.sh` (全仓收官统一由编排者兜底)

## Wave 8 (轮8 派发, 并发 4 席纯填体面; 表 `target/wave8/`; 开工普查 2026-10-12 16:2x)

- 轮8 开工重锚 (编排者): 主仓 todo! 普查 = objects-core 97 / heap-core 97 / objects-maps 81 / codegen-core 45 / js-async 43 / wasm-core 42 / codegen-arm64 40 / wasm-objects 36 / collections 32 / diagnostics 29 / objects-rest 28 / objects-arrays 23 / regexp-syntax 16 / heap-cppgc 3; 前三大面与在飞复审环各自有主, 本波取四个正交无人脸

| 席 | 文件族 (原版) | 独占 crate | todo! 基线 | 目标 | 状态 |
| --- | --- | --- | --- | --- | --- |
| W8A64 | `codegen/arm64` + `diagnostics/arm64` | v8-codegen-arm64 | 40 (utils 14/reglist 11/constant_pool 9/decoder 5/cpu 1) | 挂面填体 + 原版单测 1:1 | [x] (40 todo 全清, 16/16 绿, round1-3 连续 3 轮 clean 达成终验冻结) |
| W8DIAG | `diagnostics/` 平台无关 | v8-diagnostics | 29 (basic_block_profiler 15/compilation_statistics 6) | 挂面填体 + 推迟台账 | [x] (填体 21 处, 39/39 绿, round1-3 连续 3 轮 clean 达成终验冻结) |
| W8ARR | `objects/elements*` + `fixed-array*` | v8-objects-arrays | 23 (elements_impl 19/elements 4) + 15 skel | elements 填体 + skel 判定收编或台账 | [x] (填体 10 处核心函数, 49/49 绿, round1-3 连续 3 轮 clean 达成终验冻结) |
| W8REPS | `regexp/regexp-ast*` + parser | v8-regexp-syntax | 16 (全在 ast_impl_skel.rs 草案) | 草案收编挂载→填体 + 台账 | [x] (草案收编填体, 21/21 绿, round1-3 连续 3 轮 clean 达成终验冻结) |

- Wave 8 全席禁碰: Wave 7 在飞面 (objects-rest W7O3c / heap-core W7HCb / builtins-x64 W7BXM / 复审环 codegen-core·collections·wasm-objects·debug·builtins-arm64·objects-maps) 与各独占 crate 与 lib.rs; 禁 git commit/push; 门禁口径同 Wave 7 (每席 CARGO_TARGET_DIR 隔离 + -p 单包 + 每组即实跑 nextest)
-  Wave 7 余票并行在飞: W7O3c (objects-rest 接力) / W7HCb (heap-core 接力) / W7BXM (builtins-x64 整改) / W7O2Rb (maps 复审接力) / W7BAR4 (arm64 round4 冲刺终验); W7BXR 行整改后待 round2
| W8HGC2 | heap-gc 主算法波 (W7HGC 留下一波票面) | v8-heap-gc | 14 skel + mark_compact 172/8.6k | skel 收编 + MarkingWorklist/Sweeper/ScavengerQueue 填体 + 主算法骨架 | (开发中·会话 a84e0ebb) |

## Wave 2+ (轮2 已清, 残余为语义面归 crate 流水线)

- [x] W2-objects-rest → 轮2 R2S1 清机械面 (28 脚本/28 族: object-predicates 386 谓词 clang 锚、SerializationTag 59、scope-info tq 枚举族; 残 43 族纯语义面登记归 Developer)
- [x] W3-wasm → 轮2 R2S2 清机械面 (40 脚本: wasm-interp handler 1690 项 clang 锚、WellKnownImport 81、liftoff 寄存器族; regexp 12 脚本; 残语义面/liftoff-compiler.cc 11.7k 等登记归 Developer)
- [x] W4-snapshot-init → 轮1 已清零 (T6: 44 脚本/81 文件, 快照 Bytecode 36+ro 6、blob 头布局; init 13 全清)
- [x] W5-codegen → 轮2 R2S3 清机械面 (external-reference 191 条、optimized-compilation-info 21 位、disasm-x64/arm64 指令终态表; assembler-x64 手写面禁覆写已避让 _generated 后缀)
- [x] W6-regexp → 轮2 R2S2 清机械面 (regexp-nodes 10 枚举、x64/arm64 帧布局双态、kErrorStrings 34)
- [x] W7-sandbox → 轮2 R2S3 清机械面 (13 脚本: trusted/js-dispatch-table LA57 双分支 shift 57/49 实值)
- [x] W8-diagnostics → 轮2 R2S3 清机械面 (11 脚本: disasm 双架构助记表、etw 六族、objects-debug Verify 注册表 259 条)
- [x] W9-debug → 消账复验 (本轮轻量席): gdb-server.js FILES 16 路径已全字面量 (脚本内注释载明 W9 消账口径), 复跑 `bun tool/tool-coverage.js` 实测 debug 域 文件 43/已覆 43/缺口 0/范围外 0; 全域缺口 196 均为范围外、范围内 0、陈旧引用 0, golden/tool-coverage.json 已刷新
- [x] W10-exec-deopt-零头 → 全清 (T3 重派 + R2S3: execution 4/deoptimizer 4/ast 1/parsing 1/numbers 1/snapshot 2)
- [x] W11-stale清账 → 陈旧引用 44→0 (审计器修复 + 19 处脚本错路径修正)
- 每轮末: 复跑 `bun tool/tool-coverage.js` 更新 `golden/tool-coverage.json`, 刷新本节数字, 迭代下一轮直到范围内缺口 = 0

## Wave 9/10 (本方编排会话, 2026-10-12 20:5x 起; 表 `target/wave9/`; 与并行编排会话共写, 席位正交分账)

| 席 | 文件族 (原版) | 独占 crate | 基线 | 目标 | 状态 |
| --- | --- | --- | --- | --- | --- |
| W9SNAP | `snapshot/` 全族 + `objects/value-serializer` 51 文件 | v8-snapshot | 骨架 1522l | 脚本面 40 件 + source-sink/compression 填体 | [x] (主控复验 nextest 10/10 + clippy 0 警告, todo! 0, 台账 10 条; 并行会话标 [x] clean 3/3 一致采信) |
| W9SNAPR | 同上 (只读+tests) | v8-snapshot 审查面 | 0 | round1 逐行对抗审查 | (复审环中→停投; 三投撞线/熔断, 并行会话 w9snapr3/4 在飞接续) |
| W9TS | `compiler/turboshaft` 186 文件 | v8-turboshaft | 3499l/123 todo | 脚本 121 件 + 挂载 10 件 3562l + skel 114 收编 | 熔断 (150 轮上限, 终态 lib 编译通过待 clippy) → 接力 W9TSb |
| W9TSb | 同上接力 | v8-turboshaft | 现场 3562l/136 todo/114 skel, lib 现红 E0015×4 (const fast_hash) | clippy 清警告→挂载面核补→skel 首批收编→单测 1:1 | (开发中) |
| W9CF | `compiler/` 前端族 37 文件 | v8-compiler-frontend | 空壳 | 脚本+基线+填体 | 熔断 (150 轮, 现场 1394l/365 todo/21 skel, lib 绿) → 接力票归并行会话 w9cf2/w9cp2 台账, 本方不双投 |
| W9CP | `compiler/` reducer/typer 族 197 文件 | v8-compiler-passes | 863 todo | 分批填体 | 熔断 (现场 953l/863 todo/95 skel, lib 绿) → 同上并行会话在票 |
| W9MF | `maglev/` 前端族 25 文件 | v8-maglev-frontend | 空壳 | 脚本 25+ 件 + graph-builder 族转写 | 熔断 (150 轮, 现场 1834l/9 todo/2 skel 近收官) → 接力 W9MFb |
| W9MFb | 同上接力 | v8-maglev-frontend | 差集 9 todo + 2 skel + 门禁 | 收官门禁绿 + 单测补强 | (开发中·实测在写 Cargo.toml/graph.rs) |
| W10HC | `heap/` 语义面 (冻结 crate 残面) | v8-heap-core | todo! 189 (main 83 台账面/skel 散面) | 分面填体 + skel 收编 + 台账核对 | (开发中) |
| W10OC | `objects/` 核心残面 (冻结 crate) | v8-objects-core | todo! 78 (body-descriptors 16/slots 9/visitors_inl 8 等) | 填体 + 12.6 延期测试恢复 | (开发中) |

- L5 四票 [x] 采信口径: 并行会话按「推迟台账冻结」口径标 [x]; 本方硬门禁复验现状 = v8-turboshaft lib 红 (E0015×4, W9TSb 修飞) / passes 863 todo / frontend 365 todo / maglev-frontend 9 todo — 各接力票收官且门禁绿之前, 本方账面保留异议行, 3 轮 clean 复审环未走完的 crate 不认「终验冻结」, 后续按 7.2 交叉复审收口
- 全仓门禁由编排者兜底: 接力票全数静默后统一 `cargo check --workspace` + `./test.sh` + `./sh/clippy.sh` 复验, 不在子席跑

### Wave 9/10 收票记录 (本方主控复验口径, 2026-10-13)

- W9MFb [x] 收: 挂载 graph.rs 937 行差集收编 + labeller/frame_state 新建, src 1834→2143, todo! 9→7 全登台账, 自报 nextest 29/29 + clippy 0; 主控复验因依赖 v8-objects-core 在飞红挂起, 待 W10OC 落地复验后转复审环
- W9TSb [x] 收: 纠偏前任自述 (lib 实 7 硬错 E0015), fast_hash const 块/graph 占位/representations panic 面全修, 挂载 10 件 4094 行 todo!/unimplemented!/panic! 清零, skel 114→114 (19 组台账), readme/zh.md 68 行建立; 主控独立复验 nextest 7/7 绿 + clippy -D warnings RC=0 — turboshaft 挂载面门禁成立
- W11TS2 派: turboshaft skel 收编二波 (台账 19 组中依赖已就位件: layered_hash_map/opmask/sidetable/reducer 族), 禁动已绿语义体

- W10WC 派: v8-wasm-core 残面补波 (todo 42, R5W3 静默转接力; 实测活跃写入中)
- W10HG 派: v8-heap-gc 主算法波 (todo 208, skel 收编→填体→配测; 实测活跃写入中)
- W10MAPS 派: v8-objects-maps 挂载面填体 (todo 88 = transitions_cc_generated 37 + map_updater_cc_generated 33 + prototype 9 等)
- W11JA 派: v8-objects-js-async 挂载面填体 (todo 43 = backing_store 13/intl 7/value_serializer_skel 5 等)
- W11CG 派: v8-codegen-core 挂载面填体 (todo 44, W7CG 冻结后残面)
- 复验挂起: W9MFb 自报 29/29+clippy 0, 主控复验因 v8-objects-core 在飞瞬态红延后; 待 W10OC 落地统一复验
- 小面归属登记 (禁双投): heap-cppgc skel 107→外部 w10cppgc 票 / collections 32→外部 w9cor / wasm-objects 26→外部 w9wor / objects-rest→外部 w9oasr
- epoch 实测纠偏: 各在飞票均有近 10min 内写入, 无静默; 账面「静默」数字与本方复测矛盾时, 以 find -stat epoch 复测为准
- W10OC 熔断 (150 轮上限, 现场 todo! 78 未降/延期测试面已动, 读台账中撞线) → 接力归外部会话 w10ocb (epoch 实测 1min 内在写, 本方禁双投); 本方跟踪其收票复验
- W10HG [x] 收 (主控复验 64/64 绿 + clippy -D 0 RC=0, 台账 107 行): 差集=补测 25 例 (marking-bitmap/progress-tracker/memory-reducer 1:1); 关键 DAG 判定 — heap-gc 208 全体为对象模型耦合体 (Heap/Isolate/Tagged<HeapObject>/NormalPage/MutablePage/SlotSet/ObjectVisitor 仅存 v8-heap-core/v8-objects-core .skel 未冻), 解锁根因收敛到「heap 骨架类型面」; turboshaft skel 19 组与 cppgc skel 面同因阻塞 → 全局关键路径 = heap-core/objects-core 骨架挂载, 各在飞票 (w10hcb/w10ocb) 正攻此面, 后续波次按解锁顺序收编
- W10HCb [x] 收 (主控独立复验: nextest 98/98 绿 / clippy -D RC=0 / slot_set.rs 挂载确认; 票报「156→148」为可执行面口径, 全文 grep 156 含测试 doc 例字符串): slot_set 族 1235 行零 unsafe 收编 + Miri 4/4 + 1:1 三用例; heap-core skel 78→77, 深水组 skel-F 登账; 对象模型骨架阻塞面 (mutable_page/object_lock/pending_allocations 等) 待 objects-core/heap-core 类型面互相解锁 — 关键路径票 w10ocb 在飞
- W11JA [x] 收 (主控独立复验: nextest 10/10 绿 / clippy -D RC=0): 实现差集 0 判定采信 — 43 todo! 全为深水 (依赖 Isolate/Handle/sandbox/cppgc/ICU/PageAllocator), 分层不满足强填条件; 本轮净产出 = 43 处 todo 消息规范化为「原函数名: 前置; 解锁 crate」+ 深水 stub 补 /// v8: 锚 + 3 例纯语义核测试 (managed 收支/option_utils/value_serializer varint-zigzag-double roundtrip)。js-async 面转入「待 L6 骨架解锁」台账态
- 复验再延: W9MFb (maglev-frontend) 主控复验二次挂起 — objects-core 现编译 RC=0 (w10ocb 仍在飞, 不作冻结绿), 当前阻塞换为依赖 v8-codegen-core machine_type.rs:184 E0015 (unreachable! in const fn, W11CG 00:38 实时在写瞬态红); 待 W11CG 收官后统一复验
- W10WC 熔断 (150 轮上限, 停在「155 variants 程序化重命名」中途): 现场三件套 = src 11684 行 / todo! 42 (基线未降) / skel 0; 编译红 3 硬错 — module_decoder.rs:101 const 内 match str (E0658×2) + wasm_opcodes.rs:3371 const 断言 E0080 (0xfd120 fp16 边界与重命名后表矛盾); local_decl_encoder.rs use 头曾被冲掉, 现场已恢复绿文; 撞线席终刷后 40s epoch 静默, 判定无外部活写手 → 本方派接力票 W10WCb (先修 3 硬错恢复 lib 绿 → 续完重命名差集 → 42 todo 填体/登账 → nextest+clippy 门禁), 禁重做已落地内容
- 轮末普查 10-13 01:0x (本方): 挂载面 todo! = maps 80 (W10MAPS 在飞) / heap-core 72 (W10HCR 复审中, 数字随修复浮动) / objects-core 66 (外部 w10ocb) / js-async 43 (已收, 全深水台账) / wasm-core 42 (W10WCb 接力在飞) / codegen-core 38 (W11CG) / collections 32 (外部 w9cor) / wasm-objects 26 (外部 w9wor) / objects-rest·arrays 13/13 (外部·冻结台账) / compiler-frontend 12 (外部 w11cf3; passes 挂载面已 0, 余 .skel 面归 w11cp3) / diagnostics 8·maglev 7·arrays 13 = 冻结台账深水 / regexp-exec 3 / heap-cppgc 3 / simulator 2 — 无主小面残量不足以开新席, 全局关键路径仍为 heap-core/objects-core 骨架类型解锁 (turboshaft skel 114 / heap-gc 208 / cppgc skel 深水同因阻塞); 脚本轮: tool-coverage 3327/1700/3117, 范围内缺口 0 维持
- W11TS2 [x] 收 (主控独立复验: nextest 24/24 绿 / clippy -D RC=0 / skel 114→111 实测一致, 挂载面 4094→5356 行 +1262, 挂载 todo! 0): sidetable/layered_hash_map/opmasks_named 三组收编 1:1 (含机械自证 17 例, 原版无 unittests 已查证); opmasks.h.js 复跑覆盖现态事件已按「src 现态为准」恢复并记台账幂等注意; 余 111 skel 逐组跳过原因登 readme — 转 round1 复审环 W11TSR
- W10MAPS [x] 收 (主控独立复验: nextest 43/43 绿 / clippy -D RC=0 / todo 88→80 实测一致): TR-ISO 7 件 + MU-ISO EqualImmutableValues 差集全填 (脚本 BODIES 生成双跑幂等), test-transitions.cc 1:1 可迁 5 例 + 填体断言 6 例 + MU 用例 1 例, 台账同步减行; 深水面 (prototype 9/property_descriptor 4 等) 全保留全登记。⚠ Miri 既有红: transitions.rs:624 AtomicUsize::from_ptr 堆模拟地址契约 Stacked Borrows 违规 (复跑冻结基线既有用例同款命中, 非本波引入) — 列 must-fix 开 W11MPR round1 复审环
- W10HCR round1 出账 (heap-core): 缺陷 4 / 遗漏 4 / 新测 3 例边界, clean 计数不成立 — R1/R2 锚错位, R3 台账失实 (skel-F memory_balancer/object_lock 谎登深水, 且「miri 4/4·98/98·main_allocator 50」等计数过期, 本方 W10HCb 收票行同按当时口径留档注纠), R4 allocation_info_ 自指 Pin 约束未登记; slot_set 族逐行对账等价 + 零 unsafe 属实。整改票 W10HCS 已派 (含并发让位纪律); 收票时账面「98/98 + Miri 4/4」为 W10HCb 时点值, 复审终态 118/118 + miri 24/24
- W11TSR round1 出账 (turboshaft): 缺陷 2 (已就地修: lib.rs pub mod→mod+pub use glob 合规、layered_hash_map.insert_new_key 补 back() UB 前置 debug_assert) / 遗漏 0 / 新测 4 语义级; 三新件逐结构体 1:1 全对 (opmasks 位域偏移无错位), 台账无谎报; 主控复验 nextest 28/28 + clippy -D RC=0。发现缺陷 → clean 计数归 0, 开 round2 全新复审 W11TSR2
- W10HCR 双跑事故入账 (编排者纪律缺陷): 首发席 (R1-R4, 缺陷4/遗漏4/测3, 未改 src) 与本方补派席 (缺陷5/遗漏6/新测17, 已改 src 锚修+debug_assert+typed_slots 533 行测试族) 并行审同一 crate — 根因: 补派前未核对首发席仍存活。规则补: 拉起结果缺失时先 epoch 采样目标 crate 写入流判活再补派。终态主控复验: nextest 118/118 / clippy -D RC=0, 两报告数字自洽 (HCR#2 即见 HCR#1 全部修复面)。增量 must-fix M1-M7 归 W10HCS 合并处置: M1 readme 四数失实 (156/137/raw148、分项62≠72、skel84→76)、M2 CHECK→debug_assert 降级台账、M3 memory_balancer 两件纯函数本可填、M4 SpaceWithLinearArea 一名一主、M5 86 处锚-only todo 缺前置格式、M6 锚/行数勘正、M7 lib.rs pub mod 基线债归主控 (本方待办)。round1 clean 不成立维持, 待 HCS 落 readme 后 round2
- M7 主控处置判定 (heap-core lib.rs pub mod 基线债): 实测 = 12 件 plain pub mod, 无「pub mod + pub use 双路」二次导出违规; 跨 crate 消费面 grep 为 0, crate 内 tests 5 处走模块路径 → 现形态不触红线硬违规, 属根面精选导出的 API 风格债 (对标 turboshaft mod+pub use 形态); W10HCS 在飞禁动其 lib.rs, 判定: 挂「冻结收官前 API 整理票」统一处理, 不入 round2 阻塞
- W10WCb [x] 收 (主控独立复验: nextest 48/48 绿 / clippy -D RC=0 / todo 42 实测, 深水判定逐条核+台账规范化, c_api_cc.rs:23 生成残骸修实): 3 硬错 3/3 消账 (str_eq const 比较 / fp16 区间逐值 1:1 合 wasm-opcodes-inl.h:85-88 / signature 0x45 对位), 155 variants 重命名差集 0 (陷阱 21+跳表 134+非跳表 6, 补序锁定测试 2 例), clippy 124→0 真修 12 处。⚠ 保留 4 处 enum item 级命名 allow (保原版名) — 与「禁 allow 绕过」红线有张力, 交 round1 复审席 W11WCR 判定 (改写满足 lint 或主控豁免入账二选一)
- W11CG 熔断 (150 轮上限, 停在「基线快照 diff 扫描」中途): 现场三件套 = src 8683 行 / todo! 56 (⚠ 01:0x 普查曾 38, 撞线回升 18 处, 疑基线重扫回退已填体, 接力票第一任务=逐条判定「回退恢复 vs 新增草案」禁重发明) / 编译红 5×E0015 全在 code_reference.rs const fn 内非 const operator; 写手静默 49s 复测确认 → 派接力票 W11CGb (含 maglev 复验解锁优先级 + 临时快照产物清理授权限自生文件)。maglev 复验三延
- W11TSR2 round2 clean 入账 (turboshaft, 主控复验 nextest 39/39 + clippy -D RC=0): 缺陷 0/遗漏 0/新测 11; round1 两修复复核无回归; fast_hash Σ17^i 手推合原版、lattice「原版无 Merge」判定改穷举 From*Rep 19+13、resize 守卫 debug_assert 落地; skel 抽 3 组前置属实。clean 计数 = 1/3。⚠ 观察: 复审期间 src 三件 (sidetable/layered_hash_map/index) 有未归因外部写流 (use core::mem::swap/take 短式归一, TSR2 自证非其所加, 语义等价非破坏, epoch 静默复测确认) — round3 开席时以挂载树指纹 re-anchor (本轮锚: 见下行)
- turboshaft 挂载树指纹锚 (round2 末 02:5x): cat src/*.rs | shasum -a 256 前缀 = 3920f00b658e27d1 (16hex); round3 开席先复算, 不一致则写流归因后再启 clean 计数
- W10HCS [x] 收 (主控独立复验: nextest 126/126 / clippy -D RC=0 / fmt+miri 票面 RC=0, memory_balancer 挂载实见 lib.rs): R1/R2 让位闭合 (外部复审席 F4 已自修入树, 磁盘现态零命中旧锚), R3 台账 9 处修正 + memory_balancer 收编填体 2 件纯 EWMA + heartbeat/recompute 改登真前置 + 新测 8 例, object_lock 勘措辞不填体 (真前置 MutablePage::FromHeapObject), R4 Pin 约束登账且 grep 无移动路径。round1 整改闭环 → 开 round2 全新复审 W10HCR2; ⚠ 并发提醒: 外部席仍在推 heap-core 热文件, round2 采指纹锚纪律 (本轮锚:
bc1ebc298e568337) 挂载面 src/*.rs 拼接 SHA256 前缀, 漂移则归因后再计 clean
- W11HGR round1 出账 (heap-gc, 主控复验 nextest 69/69 + clippy -D RC=0): 缺陷 3 全就地修 (D1 memory_reducer 原版截断+std::max 逐行复现 / D2 sweeper AllocationSpace 判别值合 globals.h, 删臆造 CODE_MAP_SPACE + 编译期断言 / D3 台账列和 196≠208 校正 collector 60→82、visitor 21→11), 遗漏 2 登记不阻断, 25 项 W10HG 测试审计无弱化; forbid(unsafe_code) 零 unsafe。clean 计数归 0 → 开 round2 W11HGR2, 指纹锚 1592a924c73b422e
- W11WCR round1 出账 (wasm-core, 主控复验 nextest 58/58 + clippy -D RC=0): 真缺陷 1 修实 (local_decl_encoder Size() Rust 乘法不短路致数值局部恒调 heap_type() debug 崩, 改 if 守卫族合原版三元短路) / 遗漏 1 补 (跨 crate 边界测试族: fp16 区间穷尽/Emit 逐字节/槽序锁定 10 例) / allow 5→2。主控批准豁免 2 处 (现场核: wasm_opcodes.rs:1447 non_camel_case_types = WasmOpcodeSig 生成器覆跑+12.1 逐值对账关键形态, 2118 引用; wasm_linkage.rs:143 dead_code = 骨架字段唯一读者 unimplemented 深水), 豁免随 readme 名录入册, 若生成器面变更即失效重审。clean 归 0 → 开 round2 W12WCR2, 指纹锚 bfb588bd24953429 (cat src/*.rs 拼接 SHA256 前缀)
- W11CGb [x] 收 (主控复验 nextest 41/41 + clippy -D RC=0; 中途 v8-base-platform/os.rs E0425 瞬红 = 外部 L1 席 9s 内实时编辑依赖所致, 等待自愈复测绿, 非本票缺陷): 判定表 18 处增量 + 2 处静默回退 (machine_type 枚举整体丢失/aligned_slot 重构丢失) 全为「基线快照后脚本产物回写」回退, 0 新增草案 — 10 文件自 /tmp/W11CG_src_baseline 整体恢复未重发明; E0015×5 随恢复全消 (const 安全显式判别比较承载); todo 56→38→32 (浅水 6 填体 + RegisterConfiguration  ctor 1:1 移植), 32 深水规范登账; 新测 4; 临时基线自清。指纹锚 7b974b7814992452
- W9MFb 复验闭合 (三延后落定, 主控独立复验: nextest 29/29 绿 / clippy -D RC=0 / 挂载 todo 7 与台账一致): maglev-frontend src 2143 行门禁成立, 解锁根因 = codegen-core 转绿; 转 round1 复审环。指纹锚 2bab5927eeb314c0
- W11TSR3 round3 clean 入账 (缺陷0/遗漏0/新测8, 五面攻坚全过, 断言敏感性 3/5 破坏即红+2 例核为非缺陷): 复审时点 src 逐字节命中 round2 锚, clean 连续 2/3 成立; 但收官后主控复测发现树漂移 — types/representations @04:06 未归因写流 + lib.rs 被自称 v8-codegen-core 的写手误写入 v8-turboshaft 挂载声明 (他席 crate 越权, 幸为纯增量), 现态 nextest/clippy 仍双绿; 规则: 连续 clean 重锚重启 — round4 以新锚 07e02c4abdd64ffe 起算 (计数回退 0/3 重新连三), 并向外部/codegen 席发越权警示入账
- turboshaft 复审环挂起判定 (编排者, 04:2x): 外部开发流实时在推本 crate (锚 3min 内 3920→07e0→8fde 两漂, 测试 47→49 自增, lib.rs 越权写混入) — clean 计数轮在漂移树下无法成立, round4 及后续暂停派发, 待外部席静默 (epoch 连续 30min 无写) 后 re-anchor 重启 0/3; 已入账 round3 clean 保留为历史记录不作计数续用; 越权写 lib.rs 面归因登记待静默窗复核语义等价性
- W11HGR2 round2 出账 (heap-gc, 主控复验 nextest 72/72 + clippy -D 净): 缺陷 2 全就地修 (W2D1 Priority 判别值与 v8-isolate.h 倒置+编译期断言 / W2D2 event_state_to_string 臆造名→Option 合原版 nullptr), gc_tracer 迁移缺口补 2 + 新测 3 例; skel 抽 4 组无谎登但 3 组前置措辞过期已勘正留痕; 失败敏感 3/3 变异即红。审毕指纹口径差异 (席 3adfef60 vs 主控 cat 字典序 a9f79225) 归因: 复算公式统一为 `cat src/*.rs` zsh 字典序, 新锚 a9f792251cf33eeb; clean 连续 0/3 续战 → round3 W11HGR3 已派
- W10HCR2 队列故障中止 (Queuing failed, 已耗 120 工具调用/3.4h): 现场 = log 无 round2 节、allocation.rs 04:3x 有写、测试 126→128 绿、写流静默 19min → 补派接力复审席, 票面含「先盘点中止席已落地差集 (2 新测归属判定), 禁重做禁回退」; 锚漂移归因: bc1ebc29 (HCS 末) → 现锚 2761d10342be9845, 漂移含 W10HCS 外部席推进 + 中止席测试面
- W12WCR2 round2 出账 (wasm-core, 主控复验 nextest 60/60 + 锚 b89a2c27 一致): 缺陷 0/遗漏 1 已订正 (豁免名录精确复算: WasmOpcodeSig 引用 2118 属实、SI_I/S_II 碰撞示例失实勘正、allow 行不在生成器模板 → 「复跑即丢」脆弱点入账, 归后续生成器补插开发票)/新测 2; 同型短路全库扫 0 出; far-jump 空洞值「原版表外读 0 混叠槽 0」Rust 面无对应列观察项; 豁免两处维持。中途外部进程两次回写 local_decl_encoder (行级等价) 致聚合漂移按纪律未判 clean; 写流静默 29min → round3 W12WCR3 起锚 b89a2c27 重开连三计数 (此前 0/3)
- W11HGR3 round3 漂移停审入账 (heap-gc): 归因确为外部开发席新语义 (lib.rs 桥接 gc_prepare_linear_allocation_area + v8_heap_core::LinearAllocationArea 依赖边, 03:21-04:32 时间戳群, round2 预警写手兑现); 席位全程只读合规, D1/D2/W2D1/W2D2 存活无回归但移动靶不判 clean, 连续 clean 0/3 不推进。主控处置: 现态门禁抽验 72/72 绿 + 写流静默 35min → 重锚 5838c6fa51f50895, 派 round3 重试席 W11HGR3b (全新视角, 含桥接面一手对账)
