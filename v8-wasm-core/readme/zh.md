#

## enum item 级 #[allow] 专项处置 (W11WCR round1 复审 2026-10-13)

红线基准: 「修复警告, 禁用 allow 绕过, 按 rust 最佳实践改写」。原 W10WCb 保留 4 处命名类 `#[allow(non_camel_case_types)]` + 1 处 `#[allow(dead_code)]`, 共 5 处。本席逐处判定: **3 处改写清除, 2 处登记为有据豁免 (建议主控批准)**, src 内 allow 现存 5→2。

**已清除 (改写为 rust 惯例命名, 原版名以 `/// v8:` doc 锚保留可追溯)**:

| 枚举 | 落点 | 处置 |
|---|---|---|
| TrapReason | wasm_opcodes.rs | 21 variant `UNREACHABLE/MEM_OUT_OF_BOUNDS..` → `Unreachable/MemOutOfBounds..`, 判别值 (=序号) 不变, 原版 kTrapX 名留 doc 锚; allow 删 |
| ElementType | wasm_module_consts.rs | `kFunctionIndex/kExpression` → `FunctionIndex/Expression`, 原版 kFunctionIndexElements/kExpressionElements 留 doc 锚; allow 删 |
| WasmFarJumpBuiltin | wasm_builtin_list.rs | 5 条下划线 variant → 驼峰 (去下划线); 原版 V(NAME) 名以 name() 返回的字符串值逐字保留, doc 锚 `/// v8: V(...)`; 其余 150 条本已合规; allow 删 |

**登记豁免 (建议主控批准; 二者均「有名录」, 非无名场绕过)**:

1. `WasmOpcodeSig` (wasm_opcodes.rs:1447, `#[allow(non_camel_case_types)]`) — variant 为原版 FOREACH_SIGNATURE 的签名短记号 (`s_s/i_v/s_ss..`), 下划线**编码返回/参数元数结构**, 是 opcode→签名 12.1 逐值对账的关键可读形态; 该表有 2118 处引用且 wasm_opcodes.rs 由 `tool/src/wasm/wasm-opcodes.h.js` 生成 (「复跑覆盖」), 手写去下划线既会在下次生成被覆回, 又存在 `SI_I`/`S_II` 之类去下划线歧义碰撞 (破坏 60×2118 表逐值对账)。保名不可替代。**注**: tool/ 生成器在本席禁改面 (含 tool/lib.js), 生成器侧命名合规需另议。
2. `LinkageLocationAllocator.slot_offset_` (wasm_linkage.rs:143, `#[allow(dead_code)]`) — 骨架字段; 唯一读者是原版 `Next(rep)` 的栈槽分支 (`-1-(slot_offset_+allocator_.NextStackSlot())`), 而 `next()` 及 `LinkageAllocator::{next_gp_reg,next_fp_reg,end_slot_area}` 在本 crate 均为 `unimplemented!()` 深水骨架 (前置 codegen 寄存器分配面, 解锁 v8-codegen-core)。字段保原版成员名 1:1; 加公共访问器读取属虚构 API (原版为 private), 违反转写忠实性。**注**: 深水性质同 42 todo 台账 (两文件级), 此处为单骨架字段。

**W12WCR2 round2 豁免复核订正 (2026-10-13)**: 两处豁免均**维持**, 复核实证: (1) 引用面「2118 处」以 `grep -o 'WasmOpcodeSig::' | wc -l` 精确复算属实; 生成器 `sigVariant = toUpperCase()` 逐字保留下划线, 「复跑覆回」威胁属实。(1a) 订正: 名录示例 `SI_I`/`S_II` 碰撞对**不在现有 59 variant 中** (两种驼峰化模拟均无重名), 示例仅作歧义类形描述, 不影响核心结论。(1b) 监测项: `#[allow(non_camel_case_types)]` 行**不在生成器模板内** (tool/src/wasm/wasm-opcodes.h.js 300-303 直接 emit derive 块), 生成器复跑会丢失该行致 clippy -D 红, 复跑后须就地补插 (与「生成器侧命名合规需另议」同源, 属 tool 禁改面事项)。(2) slot_offset_ 深水形态现场复验: next() 及 LinkageAllocator 群均 `unimplemented!()`, 解锁 crate v8-codegen-core 实测存在 (含 register_config.rs/interface_descriptors.rs 寄存器面)。

## 深水 todo! 台账 (W10WCb 接力波定稿 2026-10-13)

判定基准: 真实可执行 `todo!()` 调用形 (注释/doc 文面提及不计)。实测现存 42 处 (module_instantiate_cc.rs 22 / c_api_cc.rs 20), 两文件均未挂载 lib.rs 编译面 (登记态死文件, 无运行时可达路径), 不主动挂载 — 挂载即向公共面导出 42 个 panic 入口, 违背 100% 安全面纪律。

42 处全部判深水: 各落点前置均依赖 Isolate/句柄/Factory/JS 对象模型或 wasm-c-api 句柄面, 无纯语义 1:1 浅水候选 (逐条核对原版 module-instantiate.cc 与 c-api.cc 后维持该判定)。todo 文案已规范化为台账格式 `todo!("原函数名: 前置; 解锁 crate")`。

**A 组 · module_instantiate_cc.rs — InstanceBuilder 22 处 (原 v8/src/wasm/module-instantiate.{h,cc})**

| 前置组 | 落点 (22) | 前置; 解锁 |
|---|---|---|
| 终装 (5) | Build / Build_Phase1 / Build_Phase1_Infallible / Build_Phase2 / ExecuteStartFunction | Isolate/Handles/Factory 与 WasmInstanceObject 终装面; 解锁 v8-execution-isolate+v8-handles+v8-wasm-objects+v8-wasm-js |
| JS 导入 (8) | LookupImport / SanitizeImports / ProcessImportedFunction / ProcessImportedTable / ProcessImportedWasmGlobalObject / ProcessImportedGlobal / ProcessImports / ProcessImportedMemories | WebAssembly JS 导入对象校验面; 解锁 v8-wasm-js+v8-objects-js-core |
| 堆写入 (5) | LoadDataSegments / WriteGlobalValue / GetRawUntaggedGlobalPtr / InitGlobals / AllocateMemory | 堆内存/全局裸指针写入面; 解锁 v8-heap-core+v8-wasm-objects |
| 表/标签 (4) | ProcessExports / SetTableInitialValues / LoadTableSegments / InitializeTags | 实例表段/标签初始化面; 解锁 v8-wasm-objects+v8-objects-arrays |

**B 组 · c_api_cc.rs — wasm-c-api WASM_EXPORT 骨架 20 组 (原 v8/src/wasm/c-api.cc, 覆盖约 90 个导出)**

按类分组: Config/Engine/Store/ValType/ExternType/FuncType/TableType/MemoryType/ImportType/ExportType/Ref/Trap/Foreign/Module/Extern/Func/Global/Table/Memory/Instance。共性前置: wasm-c-api 句柄类型面 (枚举在 c_api.rs.skel, 未挂载) + Engine/Store/Isolate 生命周期; 主解锁 v8-execution-isolate+rs8-capi, 类侧细分见各 todo! 文案。生成残骸修复: K_MAX_COUNTERS 行原为 C++ 原文误植 (`static const unsigned kMaxCounters = 512;,512;`), 已按 c-api.cc:328 修正为 `512`。

## 155 wasm builtin variants 对账 (W10WCb 复核 2026-10-13)

wasm_builtin_list.rs 与原版 v8/src/wasm/wasm-builtin-list.h 逐一对账: 陷阱段 21 (FOREACH_WASM_TRAPREASON, src/common/globals.h:3103-3124, VTRAP(TrapX)→ThrowWasmX) + 跳表段 134 (逐字逐序 diff 零差) + 非跳表 6 — **差集 0, 重命名完整落地**。序锁定测试 tests/wasm_builtin_list.rs (对位原版 detail::InitBuiltinToFarJumpTableIndex 槽位序=展开序), 公共面新增 `WasmFarJumpBuiltin::all()` (strum::VariantArray)。
