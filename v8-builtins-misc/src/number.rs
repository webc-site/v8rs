// 由 tool/src/builtins/builtins-misc.js 生成; 来源 v8/src/builtins
// 内建名表 + 核心调度元数据 (30 内建)

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// 内建名表 (共 30 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  AddLhsIsStringConstantInternalizeTrampoline,
  AddLhsIsStringConstantInternalizeWithVector,
  AddRhsIsStringConstantInternalizeTrampoline,
  AddRhsIsStringConstantInternalizeWithVector,
  #[strum(serialize = "Add_String_Baseline")]
  AddStringBaseline,
  BigIntAsIntN,
  BigIntAsUintN,
  BigIntConstructor,
  BigIntPrototypeToLocaleString,
  BigIntPrototypeToString,
  BigIntPrototypeValueOf,
  BigIntToI32Pair,
  BigIntToI64,
  #[strum(serialize = "Equal_Any_Baseline")]
  EqualAnyBaseline,
  #[strum(serialize = "Equal_Generic_Baseline")]
  EqualGenericBaseline,
  #[strum(serialize = "Equal_WithEmbeddedFeedback")]
  EqualWithEmbeddedFeedback,
  ExponentiateAndTryPatchCode,
  #[strum(serialize = "Exponentiate_None_Baseline")]
  ExponentiateNoneBaseline,
  #[strum(serialize = "Exponentiate_Number_Baseline")]
  ExponentiateNumberBaseline,
  I32PairToBigInt,
  I64ToBigInt,
  MathSumPrecise,
  #[strum(serialize = "Negate_Number_Baseline")]
  NegateNumberBaseline,
  NumberPrototypeToExponential,
  NumberPrototypeToFixed,
  NumberPrototypeToLocaleString,
  NumberPrototypeToPrecision,
  #[strum(serialize = "StrictEqual_Any_Baseline")]
  StrictEqualAnyBaseline,
  #[strum(serialize = "StrictEqual_Generic_Baseline")]
  StrictEqualGenericBaseline,
  #[strum(serialize = "StrictEqual_WithEmbeddedFeedback")]
  StrictEqualWithEmbeddedFeedback,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::AddLhsIsStringConstantInternalizeTrampoline => {
        "AddLhsIsStringConstantInternalizeTrampoline"
      }
      Self::AddLhsIsStringConstantInternalizeWithVector => {
        "AddLhsIsStringConstantInternalizeWithVector"
      }
      Self::AddRhsIsStringConstantInternalizeTrampoline => {
        "AddRhsIsStringConstantInternalizeTrampoline"
      }
      Self::AddRhsIsStringConstantInternalizeWithVector => {
        "AddRhsIsStringConstantInternalizeWithVector"
      }
      Self::AddStringBaseline => "Add_String_Baseline",
      Self::BigIntAsIntN => "BigIntAsIntN",
      Self::BigIntAsUintN => "BigIntAsUintN",
      Self::BigIntConstructor => "BigIntConstructor",
      Self::BigIntPrototypeToLocaleString => "BigIntPrototypeToLocaleString",
      Self::BigIntPrototypeToString => "BigIntPrototypeToString",
      Self::BigIntPrototypeValueOf => "BigIntPrototypeValueOf",
      Self::BigIntToI32Pair => "BigIntToI32Pair",
      Self::BigIntToI64 => "BigIntToI64",
      Self::EqualAnyBaseline => "Equal_Any_Baseline",
      Self::EqualGenericBaseline => "Equal_Generic_Baseline",
      Self::EqualWithEmbeddedFeedback => "Equal_WithEmbeddedFeedback",
      Self::ExponentiateAndTryPatchCode => "ExponentiateAndTryPatchCode",
      Self::ExponentiateNoneBaseline => "Exponentiate_None_Baseline",
      Self::ExponentiateNumberBaseline => "Exponentiate_Number_Baseline",
      Self::I32PairToBigInt => "I32PairToBigInt",
      Self::I64ToBigInt => "I64ToBigInt",
      Self::MathSumPrecise => "MathSumPrecise",
      Self::NegateNumberBaseline => "Negate_Number_Baseline",
      Self::NumberPrototypeToExponential => "NumberPrototypeToExponential",
      Self::NumberPrototypeToFixed => "NumberPrototypeToFixed",
      Self::NumberPrototypeToLocaleString => "NumberPrototypeToLocaleString",
      Self::NumberPrototypeToPrecision => "NumberPrototypeToPrecision",
      Self::StrictEqualAnyBaseline => "StrictEqual_Any_Baseline",
      Self::StrictEqualGenericBaseline => "StrictEqual_Generic_Baseline",
      Self::StrictEqualWithEmbeddedFeedback => "StrictEqual_WithEmbeddedFeedback",
    }
  }

  /// 关联至全局注册表 `BUILTIN_TABLE` 获取全局内建 ID
  #[inline]
  pub fn global_id(&self) -> Option<usize> {
    find_builtin(self.name())
  }

  /// 获取对应的接口描述符符号
  #[inline]
  pub fn descriptor_name(&self) -> String {
    descriptor_name(self.name())
  }

  /// 是否为 BigInt 相关内建
  #[inline]
  pub const fn is_bigint(&self) -> bool {
    matches!(
      self,
      Self::BigIntAsIntN
        | Self::BigIntAsUintN
        | Self::BigIntConstructor
        | Self::BigIntPrototypeToLocaleString
        | Self::BigIntPrototypeToString
        | Self::BigIntPrototypeValueOf
        | Self::BigIntToI32Pair
        | Self::BigIntToI64
        | Self::I32PairToBigInt
        | Self::I64ToBigInt
    )
  }

  /// 是否为 Number 原型链方法
  #[inline]
  pub const fn is_number_prototype(&self) -> bool {
    matches!(
      self,
      Self::NumberPrototypeToExponential
        | Self::NumberPrototypeToFixed
        | Self::NumberPrototypeToLocaleString
        | Self::NumberPrototypeToPrecision
    )
  }

  /// 是否为 Math 数学内建
  #[inline]
  pub const fn is_math(&self) -> bool {
    matches!(self, Self::MathSumPrecise)
  }

  /// 是否为 Baseline 或包含反馈内建
  #[inline]
  pub const fn is_baseline_or_feedback(&self) -> bool {
    matches!(
      self,
      Self::AddStringBaseline
        | Self::EqualAnyBaseline
        | Self::EqualGenericBaseline
        | Self::EqualWithEmbeddedFeedback
        | Self::ExponentiateNoneBaseline
        | Self::ExponentiateNumberBaseline
        | Self::NegateNumberBaseline
        | Self::StrictEqualAnyBaseline
        | Self::StrictEqualGenericBaseline
        | Self::StrictEqualWithEmbeddedFeedback
    )
  }

  /// 是否为字符串内化蹦床内建
  #[inline]
  pub const fn is_trampoline(&self) -> bool {
    matches!(
      self,
      Self::AddLhsIsStringConstantInternalizeTrampoline
        | Self::AddRhsIsStringConstantInternalizeTrampoline
    )
  }
}

// 统一提供 30 项零成本内建工厂函数
pub const fn add_lhs_is_string_constant_internalize_trampoline() -> Builtin {
  Builtin::AddLhsIsStringConstantInternalizeTrampoline
}

pub const fn add_lhs_is_string_constant_internalize_with_vector() -> Builtin {
  Builtin::AddLhsIsStringConstantInternalizeWithVector
}

pub const fn add_rhs_is_string_constant_internalize_trampoline() -> Builtin {
  Builtin::AddRhsIsStringConstantInternalizeTrampoline
}

pub const fn add_rhs_is_string_constant_internalize_with_vector() -> Builtin {
  Builtin::AddRhsIsStringConstantInternalizeWithVector
}

pub const fn add_string_baseline() -> Builtin {
  Builtin::AddStringBaseline
}

pub const fn big_int_as_int_n() -> Builtin {
  Builtin::BigIntAsIntN
}

pub const fn big_int_as_uint_n() -> Builtin {
  Builtin::BigIntAsUintN
}

pub const fn big_int_constructor() -> Builtin {
  Builtin::BigIntConstructor
}

pub const fn big_int_prototype_to_locale_string() -> Builtin {
  Builtin::BigIntPrototypeToLocaleString
}

pub const fn big_int_prototype_to_string() -> Builtin {
  Builtin::BigIntPrototypeToString
}

pub const fn big_int_prototype_value_of() -> Builtin {
  Builtin::BigIntPrototypeValueOf
}

pub const fn big_int_to_i32_pair() -> Builtin {
  Builtin::BigIntToI32Pair
}

pub const fn big_int_to_i64() -> Builtin {
  Builtin::BigIntToI64
}

pub const fn equal_any_baseline() -> Builtin {
  Builtin::EqualAnyBaseline
}

pub const fn equal_generic_baseline() -> Builtin {
  Builtin::EqualGenericBaseline
}

pub const fn equal_with_embedded_feedback() -> Builtin {
  Builtin::EqualWithEmbeddedFeedback
}

pub const fn exponentiate_and_try_patch_code() -> Builtin {
  Builtin::ExponentiateAndTryPatchCode
}

pub const fn exponentiate_none_baseline() -> Builtin {
  Builtin::ExponentiateNoneBaseline
}

pub const fn exponentiate_number_baseline() -> Builtin {
  Builtin::ExponentiateNumberBaseline
}

pub const fn i32_pair_to_big_int() -> Builtin {
  Builtin::I32PairToBigInt
}

pub const fn i64_to_big_int() -> Builtin {
  Builtin::I64ToBigInt
}

pub const fn math_sum_precise() -> Builtin {
  Builtin::MathSumPrecise
}

pub const fn negate_number_baseline() -> Builtin {
  Builtin::NegateNumberBaseline
}

pub const fn number_prototype_to_exponential() -> Builtin {
  Builtin::NumberPrototypeToExponential
}

pub const fn number_prototype_to_fixed() -> Builtin {
  Builtin::NumberPrototypeToFixed
}

pub const fn number_prototype_to_locale_string() -> Builtin {
  Builtin::NumberPrototypeToLocaleString
}

pub const fn number_prototype_to_precision() -> Builtin {
  Builtin::NumberPrototypeToPrecision
}

pub const fn strict_equal_any_baseline() -> Builtin {
  Builtin::StrictEqualAnyBaseline
}

pub const fn strict_equal_generic_baseline() -> Builtin {
  Builtin::StrictEqualGenericBaseline
}

pub const fn strict_equal_with_embedded_feedback() -> Builtin {
  Builtin::StrictEqualWithEmbeddedFeedback
}

/// 来自 number 相关 .tq 的声明元数据 (共 159 条)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("extern macro", "NaNStringConstant"),
  ("extern macro", "ZeroStringConstant"),
  ("extern macro", "InfinityStringConstant"),
  ("extern macro", "MinusInfinityStringConstant"),
  ("extern macro", "Log10OffsetTable"),
  ("extern macro", "TryMatchPreallocatedNumberString"),
  ("macro", "ToCharCode"),
  ("macro", "IntToDecimalStringImpl"),
  ("macro", "IntToDecimalString"),
  ("macro", "IntToString"),
  ("javascript builtin", "NumberIsFinite"),
  ("javascript builtin", "NumberIsInteger"),
  ("javascript builtin", "NumberIsNaN"),
  ("javascript builtin", "NumberIsSafeInteger"),
  ("extern macro", "TruncateFloat64ToWord32"),
  ("extern macro", "BitwiseOp"),
  ("extern macro", "RelationalComparison"),
  ("extern macro", "TruncateNumberToWord32"),
  ("macro", "UnaryOp1"),
  ("macro", "UnaryOp2"),
  ("macro", "BinaryOp1"),
  ("macro", "BinaryOp2"),
  ("builtin", "Subtract"),
  ("builtin", "Multiply"),
  ("builtin", "Divide"),
  ("builtin", "Modulus"),
  ("builtin", "Exponentiate"),
  ("builtin", "Negate"),
  ("builtin", "BitwiseNot"),
  ("builtin", "Decrement"),
  ("builtin", "Increment"),
  ("extern macro", "BinaryOpAssembler"),
  ("builtin", "ShiftLeft"),
  ("builtin", "ShiftRight"),
  ("builtin", "ShiftRightLogical"),
  ("builtin", "BitwiseAnd"),
  ("builtin", "BitwiseOr"),
  ("builtin", "BitwiseXor"),
  ("builtin", "LessThan"),
  ("builtin", "LessThanOrEqual"),
  ("builtin", "GreaterThan"),
  ("builtin", "GreaterThanOrEqual"),
  ("builtin", "Equal"),
  ("builtin", "StrictEqual"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "BigIntBuiltinsAssembler"),
  ("extern macro", "CodeStubAssembler"),
  ("extern macro", "CodeStubAssembler"),
  ("extern macro", "CodeStubAssembler"),
  ("extern macro", "CodeStubAssembler"),
  ("macro", "IsCanonicalized"),
  ("macro", "InvertSign"),
  ("macro", "AllocateEmptyBigIntNoThrow"),
  ("macro", "AllocateEmptyBigInt"),
  ("macro", "BigIntCompareAbsolute"),
  ("macro", "MutableBigIntAbsoluteSub"),
  ("macro", "MutableBigIntAbsoluteAdd"),
  ("macro", "BigIntAddImpl"),
  ("builtin", "BigIntAddNoThrow"),
  ("builtin", "BigIntAdd"),
  ("macro", "BigIntSubtractImpl"),
  ("builtin", "BigIntSubtractNoThrow"),
  ("builtin", "BigIntSubtract"),
  ("macro", "BigIntMultiplyImpl"),
  ("builtin", "BigIntMultiplyNoThrow"),
  ("builtin", "BigIntMultiply"),
  ("macro", "BigIntDivideImpl"),
  ("builtin", "BigIntDivideNoThrow"),
  ("builtin", "BigIntDivide"),
  ("macro", "BigIntModulusImpl"),
  ("builtin", "BigIntModulusNoThrow"),
  ("builtin", "BigIntModulus"),
  ("macro", "BigIntBitwiseAndImpl"),
  ("builtin", "BigIntBitwiseAndNoThrow"),
  ("builtin", "BigIntBitwiseAnd"),
  ("macro", "BigIntBitwiseOrImpl"),
  ("builtin", "BigIntBitwiseOrNoThrow"),
  ("builtin", "BigIntBitwiseOr"),
  ("macro", "BigIntBitwiseXorImpl"),
  ("builtin", "BigIntBitwiseXorNoThrow"),
  ("builtin", "BigIntBitwiseXor"),
  ("macro", "MutableBigIntLeftShiftByAbsolute"),
  ("macro", "RightShiftByMaximum"),
  ("macro", "MutableBigIntRightShiftByAbsolute"),
  ("macro", "BigIntShiftLeftImpl"),
  ("macro", "BigIntShiftRightImpl"),
  ("builtin", "BigIntShiftLeftNoThrow"),
  ("builtin", "BigIntShiftLeft"),
  ("builtin", "BigIntShiftRightNoThrow"),
  ("builtin", "BigIntShiftRight"),
  ("builtin", "BigIntEqual"),
  ("macro", "BigIntCompare"),
  ("builtin", "BigIntLessThan"),
  ("builtin", "BigIntGreaterThan"),
  ("builtin", "BigIntLessThanOrEqual"),
  ("builtin", "BigIntGreaterThanOrEqual"),
  ("builtin", "BigIntUnaryMinus"),
  ("extern macro", "IsIntPtrAbsWithOverflowSupported"),
  ("extern macro", "TrySmiAdd"),
  ("extern macro", "TrySmiSub"),
  ("extern macro", "TrySmiAbs"),
  ("extern macro", "Float64Abs"),
  ("extern macro", "Float64Ceil"),
  ("extern macro", "Float64Floor"),
  ("extern macro", "Float64Round"),
  ("extern macro", "Float64Trunc"),
  ("extern macro", "Float64Pow"),
  ("extern macro", "TruncateTaggedToFloat64"),
  ("macro", "MathPowImpl"),
  ("extern macro", "Float64Max"),
  ("extern macro", "Float64Min"),
  ("extern macro", "Float64Acos"),
  ("extern macro", "Float64Acosh"),
  ("extern macro", "Float64Asin"),
  ("extern macro", "Float64Asinh"),
  ("extern macro", "Float64Atan"),
  ("extern macro", "Float64Atan2"),
  ("extern macro", "Float64Atanh"),
  ("extern macro", "Float64Cbrt"),
  ("extern macro", "Word32Clz"),
  ("extern macro", "Float64Cos"),
  ("extern macro", "Float64Cosh"),
  ("extern macro", "Float64Exp"),
  ("extern macro", "Float64Expm1"),
  ("extern macro", "Float64Log"),
  ("extern macro", "Float64Log1p"),
  ("extern macro", "Float64Log10"),
  ("extern macro", "Float64Log2"),
  ("extern macro", "Float64Sin"),
  ("extern macro", "Float64Sinh"),
  ("extern macro", "Float64Sqrt"),
  ("extern macro", "Float64Tan"),
  ("extern macro", "Float64Tanh"),
  ("struct", "XorShift128State"),
  ("extern macro", "LoadMathRandomState"),
  ("extern macro", "StoreMathRandomState"),
  ("extern macro", "InitializeAndMaybeRefillMathRandom"),
  ("struct", "XorShift128Result"),
  ("macro", "RandomToDouble"),
  ("macro", "XorShift128"),
  ("javascript builtin", "BooleanConstructor"),
];
