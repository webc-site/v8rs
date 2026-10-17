# v8-turboshaft 推迟台账

> 格式: 组 | 符号/面 | 前置依赖 | 解锁 crate/条件
> 依据 todo.md 12.6 (延期测试台账) + 7.1 (签名先行, 体后填)。
> 门禁基线: clippy `--all-targets -D warnings` 0 / nextest 全绿 (见 log/v8-turboshaft.md)。

## 一、挂载面推迟符号 (13 个 .rs 内〔W11 收编 sidetable/layered_hash_map/opmasks_named 后〕, 已去 panic/todo, 深水待填)

| 组 | 符号/面 | 前置 | 解锁 crate/条件 |
| --- | --- | --- | --- |
| graph | Graph::Get(OpIndex)/Index(&Op)/KillOperation | operations_ 载荷实体 (operations.rs 操作结构) + OperationBuffer 真实分配 | 本 crate: operations 面 + OperationBuffer 语义体 |
| graph | Graph::BlockOf/SetBlockOf/BlockIndexOf | op_to_block_ Sidetable | sidetable.rs 已挂载 (W11 收编); 差 Graph operations_ 载荷面 |
| graph | Graph::NextIndex/PreviousIndex/NextOperationIndex | OperationBuffer 双向游标 (begin/end/end_cap) | OperationBuffer 语义体 |
| graph | Graph::Reset/AllocateNewBlocks/GrowAndNewBlock/next_block_ 空闲槽复用 | graph_zone_ (Zone 指针稳定分配) | v8-zone + OperationBuffer |
| graph | Block::ComputeDominator/PrintDominatorTree (graph.cc 支配树) | 前驱链表 (last/neighboring/single_loop_predecessor) + 逆序挂链 | graph 前驱面就位 |
| graph | Graph sidetables: source_positions_/operation_origins_/operation_types_ | Sidetable + 各 Value 类型 | sidetable.rs 已挂载 (W11); 差 OriginInfo/Type 值面 |
| graph | block_permutation_/stack_checks_to_remove_ | Vec 语义 + phase 调用点 | 各 phase skel 就位 |
| types | Type::LeastUpperBound (types.cc LUB 分派 9 kind) | 类型格 Payload: Float64Range/Word32Set/Word64Set/Tuple + 各 Type::LeastUpperBound | types 语义轮 (本 crate types.cc 族) |
| types | Type::AllocateOnHeap | Zone 堆 + 各 Payload 分配 | v8-zone |
| opmasks_named | Simd128Binop/Unary/Shift/LoadTransform 具名掩码 (原版 FOREACH 宏族, Kind 值宏链推导) | operations.rs 现仅注册 4 个 Simd128Binop 扩展变体; 已提供通用族构建函数 k_simd128_*(kind) | operations 位域 lower 面 |
| opmasks_named | Simd256 系列具名/通用掩码 | 本目标 SIMD256=0 配置 (原版即禁用) | 目标配置开启 SIMD256 |
| sidetable | SparseOpIndexSideTable | ZoneAbslFlatHashMap 存储面 | v8-zone + absl flat map |
| graph | belongs_to_this_graph 原版世代位奇偶判定 (DEBUG) | Rust release 布局 OpIndex 无世代位, 已按语义等价面替代 (valid∧¬external∧id<op_id_count) | 引入 debug 世代面时回改 |

## 二、.skel 草案分组 (余 111 件, 未编译/惰性; Wave 11 二波已收编 3 件: sidetable / layered_hash_map / opmasks_named)

> 收编优先级: 依赖面已全就位者先。以下按族分组, 余件全部因依赖未落骨架 / Isolate / broker / CSA / Zone 而暂留 .skel。

| 组 | 代表件 (件数) | 前置 | 解锁 crate/条件 |
| --- | --- | --- | --- |
| reducer-typed | typed_optimizations / typer / type_inference / maglev_assert_types / assert_types (5) | Type 格 LUB + typer.cc 语义 | types 语义轮 |
| reducer-machine | machine_optimization / machine_lowering_inl / int64_lowering / explicit_truncation / branch_elimination / select_lowering / js_generic_lowering / dataview_lowering / fast_api_call_lowering (9) | 寄存器分配 + machine 操作码 lower 面 | codegen-core + operations lower |
| reducer-loadstore | late_load_elimination / load_store_simplification / load_store_verification / store_store_elimination / memory_optimization / string_escape / late_escape (7) | 内存模型 + effects (Sidetable 已就位) | heap/effects + operations 载荷面 |
| reducer-loop | loop_optimization / loop_peeling / loop_unrolling / simplify_tf_loops / duplication_optimization / growable_stacks / if_else_cascade_to_switch (7) | loop_finder + Graph 块遍历 + 支配树 | graph 支配面 |
| reducer-remaining | dead_code_elimination / value_numbering / variable / required_optimization / simplified_optimization / pretenuring_propagation / stack_check_lowering / reducer_traits / uniform_reducer_adapter (9) | ReducerTraits 框架 + Graph 迭代器 | analyzer_iterator + graph 遍历 |
| phase | build_graph / copying / *_phase (csa_*/loop_*/wasm_* 等) / pipelines / register_allocation_phase / random_rescheduling / decompression_optimization_phase / debug_feature_lowering_phase / type_assertions_phase / typed_optimizations_phase (约 22) | PipelineData + broker + Isolate + CodeStubAssembler | v8-compiler broker + isolate (L5) |
| csa | csa_effects_computation / csa_load_elimination / csa_memory_optimization / csa_late_escape_analysis / csa_early_machine_optimization / csa_branch_elimination (6) | CodeStubAssembler + CSA 面 | v8-compiler CSA 层 |
| wasm | wasm_wrappers(_inl) / wasm_turboshaft_compiler / wasm_*_phase / wasm_*_reducer / wasm_assembler_helpers / wasm_js_lowering / wasm_revec / wasm_shuffle / wasm_simd / wasm_gc_* / wasm_code_coverage / wasm_in_js_inlining / wasm_dead_code_elimination / wasm_load_elimination (约 24) | wasm 引擎 + broker + Turbolev | v8-wasm-* 层 (L6) |
| matcher | operation_matcher (40 todo) / typeswitch | 操作码 Match 宏展开 + ReducerTraits | operations lower 面 |
| snapshot-table | snapshot_table / snapshot_table_opindex (8) | SnapshotData + Isolate | v8-snapshot (W9SNAP 在飞) |
| infra | ~~sidetable~~ / snapshot_table / ~~layered_hash_map~~ / use_map / utils / string_view / zone_with_name / tracing / analyzer_iterator / deopt_data / access_builder / call_descriptors_util / builtin_call_descriptors / runtime_call_descriptors / assembler / graph_builder / turbolev_graph_builder / turbolev_frontend_pipeline / graph_visualizer / loop_finder / ~~opmasks_named~~ / type_parser / builtin_compiler | 混合: Zone/Isolate/broker/Handle 面。W11 判定: analyzer_iterator/loop_finder 差 Block 子块链接与图遍历面; string_view 差 assembler/DisallowGarbageCollection; zone_with_name 差 ZoneStats; type_parser 差 Isolate/Handle; use_map 评估后暂缓 (Zone+Graph 依赖) | 依赖各自上游 crate; sidetable/layered_hash_map/opmasks_named 已于 W11 收编挂载 |

## 三、原版 unittests 迁移映射 (7.3 / 12.6 延期用例)

> v8/test/unittests/compiler/turboshaft/ 共 17 件 (含 reducer-test.h/.cc 框架)。
> 已迁: opmask-unittest.cc → tests/opmask_unittest.rs (opmask_test_full_mask / opmask_test_partial_mask), 原用例名保留。
> W11 收编三组 (sidetable/layered_hash_map/opmasks_named) 原版无对应 unittests (已查 v8/test/unittests/compiler/turboshaft/ 与 v8/unittests), 按 7.3 以机械自证用例补齐: tests/sidetable.rs (5) / tests/layered_hash_map.rs (5) / tests/opmask_table.rs (7)。
> 其余全部依赖 ReducerTest → Isolate + broker + CodeStubAssembler + 执行层, 当前 L5 未就位, 延期:

| 原用例文件 | 用例/面 | 缺前置 | 解锁 crate |
| --- | --- | --- | --- |
| control-flow-unittest.cc | ReducerTest 控制流图构造 | Isolate + GraphBuilder + reducer | v8-compiler broker (L5) |
| reducer-test.{cc,h} | ReducerTest 测试框架本体 | Isolate/broker/FunctionTester | L5 执行层 |
| snapshot-table-unittest.cc | SnapshotTable + Isolate 快照 | v8-snapshot (W9SNAP 在飞) | v8-snapshot |
| turboshaft-types-unittest.cc | 类型格 LUB/相交/包含 | types 语义轮 (本 crate) | v8-turboshaft types 体 |
| turboshaft-typer-unittest.cc | Typer 推断 | typer.cc + Type 格 + Graph | typer skel 就位 |
| typeswitch-unittest.cc | TYPESWITCH/CASE_ 分派 | MachineLoweringReducer + FunctionTester | L5 执行层 |
| call-runtime-unittest.cc | CallRuntime lowering | runtime 调用面 + Code | v8-runtime (L4) |
| duplication-optimization-reducer-unittest.cc | ReducerTest | reducer 框架 | L5 |
| instruction-selection-normalization-reducer-unittest.cc | ISN reducer | instruction-selection 面 | codegen-core |
| late-load-elimination-reducer-unittest.cc | LLE reducer | Sidetable + effects + L5 | L5 reducer 族 |
| load-store-address-hoisting-unittest.cc | LSAH reducer | L5 reducer 族 | L5 |
| loop-unrolling-analyzer-unittest.cc | 环展开分析器 | loop_finder + Graph 支配面 | graph 支配面 |
| machine-optimization-reducer-unittest.cc | machine-opt reducer | 寄存器分配 + machine lower | codegen-core |
| store-store-elimination-reducer-unittest.cc | SSE reducer | 内存模型 + L5 | L5 reducer 族 |
| wasm-shuffle-reducer-unittest.cc | wasm shuffle | wasm 引擎 (L6) | v8-wasm-* |
| wasm-simd-unittest.cc | wasm simd | wasm 引擎 (L6) | v8-wasm-* |

## 四、函数级对账 (12.1) 锚面现状

- 挂载件 `/// v8:` 锚: opmasks 10 / fast_hash 4 / graph 1; operations/representations/types/index/phase 以 `/// 原版 X` + `// v8: src/...` 头登记, 逐函数 `/// v8: file Sym` 锚待补 (Reviewer 抽查项, 非门禁)。
- 产物↔脚本对数: 全部 122 模块 (11 挂载 + 111 skel) 头名生成脚本存在 (缺族 = 0); tool 脚本 121 件, 同名族 (.h/.cc) 合并。
- 幂等复跑: 挂载件已就地修订, 按 414 条不重跑覆盖基线; 仅 skel 件可安全重生成。W11 对收编三组脚本复跑幂等核验通过 (layered-hash-map.h.js / sidetable.h.js / opmasks.h.js); 注意 opmasks.h.js 直写 src/opmasks.rs, 复跑覆盖已按「产物与 src 冲突以 src 现态为准」恢复, 再生的 3 个 .skel 占位已删除。
