// 由 tool/src/interpreter/bytecode-traits.h.js 从 v8/src/interpreter/bytecode-traits.h 生成; 修订就地改, 复跑覆盖
// 操作数缩放 traits 模板 → 显式常量表 (OperandScaler 展开; 对数 OPERAND_TYPE_LIST=23)

use crate::bytecode_operands::{OperandScale, OperandSize, OperandType, OperandTypeInfo};

/// 每操作数类型在三档刻度下的尺寸表 (原版 kOperandKindSizes[3][23]; 行 = Single/Double/Quadruple)
/// 由 OperandScaler<OperandType::kN, OperandScale::kS>::kOperandSize 全展开
#[rustfmt::skip]
pub static OPERAND_KIND_SIZES: [[OperandSize; 23]; 3] = [
  [OperandSize::None, OperandSize::Byte, OperandSize::Short, OperandSize::Byte, OperandSize::Short, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
  [OperandSize::None, OperandSize::Byte, OperandSize::Short, OperandSize::Byte, OperandSize::Short, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Short, OperandSize::Short, OperandSize::Short, OperandSize::Short, OperandSize::Short, OperandSize::Short, OperandSize::Short, OperandSize::Short, OperandSize::Short, OperandSize::Short, OperandSize::Short, OperandSize::Short, OperandSize::Short, OperandSize::Short, OperandSize::Short],
  [OperandSize::None, OperandSize::Byte, OperandSize::Short, OperandSize::Byte, OperandSize::Short, OperandSize::Byte, OperandSize::Byte, OperandSize::Byte, OperandSize::Quad, OperandSize::Quad, OperandSize::Quad, OperandSize::Quad, OperandSize::Quad, OperandSize::Quad, OperandSize::Quad, OperandSize::Quad, OperandSize::Quad, OperandSize::Quad, OperandSize::Quad, OperandSize::Quad, OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
];

/// 操作数类型列表 (OPERAND_TYPE_LIST 顺序, 供对照)
pub const OPERAND_TYPE_ORDER: [OperandType; 23] = [
  OperandType::None,
  OperandType::Flag8,
  OperandType::Flag16,
  OperandType::IntrinsicId,
  OperandType::RuntimeId,
  OperandType::NativeContextIndex,
  OperandType::AbortReason,
  OperandType::EmbeddedFeedback,
  OperandType::ConstantPoolIndex,
  OperandType::FeedbackSlot,
  OperandType::ContextSlot,
  OperandType::CoverageSlot,
  OperandType::UImm,
  OperandType::RegCount,
  OperandType::Imm,
  OperandType::Reg,
  OperandType::RegList,
  OperandType::RegPair,
  OperandType::RegOut,
  OperandType::RegOutList,
  OperandType::RegOutPair,
  OperandType::RegOutTriple,
  OperandType::RegInOut,
];

/// 缩放公式原样化 (原版 OperandScaler::kSize 静态计算)
/// size = 元信息基准尺寸 × (可缩放 ? 刻度值 : 1)
#[inline]
#[must_use]
pub const fn operand_scaled_size(info: OperandTypeInfo, scale: OperandScale) -> usize {
  let base = info.unscaled_size() as usize;
  if info.is_scalable() {
    base * (scale as usize)
  } else {
    base
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(OPERAND_TYPE_ORDER.len() == 23);
  assert!(operand_scaled_size(OperandTypeInfo::ScalableSignedByte, OperandScale::Quadruple) == 4);
  assert!(operand_scaled_size(OperandTypeInfo::FixedUnsignedShort, OperandScale::Quadruple) == 2);
  assert!(operand_scaled_size(OperandTypeInfo::None, OperandScale::Single) == 0);
  assert!(OPERAND_KIND_SIZES[0][OperandType::Imm as usize] as u8 == OperandSize::Byte as u8);
  assert!(OPERAND_KIND_SIZES[2][OperandType::Imm as usize] as u8 == OperandSize::Quad as u8);
  assert!(OPERAND_KIND_SIZES[1][OperandType::Flag16 as usize] as u8 == OperandSize::Short as u8);
};
