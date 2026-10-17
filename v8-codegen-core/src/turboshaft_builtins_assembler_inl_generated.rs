// 由 tool/src/codegen/turboshaft-builtins-assembler-inl.h.js 从 v8/src/codegen/turboshaft-builtins-assembler-inl.h 生成; 修订就地改, 复跑覆盖
// TS builtin 汇编别名与骨架

// 泛型别名登记 (原宏内 using 展开):
/// 原 `V` = compiler::turboshaft::V<T>
/// 原 `ConstOrV` = compiler::turboshaft::ConstOrV<Args...>
/// 原 `OptionalV` = compiler::turboshaft::OptionalV<T>
/// 原 `Label` = compiler::turboshaft::Label<Ts...>
/// 原 `LoopLabel` = compiler::turboshaft::LoopLabel<Ts...>
/// 原 `Tuple` = compiler::turboshaft::Tuple<Args...>
/// 原 `Block` = compiler::turboshaft::Block
/// 原 `OpIndex` = compiler::turboshaft::OpIndex

/// 模板方法面: 7 处 template 转发 (builtin 语义实现登记跳过)。
/// 原 IsKnownTaggedPointerTag = StrongAlias<bool> 标记。
pub struct TurboshaftBuiltinsAssemblerInl;

// TODO(dev): 依 turboshaft 框架补 builtin 汇编原语。

// —— 提取不变式断言 ——
const _: () = {};
