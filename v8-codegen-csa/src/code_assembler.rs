// 由 tool/tool/src/codegen/csa/code-assembler.js 从 v8/v8/src/compiler/code-assembler.{h,cc} 生成; 修订就地改, 复跑覆盖
// CodeAssembler 汇编器状态机与算子元数据体系

use strum::{Display, EnumString};
pub use v8_builtins_core::BuiltinKind;
pub use v8_codegen_core::CallInterfaceDescriptor;

use crate::{
  error::{Error, Result},
  tnode::{
    BoolT, Float64T, Int32T, Int64T, IntPtrT, NodeId, Object, Smi, TNode, Uint32T, Word32T, WordT,
  },
};

// -----------------------------------------------------------------------------
// 1. 比较二元算子 (CompareBinaryOp: 31 项)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString)]
pub enum CompareBinaryOp {
  /// Float32Equal(Float32T, Float32T) -> BoolT
  Float32Equal,
  /// Float32LessThan(Float32T, Float32T) -> BoolT
  Float32LessThan,
  /// Float32LessThanOrEqual(Float32T, Float32T) -> BoolT
  Float32LessThanOrEqual,
  /// Float32GreaterThan(Float32T, Float32T) -> BoolT
  Float32GreaterThan,
  /// Float32GreaterThanOrEqual(Float32T, Float32T) -> BoolT
  Float32GreaterThanOrEqual,
  /// Float64Equal(Float64T, Float64T) -> BoolT
  Float64Equal,
  /// Float64NotEqual(Float64T, Float64T) -> BoolT
  Float64NotEqual,
  /// Float64LessThan(Float64T, Float64T) -> BoolT
  Float64LessThan,
  /// Float64LessThanOrEqual(Float64T, Float64T) -> BoolT
  Float64LessThanOrEqual,
  /// Float64GreaterThan(Float64T, Float64T) -> BoolT
  Float64GreaterThan,
  /// Float64GreaterThanOrEqual(Float64T, Float64T) -> BoolT
  Float64GreaterThanOrEqual,
  /// Int32GreaterThan(Word32T, Word32T) -> BoolT
  Int32GreaterThan,
  /// Int32GreaterThanOrEqual(Word32T, Word32T) -> BoolT
  Int32GreaterThanOrEqual,
  /// Int32LessThan(Word32T, Word32T) -> BoolT
  Int32LessThan,
  /// Int32LessThanOrEqual(Word32T, Word32T) -> BoolT
  Int32LessThanOrEqual,
  /// IntPtrLessThan(WordT, WordT) -> BoolT
  IntPtrLessThan,
  /// IntPtrLessThanOrEqual(WordT, WordT) -> BoolT
  IntPtrLessThanOrEqual,
  /// IntPtrGreaterThan(WordT, WordT) -> BoolT
  IntPtrGreaterThan,
  /// IntPtrGreaterThanOrEqual(WordT, WordT) -> BoolT
  IntPtrGreaterThanOrEqual,
  /// Uint32LessThan(Word32T, Word32T) -> BoolT
  Uint32LessThan,
  /// Uint32LessThanOrEqual(Word32T, Word32T) -> BoolT
  Uint32LessThanOrEqual,
  /// Uint32GreaterThan(Word32T, Word32T) -> BoolT
  Uint32GreaterThan,
  /// Uint32GreaterThanOrEqual(Word32T, Word32T) -> BoolT
  Uint32GreaterThanOrEqual,
  /// Uint64LessThan(Word64T, Word64T) -> BoolT
  Uint64LessThan,
  /// Uint64LessThanOrEqual(Word64T, Word64T) -> BoolT
  Uint64LessThanOrEqual,
  /// Uint64GreaterThan(Word64T, Word64T) -> BoolT
  Uint64GreaterThan,
  /// Uint64GreaterThanOrEqual(Word64T, Word64T) -> BoolT
  Uint64GreaterThanOrEqual,
  /// UintPtrLessThan(WordT, WordT) -> BoolT
  UintPtrLessThan,
  /// UintPtrLessThanOrEqual(WordT, WordT) -> BoolT
  UintPtrLessThanOrEqual,
  /// UintPtrGreaterThan(WordT, WordT) -> BoolT
  UintPtrGreaterThan,
  /// UintPtrGreaterThanOrEqual(WordT, WordT) -> BoolT
  UintPtrGreaterThanOrEqual,
}

impl CompareBinaryOp {
  /// 比较算子总数 (31 项)
  pub const COUNT: usize = 31;

  #[inline]
  #[must_use]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::Float32Equal => "Float32Equal",
      Self::Float32LessThan => "Float32LessThan",
      Self::Float32LessThanOrEqual => "Float32LessThanOrEqual",
      Self::Float32GreaterThan => "Float32GreaterThan",
      Self::Float32GreaterThanOrEqual => "Float32GreaterThanOrEqual",
      Self::Float64Equal => "Float64Equal",
      Self::Float64NotEqual => "Float64NotEqual",
      Self::Float64LessThan => "Float64LessThan",
      Self::Float64LessThanOrEqual => "Float64LessThanOrEqual",
      Self::Float64GreaterThan => "Float64GreaterThan",
      Self::Float64GreaterThanOrEqual => "Float64GreaterThanOrEqual",
      Self::Int32GreaterThan => "Int32GreaterThan",
      Self::Int32GreaterThanOrEqual => "Int32GreaterThanOrEqual",
      Self::Int32LessThan => "Int32LessThan",
      Self::Int32LessThanOrEqual => "Int32LessThanOrEqual",
      Self::IntPtrLessThan => "IntPtrLessThan",
      Self::IntPtrLessThanOrEqual => "IntPtrLessThanOrEqual",
      Self::IntPtrGreaterThan => "IntPtrGreaterThan",
      Self::IntPtrGreaterThanOrEqual => "IntPtrGreaterThanOrEqual",
      Self::Uint32LessThan => "Uint32LessThan",
      Self::Uint32LessThanOrEqual => "Uint32LessThanOrEqual",
      Self::Uint32GreaterThan => "Uint32GreaterThan",
      Self::Uint32GreaterThanOrEqual => "Uint32GreaterThanOrEqual",
      Self::Uint64LessThan => "Uint64LessThan",
      Self::Uint64LessThanOrEqual => "Uint64LessThanOrEqual",
      Self::Uint64GreaterThan => "Uint64GreaterThan",
      Self::Uint64GreaterThanOrEqual => "Uint64GreaterThanOrEqual",
      Self::UintPtrLessThan => "UintPtrLessThan",
      Self::UintPtrLessThanOrEqual => "UintPtrLessThanOrEqual",
      Self::UintPtrGreaterThan => "UintPtrGreaterThan",
      Self::UintPtrGreaterThanOrEqual => "UintPtrGreaterThanOrEqual",
    }
  }

  #[inline]
  #[must_use]
  pub const fn is_float_compare(&self) -> bool {
    matches!(
      self,
      Self::Float32Equal
        | Self::Float32LessThan
        | Self::Float32LessThanOrEqual
        | Self::Float32GreaterThan
        | Self::Float32GreaterThanOrEqual
        | Self::Float64Equal
        | Self::Float64NotEqual
        | Self::Float64LessThan
        | Self::Float64LessThanOrEqual
        | Self::Float64GreaterThan
        | Self::Float64GreaterThanOrEqual
    )
  }

  #[inline]
  #[must_use]
  pub const fn is_signed_int_compare(&self) -> bool {
    matches!(
      self,
      Self::Int32GreaterThan
        | Self::Int32GreaterThanOrEqual
        | Self::Int32LessThan
        | Self::Int32LessThanOrEqual
        | Self::IntPtrLessThan
        | Self::IntPtrLessThanOrEqual
        | Self::IntPtrGreaterThan
        | Self::IntPtrGreaterThanOrEqual
    )
  }

  #[inline]
  #[must_use]
  pub const fn is_unsigned_int_compare(&self) -> bool {
    matches!(
      self,
      Self::Uint32LessThan
        | Self::Uint32LessThanOrEqual
        | Self::Uint32GreaterThan
        | Self::Uint32GreaterThanOrEqual
        | Self::Uint64LessThan
        | Self::Uint64LessThanOrEqual
        | Self::Uint64GreaterThan
        | Self::Uint64GreaterThanOrEqual
        | Self::UintPtrLessThan
        | Self::UintPtrLessThanOrEqual
        | Self::UintPtrGreaterThan
        | Self::UintPtrGreaterThanOrEqual
    )
  }
}

// -----------------------------------------------------------------------------
// 2. 二元算子 (BinaryOp: 65 项)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString)]
pub enum BinaryOp {
  /// Float32Sub(Float32T, Float32T) -> Float32T
  Float32Sub,
  /// Float32Add(Float32T, Float32T) -> Float32T
  Float32Add,
  /// Float32Mul(Float32T, Float32T) -> Float32T
  Float32Mul,
  /// Float64Add(Float64T, Float64T) -> Float64T
  Float64Add,
  /// Float64Sub(Float64T, Float64T) -> Float64T
  Float64Sub,
  /// Float64Mul(Float64T, Float64T) -> Float64T
  Float64Mul,
  /// Float64Div(Float64T, Float64T) -> Float64T
  Float64Div,
  /// Float64Mod(Float64T, Float64T) -> Float64T
  Float64Mod,
  /// Float64Atan2(Float64T, Float64T) -> Float64T
  Float64Atan2,
  /// Float64Pow(Float64T, Float64T) -> Float64T
  Float64Pow,
  /// Float64Max(Float64T, Float64T) -> Float64T
  Float64Max,
  /// Float64Min(Float64T, Float64T) -> Float64T
  Float64Min,
  /// Float64InsertLowWord32(Float64T, Word32T) -> Float64T
  Float64InsertLowWord32,
  /// Float64InsertHighWord32(Float64T, Word32T) -> Float64T
  Float64InsertHighWord32,
  /// I8x16Eq(I8x16T, I8x16T) -> I8x16T
  I8x16Eq,
  /// IntPtrAdd(WordT, WordT) -> WordT
  IntPtrAdd,
  /// IntPtrSub(WordT, WordT) -> WordT
  IntPtrSub,
  /// IntPtrMul(WordT, WordT) -> WordT
  IntPtrMul,
  /// IntPtrMulHigh(IntPtrT, IntPtrT) -> IntPtrT
  IntPtrMulHigh,
  /// UintPtrMulHigh(UintPtrT, UintPtrT) -> UintPtrT
  UintPtrMulHigh,
  /// IntPtrDiv(IntPtrT, IntPtrT) -> IntPtrT
  IntPtrDiv,
  /// IntPtrMod(IntPtrT, IntPtrT) -> IntPtrT
  IntPtrMod,
  /// IntPtrAddWithOverflow(BoolT), IntPtrT, IntPtrT) -> PAIR_TYPE(IntPtrT
  IntPtrAddWithOverflow,
  /// IntPtrSubWithOverflow(BoolT), IntPtrT, IntPtrT) -> PAIR_TYPE(IntPtrT
  IntPtrSubWithOverflow,
  /// IntPtrMulWithOverflow(BoolT), IntPtrT, IntPtrT) -> PAIR_TYPE(IntPtrT
  IntPtrMulWithOverflow,
  /// Int32Add(Word32T, Word32T) -> Word32T
  Int32Add,
  /// Int32AddWithOverflow(BoolT), Int32T, Int32T) -> PAIR_TYPE(Int32T
  Int32AddWithOverflow,
  /// Int32Sub(Word32T, Word32T) -> Word32T
  Int32Sub,
  /// Int32SubWithOverflow(BoolT), Int32T, Int32T) -> PAIR_TYPE(Int32T
  Int32SubWithOverflow,
  /// Int32Mul(Word32T, Word32T) -> Word32T
  Int32Mul,
  /// Int32MulWithOverflow(BoolT), Int32T, Int32T) -> PAIR_TYPE(Int32T
  Int32MulWithOverflow,
  /// Int32Div(Int32T, Int32T) -> Int32T
  Int32Div,
  /// Uint32Div(Uint32T, Uint32T) -> Uint32T
  Uint32Div,
  /// Int32Mod(Int32T, Int32T) -> Int32T
  Int32Mod,
  /// Uint32Mod(Uint32T, Uint32T) -> Uint32T
  Uint32Mod,
  /// Int64Add(Word64T, Word64T) -> Word64T
  Int64Add,
  /// Int64Sub(Word64T, Word64T) -> Word64T
  Int64Sub,
  /// Int64SubWithOverflow(BoolT), Int64T, Int64T) -> PAIR_TYPE(Int64T
  Int64SubWithOverflow,
  /// Int64Mul(Word64T, Word64T) -> Word64T
  Int64Mul,
  /// Int64MulHigh(Int64T, Int64T) -> Int64T
  Int64MulHigh,
  /// Uint64MulHigh(Uint64T, Uint64T) -> Uint64T
  Uint64MulHigh,
  /// Int64Div(Int64T, Int64T) -> Int64T
  Int64Div,
  /// Int64Mod(Int64T, Int64T) -> Int64T
  Int64Mod,
  /// WordOr(WordT, WordT) -> WordT
  WordOr,
  /// WordAnd(WordT, WordT) -> WordT
  WordAnd,
  /// WordXor(WordT, WordT) -> WordT
  WordXor,
  /// WordRor(WordT, IntegralT) -> WordT
  WordRor,
  /// WordShl(WordT, IntegralT) -> WordT
  WordShl,
  /// WordShr(WordT, IntegralT) -> WordT
  WordShr,
  /// WordSar(WordT, IntegralT) -> WordT
  WordSar,
  /// WordSarShiftOutZeros(WordT, IntegralT) -> WordT
  WordSarShiftOutZeros,
  /// Word32Or(Word32T, Word32T) -> Word32T
  Word32Or,
  /// Word32And(Word32T, Word32T) -> Word32T
  Word32And,
  /// Word32Xor(Word32T, Word32T) -> Word32T
  Word32Xor,
  /// Word32Ror(Word32T, Word32T) -> Word32T
  Word32Ror,
  /// Word32Shl(Word32T, Word32T) -> Word32T
  Word32Shl,
  /// Word32Shr(Word32T, Word32T) -> Word32T
  Word32Shr,
  /// Word32Sar(Word32T, Word32T) -> Word32T
  Word32Sar,
  /// Word32SarShiftOutZeros(Word32T, Word32T) -> Word32T
  Word32SarShiftOutZeros,
  /// Word64And(Word64T, Word64T) -> Word64T
  Word64And,
  /// Word64Or(Word64T, Word64T) -> Word64T
  Word64Or,
  /// Word64Xor(Word64T, Word64T) -> Word64T
  Word64Xor,
  /// Word64Shl(Word64T, Word64T) -> Word64T
  Word64Shl,
  /// Word64Shr(Word64T, Word64T) -> Word64T
  Word64Shr,
  /// Word64Sar(Word64T, Word64T) -> Word64T
  Word64Sar,
}

impl BinaryOp {
  /// 二元算子总数 (65 项)
  pub const COUNT: usize = 65;

  #[inline]
  #[must_use]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::Float32Sub => "Float32Sub",
      Self::Float32Add => "Float32Add",
      Self::Float32Mul => "Float32Mul",
      Self::Float64Add => "Float64Add",
      Self::Float64Sub => "Float64Sub",
      Self::Float64Mul => "Float64Mul",
      Self::Float64Div => "Float64Div",
      Self::Float64Mod => "Float64Mod",
      Self::Float64Atan2 => "Float64Atan2",
      Self::Float64Pow => "Float64Pow",
      Self::Float64Max => "Float64Max",
      Self::Float64Min => "Float64Min",
      Self::Float64InsertLowWord32 => "Float64InsertLowWord32",
      Self::Float64InsertHighWord32 => "Float64InsertHighWord32",
      Self::I8x16Eq => "I8x16Eq",
      Self::IntPtrAdd => "IntPtrAdd",
      Self::IntPtrSub => "IntPtrSub",
      Self::IntPtrMul => "IntPtrMul",
      Self::IntPtrMulHigh => "IntPtrMulHigh",
      Self::UintPtrMulHigh => "UintPtrMulHigh",
      Self::IntPtrDiv => "IntPtrDiv",
      Self::IntPtrMod => "IntPtrMod",
      Self::IntPtrAddWithOverflow => "IntPtrAddWithOverflow",
      Self::IntPtrSubWithOverflow => "IntPtrSubWithOverflow",
      Self::IntPtrMulWithOverflow => "IntPtrMulWithOverflow",
      Self::Int32Add => "Int32Add",
      Self::Int32AddWithOverflow => "Int32AddWithOverflow",
      Self::Int32Sub => "Int32Sub",
      Self::Int32SubWithOverflow => "Int32SubWithOverflow",
      Self::Int32Mul => "Int32Mul",
      Self::Int32MulWithOverflow => "Int32MulWithOverflow",
      Self::Int32Div => "Int32Div",
      Self::Uint32Div => "Uint32Div",
      Self::Int32Mod => "Int32Mod",
      Self::Uint32Mod => "Uint32Mod",
      Self::Int64Add => "Int64Add",
      Self::Int64Sub => "Int64Sub",
      Self::Int64SubWithOverflow => "Int64SubWithOverflow",
      Self::Int64Mul => "Int64Mul",
      Self::Int64MulHigh => "Int64MulHigh",
      Self::Uint64MulHigh => "Uint64MulHigh",
      Self::Int64Div => "Int64Div",
      Self::Int64Mod => "Int64Mod",
      Self::WordOr => "WordOr",
      Self::WordAnd => "WordAnd",
      Self::WordXor => "WordXor",
      Self::WordRor => "WordRor",
      Self::WordShl => "WordShl",
      Self::WordShr => "WordShr",
      Self::WordSar => "WordSar",
      Self::WordSarShiftOutZeros => "WordSarShiftOutZeros",
      Self::Word32Or => "Word32Or",
      Self::Word32And => "Word32And",
      Self::Word32Xor => "Word32Xor",
      Self::Word32Ror => "Word32Ror",
      Self::Word32Shl => "Word32Shl",
      Self::Word32Shr => "Word32Shr",
      Self::Word32Sar => "Word32Sar",
      Self::Word32SarShiftOutZeros => "Word32SarShiftOutZeros",
      Self::Word64And => "Word64And",
      Self::Word64Or => "Word64Or",
      Self::Word64Xor => "Word64Xor",
      Self::Word64Shl => "Word64Shl",
      Self::Word64Shr => "Word64Shr",
      Self::Word64Sar => "Word64Sar",
    }
  }

  #[inline]
  #[must_use]
  pub const fn is_bitwise(&self) -> bool {
    matches!(
      self,
      Self::WordOr
        | Self::WordAnd
        | Self::WordXor
        | Self::WordRor
        | Self::WordShl
        | Self::WordShr
        | Self::WordSar
        | Self::WordSarShiftOutZeros
        | Self::Word32Or
        | Self::Word32And
        | Self::Word32Xor
        | Self::Word32Ror
    )
  }

  #[inline]
  #[must_use]
  pub const fn is_arithmetic(&self) -> bool {
    matches!(
      self,
      Self::IntPtrAdd
        | Self::IntPtrSub
        | Self::IntPtrMul
        | Self::Int32Add
        | Self::Int32Sub
        | Self::Int32Mul
        | Self::Int64Add
        | Self::Int64Sub
        | Self::Int64Mul
        | Self::Float64Add
        | Self::Float64Sub
        | Self::Float64Mul
        | Self::Float64Div
    )
  }
}

// -----------------------------------------------------------------------------
// 3. 一元算子 (UnaryOp: 71 项)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString)]
pub enum UnaryOp {
  /// Float32Abs(Float32T) -> Float32T
  Float32Abs,
  /// Float64Abs(Float64T) -> Float64T
  Float64Abs,
  /// Float64Acos(Float64T) -> Float64T
  Float64Acos,
  /// Float64Acosh(Float64T) -> Float64T
  Float64Acosh,
  /// Float64Asin(Float64T) -> Float64T
  Float64Asin,
  /// Float64Asinh(Float64T) -> Float64T
  Float64Asinh,
  /// Float64Atan(Float64T) -> Float64T
  Float64Atan,
  /// Float64Atanh(Float64T) -> Float64T
  Float64Atanh,
  /// Float64Cos(Float64T) -> Float64T
  Float64Cos,
  /// Float64Cosh(Float64T) -> Float64T
  Float64Cosh,
  /// Float64Exp(Float64T) -> Float64T
  Float64Exp,
  /// Float64Expm1(Float64T) -> Float64T
  Float64Expm1,
  /// Float64Log(Float64T) -> Float64T
  Float64Log,
  /// Float64Log1p(Float64T) -> Float64T
  Float64Log1p,
  /// Float64Log2(Float64T) -> Float64T
  Float64Log2,
  /// Float64Log10(Float64T) -> Float64T
  Float64Log10,
  /// Float64Cbrt(Float64T) -> Float64T
  Float64Cbrt,
  /// Float64Neg(Float64T) -> Float64T
  Float64Neg,
  /// Float64Sin(Float64T) -> Float64T
  Float64Sin,
  /// Float64Sinh(Float64T) -> Float64T
  Float64Sinh,
  /// Float64Sqrt(Float64T) -> Float64T
  Float64Sqrt,
  /// Float64Tan(Float64T) -> Float64T
  Float64Tan,
  /// Float64Tanh(Float64T) -> Float64T
  Float64Tanh,
  /// Float64ExtractLowWord32(Float64T) -> Uint32T
  Float64ExtractLowWord32,
  /// Float64ExtractHighWord32(Float64T) -> Uint32T
  Float64ExtractHighWord32,
  /// BitcastTaggedToWord(Object) -> IntPtrT
  BitcastTaggedToWord,
  /// BitcastTaggedToWordForTagAndSmiBits(AnyTaggedT) -> IntPtrT
  BitcastTaggedToWordForTagAndSmiBits,
  /// BitcastMaybeObjectToWord(MaybeObject) -> IntPtrT
  BitcastMaybeObjectToWord,
  /// BitcastWordToTagged(WordT) -> Object
  BitcastWordToTagged,
  /// BitcastWordToTaggedSigned(WordT) -> Smi
  BitcastWordToTaggedSigned,
  /// TruncateFloat64ToFloat32(Float64T) -> Float32T
  TruncateFloat64ToFloat32,
  /// TruncateFloat64ToFloat16RawBits(Float64T) -> Float16RawBitsT
  TruncateFloat64ToFloat16RawBits,
  /// TruncateFloat64ToWord32(Float64T) -> Uint32T
  TruncateFloat64ToWord32,
  /// TruncateInt64ToInt32(Int64T) -> Int32T
  TruncateInt64ToInt32,
  /// ChangeFloat32ToFloat64(Float32T) -> Float64T
  ChangeFloat32ToFloat64,
  /// ChangeFloat16RawBitsToFloat64(Float16RawBitsT) -> Float64T
  ChangeFloat16RawBitsToFloat64,
  /// ChangeFloat64ToUint32(Float64T) -> Uint32T
  ChangeFloat64ToUint32,
  /// ChangeFloat64ToUint64(Float64T) -> Uint64T
  ChangeFloat64ToUint64,
  /// ChangeFloat64ToInt64(Float64T) -> Int64T
  ChangeFloat64ToInt64,
  /// ChangeInt32ToFloat64(Int32T) -> Float64T
  ChangeInt32ToFloat64,
  /// ChangeInt32ToInt64(Int32T) -> Int64T
  ChangeInt32ToInt64,
  /// ChangeUint32ToFloat64(Word32T) -> Float64T
  ChangeUint32ToFloat64,
  /// ChangeUint32ToUint64(Word32T) -> Uint64T
  ChangeUint32ToUint64,
  /// ChangeInt64ToFloat64(Int64T) -> Float64T
  ChangeInt64ToFloat64,
  /// BitcastInt32ToFloat32(Word32T) -> Float32T
  BitcastInt32ToFloat32,
  /// BitcastFloat32ToInt32(Float32T) -> Uint32T
  BitcastFloat32ToInt32,
  /// BitcastFloat64ToInt64(Float64T) -> Int64T
  BitcastFloat64ToInt64,
  /// BitcastInt64ToFloat64(Int64T) -> Float64T
  BitcastInt64ToFloat64,
  /// RoundFloat64ToInt32(Float64T) -> Int32T
  RoundFloat64ToInt32,
  /// RoundInt32ToFloat32(Int32T) -> Float32T
  RoundInt32ToFloat32,
  /// Float64SilenceNaN(Float64T) -> Float64T
  Float64SilenceNaN,
  /// Float64RoundDown(Float64T) -> Float64T
  Float64RoundDown,
  /// Float64RoundUp(Float64T) -> Float64T
  Float64RoundUp,
  /// Float64RoundTiesEven(Float64T) -> Float64T
  Float64RoundTiesEven,
  /// Float64RoundTruncate(Float64T) -> Float64T
  Float64RoundTruncate,
  /// Word32Clz(Word32T) -> Int32T
  Word32Clz,
  /// Word64Clz(Word64T) -> Int64T
  Word64Clz,
  /// Word32Ctz(Word32T) -> Int32T
  Word32Ctz,
  /// Word64Ctz(Word64T) -> Int64T
  Word64Ctz,
  /// Word32Popcnt(Word32T) -> Int32T
  Word32Popcnt,
  /// Word64Popcnt(Word64T) -> Int64T
  Word64Popcnt,
  /// Word32BitwiseNot(Word32T) -> Word32T
  Word32BitwiseNot,
  /// WordNot(WordT) -> WordT
  WordNot,
  /// Word64Not(Word64T) -> Word64T
  Word64Not,
  /// I8x16BitMask(I8x16T) -> Int32T
  I8x16BitMask,
  /// I8x16Splat(Int32T) -> I8x16T
  I8x16Splat,
  /// Int32AbsWithOverflow(BoolT) -> PAIR_TYPE(Int32T
  Int32AbsWithOverflow,
  /// Int64AbsWithOverflow(BoolT) -> PAIR_TYPE(Int64T
  Int64AbsWithOverflow,
  /// IntPtrAbsWithOverflow(BoolT) -> PAIR_TYPE(IntPtrT
  IntPtrAbsWithOverflow,
  /// Word32BinaryNot(Word32T) -> BoolT
  Word32BinaryNot,
  /// StackPointerGreaterThan(WordT) -> BoolT
  StackPointerGreaterThan,
}

impl UnaryOp {
  /// 一元算子总数 (71 项)
  pub const COUNT: usize = 71;

  #[inline]
  #[must_use]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::Float32Abs => "Float32Abs",
      Self::Float64Abs => "Float64Abs",
      Self::Float64Acos => "Float64Acos",
      Self::Float64Acosh => "Float64Acosh",
      Self::Float64Asin => "Float64Asin",
      Self::Float64Asinh => "Float64Asinh",
      Self::Float64Atan => "Float64Atan",
      Self::Float64Atanh => "Float64Atanh",
      Self::Float64Cos => "Float64Cos",
      Self::Float64Cosh => "Float64Cosh",
      Self::Float64Exp => "Float64Exp",
      Self::Float64Expm1 => "Float64Expm1",
      Self::Float64Log => "Float64Log",
      Self::Float64Log1p => "Float64Log1p",
      Self::Float64Log2 => "Float64Log2",
      Self::Float64Log10 => "Float64Log10",
      Self::Float64Cbrt => "Float64Cbrt",
      Self::Float64Neg => "Float64Neg",
      Self::Float64Sin => "Float64Sin",
      Self::Float64Sinh => "Float64Sinh",
      Self::Float64Sqrt => "Float64Sqrt",
      Self::Float64Tan => "Float64Tan",
      Self::Float64Tanh => "Float64Tanh",
      Self::Float64ExtractLowWord32 => "Float64ExtractLowWord32",
      Self::Float64ExtractHighWord32 => "Float64ExtractHighWord32",
      Self::BitcastTaggedToWord => "BitcastTaggedToWord",
      Self::BitcastTaggedToWordForTagAndSmiBits => "BitcastTaggedToWordForTagAndSmiBits",
      Self::BitcastMaybeObjectToWord => "BitcastMaybeObjectToWord",
      Self::BitcastWordToTagged => "BitcastWordToTagged",
      Self::BitcastWordToTaggedSigned => "BitcastWordToTaggedSigned",
      Self::TruncateFloat64ToFloat32 => "TruncateFloat64ToFloat32",
      Self::TruncateFloat64ToFloat16RawBits => "TruncateFloat64ToFloat16RawBits",
      Self::TruncateFloat64ToWord32 => "TruncateFloat64ToWord32",
      Self::TruncateInt64ToInt32 => "TruncateInt64ToInt32",
      Self::ChangeFloat32ToFloat64 => "ChangeFloat32ToFloat64",
      Self::ChangeFloat16RawBitsToFloat64 => "ChangeFloat16RawBitsToFloat64",
      Self::ChangeFloat64ToUint32 => "ChangeFloat64ToUint32",
      Self::ChangeFloat64ToUint64 => "ChangeFloat64ToUint64",
      Self::ChangeFloat64ToInt64 => "ChangeFloat64ToInt64",
      Self::ChangeInt32ToFloat64 => "ChangeInt32ToFloat64",
      Self::ChangeInt32ToInt64 => "ChangeInt32ToInt64",
      Self::ChangeUint32ToFloat64 => "ChangeUint32ToFloat64",
      Self::ChangeUint32ToUint64 => "ChangeUint32ToUint64",
      Self::ChangeInt64ToFloat64 => "ChangeInt64ToFloat64",
      Self::BitcastInt32ToFloat32 => "BitcastInt32ToFloat32",
      Self::BitcastFloat32ToInt32 => "BitcastFloat32ToInt32",
      Self::BitcastFloat64ToInt64 => "BitcastFloat64ToInt64",
      Self::BitcastInt64ToFloat64 => "BitcastInt64ToFloat64",
      Self::RoundFloat64ToInt32 => "RoundFloat64ToInt32",
      Self::RoundInt32ToFloat32 => "RoundInt32ToFloat32",
      Self::Float64SilenceNaN => "Float64SilenceNaN",
      Self::Float64RoundDown => "Float64RoundDown",
      Self::Float64RoundUp => "Float64RoundUp",
      Self::Float64RoundTiesEven => "Float64RoundTiesEven",
      Self::Float64RoundTruncate => "Float64RoundTruncate",
      Self::Word32Clz => "Word32Clz",
      Self::Word64Clz => "Word64Clz",
      Self::Word32Ctz => "Word32Ctz",
      Self::Word64Ctz => "Word64Ctz",
      Self::Word32Popcnt => "Word32Popcnt",
      Self::Word64Popcnt => "Word64Popcnt",
      Self::Word32BitwiseNot => "Word32BitwiseNot",
      Self::WordNot => "WordNot",
      Self::Word64Not => "Word64Not",
      Self::I8x16BitMask => "I8x16BitMask",
      Self::I8x16Splat => "I8x16Splat",
      Self::Int32AbsWithOverflow => "Int32AbsWithOverflow",
      Self::Int64AbsWithOverflow => "Int64AbsWithOverflow",
      Self::IntPtrAbsWithOverflow => "IntPtrAbsWithOverflow",
      Self::Word32BinaryNot => "Word32BinaryNot",
      Self::StackPointerGreaterThan => "StackPointerGreaterThan",
    }
  }

  #[inline]
  #[must_use]
  pub const fn is_bitcast(&self) -> bool {
    matches!(
      self,
      Self::BitcastTaggedToWord
        | Self::BitcastTaggedToWordForTagAndSmiBits
        | Self::BitcastMaybeObjectToWord
        | Self::BitcastWordToTagged
        | Self::BitcastWordToTaggedSigned
        | Self::BitcastInt32ToFloat32
        | Self::BitcastFloat32ToInt32
        | Self::BitcastFloat64ToInt64
        | Self::BitcastInt64ToFloat64
    )
  }

  #[inline]
  #[must_use]
  pub const fn is_truncate_or_change(&self) -> bool {
    matches!(
      self,
      Self::TruncateFloat64ToFloat32
        | Self::TruncateFloat64ToFloat16RawBits
        | Self::TruncateFloat64ToWord32
        | Self::TruncateInt64ToInt32
        | Self::ChangeFloat32ToFloat64
        | Self::ChangeFloat16RawBitsToFloat64
        | Self::ChangeFloat64ToUint32
        | Self::ChangeFloat64ToUint64
        | Self::ChangeFloat64ToInt64
        | Self::ChangeInt32ToFloat64
        | Self::ChangeInt32ToInt64
        | Self::ChangeUint32ToFloat64
        | Self::ChangeUint32ToUint64
        | Self::ChangeInt64ToFloat64
    )
  }
}

// -----------------------------------------------------------------------------
// 4. CodeAssemblerLabel 状态与控制流标记
// -----------------------------------------------------------------------------

/// 跳转标号状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelState {
  /// 尚未绑定代码位置
  Unbound,
  /// 已绑定当前代码块
  Bound,
  /// 已与其他控制流路径合并
  Merged,
}

/// 代码存根汇编跳转标号
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeAssemblerLabel {
  id: u32,
  name: &'static str,
  state: LabelState,
  merged_count: usize,
}

impl CodeAssemblerLabel {
  /// 创建未绑定标号
  #[inline]
  #[must_use]
  pub const fn new(id: u32, name: &'static str) -> Self {
    Self {
      id,
      name,
      state: LabelState::Unbound,
      merged_count: 0,
    }
  }

  #[inline]
  #[must_use]
  pub const fn id(&self) -> u32 {
    self.id
  }

  #[inline]
  #[must_use]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  #[inline]
  #[must_use]
  pub const fn is_bound(&self) -> bool {
    matches!(self.state, LabelState::Bound | LabelState::Merged)
  }

  #[inline]
  #[must_use]
  pub const fn merged_count(&self) -> usize {
    self.merged_count
  }

  /// 绑定该标号
  #[inline]
  pub fn bind(&mut self) -> Result<()> {
    if self.state == LabelState::Bound {
      return Err(Error::LabelAlreadyBound(self.name));
    }
    self.state = LabelState::Bound;
    Ok(())
  }

  /// 记录合并跳转流入
  #[inline]
  pub fn record_merge(&mut self) {
    self.merged_count = self.merged_count.saturating_add(1);
    if self.state == LabelState::Bound {
      self.state = LabelState::Merged;
    }
  }
}

// -----------------------------------------------------------------------------
// 5. SSA 变量与图节点记录
// -----------------------------------------------------------------------------

/// 机器表示
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MachineRep {
  Word32,
  Word64,
  Float32,
  Float64,
  Tagged,
  Pointer,
}

/// 局部汇编变量 (支持跨分支值赋值与 Phi 合并)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeAssemblerVariable {
  id: u32,
  rep: MachineRep,
  val: NodeId,
}

impl CodeAssemblerVariable {
  #[inline]
  #[must_use]
  pub const fn new(id: u32, rep: MachineRep) -> Self {
    Self {
      id,
      rep,
      val: NodeId::INVALID,
    }
  }

  #[inline]
  #[must_use]
  pub const fn id(&self) -> u32 {
    self.id
  }

  #[inline]
  #[must_use]
  pub const fn representation(&self) -> MachineRep {
    self.rep
  }

  #[inline]
  #[must_use]
  pub const fn value(&self) -> NodeId {
    self.val
  }

  #[inline]
  pub fn set(&mut self, val: NodeId) {
    self.val = val;
  }
}

/// 图节点记录种类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeOp {
  ConstantInt32(i32),
  ConstantInt64(i64),
  ConstantFloat64(u64), // f64 bits
  ConstantBool(bool),
  Compare(CompareBinaryOp),
  Binary(BinaryOp),
  Unary(UnaryOp),
  Load,
  Store,
  CallRuntime(u32),
  CallBuiltin(u32, BuiltinKind),
  Phi,
}

/// 构图中间表示节点条目
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeRecord {
  pub id: NodeId,
  pub op: NodeOp,
  pub inputs: [NodeId; 3],
  pub input_count: u8,
}

// -----------------------------------------------------------------------------
// 6. CodeAssemblerState 汇编图状态管理
// -----------------------------------------------------------------------------

/// 汇编器状态上下文
#[derive(Debug, Default)]
pub struct CodeAssemblerState {
  next_node_id: u32,
  next_label_id: u32,
  next_var_id: u32,
  nodes: Vec<NodeRecord>,
  current_block: u32,
}

impl CodeAssemblerState {
  #[inline]
  #[must_use]
  pub fn new() -> Self {
    Self {
      next_node_id: 1,
      next_label_id: 1,
      next_var_id: 1,
      nodes: Vec::new(),
      current_block: 0,
    }
  }

  #[inline]
  #[must_use]
  pub const fn current_block(&self) -> u32 {
    self.current_block
  }

  #[inline]
  pub fn alloc_node_id(&mut self) -> NodeId {
    let id = self.next_node_id;
    self.next_node_id = self.next_node_id.saturating_add(1);
    NodeId(id)
  }

  #[inline]
  pub fn alloc_label(&mut self, name: &'static str) -> CodeAssemblerLabel {
    let id = self.next_label_id;
    self.next_label_id = self.next_label_id.saturating_add(1);
    CodeAssemblerLabel::new(id, name)
  }

  #[inline]
  pub fn alloc_variable(&mut self, rep: MachineRep) -> CodeAssemblerVariable {
    let id = self.next_var_id;
    self.next_var_id = self.next_var_id.saturating_add(1);
    CodeAssemblerVariable::new(id, rep)
  }

  #[inline]
  pub fn emit(&mut self, op: NodeOp, inputs: &[NodeId]) -> NodeId {
    let id = self.alloc_node_id();
    let mut in_arr = [NodeId::INVALID; 3];
    let count = inputs.len().min(3) as u8;
    for (i, &inp) in inputs.iter().take(3).enumerate() {
      in_arr[i] = inp;
    }
    self.nodes.push(NodeRecord {
      id,
      op,
      inputs: in_arr,
      input_count: count,
    });
    id
  }

  #[inline]
  #[must_use]
  pub fn node_count(&self) -> usize {
    self.nodes.len()
  }

  #[inline]
  #[must_use]
  pub fn nodes(&self) -> &[NodeRecord] {
    &self.nodes
  }
}

// -----------------------------------------------------------------------------
// 7. CodeAssembler 核心图构造 DSL
// -----------------------------------------------------------------------------

/// 基础图构建与控制流汇编器
pub struct CodeAssembler<'a> {
  pub(crate) state: &'a mut CodeAssemblerState,
}

impl<'a> CodeAssembler<'a> {
  #[inline]
  #[must_use]
  pub fn new(state: &'a mut CodeAssemblerState) -> Self {
    Self { state }
  }

  #[inline]
  pub fn alloc_label(&mut self, name: &'static str) -> CodeAssemblerLabel {
    self.state.alloc_label(name)
  }

  #[inline]
  pub fn alloc_variable(&mut self, rep: MachineRep) -> CodeAssemblerVariable {
    self.state.alloc_variable(rep)
  }

  // --- 常量生成 ---

  #[inline]
  pub fn int32_constant(&mut self, val: i32) -> TNode<Int32T> {
    let id = self.state.emit(NodeOp::ConstantInt32(val), &[]);
    TNode::new(id)
  }

  #[inline]
  pub fn int64_constant(&mut self, val: i64) -> TNode<Int64T> {
    let id = self.state.emit(NodeOp::ConstantInt64(val), &[]);
    TNode::new(id)
  }

  #[inline]
  pub fn intptr_constant(&mut self, val: isize) -> TNode<IntPtrT> {
    let id = self.state.emit(NodeOp::ConstantInt64(val as i64), &[]);
    TNode::new(id)
  }

  #[inline]
  pub fn uintptr_constant(&mut self, val: usize) -> TNode<Uint32T> {
    let id = self.state.emit(NodeOp::ConstantInt32(val as i32), &[]);
    TNode::new(id)
  }

  #[inline]
  pub fn float64_constant(&mut self, val: f64) -> TNode<Float64T> {
    let id = self.state.emit(NodeOp::ConstantFloat64(val.to_bits()), &[]);
    TNode::new(id)
  }

  #[inline]
  pub fn bool_constant(&mut self, val: bool) -> TNode<BoolT> {
    let id = self.state.emit(NodeOp::ConstantBool(val), &[]);
    TNode::new(id)
  }

  #[inline]
  pub fn smi_constant(&mut self, val: i32) -> TNode<Smi> {
    let id = self.state.emit(NodeOp::ConstantInt32(val), &[]);
    TNode::new(id)
  }

  // --- 比较运算 ---

  #[inline]
  pub fn compare_binary_op(
    &mut self,
    op: CompareBinaryOp,
    lhs: NodeId,
    rhs: NodeId,
  ) -> TNode<BoolT> {
    let id = self.state.emit(NodeOp::Compare(op), &[lhs, rhs]);
    TNode::new(id)
  }

  #[inline]
  pub fn word_equal(&mut self, lhs: TNode<WordT>, rhs: TNode<WordT>) -> TNode<BoolT> {
    self.compare_binary_op(
      CompareBinaryOp::IntPtrLessThan,
      lhs.node_id(),
      rhs.node_id(),
    )
  }

  #[inline]
  pub fn word32_equal(&mut self, lhs: TNode<BoolT>, rhs: TNode<BoolT>) -> TNode<BoolT> {
    self.compare_binary_op(CompareBinaryOp::Int32LessThan, lhs.node_id(), rhs.node_id())
  }

  #[inline]
  pub fn int32_less_than(&mut self, lhs: TNode<Int32T>, rhs: TNode<Int32T>) -> TNode<BoolT> {
    self.compare_binary_op(CompareBinaryOp::Int32LessThan, lhs.node_id(), rhs.node_id())
  }

  #[inline]
  pub fn int32_greater_than(&mut self, lhs: TNode<Int32T>, rhs: TNode<Int32T>) -> TNode<BoolT> {
    self.compare_binary_op(
      CompareBinaryOp::Int32GreaterThan,
      lhs.node_id(),
      rhs.node_id(),
    )
  }

  // --- 二元运算 ---

  #[inline]
  pub fn binary_op(&mut self, op: BinaryOp, lhs: NodeId, rhs: NodeId) -> NodeId {
    self.state.emit(NodeOp::Binary(op), &[lhs, rhs])
  }

  #[inline]
  pub fn int32_add(&mut self, lhs: TNode<Int32T>, rhs: TNode<Int32T>) -> TNode<Int32T> {
    let id = self.binary_op(BinaryOp::Int32Add, lhs.node_id(), rhs.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn int32_sub(&mut self, lhs: TNode<Int32T>, rhs: TNode<Int32T>) -> TNode<Int32T> {
    let id = self.binary_op(BinaryOp::Int32Sub, lhs.node_id(), rhs.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn int32_mul(&mut self, lhs: TNode<Int32T>, rhs: TNode<Int32T>) -> TNode<Int32T> {
    let id = self.binary_op(BinaryOp::Int32Mul, lhs.node_id(), rhs.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn intptr_add(&mut self, lhs: TNode<IntPtrT>, rhs: TNode<IntPtrT>) -> TNode<IntPtrT> {
    let id = self.binary_op(BinaryOp::IntPtrAdd, lhs.node_id(), rhs.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn intptr_sub(&mut self, lhs: TNode<IntPtrT>, rhs: TNode<IntPtrT>) -> TNode<IntPtrT> {
    let id = self.binary_op(BinaryOp::IntPtrSub, lhs.node_id(), rhs.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn word_shl(&mut self, lhs: TNode<WordT>, shift: TNode<Word32T>) -> TNode<WordT> {
    let id = self.binary_op(BinaryOp::WordShl, lhs.node_id(), shift.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn word_sar(&mut self, lhs: TNode<WordT>, shift: TNode<Word32T>) -> TNode<WordT> {
    let id = self.binary_op(BinaryOp::WordSar, lhs.node_id(), shift.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn word_and(&mut self, lhs: TNode<WordT>, rhs: TNode<WordT>) -> TNode<WordT> {
    let id = self.binary_op(BinaryOp::WordAnd, lhs.node_id(), rhs.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn word_or(&mut self, lhs: TNode<WordT>, rhs: TNode<WordT>) -> TNode<WordT> {
    let id = self.binary_op(BinaryOp::WordOr, lhs.node_id(), rhs.node_id());
    TNode::new(id)
  }

  // --- 一元运算 ---

  #[inline]
  pub fn unary_op(&mut self, op: UnaryOp, input: NodeId) -> NodeId {
    self.state.emit(NodeOp::Unary(op), &[input])
  }

  #[inline]
  pub fn word_not(&mut self, val: TNode<WordT>) -> TNode<WordT> {
    let id = self.unary_op(UnaryOp::WordNot, val.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn bitcast_word_to_tagged(&mut self, val: TNode<WordT>) -> TNode<Object> {
    let id = self.unary_op(UnaryOp::BitcastWordToTagged, val.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn bitcast_tagged_to_word(&mut self, val: TNode<Object>) -> TNode<IntPtrT> {
    let id = self.unary_op(UnaryOp::BitcastTaggedToWord, val.node_id());
    TNode::new(id)
  }

  // --- 控制流 ---

  #[inline]
  pub fn bind(&mut self, label: &mut CodeAssemblerLabel) -> Result<()> {
    label.bind()
  }

  #[inline]
  pub fn goto(&mut self, label: &mut CodeAssemblerLabel) {
    label.record_merge();
  }

  #[inline]
  pub fn goto_if(&mut self, _cond: TNode<BoolT>, label: &mut CodeAssemblerLabel) {
    label.record_merge();
  }

  #[inline]
  pub fn goto_if_not(&mut self, _cond: TNode<BoolT>, label: &mut CodeAssemblerLabel) {
    label.record_merge();
  }

  #[inline]
  pub fn branch(
    &mut self,
    _cond: TNode<BoolT>,
    true_label: &mut CodeAssemblerLabel,
    false_label: &mut CodeAssemblerLabel,
  ) {
    true_label.record_merge();
    false_label.record_merge();
  }

  // --- 调用 ---

  #[inline]
  pub fn call_runtime(&mut self, function_id: u32, args: &[NodeId]) -> NodeId {
    let id = self.state.alloc_node_id();
    let mut in_arr = [NodeId::INVALID; 3];
    let count = args.len().min(3) as u8;
    for (i, &arg) in args.iter().take(3).enumerate() {
      in_arr[i] = arg;
    }
    self.state.nodes.push(NodeRecord {
      id,
      op: NodeOp::CallRuntime(function_id),
      inputs: in_arr,
      input_count: count,
    });
    id
  }

  #[inline]
  pub fn call_builtin(&mut self, builtin_id: u32, kind: BuiltinKind, args: &[NodeId]) -> NodeId {
    let id = self.state.alloc_node_id();
    let mut in_arr = [NodeId::INVALID; 3];
    let count = args.len().min(3) as u8;
    for (i, &arg) in args.iter().take(3).enumerate() {
      in_arr[i] = arg;
    }
    self.state.nodes.push(NodeRecord {
      id,
      op: NodeOp::CallBuiltin(builtin_id, kind),
      inputs: in_arr,
      input_count: count,
    });
    id
  }

  #[inline]
  pub fn call_stub(&mut self, descriptor: &CallInterfaceDescriptor, args: &[NodeId]) -> NodeId {
    let id = self.state.alloc_node_id();
    let mut in_arr = [NodeId::INVALID; 3];
    let count = args.len().min(3) as u8;
    for (i, &arg) in args.iter().take(3).enumerate() {
      in_arr[i] = arg;
    }
    self.state.nodes.push(NodeRecord {
      id,
      op: NodeOp::CallRuntime(descriptor.param_count() as u32),
      inputs: in_arr,
      input_count: count,
    });
    id
  }

  #[inline]
  #[must_use]
  pub fn node_count(&self) -> usize {
    self.state.node_count()
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(CompareBinaryOp::COUNT == 31);
  assert!(BinaryOp::COUNT == 65);
  assert!(UnaryOp::COUNT == 71);
};
