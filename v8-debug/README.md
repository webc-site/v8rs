[English](#en) | [中文](#zh)

---

<a name="en"></a>

#

---

<a name="zh"></a>

# v8-debug

V8 `src/debug/*` + `src/objects/{debug-objects,module,source-text-module,synthetic-module,js-source-position,call-site-info}` 族 + `src/debug/wasm/gdb-server` 的 Rust 转写 (debug 域)。

架构决策: Isolate/Handle/Factory 强绑定的语义以堆无关视图 + 闭包注入 1:1 转写; tq 布局位段以 const 表登记; .cc 方法面分为「本层可完全表达」与「深度依赖 heap, 已推迟」两档。

- [推迟台账 (Deep-Water Deferred)](#推迟台账-deep-water-deferred)
  - [source_text_module (推迟 15 项)](#source_text_module-推迟-15-项)
  - [call_site_info (推迟 10 项)](#call_site_info-推迟-10-项)
  - [debug_objects (推迟 3 项)](#debug_objects-推迟-3-项)
  - [synthetic_module (无推迟)](#synthetic_module-无推迟)
- [已完成实现摘要](#已完成实现摘要)
  - [红态修复 (10 处)](#红态修复-10-处)
  - [93 todo! 填体](#93-todo-填体)
- [门禁](#门禁)

## 推迟台账 (Deep-Water Deferred)

以下 .cc 方法/分支因深度依赖 Isolate/Factory/Zone 或 v8::Local 桥接而无法在本层忠实表达, 采用闭包注入签名或最简返回; 具体分配/执行链在 Developer 阶段完成。原函数名 / 前置条件 / 解锁 crate 三要素登记如下 (共 28 项)。

### source_text_module (推迟 15 项)
| 原函数名 | 前置条件 | 解锁 crate |
|---|---|---|
| `prepare_instantiate` | `v8::Local<Module>` + `callbacks.module_callback*` API 桥接, 本层以 `user_resolve` 闭包注入 | `v8-api` + `v8-execution-isolate` |
| `run_initialization_code` | `Execution::Call` + `NewModuleContext` 依赖 isolate 堆与 native_context | `v8-execution-isolate` + `v8-execution-core` |
| `finish_instantiate` | 需 Isolate 与 resolve_set 图遍历 (仅状态位转写) | `v8-execution-isolate` + `v8-heap-core` |
| `fetch_star_exports` | Zone + Handle<Object> 递归, 本层以 `resolve_star` 注入 | `v8-execution-isolate` + `v8-zone` + `v8-handles` |
| `gather_available_ancestors` | 依赖 Handle 递归与 zone stack | `v8-execution-isolate` + `v8-zone` + `v8-handles` |
| `get_module_namespace` | Factory 分配 namespace, 本层以 `create_namespace` 注入 | `v8-execution-isolate` + `v8-objects-core` |
| `get_import_meta` | Factory 分配 import.meta 对象 | `v8-execution-isolate` + `v8-objects-core` |
| `evaluate` | `InnerModuleEvaluation` 主循环 (Zone stack + DFS + RunInitializationCode) | `v8-execution-isolate` + `v8-execution-core` + `v8-zone` |
| `async_module_execution_fulfilled` | JSPromise 状态机 + zone stack 递归 | `v8-objects-js-async` + `v8-zone` + `v8-execution-isolate` |
| `async_module_execution_rejected` | async_parent_modules 递归传播 (仅状态位转写) | `v8-objects-js-async` + `v8-execution-isolate` |
| `execute_async_module` / `inner_execute_async_module` / `execute_module` | Execution::CallContinuation + Bytecode 执行 | `v8-interpreter` + `v8-execution-core` |
| `inner_module_evaluation` | 顶层 DFS + ZoneForwardList + Telemetry | `v8-zone` + `v8-execution-isolate` |
| `gather_asynchronous_transitive_dependencies` | Zone 图递归 (visited 集合可本地化, neighbors 注入) | `v8-zone` + `v8-execution-isolate` |
| `get_stalled_top_level_await_messages` | Handle vector + zone 消息构造 | `v8-handles` + `v8-zone` + `v8-execution-isolate` |
| `inner_get_stalled_top_level_await_module` | DFS 遍历 Isolate 图 | `v8-execution-isolate` + `v8-heap-core` |

### call_site_info (推迟 10 项)
| 原函数名 | 前置条件 | 解锁 crate |
|---|---|---|
| `construct_from_raw_data` | `Isolate::factory()->NewCallSiteInfo` 分配 (本层返回 View 而非 Handle) | `v8-execution-isolate` + `v8-objects-core` |
| `expand_deferred_frames` | `Code::GetBaselineBytecodeArray` + `GetBytecodeOffsetForBaselinePC` 需 Code 对象 | `v8-codegen-core` + `v8-objects-code` |
| `get_script_hash` | `Script::GetScriptHash` 走 hash cache + isolate 内存 | `v8-execution-isolate` + `v8-objects-core` |
| `get_eval_origin` | `IncrementalStringBuilder` + 递归 `FormatEvalOrigin` | `v8-execution-isolate` + `v8-strings` |
| `get_function_name` / `get_function_debug_name` | `JSFunction::GetDebugName` + `Builtins::NameForStackTrace` + `WasmModuleObject::GetFunctionNameOrNull` | `v8-objects-js-func` + `v8-builtins-core` + `v8-wasm-objects` |
| `get_method_name` | `InferMethodName` 走 receiver descriptor/dictionary 遍历 | `v8-objects-collections` + `v8-objects-maps` |
| `get_type_name` | receiver.constructor.name 查询 | `v8-objects-core` + `v8-execution-isolate` |
| `get_wasm_module_name` | `WasmInstanceObject->module_object->name()` | `v8-wasm-objects` + `v8-wasm-core` |
| `compute_source_position` (IsWasm 分支) | `wasm::GetSourcePosition(module, func_index, offset)` | `v8-wasm-core` + `v8-wasm-objects` |
| `get_script` (IsWasm 分支) | `WasmInstanceObject->trusted_data->module_object->script()` | `v8-wasm-objects` + `v8-objects-core` |

### debug_objects (推迟 3 项)
| 原函数名 | 前置条件 | 解锁 crate |
|---|---|---|
| `debug_info_clear_break_info` | `RedirectActiveFunctions` 需要遍历 isolate 栈 | `v8-execution-isolate` + `v8-execution-core` |
| `debug_info_has_break_point` / `debug_info_get_break_point_info` / `debug_info_set_break_point` / `debug_info_clear_break_point` | FixedArray 遍历 + Factory 分配 (本层以 `break_info_at`/`lookup` 闭包注入) | `v8-execution-isolate` + `v8-objects-core` |
| `coverage_info_coverage_info_print` | 原版 stderr 打印, 本层返回 String | `v8-base-platform` |

### synthetic_module (无推迟)
6 方法全部本层实现 (依赖 ModuleBase.exports + Cell 语义, evaluate 实现状态机)。

## 已完成实现摘要

### 红态修复 (10 处)
1. `Slot` 增补 `Copy` derive → `RequestedSlot` Copy 可派生
2. `debug_objects::DebugExecutionMode` 枚举 (Breakpoints/SideEffects, 对应 bit 5)
3. `source_text_module::ResetState` 结构体 + `source_text_module_reset` 函数
4. `error::Error` 新增 `AccessedUninitializedVariable` / `NotDefined` / `RedefineDisallowed` 三变体
5. `module.rs` 中 `lookup.is_the_hole()` → `lookup.is_none()` (Option<&ExportEntry> 上无该法)

### 93 todo! 填体
- `synthetic_module.rs`: 6/6 填体 (set_export / set_export_strict / resolve_export / prepare_instantiate / finish_instantiate / evaluate)
- `debug_objects.rs`: 21/21 填体 (DebugInfoView / BreakPointInfoView / CoverageInfoView 三视图 + 位段完整实现)
- `source_text_module.rs`: 33/33 填体 (含 `resolve_export` / `resolve_import` / `prepare_instantiate` 等核心链接语义, 深度堆依赖以闭包注入)
- `call_site_info.rs`: 33/33 填体 (含 `CallSiteInfoView` / `ScriptView` / `PositionInfo` / `MessageLocationView`, 位访问 + 位置计算 + Script 桥接)

## 门禁

```bash
CARGO_TARGET_DIR=target/agent_w7dbgr \
  cargo nextest run -p v8-debug --config 'term.quiet=false'
```
32/32 通过 (0 skipped)。
clippy: 0 警告 (-D warnings 严格通过)。

