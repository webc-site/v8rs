# rs8 开发日志

> 2026-10-12: 自 todo.md 看板行与 L7 收敛 (仅中文)。

## 门禁实测

- 单元测试: `CARGO_TARGET_DIR=target/agent_rs8 cargo nextest run -p rs8` (4/4 全部通过)
- Clippy: `CARGO_TARGET_DIR=target/agent_rs8 cargo clippy -p rs8 --all-targets -- -D warnings` (0 警告)
- 格式化: `cargo +nightly fmt --check -p rs8` (净)

## 模块与实现面

1. **d8**: d8 Shell 核心选项、状态机与旗标映射表（`SHELL_OPTIONS` 56 旗标，`SHELL_PUBLIC_STATICS` 137 项公开接口，`SHELL_PRIVATE_STATICS` 65 项私有成员，`WorkerState` 5 态生命周期，`ModuleType` 6 种模块类型）。
2. **d8_console**: 控制台委托实现与路由表（`OUTPUT_ROUTES` 6 种重定向规则，`CONSOLE_METHODS` 13 种标准方法，`CPU_PROFILE_OUTPUT_FILENAME` 与默认计时器标签）。
3. **d8_js**: 内建 Stringify 源码嵌入（`STRINGIFY_SOURCE` 完整闭包实现，`STRINGIFY_DEPTH_LIMIT` 递归防环保护）。
4. **d8_platforms**: 平台包装（`PredictablePlatform` 可预测平台时钟步长，`DelayedTasksPlatform` 任务延迟上限与工作队列隔离哨兵）。
5. **hardware_watchpoints**: 硬件观察点支持（门控宏判定，DR 寄存器槽数常量）。
6. **async_hooks_wrapper**: 异步钩子包装（`AsyncContext` 异步执行与触发上下文，`PromiseHookType` 4 种生命周期事件）。
7. **memory_access_information**: 内存访问信息与反汇编解析表（`WIDTH_FROM_SUFFIX` 5 种定宽后缀，`GPR_ALIASES` 16 组通用寄存器别名，38 项核心助记符与 8 项比较指令表）。
