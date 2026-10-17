// 由 tool/src/interpreter/interpreter-intrinsics-generator.h.js 从 v8/src/interpreter/interpreter-intrinsics-generator.{h,cc} 生成; 修订就地改, 复跑覆盖

/// 生成入口 (原版 extern 函数, 机械登记)
pub const ENTRY_SIGNATURES: [&str; 1] = [
  "extern TNode<Object> GenerateInvokeIntrinsic(InterpreterAssembler* assembler, TNode<Uint32T> function_id, TNode<Context> context, const InterpreterAssembler::RegListNodePair& args);",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(ENTRY_SIGNATURES.len() == 1);
};
