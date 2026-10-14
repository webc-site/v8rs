# v8-codegen-csa

CodeStubAssembler (CSA) 与强类型 SSA 节点 (TNode) 图构造 DSL。

## 模块架构

- `tnode`: 强类型 SSA 节点封装 `TNode<T>` 与类型标记体系 (包含基本机器表示 `UntaggedT`, `WordT`, `Int32T`, `Float64T`, `BoolT` 等以及 JS 对象类型 `Object`, `Smi`, `HeapObject`, `Map` 等)。
- `code_assembler`: 基础汇编图构造器与控制流状态机，包含 31 项比较算子 (`CompareBinaryOp`)、65 项二元算子 (`BinaryOp`)、71 项一元算子 (`UnaryOp`)、标号系统 (`CodeAssemblerLabel`)、局部变量与 Phi 合并 (`CodeAssemblerVariable`)、以及状态中枢 (`CodeAssemblerState`)。
- `code_stub_assembler`: 高层 CodeStubAssembler 语义 DSL，封装 Smi 原语、堆对象与 Map 加载、类型检查谓词、内存分配接口与控制流分支。
- `error`: 强类型错误枚举 `Error` 与 `Result`。

## 规范与质量指标

- 100% Safe Rust (`#![forbid(unsafe_code)]`)
- 零 panic、零裸 unwrap/expect
- 扁平导出架构，无二次导出
- 单元测试 6/6 通过，Clippy 0 警告