# v8-wasm-interp 开发日志

> 2026-10-12 启动: Drumbrake WebAssembly 解释器 (栈常数、寄存器优化表、处理函数分发与内联包装尺寸)

## 一、架构设计与模块组织

- `v8-wasm-interp/src/lib.rs`: 顶层声明 `#![forbid(unsafe_code)]`，扁平导出所有子模块类型。
- `error.rs`: 统一定义错误类型与 `Result<T>` 别名。
- `wasm_interpreter_consts.rs`:
  - 栈界限常量模块 `stack_limits`（初始 1MB、步进 1MB、上限 32MB、守卫 1MB、预留 ~33MB）；
  - 解释器线程状态机（`InterpreterState`: Stopped, Running, Finished, Trapped, EhUnwinding）；
  - 控制流与异常枚举（`ExceptionHandlingResult`, `ExternalCallResult`, `OperatorMode`, `RegMode`）；
  - 256 项逐 opcode 寄存器变换优化表 `REG_MODES`；
  - 哨兵常量（CATCH_ALL_TAG_INDEX = -1, DELEGATE_TO_CALLER_INDEX = 0, INVALID_CODE_OFFSET = u32::MAX 等）。
- `wasm_interpreter_inl.rs`:
  - 局部槽值类别 `SlotValueKind`（I32, I64, F32, F64, S128, Ref）；
  - 逐类别槽宽计算 `value_size_in_slots`；
  - JS↔Wasm wrapper 打包数组对齐计算 `js_to_wasm_wrapper_packed_array_size`；
  - 内联函数签名表 `INLINE_FNS`（57 项展开）。
- `instruction_handlers_generated.rs`:
  - 1690 项 `InstructionHandler` 处理函数枚举，全面遵循 Rust 2024 UpperCamelCase 命名规范（R2r*, R2s*, S2r*, S2s*）；
  - 处理函数尺寸分类 `InstrHandlerSize`（Large, Small）；
  - `from_repr(val: u16) -> Option<InstructionHandler>` 完整映射。
- `tests/interp.rs`: 覆盖栈界限常量、状态枚举、槽宽与打包装尺寸计算、1690 项指令解码边界单测（5/5 passed）。

## 二、复审环日志

- **Round 1 (全新只读席, 目标 clean 1/3)**:
  - 顶层 `#![forbid(unsafe_code)]`，扁平导出架构无二次导出；
  - `InterpreterState`, `ExceptionHandlingResult`, `ExternalCallResult`, `OperatorMode`, `RegMode`, `SlotValueKind` 判别式 1:1 对位原版；
  - 1690 项 `InstructionHandler` 消除下划线转 UpperCamelCase（免 non_camel_case_types），解码映射无差错；
  - 采纳建议将包装打包数组尺寸计算放宽为 `impl Iterator<Item = SlotValueKind>`；
  - 隔离目录 `target/agent_interp_rev1` 门禁实测 5/5 全绿，Clippy 0 警告，Fmt 检查净；
  - 结论：无反馈意见 / 0 代码缺陷 / 测试全部覆盖通过（**达成 clean 1/3**）。
- **Round 2 (全新独立 Reviewer, 目标 clean 2/3)**:
  - 顶层安全契约核验：`src/lib.rs:1` 声明 `#![forbid(unsafe_code)]`，全库 100% Safe Rust；
  - 扁平导出架构与私有子模块，杜绝二次导出；
  - 状态/控制流枚举（`InterpreterState`, `ExceptionHandlingResult`, `ExternalCallResult`, `OperatorMode`, `RegMode`, `SlotValueKind`, `InstrHandlerSize`）与 1690 项 `InstructionHandler` 及 `REG_MODES` 逐一精确对位；
  - 包装尺寸对齐算法与槽宽规则 1:1 对齐 `wasm-interpreter-inl.h`；
  - 零 panic、零裸 unwrap / expect、零 dead code、零 allow 绕过；
  - 隔离目录 `target/agent_interp_rev2` 门禁实测 5/5 全绿，Clippy 0 警告，Fmt 检查净；
  - 结论：无反馈意见 / 0 代码缺陷 / 测试全部覆盖通过（**达成 clean 2/3**）。
- **Round 3 (全新独立终验 Reviewer, 目标 clean 3/3 终检验收冻结)**:
  - 顶层安全契约核验：`src/lib.rs:1` 声明 `#![forbid(unsafe_code)]`，全库 100% Safe Rust；
  - 扁平导出架构与私有子模块，杜绝二次导出；
  - 状态/控制流枚举（`InterpreterState`, `ExceptionHandlingResult`, `ExternalCallResult`, `OperatorMode`, `RegMode`, `SlotValueKind`, `InstrHandlerSize`）与 1690 项 `InstructionHandler` 全展开大驼峰及 `REG_MODES` 逐一精确对位；
  - 包装尺寸对齐算法与槽宽规则 1:1 对齐 `wasm-interpreter-inl.h` 并经全边界测试验证；
  - 零 panic、零裸 unwrap / expect、零 dead code、零 allow 绕过；
  - 隔离目录 `target/agent_interp_rev3` 门禁实测 5/5 全绿，Clippy 0 警告，Fmt 检查净；
  - 结论：无反馈意见 / 0 代码缺陷 / 测试全部覆盖通过（**达成 clean 3/3 终验冻结**）。

## 三、轮7 W7LIT (零头席)

- [轮7 W7LIT] grep todo 计数开工 2 → 收笼 2 (全保留): `wasm_interpreter_dispatch.rs.skel::init_instruction_table_rule`、`tail_dispatch_rule`。二者均在 `.skel` (未入编译图) 且为深水面 — builtin blob 解包/trap_handler 注册依赖 `Isolate`/`EmbeddedData`, 尾分派依赖 `ReadFnId` 编码; 上层类型未就位, 禁伪装填体, 已登记 `readme/zh.md` 推迟台账。门禁实测 211/211 全绿 (含本 crate)。



