// 由 tool/src/interpreter/bytecode-decoder.h.js 从 v8/src/interpreter/bytecode-decoder.{h,cc} 生成; 修订就地改, 复跑覆盖
// 字节码解码器静态接口骨架

/// 原版 class BytecodeDecoder 成员声明 (机械 1:1 提取, 6 条, 全静态)
pub const BYTECODE_DECODER_DECLARATIONS: &[&str] = &[
  "[public] static Register DecodeRegisterOperand(Address operand_start, OperandType operand_type, OperandScale operand_scale)",
  "[public] static RegisterList DecodeRegisterListOperand(Address operand_start, uint32_t count, OperandType operand_type, OperandScale operand_scale)",
  "[public] static int32_t DecodeSignedOperand(Address operand_start, OperandType operand_type, OperandScale operand_scale)",
  "[public] static uint32_t DecodeUnsignedOperand(Address operand_start, OperandType operand_type, OperandScale operand_scale)",
  "[public] static uint8_t RacyDecodeEmbeddedFeedback(Address operand_start)",
  "[public] static std::ostream& Decode(std::ostream& os, const uint8_t* bytecode_start, Tagged<TrustedFixedArray> constant_pool, bool with_hex = true)",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_DECODER_DECLARATIONS.len() == 6);
};
