// 由 tool/src/interpreter/interpreter-generator.h.js 从 v8/src/interpreter/interpreter-generator.{h,cc} 生成; 修订就地改, 复跑覆盖

/// 两个生成入口 (原版 extern 函数, 机械登记; 语义属 CodeStubAssembler/Turboshaft 生成器族)
pub const ENTRY_SIGNATURES: [&str; 2] = [
  "extern void GenerateBytecodeHandler(compiler::CodeAssemblerState* state, Bytecode bytecode, OperandScale operand_scale);",
  "extern void GenerateBytecodeHandlerTSA(compiler::turboshaft::PipelineData* data, Isolate* isolate, compiler::turboshaft::Graph& graph, Zone* zone, Bytecode bytecode, OperandScale operand_scale);",
];

/// cc 侧生成函数定义锚 (3702 行, 每字节码 handler 的 DoXxxGenerator 语义修订阶段处理)
pub const CC_DEFINITIONS: [&str; 2] = [
  "void GenerateBytecodeHandler(compiler::CodeAssemblerState* state,",
  "void GenerateBytecodeHandlerTSA(compiler::turboshaft::PipelineData* data,",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(ENTRY_SIGNATURES.len() == 2);
  assert!(CC_DEFINITIONS.len() == 2);
};
