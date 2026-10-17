# v8-execution-isolate

V8 核心隔离实例 (`Isolate`) 与执行状态中枢的 Rust 1:1 转写实现。

## 核心职责

1. **`Isolate` 顶层实例**：
   - 独立的 JavaScript 运行时执行上下文，持有绑定的 `ThreadId`。
   - 协调调度 `StackGuard`（栈限与中断）、`Protectors`（去优化保护器位图）、`MicrotaskQueue`（微任务队列）与 `VMState`（虚拟机运行状态）。
   - 异常抛出、挂起、检查与清除机制。
   - 运行时函数调用与循环回边的物理栈限及强制终止检查（`check_execution_limits`）。

2. **`Protectors` 保护器系统** (`src/protectors.rs`，对应 `v8/src/execution/protectors.h`)：
   - 22 个优化保护器（如 `ArrayBufferDetaching`, `ArrayIteratorLookupChain`, `PromiseHook` 等）1:1 映射。
   - 22 位位图压缩无锁/轻量状态机，初始全有效（`ALL_VALID_MASK = 0x003F_FFFF`）。
   - 针对每个保护器提供细粒度查询 `is_intact` 与失效 `invalidate` 方法。

3. **`StackGuard` 栈限与中断系统** (`src/stack_guard.rs`，对应 `v8/src/execution/stack-guard.h`)：
   - 12 项中断原因标志枚举（`TERMINATE_EXECUTION`, `GC_REQUEST`, `INSTALL_CODE`, `API_INTERRUPT`, `GROW_SHARED_MEMORY` 等，对应 id 0..11）。
   - 3 级中断过滤（`NoGC`, `NoHeapWrites`, `AnyEffect`）与 `interrupt_level_mask`。
   - 原子栈限制指针（`jslimit`, `real_jslimit`, `climit`, `real_climit`）与中断哨兵地址（`INTERRUPT_LIMIT = usize::MAX`）。
   - 外部线程中断请求、清除与栈溢出检测。

4. **`VMState` 与调用签名** (`src/vm_state.rs`，对应 `v8/src/execution/vm-state.h`, `encoded-c-signature.h`, `thread-id.h`)：
   - 11 种虚拟机运行状态（`JS`, `GC`, `Parser`, `BytecodeCompiler`, `Compiler`, `External`, `Idle` 等）。
   - `ExternalCallbackScope` RAII 守卫，进出外部宿主边界时自动切换恢复状态。
   - `EncodedCSignature` C 函数通用与浮点寄存器调用约定编码。
   - 跨平台无死锁 `ThreadId`。

5. **`MicrotaskQueue` 微任务队列** (`src/microtask.rs`，对应 `v8/src/execution/microtask-queue.h`)：
   - 环形双端任务队列与深度限制（`MAX_NESTING_DEPTH = 1000`）。
   - 支持 `Auto`, `Scoped`, `Explicit` 调度策略。
   - 检查点排空机制与完成计数。

6. **`IsolateData` 字段布局** (`src/isolate_data.rs`，对应 `v8/src/execution/isolate-data-fields.h`)：
   - 80 项主字段与子字段 `IsolateFieldId` 严格 1:1 排布。

## 安全与质量保证

- **100% Safe Rust**：crate 根目录声明 `#![forbid(unsafe_code)]`。
- **零 panic**：所有可失败路径返回标准 `Result<T, Error>`。
- **机械一致性**：`tool/src/execution/` 下 4 个 ESM 脚本生成与原版 C++ 结构体双向对数。
- **单元测试**：5/5 单测全覆盖，clippy 0 警告。