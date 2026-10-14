// 由 tool/src/compiler/simplified-operator.h.js 从 v8/src/compiler/simplified-operator.h + simplified-operator.cc 生成; 修订就地改, 复跑覆盖

//! SimplifiedOperatorBuilder 与 Simplified 算子族
//!
//! 对应原版 simplified-operator.h / simplified-operator.cc。
//! 包含 NumberAdd, NumberSubtract, CheckBounds, CheckMaps 等投机优化算子。

use core::result::Result;
use std::sync::LazyLock;

use v8_compiler_graph::{
  CheckForMinusZeroMode, FeedbackSource, IrOpcode, Operator, Operator1, OperatorCounts, Properties,
};
use v8_zone::Zone;

use crate::{
  Error,
  common_operator::{DeoptimizeParameters, DeoptimizeReason},
};

/// CheckBounds 标志位 (CheckBoundsFlags)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CheckBoundsFlags(u8);

impl CheckBoundsFlags {
  pub const NONE: Self = Self(0);
  pub const CONVERT_STRING_AND_MINUS_ZERO: Self = Self(1 << 0);
  pub const ABORT_ON_OUT_OF_BOUNDS: Self = Self(1 << 1);

  pub const fn bits(self) -> u8 {
    self.0
  }
}

/// CheckBounds 参数 (CheckBoundsParameters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckBoundsParameters {
  pub feedback: FeedbackSource,
  pub flags: CheckBoundsFlags,
}

impl CheckBoundsParameters {
  pub const fn new(feedback: FeedbackSource, flags: CheckBoundsFlags) -> Self {
    Self { feedback, flags }
  }
}

/// CheckMaps 标志位 (CheckMapsFlags)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CheckMapsFlags(u8);

impl CheckMapsFlags {
  pub const NONE: Self = Self(0);
  pub const CAN_BE_CLEARED: Self = Self(1 << 0);
  pub const TRY_MIGRATE_INSTANCE: Self = Self(1 << 1);

  pub const fn bits(self) -> u8 {
    self.0
  }
}

/// CheckMaps 参数 (CheckMapsParameters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckMapsParameters {
  pub flags: CheckMapsFlags,
  pub feedback: FeedbackSource,
}

impl CheckMapsParameters {
  pub const fn new(flags: CheckMapsFlags, feedback: FeedbackSource) -> Self {
    Self { flags, feedback }
  }
}

/// 全局共享 Simplified 算子缓存
pub struct SimplifiedOperatorGlobalCache {
  pub boolean_not: Operator,
  pub number_equal: Operator,
  pub number_less_than: Operator,
  pub number_less_than_or_equal: Operator,
  pub number_add: Operator,
  pub number_subtract: Operator,
  pub number_multiply: Operator,
  pub number_divide: Operator,
  pub number_modulus: Operator,
  pub number_bitwise_or: Operator,
  pub number_bitwise_xor: Operator,
  pub number_bitwise_and: Operator,
  pub number_shift_left: Operator,
  pub number_shift_right: Operator,
  pub number_shift_right_logical: Operator,
  pub number_imul: Operator,
  pub number_abs: Operator,
  pub number_clz32: Operator,
  pub number_ceil: Operator,
  pub number_floor: Operator,
  pub number_fround: Operator,
  pub number_acos: Operator,
  pub number_acosh: Operator,
  pub number_asin: Operator,
  pub number_asinh: Operator,
  pub number_atan: Operator,
  pub number_atan2: Operator,
  pub number_atanh: Operator,
  pub number_cbrt: Operator,
  pub number_cos: Operator,
  pub number_cosh: Operator,
  pub number_exp: Operator,
  pub number_expm1: Operator,
  pub number_log: Operator,
  pub number_log1p: Operator,
  pub number_log10: Operator,
  pub number_log2: Operator,
  pub number_max: Operator,
  pub number_min: Operator,
  pub number_pow: Operator,
  pub number_round: Operator,
  pub number_sign: Operator,
  pub number_sin: Operator,
  pub number_sinh: Operator,
  pub number_sqrt: Operator,
  pub number_tan: Operator,
  pub number_tanh: Operator,
  pub number_trunc: Operator,
  pub number_to_boolean: Operator,
  pub number_to_int32: Operator,
  pub number_to_string: Operator,
  pub number_to_uint32: Operator,
  pub number_to_uint8_clamped: Operator,
  pub integral32_or_minus_zero_to_big_int: Operator,
  pub big_int_equal: Operator,
  pub big_int_less_than: Operator,
  pub big_int_less_than_or_equal: Operator,
  pub big_int_negate: Operator,
  pub string_concat: Operator,
  pub string_to_number: Operator,
  pub string_from_single_char_code: Operator,
  pub string_from_single_code_point: Operator,
  pub string_index_of: Operator,
  pub string_length: Operator,
  pub string_wrapper_length: Operator,
  pub type_of: Operator,
  pub plain_primitive_to_number: Operator,
  pub plain_primitive_to_word32: Operator,
  pub plain_primitive_to_float64: Operator,
  pub change_tagged_signed_to_int32: Operator,
  pub change_tagged_signed_to_int64: Operator,
  pub change_tagged_to_int32: Operator,
  pub change_tagged_to_int64: Operator,
  pub change_tagged_to_uint32: Operator,
  pub change_tagged_to_float64: Operator,
  pub change_tagged_to_tagged_signed: Operator,
  pub change_smi_or_hole_to_float64: Operator,
  pub change_number_or_hole_to_float64: Operator,
  pub change_float64_to_tagged_pointer: Operator,
  pub change_float64_hole_to_tagged: Operator,
  pub change_float64_or_undefined_or_hole_to_tagged: Operator,
  pub change_int31_to_tagged_signed: Operator,
  pub change_int32_to_tagged: Operator,
  pub change_int64_to_tagged: Operator,
  pub change_uint32_to_tagged: Operator,
  pub change_uint64_to_tagged: Operator,
  pub change_tagged_to_bit: Operator,
  pub change_bit_to_tagged: Operator,
  pub truncate_big_int_to_word64: Operator,
  pub change_int64_to_big_int: Operator,
  pub change_uint64_to_big_int: Operator,
  pub truncate_tagged_to_bit: Operator,
  pub truncate_tagged_pointer_to_bit: Operator,
  pub truncate_number_or_oddball_to_word32: Operator,
  pub truncate_smi_or_hole_to_word32: Operator,
  pub truncate_number_or_oddball_or_hole_to_word32: Operator,
  pub truncate_tagged_to_float64: Operator,
  pub truncate_tagged_to_float64_preserve_undefined: Operator,
  pub object_is_array_buffer_view: Operator,
  pub object_is_big_int: Operator,
  pub object_is_callable: Operator,
  pub object_is_constructor: Operator,
  pub object_is_detectable_callable: Operator,
  pub object_is_minus_zero: Operator,
  pub number_is_minus_zero: Operator,
  pub object_is_na_n: Operator,
  pub number_is_na_n: Operator,
  pub object_is_non_callable: Operator,
  pub object_is_number: Operator,
  pub object_is_receiver: Operator,
  pub object_is_smi: Operator,
  pub object_is_string: Operator,
  pub object_is_symbol: Operator,
  pub object_is_undetectable: Operator,
  pub number_is_float64_hole: Operator,
  pub number_is_finite: Operator,
  pub object_is_finite_number: Operator,
  pub number_is_integer: Operator,
  pub object_is_safe_integer: Operator,
  pub number_is_safe_integer: Operator,
  pub object_is_integer: Operator,
  pub convert_tagged_hole_to_undefined: Operator,
  pub same_value: Operator,
  pub same_value_numbers_only: Operator,
  pub number_same_value: Operator,
  pub reference_equal: Operator,
  pub string_equal: Operator,
  pub string_less_than: Operator,
  pub string_less_than_or_equal: Operator,
  pub string_or_oddball_strict_equal: Operator,
  pub to_boolean: Operator,
  pub new_cons_string: Operator,
  pub unsigned32_divide: Operator,
  pub check_equals_internalized_string: Operator,
  pub check_equals_symbol: Operator,
  pub check_heap_object: Operator,
  pub check_internalized_string: Operator,
  pub check_not_tagged_hole: Operator,
  pub check_receiver: Operator,
  pub check_receiver_or_null_or_undefined: Operator,
  pub check_symbol: Operator,
  pub checked_int32_add: Operator,
  pub checked_int32_div: Operator,
  pub checked_int32_mod: Operator,
  pub checked_int32_sub: Operator,
  pub checked_uint32_div: Operator,
  pub checked_uint32_mod: Operator,
  pub checked_additive_safe_integer_add: Operator,
  pub checked_additive_safe_integer_sub: Operator,
  pub checked_int64_add: Operator,
  pub checked_int64_sub: Operator,
  pub checked_int64_mul: Operator,
  pub checked_int64_div: Operator,
  pub checked_int64_mod: Operator,
  pub check_bounds: Operator1<CheckBoundsParameters>,
  pub check_maps: Operator1<CheckMapsParameters>,
}

#[inline]
fn unwrap_op<T>(res: Result<T, v8_compiler_graph::Error>) -> T {
  match res {
    Ok(v) => v,
    Err(_) => unreachable!(),
  }
}

impl SimplifiedOperatorGlobalCache {
  fn new() -> Self {
    Self {
      boolean_not: unwrap_op(Operator::new(
        IrOpcode::BooleanNot as u16,
        Properties::PURE,
        "BooleanNot",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_equal: unwrap_op(Operator::new(
        IrOpcode::NumberEqual as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "NumberEqual",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_less_than: unwrap_op(Operator::new(
        IrOpcode::NumberLessThan as u16,
        Properties::PURE,
        "NumberLessThan",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_less_than_or_equal: unwrap_op(Operator::new(
        IrOpcode::NumberLessThanOrEqual as u16,
        Properties::PURE,
        "NumberLessThanOrEqual",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_add: unwrap_op(Operator::new(
        IrOpcode::NumberAdd as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "NumberAdd",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_subtract: unwrap_op(Operator::new(
        IrOpcode::NumberSubtract as u16,
        Properties::PURE,
        "NumberSubtract",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_multiply: unwrap_op(Operator::new(
        IrOpcode::NumberMultiply as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "NumberMultiply",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_divide: unwrap_op(Operator::new(
        IrOpcode::NumberDivide as u16,
        Properties::PURE,
        "NumberDivide",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_modulus: unwrap_op(Operator::new(
        IrOpcode::NumberModulus as u16,
        Properties::PURE,
        "NumberModulus",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_bitwise_or: unwrap_op(Operator::new(
        IrOpcode::NumberBitwiseOr as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "NumberBitwiseOr",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_bitwise_xor: unwrap_op(Operator::new(
        IrOpcode::NumberBitwiseXor as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "NumberBitwiseXor",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_bitwise_and: unwrap_op(Operator::new(
        IrOpcode::NumberBitwiseAnd as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "NumberBitwiseAnd",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_shift_left: unwrap_op(Operator::new(
        IrOpcode::NumberShiftLeft as u16,
        Properties::PURE,
        "NumberShiftLeft",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_shift_right: unwrap_op(Operator::new(
        IrOpcode::NumberShiftRight as u16,
        Properties::PURE,
        "NumberShiftRight",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_shift_right_logical: unwrap_op(Operator::new(
        IrOpcode::NumberShiftRightLogical as u16,
        Properties::PURE,
        "NumberShiftRightLogical",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_imul: unwrap_op(Operator::new(
        IrOpcode::NumberImul as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "NumberImul",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_abs: unwrap_op(Operator::new(
        IrOpcode::NumberAbs as u16,
        Properties::PURE,
        "NumberAbs",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_clz32: unwrap_op(Operator::new(
        IrOpcode::NumberClz32 as u16,
        Properties::PURE,
        "NumberClz32",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_ceil: unwrap_op(Operator::new(
        IrOpcode::NumberCeil as u16,
        Properties::PURE,
        "NumberCeil",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_floor: unwrap_op(Operator::new(
        IrOpcode::NumberFloor as u16,
        Properties::PURE,
        "NumberFloor",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_fround: unwrap_op(Operator::new(
        IrOpcode::NumberFround as u16,
        Properties::PURE,
        "NumberFround",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_acos: unwrap_op(Operator::new(
        IrOpcode::NumberAcos as u16,
        Properties::PURE,
        "NumberAcos",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_acosh: unwrap_op(Operator::new(
        IrOpcode::NumberAcosh as u16,
        Properties::PURE,
        "NumberAcosh",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_asin: unwrap_op(Operator::new(
        IrOpcode::NumberAsin as u16,
        Properties::PURE,
        "NumberAsin",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_asinh: unwrap_op(Operator::new(
        IrOpcode::NumberAsinh as u16,
        Properties::PURE,
        "NumberAsinh",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_atan: unwrap_op(Operator::new(
        IrOpcode::NumberAtan as u16,
        Properties::PURE,
        "NumberAtan",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_atan2: unwrap_op(Operator::new(
        IrOpcode::NumberAtan2 as u16,
        Properties::PURE,
        "NumberAtan2",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_atanh: unwrap_op(Operator::new(
        IrOpcode::NumberAtanh as u16,
        Properties::PURE,
        "NumberAtanh",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_cbrt: unwrap_op(Operator::new(
        IrOpcode::NumberCbrt as u16,
        Properties::PURE,
        "NumberCbrt",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_cos: unwrap_op(Operator::new(
        IrOpcode::NumberCos as u16,
        Properties::PURE,
        "NumberCos",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_cosh: unwrap_op(Operator::new(
        IrOpcode::NumberCosh as u16,
        Properties::PURE,
        "NumberCosh",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_exp: unwrap_op(Operator::new(
        IrOpcode::NumberExp as u16,
        Properties::PURE,
        "NumberExp",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_expm1: unwrap_op(Operator::new(
        IrOpcode::NumberExpm1 as u16,
        Properties::PURE,
        "NumberExpm1",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_log: unwrap_op(Operator::new(
        IrOpcode::NumberLog as u16,
        Properties::PURE,
        "NumberLog",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_log1p: unwrap_op(Operator::new(
        IrOpcode::NumberLog1p as u16,
        Properties::PURE,
        "NumberLog1p",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_log10: unwrap_op(Operator::new(
        IrOpcode::NumberLog10 as u16,
        Properties::PURE,
        "NumberLog10",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_log2: unwrap_op(Operator::new(
        IrOpcode::NumberLog2 as u16,
        Properties::PURE,
        "NumberLog2",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_max: unwrap_op(Operator::new(
        IrOpcode::NumberMax as u16,
        Properties::PURE,
        "NumberMax",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_min: unwrap_op(Operator::new(
        IrOpcode::NumberMin as u16,
        Properties::PURE,
        "NumberMin",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_pow: unwrap_op(Operator::new(
        IrOpcode::NumberPow as u16,
        Properties::PURE,
        "NumberPow",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_round: unwrap_op(Operator::new(
        IrOpcode::NumberRound as u16,
        Properties::PURE,
        "NumberRound",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_sign: unwrap_op(Operator::new(
        IrOpcode::NumberSign as u16,
        Properties::PURE,
        "NumberSign",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_sin: unwrap_op(Operator::new(
        IrOpcode::NumberSin as u16,
        Properties::PURE,
        "NumberSin",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_sinh: unwrap_op(Operator::new(
        IrOpcode::NumberSinh as u16,
        Properties::PURE,
        "NumberSinh",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_sqrt: unwrap_op(Operator::new(
        IrOpcode::NumberSqrt as u16,
        Properties::PURE,
        "NumberSqrt",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_tan: unwrap_op(Operator::new(
        IrOpcode::NumberTan as u16,
        Properties::PURE,
        "NumberTan",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_tanh: unwrap_op(Operator::new(
        IrOpcode::NumberTanh as u16,
        Properties::PURE,
        "NumberTanh",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_trunc: unwrap_op(Operator::new(
        IrOpcode::NumberTrunc as u16,
        Properties::PURE,
        "NumberTrunc",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_to_boolean: unwrap_op(Operator::new(
        IrOpcode::NumberToBoolean as u16,
        Properties::PURE,
        "NumberToBoolean",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_to_int32: unwrap_op(Operator::new(
        IrOpcode::NumberToInt32 as u16,
        Properties::PURE,
        "NumberToInt32",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_to_string: unwrap_op(Operator::new(
        IrOpcode::NumberToString as u16,
        Properties::PURE,
        "NumberToString",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_to_uint32: unwrap_op(Operator::new(
        IrOpcode::NumberToUint32 as u16,
        Properties::PURE,
        "NumberToUint32",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_to_uint8_clamped: unwrap_op(Operator::new(
        IrOpcode::NumberToUint8Clamped as u16,
        Properties::PURE,
        "NumberToUint8Clamped",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      integral32_or_minus_zero_to_big_int: unwrap_op(Operator::new(
        IrOpcode::Integral32OrMinusZeroToBigInt as u16,
        Properties::PURE,
        "Integral32OrMinusZeroToBigInt",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      big_int_equal: unwrap_op(Operator::new(
        IrOpcode::BigIntEqual as u16,
        Properties::PURE,
        "BigIntEqual",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      big_int_less_than: unwrap_op(Operator::new(
        IrOpcode::BigIntLessThan as u16,
        Properties::PURE,
        "BigIntLessThan",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      big_int_less_than_or_equal: unwrap_op(Operator::new(
        IrOpcode::BigIntLessThanOrEqual as u16,
        Properties::PURE,
        "BigIntLessThanOrEqual",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      big_int_negate: unwrap_op(Operator::new(
        IrOpcode::BigIntNegate as u16,
        Properties::PURE,
        "BigIntNegate",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      string_concat: unwrap_op(Operator::new(
        IrOpcode::StringConcat as u16,
        Properties::PURE,
        "StringConcat",
        OperatorCounts::new(3, 0, 0, 1, 0, 0),
      )),
      string_to_number: unwrap_op(Operator::new(
        IrOpcode::StringToNumber as u16,
        Properties::PURE,
        "StringToNumber",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      string_from_single_char_code: unwrap_op(Operator::new(
        IrOpcode::StringFromSingleCharCode as u16,
        Properties::PURE,
        "StringFromSingleCharCode",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      string_from_single_code_point: unwrap_op(Operator::new(
        IrOpcode::StringFromSingleCodePoint as u16,
        Properties::PURE,
        "StringFromSingleCodePoint",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      string_index_of: unwrap_op(Operator::new(
        IrOpcode::StringIndexOf as u16,
        Properties::PURE,
        "StringIndexOf",
        OperatorCounts::new(3, 0, 0, 1, 0, 0),
      )),
      string_length: unwrap_op(Operator::new(
        IrOpcode::StringLength as u16,
        Properties::PURE,
        "StringLength",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      string_wrapper_length: unwrap_op(Operator::new(
        IrOpcode::StringWrapperLength as u16,
        Properties::PURE,
        "StringWrapperLength",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      type_of: unwrap_op(Operator::new(
        IrOpcode::TypeOf as u16,
        Properties::PURE,
        "TypeOf",
        OperatorCounts::new(1, 0, 1, 1, 0, 0),
      )),
      plain_primitive_to_number: unwrap_op(Operator::new(
        IrOpcode::PlainPrimitiveToNumber as u16,
        Properties::PURE,
        "PlainPrimitiveToNumber",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      plain_primitive_to_word32: unwrap_op(Operator::new(
        IrOpcode::PlainPrimitiveToWord32 as u16,
        Properties::PURE,
        "PlainPrimitiveToWord32",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      plain_primitive_to_float64: unwrap_op(Operator::new(
        IrOpcode::PlainPrimitiveToFloat64 as u16,
        Properties::PURE,
        "PlainPrimitiveToFloat64",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_tagged_signed_to_int32: unwrap_op(Operator::new(
        IrOpcode::ChangeTaggedSignedToInt32 as u16,
        Properties::PURE,
        "ChangeTaggedSignedToInt32",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_tagged_signed_to_int64: unwrap_op(Operator::new(
        IrOpcode::ChangeTaggedSignedToInt64 as u16,
        Properties::PURE,
        "ChangeTaggedSignedToInt64",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_tagged_to_int32: unwrap_op(Operator::new(
        IrOpcode::ChangeTaggedToInt32 as u16,
        Properties::PURE,
        "ChangeTaggedToInt32",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_tagged_to_int64: unwrap_op(Operator::new(
        IrOpcode::ChangeTaggedToInt64 as u16,
        Properties::PURE,
        "ChangeTaggedToInt64",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_tagged_to_uint32: unwrap_op(Operator::new(
        IrOpcode::ChangeTaggedToUint32 as u16,
        Properties::PURE,
        "ChangeTaggedToUint32",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_tagged_to_float64: unwrap_op(Operator::new(
        IrOpcode::ChangeTaggedToFloat64 as u16,
        Properties::PURE,
        "ChangeTaggedToFloat64",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_tagged_to_tagged_signed: unwrap_op(Operator::new(
        IrOpcode::ChangeTaggedToTaggedSigned as u16,
        Properties::PURE,
        "ChangeTaggedToTaggedSigned",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_smi_or_hole_to_float64: unwrap_op(Operator::new(
        IrOpcode::ChangeSmiOrHoleToFloat64 as u16,
        Properties::PURE,
        "ChangeSmiOrHoleToFloat64",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_number_or_hole_to_float64: unwrap_op(Operator::new(
        IrOpcode::ChangeNumberOrHoleToFloat64 as u16,
        Properties::PURE,
        "ChangeNumberOrHoleToFloat64",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_float64_to_tagged_pointer: unwrap_op(Operator::new(
        IrOpcode::ChangeFloat64ToTaggedPointer as u16,
        Properties::PURE,
        "ChangeFloat64ToTaggedPointer",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_float64_hole_to_tagged: unwrap_op(Operator::new(
        IrOpcode::ChangeFloat64HoleToTagged as u16,
        Properties::PURE,
        "ChangeFloat64HoleToTagged",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_float64_or_undefined_or_hole_to_tagged: unwrap_op(Operator::new(
        IrOpcode::ChangeFloat64OrUndefinedOrHoleToTagged as u16,
        Properties::PURE,
        "ChangeFloat64OrUndefinedOrHoleToTagged",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_int31_to_tagged_signed: unwrap_op(Operator::new(
        IrOpcode::ChangeInt31ToTaggedSigned as u16,
        Properties::PURE,
        "ChangeInt31ToTaggedSigned",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_int32_to_tagged: unwrap_op(Operator::new(
        IrOpcode::ChangeInt32ToTagged as u16,
        Properties::PURE,
        "ChangeInt32ToTagged",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_int64_to_tagged: unwrap_op(Operator::new(
        IrOpcode::ChangeInt64ToTagged as u16,
        Properties::PURE,
        "ChangeInt64ToTagged",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_uint32_to_tagged: unwrap_op(Operator::new(
        IrOpcode::ChangeUint32ToTagged as u16,
        Properties::PURE,
        "ChangeUint32ToTagged",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_uint64_to_tagged: unwrap_op(Operator::new(
        IrOpcode::ChangeUint64ToTagged as u16,
        Properties::PURE,
        "ChangeUint64ToTagged",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_tagged_to_bit: unwrap_op(Operator::new(
        IrOpcode::ChangeTaggedToBit as u16,
        Properties::PURE,
        "ChangeTaggedToBit",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_bit_to_tagged: unwrap_op(Operator::new(
        IrOpcode::ChangeBitToTagged as u16,
        Properties::PURE,
        "ChangeBitToTagged",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      truncate_big_int_to_word64: unwrap_op(Operator::new(
        IrOpcode::TruncateBigIntToWord64 as u16,
        Properties::PURE,
        "TruncateBigIntToWord64",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_int64_to_big_int: unwrap_op(Operator::new(
        IrOpcode::ChangeInt64ToBigInt as u16,
        Properties::PURE,
        "ChangeInt64ToBigInt",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      change_uint64_to_big_int: unwrap_op(Operator::new(
        IrOpcode::ChangeUint64ToBigInt as u16,
        Properties::PURE,
        "ChangeUint64ToBigInt",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      truncate_tagged_to_bit: unwrap_op(Operator::new(
        IrOpcode::TruncateTaggedToBit as u16,
        Properties::PURE,
        "TruncateTaggedToBit",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      truncate_tagged_pointer_to_bit: unwrap_op(Operator::new(
        IrOpcode::TruncateTaggedPointerToBit as u16,
        Properties::PURE,
        "TruncateTaggedPointerToBit",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      truncate_number_or_oddball_to_word32: unwrap_op(Operator::new(
        IrOpcode::TruncateNumberOrOddballToWord32 as u16,
        Properties::PURE,
        "TruncateNumberOrOddballToWord32",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      truncate_smi_or_hole_to_word32: unwrap_op(Operator::new(
        IrOpcode::TruncateSmiOrHoleToWord32 as u16,
        Properties::PURE,
        "TruncateSmiOrHoleToWord32",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      truncate_number_or_oddball_or_hole_to_word32: unwrap_op(Operator::new(
        IrOpcode::TruncateNumberOrOddballOrHoleToWord32 as u16,
        Properties::PURE,
        "TruncateNumberOrOddballOrHoleToWord32",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      truncate_tagged_to_float64: unwrap_op(Operator::new(
        IrOpcode::TruncateTaggedToFloat64 as u16,
        Properties::PURE,
        "TruncateTaggedToFloat64",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      truncate_tagged_to_float64_preserve_undefined: unwrap_op(Operator::new(
        IrOpcode::TruncateTaggedToFloat64PreserveUndefined as u16,
        Properties::PURE,
        "TruncateTaggedToFloat64PreserveUndefined",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_array_buffer_view: unwrap_op(Operator::new(
        IrOpcode::ObjectIsArrayBufferView as u16,
        Properties::PURE,
        "ObjectIsArrayBufferView",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_big_int: unwrap_op(Operator::new(
        IrOpcode::ObjectIsBigInt as u16,
        Properties::PURE,
        "ObjectIsBigInt",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_callable: unwrap_op(Operator::new(
        IrOpcode::ObjectIsCallable as u16,
        Properties::PURE,
        "ObjectIsCallable",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_constructor: unwrap_op(Operator::new(
        IrOpcode::ObjectIsConstructor as u16,
        Properties::PURE,
        "ObjectIsConstructor",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_detectable_callable: unwrap_op(Operator::new(
        IrOpcode::ObjectIsDetectableCallable as u16,
        Properties::PURE,
        "ObjectIsDetectableCallable",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_minus_zero: unwrap_op(Operator::new(
        IrOpcode::ObjectIsMinusZero as u16,
        Properties::PURE,
        "ObjectIsMinusZero",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_is_minus_zero: unwrap_op(Operator::new(
        IrOpcode::NumberIsMinusZero as u16,
        Properties::PURE,
        "NumberIsMinusZero",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_na_n: unwrap_op(Operator::new(
        IrOpcode::ObjectIsNaN as u16,
        Properties::PURE,
        "ObjectIsNaN",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_is_na_n: unwrap_op(Operator::new(
        IrOpcode::NumberIsNaN as u16,
        Properties::PURE,
        "NumberIsNaN",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_non_callable: unwrap_op(Operator::new(
        IrOpcode::ObjectIsNonCallable as u16,
        Properties::PURE,
        "ObjectIsNonCallable",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_number: unwrap_op(Operator::new(
        IrOpcode::ObjectIsNumber as u16,
        Properties::PURE,
        "ObjectIsNumber",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_receiver: unwrap_op(Operator::new(
        IrOpcode::ObjectIsReceiver as u16,
        Properties::PURE,
        "ObjectIsReceiver",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_smi: unwrap_op(Operator::new(
        IrOpcode::ObjectIsSmi as u16,
        Properties::PURE,
        "ObjectIsSmi",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_string: unwrap_op(Operator::new(
        IrOpcode::ObjectIsString as u16,
        Properties::PURE,
        "ObjectIsString",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_symbol: unwrap_op(Operator::new(
        IrOpcode::ObjectIsSymbol as u16,
        Properties::PURE,
        "ObjectIsSymbol",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_undetectable: unwrap_op(Operator::new(
        IrOpcode::ObjectIsUndetectable as u16,
        Properties::PURE,
        "ObjectIsUndetectable",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_is_float64_hole: unwrap_op(Operator::new(
        IrOpcode::NumberIsFloat64Hole as u16,
        Properties::PURE,
        "NumberIsFloat64Hole",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_is_finite: unwrap_op(Operator::new(
        IrOpcode::NumberIsFinite as u16,
        Properties::PURE,
        "NumberIsFinite",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_finite_number: unwrap_op(Operator::new(
        IrOpcode::ObjectIsFiniteNumber as u16,
        Properties::PURE,
        "ObjectIsFiniteNumber",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_is_integer: unwrap_op(Operator::new(
        IrOpcode::NumberIsInteger as u16,
        Properties::PURE,
        "NumberIsInteger",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_safe_integer: unwrap_op(Operator::new(
        IrOpcode::ObjectIsSafeInteger as u16,
        Properties::PURE,
        "ObjectIsSafeInteger",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      number_is_safe_integer: unwrap_op(Operator::new(
        IrOpcode::NumberIsSafeInteger as u16,
        Properties::PURE,
        "NumberIsSafeInteger",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      object_is_integer: unwrap_op(Operator::new(
        IrOpcode::ObjectIsInteger as u16,
        Properties::PURE,
        "ObjectIsInteger",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      convert_tagged_hole_to_undefined: unwrap_op(Operator::new(
        IrOpcode::ConvertTaggedHoleToUndefined as u16,
        Properties::PURE,
        "ConvertTaggedHoleToUndefined",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      same_value: unwrap_op(Operator::new(
        IrOpcode::SameValue as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "SameValue",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      same_value_numbers_only: unwrap_op(Operator::new(
        IrOpcode::SameValueNumbersOnly as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "SameValueNumbersOnly",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      number_same_value: unwrap_op(Operator::new(
        IrOpcode::NumberSameValue as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "NumberSameValue",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      reference_equal: unwrap_op(Operator::new(
        IrOpcode::ReferenceEqual as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "ReferenceEqual",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      string_equal: unwrap_op(Operator::new(
        IrOpcode::StringEqual as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "StringEqual",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      string_less_than: unwrap_op(Operator::new(
        IrOpcode::StringLessThan as u16,
        Properties::PURE,
        "StringLessThan",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      string_less_than_or_equal: unwrap_op(Operator::new(
        IrOpcode::StringLessThanOrEqual as u16,
        Properties::PURE,
        "StringLessThanOrEqual",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      string_or_oddball_strict_equal: unwrap_op(Operator::new(
        IrOpcode::StringOrOddballStrictEqual as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "StringOrOddballStrictEqual",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      to_boolean: unwrap_op(Operator::new(
        IrOpcode::ToBoolean as u16,
        Properties::PURE,
        "ToBoolean",
        OperatorCounts::new(1, 0, 0, 1, 0, 0),
      )),
      new_cons_string: unwrap_op(Operator::new(
        IrOpcode::NewConsString as u16,
        Properties::PURE,
        "NewConsString",
        OperatorCounts::new(3, 0, 0, 1, 0, 0),
      )),
      unsigned32_divide: unwrap_op(Operator::new(
        IrOpcode::Unsigned32Divide as u16,
        Properties::PURE,
        "Unsigned32Divide",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      check_equals_internalized_string: unwrap_op(Operator::new(
        IrOpcode::CheckEqualsInternalizedString as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckEqualsInternalizedString",
        OperatorCounts::new(2, 1, 1, 0, 1, 0),
      )),
      check_equals_symbol: unwrap_op(Operator::new(
        IrOpcode::CheckEqualsSymbol as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckEqualsSymbol",
        OperatorCounts::new(2, 1, 1, 0, 1, 0),
      )),
      check_heap_object: unwrap_op(Operator::new(
        IrOpcode::CheckHeapObject as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckHeapObject",
        OperatorCounts::new(1, 1, 1, 1, 1, 0),
      )),
      check_internalized_string: unwrap_op(Operator::new(
        IrOpcode::CheckInternalizedString as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckInternalizedString",
        OperatorCounts::new(1, 1, 1, 1, 1, 0),
      )),
      check_not_tagged_hole: unwrap_op(Operator::new(
        IrOpcode::CheckNotTaggedHole as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckNotTaggedHole",
        OperatorCounts::new(1, 1, 1, 1, 1, 0),
      )),
      check_receiver: unwrap_op(Operator::new(
        IrOpcode::CheckReceiver as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckReceiver",
        OperatorCounts::new(1, 1, 1, 1, 1, 0),
      )),
      check_receiver_or_null_or_undefined: unwrap_op(Operator::new(
        IrOpcode::CheckReceiverOrNullOrUndefined as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckReceiverOrNullOrUndefined",
        OperatorCounts::new(1, 1, 1, 1, 1, 0),
      )),
      check_symbol: unwrap_op(Operator::new(
        IrOpcode::CheckSymbol as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckSymbol",
        OperatorCounts::new(1, 1, 1, 1, 1, 0),
      )),
      checked_int32_add: unwrap_op(Operator::new(
        IrOpcode::CheckedInt32Add as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckedInt32Add",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
      )),
      checked_int32_div: unwrap_op(Operator::new(
        IrOpcode::CheckedInt32Div as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckedInt32Div",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
      )),
      checked_int32_mod: unwrap_op(Operator::new(
        IrOpcode::CheckedInt32Mod as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckedInt32Mod",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
      )),
      checked_int32_sub: unwrap_op(Operator::new(
        IrOpcode::CheckedInt32Sub as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckedInt32Sub",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
      )),
      checked_uint32_div: unwrap_op(Operator::new(
        IrOpcode::CheckedUint32Div as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckedUint32Div",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
      )),
      checked_uint32_mod: unwrap_op(Operator::new(
        IrOpcode::CheckedUint32Mod as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckedUint32Mod",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
      )),
      checked_additive_safe_integer_add: unwrap_op(Operator::new(
        IrOpcode::CheckedAdditiveSafeIntegerAdd as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckedAdditiveSafeIntegerAdd",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
      )),
      checked_additive_safe_integer_sub: unwrap_op(Operator::new(
        IrOpcode::CheckedAdditiveSafeIntegerSub as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckedAdditiveSafeIntegerSub",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
      )),
      checked_int64_add: unwrap_op(Operator::new(
        IrOpcode::CheckedInt64Add as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckedInt64Add",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
      )),
      checked_int64_sub: unwrap_op(Operator::new(
        IrOpcode::CheckedInt64Sub as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckedInt64Sub",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
      )),
      checked_int64_mul: unwrap_op(Operator::new(
        IrOpcode::CheckedInt64Mul as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckedInt64Mul",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
      )),
      checked_int64_div: unwrap_op(Operator::new(
        IrOpcode::CheckedInt64Div as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckedInt64Div",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
      )),
      checked_int64_mod: unwrap_op(Operator::new(
        IrOpcode::CheckedInt64Mod as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckedInt64Mod",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
      )),
      check_bounds: unwrap_op(Operator1::new(
        IrOpcode::CheckBounds as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckBounds",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
        CheckBoundsParameters::new(FeedbackSource::invalid(), CheckBoundsFlags::NONE),
      )),
      check_maps: unwrap_op(Operator1::new(
        IrOpcode::CheckMaps as u16,
        Properties::NO_THROW | Properties::NO_WRITE,
        "CheckMaps",
        OperatorCounts::new(1, 1, 1, 0, 1, 0),
        CheckMapsParameters::new(CheckMapsFlags::NONE, FeedbackSource::invalid()),
      )),
    }
  }
}

unsafe impl Send for SimplifiedOperatorGlobalCache {}
unsafe impl Sync for SimplifiedOperatorGlobalCache {}

static SIMPLIFIED_CACHE: LazyLock<SimplifiedOperatorGlobalCache> =
  LazyLock::new(SimplifiedOperatorGlobalCache::new);

/// SimplifiedOperatorBuilder
pub struct SimplifiedOperatorBuilder<'z, 'a> {
  zone: &'z Zone<'a>,
}

impl<'z, 'a> SimplifiedOperatorBuilder<'z, 'a> {
  /// 创建构建器
  pub fn new(zone: &'z Zone<'a>) -> Self {
    Self { zone }
  }

  /// 底层 Zone 引用
  pub fn zone(&self) -> &'z Zone<'a> {
    self.zone
  }

  // —— 纯算子 ——

  /// BooleanNot
  #[inline]
  pub fn boolean_not(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.boolean_not
  }

  /// NumberEqual
  #[inline]
  pub fn number_equal(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_equal
  }

  /// NumberLessThan
  #[inline]
  pub fn number_less_than(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_less_than
  }

  /// NumberLessThanOrEqual
  #[inline]
  pub fn number_less_than_or_equal(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_less_than_or_equal
  }

  /// NumberAdd
  #[inline]
  pub fn number_add(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_add
  }

  /// NumberSubtract
  #[inline]
  pub fn number_subtract(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_subtract
  }

  /// NumberMultiply
  #[inline]
  pub fn number_multiply(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_multiply
  }

  /// NumberDivide
  #[inline]
  pub fn number_divide(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_divide
  }

  /// NumberModulus
  #[inline]
  pub fn number_modulus(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_modulus
  }

  /// NumberBitwiseOr
  #[inline]
  pub fn number_bitwise_or(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_bitwise_or
  }

  /// NumberBitwiseXor
  #[inline]
  pub fn number_bitwise_xor(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_bitwise_xor
  }

  /// NumberBitwiseAnd
  #[inline]
  pub fn number_bitwise_and(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_bitwise_and
  }

  /// NumberShiftLeft
  #[inline]
  pub fn number_shift_left(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_shift_left
  }

  /// NumberShiftRight
  #[inline]
  pub fn number_shift_right(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_shift_right
  }

  /// NumberShiftRightLogical
  #[inline]
  pub fn number_shift_right_logical(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_shift_right_logical
  }

  /// NumberImul
  #[inline]
  pub fn number_imul(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_imul
  }

  /// NumberAbs
  #[inline]
  pub fn number_abs(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_abs
  }

  /// NumberClz32
  #[inline]
  pub fn number_clz32(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_clz32
  }

  /// NumberCeil
  #[inline]
  pub fn number_ceil(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_ceil
  }

  /// NumberFloor
  #[inline]
  pub fn number_floor(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_floor
  }

  /// NumberFround
  #[inline]
  pub fn number_fround(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_fround
  }

  /// NumberAcos
  #[inline]
  pub fn number_acos(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_acos
  }

  /// NumberAcosh
  #[inline]
  pub fn number_acosh(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_acosh
  }

  /// NumberAsin
  #[inline]
  pub fn number_asin(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_asin
  }

  /// NumberAsinh
  #[inline]
  pub fn number_asinh(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_asinh
  }

  /// NumberAtan
  #[inline]
  pub fn number_atan(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_atan
  }

  /// NumberAtan2
  #[inline]
  pub fn number_atan2(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_atan2
  }

  /// NumberAtanh
  #[inline]
  pub fn number_atanh(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_atanh
  }

  /// NumberCbrt
  #[inline]
  pub fn number_cbrt(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_cbrt
  }

  /// NumberCos
  #[inline]
  pub fn number_cos(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_cos
  }

  /// NumberCosh
  #[inline]
  pub fn number_cosh(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_cosh
  }

  /// NumberExp
  #[inline]
  pub fn number_exp(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_exp
  }

  /// NumberExpm1
  #[inline]
  pub fn number_expm1(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_expm1
  }

  /// NumberLog
  #[inline]
  pub fn number_log(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_log
  }

  /// NumberLog1p
  #[inline]
  pub fn number_log1p(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_log1p
  }

  /// NumberLog10
  #[inline]
  pub fn number_log10(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_log10
  }

  /// NumberLog2
  #[inline]
  pub fn number_log2(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_log2
  }

  /// NumberMax
  #[inline]
  pub fn number_max(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_max
  }

  /// NumberMin
  #[inline]
  pub fn number_min(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_min
  }

  /// NumberPow
  #[inline]
  pub fn number_pow(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_pow
  }

  /// NumberRound
  #[inline]
  pub fn number_round(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_round
  }

  /// NumberSign
  #[inline]
  pub fn number_sign(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_sign
  }

  /// NumberSin
  #[inline]
  pub fn number_sin(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_sin
  }

  /// NumberSinh
  #[inline]
  pub fn number_sinh(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_sinh
  }

  /// NumberSqrt
  #[inline]
  pub fn number_sqrt(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_sqrt
  }

  /// NumberTan
  #[inline]
  pub fn number_tan(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_tan
  }

  /// NumberTanh
  #[inline]
  pub fn number_tanh(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_tanh
  }

  /// NumberTrunc
  #[inline]
  pub fn number_trunc(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_trunc
  }

  /// NumberToBoolean
  #[inline]
  pub fn number_to_boolean(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_to_boolean
  }

  /// NumberToInt32
  #[inline]
  pub fn number_to_int32(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_to_int32
  }

  /// NumberToString
  #[inline]
  pub fn number_to_string(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_to_string
  }

  /// NumberToUint32
  #[inline]
  pub fn number_to_uint32(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_to_uint32
  }

  /// NumberToUint8Clamped
  #[inline]
  pub fn number_to_uint8_clamped(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_to_uint8_clamped
  }

  /// Integral32OrMinusZeroToBigInt
  #[inline]
  pub fn integral32_or_minus_zero_to_big_int(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.integral32_or_minus_zero_to_big_int
  }

  /// BigIntEqual
  #[inline]
  pub fn big_int_equal(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.big_int_equal
  }

  /// BigIntLessThan
  #[inline]
  pub fn big_int_less_than(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.big_int_less_than
  }

  /// BigIntLessThanOrEqual
  #[inline]
  pub fn big_int_less_than_or_equal(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.big_int_less_than_or_equal
  }

  /// BigIntNegate
  #[inline]
  pub fn big_int_negate(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.big_int_negate
  }

  /// StringConcat
  #[inline]
  pub fn string_concat(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.string_concat
  }

  /// StringToNumber
  #[inline]
  pub fn string_to_number(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.string_to_number
  }

  /// StringFromSingleCharCode
  #[inline]
  pub fn string_from_single_char_code(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.string_from_single_char_code
  }

  /// StringFromSingleCodePoint
  #[inline]
  pub fn string_from_single_code_point(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.string_from_single_code_point
  }

  /// StringIndexOf
  #[inline]
  pub fn string_index_of(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.string_index_of
  }

  /// StringLength
  #[inline]
  pub fn string_length(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.string_length
  }

  /// StringWrapperLength
  #[inline]
  pub fn string_wrapper_length(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.string_wrapper_length
  }

  /// TypeOf
  #[inline]
  pub fn type_of(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.type_of
  }

  /// PlainPrimitiveToNumber
  #[inline]
  pub fn plain_primitive_to_number(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.plain_primitive_to_number
  }

  /// PlainPrimitiveToWord32
  #[inline]
  pub fn plain_primitive_to_word32(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.plain_primitive_to_word32
  }

  /// PlainPrimitiveToFloat64
  #[inline]
  pub fn plain_primitive_to_float64(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.plain_primitive_to_float64
  }

  /// ChangeTaggedSignedToInt32
  #[inline]
  pub fn change_tagged_signed_to_int32(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_tagged_signed_to_int32
  }

  /// ChangeTaggedSignedToInt64
  #[inline]
  pub fn change_tagged_signed_to_int64(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_tagged_signed_to_int64
  }

  /// ChangeTaggedToInt32
  #[inline]
  pub fn change_tagged_to_int32(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_tagged_to_int32
  }

  /// ChangeTaggedToInt64
  #[inline]
  pub fn change_tagged_to_int64(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_tagged_to_int64
  }

  /// ChangeTaggedToUint32
  #[inline]
  pub fn change_tagged_to_uint32(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_tagged_to_uint32
  }

  /// ChangeTaggedToFloat64
  #[inline]
  pub fn change_tagged_to_float64(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_tagged_to_float64
  }

  /// ChangeTaggedToTaggedSigned
  #[inline]
  pub fn change_tagged_to_tagged_signed(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_tagged_to_tagged_signed
  }

  /// ChangeSmiOrHoleToFloat64
  #[inline]
  pub fn change_smi_or_hole_to_float64(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_smi_or_hole_to_float64
  }

  /// ChangeNumberOrHoleToFloat64
  #[inline]
  pub fn change_number_or_hole_to_float64(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_number_or_hole_to_float64
  }

  /// ChangeFloat64ToTaggedPointer
  #[inline]
  pub fn change_float64_to_tagged_pointer(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_float64_to_tagged_pointer
  }

  /// ChangeFloat64HoleToTagged
  #[inline]
  pub fn change_float64_hole_to_tagged(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_float64_hole_to_tagged
  }

  /// ChangeFloat64OrUndefinedOrHoleToTagged
  #[inline]
  pub fn change_float64_or_undefined_or_hole_to_tagged(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_float64_or_undefined_or_hole_to_tagged
  }

  /// ChangeInt31ToTaggedSigned
  #[inline]
  pub fn change_int31_to_tagged_signed(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_int31_to_tagged_signed
  }

  /// ChangeInt32ToTagged
  #[inline]
  pub fn change_int32_to_tagged(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_int32_to_tagged
  }

  /// ChangeInt64ToTagged
  #[inline]
  pub fn change_int64_to_tagged(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_int64_to_tagged
  }

  /// ChangeUint32ToTagged
  #[inline]
  pub fn change_uint32_to_tagged(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_uint32_to_tagged
  }

  /// ChangeUint64ToTagged
  #[inline]
  pub fn change_uint64_to_tagged(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_uint64_to_tagged
  }

  /// ChangeTaggedToBit
  #[inline]
  pub fn change_tagged_to_bit(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_tagged_to_bit
  }

  /// ChangeBitToTagged
  #[inline]
  pub fn change_bit_to_tagged(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_bit_to_tagged
  }

  /// TruncateBigIntToWord64
  #[inline]
  pub fn truncate_big_int_to_word64(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.truncate_big_int_to_word64
  }

  /// ChangeInt64ToBigInt
  #[inline]
  pub fn change_int64_to_big_int(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_int64_to_big_int
  }

  /// ChangeUint64ToBigInt
  #[inline]
  pub fn change_uint64_to_big_int(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.change_uint64_to_big_int
  }

  /// TruncateTaggedToBit
  #[inline]
  pub fn truncate_tagged_to_bit(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.truncate_tagged_to_bit
  }

  /// TruncateTaggedPointerToBit
  #[inline]
  pub fn truncate_tagged_pointer_to_bit(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.truncate_tagged_pointer_to_bit
  }

  /// TruncateNumberOrOddballToWord32
  #[inline]
  pub fn truncate_number_or_oddball_to_word32(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.truncate_number_or_oddball_to_word32
  }

  /// TruncateSmiOrHoleToWord32
  #[inline]
  pub fn truncate_smi_or_hole_to_word32(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.truncate_smi_or_hole_to_word32
  }

  /// TruncateNumberOrOddballOrHoleToWord32
  #[inline]
  pub fn truncate_number_or_oddball_or_hole_to_word32(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.truncate_number_or_oddball_or_hole_to_word32
  }

  /// TruncateTaggedToFloat64
  #[inline]
  pub fn truncate_tagged_to_float64(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.truncate_tagged_to_float64
  }

  /// TruncateTaggedToFloat64PreserveUndefined
  #[inline]
  pub fn truncate_tagged_to_float64_preserve_undefined(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.truncate_tagged_to_float64_preserve_undefined
  }

  /// ObjectIsArrayBufferView
  #[inline]
  pub fn object_is_array_buffer_view(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_array_buffer_view
  }

  /// ObjectIsBigInt
  #[inline]
  pub fn object_is_big_int(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_big_int
  }

  /// ObjectIsCallable
  #[inline]
  pub fn object_is_callable(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_callable
  }

  /// ObjectIsConstructor
  #[inline]
  pub fn object_is_constructor(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_constructor
  }

  /// ObjectIsDetectableCallable
  #[inline]
  pub fn object_is_detectable_callable(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_detectable_callable
  }

  /// ObjectIsMinusZero
  #[inline]
  pub fn object_is_minus_zero(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_minus_zero
  }

  /// NumberIsMinusZero
  #[inline]
  pub fn number_is_minus_zero(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_is_minus_zero
  }

  /// ObjectIsNaN
  #[inline]
  pub fn object_is_na_n(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_na_n
  }

  /// NumberIsNaN
  #[inline]
  pub fn number_is_na_n(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_is_na_n
  }

  /// ObjectIsNonCallable
  #[inline]
  pub fn object_is_non_callable(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_non_callable
  }

  /// ObjectIsNumber
  #[inline]
  pub fn object_is_number(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_number
  }

  /// ObjectIsReceiver
  #[inline]
  pub fn object_is_receiver(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_receiver
  }

  /// ObjectIsSmi
  #[inline]
  pub fn object_is_smi(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_smi
  }

  /// ObjectIsString
  #[inline]
  pub fn object_is_string(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_string
  }

  /// ObjectIsSymbol
  #[inline]
  pub fn object_is_symbol(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_symbol
  }

  /// ObjectIsUndetectable
  #[inline]
  pub fn object_is_undetectable(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_undetectable
  }

  /// NumberIsFloat64Hole
  #[inline]
  pub fn number_is_float64_hole(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_is_float64_hole
  }

  /// NumberIsFinite
  #[inline]
  pub fn number_is_finite(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_is_finite
  }

  /// ObjectIsFiniteNumber
  #[inline]
  pub fn object_is_finite_number(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_finite_number
  }

  /// NumberIsInteger
  #[inline]
  pub fn number_is_integer(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_is_integer
  }

  /// ObjectIsSafeInteger
  #[inline]
  pub fn object_is_safe_integer(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_safe_integer
  }

  /// NumberIsSafeInteger
  #[inline]
  pub fn number_is_safe_integer(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_is_safe_integer
  }

  /// ObjectIsInteger
  #[inline]
  pub fn object_is_integer(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.object_is_integer
  }

  /// ConvertTaggedHoleToUndefined
  #[inline]
  pub fn convert_tagged_hole_to_undefined(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.convert_tagged_hole_to_undefined
  }

  /// SameValue
  #[inline]
  pub fn same_value(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.same_value
  }

  /// SameValueNumbersOnly
  #[inline]
  pub fn same_value_numbers_only(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.same_value_numbers_only
  }

  /// NumberSameValue
  #[inline]
  pub fn number_same_value(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.number_same_value
  }

  /// ReferenceEqual
  #[inline]
  pub fn reference_equal(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.reference_equal
  }

  /// StringEqual
  #[inline]
  pub fn string_equal(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.string_equal
  }

  /// StringLessThan
  #[inline]
  pub fn string_less_than(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.string_less_than
  }

  /// StringLessThanOrEqual
  #[inline]
  pub fn string_less_than_or_equal(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.string_less_than_or_equal
  }

  /// StringOrOddballStrictEqual
  #[inline]
  pub fn string_or_oddball_strict_equal(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.string_or_oddball_strict_equal
  }

  /// ToBoolean
  #[inline]
  pub fn to_boolean(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.to_boolean
  }

  /// NewConsString
  #[inline]
  pub fn new_cons_string(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.new_cons_string
  }

  /// Unsigned32Divide
  #[inline]
  pub fn unsigned32_divide(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.unsigned32_divide
  }

  // —— Checked 算子 ——

  /// CheckEqualsInternalizedString
  #[inline]
  pub fn check_equals_internalized_string(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.check_equals_internalized_string
  }

  /// CheckEqualsSymbol
  #[inline]
  pub fn check_equals_symbol(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.check_equals_symbol
  }

  /// CheckHeapObject
  #[inline]
  pub fn check_heap_object(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.check_heap_object
  }

  /// CheckInternalizedString
  #[inline]
  pub fn check_internalized_string(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.check_internalized_string
  }

  /// CheckNotTaggedHole
  #[inline]
  pub fn check_not_tagged_hole(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.check_not_tagged_hole
  }

  /// CheckReceiver
  #[inline]
  pub fn check_receiver(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.check_receiver
  }

  /// CheckReceiverOrNullOrUndefined
  #[inline]
  pub fn check_receiver_or_null_or_undefined(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.check_receiver_or_null_or_undefined
  }

  /// CheckSymbol
  #[inline]
  pub fn check_symbol(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.check_symbol
  }

  /// CheckedInt32Add
  #[inline]
  pub fn checked_int32_add(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.checked_int32_add
  }

  /// CheckedInt32Div
  #[inline]
  pub fn checked_int32_div(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.checked_int32_div
  }

  /// CheckedInt32Mod
  #[inline]
  pub fn checked_int32_mod(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.checked_int32_mod
  }

  /// CheckedInt32Sub
  #[inline]
  pub fn checked_int32_sub(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.checked_int32_sub
  }

  /// CheckedUint32Div
  #[inline]
  pub fn checked_uint32_div(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.checked_uint32_div
  }

  /// CheckedUint32Mod
  #[inline]
  pub fn checked_uint32_mod(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.checked_uint32_mod
  }

  /// CheckedAdditiveSafeIntegerAdd
  #[inline]
  pub fn checked_additive_safe_integer_add(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.checked_additive_safe_integer_add
  }

  /// CheckedAdditiveSafeIntegerSub
  #[inline]
  pub fn checked_additive_safe_integer_sub(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.checked_additive_safe_integer_sub
  }

  /// CheckedInt64Add
  #[inline]
  pub fn checked_int64_add(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.checked_int64_add
  }

  /// CheckedInt64Sub
  #[inline]
  pub fn checked_int64_sub(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.checked_int64_sub
  }

  /// CheckedInt64Mul
  #[inline]
  pub fn checked_int64_mul(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.checked_int64_mul
  }

  /// CheckedInt64Div
  #[inline]
  pub fn checked_int64_div(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.checked_int64_div
  }

  /// CheckedInt64Mod
  #[inline]
  pub fn checked_int64_mod(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.checked_int64_mod
  }

  /// CheckBounds 算子
  pub fn check_bounds(
    &self,
    feedback: FeedbackSource,
    flags: CheckBoundsFlags,
  ) -> Result<&'z Operator, Error> {
    if !feedback.is_valid() && flags.bits() == 0 {
      return Ok(&SIMPLIFIED_CACHE.check_bounds.base);
    }
    let op = Operator1::new(
      IrOpcode::CheckBounds as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "CheckBounds",
      OperatorCounts::new(2, 1, 1, 1, 1, 0),
      CheckBoundsParameters::new(feedback, flags),
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// CheckMaps 算子
  pub fn check_maps(
    &self,
    flags: CheckMapsFlags,
    feedback: FeedbackSource,
  ) -> Result<&'z Operator, Error> {
    if !feedback.is_valid() && flags.bits() == 0 {
      return Ok(&SIMPLIFIED_CACHE.check_maps.base);
    }
    let mut props = Properties::NO_THROW;
    if (flags.bits() & CheckMapsFlags::TRY_MIGRATE_INSTANCE.bits()) == 0 {
      props |= Properties::NO_WRITE;
    }
    let op = Operator1::new(
      IrOpcode::CheckMaps as u16,
      props,
      "CheckMaps",
      OperatorCounts::new(1, 1, 1, 0, 1, 0),
      CheckMapsParameters::new(flags, feedback),
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// CheckNumber 算子
  pub fn check_number(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::CheckNumber as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "CheckNumber",
      OperatorCounts::new(1, 1, 1, 1, 1, 0),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// CheckSmi 算子
  pub fn check_smi(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::CheckSmi as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "CheckSmi",
      OperatorCounts::new(1, 1, 1, 1, 1, 0),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// CheckString 算子
  pub fn check_string(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::CheckString as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "CheckString",
      OperatorCounts::new(1, 1, 1, 1, 1, 0),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// CheckIf 算子
  pub fn check_if(
    &self,
    reason: DeoptimizeReason,
    feedback: FeedbackSource,
  ) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::CheckIf as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "CheckIf",
      OperatorCounts::new(1, 1, 1, 0, 1, 0),
      DeoptimizeParameters::new(reason, feedback),
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// CheckedInt32Mul 算子
  pub fn checked_int32_mul(&self, mode: CheckForMinusZeroMode) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::CheckedInt32Mul as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "CheckedInt32Mul",
      OperatorCounts::new(2, 1, 1, 1, 1, 0),
      mode,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }
}
