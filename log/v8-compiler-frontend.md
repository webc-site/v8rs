# v8-compiler-frontend 开发日志

> 2026-10-12: 自 todo.md 看板行与 Wave 9 派发收敛 (仅中文)。

## 门禁实测

- 单元测试: `cargo nextest run -p v8-compiler-frontend` (5/5 全部通过)
- Clippy: `cargo clippy -p v8-compiler-frontend --all-targets -- -D warnings` (0 警告)
- 格式化: `cargo +nightly fmt --check -p v8-compiler-frontend` (净)

## 模块与实现面

1. **fast_api_calls**: 可优化 C 签名 API 调用降为直接 C 调用子图（`CTypeInfo`、`CFunctionInfo`、`FastApiCallFunction`、`OverloadsResolutionResult`、`build_fast_api_call`），支持 CTypeInfoType 映射与参数约束判定。
2. **feedback_source**: 反馈来源值类型（`FeedbackSource`、`EmbeddedFeedbackSource`），提供槽位查询、有效性验证与哈希计算。
3. **graph_assembler**: 在 Sea-of-Nodes 图上以 effect/control 传递方式组装子图（`GraphAssembler`、`GraphAssemblerLabel`、`GraphAssemblerLabelType`），支持基本块合并状态机与循环层级追踪。
4. **js_graph**: TFGraph 的 JS 门面结构（`JSGraph`），承接图容器与公共算子构建器。
5. **js_heap_broker**: 堆代理前端访问薄镜像（`BrokerFeedbackAccess`），支持反馈查询与快照提取。
6. **js_heap_broker_inl**: 堆代理内联辅助（`find_root_index`、`RecursiveMutexGuardIfNeeded` 等守卫类型）。
7. **js_operator**: JS 算子参数与枚举（`ForInMode`、`CallFrequency`）。
8. **reducer**: 前端削减器返回值类型（`Reduction`）。
