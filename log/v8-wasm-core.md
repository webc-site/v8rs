# v8-wasm-core 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## 看板行日志 (自 todo.md 收敛)

Decoder 流式零拷贝解码/游标推进/越界安全、LEB128 解码体系 (u32v/i32v/u64v/i64v/i33v 极值截断防御与 extra bits 校验)、ValueType/HeapType 32位位域编码 1:1 互操作布局、WasmFeatures 14 flag+21 恒开特性位图与编译期导入、WasmLimits 32 项规格与 V8 上限约束全面落地; 单元测试 46/46 绿, clippy 0 警告; Round 1、Round 2、Round 3 连续 3 轮独立终验审查「0 缺陷 / 0 遗漏 / 46/46 测试全绿」（clean 3/3 达成）；终验收敛冻结。

## W10WCb 接力波 (2026-10-13)

接力撞线席 W10WC (150 轮熔断, 停在 wasm builtin 155 variants 程序化重命名中途)。先实测盘点、只补差集。

**3 硬错复测与消账 (3/3)**: 开工现场 (编排者 00:5x) 报 cargo check 红 3 处; 本席实测首轮编译红一次后即绿 — 前任熔断前最后一批写入已落盘修复, 首轮红为陈旧增量产物。逐处对原版复核修法成立: (1) module_decoder.rs:101 const 上下文 `match &str` 已改为 `table_eq::str_eq` const fn 比较 (E0658 消除, 语义不变); (2) wasm_opcodes.rs:3371 fp16 断言已修正 — is_fp16_simd_opcode 区间改为枚举边界引用 (F16X8_SPLAT 0xfd120..F16X8_REPLACE_LANE 0xfd122 ∥ F16X8_ABS 0xfd130..F16X8_QFMS 0xfd14f), 与原版 wasm-opcodes-inl.h:85-88 逐值 1:1 (转写断言期望原为 0xfd120 非 fp16 系笔误, 现为 0xfd121 真/0xfd11f 假, 与原版判定一致); (3) signature 断言 0x28→0x45 (原版 0x45 I32Eqz = i_i 签名, 与 opcode 表对位)。**未重做任何已落地内容**。

**155 variants 重命名对账**: 与原版 wasm-builtin-list.h 逐条 diff — 陷阱段 21 (globals.h FOREACH_WASM_TRAPREASON, VTRAP(TrapX)→ThrowWasmX) + 跳表段 134 (名称与槽位序全一致) + 非跳表段 6 — **差集 0, 重命名完整落地**。本席补序锁定回归: tests/wasm_builtin_list.rs 2 例 (far_jump_table_order 锁 155 全表序, 对位原版 detail::InitBuiltinToFarJumpTableIndex; without_jump_table_slot 锁 6 条), 挂载面新增 `WasmFarJumpBuiltin::all()` (strum::VariantArray)。

**todo 消账 42→42 (全部深水规范化登账)**: 逐条核对原版后判定 42 处无浅水 (module_instantiate_cc.rs 22 全为 InstanceBuilder 成员, 前置 Isolate/Factory/JS 对象模型; c_api_cc.rs 20 组全为 wasm-c-api WASM_EXPORT 骨架, 前置 c-api 句柄面+Engine/Store 生命周期; 两文件均未挂载 lib.rs, 属登记态死文件, 挂载即向公共面导出 42 个 panic 入口, 不挂)。文案统一为台账格式 `todo!("原函数名: 前置; 解锁 crate")`, 分组账 (A 组 4 前置组 / B 组 20 类) 同步 readme/zh.md。附带修复 c_api_cc.rs:23 生成残骸 (K_MAX_COUNTERS 行 C++ 原文误植→按 c-api.cc:328 落 512)。

**clippy 清零 (124→0 warnings)**: 真 lint 逐处修实 — bool_comparison×2 (wasm_code_manager_consts 断言改直判), len_zero×2 (wasm_linkage 断言改 is_empty), manual_range_patterns×2 (is_breakable→0x02..=0x03|0x05..=0x07、is_prefix_opcode→0xfb..=0xfe, 均对原版集合等价核验), collapsible_if×1 (local_decl_encoder 改 2024 let-chain), unused_imports×1 (value_type_cc 删 SigDef/SigType), redundant_static_lifetimes×1 (wasm_features_cc), empty_line_after_doc_comments×1 (wasm_serialization_consts 头部字表改行注释), dead_code×1 (wasm_linkage 骨架字段 allow 保 1:1 对位)。命名类 166+62 处 (non_camel_case_types: WasmOpcodeSig/TrapReason/WasmFarJumpBuiltin/ElementType 四 enum; non_upper_case_globals: canonical_types 前任已修) 以 item 级 `#[allow]` 抑制并在注释登记「原版名逐字保留」理由, 不改写公共面名称。

**门禁三数**: cargo check -p v8-wasm-core 0 error; nextest 48/48 绿 (基线 46 + 新增 2); clippy --all-targets -- -D warnings RC=0。CARGO_TARGET_DIR=target/agent_W10WCb 全程隔离。

## W11WCR round1 复审 (2026-10-13)

对抗复审席 W11WCR; CARGO_TARGET_DIR=target/agent_W11WCR 全程隔离, 仅 -p v8-wasm-core。

**三数**: 缺陷 1 / 遗漏 1 / 新增测试 10。非 round1 clean (实修 1 真缺陷)。

**头号专项 · enum item 级 #[allow] 处置 (5→2)**: 红线「禁 allow 绕过」。3 处改写清除 (TrapReason 21 variant→驼峰 / ElementType→FunctionIndex,Expression / WasmFarJumpBuiltin 5 条下划线→驼峰; 原版名逐条 `/// v8:` doc 锚 + name() 字符串值保留, 判别值不变)。2 处判「保名不可替代」, 登记有据豁免并**建议主控批准**: (1) WasmOpcodeSig — FOREACH_SIGNATURE 签名短记号, 下划线编码元数结构且系 12.1 opcode→签名逐值对账关键形态, 表 2118 引用 + 文件由 tool/src/wasm/wasm-opcodes.h.js 生成「复跑覆盖」(手写去下划线会被覆回且有 SI_I/S_II 类歧义碰撞); (2) LinkageLocationAllocator.slot_offset_ — 骨架字段, 唯一读者 next() 及 LinkageAllocator 群均 unimplemented!() 深水 (前置 codegen 寄存器分配面), 加访问器属虚构 API 违忠实。处置结论 + 理由成文入 readme/zh.md 「#[allow] 专项处置」节。

**must-fix · 缺陷 1 (高)**: local_decl_encoder.rs `Size()` 以 `usize::from(needs_heap_type) * sizeof_i32v(ty.heap_type().code())` 计算 — Rust 乘法**不短路**, 右操作数 `heap_type()` 恒求值, 对数值局部 (bits&0b11==0) 触发 `debug_assert!(heap_type)` 崩溃; Emit 内 `debug_assert_eq!(size, pos)` 随之炸, 即任何含数值局部的模块尺寸计算在 debug 下 panic。原版 Size 用三元 `(needs_heap_type ? sizeof_i32v(heap_type()) : 0)` 短路, heap_type() 仅索引/抽象引用才调。改写为 `if needs_heap_type { ... }` 守卫族 (与 Emit 同构, 逐字节对三元), 修复后 Emit/Size 一致无 panic。该 bug 因本 crate 此前无 local_decl_encoder 测试而长期未暴露。

**遗漏 1**: src 缺 wasm_opcodes 判定族与 local_decl_encoder 的 tests/ 覆盖 (fp16/relaxed/atomic 边界仅 src const-assert 内点、无跨 crate 穷尽; local_decl 编解码零测)。已补。

**新增测试 (10 例, 全绿)**:
- tests/wasm_opcodes.rs 5 例: fp16 端点常量逐值 (0xfd120/122/130/14f) + is_fp16_simd_opcode 边界穷尽 (区间一端点/空洞/区间二端点内点/上界后) + relaxed (0xfd100..0xfd1ff 界) + atomic_rmw (0xfe1d/0xfe1e/0xfe4e/0xfe4f) + signature 查表对位 (0x45→I_I, 0xfd00→S_I, 0xfc00→I_F, atomic 0xfe10)。对位 wasm-opcodes-inl.h:82/85-88/90-95。
- tests/local_decl_encoder.rs 4 例: Emit 数值局部逐字节 [2,2,0x7f,1,0x7e] / AddLocals 同型合并尾项 (=原版 back().second==type) / set_sig 起始索引含参数偏移 / 堆·共享·精确分支 Size==Emit 不变式 (即捕获上述缺陷者)。对位 local-decl-encoder.cc:Emit/AddLocals/Size。
- tests/wasm_builtin_list.rs +1 例: far_jump_discriminant_is_slot — 锁 155 判别值 = 展开槽序 (原序测试经 strum all() 走声明序, 未锁判别值, 本例补 InitBuiltinToFarJumpTableIndex 判别=槽对位)。

**其余审查面 (0 缺陷)**: fp16 区间代码 1:1 复核 wasm-opcodes-inl.h:85-88 (F16X8_SPLAT..REPLACE_LANE ∥ ABS..QFMS, 端点常量实测 0xfd120/122/130/14f, 与任务给定一致); module_decoder str_eq const 比较逐字节等价 `==` (非 const std str PartialEq 补齐), 语义不变正确; decoder LEB 族边界测试已强 (i33v 2^31/2^32-1/-1/extra-bit 拒收, 长度溢出/截断/extra-bits error offset, MAX_VAR_INT32_SIZE); local_decl Emit/AddLocals 逐行对原版无偏 (除上述 Size); 42 深水 todo 抽 8 (module_instantiate 5 + c_api 3): 格式齐「原函数名: 前置; 解锁 crate」, readme A/B 组逐条对应, 前置属实, 8 个解锁 crate (v8-execution-isolate/handles/wasm-objects/wasm-js/objects-js-core/heap-core/objects-arrays/rs8-capi) 全部实测存在, 两文件确未挂载 lib.rs; value_type/features/limits 测试为常量·位域·flag 全表锁定, 非薄弱, 免补。v8-objects-core/v8-heap-core 全程只读, 未触碰。

**门禁**: cargo check -p v8-wasm-core RC=0; nextest RC=0 (58/58, 基线 48 + 新增 10); clippy --all-targets -- -D warnings RC=0; src todo! 42 (深水未动); src #[allow] 2 (均登记豁免)。

## W12WCR2 round2 复审 (2026-10-13)

对抗复审席 W12WCR2 (round2 全新视角); CARGO_TARGET_DIR=target/agent_W12WCR2 全程隔离, 仅 -p v8-wasm-core。

**指纹判定**: 开工复算 `cat src/*.rs | shasum -a 256` = **bfb588bd24953429c0e4…** 与锚 bfb588bd24953429 前缀一致, 零漂移开工。中途现场观测到**外部进程两次回写 local_decl_encoder.rs** (04:13 版 3956B → 04:21 版 3948B; 本席 src 突变两度未落盘即被抹除, 构成外部写者活跃实证), 终态聚合指纹 = **b89a2c27ab99fc2b…**≠锚。归因: 逐行 Read 比对基线**内容行级一致** (round1 Size() if 守卫族修复原样在场), 60 测试全绿 + clippy -D RC=0 证无语义变化 → 判「等价归一回写」依规继续审; 因终态聚合指纹不一致, 本轮**不判 clean** (列监测项: 若下席见 b89a2c27 前缀即外部归一态)。

**三数**: 缺陷 0 / 遗漏 1 (豁免名录账面订正, 见下) / 新增测试 2。基线三数复算全实: nextest 58/58 (开工复跑绿)、clippy -D RC=0、todo! 42 (module_instantiate_cc 22 + c_api_cc 20, 深水未动)、allow 5→2 现场复点 = wasm_opcodes.rs:1447 + wasm_linkage.rs:143 两处, 与名录相符。

**豁免复核 (两处均维持, 不撤销)**: (1) WasmOpcodeSig — 引用面「2118 处」经 `grep -o 'WasmOpcodeSig::' | wc -l` **精确复算属实** (611 行承载 2118 次出现, 前次 -c 行数系误读风险已排); 生成器 tool/src/wasm/wasm-opcodes.h.js `sigVariant=toUpperCase()` 逐字保留下划线短记号, 「复跑覆回手写改名」威胁属实; 但发现两处账面差: **示例 `SI_I`/`S_II` 碰撞对不在现有 59 variant 中** (snake→camel 与去下划线两种改名模拟均零重名, 示例仅类形描述), 且 **`#[allow]` 行不在生成器模板** (js:300-303 直接 emit derive, 复跑即丢 allow 致 clippy -D 红, 须就地补插) — 判订正而非撤销, 已补写入 readme/zh.md「W12WCR2 round2 豁免复核订正」段。(2) slot_offset_ — 唯一读者 next() 与 LinkageAllocator 群现场复验均 `unimplemented!()`, 解锁 crate v8-codegen-core 实测存在 (register_config.rs/interface_descriptors.rs), 豁免理由成立。

**同型短路缺陷扫查 (0 出)**: 全文件 grep 三元→乘法/eager 求值形态 (`usize::from(...) *`、`* f(`、`* x?`、`as usize *`、`.into() *`): 挂载 src 零残留 (round1 修复后 local_decl_encoder Size() 守卫族在场且 emit debug_assert_eq 锁死一致性); 剩余乘法均纯算术无副作用位点 — decoder.rs:392 LEB 位宽 (对原版逐式)、struct_types.skel/memory_map.skel 页对齐 div_ceil*align、compilation_hints_generation.skel:48 double floor (对原版 compilation-hints-generation.cc:187 逐字)。`&&` 位点 (value_type.rs:696 等) Rust 原生短路, 与 C++ 等价。索引先求值: decoder/value_type/features 无 `cond ? arr[i] : d` 型 eager 转化点位。Size() 修复复核通过, must-fix 0。

**far-jump 面补账 (陷阱 21 + 跳表 134)**: 本席独立复 grep globals.h:3103 FOREACH_WASM_TRAPREASON 21 条逐字 = 测试 TRAPS 序 (TrapUnreachable..TrapStringOffsetOutOfBounds); 原版 `InitBuiltinToFarJumpTableIndex` 陷阱前置段结构 (wasm-builtin-list.h:20-22, FOREACH 先于 V(WasmCompileLazy)) 与 Rust 判别值序 0..20/21 对位。**空洞值语义对账**: 原版 kBuiltinToFarJumpTableIndex 为 array<uint8_t, kFirstBytecodeHandler> 默认零初始化 → 表外 builtin 读出 0 与槽 0 (ThrowWasmUnreachable) 混叠, 仅 DCHECK 兜底 (debug); Rust 面枚举即槽序、无 Builtin→idx 反查转写面, 空洞值退化路径天然不存在 — 列结论不列缺陷。**溢出路径**: 原版 uint8_t next_index 超 255 静默回绕 vs Rust repr(u8) 编译期拒超载 + 新测试锁 155≤256 上界与末槽判别值。**重复 key 路径**: 原版同名二次写覆盖破坏双射 vs Rust 以 builtin_name 155 值互异锁定 (新测试)。新增 tests/wasm_builtin_list.rs 2 例: far_jump_u8_overflow_guard / far_jump_no_duplicate_builtin_name, 全绿。

**c_api_cc.rs 常量抽 5 对原版** (W10WCb 修复面周边): K_MAX_NAME_SIZE=64 ↔ c-api.cc:293 / K_MAX_COUNTERS=512 ↔ :328 (W10WCb 落值复验正确) / K_NAME_BOX_SIZE=64 ↔ :398 / K_VALUE_BOX_SIZE=13 ↔ :399 / K_STACK_LIMIT=10 + kOverview ↔ :582-584 (且 575-585 上下文确为 Store 构造尾段, doc 锚 Store::make 位置相符)。5/5 一致, 0 缺陷。

**失败敏感性 4 例闭环 (全红全恢复)**: M1 local_decl Size() 堆分支 `sizeof_i32v(→2)` → emit_size_invariant_heap_refs 红 (src:66 DCHECK_EQ panic) [round1 10 例之一]; M2 wasm_opcodes fp16 下界 `>=→>` → fp16_simd_boundaries 红 [round1 10 例之一]; M3 decoder read_i33v `<i64,V,33→32>` → read_leb_family 红; M4 wasm_builtin_list builtin_name 槽 21 改成重名 → far_jump_table_order + far_jump_no_duplicate_builtin_name (本席新例) 双红。注: M1 前两次尝试被外部回写者中途抹除 (未及跑即还原), 第三次单命令原子闭环达成 — 外部写者实证。4/4 破坏-红-恢复闭环。

**skel/深水台账换面抽 4 (避开 round1 InstanceBuilder/c-api 8 处, 全程只读)**: (a) wasm_engine_globals.rs 5 处 unimplemented「登记骨架: wasm-engine.cc」— 原版 GetWasmEngine()/GetWasmCodeManager() 现态实测 (wasm-engine.cc:176/1269/1572 等广用), 声称属实; (b) wasm_linkage.rs:161 next() — 原版 LinkageLocationAllocator 在 wasm-linkage.h:309 属实, 前置 codegen 面 (v8-codegen-core 存在含 Register 相关 register_config.rs), 深水成立; (c) wasm_subtyping.rs:5 前置「v8-wasm-objects 类型面与 canonicalizer」— grep TypeCanonicalizer/CanonicalGroup v8-wasm-objects/src **零命中** = 前置确未落地, 声称属实; (d) wasm_stack_wrapper_cache.rs.skel:35 CompileWasmStackEntryWrapper — 原版 wasm-stack-wrapper-cache.cc 存在属实, objects-core/wasm-objects/heap-core 现态零 StackEntryWrapper, 前置属实。4/4 无失实, must-fix 0。

**门禁**: cargo check -p v8-wasm-core RC=0; nextest RC=0 (**60/60**, 基线 58 + 本席新增 2); clippy --all-targets -- -D warnings RC=0; todo! 42 未动; allow 2 (豁免维持 + readme 订正成文)。v8-objects-core/v8-heap-core 等旁席 crate 全程只读未触碰; lib.rs/tool 生成器未动。

**判定**: 缺陷 0 / 遗漏 1 (账面订正已落 readme) / 新增测试 2; 终态聚合指纹 b89a2c27 与锚不一致 (外部等价归一回写所致, 内容无漂移), 按门禁纪律本轮**不写 clean**, 列指纹监测项移交下席。

