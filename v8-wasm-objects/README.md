[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-wasm-objects

Structural ledger lives in `readme/zh.md` (Chinese): filled pure-logic surface with original
`v8/src/wasm/wasm-objects.cc` anchors, deferred registry (missing prerequisites and
unlocking crates) for all Isolate/heap-coupled skeletons, and test migration status.

A-1 sentinel: `wasm_objects_consts.rs`
`dispatch_table_layout::EXPOSED_TRUSTED_OBJECT_SIZE` is an explicit const-foldable
placeholder `0`. Hard constraint: its true `sizeof(ExposedTrustedObject)` value MUST be
substituted before the module is wired into `lib.rs` — never wire with the placeholder.
The true value is not directly provable from the original `wasm-objects.{h,tq}`
(tq:37 extern class has no body; h:973 only records the formula shape); it depends on
heap-object layout crates and pointer-compression/SANDBOX flags. The file stays outside
package gates until wired; a standalone `rustc` compile probe (currently 0 errors) is its
compile guard.


---

<a name="zh"></a>

# v8-wasm-objects 台账

映射原版: `v8/src/wasm/wasm-objects.{h,cc,inl.h}`、`code-space-access.cc` (清单 target/wave5/R5W.txt)。
本 crate 面分两层: `wasm_objects_consts.rs` (布局/名单常量) 与 `wasm_objects_cc.rs` (行为规则 + 方法签名骨架)。
`code_space_access_cc.rs` 对接 v8-common `RwxMemoryWriteScope`。

接线状态: `lib.rs` 已接 `wasm_objects_cc` + `code_space_access_cc` (轮7 W7WO 填体后余 21 处
深面 `todo!` 骨架, 以文件级 `#![allow(clippy::todo)]` 容存, 不被任何已实现面调用; 逐条见推迟 B);
`wasm_objects_consts.rs` 因布局哨兵未解 (见推迟 A-1) 暂不接线。该文件未挂 `lib.rs`、不进包门禁,
以独立 `rustc` 探针把关 (0 error 可编译)。

- [已填体 (纯逻辑面, 与 Isolate/堆无关; 全部带原版锚点)](#已填体-纯逻辑面-与-isolate堆无关-全部带原版锚点)
- [测试](#测试)
- [推迟登记](#推迟登记)
  - [A. 文件级](#a-文件级)
  - [B. wasm_objects_cc.rs 推迟骨架 (21 处, 轮7 W7WO 填体后余量; 原 76 骨架已填 55)](#b-wasm_objects_ccrs-推迟骨架-21-处-轮7-w7wo-填体后余量-原-76-骨架已填-55)
  - [C. 依赖面备注](#c-依赖面备注)
  - [D. 推迟测试登记 (原版用例 → 解锁条件)](#d-推迟测试登记-原版用例-解锁条件)

## 已填体 (纯逻辑面, 与 Isolate/堆无关; 全部带原版锚点)

| Rust 符号 | 原版锚点 | 说明 |
| --- | --- | --- |
| `min_growth_pages` / `new_capacity_pages` | wasm-objects.cc:1171-1175 | 拷贝增长步进与容量钳位 (骨架工具预提取) |
| `memoryobject_new_reserved_pages` | .cc:847-885 | WasmMemoryObject::New 预留页数启发式 (64 位分支实装; 32 位分支以 `cfg!(target_pointer_width="32")` 保留, 本机构译不到) |
| `memoryobject_max_size` | .cc:1078-1083 | Grow 可增长页数上限 (GetMaxSize 面) |
| `memoryobject_grow_exceeds` | .cc:1084-1085 | 页数增量越限判定 |
| `memoryobject_must_grow_in_place` | .cc:1087-1089 | 原地增长四条件析取 |
| `tableobject_grow_max_size` | .cc:301-304 | 表增长上限钳位 |
| `tableobject_grow_exceeds` | .cc:305-306 | 表增长增量越限判定 |
| `tableobject_entries_grow_delta` | .cc:312-318 | entries 指数扩容槽数 |
| `dispatchtable_grow_amount` | .cc:2588-2604 | 调度表 Grow 容量决策 |
| `dispatchtable_uses_min_free_entries` | .cc:2533-2536 | uses 列表压实阈值 |
| `dispatchtable_uses_new_capacity` | .cc:2540-2546 | uses 列表扩容新容量 |
| `exceptionpackage_getencodedsize` | .cc:2813-2844 | 异常包编码元素数 (参数 kind 表) |
| `encode_i32/i64_exception_value` | .cc:2738-2750 | 异常值 16 位槽编码 |
| `decode_i32/i64_exception_value` | .cc:2752-2764 | 异常值 16 位槽解码 |
| `CodeSpaceWriteScope::new/default` | code-space-access.cc:11-12 | 转发 v8-common RwxMemoryWriteScope |

轮7 W7WO 填体 (抽取入参/纯模型, 共 55 处骨架 → 已填; 各条原版锚点即 doc 注释行, 分类见推迟 B 引言):
Module 名字/字节切片面、Table New/Grow/Set/Get/Fill/UpdateDispatchTable/占位符规划器、Memory
Grow 全状态机规划器 (`.cc:1053-1207` 13 参)/SetNewBuffer/Refresh/UpdateInstances/FixupResizable、
TrustedInstanceData SetRawMemory/InitDataSegmentArrays/CopyTableEntries/InitTableEntries/
GetCallTarget/GetGlobalStorage/GetGlobalValue/TryGetFuncRef、Internal/Exported/GC 谓词与字段读写、
Capi 匹配判定、调度表群 (DispatchTableModel/WasmWrapperMap/uses Vec 模型: entry_set/entry_clear
核心 + set/clear/adduse/maybegrowuseslist/grow + forimports_* + dispatchtabledata_add/remove/
wrappercodepointerfordebugging)、exceptionpackage_getencodedsize_for_tag、
externalfunction_iswasmexternalfunction。

工具链登记: 本版 nightly(1.101) const 上下文不支持 `assert_eq!`/整型 `Ord::min/max`/
`From<u32>`, 不变式块统一 `assert!(==)` 折叠, 涉函数用例移至 `tests/wasm_objects.rs`;
`slice::contains` 非 const fn (E0015), const 块包含判定改块内 `const fn` while 逐字节
比较保持编译期折叠 (不降级运行时断言)。

## 测试

- 原版 `test/unittests/wasm/` 无 wasm-objects 专测; `test/cctest/wasm/` 的
  `test-grow-memory.cc`、`test-run-wasm-exceptions.cc`、`test-backing-store.cc` 全部依赖
  WasmRunner/Isolate → 推迟 (见推迟 D)。
- 已复刻: 上表纯逻辑面用例 + 轮7 W7WO 调度表/异常/外部函数纯模型面用例, 断言逐条取自原版
  函数内 static_assert/CHECK/DCHECK 语义与注释数值算例 (`.cc:1171-1175` 0.5MB+12.5% 算例、
  `.cc:858` kGBPages=16384、`.cc:2543-2546` uses 扩容步算例、`.cc:2288` SBXCHECK_BOUNDS 越界
  fail-closed、`.cc:2427-2447` 清空哨兵等), 见 `tests/wasm_objects.rs` (22 例, 全绿)。

## 推迟登记

前置码: P1=Isolate/Factory/DirectHandle/ReadOnlyRoots (解锁: v8-execution-isolate + v8-handles + v8-roots); P2=Tagged 字段读写与写屏障 (解锁: v8-heap-base + v8-objects-core); P3=JSArrayBuffer/BackingStore 含 guard 区/共享增长/GlobalBackingStoreRegistry (解锁: v8-objects 堆对象面 + v8-wasm-core 引擎面); P4=WasmCode/代码指针/wrapper 缓存 (解锁: v8-wasm-core code-manager 面 + wrapper cache); P5=类型规范化 canonical-types/CanonicalSig (解锁: v8-wasm-core canonical_types, 现 .skel); P6=反馈向量/DRUMBRAKE 解释器 (解锁: v8-ic + v8-interpreter); P7=Map/Code kind/instance 判定与 SharedFunctionInfo (解锁: v8-objects-maps + v8-objects-js-func); P8=SBXCHECK 沙箱面 (解锁: v8-sandbox)。

### A. 文件级

| # | 位置 | 缺前置 | 解锁条件 |
| --- | --- | --- | --- |
| A-1 | wasm_objects_consts.rs:62 `dispatch_table_layout::EXPOSED_TRUSTED_OBJECT_SIZE` = 显式占位 `0` (原 `todo!()` 在 const 上下文 E0080, R5W2 F14c 改可折形哨兵)。**硬约束 (升级登记, 原仅「哨兵未解」): 接线前必替换 sizeof(ExposedTrustedObject) 真值, 未替值不得挂 lib.rs**; 真值不可从原版 wasm-objects.{h,tq} 径证 (tq:37 extern class 无类体, h:973 仅公式形), 依赖堆布局件+压缩/SANDBOX 开关, 不猜 | sizeof(ExposedTrustedObject) 需堆对象布局件 (trailing torque kHeaderSize); 连带 LENGTH/CAPACITY/…/ENTRY_SIZE 公式与 `K_MIN_REF/NUM_OFFSET` 表达式串 | v8-heap-base/v8-objects-core 布局常量就位后代入, 届时整模块接 lib.rs 并把 cc 侧临时 `K_NO_MAXIMUM` 收回 consts `MEMORY_NO_MAXIMUM` 单一来源; 接线时同步补 consts 断言面 PROTECTED_FIELDS_COUNT_DRUMBRAKE==9 双态 (现仅 13/14+8, r2 观察3) |
| A-2 | wasm-objects-inl.h 访问器透传面 (口径注 r2-F8: 21 系 wasm-objects.h 侧 DECL 拼合——DECL_ACCESSORS 13 为 grep 原文行口径含 h:74 #define 体自引用, 严格调用点 12 对应总数 20; -inl.h 本体四宏顶层直调 0 行, 实际直调 = WTI_*/PRIMITIVE 本地宏 29 行 + EXTERNAL_POINTER_ACCESSORS 标准宏 1 行 (-inl.h:1487) = 30, 如 WasmModuleObject::script/set_script -inl.h:76-84、WasmInstanceObject::trusted_data/native_context -inl.h:459-516) | 未生成骨架函数; 每个访问器都是 Tagged 读写 + 写屏障 | P2; 骨架工具复跑或手工补录时逐宏登记 |

### B. wasm_objects_cc.rs 推迟骨架 (21 处, 轮7 W7WO 填体后余量; 原 76 骨架已填 55)

轮7 W7WO 填体口径: 以抽取入参/纯模型 (调度表条目 `Vec<DispatchEntry>` 模型、uses 槽位
`Vec<Option<UseEntry>>` 模型、页数/偏移算术、决策规划器) 1:1 复刻原版控制流; 越界防御统一
fail-closed 返回 None (禁 panic 纪律替代原版 CHECK/SBXCHECK abort)。仅工厂分配/堆写/代码指针/
isolate 取属性面保留 todo!。下表为**尚未填体**的 21 处 (单条锚点即 doc 注释行):

| 组 | 推迟符号 (原版 .cc 行) | 缺前置 | 解锁 crate |
| --- | --- | --- | --- |
| WasmModuleObject | moduleobject_new (:120) | P1 P2 P5 | v8-execution-isolate + 字符串工厂 + canonical_types |
| WasmTableObject | jstowasmelement (:373) | P1 P2 P4 | v8-objects-core + wrapper cache |
| WasmSuspendingObject | suspendingobject_new (:730) | P1 | v8-execution-isolate (promise/continuation 对象) |
| WasmMemoryObject | new_with_backing_store (:795)、new_from_pages (:844, 启发式已填) | P1 P3 | v8-objects 堆对象面 + v8-wasm-core 引擎面 (BackingStore) |
| WasmGlobalObject | globalobject_new (:1258) | P1 + A-1 | v8-execution-isolate + FixedArray/ByteArray 偏移 |
| WasmTrustedInstanceData | new (:1472)、getorcreatefuncref (:1771) | P1 P2 P4 | v8-execution-isolate + 堆字段 + code pointer/funcref |
| WasmInternalFunction / WasmExportedFunction | getorcreateexternal (:1839)、getwrapper (:1867)、new (:2906)、new_full (:2920) | P1 P4 P7 | v8-wasm-core code-manager + wrapper cache + maps/js-func |
| GC 对象 (CustomMap/Tag) | custommap_createjswrapper (:2147)、tagobject_new (:2218) | P2 P7 | v8-objects-core + maps (JS wrapper 工厂) |
| WasmCapiFunction | sig (:2243)、new (:2880) | P1 P5 P7 | canonical_types + maps/js-func + ExternalReference 工厂 |
| WasmExceptionPackage / WasmExceptionTag | new_with_size (:2694)、new (:2700)、getexceptiontag (:2713)、getexceptionvalues (:2725)、exceptiontag_new (:3129) | P1 (取 tag/values 另需 symbol 属性) | v8-execution-isolate (NewFixedArray/NewJSObject/GetProperty 工厂) |

> 已填体但仅解析纯逻辑面的调度表群 (WasmDispatchTableData add/remove/
> wrappercodepointerfordebugging、WasmDispatchTable/ForImports setfornonwrapper/
> setforwrapper/clear/adduse/maybegrowuseslist/grow、exceptionpackage_getencodedsize_for_tag、
> externalfunction_iswasmexternalfunction) 以 `DispatchTableModel`/`WasmWrapperMap`/
> `Vec<Option<UseEntry>>` 纯模型复刻: SBXCHECK_BOUNDS 越界 fail-closed 返回 None, 堆写屏障/
> 代码指针落盘/offheap wrapper 实际登记/isolate 工厂分配仍为深面 (WasmDispatchTable::Clear
> 的 DRUMBRAKE 间接调用缓存清理推迟 P6; Grow 的 offheap move 与用户列表更新推迟 P1 P2)。

### C. 依赖面备注

- `memoryobject_new_reserved_pages`/`dispatchtable_grow_amount` 等以调用方代入
  `engine_maximum/limit` 参数隔离了 `wasm::max_mem32/64_pages()、max_table_size()`
  (依赖 v8_flags, v8-wasm-core wasm_limits 已登记推迟) → 引擎侧解锁后由调用方补齐。

### D. 推迟测试登记 (原版用例 → 解锁条件)

| 原版文件 | 内容 | 缺前置 | 解锁 crate |
| --- | --- | --- | --- |
| test/cctest/wasm/test-grow-memory.cc | 内存 Grow 全状态机 (原地/拷贝/共享广播/上限) | P1 P3 | v8-execution-isolate + 堆对象面 |
| test/cctest/wasm/test-run-wasm-exceptions.cc | 异常包编码/抛捕链 | P1 P5 | v8-execution-isolate + canonical_types |
| test/cctest/wasm/test-backing-store.cc | guard 区/共享内存 backing store | P3 | 堆对象面 + v8-wasm-core 引擎面 |
| test/unittests/wasm/memory-protection-unittest.cc | CodeSpaceWriteScope 交互 | 依赖 Isolate 配置夹具 | v8-execution-isolate (本 crate 已以作用域回环冒烟测代替) |

