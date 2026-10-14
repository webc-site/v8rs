# v8-turboshaft 开发日志

> 2026-10-12: 自 todo.md 看板行与 Wave 9 派发收敛 (仅中文)。

## 门禁实测

- 单元测试: `cargo nextest run -p v8-turboshaft` (24/24 全部通过, Wave 11 W11TS2 后)
- Clippy: `cargo clippy -p v8-turboshaft --all-targets -- -D warnings` (0 警告)
- 格式化: `cargo +nightly fmt --check -p v8-turboshaft` (净)

## 模块与实现面

1. **representations**: 提取 Turboshaft 表示体系，包括 `MaybeRegisterRepresentation`、`RegisterRepresentation` 与 `MemoryRepresentation`，对齐 V8 x64/arm64 目标配置。
2. **operations**: 提取 Turboshaft 全部 176 项操作码（`Opcode`），对齐块终结符判定（`is_block_terminator`，前缀 9 项）与 `may_throw` 判定集合，以及 41 个 `Kind` 子枚举。
3. **types**: Turboshaft 类型系统抽象，包括 `Type`、数值边界（`Float64Range`、`Word32Set` 等）与 `Payload`。
4. **graph**: 图与基本块容器（`Graph`、`Block`、`OperationBuffer`），支持块创建、操作分配与索引。
5. **opmasks**: 小端操作掩码机制（`OpMask`、`MaskBuilder`、`OpMaskField`），支持基于首 8 字节与变长字段掩码的高性能模式匹配。
6. **fast_hash**: Turboshaft 专用快哈希算法（`fast_hash_combine`、`fast_hash_combine2`、`fast_hash_range`）。
7. **index**: 操作与块索引定义（`OpIndex`、`BlockIndex`、`OptionalOpIndex`），支持外部节点来源标记与紧凑 id 映射。
8. **phase**: 编译 phase 驱动接口（`Phase`、`PipelineData`、`TurboshaftPipelineKind`）。
9. **sidetable**: 侧表家族（`GrowingSidetable`、`FixedSidetable`、`GrowingOpIndexSidetable`、`FixedOpIndexSidetable`），对齐原版 NextSize 增长、Reset 保内存、DEBUG 图归属断言语义（Wave 11 收编）。
10. **layered_hash_map**: 分层开放寻址哈希表（`LayeredHashMap`），逐层头插链（索引代指针）、0.75 占用率 2× 扩容、按深度升序重排保证探测链不断裂（Wave 11 收编）。
11. **opmasks_named**: 具名操作掩码常量表（22 个 MaskBuilder 族 + 64 个具名掩码 + 4 个 SIMD 通用族构建函数），字段偏移按原版 Operation 4 字节基座推导（Wave 11 收编）。

## Wave 9 W9TSb 接力 (2026-10-13)

> 接力中断代理 W9TS (撞 150 轮上限)。铁律: 先实测盘点、只补差集、禁重做已落地内容。

### 实测盘点 (纠偏前任自述)

- 前任自述「lib 编译通过」不实: `cargo check`/`clippy` 实测 lib 与 lib-test **均不编译** (7 硬错误)。
- 基线纠偏: 编排者报「挂载 10 文件 3562 行 / tests 仅 main.rs」→ 实测挂载已含 `tests/opmask_unittest.rs` (前任实落), clippy 面为 error 非仅 warning。
- 差集定位: ① `src/fast_hash.rs` 顶层 `const _: ()={…}` 块内调用非 const 的 `FastHash::fast_hash`/`fast_hash_combine`/`fast_hash_range` → 7×E0015; ② `graph.rs:4` 死导入 `OperationStorageSlot`; ③ `graph.rs` `Graph::new(origin)` 参数未用 (字段全为 `()` 占位 + 20 `unimplemented!`); ④ `representations.rs` 两处 `From*Representation` 尾部 `_ => panic!`; ⑤ `types.rs` `least_upper_bound()` `todo!` 孤儿桩。

### 面 1 · clippy 清零 (含修复硬错误)

- 删 `fast_hash.rs` 坏 const 块, 精确不变式 (`combine(&[3,5])==88` 等) 1:1 移入 `tests/main.rs::test_fast_hash` (断言强度守恒, 不弱化)。
- 两处 `From*Representation` 通配 `panic!` → 按原版 `UNREACHABLE()` 枚举 kNone/kFloat16RawBits/(+Bit/MapWord/Compressed*/Protected/Indirect) → `unreachable!()` (const fn 内, 无格式化消息)。
- graph.rs: 删死导入; `Graph`/`OperationBuffer` 重写为**最小自洽实现** (origin 落地 + 空缓冲/空块表 + block_count/op_id_count/op_id_capacity/next_block_index/new_block/get_block/start_block + Block::is_merge 等), 深水面 (Get/KillOperation/dominator/sidetable/Zone) 移出并登记台账; 去 `manual_div_ceil` (改 `div_ceil`)。
- types.rs: 删 `least_upper_bound` 桩, 分派面转 `//` 推迟注释入台账。
- doc-lint: 内部说明块 `///`→`//` (fast_hash/opmasks/graph/types), 保留 `/// v8:` 锚。
- 门禁: `clippy --all-targets -D warnings` **EXIT=0 / 0 警告**; `cargo +nightly fmt --check` **净**。

### 面 2 · 挂载语义核验

- 挂载 10 件 `.rs` 共 **4094 行** (operations 2637 / representations 541 / graph 262 / index 202 / types 161 / opmasks 102 / fast_hash 88 / phase 51 / lib 40 / error 10)。
- 挂载面 `todo!`/`unimplemented!`/`panic!` **→ 0**; 仅 2× `unreachable!()` (对应原版真 UNREACHABLE)。
- representations/operations/types 关键语义逐行核原版 (From*Representation/ToMachineType/SizeInBytes 与 opmask/opcode 判定), 经现有单测断言印证。

### 面 3 · skel 收编

- **0 收编 (114 → 114)**。循「宁少勿滥」: 逐一评估候选 (layered_hash_map 双线索链非平凡且消费者全推迟; opmask 具名表需逐操作码布局; sidetable/type_parser 依赖未落), 强行收编即劣化 → 全部 114 件按族分组成 19 组入台账 (依赖/前置/解锁标注)。

### 面 4 · 测试

- 现状: `tests/main.rs` 5 例 + `tests/opmask_unittest.rs` 2 例 = **nextest 7/7 全绿**。
- 原版 17 unittests 中仅 opmask 可迁 (已在); 其余 (typeswitch/typer/types/reducer-* /wasm-*) 全依赖 ReducerTest→Isolate/broker/执行层, 17 件延用例逐件入台账「延期用例」节 (缺前置 + 解锁 crate)。

### 面 5 · 脚本核查 (非破坏)

- 121 脚本: shebang 违例 0; import 仅 node 内置 + `../../../lib.js` (无越权); lib.js 未改。
- 产物↔脚本: 122 模块 (8 挂载核 + lib + error + 114 skel) 头名生成脚本**全部存在, 缺族 0**。
- 挂载件按 414 条不重跑 (重跑覆盖修订); 幂等仅适用 skel 件。
- 对账锚现状: `/// v8:` 逐函数锚 opmasks 10 / fast_hash 4 / graph 1, 余以 `/// 原版`/`// v8:` 头登记, 逐函数锚待补 (Reviewer 抽查项, 非门禁), 入台账。

### 收官

- 门禁全绿: clippy `-D warnings` 0 / fmt 净 / nextest 7/7。
- 台账: `readme/zh.md` 建立 (挂载推迟符号 9 行 + skel 19 组 + 延期用例 17 件 + 对账锚现状)。
- 稳定态: crate 全程可编译, 无红态遗留。

## Wave 11 W11TS2 接力 (2026-10-13)

> 任务: skel 收编二波 — 自 19 组中挑依赖面已就位者, 收编挂载 → 填体 (1:1 脚本产物 + 原版源) → 配测。铁律: 全程 `cargo check` 绿, 宁停稳定态勿留红。

### 选组判定

- 逐组核对台账「前置」列: **sidetable / layered_hash_map / opmasks_named** (infra 组) 依赖面全就位 (fast_hash、index、graph、operations Kind 判别式、representations 编码均已挂载), 收编。
- 明确跳过 (留 .skel): analyzer_iterator/loop_finder 差 Block 子块链接与图遍历面 (推迟 graph 体, 禁改); string_view 差 assembler/DisallowGarbageCollection; zone_with_name 差 ZoneStats; type_parser 差 Isolate/Handle; use_map 评估后暂缓 (Zone+Graph 依赖); reducer/phase/csa/wasm 各族差 operations 载荷实体 + Typer/ReducerTraits/L5。

### 收编挂载 (3 组, 114 skel → 111)

- **sidetable.rs** (248 行): GrowingSidetable (NextSize=i+i/2+32, Reset 保内存 fill 默认), FixedSidetable (DCHECK 越界→None), Growing/FixedOpIndexSidetable (DEBUG graph_ → `Option<&Graph>` + debug_assert), swap_data, op_index_belongs_to_table_graph; Block 别名类型就位。**Graph 侧配套**: graph.rs 增加 `belongs_to_this_graph` (导出面扩展; 原版 DEBUG 世代位判定在 Rust release 布局下按语义等价面替代: valid∧¬external∧id<op_id_count)。SparseOpIndexSideTable (ZoneAbslFlatHashMap) 留台账。
- **layered_hash_map.rs** (223 行): 开放寻址 + 索引代指针逐层双探链 (NO_ENTRY 哨兵), StartLayer/DropLastLayer (链walk 清位), hash==0 空槽哨兵 + ComputeHash 0→1 修正, ResizeIfNeeded 0.75/2× 且按深度升序重排 (防探测链断裂)。
- **opmasks_named.rs** (769 行): 22 MaskBuilder 族 + 64 具名掩码 (WordBinop 16/WordUnary 2/FloatUnary 3/FloatBinop 4/Shift 9/DeoptOnOverflow 1/Phi 1/Constant 8/Projection 2/Comparison 3/Change 9/OverflowChecked 1/TaggedBitcast 4/ReplaceLaneF32x4 1) + 4 SIMD 通用族 const fn; 字段偏移按 Operation 基座 4 字节 (opcode@0/input_count@2) 逐结构体核原版。推迟入台账: Simd128Binop 等宏链具名 5 件 (operations 仅注册 4 扩展变体), Simd256 (SIMD256=0), ConvertOpTargetMask (原版仅文档示例)。
- 挂载面合计: 10 件 4094 行 → **13 件 5356 行 (+1262)**; 收编 skel 内 todo!/unimplemented! **10 处清零** (4 todo! + 6 unimplemented!), 新文件零 todo。

### 配测 (7.3 断言强度守恒)

- 查证原版: 三组**无对应 unittests** (v8/test/unittests/compiler/turboshaft/ 无 layered-hash-map/sidetable/opmask 具名表用例) → 机械构造自证 (Isolate-free): tests/sidetable.rs 5 例 (增长式/Reset 保内存/swap_data/定长越界/OpIndex 定长), tests/layered_hash_map.rs 5 例 (插查/落层保旧层/扩容保深度序 L1+L2 跨表/删层重插/零哈希哨兵), tests/opmask_table.rs 7 例 (独立字节布局 oracle 核 mask/value 全等, 含 change_op/tagged_bitcast 多字段与 SIMD 通用族)。
- 新增 17 例, nextest **7 → 24/24 全绿**。

### 脚本幂等复跑核验

- `layered-hash-map.h.js` / `sidetable.h.js` / `opmasks.h.js` 复跑通过 (断言核原版源未变, 输出 24 族/69 具名/SIMD-X-macro 5)。
- **冲突处置**: opmasks.h.js 直写 `src/opmasks.rs` 覆盖基线 — 按「产物与 src 冲突以 src 现态为准」恢复现态 (补缺失 `///` 空行; 头注更新「具名表已收编至 opmasks_named.rs」); 再生的 3 个 .rs.skel 占位删除。

### 收官

- 门禁: clippy `--all-targets -D warnings` **0** / nextest **24/24** / `cargo +nightly fmt --check` **净**; 全程无红态。
- 台账: readme/zh.md 已更新 (收编 3 组划掉, skel 114→111, 挂载推迟符号表 +4 行新延期项, graph sidetable 前置改注, 三组无原测记录, 对账 11 挂载 + 111 skel)。
- 红线性核查: 未触碰其他 crate / tool 域 / lib.js / 已挂绿色语义体 (仅 graph.rs/lib.rs 必要导出面扩展); 无 panic/unwrap/#[allow]。

## W11TSR round1 复审 (2026-10-13)

> 对抗性复审席 (只读为主 + 补缺陷 + 补测试)。先实测复核基线, 再逐件对原版核语义, 最后补语义级用例并清门禁。

### 基线复验 (编排者报数核对)

- 挂载 **13 件 5356 行** ✓ / 挂载面 `todo!`/`panic!` **0** ✓ / skel **111** ✓。
- `cargo nextest run -p v8-turboshaft` **24/24 绿** (复审前) / `clippy --all-targets -D warnings` **RC=0** (复审前)。CARGO_TARGET_DIR=target/agent_W11TSR 单包隔离, 未跑全仓。

### 面 1 · W11TS2 三新件 1:1 语义复核 (对照原版)

- **sidetable.rs vs sidetable.{h,cc}**: NextSize=`i+i/2+32` ✓ (空表 i=0→32, index 0 落位, 稀疏 get 只读不增长); Reset `fill(T::default())` 保容量 ✓; operator[] const/非 const 双版在 Rust 拆为 `get`(&self 不增长)/`get_or_grow`(&mut 增长+吞并 capacity), 差异已在注释登记; FixedOpIndex/GrowingOpIndex 的 DEBUG `graph_` → `Option<&Graph>` + `debug_assert` (is_none_or), 对齐原版 `#ifdef DEBUG` 面 ✓; `op_index_belongs_to_table_graph` = `graph->BelongsToThisGraph` ✓。SparseOpIndexSideTable 留台账 (ZoneAbslFlatHashMap), 属实。
- **layered_hash_map.rs vs layered-hash-map.h**: 0.75 装载整数精确式 `len*3 > count*4` ✓; 2× 扩容; hash==0 空槽 + `ComputeHash 0→1` ✓; `resize_if_needed` 按 `depths_heads` 索引升序 (逐层) + 层内链头前插重灌, 与原版深度升序防探测空洞语义逐位一致 ✓; `drop_last_layer` 沿链 `hash=0` 清位 ✓; 越界 `find_entry_for_key` 终止于空槽 ✓。**须记**: 原版 ctor 最小 16/bit_ceil, Rust `max(16).next_power_of_two()` ✓。
- **opmasks_named.rs vs opmasks.h**: 22 族 + 具名掩码逐结构体核 operations.h 基座 4 字节 (opcode u8@0/use u8@1/input_count u16@2), 载荷自 @4: WordBinop/Unary/Float*/Shift/Comparison kind@4 rep@5 ✓; Phi rep@4 ✓; Constant kind@4 ✓; Projection index u16@4 (size 2) ✓; Change kind@4/assumption@5/from@6/to@7 (声明序核对 operations.h ChangeOp) ✓; TaggedBitcast **字段声明序 kind@4/from@5/to@6**, MaskBuilder 实参序 (from,to,kind) 对应偏移 5,6,4 — 逐一核对无错位 ✓; OverflowChecked kind@4/rep@5 ✓; DeoptOnOverflow kind@4 ✓; Simd128* kind@4, LoadTransform transform_kind@5 (load_kind@4 packed) ✓。Kind 判别值 (WordBinop 12/Change 14/Constant 16/Comparison/TaggedBitcast/Shift/Simd128ReplaceLane) 序与原版 enum 完全一致 ✓。Simd128Binop 宏链具名 5 件 + Simd256 + ConvertOpTargetMask 延期入台账, 属实 (目标 SIMD256=0, 无消费者)。
- **graph belongs_to_this_graph release 等价**: 原版 `#ifdef DEBUG` 世代奇偶 (`idx.generation_mod2()==gen_mod2`), release 编不出; Rust 用 `valid ∧ ¬external ∧ id<op_id_count` 语义面, debug_assert-only。release 面 (无检查) 完全等价 ✓; DEBUG 面非世代位替代 (Rust OpIndex 无世代位), 已在台账行明记「引入 debug 世代面时回改」。

### 面 2 · W9TSb 抽审

- representations.rs 无 panic 残留: 仅 2× `unreachable!()` (const fn), 精确对应原版 `FromMachineRepresentation`/`MemoryRepresentation` 尾 UNREACHABLE 面 (kNone/kFloat16RawBits 等), 逐 case 对照 representations.h L278/L882 无误 ✓。
- fast_hash const 正确性: `combine2=17*acc+value`; `combine(&[..])` 权重 `17^(i-1)` 首元素权重 1 = 原版变参右折叠展开 ✓; `range` 左折叠 `17*acc+v` ✓; `pair`=`combine(&[first,second])`=`17*hash(second)+hash(first)` 对齐 `fast_hash<pair>` ✓; Option `(h<<1)+1/0` ✓。
- 台账真实性抽查: skel todo/unimplemented 消息 (type_parser parse_type / zone_with_name allocate / assembler begin·advance / loop_finder ComputeLoopInfo) 与 readme/zh.md infra·reducer-loop 组「前置/解锁」标注一一对应, **无谎报** ✓。

### 面 3 · skel 111 跳过原因核查

- analyzer_iterator 差 Block 子块链接/图遍历 (visited 需 FixedBlockSidetable + Graph), loop_finder 差支配面, string_view 差 Assembler/GC scope, zone_with_name 差 ZoneStats, type_parser 差 Isolate/Handle, reducer/phase/csa/wasm 族差 operations 载荷实体 + Typer/ReducerTraits/L5。逐组判定属实, 未见「可收编却谎报前置」。

### 缺陷修复 (src, 低风险就地)

1. **[低危·风格红线]** `lib.rs: pub mod opmasks_named;` 违反 AGENTS.md「禁止 pub 模块 (consts 除外)/无二次导出」红线 (W11TS2 引入) → 改 `mod opmasks_named; pub use opmasks_named::*;` (对齐本 crate operations/types 既有 glob 导出), tests/opmask_table.rs 导入面同步改根路径。
2. **[低危·前置]** `layered_hash_map.rs insert_new_key` 原版 `depths_heads_.back()` 无当前层为 C++ UB, Rust 侧静默插孤儿项 (resize 期无链头即丢失且 entry_count 失真) → 补 `debug_assert!(!depths_heads.is_empty(), "需先 StartLayer")`, release 行为不变 (不引 panic)。

### 补测 (4 例, 语义级, 原 24→28)

- sidetable: `growing_old_indices_valid_after_growth` (增长保旧低索引/index0/get 只读不增长/稀疏默认) + `reset_then_further_growth` (Reset 保容量后继续增长且不复活旧值)。
- layered_hash_map: `cross_layer_collision_chain_survives_resize` (跨层同桶链经 2× 重排后逐层 DropLastLayer 不截断更深层探测链, 复刻原版 a1/a2/a3 注释场景)。
- opmasks_named: `named_mask_field_union_bitwise` (mask=base|逐字段掩码无交并、value=逐字段按位或可加、XOR  confined 于掩码位; CHANGE_OP 4 字段逐位分解对账)。

### 门禁收官 (全部实跑)

- `cargo nextest run -p v8-turboshaft` **28/28 绿** (新增 4 例全过)。
- `cargo clippy -p v8-turboshaft --all-targets -- -D warnings` **RC=0**。
- 全程 CARGO_TARGET_DIR=target/agent_W11TSR 单包隔离, 未触碰其他 crate/tool/lib.js; 无 panic/unwrap/#[allow]; 注释中文; `/// v8:` 锚保留。

### 三数 + must-fix

- **缺陷数 2** (均已就地修至绿) / **遗漏数 0** (三新件 1:1 语义无缺项, release 等价论证成立) / **新增测试数 4**。
- must-fix (按严重度):
  - [中·观察非阻塞] `layered_hash_map resize_if_needed` 无原版 `CHECK_LE(size, max/2)` 溢出守卫 — 内存受限下 2× 实不可能溢出, 现以注释论证; 若未来引入可运行大表压力测试, 宜补 `debug_assert!(old.len() <= usize::MAX/2)`。
  - [低·已修] pub mod / insert_new_key 前置 (见上)。
  - [信息] 三组 opmask/sidetable/layered_hash_map 原版无 unittests, 现全部为机械 + 语义自证 (非原测移植), 台账已如实标注, 待 L5 执行层就位可考虑接 reducer-test 框架。

## W11TSR2 round2 复审 (2026-10-13)

> 全新视角, 不照抄 round1 覆盖面。方向: 复核 round1 两 src 修复无回归 + 深攻 round1 未测面 (fast_hash 金标准 / graph op_id / representations 穷举映射) + opmasks 头注一致性 + skel 3 组抽验 + resize 溢出守卫定论。

### 基线复验

- 挂载 **13 件 5356 行** ✓ / 挂载面 `todo!`/`unimplemented!`/`panic!`/`#[allow]`/`unwrap()` **grep 全 0** ✓ / skel **111** ✓。
- `cargo nextest run -p v8-turboshaft` (复审前) **28/28 绿** / `clippy --all-targets -D warnings` **RC=0**。CARGO_TARGET_DIR=target/agent_W11TSR2 单包隔离, 未跑全仓。

### 面 1 · round1 两 src 修复复核 (无回归)

- **[1a glob 重导出]** `lib.rs:37-38 mod opmasks_named; pub use opmasks_named::*;` 复核:
  - 可达性: opmasks_named 声明面 (22 `pub static` MaskBuilder + `pub const fn k_simd128_*` 4 族 + 私有 KIND_4/REP_5) 经 glob 全部落到 crate 根, `tests/opmask_table.rs` 走根路径导入 (round1 已同步), 8 用例仍绿 ✓。
  - 禁二次导出红线: 全库 `pub use`/`pub mod` **仅 lib.rs 12 行** (crate 根 mod+pub use 模式, 合规); `opmasks_named.rs` **零 pub use/pub mod** (第 4 行 `use crate::{...}` 为私有导入, 非再导出)。opmasks.rs 具名 4 符号 (MaskBuilder/OP_MASK_OPCODE_BASE/OpMask/OpMaskField) 与 opmasks_named glob **无同名交集** → 无歧义/无重复定义漂移 ✓。
- **[1b insert_new_key debug_assert]** `layered_hash_map.rs:141-144` 前置断言, 对照原版 `layered-hash-map.h`:
  - 原版 ctor `depths_heads_(zone)` **初始空**, `InsertNewKey` 内 `depths_heads_.back()` (读+写两处) 在无当前层时确为 **C++ UB** (ZoneVector::back 仅 DCHECK_GT(size,0), release 编出越界读) → Rust `debug_assert!(!depths_heads.is_empty())` 忠实显式化前置, 语义等价 ✓。
  - 位置: debug_assert 在 `resize_if_needed()` **之前**; 但 resize 不增删层数 (仅重灌既有 head), 空↔空守恒 → 前置断言置前/置后等价 ✓。release 面 `unwrap_or(NO_ENTRY)` 有定义 (孤儿项), 严格优于原版 UB, 台账论证成立 ✓。

### 面 2 · round1 未深攻面 (金标准核验)

- **fast_hash 折叠方向 (原版 fast-hash.h 变参递归)**: 手推 `C(v0..v_{k-1}) = 17·C(v1..) + hash(v0)`, 展开 = Σ 17^i·hash(v_i) (首元素权重 1)。逐一核 Rust:
  - `fast_hash_combine(&[..])` 权重循环 = Σ17^i·v_i ✓; `fast_hash_combine2(a,b)=17a+b` 对应非模板 2-size_t 重载 ✓; `fast_hash_range` 左折叠 `acc=combine2(acc,*v)` ✓; `fast_hash_pair(a,b)=combine(&[a,b])=a+17b=combine2(b,a)` (pair 走模板变参, 与 OpIndex-pair 实际用例一致) ✓; Option `(h<<1)+1` ✓。
  - **鉴别点**: range(&[3,5])=56 vs combine(&[3,5])=88 (折叠方向相反, 同输入不同值) — 新增金标准断言锁定不反向。原版 pair 对 size_t×size_t 走非模板 17·first+second, 对包装/枚举走模板 17·hash(second)+hash(first); Rust pair 建模模板路径, 与 turboshaft 实际 OpIndex/Kind pair 用法一致 ✓。
- **graph op_id 计数 (原版 graph.h L873/L883)**: `op_id_count=(size+kSlotsPerId-1)/kSlotsPerId` ≡ Rust `size().div_ceil(2)` ✓; `op_id_capacity=capacity/kSlotsPerId` (floor) ✓。**分岔鉴别**: slot=1→count=1 而 capacity=0, slot=3→count=2 而 capacity=1 (count ceil / capacity floor 语义分工) — 新增金标准覆盖 (W9TSb manual_div_ceil→div_ceil 变更点)。
- **graph belongs_to_this_graph**: 原版 DEBUG `idx.generation_mod2()==generation_mod2()`; Rust release 面无世代位, 三重合取 `valid ∧ ¬external ∧ id<op_id_count` — 越界/INVALID/external 各分支新增具体 OpIndex (offset 0/16/32/48 + external) 断言, 与 round1 论证一致 ✓。
- **representations.rs lattice**: grep 原版 representations.{h,cc} **无 Merge/Meet/Join/LUB 函数** (仅 assembler/graph-builder 的 Block kMerge, 非表示格) → 本快照原版即无表示合并格面, **非遗漏**; Rust 未实现 meet/join 正确。转攻穷举映射:
  - `RegisterRepresentation::from_machine_representation` 逐 case 对原版 L278 (Bit/W8/W16/W32→Word32, Indirect/Sandboxed→WordPtr(=Word64), MapWord→Tagged, Compressed*/Float16→…共 19 可达), 无错位 ✓。
  - `MemoryRepresentation::from_machine_representation` 逐 case 对原版 L882 (整型默认 Uint*, 8 项 UNREACHABLE 面一致) ✓。
  - `SizeInBytesLog2` 压缩指针开 (kTaggedSizeLog2=2 / kSystemPointerSizeLog2=3) 关键面: AnyTagged/TaggedPointer/TaggedSigned/ProtectedPointer→4, AnyUncompressed*/UncompressedTagged*→8, Indirect/Trusted→4, Sandboxed→8, Float16→2 — 对原版逐字节 ✓ (round1 仅测整型/Simd)。
- **opmasks.rs 头注一致性 (方向 3)**: 头注 line 2「具名表已收编至 opmasks_named.rs (挂载面)」与现态一致 ✓; 机制面 build_field_mask/encode_field_value 移位式对原版 opmasks.h L114-131 小端分支 (`ones<<(offset*8)`) 逐项核 ✓; 与 opmasks_named 符号分工无重复定义 (见 1a)。

### 面 3 · skel 3 组抽验 (避开 round1 已个体核过组)

- **builtin_call_descriptors.rs.skel** (179 行, 0 todo): 仅出 `BUILTIN_DESCRIPTOR_NAMES` 144 名 + `BuiltinCallDescriptor` 骨架; 依赖 `OpEffects`/`OperatorProperties`/Builtin 枚举 id — grep 确认 **OpEffects 仅 operations.rs 注释提及未定义, RuntimeFunctionId/OperatorProperties 全库未定义** → 前置未落属实, 留 skel 正确 ✓。
- **runtime_call_descriptors.rs.skel** (87 行, 0 todo): `RuntimeCallDescriptor` 引用 `RuntimeFunctionId::Abort` 等 — 该枚举未定义, `Linkage::GetRuntimeCallDescriptor` 装配面未落 → 跳过原因属实 ✓。
- **loop_unrolling_phase.rs.skel** (33 行, 1 unimplemented "编排 CopyingPhase 链"): 需 Zone + PipelineData + reducer 链 (全 skel) → 跳过原因属实 ✓。
- 三组均无「可收编却谎报前置」。

### 观察加固 · resize 溢出守卫定论 (方向 5)

- 原版 `ResizeIfNeeded` `CHECK_LE(table_.size(), size_t::max()/kGrowthFactor)` 为 **release 生效的硬 CHECK**。判定: **维持可达性论证 + 补 debug_assert 显式化前置** (非缺陷修复, 记为观察闭环)。
  - 论证: 扩容仅当 `len*3 <= entry_count*4` (≥75% 满) 触发; 达 `usize::MAX/2` 需插入约 3/4·MAX 条, `vec![Entry; n]` 分配 OOM abort 远早于此, debug 面 Rust 整数溢出先 panic → 2× 溢出实践不可达, 原注释论证成立。
  - 加固: 按本文件既有约定 (DCHECK_GT/DCHECK_EQ 均以 debug_assert 显式化), 在 `take(&mut self.table)` 前补 `debug_assert!(self.table.len() <= usize::MAX / GROWTH_FACTOR, "原版 CHECK_LE 溢出前置")`。release 零成本, 若日后 GROWTH_FACTOR 变动或引入大表压测可先于乘法暴露越界。**残留差**: 原版 CHECK release 生效、本断言 debug-only — 因实践不可达 + Vec OOM 兜底, 判非阻塞。

### 补测 (11 例金标准, 语义级, 28→39)

- tests/fast_hash_golden.rs (5): 变参权重式具体值 / combine2 vs combine 折叠方向交叉鉴别 / range 左折叠具体值 (range≠combine) / pair 金标准+combine2 对合 / 分发表 u64/u32/usize 恒等+Option 位移。
- tests/graph_opid_golden.rs (3): op_id_count div_ceil 逐值 / op_id_capacity floor 逐值 (与 count 分岔) / belongs_to_this_graph release 面四分支 (越界/INVALID/external)。
- tests/representations_golden.rs (3): register from_machine 19 可达穷举 / memory from_machine 13 可达穷举 / size_in_bytes 压缩指针关键面。

### 门禁收官 (全部实跑)

- `cargo nextest run -p v8-turboshaft` **39/39 绿** (新增 11 例全过)。
- `cargo clippy -p v8-turboshaft --all-targets -- -D warnings` **RC=0**。
- `cargo +nightly fmt --check -p v8-turboshaft` **RC=0**。
- 全程 CARGO_TARGET_DIR=target/agent_W11TSR2 单包隔离, 未触碰其他 crate/tool/lib.js/禁二次导出红线; src 仅 layered_hash_map 一处观察加固 (debug_assert, release 行为不变); 无 panic/unwrap/#[allow]; 注释中文; `/// v8:` 锚保留。
- 挂载面: 13 件 5356 → **5372 行** (仅观察加固块 +16); skel 111 不变。
- 共享树告警: 本仓工作树当前为多代理并发改动态 (heap-core/wasm-core/objects-core/codegen-core/maglev 等 10+ crate 未提交)。**W11TSR2 本人 src 手改仅 layered_hash_map 一处 debug_assert**; 复审期间观察到 `index.rs`/`sidetable.rs`/`layered_hash_map.rs` 出现 `use core::mem::swap/take` + 调用点全路径→导入短式 (以及 `crate::representations::RegisterRepresentation`→`RegisterRepresentation`) 的路径归一化, **非 cargo fmt 所加** (fmt 不新增导入, 且 HEAD 版路径为 --check-clean), 判定为并发 cargo-fix 类工具或另一 agent 的 cosmetic 改动, 语义等价的非破坏; 复审席不做 revert 以免冲突。三门禁在现态下全绿。

### 三数 + must-fix

- **缺陷数 0** (round1 两处修复复核正确无回归; round2 未发现功能缺陷) / **遗漏数 0** (representations 无格合并面属实、opmasks 头注/分工一致、skel 3 组跳过原因属实) / **新增测试数 11**。
- must-fix: **无阻塞项**。
  - [低·已闭环] resize CHECK_LE 溢出守卫 → debug_assert 显式化 (release 生效差因实践不可达判非阻塞, 若引入大表压测再议 release 硬守卫)。
- **round2 clean**: 0 缺陷 / 0 遗漏 / 门禁 (nextest 39/39、clippy、fmt) 全绿。

## W11TSR3 round3 复审 (2026-10-13)

> 全新视角第三席, 不照抄 round1/round2 覆盖面。方向: 指纹锚定 → graph 逐字段+批量随机对账 / index 区间算术 / operations·representations 跨文件引用一致性 / 断言强度实证 (故意破坏 impl 常量看是否转红) / skel 3 新组抽验。全程 CARGO_TARGET_DIR=target/agent_W11TSR3 单包隔离, 仅 -p v8-turboshaft, 未跑全仓门禁。

### Step 0 · 指纹锚定

- `cat v8-turboshaft/src/*.rs | shasum -a 256` = **`3920f00b658e27d1747f0d2503cd3750e79db467a318ec7e0415245cf8b09572`** (13 文件 / 5372 行)。
- 与 round2 末锚前缀 `3920f00b658e27d1` **逐字节命中** → 树一致, 走正常复审, 不降级。
- 实验期 src 经"破坏→还原"多次触点后, 收官时仍回到该指纹 (见「并发写告警」), 锚点全程自洽。

### 基线复验

- 挂载 **13 件 5372 行** ✓ / 挂载面 `todo!` **grep 0** ✓ / skel **111** ✓。
- 本轮新增测试前 nextest **39**; 新增 `tests/turboshaft_reconcile.rs` (8 例) 后 **47/47 绿**。

### 面 1 · graph.rs 逐字段 vs graph.h + 批量随机对账

- 逐字段核对: `op_id_count() = operations.size().div_ceil(SLOTS_PER_ID) as u32`; `op_id_capacity() = (operations.capacity() / SLOTS_PER_ID) as u32`; `OperationBuffer` pub 面 `slot_count`/`slot_capacity` (kSlotsPerId=2, slot=u64/8B); `belongs_to_this_graph = index.valid() && !index.is_external_id() && index.id() < self.op_id_count()`。与原版分母/取整方向一致, 无字段缺失 ✓。
- 新增加固测试 (固定种子 Lcg, 4096 轮): `graph_op_id_batch_random_reconcile` 用手写公式 count=(n+1)/2、capacity=n/2 与实现逐轮对账 + 校验和, 并断言奇偶分岔计数 >1500 (证 div_ceil≠floor, 非同义重复); `graph_op_id_emission_monotonic` 证发射单调。
- **断言强度实证**: 把 `div_ceil` 换成 `(size + SLOTS_PER_ID)/SLOTS_PER_ID` (有效语义破坏) → 该批随机对账测试 **转红**; 还原后复绿 ✓。

### 面 2 · index.rs 区间算术 vs index.h

- OpIndex(u32) 字节偏移 id 模型: `id() = self.0/8/SLOTS_PER_ID as u32`; `offset() = self.0 & UNMASK_GENERATION_MASK` (UNMASK=!2, 剥 generation 位 bit1); `encode_external_id(id) = Self(id*8 + EXTERNAL_ID_FLAG)` (EXTERNAL_FLAG=1)。
- 原版 index.h **无 SegmentedIndex** → 本轮 Rust 侧亦无该结构, 属"无对应面", 非遗漏 ✓。
- 加固测试: `opindex_offset_mask_is_release_noop` (4096 个 8 对齐 offset, 断言 release 下 `offset()==raw`、`id()==offset/16`, 证 generation mask 为 release no-op); `opindex_external_id_reconcile` (id `& 0x0fff_ffff` 保 raw<2^31, 断言 `ext.id()==id/2`); `opindex_blockindex_ordering`。
- **实证**: `encode_external_id` 首版误留高位致 `multiply overflow` panic, 收窄到 28 位后复绿 (测试自身修正, 非 src 缺陷)。

### 面 3 · operations/representations 跨文件引用一致性

- Opcode 枚举 176 项、`NUMBER_OF_BLOCK_TERMINATOR_OPCODES=9`、`may_throw()` 8-op 集合逐条对照原版 `*-operations.h`; 抽 5 个 op 定义核 write-bitmap/IsChanged 语义无漂移。
- Kind 枚举 `from_repr` 往返: FloatBinop(9)/ChangeOrDeopt(13)/Simd128Shuffle(5)/ObjectIs(20)/Word32Pair/OverflowChecked 判别式 + fallback 分支核 `kind_enums_from_repr_roundtrip` ✓。
- RegisterRepresentation 判别式 Word32=0…Simd256=7; `from_machine_representation` 中 `Bit => Word32` 合并面核 representations.rs。**实证**: 破坏该 case → `representations_golden` **转红**, 还原复绿。
- `abstract_rep` name→RegisterRepresentation 交叉引用与 representations.rs 一致 (`abstract_rep_crossref_matches_representations`) ✓。
- `opcode_may_throw_set_matches_original` (8 正 + 10 负) 锁死 may_throw 集。

### 面 4 · 断言强度审计 (round1/2 新例中随机抽 5)

- 抽样 5 例逐个"破坏 impl 常量→期望转红": graph div_ceil ✓红 / representations from_machine Bit ✓红 / fast_hash combine 权重 17→16 ✓红 / sidetable `next_size` +32 ✗不红 / layered resize 阈值 `*3 > count*4` → `*4`→`*0` ✗不红。
- 3/5 经语义破坏可靠转红; **2 例不红经核为"测试设计使然", 非实现缺陷**:
  - sidetable 增长断言用 `assert!(len >= 47 / >= 182)` 容量吸收下界, 对 +32 偏置不敏感 (设计上容许吸收)。
  - `layered_hash_map_resize_preserves_depth_order` 以 `with_capacity(16)` 插入 6+10=16 键, 恰填满至容量 → resize 阈值触发点落在断言外, 故阈值破坏不改变可见结果。
- 全部破坏已闭环还原, 还原后复跑门禁并复验指纹回到锚点。

### 面 5 · skel 3 新组抽验 (避开 round1 sidetable/layered/opmasks、round2 builtin/runtime-call-descriptors/loop_unrolling)

- **snapshot_table + snapshot_table_opindex**: 基座 SnapshotTable (StartNewSnapshot/Seal/Set/Get/NewKey/RevertCurrentSnapshot/MergePredecessors 全 `todo!`), 且挂载 src **无任何消费者**引用 SnapshotTable/Key (仅 layered 注释文本命中) → 前置未落, 正确驻 skel ✓。
- **reducer_traits**: 依赖 Reducer trait / GraphEmitter / AssemblerOpInterface / EmitProjectionReducer 全链, 而 `assembler.rs` 本身即 SKEL → 类型级 reducer 栈基座未挂, 前置未落 ✓。
- **csa_effects_computation**: `run` 依赖 `crate::phase::Zone` (phase.rs 无 Zone)、OpEffects (未挂)、BuiltinsEffectsAnalyzer/effects machinery (未挂), 且 `pipelines.rs` SKEL → 无挂载消费者注册本 phase → 前置未落 ✓。
- 三组均"前置条件未落地", 非漏挂; skel 111 不变。

### 并发写告警

- 本仓工作树仍是多代理并发改动态 (heap-core/objects-core/wasm-core/codegen-core/maglev 等未提交)。本轮破坏→还原期间, `representations.rs`/`layered_hash_map.rs` 的还原 Edit 报 "0 occurrences" —— 根因为外部并发写流已先行把内容复原至锚点字节, 收官 `shasum` 逐字节命中 round2 末锚, src 自洽。复审席不 revert 并发 cosmetic 改动, 三门禁在现态全绿。

### 门禁收官 (全部实跑)

- `cargo nextest run -p v8-turboshaft` **47/47 绿** (含本轮新增 8 例)。
- `cargo clippy -p v8-turboshaft --all-targets -- -D warnings` **RC=0**。
- `cargo +nightly fmt --check -p v8-turboshaft` **RC=0**。
- 新增仅 `tests/turboshaft_reconcile.rs` (8 例, `use aok::{OK,Void}` + `#[ctor::ctor(unsafe)]` log_init, `/// v8:` 锚, 中文注释, 无 round*/tmp/zz_ 命名); src **零改动** (收官指纹 = 锚点); 全程 CARGO_TARGET_DIR=target/agent_W11TSR3 单包隔离, 未触碰其他 crate/tool/lib.js, 无 git 提交/推送。

### 三数 + must-fix

- **缺陷数 0** (graph 逐字段 / index 区间算术 / operations·representations 跨文件引用均与原版一致; 断言强度审计两处不红核为测试设计使然非实现缺陷) / **遗漏数 0** (index.h 无 SegmentedIndex 属实; skel 3 新组前置未落属实) / **新增测试数 8**。
- must-fix: **无阻塞项**。
  - [低·测试强度] sidetable 增长用容量吸收下界 `>=`、layered resize 用例恰填满容量 → 对增长/阈值破坏不敏感; 若后续引入超容量/大表压测, 再补敏感性断言, 当前判非阻塞。
- **round3 clean**: 0 缺陷 / 0 遗漏 / 门禁 (nextest 47/47、clippy RC=0、fmt RC=0) 全绿, 指纹逐字节命中 round2 末锚 → **round3 clean (连续 2/3)**。
