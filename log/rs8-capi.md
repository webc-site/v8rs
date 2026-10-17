# rs8-capi 开发日志

> 2026-10-12: 自 todo.md 看板行与 L7 收敛 (仅中文)。

## 门禁实测

- 单元测试: `CARGO_TARGET_DIR=target/agent_rs8capi cargo nextest run -p rs8-capi` (5/5 全部通过)
- Clippy: `CARGO_TARGET_DIR=target/agent_rs8capi cargo clippy -p rs8-capi --all-targets -- -D warnings` (0 警告)
- 格式化: `cargo +nightly fmt --check -p rs8-capi` (净)

## 模块与实现面

1. **handles**: C API 句柄类型定义（`V8Isolate`、`V8Context`、`V8Value`、`V8Local`、`V8Global`、`V8HandleScope`）与判空/重置生命周期。
2. **initialization**: 全局引擎初始化与退出（`v8_initialize`、`v8_is_initialized`、`v8_dispose`、`v8_get_version`、`v8_set_flags_from_string`）。
3. **isolate**: 隔离区实例生命周期（`v8_isolate_new`、`v8_isolate_dispose`、`v8_isolate_enter`、`v8_isolate_exit`、`v8_isolate_get_current`）。
4. **context**: 上下文作用域与全局对象管理（`v8_context_new`、`v8_context_enter`、`v8_context_exit`、`v8_context_dispose`）。
5. **value**: JavaScript 基本类型构造与判断（`v8_undefined`、`v8_null`、`v8_true`、`v8_false`、`v8_boolean_new` 及其 `v8_value_is_*` 系列）。
6. **platform**: 默认平台创建、消息泵与销毁（`v8_platform_new_default`、`v8_platform_pump_message_loop`、`v8_platform_dispose`）。
7. **snapshot**: 快照启动数据封装与校验（`V8StartupData`、`v8_startup_data_is_empty`）。
