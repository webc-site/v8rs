// 由 tool/src/compiler/turboshaft/opmasks.h.js 从 v8/src/compiler/turboshaft/opmasks.h 生成; 已收编 (W11TS2), 复跑产物冲突以本体现态为准
// 具名 opmask 表 — 22 个 MaskBuilder 族 + 原版 kXxx 具名掩码 (小端编码)

use crate::{
  operations::{
    ChangeKind, ComparisonKind, ConstantKind, FloatBinopKind, FloatUnaryKind, Opcode,
    OverflowCheckedBinopKind, ShiftKind, Simd128ReplaceLaneKind, TaggedBitcastKind,
    WordBinopDeoptOnOverflowKind, WordBinopKind, WordUnaryKind,
  },
  opmasks::{MaskBuilder, OpMask, OpMaskField},
  representations::RegisterRepresentation,
};

// 布局事实 (operations.h): Operation 基座 4 字节 —
// opcode u8@0 / saturated_use_count u8@1 / input_count u16@2;
// 各 Op 载荷字段自偏移 4 起按声明序自然排布 (alignas(OpIndex)=4)。
// RegisterRepresentation 编码 (representations.rs 已核验): Word32=0, Word64=1,
// Float32=2, Float64=3, Tagged=4, Compressed=5, Simd128=6, Simd256=7;
// WordRepresentation/FloatRepresentation 为其实例子类型, 编码同值。
// 原版 WordPtr = kWord64 (64 位目标)。

/// 原版 FIELD(Op, f) — 载荷首字段恒 offset 4
const KIND_4: OpMaskField = OpMaskField {
  name: "kind",
  offset: 4,
  size: 1,
};
const REP_5: OpMaskField = OpMaskField {
  name: "rep",
  offset: 5,
  size: 1,
};

/// 原版 WordBinopMask = MaskBuilder<WordBinopOp, FIELD(kind), FIELD(rep)>
/// v8: opmasks.h WordBinopMask
pub static WORD_BINOP: MaskBuilder = MaskBuilder {
  fields: &[KIND_4, REP_5],
};
/// 原版 WordBinopKindMask = MaskBuilder<WordBinopOp, FIELD(kind)>
/// v8: opmasks.h WordBinopKindMask
pub static WORD_BINOP_KIND: MaskBuilder = MaskBuilder { fields: &[KIND_4] };
/// 原版 WordUnaryMask = MaskBuilder<WordUnaryOp, FIELD(kind), FIELD(rep)>
/// v8: opmasks.h WordUnaryMask
pub static WORD_UNARY: MaskBuilder = MaskBuilder {
  fields: &[KIND_4, REP_5],
};
/// 原版 FloatUnaryMask = MaskBuilder<FloatUnaryOp, FIELD(kind), FIELD(rep)>
/// v8: opmasks.h FloatUnaryMask
pub static FLOAT_UNARY: MaskBuilder = MaskBuilder {
  fields: &[KIND_4, REP_5],
};
/// 原版 FloatBinopMask = MaskBuilder<FloatBinopOp, FIELD(kind), FIELD(rep)>
/// v8: opmasks.h FloatBinopMask
pub static FLOAT_BINOP: MaskBuilder = MaskBuilder {
  fields: &[KIND_4, REP_5],
};
/// 原版 ShiftMask = MaskBuilder<ShiftOp, FIELD(kind), FIELD(rep)>
/// v8: opmasks.h ShiftMask
pub static SHIFT: MaskBuilder = MaskBuilder {
  fields: &[KIND_4, REP_5],
};
/// 原版 ShiftKindMask = MaskBuilder<ShiftOp, FIELD(kind)>
/// v8: opmasks.h ShiftKindMask
pub static SHIFT_KIND: MaskBuilder = MaskBuilder { fields: &[KIND_4] };
/// 原版 WordBinopDeoptOnOverflowMask = MaskBuilder<…, FIELD(kind)>
/// v8: opmasks.h WordBinopDeoptOnOverflowMask
pub static WORD_BINOP_DEOPT_ON_OVERFLOW: MaskBuilder = MaskBuilder { fields: &[KIND_4] };
/// 原版 PhiMask = MaskBuilder<PhiOp, FIELD(rep)> — PhiOp 首字段 rep@4
/// v8: opmasks.h PhiMask
pub static PHI: MaskBuilder = MaskBuilder {
  fields: &[OpMaskField {
    name: "rep",
    offset: 4,
    size: 1,
  }],
};
/// 原版 ConstantMask = MaskBuilder<ConstantOp, FIELD(kind)>
/// v8: opmasks.h ConstantMask
pub static CONSTANT: MaskBuilder = MaskBuilder { fields: &[KIND_4] };
/// 原版 ProjectionMask = MaskBuilder<ProjectionOp, FIELD(index)> — index u16@4
/// v8: opmasks.h ProjectionMask
pub static PROJECTION: MaskBuilder = MaskBuilder {
  fields: &[OpMaskField {
    name: "index",
    offset: 4,
    size: 2,
  }],
};
/// 原版 ComparisonMask = MaskBuilder<ComparisonOp, FIELD(kind), FIELD(rep)>
/// v8: opmasks.h ComparisonMask
pub static COMPARISON: MaskBuilder = MaskBuilder {
  fields: &[KIND_4, REP_5],
};
/// 原版 ComparisonKindMask = MaskBuilder<ComparisonOp, FIELD(kind)>
/// v8: opmasks.h ComparisonKindMask
pub static COMPARISON_KIND: MaskBuilder = MaskBuilder { fields: &[KIND_4] };
/// 原版 ChangeOpMask = MaskBuilder<ChangeOp, FIELD(kind), FIELD(assumption),
/// FIELD(from), FIELD(to)> — kind@4 assumption@5 from@6 to@7
/// v8: opmasks.h ChangeOpMask
pub static CHANGE_OP: MaskBuilder = MaskBuilder {
  fields: &[
    KIND_4,
    OpMaskField {
      name: "assumption",
      offset: 5,
      size: 1,
    },
    OpMaskField {
      name: "from",
      offset: 6,
      size: 1,
    },
    OpMaskField {
      name: "to",
      offset: 7,
      size: 1,
    },
  ],
};
/// 原版 OverflowCheckedBinopMask = MaskBuilder<…, FIELD(kind), FIELD(rep)>
/// v8: opmasks.h OverflowCheckedBinopMask
pub static OVERFLOW_CHECKED_BINOP: MaskBuilder = MaskBuilder {
  fields: &[KIND_4, REP_5],
};
/// 原版 TaggedBitcastMask = MaskBuilder<TaggedBitcastOp, FIELD(from), FIELD(to),
/// FIELD(kind)> — 字段声明序 kind@4 from@5 to@6, For 实参序 (from, to, kind)
/// v8: opmasks.h TaggedBitcastMask
pub static TAGGED_BITCAST: MaskBuilder = MaskBuilder {
  fields: &[
    OpMaskField {
      name: "from",
      offset: 5,
      size: 1,
    },
    OpMaskField {
      name: "to",
      offset: 6,
      size: 1,
    },
    KIND_4,
  ],
};
/// 原版 TaggedBitcastKindMask = MaskBuilder<TaggedBitcastOp, FIELD(kind)>
/// v8: opmasks.h TaggedBitcastKindMask
pub static TAGGED_BITCAST_KIND: MaskBuilder = MaskBuilder { fields: &[KIND_4] };
/// 原版 Simd128BinopMask = MaskBuilder<Simd128BinopOp, FIELD(kind)>
/// v8: opmasks.h Simd128BinopMask
pub static SIMD128_BINOP: MaskBuilder = MaskBuilder { fields: &[KIND_4] };
/// 原版 Simd128UnaryMask = MaskBuilder<Simd128UnaryOp, FIELD(kind)>
/// v8: opmasks.h Simd128UnaryMask
pub static SIMD128_UNARY: MaskBuilder = MaskBuilder { fields: &[KIND_4] };
/// 原版 Simd128ShiftMask = MaskBuilder<Simd128ShiftOp, FIELD(kind)>
/// v8: opmasks.h Simd128ShiftMask
pub static SIMD128_SHIFT: MaskBuilder = MaskBuilder { fields: &[KIND_4] };
/// 原版 Simd128LoadTransformMask = MaskBuilder<…, FIELD(transform_kind)> —
/// load_kind (packed 1B)@4, transform_kind@5
/// v8: opmasks.h Simd128LoadTransformMask
pub static SIMD128_LOAD_TRANSFORM: MaskBuilder = MaskBuilder {
  fields: &[OpMaskField {
    name: "transform_kind",
    offset: 5,
    size: 1,
  }],
};
/// 原版 Simd128ReplaceLaneMask = MaskBuilder<…, FIELD(kind)>
/// v8: opmasks.h Simd128ReplaceLaneMask
pub static SIMD128_REPLACE_LANE: MaskBuilder = MaskBuilder { fields: &[KIND_4] };

// 具面快捷: 各族按 opcode 实例化
macro_rules! named {
  ($doc:literal, $name:ident, $builder:ident, $op:expr, [$($arg:expr),*]) => {
    #[doc = $doc]
    #[must_use]
    pub const fn $name() -> OpMask {
      $builder.r#for($op.index(), &[$($arg as u64),*])
    }
  };
}

// —— WordBinopMask —— //
named!(
  "原版 kWord32Add = WordBinopMask::For<kAdd, Word32>",
  k_word32_add,
  WORD_BINOP,
  Opcode::WordBinop,
  [WordBinopKind::Add, RegisterRepresentation::Word32]
);
named!(
  "原版 kWord32Sub = WordBinopMask::For<kSub, Word32>",
  k_word32_sub,
  WORD_BINOP,
  Opcode::WordBinop,
  [WordBinopKind::Sub, RegisterRepresentation::Word32]
);
named!(
  "原版 kWord32Mul = WordBinopMask::For<kMul, Word32>",
  k_word32_mul,
  WORD_BINOP,
  Opcode::WordBinop,
  [WordBinopKind::Mul, RegisterRepresentation::Word32]
);
named!(
  "原版 kWord32SignedMulOverflownBits",
  k_word32_signed_mul_overflown_bits,
  WORD_BINOP,
  Opcode::WordBinop,
  [
    WordBinopKind::SignedMulOverflownBits,
    RegisterRepresentation::Word32
  ]
);
named!(
  "原版 kWord32UnsignedMulOverflownBits",
  k_word32_unsigned_mul_overflown_bits,
  WORD_BINOP,
  Opcode::WordBinop,
  [
    WordBinopKind::UnsignedMulOverflownBits,
    RegisterRepresentation::Word32
  ]
);
named!(
  "原版 kWord32BitwiseAnd",
  k_word32_bitwise_and,
  WORD_BINOP,
  Opcode::WordBinop,
  [WordBinopKind::BitwiseAnd, RegisterRepresentation::Word32]
);
named!(
  "原版 kWord32BitwiseOr",
  k_word32_bitwise_or,
  WORD_BINOP,
  Opcode::WordBinop,
  [WordBinopKind::BitwiseOr, RegisterRepresentation::Word32]
);
named!(
  "原版 kWord32BitwiseXor",
  k_word32_bitwise_xor,
  WORD_BINOP,
  Opcode::WordBinop,
  [WordBinopKind::BitwiseXor, RegisterRepresentation::Word32]
);
named!(
  "原版 kWord64Add",
  k_word64_add,
  WORD_BINOP,
  Opcode::WordBinop,
  [WordBinopKind::Add, RegisterRepresentation::Word64]
);
named!(
  "原版 kWord64Sub",
  k_word64_sub,
  WORD_BINOP,
  Opcode::WordBinop,
  [WordBinopKind::Sub, RegisterRepresentation::Word64]
);
named!(
  "原版 kWord64Mul",
  k_word64_mul,
  WORD_BINOP,
  Opcode::WordBinop,
  [WordBinopKind::Mul, RegisterRepresentation::Word64]
);
named!(
  "原版 kWord64BitwiseAnd",
  k_word64_bitwise_and,
  WORD_BINOP,
  Opcode::WordBinop,
  [WordBinopKind::BitwiseAnd, RegisterRepresentation::Word64]
);
named!(
  "原版 kWord64BitwiseOr",
  k_word64_bitwise_or,
  WORD_BINOP,
  Opcode::WordBinop,
  [WordBinopKind::BitwiseOr, RegisterRepresentation::Word64]
);
named!(
  "原版 kWord64BitwiseXor",
  k_word64_bitwise_xor,
  WORD_BINOP,
  Opcode::WordBinop,
  [WordBinopKind::BitwiseXor, RegisterRepresentation::Word64]
);
named!(
  "原版 kBitwiseAnd = WordBinopKindMask::For<kBitwiseAnd>",
  k_bitwise_and,
  WORD_BINOP_KIND,
  Opcode::WordBinop,
  [WordBinopKind::BitwiseAnd]
);
named!(
  "原版 kBitwiseXor = WordBinopKindMask::For<kBitwiseXor>",
  k_bitwise_xor,
  WORD_BINOP_KIND,
  Opcode::WordBinop,
  [WordBinopKind::BitwiseXor]
);

// —— WordUnaryMask —— //
named!(
  "原版 kWord32ReverseBytes",
  k_word32_reverse_bytes,
  WORD_UNARY,
  Opcode::WordUnary,
  [WordUnaryKind::ReverseBytes, RegisterRepresentation::Word32]
);
named!(
  "原版 kWord64ReverseBytes",
  k_word64_reverse_bytes,
  WORD_UNARY,
  Opcode::WordUnary,
  [WordUnaryKind::ReverseBytes, RegisterRepresentation::Word64]
);

// —— FloatUnaryMask —— //
named!(
  "原版 kFloat32Negate",
  k_float32_negate,
  FLOAT_UNARY,
  Opcode::FloatUnary,
  [FloatUnaryKind::Negate, RegisterRepresentation::Float32]
);
named!(
  "原版 kFloat64Abs",
  k_float64_abs,
  FLOAT_UNARY,
  Opcode::FloatUnary,
  [FloatUnaryKind::Abs, RegisterRepresentation::Float64]
);
named!(
  "原版 kFloat64Negate",
  k_float64_negate,
  FLOAT_UNARY,
  Opcode::FloatUnary,
  [FloatUnaryKind::Negate, RegisterRepresentation::Float64]
);

// —— FloatBinopMask —— //
named!(
  "原版 kFloat32Sub",
  k_float32_sub,
  FLOAT_BINOP,
  Opcode::FloatBinop,
  [FloatBinopKind::Sub, RegisterRepresentation::Float32]
);
named!(
  "原版 kFloat32Mul",
  k_float32_mul,
  FLOAT_BINOP,
  Opcode::FloatBinop,
  [FloatBinopKind::Mul, RegisterRepresentation::Float32]
);
named!(
  "原版 kFloat64Sub",
  k_float64_sub,
  FLOAT_BINOP,
  Opcode::FloatBinop,
  [FloatBinopKind::Sub, RegisterRepresentation::Float64]
);
named!(
  "原版 kFloat64Mul",
  k_float64_mul,
  FLOAT_BINOP,
  Opcode::FloatBinop,
  [FloatBinopKind::Mul, RegisterRepresentation::Float64]
);

// —— ShiftMask / ShiftKindMask —— //
named!(
  "原版 kWord32ShiftLeft",
  k_word32_shift_left,
  SHIFT,
  Opcode::Shift,
  [ShiftKind::ShiftLeft, RegisterRepresentation::Word32]
);
named!(
  "原版 kWord32ShiftRightArithmetic",
  k_word32_shift_right_arithmetic,
  SHIFT,
  Opcode::Shift,
  [
    ShiftKind::ShiftRightArithmetic,
    RegisterRepresentation::Word32
  ]
);
named!(
  "原版 kWord32ShiftRightArithmeticShiftOutZeros",
  k_word32_shift_right_arithmetic_shift_out_zeros,
  SHIFT,
  Opcode::Shift,
  [
    ShiftKind::ShiftRightArithmeticShiftOutZeros,
    RegisterRepresentation::Word32
  ]
);
named!(
  "原版 kWord32ShiftRightLogical",
  k_word32_shift_right_logical,
  SHIFT,
  Opcode::Shift,
  [ShiftKind::ShiftRightLogical, RegisterRepresentation::Word32]
);
named!(
  "原版 kWord32RotateRight",
  k_word32_rotate_right,
  SHIFT,
  Opcode::Shift,
  [ShiftKind::RotateRight, RegisterRepresentation::Word32]
);
named!(
  "原版 kWord64ShiftLeft",
  k_word64_shift_left,
  SHIFT,
  Opcode::Shift,
  [ShiftKind::ShiftLeft, RegisterRepresentation::Word64]
);
named!(
  "原版 kWord64ShiftRightArithmetic",
  k_word64_shift_right_arithmetic,
  SHIFT,
  Opcode::Shift,
  [
    ShiftKind::ShiftRightArithmetic,
    RegisterRepresentation::Word64
  ]
);
named!(
  "原版 kWord64ShiftRightLogical",
  k_word64_shift_right_logical,
  SHIFT,
  Opcode::Shift,
  [ShiftKind::ShiftRightLogical, RegisterRepresentation::Word64]
);
named!(
  "原版 kShiftLeft = ShiftKindMask::For<kShiftLeft>",
  k_shift_left,
  SHIFT_KIND,
  Opcode::Shift,
  [ShiftKind::ShiftLeft]
);

// —— WordBinopDeoptOnOverflowMask —— //
named!(
  "原版 kWordSignedAddDeoptOnOverflow",
  k_word_signed_add_deopt_on_overflow,
  WORD_BINOP_DEOPT_ON_OVERFLOW,
  Opcode::WordBinopDeoptOnOverflow,
  [WordBinopDeoptOnOverflowKind::SignedAdd]
);

// —— PhiMask —— //
named!(
  "原版 kTaggedPhi = PhiMask::For<Tagged>",
  k_tagged_phi,
  PHI,
  Opcode::Phi,
  [RegisterRepresentation::Tagged]
);

// —— ConstantMask —— //
named!(
  "原版 kWord32Constant",
  k_word32_constant,
  CONSTANT,
  Opcode::Constant,
  [ConstantKind::Word32]
);
named!(
  "原版 kWord64Constant",
  k_word64_constant,
  CONSTANT,
  Opcode::Constant,
  [ConstantKind::Word64]
);
named!(
  "原版 kFloat64Constant",
  k_float64_constant,
  CONSTANT,
  Opcode::Constant,
  [ConstantKind::Float64]
);
named!(
  "原版 kExternalConstant",
  k_external_constant,
  CONSTANT,
  Opcode::Constant,
  [ConstantKind::External]
);
named!(
  "原版 kHeapConstant",
  k_heap_constant,
  CONSTANT,
  Opcode::Constant,
  [ConstantKind::HeapObject]
);
named!(
  "原版 kSmiConstant",
  k_smi_constant,
  CONSTANT,
  Opcode::Constant,
  [ConstantKind::Smi]
);
named!(
  "原版 kTaggedIndexConstant",
  k_tagged_index_constant,
  CONSTANT,
  Opcode::Constant,
  [ConstantKind::TaggedIndex]
);
named!(
  "原版 kWasmStubCallConstant",
  k_wasm_stub_call_constant,
  CONSTANT,
  Opcode::Constant,
  [ConstantKind::RelocatableWasmStubCall]
);

// —— ProjectionMask —— //
named!(
  "原版 kProjection0",
  k_projection0,
  PROJECTION,
  Opcode::Projection,
  [0u16]
);
named!(
  "原版 kProjection1",
  k_projection1,
  PROJECTION,
  Opcode::Projection,
  [1u16]
);

// —— ComparisonMask / ComparisonKindMask —— //
named!(
  "原版 kWord32Equal",
  k_word32_equal,
  COMPARISON,
  Opcode::Comparison,
  [ComparisonKind::Equal, RegisterRepresentation::Word32]
);
named!(
  "原版 kWord64Equal",
  k_word64_equal,
  COMPARISON,
  Opcode::Comparison,
  [ComparisonKind::Equal, RegisterRepresentation::Word64]
);
named!(
  "原版 kComparisonEqual = ComparisonKindMask::For<kEqual>",
  k_comparison_equal,
  COMPARISON_KIND,
  Opcode::Comparison,
  [ComparisonKind::Equal]
);

// —— ChangeOpMask —— //
// 原版 ChangeOp::Assumption::kNoAssumption = 0 (operations.h 声明序首项)
const NO_ASSUMPTION: u64 = 0;
named!(
  "原版 kChangeInt32ToInt64",
  k_change_int32_to_int64,
  CHANGE_OP,
  Opcode::Change,
  [
    ChangeKind::SignExtend,
    NO_ASSUMPTION,
    RegisterRepresentation::Word32,
    RegisterRepresentation::Word64
  ]
);
named!(
  "原版 kChangeUint32ToUint64",
  k_change_uint32_to_uint64,
  CHANGE_OP,
  Opcode::Change,
  [
    ChangeKind::ZeroExtend,
    NO_ASSUMPTION,
    RegisterRepresentation::Word32,
    RegisterRepresentation::Word64
  ]
);
named!(
  "原版 kChangeFloat32ToUint32 = For<kBitcast, kNoAssumption, Float32, Word32>",
  k_change_float32_to_uint32,
  CHANGE_OP,
  Opcode::Change,
  [
    ChangeKind::Bitcast,
    NO_ASSUMPTION,
    RegisterRepresentation::Float32,
    RegisterRepresentation::Word32
  ]
);
named!(
  "原版 kFloat64ExtractHighWord32",
  k_float64_extract_high_word32,
  CHANGE_OP,
  Opcode::Change,
  [
    ChangeKind::ExtractHighHalf,
    NO_ASSUMPTION,
    RegisterRepresentation::Float64,
    RegisterRepresentation::Word32
  ]
);
named!(
  "原版 kTruncateFloat64ToInt64OverflowToMin",
  k_truncate_float64_to_int64_overflow_to_min,
  CHANGE_OP,
  Opcode::Change,
  [
    ChangeKind::SignedFloatTruncateOverflowToMin,
    NO_ASSUMPTION,
    RegisterRepresentation::Float64,
    RegisterRepresentation::Word64
  ]
);
named!(
  "原版 kTruncateFloat64ToUint64OverflowToMin",
  k_truncate_float64_to_uint64_overflow_to_min,
  CHANGE_OP,
  Opcode::Change,
  [
    ChangeKind::UnsignedFloatTruncateOverflowToMin,
    NO_ASSUMPTION,
    RegisterRepresentation::Float64,
    RegisterRepresentation::Word64
  ]
);
named!(
  "原版 kTruncateFloat32ToInt32OverflowToMin",
  k_truncate_float32_to_int32_overflow_to_min,
  CHANGE_OP,
  Opcode::Change,
  [
    ChangeKind::SignedFloatTruncateOverflowToMin,
    NO_ASSUMPTION,
    RegisterRepresentation::Float32,
    RegisterRepresentation::Word32
  ]
);
named!(
  "原版 kTruncateFloat32ToUint32OverflowToMin",
  k_truncate_float32_to_uint32_overflow_to_min,
  CHANGE_OP,
  Opcode::Change,
  [
    ChangeKind::UnsignedFloatTruncateOverflowToMin,
    NO_ASSUMPTION,
    RegisterRepresentation::Float32,
    RegisterRepresentation::Word32
  ]
);
named!(
  "原版 kTruncateWord64ToWord32",
  k_truncate_word64_to_word32,
  CHANGE_OP,
  Opcode::Change,
  [
    ChangeKind::Truncate,
    NO_ASSUMPTION,
    RegisterRepresentation::Word64,
    RegisterRepresentation::Word32
  ]
);

// —— OverflowCheckedBinopMask —— //
named!(
  "原版 kOverflowCheckedWord32Add",
  k_overflow_checked_word32_add,
  OVERFLOW_CHECKED_BINOP,
  Opcode::OverflowCheckedBinop,
  [
    OverflowCheckedBinopKind::SignedAdd,
    RegisterRepresentation::Word32
  ]
);

// —— TaggedBitcastMask / TaggedBitcastKindMask —— //
named!(
  "原版 kBitcastTaggedToWordPtrForTagAndSmiBits (WordPtr = kWord64, 64 位目标)",
  k_bitcast_tagged_to_word_ptr_for_tag_and_smi_bits,
  TAGGED_BITCAST,
  Opcode::TaggedBitcast,
  [
    RegisterRepresentation::Tagged,
    RegisterRepresentation::Word64,
    TaggedBitcastKind::TagAndSmiBits
  ]
);
named!(
  "原版 kBitcastWordPtrToSmi",
  k_bitcast_word_ptr_to_smi,
  TAGGED_BITCAST,
  Opcode::TaggedBitcast,
  [
    RegisterRepresentation::Word64,
    RegisterRepresentation::Tagged,
    TaggedBitcastKind::Smi
  ]
);
named!(
  "原版 kTaggedBitcastSmi",
  k_tagged_bitcast_smi,
  TAGGED_BITCAST_KIND,
  Opcode::TaggedBitcast,
  [TaggedBitcastKind::Smi]
);
named!(
  "原版 kTaggedBitcastHeapObject",
  k_tagged_bitcast_heap_object,
  TAGGED_BITCAST_KIND,
  Opcode::TaggedBitcast,
  [TaggedBitcastKind::HeapObject]
);

// —— Simd128 族 —— //
// 具名 kSimd128I32x4Mul / kSimd128I16x8Mul / kSimd128And / kSimd128AndNot /
// kSimd128Xor 延期: 其 Kind 值由 FOREACH_SIMD_128_BINARY_OPCODE 宏链展开而定,
// operations.rs 的 Simd128BinopKind 仅登记 extension 首末 4 变体, 全量宏展开
// 待 operations lower 面 (台账)。以下泛型面为 SIMD_*_MASK X-macro 的机械等价:
// 实参为原版 Kind 判别值 (uint8_t)。

/// 原版 SIMD_EXTENSION_BINOP_MASK(kind) 族 — kSimd128<kind>
/// v8: opmasks.h FOREACH_SIMD_128_BINARY_EXTENSION_OPCODE 展开面
#[must_use]
pub const fn k_simd128_binop(kind: u8) -> OpMask {
  SIMD128_BINOP.r#for(Opcode::Simd128Binop.index(), &[kind as u64])
}

/// 原版 SIMD_UNARY_MASK(kind) 族
/// v8: opmasks.h FOREACH_SIMD_128_UNARY_OPCODE 展开面
#[must_use]
pub const fn k_simd128_unary(kind: u8) -> OpMask {
  SIMD128_UNARY.r#for(Opcode::Simd128Unary.index(), &[kind as u64])
}

/// 原版 SIMD_SHIFT_MASK(kind) 族
/// v8: opmasks.h FOREACH_SIMD_128_SHIFT_OPCODE 展开面
#[must_use]
pub const fn k_simd128_shift(kind: u8) -> OpMask {
  SIMD128_SHIFT.r#for(Opcode::Simd128Shift.index(), &[kind as u64])
}

/// 原版 SIMD_LOAD_TRANSFORM_MASK(kind) 族
/// v8: opmasks.h FOREACH_SIMD_128_LOAD_TRANSFORM_OPCODE 展开面
#[must_use]
pub const fn k_simd128_load_transform(transform_kind: u8) -> OpMask {
  SIMD128_LOAD_TRANSFORM.r#for(
    Opcode::Simd128LoadTransform.index(),
    &[transform_kind as u64],
  )
}

named!(
  "原版 kSimd128ReplaceLaneF32x4",
  k_simd128_replace_lane_f32x4,
  SIMD128_REPLACE_LANE,
  Opcode::Simd128ReplaceLane,
  [Simd128ReplaceLaneKind::F32x4]
);

// Simd256UnaryMask / FOREACH_SIMD_256_UNARY 族: 目标配置 SIMD256=0
// (arm64 无 SIMD256, 见 operations.rs 头注), 原版 #if V8_ENABLE_SIMD256 不编译 —
// 登记不实现。ConvertOpTargetMask 族: 原版仅文档示例 (opmasks.h 头注释),
// 无具名消费者 — 登记不实现。
