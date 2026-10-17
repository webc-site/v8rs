# v8-init 开发日志

> 2026-10-12: 自 todo.md 看板行与 Wave 9 派发收敛 (仅中文)。

## 门禁实测

- 单元测试: `CARGO_TARGET_DIR=target/agent_w9init cargo nextest run -p v8-init` (2/2 全部通过)
- Clippy: `CARGO_TARGET_DIR=target/agent_w9init cargo clippy -p v8-init --all-targets -- -D warnings` (0 警告)
- 格式化: `cargo +nightly fmt --check -p v8-init` (净)

## 模块与实现面

1. **v8**: V8 引擎全局启动状态机（`V8StartupState`），涵盖 `NotStarted`、`PlatformInitializing`、`PlatformInitialized`、`Initialized`、`Disposing`、`Disposed` 等 9 种完整生命周期阶段，提供单调推进验证。
2. **isolate_group**: 隔离组状态与配置常量，包含只读堆共享标志（`ENABLE_ATTACHED_SNAPSHOT_DATA`）、全局组哨兵及生命周期守卫。
3. **bootstrapper**: 引导器上下文配置（`Bootstrapper`），定义内建原型链建立与原生上下文槽位索引。
4. **icu_util**: ICU 国际化库绑定与初始化模式（`IcuInitializationMode`），支持系统数据路径加载与内嵌二进制数据切换。
