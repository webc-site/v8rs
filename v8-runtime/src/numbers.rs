// 由 tool/src/runtime/gen-runtime.js 生成
// 数值/字符串/BigInt/Atomics/Date/Operators 运行时函数

use strum::{Display, EnumIter, EnumString};

use crate::table::{RuntimeFunction, RuntimeFunctionId, find_runtime_function};

/// 数值与基础操作运行时函数 (64 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum NumbersRuntimeFunction {
  #[strum(serialize = "Runtime_StringToNumber", serialize = "StringToNumber")]
  StringToNumber,
  #[strum(serialize = "Runtime_StringParseInt", serialize = "StringParseInt")]
  StringParseInt,
  #[strum(serialize = "Runtime_StringParseFloat", serialize = "StringParseFloat")]
  StringParseFloat,
  #[strum(
    serialize = "Runtime_NumberToStringSlow",
    serialize = "NumberToStringSlow"
  )]
  NumberToStringSlow,
  #[strum(
    serialize = "Runtime_Float64ToStringSlow",
    serialize = "Float64ToStringSlow"
  )]
  Float64ToStringSlow,
  #[strum(serialize = "Runtime_MaxSmi", serialize = "MaxSmi")]
  MaxSmi,
  #[strum(serialize = "Runtime_IsSmi", serialize = "IsSmi")]
  IsSmi,
  #[strum(serialize = "Runtime_GetSubstitution", serialize = "GetSubstitution")]
  GetSubstitution,
  #[strum(
    serialize = "Runtime_StringReplaceOneCharWithString",
    serialize = "StringReplaceOneCharWithString"
  )]
  StringReplaceOneCharWithString,
  #[strum(
    serialize = "Runtime_StringLastIndexOf",
    serialize = "StringLastIndexOf"
  )]
  StringLastIndexOf,
  #[strum(serialize = "Runtime_StringSubstring", serialize = "StringSubstring")]
  StringSubstring,
  #[strum(serialize = "Runtime_StringAdd", serialize = "StringAdd")]
  StringAdd,
  #[strum(
    serialize = "Runtime_StringAdd_LhsIsStringConstant_Internalize",
    serialize = "StringAdd_LhsIsStringConstant_Internalize",
    serialize = "StringAddLhsIsStringConstantInternalize"
  )]
  StringAddLhsIsStringConstantInternalize,
  #[strum(
    serialize = "Runtime_StringAdd_RhsIsStringConstant_Internalize",
    serialize = "StringAdd_RhsIsStringConstant_Internalize",
    serialize = "StringAddRhsIsStringConstantInternalize"
  )]
  StringAddRhsIsStringConstantInternalize,
  #[strum(
    serialize = "Runtime_InternalizeString",
    serialize = "InternalizeString"
  )]
  InternalizeString,
  #[strum(serialize = "Runtime_StringCharCodeAt", serialize = "StringCharCodeAt")]
  StringCharCodeAt,
  #[strum(
    serialize = "Runtime_StringCodePointAt",
    serialize = "StringCodePointAt"
  )]
  StringCodePointAt,
  #[strum(
    serialize = "Runtime_StringBuilderConcat",
    serialize = "StringBuilderConcat"
  )]
  StringBuilderConcat,
  #[strum(serialize = "Runtime_StringToArray", serialize = "StringToArray")]
  StringToArray,
  #[strum(serialize = "Runtime_StringLessThan", serialize = "StringLessThan")]
  StringLessThan,
  #[strum(
    serialize = "Runtime_StringLessThanOrEqual",
    serialize = "StringLessThanOrEqual"
  )]
  StringLessThanOrEqual,
  #[strum(
    serialize = "Runtime_StringGreaterThan",
    serialize = "StringGreaterThan"
  )]
  StringGreaterThan,
  #[strum(
    serialize = "Runtime_StringGreaterThanOrEqual",
    serialize = "StringGreaterThanOrEqual"
  )]
  StringGreaterThanOrEqual,
  #[strum(serialize = "Runtime_StringEqual", serialize = "StringEqual")]
  StringEqual,
  #[strum(serialize = "Runtime_StringCompare", serialize = "StringCompare")]
  StringCompare,
  #[strum(serialize = "Runtime_FlattenString", serialize = "FlattenString")]
  FlattenString,
  #[strum(serialize = "Runtime_StringMaxLength", serialize = "StringMaxLength")]
  StringMaxLength,
  #[strum(
    serialize = "Runtime_StringEscapeQuotes",
    serialize = "StringEscapeQuotes"
  )]
  StringEscapeQuotes,
  #[strum(
    serialize = "Runtime_StringIsWellFormed",
    serialize = "StringIsWellFormed"
  )]
  StringIsWellFormed,
  #[strum(
    serialize = "Runtime_StringToWellFormed",
    serialize = "StringToWellFormed"
  )]
  StringToWellFormed,
  #[strum(
    serialize = "Runtime_BigIntCompareToNumber",
    serialize = "BigIntCompareToNumber"
  )]
  BigIntCompareToNumber,
  #[strum(
    serialize = "Runtime_BigIntCompareToString",
    serialize = "BigIntCompareToString"
  )]
  BigIntCompareToString,
  #[strum(
    serialize = "Runtime_BigIntEqualToBigInt",
    serialize = "BigIntEqualToBigInt"
  )]
  BigIntEqualToBigInt,
  #[strum(
    serialize = "Runtime_BigIntEqualToNumber",
    serialize = "BigIntEqualToNumber"
  )]
  BigIntEqualToNumber,
  #[strum(
    serialize = "Runtime_BigIntEqualToString",
    serialize = "BigIntEqualToString"
  )]
  BigIntEqualToString,
  #[strum(serialize = "Runtime_BigIntToNumber", serialize = "BigIntToNumber")]
  BigIntToNumber,
  #[strum(serialize = "Runtime_ToBigInt", serialize = "ToBigInt")]
  ToBigInt,
  #[strum(
    serialize = "Runtime_ToBigIntConvertNumber",
    serialize = "ToBigIntConvertNumber"
  )]
  ToBigIntConvertNumber,
  #[strum(
    serialize = "Runtime_BigIntExponentiate",
    serialize = "BigIntExponentiate"
  )]
  BigIntExponentiate,
  #[strum(serialize = "Runtime_BigIntUnaryOp", serialize = "BigIntUnaryOp")]
  BigIntUnaryOp,
  #[strum(serialize = "Runtime_AtomicsLoad64", serialize = "AtomicsLoad64")]
  AtomicsLoad64,
  #[strum(serialize = "Runtime_AtomicsStore64", serialize = "AtomicsStore64")]
  AtomicsStore64,
  #[strum(serialize = "Runtime_AtomicsExchange", serialize = "AtomicsExchange")]
  AtomicsExchange,
  #[strum(
    serialize = "Runtime_AtomicsCompareExchange",
    serialize = "AtomicsCompareExchange"
  )]
  AtomicsCompareExchange,
  #[strum(serialize = "Runtime_AtomicsAdd", serialize = "AtomicsAdd")]
  AtomicsAdd,
  #[strum(serialize = "Runtime_AtomicsSub", serialize = "AtomicsSub")]
  AtomicsSub,
  #[strum(serialize = "Runtime_AtomicsAnd", serialize = "AtomicsAnd")]
  AtomicsAnd,
  #[strum(serialize = "Runtime_AtomicsOr", serialize = "AtomicsOr")]
  AtomicsOr,
  #[strum(serialize = "Runtime_AtomicsXor", serialize = "AtomicsXor")]
  AtomicsXor,
  #[strum(
    serialize = "Runtime_AtomicsLoadSharedStructOrArray",
    serialize = "AtomicsLoadSharedStructOrArray"
  )]
  AtomicsLoadSharedStructOrArray,
  #[strum(
    serialize = "Runtime_AtomicsStoreSharedStructOrArray",
    serialize = "AtomicsStoreSharedStructOrArray"
  )]
  AtomicsStoreSharedStructOrArray,
  #[strum(
    serialize = "Runtime_AtomicsExchangeSharedStructOrArray",
    serialize = "AtomicsExchangeSharedStructOrArray"
  )]
  AtomicsExchangeSharedStructOrArray,
  #[strum(
    serialize = "Runtime_AtomicsCompareExchangeSharedStructOrArray",
    serialize = "AtomicsCompareExchangeSharedStructOrArray"
  )]
  AtomicsCompareExchangeSharedStructOrArray,
  #[strum(serialize = "Runtime_DateCurrentTime", serialize = "DateCurrentTime")]
  DateCurrentTime,
  #[strum(serialize = "Runtime_Add", serialize = "Add")]
  Add,
  #[strum(serialize = "Runtime_Equal", serialize = "Equal")]
  Equal,
  #[strum(serialize = "Runtime_NotEqual", serialize = "NotEqual")]
  NotEqual,
  #[strum(serialize = "Runtime_StrictEqual", serialize = "StrictEqual")]
  StrictEqual,
  #[strum(serialize = "Runtime_StrictNotEqual", serialize = "StrictNotEqual")]
  StrictNotEqual,
  #[strum(serialize = "Runtime_ReferenceEqual", serialize = "ReferenceEqual")]
  ReferenceEqual,
  #[strum(serialize = "Runtime_LessThan", serialize = "LessThan")]
  LessThan,
  #[strum(serialize = "Runtime_GreaterThan", serialize = "GreaterThan")]
  GreaterThan,
  #[strum(serialize = "Runtime_LessThanOrEqual", serialize = "LessThanOrEqual")]
  LessThanOrEqual,
  #[strum(
    serialize = "Runtime_GreaterThanOrEqual",
    serialize = "GreaterThanOrEqual"
  )]
  GreaterThanOrEqual,
}

impl NumbersRuntimeFunction {
  pub const fn name(&self) -> &'static str {
    match self {
      Self::StringToNumber => "StringToNumber",
      Self::StringParseInt => "StringParseInt",
      Self::StringParseFloat => "StringParseFloat",
      Self::NumberToStringSlow => "NumberToStringSlow",
      Self::Float64ToStringSlow => "Float64ToStringSlow",
      Self::MaxSmi => "MaxSmi",
      Self::IsSmi => "IsSmi",
      Self::GetSubstitution => "GetSubstitution",
      Self::StringReplaceOneCharWithString => "StringReplaceOneCharWithString",
      Self::StringLastIndexOf => "StringLastIndexOf",
      Self::StringSubstring => "StringSubstring",
      Self::StringAdd => "StringAdd",
      Self::StringAddLhsIsStringConstantInternalize => "StringAdd_LhsIsStringConstant_Internalize",
      Self::StringAddRhsIsStringConstantInternalize => "StringAdd_RhsIsStringConstant_Internalize",
      Self::InternalizeString => "InternalizeString",
      Self::StringCharCodeAt => "StringCharCodeAt",
      Self::StringCodePointAt => "StringCodePointAt",
      Self::StringBuilderConcat => "StringBuilderConcat",
      Self::StringToArray => "StringToArray",
      Self::StringLessThan => "StringLessThan",
      Self::StringLessThanOrEqual => "StringLessThanOrEqual",
      Self::StringGreaterThan => "StringGreaterThan",
      Self::StringGreaterThanOrEqual => "StringGreaterThanOrEqual",
      Self::StringEqual => "StringEqual",
      Self::StringCompare => "StringCompare",
      Self::FlattenString => "FlattenString",
      Self::StringMaxLength => "StringMaxLength",
      Self::StringEscapeQuotes => "StringEscapeQuotes",
      Self::StringIsWellFormed => "StringIsWellFormed",
      Self::StringToWellFormed => "StringToWellFormed",
      Self::BigIntCompareToNumber => "BigIntCompareToNumber",
      Self::BigIntCompareToString => "BigIntCompareToString",
      Self::BigIntEqualToBigInt => "BigIntEqualToBigInt",
      Self::BigIntEqualToNumber => "BigIntEqualToNumber",
      Self::BigIntEqualToString => "BigIntEqualToString",
      Self::BigIntToNumber => "BigIntToNumber",
      Self::ToBigInt => "ToBigInt",
      Self::ToBigIntConvertNumber => "ToBigIntConvertNumber",
      Self::BigIntExponentiate => "BigIntExponentiate",
      Self::BigIntUnaryOp => "BigIntUnaryOp",
      Self::AtomicsLoad64 => "AtomicsLoad64",
      Self::AtomicsStore64 => "AtomicsStore64",
      Self::AtomicsExchange => "AtomicsExchange",
      Self::AtomicsCompareExchange => "AtomicsCompareExchange",
      Self::AtomicsAdd => "AtomicsAdd",
      Self::AtomicsSub => "AtomicsSub",
      Self::AtomicsAnd => "AtomicsAnd",
      Self::AtomicsOr => "AtomicsOr",
      Self::AtomicsXor => "AtomicsXor",
      Self::AtomicsLoadSharedStructOrArray => "AtomicsLoadSharedStructOrArray",
      Self::AtomicsStoreSharedStructOrArray => "AtomicsStoreSharedStructOrArray",
      Self::AtomicsExchangeSharedStructOrArray => "AtomicsExchangeSharedStructOrArray",
      Self::AtomicsCompareExchangeSharedStructOrArray => {
        "AtomicsCompareExchangeSharedStructOrArray"
      }
      Self::DateCurrentTime => "DateCurrentTime",
      Self::Add => "Add",
      Self::Equal => "Equal",
      Self::NotEqual => "NotEqual",
      Self::StrictEqual => "StrictEqual",
      Self::StrictNotEqual => "StrictNotEqual",
      Self::ReferenceEqual => "ReferenceEqual",
      Self::LessThan => "LessThan",
      Self::GreaterThan => "GreaterThan",
      Self::LessThanOrEqual => "LessThanOrEqual",
      Self::GreaterThanOrEqual => "GreaterThanOrEqual",
    }
  }

  pub fn descriptor(&self) -> Option<&'static RuntimeFunction> {
    find_runtime_function(self.name())
  }

  pub fn function_id(&self) -> Option<RuntimeFunctionId> {
    self.descriptor().map(|f| f.id)
  }

  pub const fn is_numbers(&self) -> bool {
    matches!(
      self,
      Self::StringToNumber
        | Self::StringParseInt
        | Self::StringParseFloat
        | Self::NumberToStringSlow
        | Self::Float64ToStringSlow
        | Self::MaxSmi
        | Self::IsSmi
    )
  }

  pub const fn is_strings(&self) -> bool {
    matches!(
      self,
      Self::GetSubstitution
        | Self::StringReplaceOneCharWithString
        | Self::StringLastIndexOf
        | Self::StringSubstring
        | Self::StringAdd
        | Self::StringAddLhsIsStringConstantInternalize
        | Self::StringAddRhsIsStringConstantInternalize
        | Self::InternalizeString
        | Self::StringCharCodeAt
        | Self::StringCodePointAt
        | Self::StringBuilderConcat
        | Self::StringToArray
        | Self::StringLessThan
        | Self::StringLessThanOrEqual
        | Self::StringGreaterThan
        | Self::StringGreaterThanOrEqual
        | Self::StringEqual
        | Self::StringCompare
        | Self::FlattenString
        | Self::StringMaxLength
        | Self::StringEscapeQuotes
        | Self::StringIsWellFormed
        | Self::StringToWellFormed
    )
  }

  pub const fn is_bigint(&self) -> bool {
    matches!(
      self,
      Self::BigIntCompareToNumber
        | Self::BigIntCompareToString
        | Self::BigIntEqualToBigInt
        | Self::BigIntEqualToNumber
        | Self::BigIntEqualToString
        | Self::BigIntToNumber
        | Self::ToBigInt
        | Self::ToBigIntConvertNumber
        | Self::BigIntExponentiate
        | Self::BigIntUnaryOp
    )
  }

  pub const fn is_atomics(&self) -> bool {
    matches!(
      self,
      Self::AtomicsLoad64
        | Self::AtomicsStore64
        | Self::AtomicsExchange
        | Self::AtomicsCompareExchange
        | Self::AtomicsAdd
        | Self::AtomicsSub
        | Self::AtomicsAnd
        | Self::AtomicsOr
        | Self::AtomicsXor
        | Self::AtomicsLoadSharedStructOrArray
        | Self::AtomicsStoreSharedStructOrArray
        | Self::AtomicsExchangeSharedStructOrArray
        | Self::AtomicsCompareExchangeSharedStructOrArray
    )
  }

  pub const fn is_date(&self) -> bool {
    matches!(self, Self::DateCurrentTime)
  }

  pub const fn is_operators(&self) -> bool {
    matches!(
      self,
      Self::Add
        | Self::Equal
        | Self::NotEqual
        | Self::StrictEqual
        | Self::StrictNotEqual
        | Self::ReferenceEqual
        | Self::LessThan
        | Self::GreaterThan
        | Self::LessThanOrEqual
        | Self::GreaterThanOrEqual
    )
  }
}

pub const fn string_to_number() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringToNumber
}

pub const fn string_parse_int() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringParseInt
}

pub const fn string_parse_float() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringParseFloat
}

pub const fn number_to_string_slow() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::NumberToStringSlow
}

pub const fn float64_to_string_slow() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::Float64ToStringSlow
}

pub const fn max_smi() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::MaxSmi
}

pub const fn is_smi() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::IsSmi
}

pub const fn get_substitution() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::GetSubstitution
}

pub const fn string_replace_one_char_with_string() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringReplaceOneCharWithString
}

pub const fn string_last_index_of() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringLastIndexOf
}

pub const fn string_substring() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringSubstring
}

pub const fn string_add() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringAdd
}

pub const fn string_add_lhs_is_string_constant_internalize() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringAddLhsIsStringConstantInternalize
}

pub const fn string_add_rhs_is_string_constant_internalize() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringAddRhsIsStringConstantInternalize
}

pub const fn internalize_string() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::InternalizeString
}

pub const fn string_char_code_at() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringCharCodeAt
}

pub const fn string_code_point_at() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringCodePointAt
}

pub const fn string_builder_concat() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringBuilderConcat
}

pub const fn string_to_array() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringToArray
}

pub const fn string_less_than() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringLessThan
}

pub const fn string_less_than_or_equal() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringLessThanOrEqual
}

pub const fn string_greater_than() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringGreaterThan
}

pub const fn string_greater_than_or_equal() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringGreaterThanOrEqual
}

pub const fn string_equal() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringEqual
}

pub const fn string_compare() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringCompare
}

pub const fn flatten_string() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::FlattenString
}

pub const fn string_max_length() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringMaxLength
}

pub const fn string_escape_quotes() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringEscapeQuotes
}

pub const fn string_is_well_formed() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringIsWellFormed
}

pub const fn string_to_well_formed() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StringToWellFormed
}

pub const fn big_int_compare_to_number() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::BigIntCompareToNumber
}

pub const fn big_int_compare_to_string() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::BigIntCompareToString
}

pub const fn big_int_equal_to_big_int() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::BigIntEqualToBigInt
}

pub const fn big_int_equal_to_number() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::BigIntEqualToNumber
}

pub const fn big_int_equal_to_string() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::BigIntEqualToString
}

pub const fn big_int_to_number() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::BigIntToNumber
}

pub const fn to_big_int() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::ToBigInt
}

pub const fn to_big_int_convert_number() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::ToBigIntConvertNumber
}

pub const fn big_int_exponentiate() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::BigIntExponentiate
}

pub const fn big_int_unary_op() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::BigIntUnaryOp
}

pub const fn atomics_load64() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::AtomicsLoad64
}

pub const fn atomics_store64() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::AtomicsStore64
}

pub const fn atomics_exchange() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::AtomicsExchange
}

pub const fn atomics_compare_exchange() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::AtomicsCompareExchange
}

pub const fn atomics_add() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::AtomicsAdd
}

pub const fn atomics_sub() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::AtomicsSub
}

pub const fn atomics_and() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::AtomicsAnd
}

pub const fn atomics_or() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::AtomicsOr
}

pub const fn atomics_xor() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::AtomicsXor
}

pub const fn atomics_load_shared_struct_or_array() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::AtomicsLoadSharedStructOrArray
}

pub const fn atomics_store_shared_struct_or_array() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::AtomicsStoreSharedStructOrArray
}

pub const fn atomics_exchange_shared_struct_or_array() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::AtomicsExchangeSharedStructOrArray
}

pub const fn atomics_compare_exchange_shared_struct_or_array() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::AtomicsCompareExchangeSharedStructOrArray
}

pub const fn date_current_time() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::DateCurrentTime
}

pub const fn add() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::Add
}

pub const fn equal() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::Equal
}

pub const fn not_equal() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::NotEqual
}

pub const fn strict_equal() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StrictEqual
}

pub const fn strict_not_equal() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::StrictNotEqual
}

pub const fn reference_equal() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::ReferenceEqual
}

pub const fn less_than() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::LessThan
}

pub const fn greater_than() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::GreaterThan
}

pub const fn less_than_or_equal() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::LessThanOrEqual
}

pub const fn greater_than_or_equal() -> NumbersRuntimeFunction {
  NumbersRuntimeFunction::GreaterThanOrEqual
}
