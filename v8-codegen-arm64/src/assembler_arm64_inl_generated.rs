// 由 tool/src/codegen/arm64/assembler-arm64-inl.h.js 从 v8/src/codegen/arm64/assembler-arm64-inl.h 生成; 修订就地改, 复跑覆盖
// arm64 汇编器内联面骨架

// 禁覆写面登记: 手写 assembler.rs / operand.rs 保持权威, 本文件仅登记 inl 方法面。
// 内联方法 (54 条):
// - IsHeapNumberRequest
// - IsImmediate
// - IsPlainRegister
// - IsShiftedRegister
// - IsExtendedRegister
// - IsZero
// - ImmediateValue
// - reg
// - Unreachable
// - target_pointer_address_at
// - target_address_at
// - set_embedded_object_index_referenced_from
// - deserialization_special_target_size
// - deserialization_set_target_internal_reference_at
// - set_target_address_at
// - set_target_compressed_address_at
// - uint32_constant_at
// - set_uint32_constant_at
// - LinkAndGetBranchInstructionOffsetTo
// - Flags
// - Cond
// - ImmPCRelAddress
// - ImmUncondBranch
// - ImmCondBranch
// - ImmCmpBranch
// - ImmTestBranch
// - ImmTestBranchBit
// - SF
// - ImmAddSub
// - ImmS
// - ImmR
// - ImmSetBits
// - ImmRotate
// - ImmLLiteral
// - BitN
// - ShiftDP
// - ImmDPShift
// - ExtendMode
// - ImmExtendShift
// - ImmCondCmp
// - Nzcv
// - ImmLSUnsigned
// - ImmLS
// - ImmLSPair
// - ImmShiftLS
// - ImmException
// - ImmSystemRegister
// - ImmHint
// - ImmBarrierDomain
// - ImmBarrierType
// - ImmMoveWide
// - ShiftMoveWide
// - FPType
// - FPScale

pub struct AssemblerInl;

// TODO(dev): 依手写 assembler.rs 的 API 对位补内联实现。

// —— 提取不变式断言 ——
const _: () = {};
