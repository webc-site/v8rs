# v8-api 开发日志

> 2026-10-12: 自 todo.md 看板行与 Wave 9 派发收敛 (仅中文)。

## 门禁实测

- 单元测试: `CARGO_TARGET_DIR=target/agent_w9api cargo nextest run -p v8-api` (4/4 全部通过)
- Clippy: `CARGO_TARGET_DIR=target/agent_w9api cargo clippy -p v8-api --all-targets -- -D warnings` (0 警告)
- 格式化: `cargo +nightly fmt --check -p v8-api` (净)

## 模块与实现面

1. **api**: V8 顶层 API 转换表与句柄解引（`TO_LOCAL_LIST`、`TO_LOCAL_NAMES`、`OPEN_HANDLE_LIST`），对齐 C++ 句柄解包与 Local 句柄包装逻辑。
2. **api_arguments**: 函数回调参数与属性回调参数内存布局（`FCA_*` 与 `PCA_*` 槽位索引），确保参数数组指针与 V8 C++ 内部调用约定 ABI 逐槽对齐。
3. **api_natives**: 静态根（Static Roots）地址表与内置原型枚举（`FastIterateResult`、`MapAsArrayKind`、`SetAsArrayKind`）。
