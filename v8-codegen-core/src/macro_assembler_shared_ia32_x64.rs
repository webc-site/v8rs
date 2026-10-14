// 由 tool/src/codegen/shared-ia32-x64/macro-assembler-shared-ia32-x64.h.js 从 v8/src/codegen/shared-ia32-x64/macro-assembler-shared-ia32-x64.{h,cc} 生成; 修订就地改, 复跑覆盖
// x86 家族共享宏汇编器骨架

// ia32/x64 双目标共享面 (本仓库 x64 目标适用; ia32 登记跳过)。

/// wasm 栈保存 FP 大小分支:
/// - wasm 全浮点寄存器: 2 * kDoubleSize
/// - 其余: kDoubleSize
pub const STACK_SAVED_SAVED_FP_SIZE_WASM: usize = 2 * 8;
pub const STACK_SAVED_SAVED_FP_SIZE: usize = 8;

/// 原 class SharedMacroAssemblerBase 方法面 (63 条,
/// SSE/标量指令宏: Move/Add/And/Movhps/Movlps/Blendvps/Pinsrb/... ):
// - Move
// - Add
// - And
// - Movhps
// - Movlps
// - Blendvps
// - Blendvpd
// - Pblendvb
// - Pinsrb
// - Pinsrw
// - Pshufb
// - Shufps
// - Pshufd
// - emit
// - macro_name
// - F64x2ExtractLane
// - F64x2ReplaceLane
// - F64x2Min
// - F64x2Max
// - F32x4Splat
// - F32x4ExtractLane
// - F32x4Min
// - F32x4Max
// - S128Store32Lane
// - I8x16Splat
// - I8x16Shl
// - I8x16ShrS
// - I8x16ShrU
// - I16x8Splat
// - I16x8ExtMulLow
// - I16x8ExtMulHighS
// - I16x8ExtMulHighU
// - I16x8SConvertI8x16High
// - I16x8UConvertI8x16High
// - I16x8Q15MulRSatS
// - I16x8DotI8x16I7x16S
// - I32x4DotI8x16I7x16AddS
// - I32x4ExtAddPairwiseI16x8U
// - I32x4ExtMul
// - I32x4SConvertI16x8High
// - I32x4UConvertI16x8High
// - I64x2Neg
// - I64x2AbsPreAvx10
// - I64x2GtS
// - I64x2GeS
// - I64x2ShrSPreAvx10
// - I64x2MulPreAvx10
// - I64x2ExtMul
// - I64x2SConvertI32x4High
// - I64x2UConvertI32x4High
// - S128NotPreAvx10
// - S128SelectPreAvx10
// - S128Load8Splat
// - S128Load16Splat
// - S128Load32Splat
// - S128Store64Lane
// - F64x2Qfma
// - F64x2Qfms
// - F32x4Qfma
// - F32x4Qfms
// - PinsrHelper
// - I8x16SplatPreAvx2
// - I16x8SplatPreAvx2

pub struct SharedMacroAssemblerBase;

// TODO(dev): 依 x64 指令编码补实现 (cc 1404 行语义面登记跳过)。

// —— 提取不变式断言 ——
const _: () = {};
