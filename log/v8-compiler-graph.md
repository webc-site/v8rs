# v8-compiler-graph 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### 台账锚定结论: `[compiler-graph] 缺新算法: NativeContext::BuildGraph`

登记名在 C++ 源码中无字面单元 (全 `v8/src` grep: 无 `NativeContext` 类挂 `BuildGraph`)。三候选比对:

- `js-native-context-specialization.cc` — 类名含 NativeContext 但无 BuildGraph 成员 (Reduce → Build* 特化族, cc:87 起), 属 v8-compiler-passes (todo.md 看板); 排除
- `turboshaft/build-graph-phase.cc:18` Run → `turboshaft::BuildGraph` (graph-builder.cc:2944) — 有 BuildGraph 名但全文件无 NativeContext 面, 属 v8-turboshaft; 排除
- `bytecode-graph-builder.cc:4885` `BuildGraphFromBytecode` (本 checkout 旧名 `BytecodeGraphBuilder::BuildGraph` 已改 `CreateGraph`, cc:1183) — 唯一同时具备 BuildGraph 名与 NativeContext 面的建图入口: 入口直传 `broker->target_native_context()` (cc:4894), builder 持 `NativeContextRef native_context_` (cc:473), 建图期建立/消费 native context 节点 (`CreateNativeContextNode` cc:1158 / `BuildLoadNativeContextField` cc:1163, `native_context_node_` 全链 6 处); 且依赖面正是字节码分析 (`bytecode_analysis_` 字段 cc:483 / 构造初始化 cc:1090)。**裁定为该单元的转述锚**

该单元主体硬耦合 JSHeapBroker/JSGraph/BytecodeAnalysis/CommonOperatorBuilder (推迟登记见 readme/zh.md「推迟登记」节), 本 crate 仅落地纯图构建子集 (`src/build_graph.rs`)。

> en.md 对应节 (Ledger anchoring verdict) 与上同源, 无 zh 之外的独有信息, 未另迁。
